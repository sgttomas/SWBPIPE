# F4 RF-F4-001 successor return

Status: `CANDIDATE_COMPLETE_PENDING_RF_BACKCHECK`

The test-only successor closes the evidence gap identified by `RF-F4-001`. It proves both signed-zero assumed branches through the actual base-sign fallback, affine coupling, branch-admissibility result, active-set evaluation, and caller convergence expression. It also mirrors the assumed-negative branch ending at exact zero.

The source SHA-256 is `6e163a47db60288179f844d473992d28a53599ad2204149093186c760723ad46`. The complete base-to-successor diff SHA-256 is `ff74ab001f108f36a6e0f4105b4fe40a7a7d784045e4fc0827518074a62d97cc`; the immutable V1-to-successor test-only diff SHA-256 is `9bdf1403176e697534c4a373250d4f81930336065b13d7ee79a618e21aa1d09e`.

Manager full-crate verification passed 29/29 and formatting passed. Both immutable non-test regions retain their V1 hashes. The original nonportable sealed brief is preserved losslessly by a structural serialized-original record; its portable V2 successor changes no task semantics.

The Rust slot is released. The candidate is frozen for the same RF reviewer’s backcheck. No commit or push was made.
