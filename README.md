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
