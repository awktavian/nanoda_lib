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

Verified 2026-09-18 at `a2f789e` (plus lane W11-A4 doc/API changes):
`cargo test` rc=0 (38 unit tests passed, 0 failed; doctests: 1 passed,
7 ignored), `cargo run -- --help` rc=0 (prints README),
`cargo clippy --all-targets` rc=0 (warnings present, not failures). These are
unit-test facts only — per the Claim Rule below they license no compatibility
claim.

The release benchmark must add a corpus command that records commit SHA,
Lean/export version, corpus path, declaration count, failure count, and peak
RSS.

## Claim Rule

Do not claim Lean compatibility from unit tests alone. Compatibility claims
require a named corpus and a failure taxonomy.
