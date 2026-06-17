# Profile-Guided Optimization (PGO) for bl1z

PGO lets the compiler make better decisions using real execution data.
The profile highlights which code paths are hot and which branches are
typically taken, guiding the optimizer to place code for the common case.

## Why PGO helps bl1z

The evaluator (`src/eval.rs`) is dominated by a large `match` on `Expr`
variants — `BinaryOp`, `Call`, `FunctionCall`, `If`, `MapLiteral`, etc.
Without PGO the compiler treats each arm roughly equally. With a profile
the compiler learns which variants are most frequent and can:

* **Reorder match arms** so the hot variant is tested first.
* **Inline hot functions** — the evaluator's inner helpers, lambda
  application, and property-access paths become smaller and faster.
* **Improve branch prediction** — conditional error paths (type checks,
  bounds checks) are laid out as cold code, keeping the happy path
  contiguous.
* **Optimize loop bodies** — builtins like `sum`, `map`, and
  `filter` that iterate over arrays benefit from better register
  allocation and loop unrolling.

Combined with the existing release profile (`opt-level = 3`, LTO,
`codegen-units = 1`), PGO typically adds another 5–15% throughput
improvement for this kind of codebase.

## Collecting profiles

### Step 1 — Instrumented build

```bash
RUSTFLAGS='-Cprofile-generate' cargo build --release
```

This produces instrumented binaries that write a `.profraw` file for
each thread on exit.

### Step 2 — Run the benchmark suite

```bash
cargo bench --bench evaluation
# or for a quicker single-shot run:
cargo bench --bench evaluation -- --quick
```

The criterion harness exercises all the hot paths (arithmetic, function
calls, array operations, lambdas, access chaining, sets, dates, cache
hits).  After the run you will see one or more `.profraw` files in the
project root (or in a path printed by the instrumented binary).

### Step 3 — Merge profiles

```bash
# Locate the llvm-profdata tool shipped with your toolchain:
llvm-profdata merge -sparse *.profraw -o bl1z.profdata
```

If you have multiple profiles you can merge them:

```bash
llvm-profdata merge -sparse *.profraw -o bl1z.profdata
```

### Step 4 — Optimized build

```bash
RUSTFLAGS='-Cprofile-use=bl1z.profdata' cargo build --release
```

The resulting binary is PGO-optimized.

### Quick one-liner (Linux / macOS)

```bash
RUSTFLAGS='-Cprofile-generate' cargo build --release && \
cargo bench --bench evaluation && \
llvm-profdata merge -sparse *.profraw -o bl1z.profdata && \
RUSTFLAGS='-Cprofile-use=bl1z.profdata' cargo build --release
```

## Running the benchmark suite

The benchmarks live in `benches/evaluation.rs` and use
[criterion](https://bheisler.github.io/criterion.rs/book/) (v0.8).

| Command | Description |
|---------|-------------|
| `cargo bench` | Run all benchmarks, generate HTML reports |
| `cargo bench --bench evaluation` | Run only the evaluation benchmarks |
| `cargo bench -- "basic_arithmetic"` | Run a single benchmark by name |
| `cargo bench -- --quick` | Faster, less precise run (good for profiling) |

After a run, open `target/criterion/report/index.html` for interactive
charts and comparison with previous baselines.

## Baseline benchmarks

These numbers are from bl1z **v0.2.15** on a typical development machine
(LTO + `codegen-units = 1`, no PGO).

| Benchmark | Time | Description |
|-----------|------|-------------|
| `basic_arithmetic` | ~1.42 µs | `1 + 2 * 3` full pipeline |
| `complex_expression` | ~7.65 µs | `if`, `sum`, `upper` |
| `large_array_sum` | ~64.19 µs | `sum([1..100])` |
| `nested_functions` | ~5.41 µs | `upper(join([…]))` |
| `date_operations` | ~3.01 µs | `year(date_add(…))` |
| `map_operations` | ~3.16 µs | `{a: 1, b: 2, c: 3}` |
| `phase8_access_chaining` | ~4.68 µs | `user.profile.score` |
| `phase9_map_filter` | ~15.68 µs | HOF filter + map chain |
| `udf_call` | ~1.66 µs | User-defined function |
| `lambda_call` | ~0.83 µs | Pre-evaluated lambda |
| `with_cache_hit` | ~0.65 µs | Eval-only (cache hit) |

With PGO expect these times to drop by roughly **5–15%**, with larger
gains on the more complex benchmarks (higher-order functions, UDF calls)
where branch prediction and inlining matter most.

## Tips for maintaining profile quality

1. **Re-collect profiles when the code structure changes significantly** —
   adding new `Expr` variants, restructuring the evaluator's match arms,
   or changing which builtins are registered can shift the hot path.
   Re-run the full 4-step workflow after such changes.

2. **Profile on a realistic workload** — the criterion benchmarks are a
   good starting point.  If your production usage evaluates different
   formula patterns, consider writing an additional benchmark or running
   a representative script and merging its profile with the criterion
   profile.

3. **Use `llvm-profdata merge` to combine profiles** — you can merge
   profiles from different runs (development + production) into a single
   `.profdata`:

   ```bash
   llvm-profdata merge -sparse dev.profraw prod.profraw -o combined.profdata
   ```

4. **Check that `.profraw` files are being generated** — if no `.profraw`
   appears after the instrumented run, check that
   `LLVM_PROFILE_FILE` is not set to a location you didn't notice, or
   that the instrumented binary is actually being executed.

5. **Keep the release profile as-is** — the current settings
   (`opt-level = 3`, `lto = true`, `codegen-units = 1`) are already
   optimal for PGO.  Do not increase `codegen-units` as it reduces
   inlining opportunities.
