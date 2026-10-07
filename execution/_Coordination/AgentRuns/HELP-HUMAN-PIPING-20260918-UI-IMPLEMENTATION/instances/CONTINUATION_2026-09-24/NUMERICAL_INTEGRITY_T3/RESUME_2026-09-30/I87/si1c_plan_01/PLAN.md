# I87 — T3-SI1c plan: point-path results over non-finite intermediates (documents only)

TASK (Type 2) I87, a planner dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is the return path, and I delegated nothing. Brief: `R/BRIEFS/SI1C_PLAN.md` (sha256 `878b75798a16ce28…`, verified before I relied on it). This is the single return.

Placeholders are as in the dispatch: `WT`, `NUM`, `P`, `T`, `R`, `RR`, `VENV`. No machine paths are recorded. Code references are to main `025c1cf326`, whose maintained tree NUM carries (no difference under `P` outside `execution/`):
- `EE` is `P/core/rules/expression_evaluator/src/lib.rs` (sha256 `07d9a7a525d6d151…`);
- `RCR` is `P/core/rules/rule_check_runner/src/lib.rs` (`3f58533fc46e8f14…`);
- `TAURI` is `P/apps/desktop/src-tauri/src/lib.rs`;
- `UNITS` is `P/core/units/src/lib.rs`.

## 0. In brief

**The exact set (§2).**
- Only a value **produced inside a formula** can be non-finite on the point path. Bindings and literals are checked (`EE:448`, `EE:513`). The runner turns a non-finite caller value or limit into a block (N-4).
- From finite operands, four operations produce one:
  - add and subtract;
  - multiply;
  - the two divide arms other than the ratio arm;
  - interpolation.
- Every consumer then decides:
  - the six comparisons, by IEEE rules (with NaN, every ordered comparison and `=` read false, and `≠` reads true);
  - `not`, `and` and `or`;
  - `select`;
  - `min` and `max`, which drop a NaN operand;
  - division of a finite value by an infinity, which gives 0.
- Interval mode reads every one of these U, with the note `non_finite_enclosure`, under eager U.

**One correction to the routed premise (§2.5).** "A quantity formula is safe" holds only when the non-finite value reaches the top of the formula.
- Four forms turn a non-finite intermediate into a finite value, which then passes or fails at the synthesized compare:
  - `max(NaN, s)`;
  - `select(c, inf, s)`;
  - `s / inf`;
  - an interpolation whose rows span more than `f64::MAX`.
- The quantity case that does block emits `computed_value.value: null`, which the schema forbids. That is 321 lines of RV104's runner corpus. N-4's `bound_inputs[].value: null` is 18,927 more.

**Options (§3).**
- **The brief's three:** (A) block at a comparison with a non-finite operand; (B) block only a NaN operand at a comparison; (C) keep the behaviour and state it.
- **One I add:** (D) block at the operation that produces the non-finite value, so that no intermediate is ever non-finite.
- **What they share.** Every option leaves every input that never reaches a non-finite intermediate byte-identical.
- **How far each reaches.**
  - Only D closes the set.
  - A and B leave the absorbing forms decided.
  - B also leaves every ±inf comparison decided, including two infinities of the same sign, whose order is arbitrary.

**Recommendation: D**, with N-4 (variant N4-1, which changes no status) and N-5 in the same slice.

**Public meaning (§4): yes, in my reading, for A, B and D.**
- On every product surface that shows a rule result, a published `USER_RULE_CHECKED` or `USER_RULE_FAILED` becomes `RULE_INPUTS_INCOMPLETE`. Those surfaces are:
  - the desktop's Rule-check panel;
  - the status bar;
  - the analysis-run record and the saved project;
  - the result export.
- The owner's package is §4.4.
- **A second question goes with it.** DEC-022's versioning rule, as the conformance corpus README states it, requires a `grammar_version` bump for "any semantic change". I recommend reading SI1c as a repair within grammar 1.0.0: no bump, and no corpus change.

**Practical reach.** No realistic engineering input reaches this set: an intermediate must pass 1.8e308. What is at stake is the guarantee, and agreement with interval mode before S-I2 lets the mode depend on a row's class (§7).

**The slice (§6).**
- **Files:** two Rust files, the evaluator README, and optionally the Python oracle.
- **Tests:** about 12 new evaluator tests, about 4 new runner tests, and 6 + 1 revised tests.
- **Method:** a base-against-candidate differential with an instrumented base as the oracle for the intended lines.
- **Gates:** the full product gate set, without Pass B.
- **Estimate for D:** 10–14 h implementation, 5–8 h review, and about 3–5 h of gate wall time.

## 1. Basis read (origins and sha256 prefixes)

| Record | sha256 |
|---|---|
| `NUM/AGENTS.md`; `NUM/agents/AGENT_TASK.md` | `f96feb19d297c74e…`; `1a13a5b00b3ce01f…` |
| `R/BRIEFS/SI1C_PLAN.md` | `878b75798a16ce28…` |
| `EE`; `RCR`; `P/core/rules/rule_check_runner/tests/point_path_non_finite_run.rs` | `07d9a7a525d6d151…`; `3f58533fc46e8f14…`; `de59c4177135ac10…` |
| `P/core/rules/expression_evaluator/README.md`; `P/fixtures/rule_expressions/conformance_corpus/README.md` | `9a582419937b54b5…`; `88ce1ba3bc811533…` |
| `UNITS` (`convert_for_dimension`); `TAURI` (`run_rule_checks`, binding parsers) | `521717eb4cac8114…`; `8f9281c3184a5c40…` |
| `P/schemas/rule_check_run_result.schema.json`; `P/tests/test_rule_interval.py` | `9b75a1290ab07d2c…`; `46ee5156d657039d…` |
| Desktop readers: `RuleCheckRunPanel.tsx`, `ruleCheckService.ts`, `previewService.ts`, `shellLayout.ts`, `resultExportAdapter.ts`, `reportPackageRequest.ts` | `28d3ca8faa2ac0d5…`, `e92d5b55036b005d…`, `8de1b6a8940da360…`, `9dce386b4b5f8c48…`, `b2ba958adc6465b4…`, `3b49e6b0bf5bda8e…` |
| `P/core/reporting/report_package/src/wire.rs` | `29e7d8a46f0fc8a4…` |
| `P/execution/_Coordination/_DECISIONS/D-02_rule_pack_expression_grammar.md` (DEC-022); `P/docs/TYPES.md` §4; `P/docs/SPEC.md` §7 | `dd85daff655f73bd…`; `28bc8b6a827b91a2…`; `461570f14ada321b…` |
| The work graph, `WORK_GRAPH.md`, section "T3 current route" (owner-held list) | `f6dda4e4891202c5…` |
| `RR`: "RV103 passes #1103; …" (item 3), "RV104 passes SI1b; …", "#1106 merged: …", "T3's gate set and Git rules, consolidated …", "D-15 option C (ROOT, crossing message)", and the F-1 ruling ("Not owner-held …") | file `a955e376dbb28bb2…` |
| D2, `T/DESIGN_STANDING/DESIGN.md` §4.11 and §4.5.2 (R5-5) | `993f5f3ab4bd768b…` |
| `T/IMPLEMENTATION/SI1B/CHANGE_RECORD.md` §3; `T/IMPLEMENTATION/S_I1/CHANGE_RECORD.md`; `T/IMPLEMENTATION/SI1B_MERGE/RECORD.md` | `a5cdad8adac704e5…`; `983772c59eb45df2…`; `8b08b862a44c91c1…` |
| `R/I79/si1b_01/RETURN.md` §4; `REPAIR_01.md`; the probe and its harness (`_run_records/probe/probe_carried_cand.txt`, `harness/i79_carried_non_finite_probe.rs`) | `44d82fc89d45cfb5…`; `a44a274dbb7b995e…`; `892420991137c831…`, `c4a269e221faaeda…` |
| `R/REVIEW_RV104/si1b_01/REVIEW.md` (N-4, N-5, probes 12–14 and 17); `evidence/probe/probe_cand.txt`; `evidence/harness/rv104_ee_diff.rs` | `689c5e1cd6605cbe…`; `c07aa2fe58f89ee6…`; `080d4237ec986254…` |
| `R/I73/s_i1_01/CHECKPOINT_1.md` §5, §6.2, §6.3; `RETURN.md` §2.2, §5 | `b992efcfa216f8da…`; `ab36c3d31f7a4b7a…` |
| `R/I61/u8_plan_01/PLAN.md` §3 and §5, decision 14 | `f274a6149051292c…` |

