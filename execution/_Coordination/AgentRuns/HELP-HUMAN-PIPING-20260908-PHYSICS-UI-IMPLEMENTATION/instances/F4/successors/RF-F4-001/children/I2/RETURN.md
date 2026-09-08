# F4-I2 Return

Status: candidate successor complete; pending fresh independent backcheck and manager validation. No acceptance claim is made.

- Scope: test-only changes inside the primary `#[cfg(test)] mod tests` in `projects/chirality-piping/core/solver/nonlinear_integration/src/lib.rs`; production bytes and the later computed-finite test module are unchanged.
- Frozen V1 source SHA-256: `da4cc3f...` (full value recorded by the owning F4 run).
- Candidate source SHA-256: `6e163a47db60288179f844d473992d28a53599ad2204149093186c760723ad46`.
- Immutable-region SHA-256: before primary tests `311b4d6657c103982c78d8e86d4efa7dc5d450556802dc3a45d5ce6264bf42ae`; later computed-finite module through EOF `73463334658edb9ecf09dd48b4b733410e89d1fe95219fb8f485ff3ff7484ea8`.
- Base-to-successor patch: `successors/RF-F4-001/CANDIDATE_DIFF_BASE_TO_SUCCESSOR.patch`, SHA-256 `ff74ab001f108f36a6e0f4105b4fe40a7a7d784045e4fc0827518074a62d97cc`.
- V1-to-successor patch: `successors/RF-F4-001/CANDIDATE_DIFF_V1_TO_SUCCESSOR.patch`, SHA-256 `9bdf1403176e697534c4a373250d4f81930336065b13d7ee79a618e21aa1d09e`.
- Both persisted patches were verified byte-identical to their live generated diffs.
- Focused proof passed for both nonzero assumed branches accepting exact zero current normal, including the mirrored negative branch, and for signed `+0.0`/`-0.0` active zero branches rejecting a cross-coupled final normal of `11/41` after an exact affine base normal of zero. All retained focused cases passed.
- Full crate: `cargo test --locked --offline` passed, 29 passed, 0 failed; doc tests 0.
- Formatting: the initial `cargo fmt --check` failed only on new formatting; `cargo fmt` and the final `cargo fmt --check` passed. `git diff --check` passed.
- Private Cargo target and temporary V1 reconstruction were removed.
- Unresolved blockers: none.

Exclusive source/Rust ownership is released with this terminal return.
