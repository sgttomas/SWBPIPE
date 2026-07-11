# R0b Calibration Notes — DEL-07-05 Results viewer

**Run:** DELIVERABLE_CONCORDANCE_2026-07-11_1305 (activation D-41/DEC-073), R0b second calibration round
**Pilot deliverable:** DEL-07-05 "Results viewer" (PKG-07, GUI class)
**Frozen evidence tree:** `/Users/ryan/ai-env/projects/chirality/.claude-worktrees/piping-frozen-551f84ef6` at commit `551f84ef6be656f1603ce0acfa5e3935aa9683c7`
**Ledger:** `R0B_CLAIM_CONCORDANCE_DEL-07-05.csv` — 23 claim rows
**Disposition histogram:** ALIGNED 16 · PARTIALLY_IMPLEMENTED 2 · STALE_SETUP_SPECIFICATION 4 · UNKNOWN 1

## 1. Sources read

Method (working tree, permitted exception):
- `plans/PLAN_2026-07-10_deliverable_implementation_reconciliation.md` §§3, 6, 7 (frozen tree copy — the pinned revision)
- `R0B_CALIBRATION/R0B_CONVENTIONS.md` (working tree)

Deliverable folder (frozen tree):
- `Specification.md`, `Datasheet.md`, `Guidance.md`, `Procedure.md`
- `_STATUS.md` (incl. `## Remaining`), `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `Dependencies.csv`, `MEMORY.md`, `_REVIEW.md`, `Review_Findings.csv`
- `_run_records/`: TASK_RUN_2026-05-08 (Type 2 implementation), TASK_RUN_2026-06-06 CHECKING_READINESS_REVIEW, TASK_RUN_2026-06-12_1110, WORKING_ITEMS_RUN_2026-06-11 (results family selector; viewport deformation overlay), WORKING_ITEMS_RUN_2026-06-17/18 (unit policy / lint units) — via MEMORY summaries plus targeted reads

Implementation / verification / coordination (frozen tree):
- `core/gui/results_viewer/{__init__.py,engine.py}`; `tests/test_results_viewer_contract.py`
- `apps/desktop/src/features/results/{ResultsPanel.tsx,resultInterpretation.ts}`; `apps/desktop/src/App.test.tsx` (grep-level); `apps/desktop/e2e/r2-smoke.spec.ts` (results/report sections); `apps/desktop/SMOKE.md` (TP-MAC index; entries 89/108/142/215 located)
- `schemas/results.schema.yaml` (basis enum spot-check); `fixtures/product_preview/invented_mechanics_result.json` (grep-level: 830-row state, no ratio/equipment rows, 47 rad rows)
- `execution/_Coordination/_COORDINATION.md` (H4 ruled workflow record); `execution/_Coordination/_DECISIONS/_REGISTER.md` (D-41 row, frozen state AWAITING_RULING); `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 (DEC-018, DEC-025 resolved)
- `docs/PRD.md` FR-015; `plans/PLAN_2026-06-17_prd_completion.md` §4 (FR-015 A6 residual resolution)
- `validation/evidence/sweeps/SWEEP_20260711T040758Z_e648462f1d05.json` (DEC-025 five-surface sweep, overall pass)
- DEL-16-02 `_run_records/WORKING_ITEMS_RUN_2026-07-10_TP-APP-R5-FIELDRULES-001.md` (sweep addendum locating the near-frozen sweep chain)
- `DEL-07-01 .../_STATUS.md` (convention-3 homing check)

Side-effect-free re-executions inside the frozen tree (convention 7), `git status --porcelain` verified clean after each:
1. `PYTHONDONTWRITEBYTECODE=1 python3 tests/test_results_viewer_contract.py` → PASS
2. `python3 tools/validation/validate_dependencies_schema.py .../Dependencies.csv` → VALID (29 required columns, 19 rows)
3. Git-plumbing reads only (log/diff/merge-base) to establish that no product-code path (`apps/`, `core/`, `tests/`, `fixtures/`, `schemas/`) changed between sweep commit `e648462f1` and frozen HEAD `551f84ef6` — the recorded desktop Vitest/Playwright passes therefore bind a product-code state byte-identical to the frozen SHA.

Not run: Playwright e2e, Vitest, desktop build (not side-effect-free; recorded evidence cited with the convention-7 marker).

## 2. Per-convention assessment

**C1 — Stale-prose two-signal split (D1). Applied, unambiguous, changed encodings.**
Applied throughout. Without it I would have marked REQ-07-05-009 ("the future implementation shall include UI tests") and VER-07-05-005 ("remains TBD until source implementation exists") as STALE_SETUP_SPECIFICATION; under C1 they took substance dispositions (ALIGNED / PARTIALLY_IMPLEMENTED) and the tense staleness went to the per-surface row DEL-07-05-C02. It also cleanly split the Scope-paragraph exclusion: the durable product boundary became EXCLUSION row C01 (ALIGNED) while the overtaken "does not implement GUI source code, tests, schemas" clause went to C02. Verdict: applied-clean.

