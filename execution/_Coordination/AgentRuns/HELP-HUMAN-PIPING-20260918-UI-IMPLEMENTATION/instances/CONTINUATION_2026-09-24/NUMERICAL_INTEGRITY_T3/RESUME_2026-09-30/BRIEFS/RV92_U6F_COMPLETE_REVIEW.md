# RV92: U6f, the fresh complete-diff review of U6 (carriers and standing)

TASK (Type 2), a **fresh** independent reviewer dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path, and you do not delegate. You have reviewed none of U6. Unit reviews passed each piece (RV88, RV91, RV90). **Your job is the assembled whole** (workflow §6: components alone do not establish the whole): the relationships, parity and interfaces that unit reviews could not see.

## The candidate

- **The head** of `codex/piping-f2a-carriers-20261004`, given in your dispatch prompt, **diffed against NUM's merge base** (NUM `a634ac8b53` merged U3 1b and 1c; the carriers branch forked from NUM `7e4f5a51dd`). Use `git merge-base` to find the exact base, and review the whole diff.
- **What it holds:**
  - **U6a:** Rust carrier dispatch, standing, the derivative and the Python entry (D-U6-1);
  - **U6c:** the schemas and the branch-order pins;
  - **U6b:** the Python carriers;
  - **U6e:** the reader round to snapshot 07h;
  - **U6d:** the TypeScript carriers;
  - each with its repair round, plus the shared case file (format v2, with `declared_differences`).
- **Read:**
  - the plan `R/I66/u6_scoping_01/PLAN.md` (§3, §4, §6 and U6f's items);
  - `BRIEFS/U6_FANOUT_COMMON.md`;
  - every U6 ruling in `T3/ROOT_RULINGS_V1.md`, from "U6 plan accepted…" through "U6d follow-up (round 03)…";
  - the unit reviews in `R/REVIEW_RV88/`, `R/REVIEW_RV90/` and `R/REVIEW_RV91/`, which you use as leads, not as warrants.

## Review, in priority order

1. **The three-language parity table.** Build it yourself. For every behaviour U6 adds (dispatch, the downgrade guards, standing, binding, the summary, AnalysisRun copy and validate, the derivative disclosures, legacy refusals, transport), give the Rust, Python and TS outcomes on the same inputs. Use the 20 shared cases plus your own. **Every difference must be one of the four ruled `declared_differences`;** anything else is a defect.
2. **Existing behaviour unchanged across the whole diff.** Run your own end-to-end sweep: every committed result, AnalysisRun and stress-neutral document, and every existing-identity envelope, through all three languages' carriers, against the merge base. Run ROOT's acceptance suites yourself:
   - result_export;
   - the 24-file Python sweep (`R/I66/u6c_schemas_01/_run_records/sweep_test_files.txt`) plus the retained contract, schema and carrier tests;
   - desktop Vitest and `tsc`;
   - runner/headless;
   - PP's U1 and U3 pin tests against this result_export.
3. **The receipt's survival end to end.** Take the milestone successor (both modes) through every carrier path (derive, validate, AnalysisRun build and validate, reopen) and back. It must come back byte-equal and revalidated, with standing `needs_recompute`, in every language. Every receipt mutation must be refused with its ruled code.
4. **The R-1 carrier-interface items (C-1 to C-3)** against U3's public carrier (facade commits `4b31bbf23a` and `886bef131a`, merged in NUM):
   - **C-1:** `envelope()` and `into_parts()` stay ordinary-only, and silently omit a successor. Is that documented as "ordinary base only"?
   - **C-2:** `Successor(Value)` is precommit-validated, and carriers revalidate it rather than trusting it.
   - **C-3:** the only successor-preserving path is `into_publication()`, with no product caller until native activation (F-1).
   
   Say whether U6's carriers are consistent with this interface.
5. **Nothing weakened anywhere in the whole diff.** Read every removed line in product code and tests. The eligibility flags stay false, and no T6 file is edited, except the reservations D-U6-8 ruled.
6. **Cost.** F7 and RV88's U6a N-2: binding revalidates the whole statement per row, 3.8 s for 98 rows in debug. Is this acceptable for the milestone, or must it be memoized before U7?
7. **The U7 preconditions** already recorded (RV91 N-2 and N-5; RV88 U6d S-1). Are they complete? List any further precondition the whole diff reveals.

## Host

- **Your copies:** from `git archive` of the head and of the merge base, in `WT/rv92/`, with targets `WT/targets/rv92/` and scratch `WT/scratch/rv92_u6f/` (`TMPDIR` there).
- **TypeScript:** link REPO_ROOT's `P/node_modules`, and copy the prebuilt WASM from `WT/f2a-readers`. Never install or build, and disclose both.
- **Python:** the checked-JSON and units CLIs, built from your archive with `--locked --offline --release`.
- **Cargo:** the default toolchain, `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, one job at a time. The memory guard must be running.
- **Never:** Git writes, installs, new tooling, or native, solver or DEC-025 jobs.

## Output

- **The report:** `NUM/R/REVIEW_RV92/u6f_01/REVIEW.md`, containing a verdict, counts, the parity table, findings (path:line, evidence, remedy) and SHA256SUMS.
- **Time box:** 4 h.
- **End your turn** with the verdict, the counts, one line per finding, the sha256, and anything ROOT must rule on.