**RV104's kept dumps.** I read RV104's two candidate dumps (`ee_cand.txt`, `run_cand.txt`) from `WT/scratch/rv104_si1b_01/dumps/`. Their uncompressed sha256s are `be1266be2f1799be…` and `31dc84473bc88028…`, equal to RV104's `evidence/differential/dump_sha256.txt`. RV104's ADDENDUM_01 shows them byte-identical across the repair round, so they are main's behaviour. My two read-only census scripts and their outputs are in `_run_records/` (§2.7).

**Host.**
- No cargo, vitest, native or solver jobs, no installs, and no Git writes.
- Git was read with `GIT_OPTIONAL_LOCKS=0`. NUM was clean at `981206aca9`.
- Python: only VENV's interpreter, read-only, over RV104's dumps and the committed schema.
- Scratch was in `WT/scratch/i87_si1c_plan/`, with `TMPDIR` set there. Nothing went to the system temp directory.
- There were no waits.
- The write guard did not refuse NUM, so this record is at `R/I87/si1c_plan_01/`.

## 2. The exact set

### 2.1 Where a non-finite value can enter

| Source | Point path today | Interval mode |
|---|---|---|
| Caller binding given to `evaluate` | blocks: `NonFiniteInput`, "variable binding quantity must be finite" (`EE:448-456`) | the same block (shared `build_binding_map`) |
| Literal | blocks: `NonFiniteInput` / `literal` (`EE:513-520`) | the same block |
| Caller value in the runner (NaN, ±inf, or ±inf after unit normalization) | `Quantity::new` fails and the binding becomes `missing` (`RCR:831-845`). The evaluator then reports `MissingRequiredValue` twice, while `bound_inputs` says `supplied: true, value: null`. This is **N-4** | the same; the input never becomes an overlay |
| Slot limit (NaN, ±inf, or ±inf after normalization) | `RULE_EVALUATOR_ERROR`, "value-slot limit has missing or unknown unit/dimension metadata" (`RCR:1019-1049`). **N-4** | the same text (`RCR:1487-1504`) |
| Overlay end (interval only) | — | blocks: `NonFiniteInput` (`EE:1949-1957`); `enclosure: None` is U |
| **Produced inside the formula** | **carried, then decided (§2.2–§2.3)** | **U** |

**Unit normalization can create an infinity from a finite value.** `UNITS:917-950` (`convert_for_dimension`) checks only that its input is finite (`:923`). The output `(x·f + o − o′)/f′` is unchecked, so 1e300 GPa becomes +inf Pa. That is the one desktop route to N-4. The desktop sends values as JSON, which carries no NaN or infinity: `TAURI:3337-3375` reads them with `as_f64`, and JavaScript serializes non-finite numbers as `null`.

### 2.2 The producers (finite operands in, non-finite value out)

| Operation | Code | What it gives |
|---|---|---|
| `add`, `subtract` | `EE:1065-1086` | ±inf on overflow. NaN only from carried infinities (`inf − inf`) |
| `multiply`: the dimensionless-left, dimensionless-right and derived-product arms | `EE:1227-1260` | ±inf on overflow. NaN from `0·inf` (carried) |
| `divide`: the dimensionless-divisor arm (including ratio/ratio) and the derived-quotient arm | `EE:1277-1283`, `EE:1302-1309` | ±inf on overflow (RV104 probes 12–14). NaN from `inf/inf` (carried). **A finite value over ±inf gives ±0**, which absorbs the infinity |
| `divide`: the same-dimension ratio arm | `EE:1284-1301` | **blocks** with `NonFiniteInput` / `divide` (SI1b). A finite value over a carried infinity is still the ratio 0 (kept by SI1b) |
| `divide` by zero | `EE:1267-1274` | blocks `DivisionByZero` before any arm (a NaN divisor is not `== 0`) |
| `interpolate` | `EE:978-1000`; the formula is at `:996-998` | ±inf or NaN when the rise, offset or run overflows. **It can also absorb:** with rows spanning more than `f64::MAX`, the run is +inf, the fraction is 0, and the result is a finite but wrong value. With rows (−MAX, 0), (MAX, 10), x = 0 gives 0, not 5 |
| `negate`, `abs` | `EE:658-671` | pass a non-finite value through |
| `min`, `max`, `select`, step and exact lookups | — | produce nothing new: their row results are validated finite |

### 2.3 The consumers: what the point path decides, and what interval mode reads

Interval mode reads every row below U.
- At the producer, the non-finite result has no enclosure. `IntervalState::finite` records `non_finite_enclosure` (`EE:1992-1997`, via `outward`, `EE:1622-1632`).
- Any note makes the whole result U (`EE:1900-1904`, `EE:2023-2036`: eager U, D2 §4.11.3).
- A comparison with no enclosure is itself Indeterminate (`EE:1719-1721`).
- In the runner, U is `RULE_INPUTS_INCOMPLETE`, with `RULE_RESULT_INDETERMINATE` (warning) and `causes=non_finite_enclosure` (`RCR:1579-1651`). Interval mode runs only for a check with an interval input.

