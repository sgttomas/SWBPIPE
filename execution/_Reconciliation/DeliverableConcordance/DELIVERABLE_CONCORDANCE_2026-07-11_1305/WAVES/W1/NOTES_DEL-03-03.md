# NOTES — DEL-03-03 Bend and elbow component model fields (W1)

Run: DELIVERABLE_CONCORDANCE_2026-07-11_1305, R2 wave W1. Frozen SHA
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Ledger:
`CLAIM_CONCORDANCE_DEL-03-03.csv` (22 rows, 20 columns, RFC-4180 clean).

## Run-level aliases (declared once per addendum 12)

- `KIT/` in `NormativeSource` = the frozen deliverable folder
  `FROZEN/projects/chirality-piping/execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-03_Bend and elbow component model fields/`.
- Requirement-ID mapping (bare tokens → addendum-12 ClaimID form):
  R01→REQ-001, R02→REQ-002, R03→REQ-003, R04→REQ-004, R05→REQ-005,
  R06→REQ-006, R07→REQ-007, R08→REQ-008, R09→REQ-009, R10→REQ-010,
  R11→REQ-011.

## Histograms (recounted from the CSV)

ClaimType histogram (11 + 5 + 6 = 22):
- REQUIREMENT: 11
- EXCLUSION: 5
- DECLARED_STATE: 6
- ACCEPTANCE: 0
- REMAINING_WORK: 0
- IMPLEMENTED_UNMAPPED: 0

Disposition histogram (22):
- ALIGNED: 22

Census rationale for the empty ClaimTypes:
- ACCEPTANCE = 0: the Specification "Verification Hook" column and "Verification"
  section merely restate the requirements; per the census rule they do NOT get
  mirrored ACCEPTANCE rows, and no distinct addendum-12-grain acceptance-criteria
  artifact exists.
- REMAINING_WORK = 0: `_STATUS.md` `## Remaining` carries only the seeded
  `(gated: D-41)` bootstrap item, recorded verbatim in the DECL-005 (`_STATUS.md`)
  `RecordedRemaining` and excluded from all residual/gate/selectability analysis
  (addendum 2). `DELIVERABLE_INVENTORY.csv` confirms `NonBootstrapItems=NONE`.
- IMPLEMENTED_UNMAPPED = 0: every DEL-03-03 material surface in its orbit
  (component.schema.yaml SURF-181, model.schema.yaml SURF-190, fixtures/component
  SURF-150, core/product_physics SURF-102, and the desktop feature panels
  SURF-018/029/030/031/032/036/038/043/051/052) is already deliverable-attributed
  in `IMPLEMENTATION_SURFACES.csv`; none is on the R1 `NONE_FOUND` unmapped
  shortlist.

## Why every substance row is ALIGNED

This is a mature, doc-refreshed, human-gated deliverable and the concordance
found no substance divergence:
- The four-document kit was refreshed by the 2026-06-05 evidence-reconciliation
  TASK to describe the implemented schema/fixture/test slice (present tense,
  evidence-scoped) — so no `STALE_SETUP_SPECIFICATION` on the declared-state
  rows (contrast DEL-02-02, whose setup kit was never refreshed and drew four
  STALE declared-state rows).
- The three PKG-02 downstream findings (`PKG03-DEL-03-03-PKG02-001..003`) were
  accepted `ACCEPT_AS_IS`/`RESOLVED` by the Gate D human ruling (2026-06-05);
  their subject records (elbow canonical compatibility, unit-vocabulary
  compatibility, strict-fixture persistence) are current at the frozen SHA, so
  the requirements resting on them are ALIGNED and `REVIEWED` (REQ-001, REQ-008,
  REQ-010).
- Field-model substance is verified by PY-20
  (`tests/test_component_section_schema.py`), re-executed here (2/2 pass).
- Mechanics-formulation and solver-consumption correctness (user SIF/flexibility
  realization, curved-bend macro-element) are routed to PKG-04/PKG-05 per
  convention 3, not adjudicated here (see REQ-003/004/010, EXC-001).

## KNOWN R1 ANOMALY — DEL-03-03 / DEL-03-05 requirement-ID collision

R1 (`RUN_BASIS.md`) flagged that DEL-03-03 and DEL-03-05 carry identical bare
requirement-ID *token sets* (both R01–R11). The two token sets label DIFFERENT
requirement text (DEL-03-03 = bend/elbow fields; DEL-03-05 = rigid/semi-rigid
components) — confirmed by reading both Specifications. The addendum-12 ClaimID
form (`DEL-03-03-REQ-00N`) disambiguates mechanically; the R0N→REQ-00N mapping
is recorded above.

