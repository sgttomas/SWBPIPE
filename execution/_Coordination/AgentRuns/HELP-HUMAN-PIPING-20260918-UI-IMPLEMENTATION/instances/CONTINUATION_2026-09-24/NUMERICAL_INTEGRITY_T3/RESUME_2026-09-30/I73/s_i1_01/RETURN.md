# I73 — S-I1 final return: option C's interval evaluator, runner outcomes and Python reference

TASK (Type 2) I73, dispatched by ROOT (HELP_HUMAN). Brief `R/BRIEFS/I73_S_I1.md` (sha256 `9e40b13a…`).

**Status: complete.** All four deliverables are built. All four controls pass. All 21 mutants are killed. Every suite's existing outcomes equal the base head's.

- **Basis:** D2 revision **5b.3** (`T/DESIGN_STANDING/DESIGN.md`, sha256 `993f5f3ab4bd768b…`), §4.11.1–§4.11.6, as amended by ROOT's ruling in RR "I73's checkpoint 1 and I74's plan ruled; D2 5b.3; the T6 slice dispatched" (`T/ROOT_RULINGS_V1.md`, RR:12394). RR read at sha256 `8128a5a7c76f36f9…`; it has since grown by later, unrelated sections and is now `5d89b4269975ab64…`, with that ruling unchanged. This return also builds on `CHECKPOINT_1.md` in this folder (`b992efcf…`, unchanged).
- **Placeholders:** `WT` is the T3 host root; `NUM = WT/numerics`; `P = projects/chirality-piping`; `T`, `R` and D2 as in the dispatch. No machine paths are recorded.
- **Base head:** `c1bfc460fc`. Worktree `WT/s-i1`, branch `codex/piping-s-i1-20261005`. **Everything is left uncommitted.**

## 1. Changed files (all inside the fence)

| File (`P/…`) | Change | Lines | sha256 |
|---|---|---|---|
| `core/rules/expression_evaluator/src/lib.rs` | modified; **+2,197 / −0** (two insertion hunks: interval mode after `validate_finite`, and `mod interval_tests`) | 4,528 | `8925cc0d68e8f41d7d84d0d317bf629e2ca11ea9ad039d7a9bc0a0384bd350b6` |
| `core/rules/rule_check_runner/src/lib.rs` | modified; **+550 / −3** (the 3 removed lines are one import list rewrapped and one match arm extended) | 2,311 | `1803a02d28203482999efae2de2349603faa63336eaa97872c88821bc883c36c` |
| `core/rules/rule_check_runner/tests/interval_bounds_run.rs` | new (9 runner tests) | 452 | `6c8aa7984da6d756be4641e430bd62b5ee343fcfd06b217467b390cd7214203d` |
| `core/rules/rule_check_runner/tests/rule_interval_cases.rs` | new (Rust side of the shared cases) | 284 | `549e991010edfe377257b58d058ed1965efa9f6a407576265f7aff68f6f7d093` |
| `core/analysis_runs/rule_interval.py` | new (Python reference evaluator) | 739 | `c5445be1614fd23cd7a98771b53245c0131b628232eb7733165598c9084f00e1` |
| `fixtures/rule_interval/rule_interval_cases.json` | new folder and file (83 shared cases) | 3,851 | `85af98460797db955264898fbbc74ab953c3002ad5851d4e89ae677a63941edd` |
| `tests/test_rule_interval.py` | new (Python parity, oracle, property) | 378 | `2b3dd690248e1042347b915e32bb6b832c3059c442aa71864ef293aa1f78708e` |

Base sha256 values: evaluator `461eb4f4…`, runner `5d2c92eb…`.

**What did not change:**
- No schema, `Cargo.toml` or `Cargo.lock` changed. The src-tauri lock is `4de71f1b…`, the same as base.
- No dependency was added.
- `fixtures/rule_expressions/conformance_corpus/` is untouched; its walker still passes.
- No other file in the worktree changed. `git status --ignored` shows only these seven paths, with no `target/` or bytecode left in the tree.

**Size.** The evaluator's production part is 1,070 lines with doc comments. The runner's production change is the whole +550, because its tests are in the two new files. The Python reference is 739 lines against D2's estimate of about 400, because it carries its own formula decoder and the point path's structural checks so that blocked cases are at parity too.

