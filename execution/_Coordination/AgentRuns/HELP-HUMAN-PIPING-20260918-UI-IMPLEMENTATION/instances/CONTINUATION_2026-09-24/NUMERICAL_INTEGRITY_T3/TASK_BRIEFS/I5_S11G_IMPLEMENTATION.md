# I5: implement slice S11-G (the formation-noise guard)

This is an implementation TASK. Read `_COMMON.md` first; this brief overrides it where they differ (writes and builds).

## Purpose

Implement slice S11-G of the selected design, so that the product never publishes Passed on a value whose formed terms carry more formation noise than the criterion allows. That covers:
- **the load-row guard,** built from exact signed defects of formed ledger terms;
- **the recovery guard R-b′,** on formed K_e·u end actions.

A fired case is demoted to **Sensitive**: never refused, never `Err`, and no new envelope field. After S11-G, **`GATE/FORMATION_EXCEPTIONS.json` is empty**. Never publish a silently wrong value, strip a load or feature, skip a test or raise a timeout.

## Governing basis (read in this order)

1. `T3/ROOT_RULINGS_V1.md`:
   - "I4's F12 stop" and the amendment on formation rows;
   - "S11-G note: rulings", and "S11-G after V1's S11G_CHECK" with its clarifications;
   - "S11-G note revision 2: rulings", and "S11-G revision 2 after V1's delta check";
   - **"Selection: the S11-G design"**.
2. **`T3/DESIGN_NUMERICS/S11G_GUARD.md` revision 2.1** (sha256 `7c052c9e…`), the selected design:
   - §3, the load-row guard: formed and input terms, per-family records, carriage in the ledger, the statistic, the S\* floor on the self-equilibrated defect only (two exact accumulators), the exact decision, and what firing does, including the no-op rule;
   - §4, R-b and R-b′ (**R-b′ is selected**);
   - §7, the write set; §8, the tests and mutations; §9, the K-D5 boundary.
3. `T3/REVIEW/S11G_CHECK.md` (all sections, and the delta-2.1 verdict): V1's counterexamples and NOTEs.
4. S11-K and S11-F as merged on main: the ledger, `AssembledForce` and `ExactAccumulator`; S11-F's formation-row pins (`FORMATION_PINS`) and its F1/F11/F12 gate test.
5. `T3/GATE/FORMATION_EXCEPTIONS.json` and `S11_EXCEPTIONS.json`, and their generator `GATE/pin_s11_exceptions.py`.

## Base, worktree and branch

- ROOT creates `<wt>/s11g` on a new branch `codex/piping-s11g-<date>` from **main after S11-F merges.**
- The PP line numbers in the note's §7 refer to S11-F's candidate tree. Re-locate every site on your base, and record the actual lines.
- Make no Git writes. The manager commits.
- Build the two authority targets first, and never delete them.

## Write set (note §7)

- `FK/load_ledger.rs`:
  - `Formation` (with `CannotBound`) and `push_formed(…, self_equilibrated)`;
  - parallel vectors;
  - `formation_rows`, with **two exact accumulators per row** (net and self-equilibrated);
  - `FormationRow`;
  - a `Debug` that excludes the new vectors;
  - unit tests.
- `FK/lib.rs`: re-export only.
- `straight_pipe/src/lib.rs`: `equivalent_global_nodal_loads_with_spans_formed` (today's values plus `Formation::Exact`), and `bending_formation_bound` (R-b′'s B).
- **New `product_physics/src/formation_guard.rs`:**
  - the exact load-row decision;
  - R-b′'s recovery decision;
  - body S\*;
  - `FormationFinding`;
  - reason sentences.
- **PP:**
  - the formed push sites;
  - the guard call after `finish_case_ledger`;
  - the recovery record in `solve_load_case`'s straight element-recovery branch;
  - `source_eligible`;
  - `append_integrity_report` (with the no-op rule) and its two call sites.
- New `product_physics/src/s11g_tests.rs`, and `tests/s11f_site_test.rs` (site-table extensions).
- **`GATE/FORMATION_EXCEPTIONS.json` emptied by its generator** (`GATE/pin_s11_exceptions.py`), in the T3 records. Coordinate with the manager: the manager commits the GATE change on the T3 branch. It must be emptied only when your gate run shows all 14 rows non-Passed.
- **Not touched:** SA, FK `structural.rs`, `solve_preview_reduced_system`, CB, `pressure_runtime`, and any library or code-rule data.

## Implementation requirements beyond the note

- **D21-1 (V1).** Decide |A_net| > 12·(T0 − B) **exactly**: add ±12B and ∓12T0 exactly into a copy of the net accumulator, and test the sign. No rounded comparison.
- **D21-2 (V1).** The scaled-RoundedProduct underflow fallback is **γ2·|value| + |k|·2^-1074** (not + 2^-1074).
- **The threshold constant** is RD(10^-9), or the exact rational 10^-9 (DN-3).
- **The FMA error term:** the stated underflow condition (|a·b| ≥ 2^-969, else Bounded), and 12× overflow into `SumError` (DN-2).
- **The no-op rule:** a case already Sensitive is left untouched, with no appended text and no byte change.
- **Firing:** demote to Sensitive at `append_integrity_report` on both the linear and nonlinear routes; a reason sentence in the existing message; `source_eligible = false`. No new field or code, and never `Err`. If anything would need a new envelope field, **stop and report.**

