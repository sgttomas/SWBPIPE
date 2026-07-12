# NOTES — DEL-03-06 Expansion joint component model (W1)

Run: DELIVERABLE_CONCORDANCE_2026-07-11_1305, R2 wave W1. Frozen SHA
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Ledger:
`CLAIM_CONCORDANCE_DEL-03-06.csv` (20 rows, 20 columns, RFC-4180 clean, CRLF).

## Run-level aliases (declared once per addendum 12)

- `KIT/` in `NormativeSource`/evidence cells = the frozen deliverable folder
  `FROZEN/projects/chirality-piping/execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-06_Expansion joint component model/`.
- Requirement-ID mapping (deliverable scheme → addendum-12 ClaimID form):
  DEL-03-06-R-001→DEL-03-06-REQ-001, R-002→REQ-002, R-003→REQ-003,
  R-004→REQ-004, R-005→REQ-005, R-006→REQ-006, R-007→REQ-007,
  R-008→REQ-008, R-009→REQ-009. The scheme is self-identifying
  (`DEL-03-06-R-*`), so no collision hazard exists (contrast the
  DEL-03-03/DEL-03-05 bare-token collision flagged at R1).

## Histograms (recounted from the CSV before writing)

ClaimType histogram (9 + 5 + 6 = 20):
- REQUIREMENT: 9
- EXCLUSION: 5
- DECLARED_STATE: 6
- ACCEPTANCE: 0
- REMAINING_WORK: 0
- IMPLEMENTED_UNMAPPED: 0

Disposition histogram (20):
- ALIGNED: 15
- ACCEPTED_DIVERGENCE: 2
- STALE_SETUP_SPECIFICATION: 3

Census rationale for the empty ClaimTypes:
- ACCEPTANCE = 0: the Specification "Verification" table is keyed one-to-one
  to the requirement IDs and merely restates/points at their evidence; per the
  addendum-12 grain rule it gets no mirrored ACCEPTANCE rows, and no distinct
  acceptance-criteria artifact exists in the kit.
- REMAINING_WORK = 0: `_STATUS.md ## Remaining` carries only the seeded
  `(gated: D-41)` bootstrap item, recorded verbatim in DECL-005's
  `RecordedRemaining` and excluded from all residual/gate/selectability
  analysis (addendum 2). `DELIVERABLE_INVENTORY.csv` confirms
  `NonBootstrapItems=NONE`. The kit's declared TBDs are bounded deferrals
  handled on their substance rows (addendum 5), not omitted evidence-backed
  residuals — see the ACCEPTED_DIVERGENCE rationale below.
- IMPLEMENTED_UNMAPPED = 0: every material surface in this deliverable's
  orbit is already deliverable-attributed to DEL-03-06 in
  `IMPLEMENTATION_SURFACES.csv` (component/model/units/persistence/section
  schemas SURF-181/190/197/206/209 family; desktop features SURF-030/031/032/
  038/051; validate_dependencies_schema SURF-225; core/product_physics via
  the EJSTIFF run record). None is on the R1 `NONE_FOUND` unmapped shortlist.

## Disposition rationale

- **ALIGNED substance rows (REQ-001/002/005/006/007/008/009, EXC-001..005).**
  The 2026-06-05 evidence-reconciliation TASK rewrote the four-document kit to
  present-tense, evidence-scoped prose, and the schema/fixture/test slice
  matches it exactly: `linear_stiffness`/`rotational_stiffness`/
  `effective_area`/`movement_limit`/`hardware_flag`/`hardware_reference`
  slots, the `comp.invented.expansion.alpha` fixture record with all values
  `missing`, the blocking `EXPANSION_JOINT_STIFFNESS_DATA_MISSING`
  completeness finding, five `EXPANSION_JOINT_*` schema diagnostic codes,
  required provenance fields, and the forbidden-text guardrail scan. The
  targeted test was re-executed at the frozen SHA (2/2 pass).
- **ACCEPTED_DIVERGENCE (REQ-003, REQ-004).** Addendum 5 + addendum 11: the
  movement-limit class taxonomy and hardware flag/enumeration taxonomy are
  declared bounded TBDs whose deferral is *permitted* by a named human
  ruling — the 2026-06-05 Gate C disposition accepted
  `PKG03-DEL-03-06-PKG02-002` (whose proposed disposition text explicitly
  frames those taxonomies as future sealed-task TBDs with no completeness
  claim) as `ACCEPT_AS_IS`/`RESOLVED`. Per addendum 8 precedence
  (`ACCEPTED_DIVERGENCE` > `ALIGNED` when both fit) these two rows take
  ACCEPTED_DIVERGENCE with the failed homing recorded in `RemainingWork`
  (candidate-home search bounded to DEL-03-06's own records: the TBDs are
  named in Specification/Datasheet/MEMORY but have no `## Remaining` home).
