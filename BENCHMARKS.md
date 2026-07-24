# nanoda_lib Benchmark Contract

## Required Benchmarks

| Benchmark | Metric |
|---|---|
| Export corpus parse | files/sec, failures by reason |
| Typecheck corpus | declarations/sec, memory peak |
| Small artifact embed | cold-start time and binary size |
| Regression corpus | accepted/rejected delta vs previous release |

## Commands

Current smoke command:

```bash
cd nanoda_lib
cargo test
cargo run -- --help
```

The release benchmark must add a corpus command that records commit SHA,
Lean/export version, corpus path, declaration count, failure count, and peak
RSS.

## Claim Rule

Do not claim Lean compatibility from unit tests alone. Compatibility claims
require a named corpus and a failure taxonomy.
