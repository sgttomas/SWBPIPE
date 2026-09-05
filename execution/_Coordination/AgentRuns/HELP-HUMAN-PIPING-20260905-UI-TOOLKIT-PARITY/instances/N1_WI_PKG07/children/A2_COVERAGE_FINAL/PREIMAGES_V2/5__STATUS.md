# Status: DEL-07-02 Model tree and property inspector

**Current State:** IN_PROGRESS
**Last Updated:** 2026-08-21

## Remaining
- Add GUI entry/emission of the model-level modulus_basis_records table (schema model.schema.yaml; per-load-case modulus_basis_ref selection plus bend_pipe_ref/mill-tolerance/equivalent-static entry landed via TP-PMM-GUIEMIT-001 PR #156 and applier acceptance TP-APP-R5-FIELDRULES-001 PR #162) (source: Receipt 10 delta / model.schema.yaml)
- Broaden app unit entry/pickers beyond the covered B-tail surfaces (residual hardening, select when it de-risks current-stage work) (source: PRD plan §3 Phase B-tail row / FR-002)
- Backfill unit tests and factoring for LoadCaseManagerPanel, PipeViewport, PropertyInspector (see also DEL-07-01) (source: PRD plan §3 hardening row H3 / seam plan §9.4)
- Complete broader canvas gestures and full model-tree/property-editor UX beyond landed selection/creation/drafting (source: PRD plan §4 FR-003/FR-013/FR-014 A3 residuals)
- Current toolkit adds rich support/hanger/nonlinear/family configuration, reference-guarded removal, explicit equipment/nozzle DOFs and real Rust-backed display readouts. Unsupported display dimensions retain entered values; drafts/model/evidence stay unchanged. Final N7 integrated review remains pending; these implementation records do not close broader UX, modulus-basis or independent validation residuals. See DEL-07-09 current coverage and routing records.

## History
- 2026-08-21 - Vocabulary round 3 completed Inspector creation forms for tee, reducer, valve, flange, and expansion joint. Required connectivity roles initialize/reset empty for non-bend kinds; expansion joints require explicit geometry, 4-axis stiffness, units, pressure-thrust/source references, and provenance. Fresh N1/N2 reviews passed after one preserved repair cycle each; commits `8ca1984db45a9a8f6f3111a905b07c7d3da47c33` and `d1a8e20ae413be040b82428cdd0bdbef0809e8de`. Rows 14 and 15 are closed; existing Remaining items and IN_PROGRESS lifecycle are unchanged.
- 2026-08-21 - Accepted the owner-amended N2 bend-only Property Inspector creation form and shared queued-target reservation evidence. Focused UI/build checks and fresh 100%-diff review passed; tee, reducer, valve, flange, and expansion-joint creation remain open. Existing Remaining items and lifecycle are unchanged.
- 2026-04-30 - State set to OPEN (PREPARATION)
- 2026-04-30 - State set to INITIALIZED (TASK+four-documents P1_P2)
- 2026-04-30 - State set/verified as SEMANTIC_READY (TASK+semantic-matrix-build)
- 2026-05-08 - State set to CHECKING by CHANGE-managed Tranche L closeout preparation using WORKING_TREE implementation evidence.
- 2026-05-09 - Evidence promoted to COMMITTED by CHANGE-managed Tranche L promotion using implementation commit 6e0b8f4.
- 2026-05-11 - TP-RECON-01 reconciled committed Tranche L evidence for commit 6e0b8f4 while preserving CHECKING state and downstream-gated scope.
- 2026-05-11 - Lifecycle correction: prior CHECKING state represented bounded implementation-evidence closeout, not full deliverable readiness; state reset to IN_PROGRESS pending further development. Human ruling recorded in `execution/_Reconciliation/LifecycleCorrection/LIFECYCLE_CORRECTION_2026-05-11_2052/Decision_Log.md`.
- 2026-06-06 - State set to CHECKING by human instruction after PKG-07 CHECKING-readiness fan-in recommended `MOVE_TO_CHECKING` for DEL-07-02; lifecycle advancement remains review-gate handoff only, not acceptance or issuance.
- 2026-07-02 - State set to IN_PROGRESS (affirmed; human K-CONFLICT-1 ruling, bridge Loop 2): records the 2026-06-16 header reversal from CHECKING that commit 28219696d left unlogged; ruling record at execution/_Reconciliation/LifecycleCorrection/LIFECYCLE_CORRECTION_2026-07-02_2050/Decision_Log.md.
- 2026-07-10 - Remaining section added: open scope rehomed from plans/ per owner-adopted consolidation (loop Receipt 12); no state change.
- 2026-07-11 - Remaining seeded with the gated D-41 concordance bootstrap item at packet time per plans/PLAN_2026-07-10_deliverable_implementation_reconciliation.md §4 (D-41 AWAITING_RULING; item inert until the ruling flips the gate suffix and supplies the pinned SHA; no state change).
- 2026-07-12 - D-41 R5 T1 applied DEC-074 O2: DEL-00-05 remains the GUI state/interaction architecture owner and DEL-07-02 is the model-tree/property-inspector behavior implementation owner within that architecture. O2 is landed; the D-41 program bootstrap remains in Remaining until applicable PDU-054/PDU-055 declaration-currentness updates are backchecked. Four genuine product/hardening residuals also remain. No lifecycle or functionality change.
- 2026-07-12 - D-41 R5 T7/PDU-054 reconciled 2 cited declaration claims to the live implemented slice, current authority, and surviving residuals; per-deliverable backcheck removed the exact D-41 bootstrap item. State remains IN_PROGRESS; no review, validation, issuance, or lifecycle ruling was made.
- 2026-07-16 - DEC-081 claims-language alignment applied to ScopeOfWork.md (D-48 Wave 2).
