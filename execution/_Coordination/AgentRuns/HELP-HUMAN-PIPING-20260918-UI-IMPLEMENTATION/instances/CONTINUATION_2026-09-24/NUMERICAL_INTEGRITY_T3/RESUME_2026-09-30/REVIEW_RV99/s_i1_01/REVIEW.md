# RV99: independent review of S-I1 (option C's interval evaluator)

**Reviewer:** RV99, TASK (Type 2), dispatched directly by ROOT (HELP_HUMAN, Agent 0). ROOT is the return path. No descendants. I wrote none of the work under review and I am not an F2a reviewer.

**Placeholders:** `WT` is the T3 host root; `NUM = WT/numerics`; `P = projects/chirality-piping`; `T` is the T3 records folder; `R = T/RESUME_2026-09-30`; `RR = T/ROOT_RULINGS_V1.md`; D2 is `T/DESIGN_STANDING/DESIGN.md`; `VENV` is the piping virtual environment. No machine paths are recorded here or in `evidence/`.

**Brief and basis read** (sha256):
- `NUM/AGENTS.md` (`f96feb19…`), `NUM/agents/AGENT_TASK.md` (`1a13a5b0…`), `NUM/P/AGENTS.md` (`d9f2b23a…`);
- `R/BRIEFS/RV99_S_I1_REVIEW.md` (`2a63fae8…`, verified), and ROOT's dispatch text with ROOT ruling 3's no-panic item and the schema-edit item;
- D2 **revision 5b.3** (`993f5f3a…`, the same bytes I73 cites): §0.5 (row 5b.3) and §4.11.1–§4.11.6;
- RR "I73's checkpoint 1 and I74's plan ruled; D2 5b.3; the T6 slice dispatched" (readings 1–5) and "S-I1 committed; RV99 dispatched" (RR read at `cf241f7d…`);
- `R/I61/u8_plan_01/PLAN.md` §3 (`f274a614…`);
- I73's `R/I73/s_i1_01/RETURN.md` (`ab36c3d3…`, verified) and `CHECKPOINT_1.md` (`b992efcf…`, verified), and I73's mutant scripts in `_run_records/final/` (their patch texts only).

**Candidate:** `c1bfc460fc..8f956d399a` on `codex/piping-s-i1-20261005`: `4920e4b1b0` (I73's work, 7 files) and `8f956d399a` (ROOT's one-line description edit in `P/schemas/rule_check_run_result.schema.json`). 8 files, +8,452 / −4. Read with `GIT_OPTIONAL_LOCKS=0`; nothing written to `WT/s-i1`.

