# I7 return: slice F1a (the D-5 evidence line and SUP-17)

**Status: complete and green.** The clean point is the `<wt>/f1a` working tree on `codex/piping-f1a-20260927`, base main `5ae22926e`, with no Git writes. The manager commits.

- **Suites:** every touched and dependent crate passes, with 0 failures.
- **T9:** 112 of 112 outputs byte-identical against main.
- **Mutations:** all ten mutants are killed at behavioural assertions, and the no-patch control passes.
- **Callers:** enumerated by lexer scan.
- **The SUP-17 stop rule** did not trigger: no committed fixture or evidence bytes change.

Records use `<wt>`, `<scratch>` and `<home>` placeholders; there are no machine paths.

## 1. Brief, basis and rulings

- **Brief:** `TASK_BRIEFS/I7_F1_IMPLEMENTATION.md` (§F1a), read with `_COMMON.md` and `I6_K2A_IMPLEMENTATION.md`.
- **ROOT's split:** F1a only. F1b waits for K1 and K2b; assembly, the reduction maps, `source_recovery` and the nonlinear loop are untouched.
- **Design:** D1 `DESIGN.md` revision 5a.2 (sha256 `fb62ef4a…`, verified): §4.3.1 D5C-3, §4.9 SUP-17, §5 items 5a and 10, and the §6 F1 row.
- **Rulings relayed by the manager:**
  - **SUP-17:** the design's "…" is the existing `support_contribution_summary(&model)` argument. The historical records under `execution/**` stay untouched, and nothing is regenerated.
  - **The evidence-line format** (ROOT, numerics `a61e891ce`) comes with three conditions:
    - (a) the reason tokens are K-D5's identifiers, pinned against the enum;
    - (b) the fields come in a fixed order, f64 values use `{:?}`, and rows use `integrity_dof_label`;
    - (c) the line goes after S11-G's step under its no-op rule, and a test must prove byte identity with main when no record exists.
  - **The private plumbing field** is allowed; "no new field" means no new published field.
  - **The write set** is extended to one comment in `tests/formation_check_runtime.rs`.
  - **No gate** is needed for F1a.
  - **Mutant M9** was added.
- **Read:** Root `AGENTS.md`; the briefs above; the design sections above; K-D5's and S11-G's CHANGE_RECORD and RETURN; `formation_guard.rs`; `append_integrity_report` and its callers; K-D5's `formation_check.rs` and `finish_checked_factor`; S11-K's fixture harness.

## 2. Files and line counts

| File | Lines | sha256 (candidate) |
|---|---|---|
| `P/core/product_physics/src/lib.rs` | +62 −8 | `4768930b…` |
| `P/core/product_physics/src/f1a_tests.rs` (new) | 663 | `fa31c436…` |
| `P/core/product_physics/tests/formation_check_runtime.rs` | +3 (comment only) | `faaf940d…` |

There are no other product, test, fixture, schema, Cargo or lockfile changes.

## 3. Write-set items

**(i) The D-5 evidence line** (`PP`, where the integrity diagnostic is composed).
- `append_integrity_report` (`PP:1078-1108`) gains `formation_check: Option<&FormationCheck>`. After S11-G's load-row step (`PP:1098-1100`) it appends `" " + formation_check_evidence_line(model, check)` when a record is present (`PP:1101-1107`).
- `formation_check_evidence_line` (`PP:1110-1127`) renders in ROOT's format:
  - `formation_check: reason=estimate; row=<label>; doubled_correction=<{:?}>; scale=<{:?}>; trigger_ratio=<{:?}>`
  - `formation_check: reason=formation_check_unavailable; detail=<detail>`
  - An `Estimate` record always carries a row (`formation_check.rs:329-335`). The `row=none` branch for a missing row is unreachable and is kept only so the match needs no panic.
