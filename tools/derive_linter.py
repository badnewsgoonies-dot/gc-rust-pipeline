#!/usr/bin/env python3
"""Derive-set linter for compile_intent-style Rust specs.

Takes a spec text (natural language + Rust fragments) and:
  1. Finds the primary struct name and its declared derive set
  2. Finds the impl block's method signatures (return types)
  3. Scans the test block for assertion patterns
  4. Computes the minimum required derive set from those patterns
  5. Reports the gap between declared and required

Supported patterns (v1):
  - unwrap_err() on a method returning Result<&T, E>  → T: Debug
  - unwrap_err() on a method returning Result<T, E> where T is the struct → struct: Debug
  - assert_eq!(m(), None) where m() returns Option<&Struct> → Struct: PartialEq
  - assert_eq!(m(), Some(&val)) where m() returns Option<&Struct> → Struct: PartialEq
  - assert_eq!(m(), value) where m() returns Struct → Struct: PartialEq + Debug
  - .clone() on a struct instance in test body → struct: Clone

Not auto-fix; emits a report. Exit code 0 if declared ⊇ required, 1 otherwise.

Usage: python3 derive_linter.py <spec.txt>
       python3 derive_linter.py --demo
"""
import re
import sys
import pathlib

# ── parsing ─────────────────────────────────────────────────────────────────

DERIVE_LINE_RE = re.compile(r"#\[derive\(([^)]+)\)\]")
STRUCT_DECL_RE = re.compile(r"pub\s+struct\s+(\w+)(?:<([^>]+)>)?\s*\{")
METHOD_SIG_RE = re.compile(
    r"pub\s+fn\s+(\w+)\s*\([^)]*\)\s*->\s*([^\n{;]+)"
)
TEST_BLOCK_RE = re.compile(
    r"#\[cfg\(test\)\].*?mod\s+tests.*?\{(.+)\}\s*\Z", re.DOTALL
)


def parse_struct(text: str) -> dict:
    """Return {name, declared_derives, method_returns} for the first struct."""
    struct_match = STRUCT_DECL_RE.search(text)
    if not struct_match:
        return {}
    name = struct_match.group(1)
    # Find the #[derive(...)] line within 3 lines before the struct
    preamble = text[: struct_match.start()]
    derive_match = None
    for m in DERIVE_LINE_RE.finditer(preamble):
        derive_match = m  # last match before struct
    declared = set()
    if derive_match:
        declared = {d.strip() for d in derive_match.group(1).split(",")}
    # Find all method return types — scan whole text, not just impl block
    method_returns = {}
    for m in METHOD_SIG_RE.finditer(text):
        mname = m.group(1)
        ret = m.group(2).strip().rstrip(",").rstrip()
        method_returns[mname] = ret
    return {
        "name": name,
        "declared": declared,
        "method_returns": method_returns,
    }


# ── assertion pattern detectors ─────────────────────────────────────────────

# ── walk-backward balanced-paren parser for method chain detection ──────────


def _skip_ws_back(text: str, i: int) -> int:
    while i >= 0 and text[i] in " \t\n":
        i -= 1
    return i


def _read_ident_back(text: str, i: int) -> tuple[str, int]:
    """Read an identifier ending at i (inclusive). Returns (ident, new_i_before_ident)."""
    j = i
    while j >= 0 and (text[j].isalnum() or text[j] == "_"):
        j -= 1
    return text[j + 1 : i + 1], j


def _skip_balanced_parens_back(text: str, i: int) -> int:
    """Given that text[i] == ')', walk backward past the matching '('. Return index of '(' - 1."""
    if i < 0 or text[i] != ")":
        return i
    depth = 1
    i -= 1
    while i >= 0 and depth > 0:
        c = text[i]
        if c == ")":
            depth += 1
        elif c == "(":
            depth -= 1
            if depth == 0:
                return i - 1
        i -= 1
    return i


def walk_back_method_call(text: str, err_pos: int) -> tuple[str, str] | None:
    """Given position of '.' in '.unwrap_err()' (or similar), walk backward to find
    the preceding `obj.method(...)` and return (obj, method). Handles arbitrary paren nesting.
    Returns None if the preceding expression doesn't look like a method call on an identifier.
    """
    i = _skip_ws_back(text, err_pos - 1)
    if i < 0 or text[i] != ")":
        return None
    i = _skip_balanced_parens_back(text, i)
    i = _skip_ws_back(text, i)
    method, i = _read_ident_back(text, i)
    if not method:
        return None
    i = _skip_ws_back(text, i)
    if i < 0 or text[i] != ".":
        return None
    i = _skip_ws_back(text, i - 1)
    obj, _ = _read_ident_back(text, i)
    if not obj:
        return None
    return (obj, method)


