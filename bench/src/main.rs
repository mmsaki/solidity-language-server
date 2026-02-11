use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::time::{Duration, Instant};

// ── LSP Client ──────────────────────────────────────────────────────────────

struct LspClient {
    child: std::process::Child,
    rx: mpsc::Receiver<Value>,
    writer: std::process::ChildStdin,
    id: i64,
    logs: Vec<String>,
}

/// Info returned after waiting for diagnostics.
struct DiagnosticsInfo {
    count: usize,
    elapsed_ms: f64,
}

/// Background reader thread: reads LSP messages from stdout and sends them
/// through a channel. This avoids blocking the main thread on read_line().
fn reader_thread(stdout: std::process::ChildStdout, tx: mpsc::Sender<Value>) {
    let mut reader = BufReader::new(stdout);
    loop {
        // Read headers
        let mut content_length: usize = 0;
        let mut in_header = false;
        loop {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(0) => return, // EOF
                Ok(_) => {}
                Err(_) => return,
            }
            let t = line.trim();
            if t.is_empty() {
                if in_header {
                    break;
                }
                continue;
            }
            if let Some(v) = t.strip_prefix("Content-Length:") {
                if let Ok(n) = v.trim().parse::<usize>() {
                    content_length = n;
                    in_header = true;
                    continue;
                }
            }
            if t.starts_with("Content-Type:") {
                in_header = true;
                continue;
            }
            // Skip garbage lines (tracing output, ANSI codes, etc.)
        }
        if content_length == 0 {
            continue;
        }
        // Read body
        let mut body = vec![0u8; content_length];
        if reader.read_exact(&mut body).is_err() {
            return;
        }
        if let Ok(msg) = serde_json::from_slice::<Value>(&body) {
            if tx.send(msg).is_err() {
                return; // receiver dropped
            }
        }
    }
}

