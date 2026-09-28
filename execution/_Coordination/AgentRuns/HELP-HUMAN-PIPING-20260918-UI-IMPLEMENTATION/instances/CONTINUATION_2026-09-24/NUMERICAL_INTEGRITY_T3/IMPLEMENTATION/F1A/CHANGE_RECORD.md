# F1a change record: the D-5 evidence line and SUP-17

This is the draft PR record for slice F1a of T3 (numerical integrity), following `.agents/skills/chirality-change/SKILL.md`. It was implemented by I7 (TASK).

- **Branch:** `codex/piping-f1a-20260927`, from main `5ae22926e` (after K-D5, PR #1017).
- **Split:** ROOT split slice F1. F1a is facade-only and has no kernel dependency. F1b (the sparse wiring and W2 at formation) waits for K1 and K2b and is not part of this change.
- **Basis:**
  - D1 `DESIGN.md` revision 5a.2 (`fb62ef4a…`): §4.3.1 D5C-3, §4.9 (SUP-17), §5 items 5a and 10, and the §6 F1 row;
  - `ROOT_RULINGS_V1.md`: the K-D5 and S11-G sections (the integrity diagnostic, its layout and the no-op rule);
  - the I7 brief (`TASK_BRIEFS/I7_F1_IMPLEMENTATION.md`, §F1a) with ROOT's split ruling;
  - ROOT's rulings relayed by the manager: the SUP-17 "…" is the existing `support_contribution_summary(&model)` argument; the evidence-line format (numerics `a61e891ce`), with conditions (a) reason tokens are K-D5's identifiers, pinned against the enum, (b) fixed field order, `{:?}` for f64, `integrity_dof_label` rows, and (c) placement after S11-G's step under its no-op rule, with byte identity to main when no record exists, proven by a test.

## What changes

**1. The D-5 evidence line.** K-D5 attaches a `FormationCheck` record to `StructuralSolution` only when its check demotes a case from Passed to Sensitive, and until now nothing rendered it. F1a renders it as one evidence line at the end of that case's integrity diagnostic (`diagnostic:numerical-integrity:<case>`), appended with one space:

- `formation_check: reason=estimate; row=<node>:<DOF>; doubled_correction=<2|w_i|>; scale=<max(|q_i|,S*)>; trigger_ratio=<2|w_i|/(1e-9*scale)>`
- `formation_check: reason=formation_check_unavailable; detail=<detail>`

The row label comes from `integrity_dof_label`, with its `global_dof=<i>` fallback. f64 values use `{:?}`, the shortest round-trip form; the zero-scale clause prints `trigger_ratio=inf`. The reason tokens are the snake_case of K-D5's `FormationCheckReason` variants, the names its doc comments and D1 §4.3.1 use. K-D5 has no string form of its own.

**Where it can appear.** Only on a K-D5-demoted case, which is always Sensitive:
- The kernel sets `quality: Sensitive` whenever it attaches a record (`FK/structural.rs:1480-1490`, the only constructor of the field).
- The integrity code therefore starts as NUMERICAL_INTEGRITY_SENSITIVE (`PP:1089`).
- S11-G's load-row step (`PP:1098-1100`) and R-b′ after the recovery loop (`PP:3853`) both call `formation_guard::demote`, which is a no-op unless the code is CHECKS_PASSED (`formation_guard.rs:481-484`).

So a K-D5-demoted case gets no S11-G sentence and exactly one D-5 line, as before F1a it got no sentence and no line. A case with both an S11-G sentence and a D-5 line cannot arise. The nonlinear route never carries a record (K-D5 does not select it), and its call passes `None`.

**2. SUP-17** (D1 §4.9, a message text change only). The `SOLVER_SYSTEM_BLOCKED` under-restraint message now reads:

> fewer than six independent ground constraints including positive springs: the six rigid-body modes of a connected structure cannot all be removed; directly restrained global DOF classes: {restrained}; global DOF classes with no direct restraint: {missing} (not a rigid-body mode analysis; separated restraints can resist rotations); support contributions: {…}

It replaces "…; restrained global DOF classes: …; missing global rigid-body DOF classes: …", which called directly restrained DOF classes "missing rigid-body" classes. The computed values, the code, the severity and the id are unchanged.

## Standing and values

- **No quality, standing or value changes.** No solve, report, code, severity, id, affected reference or result row changes. The D-5 line changes only the message text of cases K-D5 already demotes. SUP-17 changes only the message text of models that are already blocked.
- **No `StructuralReport` change** (D5C-3), and no new published field, diagnostic code or enum value. One private plumbing field, `PreviewLinearSolve.formation_check: Option<FormationCheck>`, carries the record from `solve_preview_reduced_system` to `append_integrity_report`, which gains one `Option<&FormationCheck>` parameter. `PreviewLinearSolve` derives only `Debug, Clone`, is built once and is never formatted or serialized. `FormationCheck` has no serde derive.
- **The committed-fixture diff (T9):** 112 of 112 outputs are byte-identical, base `5ae22926e` against the candidate: fixtures 72, validation 30, core 10, including the 6 outputs that are `ERR` on both trees. No committed request is K-D5-demoted or under-restrained, so neither change reaches a committed output. The stop rule did not trigger.
- **SUP-17 and committed bytes:** no committed fixture, raw, schema or hash pin carries the old text. It survives only in historical records under `P/execution/**`: source snapshots, design and review records, and old evaluation and native-verify outputs. Those are history, nothing reads them, and nothing is regenerated (ROOT, relayed).

## Files

| File | Change |
|---|---|
| `P/core/product_physics/src/lib.rs` | `append_integrity_report` gains `formation_check: Option<&FormationCheck>` and appends the line after the S11-G step; new `formation_check_evidence_line`; the private `PreviewLinearSolve.formation_check`, filled from `checked.formation_check`; the linear call passes it and the nonlinear call passes `None`; the SUP-17 text; `tests::under_restrained_model_reports_solver_diagnostic` now pins the whole message; `#[cfg(test)] mod f1a_tests;` |
| `P/core/product_physics/src/f1a_tests.rs` (new) | 7 tests: the product-level line on the 122 case (both entries and both modes); no line on non-demoted and S11-G-only cases; the case demoted by both K-D5 and S11-G; exact text; reason tokens against the enum; the composition matrix, including R-b′; byte identity to main when there is no record, against a verbatim copy of main's function |
| `P/core/product_physics/tests/formation_check_runtime.rs` | comment only (the manager extended the write set for it): the `!message.contains("FormationCheck")` assertion stays, and now guards against a Debug rendering of the record |

## Checks

All run on the candidate with `RUSTUP_TOOLCHAIN=1.97.1`, `CARGO_INCREMENTAL=0` and `--offline --locked` (RETURN §4–§7):
- **Suites**, all passing with 0 failures:
  - product_physics 523, with 1 ignored as before (the 516 existing plus the 7 in `f1a_tests`);
  - headless 84, operation_applier 194, self_weight_wasm 14, physics_audit_regression 15, result_export 91, src-tauri 114;
  - numerical_integrity builds (0 tests).
- **T9:** 112 of 112 byte-identical, as above.
- **Mutations:** the no-patch control passes. All ten mutants, M1–M8, M9a and M9b, are killed at behavioural assertions, with no compile failure.
- **The 122 case as rendered,** on both entries:
  - sparse: `formation_check: reason=estimate; row=N1:RX; doubled_correction=8.093801056572112e-14; scale=3.333666549241316e-5; trigger_ratio=2.427897612739378`;
  - dense: `trigger_ratio=4.852606290728372`.

## Remaining

- F1b (sparse wiring, W2 at formation, the `range_scaling` line) after K1 and K2b.
- The PR's hosted CI (with the surface-4 dispatch), the DEC-025 sweep, the independent complete-diff review and GEN-8 belong to the manager and ROOT.
- The both-entry gate was not run. F1a changes no solve, quality, standing or value, as the T9 diff and the byte-identity oracle show; ROOT agreed the gate is not required.
