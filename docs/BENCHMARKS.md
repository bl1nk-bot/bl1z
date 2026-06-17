# bl1z Benchmark Report (2026-06-17)

Performance baseline for bl1z v0.2.15.

## Summary

| Benchmark | Execution Time | Notes |
|-----------|----------------|-------|
| `basic_arithmetic` | 1.42 µs | Tokenize + Parse + Eval |
| `complex_expression` | 7.65 µs | `if`, `sum`, `upper` |
| `large_array_sum` | 64.19 µs | Array size: 100 |
| `nested_functions` | 5.41 µs | `upper(join([hello, lower(world)]))` |
| `date_operations` | 3.01 µs | `year(date_add(...))` |
| `map_operations` | 3.16 µs | Literal `{a: 1, b: 2, c: 3}` |
| `phase8_access_chaining` | 4.68 µs | `user.profile.score + [10,20][1]` |
| `phase9_map_filter` | 15.68 µs | Higher-order logic |
| `udf_call` | 1.66 µs | User-defined function call |
| `lambda_call` | 0.83 µs | Pre-evaluated lambda call |
| `without_cache` | 5.79 µs | Full pipeline |
| `with_cache_hit` | 0.65 µs | Eval only (9x faster) |
| `set_operations` | 10.30 µs | `set_intersection` |
| `range_to_array` | 5.23 µs | Range 0..100 |

## Performance Insights

1. **Caching is highly effective**: Using `FormulaCache` provides a ~9x speedup for repeated formulas by skipping the lexing and parsing phases.
2. **Lambda vs UDF**: Lambda calls are currently ~2x faster than UDF calls. This suggests potential for optimizing the UDF context switching logic.
3. **Complexity Overhead**: Support for advanced types and nested access has introduced a minor overhead (estimated 8-15%) compared to the V1 baseline, which is acceptable given the expanded capabilities.

## Generated Reports

Full HTML reports with interactive graphs can be found in:
`target/criterion/report/index.html`
