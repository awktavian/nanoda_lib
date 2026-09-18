# STATUS — Campaign a9 (zero-edge islands) — quick note

**Decision:** ARCHIVE  
**Date:** 2026-08-08  
**Visibility:** upstream (`ammkrn/nanoda_lib`) + personal fork (`awktavian/nanoda_lib`)

## Why

External Lean 4 typechecker checkout with no in-estate consumer path — keep as frozen upstream reference, do not treat as an awkronos product.

## Archive manifest

Keep git history and `fork` remote; no feature work; re-open only if Lean export checking becomes a studio gate.

## Sync ledger (measured 2026-09-18)

- "No feature work" holds for the estate: the only commits after this archive note are upstream safety merges (PR #26–#28: recursor checks, is_sort guards, orphan-recursor prohibition). Last upstream sync: `a2f789e` (2026-08-29); nothing newer on any remote since.
- "No in-estate consumer path" holds as of 2026-09-18: the earlier `pcc-lean` bindings (`check_and_type_bytes`, `type_bytes_only` in `src/lib.rs`, added 2026-06-05/06) have no consumer — no `pcc-lean` checkout remains in `~/Projects` — and the crate is not published on crates.io (`cargo info nanoda_lib`: not found, 2026-09-18). The public API is therefore dormant surface, not a live dependency.
- Mechanical state at the 2026-09-18 doc-truth lane (`a2f789e` + doc/lib-example commits): `cargo test` rc=0 (38 unit tests pass; doctests 1 passed / 7 ignored), `cargo clippy --all-targets` rc=0 with warnings (not errors).