Cross-reference safety check (per dispatch): I grepped all DEL-03-03-owned
records for bare `R0N`/`R1N` requirement-token citations outside the
Specification requirement table. **Result: none.** No DEL-03-03 record cites a
bare requirement token in a way that could cross-reference DEL-03-05. Per the
dispatch rule ("encode a finding only if the deliverable's own records
mis-cross-reference"), **no finding is encoded** — the collision is a
corpus-level ID-scheme hazard, inert for this deliverable at the frozen SHA.

## Self-flagged rows

- **DEL-03-03-REQ-011** — judgment call: R11's own wording scopes the deliverable
  to *recording* the adapter/plugin no-bypass constraint ("adapter/API proof
  remains outside DEL-03-03 scope"). I read the recorded-constraint as satisfied
  (ALIGNED) and routed the enforcement proof cross-package (convention 3, homed
  to adapter/plugin deliverables e.g. DEL-02-04). A reviewer preferring to treat
  unproven no-bypass enforcement as an open requirement would downgrade this.
- **DEL-03-03-DECL-002** — Datasheet cites `SOFTWARE_DECOMP.md` revision 0.7 as
  decomposition basis; the current live basis is 0.8 (DEC-073 on main; the frozen
  tree carries through DEC-072). I judged this a decomposition-revision citation
  lag (metadata), not a staleness of the field-slice description, so kept ALIGNED
  at MEDIUM rather than `STALE_SETUP_SPECIFICATION`. Reviewer eyes on that
  boundary call.
- **DEL-03-03-DECL-006** — MEMORY.md's 2026-05-01 session lists open TBDs (public
  source catalogs, public fixture-value policy, solver use of flexibility, import
  formats, GUI editor) that persist and are not on `_STATUS.md ## Remaining`.
  Per addendum 1, dated MEMORY entries are historical and this drift is a surface
  note, never a staleness disposition — so ALIGNED at MEDIUM. Flagged so a
  reviewer can confirm none of those TBDs should have been promoted to a
  `_STATUS` residual (they are downstream/future scope, not field-model residuals).

## Evidence-execution log

- **Re-executed (side-effect-free, addendum 9):**
  `PYTHONDONTWRITEBYTECODE=1 python3 -m pytest -p no:cacheprovider -q tests/test_component_section_schema.py`
  in the frozen worktree — **2 passed in 0.14s**. `git -C FROZEN status
  --porcelain` empty BEFORE and AFTER. No external build artifacts (pytest only,
  bytecode suppressed). This is a real re-execution at the frozen SHA, not just
  the recorded-pass qualifier.
- **Cited as recorded:** `VERIFICATION_INDEX.csv` PY-20 records the pass at
  ancestor commit `e648462f1d05` with the addendum-10 content-identical
  qualifier (`diff empty over tests/, schemas/, fixtures/, ...`). My re-execution
  supersedes the need for that qualifier but I retain the PY-20 binding in the
  ledger for traceability.
- **Not re-executed:** the 2026-06-20/21 desktop Vitest/Playwright and
  `cargo test core/product_physics` passes (cited in MEMORY / WORKING_ITEMS run
  records) were not re-run — they exercise cross-package app/mechanics surfaces
  outside this field-model deliverable's substance and are not required to settle
  any DEL-03-03 row. The protected-content scan (REQ-006/EXC-003) is cited as
  agent-recorded (MEMORY 2026-05-01), hence `UNVERIFIED`.
- Product surfaces read for citation (read-only): `schemas/component.schema.yaml`,
  `schemas/model.schema.yaml`, `schemas/units.schema.yaml`,
  `fixtures/component/invented_component_library_valid.json` (existence + test
  linkage), `tests/test_component_section_schema.py`.

## Convention-friction notes

- **DecisionBasis for a component-realization deliverable.** No single decision
  governs the pure schema-shape field requirements; DEC-045 (D-18 Option C,
  component realization: `mechanics_geometry_only` for bend) is the accepted
  decision that governs the realization posture, so I cited it on REQ-001/002/003/
  004/010 and `NONE_FOUND` on the invariant-driven governance requirements
  (R05/R06/R07/R09/R11). DEC-045's *solve path* was partially superseded by
  DEC-066/DEC-070; that supersession lives in PKG-04/PKG-05 and does not disturb
  the field-model slots, which is why I did not treat it as a divergence here.
  Reasonable reviewers could instead cite `NONE_FOUND` throughout or attach
  DEC-070/DEC-066 to REQ-004.
- **REQ-008 DecisionBasis = DEC-018.** R08's dimensional-checking relies on the
  units contract accepted under DEC-018 (governs DEL-02-02); cited as the
  governing unit-contract decision because the ComponentQuantityDimension
  vocabulary draws from it.
- **REVIEWED vs UNVERIFIED (addendum 6).** I set `REVIEWED` only on rows whose
  cited record is covered by the named Gate D human disposition (REQ-001, REQ-008,
  REQ-010) and `UNVERIFIED` on the remaining agent-generated schema/test evidence
  rows, matching the DEL-02-02 exemplar's conservatism.
- **`solver_consumption` enum as a field-model artifact.** The component schema
  carries `curved_bend_macro_element` and `mechanics_geometry_and_user_flexibility`
  as *label* enum values (the DEC-070/DEC-066 model-side reflection). I treated
  these as field-model evidence (the deliverable's job) and explicitly routed the
  underlying solver formulation to PKG-04/PKG-05, per the dispatch domain note.

## Boundary-compliance statement

All fences held. Discovery was read-only outside the two W1 output files
(`CLAIM_CONCORDANCE_DEL-03-03.csv`, `NOTES_DEL-03-03.md`); no `_STATUS.md`,
register, product, or DAG file was modified; no lifecycle transition applied
(none proposed). No release-readiness, issuance, certification, sealing,
professional-approval, or code-compliance claim appears in these outputs
(F-PIP-1..5); EXC-005 records the deliverable's own out-of-scope exclusion of
such claims without asserting any. All dispositions are agent judgments, routed
via `AuthorityNeeded`, never phrased as owner or engineering rulings. The frozen
evidence worktree was verified `git status --porcelain` empty before and after
the single re-executed check and remains clean; no writes (even git-ignored)
were made under `FROZEN`.