| # | Form | Code | Point path today | Runner outcome today (point) |
|---|---|---|---|---|
| 1 | The six comparisons with a non-finite operand | `EE:544-552`, `EE:1336-1371` | **Decided** by IEEE rules (§2.4) | Boolean formula: `USER_RULE_CHECKED` or `USER_RULE_FAILED` (`RCR:909-928`) |
| 2 | `not` over 1 | `EE:531-534`, `EE:680-682` | **Decided**: `not(NaN > 100)` is true | `USER_RULE_CHECKED` |
| 3 | `and` / `or` over 1 | `EE:553-562`, `EE:694-713` | **Decided** | decided |
| 4 | Boolean `select` whose condition is 1 | `EE:563-574`, `EE:715-736` | **Decided**: the condition picks a branch | decided |
| 5 | Quantity `select` | `EE:737-755` | **The untaken branch is dropped**, finite or not; a taken non-finite branch is carried | finite: decided at the synthesized compare. Non-finite: row 9 |
| 6 | `min` / `max` | `EE:767-822` (`:816-819`) | `f64::min`/`max` **return the other operand when one is NaN**. `min(+inf, s) = s` and `max(−inf, s) = s` | finite: **decided** |
| 7 | A finite value over ±inf (all three divide arms) | as §2.2 | **±0** | **decided** |
| 8 | Interpolation with an internal overflow | `EE:996-998` | finite (possibly wrong) or non-finite | finite: **decided**. Non-finite: row 9 |
| 9 | Quantity formula with a non-finite final value | `RCR:929-1092` | carried to the runner | **blocked** at the synthesized compare's literal: `NonFiniteInput` / `literal` (`RCR:1052-1079`), status `RULE_INPUTS_INCOMPLETE`. `computed_value` carries the value (`RCR:968`), **serialized as JSON `null`** |
| 10 | Table argument | `EE:946-986` | NaN: `NonFiniteInput` (interpolate, step; SI1b) or `TableKeyNotFound` (exact). ±inf: `TableOutOfRange` | blocked |
| 11 | Same-dimension ratio with a non-finite result | `EE:1284-1301` | `NonFiniteInput` / `divide` (SI1b) | blocked |

**Reproducers:**
- **I79's** (`R/I79/si1b_01/_run_records/probe/`; x = 1e300 stress, `inf = 1e300·x`) give rows 1 and 2: `not((inf−inf) > 100)`, `(inf−inf) ≠ 100` and `inf ≥ 100` are each `true` with no finding, and interval mode reads `Indeterminate` with `non_finite_enclosure`.
- **RV104's probes 12–14** give the three carried divide overflows (Moment/Length, Stress/ratio, ratio/ratio).
- **Rows 5–8** are from code reading. The slice's probe pins each one (§6.3).

### 2.4 The comparison truth table (point path today)

| Operands | `<` | `≤` | `>` | `≥` | `=` | `≠` | Does the result reflect the real value behind it? |
|---|---|---|---|---|---|---|---|
| NaN against anything (finite, ±inf, NaN) | F | F | F | F | F | **T** | No. NaN has no value; `≠` and `not(<…)` can pass |
| +inf against finite c | F | F | T | T | F | T | Yes, by sign: the real value exceeds `f64::MAX`, as far as any point-path value is real |
| −inf against finite c | T | T | F | F | F | T | Yes, by sign |
| finite c against +inf / −inf | T / F | T / F | F / T | F / T | F | T | Yes, by sign |
| +inf against +inf (or −inf against −inf) | F | **T** | F | **T** | **T** | F | No. The two real values' order is unknown |
| +inf against −inf | F | F | T | T | F | T | Yes |

Interval mode reads every row U.

### 2.5 Two corrections to the premise as routed

1. **"A quantity formula is safe" is true only for row 9.** Rows 5–8 produce a finite final value from a non-finite intermediate, and it passes or fails at the synthesized compare. A boolean formula built on rows 5–8 (for example `max(inf − inf, s) ≤ c`) passes, too.
2. **Row 9 blocks, but with an output the schema forbids.**
   - `ComputedQuantity.value` is `"type": "number"`, and serde writes a non-finite `f64` as `null`.
   - N-4 does the same to `BoundInput.value`.
   - The desktop's `QuantityReadout` renders a `null` value as blank in "entered" units, or as the notice "quantity is not a finite numeric value" in SI/US units.

### 2.6 Reachability in the product

- **The desktop runs the point path only.**
  - `TAURI:2911` calls `run_rule_checks`; nothing calls `run_rule_checks_with_bounds`.
  - Interval mode reaches users only with S-I2 (S-I1 CHANGE_RECORD §3).
- **Reaching the set needs an intermediate beyond 1.8e308.** Examples:
  - two inputs of 1.4e154 or more multiplied;
  - a divisor below 1e-300;
  - a user table with rows near ±1e308;
  - a caller value of about 1e300 converted to a smaller unit.
- **No realistic engineering quantity does this.** A mistyped value (1e300) or a runaway result can.
- **Nothing else consumes the evaluator.** Its only dependents are `rule_check_runner` and `rule_pack_document` (whose tests evaluate the finite invented demo). The headless runner takes a caller's aggregate string; it does not run rules.

### 2.7 An empirical cross-check (RV104's dumps; `_run_records/`)

**The evaluator** (`census_ee_cand.txt`). Of RV104's 125,133 point evaluations on main, interval mode with every input bound as its exact point (variant i1, b = 0) reads U with `non_finite_enclosure` where the point path is decided:

| Family | Decided booleans (true / false) | Decided finite quantities |
|---|---|---|
| All | 1,257 (722 / 535) | 3,482 |
| Generated | 921 | 413 |
| Quotient | 168 | 2,702 |
| Table | 168 | 367 |

These are upper bounds, not the set. Interval mode also steps one ulp outward, so a result at ±`MAX` reads U with no point overflow; RV104's grids include `MAX`. The slice's instrumented base measures the set itself (§6.4).

**The runner** (`census_run_schema.txt`).
- 14,851 of RV104's 45,604 point-run JSON lines fail `rule_check_run_result.schema.json`:
  - 18,927 `bound_inputs[].value: null` (N-4);
  - 321 `computed_value.value: null` (row 9).
- Every check carrying a `null` is `RULE_INPUTS_INCOMPLETE`.

## 3. The options

Every option keeps every input that never reaches a non-finite intermediate **byte-identical**: the evaluator's and the runner's outputs, in every mode. Each new branch fires only on a non-finite value, after every check that precedes it today.

### (A) Block at a comparison with a non-finite operand

- **Rule.** In `eval_compare`, after the dimension and unit checks (`EE:1350-1361`): if either operand is NaN or ±inf, push `NonFiniteInput` and return `None`. The subject is `comparison` (its existing subject); the message is "comparison operands must be finite".
- **Blocks:** rows 1–4, and every comparison anywhere in the formula, including an untaken `select` branch, because evaluation is eager.
- **Unchanged:** rows 5–8 stay decided, and row 9 is unchanged. The synthesized compare's literal check fires first, so the new check never sees a non-finite formula value there.
- **Alignment with U:** partial. Interval mode reads U on rows 5–8 too.
- **Interval path:** unaffected. It calls `eval_compare` only with shadow values of 1.0 (`EE:2015-2020`).
- **Existing tests:** none change.
- **Runner extra.** The schema-invalid `computed_value: null` (row 9) needs its own guard: omit a non-finite `computed_value`.

### (B) Block only a NaN operand at a comparison

- **Rule.** As A, with `is_nan` in place of `!is_finite`. The message is "comparison operand is NaN".
- **Blocks:** the NaN rows of 1–4.
- **Stays decided:** every ±inf comparison, including the arbitrary same-sign pair (§2.4), and rows 5–8.
- **It also misses part of its own target.** `min`/`max` drop a NaN silently (row 6), so a NaN can still decide a check.
- **Alignment:** the smallest of A, B and D.
- **Existing tests:** none change. It needs the same runner guard as A.
- **B′, for completeness:** NaN plus same-sign infinities. It closes §2.4's arbitrary rows, keeps the sign-correct ones decided, and still leaves rows 5–8.

### (C) Keep the behaviour and state it

