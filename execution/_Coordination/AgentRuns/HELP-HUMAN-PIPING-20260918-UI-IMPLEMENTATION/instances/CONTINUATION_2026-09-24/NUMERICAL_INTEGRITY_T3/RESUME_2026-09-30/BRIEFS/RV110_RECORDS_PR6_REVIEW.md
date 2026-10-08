# RV110: independent review of the T3 records-only PR after #1107

TASK (Type 2), an independent reviewer dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path, and you do not delegate. **You wrote none of these records.**

## The method

Follow `R/BRIEFS/RV103_RECORDS_PR4_REVIEW.md`, items 1–6, with its host rules and output form, using the substitutions below. Read the precedents first: RV106's brief and review (`R/BRIEFS/RV106_RECORDS_PR5_REVIEW.md`, `R/REVIEW_RV106/records_01/`), then RV103's.

| In RV103's brief | For you |
|---|---|
| The PR | [#1108](https://github.com/sgttomas/chirality/pull/1108), branch `codex/piping-t3-records-20261007b`, head `145443e9e4`, one commit on main `2007709549` (#1107's merge) |
| NUM's execution tree at | `25c745f905` |
| Expected modified paths | ROOT_RULINGS_V1.md and the work graph (766 added, 2 modified, 0 deleted) |
| The next unused IDs at that NUM | I90 and RV110 (you are RV110; the next reviewer will be RV111) |
| Your folders | copy `WT/rv110/`, logs `WT/scratch/rv110_records_01/`, report `R/REVIEW_RV110/records_01/` |

**Waits:** one wait per job, ending when the job's process is gone. Stop your own waits before you return. You should need no cargo. GEN-8 is pytest at the checkout root of your copy.

## Item 4 (the living documents tell the truth): check these in particular

- **#1106 → `025c1cf326`** at head `b4f22e6ce7`, with its gates, against `IMPLEMENTATION/SI1B_MERGE/` and GitHub (merge commit, parents, CI runs, dispatch 37555520168).
- **#1107 → `2007709549`** at head `1199726f69`, against `IMPLEMENTATION/B6_MERGE/` and GitHub.
  - CI includes the refused short-SHA dispatch 37620732340 and the successful 37621653258.
  - B6_MERGE's corrections to the package (A-N1 to A-N3) match RV108's ADDENDUM_01.
- **#1105 → `47a3bdfcf5`** (squash), against `IMPLEMENTATION/RECORDS_MERGE_2026-10-07/`.
- **The rulings since `030020aca3`,** including:
  - the owner's decisions, quoted exactly as recorded: M up to 64 GiB for host jobs, M ≤ 12 GiB with the target machines, and SI1c's option D as a repair within grammar 1.0.0;
  - **that no owner-held decision was taken by ROOT.** In particular, check SI1c's public-meaning ruling, which went to the owner, and B6's item-2 widening ruling, which ROOT judged not to change public meaning. Say whether you agree with that judgement and why;
  - the B6, SW, ST and R3 rulings, and RV108's and RV109's findings as routed.
- **The work graph's T3 section:**
  - the positions;
  - the owner-held list, now "M above 12 GiB";
  - the owner decisions in force;
  - the next unused IDs;
  - the next safe action.

## Item 5 (integrity): sum files that must verify in the committed tree

- `IMPLEMENTATION/SI1B/`, `SI1B_MERGE/`, `B6/` and `B6_MERGE/`.
- `R/I81/`, `R/I82/` (with `ADDENDUM_01`), `R/I83/`, `R/I84/` (both sum files), `R/I85/` (with `repair_01`), `R/I86/` and `R/I87/`.
- `R/REVIEW_RV104/` to `R/REVIEW_RV109/`: every sum file, including the addenda.
- Any other added folder with a sum file.
- **The 28 gzipped evidence files:** confirm that they decompress, and screen their contents as you screen text.

## Gate evidence

ROOT's GEN-8 on the head passed (1 passed, 10 deselected). It goes into the merge record.

## End your turn with

- the verdict;
- the counts, with one line per finding;
- REVIEW.md's sha256;
- anything ROOT must rule on.