**C2 — DECLARED_STATE ClaimType (D2). Applied, unambiguous.**
Six DECLARED_STATE rows (C02–C07), one per prose surface (Specification, Datasheet, Guidance, Procedure, MEMORY, _STATUS). The enum extension worked without strain. One open grain question (see §3, MEMORY). Verdict: applied-clean.

**C3 — Residual homing before mismatch (D3). Applied, with friction.**
Two candidate omitted residuals from the 2026-06-12 run record:
- *Schema basis enum* ("`ResultMetadata.basis` enum does not yet list `solved_from_global_linear_system`; left for the schema owner") — resolved in the frozen tree (`schemas/results.schema.yaml` line 607 lists the value). Landed; no finding, no row.
- *Rotational-deformation visualization* (rx/ry/rz emitted, not rendered) — homing check found it in neither DEL-07-05 nor DEL-07-01 `_STATUS.md`; `plans/PLAN_2026-06-17_prd_completion.md` marks FR-015 met without it. The convention's branch structure (homed-elsewhere → no finding; unresolved → UNKNOWN) worked, and row C09 took UNKNOWN with the smallest next check. **Friction:** the convention assumes the item *is* a residual and only its home is in question; here the prior question — whether a disclosed limitation in a run record is a residual at all when the governing plan assessment closed the parent requirement without it — is not covered. Without the convention I would have been tempted to record REMAINING_STATE_MISMATCH (evidence-backed residual omitted); C3 correctly pushed me to UNKNOWN instead. Verdict: applied-with-friction.

**C4 — ID column controlled values (D4). Applied, unambiguous.**
All rows `PKG-07/DEL-07-05`. Ownership observations for adjacent surfaces (ResultExportPanel → DEL-08-04) stayed in evidence cells and notes, never the ID cell. Not stress-tested (single-owner deliverable, no UNMAPPED rows). Verdict: applied-clean (lightly exercised).

**C5 — Column defaults and rubrics (D5, T2). Applied, minor friction.**
`GateOrStageConstraint`: NONE_RECORDED on no-residual rows; UNGATED on rows touched by the gate-suffix-free ratio residual — unambiguous and it changed what I would otherwise have written (I would have defaulted to UNGATED everywhere). `SourceReliability` rubric applied (REVIEWED for project records; NOT_APPLICABLE for declared-state prose rows; UNVERIFIED for the C09 question). **Friction (a):** the ClaimClass rubric addresses diagnostics/warning *behavior* claims; REQ-07-05-006 is a diagnostics-*visibility* GUI claim — I kept GUI, but a literal reading of the rubric could send it to WORKFLOW. A GUI-panel carve-out would remove the wobble. **Friction (b):** the C07 (_STATUS) row carries two residuals with different gate states; I encoded both in one cell ("UNGATED (ratio item); (gated: D-41) (bootstrap item…)") — the single-value column contract does not anticipate multi-residual declared-state rows. Verdict: applied-with-friction.

**C6 — Mechanical selectability + SECURITY encoding (D6, T3). Applied (selectability); SECURITY not exercised.**
Selectability derived mechanically: deliverable IN_PROGRESS + ungated recorded residual → YES on rows the ratio residual touches; NO elsewhere (nothing recorded to select). The owner suspension was NOT written into any row (run-level caveat per RUN_BASIS). Without C6 I would likely have put UNKNOWN per-row citing the suspension. No SECURITY-class claims in this deliverable, so the SECURITY encoding was not exercised. Verdict: applied-clean (SECURITY branch not-exercised).

**C7 — Evidence execution and basis resolvability (D7, A2–A4). Applied, unambiguous, high value.**
Re-executed two genuinely side-effect-free checks (contract test, dependency-schema validator) and verified the frozen tree clean. All other passes cited as recorded with run-record/artifact + commit binding + the standardized marker `not re-executed at frozen SHA 551f84ef6`. The byte-identity proof (sweep commit `e648462f1` → frozen HEAD touches no product code) materially strengthened the recorded citations. DecisionBasis resolvability enforced: DEC-018/DEC-025 resolved to `SOFTWARE_DECOMP.md` §12, H4 to `_COORDINATION.md`, SCA-001 to `_CONTEXT.md` + decomposition, Receipt 12 to `loop/LOOP_RECEIPTS.md`; no ATTESTED markers needed; stage context (DEC-053/054) correctly excluded as non-governing. Verdict: applied-clean.

