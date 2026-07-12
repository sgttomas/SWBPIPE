# NOTES — DEL-06-05 Invented non-code example rule pack (R2 wave W3)

Deliverable: **DEL-06-05** (PKG-06), IN_PROGRESS. Ledger:
`WAVES/W3/CLAIM_CONCORDANCE_DEL-06-05.csv` (23 rows, RFC-4180 clean, 20 columns).
Frozen source: `main` @ `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.

Run-level `NormativeSource` path alias (addendum 12): all bare kit-file names
(`Specification.md`, `Datasheet.md`, `Guidance.md`, `Procedure.md`, `MEMORY.md`,
`_STATUS.md`, `_CONTEXT.md`, `_REFERENCES.md`, `Dependencies.csv`) resolve under
`FROZEN/projects/chirality-piping/execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/DEL-06-05_Invented non-code example rule pack/`.
Repo-relative paths (`examples/`, `docs/`, `schemas/`, `tests/`, `tools/`,
`execution/`) resolve from `FROZEN/projects/chirality-piping/`.

## 1. Histograms (recount from the CSV)

Disposition histogram:
- ALIGNED — 19
- STALE_SETUP_SPECIFICATION — 4
- (total 23)

ClaimType histogram:
- REQUIREMENT — 8
- ACCEPTANCE — 6
- EXCLUSION — 3
- DECLARED_STATE — 6
- (total 23)

Row census rationale:
- 8 REQUIREMENT rows, one per current requirement ID DEL-06-05-REQ-01..08
  (re-verified against the frozen Specification and against
  `DELIVERABLE_INVENTORY.csv` row 42; ClaimIDs use the addendum-12 fixed 3-digit
  form DEL-06-05-REQ-NNN, with the native 2-digit ID recorded in NormativeSource).
- 6 ACCEPTANCE rows at addendum-12 grain, one per Specification Verification
  entry VER-01..06. These are distinct acceptance METHODS (kit-existence,
  no-repo-edit, dependency-schema validity, enum canonicality, boundary
  visibility, semantic-artifact existence), not restatements of the requirement
  set, so they are mirrored per the brief.
- 3 EXCLUSION rows, one per Specification "The deliverable excludes" bullet.
- 6 DECLARED_STATE rows per addendum 1 census: Specification, Datasheet,
  Guidance, Procedure, MEMORY.md, _STATUS.md. There is **no deliverable-owned
  in-tree README** in the folder, so no README DECL row.
- 0 REMAINING_WORK rows: `_STATUS.md` `## Remaining` is bootstrap-only (the
  seeded `(gated: D-41)` item), which is transcribed verbatim into the _STATUS
  surface row's RecordedRemaining and excluded from all residual/gate/
  selectability analysis (addendum 2 / assignment note). MEMORY "Open Items"
  (checksum registry, result-envelope integration, API/GUI/storage) are
  explicitly homed to other deliverables (DEL-06-04, PKG-08, etc.) and are scope
  exclusions here, not omitted residuals of DEL-06-05 — so no UNKNOWN/omitted-
  residual row is warranted.
- 0 IMPLEMENTED_UNMAPPED rows: the material surfaces in this deliverable's orbit
  (`examples/rule_packs/invented_demo.yaml`, `schemas/rule_pack.schema.yaml`) are
  already mapped (R1 IMPLEMENTATION_SURFACES SURF-144/SURF-205 both list
  DEL-06-05); `docs/_Examples/rule_pack_notice.md` is a docs notice, not a
  crate/binary/schema/app-panel material surface, so it is out of grain.

## Core deliverable posture (fence-adjacent property)

DEL-06-05's core property: the example rule pack is **INVENTED and non-code**.
The kit and the landed artifacts hold this boundary. `docs/_Examples/rule_pack_notice.md`
and `examples/rule_packs/invented_demo.yaml` carry only invented non-engineering
values and explicit disclaimers ("does not certify, approve, seal, authenticate,
or declare engineering code compliance for reliance"; `software_makes_certification_claim=false`).
A frozen-SHA grep found **no positive code-compliance / professional-approval
claim** anywhere (only negations). No output in this ledger phrases anything as
code compliance, certification, sealing, professional approval, or release
readiness (F-PIP-1..5 held).

## Central finding: setup-era kit prose vs a landed implementation

The four-document kit (drafted 2026-04-30) frames the invented example as a
**future** artifact and states the deliverable "does not create or modify
repo-level example artifacts such as `examples/rule_packs/invented_demo.yaml`".
At the frozen SHA that example **exists and is implemented** (schema_version
0.4.0, grammar_version 1.0.0 frozen DEC-022 declarative AST, verified
`rule_pack_checksum`), together with `docs/_Examples/rule_pack_notice.md`
(MEMORY Implementation Summary; bounded implementation commit `73506b7`; later
upgrades TP-C2-SCHEMA-001 2026-06-12 and DEC-038/DEC-039). The future/setup-only
declarations therefore no longer describe the frozen implemented slice, so the
Specification, Datasheet, Guidance, and Procedure DECLARED_STATE surfaces take
`STALE_SETUP_SPECIFICATION` under widened addendum 4, drift facts in-row,
`AuthorityNeeded=OWNER` (R5 repair candidates, recorded not applied). This
mirrors the W1 rev-drift STALE-side precedent and the R0b DEL-07-05 exemplar
(C02–C05). Requirement/acceptance/exclusion rows keep substance dispositions and
never take STALE (convention 1).

## 2. Self-flagged rows

- **DEL-06-05-REQ-007** — "the setup run shall not write outside the assigned
  folder / no ISSUED move." Judgment call on grain: the *setup run* honored this
  (run records confined to the folder; never ISSUED), so ALIGNED — but the
  deliverable's later bounded implementation (commit `73506b7`, human-approved
  closeout) did write `examples/` and `docs/` under separate authorization. I
  ledgered the requirement ALIGNED at setup-run grain and routed the blanket
  "no repo-level artifact" scope drift onto the Specification DECL row
  (DECL-001). Reviewer eyes on the grain choice.
