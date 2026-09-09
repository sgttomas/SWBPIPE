# F4 implementation manager return V1

Status: **FOCUSED IMPLEMENTATION PASS; SOURCE FROZEN FOR ROOT COMBINED REVIEW**

Recorded: `2026-09-09T07:25:58Z`

The bounded static Coulomb repair is implemented in exactly the two released solver files. The confirmed current-SHA seed defect is removed: the final state is selected from current contact, current normal branch, final motion, and final force evidence; the initial friction state remains only a numerical warm start.

The final algorithm uses the root-accepted reconciled order in `implementation/IMPLEMENTATION_LAUNCH_BRIEF_AMENDMENT_V2.md` at SHA-256 `b44297669accae8872102b347b4818c1d263428fe98486b8406ba0991ab2d5cc`: contact validity; the one deferred first sliding iterate; derived signed-normal branch validity; then zero-limit or positive-limit tangential admissibility. A force constructed from a disproven derived-normal branch is provisional, retains `Sliding`, and cannot converge until retried on the observed branch. A branch-valid positive-limit row re-sticks when its applied or reported force fails to oppose nonzero motion.

## Frozen source identity

| Source | Preimage SHA-256 | Frozen SHA-256 |
| --- | --- | --- |
| `projects/chirality-piping/core/solver/nonlinear_supports/src/lib.rs` | `f6c6e62994564dddfc84e60df63f2e2f3a293b1d48a5950469c4368509733062` | `956f64abfab50f16310641e2ffa63eccf37a3cb31ce48332fe7c14be28125d11` |
| `projects/chirality-piping/core/solver/nonlinear_integration/src/lib.rs` | `6e163a47db60288179f844d473992d28a53599ad2204149093186c760723ad46` | `d0b130fd8fe78c6eb18a824ebfec917bbff3f7865ceef9ccca1d332b17f228ce` |

Restricted `git diff --binary` SHA-256: `1246d391a5c362b3f92a57288fe3c03a51b2eb88f4e6d733dc90b27c3ee6404f`.

## Focused validation

- `nonlinear_supports`: 22/22 PASS.
- `nonlinear_integration`: 38/38 PASS.
- nonlinear benchmark crate: 19/19 PASS.
- product `friction_preview`: 3/3 PASS.
- product mixed nonlinear preview: 1/1 PASS.
- Rust formatting, restricted diff whitespace, and owned-path scope validation: PASS.

The tests bind sub-limit, exact representable boundary neighbors, super-limit, both signs, both initial states, dense/sparse modes, positive-contact `mu=0`, zero/negative contact, derived current-normal coupling, exact-zero derived contact, simultaneous mixed rows, branch retry, iteration caps, and both aggregate convergence gates. Planned mutation checks are exercised where represented by these tests; no unrun mutation case is claimed as executed.

## Evidence fan-in

- Current-SHA defect probe: `{EVALUATION_ROOT}/returns/NS/RUNTIME_PROBE.md`, SHA-256 `bb7f3d08e6c9caafa4af602a8226ffcbe37bf8384e95d176ff259a9b50e04378`.
- Independent design refutation: `{RUN_ROOT}/instances/FREFUTE/design/DESIGN_REFUTATION_V1.md`, SHA-256 `696cb9cc851993038709b75da01ed005d2b0c880b7518787c156f9ad7f57e74e`.
- Independent F4 reconciliation: `{RUN_ROOT}/instances/FREFUTE/design/F4_RECONCILIATION_V1.md`, SHA-256 `9e2828f9364d77693db0d68e6c6d695f197def9b7a3eb89b3668e031e2cd0c45`, verdict PASS.
- Independent derived-normal priority reconciliation: `{RUN_ROOT}/instances/FREFUTE/design/DERIVED_NORMAL_PRIORITY_RECONCILIATION_V1.md`, SHA-256 `b18d7d9ed148130a121024006489eb7bd230b4fbe6a18912150c9dd4b35c4e5d`, verdict PASS for the sealed amendment.
- Active normalized preparation pointer: `NORMALIZED_POINTER_V1.json`, SHA-256 `5bcd7f04e8f824848e7f5b15cbf7e4ddf43d9aa396ec0dd19c1d5da307e67db9`.
- Child implementation return: `implementation/RETURN.md`, active normalized SHA-256 `95febf9f25882077f8bd1be7f475779d700b73319551c5a56c158361fe153591`.
- Child validation: `implementation/VALIDATION.json`, active normalized SHA-256 `3e87ed13b58af1c2d2d892208f48218202ab1766b522e9e8c24d1d3b10536e56`.
- Terminal path-normalization proof: `implementation/RETURN_PATH_NORMALIZATION_V1.json`, SHA-256 `3da4a78dfc4b52f79732c88253386aa2275cc86d3217f09520d560fc556cb511`.
- Child status: `implementation/STATUS.json`, SHA-256 `8c4aac106b2dc7fde2727ed9a0fdff7a8d2f4e3527c8ebbb87fab7f67c0b92aa`.

No external solver comparison was run, and no external-validation claim is made. No Git mutation, public schema, tolerance, diagnostic code, friction history, lifecycle state, dependency state, DAG state, or decision state changed.

The source and Cargo slot are frozen. Root owns the fresh combined 100 percent diff review. Deliverable `DEL-04-04` memory/status closeout is authorized only after that review passes; it must replace stale unconditional anti-chatter prose while retaining the static/no-history limitations and other open physics items, with no lifecycle or DAG promotion.