def find_all_method_before(text: str, suffix: str) -> list[tuple[str, str]]:
    """Find all occurrences of `suffix` in text and walk backward from each to find the
    preceding method call. `suffix` should start with '.' (e.g. '.unwrap_err()')."""
    found: list[tuple[str, str]] = []
    start = 0
    while True:
        idx = text.find(suffix, start)
        if idx == -1:
            break
        pair = walk_back_method_call(text, idx)
        if pair is not None:
            found.append(pair)
        start = idx + len(suffix)
    return found


def find_assert_eq_with(test_body: str, rhs_matcher) -> list[tuple[str, str]]:
    """Find all `assert_eq!(<obj>.<method>(...), <rhs>)` where rhs_matcher(rhs_text)
    returns True for the comparand. Walks arguments with paren balancing.
    """
    out: list[tuple[str, str]] = []
    i = 0
    while True:
        idx = test_body.find("assert_eq!", i)
        if idx == -1:
            break
        j = idx + len("assert_eq!")
        while j < len(test_body) and test_body[j] in " \t":
            j += 1
        if j >= len(test_body) or test_body[j] != "(":
            i = idx + 1
            continue
        # Find the matching close paren for the assert_eq! call
        depth = 1
        k = j + 1
        while k < len(test_body) and depth > 0:
            if test_body[k] == "(":
                depth += 1
            elif test_body[k] == ")":
                depth -= 1
                if depth == 0:
                    break
            k += 1
        args = test_body[j + 1 : k]
        # Split on the top-level comma
        depth = 0
        comma_idx = -1
        for m in range(len(args)):
            c = args[m]
            if c == "(":
                depth += 1
            elif c == ")":
                depth -= 1
            elif c == "," and depth == 0:
                comma_idx = m
                break
        if comma_idx == -1:
            i = k + 1
            continue
        lhs = args[:comma_idx].strip()
        rhs = args[comma_idx + 1 :].strip()
        if not rhs_matcher(rhs):
            i = k + 1
            continue
        # lhs should end with `)` from a method call
        if lhs.endswith(")"):
            pair = walk_back_method_call(lhs, len(lhs))  # walk back from imaginary pos-after-expr
            # walk_back expects err_pos = position of '.' before '.unwrap_err' — here we want the
            # method call itself, so we walk back from len(lhs) treating it as if there's no suffix.
            # Simpler: parse directly.
            lhs_end = len(lhs) - 1  # position of ')'
            inner = _skip_balanced_parens_back(lhs, lhs_end)
            inner = _skip_ws_back(lhs, inner)
            method, rem = _read_ident_back(lhs, inner)
            if method:
                rem = _skip_ws_back(lhs, rem)
                if rem >= 0 and lhs[rem] == ".":
                    rem = _skip_ws_back(lhs, rem - 1)
                    obj, _ = _read_ident_back(lhs, rem)
                    if obj:
                        out.append((obj, method))
        i = k + 1
    return out


def _collect_let_bindings(test_body: str) -> dict[str, tuple[str, str]]:
    """Find `let <name> = <obj>.<method>(...);` patterns and return {name: (obj, method)}.
    Handles nested parens in the arguments via balanced-paren walking.
    """
    bindings: dict[str, tuple[str, str]] = {}
    for m in re.finditer(r"\blet\s+(?:mut\s+)?(\w+)\s*=\s*", test_body):
        name = m.group(1)
        start = m.end()
        # Expression runs until the terminating `;` at depth 0
        depth = 0
        end = start
        while end < len(test_body):
            c = test_body[end]
            if c == "(":
                depth += 1
            elif c == ")":
                depth -= 1
            elif c == ";" and depth == 0:
                break
            end += 1
        expr = test_body[start:end].strip()
        # Strip trailing .unwrap() / .expect(...) / .unwrap_err() / method chains we don't care about
        # by finding the innermost `<obj>.<method>(...)` prefix.
        if expr.endswith(")"):
            call_end = len(expr) - 1  # index of ')'
            inner = _skip_balanced_parens_back(expr, call_end)
            inner = _skip_ws_back(expr, inner)
            method, rem = _read_ident_back(expr, inner)
            if method:
                rem = _skip_ws_back(expr, rem)
                if rem >= 0 and expr[rem] == ".":
                    rem = _skip_ws_back(expr, rem - 1)
                    obj, _ = _read_ident_back(expr, rem)
                    if obj:
                        bindings[name] = (obj, method)
    return bindings


