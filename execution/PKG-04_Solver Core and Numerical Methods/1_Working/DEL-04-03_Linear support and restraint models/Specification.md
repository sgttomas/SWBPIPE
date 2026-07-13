# Specification: DEL-04-03 Linear support and restraint models

## Scope

This deliverable provides the current linear support and restraint mechanics-boundary implementation for PKG-04. The implemented crate is `core/solver/linear_supports`; it covers anchors, guides, line stops, vertical supports, springs, and imposed displacement boundary data as listed in SOW-011.

Out of scope:

- Nonlinear active-set behavior such as one-way restraints, lift-off, gaps, and friction, assigned to DEL-04-04.
- Sparse solver selection, performance harness work, final result-envelope integration, and support coordinate policy beyond explicit node DOFs.
- Code compliance, certification, professional approval, or bundled protected standards/vendor values.

## Requirements

| ID | Requirement | Source |
|---|---|---|
| DEL-04-03-R01 | The support model shall cover the SOW-011 families: anchors, guides, line stops, vertical supports, springs, and imposed displacement boundary data. | `docs/_Registers/ScopeLedger.csv` row SOW-011 |
| DEL-04-03-R02 | Linear support data shall be expressible against the 3D centerline/frame model and the six node degrees of freedom through the frame-kernel `FrameDof` boundary. | `docs/CONTRACT.md` OPS-K-MECH-1; `docs/SPEC.md` section 4.1; `core/solver/linear_supports/src/lib.rs` |
| DEL-04-03-R03 | The support model shall keep mechanics solving separate from user-rule evaluation and human professional compliance judgment. | `docs/CONTRACT.md` OPS-K-MECH-2 |
| DEL-04-03-R04 | Support stiffnesses, imposed displacements, rotations, and related numerical data shall retain unit intent through `SupportQuantity`; caller-supplied unit metadata shall match the support quantity dimension. | `docs/CONTRACT.md` OPS-K-UNIT-1; `core/solver/linear_supports/src/lib.rs` |
| DEL-04-03-R05 | Missing solve-required support data shall be reported as explicit findings or diagnostics, never silently filled by defaults. | `docs/CONTRACT.md` OPS-K-DATA-2 |
| DEL-04-03-R06 | Public support examples and test fixtures shall avoid protected standards text, protected tables, copied formulas, proprietary vendor data, and unsupported default values. | `docs/CONTRACT.md` OPS-K-IP-1 |
| DEL-04-03-R07 | Support findings shall remain explicit at the crate boundary; mapping those findings into final AB-00-06 result-envelope fields remains downstream integration work. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-06 |
| DEL-04-03-R08 | The support model shall preserve module boundaries and inward dependencies toward domain contracts; adapters/plugins must not bypass validation, diagnostics, or governance. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-02 |
| DEL-04-03-R09 | Solver-facing support changes shall be covered by deterministic verification tests before release use. | `docs/CONTRACT.md` OPS-K-SOLVER-1; `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-08 |
| DEL-04-03-R10 | Unknown support coordinate policy, sparse solver integration points, final result-envelope integration, and release-readiness criteria shall remain `TBD` until resolved by a governed brief or human decision. | `docs/CONTRACT.md` OPS-K-AGENT-1; `_CONTEXT.md` Still TBD |
| DEL-04-03-R11 | Documentation shall not imply nonlinear support behavior, a penalty method, sparse solver integration, release approval, professional approval, or code-compliance acceptance. | Human hard stop; `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-01 |
| DEL-04-03-R12 | Boundary preparation and application shall use `prepare_boundary` and `apply_linear_supports` as the current implemented surfaces for dense-system support handling. | `core/solver/linear_supports/src/lib.rs`; `core/solver/linear_supports/README.md` |

## Standards

- Public project invariants in `docs/CONTRACT.md` apply, especially OPS-K-MECH-1, OPS-K-MECH-2, OPS-K-UNIT-1, OPS-K-SOLVER-1, OPS-K-DATA-2, OPS-K-AGENT-1..4, and OPS-K-IP-1.
- SCA-001 architecture basis IDs AB-00-01, AB-00-02, AB-00-03, AB-00-06, and AB-00-08 apply as dispatch constraints.
- External piping design-code clauses, support stiffness defaults, and vendor support catalogs are not accessible sources for this deliverable kit; any such requirement is `TBD` and must be user-supplied or rights-cleared before use.

## Verification

| Requirement | Verification approach / current evidence |
|---|---|
| R01, R02 | Implementation review confirms all SOW-011 support families are represented without leaving the six-DOF centerline model; `FrameDof` is re-exported from `open_pipe_stress_frame_kernel` and `NodeDof::global_index()` uses `node_dof_index`. |
| R03 | Result/status review confirms mechanics-solved state is not represented as rule-pass or professional approval. |
| R04 | Unit tests cover `SupportQuantity` finite/positive validation, canonical dimension IDs, caller-supplied unit metadata, and metadata dimension mismatch rejection. |
| R05, R07 | Negative tests omit required support data and verify explicit support findings; final result-envelope mapping remains deferred. |
| R06 | Protected-content/provenance review checks public examples and tests. |
| R08 | Architecture review checks dependency direction and no-bypass behavior. |
| R09 | June 5 evidence records `cargo test --manifest-path core/solver/linear_supports/Cargo.toml --locked` passing 14 tests. |
| R10 | Open policy and integration choices are tracked as `TBD` or decision-record inputs, not hidden assumptions. |
| R11 | Review confirms artifacts do not claim nonlinear behavior, release approval, professional approval, code compliance, sparse solver integration, or invented support stiffness/defaults. |
| R12 | Tests cover `prepare_boundary` output and `apply_linear_supports` spring diagonal application, imposed displacement force adjustment, rigid zero-displacement reduction, and blocking findings before reduction. |

## Documentation

The implemented evidence for this deliverable includes:

- `core/solver/linear_supports/README.md`;
- `core/solver/linear_supports/src/lib.rs`;
- deliverable-local run records for the May 15 and June 5 implementation slices;
- this four-document kit and `MEMORY.md`.

Decision records are still required for any later resolution of support coordinate policy, sparse solver integration, final result-envelope integration, release criteria, or other governed choices outside the current bounded implementation.

## D-41 R5 T7 PDU-054 current declaration

Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. The linear-support slice implements the recorded support/restraint families and DEC-049 hanger user data. Its current evidence and residuals are those named by the implemented crate and tests; no review, validation, or lifecycle ruling is made here.
