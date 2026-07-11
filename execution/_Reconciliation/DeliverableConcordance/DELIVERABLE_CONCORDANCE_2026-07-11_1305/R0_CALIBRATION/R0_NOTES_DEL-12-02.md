# R0 Calibration Notes — DEL-12-02 Private data redaction and export controls

**Run:** DELIVERABLE_CONCORDANCE_2026-07-11_1305 / R0 pilot (activation D-41/DEC-073)
**Reviewed source state:** frozen worktree at `551f84ef6be656f1603ce0acfa5e3935aa9683c7`
(`.claude-worktrees/piping-frozen-551f84ef6`, project root `projects/chirality-piping`)
**Ledger:** `R0_CLAIM_CONCORDANCE_DEL-12-02.csv` — 32 rows
(15 ALIGNED, 12 PARTIALLY_IMPLEMENTED, 3 STALE_SETUP_SPECIFICATION,
1 IMPLEMENTED_DIFFERENTLY, 1 IMPLEMENTED_UNDOCUMENTED; 0 UNKNOWN)

This is an agent-authored evidence artifact. No disposition here is an owner or
engineering ruling; no lifecycle state was changed; no compliance, assurance,
release-readiness, certification, or professional claim is made or implied.

## Sources read (repo-relative, frozen tree)

- `plans/PLAN_2026-07-10_deliverable_implementation_reconciliation.md` (§§3, 4, 5, 6, 7 method)
- Deliverable kit: `execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-02_Private data redaction and export controls/`
  `Specification.md`, `Datasheet.md`, `Guidance.md`, `Procedure.md` (scanned), `_STATUS.md`,
  `_CONTEXT.md`, `_REFERENCES.md`, `MEMORY.md`, `Dependencies.csv`, `_SEMANTIC.md` (grep),
  `_SEMANTIC_LENSING.md`
- Run records: `_run_records/WORKING_ITEMS_RUN_2026-07-10_TP-E4-REDACTION-001.md` (full read),
  `_run_records/` listing, `TASK_RUN_2026-06-07_0935_redaction-export-hardening.md` (existence),
  package `_run_records/WORKING_ITEMS_RUN_2026-06-07_0957_TP-PKG12-REDACTION-SECRET-GUARD-CLOSEOUT.md` (existence)
- Loop: `loop/LOOP_RECEIPTS.md` Receipts 11 and 12; `loop/` listing (WORKPLAN_2026-07-10 is newest)
- Decisions: `execution/_Coordination/_DECISIONS/_REGISTER.md` D-41 row;
  `D-41_concordance_activation.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` DEC-054 row
- Implementation: `core/security/redaction/{__init__.py,controls.py}`,
  `schemas/redaction_export_controls.schema.yaml` (enum sections),
  `apps/desktop/src/features/redaction-controls/` (all four files, sizes/structure),
  `apps/desktop/src/App.tsx` (wiring), `fixtures/redaction_export_controls/cases.json` (existence),
  `docs/security/redaction_export_controls.md` (Application Binding section),
  `apps/desktop/src/features/export-review/ExportReviewPanel.tsx` (unit_evidence_matrix location),
  `core/reporting/protected_content_linter` (existence), `plans/PLAN_2026-06-17_prd_completion.md` rows E4–E8
- Checks re-executed at the frozen SHA (read-only; frozen tree verified clean after):
  `python3 -m pytest tests/security/test_redaction_export_controls.py -q -p no:cacheprovider`
  (PYTHONDONTWRITEBYTECODE=1) → 11 passed;
  `tools/validation/validate_dependencies_schema.py` on `Dependencies.csv` → VALID/29 cols/exit 0;
  MatrixError grep → 0; lens-coverage row count → exactly 96.
  Desktop vitest/Playwright were NOT re-executed (npm install would write into the frozen tree);
  vitest results are cited from the TP-E4 run record and marked as not re-executed.

## Calibration finding of record (the boundary this pilot was asked to test)

