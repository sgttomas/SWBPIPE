#!/usr/bin/env python3
"""RV111 (T3-SI1c review): scratch-only instrumentation of a copy of main's evaluator.

Adds a thread-local event log (module `rv111_flags`) that records, without changing any
computed value or control flow:
  - E / I            : the start of `evaluate` / `evaluate_interval`;
  - F|site|subject|k|sources|mask : a D producer whose result is not finite (from the operands it
                       was given), with the number of findings already pushed (k) and the
                       source variable ids pushed so far; for interpolation, the mask of the
                       six floating steps that are not finite;
  - C|kind           : a consumer that receives a non-finite operand (for the per-row report).
Each patch is an exact, single-occurrence string replacement (asserted).
Usage: rv111_instrument_base.py <path to ibase expression_evaluator/src/lib.rs>
"""
import sys

path = sys.argv[1]
src = open(path, encoding="utf-8").read()


def patch(old, new, count=1):
    global src
    n = src.count(old)
    assert n == count, (n, old[:80])
    src = src.replace(old, new)


MODULE = r'''
/// RV111 scratch-only instrumentation (never committed; not part of any candidate).
pub mod rv111_flags {
    use std::cell::RefCell;
    thread_local! {
        static EVENTS: RefCell<Vec<String>> = RefCell::new(Vec::new());
        static SOURCES: RefCell<Vec<String>> = RefCell::new(Vec::new());
    }
    pub fn take() -> Vec<String> {
        EVENTS.with(|e| std::mem::take(&mut *e.borrow_mut()))
    }
    pub(crate) fn start(marker: &str) {
        SOURCES.with(|s| s.borrow_mut().clear());
        EVENTS.with(|e| e.borrow_mut().push(marker.to_string()));
    }
    pub(crate) fn source(id: &str) {
        SOURCES.with(|s| s.borrow_mut().push(id.to_string()));
    }
    pub(crate) fn flag(site: &str, subject: &str, findings_len: usize, mask: &str) {
        let mut sources = SOURCES.with(|s| s.borrow().clone());
        sources.sort();
        sources.dedup();
        EVENTS.with(|e| {
            e.borrow_mut().push(format!(
                "F|{site}|{subject}|{findings_len}|{}|{mask}",
                sources.join(",")
            ))
        });
    }
    pub(crate) fn consumer(kind: &str) {
        EVENTS.with(|e| e.borrow_mut().push(format!("C|{kind}")));
    }
}
'''

# Module after the `use` block.
patch("use std::fmt;\n", "use std::fmt;\n" + MODULE)

# evaluate / evaluate_interval start markers; final non-finite quantity.
patch(
    "pub fn evaluate(input: &EvaluationInput) -> EvaluationResult {\n    let mut findings = Vec::new();\n",
    "pub fn evaluate(input: &EvaluationInput) -> EvaluationResult {\n    rv111_flags::start(\"E\");\n    let mut findings = Vec::new();\n",
)
patch(
    "    source_variable_ids.sort();\n    source_variable_ids.dedup();\n\n    EvaluationResult {\n",
    "    source_variable_ids.sort();\n    source_variable_ids.dedup();\n"
    "    if let Some(EvaluationValue::Quantity(q)) = &value {\n"
    "        if !q.value.is_finite() {\n            rv111_flags::consumer(\"final_quantity\");\n        }\n    }\n\n"
    "    EvaluationResult {\n",
)
patch(
    ") -> IntervalEvaluationResult {\n    let mut findings = Vec::new();\n",
    ") -> IntervalEvaluationResult {\n    rv111_flags::start(\"I\");\n    let mut findings = Vec::new();\n",
)

# Sources pushed by variable references.
patch(
    "    source_variable_ids.push(variable_id.to_string());\n",
    "    source_variable_ids.push(variable_id.to_string());\n    rv111_flags::source(variable_id);\n",
)

