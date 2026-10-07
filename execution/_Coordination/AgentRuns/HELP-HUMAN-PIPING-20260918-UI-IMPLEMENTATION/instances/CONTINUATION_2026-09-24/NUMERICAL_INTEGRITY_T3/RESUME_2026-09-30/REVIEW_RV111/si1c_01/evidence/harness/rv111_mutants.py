#!/usr/bin/env python3
"""RV111 (T3-SI1c review): RV111's own mutants of the candidate, at least one per site.

Each mutant is an exact single-occurrence replacement in a copy of the candidate (`WT/rv111/mut`,
target `WT/targets/rv111-mut`). Killing tests: for evaluator mutants the evaluator's `cargo test --lib` and the runner's
`point_path_non_finite_run`; for runner mutants the runner crate's whole `cargo test`. Every cargo command goes through
`WT/tools/t3_cargo.sh` with `--offline --locked`. With `--diff <ids>`, the named mutants are also run
through RV111's differential harness (dumps in `dumps/mut_<id>/`).
Usage: rv111_mutants.py [--only id,id] [--diff id,id]
"""
import json
import os
import shutil
import subprocess
import sys
import time

WT = "WT"
S = f"{WT}/scratch/rv111_si1c_01"
P = "projects/chirality-piping"
CAND = f"{WT}/rv111/cand/{P}"
MUT = f"{WT}/rv111/mut/{P}"
EE = "core/rules/expression_evaluator/src/lib.rs"
RCR = "core/rules/rule_check_runner/src/lib.rs"
ENV = dict(os.environ, TMPDIR=f"{S}/tmp", CARGO_TARGET_DIR=f"{WT}/targets/rv111-mut")

FR = lambda op, subj, msg, ind: (  # noqa: E731
    f"{ind}let value = finite_result(\n{ind}    left.value {op} right.value,\n{ind}    \"{subj}\",\n"
    f"{ind}    {msg},\n{ind}    findings,\n{ind})?;\n")

