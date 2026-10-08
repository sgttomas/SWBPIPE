#!/usr/bin/env python3
"""I88 T3-SI1c: build the instrumented base (scratch only, not repository content).

Applies a side channel to a `git archive` copy of main 025c1cf326 so that the
base itself reports, per top-level call, where option D would block:

* evaluator: at each D producer site (add/subtract; multiply, three arms;
  divide, the dimensionless-divisor and derived arms; each of interpolation's
  six floating steps) the first site whose result is not finite (`FIRST`), the
  first consumer that then receives a non-finite operand (`CONSUMER`), and the
  number of such results (`FIRES`). The ratio arm is not a D site.
* runner: per check, the formula evaluation's record, the synthesized
  comparison's record, and every N-4 value (a caller value or slot limit that
  is not finite, raw or after unit normalization).

Records go to the file named by `SI1C_FLAGS_OUT` (one line per top-level
`evaluate` or `run_rule_checks_with_bounds` call). Nothing else changes: the
dump the harness writes must equal the plain base dump byte for byte.

Usage: si1c_instrument_base.py <tree root containing projects/chirality-piping>
"""
import sys
from pathlib import Path

root = Path(sys.argv[1]) / "projects/chirality-piping/core/rules"
ee = root / "expression_evaluator/src/lib.rs"
rcr = root / "rule_check_runner/src/lib.rs"


def patch(text: str, old: str, new: str, count: int = 1) -> str:
    found = text.count(old)
    assert found == count, (found, count, old[:80])
    return text.replace(old, new)


ORACLE = r'''
// ---- I88 SI1c instrumented base (scratch only; not repository content) ----
#[doc(hidden)]
pub mod si1c_oracle {
    use std::cell::RefCell;
    use std::io::Write;
    use std::sync::Mutex;

    thread_local! {
        static FIRST: RefCell<Option<String>> = const { RefCell::new(None) };
        static CONSUMER: RefCell<Option<String>> = const { RefCell::new(None) };
        static FIRES: RefCell<u32> = const { RefCell::new(0) };
        static IN_RUNNER: RefCell<bool> = const { RefCell::new(false) };
        static LAST: RefCell<String> = const { RefCell::new(String::new()) };
        static EVENTS: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
        static CHECK: RefCell<usize> = const { RefCell::new(0) };
    }
    static FILE: Mutex<Option<std::fs::File>> = Mutex::new(None);

    pub fn write(record: &str) {
        let Ok(path) = std::env::var("SI1C_FLAGS_OUT") else { return };
        let mut file = FILE.lock().unwrap();
        if file.is_none() {
            *file = Some(
                std::fs::OpenOptions::new().create(true).append(true).open(path).unwrap(),
            );
        }
        let name = std::thread::current().name().unwrap_or("-").to_string();
        let line = format!("{name}\t{record}\n");
        file.as_mut().unwrap().write_all(line.as_bytes()).unwrap();
    }

    /// A D producer's result: fires when it is not finite.
    pub fn produce(subject: &str, value: f64) {
        if !value.is_finite() {
            FIRES.with(|f| *f.borrow_mut() += 1);
            FIRST.with(|f| {
                if f.borrow().is_none() {
                    *f.borrow_mut() = Some(subject.to_string());
                }
            });
        }
    }

    /// The six floating steps of a point interpolation.
    pub fn interpolate(subject: &str, rows: (f64, f64, f64, f64), x: f64) {
        let (low_argument, low_result, high_argument, high_result) = rows;
        let rise = high_result - low_result;
        let offset = x - low_argument;
        let run = high_argument - low_argument;
        let fraction = offset / run;
        let product = rise * fraction;
        let sum = low_result + product;
        if [rise, offset, run, fraction, product, sum].iter().any(|v| !v.is_finite()) {
            produce(subject, f64::NAN);
        }
    }

    /// A consumer receiving an operand: notes the first non-finite one.
    pub fn consume(kind: &str, value: f64) {
        if !value.is_finite() {
            CONSUMER.with(|c| {
                if c.borrow().is_none() {
                    *c.borrow_mut() = Some(kind.to_string());
                }
            });
        }
    }

    pub fn begin_evaluate() {
        FIRST.with(|f| *f.borrow_mut() = None);
        CONSUMER.with(|c| *c.borrow_mut() = None);
        FIRES.with(|f| *f.borrow_mut() = 0);
    }

    pub fn end_evaluate() {
        let first = FIRST.with(|f| f.borrow_mut().take()).unwrap_or_else(|| "-".to_string());
        let consumer =
            CONSUMER.with(|c| c.borrow_mut().take()).unwrap_or_else(|| "-".to_string());
        let fires = FIRES.with(|f| *f.borrow());
        let record = format!("{first}\t{consumer}\t{fires}");
        if IN_RUNNER.with(|r| *r.borrow()) {
            LAST.with(|l| *l.borrow_mut() = record);
        } else {
            write(&format!("E\t{record}"));
        }
    }

    pub fn take_last() -> String {
        LAST.with(|l| std::mem::take(&mut *l.borrow_mut())).replace('\t', ",")
    }

    pub fn begin_run() {
        IN_RUNNER.with(|r| *r.borrow_mut() = true);
        EVENTS.with(|e| e.borrow_mut().clear());
        CHECK.with(|c| *c.borrow_mut() = 0);
    }

    pub fn set_check(index: usize) {
        CHECK.with(|c| *c.borrow_mut() = index);
    }

    pub fn event(kind: &str, detail: &str) {
        let index = CHECK.with(|c| *c.borrow());
        EVENTS.with(|e| e.borrow_mut().push(format!("{index}|{kind}|{detail}")));
    }

    pub fn end_run() {
        IN_RUNNER.with(|r| *r.borrow_mut() = false);
        let events = EVENTS.with(|e| std::mem::take(&mut *e.borrow_mut()));
        write(&format!("R\t{}", events.join(";")));
    }
}
'''

