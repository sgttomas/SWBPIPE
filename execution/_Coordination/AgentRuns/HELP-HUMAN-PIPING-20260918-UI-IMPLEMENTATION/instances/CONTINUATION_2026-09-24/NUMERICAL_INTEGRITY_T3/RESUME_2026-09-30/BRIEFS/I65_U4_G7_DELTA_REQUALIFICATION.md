# I65: U4 G7, delta re-qualification of the registered profile on the integrated basis

I65 continues as U4's owner, with its existing context. ROOT is the return path. You do not delegate.

## Why

The profile was qualified (G6, `R/I65/u4_g6_01/QUALIFICATION.md`) and registered (`0c7827b6ad`) on the memory branch. That branch carries the precommit reader from before U6.

U6 is now merged into NUM (`f172f86abe`), and it changes `result_export`'s production code:
- `retained_precision.rs`, including F5's exact `diagnostic_refs` list (D-U6-7);
- `semantic_contract.rs`;
- `derivative.rs`.

None of the 14 reviewed inputs changes. The reader code that T17 prices does change.

Under RR "D-6 … extended" and QUALIFICATION.md §11, a change to the D1 call graph requires re-qualification before the registered build may rely on the merged basis. The memory branch merges into NUM only after this re-qualification and its review.

## Basis

- **The integrated tree:** `ba1faa1c858ce3630a22767677310b1902a14b83`, the clean merge of NUM `f172f86abe` and memory `0c7827b6ad`. ROOT extracted `projects/chirality-piping` (without `execution/`) to `WT/scratch/i65_u4_g7_01/basis/`. Treat it as read-only: build from your own copy of it.
- **I61's U3 grant 2** is in progress in WT/f2a-memory. It changes `PP/lib.rs`, the dispatch-count hook (B-1) and possibly the permit's binding. It lands later, so G7 has two passes.
  - **Pass A** (now): the U6 delta on `ba1faa1c…`.
  - **Pass B** (after grant 2 is committed and NUM is merged into the memory branch): a mechanical rerun of your Pass A script on that final basis.
- **G4–G6 records,** the rulings through "Registration applied; M = 4,026,531,840 B selected under D-7…", and RV87's and RV89's reviews.

## Deliverables (Pass A)

1. **The delta inventory.**
   - List every production-code change between `0c7827b6ad` and `ba1faa1c…`, by path:line (`#[cfg(test)]` code excluded).
   - For each, say whether it is reachable from the D1 call graph: `admit` → ordinary run → W1 → W4 precommit reader `validate` → publication. Classify each as priced, unreachable (with the reason), or new.
   - Expected starting points: F5's `exact` vector and the list comparison in the reader; whether `semantic_contract.rs` and `derivative.rs` are reached from precommit at all.
2. **Identity and statics, in your copy of the integrated tree's registered build:**
   - the build identity and the 14 reviewed-input hashes equal the registered entry;
   - no new production `include_str!` or `OnceLock` static is reachable from `validate`;
   - the reader layout witnesses compile with the registered `TypeLayout` values.
   
   If any of these fails, the build is Stale on the merged basis. **Stop and return.**
3. **Price the delta** with the G4/G5 method: T17 and any other phase the inventory touches.
   - Recompute E_mov,max + R against M, per mode, at in-build strides.
   - Report the new maxima and the margin against 0.9 M, and state the counting rule for each new term.
   - **If the margin trips, stop and return.**
4. **TEXT and the identifier audit.**
   - Rerun the TEXT chain and the enforced identifier audit (id-unaudited, stale-key, stale-audit-entry) on the integrated tree.
   - Discharge §11: rerun the by-type sweep of the non-candidates, or close the residual with an explicit-row rule. Report which you chose and why.
5. **Witnesses and behaviour, registered build of the integrated tree:**
   - the nine witnesses and the challenge;
   - PP (all targets, `--no-fail-fast`) and runner/headless.
   
   Compare them with ROOT's registered baseline on `0c7827b6ad`: PP 699 passed, 1 failed (t13), 10 ignored; runner 85 passed, 2 failed (base's `load_reference`). Compare also with NUM `f172f86abe` unregistered. Explain every outcome difference.
6. **A rerunnable Pass B script.**
   - It takes a basis path and repeats items 2–5 mechanically.
   - It reports either byte-identical TEXT and profile outputs or the exact deltas.
7. **Output:**
   - a statement of whether the registered entry stays byte-identical, or a regenerated `registration.diff`;
   - QUALIFICATION_G7.md, an addendum to G6's QUALIFICATION.md, with the new maxima and the §11 discharge.

## Where and how

- **Copy and targets:** `WT/scratch/i65_u4_g7_01/work/` and `WT/targets/i65_g7/`. Records go in `NUM/R/I65/u4_g7_01/`, with RETURN.md and SHA256SUMS, and placeholder paths only.
- **Fence:** read-only for every source file. G7 changes no source. If repricing shows a source change is needed, stop and return a proposal.
- **Host:**
  - the default toolchain, `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, one cargo job at a time;
  - memguard (PID 5387) must be running;
  - never: Git writes or index operations, installs, or native, solver-at-scale or DEC-025 jobs;
  - nothing goes to the system temp directory.
- **Other TASKs are working.** I61 is in WT/f2a-memory. Don't touch its files.

## Budget and return

Pass A: 3 h. Return once, with RETURN.md, SHA256SUMS and a concise status: the inventory count, identity and statics, the new maxima, TEXT, witnesses and suites, and whether the registered entry stays byte-identical.