MUTANTS = [
    # --- evaluator: D at each producer
    ("E01_add_unchecked", EE,
     "    let value = finite_result(\n        left.value + sign * right.value,\n        \"add_subtract\",\n        NON_FINITE_SUM,\n        findings,\n    )?;\n",
     "    let value = left.value + sign * right.value;\n"),
    ("E02_mul_dimensionless_left_unchecked", EE,
     "        (Dimension::Dimensionless, _) => {\n" + FR("*", "multiply", "NON_FINITE_PRODUCT", "            "),
     "        (Dimension::Dimensionless, _) => {\n            let value = left.value * right.value;\n"),
    ("E03_mul_dimensionless_right_unchecked", EE,
     "        (_, Dimension::Dimensionless) => {\n" + FR("*", "multiply", "NON_FINITE_PRODUCT", "            "),
     "        (_, Dimension::Dimensionless) => {\n            let value = left.value * right.value;\n"),
    ("E04_mul_derived_unchecked", EE,
     "            Some(product) => {\n" + FR("*", "multiply", "NON_FINITE_PRODUCT", "                "),
     "            Some(product) => {\n                let value = left.value * right.value;\n"),
    ("E05_div_dimensionless_divisor_unchecked", EE,
     "        (dim, Dimension::Dimensionless) => {\n" + FR("/", "divide", "NON_FINITE_QUOTIENT", "            "),
     "        (dim, Dimension::Dimensionless) => {\n            let value = left.value / right.value;\n"),
    ("E06_div_derived_unchecked", EE,
     "            DimensionQuotient::Unique(quotient) => {\n" + FR("/", "divide", "NON_FINITE_QUOTIENT", "                "),
     "            DimensionQuotient::Unique(quotient) => {\n                let value = left.value / right.value;\n"),
    ("E07_block_continues_with_value", EE,
     "        message,\n    ));\n    None\n}\n", "        message,\n    ));\n    Some(value)\n}\n"),
    ("E08_nan_only", EE, "    if value.is_finite() {\n        return Some(value);", "    if !value.is_nan() {\n        return Some(value);"),
    ("E09_is_infinite_for_not_finite", EE, "    if value.is_finite() {\n        return Some(value);", "    if !value.is_infinite() {\n        return Some(value);"),
    ("E10_max_blocks", EE, "    if value.is_finite() {\n        return Some(value);", "    if value.is_finite() && value.abs() < f64::MAX {\n        return Some(value);"),
    ("E11_subnormal_blocks", EE, "    if value.is_finite() {\n        return Some(value);",
     "    if value.is_finite() && (value == 0.0 || value.abs() >= f64::MIN_POSITIVE) {\n        return Some(value);"),
    ("E12_add_check_before_structure", EE,
     ") -> Option<EvaluationValue> {\n    if left.dimension != right.dimension {\n        findings.push(dimension_mismatch(\n            \"add_subtract\",",
     ") -> Option<EvaluationValue> {\n    finite_result(left.value + sign * right.value, \"add_subtract\", NON_FINITE_SUM, findings)?;\n"
     "    if left.dimension != right.dimension {\n        findings.push(dimension_mismatch(\n            \"add_subtract\","),
    ("E13_mul_check_before_product_lookup", EE,
     "    match (left.dimension, right.dimension) {\n        (Dimension::Dimensionless, _) => {\n",
     "    finite_result(left.value * right.value, \"multiply\", NON_FINITE_PRODUCT, findings)?;\n"
     "    match (left.dimension, right.dimension) {\n        (Dimension::Dimensionless, _) => {\n"),
    ("E14_div_check_before_zero", EE,
     "    if right.value == 0.0 {\n        findings.push(EvaluationFinding::new(\n            FindingCode::DivisionByZero,",
     "    finite_result(left.value / right.value, \"divide\", NON_FINITE_QUOTIENT, findings)?;\n"
     "    if right.value == 0.0 {\n        findings.push(EvaluationFinding::new(\n            FindingCode::DivisionByZero,"),
    ("E15_div_check_before_quotient_lookup", EE,
     "        (left_dim, right_dim) => match dimension_quotient(left_dim, right_dim) {",
     "        (left_dim, right_dim) => match {\n            finite_result(left.value / right.value, \"divide\", NON_FINITE_QUOTIENT, findings)?;\n"
     "            dimension_quotient(left_dim, right_dim)\n        } {"),
    ("E16_divide_subject", EE,
     "                left.value / right.value,\n                \"divide\",\n                NON_FINITE_QUOTIENT,",
     "                left.value / right.value,\n                \"quotient\",\n                NON_FINITE_QUOTIENT,"),
    ("E17_derived_product_message", EE,
     "                    left.value * right.value,\n                    \"multiply\",\n                    NON_FINITE_PRODUCT,",
     "                    left.value * right.value,\n                    \"multiply\",\n                    NON_FINITE_SUM,"),
    ("E18_ratio_arm_quotient_message", EE,
     "                    \"divide\",\n                    \"same-dimension quotient (ratio) must be finite\",",
     "                    \"divide\",\n                    NON_FINITE_QUOTIENT,"),
    # --- evaluator: the six interpolation steps
    ("I01_rise_unchecked", EE, "    let rise = finite(high.result - low.result)?;", "    let rise = high.result - low.result;"),
    ("I02_offset_unchecked", EE, "    let offset = finite(x - low.argument)?;", "    let offset = x - low.argument;"),
    ("I03_run_unchecked", EE, "    let run = finite(high.argument - low.argument)?;", "    let run = high.argument - low.argument;"),
    ("I04_fraction_unchecked", EE, "    let fraction = finite(offset / run)?;", "    let fraction = offset / run;"),
    ("I05_product_unchecked", EE, "    let product = finite(rise * fraction)?;", "    let product = rise * fraction;"),
    ("I06_sum_unchecked", EE, "    finite(low.result + product)\n}", "    Some(low.result + product)\n}"),
    ("I07_four_unchecked_jointly", EE,
     "    let rise = finite(high.result - low.result)?;\n    let offset = finite(x - low.argument)?;\n"
     "    let run = finite(high.argument - low.argument)?;\n    let fraction = finite(offset / run)?;\n"
     "    let product = finite(rise * fraction)?;\n",
     "    let rise = high.result - low.result;\n    let offset = x - low.argument;\n"
     "    let run = finite(high.argument - low.argument)?;\n    let fraction = offset / run;\n"
     "    let product = rise * fraction;\n"),
    ("I08_final_value_only", EE,
     "    let rise = finite(high.result - low.result)?;\n    let offset = finite(x - low.argument)?;\n"
     "    let run = finite(high.argument - low.argument)?;\n    let fraction = finite(offset / run)?;\n"
     "    let product = finite(rise * fraction)?;\n",
     "    let rise = high.result - low.result;\n    let offset = x - low.argument;\n"
     "    let run = high.argument - low.argument;\n    let fraction = offset / run;\n"
     "    let product = rise * fraction;\n"),
    ("I09_interpolation_subject", EE,
     "                        &subject_id,\n                        NON_FINITE_INTERPOLATION,",
     "                        \"interpolate\",\n                        NON_FINITE_INTERPOLATION,"),
    # --- runner: N-4 inputs
    ("N01_raw_check_removed", RCR, "            _ if raw_non_finite_to_convert => {", "            _ if false => {"),
    ("N02_raw_other_unit_supplied", RCR,
     "            _ if raw_non_finite_to_convert => {\n                evaluator_findings.push(non_finite_input_finding(ref_id));\n                (None, None)\n            }",
     "            _ if raw_non_finite_to_convert => (raw_value, Some(unit_ref.clone())),"),
    ("N03_note_dropped", RCR,
     "            note: if non_finite {\n                Some(NON_FINITE_INPUT_NOTE.to_string())\n            } else {\n                note\n            },",
     "            note,"),
    ("N04_value_kept", RCR, "            value: value.filter(|v| v.is_finite()),", "            value,"),
    ("N05_formula_check_removed", RCR, "            Some(rv) if !rv.value.is_finite() => {", "            Some(rv) if false => {"),
    ("N06_dedup_removed", RCR, "                if !non_finite_inputs.contains(&id) {", "                if true {"),
    ("N07_block_after_mode_dispatch", RCR,
     "    if !non_finite_inputs.is_empty() {\n        return blocked_after_completeness(",
     "    if !non_finite_inputs.is_empty() && intervals.is_empty() {\n        return blocked_after_completeness("),
    ("N08_unreferenced_input_blocks", RCR,
     "    let mut non_finite_inputs: Vec<&str> = Vec::new();\n",
     "    let mut non_finite_inputs: Vec<&str> = Vec::new();\n    for (rid, rv) in &resolved {\n"
     "        if !rv.value.is_finite() {\n            non_finite_inputs.push(rid.as_str());\n"
     "            evaluator_findings.push(non_finite_input_finding(rid));\n        }\n    }\n"),
    ("N09_trim_dropped", RCR, "if !v.is_finite() && u.trim() != unit_ref.trim()", "if !v.is_finite() && u != unit_ref"),
    ("N10_note_nan_only", RCR, "value.is_some_and(|v| !v.is_finite());", "value.is_some_and(|v| v.is_nan());"),
    ("N11_formula_check_nan_only", RCR, "            Some(rv) if !rv.value.is_finite() => {", "            Some(rv) if rv.value.is_nan() => {"),
    ("N12_input_message", RCR, "message: \"supplied value must be finite (NaN or ±inf after unit normalization)\".to_string(),",
     "message: \"required variable has no supplied value\".to_string(),"),
    # --- runner: N-4 limits
    ("L01_limit_raw_check_removed", RCR,
     "            if !binding.value.is_finite() {\n                return Err(non_finite_limit_finding(slot_id));\n            }\n", ""),
    ("L02_limit_normalized_check_removed", RCR,
     "            if !value.is_finite() {\n                return Err(non_finite_limit_finding(slot_id));\n            }\n", ""),
    ("L03_limit_subject", RCR, "        subject_id: slot_id.to_string(),", "        subject_id: \"value_slot\".to_string(),"),
    ("L04_limit_as_missing", RCR,
     "            if !binding.value.is_finite() {\n                return Err(non_finite_limit_finding(slot_id));\n            }\n",
     "            if !binding.value.is_finite() {\n                return Ok(None);\n            }\n"),
    # --- SI1b's guards, unreachable under D (I88's recorded equivalences, re-tested)
    ("X01_step_nan_check_removed", EE, "        Some(LookupMode::Step) => {\n            if x.is_nan() {\n", "        Some(LookupMode::Step) => {\n            if false {\n"),
    ("X02_interpolate_nan_check_removed", EE, "        None => {\n            if x.is_nan() {\n", "        None => {\n            if false {\n"),
    ("X03_ratio_check_infinite_only", EE, "            let ratio = left.value / right.value;\n            if !ratio.is_finite() {\n",
     "            let ratio = left.value / right.value;\n            if ratio.is_infinite() {\n"),
    ("X04_nan_argument_code_out_of_range", EE,
     "    EvaluationFinding::new(\n        FindingCode::NonFiniteInput,\n        subject_id,\n        \"table argument must be finite",
     "    EvaluationFinding::new(\n        FindingCode::TableOutOfRange,\n        subject_id,\n        \"table argument must be finite"),
]


