# nanoda_lib Roadmap

Status annotations added 2026-09-18 (lane W11-A4) by checking each item against
source at `a2f789e`; the original plan is unchanged.

## Phase 1: Scope

- Define supported Lean export format and unsupported constructs.
  PARTIAL: the supported export-format version range is enforced in code
  (`MIN_SEMVER`/`MAX_SEMVER` in `src/parser.rs`, `[3.1.0, 3.2.0)`) and is now
  documented in README ("Supported export format"). The enumeration of
  unsupported kernel constructs is still missing from the docs.
- Add a public API example for library embedding.
  DONE (this lane): the `src/lib.rs` `Placeholder` doctest was replaced by a
  compiled, passing example that builds a `Config`, parses
  `test_resources/Empty/export` via `to_export_file`, and runs
  `check_all_declars` (doctest result: 1 passed, 7 ignored). Remaining
  `ignore`d examples in module doc comments are unchanged.
- Document error kinds and stability policy.
  OPEN: config-level errors exist in `src/util.rs` (mutually-exclusive option
  checks) but there is no error-kind catalogue or stability policy document.

## Phase 2: Corpus Evidence

- Build a small checked fixture corpus.
  DONE: `test_resources/Empty` and `test_resources/ProjFromProp` (with its own
  config.json) are consumed by `src/tests/util.rs`.
- Add a mathlib-export benchmark mode.
  OPEN: no benchmark mode exists in the binary.
- Produce a compatibility report with failing declarations grouped by cause.
  OPEN: blocked on the mathlib-export benchmark mode above.

## Phase 3: Packaging

- Publish docs.rs examples.
  OPEN: `nanoda_lib` is not published on crates.io (`cargo info`: not found,
  2026-09-18); docs.rs builds require publication. First missing input: a
  publish decision.
- Add release checklist with `cargo test`, corpus smoke, and binary-size check.
  OPEN: `cargo test` is the only piece that exists.

## Exit Criteria

`nanoda_lib` is release-ready only when users can independently check a real
Lean export artifact and reproduce the compatibility/performance report.
