# WORKING_ITEMS RUN — 2026-06-10 — Apply-operation command path (TP-APP-R2-EDITLOOP-001)

- **Tranche:** `TP-APP-R2-EDITLOOP-001` — completion plan Phase A1
  (`plans/PLAN_2026-06-10_prd_completion.md` §3), the first item on the
  R2 dependency spine A1 → A2 → A5.
- **Run by:** WORKING_ITEMS (Type 1 persona) working locally inside the
  sealed app tranche; two parallel TASK workers produced the D-01 and D-08
  decision packets in a disjoint write scope
  (`execution/_Coordination/_DECISIONS/`).
- **Authority basis:** `SOFTWARE_DECOMP` revision 0.7 + approved `DAG-006`;
  DEL-16-01/16-02/16-03 contracts as mature design authority (`CHECKING`);
  PKG-16 Python engines (`core/model_operations/validation_preview/engine.py`,
  `audit_trail/engine.py`) as the semantic reference. This run does not
  change lifecycle state.

## What landed

1. **New Rust crate `core/model_operations/operation_applier`**
   (`open_pipe_stress_operation_applier`): validate → diff-preview → apply
   engine for the desktop app's structured `EditorOperationIntent` records
   against a model document. Behavior: never mutates the input model;
   blocking findings instead of silent fallbacks (`OP-STALE-BEFORE-VALUE`,
   `OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE`, `OP-UNIT-DIMENSION-UNKNOWN`,
   `OP-TARGET-NOT-FOUND`, `OP-REFERENCE-NOT-FOUND`,
   `OP-RESTRAINT-TOKEN-INVALID`, `OP-VALUE-NOT-NUMERIC`,
   `OP-VALUE-NOT-POSITIVE`, `OP-QUANTITY-OBJECT-MISSING`,
   `OP-GEOMETRY-INPUT-INCOMPLETE`, `OP-FIELD-EDIT-DEFERRED`,
   `OP-DIRECT-MUTATION-BLOCKED`, …); apply returns a *new* model document
   plus an acceptance receipt limited to
   `user_initiated_apply_in_local_session` with
   `acceptance_is_professional_approval=false`.
2. **Two new Tauri commands** in `apps/desktop/src-tauri/src/lib.rs`:
   `validate_model_operation`, `apply_model_operation` (thin wrappers over
   the crate; first mutating-model command path in the app).
3. **Frontend seam:** `apps/desktop/src/services/operationService.ts`
   (Tauri route + honestly labeled browser-fixture local engine with the
   same semantics), new `OperationApplyPanel` (validate/apply per queued
   intent, outcome diagnostics, applied-operation session receipts), App
   wiring that replaces the session model on apply, clears stale solve
   results/proposals, and resets the solve-job audit with a model-changed
   event. Save persists the edited model through the existing envelope path.
4. **Honesty fixes found during the run:** node `position.*` quantities
   resolve their unit basis from `project.units.length` (sibling-unit
   assumption was wrong in both engines; the masked Rust determinism test
   now asserts an applied outcome); toolbar review context reports
   `applied_operations=N` instead of hardcoded `applied=false`.

## Scope notes (visible limits, not silent gaps)

- Apply path covers inspector modify intents (`set_field`, `update_load`,
  `update_support`). Viewport gesture intents (`create_node`,
  `connect_pipe_run`, `insert_component_symbol`) validate to a blocking
  `OP-GEOMETRY-INPUT-INCOMPLETE` finding — explicit geometry capture is
  completion-plan A3; no values are invented.
- `Component.kind`, `Load.status`, `Load.kind`, `Combination.basis`,
  `Combination.terms` return `OP-FIELD-EDIT-DEFERRED` (A4 / later editors).
- No unit conversion (Phase B, decision D-01). Same-unit edits only.
- Applied receipts are session-state only pending A2 model-document
  persistence (decision D-08).
- Cross-language hash canonicalization differs (serde_json vs ECMAScript
  number formatting), so UI-claimed model hashes are echoed beside
  backend-computed hashes with
  `binding_status=claimed_hash_echoed_cross_canonicalization_equality_not_evaluated`;
  field-level staleness is guarded by numeric before-value checks. True
  RFC 8785 alignment is a D-08-adjacent follow-up.
- Vocabulary note: `CANONICAL_DIMENSIONS` in the crate is the accepted
  PKG-02 30-id set (includes `force_per_length`); the DEL-16-02 Python
  engine's set omitted `force_per_length` — discrepancy surfaced in the D-01
  decision packet (`execution/_Coordination/_DECISIONS/D-01_unit_catalog_acceptance.md` §2/§7).
  During this session the human project authority reconciled the Python
  sets in-tree (`core/model_operations/validation_preview/engine.py`,
  `core/constraints/validation/engine.py`, `core/gui/pkg02_boundary.py`)
  and added a parity test pinning all three to the
  `schemas/units.schema.yaml` `DimensionId` enum
  (`tests/test_operation_validation_preview.py`); those human edits are
  included in this tranche's commit and covered by the 342/342 pytest run.

## Evidence

- `cargo test` `core/model_operations/operation_applier`: **19/19**.
- `cargo test` `apps/desktop/src-tauri`: **14/14** (two new command tests:
  fixture-model apply + re-solve through `run_preview_mechanics`; stale
  intent blocked without mutation).
- `npm test --workspace apps/desktop`: **21/21** (new: operationService
  engine suite; App R2 edit-loop journey edit → apply → stale-block →
  re-solve → save; viewport-gesture blocking).
- `npm run build --workspace apps/desktop`: pass.
- `python3 -m pytest -q tests`: **342/342** (no Python surfaces changed).
- Browser smoke: `apps/desktop/SMOKE.md` → **TP-MAC-82** (live preview:
  queue → validate → apply → re-solve 647 rows → save; receipts, cleared
  results, console clean).

## Boundary review

Local-only execution; no network, daemon, telemetry, or repository-default
private-data writes; bundled fixtures remain invented; user edits live in
session state and the local project store only and are not committed to the
repository; software-emitted statuses stay within the accepted vocabulary;
receipts and panel copy make no release, professional, certification,
sealing, authentication, approval, or code-compliance claims. Git closeout
is source-control hygiene only and is not lifecycle issuance.

## Affected surfaces

`core/model_operations/operation_applier/**` (new),
`apps/desktop/src-tauri/{Cargo.toml,src/lib.rs}`,
`apps/desktop/src/{App.tsx,App.test.tsx,types.ts,styles.css}`,
`apps/desktop/src/services/{operationService.ts,operationService.test.ts}`,
`apps/desktop/src/features/operations/{OperationApplyPanel.tsx,OperationLedgerPanel.tsx}`,
`apps/desktop/src/features/validation-evidence/ValidationEvidencePanel.tsx`,
`apps/desktop/SMOKE.md`, this run record, `MEMORY.md` (this deliverable),
plus coordination fan-in: decision packets D-01/D-08 (TASK workers),
`_DECISIONS/_REGISTER.md`, completion-plan row updates.
