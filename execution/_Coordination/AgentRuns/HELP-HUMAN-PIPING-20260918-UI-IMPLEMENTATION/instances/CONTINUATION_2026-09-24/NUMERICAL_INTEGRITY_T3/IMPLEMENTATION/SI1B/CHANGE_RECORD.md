# T3-SI1b: point-path panics become blocking findings: change record

T3 (numerical integrity), node T3-SI1b. Records live on the integration branch `codex/piping-numerical-integrity-20260926` (NUM). This package is the PR's only execution content.

## 1. What the PR contains

Three maintained files under `projects/chirality-piping/`. Their maintained diff from main equals SI1b's branch at `0730c87aef`, and NUM carries the same blobs (`source_equality.py`).

| File | Change | sha256 (prefix) |
|---|---|---|
| `core/rules/expression_evaluator/src/lib.rs` | +430 / −3: the two repaired sites, a doc comment on `FindingCode::NonFiniteInput`, and 8 tests | `07d9a7a525d6d151` |
| `core/rules/rule_check_runner/tests/point_path_non_finite_run.rs` | new: 2 runner tests | `de59c4177135ac10` |
| `tests/test_rule_interval.py` | +5 / −3: comments and one docstring in the Python oracle, which no longer say the point path panics | `46ee5156d657039d` |

## 2. What it does

- **The problem.** The ordinary point `evaluate` panicked, rather than returning a finding, on inputs reachable at extreme magnitudes:
  - a same-dimension quotient that overflows (`divide` → `ratio_quantity(…).expect(…)`);
  - a NaN argument to `interpolate` or a step `lookup` (the `.expect("in-range … always has a …")` calls).

  I73 found this during S-I1. It was routed here, because S-I1 had to keep point mode unchanged (RR "I73's checkpoint 1 and I74's plan ruled; …", ruling 3).
- **The repair.** Each site now returns the crate's existing blocking `NonFiniteInput` finding, with a message that names its cause:
  - "same-dimension quotient (ratio) must be finite";
  - "table argument must be finite: a NaN argument is neither inside nor outside the table range".

  The panic was `.expect` on that same `EvaluationError::NonFiniteInput`. The runner and `stress_recovery` already use the code for computed non-finite values, so no public vocabulary is added (RR "RV103 passes #1103; …", item 1).
- **What is kept:**
  - an infinite table argument still reads `TableOutOfRange`;
  - an exact lookup of NaN still reads `TableKeyNotFound`;
  - unit and zero-divisor checks keep their order;
  - the other divide arms still carry non-finite values through;
  - a blocked check does not stop a rule run.

## 3. What it does not do

- **No byte changes for other inputs.** Every input that did not panic on main gives byte-identical results, in the evaluator and in `run_rule_checks`.
- **Interval mode is unchanged:** `evaluate_interval` and `run_rule_checks_with_bounds` with b > 0. S-I1's 94 shared cases and `test_rule_interval.py` (193) pass.
- **No schema, dependency or lock change,** and no product caller change.
- **Outside the F2a D1 milestone's build closure** (the rules crates; I61 PLAN §3), so there is no Pass B.
- **Not addressed here: point-path boolean formulas over a NaN or infinite intermediate** are still decided and can pass (`NaN ≠ 100`). That changes outputs for inputs that do not panic today, so it is T3-SI1c's, with RV104's N-4 and N-5.

## 4. Review

- **I79 implemented it.** Its differential covered three input sets: I73's 36,069 inputs, an extreme set (36,000) and a table set (12,000).
  - Only the 503 base panics changed, and each now blocks.
  - The interval dumps are identical.
- **RV104 reviewed it independently,** with its own generators: 125,133 point and 501,588 interval evaluations, and 216,478 runner lines.
  - On the candidate, no panic was reachable. On main, the same inputs panicked 5,877 and 2,609 times, all at the three known sites, with overflow checks on.
  - There were no unexpected byte differences.
  - **Verdict:** PASS, 0/0/6.
- **The repair round** added assertions that kill RV104's three surviving mutants, and completed the doc comment (I79 REPAIR_01; RV104 confirms in ADDENDUM_01).
- **Mutants:** I79's 13 and RV104's 11 are all killed at the head. RV104's R2, R3 and R6 are killed only by the repair round's two tests (RV104 ADDENDUM_01: CONFIRMED; no executable change; its 626,721-line evaluator and 216,478-line runner dumps byte-identical across the round).

## 5. Gates

| Gate | Result |
|---|---|
| Independent complete-diff review (RV104) | PASS (0/0/6); repair round CONFIRMED (ADDENDUM_01) |
| Suites against main | `expression_evaluator` 49 → 57; `rule_check_runner` 33 → 35; `rule_pack_document` 10; Python `test_rule_interval.py` 193. Every count change is an added test |
| `source_equality.py`, `check_citations.py`, GEN-8 on the exact head, hosted CI with the full-SHA dispatch, and the exact-head Mac DEC-025 against a fresh main baseline | See the merge record |
| Pass B | Not applicable: the rules crates are outside PP's closure |

## 6. Open obligations carried forward

- **T3-SI1c:** a plan for point-path booleans over non-finite intermediates, with RV104 N-4 (the cause named for non-finite caller values and limits) and N-5 (the Logical/Select doc). If it changes public meaning, it goes to the owner.
- **S-I2:** binding solver bounds into rules accounts for SI1c.
