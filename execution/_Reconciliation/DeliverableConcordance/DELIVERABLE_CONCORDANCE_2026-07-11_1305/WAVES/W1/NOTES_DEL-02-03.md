# Notes — DEL-02-03 Code-neutral analysis boundary model (W1)

Run: DELIVERABLE_CONCORDANCE_2026-07-11_1305, R2 wave W1. Frozen tree
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Binding set: `R1_CONVENTIONS.md`
(conventions 1-8 + addenda 1-13). Ledger: `CLAIM_CONCORDANCE_DEL-02-03.csv`
(24 rows). Lifecycle at frozen tree: IN_PROGRESS.

This deliverable is F-PIP-2 fence-adjacent: code-neutrality means no
code-compliance determinations. Every ledger cell that touches
compliance/certification language **quotes** the surface (schema field names,
requirement text, doc prose) rather than asserting any compliance status. No
row certifies, clears, or determines compliance of any surface.

## NormativeSource path alias (declared once per addendum 12)

`KIT/` = `execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-03_Code-neutral analysis boundary model/`
(the deliverable folder in the frozen tree). Repo-relative product surfaces
(`schemas/...`, `docs/...`, `tests/...`, `fixtures/...`) are written from the
`projects/chirality-piping/` working root.

## Requirement-ID scheme mapping (self-identifying -> addendum-12 ClaimID)

Requirement scheme `DEL-02-03-R*` is self-identifying. Addendum-12 ClaimID form
`<DEL-ID>-<TYPE>-NNN` is applied; the mapping is 1:1:

| Requirement ID | ClaimID |
|---|---|
| DEL-02-03-R01 | DEL-02-03-REQ-001 |
| DEL-02-03-R02 | DEL-02-03-REQ-002 |
| DEL-02-03-R03 | DEL-02-03-REQ-003 |
| DEL-02-03-R04 | DEL-02-03-REQ-004 |
| DEL-02-03-R05 | DEL-02-03-REQ-005 |
| DEL-02-03-R06 | DEL-02-03-REQ-006 |
| DEL-02-03-R07 | DEL-02-03-REQ-007 |
| DEL-02-03-R08 | DEL-02-03-REQ-008 |
| DEL-02-03-R09 | DEL-02-03-REQ-009 |
| DEL-02-03-R10 | DEL-02-03-REQ-010 |
| DEL-02-03-R11 | DEL-02-03-REQ-011 |
| DEL-02-03-R12 | DEL-02-03-REQ-012 |

EXC/DECL ClaimIDs are run-local (no source IDs); numbered NNN from 001 within
each TYPE. EXC-001..006 map to the six `Specification.md` "Out of scope"
bullets in listed order. DECL-001..006 map to Specification.md, Datasheet.md,
Guidance.md, Procedure.md, _STATUS.md, MEMORY.md respectively.

## Disposition histogram (reproduces from CSV column 17)

- ALIGNED — 22
- STALE_SETUP_SPECIFICATION — 2

Total 24.

## ClaimType histogram (reproduces from CSV column 4)

- REQUIREMENT — 12
- EXCLUSION — 6
- DECLARED_STATE — 6

Total 24.

## Census decisions

- **Requirements (12):** one row per current requirement ID R01-R12
  (substance disposition per convention 1; never STALE). R12 is a
  Specification requirement labeled `ASSUMPTION`; it is still a current
  requirement ID and takes a REQUIREMENT row with substance disposition.
- **Acceptance (0):** the `Specification.md` Verification table (V01-V11) and
  the per-status "Acceptance check" table restate the R01-R12 obligations as
  verification checks rather than introducing independent acceptance criteria
  at addendum-12 grain. Per the brief, verification tables that merely restate
  requirements do NOT get mirrored ACCEPTANCE rows. Zero acceptance rows.
  (Judgment call — self-flagged below.)
- **Exclusions (6):** one row per distinct `Specification.md` "Out of scope"
  bullet. Kept per-bullet (not consolidated) because the code-neutrality and
  protected-data exclusions are exactly this deliverable's F-PIP-2
  fence-adjacent boundary and each is separately evidenced.
- **Declared-state (6):** four-document kit (Specification, Datasheet,
  Guidance, Procedure) + `_STATUS.md` + `MEMORY.md` per addendum 1. No
  deliverable-owned in-tree README exists in the folder, so no README
  declared-state row (see boundary note below re: the DEL-04-01 R0 pilot).
- **Remaining-work (0):** the only `_STATUS.md` `## Remaining` entry is the
  seeded `(gated: D-41)` bootstrap item. Per addendum 2 it is recorded
  verbatim only in the `_STATUS.md` surface row's `RecordedRemaining`
  (DEL-02-03-DECL-005), gets no own row, and is excluded from all
  residual/gate/selectability analysis. No non-bootstrap residual exists
  (`DELIVERABLE_INVENTORY.csv` NonBootstrapItems=NONE). The implementation-level
  TBDs in Datasheet/MEMORY/arch-doc are the accepted substance of R11/R12
  (design deferrals), not `_STATUS.md` work-discovery residuals, so they are
  captured on the R11/R12 rows and the declared-state rows, not as
  REMAINING_WORK claim rows.
