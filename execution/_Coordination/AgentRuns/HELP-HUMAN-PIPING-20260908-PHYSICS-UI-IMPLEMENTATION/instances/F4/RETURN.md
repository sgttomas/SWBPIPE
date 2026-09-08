# F4 implementation return

Status: `CANDIDATE_COMPLETE_PENDING_FRESH_INDEPENDENT_REVIEW`

The frozen V2 current-normal affine friction correction is implemented only in `core/solver/nonlinear_integration/src/lib.rs`. Manager review found no actionable defect or scope drift. The live source SHA-256 is `da4cc3f5d21f1e1841d375dd2525f3bb2d3f0e5d3e960bd2389d32cef3a8b320`; the complete live-matching diff SHA-256 is `8080a8c242931409863771aba963897a72b055c39aee1d61fd567eee4bb3801b`. Product-physics remains unchanged at `e757b8a51e2c4ae68ac4d6c37620663bf4d698ff03b8d6349b40b484bb591903`.

Manager offline locked crate validation passed 29/29 and formatting passed. Child focused fixtures, unchanged M1-N-008 replay, configured-interpreter 1012-test piping suite, and harness self-check passed. Runtime branch/convergence logic has no tolerance. The clean-source evidence sweep is deferred to root during concurrent U7 writes.

The isolated Cargo targets were removed and the exclusive Rust slot is released. The candidate requires fresh independent 100% diff review before acceptance or release. No commit or push was made.