**Independence.** I built my own oracles and did not rely on I73's tests or expected values:
- **My own point-path oracle:** the ordinary `evaluate` and `run_rule_checks` (textually unchanged from base, and byte-identical to it by §2) run at sample points of every input box, with bisection to every pass/fail flip point (each flip contributes both adjacent binary64 values), and a dense re-sampling (up to 4,096 values per input) of every decided case.
- **My own exact oracle:** the formula over exact rationals (`Fraction`) with eager undefinedness, at sampled points.
- **My own enclosure transcription** of D2 §4.11.2, compared bit for bit with Rust's `enclosure_from_bound`, plus exact containment of [q − b, q + b].
- **My own cases:** a seeded generator (not I73's), RV99's straddle matrix, negative controls and targeted corners. I73's 83 shared cases were also re-run through my oracle with my own samples.
- **My own point-mode differential**, packs and run-fixture rows (not I73's template).
- **16 mutants of my own**, beside I73's 21 rerun on my copy.

## Verdict: **PASS**

| | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 3 |
| NOTE | 6 |

**The headline:**
- **No soundness violation anywhere.** Over 30,358 evaluator cases (4.87 million point-path samples, 450,297 exact-rational samples) and 6,615 runner cases (832,113 point-path samples), no T or F disagrees with the point path or with exact arithmetic, and every finite enclosure contains every point-path and exact value. **All 7,999 evaluator boxes and all 1,548 runner boxes in which the point path both passes and fails read U** (`RULE_INPUTS_INCOMPLETE` with `RULE_RESULT_INDETERMINATE` in the runner). I73's soundness table holds rule by rule.
- **Ruling 3 holds.** Interval mode and every bounded check never panicked, including on 150 evaluator boxes and 16 runner boxes where the point path panics (the overflowing same-dimension quotient, and NaN arguments to interpolate and step lookup). See N-2 for the b = 0 reading.
- **Point mode is byte-identical** to the base over my differential: 2,888 packs (both committed rule packs, 10 demo variants, 2,876 generated), 63,086 runs including all 1,863 finite (value, unit) rows of the 80 committed run fixtures. `run_rule_checks`, `run_rule_checks_with_bounds(&[])` and inert bounds all give the base's sha256 `a2bbfa3c…`; the evaluator's point path `e8db0906…` on both sides.
- **Rust and Python agree bit for bit** (truth, enclosure bits, notes, finding codes and subjects) on I73's 83 cases and on 30,275 cases of my own. They diverge only on invalid inputs (S-3).
- **I73's 21 mutants are all killed** as I73 reported. Of my 16, 13 are killed and 3 survive (V3, V9, V11). V3 and V9 are unsound and show two test gaps (S-1, S-2).
- **Fence and contracts hold.** Only the fenced files plus ROOT's description edit changed. The schema's shape and validation are identical, and its new wording matches the severities the runner emits. No manifest, lock or dependency moved, src-tauri's included. Codes and outcomes match D2 §4.11.3 and §4.11.5.

## Findings

| # | Sev | Where | Evidence | Remedy |
|---|---|---|---|---|
| S-1 | SHOULD-FIX | Tests: `expression_evaluator` `interval_tests`, the shared case file, `rule_interval_cases.rs`, `test_rule_interval.py` | **`=`/`≠` between two inputs with identical non-degenerate enclosures is not pinned.** My mutant V3 (`interval_equal` returns T when `a.lo == b.lo && a.hi == b.hi`) is unsound: `x = y` with x and y each in [1 − 2⁻⁴⁰, 1 + 2⁻⁴⁰] reads T, while the point path returns false at x ≠ y (my oracle: `T_but_point_not_true`, `exact_contradicts_truth`; `evidence/mutants/V3_on_pass1_check.txt`). **V3 survives every committed suite** (evaluator lib, runner, shared cases). The case file's equality cases all compare an interval with a point (`equal_overlap`, `equal_points`, …). The production code is correct. | Add a shared case (Rust and Python) with two independent inputs bound to the same q and b: `x = y` reads U and `x ≠ y` reads U. Keep `x = x` out of it (true everywhere, so U is merely conservative there). |
| S-2 | SHOULD-FIX | Same tests; `interval_contains_zero` | **A divisor range with a zero end is pinned only through a note code.** My mutant V9 (strict `e.lo < 0.0 && 0.0 < e.hi`) is unsound: `0 / (−abs(x)) ≤ 1` with x straddling 0 has the divisor enclosure [−1, −0]; the corner quotients are −0 and NaN, `min2`/`max2` skip the NaN, and V9 reads **T** where the point path blocks (`DivisionByZero`) at x = 0 (`evidence/mutants/V9_on_extra_corners_check.txt`). V9 survives the evaluator's own suite (I73's criterion for evaluator mutants) and is killed only incidentally, by the note code of `divide_by_zero_point` in the shared cases (`non_finite_enclosure` instead of `divide_by_zero_range`). | Add the case `0 / (−abs(x)) ≤ 1`, x straddling 0, expecting U with `divide_by_zero_range`, to the evaluator tests and the shared cases. |
| S-3 | SHOULD-FIX | `P/core/analysis_runs/rule_interval.py`, `evaluate_interval` input handling | **The Python reference fails open on invalid inputs where Rust refuses** (`evidence/probes/python_reference_invalid_inputs.txt`). A NaN or negative `bound` is treated as "no bound" (`item.get("bound", 0.0) > 0.0`), so `x ≤ 10` with q = 9.5 reads **T**. Rust's `enclosure_from_bound` returns `None` (U) for these, and the runner blocks them (`RULE_EVALUATOR_ERROR`, input unsupplied). Explicit `enclosure` inputs are not validated either: an inverted (11, 9), a NaN end or an infinite end all read **T**, where Rust's `build_interval_overlays` blocks (`InvalidReference` / `NonFiniteInput`). The module claims to mirror Rust "decision for decision". No product code calls it (D2: parity cases and the validation harness only), so this is not blocking. | Mirror Rust: a present `bound` that is not finite and non-negative gives an overlay of `None` (U) or a blocking finding, and explicit enclosures get Rust's two checks with the same codes and subjects. Add parity cases for each. |
| N-1 | NOTE | `rule_check_runner` `normalize_enclosure_to_declared_unit`; `interval_bounds_run.rs::unit_normalization_steps_each_end_outward` | **The lower end of the unit-normalization chain is not pinned bit for bit.** My mutant V11 (the lower end's offset step taken upward) survives the runner suite: I73's test pins the upper end exactly but the lower end only against `lo × 0.9999`. My point-path oracle cannot see V11, or I73's R3 (no outward steps in the conversion), because the remaining outward steps absorb them: the enclosure still contains the point path's conversion of every box value. They breach only D2 §4.11.2's "one outward step after every floating operation" and exact-real containment. | Optional: pin the lower end's bits in that test, as for the upper end. |
| N-2 | NOTE | ROOT ruling 3; `run_rule_checks_with_bounds` | **Reading for ROOT to confirm.** With b = 0, or no bound, a check runs on the point path, so `run_rule_checks_with_bounds` still panics on T3-SI1b's input (probe j: `actual` = 1e308, `limit` = 1e-308, b = 0). With any b > 0, a subnormal included, it reads U and does not panic. I read ruling 3's "bounded runner path" as checks with at least one interval input (b > 0), and the b = 0 panic as T3-SI1b's, since D2 §4.11.2 makes b = 0 the exact point with no interval code. | ROOT to confirm. T3-SI1b's repair covers the b = 0 route automatically. |
| N-3 | NOTE | `P/schemas/rule_check_run_result.schema.json`, `RunFinding.severity` description (ROOT's edit) | **The edit is description-only and its new sentence matches what the runner emits.** The retained clause "emitted vocabulary at publication is 'blocking'" was **already incomplete on base**: completeness findings carry the completeness checker's own severity, for example `RuleIncompleteData` with `info` when the caller's statuses include `USER_RULE_CHECKED` (probe a, plain `run_rule_checks`), and `UnexpectedSuppliedInput` is `Warning` in the checker. S-I1 did not introduce this. | Optional, in a later documentary pass: say that completeness findings carry the checker's severities. |
| N-4 | NOTE | `rule_check_runner` `finish_interval_check` → `enforce_declared` | **A downgraded interval check keeps its interval code.** In a pack whose `result_statuses` omit `USER_RULE_CHECKED`, an all-pass interval check becomes `RULE_INPUTS_INCOMPLETE` with `STATUS_NOT_DECLARED`, but keeps `RULE_INTERVAL_ALL_PASS` in `diagnostic_codes` and its info finding (probe b). The point path emits no code in that case. This is never a pass, but D2 §4.11.5 pairs `RULE_INTERVAL_ALL_PASS` with `USER_RULE_CHECKED`. | For S-I2: key the UI text on the status first, or push the code after `enforce_declared`. |
| N-5 | NOTE | `run_rule_checks_with_bounds`, `bound_by_input` | **Duplicate bounds for one input: the last one wins silently** (a `HashMap` collect). With [1, 60] the check reads U; with [60, 1] it reads `USER_RULE_CHECKED` (probe c). This is the same convention as duplicate `SolverResultBinding` values today, and there is no caller yet. | For S-I2: pass one bound per input, or have the runner refuse duplicates. |
| N-6 | NOTE | Runner message and note forms | **The forms go beyond D2's fixed form,** as I73's readings list: a `; causes=<tokens>` suffix, `enclosure=none unit=<u>`, and `enclosure=none unit=none` for boolean formulas. `BoundInput.note` renders b in Rust `{:e}` (`interval ±1e0 from receipt`). A bound on an input that the check lists but its formula does not use still gets the interval note, although the check runs on the point path (probe g). | For S-I2/T6 consumers: parse the suffix, and do not infer "interval-evaluated" from the note alone. |

S-1 to S-3 are cheap and test- or reference-only; none changes production Rust. I recommend repairing them before the PR, and will confirm the repairs.

## 1. Item 1: soundness

### 1.1 The code against D2 5b.3, rule by rule

I read the whole interval mode (`expression_evaluator/src/lib.rs`, candidate lines 1468–2537) and the runner wiring against D2 §4.11.2–§4.11.4 and the base point path.

| Form | Implementation (candidate) | Encloses, and why | Verdict |
|---|---|---|---|
| Input | `enclosure_from_bound`: `[nd(fl(q−b)), nu(fl(q+b))]`; b = 0 is the point; non-finite q or b, b < 0, or a non-finite end give `None` | `nd(fl(y)) ≤ y ≤ nu(fl(y))` for round-to-nearest, subnormals included; my transcription equals Rust's on every case and contains [q−b, q+b] exactly | ✓ |
| Literal, point variable | `[x, x]` from the point path's own literal and variable checks | exact | ✓ |
| `negate` | `[−hi, −lo]` | exact | ✓ |
| `abs` | lo ≥ 0: e; hi ≤ 0: `[−hi, −lo]`; else `[0, max(−lo, hi)]` | exact; −0.0 ends are numerically harmless | ✓ |
| `add`, `subtract` | `outward(a.lo + b.lo, a.hi + b.hi)`, `outward(a.lo − b.hi, a.hi − b.lo)` | the point path's `l + 1.0·r` and `l + (−1.0)·r` are the same single rounding; monotone | ✓ |
| `multiply` | min/max of four rounded corners, then outward | bilinear; `fl` monotone; finite × finite is never NaN, and any overflow reaches an end and gives `None` | ✓ |
| `divide` | divisor enclosure containing 0 (`lo ≤ 0 ≤ hi`, the point 0 included) or `None`: note `divide_by_zero_range`; else four rounded quotients, then outward | monotone on a 0-free box; an overflow gives `None` | ✓ (S-2: the zero-end case is unpinned) |
| `compare` (6) | ≤: T iff a.hi ≤ b.lo, F iff a.lo > b.hi; <: T iff a.hi < b.lo, F iff a.lo ≥ b.hi; ≥ and > symmetric; `=`: T iff two equal points, F iff disjoint; `≠` = Kleene not of `=` | checked against every strict and non-strict shared end in §1.3 | ✓ (S-1: identical ranges unpinned) |
| `not`, `and`, `or` | Kleene, exhaustive matches | — | ✓ |
| `select` | T or F: that branch; U: hull of quantity branches, or the agreeing boolean, else U; eager | the point path picks one of the two at every point | ✓ |
| `min`, `max` | endpoint-wise | exact; a point result implies a constant point value | ✓ |
| `interpolate` | outside [first, last]: note `table_argument_range`; a point at a row: that row's exact result; else for every segment met, the point path's six-operation formula over the clipped argument, each step outward, joined; a `run` containing 0 gives `None` | each operation obeys the lemma; at a segment end the chain also encloses the exact row value the point path returns (the exact chain equals the row there). D2 5b.3 | ✓ |
| `lookup` step | not inside: note; else the hull of the rows governed at lo through hi (`step_governing_row` equals the point path's `rev().find(≤)`) | monotone governing index | ✓ |
| `lookup` exact | a point argument follows the point path verbatim (value or block); otherwise note `exact_lookup_range` | a point enclosure means a constant argument | ✓ |
| `unsupported_form`, `unsafe_host_access` | blocked by the point path's own arm | — | ✓ |
| Non-finite end; input without a finite enclosure | `outward` refuses non-finite ends before and after the step; overlay `None`; note `non_finite_enclosure` | — | ✓ |
| Eager U | any note makes the value U, and a quantity loses its enclosure (`into_public`) | blocks and non-finite values anywhere in the box are possible at some point; D2 5b.3 | ✓ |
| Structure | the point path's own checkers on a shadow value 1.0 (never a zero divisor), and line-by-line mirrors of `eval_aggregate` and `eval_table_expression` | same findings as the point path wherever its block is structural | ✓ |
| Runner: when | interval mode only when some formula variable carries an interval (b > 0); otherwise the point path | — | ✓ |
| Runner: units | `((x·f_from + o_from) − o_to) / f_to`, one outward step after each of the four operations; positive factors are checked; identical symbols or the same catalog unit pass through | the units crate does the same two-plus-two roundings (no FMA in Rust) | ✓ (N-1) |
| Runner: synthesized `Compare` | `evaluate_interval` on `interval_formula_value` (the formula's enclosure, or `None` if it has notes) against the user's point limit | as `compare`; the limit is a point on both paths | ✓ |
| Runner: outcome | T → `USER_RULE_CHECKED` + `RULE_INTERVAL_ALL_PASS` (info); F → `USER_RULE_FAILED` + `RULE_INTERVAL_ALL_FAIL` (info); U → `RULE_INPUTS_INCOMPLETE` + `RULE_RESULT_INDETERMINATE` (warning); then `enforce_declared` | U is never CHECKED | ✓ (N-4) |

Other things I checked:
- **No interval enclosure is ever inverted.** Every rule keeps lo ≤ hi, and the interpolation clip is non-empty under its overlap test.
- **`step_lookup_enclosure`'s slice is always valid,** a one-row table included.
- **Every match is exhaustive with no `_` arm,** so a new grammar operator fails to compile until it gets a rule. That is D2's enforcement.

### 1.2 I73's soundness table (RETURN §3), rule by rule

| I73 row | Checked against | Result |
|---|---|---|
| input (verified row) | code, my enclosure transcription and exact containment | confirmed |
| literal, point variable | code | confirmed |
| `negate` | code | confirmed |
| `abs` (three branches; straddle U) | code; my `neg_abs` case reads U | confirmed |
| `add`, `subtract` ("an exceeding exact sum hidden by rounding is U") | code; my `neg_cancel` and exact oracle | confirmed |
| `multiply` (corners; `x·x ≤ c` U) | code; my `neg_square`, `neg_square2` | confirmed |
| `divide` (eager U on a 0-containing or missing divisor) | code; my `neg_div0`, the zero-end corners | confirmed (S-2) |
| `compare` (6; shared strict ends U) | code; my 588-case straddle matrix (§1.3) | confirmed (S-1) |
| `not`, `and`, `or` | code; my `neg_eager_or`, mutant V4 | confirmed |
| `select` | code; my `neg_select`, `neg_eager_select`, mutant V5 | confirmed |
| `min`, `max` | code; my `neg_min_max` | confirmed |
| `interpolate` (per segment, outward, joined; the counterexample not T) | code; 40 perturbed counterexamples of my own, mutants M4c and V7 | confirmed |
| `lookup` step | code; my row-end corners (mutant V6 found by them) | confirmed |
| `lookup` exact | code; my `neg_exact_range` | confirmed |
| `unsupported_form`, `unsafe_host_access` | code | confirmed |
| non-finite end, no finite enclosure | code; my ruling-3 and extreme cases | confirmed |
| eager U (any note) | code; mutants M5 and V14 | confirmed |
| runner unit normalization | code; random catalog conversions (stress and affine temperature) | confirmed (N-1) |
| runner synthesized `Compare` | code; 1,548 runner straddles all U | confirmed |
| runner outcome | code; my checker on every runner case | confirmed (N-4) |

I73's basis note (b is operational evidence, not a forward-error enclosure) is D2 §4.11.2's own, and I agree with it.

### 1.3 My straddle cases at each comparison boundary

**The matrix:** `x op c` and `c op x` for all six operators. The limits are c ∈ {−3.5, 0, 1, 100, 1e−300, 1e300, 5e−324}, and each is set against seven boxes:
- below;
- the enclosure's upper end equal to c;
- straddling c;
- the lower end equal to c;
- above;
- the exact point c;
- a wide box.

That gives 588 cases (`evidence/soundness/straddle_matrix.txt`). Two more groups:
- **24 pairs,** x op y, with a shared end, the same box, disjoint boxes and overlapping boxes;
- **18 of my own instances of D2 §4.11.5's negative controls,** plus x − x = 0, a cancellation, and max < min on equal boxes.

**What every box gives:**
- **Every box in which the point path both passes and fails reads U.** The ends are sampled, and the shared-end cases contain c as a sample, so "end equals c" is a true straddle for the strict operators.
- **A decided box reads T or F only when every sample agrees.**
- **The exact point reads the point path's answer.**

The matrix pattern is what D2 prescribes: for example, with x's upper end at c, `x ≤ c` reads T and `x < c` reads U. For c = 0 and c = 5e−324, the tiny bound underflows to 0, so those boxes are points.

**Totals:**
- **Evaluator:** across all runs, 7,999 evaluator boxes straddle (903 in pass 1), and all read U.
- **Runner:** across pass 1 and seeds 1–4, 1,548 runner boxes straddle (149 in pass 1), and all read `RULE_INPUTS_INCOMPLETE`, with `RULE_RESULT_INDETERMINATE` whenever evaluation was reached (`evidence/soundness/straddles_and_panics.txt`).

### 1.4 The oracle runs

**Rust:** one scratch harness (`evidence/tools/rv99_harness.rs`), compiled into my copy only. For each case it calls `evaluate_interval` (or `run_rule_checks_with_bounds`), and the ordinary point path at every sample, under `catch_unwind`.

**Python** (`rv99_check.py`) then checks:
- **T:** every sample `true`, and the exact predicate true;
- **F:** every sample `false`, and exact false;
- **an enclosure:** every point-path value and exact value inside it;
- **"decided":** never with an exact zero divisor, out-of-range argument or missed key;
- **a structural block:** the point path blocks at every sample;
- **Rust/Python parity,** the input enclosures, and the runner's codes, severities, subjects, message form, note form and b = 0 identity.

| Run | Cases | Point samples | Exact samples | T / F | Violations |
|---|---|---|---|---|---|
| Pass 1 (seed 990099): matrix, negatives, interpolation family, ruling 3, 2,500 random formulas over the whole grammar | 3,227 eval + 603 runner | 247,142 + 11,448 | 39,290 | 426 / 411 | 0 |
| Dense re-sampling of every decided pass-1 case (up to 4,096 values per input, ±32 ulps around ends, q, literals and table arguments) | 1,080 eval + 341 runner | 2,394,299 + 707,783 | 59,751 | 426 / 411 | 0 |
| Seeds 1–4, 6,000 random formulas and 1,500 runner cases each | 26,908 eval + 6,012 runner | 2,227,557 + 112,882 | 348,897 | 2,893 / 2,901 | 0 |
| I73's 83 shared cases, re-sampled by me | 83 | 4,579 | 2,149 | 9 / 8 | 0 |
| Targeted corners (zero-end divisors; table arguments ending exactly on a row) | 140 | 904 | 210 | 35 / 35 | 0 |

**The oracle is not vacuous.** It flags 35 of the 37 mutants as unsound or divergent; only R3 and V11 escape it (§4, N-1). For example:
- M1 (no outward step) gives 2,711 soundness violations;
- M2 (the U arm of `compare` replaced by T) gives 2,127;
- M4c (D2's old interpolation hull) gives 4;
- V9 and V6 are caught on the targeted corners.

### 1.5 Ruling 3: no panics

**Inputs:**
- **Evaluator:** the overflowing same-dimension quotient (1e308 / 1e-308, stress / stress) with point overlays and with b ∈ {1, 1e292} and subnormal bounds;
- **NaN arguments:** `(z·1e300)·1e300 − (z·1e300)·1e300` fed to interpolate, step lookup and exact lookup, as predicates and as quantities;
- **NaN or infinity elsewhere:** through min, max, select, abs, negate, inf/inf and inf × 0;
- **extreme bounds:** f64::MAX, subnormal, and 1e308 ± 1e308;
- **runner:** `actual` = 1e308 against `limit` = 1e-308 with b ∈ {1e292, 1, 1e-300}.

**Result:**
- **The point path panics** in 150 evaluator boxes and 16 runner boxes (base lines 1457, 968 and 983, as I73 reported).
- **Interval mode and the bounded runner never panicked.** Every such box reads U, or a quantity with no enclosure, with `non_finite_enclosure` and, for the tables, `table_argument_range` or `exact_lookup_range`. The runner reads `RULE_INPUTS_INCOMPLETE` + `RULE_RESULT_INDETERMINATE` (`enclosure=none unit=ratio; causes=non_finite_enclosure`).
- **The Python reference** raised no exception on any of them, and agrees.
- For b = 0 see N-2.

## 2. Item 2: point mode unchanged

**Textually:**
- **The evaluator** gains two pure insertion hunks (+1,070 after `validate_finite`, +1,127 test module; 0 lines removed).
- **The new items** add no impl of an existing type. So `evaluate` and its helpers are unchanged by construction.
- **The runner** removes 3 lines: the import list rewrapped, and the `Ok(q) => bindings.push(…)` arm, now a block that pushes the same binding and records an interval only when one exists.
- **With no bound,** `bound` is `None` for every input and `intervals` stays empty, so `run_one_check` reaches the unchanged point path.

**My differential** (`evidence/differential/`, tools `diff_prep.py` and `rv99_diff.rs.tmpl`) uses the same input file on base and candidate. Packs:
- `examples/rule_packs/invented_demo.yaml` and `fixtures/product_preview/invented_demo_rule_pack.json`, the two committed packs (equal as JSON);
- 10 variants of the demo pack: the four ordering `acceptability_relation` tokens, `equal` and `bogus` (both refused by the runner) and the empty token; declared units MPa and Pa; a `result_statuses` without `USER_RULE_CHECKED`;
- 2,876 packs built from my generated formulas.

Runs:
- **Fixture rows:** every finite (value, unit) row of the run fixtures (`fixtures/product_preview/invented_mechanics_result*.json` and `fixtures/results/**/*.json`: 80 files, 1,863 distinct rows), bound in the declared unit and in the row's own unit;
- **Scenarios:** no solver value, a refused solver result, no slot, `MODEL_INCOMPLETE`, a zero limit, a negative limit with `HUMAN_APPROVED_FOR_PROJECT`;
- **Formula packs:** six sample points each.

Total: **63,086 runs** (18,128 checked, 12,056 failed, 32,902 incomplete).

| Dump | Base `c1bfc460fc` | Candidate `8f956d399a` |
|---|---|---|
| `run_rule_checks` (serde bytes per run) | `a2bbfa3c…` | `a2bbfa3c…` |
| `run_rule_checks_with_bounds(&[])` | — | `a2bbfa3c…` |
| inert bounds (b = 0 on every solver input; 7.5 on user inputs and slots; an unknown id) | — | `a2bbfa3c…` |
| evaluator `evaluate` `Debug` + value bits on the same runs | `e8db0906…` | `e8db0906…` |

In my runner cases, b = 0 also gave the no-bound bytes in every case (603 + 6,012). The exceptions are the T3-SI1b inputs, where both sides panic identically (N-2).

**Suites** (`evidence/suites/`; my copies, my targets):

| Suite | Base | Candidate |
|---|---|---|
| `expression_evaluator` lib / corpus / doc | 31 / 1 / 0 | 49 / 1 / 0 |
| `rule_check_runner` lib / acceptability / demo / doc | 14 / 4 / 3 / 0 | 14 / 4 / 3 / 0, plus `interval_bounds_run` 9 and `rule_interval_cases` 1 |
| `rule_pack_document` lib / corpus parity / demo / doc | 6 / 1 / 3 / 0 | 6 / 1 / 3 / 0 |
| Python `test_rule_interval.py` | — | 170 passed |
| Python related walkers and readers (`test_operation_result_schemas`, `test_ci_numerical`, `test_rule_pack_schema`, `test_retained_precision_carriers`) | outcomes `py_related_base.outcomes` | identical outcomes (5, 8 and 5 passed; the same 24 failures on both sides) |

The 24 failures are environmental: the checked-JSON authority was stubbed, so that no test could start cargo outside the lock. They include `test_private_seams_have_no_product_callers`, which walks `core/**/*.py` and passes. Every count change is an added test, as I73 reported.

## 3. Item 3: parity

- **The case file:** the Rust `rust_interval_mode_matches_every_shared_case` and the Python 83 + 83 parametrized cases both pass on my copy. I also ran the 83 cases through my harness with my own samples. Rust and Python agree on truth, enclosure bits, notes, and finding codes and subjects for every case, and my oracles accept every outcome.
- **My own cases:** 30,275 cases, compared field by field:
  - the deterministic set, in each of the five generator runs: the 588-case boundary matrix, 24 pairs, 18 negative controls, 40 interpolation variants, and 57 ruling-3 and extreme cases;
  - 26,500 random formulas with random bounds around the limit (the limit at the point value, its neighbours, a relative perturbation, or random);
  - 140 targeted corners.

  **0 mismatches.**
- **Dimensions:** Rust prints them as `Debug` (`Force`) and Python as the token (`force`), so my checker maps the case. This is not a mismatch.
- **Invalid inputs** are the one divergence (S-3).

## 4. Item 4: mutants

Each patch applies exactly once to my copy and is restored, and every restore was verified by sha256. At the end, the copy equals `git archive 8f956d399a` file for file. Kill criteria are I73's:
- **evaluator mutants:** `cargo test --lib` in `expression_evaluator` must fail (I also report the runner suite);
- **runner mutants:** the full runner suite must fail;
- **Python mutants:** `test_rule_interval.py` must fail.

"RV99 oracle" counts my oracle's soundness violations on pass 1, or on the corners (`evidence/mutants/results.json`).

| Mutant | Killed | Failing tests (runner suite) | RV99 oracle (soundness / other) |
|---|---|---|---|
| M1 outward step removed | yes | 6 (2) | 2,711 / 4,966 |
| M2 U arm of `compare` → T | yes | 8 (3) | 2,127 / 1,051 |
| M4a multiply inward | yes | 3 (1) | 33 / 29 |
| M4b abs straddle inward | yes | 2 (1) | 41 / 25 |
| M4c interpolation as D2's old hull | yes | 2 (1) | 4 / 27 |
| M5 eager-U poisoning removed | yes | 3 (1) | 99 / 56 |
| M6 0-containing divisor not refused | yes | 2 (2) | 56 / 112 |
| M7 Kleene `T and U = T` | yes | 2 (1) | 2 / 1 |
| M8 select-U takes then | yes | 2 (1) | 25 / 21 |
| M9 step hull first row only | yes | 2 (1) | 8 / 4 |
| M10 table range check dropped | yes | 2 (1) | 44 / 88 |
| R3c indeterminate collapsed to pass | yes | 4 | 209 / 470 |
| R2 interval mode bypassed | yes | 6 | 151 / 541 |
| R3 conversion not outward | yes | 1 | 0 / 0 (equivalent at the point level; N-1) |
| R4 invalid bound bound as point | yes | 1 | 0 / 4 |
| R5 formula enclosure collapsed in compare | yes | 1 | 37 / 0 |
| P1–P5 (Python) | yes | 26, 10, 4, 3, 3 | parity 696, 186, 27, 56, 24 |
| **V1** `<` strictness lost (T iff a.hi ≤ b.lo) | yes | 2 (1) | 60 / 30 |
| **V2** `≥` T uses the upper end | yes | 4 (1) | 297 / 143 |
| **V3** `=` T for equal ranges | **no** | 0 (0) | **4 / 8 (S-1)** |
| **V4** Kleene `F or U = F` | yes | 2 (1) | 4 / 3 |
| **V5** select-U boolean takes then | yes | 2 (1) | 2 / 1 |
| **V6** step governing row strict `<` | yes | 1 (0) | 0 on pass 1; 28 on the corners |
| **V7** interpolation first segment only | yes | 1 (0) | 0 / 1 |
| **V8** input enclosure not outward | yes | 1 (2) | 2,196 (input box not contained) |
| **V9** divisor zero end not refused | **no** (evaluator suite); runner suite fails only through a note code | 0 (1) | 0 on pass 1; **8 on the corners: T in 4 boxes where the point path blocks (S-2)** |
| **V14** table-range note not eager | yes | 1 (0) | 62 / 31 |
| **V10** runner: all-fail reported as pass | yes | 2 | 191 / 0 |
| **V11** runner: lower offset step inward | **no** | 0 | 0 / 0 (N-1) |
| **V12** runner: subnormal bound as point | yes | 1 | 18 / 70 |
| **V13** runner: limit compare on a point | yes | 1 | 40 / 0 |
| **PV1** Python `<` strictness lost | yes | 2 | parity 30 |
| **PV2** Python `F or U = F` | yes | 2 | parity 16 |

**Results:**
- **I73's 21 mutants:** all killed, with the same failing-test counts I73 recorded.
- **My 16:** 13 killed.
- **Survivors:**
  - **V3:** unsound, survives everything (S-1);
  - **V9:** unsound in one corner, survives the evaluator suite (S-2);
  - **V11:** equivalent at the point level (N-1).

## 5. Item 5: fence and contracts

**Files** (`git diff --name-status c1bfc460fc 8f956d399a`):
- **added:** `P/core/analysis_runs/rule_interval.py`, `P/core/rules/rule_check_runner/tests/{interval_bounds_run,rule_interval_cases}.rs`, `P/fixtures/rule_interval/rule_interval_cases.json`, `P/tests/test_rule_interval.py`;
- **modified:** `P/core/rules/{expression_evaluator,rule_check_runner}/src/lib.rs` and `P/schemas/rule_check_run_result.schema.json`.

The first seven are exactly I73's fence (I73's brief; D2 §4.11.6; PLAN §3, with the new `fixtures/rule_interval/` folder kept away from the conformance-corpus walkers). The eighth is ROOT's separate commit.

**The schema edit** (`8f956d399a`):
- **Description text only.** Parsed as JSON with every `description` removed, base and candidate are equal. Exactly one description changed, `/$defs/RunFinding/properties/severity`.
- **Its new sentence matches what the runner emits:** "info" for `RULE_INTERVAL_ALL_PASS` and `RULE_INTERVAL_ALL_FAIL`, and "warning" for `RULE_RESULT_INDETERMINATE` and `RULE_INTERVAL_DIVIDE_BY_ZERO_RANGE`. My checker asserted these severities on every runner case: 150 + 191 + 235 outcome findings and 53 divide findings in pass 1, and more in seeds 1–4.
- The retained first clause was already incomplete on base (N-3).

**No dependency or lock moved:**
- **Manifests and locks:** no `Cargo.toml` or `Cargo.lock` is in the diff. The rules crates' own locks and src-tauri's `Cargo.toml` and `Cargo.lock` have identical sha256 at both commits; the src-tauri lock is `4de71f1b…`.
- **Python:** `rule_interval.py` imports only `math`, `struct` and `typing`; the test imports only the standard library and pytest.
- **Callers:** no code outside the rules crates names `run_rule_checks_with_bounds`, `SolverResultBound` or `evaluate_interval` (only ROOT's schema description text names the first). src-tauri imports the runner by module alias, with no glob import, so the new public names cannot collide. The bounded entry is dormant until S-I2.

**Codes and outcomes against D2's wording:**

| D2 | Candidate | Match |
|---|---|---|
| §4.11.5: T, ≥ 1 interval input → `USER_RULE_CHECKED`, `RULE_INTERVAL_ALL_PASS` (info), in `diagnostic_codes` and a finding | as stated | ✓ |
| F → `USER_RULE_FAILED`, `RULE_INTERVAL_ALL_FAIL` (info) | as stated | ✓ |
| U → `RULE_INPUTS_INCOMPLETE`, `RULE_RESULT_INDETERMINATE` (warning) | as stated | ✓ |
| §4.11.3: divisor range containing 0 → U with finding `RULE_INTERVAL_DIVIDE_BY_ZERO_RANGE` | one finding per check, warning, subject `divide` | ✓ |
| Finding message `enclosure=[0x<lo bits>,0x<hi bits>] unit=<u>` | that form, with extensions | ✓ (N-6) |
| `BoundInput.note` reads `interval ±<b> from receipt` | `interval ±{b:e} from receipt` | ✓ (N-6) |
| No schema changes; existing fields only | status, `diagnostic_codes`, `evaluator_findings`; `computed_value` omitted for an interval quantity check | ✓ |
| §4.11.2: b = 0 binds the exact point with no widening; a subnormal b binds an interval | probe h; runner b = 0 is byte-identical to no bound | ✓ |
| No wire codes beyond D2's (ruling 4) | the invalid-bound finding reuses `RULE_EVALUATOR_ERROR` | ✓ |

## Host, commands and limits

**Copies and targets:**
- **Copies:** `git archive 8f956d399a` in `WT/rv99/` and `c1bfc460fc` in `WT/rv99/base/`. Both cover `P/` only, without `P/execution/` (1.9 GB of records). Neither crate nor test reads it.
- **Targets:** `WT/targets/rv99/{cand,base}`. Logs and scratch are in `WT/scratch/rv99_s_i1_01/`.
- **My scratch files** in the copies' `tests/` folders (the harness, the differential and the probes) were removed before the final suites. Each copy then equals its commit file for file (2,955 and 2,950 files).

**Cargo:**
- every job went through `WT/tools/t3_cargo.sh` (the memory guard was running), with `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, `RUSTUP_TOOLCHAIN=1.97.1` (rustc `8bab26f4f68e`), `CARGO_INCREMENTAL=0` and `TMPDIR` in my scratch;
- 110 cargo jobs in all;
- **commands:** `cargo test` for each rules crate on both copies; `cargo test --release --test rv99_harness|rv99_diff|rv99_probes` with `RV99_*` input paths; `cargo test --lib` and full runner suites for mutants.

**Python:**
- `VENV` python with `PYTHONDONTWRITEBYTECODE=1` and `-p no:cacheprovider`;
- `PATH=/usr/bin:/bin`, so no test could reach cargo;
- `OPENPIPESTRESS_CHECKED_JSON_BIN` and `OPENPIPESTRESS_UNITS_BIN` set to a missing path, since the tests I ran do not need them (the 24 failures above are this stub).

**Rerun:** substitute `WT` and `VENV` in `evidence/tools/*`, run `gen_rv99.py` (deterministic: seed 990099, or `RV99_SEED`), then the harness, then `rv99_check.py`. The input and output hashes are in `evidence/soundness/inputs_outputs_sha256.txt`.

**Not done:**
- **No Git writes** and no installs.
- **No DEC-025, native, solver-at-scale or TS run.**
- **No src-tauri build.** It is outside the diff; I relied on the unchanged lock and API, I73's 116/116, and ROOT's planned 40-manifest suite.
- **Not the full Python sweep;** I73's sweep stands.

**Limits:**
- Sampling and bisection are not a proof over every binary64 value of a box. They are backed by the code review in §1.1 and the lemma.
- My generated runner formulas use stress and temperature catalog conversions; other dimensions go through the same code.
- My oracle cannot see conversion mutants that stay inside the remaining outward margin (R3, V11).
- Soundness is relative to b's operational basis (D2 §4.11.2).

**Cleanup:**
- I deleted `WT/rv99/` and `WT/targets/rv99/` at the end.
- `WT/scratch/rv99_s_i1_01/` (about 1 GB) stays for ROOT's post-merge cleanup. It holds the regenerable case and output files and the logs.

## Evidence (`evidence/`, every file in SHA256SUMS)

- `tools/`: my generator, oracle, harness, checker, dense pass, differential (prep and template), mutant runner, probes, corners, I73-case converter, Python invalid-input probe, cargo wrapper.
- `soundness/`: checker outputs for pass 1, dense, seeds 1–4, I73's cases and the corners; the straddle matrix; straddle and panic totals; input and output hashes.
- `differential/`: dump hashes, line counts, summary.
- `mutants/`: `results.json` (all 37), the run lines, and the V3, V6, V8 and V9 oracle outputs.
- `probes/`: `probes.jsonl` (probes a–j) and the Python invalid-input output.
- `suites/`: per-crate suite summaries for base and candidate, and the related Python outcomes.