## 2. What was built

### 2.1 The interval mode (deliverable 1; unchanged since checkpoint 1 except one added test and `rustfmt`)

`evaluate_interval(&EvaluationInput, &[IntervalBinding]) -> IntervalEvaluationResult` covers the whole grammar. The rest of the design is in CHECKPOINT_1 §2–§3:
- outward `next_down`/`next_up` after every floating operation;
- the point path's own structural checks, run on "shadow" values;
- per-segment outward interpolation (D2 5b.3);
- eager U: any note makes the whole predicate U (D2 5b.3).

`evaluate` and every point-path line are untouched.

**ROOT ruling 3 (no panics).** The added test `interval_mode_never_panics_where_the_point_path_can` feeds the exact inputs on which the point path panics, with point inputs and with interval inputs:
- an overflowing same-dimension quotient, 1e308 / 1e-308;
- a NaN argument to interpolate, to step lookup and to exact lookup, from `(z·1e300)·1e300 − (z·1e300)·1e300`.

Interval mode reads U every time and never panics. The scratch probe `_run_records/final/probe_panics.summary.txt` runs the same inputs through both paths:

| Input | Point path | Interval mode / bounded runner |
|---|---|---|
| overflowing quotient | **PANIC** at base `lib.rs:1457` (`ratio_quantity` expect) | quantity with no enclosure, note `non_finite_enclosure` |
| NaN interpolate argument | **PANIC** at `lib.rs:983` | no enclosure, notes `non_finite_enclosure`, `table_argument_range` |
| NaN step-lookup argument | **PANIC** at `lib.rs:968` | same |
| runner, actual = 1e308, limit = 1e-308 | `run_rule_checks` **PANIC** | `run_rule_checks_with_bounds` (b = 1e292): `RULE_INPUTS_INCOMPLETE` with `RULE_RESULT_INDETERMINATE` |

The runner's committed test `bounded_checks_never_panic_where_the_point_path_can` pins the bounded side. When mutant R2 routes the bounded check through the point path, that test fails with the base panic (`mut_R2` log). A check with no interval input still follows the point path, so the panic remains reachable there exactly as today. That is T3-SI1b's.

### 2.2 Runner wiring (deliverable 2; the interface ROOT accepted)

**Public additions** (no existing struct or signature changed):
- `SolverResultBound { input_id, absolute_bound }`;
- `run_rule_checks_with_bounds(&RuleCheckRunInput, &[SolverResultBound])`;
- the constants `RULE_INTERVAL_ALL_PASS`, `RULE_INTERVAL_ALL_FAIL`, `RULE_RESULT_INDETERMINATE` and `RULE_INTERVAL_DIVIDE_BY_ZERO_RANGE`.

`run_rule_checks(input)` now delegates with no bounds. Its path is unchanged; control 1 shows this.

**Bounds** are consulted only for a bound, unrefused `solver_result` input:
- A b that is negative, NaN or infinite gives a blocking completeness finding, `RULE_EVALUATOR_ERROR`. The input is treated as unsupplied (note `invalid absolute bound <b>: treated as unsupplied`), and the policy's `missing_input` code applies.
- **b = 0** binds the exact point q: no note, and output identical to having no bound.
- **b > 0** binds `enclosure_from_bound(q, b)`. Its ends are unit-normalized by `normalize_enclosure_to_declared_unit`. That function mirrors the units crate's `((x·f_from + o_from) − o_to) / f_to`, with one outward step after each of the four operations. Identical units, or units that resolve to the same catalog unit, convert exactly, as the point does. Factors must be positive, which makes every step monotone. The point itself is normalized by the unchanged `normalize_value_to_declared_unit`.
- `BoundInput.note` reads `interval ±<b> from receipt`, with b in Rust `{:e}` form, for example `interval ±1e0 from receipt`.

**Interval mode** runs only when at least one of the formula's bound variables carries an interval. Everything else is the point path.