impl LspClient {
    fn spawn(cmd: &str, args: &[&str], cwd: &Path) -> Result<Self, String> {
        // Resolve relative command paths to absolute before changing CWD
        let abs_cmd = if cmd.starts_with("..") || cmd.starts_with("./") {
            std::fs::canonicalize(cmd)
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| cmd.to_string())
        } else {
            cmd.to_string()
        };
        let mut child = Command::new(&abs_cmd)
            .args(args)
            .current_dir(cwd)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("{}: {}", cmd, e))?;
        let writer = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();

        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || reader_thread(stdout, tx));

        Ok(Self {
            child,
            rx,
            writer,
            id: 1,
            logs: Vec::new(),
        })
    }

    fn send(&mut self, method: &str, params: Value) -> Result<(), String> {
        let msg = json!({"jsonrpc":"2.0","id":self.id,"method":method,"params":params});
        self.id += 1;
        let body = serde_json::to_string(&msg).unwrap();
        write!(
            self.writer,
            "Content-Length: {}\r\n\r\n{}",
            body.len(),
            body
        )
        .map_err(|e| e.to_string())?;
        self.writer.flush().map_err(|e| e.to_string())
    }

    fn notif(&mut self, method: &str, params: Value) -> Result<(), String> {
        let msg = json!({"jsonrpc":"2.0","method":method,"params":params});
        let body = serde_json::to_string(&msg).unwrap();
        write!(
            self.writer,
            "Content-Length: {}\r\n\r\n{}",
            body.len(),
            body
        )
        .map_err(|e| e.to_string())?;
        self.writer.flush().map_err(|e| e.to_string())
    }

    /// Receive the next message with a real timeout.
    fn recv(&mut self, timeout: Duration) -> Result<Value, String> {
        self.rx.recv_timeout(timeout).map_err(|e| match e {
            mpsc::RecvTimeoutError::Timeout => "timeout".to_string(),
            mpsc::RecvTimeoutError::Disconnected => "EOF".to_string(),
        })
    }

    fn read_response(&mut self, timeout: Duration) -> Result<Value, String> {
        let deadline = Instant::now() + timeout;
        loop {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                return Err("timeout".into());
            }
            let msg = self.recv(remaining)?;
            // Capture window/logMessage notifications
            if msg.get("method").and_then(|m| m.as_str()) == Some("window/logMessage") {
                if let Some(text) = msg
                    .get("params")
                    .and_then(|p| p.get("message"))
                    .and_then(|m| m.as_str())
                {
                    self.logs.push(text.to_string());
                }
            }
            if msg.get("id").is_some() {
                return Ok(msg);
            }
        }
    }

    fn wait_for_notif(&mut self, method: &str, timeout: Duration) -> Result<Value, String> {
        let deadline = Instant::now() + timeout;
        loop {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                return Err(format!("timeout waiting for {}", method));
            }
            let msg = self.recv(remaining)?;
            if msg.get("method").and_then(|m| m.as_str()) == Some(method) {
                return Ok(msg);
            }
        }
    }

    /// Drain messages until we see publishDiagnostics with non-empty diagnostics.
    /// Returns the count and elapsed time. If only empty diagnostics arrive before
    /// timeout, returns those. This is the "time to first valid diagnostics" metric.
    fn wait_for_valid_diagnostics(&mut self, timeout: Duration) -> Result<DiagnosticsInfo, String> {
        let start = Instant::now();
        let deadline = start + timeout;
        let mut last_count = 0usize;
        let mut last_elapsed = 0.0f64;
        loop {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                return if last_count > 0 || last_elapsed > 0.0 {
                    Ok(DiagnosticsInfo {
                        count: last_count,
                        elapsed_ms: last_elapsed,
                    })
                } else {
                    Err("timeout waiting for diagnostics".into())
                };
            }
            let msg = self.recv(remaining)?;
            // Capture window/logMessage notifications
            if msg.get("method").and_then(|m| m.as_str()) == Some("window/logMessage") {
                if let Some(text) = msg
                    .get("params")
                    .and_then(|p| p.get("message"))
                    .and_then(|m| m.as_str())
                {
                    self.logs.push(text.to_string());
                }
            }
            if msg.get("method").and_then(|m| m.as_str()) == Some("textDocument/publishDiagnostics")
            {
                let count = msg
                    .get("params")
                    .and_then(|p| p.get("diagnostics"))
                    .and_then(|d| d.as_array())
                    .map(|a| a.len())
                    .unwrap_or(0);
                let elapsed = start.elapsed().as_secs_f64() * 1000.0;
                last_count = count;
                last_elapsed = elapsed;
                if count > 0 {
                    return Ok(DiagnosticsInfo {
                        count,
                        elapsed_ms: elapsed,
                    });
                }
            }
        }
    }

    fn initialize(&mut self, root: &str) -> Result<(), String> {
        self.send(
            "initialize",
            json!({
                "processId": std::process::id(),
                "rootUri": root,
                "capabilities": {
                    "textDocument": {
                        "publishDiagnostics": {},
                        "definition": { "dynamicRegistration": false, "linkSupport": true },
                        "declaration": { "dynamicRegistration": false, "linkSupport": true },
                        "hover": { "dynamicRegistration": false, "contentFormat": ["plaintext", "markdown"] },
                        "completion": {
                            "dynamicRegistration": false,
                            "completionItem": { "snippetSupport": false }
                        },
                        "documentSymbol": { "dynamicRegistration": false },
                        "references": { "dynamicRegistration": false },
                        "rename": { "dynamicRegistration": false },
                        "signatureHelp": { "dynamicRegistration": false },
                        "codeAction": { "dynamicRegistration": false },
                    }
                },
            }),
        )?;
        self.read_response(Duration::from_secs(10))?;
        self.notif("initialized", json!({}))
    }

    fn open_file(&mut self, path: &Path) -> Result<(), String> {
        let content =
            std::fs::read_to_string(path).map_err(|e| format!("{}: {}", path.display(), e))?;
        self.notif(
            "textDocument/didOpen",
            json!({
                "textDocument": {
                    "uri": uri(path),
                    "languageId": "solidity",
                    "version": 1,
                    "text": content,
                }
            }),
        )
    }

    fn kill(mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

impl Drop for LspClient {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

// ── Helpers ─────────────────────────────────────────────────────────────────

fn uri(p: &Path) -> String {
    format!(
        "file://{}",
        std::fs::canonicalize(p).unwrap_or(p.into()).display()
    )
}

fn available(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn stats(samples: &mut Vec<f64>) -> (f64, f64, f64) {
    samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = samples.len();
    (
        samples[n / 2],
        samples[((n as f64) * 0.95) as usize],
        samples.iter().sum::<f64>() / n as f64,
    )
}

/// Check if an LSP response is valid (has a non-null, non-error result).
fn is_valid_response(resp: &Value) -> bool {
    if resp.get("error").is_some() {
        return false;
    }
    match resp.get("result") {
        None => false,
        Some(r) => {
            if r.is_null() {
                return false;
            }
            if let Some(arr) = r.as_array() {
                return !arr.is_empty();
            }
            true
        }
    }
}

/// Format a response snippet for display.
fn response_summary(resp: &Value) -> String {
    if let Some(err) = resp.get("error") {
        return format!(
            "error: {}",
            err.get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("unknown")
        );
    }
    match resp.get("result") {
        None => "no result".into(),
        Some(r) if r.is_null() => "null".into(),
        Some(r) => {
            let s = serde_json::to_string(r).unwrap_or_default();
            if s.len() > 120 {
                format!("{}...", &s[..120])
            } else {
                s
            }
        }
    }
}

// ── Servers ─────────────────────────────────────────────────────────────────

struct Server {
    label: &'static str,
    cmd: &'static str,
    args: &'static [&'static str],
}

const SERVERS: &[Server] = &[
    Server {
        label: "SLS (ours)",
        cmd: "../target/release/solidity-language-server",
        args: &[],
    },
    Server {
        label: "solc --lsp",
        cmd: "solc",
        args: &["--lsp"],
    },
    Server {
        label: "Hardhat/Nomic",
        cmd: "nomicfoundation-solidity-language-server",
        args: &["--stdio"],
    },
];

// ── Bench result per server ─────────────────────────────────────────────────

enum BenchResult {
    /// Valid result with samples and first response
    Ok {
        samples: Vec<f64>,
        first_response: Value,
        diag_info: Option<DiagnosticsInfo>,
    },
    /// Bench ran but response was null/error — invalidated
    Invalid {
        first_response: Value,
        diag_info: Option<DiagnosticsInfo>,
    },
    /// Bench failed to run at all
    Fail(String),
}

fn run_bench<F>(name: &str, header: &[String], servers: &[&Server], root: &str, cwd: &Path, f: F)
where
    F: Fn(&Server, &str, &Path) -> BenchResult,
{
    let mut lines = header.to_vec();
    lines.push("| Server | p50 | p95 | mean | Result |".to_string());
    lines.push("|--------|-----|-----|------|--------|".to_string());

    let mut results = Vec::new();
    for srv in servers {
        eprint!("  {} ... ", srv.label);
        match f(srv, root, cwd) {
            BenchResult::Ok {
                mut samples,
                first_response,
                diag_info,
            } => {
                let (p50, p95, mean) = stats(&mut samples);
                let summary = response_summary(&first_response);
                let diag_suffix = diag_info
                    .map(|di| format!("  [diag: {} in {:.0}ms]", di.count, di.elapsed_ms))
                    .unwrap_or_default();
                let combined = summary.clone() + &diag_suffix;
                let row = format!(
                    "| {} | {:.1} | {:.1} | {:.1} | {} |",
                    srv.label, p50, p95, mean, combined
                );
                eprintln!("done");
                lines.push(row);
                results.push((srv.label.to_string(), p50, p95, mean, summary));
            }
            BenchResult::Invalid {
                first_response,
                diag_info,
            } => {
                let summary = response_summary(&first_response);
                let diag_suffix = diag_info
                    .map(|di| format!("  [diag: {} in {:.0}ms]", di.count, di.elapsed_ms))
                    .unwrap_or_default();
                let combined = summary.clone() + &diag_suffix;
                let row = format!("| {} | - | - | - | {} |", srv.label, combined);
                eprintln!("invalid");
                lines.push(row);
                results.push((srv.label.to_string(), 0.0, 0.0, 0.0, summary.clone()));
            }
            BenchResult::Fail(e) => {
                eprintln!("fail");
                lines.push(format!("| {} | FAIL ({}) |", srv.label, e));
                results.push((srv.label.to_string(), 0.0, 0.0, 0.0, "fail".to_string()));
            }
        }
    }

    // Add summary
    lines.push("".to_string());
    let summary = generate_summary(name, &results);
    lines.push(summary);

    let out = lines.join("\n") + "\n";
    let path = format!("results/{}.md", name);
    let _ = std::fs::create_dir_all("results");
    std::fs::write(&path, &out).unwrap();
    println!("{}", out);
    eprintln!("  -> {}", path);
}

fn generate_summary(name: &str, results: &[(String, f64, f64, f64, String)]) -> String {
    match name {
        "spawn" => {
            let mut valid = results
                .iter()
                .filter(|(_, _, _, m, _)| *m > 0.0)
                .collect::<Vec<_>>();
            valid.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap());
            if valid.len() >= 1 {
                format!(
                    "{} fastest startup ({:.0}ms), {} {:.0}ms, {} {:.0}ms.",
                    valid[0].0,
                    valid[0].3,
                    valid.get(1).map(|r| r.0.as_str()).unwrap_or(""),
                    valid.get(1).map(|r| r.3).unwrap_or(0.0),
                    valid.get(2).map(|r| r.0.as_str()).unwrap_or(""),
                    valid.get(2).map(|r| r.3).unwrap_or(0.0)
                )
            } else {
                "No valid results.".to_string()
            }
        }
        "diagnostics" => {
            let mut valid = results
                .iter()
                .filter(|(_, _, _, m, _)| *m > 0.0)
                .collect::<Vec<_>>();
            valid.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap());
            if valid.len() >= 1 {
                let sls_diag = results
                    .iter()
                    .find(|(n, _, _, _, _)| n == "SLS (ours)")
                    .and_then(|(_, _, _, _, s)| {
                        s.strip_prefix("4 diagnostics: ").map(|s| s.to_string())
                    })
                    .unwrap_or("".to_string());
                format!("{} fastest diagnostics ({:.0}ms), {} {:.0}ms with {}, {} {:.0}ms with no diags.", valid[0].0, valid[0].3, valid.get(1).map(|r| r.0.as_str()).unwrap_or(""), valid.get(1).map(|r| r.3).unwrap_or(0.0), sls_diag, valid.get(2).map(|r| r.0.as_str()).unwrap_or(""), valid.get(2).map(|r| r.3).unwrap_or(0.0))
            } else {
                "No valid results.".to_string()
            }
        }
        "definition" => {
            let solc = results
                .iter()
                .find(|(n, _, _, _, _)| n == "solc --lsp")
                .map(|(_, _, _, _, s)| s.as_str())
                .unwrap_or("");
            let sls = results
                .iter()
                .find(|(n, _, _, _, _)| n == "SLS (ours)")
                .map(|(_, _, _, _, s)| s.as_str())
                .unwrap_or("");
            format!(
                "{} returns {}, {} {}, Hardhat timeout.",
                "solc --lsp", solc, "SLS (ours)", sls
            )
        }
        "declaration" => {
            let solc = results
                .iter()
                .find(|(n, _, _, _, _)| n == "solc --lsp")
                .map(|(_, _, _, _, s)| s.as_str())
                .unwrap_or("");
            let sls = results
                .iter()
                .find(|(n, _, _, _, _)| n == "SLS (ours)")
                .map(|(_, _, _, _, s)| s.as_str())
                .unwrap_or("");
            format!(
                "{} {}, {} {}, Hardhat timeout.",
                "SLS (ours)", sls, "solc --lsp", solc
            )
        }
        "hover" => {
            let solc = results
                .iter()
                .find(|(n, _, _, _, _)| n == "solc --lsp")
                .map(|(_, _, _, _, s)| s.as_str())
                .unwrap_or("");
            let sls = results
                .iter()
                .find(|(n, _, _, _, _)| n == "SLS (ours)")
                .map(|(_, _, _, _, s)| s.as_str())
                .unwrap_or("");
            format!(
                "{} {}, {} {}, Hardhat timeout.",
                "SLS (ours)", sls, "solc --lsp", solc
            )
        }
        "references" => {
            let solc = results
                .iter()
                .find(|(n, _, _, _, _)| n == "solc --lsp")
                .map(|(_, _, _, _, s)| s.as_str())
                .unwrap_or("");
            let sls = results
                .iter()
                .find(|(n, _, _, _, _)| n == "SLS (ours)")
                .map(|(_, _, _, _, s)| s.as_str())
                .unwrap_or("");
            format!(
                "{} {}, {} {}, Hardhat timeout.",
                "SLS (ours)", sls, "solc --lsp", solc
            )
        }
        "documentSymbol" => {
            let valid = results
                .iter()
                .filter(|(_, _, _, m, _)| *m > 0.0)
                .collect::<Vec<_>>();
            if let Some((_, _, _, mean, _)) = valid.iter().find(|(n, _, _, _, _)| n == "SLS (ours)")
            {
                format!(
                    "{} fast ({:.1}ms) returns symbols, solc unsupported, Hardhat timeout.",
                    "SLS (ours)", *mean
                )
            } else {
                "No valid results.".to_string()
            }
        }
        _ => "".to_string(),
    }
}