# Unary pass-through consumers.
patch(
    "        (UnaryOperator::Negate, EvaluationValue::Quantity(quantity)) => Some(\n",
    "        (UnaryOperator::Negate, EvaluationValue::Quantity(quantity)) if { if !quantity.value.is_finite() { rv111_flags::consumer(\"negate\"); } false } => None,\n"
    "        (UnaryOperator::Negate, EvaluationValue::Quantity(quantity)) => Some(\n",
)
patch(
    "        (UnaryOperator::Abs, EvaluationValue::Quantity(quantity)) => Some(\n",
    "        (UnaryOperator::Abs, EvaluationValue::Quantity(quantity)) if { if !quantity.value.is_finite() { rv111_flags::consumer(\"abs\"); } false } => None,\n"
    "        (UnaryOperator::Abs, EvaluationValue::Quantity(quantity)) => Some(\n",
)

# Quantity select (taken / untaken non-finite branch).
patch(
    "            Some(EvaluationValue::Quantity(if condition {\n                then_value\n            } else {\n                else_value\n            }))\n",
    "            {\n"
    "                let (taken, untaken) = if condition { (&then_value, &else_value) } else { (&else_value, &then_value) };\n"
    "                if !taken.value.is_finite() { rv111_flags::consumer(\"select_quantity_taken\"); }\n"
    "                if !untaken.value.is_finite() { rv111_flags::consumer(\"select_quantity_untaken\"); }\n"
    "            }\n"
    "            Some(EvaluationValue::Quantity(if condition {\n                then_value\n            } else {\n                else_value\n            }))\n",
)

# min / max.
patch(
    "    let first = quantities[0].clone();\n    let mut selected = first.value;\n",
    "    if quantities.iter().any(|q| !q.value.is_finite()) {\n        rv111_flags::consumer(subject_id);\n    }\n"
    "    let first = quantities[0].clone();\n    let mut selected = first.value;\n",
)

# Table argument.
patch(
    "    let x = argument.value;\n",
    "    let x = argument.value;\n"
    "    if !x.is_finite() {\n        rv111_flags::consumer(match mode {\n"
    "            None => \"table_argument_interpolate\",\n"
    "            Some(LookupMode::Step) => \"table_argument_step\",\n"
    "            Some(LookupMode::Exact) => \"table_argument_exact\",\n        });\n    }\n",
)

# Interpolation: the six floating steps, computed separately for the flag only.
patch(
    "                let (low, high) = (pair[0], pair[1]);\n                low.result\n",
    "                let (low, high) = (pair[0], pair[1]);\n"
    "                {\n"
    "                    let rise = high.result - low.result;\n"
    "                    let offset = x - low.argument;\n"
    "                    let run = high.argument - low.argument;\n"
    "                    let fraction = offset / run;\n"
    "                    let product = rise * fraction;\n"
    "                    let sum = low.result + product;\n"
    "                    let steps = [rise, offset, run, fraction, product, sum];\n"
    "                    let mask: String = steps.iter().map(|v| if v.is_finite() { '0' } else { '1' }).collect();\n"
    "                    if mask.contains('1') {\n"
    "                        rv111_flags::flag(\"interpolate\", &subject_id, findings.len(), &mask);\n"
    "                    }\n"
    "                }\n"
    "                low.result\n",
)

# Binary consumers (a non-finite operand reaching arithmetic).
patch(
    "    match operator {\n        BinaryOperator::Add => add_or_subtract(left, right, 1.0, findings),\n",
    "    if !left.value.is_finite() || !right.value.is_finite() {\n"
    "        rv111_flags::consumer(match operator {\n"
    "            BinaryOperator::Add | BinaryOperator::Subtract => \"add_subtract_operand\",\n"
    "            BinaryOperator::Multiply => \"multiply_operand\",\n"
    "            BinaryOperator::Divide => if !right.value.is_finite() { \"divide_divisor\" } else { \"divide_numerator\" },\n"
    "        });\n    }\n"
    "    match operator {\n        BinaryOperator::Add => add_or_subtract(left, right, 1.0, findings),\n",
)

# add / subtract producer.
patch(
    "    Some(EvaluationValue::Quantity(\n        left.with_value(left.value + sign * right.value),\n    ))\n",
    "    let rv111_value = left.value + sign * right.value;\n"
    "    if left.value.is_finite() && right.value.is_finite() && !rv111_value.is_finite() {\n"
    "        rv111_flags::flag(\"add_subtract\", \"add_subtract\", findings.len(), \"\");\n    }\n"
    "    Some(EvaluationValue::Quantity(left.with_value(rv111_value)))\n",
)