**The outcome** (D2 §4.11.5):
- **T:** `USER_RULE_CHECKED`, with `RULE_INTERVAL_ALL_PASS` (info).
- **F:** `USER_RULE_FAILED`, with `RULE_INTERVAL_ALL_FAIL` (info).
- **U:** `RULE_INPUTS_INCOMPLETE`, with `RULE_RESULT_INDETERMINATE` (warning).

Details:
- Each code goes into `diagnostic_codes` and into an `evaluator_findings` entry whose subject is the check id.
- The message is the fixed form `enclosure=[0x<lo>,0x<hi>] unit=<u>`. It is `enclosure=none unit=<u>` when the formula has no enclosure, and `enclosure=none unit=none` for a boolean formula. When the formula has interval notes, `; causes=<tokens>` follows.
- A divisor range containing 0 adds one `RULE_INTERVAL_DIVIDE_BY_ZERO_RANGE` finding (warning, subject `divide`).
- `computed_value` is omitted for an interval quantity check. `limit_value` is the user's point.
- Relation and limit errors are reported exactly as the point path reports them, minus `computed_value`.
- The **synthesized `Compare(formula, relation, limit)`** runs through `evaluate_interval`, with the formula's enclosure entering as a variable overlay. Its structural findings, such as a dimension mismatch against the limit, are therefore the point path's.
- `enforce_declared` applies as today.
- No new wire codes beyond D2's.

### 2.3 Python reference (deliverable 3)

`P/core/analysis_runs/rule_interval.py` mirrors the Rust interval mode decision for decision:
- outward `math.nextafter`;
- the same tie order for min and max, so bits are identical, signed zeros included;
- Kleene logic, eager select, per-segment interpolation and eager U;
- the point path's structural finding codes and subjects;
- its own decoder for the node-tagged formula JSON, which refuses unknown nodes, operators, modes and dimensions as `rule_pack_document::decode_expression` does.

It has no dependency and no I/O, and it is not imported by `analysis_runs/__init__` (outside the fence).

### 2.4 The shared cases (deliverable 4)

`P/fixtures/rule_interval/rule_interval_cases.json` holds **83 cases**:
- 32 quantities with exact enclosure bits;
- truth cases: 9 T, 8 F and 25 U;
- 9 blocked;
- 23 negative controls;
- 9 cases with notes.

**Coverage:**
- every grammar feature: all 25 node and operator tokens, including `unsupported_form` and `unsafe_host_access`;
- every negative control of D2 §4.11.5: `abs(x) ≥ c` straddling 0, `x·x ≤ c`, a divisor range containing 0, `not(x > c)`, two inputs with an interior extremum, `=` and `≠` on overlapping ranges, `select` with a U condition, and interpolation partly out of range;
- boundary straddles for all six comparisons, with strict and non-strict shared ends;
- outward-rounding edges:
  - the step on points;
  - fl(1 + 2⁻⁵³) = 1 hiding a real excess;
  - a subnormal bound;
  - an overflow;
  - an input with no finite enclosure;
  - D2 5b.3's interpolation counterexample;
- eager U in an untaken branch and beside a true `or`;
- structural blocks.

**How the expectations were made.** They were derived by hand in `_run_records/final/gen_cases.py`. Each quantity case spells out its own chain of outward operations, and each truth case states its outcome. Neither evaluator was run to produce them.
- Inputs and expected ends are binary64 bit strings, with a readable copy checked against the bits.
- Formula literals are short decimals that every JSON reader parses exactly. The Python test enforces this, because serde_json's default float parsing is not guaranteed exact for long decimals.

**Parity.** Rust (`rule_check_runner/tests/rule_interval_cases.rs`, using the production `decode_expression`) and Python (`tests/test_rule_interval.py`) each reproduce **all 83** expected outcomes, bit patterns and notes. So Rust and Python are bit-identical on the case file. Both assert every feature is covered and no negative control reads T.

**Independent oracle** (`tests/test_rule_interval.py`): a float transcription of the ordinary point path, and an exact `Fraction` evaluation of the same formula. It is evaluated at sampled points of every input box:
- ends and neighbouring ulps;
- interior points;
- zero and every table row argument with its neighbours;
- exact rational interior points.

It checks that:
- T means every sample passes in both arithmetics;
- F means every sample fails;
- every finite enclosure contains every float and exact value;
- every negative control has a sample that does not pass.

