# I80: the T6 slice's PR evidence package (records only)

TASK (Type 2), dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path, and you do not delegate. **You are a fresh instance.** Your ID names a records folder and a role, and carries no memory of earlier sessions. You wrote none of the slice.

## Why

The T6 successor-output slice (T3-T6S) is the next product PR, by the merge order S-I1, U8, T6S (RR "U8's full suite passes; I77's package accepted; NUM sequencing for the product PRs"). U8's package was drafted by I77 in parallel with its last gates, and ROOT completed it at the PR head. This is the same job for T6S.

## The slice

- **Branch:** `codex/piping-t6-successor-outputs-20261005`, worktree `WT/t6-outputs`.
- **Base:** main `c1bfc460fc`.
- **Commits:**
  - `055ee0c0bc`, I76: T6S-1 and T6S-2;
  - `2033260c57`, I75: T6S-3 to T6S-5;
  - `fdcdb5e024`, I75's REPAIR_01 for RV101's SF-1, committed by ROOT.
- **What changed:** 19 maintained files under `P`. None of them is touched by main's later commits through `f8ed4f0551` (S-I1, U8 and the records), so the PR's diff against current main is the slice's diff.
- **RV101** passed the slice (0/1/10) and is now confirming SF-1's repair (`R/REVIEW_RV101/t6s_01/ADDENDUM_01.md`, when it lands).

## The basis

- **The plan:** I74's PLAN (`R/I74/t6_slice_plan_01/PLAN.md`), with ROOT's rulings on its decisions 1–14 (RR "I73's checkpoint 1 and I74's plan ruled; D2 5b.3; the T6 slice dispatched").
- **The rulings:**
  - "I76's checkpoint: the Rust goldens; three choices accepted";
  - "I76's return verified; …";
  - "I75's checkpoint: S-1 granted; readings R-1 to R-7; a B8 choice prepared for the owner";
  - "T6S complete; RV101 dispatched";
  - "RV101 passes the T6 slice; SF-1 repaired by I75; no DEC-025 rerun";
  - the latest RR section.
- **The returns:**
  - `R/I76/t6s_01/` (RETURN.md and CHECKPOINT_1.md);
  - `R/I75/t6s_01/` (RETURN.md and REPAIR_01.md);
  - `R/REVIEW_RV101/t6s_01/`.
- **The common brief:** `R/BRIEFS/T6S_COMMON.md`.
- **The form:** U8's package (`T/IMPLEMENTATION/U8/`) and S-I1's (`T/IMPLEMENTATION/S_I1/`), which are #1082's form scaled down. The tools are main's copies in `T/IMPLEMENTATION/F2A_D1/`.

## Deliverable: `T/IMPLEMENTATION/T6S/`

1. **CHANGE_RECORD.md** has these sections:
   - **What the PR contains:** each of the 19 files, its +/− lines, a one-line change, and its sha256 at `fdcdb5e024`.
   - **What it does,** and **what it does not do:** no product caller of the successor yet; activation and native Current are B8's; the owner-held items untouched; G10's moved half (B8).
   - **Review:** RV101's numbers, SF-1, and its repair (the 16- and 17-digit ties).
   - **Gates:**
     - the rows that need a head (source equality, GEN-8, CI and the dispatch, DEC-025) read "see the merge record";
     - **Pass B:** state, from the diff, whether the slice touches the F2a D1 milestone's call graph (`RE/src`, PP, the readers' maintained sources) or only tests, schemas, fixtures and the desktop. Give the evidence, and recommend; ROOT rules.
   - **Routed notes:** NT-1 to PR-B1; NT-9 to S-I2; NT-7, NT-10 and I75's item d to T6's later slot; I76's item c (the historical generation records); R-2 (owner-held, B8).
2. **PR_BODY.md:** short and plain, in the form of U8's and S-I1's. State that the reviews are agent reviews, not personal review by the owner.
3. **citations.json:** an index for `check_citations.py`, pinned at NUM's head at the time you write it. Run the tool:
   - `--base` main `f8ed4f0551` and `--head` `fdcdb5e024`, as a dry run on the slice's diff;
   - record the output in `T/IMPLEMENTATION/T6S/_draft_run_records/`, which ROOT removes at the cut.

   If the slice's added lines cite no document, say so; S-I1 and U8 show how the index handles that.
4. **SHA256SUMS** over the package.

## Rules

- **Records only.** No source edits, no Git writes, no cargo, vitest or native jobs, and no installs.
  - You may run `check_citations.py` and `source_equality.py --help`. Use Python from `VENV`.
- **Placeholder paths only** (`WT`, `NUM`, `P`, `T`, `R`). Nothing goes to the system temp directory; scratch goes in `WT/scratch/i80_t6s_pkg/`.
- **Every claim is checked** against Git or the records. Where the records disagree, say so rather than choose.
- **If the host's write guard refuses a write into NUM,** write to `WT/scratch/i80_t6s_pkg/records/` and say so.

## Return

End your turn with:
- the package's file list with sha256;
- your Pass B recommendation and its evidence;
- the citation result;
- anything ROOT must rule on.

**Budget:** 2–3 h.