- **The private plumbing:** `PreviewLinearSolve.formation_check: Option<FormationCheck>` (`PP:4374`) is filled from `checked.formation_check` in the one constructor (`PP:4473`).
  - The linear call passes `linear.formation_check.as_ref()` (`PP:3006`); the nonlinear call passes `None` (`PP:3048`).
  - `PreviewLinearSolve` derives only `Debug, Clone`. No format string or serializer takes it: `append_linear_solver_mode_evidence` reads named fields only.
  - `FormationCheck` derives `Debug, Clone, PartialEq` and has no serde derive.
  - T9's byte identity is the output-level proof that nothing picks the field up.
- **No change** to `StructuralReport`, and no new published field, diagnostic code or enum value.

**(ii) SUP-17** (`PP:1810`). The message now reads, verbatim from §4.9 with the ruled "…":

> fewer than six independent ground constraints including positive springs: the six rigid-body modes of a connected structure cannot all be removed; directly restrained global DOF classes: {restrained}; global DOF classes with no direct restraint: {missing} (not a rigid-body mode analysis; separated restraints can resist rotations); support contributions: {support_contribution_summary}

- `tests::under_restrained_model_reports_solver_diagnostic` now pins the whole message with `assert_eq!` and asserts that the old phrase is absent.
- The code comment at `PP:1791` ("fewer than six independent ground DOFs cannot remove six rigid-body modes") is left as it is. It is a correct statement of the check, not published text.

**The stale comment** (the write-set extension). In `tests/formation_check_runtime.rs`, three comment lines now explain that the `!message.contains("FormationCheck")` assertion guards against a Debug rendering of the record. The assertion itself is unchanged.

### 3.1 Composition with S11-G, traced (the manager's check)

Lines are at this candidate.
- **A record implies a Sensitive report.** `FK/structural.rs:1466-1490` runs the check only when `!ordinary_sensitive`. It builds the solution with `quality: Sensitive` if `ordinary_sensitive || formation_check.is_some()`. Line 1483 is the only non-test constructor of `StructuralSolution.formation_check`, checked by grep across FK, SA and sparse_direct.
- **PP keeps the pair together.** It takes report and record from that one solution (`PP:4469-4473`), and nothing rewrites `structural_report.quality`.
- **So the record starts as NUMERICAL_INTEGRITY_SENSITIVE:** `append_integrity_report` sets the code from `report.quality` (`PP:1089`).
- **No S11-G sentence can be added.** The load-row step calls `formation_guard::demote`, which returns false at once unless the code is CHECKS_PASSED (`formation_guard.rs:481-484`). The D-5 line then follows (`PP:1101-1107`).
- **The reverse composition cannot arise.** A record would need an S11-G sentence and a D-5 line together. The only later S11-G step is R-b′, `amend_integrity_report` at `PP:3853` (`formation_guard.rs:493-502`). It uses the same `demote`, so it is a no-op on any record that carries a line. The line is appended once, together with the record. A `formation_check_unavailable` record demotes in the same way (line 1486).
- **The pins:**
  - This trace is quoted in the doc comment of `f1a_case_demoted_by_both_kd5_and_s11g_has_the_d5_line_alone`.
  - The unit test runs R-b′ on a record carrying a line and asserts `false` and unchanged bytes.
  - The sentence-then-line order is pinned at unit level for the unreachable combination (a Passed report with a record), and the test says it is unreachable.

## 4. Tests (`src/f1a_tests.rs`, 7; all pass)

Product tests first assert that the paths differ: the case is K-D5-demoted, or the guard fires, computed in the test. P1's requests are read byte for byte from `tests/formation_check_runtime.rs`.