The known context verified TRUE: `_STATUS.md` `## Remaining` (updated 2026-07-11) accurately
distinguishes the landed application-side redaction binding (E4, `TP-E4-REDACTION-001`, PR #167)
from the residual REXC-REQ-012 breadth, while older deliverable prose still describes runtime
integration as wholly absent:

- `Specification.md` Scope ("still does not ... integrate runtime report/export routes"),
  the Documentation deferred list ("runtime report/export integration; GUI controls"),
  `Datasheet.md` Implementation status, and `Guidance.md` REXC-OI-002/REXC-OI-007 all predate
  the 2026-07-10 binding.
- Boundary applied: because the **status file is current** and the **spec prose lags**, these rows
  are `STALE_SETUP_SPECIFICATION` (C04, C05, C07), NOT `REMAINING_STATE_MISMATCH`. I reserved
  `REMAINING_STATE_MISMATCH` for defects in `## Remaining` itself (landed item still recorded,
  evidence-backed residual omitted, or wrong ownership/gate metadata) — none found here; the
  residual row is `ALIGNED` (C08). `ALIGNED` was used only where wording, implementation, and
  evidence agree (e.g. REXC-REQ-004, REXC-REQ-007, the VER rows, exclusions).
- Requirement rows carry the substance gap (`PARTIALLY_IMPLEMENTED` for breadth) while the prose
  lag is ledgered separately on the declared-state claims. This split kept dispositions
  single-valued without losing either signal.

## Method ambiguities hit (calibration friction — as valuable as the ledger)

1. **§6 column count.** The plan's ledger table has 20 rows but one row names two columns
   (`PackageID` / `DeliverableID`). The dispatch said "the exact 20 ledger columns", and the
   sibling R0 pilot (DEL-10-05) had already emitted a single combined `PackageID/DeliverableID`
   column with values like `PKG-10/DEL-10-05`. I matched that convention for run consistency
   (value `PKG-12/DEL-12-02`; `UNMAPPED` for the unmapped row). The kernel/ratified method should
   fix one reading explicitly.
2. **Does SECURITY class require validation evidence?** §6 mandates validation only for mechanics
   claims. For security-behavior claims I recorded `ValidationEvidence = NONE_FOUND` with the
   explicit note that legal/security sufficiency review is deferred (REXC-OI-005; §17.5 owner-gated),
   and did NOT downgrade otherwise-complete rows to `VERIFIED_NOT_VALIDATED`, because the
   deliverable's own accepted scope defers sufficiency review — treating that as a missing
   validation basis would manufacture a gap the scope does not claim. One row (REXC-REQ-007
   non-mutation) is `ALIGNED` with `ValidationEvidence NOT_APPLICABLE` as a behavioral invariant.
   A ruled checking-entry profile for SECURITY_CONTROL deliverables should settle this class rule.
3. **Verification-column claims embedded inside requirement rows.** Spec requirement rows carry
   their own declared-state text in the Verification column (e.g. "final UI/CLI approval flow
   remains `TBD`", "runtime source-model integration remains `TBD`") which is itself stale for the
   landed slice. I kept one row per requirement and noted the column lag in DeclaredState/RemainingWork
   rather than splitting each requirement into requirement+verification-prose claims; the boundary
   between "requirement claim" and "declared-state claim inside a requirement row" needs a rule.
4. **Cross-deliverable residual homing (REXC-REQ-003).** The protected-content linter exists at
   `core/reporting/protected_content_linter` and its integration/release-scan work is recorded under
   DEL-08-05 / completion-plan E7 (gate D-20), not in DEL-12-02's `## Remaining`. Per the §5 rule
   ("absent means no recorded item beyond specification fallback") I recorded `NONE_RECORDED`
   rather than flagging an omission. Whether a deliverable must mirror residuals owned elsewhere
   is a method question for R3 cross-package reconciliation.
5. **Frozen-tree verification reproduction limits.** Python-side tests could be re-run without
   writing to the frozen tree; the desktop vitest surfaces could not (dependency install writes).
   The contract should say explicitly when run-record citation suffices for `VerificationEvidence`
   and how to mark "not re-executed" (I marked it inline).
6. **D-41 state inside the frozen tree.** At `551f84ef6` the D-41 register row is `AWAITING_RULING`
   and the seeded bootstrap item is still `(gated: D-41)`; the activation (D-41/DEC-073) named in
   this dispatch post-dates the freeze. Consequence recorded, not resolved: rows citing gate state
   reflect the frozen source state, and the run's authority comes from the dispatch, not the frozen
   register. R1 should re-verify the live register row per §3 boundary 6 / §8 program-state homes.

## Special-rule handling — bootstrap item

Per dispatch, `_STATUS.md` `## Remaining` item 2 is program mechanics and got NO claim row.
Verbatim, for the record:
`Run claim-level concordance per the reconciliation method (source: plans/PLAN_2026-07-10_deliverable_implementation_reconciliation.md §§6–8 at the D-41-pinned main revision) (gated: D-41)`
Only residual item 1 (REXC-REQ-012 breadth) was treated as a deliverable claim (row DEL-12-02-C08,
copied verbatim into `RecordedRemaining` wherever it applies).

## AUTHORITY_CONFLICT

None found. No two live normative sources conflicted; all divergences were current-status-vs-stale-prose
(dispositioned as staleness) or naming drift (IMPLEMENTED_DIFFERENTLY, DEL-12-02-C06).

## Deferred / flagged rows

- `UNMAPPED-DEL-12-02-01` (Export Safety Review `unit_policy_summary`/`unit_evidence_matrix` in
  `apps/desktop/src/features/export-review/ExportReviewPanel.tsx`): maintained through eleven
  DEL-12-02-primary/supporting TP-UNITS-BTAIL tranches under DEC-018, but mapped to no REXC
  requirement. `IMPLEMENTED_UNDOCUMENTED`, `AuthorityNeeded=OWNER` — ownership may belong to
  DEL-02-02/DEL-08-04/DEL-12-03; per §3 boundary 1 no scope absorption is proposed.
- `REXC-REQ-003` carries `AuthorityNeeded=REVIEW` (the §17.5 legal review workflow is a
  human-gated surface; this pilot cannot and does not assess redaction/legal sufficiency).
- No `DEFERRED_AGENT_WORKFLOW` rows: no claim resolution depended on agent-instruction or
  skill-contract analysis. Agent files were read only as frozen process inputs (none needed).
- `fixtures/redaction_export_controls/cases.json` is not listed in the Specification's evidence
  list (it post-dates it); treated as verification evidence under REXC-REQ-014/parity rather than
  as unmapped implementation — R5 document repair should add it to the evidence list.
