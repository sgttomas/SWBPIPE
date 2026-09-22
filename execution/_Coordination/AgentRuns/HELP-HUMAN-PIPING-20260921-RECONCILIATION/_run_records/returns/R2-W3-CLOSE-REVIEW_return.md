VERDICT: FINDINGS

# R2-W3-CLOSE independent review (TASK, fresh context)

Run HELP-HUMAN-PIPING-20260921-RECONCILIATION. Scope: f250882c1..4172aa98e (W3 rolling queue, W3 and cross-wave resolutions, W3_ASSESSMENT.md, departures). The reviewer did not take part in the run. Nothing is BLOCKING. Three SHOULD-FIX items should be settled before R3: one resolutions row pair and two gaps in the assessment's disclosure. Paths below are relative to RUN = `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS` and AR = `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260921-RECONCILIATION`.

## Findings

1. **SHOULD-FIX: the two DEL-09-04/05 CONTEXT rows have a column shift.** The rows are in `WAVES/W3/RESOLUTIONS.csv`, keys `DEL-09-04:CONTEXT` and `DEL-09-05:CONTEXT` (WEAK, source PKG-09 §4.2 W-6).
   - Each row has `BaselineClass=NONE` and `DivergenceLayers` empty. Its own Basis says "the other four columns follow the sealed ALIGNED convention (as DEL-09-01..03:CONTEXT)". Those three sealed rows carry `DivergenceLayers=NONE` and an empty BaselineClass. So `NONE` landed one column early.
   - These are the only resolution rows in any wave where ALIGNED has DivergenceLayers other than NONE (checked by script across W1, W2, W3 and CROSS_WAVE).
   - The batch result is not affected today. After the override the rows form the group `CP-02/ALIGNED`, and they are its only two members. But the batch overlay sets DivergenceLayers to an empty string, and R3 reads these values. Fix it in the resolutions file, which is not a sealed ledger. The combined file must then be rebuilt.

2. **SHOULD-FIX: the assessment omits what the DEL-15-02 rerun verifier found.**
   - The rerun verifier found one firm false alignment, `DEL-15-02:SOW#CLM-020`, in a targeted check outside its sample. It is the same defect class that triggered the rerun (PKG-15_RERUN1_VERIFICATION.md §2, §3, §4). Counting the targeted checks, the rate is 1/11 = 9.1%.
   - RUN_STATE.jsonl L356 records this, and a FIRM row (`DEL-15-02:SOW#CLM-020`) corrects it.
   - The W3_ASSESSMENT.md table (L24) says only "rerun slice 0.0% on the deterministic sample", and L29 says both slices were "accepted". That is accurate but leaves out the one fact that bears on whether the rerun worked. Add a sentence to L24 or L29.
   - The same kind of omission appears elsewhere. The assessment gives the out-of-sample figure for PKG-06 (1.8%) but not for PKG-11 (2/51 = 3.9%, PKG-11 §3) or PKG-13 (2/38 = 5.3%, PKG-13 §3, above 5%).

3. **SHOULD-FIX: the owner-item list (W3_ASSESSMENT.md L61-86) leaves out items that the verifiers' "for the owner" sections name.**
   - PKG-13 §7 item 4: the desktop Knowledge panel does not validate against the design-knowledge schema (DEL-13-01 REQ-13-01-011, INVARIANT, OWNER).
   - PKG-13 §7 item 5 and PKG-15 §7 item 4: the desktop shows constraint-validation status that no engine produced, and no product path emits schema-compliant PKG-15 artifacts.
   - PKG-12 §7 item 3: CF-001 and CF-002 (secret provider and encrypted-storage default) are UNKNOWN pending an owner ruling.
   - PKG-17 §7 item 7: the PDU-031 OWNER_HOLD, the desktop GUI panels that contradict the SOW GUI exclusions, and the `openpipestress_jcs_ijson_v1` frozen-contract rename (also PKG-14 §7 item 5).
   - PKG-15 §7 item 6: no DivergenceLayers value fits a pure product-implementation gap.
   - The Method list should also carry a post-gate question: should a package above 5% after the gate (PKG-12 at 9.5%) go to the owner? Departure 4 is an Agent 0 reading of a plan that says nothing on this point (see Q4).
   - Two items in the list come from worker notes, not verifier reports. "Section unit other than m treated as millimetres" is in `DEL-17-07_notes.md` L26-27, and REXC-CON-002 is in `DEL-12-02_notes.md` L16. The list should say they are not verifier-confirmed.

