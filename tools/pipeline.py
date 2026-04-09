#!/usr/bin/env python3
"""Pipeline gate: spec_file → codex exec → extract Rust → derive linter → cargo check.

Usage:
    pipeline.py <spec_file> <module_name>

Exit codes:
    0 — generated, linted, cargo-tested clean
    1 — codex extraction failed
    2 — derive linter found missing derives
    3 — cargo test failed
    4 — codex subprocess failed entirely

Outputs the generated body to /tmp/pipeline-bodies/<module_name>.rs on success.
"""
import json
import pathlib
import re
import subprocess
import sys
import time

import importlib.util

# Load derive_linter as a module
_spec = importlib.util.spec_from_file_location("derive_linter", "/root/tools/derive_linter.py")
_dl = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(_dl)


def run_codex(spec_text: str, workdir: str = "/tmp/pipeline-work") -> tuple[str, float]:
    pathlib.Path(workdir).mkdir(parents=True, exist_ok=True)
    t0 = time.time()
    import os
    env = os.environ.copy()
    # Ensure PATH includes the npm + local bin directories where codex lives
    env["PATH"] = (
        "/root/.local/bin:/root/.npm-global/bin:" + env.get("PATH", "/usr/bin:/bin")
    )
    result = subprocess.run(
        [
            "timeout",
            "300",
            "codex",
            "exec",
            "--dangerously-bypass-approvals-and-sandbox",
            "--skip-git-repo-check",
            "-C",
            workdir,
            spec_text,
        ],
        capture_output=True,
        text=True,
        stdin=subprocess.DEVNULL,
        env=env,
    )
    return result.stdout + result.stderr, time.time() - t0


def extract_rust(codex_output: str) -> str | None:
    idx = codex_output.rfind("\ncodex\n")
    after = codex_output[idx + len("\ncodex\n") :] if idx != -1 else codex_output
    m = re.search(r"(#\[.*|use .*|pub .*)", after, re.DOTALL)
    if not m:
        return None
    body = re.split(r"\ntokens used", m.group(1))[0]
    lines = body.rstrip().split("\n")
    last_brace = max(
        (i for i, line in enumerate(lines) if line.strip() == "}"), default=-1
    )
    if last_brace == -1:
        return None
    return "\n".join(lines[: last_brace + 1]) + "\n"


def cargo_check(body_text: str, name: str) -> tuple[bool, str]:
    crate = pathlib.Path(f"/tmp/cargo-scratch/{name}")
    crate.mkdir(parents=True, exist_ok=True)
    (crate / "Cargo.toml").write_text(
        f'[package]\nname = "{name}"\nversion = "0.0.0"\nedition = "2021"\n[lib]\npath = "src/lib.rs"\n'
    )
    src = crate / "src"
    src.mkdir(exist_ok=True)
    (src / "lib.rs").write_text(body_text)
    result = subprocess.run(
        ["cargo", "test", "--manifest-path", str(crate / "Cargo.toml")],
        capture_output=True,
        text=True,
    )
    return result.returncode == 0, result.stdout + result.stderr


def pipeline(spec_path: str, module_name: str) -> dict:
    spec = pathlib.Path(spec_path).read_text()
    record = {"spec": spec_path, "module": module_name, "stages": {}}

    out, codex_time = run_codex(spec)
    record["stages"]["codex"] = {"time_s": round(codex_time, 1), "stdout_len": len(out)}
    body = extract_rust(out)
    if body is None:
        record["status"] = "extract_fail"
        record["exit"] = 1
        return record
    record["stages"]["extract"] = {"body_len": len(body)}

    lint_result = _dl.lint_spec(body)
    record["stages"]["lint"] = {
        "declared": lint_result.get("declared", []),
        "required": lint_result.get("required", []),
        "missing": lint_result.get("missing", []),
        "evidence_count": len(lint_result.get("evidence", [])),
    }
    if lint_result.get("missing"):
        record["status"] = "lint_fail"
        record["exit"] = 2
        return record

    cargo_ok, cargo_out = cargo_check(body, module_name)
    record["stages"]["cargo"] = {"passed": cargo_ok}
    if not cargo_ok:
        record["status"] = "cargo_fail"
        record["exit"] = 3
        record["stages"]["cargo"]["last_err"] = cargo_out[-400:]
        return record

    out_dir = pathlib.Path("/tmp/pipeline-bodies")
    out_dir.mkdir(exist_ok=True)
    out_path = out_dir / f"{module_name}.rs"
    out_path.write_text(body)
    record["stages"]["store_local"] = {"path": str(out_path)}
    record["status"] = "ok"
    record["exit"] = 0
    return record


if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("usage: pipeline.py <spec_file> <module_name>", file=sys.stderr)
        sys.exit(2)
    result = pipeline(sys.argv[1], sys.argv[2])
    print(json.dumps(result, indent=2))
    sys.exit(result.get("exit", 4))