- **Rule:** no evaluator change.
- **Text:** a declared limit in the evaluator README and the `Expression::Compare` and `FindingCode::NonFiniteInput` doc comments. It would read: "Intermediate results follow IEEE 754 binary64: an overflow is ±inf, an undefined operation is NaN, and comparisons, `min`/`max`, `select` and division by an infinity decide over them. Interval mode reads such a formula indeterminate."
- **Optionally,** a sentence in `P/docs/SPEC.md` §7.
- **Blocks:** nothing.
- **Alignment:** none. The point and interval readings of one formula stay opposite.
- **Should still carry:** N-4 and the `computed_value: null` guard.

### (D) Block at the producer: every intermediate must be finite (added)

**Rule.** At each producer (§2.2), after its existing structural checks:

| Site | Subject | Message (proposed) |
|---|---|---|
| `add_or_subtract` | `add_subtract` | "sum or difference must be finite (it overflowed)" |
| `multiply`, all three arms | `multiply` | "product must be finite (it overflowed)" |
| `divide`, dimensionless-divisor and derived arms | `divide` | "quotient must be finite (it overflowed)" |
| `divide`, ratio arm | `divide` | **unchanged**: "same-dimension quotient (ratio) must be finite" |
| Interpolation | the table id | "interpolated table value must be finite (a step of the interpolation overflowed)" |

- **What it means.** Each site pushes `NonFiniteInput` and returns `None` when its result is not finite. For interpolation, the check covers each of the formula's six floating steps (rise, offset, run, fraction, product, sum), as `interpolate_segment` does for interval mode (`EE:2557-2570`).
- **Every subject is an existing one.**
- **No consumer ever sees a non-finite value,** so rows 1–9 all block, at the operation that caused them.

**What changes for inputs that already block** (all of them reach a non-finite intermediate):
- **Row 9:** the finding moves from `literal` to the producer. `computed_value` and `limit_value` are omitted, and the relation is `none`. That also ends the schema-invalid `null`.
- **Rows 10 and 11, when the argument or operand was carried:** the producer's finding replaces `TableOutOfRange`, `TableKeyNotFound` or the ratio finding.
- **The `?` short-circuit stops at the producer,** so a later operand's findings no longer appear. That is N-5's documented behaviour.

**Statuses of already-blocked inputs** do not change.

**Unreachable after D:**
- SI1b's NaN-argument checks (`EE:962`, `EE:979`) and its carried-operand ratio block;
- `TableOutOfRange` for an infinite argument;
- row 9's literal block for a formula value.

I recommend keeping them, as I79 kept the `.expect`s. Their SI1b mutants become equivalent (§6.5).

**Alignment.** D is the point-path counterpart of eager U.
- Interval mode records a note where the point path would block (a zero divisor, `divide_by_zero_range`) or carries a non-finite value (`non_finite_enclosure`). D makes the second behave like the first.
- After D, the two modes differ only on straddles (by design) and at the ±`MAX` boundary, where interval mode's outward ulp reads U and the point path still decides a finite `MAX`. In both cases interval mode is the conservative one.

**Interval path: unaffected.** `eval_binary` is called there only with shadow values of 1.0, so the sum is 2, the difference 0, and the product and quotient 1, all finite. Interval mode never calls `eval_table_expression`.

**Existing tests that change** (§6.3): six evaluator tests and one runner test. They pin SI1b's "other arms still carry non-finite values" scope, which D is meant to reverse.

### Comparison

| | A | B | C | D |
|---|---|---|---|---|
| Rows 1–4 (comparisons, `not`, `and`/`or`, boolean `select`) | block | NaN only | decided | block |
| Rows 5–8 (absorbing forms) | decided | decided | decided | block |
| Same-sign infinities (arbitrary order) | block | decided | decided | block |
| Agrees with interval mode's U | partly | least | no | yes (except at the ±`MAX` boundary) |
| Already-blocked inputs | unchanged | unchanged | unchanged | finding moves to the producer |
| Schema-invalid `computed_value: null` | needs a guard | needs a guard | needs a guard | gone |
| Existing tests changed | 0 | 0 | 0 | 6 evaluator + 1 runner |
| Size (non-test lines) | about 10 | about 10 | docs only | about 50–70 |

**My recommendation is D.**
1. **It is the only option after which "no rule result rests on a non-finite value" is true.** The product surfaces would then mean what they say, and the owner is asked once, not again when rows 5–8 surface.
2. **It extends an invariant the evaluator already holds** for bindings, literals, ratios and NaN table arguments to every intermediate, with the existing code and subjects.
3. **It mirrors eager U exactly,** so S-I2 has one sentence to state (§7).
4. **It costs nothing a user could rely on.** It removes no realistic computation: every value it blocks is beyond 1.8e308.

**Second choice: A**, with the runner guard, if the owner wants the smallest change to the decided booleans that I79 reported. Its residual (rows 5–8) should then be written into the declared limit, as in C.

**B: not recommended.** It keeps arbitrary infinity comparisons and misses the NaN that `min`/`max` drop. **C** is a legitimate owner choice given the reach (§2.6), but it leaves the two modes contradicting each other once S-I2 ships.

## 4. Public meaning, and the owner's package

### 4.1 What a reader sees change

**A, B and D turn some point-path passes, and some fails, into blocks** (worked examples in §4.4):
- `not((inf−inf) > 100)` goes from `USER_RULE_CHECKED` to `RULE_INPUTS_INCOMPLETE`;
- `(inf−inf) > 100` goes from `USER_RULE_FAILED` to `RULE_INPUTS_INCOMPLETE`;
- under D, `max(inf−inf, s) ≤ c` also goes from `USER_RULE_CHECKED` to `RULE_INPUTS_INCOMPLETE`.

**Run aggregates follow worst-of** (failed, then incomplete, then checked). A run whose only failing check becomes blocked reads incomplete instead of failed. Both are non-passes.

**C changes no output.**

**N-4 (variant N4-1) changes no status,** only the finding, the note and the JSON `null`.

### 4.2 My reading: A, B and D change public meaning, so the remedy is owner-held

1. **The outcomes are public now.**
   - The Rule-check panel, the status bar, the analysis-run record (persisted with the project) and the result export all show the aggregate (§4.3).
   - RR's F-1 ruling is not owner-held only because "successors are not public before B8". Rule-check results are public today.
2. **ROOT's own D-15 test** (RR "D-15 option C"): "An owner question would arise only if C changed how covered rows bind, or let a bound-straddling result pass." SI1c changes what checks with only point inputs read. That is the covered side of that line.
3. **DEC-022 is an owner ruling.**
   - Its versioning rule reads: "Minor versions are additive-only; any breaking change requires a new major version and a recorded human ruling" (D-02 §5 item 4).
   - The corpus README is stricter: "any semantic change requires a `grammar_version` bump".
   - Reading SI1c as outside that rule is itself a reading of an owner ruling.
4. **What argues the other way** (for ROOT to weigh):
   - no token, code or display form changes;
   - SPEC §7 already lists "non-finite inputs" among the evaluator's blocking findings, and the README lists "non-finite values";
   - interval mode, which the owner's order already includes, never passes on these inputs;
   - SI1b's ruling ("no new code means no new public vocabulary") covered a code, not an outcome.
5. **One more fact for the owner.** `docs/TYPES.md` §4 defines `RULE_INPUTS_INCOMPLETE` as "rule-pack required values are missing". Every evaluator block (`DivisionByZero`, table range) and S-I1's U already map to it, so A, B and D follow existing practice. They also widen that practice slightly.

