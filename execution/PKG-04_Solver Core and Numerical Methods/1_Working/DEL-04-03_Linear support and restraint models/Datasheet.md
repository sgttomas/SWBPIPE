# Datasheet: DEL-04-03 Linear support and restraint models

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-04-03 |
| Package ID | PKG-04 |
| Package name | Solver Core and Numerical Methods |
| Type | BACKEND_FEATURE_SLICE |
| Scope item | SOW-011 |
| Objective | OBJ-003 |
| Context envelope | M |
| Anticipated artifacts | support model; linear restraint tests |

## Attributes

| Attribute | Current value | Source |
|---|---|---|
| Covered support families | Anchors, guides, line stops, vertical supports, springs, and imposed displacement boundary data. | `_CONTEXT.md` Description; `docs/_Registers/ScopeLedger.csv` row SOW-011 |
| Analysis basis | Support behavior is part of the primary 3D centerline/frame global model with six degrees of freedom per node. | `docs/CONTRACT.md` OPS-K-MECH-1; `docs/SPEC.md` section 4.1 |
| Support implementation surface | `core/solver/linear_supports` implements the bounded mechanics-boundary slice for linear support records, prepared boundary data, and dense-system support application. | `core/solver/linear_supports/README.md`; `core/solver/linear_supports/src/lib.rs` |
| Frame DOF boundary | `FrameDof` is re-exported from `open_pipe_stress_frame_kernel`; `NodeDof::global_index()` delegates to frame-kernel `node_dof_index`. | `core/solver/linear_supports/src/lib.rs`; `_run_records/TASK_RUN_2026-06-05_0732_TP-DEL-04-03-04-06_SUPPORT-BOUNDARY-HARDENING_A.md` |
| Linear-only boundary | This deliverable covers implemented linear support/restraint behavior. Nonlinear one-way behavior, lift-off, gaps, and friction belong to DEL-04-04 unless explicitly deferred back by human authority. | `execution/_Decomposition/SOFTWARE_DECOMP.md` rows DEL-04-03 and DEL-04-04 |
| Unit metadata | `SupportQuantity` retains dimension intent and can carry caller-supplied `UnitSystemRef` and `QuantityUnitMetadata`; it validates metadata dimension matches and does not convert or default values. | `core/solver/linear_supports/src/lib.rs`; `core/solver/linear_supports/README.md` |
| Imposed displacement boundary data | Imposed displacement entries are prepared as prescribed DOFs with explicit displacement or rotation values for the frame-kernel prescribed-displacement boundary. | `core/solver/linear_supports/src/lib.rs` |
| Default stiffness values | TBD. No invented support stiffness/defaults are introduced by the implementation or this document kit. | `docs/CONTRACT.md` OPS-K-AGENT-1; human hard stop |

## Conditions

- Public artifacts must not include protected standards text, protected tables, copied formulas, material allowables, SIF/flexibility tables, protected dimensional tables, or proprietary support/vendor data. Source: `docs/CONTRACT.md` OPS-K-IP-1.
- Missing solve-required support data must become explicit findings or diagnostics, not silent defaults. Source: `docs/CONTRACT.md` OPS-K-DATA-2.
- Numerical values affecting calculations must be unit-aware and dimensionally checked. Source: `docs/CONTRACT.md` OPS-K-UNIT-1.
- Solver output remains a mechanics result; rule-pack acceptability and professional compliance remain outside this deliverable. Source: `docs/CONTRACT.md` OPS-K-MECH-2.
- Solver changes require deterministic verification tests before release. Source: `docs/CONTRACT.md` OPS-K-SOLVER-1.

## Construction

| Construction slot | Current state |
|---|---|
| Support model | Implemented Rust types cover rigid restraints, springs, and imposed displacement boundary data for the SOW-011 families. The crate remains mechanics-only and linear-only. |
| DOF mapping | Support effects map to frame-kernel `FrameDof` values `Ux`, `Uy`, `Uz`, `Rx`, `Ry`, and `Rz`; global indices are computed through `node_dof_index`. Support coordinate policy beyond explicit node DOFs remains TBD. |
| Boundary preparation | `prepare_boundary` produces restrained DOFs, spring entries, imposed displacement entries, and deterministic support findings for missing, invalid, duplicate, or out-of-range support data. |
| Boundary application | `apply_linear_supports` validates a dense global system through the frame kernel, adds prepared spring stiffness to global stiffness diagonals, and reduces rigid plus imposed displacement DOFs through the frame-kernel prescribed-displacement boundary. Sparse solver integration and final result-envelope integration remain downstream. |
| Diagnostics | Current crate findings include local support finding code, support ID, and message. AB-00-06 result-envelope mapping remains downstream integration work. |
| Tests | Current evidence records `cargo test --manifest-path core/solver/linear_supports/Cargo.toml --locked` passing 14 tests, including frame-kernel DOF parity, unit metadata, boundary preparation, and boundary application coverage. |

## References

- `_CONTEXT.md` for sealed deliverable identity and applicable architecture basis IDs.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 for SOW-011, OBJ-003, PKG-04, DEL-04-03, and AB-00-01/02/03/06/08.
- `docs/_Registers/Deliverables.csv`, `ScopeLedger.csv`, and `ContextBudgetQA.csv` for register rows.
- `docs/CONTRACT.md` for applicable invariants.
- `docs/SPEC.md`, `docs/TYPES.md`, and `docs/INTENT.md` for public architecture and vocabulary slices.

## D-41 R5 T7 PDU-054 current declaration

Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. The linear-support slice implements the recorded support/restraint families and DEC-049 hanger user data. Its current evidence and residuals are those named by the implemented crate and tests; no review, validation, or lifecycle ruling is made here.
