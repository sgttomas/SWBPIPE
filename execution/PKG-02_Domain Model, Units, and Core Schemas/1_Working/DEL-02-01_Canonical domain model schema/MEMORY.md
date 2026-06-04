# MEMORY: DEL-02-01 Canonical domain model schema

## Session 2026-04-30

Human ruling consumed:

- Accepted the completed `DEL-01-01` pilot pattern.
- Authorized exactly one next bounded DAG item: `DEL-02-01`.
- Explicitly prohibited broad fan-out.

Bounded write scope used:

- `schemas/model.schema.yaml`
- `docs/TYPES.md`
- `tests/test_model_schema.py`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-02-01.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`
- `execution/_Coordination/_COORDINATION.md`
- this `MEMORY.md`

Work notes:

- Preserved JSON Schema 2020-12 and strict JSON syntax in `schemas/model.schema.yaml`.
- Tightened canonical schema requirements for model collections, unit-bearing quantities, diagnostics, hash payload scope, result/report evidence, rule-pack references, support/load provenance, and common object coverage.
- Updated `docs/TYPES.md` Section 8 with schema syntax and common-record vocabulary.
- Added focused stdlib checks in `tests/test_model_schema.py`.

Guardrails preserved:

- No lifecycle state transition.
- No `Dependencies.csv`, `_DEPENDENCIES.md`, `DAG-001`, candidate-edge, or blocker-queue edits.
- No protected standards text, protected tables, proprietary values, private data, or automatic compliance/certification/sealing claims introduced.

Open items:

- `C-02-01-001` objective mapping remains unresolved: `DEL-02-01` owns `OBJ-001`; `SOW-041` also maps `OBJ-012` in the scope ledger.
- `C-02-01-002` stale metadata pointer remains unresolved in local references.
- Physical project container and migration framework remain owned by persistence work unless later human authority changes that boundary.

## Session 2026-05-03 - Revision 0.5 Supplement

Human rulings consumed:

- `DEL-02-01` supplemental revision `0.5` schema/context work is required
  before `DAG-002` graph approval.
- Execute the supplement now as bounded `DEL-02-01` work.

Bounded write scope used:

- `schemas/model.schema.yaml`
- `tests/test_model_schema.py`
- `docs/TYPES.md`
- `docs/SPEC.md`
- this deliverable `_CONTEXT.md`
- this `MEMORY.md`
- coordination/DAG approval-review state needed to close the control loop

Work notes:

- Refreshed `_CONTEXT.md` to revision `0.5`, adding `SOW-065`, `OBJ-012`, and
  `OBJ-014`.
- Extended the canonical `Model` schema with a `model_role`, unresolved
  assumptions, traceability links, and typed references to design knowledge,
  constraints, equipment interfaces, operations, states/runs, comparisons,
  handoff packages, and external-prover metadata.
- Added `Assumption`, `ModelRole`, and `TraceabilityLink` definitions while
  leaving detailed PKG-13 through PKG-16 contracts to their owning deliverables.
- Updated `docs/TYPES.md`, `docs/SPEC.md`, and schema tests for the revision
  `0.5` vocabulary.

Guardrails preserved:

- No lifecycle state transition.
- No `Dependencies.csv`, `_DEPENDENCIES.md`, blocker queue, or graph approval
  record edits.
- No Type 2 dispatch, `PREPARATION` scaffold, or Chirality corpus promotion.
- No protected standards text, protected tables, proprietary values, private
  data, public code-specific defaults, automatic compliance/certification/
  sealing claims, or software-generated professional approval claims
  introduced.

Open items:

- Downstream PKG-13 through PKG-16 deliverables still own detailed schemas and
  services for design knowledge, constraints, states/runs, comparisons,
  handoff, external-prover metadata, and structured model operations.
- Physical project container and migration implementation details remain owned
  by persistence work unless later human authority changes that boundary.

## 2026-05-11 TP-RECON-01 Reconciliation

Sources reconciled:

- `plans/TP-RECON-01_DISPATCH_MATRIX.csv` row `DEL-02-01`.
- Current DEL files and `_run_records/` for `DEL-02-01`.
- Archived evidence bundle listed by the dispatch row, including
  `DEV-001_IMPLEMENTATION_EVIDENCE.csv`,
  `DEV-001_REV05_IMPLEMENTATION_EVIDENCE_STATUS.csv`,
  `REV05_LIFECYCLE_STATE_SNAPSHOT.csv`,
  `SCA-002_REV05_TARGETED_REVIEW_DEL-01-04_DEL-02-01.md`,
  `DEV-001_DISPATCH_DEL-02-01.md`, tranche E/F/G proposal references, DAG-002
  pre-gate/proposal records, and TP-MAC-01-C planning records.
- `git show --name-status 7b256f3`.

Reconciled evidence:

- Historical committed implementation evidence maps `DEL-02-01` to commit
  `7b256f3` (`schema: tighten canonical domain model contract`) with handoff
  commit `8f57f85`; `git show --name-status 7b256f3` shows changes to
  `schemas/model.schema.yaml`, `docs/TYPES.md`, `tests/test_model_schema.py`,
  this deliverable `MEMORY.md`, and bounded coordination state/dispatch files.
- The revision `0.5` evidence status row maps this deliverable to
  `SOW-041,SOW-065` and `OBJ-001,OBJ-012,OBJ-014`, with historical evidence
  accepted for graph-authoring reliance only after targeted review and required
  supplement work.
- The lifecycle snapshot records `DEL-02-01` as `CHECKING`, with present
  context, dependency, status, and local dependency-register surfaces, and notes
  `REV05_TARGETED_REVIEW_AND_SUPPLEMENT_COMPLETE`.
- Current DEL records preserve the JSON Schema 2020-12 baseline, unit-aware
  quantities, provenance/status records, diagnostics/result/report envelope
  compatibility, public/private data boundary, and no software-generated
  professional reliance state.
- SCA-002 targeted review records `DEL-02-01` as a foundational predecessor for
  downstream PKG-13 through PKG-16, `DEL-07-08`, and `DEL-08-06` surfaces while
  leaving those detailed contracts to their owning deliverables.
- TP-MAC-01-C treats `DEL-02-01` as the model-entity and physical-model
  source-of-truth contract for a future local application-service bridge, but
  explicitly does not authorize schema edits or product/release reliance from
  this reconciliation.

State and boundaries:

- Current state remains `CHECKING`; no final lifecycle transition or release
  readiness claim is made here.
- Open ownership boundaries remain: detailed PKG-13 through PKG-16 contracts,
  physical project packaging, migration implementation, schema `$id`,
  code-generation tooling, fixture organization, and unresolved human-ruling
  items stay deferred to their owning gates.

## 2026-05-16 PKG-02 Foundation-Slice Hardening

Scope executed:

- Converted `tests/test_model_schema.py` from script-only assertions into
  pytest-collected contract and fixture tests while preserving direct
  `python3` execution.
- Added invented public fixtures under `fixtures/domain/` for a minimal
  project/model envelope and a physical source-of-truth model.

Evidence:

- Pytest now collects and passes `tests/test_model_schema.py`.
- Direct execution with `python3 tests/test_model_schema.py` passes.
- Fixture checks cover project/model envelope fields, unit-bearing quantities,
  diagnostics, provenance, traceability, unresolved assumptions, hashes, and
  forbidden professional/compliance language.

Boundaries preserved:

- Lifecycle remains `IN_PROGRESS`; no lifecycle transition was made.
- No dependency register, DAG, blocker queue, or candidate-edge edits.
- No schema-domain expansion beyond the existing DEL-02-01 context.

## 2026-05-17 TP-PHYS-014-A Schema Orientation And Load Contract

Scope executed:

- Added canonical `force_per_length` to `schemas/model.schema.yaml` and shared
  docs/tests.
- Replaced the prior generic `LoadRecord` shape with strict typed records for
  `nodal_force`, `nodal_moment`, `element_point_force`, and
  `element_uniform_distributed_force`.
- Added governed straight-pipe orientation shape: `local_coordinate_system`
  can carry `y_reference` and provenance, and straight-pipe elements require
  `y_reference`.
- Kept `local_x_axis` out of the canonical schema.

Evidence:

- `python3 tests/test_model_schema.py` passed.
- `python3 tests/test_units_schema.py` passed.
- `python3 -m pytest tests/test_model_schema.py tests/test_units_schema.py`
  passed with 7 tests.

Boundary note:

- JSON Schema 2020-12 enforces dimensionless span fractions in the range
  `0 <= value <= 1`; the sibling comparison `start_fraction.value <
  end_fraction.value` still needs a runtime semantic check or nonstandard JSON
  Schema extension.

## 2026-06-03 - TP-CODE-EVIDENCE-MIGRATION-RECONCILIATION-001 CHECKING transition
- Human approval accepted non-resolving DEV-001 evidence commits as migration-caused aberrations and approved lifecycle advancement to `CHECKING` for formal review.
- Evidence basis: `TP-CODE-EVIDENCE-AUDIT-001_2026-06-03` found current source/schema/fixture/test evidence and passing targeted/full-gate checks; `TP-CODE-EVIDENCE-MIGRATION-RECONCILIATION-001_2026-06-03` reconciled the migration-era commit-pointer gap.
- Local `_STATUS.md`, DEV-001 blocker queue lifecycle displays, and DAG-005 deliverable display surfaces were aligned to `CHECKING` where applicable.
- Boundary preserved: this is review-readiness only; no `ISSUED`, release-readiness, external compatibility, code-compliance, protected-IP/private-data, or professional-engineering authentication claim is made.

## 2026-06-04 - TP-AUTHORITY-REFRESH-0_7-DAG006

- WORKING_ITEMS/TASK workforce current-authority refresh applied to active deliverable-local surfaces for `DEL-02-01`.
- Current authority basis is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7` plus approved `execution/_DAG/DAG-006/` active graph authority.
- Historical run records, historical DAG row IDs, review dispositions, lifecycle `_STATUS.md`, aggregate DAG artifacts, candidate edges, repo-level governance files, schemas, code, and tools were intentionally not changed by this refresh.
- Preserved historical references remain evidence of earlier work, not current authority claims.