4. **MINOR: the tier hint also reached the DEL-15-02 rerun verifier.** The disclosure (RUN_STATE L357; W3_ASSESSMENT.md departure 5) says the rerun verifier "confirmed the underlying defects independently".
   - The verifier read the manager's `LAUNCH_G1.md` "as context only" (PKG-15_RERUN1_VERIFICATION.md L10-13). The hint is in that file: "the verifier's view is that the correct tier is INVARIANT" (AR/_run_records/W3-PKG-15-RERUN1-MANAGER/LAUNCH_G1.md L36).
   - The code defects were confirmed independently from contract.py line references, but the verifier's confirmation of the INVARIANT tier was made with the hint in view. Add this to the disclosure.
   - The disclosure is also slightly broader than it needs to be. The hint named only Units and Missing-values, while the disclosure also covers Provenance (`.r03`). That errs on the safe side.

5. **MINOR: W3_ASSESSMENT.md L35 says "the adopted W1 and W2 files are unchanged", which is true only for the cross-wave step.**
   - In this range, commit 0aec23cea changed W1/RESOLUTIONS.csv (7 rows) and W2/RESOLUTIONS.csv (23 rows). Only the Class (AGENT_READING to OWNER_CONFIRMED) and Basis changed, under Direction 8. No values changed (checked field by field).
   - The same commit clarified CONVENTIONS F3 while the W3 queue was running. Both changes are disclosed as BOUND_INPUTS (RUN_STATE L213-214) but are not listed in the assessment's Departures.
   - Departure 4 says "no mid-queue brief change". That is true of the briefs, but a bound convention did change mid-queue, and the change produced PKG-06 X3 and PKG-09 W-2.

6. **MINOR: two keys appear twice in W3/RESOLUTIONS.csv**, `DEL-12-04:SOW#CLM-011.r01` (FIELD and OBSERVED) and `DEL-11-04:SOW#CLM-011/R-DEL-11-04-002` (CONTESTED and FIELD). All four rows have empty values, so the validator, which keeps the last row per key, is unaffected. R3 should read both rows of each pair.