- **STALE_SETUP_SPECIFICATION (DECL-001, DECL-002, DECL-004).** Widened
  definition (addendum 4): these are not setup-era documents — they were
  refreshed 2026-06-05 — but their "current evidence is limited to /
  bounded to" the schema/fixture/test slice declarations no longer describe
  the frozen implemented slice. The 2026-06-21/22 EJSTIFF/EJTHRUST tranches
  (recorded in this deliverable's own `_run_records` and MEMORY, deliverable
  header `DEL-03-06`) landed core/product_physics expansion-joint validation,
  the invented preview `component:C-150` with user-entered stiffness values,
  pressure-thrust result rows, and desktop-surface visibility. That is
  post-alignment drift on the kit's evidence-inventory declarations.
  Guidance (DECL-003) stays ALIGNED because its normative content
  (principles, boundaries, trade-offs, preserved-TBD instructions) remains
  accurate; only its Considerations sentence shares the inventory lag.
- **No REMAINING_STATE_MISMATCH.** The persisting TBDs are declared and
  Gate-C-accepted deferrals, not landed/ruled-shut items still recorded nor
  omitted evidence-backed residuals; convention-3/addendum-5 homing was
  checked before any mismatch encoding.

## Self-flagged rows

- **DEL-03-06-REQ-003, DEL-03-06-REQ-004** — judgment call the conventions
  leave open: the requirement wording itself internalizes the TBD ("while
  taxonomy remains TBD"), so plain ALIGNED also fits; I applied the
  addendum-5/addendum-8 branch (named Gate C ACCEPT_AS_IS ruling permits the
  deferral → ACCEPTED_DIVERGENCE wins). A reviewer preferring
  requirement-wording literalism would re-encode both as ALIGNED with the
  same RemainingWork text; no other cell changes.
- **DEL-03-06-REQ-008** — same call as sibling DEL-03-03-REQ-011: the
  Specification's own Verification row scopes current evidence to
  schema/fixture/test and routes bypass-proof downstream (DEL-02-04 orbit),
  so I read the recorded-constraint as satisfied (ALIGNED, MEDIUM) rather
  than PARTIALLY_IMPLEMENTED. Encoded identically to the sibling for wave
  consistency; reviewer eyes on whether unproven no-bypass enforcement
  should downgrade.
- **DEL-03-06-DECL-001/002/004 vs DECL-003** — the STALE-vs-ALIGNED boundary
  across the four kit surfaces is my call under the widened addendum-4
  definition: Specification/Datasheet/Procedure make explicit
  "limited to/bounded to/current implementation evidence is exactly these
  three files" declarations (falsified by the deliverable's own 2026-06-21/22
  run records), while Guidance's declarations are principles that still hold.
  A reviewer could mark all four STALE or none; the asymmetry is deliberate
  and evidence-cited.
- **DEL-03-06-DECL-006** — MEMORY Open Items TBDs (source catalogs,
  fixture-value policy, DOF mapping, hardware taxonomy, import formats, GUI
  editor) persist with no `_STATUS` residual home. Per addendum 1 this is a
  surface note, never a staleness disposition; flagged so a reviewer can
  confirm none should have been promoted to a `## Remaining` item (I judged
  them declared deferrals — two of them Gate-C-accepted — not omitted
  residuals).
- **DEL-03-06-REQ-005** — the "when persisted, imported" clause is partially
  vacuous at the frozen SHA (no import path exists; import formats are a
  declared TBD). I encoded ALIGNED at MEDIUM for the paths that exist rather
  than PARTIALLY_IMPLEMENTED for paths that don't; reviewer eyes welcome.

## Evidence-execution log

- **Re-executed (side-effect-free, addendum 9):**
  `PYTHONDONTWRITEBYTECODE=1 python3 -m pytest -p no:cacheprovider tests/test_component_section_schema.py`
  in the frozen worktree — **2 passed in 0.12s**. `git -C FROZEN status
  --porcelain` empty BEFORE and AFTER. No cargo/npm execution; pytest cache
  provider disabled and bytecode suppressed; nothing written under FROZEN.