It runs on all 83 cases and on 250 seeded generated formulas through the Python reference.

## 3. Soundness table (control 2): why no straddling result can pass (D2 5b.3 §4.11.4)

**Lemma (each row).** Each operation's enclosure contains both the exact real result and the point path's binary64 result, for every point assignment inside the operand enclosures. The basis is that round-to-nearest is monotone, and `next_down(fl(y)) ≤ y ≤ next_up(fl(y))` for every real y.

**Theorem.** By induction over the tree, the final predicate is T only if it holds at every point of the input box, and F only if it fails at every point. A straddle, where some point fails, can therefore never be T. The runner maps only T to `USER_RULE_CHECKED`.

| Form | Interval rule | Why it encloses | Why a straddle cannot pass | Pinned by |
|---|---|---|---|---|
| input (verified row) | `[nd(fl(q−b)), nu(fl(q+b))]`; b = 0 is the point q | each end is one outward step from a rounded end, so it covers [q−b, q+b] | it defines the box; nothing is decided here | `bound_forms_…`, `variable_*` cases, `an_invalid_bound_blocks_the_input` |
| literal, point variable | `[x, x]` | exact | — | `literal_point` |
| `negate` | `[−hi, −lo]` | exact and monotone | — | `negate_exact` |
| `abs` | lo ≥ 0: e; hi ≤ 0: `[−hi, −lo]`; else `[0, max(−lo, hi)]` | exact; \|x\| is monotone on each sign side, and 0 is reached when straddling | `abs(x) ≥ c` straddling 0 is U | `abs_*` cases; M4b, P5 |
| `add`, `subtract` | `[nd(a.lo ± b.·), nu(a.hi ± b.·)]` | monotone in each operand; fl is monotone; the step covers the exact end | an exceeding exact sum hidden by rounding is U (`rounding_hides_real_excess`) | M1, P1 |
| `multiply` | nd/nu of the min/max of the four corner products | bilinear, so extremes are at corners; fl is monotone | `x·x ≤ c` is U; the dependency effect only widens | `multiply_*`, `square_*`; M4a |
| `divide` | the divisor excludes 0: corners, outward; a divisor containing 0 (even the point 0) or with no enclosure: **eager U** | x/y is monotone in each argument on a 0-free box | the point path blocks at y = 0 somewhere in the box, so U | `divide_*`; M6 |
| `compare` (6) | ≤: T iff a.hi ≤ b.lo, F iff a.lo > b.hi (and the others with strict ends); `=`: T only for two equal points, F iff disjoint; `≠` = Kleene not of `=` | operand enclosures contain every operand value, so a decided comparison holds at every point | overlap means U; shared strict ends are U | `comparisons_…`, 20 compare cases; M2, P2 |
| `not`, `and`, `or` | Kleene | T/F only when the decided operands force it at every point | `T and U` = U, `F or U` = U | `kleene_…`; M7 |
| `select` | condition T/F: that branch; U: hull of the quantity branches, or the agreeing boolean, else U | the point path picks the same branch everywhere (T/F), or one of the two (U); both are enclosed | branches straddling the limit give U | `select_*`; M8 |
| `min`, `max` | endpoint-wise min/max | exact, monotone in each operand | the hull straddle is U | `min/max_aggregate` |
| `interpolate` | argument not inside [first, last]: **eager U**; else, per segment met, the point path's six-operation formula over the clipped argument, each step outward, joined; a point at a row is exact | each operation obeys the lemma, including rounding near rows and cancellation (D2 5b.3) | partly out of range is U; the counterexample is not T | `interpolation_*`, `interpolate_*`; M4c, M10, P3 |
| `lookup` step | not inside: **eager U**; else the hull of the spanned rows (exact) | the step function takes only those row values | spanning rows with different results straddles: U at compare | `lookup_step_*`; M9 |
| `lookup` exact | point: the point path verbatim (value or block); range: **eager U** | exact | a range could miss keys: U | `lookup_exact_*` |
| `unsupported_form`, `unsafe_host_access` | blocked, as the point path | — | blocked is never a pass | `blocked_*` |
| non-finite end, input with no finite enclosure | **eager U** | no finite enclosure is claimed | the point path carries inf/NaN or panics there | `overflow_non_finite`, `input_without_finite_enclosure`; M5, P4 |
| eager U (any note) | the whole predicate becomes U; a quantity loses its enclosure | — | the point path is eager, so a block in any branch is reached | `eager_u_*`; M5, P4 |
| runner unit normalization | the four conversion operations, each outward, with positive factors | monotone; the lemma per operation | a limit at the units crate's own upper end reads U | `unit_normalization_steps_each_end_outward`; R3 |
| runner synthesized `Compare` | `evaluate_interval` on the formula's enclosure against the limit point | as `compare` | q = limit with any b > 0 (1, 1e-12, 2⁻¹⁰⁷⁴) is U although the point path passes | `a_straddling_result_is_indeterminate_never_a_pass`; R5 |
| runner outcome | T → CHECKED, F → FAILED, U → INCOMPLETE | — | U is never CHECKED | R3c, R2 |

