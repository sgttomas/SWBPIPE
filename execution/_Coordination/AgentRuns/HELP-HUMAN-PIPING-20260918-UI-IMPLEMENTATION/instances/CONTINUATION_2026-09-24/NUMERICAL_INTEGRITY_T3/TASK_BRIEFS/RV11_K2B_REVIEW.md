# RV11: independent full-diff review of slice K2b

This is a review TASK. Read Root `AGENTS.md`, `agents/AGENT_TASK.md` and `_COMMON.md` first. The Mac host rules in `I8R_K1_RESUME.md` ("The Mac host") apply to you in full.

**Independence.** You did not design, implement or test K2b (I10 is excluded). Find defects; do not confirm. You fix nothing, and you make no Git writes. ROOT is your return path.

## Candidate

- **PR #1040,** branch `codex/piping-k2b-20260928`, head `087b3a088`. Verify it after a fetch.
- **Base:** main `eb52114e9` for the slice. The head also merges main `98b1723b1`, which brings the skew M03 pin, tests only.
- **Review the complete slice diff** `git diff eb52114e9 ca20b9eca`, commit by commit:

| Commit | Content |
|---|---|
| `6ce4d694b` | Checkpoint A |
| `70828d4d6` | Checkpoint B: ROOT's rulings A–C |
| `8e6698282` | C's two assertion groups |
| `b461dfc0f` | The b-rule limitation pin |
| `ca20b9eca` | The records |

- **Confirm** that the merge `087b3a088` adds nothing but main's changes, and that it conflicts with nothing.
- **Build only from `git archive` copies,** in your scratch `<wt>/scratch/rv11`, with target `<wt>/rv11-target`.
- **Write set,** in `<wt>/numerics`, left uncommitted: `T3/REVIEW/K2B_REVIEW.md` and `T3/REVIEW/_run_records/k2b_review/**`, with its own SHA256SUMS.

## Basis

1. `DESIGN.md` revision 5a.2 §4.7 (all of it), the K2b and F1 rows of §6, and §7.3.
2. `TASK_BRIEFS/I10_K2B_IMPLEMENTATION.md`.
3. `ROOT_RULINGS_V1.md`: the four K2b sections dated 2026-09-28:
   - kernel only and the LEF restatement;
   - the rulings on checkpoint 0 (1–7);
   - the rulings on the checkpoint-A stop (A–C);
   - the b-rule window (the "third attempt").
4. K2a's and K1's records, as needed.
5. The candidate's `T3/IMPLEMENTATION/K2B/` (CHANGE_RECORD, RETURN, `_run_records/`).

## Check at least

1. **The kernel-only claim.**
   - Re-derive by lexer scan that no product path calls a new entry.
   - Check that every existing entry is unchanged in behaviour at b = 0: the bodies, the fail-closed guards, and the added `Debug` field of the evidence types.
   - Spot-check I10's b = 0 Debug probe (439/439) by re-running a sample from archives.
2. **The even-b derivation** (RETURN §4). Check it step by step, independently.
   - **Its premise** is "every rounded operation is zero or normal at both scales; the dense Cholesky, the triangular solves and the skyline LDLᵀ are unchecked". Is it stated where a reader will see it, and is it adequate?
   - **Try to break the invariance:** a normal-range model where forced even b changes u's bits, or changes the unscaled report.
3. **The b-rule.**
   - The census scope: predicted exponents, including intermediates; the ±3 bound on predicted against true exponents; the subnormal refusal.
   - The window, the floor and the parity.
   - I10's counterexample: the rule refuses a case that another b in the window solves. Is the pin test correct, and labelled as a documented limitation?
   - **Look for a case that is worse than refused:** any b the rule picks that publishes a trusted wrong value. That is BLOCKING.
4. **The unscaling outcomes and the step-5 departure** (rulings 3 and B).
   - Actions and reactions: normal is exact; subnormal carries its precision; underflow and overflow are refused and never flushed.
   - Residual records: an explicit outcome, never a silent zero, and never a refusal.
   - Check the field-by-field table in RETURN against the code.
5. **The interactions under b:**
   - K-D5's formation check (demotion parity);
   - S11-K's ledger (L1 `force_scaled`; a product scales one factor);
   - K1's pattern path;
   - K2a's names, which must survive where no b fits;
   - the nonlinear loop, which reaches no scaled entry, behaviourally and by pin.
6. **The mutation table.**
   - Re-kill a sample from clean archives: at least the odd midpoint, the stepwise unscaling, record-refuses, record-silent, the third attempt, one pin mutant, and one original pin mutant.
   - Write at least two mutants of your own, aimed at gaps you suspect.
7. **The pins and site tables.** The extensions must be additive, and the original mutants' kill sites unchanged (the comparison in `_run_records/`).
8. **Records and hygiene:**
   - SHA256SUMS verify, and GEN-8 passes on the head;
   - no machine paths (use `/usr/bin/grep`) and no model identifiers;
   - T9 is stated as Mac-only, and the suites claim matches the logs;
   - the disclosures are honest (the overwritten checkpoint-A logs, the probe's coverage, the raw logs' trailing blank lines);
   - the "F1b interface" signatures are exact at the head.

## Verdict

**PASS** (no unresolved BLOCKING findings) or **FAIL**, with a findings table (ID, severity BLOCKING / SHOULD-FIX / NOTE, site, evidence, resolution). End your turn with a summary for ROOT: the verdict, the finding counts, each BLOCKING or SHOULD-FIX finding in one line, and the review file's sha256.
