# F4 bounded implementation return

Status: **SUCCESS — READY_SOURCE_FREEZE**

Recorded: `2026-09-09T07:17:02Z`

## Authority and fence

- Active normalized launch brief: `IMPLEMENTATION_LAUNCH_BRIEF_V1.md` at SHA-256 `88722b8597722f0a16e754d285cbafae602aa3f225b4a14e824d5bd98ac5b81d`.
- Root-accepted priority amendment: `IMPLEMENTATION_LAUNCH_BRIEF_AMENDMENT_V2.md` at SHA-256 `b44297669accae8872102b347b4818c1d263428fe98486b8406ba0991ab2d5cc`.
- Active normalized plan: SHA-256 `8e0b0f352921c98d05b5fce603a02e2f05106ec432f14f69c1bad4686366db7c`, bound through `../NORMALIZED_POINTER_V1.json` at SHA-256 `5bcd7f04e8f824848e7f5b15cbf7e4ddf43d9aa396ec0dd19c1d5da307e67db9`.
- Frozen validation matrix: `../FRICTION_REPAIR_VALIDATION_MATRIX_V1.md` at SHA-256 `91a2789b856cf5244dc942fb414e86753332bbc452b1859238e512ad23f4251b`.
- The only edited source files are the two files authorized by the sealed brief. Git state was inspected read-only; no Git mutation was performed.

## Implemented behavior

- `nonlinear_supports` now exposes an evaluator that applies integration-resolved friction states before state-change residual, convergence, and diagnostic formation. The ordinary evaluator remains source compatible and uses current trial classification without displacement-only sliding persistence.
- `nonlinear_integration` resolves all current sliding rows from same-iterate evidence, then passes the complete resolved set to the support evaluator. Contact validity, deferred warm start, derived signed-normal validity, zero-limit behavior, and tangential opposition are applied in the amended binding order.
- Convergence requires the active-set result, a nonblocking result, completion of any deferred warm start, aggregate tangential admissibility, and aggregate derived signed-normal admissibility.
- An inadmissible derived signed-normal branch retains `Sliding`, skips provisional tangential judgment, and forces a retry through the derived aggregate gate. Explicit-positive-normal sublimit candidates still re-stick when force/motion evidence is inadmissible.
- No public schema, tolerance, diagnostic code, load-step history, or friction-history contract was added.

## Regression evidence

- Both derived-normal sign-flip cases retain `Sliding` on iteration 2 with the sealed rational observations and converge on iteration 3 with opposing, current-normal-bounded force.
- Derived `abs(R_n) == 0` is proven at the assembled solver level to select `Inactive` for both load signs and sparse/dense modes. Explicit zero/negative contact remains inactive; positive-contact zero-limit friction remains a distinct released-sliding case.
- A mixed-row oracle gives one admissible and one inadmissible tangential row. The active-set classifier is deliberately within its configured tolerance while the aggregate is false, proving that the aggregate gate independently blocks caller convergence.
- Mixed rows update together independent of row order; explicit sublimit, exact boundary, superlimit, iteration-cap, seed, sign, and solver-mode cases remain covered.

## Final source identity

| Source | Preimage SHA-256 | Final SHA-256 |
| --- | --- | --- |
| `projects/chirality-piping/core/solver/nonlinear_supports/src/lib.rs` | `f6c6e62994564dddfc84e60df63f2e2f3a293b1d48a5950469c4368509733062` | `956f64abfab50f16310641e2ffa63eccf37a3cb31ce48332fe7c14be28125d11` |
| `projects/chirality-piping/core/solver/nonlinear_integration/src/lib.rs` | `6e163a47db60288179f844d473992d28a53599ad2204149093186c760723ad46` | `d0b130fd8fe78c6eb18a824ebfec917bbff3f7865ceef9ccca1d332b17f228ce` |

The SHA-256 of `git diff --binary` restricted to those two source files is `1246d391a5c362b3f92a57288fe3c03a51b2eb88f4e6d733dc90b27c3ee6404f`.

## Validation

All final Cargo results used the isolated target directory:

`{RUN_ROOT}/instances/F4/implementation/.cargo-target`

- `nonlinear_supports`: 22 unit tests passed; doc tests passed.
- `nonlinear_integration`: 38 unit tests passed; doc tests passed.
- nonlinear benchmark crate: 19 unit tests passed; doc tests passed.
- `product_physics` `friction_preview`: 3 tests passed.
- `product_physics` `mixed_nonlinear_preview_bundle_converges_and_emits_each_support_state`: 1 test passed.
- `rustfmt` completed for both source files.
- `git diff --check` passed for the two-file source diff.
- `validate_change_scope.py` passed for the two source paths and this implementation evidence root.

Exact commands, attempt history, results, hashes, and scope evidence are recorded in `VALIDATION.json`.

## Residual coordination

- The root owns the combined final read-only reviewer and any later Git operation.
- The evidence-local Cargo target is retained as isolated scratch and is outside the source diff.
- No external solver comparison was run; it was not required or available for this bounded repair.
