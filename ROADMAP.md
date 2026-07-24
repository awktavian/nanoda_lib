# nanoda_lib Roadmap

## Phase 1: Scope

- Define supported Lean export format and unsupported constructs.
- Add a public API example for library embedding.
- Document error kinds and stability policy.

## Phase 2: Corpus Evidence

- Build a small checked fixture corpus.
- Add a mathlib-export benchmark mode.
- Produce a compatibility report with failing declarations grouped by cause.

## Phase 3: Packaging

- Publish docs.rs examples.
- Add release checklist with `cargo test`, corpus smoke, and binary-size check.

## Exit Criteria

`nanoda_lib` is release-ready only when users can independently check a real
Lean export artifact and reproduce the compatibility/performance report.
