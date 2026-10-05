# RV100: independent review of the follow-up T3 records-only PR

TASK (Type 2), an independent reviewer dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path. You do not delegate. **You wrote none of these records.**

## The candidate

- **The PR** brings the T3 integration branch's execution records added since #1084 to main. That branch is NUM, `codex/piping-numerical-integrity-20260926`.
- **How it is cut:** from current main (#1084's squash `f506f3e2de` or later), as a single commit. `projects/chirality-piping/execution/` comes from NUM at the head named in your dispatch.
- **What it must contain:**
  - only paths under `projects/chirality-piping/execution/`;
  - an empty non-execution diff against main;
  - no deletions;
  - no commit from NUM's history: the branch's only ancestor outside the PR is main.
- **Precedent:** #1084, reviewed by RV96 (`R/REVIEW_RV96/records_01/`, with REVIEW and ADDENDUM_01–02), and its merge record `T3/IMPLEMENTATION/RECORDS_MERGE_2026-10-05/RECORD.md`. Read RV96's findings first: B-1 (GEN-8), S-1 and S-2 (whole-host process data), B-2 (the merge method), N-8 (the squash message).

## Review, in priority order

1. **Scope.**
   - Every changed path is under `projects/chirality-piping/execution/`.
   - The PR's execution tree equals NUM's at the named head, blob for blob.
   - The branch is one commit on main.
   - List every modified (not added) path, with the NUM commit that changed it.
2. **Nothing that must not be published:**
   - no credentials, tokens, keys or private-key material (search at least for `ghp_`, `gho_`, `github_pat_`, `sk-`, `AKIA`, `BEGIN .* PRIVATE KEY`, `password`, `secret`, `token=`, `Authorization:`), and no personal data beyond the owner's configured Git identity;
   - **no whole-host data:** process listings, application names outside the build and test toolchain, chat or agent session IDs;
   - no file over 50 MB, and no binary build output.
3. **Portability.** GEN-8 must pass on the PR head:
   ```
   python3 -m pytest -q -p no:cacheprovider tools/practitioner_harness/test_live_baseline.py -k gen8
   ```
   Also count machine-absolute paths in the PR's added and modified files. Living documents must have none: ROOT_RULINGS_V1.md, ROOT_CURRENT.md, the work graph, the handoff and its prompt, and briefs.
4. **The living documents tell the truth.** Check the changed handoff, steering prompt, ROOT_CURRENT and work-graph text against the records and Git:
   - heads and PR numbers;
   - the next unused IDs (I75, RV101);
   - the merge-method rule;
   - the never-merge-NUM rule;
   - the cleanup procedure's `gather` step (compare `WT/tools/t3_cleanup.py` with its committed copy in `IMPLEMENTATION/HANDOFF_2026-10-05/host_tools/`).
   
   Report contradictions or claims the records don't support.
5. **Gate evidence.** Check each item ROOT relays: it exists, it ran on the exact head it claims, and it shows the expected result. The items are:
   - hosted CI and the full-SHA dispatch;
   - DEC-025;
   - GEN-8.
   
   **You do not run DEC-025 or native jobs; ROOT does.**
6. **Integrity.**
   - These folders' SHA256SUMS verify: `IMPLEMENTATION/RECORDS_MERGE_2026-10-05/`, `IMPLEMENTATION/HANDOFF_2026-10-05/`, `R/REVIEW_RV96/records_01/`.
   - ROOT_RULINGS_V1.md is append-only: main's copy must be a byte prefix of the PR's.
   - The squash is adequate: no blob in the PR tree is one of the 13 `original_sha256` blobs in `IMPLEMENTATION/HANDOFF_2026-10-05/_run_records/REDACTIONS.json`.

## Host and method

- **Your copy:** a `git archive` of the PR head into `WT/rv100/`, with logs in `WT/scratch/rv100_records_01/`. Delete the copy afterwards.
- **What you may run:** only the single GEN-8 pytest. No cargo or native work, no Git writes, no installs.
- **DEC-025 may be running:** don't build or run other tests during it.
- Nothing goes to the system temp directory.
- **The memory guard** (`pgrep -f memguard.sh`) must be running.

## Output

- **The report:** `NUM/R/REVIEW_RV100/records_01/REVIEW.md` plus SHA256SUMS, with placeholder paths only. It contains:
  - a verdict, PASS or FAIL;
  - counts of BLOCKING, SHOULD-FIX and NOTE findings;
  - a findings table with path, evidence and remedy;
  - a section per item.
- **Time box:** 90 minutes.
- **End your turn** with the verdict, the counts with one line per finding, the report's sha256, and anything ROOT must rule on.