The basis of b is unchanged (D2 §4.11.2): it is the stop rule's operational bound, not a forward-error enclosure. So these results are conservative to that standard.

## 4. Controls

### Control 1: the committed differential (point mode unchanged)

**(a) `expression_evaluator`.** I ran `evaluate` over the 69 committed conformance cases plus 36,000 seeded inputs (harness in CHECKPOINT_1). Base and the frozen candidate each give 36,069 lines, sha256 `e74a69c43af2b277…`, **byte-identical**.

**(b) `rule_check_runner`, the brief's control.** I serialized `RuleCheckRunResult` (serde JSON bytes) over:
- **the committed rule packs:** `examples/rule_packs/invented_demo.yaml`, `fixtures/product_preview/invented_demo_rule_pack.json` (byte-identical to the yaml), and the runner's inline test pack; plus the yaml with each acceptability relation (4 valid and 1 invalid token) and with declared units `MPa` and `Pa` (the src-tauri test variant);
- **the run fixtures:** every finite numeric result row of the committed solved-result envelopes (`fixtures/product_preview/invented_mechanics_result*.json` and `fixtures/results/**/*.json`). That is 1,561 distinct (value, unit) pairs, each bound as the pack's solver input in the declared unit and in the row's own unit;
- **five scenarios** on a sample of rows: no solver value, a refused solver result, no limit slot, `MODEL_INCOMPLETE`, and a zero limit.

That is 32,820 runs. Statuses: 10,433 CHECKED, 4,958 FAILED and 17,429 INCOMPLETE. Base `run_rule_checks` is compared with three candidate variants: `run_rule_checks`, `run_rule_checks_with_bounds(&[])`, and inert bounds (b = 0 on the solver inputs, plus bounds on user inputs and unknown ids). **All four dumps are byte-identical**, sha256 `697b31fd1c32c420…`. The harness is `_run_records/final/run_diff_template.rs`.

### Control 2: soundness table

§3. It is backed by the oracle (§2.4) and the Rust property test (4,000 formulas against the point path; checkpoint 1).

### Control 3: mutants, each killed by an assertion

Run on the frozen candidate. **21 of 21 killed.**

| Mutant | Brief | Killed by (count; examples) |
|---|---|---|
| M1 outward step removed (evaluator) | 3a | 6: `every_floating_operation_steps_outward_even_on_points`, `rounding_that_hides_a_real_excess_is_not_a_pass`, … |
| M2 the U arm of every `compare` → T (evaluator) | 3b | 8: comparisons, negative controls, property test |
| **R3c indeterminate collapsed to pass (runner)** | **3c** | 4: `a_straddling_result_is_indeterminate_never_a_pass`, `a_divisor_range_…`, `bounded_checks_never_panic…`, `unit_normalization…` |
| M4a multiply made inward (evaluator) | 3d | 3: sign-combination test, property test, … |
| M4b abs straddle made inward | 3d (extra) | 2 |
| M4c interpolation as D2's old point hull | extra | 2: the counterexample test and the property test |
| M5 eager-U poisoning removed | extra | 3 |
| M6 a divisor containing 0 not refused | extra | 2 |
| M7 Kleene `T and U = T` | extra | 2 |
| M8 select-U takes the then-branch | extra | 2 |
| M9 step hull keeps the first row only | extra | 2 |
| M10 interpolation range check dropped | extra | 2 |
| R2 interval mode bypassed in the runner | extra | 6 (`bounded_checks_never_panic…` fails with the base panic) |
| R3 unit conversion not outward | extra | 1: `unit_normalization_steps_each_end_outward` |
| R4 an invalid bound bound as a point | extra | 1: `an_invalid_bound_blocks_the_input` |
| R5 the formula enclosure collapsed in the synthesized compare | extra | 1: `a_straddling_result_is_indeterminate_never_a_pass` |
| P1 Python outward step removed | parity | 26 |
| P2 Python U arm of `≤` → T | parity | 10 |
| P3 Python interpolation as the old point hull | parity | 4 |
| P4 Python eager U removed | parity | 3 |
| P5 Python abs straddle made inward | parity | 3 |

