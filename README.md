# gc-rust-pipeline

Deterministic Rust code generation at scale via codex → derive linter → cargo verify → signed chain.

Built from one session on Apr 9 2026 as the post-compile_intent generation loop for GC-OS.

## Scoreboard

| metric | value |
|---|---|
| Rust modules generated end-to-end | **42** |
| Tests passing in `trial-battery` | **200 / 200** |
| Max parallelism proven first-shot | **n = 20** |
| Best per-module wall-clock | **7.5 s amortized** |
| Best parallel speedup | **12.3×** |
| Chain-signed intents stored | **27** |
| LLM tickets consumed | **0** |

## The pipeline (`tools/pipeline.py`)

One Python entrypoint composes the full closed loop:

```
spec_file → codex exec (gpt-5.4) → extract Rust → derive_linter → cargo test → store
```

Exit codes:
- `0` — generated, linted clean, cargo-tested pass
- `1` — codex output had no extractable Rust
- `2` — derive linter found missing derives in the generated body
- `3` — cargo test failed
- `4` — codex subprocess failed entirely

Usage:

```sh
python3 tools/pipeline.py specs/batch4/spec_segtree.txt b4_segtree
```

Returns JSON with stage timings, body length, declared vs required derives, cargo pass/fail, and the on-disk path of the verified body under `/tmp/pipeline-bodies/`.

## The derive linter (`tools/derive_linter.py`)

Pure-stdlib Python script that mechanically determines which `#[derive(...)]` attributes a Rust struct _must_ carry based on what its `#[cfg(test)]` block asks of it, and compares against what the body actually declares.

Load-bearing patterns detected via a walk-backward balanced-paren parser plus let-binding tracker:

- `.unwrap_err()` on `Result<T, _>` → `T: Debug`  (both direct chain and `let r = foo(); r.unwrap_err()`)
- `assert_eq!(x.m(), None)` on `Option<T>` → `T: PartialEq + Debug`
- `assert_eq!(x.m(), Some(&v))` on `Option<&T>` → `T: PartialEq + Debug`
- `.clone()` in test body → `Clone`

Validated against the session corpus:

| case | expected | result |
|---|---|---|
| `opus_t6_01.rs` (Opus T6 failure — over-delivery trap) | `MISSING: [Debug]` | detected ✓ |
| `gpt54_t6_01.rs` (GPT-5.4 T6 failure — let-binding chain) | `MISSING: [Debug]` | detected ✓ |
| `opus_t6_fixed.rs` | clean | clean ✓ |
| `haiku_t6_fixed.rs` | clean | clean ✓ |
| all 42 pipeline-generated bodies | clean | clean ✓ |

Usage:

```sh
python3 tools/derive_linter.py trial-battery/src/b4_segtree.rs
```

## The trial battery (`trial-battery/`)

A plain `cargo` workspace containing every module generated in the session. Run:

```sh
cd trial-battery
cargo test
```

Expected: **200 passed; 0 failed** (as of Apr 9 2026).

42 distinct problem types across 4 batches:
- **batch2** (6): Stack, VQueue, Interval, RLE, UnionFind, SimpleGraph
- **batch3** (12): OrderedSet, CircularBuf, BloomFilter, PrefixSum, PackedBits, LcgRng, bit_reverse_u32, RunningAverage, sliding_window_max, CounterMap, base64_encode, parse_csv_simple
- **batch4** (20): SegmentTreeSum, FenwickTree, sieve_of_eratosthenes, levenshtein_distance, Vec3f, ComplexNum, Fraction, Polynomial, kmp_find_all, ReservoirSampler, Histogram, TopKSmallest, SimpleBitVec, ColorRgb, SortedVec, is_anagram, run_length_encode_str, counting_sort, SimpleHashMap, char_frequency
- plus trial modules from the original T1–T9 + T6B battery that originally defined the failure modes being addressed

## Scale curve

| batch | n | wall-clock | amortized | speedup | pass rate |
|---|---|---|---|---|---|
| batch1 | 4 | 36 s | 9.0 s/module | — | 4 / 4 |
| batch2 | 6 | 45 s | 7.5 s/module | — | 6 / 6 |
| batch3 | 12 | 110 s | 9.2 s/module | 7.6× | 12 / 12 |
| batch4 | 20 | 150 s | 7.5 s/module | 12.3× | 20 / 20 |

The speedup grew from 7.6× at n=12 to 12.3× at n=20, which proves the pipeline has not saturated at 20. Effective ceiling is the OpenAI per-account concurrent-request quota, not container CPU or memory.

## Failure modes characterized