**Recommendation to ROOT:** rule that A, B and D change public meaning, and put §4.4 to the owner. N-4 (N4-1) and N-5 change no status, so they are ROOT's.

### 4.3 Who reads rule-check results

| Reader | Code | What it shows | Changed by SI1c? |
|---|---|---|---|
| **Rule-check panel** | `RuleCheckRunPanel.tsx:1006-1079` | The aggregate and per-check status with `aggregateLabel` (`RULE_INPUTS_INCOMPLETE` is shown as "(blocked on missing inputs)"); relation, computed and limit readouts; inputs as `supplied`/`MISSING`; diagnostics; each finding as `[severity] code (subject): message` | **Yes** |
| **Status bar chip** | `shellLayout.ts:401`, `:445-460` | "Rule pack · User-rule checked / User-rule failed / Rule inputs incomplete" (registered labels, `statusLabels.ts`) | **Yes** |
| **App-held analysis-run record** | `workspaceSession.ts:930-980`; `previewService.ts:274-305` | `analysis_status` carries the aggregate; persisted with the project (`projectService.ts:310`); shown again as a historical run (`HistoricalRunContext.tsx:232`); read by Python `compatibility.py:608` | **Yes**, for new runs. Saved records keep what they recorded |
| **Result export** | `resultExportAdapter.ts:203` (`analysis_status: run.analysis_status`), `:245` | the exported document's `analysis_status` | **Yes** |
| Report package | `reportPackageRequest.ts:272-276`; `report_package/src/wire.rs:694-698` | refuses whenever any rule aggregate is present ("REPORT-PACKAGE-RULE-PACK-BINDING-UNAVAILABLE") | No: refused before and after |
| R3 journey line | `App.tsx:1571-1578` | only whether an aggregate exists | No |
| Missing-data panel | `MissingDataBlockingPanel.tsx:144` | the solve envelope's own `rule_check`, not the aggregate | No |
| Headless runner | `openpipestress-runner.rs:48`, `:787-791`; `headless/src/lib.rs:806-813` | maps a caller-supplied aggregate string; runs no rules | No code change; it carries whatever its caller computed |

### 4.4 The owner's decision package

**The question.** Rule checks on ordinary (point) inputs can pass or fail when a formula's arithmetic overflows past the largest number the computer holds (about 1.8 × 10^308), or produces "not a number". For example, `(a·10^300 − a·10^300) ≠ 100` reads "User-rule checked" when `a·10^300` has overflowed. The newer interval mode reads such checks as indeterminate, never as a pass. Should the ordinary path block them instead? It would show "Rule inputs incomplete", with a finding that names the overflowing operation.

**The options.**
- **D (recommended): block at the operation that overflows.** No rule result ever rests on an overflowed or undefined value, and the ordinary path agrees with interval mode.
- **A: block only where a comparison meets such a value.** Formulas that pass through `min`/`max`, `select`, division or a table can still pass on one.
- **B: block only "not a number" at comparisons.** Comparisons against an overflow stay decided, including two overflows of the same sign, whose order is arbitrary.
- **C: keep today's behaviour,** and state it as a declared limit.

**Why D.**
- It makes the guarantee true without exceptions, with the code and wording the product already uses for non-finite inputs.
- It affects only values beyond 10^308, which no engineering quantity reaches.
- It keeps the two evaluation modes from contradicting each other once verified bounds reach rule checks (S-I2).

**The sub-question (DEC-022).** The rule grammar's versioning rule requires a version bump and your ruling for a breaking semantic change.
- **Recommended reading:** a repair within grammar 1.0.0. Grammar 1.0.0 already lists non-finite values among its blocking findings. Its frozen conformance corpus cannot express these inputs and does not change. A bump to 2.0.0 would leave every saved rule pack, which declares 1.0.0, unsupported unless both versions were kept.
- **The alternative:** treat it as a breaking change, with a new major version and a corpus extension.

**What each reader would show.** Panel wording below uses the demo pack's diagnostic policy. "Blocked" means the per-check and aggregate label `RULE_INPUTS_INCOMPLETE (blocked on missing inputs)`, the status-bar chip "Rule pack · Rule inputs incomplete", and `RULE_INPUTS_INCOMPLETE` in the analysis-run record and the result export.

| Example | Today | D | A | B | C |
|---|---|---|---|---|---|
| E1. Boolean `not((x·1e300 − x·1e300) > 100)`, x = 1e300 (I79) | Panel `USER_RULE_CHECKED (pass)`; chip "User-rule checked"; record and export `USER_RULE_CHECKED`; no finding | Blocked. Finding `[blocking] NonFiniteInput (multiply): product must be finite (it overflowed)`; diagnostics `RULE_EVALUATOR_ERROR`; relation `none` | Blocked. `NonFiniteInput (comparison): comparison operands must be finite` | as A (NaN operand) | as today |
| E2. Boolean `x·1e300 ≥ 100` | as E1 (pass) | Blocked, as E1 | Blocked, as E1 | **pass, as today** | as today |
| E3. Boolean `(x·1e300 − x·1e300) > 100` | `USER_RULE_FAILED (fail)` everywhere | Blocked (fail → incomplete) | Blocked | Blocked | as today |
| E4. Quantity `max(x·1e300 − x·1e300, s)` against a limit above s | `USER_RULE_CHECKED (pass)`, computed = s | Blocked, as E1 | **pass, as today** | **pass, as today** | as today |
| E5. Quantity `x·1e300` against a limit | Blocked. `NonFiniteInput (literal)`; computed shows a blank value; the JSON has `null` | Blocked. `NonFiniteInput (multiply)`; no computed or limit readout; relation `none` | Blocked as today; computed omitted (guard) | as A | as A (guard) |
| E6. Report package after any rule run | unavailable (rule-pack binding) | same | same | same | same |

## 5. N-4 and N-5

### 5.1 N-4: name the cause for non-finite caller values and limits

