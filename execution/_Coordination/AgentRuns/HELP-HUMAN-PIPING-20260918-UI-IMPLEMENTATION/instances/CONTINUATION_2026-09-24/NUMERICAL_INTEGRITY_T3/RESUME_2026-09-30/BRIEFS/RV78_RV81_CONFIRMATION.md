# RV78–RV81: confirming the reader repairs (snapshot 07a)

The four reviewers of `reader_review_01` resume with their context, as TASKs (Type 2) dispatched directly by ROOT (HELP_HUMAN, Agent 0). ROOT is the return path, and none delegates. You confirm your own findings; you did not write any of the repairs.

## The candidate

- **READER:** branch `codex/piping-f2a-readers-20261003`. Your dispatch prompt gives the exact head.
- **Repairs since your review** (head `6b607fd01f`): Python and the shared files (I62), Rust (I63) and TypeScript (I64). The authors' RETURNs are under `R/I62/review_repair_07/`, `R/I63/review_repair_07/` and `R/I64/review_repair_07/`.
- **The shared corpus is snapshot 07a:** 15 cases, 236 mutations and 19 must-pass entries. It is described by `R/I62/review_repair_07/SHARED_SNAPSHOT_07.json` together with `SHARED_SNAPSHOT_07A.json`.
- **The decisions:** D1–D18, in `T3/ROOT_RULINGS_V1.md`, from "Reader review RV78–RV81: consolidated ruling and the repair wave" to the end. That span includes checkpoint A, the settled readings and two correction brackets.

## What each reviewer does

1. **Your findings.** For each one, report fixed, not fixed, or superseded by a decision (name the D-number). Evidence comes from your own probes and mutants rerun on the new head, not from the authors' tests.
2. **The repair diffs,** from `6b607fd01f` to the head, for your reader (RV78: the shared files and harness formats). Look for:
   - regressions;
   - any check that was removed or weakened (a stop condition since D18);
   - a decision implemented differently from its text.
3. **Challenges.** Any decision may be challenged with citations; that is a finding, not a defect in your review.
4. **RV79–RV81:** rerun your mutants. Report the survivors, each with its reason: no base exists, the decision deferred it, or a genuine gap.
5. **RV78 only:**
   - **The joint parity run on 07a:** every case, mutation and must-pass entry through all three readers' validate entries, with your own edit, canonicalization and rehash code. G7 is compared per reader.
   - **Your PROBES.json on all three readers,** with each outcome compared against its decision.
   - **Contract fidelity and emittability:** at least 40 of the 58 entries new since 06d, for expected gate and code, and for whether the producer could emit the base.
   - **The deferred list** and its stated reasons.

## Host and method

The same as `reader_review_01`, with these changes:
- **Cargo** uses the default toolchain (the Xcode licence is accepted); do not set `DEVELOPER_DIR`. Use your own targets, `WT/targets/rv<NN>`.
- **TypeScript archive copies** need READER's prebuilt `public/wasm-engine` and `public/self-weight-engine`, copied and not built, and the `node_modules` link. Disclose both.
- **Never:** Git writes, installs, new tooling, or native, solver or DEC-025 jobs.
- **The memory guard** must be running.

## Output

- **The report:** `NUM/R/REVIEW_RV<NN>/reader_confirm_02/REVIEW.md`, containing:
  - a verdict, PASS or FAIL;
  - counts of BLOCKING, SHOULD-FIX and NOTE findings;
  - a table of your original findings with each one's disposition;
  - new findings, with path:line, evidence and remedy.
  
  Add your evidence files and a SHA256SUMS. Use placeholder paths only.
- **Time box:** 90 minutes from your first tool call. Report anything unfinished.
- **End your turn** with a concise status: the verdict, the counts, the report's sha256, and anything ROOT must rule on.