# multiply producers (three arms).
patch(
    "        (Dimension::Dimensionless, _) => Some(EvaluationValue::Quantity(\n            right.with_value(left.value * right.value),\n        )),\n"
    "        (_, Dimension::Dimensionless) => Some(EvaluationValue::Quantity(\n            left.with_value(left.value * right.value),\n        )),\n",
    "        (Dimension::Dimensionless, _) => {\n"
    "            let rv111_value = left.value * right.value;\n"
    "            if left.value.is_finite() && right.value.is_finite() && !rv111_value.is_finite() {\n"
    "                rv111_flags::flag(\"multiply_dimensionless_left\", \"multiply\", findings.len(), \"\");\n            }\n"
    "            Some(EvaluationValue::Quantity(right.with_value(rv111_value)))\n        }\n"
    "        (_, Dimension::Dimensionless) => {\n"
    "            let rv111_value = left.value * right.value;\n"
    "            if left.value.is_finite() && right.value.is_finite() && !rv111_value.is_finite() {\n"
    "                rv111_flags::flag(\"multiply_dimensionless_right\", \"multiply\", findings.len(), \"\");\n            }\n"
    "            Some(EvaluationValue::Quantity(left.with_value(rv111_value)))\n        }\n",
)
patch(
    "            Some(product) => Some(EvaluationValue::Quantity(Quantity {\n                value: left.value * right.value,\n",
    "            Some(product) => Some(EvaluationValue::Quantity(Quantity {\n                value: {\n"
    "                    let rv111_value = left.value * right.value;\n"
    "                    if left.value.is_finite() && right.value.is_finite() && !rv111_value.is_finite() {\n"
    "                        rv111_flags::flag(\"multiply_derived\", \"multiply\", findings.len(), \"\");\n                    }\n"
    "                    rv111_value\n                },\n",
)

# divide producers (dimensionless-divisor and derived arms).
patch(
    "        (dim, Dimension::Dimensionless) => Some(EvaluationValue::Quantity(Quantity {\n            value: left.value / right.value,\n",
    "        (dim, Dimension::Dimensionless) => Some(EvaluationValue::Quantity(Quantity {\n            value: {\n"
    "                let rv111_value = left.value / right.value;\n"
    "                if left.value.is_finite() && right.value.is_finite() && !rv111_value.is_finite() {\n"
    "                    rv111_flags::flag(\"divide_dimensionless_divisor\", \"divide\", findings.len(), \"\");\n                }\n"
    "                rv111_value\n            },\n",
)
patch(
    "            DimensionQuotient::Unique(quotient) => Some(EvaluationValue::Quantity(Quantity {\n                value: left.value / right.value,\n",
    "            DimensionQuotient::Unique(quotient) => Some(EvaluationValue::Quantity(Quantity {\n                value: {\n"
    "                    let rv111_value = left.value / right.value;\n"
    "                    if left.value.is_finite() && right.value.is_finite() && !rv111_value.is_finite() {\n"
    "                        rv111_flags::flag(\"divide_derived\", \"divide\", findings.len(), \"\");\n                    }\n"
    "                    rv111_value\n                },\n",
)
# The ratio arm already blocks on main (SI1b); record it as a consumer event only.
patch(
    "            let ratio = left.value / right.value;\n",
    "            let ratio = left.value / right.value;\n"
    "            if !ratio.is_finite() && left.value.is_finite() && right.value.is_finite() {\n"
    "                rv111_flags::consumer(\"ratio_block_from_finite\");\n            }\n",
)

# Comparison consumer (after the structural checks).
patch(
    "    let result = match operator {\n        ComparisonOperator::LessThan => left.value < right.value,\n",
    "    if !left.value.is_finite() || !right.value.is_finite() {\n        rv111_flags::consumer(\"compare\");\n    }\n"
    "    let result = match operator {\n        ComparisonOperator::LessThan => left.value < right.value,\n",
)

open(path, "w", encoding="utf-8").write(src)
print("instrumented", path)
