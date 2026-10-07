# RV103: independent review of the T3 records-only PR after #1102

TASK (Type 2), an independent reviewer dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path, and you do not delegate. **You wrote none of these records.**

## The candidate

- **The PR:** [#1103](https://github.com/sgttomas/chirality/pull/1103), branch `codex/piping-t3-records-20261006b`, head given at dispatch. It is one commit on main `f8ed4f0551` (#1102's merge).
- **What it carries:** `projects/chirality-piping/execution/` from NUM (`codex/piping-numerical-integrity-20260926`) at `ea0e288e8a`.
- **What it must contain:**
  - only paths under `projects/chirality-piping/execution/`;
  - an empty non-execution diff against main;
  - no deletions;
  - no commit from NUM's history.
- **Precedents:** #1088 (RV100, `R/REVIEW_RV100/records_01/`) and #1101 (RV102, `R/REVIEW_RV102/records_01/`), with the merge record `T3/IMPLEMENTATION/RECORDS_MERGE_2026-10-06/`. Read RV102's findings first.
- **The rules** (in RR):
  - the proportionate-CI ruling: a records-only PR's gates are GEN-8, the automatic CI and an independent review;
  - E-4's GEN-8 method;
  - the ruling "T3's gate set and Git rules, consolidated after the handoff was made ephemeral";
  - A1-S-1 (a records PR leaves out an open product PR's package); no product PR is open at this cut;
  - return verification checks sums against the committed tree.

## Review, in priority order

1. **Scope.**
   - Every changed path is under `projects/chirality-piping/execution/`.
   - The PR's execution tree equals NUM's at `ea0e288e8a`, blob for blob.
   - The branch is one commit on main.
   - List the modified paths: ROOT_RULINGS_V1.md and the work graph are expected.
2. **Nothing that must not be published.**
   - No credentials, tokens or key material. Search at least `ghp_`, `gho_`, `github_pat_`, `sk-`, `AKIA`, `BEGIN .* PRIVATE KEY`, `password`, `secret`, `token=` and `Authorization:`.
   - No personal data beyond the owner's configured Git identity.
   - **No whole-host data:** process listings, application names outside the build and test toolchain, chat or agent session IDs. RV101's evidence (55 files) carries host-lock log excerpts, so read them with this in mind.
   - No file over 50 MB, and no binary build output.
3. **Portability.**
   - GEN-8 passes on the PR head. Use `python3 -m pytest -q -p no:cacheprovider tools/practitioner_harness/test_live_baseline.py -k gen8`, in a checkout of the exact head.
   - Count machine-absolute paths in added and modified files. Living documents must have none: ROOT_RULINGS_V1.md, the work graph and the briefs.
4. **The living documents tell the truth.** Check the work graph's T3 section and the new rulings against the records and Git. Report contradictions or claims the records don't support. In particular:
   - #1102 → `f8ed4f0551`, at head `f7a7572e35`, with its gates;
   - the next unused IDs: I80 and RV103 at `ea0e288e8a`;
   - **the ruling "#1102 merged; DEC-025's suites rerun in a fresh target; …":** check its account of the shared-target artefact against `IMPLEMENTATION/U8_MERGE/dec025/`. Does the evidence support the cause it names, the rerun's sufficiency and the "no earlier result is reopened" reasoning? The first comparison, the excerpt, the fresh comparison and the host-tool copy are all there.
   - `IMPLEMENTATION/U8_MERGE/RECORD.md` against its `_run_records/` and `dec025/`, and against GitHub (the PR's merge commit, its parents, and its CI runs on `f7a7572e35`).
5. **Integrity.**
   - These record folders' SHA256SUMS verify, in the committed tree:
     - `IMPLEMENTATION/U8_MERGE/`;
     - `R/REVIEW_RV97/*` (all sum files), `R/REVIEW_RV101/*` and `R/REVIEW_RV102/*`;
     - any other added folder with a sum file.
   - ROOT_RULINGS_V1.md is append-only: main's copy is a byte prefix of the PR's.
   - No blob in the PR tree is one of the 13 `original_sha256` blobs in `IMPLEMENTATION/HANDOFF_2026-10-05/_run_records/REDACTIONS.json`.
6. **Gate evidence.** Check that GEN-8 ran on the exact head (ROOT's `gen8.txt`, relayed at dispatch), and that the PR's automatic CI runs are on that head.

## Host and method

- **Your copy:** a `git archive` of the PR head into `WT/rv103/`, with logs in `WT/scratch/rv103_records_01/`. Delete the copy afterwards.
- **What you may run:** only the single GEN-8 pytest. No cargo, no native work, no Git writes, no installs.
- **Other jobs may hold the T3 lock:** don't run other tests. Nothing goes to the system temp directory.

## Output

- **The report:** `NUM/R/REVIEW_RV103/records_01/REVIEW.md` plus SHA256SUMS, with placeholder paths only. It contains:
  - a verdict, PASS or FAIL;
  - counts of BLOCKING, SHOULD-FIX and NOTE findings;
  - a findings table with path, evidence and remedy;
  - a section per item.
- **If the host's write guard refuses a write into NUM,** write to `WT/scratch/rv103_records_01/records/` and say so.
- **Time box:** 90 minutes.
- **End your turn** with the verdict, the counts with one line per finding, the report's sha256, and anything ROOT must rule on.