def cargo(crate, args, log):
    cwd = f"{MUT}/core/rules/{crate}"
    with open(log, "w") as fh:
        rc = subprocess.call([f"{WT}/tools/t3_cargo.sh"] + args, cwd=cwd, env=ENV, stdout=fh, stderr=subprocess.STDOUT)
    return rc


def restore():
    for f in (EE, RCR):
        shutil.copyfile(f"{CAND}/{f}", f"{MUT}/{f}")


def failed_tests(log):
    out = []
    for line in open(log, errors="replace"):
        if line.startswith("test ") and line.rstrip().endswith("FAILED"):
            out.append(line.split()[1])
    return out


def main():
    only = diff = None
    if "--only" in sys.argv:
        only = set(sys.argv[sys.argv.index("--only") + 1].split(","))
    if "--diff" in sys.argv:
        diff = set(sys.argv[sys.argv.index("--diff") + 1].split(","))
    os.makedirs(f"{S}/mutants", exist_ok=True)
    results_path = f"{S}/mutants/mutants.json"
    results = json.load(open(results_path)) if os.path.exists(results_path) else {}
    for mid, f, old, new in MUTANTS:
        if only and mid not in only:
            continue
        if diff is not None and mid not in diff:
            continue
        restore()
        path = f"{MUT}/{f}"
        src = open(path, encoding="utf-8").read()
        n = src.count(old)
        if n != 1:
            results[mid] = {"error": f"anchor occurs {n} times"}
            continue
        open(path, "w", encoding="utf-8").write(src.replace(old, new))
        r = results.get(mid, {}) if diff else {"file": f}
        if diff is None:
            jobs = []
            if f == EE:
                jobs.append(("ee_lib", "expression_evaluator", ["test", "--offline", "--locked", "--lib", "--no-fail-fast"]))
            if f == EE:
                jobs.append(("runner_pp", "rule_check_runner", ["test", "--offline", "--locked", "--no-fail-fast",
                                                                "--test", "point_path_non_finite_run"]))
            else:
                jobs.append(("runner_all", "rule_check_runner", ["test", "--offline", "--locked", "--no-fail-fast"]))
            killed_by = []
            for name, crate, args in jobs:
                if name == "runner_pp" and mid.startswith("X"):
                    # SI1b's NaN-argument guards: no runner test can reach them either (host time).
                    r[name] = {"skipped": "guard unreachable from the runner tests"}
                    continue
                if name == "runner_pp" and killed_by:
                    # Already killed by the evaluator's own tests: the runner run is skipped (host time).
                    r[name] = {"skipped": "killed by ee_lib"}
                    continue
                log = f"{S}/mutants/{mid}.{name}.log"
                rc = cargo(crate, args, log)
                text = open(log, errors="replace").read()
                if "error[E" in text or "could not compile" in text:
                    r["compile_error"] = name
                failed = failed_tests(log)
                r[name] = {"rc": rc, "failed": failed}
                killed_by += failed
            r["killed"] = bool(killed_by) or "compile_error" in r
        else:
            shutil.copyfile(f"{S}/harness/rv111_si1c.rs", f"{MUT}/core/rules/rule_check_runner/tests/rv111_si1c.rs")
            shutil.copyfile(f"{S}/harness/rv111_probe.rs", f"{MUT}/core/rules/rule_check_runner/tests/rv111_probe.rs")
            out = f"{S}/dumps/mut_{mid}"
            os.makedirs(out, exist_ok=True)
            env = dict(ENV, RV111_OUT=out)
            log = f"{S}/mutants/{mid}.diff.log"
            with open(log, "w") as fh:
                rc = subprocess.call([f"{WT}/tools/t3_cargo.sh", "test", "--offline", "--locked", "--test", "rv111_si1c",
                                      "--test", "rv111_probe", "--", "--test-threads=2"], cwd=f"{MUT}/core/rules/rule_check_runner", env=env,
                                     stdout=fh, stderr=subprocess.STDOUT)
            os.remove(f"{MUT}/core/rules/rule_check_runner/tests/rv111_si1c.rs")
            os.remove(f"{MUT}/core/rules/rule_check_runner/tests/rv111_probe.rs")
            r["diff_rc"] = rc
        results[mid] = r
        json.dump(results, open(results_path, "w"), indent=1, sort_keys=True)
        print(time.strftime("%H:%M:%S"), mid, json.dumps(r)[:300], flush=True)
    restore()


if __name__ == "__main__":
    main()