- **DEL-06-05-ACC-002** — parallel grain call for VER-02 ("no repo-level example
  path was edited"): ALIGNED at setup-run grain / frozen porcelain empty, with
  the later repo-level creation ledgered STALE on DECL-001.
- **DEL-06-05-ACC-001** and **DEL-06-05-ACC-004** — the acceptance METHOD tools
  named by the kit (`tools/validation/check_four_documents.sh`,
  `tools/validation/validate_enum.py`) do **not exist** in the frozen tree (only
  `validate_dependencies_schema.py` is present). Acceptances met by direct
  inspection; the method tools' absence is disclosed in-row and drives MEDIUM
  confidence. (Same tool-absence observed in the R0b DEL-07-05 exemplar C05.)
- **DEL-06-05-DECL-004** — Procedure disposition. A Procedure is a process
  document with a weaker current-state claim than the other three kit docs; I
  marked it STALE_SETUP_SPECIFICATION (end-state SEMANTIC_READY overtaken by
  IN_PROGRESS after a landed implementation; it names two absent validation
  tools) at MEDIUM confidence. A reviewer preferring to treat pure process prose
  as non-declarative could downgrade this to ALIGNED-with-note.
- **DEL-06-05-DECL-005** — MEMORY marked ALIGNED-with-note under calibration
  item 9 (the undated header block's 0.1.0-era drift is corrected in-file by the
  dated 2026-06-12 entry). Reviewer eyes on the item-9 carve-out and on the two
  historical-drift notes carried in-row (see §4).

## 3. Evidence-execution log

Frozen-tree porcelain was empty before and after every operation
(`git -C FROZEN status --porcelain`). Re-executions (addendum 9 side-effect-free:
`PYTHONDONTWRITEBYTECODE=1`, pytest `-p no:cacheprovider`, no build artifacts):

- `python3 -m json.tool examples/rule_packs/invented_demo.yaml` → strict-JSON
  parse OK (pure read). Cited on REQ-001/003/005.
- `python3 tools/validation/validate_dependencies_schema.py <local Dependencies.csv>`
  → VALID, 29 columns (29 required + 0 extension), 20 data rows. Cited on
  ACC-003.
- `python3 -m pytest tests/test_rule_pack_schema.py -p no:cacheprovider` →
  **5 passed** (jsonschema 4.26.0 present; the suite JSON-Schema-validates
  `examples/rule_packs/invented_demo.yaml`). Cited on REQ-005/006/008. This is a
  live frozen-SHA pass, so the ancestor content-identical qualifier is NOT used;
  the recorded sweep `SWEEP_20260711T040758Z_e648462f1d05.json` (python_pytest=pass
  at commit e648462f1, ancestor of the frozen SHA; R1 VERIFICATION_INDEX PY-65)
  is cited only as corroboration.

Porcelain confirmed empty after all three. No build/bytecode/cache artifacts
were written into the frozen tree.

Cited-as-recorded (not re-executed): the focused protected-content /
prohibited-claim scans and the `python3 -m json.tool` / focused schema-surface
assertions in `MEMORY.md` (2026-05-02, 2026-05-11 TP-RECON-01, 2026-06-05
recheck); the DEV-001 PKG-02 audit and its human disposition
(`Review_Findings.csv` PKG06-05-PKG02-001 → HumanDisposition=ACCEPT_AS_IS,
Status=RESOLVED, 2026-06-05).

## 4. Convention-friction notes

- **Rev-0.7 decomp authority-pointer drift (W1 calibration item 1).** The kit
  cites `SOFTWARE_DECOMP.md` revision **0.7** (+ DAG-006) in `_CONTEXT.md`
  (lines 40, 52), `_REFERENCES.md` (line 15), and a dated `MEMORY.md`
  2026-06-04 entry; the frozen decomp header is **revision 0.8,
  `status: current_basis`** (frozen tree carries through DEC-072; DEC-073 lands
  only on `main`, per RUN_BASIS — ruling-after-freeze mechanics, not a conflict).
  Calibration item 1 directs encoding this as STALE on the affected
  DECLARED_STATE surface row. **Here the pointer drift lands only on non-census
  surfaces** (`_CONTEXT.md`, `_REFERENCES.md` are not in the addendum-1
  DECLARED_STATE census) and on a *dated* MEMORY log entry (historical per
  addendum 1). It therefore generates **no STALE row of its own**; I recorded it
  as a historical in-row note on the MEMORY surface (DECL-005) and here.
  Owner-calibration caveat (recorded once): the rev-0.7→0.8 pointer drift is
  pure authority-pointer drift with `AuthorityNeeded=NO`; the kit carries no
  overtaken TBD register tied to the pointer, so no OWNER routing attaches to the
  drift itself. (The STALE dispositions on the kit DECL rows are driven by the
  separate future-tense-vs-landed-implementation drift, which does carry OWNER
  R5-repair routing.)
- **Non-resolving record path (W2 calibration item 15).** The dated `MEMORY.md`
  2026-05-16 addendum references
  `execution/_Reconciliation/Reviews/DEV001_FINDING_RESOLUTION_PKG02_GROUNDED_2026-05-16/`
  (RESOLUTION_MATRIX.csv, VALIDATION_SUMMARY.md). A full-tree `find` at the
  frozen SHA confirms these are **absent**. Because the reference sits in a dated
  MEMORY log entry (historical), it is recorded as an in-row note on DECL-005,
  not a staleness disposition, and I did not rely on those absent records for any
  disposition (the human disposition I cite is in `Review_Findings.csv`, which is
  present). Noted per item 15 with the frozen-SHA phrasing.
- **SourceReliability keying (W2 calibration item 13).** All REQUIREMENT,
  ACCEPTANCE, and EXCLUSION rows are `UNVERIFIED`: their load-bearing evidence is
  agent-run (re-executed deterministic tests, direct inspection, recorded
  protected-content scans). The one human disposition on record
  (ACCEPT_AS_IS/RESOLVED) covers a single specific finding
  (PKG06-05-PKG02-001, schema `output_dimension` compatibility), not the full
  requirement/acceptance set, and item 13 keys to the weakest load-bearing leg —
  a human-approved verification/finding leg does not lift rows whose broader
  evidence is agent-generated. That finding is RESOLVED (no longer
  TECHNICALLY_ADDRESSED_PENDING_HUMAN), so the addendum-13 MEDIUM cap does not
  apply and HIGH confidence is used where evidence is strong. All 6
  DECLARED_STATE prose rows are `NOT_APPLICABLE` per addendum 6.
- **Bootstrap _STATUS cell scoping (W2 calibration item 11).** DECL-006 uses the
  exclusion variant: the `(gated: D-41)` item is transcribed byte-exact
  (verified against `_STATUS.md`; `§§6–8` with en-dash preserved, no
  transliteration) into RecordedRemaining, and GateOrStageConstraint /
  RemainingSource are `NONE_RECORDED` scoped to non-bootstrap content — the
  bootstrap item is not annotated into the gate/source cells.
- **SelectableUnderCurrentLoop.** Mechanical (DAG/lifecycle/gate) only. No
  non-bootstrap residual exists, so every row is `NO` (addendum 12:
  `NO` on rows with no recorded item), consistent with
  `DELIVERABLE_INVENTORY.csv` (SelectableUnderCurrentLoop=NO). The owner
  suspension is a run-level caveat, not applied per row.
- **ClaimClass judgment.** The requirement-set is documentary/governance-heavy;
  I classed the IP/professional-boundary and write-scope requirements
  GOVERNANCE/WORKFLOW, and the schema-surface/declarative/architecture-basis
  requirements SCHEMA (the enum has no dedicated "documentation-content
  requirement" class). No MECHANICS/VALIDATION rows: this is a DOC_UPDATE
  deliverable with a declarative example; no numeric correctness or benchmark
  suitability claim is made, and none is promoted from the verification tests.

## 5. Boundary-compliance statement

- Discovery was READ-ONLY outside the two output files. No lifecycle transition
  applied (`LIFECYCLE_REASSESSMENT_REQUIRED` not used; STALE R5-repair
  candidates recorded as dispositions/AuthorityNeeded only). No DAG mutation, no
  cross-project edit, no edit to any `_STATUS.md`, register, or product file.
- No release-readiness, issuance, certification, sealing, professional-approval,
  or code-compliance claim appears anywhere in the ledger or these notes
  (F-PIP-1..5). All dispositions are agent judgments, routed via
  `AuthorityNeeded`, never phrased as owner or engineering rulings.
- Frozen tree clean: `git -C FROZEN status --porcelain` was empty before and
  after every read and every side-effect-free re-execution; no writes (including
  git-ignored paths) were made under the frozen worktree.
- Writes confined to exactly two files:
  `WAVES/W3/CLAIM_CONCORDANCE_DEL-06-05.csv` and `WAVES/W3/NOTES_DEL-06-05.md`.