Scripts and results: `_run_records/final/ee_mutants.py`, `rcr_py_mutants.py`, `*_results_final.json`. Each patch must apply exactly once. The R and P mutants run on a scratch copy and are restored, and the restore is verified by `cmp`.

### Control 4: suites compared with base `c1bfc460fc`

| Suite | Base | Candidate | Change |
|---|---|---|---|
| `expression_evaluator` (lib / corpus / doc) | 31 / 1 / 0 | **49** / 1 / 0 | +18: `interval_tests` (17 at checkpoint 1, plus the no-panic test) |
| `rule_check_runner` (lib / acceptability / demo / doc) | 14 / 4 / 3 / 0 | 14 / 4 / 3 / 0, **+9** `interval_bounds_run`, **+1** `rule_interval_cases` | +10, the two new test files |
| `rule_pack_document` (lib / corpus parity / demo / doc) | 6 / 1 / 3 / 0 | 6 / 1 / 3 / 0 | none |
| src-tauri (lib / main / doc) | 116 / 0 / 0 | 116 / 0 / 0 | none; lock unchanged |
| Python sweep (`pytest -q tests --dist loadscope`) | 3,465 passed, 3 failed, 64 errors, 40 skipped, 130 subtests | **3,635** passed, 3 failed, 64 errors, 40 skipped, 130 subtests | +170: `tests/test_rule_interval.py` (83 + 83 parametrized cases + 4) |

Per-test comparison:
- Rust: every base test name and outcome is present and unchanged (`_run_records/final/*.txt`: 0 removed or changed; added 18 / 10 / 0 / 0).
- Python: `comm` of the junit outcomes. **Every base outcome is identical in the candidate**, and the only additions are the 170 new tests (`sweep_base.outcomes`, `sweep_cand.outcomes`).

**The 3 failures and 64 errors are environmental and identical on both sides** (`sweep_base_environment_causes.txt`):
- The sweep ran with **cargo removed from PATH**, so no test could start an unlocked cargo job:
  - 19 + 1 `test_headless_runner_contract` tests hit `FileNotFoundError: 'cargo'`;
  - 45 `test_binary64_canonical_json_adapter` tests need a second CLI built by cargo;
  - `test_evidence_sweep::test_wasm_artifact_resolver…` runs `cargo metadata` through node.
- The sweep trees are `git archive` copies (no `.git`), so `PinTests::test_pinned_identities_match_recorded_bytes` cannot `git show`.
- Both trees exclude `execution/_Coordination/AgentRuns` (1.9 GB).

How it ran:
- `-n 6` instead of `auto`, on this shared host.
- The checked-JSON and units CLIs were prebuilt from the base archive through `t3_cargo.sh`. Those crates are unchanged.

## 5. Readings, limits and items for ROOT

1. **Cite D2 5b.3.** The implementation follows the amended §4.11.3 (interpolation and the eager-U note) and §4.11.4. Nothing beyond 5b.3 and ROOT's checkpoint rulings was read in.
2. **No ruling is needed for anything new.** The runner behaviour is exactly the accepted list. The remaining choices are textual details:
   - the invalid-bound finding uses the existing code `RULE_EVALUATOR_ERROR`;
   - the boolean-formula message is `enclosure=none unit=none`;
   - interval causes follow the fixed form after `; causes=`;
   - there is one `RULE_INTERVAL_DIVIDE_BY_ZERO_RANGE` finding per check, with severity warning.