**C8 — Disposition precedence (D8, D9, FN1, FP2). Applied, with friction.**
- *ACCEPTED_DIVERGENCE > ALIGNED:* considered for REQ-07-05-005/C08. I used PARTIALLY_IMPLEMENTED for the requirement and ALIGNED for the residual row, because the residual's provenance (owner-adopted plan consolidation rehoming, Receipt 12) is an adoption of a work surface, not a decision *permitting* the divergence. **Friction:** the precedence rule tells me ACCEPTED_DIVERGENCE beats ALIGNED "when both fit a residual row" but gives no test for when a recorded residual's provenance rises to a "human decision permits" basis; a one-line test (named ruling/decision ID vs. mere recorded residual) would settle it.
- *STALE_REVIEW_OR_EVIDENCE clause:* the deliverable-local recorded passes (2026-06-17/18, 737-row era) are overtaken by the frozen tree (830-row fixture after DEC-067 reconciliation). The clause as written covers *validation* evidence only; for these GUI rows the class-required evidence is verification. I avoided the question by citing the near-frozen sweep with the byte-identity proof, but had that sweep not existed the conventions would not have told me whether overtaken *verification* passes force STALE_REVIEW_OR_EVIDENCE (see §3).
- *IMPLEMENTED_UNMAPPED grain:* applied at material-surface grain; result: **none**. Candidate surfaces all resolved to owners — viewport deformation overlay maps to SOW-023 graphical review / `_CONTEXT.md` "visual overlays" via this deliverable's own run records; ResultExportPanel self-declares DEL-08-04; comparison/report panels are PKG-14/PKG-08. Sub-panel internals (DualUnitCompanion, MechanicsGapLedger) are below the grain, as intended.
- *RecordedRemaining verbatim only where touched:* ratio residual copied verbatim on REQ-001, REQ-005, VER-005, C08; both items verbatim on C07 (the _STATUS surface row); NONE_RECORDED elsewhere. D-41 gate cells reflect the frozen register state (AWAITING_RULING) without re-derivation.
Verdict: applied-with-friction.

## 3. Residual ambiguities the conventions do not cover

1. **Is a run-record "residual" a residual?** (C09 driver.) Conventions cover homing an omitted residual, not whether a disclosed limitation counts as one when the governing plan assessment closed the parent requirement without it. Recommend: a rule that run-record residual lists are candidate evidence only, with `## Remaining` + the governing plan/decision surfaces as the arbiters.
2. **Overtaken verification (not validation) passes.** Convention 8's staleness clause names class-required *validation* evidence. For GUI/H4 deliverables the class-required evidence is verification; whether a recorded verification pass that predates material fixture/expectation changes forces STALE_REVIEW_OR_EVIDENCE absent bind-current proof is unstated. Here the near-frozen DEC-025 sweep + byte-identity check made it moot; a corpus run cannot count on that.
3. **MEMORY.md as a declared-state surface.** Dated durable-context entries are historical records, not current-state claims; whether stale quantitative details inside dated entries (737 vs 830 rows) constitute declaration staleness is a grain question. I encoded ALIGNED/MEDIUM with a note; a convention should say whether dated-entry drift is ledgerable at all.
4. **Multi-residual gate cells on surface rows.** See C5 friction (b): one declared-state row for `_STATUS.md` carries two residuals with different gate states; the single-value `GateOrStageConstraint` contract has no composition rule.
5. **Program-mechanics bootstrap item.** The special rule ("copy verbatim only where RecordedRemaining requires, exclude from residual analysis") worked, but I had to decide that the only row it "touches" is the `_STATUS.md` surface row (C07). A corpus-run convention should fix that reading, or R0b pilots will diverge on whether the bootstrap item appears in zero, one, or all RecordedRemaining cells.
6. **Selectability semantics for non-residual rows.** I read `SelectableUnderCurrentLoop=NO` as "no recorded item to select" for requirement rows without residuals. An alternative reading (NOT_APPLICABLE-like UNKNOWN) is defensible; the column contract's YES/NO/UNKNOWN enum has no explicit meaning for claim rows that carry no work item.
7. **Stale tool references in Procedure verification tables.** Procedure.md names `check_four_documents.sh` / `validate_enum.py`, which do not exist at the frozen SHA. I folded this into the C05 STALE_SETUP_SPECIFICATION row; arguably it is also a REMAINING_STATE_MISMATCH-adjacent metadata staleness (a named verification method no longer executable). Conventions do not say which disposition owns dead tool pointers.

## 4. Deferred claims

- **DEL-07-05-C09** (rotational-deformation visualization residual status): held at UNKNOWN, smallest next check recorded (owner confirmation of deferred-scope vs disclosed-boundary status). Not deferred for agent-workflow or concurrency reasons — it is an authority question.
- No DEFERRED_AGENT_WORKFLOW rows: no claim resolution depended on agent instructions, skill contracts, or workflow semantics.
- No claims deferred for concurrent-implementation reasons (suspended frozen tree; no overlapping active work observed on results surfaces).

## 5. Frozen-tree hygiene attestation

All evidence reads and the three re-executed checks ran inside the frozen worktree; `git status --porcelain` was verified clean after each execution batch. No file in the frozen worktree was modified. The only writes of this pilot are the two files in `R0B_CALIBRATION/`.