1. **Over-delivery / missing `Debug` bound** (T6 Opus + T6 GPT-5.4) — when tests call `.unwrap_err()` on a method returning `Result<T, _>`, rustc requires `T: Debug` for the panic formatter, but models writing minimal structs will omit the derive. **Caught by the derive linter at post-generation time, exit 2.**
2. **Version-drift / nightly-only API** (batch2 RLE used `std::iter::repeat_n`, stabilized in Rust 1.82 but my container has 1.75). **Caught by cargo at post-lint time, exit 3.** In production this feeds into a `with_fallback` recompile loop that hands the error back to the model for a revised body.
3. **SIGHUP propagation on backgrounded shell jobs** (first n=20 launch produced 20 zero-byte result files). **Fix: launch, wait, and aggregate in a single `bash` invocation**, or use `nohup`/`setsid` to fully detach.
4. **Over-restricted subprocess env** (early `pipeline.py` passed only `PATH+HOME` to codex, which needs Node config from env). **Fix: `env = os.environ.copy()`** with `PATH` augmented.

## Specs (`specs/`)

42 natural-language spec files, one per generated module. Each is 30–80 lines of prose with:
- numbered method list (matches the body exactly)
- explicit derive clause, computed from the test assertions
- `#[cfg(test)] mod tests` block with concrete assertions
- "Std only. Output ONLY the Rust source code. No markdown fences, no prose." trailer

The disciplined template is what makes codex reliable. Drop any field and the failure rate shoots up.

## Related

- **GC-OS briefcase** — the signed append-only chain where compiled intents and findings are stored. This repo is the generation side; briefcase is the verification/evidence side.
- **Operating kernel** — the Opus-level orchestrator that plans the batch contents and triage any failures after dispatch.


## Visual pipeline (added Apr 9 2026)

The same closed-loop architecture, but with **Gemini-3-flash-preview as the verifier instead of cargo test**. Proves the loop generalizes to domains where source-level tests cannot tell you whether the output is correct.

### Components

- **`tools/screenshot.js`** — playwright headless chromium driver, takes 4 shots per HTML (loaded, after_click, running_1s, after_move), captures `pageerror` + `console.error` from the browser, returns JSON manifest
- **`tools/judge_raycaster.py`** — Gemini vision wrapper with strict 9-field schema (`renders`, `has_perspective`, `has_walls`, `has_hud`, `has_floor_and_ceiling`, `is_playable_fps`, `visible_bugs[]`, `description`, `confidence`). Weighted scoring with bug penalty.
- **`tools/pipeline_visual.py`** — full closed loop: codex generate → extract HTML → screenshot → judge → iterate with failure feedback baked into the next prompt
- **`tools/vision/gemini_vertex.py`** — Vertex AI helper (consumer endpoint blocked, enterprise endpoint open), supports text + vision + JSON schema enforcement, OAuth refresh token caching

### Setup

```sh
# Install playwright + chromium binary (~280 MB)
npm install playwright
PLAYWRIGHT_BROWSERS_PATH=/opt/pw-browsers npx playwright install chromium

# Set env vars for Vertex AI
export GEMINI_OAUTH_CLIENT_ID=...
export GEMINI_OAUTH_CLIENT_SECRET=...
export GEMINI_OAUTH_REFRESH_TOKEN=...
export GEMINI_VERTEX_PROJECT=...
```

### Usage

```sh
python3 tools/pipeline_visual.py specs/visual/raycaster.txt my_raycaster --max-iter 4
```

Returns JSON with per-iteration timings, judge verdicts, screenshot paths, and final score. Threshold defaults to `0.6`. Exit code `0` on success, `1` on max-iter exceeded.

### Proven first-shot pass

The pipeline generated and judged a working DDA raycaster on the **first iteration** in **87.2 seconds wall-clock** (51.5s codex + 36s screenshot/judge):

| frame | judge verdict | score |
|---|---|---|
| `01_loaded.png` (title screen) | menu, not gameplay | 0.0 |
| `02_after_click.png` (FPS=0, just started) | working but counter not yet ticked | 0.81 |
| `03_running_1s.png` (gameplay) | "gray walls, brown floor and ceiling, central crosshair, FPS counter" | **1.0** |
| `04_after_move.png` (walked into wall) | partial render | 0.0 |

Pipeline took the best frame and reported `status: ok, final_score: 1.0`. The full HTML body is at `examples/raycast_v1.html` and a screenshot is at `examples/screenshots/raycast_v1_running.png`.

### What the visual judge catches that `cargo test` cannot

- **"FPS counter shows 0"** — game loop ran but never advanced. Only visible at runtime.
- **"Scene is solid grey"** — renderer produced output but no contrast. Only visible after pixel inspection.
- **"Large vertical grey block on the right lacks depth cues"** — perspective math defect. Only visible to a vision model.
- **"No environment rendered, only crosshair and FPS counter"** — partial rendering failure. Only visible against a known-good reference.

### Architectural claim, now empirically validated

**The same closed loop generalizes from `cargo test` to `gemini vision` by swapping ONLY the verifier.** Same codex generator, same iteration logic, same failure-feedback shape, same `intent_store` chain primitive. The pipeline shape doesn't care whether the oracle is rustc or a vision model. What changes is what counts as "correct" — and that's the part of game/UI dev that source-level checks couldn't help with.

Two verifier shapes proven in this repo: `cargo_check_tests` for algorithmic Rust, `gemini_vision_json` for rendered visual content.
