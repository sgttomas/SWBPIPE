# RV119: independent review of the T3 records-only PR after #1111

TASK (Type 2), an independent reviewer dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path, and you do not delegate. **You wrote none of these records.**

## The method

Follow `R/BRIEFS/RV103_RECORDS_PR4_REVIEW.md`, items 1–6, with its host rules and output form, using the substitutions below. Read the precedents first:
- RV117's brief and review (`R/BRIEFS/RV117_RECORDS_PR7_REVIEW.md`, `R/REVIEW_RV117/records_01/`);
- RV110's (`R/BRIEFS/RV110_RECORDS_PR6_REVIEW.md`, `R/REVIEW_RV110/records_01/`);
- RV103's.

| In RV103's brief | For you |
|---|---|
| The PR | [#1114](https://github.com/sgttomas/chirality/pull/1114), branch `codex/piping-t3-records-20261008`, head `57f078b4c8`, one commit on main `f4358eb0be` (#1113's merge) |
| NUM's execution tree at | `96cf68289f` |
| Expected changes | Under `execution/`: 1,055 added, 2 modified (ROOT_RULINGS_V1.md and the work graph), 0 deleted. Outside it: only `projects/chirality-piping/validation/portability_policy.json`, with 2 appended entries |
| The next unused IDs at that NUM | I100 and RV119 (you are RV119; the next reviewer will be RV120) |
| Your folders | copy `WT/rv119/`, logs `WT/scratch/rv119_records_01/`, report `R/REVIEW_RV119/records_01/` |

**Host:**
- one wait per job, ending when the job's process is gone; stop your own waits before you return;
- you should need no cargo;
- GEN-8 is pytest at the checkout root of your copy, run through `WT/tools/t3_slot.sh`;
- Python is VENV's only, with `PYTHONDONTWRITEBYTECODE=1`.

## Item 3 (screening): this PR's particular checks

- **The leak validator:** run main's `tools/validation/validate_run_record_leaks.py` on the PR (`--base f4358eb0be --head 57f078b4c8`). Confirm that the PR adds no symlink (no mode `120000`).
- **The host screen:**
  - the strict pattern of B1_COMMON;
  - the machine's host names, which means its network name and any `MacBook` form, case-insensitive, besides the earlier form;
  - `.local`, judged by what it names;
  - the junit `hostname` attribute;
  - in text and in every decompressed `.gz` (156 files).

  Judge each hit: pattern or rule text and already-redacted placeholders are acceptable; a machine datum is not.
- **E-16's redaction** (`IMPLEMENTATION/REDACTION_E16/RECORD.md`, and RR "Erratum E-16: …" with its addendum):
  - none of the 42 listed old sha256s is present in the PR;
  - each listed file has its new sha256;
  - each decompresses with no `hostname` attribute;
  - `R/REVIEW_RV113/rvr_sr_py_01/SHA256SUMS` verifies, and REVIEW.md is `d8611e59…`.

  Also confirm that no file from E-10's list (RR "Erratum E-10: …") is present in its original form.
- **E-17's portability entries:** the policy file's only change is the two appended entries. Each path exists, each sha256 equals that file's bytes in the PR, and the roles are right: CONTROL for the brief, EVIDENCE for RV117's review. Each matches RR "RV115 and RV118 confirm B2-C revision 02: …". Run GEN-8 yourself.

## Item 4 (the living documents tell the truth): check these in particular

- **#1111 → `54f1ba1f6d`** (squash), against `IMPLEMENTATION/RECORDS_MERGE_2026-10-07C/` and GitHub.
- **#1112 → `0b6c5d7362`** (merge), against `IMPLEMENTATION/SI1C_MERGE/`.
- **The rulings since NUM `0b8299e496`, including:**
  - **that ROOT took no owner-held decision.** In particular:
    - B2/B3's decisions 22–24 stay prepared and undecided;
    - M above 12 GiB is the owner's;
    - SA3-1's option (ii), C-1 to C-16, the header move's nine transport changes and the alignment set are within ROOT's delegation, and none changes public meaning before B8. Say if any does.
  - **the owner's decisions, quoted exactly as recorded:**
    - SI1c's option D and "Repair within 1.0.0";
    - the memory decision and its clarification (2026-10-08);
    - E-16's choice of redaction in place.
  - **the I3 step's and the readers' rounds' verifications,** and the census rulings (the nine transport changes);
  - **B2-C's path to final for J1:** RV118's review, revisions 01 and 02, and RV115's addenda 02–04.
- **The work graph's T3 section:** the positions, the owner-held list, the owner decisions in force, the next unused IDs and the next safe action. Note any line already stale at NUM `96cf68289f`.
- **The PR description against the content.** RV117's N-3 found an overstatement in #1111's. ROOT corrected this description once before dispatching you.

## Item 5 (integrity): sum files that must verify in the committed tree

- `IMPLEMENTATION/RECORDS_MERGE_2026-10-07C/`, `SI1C_MERGE/` and `REDACTION_E16/`.
- **Every sum file** in the added or changed folders:
  - `R/I85/b1_sp_01/` (`i3_01`);
  - `R/I90/`, `R/I91/` (repair_02 and repair_02_item4) and `R/I92/` (repair_01 and repair_01_item3);
  - `R/I97/b2_c_01/`, all three: v0's 13, revision_01's 10 and revision_02's 9;
  - `R/I98/`, `R/I99/`;
  - `R/REVIEW_RV109/rvp_round2_01/`, `R/REVIEW_RV111/`, `R/REVIEW_RV113/` (all three folders), `R/REVIEW_RV115/` (addenda 02–04), `R/REVIEW_RV117/` and `R/REVIEW_RV118/` (review and addenda 01–02).
- **The sealed files are unchanged** wherever a later record supersedes them in a new file: CONTRACT and REVISION_01 beside REVISION_02; REPAIR_02 beside REPAIR_02_ITEM4.

## Gate evidence

ROOT's runs at the head:
- **GEN-8:** passed (1 passed, 10 deselected).
- **`validate_run_record_leaks.py`:** PASS on 1,056 files: 0 credentials, 0 machine-local symlinks, and one size warning for an 8.1 MB evidence file.

Both go into the merge record. The automatic CI runs on the PR.

## End your turn with

- the verdict;
- the counts, with one line per finding;
- REVIEW.md's sha256;
- anything ROOT must rule on.