s = ee.read_text()
s = patch(s, "use std::fmt;\n", "use std::fmt;\n" + ORACLE)
# The public entry: a wrapper around the unchanged body.
s = patch(s, "pub fn evaluate(input: &EvaluationInput) -> EvaluationResult {\n",
          "pub fn evaluate(input: &EvaluationInput) -> EvaluationResult {\n"
          "    si1c_oracle::begin_evaluate();\n"
          "    let result = si1c_evaluate_body(input);\n"
          "    if let Some(EvaluationValue::Quantity(q)) = &result.value {\n"
          "        si1c_oracle::consume(\"final_quantity\", q.value);\n"
          "    }\n"
          "    si1c_oracle::end_evaluate();\n"
          "    result\n"
          "}\n\n"
          "fn si1c_evaluate_body(input: &EvaluationInput) -> EvaluationResult {\n")
# add/subtract
s = patch(s, "    Some(EvaluationValue::Quantity(\n        left.with_value(left.value + sign * right.value),\n    ))\n}",
          "    si1c_oracle::produce(\"add_subtract\", left.value + sign * right.value);\n"
          "    Some(EvaluationValue::Quantity(\n        left.with_value(left.value + sign * right.value),\n    ))\n}")
# multiply, three arms (each computes the product; an unrepresentable product does not)
s = patch(s, "    match (left.dimension, right.dimension) {\n        (Dimension::Dimensionless, _) => Some(EvaluationValue::Quantity(\n",
          "    if left.dimension == Dimension::Dimensionless\n"
          "        || right.dimension == Dimension::Dimensionless\n"
          "        || dimension_product(left.dimension, right.dimension).is_some()\n"
          "    {\n"
          "        si1c_oracle::produce(\"multiply\", left.value * right.value);\n"
          "    }\n"
          "    match (left.dimension, right.dimension) {\n        (Dimension::Dimensionless, _) => Some(EvaluationValue::Quantity(\n")
# divide: consumer of a non-finite divisor, the dimensionless-divisor arm, the ratio arm (consumer only), the derived arm
s = patch(s, "    match (left.dimension, right.dimension) {\n        (dim, Dimension::Dimensionless) => Some(EvaluationValue::Quantity(Quantity {\n",
          "    si1c_oracle::consume(\"divide_by\", right.value);\n"
          "    if right.dimension == Dimension::Dimensionless\n"
          "        || (left.dimension != right.dimension\n"
          "            && matches!(dimension_quotient(left.dimension, right.dimension), DimensionQuotient::Unique(_)))\n"
          "    {\n"
          "        si1c_oracle::produce(\"divide\", left.value / right.value);\n"
          "    }\n"
          "    match (left.dimension, right.dimension) {\n        (dim, Dimension::Dimensionless) => Some(EvaluationValue::Quantity(Quantity {\n")
s = patch(s, "            let ratio = left.value / right.value;\n",
          "            si1c_oracle::consume(\"ratio_arm\", left.value);\n"
          "            let ratio = left.value / right.value;\n")
# interpolation steps
s = patch(s, "                let (low, high) = (pair[0], pair[1]);\n",
          "                let (low, high) = (pair[0], pair[1]);\n"
          "                si1c_oracle::interpolate(&subject_id, (low.argument, low.result, high.argument, high.result), x);\n")
# consumers: table argument, comparison, quantity select, min/max
s = patch(s, "    let x = argument.value;\n    let first = table.rows[0].argument;\n",
          "    let x = argument.value;\n    si1c_oracle::consume(\"table_argument\", x);\n    let first = table.rows[0].argument;\n")
s = patch(s, "    let result = match operator {\n        ComparisonOperator::LessThan => left.value < right.value,\n",
          "    si1c_oracle::consume(\"compare\", left.value);\n"
          "    si1c_oracle::consume(\"compare\", right.value);\n"
          "    let result = match operator {\n        ComparisonOperator::LessThan => left.value < right.value,\n")
