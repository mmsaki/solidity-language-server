use serde_json::{Value, json};
use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::time::{Duration, Instant};
use tempfile::TempDir;

struct LspProc {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    next_id: u64,
}

impl LspProc {
    fn spawn(cwd: &Path) -> Self {
        let bin = option_env!("CARGO_BIN_EXE_solidity-language-server")
            .or(option_env!("CARGO_BIN_EXE_solidity_language_server"))
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("target")
                    .join("debug")
                    .join("solidity-language-server")
            });

        let mut child = Command::new(bin)
            .arg("--stdio")
            .current_dir(cwd)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("spawn solidity-language-server");

        let stdin = child.stdin.take().expect("stdin");
        let stdout = BufReader::new(child.stdout.take().expect("stdout"));
        Self {
            child,
            stdin,
            stdout,
            next_id: 1,
        }
    }

    fn send_notification(&mut self, method: &str, params: Value) {
        let msg = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
        });
        self.write_msg(&msg);
    }

    fn send_request(&mut self, method: &str, params: Value) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let msg = json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params,
        });
        self.write_msg(&msg);
        id
    }

    fn write_msg(&mut self, msg: &Value) {
        let body = serde_json::to_vec(msg).expect("serialize");
        let header = format!("Content-Length: {}\r\n\r\n", body.len());
        self.stdin.write_all(header.as_bytes()).expect("write header");
        self.stdin.write_all(&body).expect("write body");
        self.stdin.flush().expect("flush");
    }

    fn read_msg(&mut self) -> Value {
        let mut content_length: usize = 0;
        let mut line = String::new();
        loop {
            line.clear();
            let n = self.stdout.read_line(&mut line).expect("read header line");
            assert!(n > 0, "LSP stream closed while reading headers");
            if line == "\r\n" {
                break;
            }
            if let Some(v) = line.strip_prefix("Content-Length:") {
                content_length = v.trim().parse::<usize>().expect("parse content length");
            }
        }

        assert!(content_length > 0, "missing Content-Length");
        let mut body = vec![0u8; content_length];
        self.stdout.read_exact(&mut body).expect("read body");
        serde_json::from_slice::<Value>(&body).expect("parse jsonrpc message")
    }

    fn wait_response(&mut self, id: u64, timeout: Duration) -> Value {
        let deadline = Instant::now() + timeout;
        loop {
            assert!(Instant::now() < deadline, "timed out waiting response id={id}");
            let msg = self.read_msg();
            if msg.get("id").and_then(|v| v.as_u64()) == Some(id) {
                return msg;
            }
        }
    }

    fn shutdown(mut self) {
        let shutdown_id = self.send_request("shutdown", Value::Null);
        let _ = self.wait_response(shutdown_id, Duration::from_secs(5));
        self.send_notification("exit", Value::Null);
        let _ = self.child.wait();
    }
}

fn write_foundry_project(dir: &Path) -> (String, String, String) {
    let src = dir.join("src");
    fs::create_dir_all(&src).expect("create src");
    fs::write(
        dir.join("foundry.toml"),
        r#"[profile.default]
src = "src"
"#,
    )
    .expect("write foundry.toml");

    let a = src.join("A.sol");
    let b = src.join("B.sol");
    fs::write(
        &a,
        "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\ncontract A {}\n",
    )
    .expect("write A.sol");
    fs::write(
        &b,
        "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\nimport {A} from \"./A.sol\";\ncontract B is A {}\n",
    )
    .expect("write B.sol");

    let root_uri = format!("file://{}", dir.display());
    let a_uri = format!("file://{}", a.display());
    let b_uri = format!("file://{}", b.display());
    (root_uri, a_uri, b_uri)
}

fn initialize_server(lsp: &mut LspProc, root_uri: &str) {
    let init_id = lsp.send_request(
        "initialize",
        json!({
            "processId": null,
            "rootUri": root_uri,
            "capabilities": {
                "workspace": {
                    "fileOperations": {
                        "willDelete": true
                    }
                }
            }
        }),
    );
    let init_resp = lsp.wait_response(init_id, Duration::from_secs(10));
    assert!(init_resp.get("result").is_some(), "initialize failed: {init_resp}");
    lsp.send_notification("initialized", json!({}));
}

#[test]
#[ignore = "E2E stdio test; run manually when validating LSP file-op settings"]
fn will_delete_files_returns_edits_when_enabled() {
    let tmp = TempDir::new().expect("tmp dir");
    let (root_uri, a_uri, b_uri) = write_foundry_project(tmp.path());
    let mut lsp = LspProc::spawn(tmp.path());
    initialize_server(&mut lsp, &root_uri);

    let req_id = lsp.send_request(
        "workspace/willDeleteFiles",
        json!({
            "files": [{ "uri": a_uri }]
        }),
    );
    let resp = lsp.wait_response(req_id, Duration::from_secs(15));
    let result = resp.get("result").expect("missing result");
    let changes = result
        .get("changes")
        .and_then(Value::as_object)
        .expect("expected workspace edit changes");
    assert!(
        changes.contains_key(&b_uri),
        "expected B.sol import edit in response: {resp}"
    );

    lsp.shutdown();
}

#[test]
#[ignore = "E2E stdio test; run manually when validating LSP file-op settings"]
fn will_delete_files_returns_null_when_update_imports_on_delete_disabled() {
    let tmp = TempDir::new().expect("tmp dir");
    let (root_uri, a_uri, _b_uri) = write_foundry_project(tmp.path());
    let mut lsp = LspProc::spawn(tmp.path());
    initialize_server(&mut lsp, &root_uri);

    lsp.send_notification(
        "workspace/didChangeConfiguration",
        json!({
            "settings": {
                "solidity-language-server": {
                    "fileOperations": {
                        "updateImportsOnDelete": false
                    }
                }
            }
        }),
    );

    let req_id = lsp.send_request(
        "workspace/willDeleteFiles",
        json!({
            "files": [{ "uri": a_uri }]
        }),
    );
    let resp = lsp.wait_response(req_id, Duration::from_secs(15));
    assert!(
        resp.get("result").is_some_and(Value::is_null),
        "expected null result when disabled: {resp}"
    );

    lsp.shutdown();
}