7. **MINOR: PKG-17 W-1c was relabelled.** The verifier rated W-1c (DEL-17-07/08/09 CONTEXT#architecture-basis-injection) FIELD. The resolutions record it as WEAK with values that change the main row's disposition, IMPLEMENTED_DIFFERENTLY · PROJECT_BASELINE becoming STALE_REVIEW_OR_EVIDENCE · LOCAL_DESIGN. The OtherCorrections column discloses this ("Verifier rated W-1c FIELD, not weak"), and the values follow the verifier's own right values. The verifier's label and its values do not agree with each other, so R3 should know about the relabel.

## Q1 Resolutions fidelity

- The adopted file has 425 rows: the 418 rows of drafts 1, 2A, 2B and 2C, verbatim as whole-row multiset matches with none missing or altered, plus exactly 7 rows sourced from `CROSS_PACKAGE/W3_CROSS_PACKAGE_VERIFICATION.md`: 6 RESOLVED_PAIR and 1 FIRM, which matches that report's "3 LEGITIMATE, 4 CORRECT".
- Rows that change what batch mode sees: the validator overrides the four consistency fields for every row with a non-empty Disposition. Of the 425 rows, 32 change values: 15 FIRM, 13 WEAK and 3 RESOLVED_PAIR, plus one cross-package FIRM row that is counted among the 15. Another 59 rows set values equal to the sealed ones (49 OWNER_CONFIRMED, 9 RESOLVED_PAIR, 1 WEAK). All 335 CONTESTED, FIELD and OBSERVED rows leave the values empty.
- I checked every value-changing row against its report's "Right values", "Preferred values" or summary table:
  - PKG-05 F-1; PKG-06 X1 and X3;
  - PKG-08 F-1 and F-2; PKG-09 W-2 and W-6;
  - PKG-10 F-1; PKG-11 F-1, F-2, F-3 and the out-of-sample CLM-004.r01;
  - PKG-12 F-1, F-2 and F-4; PKG-13 F1 and W2; PKG-15 W1;
  - PKG-17 W-1a, W-1b and W-1c; PKG-15-RERUN1 CLM-020;
  - cross-package 1a, 1b, 1c and 3; cross-wave 3.
- All match, apart from finding 1 and the relabel in finding 7. No values were invented.
- Where the verifier offered a defensible alternative, Agent 0 recorded the row as CONTESTED with empty values rather than choosing one. Examples are PKG-12 F-3 (tier) and PKG-06 X2 (conditional on G1). That is conservative.
- Beyond the value rows, I spot-checked 12 more rows chosen at random (seed 7) across the CONTESTED, FIELD, OBSERVED and OWNER_CONFIRMED classes. All are faithful to their source reports.
- PKG-10 FD-1: every DEL-10-04/05 ledger row that fits the pattern is covered (74/74 and 41/41 for NOT_APPLICABLE evidence, 76/76 and 64/64 for empty DecisionBasis). The verifier's own counts were 76 and 42; its §4.3 prose is slightly off, not the resolutions.
- Missing disagreements: a script collected every claim key cited in each report's §4 that is not in the resolutions. The only keys it found are agreed items ("right", "correct", "upheld") or first-run DEL-15-02 keys whose ledger was superseded. No disagreement is missing.
- Every RESOLVED_PAIR row, in W3 (12) and CROSS_WAVE (5), has a Source containing `_VERIFICATION.md`, as validator L408 requires.

## Q2 Batch claims (re-run by the reviewer with PYTHONDONTWRITEBYTECODE=1 and shell globs)

- W3 batch (58 ledgers, W3/RESOLUTIONS.csv): `PASS batch of 58 ledgers: 0 consistency findings`, rc=0.
- Corpus batch (102 ledgers, CROSS_WAVE/ALL_WAVES_RESOLUTIONS_COMBINED.csv): `PASS batch of 102 ledgers: 0 consistency findings`, rc=0.
- The glob `WAVES/W*/PKG-*/DEL-*/DEL-*_forward.csv` matches 102 files, none under `superseded_1/`. The three superseded forwards (DEL-03-07, DEL-13-02, DEL-15-02) sit one level deeper and are not matched.
- The combined file's 599 data rows, parsed as CSV, equal the W1 (47), W2 (122), W3 (425) and CROSS_WAVE (5) data rows in that order, with one `#END` sentinel (599). It is not byte-identical to a plain `cat`, only because each part carries its own `#END` row. The content claim holds.

## Q3 Rerun procedure

- Both reruns followed WAVE_PLAN "Verifier reruns":
  - a fresh manager with the same brief, dc8a6da2 (RUN_STATE L326 and L330);
  - one worker, taking the rerun clause;
  - a fresh verifier at STANDARD sampling (L345 and L353);
  - one cycle each.
- The launch hashes in RUN_STATE match the launch files.
- Superseded files: for both deliverables, the `superseded_1/` forward, reverse, notes and SEAL files are byte-identical to the committed first-run files at the parent of the commit that added them (4e75ffdf3 for DEL-13-02, df3fce723 for DEL-15-02). Each superseded SEAL hash matches its superseded forward (5b5ab108… and 86d7e398…). The fresh seals match too; DEL-15-02 is 989d1fc9….
- The DEL-13-02 launch names only the defect. The DEL-15-02 launch also states the verifier's tier. The stated effect is accurate: AuthorityTier on `CLM-005.r01`-`.r03`. The disclosure is adequate apart from finding 4.
- Agent 0 applied the literal rule to PKG-15 (RUN_STATE L325) and did not rerun PKG-08, whose firm row was in the 10% class. Both follow the written rule, so the reading was applied consistently, and it goes to R4.

## Q4 Departures

1. **529 pauses** (RUN_STATE L263-307): disclosed. Resuming by message keeps both context and seals. The pauses do not affect independence.
2. **PKG-11 resume** (L298): disclosed.
   - `INTERRUPTED_G1_attempt1.txt` shows the seals of the three interrupted forwards re-verified PASS. The current DEL-11-01, 02 and 03 forward hashes (cd1f14…, fc15e5…, c5c5eb…) equal those seals.
   - There is no `superseded_*` directory under W3/PKG-11. The rerun worker wrote nothing.
   - Resuming a worker is not a rerun, and a later verifier covered the whole package. Independence, sealing and coverage are not weakened.
3. **Agent 0 G2 launches for PKG-11 and PKG-14** (L308): disclosed, and the hashes match.
   - Diffing each launch against the manager's sealed `LAUNCH_G2.md` shows only an appended dispatch note about routing and scratch hygiene, with no content hints.
   - The launches fit AGENTS.md (HELP_HUMAN may dispatch Type 2 work directly). No guarantee is weakened.
4. **PKG-12 at 9.5%** (L327): disclosed.
   - WAVE_PLAN "Later waves" does set per-package conditions after the gate, but it names no per-package 5% limit. The 5% condition appears only in the gate definitions (L6, L42, L90). The reading is supported by the text.
   - All four firm errors are corrected or contested in the resolutions: F-1, F-2 and F-4 as FIRM, F-3 as CONTESTED.
   - Because the plan says nothing on this point, it should be an explicit owner item (finding 3).
5. **DEL-15-02 hint** (L357): see Q3 and finding 4.
6. **PKG-17 WEAK ratings not upgraded** (L360): disclosed, and correct in principle, because the rating belongs to the verifier.
   - The rows are settled through F6 with the verifier's values, which is the route the verifier itself recommended (PKG-17 §2 and §7 item 8).
   - If the owner reads them as firm, reruns are owed. The assessment should keep that in the R4 rerun-rule item.
7. **Scratch collision** (L278): disclosed, and the seals were re-verified. No effect.

## Q5 Coverage (script over all 102)

- DELIVERABLE_INVENTORY.csv lists 102 deliverables. There are 102 current forwards, split 13, 31 and 58 across the waves, with no missing or extra deliverables.
- The recomputed forward SHA-256 matches the SEAL for all 102. A reverse file and a notes file exist for all 102.
- Single-mode validation with `--inventory ROUTING/<PKG>_capabilities.csv`:
  - All 89 W2 and W3 ledgers pass with `--notes-gap`.
  - DEL-07-09 also passes with `--notes-gap`.
  - All 13 W1 ledgers pass without `--notes-gap`. Twelve of them do not pass with it, as expected: they were sealed before Part F.
- Every package's final verdict is ACCEPT WITH CONTESTED ROWS. The W2 PKG-03, W3 PKG-13 and W3 PKG-15 verdicts come after one rerun each.
- All 32 file hashes recorded in RUN_STATE from L200 onward match the files on disk, and so do all 26 launch hashes.

## Q6 Assessment accuracy

- Each table row matches its report's §3 figure. The pooled figure sums correctly: 1+0+2+0+0+1+4+1+0+1+0 = 10, over 40+56+45+29+21+50+42+37+43+27+85 = 475, which is 2.1%.
- The cluster table agrees with the LEGITIMATE rows from the cross-package and cross-wave reports and with the reports' §5 sections.
- The owner-item list omits items (finding 3), and two figures on out-of-sample and targeted checks are not reported (finding 2).

## Q7 Containment

- `git diff --name-only f250882c1..4172aa98e` outside RUN and AR is empty, so no deliverable, code, lifecycle or DAG file changed.
- Inside RUN, the only changes outside WAVES/W3 are CONVENTIONS.md (the F3 clarification under Direction 8), W1/W2 RESOLUTIONS.csv (Class relabel only), RUN_STATE.jsonl (+173 lines, 0 deletions) and new CROSS_WAVE files.
- FREEZE: HEAD is 00115c71931bcae79909602d653740d3bb72dfa1, and `status --porcelain --ignored` is empty.
- In the working tree, the only change is one appended line in RUN_STATE.jsonl (this review's LAUNCH).

Scratch files: `.../scratchpad/review_w3/`. The reviewer made no git writes and edited no ledgers or resolutions.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