## Tests and mutations (note §8; every listed item is required)

- **Behavioural tests:** T1–T6, **T6a**, **T6b**, T7–T9, **T10**, **T10b**, **T11–T17**. Every verdict pin has its **paths-differ precondition** asserted inside the test.
- **Mutations:** M1–M18, each killed by its named test. Record each patch, the command and the killing test.
- **R-b′ is the only INPLANE catch** after S11-F (K-D5 is silent there), so **T11, T17 and M14 are load-bearing.** Run M14 in both variants (signed sums, and |Tu|), and add any further mutant you can devise against `bending_formation_bound` and R-b′'s clauses. A survivor is a defect to report.
- **Gate (T16, extended from S11-F's F1/F11/F12):** on both entries and both modes, **all 14 formation rows are published non-Passed** (6 UDL by the load-row guard, 8 INPLANE by R-b′), the S11 list stays empty, and there is no breach outside the lists. Then the manager runs the GATE generator to empty FORMATION_EXCEPTIONS.json.

## Committed bytes and fixtures

- **The committed-byte forecast is zero.** Run every committed request through base and candidate, in both modes. **Any committed byte change stops the work** and is reported with its site, reason and size. There is no pre-approval for S11-G regeneration.
- The formation-row pins from S11-F stay bit-identical; only verdicts change.

## K-D5 boundary (note §9)

- No function is edited by both slices; K-D5 (I3) may land before or after S11-G.
- If your edits touch `solve_preview_reduced_system`, FK `structural.rs` (beyond nothing), SA or `finish_checked_factor`, **stop and tell the manager.**
- The no-op rule keeps S11-G silent on cases K-D5 has already demoted.

## Build and cargo

- `RUSTUP_TOOLCHAIN=1.97.1`, `RUSTUP_AUTO_INSTALL=0`, `CARGO_INCREMENTAL=0`, `--offline --locked`, and your own `CARGO_TARGET_DIR` (`<wt>/s11g-target`).
- One heavy cargo job at a time. Alternate with the other builders, and hold during any sweep: check `pgrep -x cargo` and `pgrep -f 'python[0-9.]* .*run_evidence_sweep'`.
- Keep free disk above about 8 GB.
- **Suites (`--no-fail-fast`):** every touched crate and its path dependents (product_physics, headless, result_export, operation_applier, src-tauri, validation benchmarks), the Python suites, and desktop vitest and build if any reader is touched.
- `cargo fmt`. **No Git index operations.**

## Disclosure and return

- **`T3/IMPLEMENTATION/S11G/CHANGE_RECORD.md`** (chirality-change format) covers:
  - what can change standing and why;
  - that no value changes;
  - the zero fixture diff;
  - **the CannotBound availability loss for loaded curved spans;**
  - **the DN-4 residual, routed to W1/F2;**
  - the formation list emptied;
  - no in-band marker.
- **`T3/IMPLEMENTATION/S11G/RETURN.md`**, with `_run_records/` and `SHA256SUMS`. It covers:
  - the files and lines changed;
  - the caller list (lexer scan);
  - T1–T17 results;
  - the mutation table (M1–M18 plus extras);
  - the gate table for the 14 rows;
  - the fixture diff;
  - suite counts;
  - the toolchain;
  - what was not done.
- **No machine paths** in committed files (use `<wt>` and `<scratch>`). The manager runs GEN-8 before committing.

Send the manager a SendMessage summary. Message **at once** if the stop rule triggers, if a design item cannot be implemented as specified, or if the K-D5 boundary cannot be kept. Don't improvise a different design.

## Addendum 1 (ROOT, 2026-09-27): "never refused", qualified

The rulings are recorded in `T3/ROOT_RULINGS_V1.md` under "S11-G implementation: I5 rulings".

**One exception to "never refused".** The multi-case captured receipt refusal is a **disclosed, fail-closed residual** under ruling 3. In a captured invocation in which one case is source-selected, and another is demoted from Passed by a guard, receipt finalization refuses the invocation with `SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED`. That refusal is accepted for S11-G. The fix, a "demoted-ordinary" receipt form, belongs to T3's composite `SOURCE_BLOCKS_FINALIZATION_FAILED` item and must close before T3 closes. Make no receipt or reader edits.

**Everything else stays never-refuse:** a fired case is demoted to Sensitive, never refused, never `Err`, with no new envelope field.

**Required by ruling 3:**
- the reach per slice, recorded as facts: through S11-F's `LOAD_CONTRIBUTION_ABSORBED` demotion on main, through K-D5's formation-check demotion, and through S11-G's guards;
- the characterization test, labelled a known residual, asserting:
  - refusal with the finalization code;
  - no published case value;
  - single-case captured invocations demoted and not refused;
- disclosure in CHANGE_RECORD and in the PR-body text.

**Also required:** rulings 1 (the reader-window condition, with its tests) and 2 (the `B > 0 && B ≥ T0` erratum, with its boundary tests and mutation).
