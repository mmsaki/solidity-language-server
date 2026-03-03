#!/usr/bin/env python3
"""
extract_solc_error_codes.py
===========================
Extracts all solc error codes from the solidity compiler source tree and writes
a structured JSON database to data/error_codes.json.

Usage
-----
Run from the root of the solidity-language-server repo:

    python3 docs/scripts/extract_solc_error_codes.py \
        --solidity-repo /path/to/argotorg/solidity \
        --output data/error_codes.json

Dependencies
------------
- ripgrep (rg) must be on PATH
- Python 3.8+

How it works
------------
1.  Calls rg with -C 2 (2 lines of context) so that multiline error reporter
    calls are captured as a single window.
2.  Parses each context block to extract:
      code     – 4-digit error code (integer)
      severity – error | warning | info (inferred from the reporter call name)
      message  – first human-readable quoted string in the window
      source   – lib/file.cpp (last two path components)
      line     – line number of the match
3.  Writes one JSON object per unique code, sorted by code, with an `action`
    field (null by default) reserved for codeAction fix descriptors.

Regenerating
------------
Re-run this script whenever the solidity compiler repo is updated.  The output
file is checked into the language server repo so the server binary does not
need the compiler sources at runtime (it embeds the JSON with include_str!).
"""

import argparse
import json
import re
import subprocess
import sys
from collections import Counter
from pathlib import Path


# ---------------------------------------------------------------------------
# Patterns
# ---------------------------------------------------------------------------

CODE_PAT = re.compile(r"(\d{4})_error")

TYPE_PAT = re.compile(
    r"(fatalParserError|parserError|parserWarning|parserInfo"
    r"|typeError|typeWarning|typeInfo"
    r"|declarationError|declarationWarning|declarationInfo"
    r"|syntaxError|syntaxWarning"
    r"|unimplementedFeatureError|docstringParsingError"
    r"|m_errorReporter\.warning"
    r"|m_errorReporter\.info"
    r"|m_errorReporter\.error"
    r"|m_errorReporter\.\w+Warning"
    r"|m_errorReporter\.\w+Info"
    r"|m_errorReporter\.\w+Error)"
)

MSG_PAT = re.compile(r'"([^"]{4,})"')

SEV_MAP = {
    "fatalParserError": "error",
    "parserError": "error",
    "parserWarning": "warning",
    "parserInfo": "info",
    "typeError": "error",
    "typeWarning": "warning",
    "typeInfo": "info",
    "declarationError": "error",
    "declarationWarning": "warning",
    "declarationInfo": "info",
    "syntaxError": "error",
    "syntaxWarning": "warning",
    "unimplementedFeatureError": "error",
    "docstringParsingError": "error",
}


def severity_from_call(call: str) -> str:
    for k, v in SEV_MAP.items():
        if k in call:
            return v
    # m_errorReporter.warning / m_errorReporter.info / m_errorReporter.error
    lower = call.lower()
    if "warning" in lower:
        return "warning"
    if "info" in lower:
        return "info"
    return "error"


def shorten_path(p: str) -> str:
    parts = p.split("/")
    return "/".join(parts[-2:]) if len(parts) >= 2 else p


# ---------------------------------------------------------------------------
# Parsing
# ---------------------------------------------------------------------------


def parse_context_output(text: str) -> list[dict]:
    """
    Parse rg -C 2 output into a list of unique error code entries.

    rg context output format:
        file:lineno:content   <- match line
        file-lineno-content   <- context line
        --                    <- block separator
    """
    seen: dict[str, dict] = {}
    current_file = None
    current_lines: list[tuple[int, str]] = []

    def flush(filepath, lines):
        window = " ".join(c for _, c in lines)
        m_code = CODE_PAT.search(window)
        if not m_code:
            return
        code = m_code.group(1)
        if code in seen:
            return

        m_type = TYPE_PAT.search(window)
        severity = severity_from_call(m_type.group(1)) if m_type else "error"

        messages = [
            s
            for s in MSG_PAT.findall(window)
            if "/" not in s and "\\" not in s and len(s) >= 5 and not s.startswith("0x")
        ]
        message = messages[0] if messages else ""

        match_ln = next(
            (ln for ln, c in lines if CODE_PAT.search(c)),
            lines[0][0] if lines else 0,
        )

        seen[code] = {
            "code": int(code),
            "severity": severity,
            "message": message,
            "source": shorten_path(filepath or ""),
            "line": match_ln,
            "action": None,
        }

    match_re = re.compile(r"^([^:]+):(\d+):(.*)")
    context_re = re.compile(r"^([^-\n][^:]*)-(\d+)-(.*)")

    for raw in text.splitlines():
        if raw == "--":
            if current_lines:
                flush(current_file, current_lines)
            current_file = None
            current_lines = []
            continue

        m = match_re.match(raw)
        if m:
            fp, ln, content = m.group(1), int(m.group(2)), m.group(3)
            if current_file is None:
                current_file = fp
            current_lines.append((ln, content))
            continue

        m = context_re.match(raw)
        if m:
            fp, ln, content = m.group(1), int(m.group(2)), m.group(3)
            if current_file is None:
                current_file = fp
            current_lines.append((ln, content))

    if current_lines:
        flush(current_file, current_lines)

    return sorted(seen.values(), key=lambda x: x["code"])


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

SEARCH_DIRS = [
    "libsolidity",
    "liblangutil",
    "libyul",
    "libevmasm",
    "libsolutil",
]


def run_rg(solidity_repo: Path) -> str:
    cmd = [
        "rg",
        "--no-heading",
        "-n",
        r"\b\d{4}_error\b",
        "-C",
        "2",
        "-S",
        "-g",
        "*.cpp",
        "-g",
        "*.h",
    ] + [d for d in SEARCH_DIRS if (solidity_repo / d).exists()]

    result = subprocess.run(
        cmd,
        cwd=solidity_repo,
        capture_output=True,
        text=True,
    )
    if result.returncode not in (0, 1):  # 1 = no matches (unlikely but ok)
        print(f"rg failed: {result.stderr}", file=sys.stderr)
        sys.exit(1)
    return result.stdout


def main():
    parser = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    parser.add_argument(
        "--solidity-repo",
        default="../argotorg/solidity",
        help="Path to the solidity compiler source tree (default: ../argotorg/solidity)",
    )
    parser.add_argument(
        "--output",
        default="data/error_codes.json",
        help="Output JSON file path (default: data/error_codes.json)",
    )
    args = parser.parse_args()

    solidity_repo = Path(args.solidity_repo).expanduser().resolve()
    if not solidity_repo.is_dir():
        print(f"error: solidity repo not found at {solidity_repo}", file=sys.stderr)
        sys.exit(1)

    output_path = Path(args.output)
    output_path.parent.mkdir(parents=True, exist_ok=True)

    print(f"Searching {solidity_repo} ...")
    raw = run_rg(solidity_repo)

    print("Parsing context blocks ...")
    entries = parse_context_output(raw)

    counts = Counter(e["severity"] for e in entries)
    print(f"  Total unique codes : {len(entries)}")
    for sev, n in sorted(counts.items()):
        print(f"    {sev:8s}: {n}")

    with open(output_path, "w") as f:
        json.dump(entries, f, indent=2)
        f.write("\n")

    print(f"Written {output_path}")


if __name__ == "__main__":
    main()
