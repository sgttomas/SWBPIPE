# RV12: independent full-diff review of slice K3

This is a review TASK. Read Root `AGENTS.md`, `agents/AGENT_TASK.md` and `_COMMON.md` first. The Mac host rules in `I8R_K1_RESUME.md` ("The Mac host") apply to you in full.

**Independence.** You did not design, implement or test K3 (I11 is excluded). Find defects; do not confirm. You fix nothing, and you make no Git writes. ROOT is your return path.

## Candidate

- **PR #1041,** branch `codex/piping-k3-20260928`, head `b7e93650e`. Verify it after a fetch.
- **The slice diff:** `eb52114e9` to `b7e93650e`, excluding the merge `de719cbdc` of main `98b1723b1`, which brings the skew M03 pin, tests only. Review it commit by commit:

| Commit | Content |
|---|---|
| `74add6078` | Checkpoint A |
| `8cacbfaf4` | The test profile, **since withdrawn** |
| `9aee9854c` | The profile guard test |
| `664ef5c5e` | The records |
| `e83e22356` | The profile withdrawn |
| `b7e93650e` | The records addendum |

- **Confirm** that the merge adds only main's changes, and that `FK/Cargo.toml` at the head equals main's.
- **Build only from `git archive` copies,** in `<wt>/scratch/rv12`, with target `<wt>/rv12-target`, at the default opt-level 0. Add no profile.
- **Write set,** in `<wt>/numerics`, left uncommitted: `T3/REVIEW/K3_REVIEW.md` and `T3/REVIEW/_run_records/k3_review/**`, with its own SHA256SUMS.

## Basis

1. `DESIGN.md` revision 5a.2:
   - §4.1.1 (the `wide.rs` bullet, `RetainedCombination`);
   - §4.1.2, §4.1.4, §4.1.6, §4.1.7 and §4.1.8;
   - §4.11;
   - §5 item 7;
   - the K3a, K3 and K4 rows of §6;
   - §7.3 and §7.4.
2. `TASK_BRIEFS/I11_K3_IMPLEMENTATION.md`, with its "ROOT rulings for this slice".
3. `ROOT_RULINGS_V1.md`:
   - "K3: spawn and rulings";
   - "K3: rulings on I11's checkpoint-0 plan";
   - "K3: Q7…" (SUPERSEDED);
   - "K3: Q7 reversed".
4. K3a's records (`IMPLEMENTATION/K3A/`, `REVIEW/K3A_REVIEW.md`), and `IMPLEMENTATION/KD5/RETURN.md` item 8.
5. The candidate's `T3/IMPLEMENTATION/K3/`: CHANGE_RECORD, RETURN with addendum 1, and `_run_records/`.

## Check at least

1. **`Wide<2>` untouched, and K-D5's published-byte surface safe.**
   - Diff `wide.rs` against `eb52114e9`: only the approved lines may change.
   - K3a's tests and generator `--check` pass.
   - The `Display` pin covers every existing string.
   - T9 is 112/112. Spot-check it from archives if you can.
2. **The arithmetic is correct. Test it against an oracle independent of I11's generator,** as RV2 did for K3a. Use Python `Fraction` or `decimal` at high precision, written by you.
   - Sample every operation at L = 4, 8 and 16.
   - Include limb boundaries, ties, far sticky bits, massive cancellation, near-exact ÷ and √, and exponent extremes.
3. **The conversion to binary64.**
   - Single rounding, with no double rounding at the subnormal boundary.
   - The overflow midpoint, and 2^-1075 exactly.
   - The zero conventions (ruled).
   - `relative_precision` rounded upward.
   - Test these against your own oracle.
4. **K4's arithmetic (Q3).** TwoSum and TwoProduct are error-free; widening is exact; narrowing is correctly rounded; the integer constructor matches the ledger's quantum. Check the claims, and whether the "K4 interface" (RETURN §14) is complete for §4.1.2, §4.1.4 and §4.1.6.
5. **No heap allocation per operation, and integers only.** Read `multi.rs` for any `Vec`, `Box` or float in the arithmetic path.
6. **The work counter and its limb-cost table (Q9).** Deterministic, per width, mergeable per attempt.
7. **The mutation table.**
   - Re-kill a sample from clean archives, at least M1, M4, M8, M11, M14, M17 and P1.
   - Write at least two mutants of your own, aimed at gaps you suspect.
   - A mutant killed only by a panic or at compile time is a finding.
8. **The Q7 history.** The records must state the tried and withdrawn profile honestly, `Cargo.toml` must equal main's, and the opt-level 0 evidence (FK 227; the mutation table at O0) must be real. Spot-check it.
9. **Records and hygiene:**
   - SHA256SUMS verify, and GEN-8 passes on the head;
   - no machine paths (use `/usr/bin/grep`) and no model identifiers;
   - T9 is stated as Mac-only;
   - the dead-code labels are truthful;
   - the test data size (about 5 MB) is justified, and the digests cover every record.

## Verdict

**PASS** (no unresolved BLOCKING findings) or **FAIL**, with a findings table (ID, severity BLOCKING / SHOULD-FIX / NOTE, site, evidence, resolution). End your turn with a summary for ROOT: the verdict, the finding counts, each BLOCKING or SHOULD-FIX finding in one line, and the review file's sha256.
