# nanoda_lib Positioning

## Wedge

`nanoda_lib` is an independent Lean 4 export/typechecking library. Its value is
not replacing Lean; it is providing a small, inspectable proof-artifact checker
for Lean-adjacent tooling.

## ICP

- Formal-methods teams needing independent Lean artifact inspection.
- Proof-carrying-code systems that want a Rust checker boundary.
- Researchers studying Lean kernel/export behavior.

## Comparable Systems

| Alternative | Strength | Gap nanoda_lib must fill |
|---|---|---|
| Lean kernel | Authoritative checker. | Smaller embeddable Rust artifact checker. |
| lean4export | Export format and tooling. | Library API with reproducible corpus checks. |
| Custom proof checkers | Tailored to one project. | Lean-specific semantics and test corpus. |

## Honest Limitations

- This is specialist infrastructure; the market is small.
- Supported Lean syntax/export scope must be documented precisely.
- Performance and compatibility claims need mathlib-scale corpus evidence.

## Release gate

The release bar is publishing a compatibility matrix, corpus benchmark over
Lean export artifacts, and an embeddable API example that checks a real proof
artifact end to end.
