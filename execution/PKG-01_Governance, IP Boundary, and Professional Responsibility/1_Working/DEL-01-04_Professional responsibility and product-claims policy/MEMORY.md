# MEMORY: DEL-01-04 Professional responsibility and product-claims policy

## Session 2026-05-01

Sealed dispatch brief:
`execution/_Coordination/DEV-001_DISPATCH_DEL-01-04.md`.

Authorized write targets used:

- `docs/PROFESSIONAL_BOUNDARY.md`
- `docs/report_notice_template.md`
- this `MEMORY.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

Decisions and constraints applied:

- Preserve the professional boundary from `OPS-K-AUTH-1`,
  `OPS-K-AUTH-2`, `OPS-K-MECH-2`, `OPS-K-REPORT-1`,
  `OPS-K-REPORT-2`, `OPS-K-GOV-3`, and `OPS-K-AGENT-*`.
- Keep OpenPipeStress report and product claims as decision-support language.
- Keep mechanics solve, user-rule check, and human professional acceptance as
  separate authority surfaces.
- Mark jurisdiction-specific professional-practice wording and exact
  acceptance-record workflow details as `TBD`.
- No lifecycle transition, blocker queue refresh, DAG edit, candidate-edge
  promotion, or dependency-register edit was authorized.

Artifacts drafted:

- `docs/PROFESSIONAL_BOUNDARY.md` defines permitted and prohibited claims,
  report notice requirements, hash-bound acceptance record constraints, and a
  product-claim review checklist.
- `docs/report_notice_template.md` provides baseline report notice language,
  report metadata slots, optional human acceptance notice language, and a
  review checklist.

Open TBDs:

- Jurisdiction-specific legal or professional-practice wording, if any.
- Exact storage and invalidation workflow for human acceptance records.
- Release-label vocabulary and final release policy language.

## 2026-05-11 TP-RECON-01 Reconciliation

Dispatch source: `plans/TP-RECON-01_DISPATCH_MATRIX.csv` row `DEL-01-04`
limits this reconciliation write scope to this `MEMORY.md` and `_STATUS.md`.

Reconciled evidence:

- Historical DEV-001 evidence records identify `DEL-01-04` as `COMMITTED`
  bounded documentation evidence at commit `65f3119` (`docs: add professional
  boundary policy`) on 2026-04-30, with later handoff reference `474b56d`.
- `git show --name-status 65f3119` shows added
  `docs/PROFESSIONAL_BOUNDARY.md`, `docs/report_notice_template.md`, and this
  `MEMORY.md`, plus an update to the coordination handoff state.
- The archived DEV-001 dispatch authorized the professional-boundary policy
  and report-notice template work, with explicit constraints against software
  reliance overclaims, protected-content reuse, lifecycle transition, blocker
  refresh, dependency-register edits, or candidate-edge changes.
- The SCA-002 targeted review records `DEL-01-04` as sufficient revision `0.5`
  graph-authoring predecessor evidence for `SOW-064` / `OBJ-018`; downstream
  comparison, handoff, external-prover, design-authoring, and agent-rationale
  controls remain owned by their deliverables.
- The revision `0.5` lifecycle and evidence projections record this
  deliverable as `CHECKING` with `COMMITTED` implementation evidence.
- Tranche B, E, and J proposal records reuse `65f3119` as upstream readiness
  evidence for validation/release gates, constraint/operation schemas,
  external-prover metadata, and agent professional-boundary controls.

Current artifact interpretation:

- `docs/PROFESSIONAL_BOUNDARY.md` and `docs/report_notice_template.md` are
  draft governance/report-notice surfaces supporting decision-support
  language, status separation, user-supplied rule/data boundaries, warning and
  provenance disclosure, hash-bound review references, and competent human
  review before project reliance.
- Deliverable-local semantic and lensing records retain open items for
  jurisdiction-specific wording, human-review record workflow, final release
  wording, and publication checklist detail.

Preserved boundaries:

- This reconciliation makes no code, schema, test, specification, procedure,
  dependency, aggregate DAG, blocker queue, evidence projection, or dispatch
  edits.
- No lifecycle promotion was made by the reconciliation. The later lifecycle
  correction recorded in `_STATUS.md` reset the current state to
  `IN_PROGRESS`.

## 2026-06-04 - TP-DEL-01-04-CURRENT-BASIS-REFRESH-001

- Human approved the proposed WORKING_ITEMS tranche to refresh DEL-01-04 to
  current basis within deliverable-local write scope.
- Updated active local context, references, datasheet, specification,
  guidance, procedure, semantic note, semantic-lensing notes, dependency CSV,
  and dependency summary to align with
  `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7` and approved
  `DAG-006`.
- Recorded current-basis scope/objective evidence:
  - `SOW-034` / `OBJ-011` remain the professional-responsibility boundary.
  - `SOW-064` / `OBJ-018` add the design-engine/product-claims boundary:
    OpenPipeStress may be framed as an analysis-grade design engine and
    stress-model authoring environment only while preserving the
    non-authoritative professional reliance boundary.
- Recorded current-basis dependency evidence:
  - Existing `DAG-002-*` architecture-basis rows were preserved as
    deliverable-local evidence and not promoted to aggregate authority.
  - Historical `DAG-002-E0392` remains retired and was not reactivated.
  - `DEP-DEL-01-04-E003` and `DEP-DEL-01-04-E004` now record the existing
    draft repo-level artifacts `docs/PROFESSIONAL_BOUNDARY.md` and
    `docs/report_notice_template.md` as satisfied handoff evidence.
  - `DEP-DEL-01-04-E006` and `DEP-DEL-01-04-E007` preserve pending
    human/legal/professional review and human project-authority acceptance.
- Preserved unresolved decisions as `TBD`: jurisdiction-specific
  professional-practice wording, exact human-acceptance record
  storage/invalidation workflow, release-label vocabulary, final release policy
  language, legal-review authority, and final acceptance or revision of the
  draft policy/template text.
- Boundary preserved: no lifecycle state edit, aggregate DAG edit,
  candidate-edge promotion, repo-level policy/template edit, legal conclusion,
  professional approval, certification, sealing, authentication,
  code-compliance claim, release claim, or issued-policy claim.
- Run record:
  `_run_records/WORKING_ITEMS_RUN_2026-06-04_TP-DEL-01-04-CURRENT-BASIS-REFRESH-001.md`.

## 2026-06-04 - TP-AUTHORITY-REFRESH-0_7-DAG006

- WORKING_ITEMS/TASK workforce current-authority refresh applied to active deliverable-local surfaces for `DEL-01-04`.
- Current authority basis is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7` plus approved `execution/_DAG/DAG-006/` active graph authority.
- Historical run records, historical DAG row IDs, review dispositions, lifecycle `_STATUS.md`, aggregate DAG artifacts, candidate edges, repo-level governance files, schemas, code, and tools were intentionally not changed by this refresh.
- Preserved historical references remain evidence of earlier work, not current authority claims.
