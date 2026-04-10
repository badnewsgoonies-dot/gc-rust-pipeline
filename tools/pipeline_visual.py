#!/usr/bin/env python3
"""Visual pipeline: codex generate HTML -> screenshot -> gemini judge -> iterate.

Usage:
    pipeline_visual.py <spec_file> <module_name> [--max-iter N] [--threshold T]

Exit codes:
    0 — judged playable (score >= threshold, default 0.6)
    1 — max iterations exceeded, no passing version found
    2 — codex failed completely on all iterations
    3 — screenshot subprocess failed
"""
import json
import os
import pathlib
import re
import subprocess
import sys
import time

# Import the vision judge
sys.path.insert(0, "/root/tools")
from judge_raycaster import judge, score_from_verdict


def run_codex(spec_text: str, workdir: str = "/tmp/visual-work") -> tuple[str, float]:
    pathlib.Path(workdir).mkdir(parents=True, exist_ok=True)
    t0 = time.time()
    env = os.environ.copy()
    env["PATH"] = "/root/.local/bin:/root/.npm-global/bin:" + env.get("PATH", "/usr/bin:/bin")
    result = subprocess.run(
        [
            "timeout", "300",
            "codex", "exec",
            "--dangerously-bypass-approvals-and-sandbox",
            "--skip-git-repo-check",
            "-C", workdir,
            spec_text,
        ],
        capture_output=True,
        text=True,
        stdin=subprocess.DEVNULL,
        env=env,
    )
    return result.stdout + result.stderr, time.time() - t0


def extract_html(codex_output: str) -> str | None:
    # Codex sometimes emits multiple HTML blocks in one response (initial attempt
    # followed by prompt echo and a second attempt). Take the LAST complete
    # <!DOCTYPE...</html> block — that's the model's final answer.
    last_doctype = codex_output.rfind("<!DOCTYPE")
    if last_doctype == -1:
        last_doctype = codex_output.rfind("<html")
    if last_doctype == -1:
        return None
    # First </html> AFTER the last DOCTYPE
    end = codex_output.find("</html>", last_doctype)
    if end == -1:
        return None
    return codex_output[last_doctype : end + len("</html>")]


def take_screenshots(html_path: str, out_dir: str) -> tuple[bool, list[str], list]:
    pathlib.Path(out_dir).mkdir(parents=True, exist_ok=True)
    env = os.environ.copy()
    env["PLAYWRIGHT_BROWSERS_PATH"] = "/opt/pw-browsers"
    result = subprocess.run(
        ["node", "/tmp/playwright-test/screenshot.js", html_path, out_dir, "click"],
        capture_output=True,
        text=True,
        env=env,
        timeout=60,
    )
    if result.returncode != 0:
        return False, [], [{"kind": "subprocess", "msg": result.stderr[-400:]}]
    try:
        report = json.loads(result.stdout)
        shots = [os.path.join(out_dir, s) for s in report.get("shots", [])]
        return True, shots, report.get("errors", [])
    except Exception as e:
        return False, [], [{"kind": "json_parse", "msg": str(e)}]


def judge_best(shots: list[str]) -> dict:
    """Judge all shots and return the best verdict."""
    results = []
    for shot in shots:
        try:
            verdict = judge(shot)
            score = score_from_verdict(verdict)
            results.append({"image": os.path.basename(shot), "verdict": verdict, "score": score})
        except Exception as e:
            results.append({"image": os.path.basename(shot), "error": str(e), "score": 0.0})
    if not results:
        return {"best_score": 0.0, "all": results}
    best = max(results, key=lambda r: r.get("score", 0))
    return {"best_score": best["score"], "best_image": best.get("image"), "best_verdict": best.get("verdict"), "all": results}


def build_failure_feedback(best: dict, js_errors: list) -> str:
    """Turn a failed verdict into actionable prose for the next codex call."""
    lines = ["The previous attempt did not produce a playable raycaster. Failure details:"]
    if js_errors:
        lines.append("JavaScript runtime errors:")
        for e in js_errors[:5]:
            lines.append(f"  - [{e.get('kind', 'error')}] {e.get('msg', '')[:200]}")
    v = best.get("best_verdict") or {}
    if v:
        lines.append(f"Visual judge verdict on best frame ({best.get('best_image', '?')}):")
        lines.append(f"  description: {v.get('description', '')}")
        flags = {k: v.get(k) for k in ["renders", "has_perspective", "has_walls", "has_hud", "has_floor_and_ceiling", "is_playable_fps"]}
        lines.append(f"  flags: {flags}")
        bugs = v.get("visible_bugs") or []
        if bugs:
            lines.append(f"  visible_bugs: {bugs}")
    lines.append(
        "Fix the issues and output ONLY the corrected HTML source. "
        "No markdown fences, no prose. The entire file must be self-contained HTML."
    )
    return "\n".join(lines)


def pipeline(spec_path: str, module_name: str, max_iter: int = 3, threshold: float = 0.6) -> dict:
    base_spec = pathlib.Path(spec_path).read_text()
    record = {"spec": spec_path, "module": module_name, "iterations": [], "threshold": threshold}

    current_spec = base_spec
    for it in range(1, max_iter + 1):
        iter_rec = {"iter": it}
        iter_start = time.time()

        out, codex_time = run_codex(current_spec)
        iter_rec["codex_time_s"] = round(codex_time, 1)
        html = extract_html(out)
        if html is None:
            iter_rec["status"] = "extract_fail"
            iter_rec["codex_tail"] = out[-300:]
            record["iterations"].append(iter_rec)
            continue

        html_path = f"/tmp/visual-bodies/{module_name}_iter{it}.html"
        pathlib.Path(html_path).parent.mkdir(parents=True, exist_ok=True)
        pathlib.Path(html_path).write_text(html)
        iter_rec["html_path"] = html_path
        iter_rec["html_len"] = len(html)

        shot_dir = f"/tmp/visual-shots/{module_name}_iter{it}"
        ok, shots, js_errors = take_screenshots(html_path, shot_dir)
        iter_rec["screenshots"] = [os.path.basename(s) for s in shots]
        iter_rec["js_errors"] = js_errors
        if not ok:
            iter_rec["status"] = "screenshot_fail"
            record["iterations"].append(iter_rec)
            current_spec = (
                base_spec + "\n\nPREVIOUS ATTEMPT FAILED: Screenshot subprocess crashed. "
                "Ensure the HTML is valid and the JS parses."
            )
            continue

        verdict = judge_best(shots)
        iter_rec["verdict"] = verdict
        iter_rec["iter_total_s"] = round(time.time() - iter_start, 1)
        record["iterations"].append(iter_rec)

        if verdict["best_score"] >= threshold:
            record["status"] = "ok"
            record["final_score"] = verdict["best_score"]
            record["final_html"] = html_path
            return record

        # Feed the failure back into the next iteration
        feedback = build_failure_feedback(verdict, js_errors)
        current_spec = base_spec + "\n\n" + feedback

    record["status"] = "max_iter_exceeded"
    record["final_score"] = max((i.get("verdict", {}).get("best_score", 0) for i in record["iterations"]), default=0)
    return record


if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("usage: pipeline_visual.py <spec_file> <module_name> [--max-iter N]", file=sys.stderr)
        sys.exit(2)
    max_iter = 3
    if "--max-iter" in sys.argv:
        max_iter = int(sys.argv[sys.argv.index("--max-iter") + 1])
    result = pipeline(sys.argv[1], sys.argv[2], max_iter=max_iter)
    print(json.dumps(result, indent=2, default=str))
    sys.exit(0 if result.get("status") == "ok" else 1)
