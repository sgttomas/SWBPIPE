# Notes — DEL-07-07 Solve execution UX: progress, cancellation, and diagnostics (W3)

Deliverable `DEL-07-07` (PKG-07, IN_PROGRESS). Frozen tree `551f84ef6`. Ledger:
`CLAIM_CONCORDANCE_DEL-07-07.csv` (23 rows, 20 columns, RFC-4180/CRLF clean).
Requirement scheme `REQ-07-07-*` re-verified against the frozen Specification and
`DELIVERABLE_INVENTORY.csv`: REQ-07-07-001 through -011 (11). ClaimIDs use the
addendum-12 fixed form `DEL-07-07-<TYPE>-NNN`; native IDs recorded in
`NormativeSource`.

Run-level `NormativeSource` alias (addendum 12): kit paths are relative to the
deliverable folder `execution/PKG-07_Graphical User Interface and Engineering
Workflow/1_Working/DEL-07-07_Solve execution UX- progress, cancellation, and
diagnostics/`; code/test/app paths are repo-root-relative within the frozen
worktree.

## 1. Histograms (recomputed from the CSV)

ClaimType histogram (23):
- REQUIREMENT: 11
- ACCEPTANCE: 5
- EXCLUSION: 1
- DECLARED_STATE: 6
- REMAINING_WORK: 0
- IMPLEMENTED_UNMAPPED: 0

Disposition histogram (23):
- ALIGNED: 18
- STALE_SETUP_SPECIFICATION: 4
- PARTIALLY_IMPLEMENTED: 1

## 2. Self-flagged rows

- **DEL-07-07-REQ-005** (PARTIALLY_IMPLEMENTED) — grain call under calibration
  item 6. Full machine-readable diagnostic field set (class/remediation/
  provenance) is preserved at the core engine (`core/gui/pkg02_boundary.py`
  `pkg02_diagnostic`) and in the SolvePanel JSON packet, but the desktop
  `Diagnostic` type (`apps/desktop/src/types.ts`) carries only
  `{id,code,severity,message,source,affected_refs}`. Chose PARTIALLY_IMPLEMENTED
  (rather than ALIGNED-at-contract-grain) because the desktop UI type
  structurally cannot hold class/remediation/provenance. Reviewer eyes: confirm
  the contract-vs-UI grain is the intended read.
- **DEL-07-07-REQ-006** (ALIGNED, MEDIUM) — grain call under calibration item 6.
  Warning classes are preserved as diagnostic code+severity where present; the
  six named DEL-00-06 classes (esp. PROVENANCE/ASSUMPTION/NONLINEAR/IP_BOUNDARY)
  are not all exercised in preview fixtures. Chose ALIGNED-at-contract-grain
  (conditional clause vacuous for the unexercised subset) rather than PARTIALLY.
- **DEL-07-07-REQ-009** (ALIGNED, MEDIUM) — grain call under calibration item 6.
  Reproducibility traceability is preserved for the supplied fields (model/result
  hashes, analysis-run ref, input-manifest refs, unit policy); solver-version and
  rule-pack version/checksum fields are unexercised (no rule-pack rows in preview
  envelopes). Chose ALIGNED-at-contract-grain.
- **DEL-07-07-REQ-011** (ALIGNED, MEDIUM) — future-tense requirement wording is
  overtaken (UI tests exist). Substance disposition on the requirement row is
  ALIGNED; the future-tense staleness itself is carried on the Specification
  declared-state row DECL-001 (requirement rows never take STALE per convention 1
  / row-census rule). Flagged to confirm the split is read correctly.
- **DEL-07-07-ACC-002** (ALIGNED, MEDIUM) — judgment call. VER-07-07-002 is a
  setup-run write-scope boundary acceptance; GUI/core source now exists under
  separately authorized tranches. Read the acceptance as scoping the setup run
  only (ALIGNED), with the surrounding Specification's setup-only framing carried
  STALE on DECL-001.
- **DEL-07-07-DECL-003** (Guidance, STALE_SETUP_SPECIFICATION, MEDIUM) — judgment
  call. Guidance's "prepares a future GUI slice" framing and Open-Issue TBDs are
  overtaken by the implemented slice, but the guidance principles remain valid and
  OI-07-07-003 (report/export handoff) is genuinely still open. Encoded STALE
  under widened addendum 4 (setup-era future-tense prose), naming the surviving
  open item in-row.
- **DEL-07-07-DECL-004** (Procedure, STALE_SETUP_SPECIFICATION, MEDIUM) — judgment
  call. Unlike DEL-07-05's Procedure, the named verification tool
  (`tools/validation/validate_dependencies_schema.py`) exists and re-runs VALID —
  no missing-tool defect. STALE rests solely on the setup-production / "may be
  left at SEMANTIC_READY" framing no longer describing the frozen implemented
  slice (widened addendum 4). A reviewer could reasonably read a self-limited
  setup-production procedure as historical-not-stale; flagged for that reason.

