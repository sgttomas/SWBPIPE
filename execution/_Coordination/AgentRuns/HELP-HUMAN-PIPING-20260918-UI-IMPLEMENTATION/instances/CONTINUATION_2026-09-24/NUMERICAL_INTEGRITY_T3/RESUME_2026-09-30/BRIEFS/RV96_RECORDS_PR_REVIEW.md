# RV96: independent review of the T3 records-only PR

TASK (Type 2), an independent reviewer dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path. You do not delegate. **You wrote none of these records.**

## The candidate

- **The PR** brings the T3 integration branch's execution records to main. That branch is NUM, `codex/piping-numerical-integrity-20260926`.
- **How it is cut:** from current main, with `projects/chirality-piping/execution/` taken from NUM at the head named in your dispatch.
- **What it must contain:**
  - only paths under `projects/chirality-piping/execution/`;
  - an empty non-execution diff against main;
  - no deletions of records main already has.
- **Precedent:** PR1068 (`T3/IMPLEMENTATION/RESUME_RECORDS_MERGE/RECORD.md`), reviewed by RV27. Under it, a records-only PR needs:
  - an independent review;
  - hosted CI with the full-SHA dispatch;
  - GEN-8;
  - an exact-head Mac DEC-025.
  
  It does not need T9 or the both-entry gates, because no product source changes.

## Review, in priority order

1. **Scope.**
   - Every changed path is under `projects/chirality-piping/execution/`.
   - The PR's execution tree equals NUM's at the named head, blob for blob.
   - Main's records are untouched except where NUM's history changed them on purpose. List any such modification with its NUM commit.
2. **Nothing that must not be published:**
   - no credentials, tokens, keys or private-key material (search at least for `ghp_`, `gho_`, `github_pat_`, `sk-`, `AKIA`, `BEGIN .* PRIVATE KEY`, `password`, `secret`, `token=`, `Authorization:`), and no personal data beyond the owner's configured Git identity;
   - no file over 50 MB, and no binary build output (executables, `.rlib`, `.wasm` build products, `target/` content).
   
   The repository is public. Host paths of the form `/Users/<user>/…` already appear in 4,688 record files on main. Count the PR's new ones and report them as a NOTE, not a blocker, unless they expose something beyond a directory layout.
3. **GEN-8 placement.** Run `python3 -m pytest -q -p no:cacheprovider tools/practitioner_harness/test_live_baseline.py -k gen8` on the PR head, as in RR "U9 cut…".
4. **Gate evidence.** Check each item ROOT relays: it exists, it ran on the exact head it claims, and it shows the expected result. The items are:
   - hosted CI and the dispatch;
   - DEC-025 against the fresh Mac baseline on main;
   - GEN-8.
   
   **You do not run DEC-025 or native jobs; ROOT does.**
5. **Integrity of the sealed records.**
   - Spot-check that the SHA256SUMS files in the newest record folders verify: at least the `IMPLEMENTATION/F2A_D1_MERGE/`, `R/REVIEW_RV95/u9_01/`, `R/I65/u9_refreeze_01/` and `R/REVIEW_RV89/u4_g7_03/` folders, plus the rulings' last ten sections' cited records.
   - ROOT_RULINGS_V1.md is append-only: main's copy must be a byte prefix of the PR's.

## Host and method

- **Your copy:** a `git archive` of the PR head into `WT/rv96/`, with logs in `WT/scratch/rv96_records_01/`. Delete the copy afterwards.
- **No cargo or native work,** and no Git writes or installs. Nothing goes to the system temp directory.
- **The memory guard** (PID 5387) must be running.

## Output

- **The report:** `NUM/R/REVIEW_RV96/records_01/REVIEW.md` plus SHA256SUMS, with placeholder paths only. It contains:
  - a verdict, PASS or FAIL;
  - counts of BLOCKING, SHOULD-FIX and NOTE findings;
  - a findings table with path, evidence and remedy;
  - a section per item.
- **Time box:** 2 hours.
- **End your turn** with the verdict, the counts with one line per finding, the report's sha256, and anything ROOT must rule on.