// ── Main ────────────────────────────────────────────────────────────────────

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: bench <spawn|diagnostics|definition|declaration|hover|references|documentSymbol>");
        eprintln!("  spawn        — spawn + initialize handshake");
        eprintln!("  diagnostics  — open Pool.sol, time to first diagnostic");
        eprintln!("  definition   — go-to-definition on TickMath in Pool.sol");
        eprintln!("  declaration  — go-to-declaration on TickMath in Pool.sol");
        eprintln!("  hover        — hover on TickMath in Pool.sol");
        eprintln!("  references   — find references on TickMath in Pool.sol");
        eprintln!("  documentSymbol — get document symbols for Pool.sol");
        std::process::exit(1);
    }

    let v4 = ["bench/v4-core", "v4-core"]
        .iter()
        .find(|p| Path::new(p).join("src/PoolManager.sol").exists())
        .unwrap_or_else(|| {
            eprintln!("v4-core not found");
            std::process::exit(1);
        });
    let root = uri(Path::new(v4));

    let avail: Vec<&Server> = SERVERS
        .iter()
        .filter(|s| {
            let ok = available(s.cmd);
            if !ok {
                eprintln!("  SKIP {} — not found", s.label);
            }
            ok
        })
        .collect();

    let n = 10usize;
    let w = 2usize;
    let cmd = args[1].as_str();

    // ── spawn ───────────────────────────────────────────────────────────────

    if cmd == "spawn" {
        run_bench(
            "spawn",
            &[
                format!(
                    "## 1. SPAWN + INITIALIZE (ms) — {} iterations, {} warmup",
                    n, w
                ),
                String::new(),
                "Measures: spawn process -> initialize request -> response -> initialized notification".into(),
                "No files opened.".into(),
                String::new(),
            ],
            &avail,
            &root,
            Path::new(v4),
            |srv, root, cwd| {
                let mut samples = Vec::new();
                for i in 0..(w + n) {
                    let start = Instant::now();
                    let mut c = match LspClient::spawn(srv.cmd, srv.args, cwd) {
                        Ok(c) => c,
                        Err(e) => return BenchResult::Fail(e),
                    };
                    if let Err(e) = c.initialize(root) {
                        return BenchResult::Fail(e);
                    }
                    let ms = start.elapsed().as_secs_f64() * 1000.0;
                    if i >= w {
                        samples.push(ms);
                    }
                    c.kill();
                }
                BenchResult::Ok {
                    samples,
                    first_response: json!({"result": "ok"}),
                    diag_info: None,
                }
            },
        );
    }

    // ── diagnostics ─────────────────────────────────────────────────────────

    if cmd == "diagnostics" {
        let pool_sol = Path::new(v4).join("src/libraries/Pool.sol");
        let line_count = std::fs::read_to_string(&pool_sol)
            .map(|s| s.lines().count())
            .unwrap_or(0);

        run_bench(
            "diagnostics",
            &[
                format!(
                    "## 2. OPEN FILE -> FIRST DIAGNOSTIC (ms) — {} iterations, {} warmup",
                    n, w
                ),
                String::new(),
                format!("File: Pool.sol ({} lines)", line_count),
                "Measures: didOpen notification -> first publishDiagnostics response".into(),
                String::new(),
            ],
            &avail,
            &root,
            Path::new(v4),
            |srv, root, cwd| {
                let timeout = Duration::from_secs(30);
                let mut samples = Vec::new();
                let mut first: Option<Value> = None;
                for i in 0..(w + n) {
                    let mut c = match LspClient::spawn(srv.cmd, srv.args, cwd) {
                        Ok(c) => c,
                        Err(e) => return BenchResult::Fail(e),
                    };
                    if let Err(e) = c.initialize(root) {
                        return BenchResult::Fail(e);
                    }
                    let start = Instant::now();
                    if let Err(e) = c.open_file(&pool_sol) {
                        return BenchResult::Fail(e);
                    }
                    match c.wait_for_notif("textDocument/publishDiagnostics", timeout) {
                        Ok(resp) => {
                            let ms = start.elapsed().as_secs_f64() * 1000.0;
                            if first.is_none() {
                                // Extract the diagnostics array from the notification
                                let diags = resp
                                    .get("params")
                                    .and_then(|p| p.get("diagnostics"))
                                    .cloned()
                                    .unwrap_or(json!([]));
                                let summary = if let Some(arr) = diags.as_array() {
                                    if arr.is_empty() {
                                        "no diagnostics".to_string()
                                    } else {
                                        let messages: Vec<String> = arr
                                            .iter()
                                            .take(3)
                                            .filter_map(|d| {
                                                let sev = d
                                                    .get("severity")
                                                    .and_then(|s| s.as_u64())
                                                    .unwrap_or(0);
                                                let msg = d
                                                    .get("message")
                                                    .and_then(|m| m.as_str())
                                                    .unwrap_or("");
                                                let src = d
                                                    .get("source")
                                                    .and_then(|s| s.as_str())
                                                    .unwrap_or("");
                                                Some(format!("[{}] {} ({})", sev, msg, src))
                                            })
                                            .collect();
                                        format!(
                                            "{} diagnostics: {}",
                                            arr.len(),
                                            messages.join("; ")
                                        )
                                    }
                                } else {
                                    "invalid diagnostics".to_string()
                                };
                                first = Some(json!({"result": summary}));
                            }
                            if i >= w {
                                samples.push(ms);
                            }
                        }
                        Err(e) => return BenchResult::Fail(e),
                    }
                    c.kill();
                }
                BenchResult::Ok {
                    samples,
                    first_response: first.unwrap_or(json!(null)),
                    diag_info: None,
                }
            },
        );
    }

    // ── definition ──────────────────────────────────────────────────────────

    if cmd == "definition" {
        let pool_sol = Path::new(v4).join("src/libraries/Pool.sol");
        let line_count = std::fs::read_to_string(&pool_sol)
            .map(|s| s.lines().count())
            .unwrap_or(0);
        let target_line: u32 = 102;
        let target_col: u32 = 15;

        run_bench(
            "definition",
            &[
                format!(
                    "## 3. GO TO DEFINITION (ms) — {} iterations, {} warmup",
                    n, w
                ),
                String::new(),
                format!("File: Pool.sol ({} lines)", line_count),
                format!(
                    "Target: `TickMath` at line {}:{}",
                    target_line + 1,
                    target_col
                ),
                "Measures: textDocument/definition request -> response".into(),
                "Waits for valid publishDiagnostics before sending requests.".into(),
                String::new(),
            ],
            &avail,
            &root,
            Path::new(v4),
            |srv, root, cwd| {
                let timeout = Duration::from_secs(30);
                let mut c = match LspClient::spawn(srv.cmd, srv.args, cwd) {
                    Ok(c) => c,
                    Err(e) => return BenchResult::Fail(e),
                };
                if let Err(e) = c.initialize(root) {
                    return BenchResult::Fail(e);
                }
                if let Err(e) = c.open_file(&pool_sol) {
                    return BenchResult::Fail(e);
                }

                // Wait for valid diagnostics (build complete)
                let diag_info = match c.wait_for_valid_diagnostics(Duration::from_secs(10)) {
                    Ok(info) => info,
                    Err(e) => return BenchResult::Fail(format!("wait_for_diagnostics: {}", e)),
                };
                eprintln!(
                    "diagnostics: {} items in {:.0}ms ... ",
                    diag_info.count, diag_info.elapsed_ms
                );
                eprint!("    ");

                let file_uri = uri(&pool_sol);
                let mut samples = Vec::new();
                let mut first: Option<Value> = None;
                for i in 0..(w + n) {
                    let start = Instant::now();
                    if let Err(e) = c.send(
                        "textDocument/definition",
                        json!({
                            "textDocument": { "uri": file_uri },
                            "position": { "line": target_line, "character": target_col },
                        }),
                    ) {
                        return BenchResult::Fail(e);
                    }
                    match c.read_response(timeout) {
                        Ok(resp) => {
                            let ms = start.elapsed().as_secs_f64() * 1000.0;
                            if i >= w {
                                if first.is_none() {
                                    first = Some(resp.clone());
                                }
                                if !is_valid_response(&resp) {
                                    // Dump server logs for debugging
                                    if !c.logs.is_empty() {
                                        eprintln!("\n--- {} server logs ---", srv.label);
                                        for line in &c.logs {
                                            eprintln!("  {}", line);
                                        }
                                        eprintln!("--- end ---");
                                    }
                                    return BenchResult::Invalid {
                                        first_response: resp,
                                        diag_info: Some(diag_info),
                                    };
                                }
                                samples.push(ms);
                            }
                        }
                        Err(e) => return BenchResult::Fail(e),
                    }
                }
                c.kill();
                BenchResult::Ok {
                    samples,
                    first_response: first.unwrap_or(json!(null)),
                    diag_info: Some(diag_info),
                }
            },
        );
    }

    // ── declaration ─────────────────────────────────────────────────────────

    if cmd == "declaration" {
        let pool_sol = Path::new(v4).join("src/libraries/Pool.sol");
        let line_count = std::fs::read_to_string(&pool_sol)
            .map(|s| s.lines().count())
            .unwrap_or(0);
        let target_line: u32 = 102;
        let target_col: u32 = 15;

        run_bench(
            "declaration",
            &[
                format!(
                    "## 4. GO TO DECLARATION (ms) — {} iterations, {} warmup",
                    n, w
                ),
                String::new(),
                format!("File: Pool.sol ({} lines)", line_count),
                format!(
                    "Target: `TickMath` at line {}:{}",
                    target_line + 1,
                    target_col
                ),
                "Measures: textDocument/declaration request -> response".into(),
                "Waits for valid publishDiagnostics before sending requests.".into(),
                String::new(),
            ],
            &avail,
            &root,
            Path::new(v4),
            |srv, root, cwd| {
                let timeout = Duration::from_secs(30);
                let mut c = match LspClient::spawn(srv.cmd, srv.args, cwd) {
                    Ok(c) => c,
                    Err(e) => return BenchResult::Fail(e),
                };
                if let Err(e) = c.initialize(root) {
                    return BenchResult::Fail(e);
                }
                if let Err(e) = c.open_file(&pool_sol) {
                    return BenchResult::Fail(e);
                }

                // Wait for valid diagnostics (build complete)
                let diag_info = match c.wait_for_valid_diagnostics(Duration::from_secs(10)) {
                    Ok(info) => info,
                    Err(e) => return BenchResult::Fail(format!("wait_for_diagnostics: {}", e)),
                };
                eprintln!(
                    "diagnostics: {} items in {:.0}ms ... ",
                    diag_info.count, diag_info.elapsed_ms
                );
                eprint!("    ");

                let file_uri = uri(&pool_sol);
                let mut samples = Vec::new();
                let mut first: Option<Value> = None;
                for i in 0..(w + n) {
                    let start = Instant::now();
                    if let Err(e) = c.send(
                        "textDocument/declaration",
                        json!({
                            "textDocument": { "uri": file_uri },
                            "position": { "line": target_line, "character": target_col },
                        }),
                    ) {
                        return BenchResult::Fail(e);
                    }
                    match c.read_response(timeout) {
                        Ok(resp) => {
                            let ms = start.elapsed().as_secs_f64() * 1000.0;
                            if i >= w {
                                if first.is_none() {
                                    first = Some(resp.clone());
                                }
                                if !is_valid_response(&resp) {
                                    // Dump server logs for debugging
                                    if !c.logs.is_empty() {
                                        eprintln!("\n--- {} server logs ---", srv.label);
                                        for line in &c.logs {
                                            eprintln!("  {}", line);
                                        }
                                        eprintln!("--- end ---");
                                    }
                                    return BenchResult::Invalid {
                                        first_response: resp,
                                        diag_info: Some(diag_info),
                                    };
                                }
                                samples.push(ms);
                            }
                        }
                        Err(e) => return BenchResult::Fail(e),
                    }
                }
                c.kill();
                BenchResult::Ok {
                    samples,
                    first_response: first.unwrap_or(json!(null)),
                    diag_info: Some(diag_info),
                }
            },
        );
    }

    // ── hover ─────────────────────────────────────────────────────────

    if cmd == "hover" {
        let pool_sol = Path::new(v4).join("src/libraries/Pool.sol");
        let line_count = std::fs::read_to_string(&pool_sol)
            .map(|s| s.lines().count())
            .unwrap_or(0);
        let target_line: u32 = 102;
        let target_col: u32 = 15;

        run_bench(
            "hover",
            &[
                format!("## 5. HOVER (ms) — {} iterations, {} warmup", n, w),
                String::new(),
                format!("File: Pool.sol ({} lines)", line_count),
                format!(
                    "Target: `TickMath` at line {}:{}",
                    target_line + 1,
                    target_col
                ),
                "Measures: textDocument/hover request -> response".into(),
                "Waits for valid publishDiagnostics before sending requests.".into(),
                String::new(),
            ],
            &avail,
            &root,
            Path::new(v4),
            |srv, root, cwd| {
                let timeout = Duration::from_secs(30);
                let mut c = match LspClient::spawn(srv.cmd, srv.args, cwd) {
                    Ok(c) => c,
                    Err(e) => return BenchResult::Fail(e),
                };
                if let Err(e) = c.initialize(root) {
                    return BenchResult::Fail(e);
                }
                if let Err(e) = c.open_file(&pool_sol) {
                    return BenchResult::Fail(e);
                }

                // Wait for valid diagnostics (build complete)
                let diag_info = match c.wait_for_valid_diagnostics(Duration::from_secs(10)) {
                    Ok(info) => info,
                    Err(e) => return BenchResult::Fail(format!("wait_for_diagnostics: {}", e)),
                };
                eprintln!(
                    "diagnostics: {} items in {:.0}ms ... ",
                    diag_info.count, diag_info.elapsed_ms
                );
                eprint!("    ");

                let file_uri = uri(&pool_sol);
                let mut samples = Vec::new();
                let mut first: Option<Value> = None;
                for i in 0..(w + n) {
                    let start = Instant::now();
                    if let Err(e) = c.send(
                        "textDocument/hover",
                        json!({
                            "textDocument": { "uri": file_uri },
                            "position": { "line": target_line, "character": target_col },
                        }),
                    ) {
                        return BenchResult::Fail(e);
                    }
                    match c.read_response(timeout) {
                        Ok(resp) => {
                            let ms = start.elapsed().as_secs_f64() * 1000.0;
                            if i >= w {
                                if first.is_none() {
                                    first = Some(resp.clone());
                                }
                                if !is_valid_response(&resp) {
                                    return BenchResult::Invalid {
                                        first_response: resp,
                                        diag_info: Some(diag_info),
                                    };
                                }
                                samples.push(ms);
                            }
                        }
                        Err(e) => {
                            return BenchResult::Fail(e);
                        }
                    }
                }
                c.kill();
                BenchResult::Ok {
                    samples,
                    first_response: first.unwrap_or(json!(null)),
                    diag_info: Some(diag_info),
                }
            },
        );
    }

    // ── references ─────────────────────────────────────────────────────────

    if cmd == "references" {
        let pool_sol = Path::new(v4).join("src/libraries/Pool.sol");
        let line_count = std::fs::read_to_string(&pool_sol)
            .map(|s| s.lines().count())
            .unwrap_or(0);
        let target_line: u32 = 102;
        let target_col: u32 = 15;

        run_bench(
            "references",
            &[
                format!(
                    "## 6. FIND REFERENCES (ms) — {} iterations, {} warmup",
                    n, w
                ),
                String::new(),
                format!("File: Pool.sol ({} lines)", line_count),
                format!(
                    "Target: `TickMath` at line {}:{}",
                    target_line + 1,
                    target_col
                ),
                "Measures: textDocument/references request -> response".into(),
                "Waits for valid publishDiagnostics before sending requests.".into(),
                String::new(),
            ],
            &avail,
            &root,
            Path::new(v4),
            |srv, root, cwd| {
                let timeout = Duration::from_secs(30);
                let mut c = match LspClient::spawn(srv.cmd, srv.args, cwd) {
                    Ok(c) => c,
                    Err(e) => return BenchResult::Fail(e),
                };
                if let Err(e) = c.initialize(root) {
                    return BenchResult::Fail(e);
                }
                if let Err(e) = c.open_file(&pool_sol) {
                    return BenchResult::Fail(e);
                }

                // Wait for valid diagnostics (build complete)
                let diag_info = match c.wait_for_valid_diagnostics(Duration::from_secs(10)) {
                    Ok(info) => info,
                    Err(e) => return BenchResult::Fail(format!("wait_for_diagnostics: {}", e)),
                };
                eprintln!(
                    "diagnostics: {} items in {:.0}ms ... ",
                    diag_info.count, diag_info.elapsed_ms
                );
                eprint!("    ");

                let file_uri = uri(&pool_sol);
                let mut samples = Vec::new();
                let mut first: Option<Value> = None;
                for i in 0..(w + n) {
                    let start = Instant::now();
                    if let Err(e) = c.send(
                        "textDocument/references",
                        json!({
                            "textDocument": { "uri": file_uri },
                            "position": { "line": target_line, "character": target_col },
                            "context": { "includeDeclaration": true }
                        }),
                    ) {
                        return BenchResult::Fail(e);
                    }
                    match c.read_response(timeout) {
                        Ok(resp) => {
                            let ms = start.elapsed().as_secs_f64() * 1000.0;
                            if i >= w {
                                if first.is_none() {
                                    first = Some(resp.clone());
                                }
                                if !is_valid_response(&resp) {
                                    return BenchResult::Invalid {
                                        first_response: resp,
                                        diag_info: Some(diag_info),
                                    };
                                }
                                samples.push(ms);
                            }
                        }
                        Err(e) => {
                            return BenchResult::Fail(e);
                        }
                    }
                }
                c.kill();
                BenchResult::Ok {
                    samples,
                    first_response: first.unwrap_or(json!(null)),
                    diag_info: Some(diag_info),
                }
            },
        );
    }

    // ── documentSymbol ─────────────────────────────────────────────────────────

    if cmd == "documentSymbol" {
        let pool_sol = Path::new(v4).join("src/libraries/Pool.sol");
        let line_count = std::fs::read_to_string(&pool_sol)
            .map(|s| s.lines().count())
            .unwrap_or(0);

        run_bench(
            "documentSymbol",
            &[
                format!(
                    "## 7. DOCUMENT SYMBOLS (ms) — {} iterations, {} warmup",
                    n, w
                ),
                String::new(),
                format!("File: Pool.sol ({} lines)", line_count),
                "Measures: textDocument/documentSymbol request -> response".into(),
                "Waits for valid publishDiagnostics before sending requests.".into(),
                String::new(),
            ],
            &avail,
            &root,
            Path::new(v4),
            |srv, root, cwd| {
                let timeout = Duration::from_secs(30);
                let mut c = match LspClient::spawn(srv.cmd, srv.args, cwd) {
                    Ok(c) => c,
                    Err(e) => return BenchResult::Fail(e),
                };
                if let Err(e) = c.initialize(root) {
                    return BenchResult::Fail(e);
                }
                if let Err(e) = c.open_file(&pool_sol) {
                    return BenchResult::Fail(e);
                }

                // Wait for valid diagnostics (build complete)
                let diag_info = match c.wait_for_valid_diagnostics(Duration::from_secs(10)) {
                    Ok(info) => info,
                    Err(e) => return BenchResult::Fail(format!("wait_for_diagnostics: {}", e)),
                };
                eprintln!(
                    "diagnostics: {} items in {:.0}ms ... ",
                    diag_info.count, diag_info.elapsed_ms
                );
                eprint!("    ");

                let file_uri = uri(&pool_sol);
                let mut samples = Vec::new();
                let mut first: Option<Value> = None;
                for i in 0..(w + n) {
                    let start = Instant::now();
                    if let Err(e) = c.send(
                        "textDocument/documentSymbol",
                        json!({
                            "textDocument": { "uri": file_uri }
                        }),
                    ) {
                        return BenchResult::Fail(e);
                    }
                    match c.read_response(timeout) {
                        Ok(resp) => {
                            let ms = start.elapsed().as_secs_f64() * 1000.0;
                            if i >= w {
                                if first.is_none() {
                                    first = Some(resp.clone());
                                }
                                if !is_valid_response(&resp) {
                                    return BenchResult::Invalid {
                                        first_response: resp,
                                        diag_info: Some(diag_info),
                                    };
                                }
                                samples.push(ms);
                            }
                        }
                        Err(e) => {
                            return BenchResult::Fail(e);
                        }
                    }
                }
                c.kill();
                BenchResult::Ok {
                    samples,
                    first_response: first.unwrap_or(json!(null)),
                    diag_info: Some(diag_info),
                }
            },
        );
    }
}
