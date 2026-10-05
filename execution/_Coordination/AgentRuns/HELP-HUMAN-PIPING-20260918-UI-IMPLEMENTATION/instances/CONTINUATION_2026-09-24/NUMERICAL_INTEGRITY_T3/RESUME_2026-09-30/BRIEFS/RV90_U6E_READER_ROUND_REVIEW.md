# RV90: independent review of U6e, the reader round (F5, RV79-N1, RV80-N2; snapshot 07g)

TASK (Type 2), an independent reviewer dispatched directly by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path, and you do not delegate. **You did not write this code or the corpus. Don't rely on the author's tests or tables as your oracles.**

## The candidate

- **The commit:** on branch `codex/piping-f2a-readers-round-20261004`, against base `844448112f` (U6a). Your dispatch prompt gives the head.
- **The change:** I61's U6e.
  - **The three readers:** PY `core/analysis_runs/retained_precision.py`, RS `core/reporting/result_export/src/retained_precision.rs` and TS `apps/desktop/src/features/results/retainedPrecision.ts`, with their tests.
  - **The shared corpus** `fixtures/results/retained_precision_cases.json`, now snapshot **07g**. It has 15 cases (all bases repaired), 274 mutations, 22 must-pass entries, and a new top-level `d37` table.
- **The author's account:** `R/I61/u6e_reader_round_01/RETURN.md`, `SHARED_SNAPSHOT_07G.json`, `D37_TABLE.md` and `_run_records/`, including `base_repairs_07g.json` and `OUTCOME_DELTA_07F_07G.json`.
- **The rulings** in `T3/ROOT_RULINGS_V1.md`:
  - "U6 plan accepted: D-U6-1 to D-U6-9" (D-U6-7: F5 amends checkpoint A's D6a);
  - decision 2 (A2), in "Step 4 planned";
  - checkpoint A's D6a;
  - D35 and D37;
  - RV79's and RV80's reader_confirm reports for N1 and N2.

## Review, in priority order

1. **F5 against A2.** In each reader, the list must be exactly the envelope diagnostics whose `affected_refs` name the case, once each, in envelope order, excluding `RETAINED_PRECISION_*`.
   - Is the check identical across the three readers, at the same gate, class and point (G5 ATTEMPT, class 2)?
   - Is D6a's unique-and-resolve check kept?
   - Is any legitimate producer output now refused? Run both real milestone receipts (the U6a fixtures) through all three readers yourself.
2. **The base repairs.** For each of the 15 bases, confirm by your own diff that only `diagnostic_refs` and the dependent receipt hash changed, and that the new list is exactly A2's.
3. **RV79-N1, the independence of the D37 table.**
   - Derive the error-kind ↔ stage-record table yourself from native source: the stage transitions in `PP/retained_receipt.rs`, and `prepare_owned_case`, `solve_native` and `freeze_candidate` in `PP/retained_product.rs`, plus C3's error kinds. Compare yours with `d37` in the corpus, entry by entry.
   - Confirm that each reader's D37 test takes its expectations from the corpus table, not from its own constants.
   - Check that the TS `errorStageRecordAgrees` refactor leaves `productAttempts`' behaviour unchanged.
4. **RV80-N2:** `integral_receipt`'s scope, in Rust and Python. Is the claim that nothing observable distinguishes the scope across readers sound?
5. **No unintended outcome change.** Confirm `OUTCOME_DELTA_07F_07G.json` by your own run of the 07f readers (base `844448112f`) and the 07g readers on the 07g corpus. The only changes allowed are the 6 new F5 mutations. Confirm the two flipped tests (Rust, TS) flipped only for F5.
6. **Parity and suites.**
   - Python, Rust (result_export) and TS (vitest plus `tsc`) pass 07g in full.
   - Every reader asserts the corpus outcome on every entry.
   - The completeness flags are still false.
7. **Mutants.** Re-run I61's 22, and add at least six of your own: F5's order check dropped, the `RETAINED_PRECISION_*` exclusion dropped, D37 expectations taken from the reader instead of the corpus, and similar. Report the survivors.

## Host and method

- **Your copy:** from `git archive` of the candidate, in `WT/rv90/`, with targets `WT/targets/rv90/` and scratch `WT/scratch/rv90_u6e/`. Delete the copies afterwards. Never write to the system temp directory.
- **TypeScript:** link `P/node_modules` from REPO_ROOT and copy the prebuilt `apps/desktop/public` WASM from WT/f2a-readers. Never build or install. Disclose both.
- **Python:** build the checked-JSON and units CLIs from your archive, with `--locked --offline --release`.
- **Cargo:** the default toolchain, `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, one job at a time. The memory guard must be running.
- **Never:** Git writes, installs, new tooling, or native, solver or DEC-025 jobs.

## Output

- **The report:** `NUM/R/REVIEW_RV90/u6e_reader_round_01/REVIEW.md`, containing a verdict, counts, findings (path:line, evidence, remedy) and SHA256SUMS. Use placeholder paths only.
- **Time box:** 3 h.
- **End your turn** with the verdict, the counts, one line per finding, the sha256, and anything ROOT must rule on.
