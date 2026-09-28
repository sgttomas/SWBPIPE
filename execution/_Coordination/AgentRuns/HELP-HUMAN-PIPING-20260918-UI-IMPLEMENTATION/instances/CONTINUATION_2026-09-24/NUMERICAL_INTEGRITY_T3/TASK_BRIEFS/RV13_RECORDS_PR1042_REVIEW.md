# RV13: independent review of records PR #1042

This is a review TASK. Read Root `AGENTS.md`, `agents/AGENT_TASK.md` and `TASK_BRIEFS/_COMMON.md` first. The Mac host rules in `I8R_K1_RESUME.md` ("The Mac host") apply. This review needs no cargo, npm or test suite; the only test run is GEN-8.

**Independence.** You wrote none of these records. Find defects; do not confirm. You fix nothing, and you make no Git writes. ROOT is your return path.

## Candidate

- **PR #1042,** branch `codex/piping-numerical-integrity-20260926`, head `a8895ee84`. Verify it after a fetch. It is `<wt>/numerics` HEAD; ROOT keeps further numerics commits local until your review returns.
- **Base:** main `e7d930d49`, which is merged into the head at `08e4b05fe`.
- **Review the complete diff** `git diff origin/main...a8895ee84`. The PR body lists the commits since PR #1035's head `59660791a`.
- **Write set,** in `<wt>/numerics`, left uncommitted: `T3/REVIEW/RECORDS_PR1042_REVIEW.md` and `T3/REVIEW/_run_records/records_pr1042/**`, with its own SHA256SUMS.
- Untracked files in `<wt>/numerics` (this brief, any later brief) are not in the candidate.

## Check at least

1. **Scope:** records only, every path under `projects/chirality-piping/execution/`. No product code, test, fixture, schema or tool.
2. **Hygiene:**
   - GEN-8 passes at the candidate;
   - no machine paths (home, temp, system-private, tool-install or tilde) and no model identifiers in any added or changed file. Use `/usr/bin/grep`; the shell's `grep` is a ugrep wrapper that gives false matches;
   - every SHA256SUMS in a new or changed folder verifies, and lists exactly the folder's tracked files.
3. **Rulings integrity:** `ROOT_RULINGS_V1.md` and every other modified file keep main's text verbatim, except for disclosed in-place corrections or bracketed pointers. Appended sections are dated. Report any silent rewrite.
4. **The merge records** (`M03_SKEW_PIN_MERGE`, `K3_MERGE`, `K2B_MERGE`): check every fact against GitHub and Git:
   - the PR, merge, head and base SHAs; `--match-head-commit`;
   - the CI run ids, their conclusions and heads, and the dispatch's `target_base`;
   - the chains, commit by commit;
   - the DEC-025 summaries and the stated original hashes where checkable; the suites comparisons against the Mac baselines; the vitest disclosures;
   - the review verdicts and the review files' hashes as cited.
5. **The reviews as filed** (`M03_SKEW_PIN_REVIEW.md`, `K3_REVIEW.md`, `K2B_REVIEW.md` and their `_run_records`): the verdicts stated in merge records and rulings match the review files.
6. **The rulings' history is honest:** the Q7 ruling and its reversal; the b-rule "third attempt" framing; RV11-1 and the premise correction (RV11-3); the reversal on `force_scaled_end_actions`; the d2df479f3 and `1d105d633` disclosures.
7. **K4's brief and rulings** (`I12_K4_IMPLEMENTATION.md`; ROOT_RULINGS_V1 "K4: spawn and rulings"): each ruling is consistent with `DESIGN.md` revision 5a.2 (hash-pinned; cite sections) or is an explicit, recorded departure. The stale-design list is accurate against the code on main. Check especially Q4 (no p + 64 residual at the ceiling) and Q5 (the reading of §4.1.7).
8. **Currency:** no stale present-tense status in the work graph or in the handoff and operating notes. Anything the head states as current must be current at the head.

## Verdict

**PASS** (no unresolved BLOCKING findings) or **FAIL**, with a findings table (ID, severity BLOCKING / SHOULD-FIX / NOTE, site, evidence, resolution). End your turn with a summary for ROOT: the verdict, the finding counts, each BLOCKING or SHOULD-FIX finding in one line, and the review file's sha256.