**Today** (identical on main; RV104's dumps, e.g. label `k_demo_0_24_0`):
- **An input.** `bound_inputs` has `supplied: true, value: null`. The evaluator reports `MissingRequiredValue` twice ("required variable has no supplied value", "variable reference has no supplied value").
- **A limit.** `RULE_EVALUATOR_ERROR`, "value-slot limit has missing or unknown unit/dimension metadata".
- **A non-finite raw value in a different entered unit.** It reaches `convert_for_dimension` first and reads `UnitMismatch` ("… unit conversion failed: quantity must be finite, got NaN"). This route is Rust-API only.
- **The status** is `RULE_INPUTS_INCOMPLETE`, with the diagnostic `evaluator_error` (or `UnitMismatch`).

**Remedy, variant N4-1 (recommended; no status change):**
1. **Test the raw value before normalization, and the normalized value after.** A non-finite resolved value is recorded in `bound_inputs` with:
   - `supplied: true` (it was supplied);
   - `value` omitted (no `null`);
   - `note`: "non-finite value (NaN or ±inf, after unit normalization): not bound".
2. **For a formula input,** the runner pushes one blocking evaluator finding per such input: `NonFiniteInput`, subject the input id, message "supplied value must be finite (NaN or ±inf after unit normalization)". It then returns `blocked_after_completeness` without calling the evaluator.
   - The status, diagnostic and relation are today's.
   - The two `MissingRequiredValue` findings, and any other evaluator finding of that check, are replaced by the one finding. The latter is N-5's behaviour, stopping at the first block.
   - An input the formula does not reference blocks nothing, as today.
3. **For a limit** (`RCR:1019-1049`, and identically at `RCR:1487-1504`), the non-finite case is split from the missing-metadata case. It gets `NonFiniteInput`, subject the slot id, message "value-slot limit must be finite (NaN or ±inf after unit normalization)". The alternative keeps `RULE_EVALUATOR_ERROR` with the new message (a decision for ROOT, §8).
4. **The same code runs in point and interval mode.** Inputs and limits are resolved before the mode is chosen.

**Variant N4-2 (not recommended):** treat the value as unsupplied at completeness, as the invalid-bound refusal does (`RCR:621-639`).
- It is symmetric with that precedent, but the diagnostic becomes `missing_input`.
- A required input the formula does not reference would then block a check that passes today: a status change, and so part of the owner's question.

**Does N-4 change outputs?** Yes: findings, the note, and the removal of the JSON `null`. It changes no status under N4-1.

**Does it ride with the remedy?** Yes. It touches the same runner file, the same differential and the same reviewer, and it uses the same code. If the owner picks C, or the answer is slow, N-4 and N-5 can ship on their own as a ROOT-decided slice (§6.1).

### 5.2 N-5: the doc comments on `Logical` and `Select` (comment-only)

**Today** (`EE:251-252`): "Evaluation is eager: both operands are always evaluated, so diagnostics in either operand always surface."
- In fact `eval_expression` returns at the first blocking operand (`?`, `EE:559-560`, `EE:570-572`).
- RV104's probe 17 shows it: `and(ratio > 1, missing > 1)` reports only the ratio block.
- `Select`'s comment (`EE:258-262`), the inline comments at `EE:558` and `EE:568-569`, the interval comment at `EE:2240`, and the README's "all subexpressions always evaluated" say the same.

**Proposed wording:**
- **`Logical`:** "Boolean conjunction/disjunction. Evaluation never short-circuits on a value: the right operand is evaluated even when the left one decides the result, so a blocking diagnostic in it still blocks. Operands are evaluated left to right, and evaluation stops at the first operand that blocks, so a later operand's diagnostics are not reported."
- **`Select`:** "Eager conditional: the condition, then-branch and else-branch are evaluated in that fixed order whatever the condition's value, so a blocking diagnostic in the unselected branch still blocks. Evaluation stops at the first of them that blocks, so a later one's diagnostics are not reported. Branches must both be…" (the rest unchanged).
- **The three inline comments and the README line** are aligned to the same wording.
- **Under D**, also update:
  - `FindingCode::NonFiniteInput`'s list, adding "an arithmetic or interpolation result (an overflow)";
  - the interval header's "where the point path computes infinities or NaN, and can fail on them" (`EE:1523-1524`), to read "where the point path blocks".

**Check:** RV104's ADDENDUM_01 method. Non-test code with comment lines removed must be byte-identical; only doc and comment lines differ.

## 6. The slice (written for D; §6.9 gives A, B and C)

### 6.1 Route and sequencing

- **A branch from main** (`codex/piping-t3-si1c-<date>`, worktree `WT/s-i1c`), with records on NUM and its own PR, as for SI1b.
- **One fresh implementer.** It does not start the remedy until the owner has answered (if ROOT rules it owner-held).
- **If the answer is not in by B6's merge,** ROOT may dispatch N-4 and N-5 alone as SI1c-1, and the remedy as SI1c-2. The cost is a second gate run.
- **NUM sequencing** (one unmerged product slice at a time): SI1c's merge into NUM and its PR follow B6's. Its files are disjoint from B6's and B1's.

### 6.2 The write set

| File | Change |
|---|---|
| `P/core/rules/expression_evaluator/src/lib.rs` | D's checks at the five producer sites (§3 D); doc comments (N-5, `NonFiniteInput`, the interval header); new and revised tests in `mod tests`; comment updates in `interval_tests` that say the point path "computes infinities" |
| `P/core/rules/rule_check_runner/src/lib.rs` | N-4 (N4-1): raw and normalized finiteness for inputs and limits, in both limit blocks; no `null` in `bound_inputs` |
| `P/core/rules/rule_check_runner/tests/point_path_non_finite_run.rs` | revise `a_nan_table_argument_check_blocks`; add runner tests (§6.3) |
| `P/core/rules/expression_evaluator/README.md` | N-5 wording; one line: "an intermediate result that is not finite (an overflow) is a blocking `NonFiniteInput` finding" |
| `P/tests/test_rule_interval.py` (optional, decision 9) | align `point_value`, the point-path oracle, to D: `Block` on a non-finite add, subtract, multiply or interpolation step. Its assertions do not change: interval T, F or a finite enclosure implies every sample is finite. 193 tests before and after, run under the N-6 host rule |

**Not touched:**
- the conformance corpus and its README (decision 5);
- schemas, `Cargo.toml` and lockfiles;
- src-tauri and the desktop;
- `docs/SPEC.md` and `docs/TYPES.md`;
- `rule_interval.py` (interval only).

### 6.3 Tests

**New evaluator tests (`mod tests`; exact `finding_records` throughout):**
1. **Each producer overflows and blocks** with its subject, one finding, and value `None`:
   - `1e308 + 1e308` and `−1e308 − 1e308`;
   - ratio × stress at 1e200;
   - the derived product Force × Length;
   - stress / ratio 1e-308;
   - Moment / Length (RV104 probes 12–14);
   - the rise overflow in interpolation (RV104 probe 29's table);
   - the internal run overflow: rows (−MAX, 0), (MAX, 10) at x = 0, which today gives 0.
2. **I79's three reproducers** block, at `multiply`, and none passes.
3. **The absorbing forms block** (rows 5–7):
   - `max(x·1e300 − x·1e300, s)`;
   - `min(x·1e300, s)`;
   - `s / (x·1e300)` (all three divide arms);
   - `select(c, x·1e300, s)` with c true and with c false.
4. **The comparison table (§2.4)** cannot be reached any more. One test asserts that each row's construction blocks at its producer.
5. **Finite boundaries still evaluate:**
   - `MAX + 0`, `MAX·1`, `MAX/2 + MAX/2`;
   - an underflow to a subnormal and to 0;
   - −0;
   - `MAX/1` (the largest ratio);
   - `5e-324/MAX`.
6. **Order:**
   - a unit or dimension mismatch on overflowing values is reported first, and alone;
   - `DivisionByZero` precedes the arms;
   - a block at the producer stops the enclosing expression: `(x·1e300) + missing` and `(x·1e300)/(c/0)` give exactly one finding, and `source_variable_ids` stops there.
7. **Interval mode is unchanged** on the same inputs: U with `non_finite_enclosure`, and no new finding (the shadow values).

**Revised evaluator tests** (each pins SI1b's carry scope, which D reverses; names change where the meaning inverts):

| Test | Change under D |
|---|---|
| `blocks_overflowing_same_dimension_quotient_instead_of_panicking` | its second half (carried +inf and NaN numerators) now blocks at `multiply` |
| `same_dimension_quotients_that_did_not_panic_are_unchanged` | "a finite numerator over a carried infinite divisor is 0" now blocks at `multiply` |
| `non_finite_quotients_outside_the_ratio_arm_still_carry_their_value` | inverts: renamed, now asserting a block at `divide` |
| `blocks_nan_interpolation_and_step_lookup_arguments_instead_of_panicking` | blocks at `multiply` |
| `non_finite_table_arguments_that_did_not_panic_are_unchanged` | blocks at `multiply` |
| `blocks_generated_nan_table_arguments` | blocks at `multiply`, and at `add_subtract` for `z + z` |

Unchanged: `a_blocked_ratio_stops_the_enclosing_expression` and `blocks_an_i73_differential_quotient_input`. Both are ratio overflows from finite operands.

**Runner tests:**
- **Revised:** `a_nan_table_argument_check_blocks` (subject `multiply`; exact lookup too).
- **New:**
  1. A boolean check over I79's reproducer reads `RULE_INPUTS_INCOMPLETE` with one `NonFiniteInput`, both plain and at b = 0.
  2. A quantity check with `max` absorption is blocked, with no `computed_value`.
  3. **N-4:**
     - an input of 1e300 GPa declared in Pa: `supplied: true`, `value` absent, the note, one `NonFiniteInput` with the input id, the status and diagnostic as today;
     - a slot limit the same way;
     - a NaN or +inf input through the Rust API;
     - a non-finite raw value with a differing unit.
  4. **No `null` anywhere** in the serialized outcomes of tests 1–3.

**Test names.** The DEC-025 per-test comparison will show the renamed tests as removed and added. The package lists each one.

### 6.4 The differential (base against candidate; only the intended lines differ)

- **Trees.** `git archive` of current main (base) and of the slice head (candidate), each with its own fresh target, as I79 and RV104 did.
- **Inputs:**
  - I79's committed harness: I73's 36,069 inputs, set 2 (36,000), set 3 (12,000), the runner fixture rows (32,821) and the runner extreme set (39,438), at the recorded harness hashes;
  - RV104's: 125,133 point and 501,588 interval evaluations; 45,604 runner lines × 7 modes;
  - **a new SI1c family:**
    - every producer site, including each divide and multiply arm and each interpolation step;
    - against every consumer: the six comparisons, `not`, `and`, `or`, boolean and quantity `select` (taken and untaken), `min`, `max`, divide-by, table argument and the final quantity;
    - with +inf, −inf and NaN (via `inf − inf`, `0·inf`, `inf/inf`), at depths 1–3;
    - plus the finite-boundary controls and the N-4 values (NaN, ±inf, and overflow at unit normalization).
- **The independent oracle for "intended".**
  - A scratch-only instrumented copy of the base: a thread-local flag set at each D producer when its result is not finite. Under A, the flag is set in `eval_compare` on a non-finite operand.
  - It is written to a side column.
  - The instrumented base's dump, minus the column, must equal the plain base dump byte for byte.
- **Pass conditions:**
  1. **Unflagged lines are identical:** every line with no flag and no N-4 value is byte-identical, in every mode (point, plain, b = 0, bounded, and interval evaluator).
  2. **Flagged lines block:** every flagged line is blocked on the candidate (`value: None`; the check `RULE_INPUTS_INCOMPLETE`), with `NonFiniteInput` as its last finding and a producer subject. No flagged line passes or fails.
  3. **Interval evaluator dumps** are byte-identical on every line.
  4. **Runner lines in bounded mode** are byte-identical except N-4 lines, those with a non-finite caller value or limit after normalization. Each of those is checked against §5.1.
  5. **Plain equals b = 0** on every candidate line (RV99 N-2).
  6. **The schema holds:** every candidate runner line validates against `rule_check_run_result.schema.json` with VENV's `jsonschema`. On the base, the failures are exactly the two `null` classes of §2.7.
- **Report:** counts by family and by consumer row (§2.3). The flagged count replaces §2.7's upper bounds.

### 6.5 Mutants (at least one per site; killed by `cargo test --lib` or the runner file)

| ID | Mutant | Expected killer |
|---|---|---|
| D1 | The add/subtract check removed | new test 1 |
| D2a, D2b, D2c | The multiply check removed, in each arm | new test 1 |
| D3a, D3b | The divide check removed: dimensionless-divisor arm, derived arm | new tests 1 and 3 |
| D4 | The interpolation check only on the final value | the internal-run test |
| D5 | A check pushes its finding but returns the value | the order test (one finding) |
| D6 | A check placed before the unit or dimension check | the order test |
| D7 | `>=` `MAX` treated as overflow | the boundary test |
| D8 | Underflow blocked | the boundary test |
| D9 | The check moved into `eval_compare` (that is, option A) | the absorption tests |
| D10 | A wrong subject | exact `finding_records` |
| N1 | N-4's finiteness check removed | the runner N-4 tests (`MissingRequiredValue` returns) |
| N2 | N-4 keeps `value: Some(NaN)` | the `null` test |
| N3 | The limit message reverted | the runner N-4 tests |
| N4 | N-4 applied to an unreferenced input (a status change) | an unreferenced-input test |

**Equivalent, and recorded as such:**
- **`is_infinite` in place of `!is_finite` at a producer.** From finite operands a single operation cannot give NaN.
- **SI1b's NaN-argument mutants** (I79's S1, I1, T1, T2; RV104's S1, S2, I1, I2, N1). D makes those sites unreachable.

The reviewer runs its own set as well.

### 6.6 The gate set

The product gate set (RR "T3's gate set and Git rules, consolidated …", items 1–6):
1. a compact cut from main, with `source_equality.py` and checked citations;
2. a fresh independent complete-diff review, with the same reviewer confirming each repair;
3. the full 40-manifest suite before the freeze, satisfied by the exact-head DEC-025 as for S-I1, T6S and SI1b;
4. hosted CI and the full-SHA dispatch;
5. GEN-8 on the exact head;
6. the exact-head Mac DEC-025 against a fresh baseline of main, in fresh targets.

**Pass B does not apply.**
- The write set is the rules crates (plus, optionally, a Python test). They are outside PP's closure: I61 PLAN §3, and SI1B_MERGE's RECORD, where RV104 found that none of PP's 15 lockfile workspace crates is a rules crate.
- No D1 call graph or registered identity is touched, so item 7's T9 and both-entry gates do not apply either.

**Formatting.**
- `cargo fmt --check` is clean on the evaluator crate.
- In `RCR`, format only the changed hunks. Its pre-existing drift (RR, I79's item 5) stays.

**Expected DEC-025 deltas:**
- `expression_evaluator` 58 → 58 + about 12, with the renames listed;
- `rule_check_runner` 35 → 35 + about 4;
- `rule_pack_document` 10, unchanged;
- pytest 3,773 (`test_rule_interval.py` 193), vitest 3,612 and the builds unchanged;
- src-tauri's suite unchanged (its rule-check tests use finite demo values).

### 6.7 The reviewer: scope and oracles

**Who.** A fresh RV, the next unused (RV110), not RV104. The fresh-ID rule applies, and SI1c reverses tests that RV104's repair round added. RV104's committed harnesses are available as a starting point. The reviewer builds its own oracle for the intended set.

**Scope:**
- the complete diff;
- §6.4's six pass conditions, re-established independently;
- every row of §2.3 and §2.4, reproduced;
- the soundness property (`interval_outcomes_are_sound_against_the_point_path`) still holds;
- N-4's outputs against §5.1;
- N-5 comment-only by non-test code equality;
- the file fence;
- the test renames;
- mutants, its own and the implementer's.

**Oracles:**
- an instrumented base (or the reviewer's own flag);
- an independent transcription of the point path (for example `point_value` from `test_rule_interval.py`, extended to flag non-finite intermediates) over generated formulas, as a second classifier;
- `jsonschema` validation of every runner line;
- exact-rational spot checks at the ±`MAX` boundary.

### 6.8 Estimates

| Option | Implementation | Review | Gates (wall) |
|---|---|---|---|
| **D** + N4-1 + N-5 | 10–14 h (code 2–3, tests 4–5, harness and differential 3–4, mutants and records 1–2) | 5–8 h | about 3–5 h (DEC-025 and CI), plus ROOT's package and PR, 1–2 h |
| A (+ guard) + N4-1 + N-5 | 6–9 h | 4–6 h | the same |
| B (+ guard) + N4-1 + N-5 | 6–9 h | 4–6 h | the same |
| C (+ guard) + N4-1 + N-5 | 4–6 h | 2–4 h | the same |
| N4-1 + N-5 alone (if split) | 3–5 h | 2–3 h | the same, again |

### 6.9 The slice under A, B or C

- **A and B:** one check in `eval_compare` (§3), and the runner guard that omits a non-finite `computed_value`. No existing test changes. The new tests cover rows 1–4 and §2.4, and pin rows 5–8 as still decided (the declared residual). The differential flag sits in `eval_compare`.
- **C:** the README and doc-comment text of §3 C, the runner guard and N-4. The tests pin rows 1–8 as decided.
- **N-4 and N-5** are as in §5 under every option.

## 7. How each option affects S-I2

**What S-I2 does.** S-I2 binds `absolute_verified` rows as intervals (D2 §4.11.5–§4.11.6). Covered rows stay points. A check runs in interval mode as soon as one of its inputs carries a bound, so for the same formula the mode depends on the solved model's row classes.

**Under D:**
- **The modes agree.** Neither ever decides a check over a non-finite intermediate. The point path blocks with `NonFiniteInput`; interval mode reads U with `causes=non_finite_enclosure`. Both statuses are `RULE_INPUTS_INCOMPLETE`.
- **The remaining differences are straddles** (the design) and the ±`MAX` ulp boundary (interval mode is more conservative).
- **D2's theorem** ("a pass under S-I implies a pass under the point path at every admitted value") still holds. It holds under every option.
- **F2b's check-level gate** (D2 §4.5.2, R5-5) cannot be stopped by a non-finite artefact: such a check is undecided under both identities.

**Under A or B:**
- Rows 5–8 (and under B, the ±inf comparisons) still read decided on points and U with bounds.
- A check can flip between `USER_RULE_CHECKED` and `RULE_INPUTS_INCOMPLETE` when a row's class changes.
- S-I2's UI must explain that, and a committed check of that kind would stop F2b's gate in its domain. None reaches these values today.

**Under C:** the same, for every row. Expect the question to return when S-I2 makes the difference visible to users.

**What S-I2's planning should carry, under any option:**
1. **UI text.** D2 §4.11.5's indeterminate text speaks only of a straddle ("passes for some values and fails for others"). S-I2 needs a text for the cause "cannot be enclosed" (`non_finite_enclosure`, and the other causes), and one for the point path's `NonFiniteInput` block. With RV99 N-6 (message forms), this is also the place to revisit the panel's "(blocked on missing inputs)" label for evaluator blocks. That parenthetical is local panel text, not the registered "Rule inputs incomplete" form.
2. **Baseline.** S-I2's "covered rows unchanged" differential uses main after SI1c as its base.
3. **I61 decision 14 is unaffected.** SI1c touches no file on the precommit call graph or in PP's closure. S-I2's Pass B classification starts from main after SI1c.
4. **N-4 also reaches S-I2.** A solver value that overflows at unit normalization shows as N-4's note and finding. S-I2's binding sites (`TAURI:3208`, `solver_result_row_value`) pass b only for finite rows.

## 8. Decisions

Under the work graph's owner-held list, "any change to public meaning" is the owner's.

| # | Decision | My recommendation | Decider |
|---|---|---|---|
| 1 | Does turning a point-path pass or fail into a block (A, B, D) change public meaning? | **Yes** (§4.2), so decision 2 goes to the owner | ROOT (RR "RV103 passes #1103; …", item 3) |
| 2 | The remedy: D, A, B or C | **D** (§3) | **Owner** if decision 1 is yes; otherwise ROOT |
| 3 | DEC-022: a repair within grammar 1.0.0, or a breaking change (new major version and corpus extension)? | **A repair within 1.0.0; no bump** (§4.4) | **Owner** (DEC-022 is a human ruling) |
| 4 | Code and subjects for the remedy | `NonFiniteInput`, with the producers' existing subjects and the messages of §3 D (§3 A/B for those options) | ROOT (no new vocabulary; SI1b precedent) |
| 5 | Extend the conformance corpus? | **No.** Pin in unit tests, as the corpus README already does for `non_finite_input`. Mandatory only if decision 3 rules a breaking change | ROOT; follows decision 3 |
| 6 | N-4's design, and the limit's code | **N4-1** (no status change). The limit takes `NonFiniteInput` with the slot as subject; the alternative is `RULE_EVALUATOR_ERROR` with the new message | ROOT |
| 7 | Does N-4 (and N-5) ride with the remedy? | **Yes**, in one slice. Split (SI1c-1 now, SI1c-2 later) only if the owner's answer is not in by B6's merge | ROOT |
| 8 | N-5's wording, including the README line | As in §5.2 | ROOT |
| 9 | Align the Python oracle `point_value` to D | **Yes.** It is a transcription of the point path, and its assertions are unaffected. Or comments only, as I79 did | ROOT |
| 10 | The guard for the schema-invalid `computed_value: null` | Included under A, B and C. Under D it is unreachable, and no guard is added | ROOT |
| 11 | Keep SI1b's checks that become unreachable under D (NaN table argument; carried ratio operand) | **Keep them**, and record their mutants as equivalent | ROOT |
| 12 | Accept that D reverses the tests RV104's repair round added (R2/R3 scope), with the renames listed | Accept: they pinned SI1b's scope, which SI1c exists to change | ROOT |
| 13 | The reviewer | A fresh RV (RV110) | ROOT |
| 14 | Route the panel's "(blocked on missing inputs)" parenthetical, and the cause-specific indeterminate texts, to S-I2's UI work | Route them, with RV99 N-6 | ROOT |
| 15 | Sequencing | After B6's merge into NUM and its PR (NUM sequencing). The implementer may start once decision 2 is made | ROOT |

## 9. Limits

- **No code was run.** Rows 5–8 of §2.3 and the interpolation absorption are from reading `EE` and Rust's documented `f64::min`/`max` semantics. The slice's probe and tests pin them.
- **§2.7's counts are upper bounds,** from RV104's kept dumps, which carry no expression text. The flagged differential replaces them.
- **The reader inventory** comes from `git grep` over `P` outside `execution/`, for the runner's types and status tokens. A reader that copies the aggregate under another name would be missed. I found no such reader.
- **"Public meaning" is ROOT's reading to make;** §4.2 gives both sides. Nothing here decides an owner-held matter.