- **Implemented-unmapped (0):** the material surfaces in this deliverable's
  orbit — `schemas/analysis_boundary.schema.yaml` (SURF-174),
  `docs/architecture/code_neutral_analysis_boundary.md`,
  `tests/test_analysis_boundary_schema.py` (PY-09),
  `fixtures/analysis_boundary/` (SURF-146) — are all attributed to DEL-02-03,
  i.e. mapped. `IMPLEMENTED_UNMAPPED` is for surfaces with no deliverable
  mapping; none qualify. Note `schemas/analysis_status.schema.yaml` (SURF-176)
  is present in the DEL-02-03 surface attribution only because MEMORY.md names
  it; MEMORY.md 2026-05-16 explicitly scopes that file to **DEL-05-04**, so it
  is out of this deliverable's orbit and gets no row here.

## Boundary-compliance statement

- Fences held. Discovery was READ-ONLY outside the two W1 output files. No
  lifecycle transition applied; no DAG mutation; no cross-project edit; no
  edit to any `_STATUS.md`, register, or product file.
- F-PIP-1..5 respected: no row asserts release-readiness, issuance,
  certification, sealing, professional-approval, or code-compliance status.
  Where requirements/exclusions concern code-neutrality (R02, EXC-005), the
  cells quote the schema's own `forbidden_software_claims` /
  `professional_boundary` fields and the requirement text; they do not
  themselves determine compliance of anything.