## 3. Evidence-execution log

Re-executed side-effect-free at frozen SHA `551f84ef6` (`PYTHONDONTWRITEBYTECODE=1`,
no cache writes; `git -C <frozen> status --porcelain` **empty before and after
each**):
- `python3 tests/test_solve_execution_ux.py` → PASS (exit 0; asserts
  `solver_execution=not_performed_by_gui_contract`,
  `job_orchestration=invented_state_transitions_only`, cancellation
  `mutates_solver_process_directly=false`, completed-event
  `hash_boundary.hashes_present=true`, and absence of any "professional
  acceptance" string). Cited on REQ-001..004, REQ-007, REQ-009..011, EXC-001,
  ACC-003.
- `python3 tools/validation/validate_dependencies_schema.py <DEL-07-07
  Dependencies.csv>` → `VALID` (29 required columns, 18 data rows). Cited on
  ACC-005.

Cited as recorded (NOT re-executed, per the GUI/node-tooling caution):
- DEC-025 evidence sweep
  `validation/evidence/sweeps/SWEEP_20260711T040758Z_e648462f1d05.json` at commit
  `e648462f1` (ancestor of frozen SHA); surfaces `cargo_crate_sweep`,
  `python_pytest`, `desktop_vitest`, `desktop_playwright_e2e`,
  `desktop_production_build` all pass, overall pass, clean tree. Marked
  `not re-executed at frozen SHA 551f84ef6` on every row that cites it. I did not
  run vitest/playwright/cargo in the frozen tree (they would write build/node
  artifacts into it); no addendum-12 byte-identical out-of-tree copy pattern was
  used. `VERIFICATION_INDEX.csv` PY-68 independently records the pytest surface
  as CONTENT_IDENTICAL at the frozen SHA; I did not re-assert the addendum-10
  content-identical qualifier for the TypeScript/Rust paths (no diff run over
  them), so those rows carry only the recorded-pass + not-re-executed marker.
- Recorded boundary review `_REVIEW.md` DEV-001 PKG-02 audit 2026-05-16 (verdict
  WARNING; finding `PKG07-DEL0707-PKG02-001` later `ACCEPT_AS_IS` / `RESOLVED`).
  Cited on REQ-007/008/009 (DecisionBasis), REQ-010, ACC-003.

## 4. Convention-friction notes

- **Rev-0.7 / DAG-006 authority-pointer drift (calibration item 1) — owner
  calibration caveat, recorded once here.** `_CONTEXT.md` ("Accepted Revision:
  0.7", "Decomposition Revision: revision 0.7") and `_REFERENCES.md` ("Accepted
  revision 0.7 current decomposition basis") cite SOFTWARE_DECOMP revision 0.7;
  the 2026-06-04 MEMORY entry cites "revision 0.7 plus approved DAG-006". The
  frozen `execution/_Decomposition/SOFTWARE_DECOMP.md` header is `revision: 0.8`,
  `status: current_basis`, and the live DAG is DAG-007. This is pure
  authority-pointer drift (the decomp itself states downstream surfaces "may be
  stale relative to revision 0.8 until refreshed by their owning workflows").
  **No census DECLARED_STATE surface row takes STALE for it**, because the rev-0.7
  pointer lives only in non-census surfaces (`_CONTEXT.md`, `_REFERENCES.md`) and
  in a *dated* MEMORY entry (historical, protected under addendum 1). The
  addendum-1 DECLARED_STATE census is the four-document kit + `_STATUS.md` +
  `MEMORY.md` + in-tree READMEs; `_CONTEXT.md`/`_REFERENCES.md` are not in it.
  `AuthorityNeeded=NO` for pure pointer drift; no owner routing raised on this
  account (the four STALE kit rows route OWNER on the separate setup-only-vs-
  implemented-slice ground, not the rev pointer).
- **SourceReliability keyed to the weakest load-bearing leg (calibration item
  13).** All REQUIREMENT/ACCEPTANCE/EXCLUSION rows are `UNVERIFIED`: their
  load-bearing verification evidence is agent-generated (the re-executed Python
  contract test and/or the recorded DEC-025 sweep) with no human disposition at
  the evidence grain. The DEV-001 `ACCEPT_AS_IS` human disposition covers the
  PKG-02 compatibility *finding* (status separation + hash/provenance), not the
  desktop vitest/playwright evidence, so it does not lift any row to `REVIEWED`.
  DECLARED_STATE prose rows are `NOT_APPLICABLE` (addendum 6). This diverges from
  the R0b DEL-07-05 exemplar's `REVIEWED` requirement rows — expected, since
  DEL-07-05 predates adopted item 13 and is calibration-only.
- **AuthorityNeeded as an adjudication router, not a work queue (calibration item
  14).** No requirement carries `OWNER`/`ENGINEERING`: the contract-level TBDs
  (REQ-002 exact progress phases; REQ-009 unexercised rule-pack/solver-version
  fields) are deferred by the requirements' own wording with no numeric/authority
  claim being promoted, so `AuthorityNeeded=NO`. The four STALE kit rows route
  `OWNER` as R5 document-repair candidates only. No gate-named tokens (e.g. a
  `D-##` token) appear in this deliverable's requirements.
- **Bootstrap _STATUS scoping (addendum 2 / calibration items 5 & 11).**
  DEL-07-07's `## Remaining` holds *only* the seeded `(gated: D-41)` item.
  Transcribed byte-exact into the DECL-005 `RecordedRemaining` (kept `§§6–8` and
  the en-dash; no transliteration). Gate/source cells use the exclusion variant
  (`NONE_RECORDED`, scoped to non-bootstrap content) — not annotated with the
  bootstrap item. No standalone REMAINING_WORK row (addendum 2). This matches
  `DELIVERABLE_INVENTORY.csv` (`NonBootstrapItems=NONE`,
  `SelectableUnderCurrentLoop=NO`).
- **Zero REMAINING_WORK / zero IMPLEMENTED_UNMAPPED rows.** No non-bootstrap
  residual is recorded, and the MEMORY-disclosed "residual TBDs" (no backend
  percent-progress stream; no preemptive mid-solve interruption; panel-level
  headless seams — MEMORY 2026-06-10) are accepted design postures **consistent**
  with the requirements (REQ-003 no-synthesized-percentages; REQ-004
  cancellation-is-a-request/cooperative), not omitted work items — so no
  UNKNOWN/candidate-residual row was raised (contrast DEL-07-05-C09). All material
  surfaces in DEL-07-07's orbit (`core/gui/solve_execution` SURF-078,
  `apps/desktop/src/features/solve` SURF-048,
  `apps/desktop/src/features/diagnostics` SURF-016) are already deliverable-
  attributed in `IMPLEMENTATION_SURFACES.csv`, so no IMPLEMENTED_UNMAPPED rows.
- **Acceptance grain (addendum 12).** VER-07-07-003 (Contract review) and
  VER-07-07-007 (Future implementation test) merely restate requirements
  (REQ-001/002/004/005 and REQ-011 respectively) and were **not** mirrored as
  ACCEPTANCE rows; the remaining five verification items (document, boundary,
  protected-content, semantic-setup, dependency-register) add independent
  acceptance grain and are mirrored.
- **Verification ≠ validation (solver-diagnostics interaction).** All GUI/
  REPORTING rows carry `ValidationEvidence=NOT_APPLICABLE` with the claim class
  reason; no unit/e2e suite was promoted to engineering validation. Solve-
  diagnostics claims that touch PKG-04 solver surfaces (REQ-005/006 diagnostics,
  REQ-004 cancellation, DEL-04-06 diagnostic generation) are recorded as
  concordance facts for THIS deliverable's GUI claims only — no PKG-04 row was
  re-adjudicated.
- **Undated-MEMORY rule (calibration item 9) not triggered.** `MEMORY.md` is
  entirely dated log entries; there is no undated header/state block, so DECL-006
  is ALIGNED (dated records), with the rev-0.7 pointer noted as historical.

## 5. Boundary-compliance statement

All fences held. Discovery was read-only: the only writes were the two output
files under `RUN/WAVES/W3/` (`CLAIM_CONCORDANCE_DEL-07-07.csv`,
`NOTES_DEL-07-07.md`) plus the generator script in the session scratchpad
(outside every project tree). No `_STATUS.md`, register, DAG, product, or
cross-project file was edited; no lifecycle transition applied
(`LIFECYCLE_REASSESSMENT_REQUIRED` not used); no DAG mutation. No release-
readiness, issuance, certification, sealing, professional-approval, or code-
compliance claim appears anywhere in the outputs (F-PIP-1..5). Dispositions are
agent judgments, routed via `AuthorityNeeded`, never phrased as owner or
engineering rulings. No `DEFERRED_AGENT_WORKFLOW` disposition was needed. The
frozen evidence worktree (`.claude-worktrees/piping-frozen-551f84ef6`, HEAD
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`) porcelain was verified **empty before
and after** every re-execution; no writes were made under the frozen tree,
including git-ignored paths.

No STOP-worthy contradiction found.