s = patch(s, "            Some(EvaluationValue::Quantity(if condition {\n                then_value\n",
          "            let (taken, untaken) = if condition { (then_value.value, else_value.value) } else { (else_value.value, then_value.value) };\n"
          "            si1c_oracle::consume(\"select_taken\", taken);\n"
          "            si1c_oracle::consume(\"select_untaken\", untaken);\n"
          "            Some(EvaluationValue::Quantity(if condition {\n                then_value\n")
s = patch(s, "        selected = match function {\n",
          "        si1c_oracle::consume(\"min_max\", selected);\n"
          "        si1c_oracle::consume(\"min_max\", quantity.value);\n"
          "        selected = match function {\n")
ee.write_text(s)

r = rcr.read_text()
r = patch(r, "    let checks: Vec<CheckOutcome> = doc\n        .get(\"check_definitions\")\n        .and_then(Value::as_array)\n        .map(|arr| arr.iter().map(|c| run_one_check(&ctx, c)).collect())\n        .unwrap_or_default();\n",
          "    open_pipe_stress_expression_evaluator::si1c_oracle::begin_run();\n"
          "    let checks: Vec<CheckOutcome> = doc\n        .get(\"check_definitions\")\n        .and_then(Value::as_array)\n"
          "        .map(|arr| arr.iter().enumerate().map(|(i, c)| { open_pipe_stress_expression_evaluator::si1c_oracle::set_check(i); run_one_check(&ctx, c) }).collect())\n"
          "        .unwrap_or_default();\n"
          "    open_pipe_stress_expression_evaluator::si1c_oracle::end_run();\n")
# N-4 inputs: raw and normalized
r = patch(r, "        let (value, unit) = match (raw_value, raw_unit) {\n            (Some(v), Some(u)) => {\n",
          "        if let Some(v) = raw_value {\n"
          "            if !v.is_finite() {\n"
          "                open_pipe_stress_expression_evaluator::si1c_oracle::event(\"n4in\", &format!(\"{ref_id}:raw:{}\", if raw_unit.as_deref().map(str::trim) == Some(unit_ref.trim()) { \"same\" } else { \"converted\" }));\n"
          "            }\n"
          "        }\n"
          "        let (value, unit) = match (raw_value, raw_unit) {\n            (Some(v), Some(u)) => {\n")
r = patch(r, "                    Ok((normalized_value, normalized_unit)) => {\n                        (Some(normalized_value), Some(normalized_unit))\n",
          "                    Ok((normalized_value, normalized_unit)) => {\n"
          "                        if v.is_finite() && !normalized_value.is_finite() {\n"
          "                            open_pipe_stress_expression_evaluator::si1c_oracle::event(\"n4in\", &format!(\"{ref_id}:norm:converted\"));\n"
          "                        }\n"
          "                        (Some(normalized_value), Some(normalized_unit))\n")
# the formula's point evaluation and the synthesized comparison
r = patch(r, "    let formula_eval = evaluate(&EvaluationInput {\n        expression,\n",
          "    let formula_eval = evaluate(&EvaluationInput {\n        expression,\n", 1)
r = patch(r, "        declared_grammar_version: ctx.grammar_version.to_string(),\n    });\n    for f in &formula_eval.findings {\n",
          "        declared_grammar_version: ctx.grammar_version.to_string(),\n    });\n"
          "    open_pipe_stress_expression_evaluator::si1c_oracle::event(\"formula\", &open_pipe_stress_expression_evaluator::si1c_oracle::take_last());\n"
          "    for f in &formula_eval.findings {\n")
r = patch(r, "            for f in &comparison.findings {\n                evaluator_findings.push(RunFinding {\n",
          "            open_pipe_stress_expression_evaluator::si1c_oracle::event(\"compare\", &open_pipe_stress_expression_evaluator::si1c_oracle::take_last());\n"
          "            for f in &comparison.findings {\n                evaluator_findings.push(RunFinding {\n")
# N-4 limits (resolve_limit is shared by the point and interval limit blocks)
r = patch(r, "            let (value, unit) = normalize_value_to_declared_unit(\n                binding.value,\n",
          "            if !binding.value.is_finite() {\n"
          "                open_pipe_stress_expression_evaluator::si1c_oracle::event(\"n4lim\", &format!(\"{slot_id}:raw:{}\", if binding.unit.trim() == declared_unit.trim() { \"same\" } else { \"converted\" }));\n"
          "            }\n"
          "            let (value, unit) = normalize_value_to_declared_unit(\n                binding.value,\n")
r = patch(r, "            return Ok(Some((value, unit, decode_dimension(declared_dimension))));\n",
          "            if binding.value.is_finite() && !value.is_finite() {\n"
          "                open_pipe_stress_expression_evaluator::si1c_oracle::event(\"n4lim\", &format!(\"{slot_id}:norm:converted\"));\n"
          "            }\n"
          "            return Ok(Some((value, unit, decode_dimension(declared_dimension))));\n")
rcr.write_text(r)
print("instrumented", ee, rcr)
