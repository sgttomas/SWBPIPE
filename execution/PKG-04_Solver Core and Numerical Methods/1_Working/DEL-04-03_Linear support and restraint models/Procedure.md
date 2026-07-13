# Procedure: DEL-04-03 Linear support and restraint models

## Purpose

Use this procedure to verify DEL-04-03 review-readiness evidence for the implemented linear support/restraint slice without expanding scope into nonlinear supports, sparse solver integration, final result-envelope integration, release approval, or professional/code-compliance claims.

## Prerequisites

- Confirm the active scope is DEL-04-03 / PKG-04 and SOW-011 only.
- Read `_CONTEXT.md`, `_REFERENCES.md`, `docs/CONTRACT.md`, and the SOFTWARE_DECOMP rows for SOW-011, OBJ-003, DEL-04-03, DEL-04-04, and AB-00-01/02/03/06/08.
- Confirm protected data and professional-compliance hard stops before drafting support examples or tests.
- Confirm the current implementation evidence in `core/solver/linear_supports/README.md`, `core/solver/linear_supports/src/lib.rs`, and the June 5 support-boundary-hardening run record.
- Confirm any support coordinate policy, sparse solver integration, final result-envelope integration, release-readiness criterion, or professional/code-compliance conclusion is either already accepted by a governed decision or marked `TBD`.

## Steps

1. Confirm the support family taxonomy in `LinearSupport` covers anchors, guides, line stops, vertical supports, springs, and imposed displacement boundary data.
2. Confirm `FrameDof` is re-exported from `open_pipe_stress_frame_kernel` and `NodeDof::global_index()` delegates to frame-kernel `node_dof_index`.
3. Confirm each support family maps to explicit node DOFs and that support coordinate policy beyond explicit DOF selection remains `TBD`.
4. Confirm `SupportQuantity` preserves value, dimension, optional unit-system reference, and optional unit metadata; verify dimension mismatch is rejected rather than converted or defaulted.
5. Confirm `prepare_boundary` returns restrained DOFs, spring entries, imposed displacement entries, and deterministic support findings for missing, invalid, duplicate, or out-of-range data.
6. Confirm `apply_linear_supports` validates a dense global system through the frame kernel, adds spring stiffness to global diagonals, and reduces rigid plus imposed displacement DOFs through the frame-kernel prescribed-displacement boundary.
7. Confirm current local evidence records 14 passing `linear_supports` tests and does not rely on protected support catalog values, code-derived defaults, or copied commercial benchmarks.
8. Confirm nonlinear one-way, gap, lift-off, friction, active-set behavior, sparse solver integration, final result-envelope integration, support coordinate policy, release claims, and professional/code-compliance claims remain outside this deliverable alignment.
9. Record any later resolved architecture or representation choices through the approved decision-record path. The exact repo-level ADR location is outside this task and remains `TBD`.

## Verification

- Four-document kit exists and uses consistent terminology for linear support/restraint models.
- Required support-data gaps are represented as `TBD` or support findings, not defaults.
- Current test evidence references deterministic mechanics verification and unit/dimensional checks for support stiffness and imposed displacement values.
- Implementation evidence includes `prepare_boundary`, `apply_linear_supports`, frame-kernel `FrameDof` re-export, `node_dof_index` indexing, `SupportQuantity` unit metadata, and 14 passing `linear_supports` tests.
- No protected standards text, copied formulas, protected tables, proprietary vendor data, or certification/compliance claims are introduced.
- Nonlinear support behavior is not implemented or specified as part of this linear deliverable.

## Records

- `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md`.
- `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` historical semantic artifacts.
- `Dependencies.csv` v3.1 and `_DEPENDENCIES.md`.
- `_run_records/TASK_RUN_*.md` entries for four-documents P1/P2, semantic matrix, lens register, four-documents P3, dependency extraction, implementation evidence, support boundary hardening, and this review-readiness alignment.

## D-41 R5 T7 PDU-054 current declaration

Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. The linear-support slice implements the recorded support/restraint families and DEC-049 hanger user data. Its current evidence and residuals are those named by the implemented crate and tests; no review, validation, or lifecycle ruling is made here.