| Test | What it pins |
|---|---|
| `f1a_kd5_demoted_case_carries_exactly_one_d5_line` | 122, captured and typed, sparse and dense: SENSITIVE/warning, `quality: Sensitive`, no S11-G text, exactly one line as the message suffix after one space, fields in ROOT's order, `reason=estimate`, a free-row label (N1:RX), trigger > 1, and the printed trigger bit-equal to `doubled/(1e-9·scale)` |
| `f1a_non_demoted_cases_carry_no_d5_line` | 345, CHAIN-T n03 and CHAIN-A n03, both entries and modes: CHECKS_PASSED and no line. RF-CANCEL-UDL-W1e8 (report Passed, S11-G-demoted): exactly one S11-G sentence and no line |
| `f1a_case_demoted_by_both_kd5_and_s11g_has_the_d5_line_alone` | 122 plus an invented uniform load W = 1e8 N/m (global y, on M1), cancelled at every free row by nodal loads equal to minus the product's own rounded per-DOF resultant of that load. Preconditions: `formation_guard::load_row_finding` fires on the case's own rows, and the report is `quality: Sensitive`. Pin, both entries and modes: SENSITIVE, no S11-G sentence, one line with `reason=estimate` |
| `f1a_evidence_line_exact_text` | the exact strings for estimate, the zero-scale `trigger_ratio=inf`, the `row=global_dof=12` fallback, and unavailable |
| `f1a_reason_tokens_are_the_enum_variants` | condition (a): each token equals the snake_case of the variant name read from the enum's Debug form; an exhaustive match breaks the build if a variant is added |
| `f1a_composition_with_s11g_layout_and_no_op_rule` | no record gives today's diagnostic; a record appends exactly `" <line>"`; with both K-D5 and S11-G the line stands alone; R-b′ afterwards is a no-op; the S11-G finding still demotes a Passed record (control); the order is sentence then line |
| `f1a_no_record_is_byte_identical_to_main` | condition (c): against a verbatim copy of main's `append_integrity_report` (`5ae22926e`, `PP:1063-1090`; the body was diffed against `git show` and is identical), serialized output is byte-identical for Passed and Sensitive, and for no finding, a LoadRow finding or a Recovery finding, after an existing diagnostic; with a record, it is main's bytes plus exactly `" <line>"` |