3. **A documentation note for a later slice (no schema change made).** `rule_check_run_result.schema.json` describes `RunFinding.severity` as an open string whose "emitted vocabulary at publication is 'blocking'". S-I1 now also emits `info` and `warning`, per D2 §4.11.5. The schema accepts them; its description text is out of date. Updating the description is a schema-file edit outside this fence. Suggested for S-I2 or a documentation closeout.
4. **T3-SI1b.** The point-path panics are reproduced in §2.1, with their base line numbers (1457, 968, 983). Interval mode and the bounded runner path never panic on them.
5. **What S-I2 needs.** The native binding sites can call `run_rule_checks_with_bounds`, building `SolverResultBound` from `rule_binding_interval(…)`, without touching any existing struct. Alternatively, S-I2 adds `SolverResultBinding.absolute_bound` as D2 §4.11.5 names it and maps it onto the side list. The UI texts can read b from `BoundInput.note` or from S-I2's own receipt binding.
6. **Limits:**
   - The runner differential covers the committed packs and run-fixture rows as enumerated. It is not every possible pack.
   - The oracle samples points; it does not cover every binary64 value in a box.
   - Soundness is relative to b's operational basis (D2 §4.11.2).
   - No native app, DEC-025, solver-at-scale, TS or Vitest work was run, and none is in S-I1's write set.

## 6. Host and commands

**Cargo.** Every cargo command ran through `WT/tools/t3_cargo.sh` (the wrapper `_run_records/final/cargo_run.sh`), with:
- `--locked --offline`;
- `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, `RUSTUP_TOOLCHAIN=1.97.1`, `RUSTUP_AUTO_INSTALL=0`, `CARGO_INCREMENTAL=0`;
- targets `WT/targets/i73-s-i1` and `WT/targets/i73-s-i1-base`;
- the memory guard running, and `TMPDIR` under `WT/scratch/i73_s_i1/tmp`.

**Commands:**
- `cargo test --locked --offline` for each of the three rules crates (in the worktree for the candidate; in a `git archive` copy for base) and for src-tauri (both in scratch copies);
- `cargo test --locked --offline --test i73_point_diff` and `--test i73_run_diff` for the differentials;
- `cargo test --locked --offline --lib` for the evaluator mutants and full-crate runs for the runner mutants;
- `cargo build --locked --offline --release …` for the two CLIs the Python harness needs.

**Python.** `VENV/bin/python -m pytest -q -p no:cacheprovider`, with `PYTHONDONTWRITEBYTECODE=1`.

**Formatting.** `rustfmt` 1.97.1 was applied to the evaluator (base was clean, and only the inserted hunks changed) and to the two new test files. In the runner file it was applied only to the new code, because base's existing code is not `rustfmt`-clean and was left alone. Every suite, differential and mutant above was re-run after formatting.

**No** Git writes, installs, new dependencies, lock moves, native or DEC-025 jobs, or writes to the system temp directory.

**Scratch.** `WT/scratch/i73_s_i1/` holds the base and candidate archives, the sweep trees (about 0.8 GB each), the 10 MB and 60 MB dumps, and the logs. It is left for ROOT to prune.

**Host note.** As at checkpoint 1, this agent switched between `WT/s-i1` and NUM with the host's supported worktree switch, with no Git write.

## 7. Run records (`_run_records/final/`, sanitized to `WT`/`<WT>`/`<VENV>`)

- `gen_cases.py`: the case generator, with the independent expectations.
- `run_diff_template.rs`: runner control 1.
- `differential_dump_sha256.txt`.
- `i73_panic_probe.rs` and `probe_panics.summary.txt`.
- `ee_mutants.py`, `rcr_py_mutants.py` and `*_results_final.json`.
- Suite summaries and per-test outcome lists for base and candidate.
- `sweep_base.outcomes`, `sweep_cand.outcomes`, `sweep_outcomes.py`, `sweep_*.summary.txt` and `sweep_base_environment_causes.txt`.
- `run_sweep.sh`, `run_py.sh` and `sync_cand.sh`.

The checkpoint-1 records in `_run_records/checkpoint1/` are unchanged.