- Re: the DEL-04-01 R0 pilot's cited "DEL-02-03 review PASS on its
  Non-Compliance Boundary" — the DEL-02-03 review of record is `KIT/_REVIEW.md`
  (a `SELF_CHECK / AGENT_CHECK` mechanical pass, Codex, status
  `FINDINGS_CAPTURED`, 3 MINOR traceability findings, 0 CRITICAL/MAJOR). It is
  an **agent** review, not a human ruling, and is not a README. Its SC-003
  line ("No software certification, sealing, approval, authentication, or
  code-compliance claim found") is a review finding recorded on that surface,
  not adopted as an assertion in any ledger cell; `_REVIEW.md` is not in the
  addendum-1 declared-state census (not a kit doc, `_STATUS`, `MEMORY`, or
  README) so it takes no row.
- Frozen tree porcelain empty before and after all reads and the sandboxed
  test re-execution. Writes confined to
  `WAVES/W1/CLAIM_CONCORDANCE_DEL-02-03.csv` and this notes file.

## Evidence-execution log

- **Re-executed (side-effect-free, addendum 9):**
  `tests/test_analysis_boundary_schema.py` (suite PY-09) from the frozen
  working root with `PYTHONDONTWRITEBYTECODE=1`, `-p no:cacheprovider`, and
  `TMPDIR` redirected to scratch: **2 passed in 0.01s**. `git -C FROZEN status
  --porcelain` was empty immediately before and immediately after. This
  binds R01-R08 and R12 (the two tests assert the AnalysisStatus vocabulary,
  the AutomaticAnalysisStatus exclusion of `CODE_COMPLIANT`/
  `HUMAN_APPROVED_FOR_PROJECT`, `forbidden_software_claims`,
  `professional_boundary` const-false fields, the mechanics/user-rule status
  separation, `rule_pack_ref`/`supplied_by_user`, `HumanAcceptanceRef`
  binding, `Provenance`/`MissingInputFinding` required fields, and fixture
  authority split).
- **Cited recorded (not re-derived):** `VERIFICATION_INDEX.csv` PY-09 records
  the DEC-025 sweep `SWEEP_20260711T040758Z_e648462f1d05.json`
  (`python_pytest=pass`, clean tree) at ancestor commit `e648462f1d05...` with
  the addendum-10 qualifier `CONTENT_IDENTICAL (content-identical at frozen SHA
  551f84ef6... (diff empty over tests/, schemas/, fixtures/, ...))`. My live
  re-execution supersedes the need to rely on that binding, but it is
  consistent with it.
- **Direct inspection at frozen SHA** for R09 (schema `$schema` =
  `.../draft/2020-12/schema`, not asserted by PY-09), R10 (structurally closed
  objects; no adapter/API surface exists to exercise no-bypass), R11 (`TBD`
  enum tokens present), and the scope exclusions.
- **DecisionBasis resolvability (convention 7):** `DEC-010` (R09) resolves to
  `execution/_Decomposition/SOFTWARE_DECOMP.md` section 12 (line ~588 in the
  frozen tree; decisions register; JSON Schema 2020-12 baseline; human-approved
  2026-04-30) and governs the schema-baseline claim. All other requirement
  rows carry `NONE_FOUND` (they derive from governing docs/SOW-002, not from a
  specific accepted decision record).
- **Validation:** no `VALIDATION_AND_PROVENANCE_INDEX.csv` rows exist for
  DEL-02-03 (confirmed by grep). This is a code-neutral draft boundary model
  with no numeric-mechanics/benchmark/witness class, so `ValidationEvidence` is
  `NOT_APPLICABLE` on every row (verification != validation; no unit test was
  promoted to validation).

## Self-flagged rows

- **DEL-02-03-DECL-001 (Specification.md) — STALE_SETUP_SPECIFICATION.**
  Judgment call. The surface's Documentation/Scope prose frames the schema as
  an anticipated artifact ("analysis_status enum/schema location TBD") and
  `docs/SPEC.md` integration as future, while a concrete
  `schemas/analysis_boundary.schema.yaml` plus tests and fixtures exist in the
  frozen tree. Read as widened-STALE post-alignment drift (addendum 4) on a
  declared-state surface. Confidence MEDIUM: a reviewer could instead read the
  draft/TBD framing as accurate about the *final* sealed artifact (path,
  field-split, docs/SPEC.md section all genuinely still TBD per R11), in which
  case ALIGNED. The requirement substance is ALIGNED regardless (see REQ rows);
  this is a surface-drift note only, no document repair proposed.
- **DEL-02-03-DECL-002 (Datasheet.md) — STALE_SETUP_SPECIFICATION.** Same
  judgment call as DECL-001. The Construction field table marks fields that the
  frozen schema concretely realizes (`status_authority` -> `authority_model`,
  `diagnostics`) as `ASSUMPTION`/`exact schema TBD`, and the doc never
  references the committed schema. Confidence MEDIUM.
- **DEL-02-03-REQ-010 (R10, adapter/API no-bypass) — ALIGNED at MEDIUM.**
  Judgment call. The requirement is that the *model* preserve app-service/API
  boundary separation and that adapters/plugins not bypass controls. This
  deliverable is a DATA_MODEL draft with no adapter/API/plugin surface in
  scope, so there is nothing behavioral to exercise no-bypass on. Called
  ALIGNED at model/documentation grain (schema `additionalProperties:false`
  throughout; arch doc references AB-00-02/AB-00-07; Specification V11) with
  MEDIUM confidence; downstream behavioral verification is out of scope.
- **Acceptance-row census (0).** Flagged for reviewer awareness: I created no
  ACCEPTANCE rows because V01-V11 and the per-status acceptance table restate
  the requirements. If the reviewer reads the per-status "Acceptance check"
  table as independent addendum-12-grain criteria, up to 7 ACCEPTANCE rows
  could be added.

## Convention friction notes

- **STALE on declared-state vs. deliberate-draft TBD (conventions 1/4).** The
  widened `STALE_SETUP_SPECIFICATION` ("no longer describes the frozen
  implemented slice") and a legitimately-draft deliverable that *correctly*
  holds final specifics as TBD (per R11) are hard to separate when an interim
  implementation slice (a draft schema) exists but the four-doc kit still
  frames it in setup/anticipatory language. I resolved it by asking whether the
  surface prose would leave a reader unaware that a concrete artifact exists
  (Specification/Datasheet: yes -> STALE; Guidance/Procedure: no, they are
  timeless usage/process docs -> ALIGNED). This boundary is the main
  reviewer-eyes item.
- **SourceReliability ladder (addendum 6).** All 12 requirement rows are
  `UNVERIFIED`: the schema/tests/fixtures are project-original agent-generated
  technical evidence. A human disposition exists (2026-06-03
  TP-CODE-EVIDENCE-MIGRATION-RECONCILIATION-001 accepted the audit evidence and
  advanced the deliverable to CHECKING), which could arguably support
  `REVIEWED` per addendum 6. I chose the conservative `UNVERIFIED` because (a)
  that CHECKING acceptance was explicitly *bounded* implementation-evidence
  closeout and was later reset to IN_PROGRESS (2026-05-11 correction;
  reaffirmed 2026-07-02 K-CONFLICT-1 human ruling) for readiness reasons, and
  (b) I did not run a diff establishing that the 2026-06-03-accepted content is
  identical to the frozen-SHA content, so I cannot bind that human disposition
  to the frozen slice. The available human-acceptance history is noted here
  rather than promoted to `REVIEWED`.
- **RF-001 review finding is itself overtaken.** `Review_Findings.csv` RF-001
  (OPEN) says `_REFERENCES.md` records the decomposition as "accepted v0.2";
  the frozen `_REFERENCES.md` actually reads "Accepted revision 0.7". The
  finding is stale/superseded by later content updates. This does not affect
  any requirement disposition (it is a traceability-metadata note on a
  non-census surface) and is recorded here for the reviewer, not as a ledger
  row.
- **RF-003 (`_SEMANTIC_LENSING.md` stale candidate conflict)** and the
  `_SEMANTIC_LENSING.md`/`_SEMANTIC.md` surfaces are outside the addendum-1
  declared-state census (not kit docs, `_STATUS`, `MEMORY`, or README) and take
  no rows. RF-003 is a known, self-reported staleness on a worklist register,
  consistent with `Guidance.md` recording no unresolved source conflicts; noted
  for the reviewer only.
- **Agent-workflow implications:** none observed that would route
  `DEFERRED_AGENT_WORKFLOW`.

## STOP-worthy contradictions

NONE.