SUP-17 is pinned by `tests::under_restrained_model_reports_solver_diagnostic` in `lib.rs`. At product level, condition (c) is also covered by the committed-raw byte tests already in the suites (S11-G's T13; headless `load_reference_route_tests`) and by T9.

**The 122 lines as rendered** (`_run_records/suites/f1a_122_lines.log`; the same on both entries):
- sparse: `formation_check: reason=estimate; row=N1:RX; doubled_correction=8.093801056572112e-14; scale=3.333666549241316e-5; trigger_ratio=2.427897612739378`;
- dense: `formation_check: reason=estimate; row=N1:RX; doubled_correction=1.6176971209150903e-13; scale=3.3336665371059296e-5; trigger_ratio=4.852606290728372`.

## 5. Suites (`_run_records/suites/`)

Every run used `RUSTUP_TOOLCHAIN=1.97.1`, `RUSTUP_AUTO_INSTALL=0`, `CARGO_INCREMENTAL=0`, `--offline --locked --no-fail-fast` and `CARGO_TARGET_DIR=<wt>/f1a-target`. The target was pruned between phases.

| Crate | Passed | Failed | Ignored |
|---|---|---|---|
| product_physics | 523 | 0 | 1 (the existing `source_receipt` measurement) |
| headless | 84 | 0 | 0 |
| operation_applier | 194 | 0 | 0 |
| self_weight_wasm | 14 | 0 | 0 |
| physics_audit_regression | 15 | 0 | 0 |
| numerical_integrity | 0 (builds) | 0 | 0 |
| result_export | 91 | 0 | 0 |
| src-tauri | 114 | 0 | 0 |

product_physics was 516 on main (K-D5 repair) and is 523 here: the 7 new tests. The compiler warnings are the existing ones; none is in F1a's lines.

## 6. Committed-fixture diff (T9; `_run_records/fixture_diff/`)

- **Method:** S11-K's harness (`fixdiff_main.rs.txt`, sha256 `ec089c1d…`, unchanged) was built twice, `--release --offline`, with the lock copied from `core/product_physics`. The base and candidate locks are identical after resolution.
  - One build is against a `git archive` of main `5ae22926e` (core, fixtures, validation, schemas).
  - The other is against the candidate.
  - The committed inputs are identical in the two trees; only `core/product_physics` differs.
- **Inputs:** every committed JSON request or model under `P/fixtures`, `P/validation` and `P/core`, in both modes.
- **Result: 112 of 112 outputs byte-identical** (fixtures 72, validation 30, core 10), including the 6 outputs that are `ERR` on both trees.
- **Cross-check:** the base hashes equal K-D5's recorded combined-candidate hashes (`KD5/_run_records/combined/fixture_diff/output_sha256_candidate.txt`).
- **No output on either tree** contains `formation_check`, "ground constraints" or "rigid-body DOF classes". No committed request is K-D5-demoted or under-restrained, so the stop rule did not trigger.

**SUP-17 and committed bytes** (`_run_records/sup17_grep.txt`):
- Outside `execution/`, the old text occurs only in `PP` (the message, a comment and the test).
- Inside `execution/**` it occurs in 52 historical files: source snapshots, design and review records, the `_Evaluation/PHYSICS_AUDIT_2026-09-05/I1` results and replay, and native-verify raws.
- Nothing reads them; `physics_audit_regression` uses `post_repair/P9`. They stay as history, and nothing is regenerated.

## 7. Mutations (`_run_records/mutations/`)

- **Method:** each mutant gets a fresh `tar -m` copy of the candidate's `core`, `fixtures`, `schemas` and `validation/benchmarks/numerical_integrity`, one mutation (`mutate.py.txt`) and a clean target, which is removed afterwards.
- **Targeted tests:** `cargo test --lib --test formation_check_runtime -- f1a_ under_restrained_model_reports_solver_diagnostic kd5_required_true_positive`.
- **The no-patch control passes:** 8 lib tests and 1 integration test.
  - The first control and M1–M5 ran without the `numerical_integrity` folder, because of a tar option-order slip; that control passed too, and its log is kept.
  - The control was re-run with the folder present before M6–M9b.
  - The targeted tests do not read that folder.

| Mutant | Change | Result: killed by (behavioural assertions; no compile error) |
|---|---|---|
| M1 | the line is never appended | kd5_demoted, both_demoted, composition, no_record_byte_identical |
| M2 | the linear call site passes `None` | kd5_demoted, both_demoted (product level; unit tests pass) |
| M3 | the line is appended before the S11-G step | composition, no_record_byte_identical |
| M4 | the unavailable reason is rendered with the `estimate` token | exact_text, reason_tokens |
| M5 | `trigger_ratio` prints the scale | exact_text, kd5_demoted (bit identity), both_demoted |
| M6 | a newline separator instead of one space | kd5_demoted, both_demoted, composition, no_record_byte_identical |
| M7 | SUP-17 reverted to main's text | `under_restrained_model_reports_solver_diagnostic` |
| M8 | the record is Debug-rendered instead of the line | kd5_demoted, both_demoted, composition, no_record_byte_identical |
| M9a | an editor-wide rename of `FormationCheckReason::Estimate` to `EstimateExceeded` (9 sites), token left as written | reason_tokens (`"estimate_exceeded"` against `"estimate"`) |
| M9b | a hard-coded token `reason=exceeded` | kd5_demoted, both_demoted, exact_text, reason_tokens |

Cargo stops after the first failing test target, so for the killed mutants `formation_check_runtime` was not reached. M8 would also fail K-D5's `!contains("FormationCheck")` assertion, but it is killed by the lib tests first.

## 8. Callers (`_run_records/callers.txt`, `scan_callers.py.txt`)

The scan is a lexer scan adapted from K-D5's: comments and strings are removed and test code is marked. It covers every `.rs` file outside `execution/`, `target/` and `node_modules/`. Patterns: `append_integrity_report`, `formation_check_evidence_line`, `PreviewLinearSolve`, `solve_preview_reduced_system`, `.formation_check`, `formation_guard::demote`, `amend_integrity_report`, the two support summaries, and `FormationCheck`/`FormationCheckReason`. There are 83 sites, 33 of them non-test, each classified.
- **The record reaches text only through the linear `append_integrity_report` call** (`PP:2999`). The nonlinear call (`PP:3041`) passes `None`.
- **The renderer has one caller.**
- **`PreviewLinearSolve`** is private, built once in `solve_preview_reduced_system` (whose only caller is `PP:2745`) and read by named field.
- **The SUP-17 message has one producer.** Its only text consumer in the repository is PP's own test (git grep over all files outside `execution/`: no TS, desktop, Python or other crate).

## 9. The dense trigger value for 122: 4.852606 against K-D5's 4.827 (the manager's reconciliation item)

The records settle it without a build, and no solve differs from main's.

- **Where 4.827 comes from.** It is the adapter-level test on K-D5's generated model `F122`, not the product route.
  - K-D5's own RETURN §3a heads the column "K-D5 trigger (adapter test, identical system)".
  - K-D5's §3 table and logs record it from `structural_adapter::kd5_tests::kd5_required_true_positive_skew_cantilever_122_demotes_in_both_modes`, both in `_run_records/suites/core_solver_nonlinear_integration.rerun_nocapture.log` and on the post-S11-G repair tree in `_run_records/repair/tests/ni_kd5.log`: dense `trigger=4.826728293925875`, actual 2.4133641591897357; sparse `trigger=2.4278923279576334`, actual 1.2139461517530596.
  - That model's section values (`kd5_models.rs` `F122`: A = 0.005969026041820614, I = 2.700984283923829e-05, J = 5.401968567847658e-05) are the generator's (`kd5_models.py`), fed straight to `AssemblyEvidence`. They are not PP's own derivation from OD 0.2 m and wall 0.01 m.
- **Where 4.8526 comes from.** It is the product route through PP.
  - K-D5's RETURN §3a records the product's own frozen-reference breach for 122 as **2.4263× dense** and 1.2139× sparse. That is its column "Frozen-reference comparison (the product's own values)".
  - F1a's rendered dense record gives EF = 4.852606290728372 / 2 = **2.4263**, equal to that product figure, as §4.3.1 expects (EF ≈ actual).
  - The rendered sparse trigger, 2.427897612739378 (EF 1.21395), likewise matches the product's 1.2139.
- **Conclusion.**
  - K-D5 quoted the adapter model's trigger beside the product's breach. The adapter system and the PP-built system are not bit-identical, despite the column heading: their dense actual errors differ, 2.4134 against 2.4263, in K-D5's own table.
  - The dense solve at cond 3.8e7 amplifies the difference. The sparse figures agree to about 2e-6 relative.
  - It is not a tree difference. 4.827 appears unchanged on K-D5's post-S11-G repair tree, and T9 shows the candidate's outputs byte-identical to main's.
- **Not established without a build:** which binary64 input differs between the adapter's F122 and PP's system (the section properties from PP's own derivation, or the member and spring assembly path). It does not bear on F1a. K-D5's committed records are not edited.

## 10. The gate: not run

F1a changes no solve, quality, standing, value, code, severity, id or row. It changes only the message text of cases K-D5 already demotes (the D-5 line) and of models already blocked (SUP-17). T9 (112 of 112 identical) and the byte-identity oracle back this. ROOT agreed the gate is not required. The PR's DEC-025 sweep covers the rest.

## 11. Not done

- Not done by design: F1b (sparse wiring, W2 at formation, the `range_scaling` line, `source_recovery`, the nonlinear loop).
- Left to the manager and ROOT: commits, the PR, hosted CI (surface-4 dispatch), the DEC-025 sweep, GEN-8, and the independent complete-diff review.
- Not run: the both-entry gate (§10) and the Python and desktop TS suites. No fixture reader or TS consumer of either text exists.
- Not edited: any fixture, reference, schema, hash pin, protected criterion, historical record, or K-D5 or S11-G committed record.
- rustfmt is not installed for toolchain 1.97.1, so the new test file was not machine-formatted. CI has no fmt or clippy step.