- **Re-executed (read-only git):**
  `git diff e648462f1d05..551f84ef6 -- projects/chirality-piping/{core/product_physics,schemas,fixtures,tests}`
  — **empty**. This diff was actually run, so the addendum-10 qualifier
  `content-identical at frozen SHA 551f84ef6be656f1603ce0acfa5e3935aa9683c7
  (diff empty over core/product_physics, schemas/, fixtures/, tests/)` is
  lawfully attached to the recorded EJSTIFF/EJTHRUST and PY-20/RUST-08
  ancestor-commit evidence. Porcelain re-checked empty after.
- **Cited as recorded (not re-executed at frozen SHA 551f84ef6):**
  the 2026-06-21/22 WORKING_ITEMS run-record passes (cargo test
  core/product_physics 34/34; desktop vitest 406/406; playwright e2e 18/18)
  and the DEC-025 sweep `SWEEP_20260711T040758Z_e648462f1d05.json`
  (python_pytest pass, cargo_crate_sweep pass) at commit `e648462f1`,
  per `VERIFICATION_INDEX.csv` PY-20/RUST-08.
- **Resolvability checks (read-only):** verified in-tree existence of
  `D-18_component_macro_element_realization.md`, both LifecycleCorrection
  `Decision_Log.md` records, DEC-018/DEC-045 rows in
  `SOFTWARE_DECOMP.md` §12, `Review_Findings.csv` ACCEPT_AS_IS/RESOLVED
  rows, and the schema/fixture/test artifacts themselves. Noted:
  `execution/_Reconciliation/Reviews/DEV001_FINDING_RESOLUTION_PKG02_GROUNDED_2026-05-16/RESOLUTION_MATRIX.csv`
  cited by MEMORY 2026-05-16 is **not present** in the frozen tree; nothing
  in the ledger uses it as a DecisionBasis (the Gate C basis resolves via
  `Review_Findings.csv` + `_STATUS.md`/MEMORY entries instead), so it is
  recorded here as a provenance note only.
- **Session note:** this pilot's first session was cut off by a connection
  error after all discovery reads and before any output was written; it was
  resumed from its own transcript and both output files were written in the
  resumed session. No partial artifacts existed before the resume.

## Convention-friction notes

- **Kit "refreshed but re-drifted" is awkward for addendum 4.** The widened
  `STALE_SETUP_SPECIFICATION` ("no longer describes the frozen implemented
  slice") technically covers a kit that was refreshed mid-life and then
  overtaken by later tranches, but the label's name still says "setup" —
  reviewers aggregating on the token may over-read these three rows as
  never-refreshed setup documents. A drift-vintage note field (or a notes
  convention) would disambiguate.
- **ACCEPTED_DIVERGENCE via a finding disposition.** Addendum 11 requires a
  named ruling that *permits* the deferred state. The Gate C ruling accepted
  a finding whose proposed-disposition text frames the taxonomy deferral;
  whether "ACCEPT_AS_IS on a finding that describes the deferral" equals
  "permits the deferral" is interpretation. I read yes; flagged on the rows.
- **ClaimClass for boundary/guardrail exclusions.** Assigned GOVERNANCE to
  EXC-001..004 (IP/claims boundaries) and SCHEMA to EXC-005 (data-model
  scope boundary); the enum has no obvious single home for scope exclusions
  and addendum 7 does not cover them.
- **REQ-005 unit claim class.** Addendum 7 maps "numeric/unit correctness"
  to MECHANICS, but this claim asserts a schema-level dimensional-vocabulary
  contract, not numeric correctness; encoded SCHEMA with DEC-018 as basis
  (consistent with sibling DEL-03-03's dimensional-checking row).
- **DecisionBasis plurality.** REQ-001 carries both the Gate C disposition
  (governs the stiffness-dimension compatibility claim) and DEC-045 (governs
  the user-stiffness realization posture); §6 gives no rule for multiple
  governing bases, so both are cited semicolon-separated.

## Boundary-compliance statement

All fences held. Discovery was read-only outside the two W1 output files
(`CLAIM_CONCORDANCE_DEL-03-06.csv`, `NOTES_DEL-03-06.md`); no `_STATUS.md`,
register, product, DAG, or cross-project file was touched; no lifecycle
transition applied (none proposed). No release-readiness, issuance,
certification, sealing, professional-approval, or code-compliance claim
appears in these outputs (F-PIP-1..5); EXC-004 records the deliverable's own
exclusion of such claims without asserting any. All dispositions are agent
judgments routed via `AuthorityNeeded`, never phrased as owner or engineering
rulings. The frozen evidence worktree was verified `git status --porcelain`
empty before and after the re-executed pytest check and the read-only git
diff, and remains clean; no writes (including git-ignored paths) were made
under `FROZEN`. The generator script lives in the session scratchpad, not in
the run folder or frozen tree.