def scan_test_block(test_body: str, struct_name: str, method_returns: dict) -> dict:
    """Return {required_derives: set, evidence: list[str]}."""
    required: set[str] = set()
    evidence: list[str] = []
    primitives = {
        "()",
        "bool",
        "i8",
        "i16",
        "i32",
        "i64",
        "u8",
        "u16",
        "u32",
        "u64",
        "usize",
        "isize",
        "String",
        "char",
        "f32",
        "f64",
    }
    let_bindings = _collect_let_bindings(test_body)

    def method_return(method: str) -> str | None:
        return method_returns.get(method)

    def propagate(t: str, derive: str, why: str) -> None:
        if t == struct_name or t == "Self":
            required.add(derive)
            evidence.append(f"{why} → {struct_name}: {derive}")

    def check_unwrap_err_for_method(obj: str, method: str) -> None:
        ret = method_return(method) or ""
        rm = re.match(r"Result\s*<\s*(&?\s*[\w]+)", ret)
        if rm:
            t = rm.group(1).replace("&", "").strip()
            if t not in primitives and t != "T":
                propagate(
                    t, "Debug", f"{obj}.{method}().unwrap_err() (Result<{t},_>)"
                )

    # ── Direct chain: `obj.method(...).unwrap_err()` ────────────────────────
    for obj, method in find_all_method_before(test_body, ".unwrap_err()"):
        check_unwrap_err_for_method(obj, method)

    # ── Bare variable chain: `let x = obj.method(...); ... x.unwrap_err()` ──
    # Find all occurrences of `<name>.unwrap_err()` where name is in let_bindings
    for name, (obj, method) in let_bindings.items():
        if re.search(rf"\b{re.escape(name)}\s*\.\s*unwrap_err\(\)", test_body):
            check_unwrap_err_for_method(obj, method)

    # ── assert_eq!(x.m(), None) → PartialEq + Debug on inner of Option ─────
    for obj, method in find_assert_eq_with(test_body, lambda rhs: rhs == "None"):
        ret = method_return(method) or ""
        om = re.match(r"Option\s*<\s*(&?\s*[\w]+)", ret)
        if om:
            t = om.group(1).replace("&", "").strip()
            if t not in primitives and t != "T":
                propagate(
                    t, "PartialEq", f"assert_eq!({obj}.{method}(), None) (Option<{t}>)"
                )
                propagate(
                    t, "Debug", f"assert_eq!({obj}.{method}(), None) (panic-format)"
                )

    # ── assert_eq!(x.m(), Some(&...)) → PartialEq on T if method returns Option<&T> ─
    for obj, method in find_assert_eq_with(
        test_body, lambda rhs: rhs.startswith("Some(&")
    ):
        ret = method_return(method) or ""
        if "Option<&" in ret:
            tm_inner = re.search(r"Option\s*<\s*&\s*([\w]+)", ret)
            if tm_inner:
                t = tm_inner.group(1)
                if t not in primitives and t != "T":
                    propagate(
                        t,
                        "PartialEq",
                        f"assert_eq!({obj}.{method}(), Some(&_)) (Option<&{t}>)",
                    )
                    propagate(
                        t,
                        "Debug",
                        f"assert_eq!({obj}.{method}(), Some(&_)) (panic-format)",
                    )

    # ── .clone() in test body → Clone ──────────────────────────────────────
    if ".clone()" in test_body and struct_name in test_body:
        if re.search(rf"\b\w+\s*:\s*{struct_name}\b", test_body) or re.search(
            rf"{struct_name}::new", test_body
        ):
            if "Clone" not in required:
                required.add("Clone")
                evidence.append(f".clone() used in tests → {struct_name}: Clone")

    return {"required": required, "evidence": evidence}


# ── top level ───────────────────────────────────────────────────────────────


def lint_spec(text: str) -> dict:
    info = parse_struct(text)
    if not info:
        return {"ok": False, "error": "no pub struct found"}
    tm = TEST_BLOCK_RE.search(text)
    test_body = tm.group(1) if tm else ""
    if not test_body:
        return {
            "ok": False,
            "error": "no #[cfg(test)] mod tests block found",
            "struct": info["name"],
            "declared": sorted(info["declared"]),
        }
    scan = scan_test_block(test_body, info["name"], info["method_returns"])
    missing = scan["required"] - info["declared"]
    extra = info["declared"] - scan["required"] - {"Debug", "Clone", "PartialEq"}  # don't flag common surplus
    ok = not missing
    return {
        "ok": ok,
        "struct": info["name"],
        "declared": sorted(info["declared"]),
        "required": sorted(scan["required"]),
        "missing": sorted(missing),
        "evidence": scan["evidence"],
        "method_returns": info["method_returns"],
    }


def format_report(result: dict) -> str:
    if "error" in result:
        return f"ERROR: {result['error']}"
    lines = [
        f"struct:     {result['struct']}",
        f"declared:   {result['declared']}",
        f"required:   {result['required']}",
    ]
    if result["missing"]:
        lines.append(f"MISSING:    {result['missing']}  ❌")
    else:
        lines.append("status:     ALL REQUIRED DERIVES PRESENT ✓")
    if result["evidence"]:
        lines.append("evidence:")
        for e in result["evidence"]:
            lines.append(f"  - {e}")
    return "\n".join(lines)


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print(__doc__)
        sys.exit(2)
    arg = sys.argv[1]
    text = pathlib.Path(arg).read_text() if arg != "-" else sys.stdin.read()
    result = lint_spec(text)
    print(format_report(result))
    sys.exit(0 if result.get("ok") else 1)
