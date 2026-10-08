# RV117: independent review of the T3 records-only PR after #1108

TASK (Type 2), an independent reviewer dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path, and you do not delegate. **You wrote none of these records.**

## The method

Follow `R/BRIEFS/RV103_RECORDS_PR4_REVIEW.md`, items 1–6, with its host rules and output form, using the substitutions below. Read the precedents first: RV110's brief and review (`R/BRIEFS/RV110_RECORDS_PR6_REVIEW.md`, `R/REVIEW_RV110/records_01/` with its ADDENDUM_01), then RV103's.

| In RV103's brief | For you |
|---|---|
| The PR | [#1111](https://github.com/sgttomas/chirality/pull/1111), branch `codex/piping-t3-records-20261007c`, head `18a20d329f`, one commit on main `e33f3e2f1b` (#1110's merge) |
| NUM's execution tree at | `0b8299e496` |
| Expected modified paths | ROOT_RULINGS_V1.md and the work graph (1,043 added, 2 modified, 0 deleted) |
| The next unused IDs at that NUM | I97 and RV117 (you are RV117; the next reviewer will be RV118) |
| Your folders | copy `WT/rv117/`, logs `WT/scratch/rv117_records_01/`, report `R/REVIEW_RV117/records_01/` |

**Waits:** one wait per job, ending when the job's process is gone. Stop your own waits before you return. You should need no cargo. GEN-8 is pytest at the checkout root of your copy.

## Item 3 (screening): one new check

Main's `tools/validation/validate_run_record_leaks.py` now blocks any changed run-record symlink whose target is absolute or leaves the repository (RR "NUM absorbs main with #1109's RV58 fixture repair; …", erratum E-11).
- Run it on the PR (`--base e33f3e2f1b --head 18a20d329f`) in your copy.
- Confirm independently that the PR adds no symlink (no mode `120000`), and that it leaves `R/REVIEW_RV58/` exactly as main has it. #1109 replaced RV58's 104 machine-local links on main, and a records PR from a NUM that had not absorbed it would have put them back.

## Item 4 (the living documents tell the truth): check these in particular

- **#1108 → `4f37590bfb`** (squash), against `IMPLEMENTATION/RECORDS_MERGE_2026-10-07B/` and GitHub. No product PR merged between #1108 and this cut.
- **#1109,** another session's records-only fix to T3's RV58 fixture, as RR's E-11 section records it.
- **The rulings since `6f983f12f3`,** including:
  - **that no owner-held decision was taken by ROOT.** Check in particular:
    - B2/B3's decisions 22–24, which must stay prepared and undecided;
    - B3-S's per-route pricing and decision 26 (M within 12 GiB is ROOT's; above it is the owner's);
    - SP's headline rule;
    - B3-D's rulings (B3D-1 to B3D-18), and whether any of them changes public meaning before B8;
    - R-COMB-1, of which the owner was informed but which the owner did not decide;
  - **the owner's decisions,** quoted exactly as recorded (no new owner decision was made in this span);
  - B1's phase-2 rulings (R3′, SA, the parked-slot patch, I2, SR-RS/SR-PY/SR-TS), SI1c's rulings 2 and 3 with RV111's confirmation, and B2-KD's R-1 to R-11 and B3-K's K3-1/K3-2, as routed.
- **The work graph's T3 section:**
  - the positions;
  - the owner-held list;
  - the owner decisions in force;
  - the next unused IDs;
  - the next safe action.

  Note any line already stale at NUM `0b8299e496`.

## Item 5 (integrity): sum files that must verify in the committed tree

- `IMPLEMENTATION/RECORDS_MERGE_2026-10-07B/`.
- `R/I85/b1_sp_01/`, `R/I88/si1c_01/` (RETURN's and REPAIR_01's), `R/I89/` (all three), `R/I90/` to `R/I96/`: every sum file. **For I96, both `SHA256SUMS` (v0, 11 entries) and `SHA256SUMS.revision_01` (20)**, with `statics/r1/` beside the unchanged v0 `statics/`.
- `R/REVIEW_RV109/rvp_r3p_read_01/` and `R/REVIEW_RV110/` to `R/REVIEW_RV116/`: every sum file, including the addenda.
- Any other added folder with a sum file.
- **The 8 gzipped evidence files:** confirm that they decompress, and screen their contents as you screen text.

## Gate evidence

ROOT's GEN-8 on the head passed (1 passed, 10 deselected), and so did `validate_run_record_leaks.py` (1,044 files; 0 credentials; 0 machine-local symlinks). Both go into the merge record.

## End your turn with

- the verdict;
- the counts, with one line per finding;
- REVIEW.md's sha256;
- anything ROOT must rule on.
