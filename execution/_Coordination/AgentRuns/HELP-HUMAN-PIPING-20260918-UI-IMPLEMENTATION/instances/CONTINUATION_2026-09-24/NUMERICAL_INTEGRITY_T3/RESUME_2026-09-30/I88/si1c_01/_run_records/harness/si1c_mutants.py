#!/usr/bin/env python3
"""I88 T3-SI1c mutants (scratch tool, not repository content).

Each mutant patches the candidate's evaluator or runner source in a mutant
copy (WT/scratch/i88_si1c/trees/mut, a copy of the candidate tree); each patch
must apply exactly once. A mutant is killed when, still compiling, the
evaluator's `cargo test --lib` or the runner's `point_path_non_finite_run`
fails (both are always run). Every cargo command goes through the T3 lock with
--locked --offline. A survivor is then run through the SI1c, RV104 and I79
evaluator harnesses (and, for a runner mutant, the SI1c runner family), and
its dumps compared with the candidate's: identical dumps support an
equivalence claim. The source is restored after every mutant.

Usage: si1c_mutants.py OUT_JSON [names...]
"""
import json
import os
import re
import shutil
import subprocess
import sys
from pathlib import Path

WT = Path("WT")
S = WT / "scratch/i88_si1c"
OUT = Path(sys.argv[1])
ONLY = sys.argv[2:]
P = S / "trees/mut/projects/chirality-piping"
EE = P / "core/rules/expression_evaluator/src/lib.rs"
RCR = P / "core/rules/rule_check_runner/src/lib.rs"
LOGS = S / "logs/mutants"
LOGS.mkdir(parents=True, exist_ok=True)
DUMPS = S / "dumps/mut"
ORIGINAL = {EE: EE.read_text(), RCR: RCR.read_text()}

FR = lambda op, subject, const, indent: (  # noqa: E731 - the producer call as formatted in the candidate
    f"{indent}let value = finite_result(\n"
    f"{indent}    {op},\n"
    f"{indent}    \"{subject}\",\n"
    f"{indent}    {const},\n"
    f"{indent}    findings,\n"
    f"{indent})?;\n")

ADD = FR("left.value + sign * right.value", "add_subtract", "NON_FINITE_SUM", "    ")
MUL_L = "        (Dimension::Dimensionless, _) => {\n" + FR("left.value * right.value", "multiply", "NON_FINITE_PRODUCT", "            ")
MUL_R = "        (_, Dimension::Dimensionless) => {\n" + FR("left.value * right.value", "multiply", "NON_FINITE_PRODUCT", "            ")
MUL_D = "            Some(product) => {\n" + FR("left.value * right.value", "multiply", "NON_FINITE_PRODUCT", "                ")
DIV_D = "        (dim, Dimension::Dimensionless) => {\n" + FR("left.value / right.value", "divide", "NON_FINITE_QUOTIENT", "            ")
DIV_Q = "            DimensionQuotient::Unique(quotient) => {\n" + FR("left.value / right.value", "divide", "NON_FINITE_QUOTIENT", "                ")
FINITE_HEAD = "    if value.is_finite() {\n        return Some(value);\n    }\n"
FINITE_TAIL = "        message,\n    ));\n    None\n}"
STEPS = (
    "    let rise = finite(high.result - low.result)?;\n"
    "    let offset = finite(x - low.argument)?;\n"
    "    let run = finite(high.argument - low.argument)?;\n"
    "    let fraction = finite(offset / run)?;\n"
    "    let product = finite(rise * fraction)?;\n"
    "    finite(low.result + product)\n")
COMPARE_RESULT = "    let result = match operator {\n        ComparisonOperator::LessThan => left.value < right.value,\n"
ADD_STRUCTURAL = "    if left.dimension != right.dimension {\n        findings.push(dimension_mismatch(\n            \"add_subtract\",\n"


def unwrap(anchor: str, op: str) -> str:
    return anchor.split("let value = finite_result(")[0] + f"let value = {op};\n"


EE_MUTANTS = {
    "D1_add_subtract_check_removed": [(ADD, "    let value = left.value + sign * right.value;\n")],
    "D2a_multiply_dimensionless_left_removed": [(MUL_L, unwrap(MUL_L, "left.value * right.value"))],
    "D2b_multiply_dimensionless_right_removed": [(MUL_R, unwrap(MUL_R, "left.value * right.value"))],
    "D2c_multiply_derived_removed": [(MUL_D, unwrap(MUL_D, "left.value * right.value"))],
    "D3a_divide_dimensionless_divisor_removed": [(DIV_D, unwrap(DIV_D, "left.value / right.value"))],
    "D3b_divide_derived_removed": [(DIV_Q, unwrap(DIV_Q, "left.value / right.value"))],
    "D4_interpolation_final_value_only": [(STEPS,
        "    let value = low.result\n        + (high.result - low.result) * ((x - low.argument) / (high.argument - low.argument));\n"
        "    finite(value)\n")],
    "D5_push_but_return_value": [(FINITE_TAIL, "        message,\n    ));\n    Some(value)\n}")],
    "D6_add_check_before_structural_checks": [(ADD_STRUCTURAL,
        "    finite_result(left.value + sign * right.value, \"add_subtract\", NON_FINITE_SUM, findings)?;\n" + ADD_STRUCTURAL)],
    "D7_max_treated_as_overflow": [(FINITE_HEAD, "    if value.is_finite() && value.abs() < f64::MAX {\n        return Some(value);\n    }\n")],
    "D8_underflow_blocked": [(FINITE_HEAD, "    if value.is_finite() && !value.is_subnormal() {\n        return Some(value);\n    }\n")],
    "D9_option_a_compare_blocks_instead": [
        (FINITE_HEAD, "    if true {\n        return Some(value);\n    }\n"),
        (STEPS, "    let value = low.result\n        + (high.result - low.result) * ((x - low.argument) / (high.argument - low.argument));\n    Some(value)\n"),
        (COMPARE_RESULT,
         "    if !left.value.is_finite() || !right.value.is_finite() {\n"
         "        findings.push(EvaluationFinding::new(FindingCode::NonFiniteInput, \"comparison\", \"comparison operands must be finite\"));\n"
         "        return None;\n    }\n" + COMPARE_RESULT)],
    "D10_wrong_subject": [(MUL_L, MUL_L.replace("\"multiply\"", "\"product\""))],
    "E1_run_step_unchecked": [("    let run = finite(high.argument - low.argument)?;\n", "    let run = high.argument - low.argument;\n")],
    "E2_rise_step_unchecked": [("    let rise = finite(high.result - low.result)?;\n", "    let rise = high.result - low.result;\n")],
    "E3_offset_step_unchecked": [("    let offset = finite(x - low.argument)?;\n", "    let offset = x - low.argument;\n")],
    "E4_fraction_step_unchecked": [("    let fraction = finite(offset / run)?;\n", "    let fraction = offset / run;\n")],
    "E5_product_step_unchecked": [("    let product = finite(rise * fraction)?;\n", "    let product = rise * fraction;\n")],
    "E6_sum_step_unchecked": [("    finite(low.result + product)\n", "    Some(low.result + product)\n")],
    "E7_infinite_only_at_producers": [(FINITE_HEAD, "    if !value.is_infinite() {\n        return Some(value);\n    }\n")],
    "E8_interpolation_subject_changed": [("                        &subject_id,\n                        NON_FINITE_INTERPOLATION,\n",
                                         "                        \"interpolate\",\n                        NON_FINITE_INTERPOLATION,\n")],
    "E9_quotient_message_is_the_ratio_message": [("const NON_FINITE_QUOTIENT: &str = \"quotient must be finite (it overflowed)\";",
                                                 "const NON_FINITE_QUOTIENT: &str = \"same-dimension quotient (ratio) must be finite\";")],
}

# SI1b's mutants (I79 si1b_mutants.py and RV104 rv104_mutants.py, patches verbatim
# where their anchors are unchanged), re-run against SI1c's candidate.
RATIO_BLOCK_HEAD = (
    "            if !ratio.is_finite() {\n"
    "                findings.push(EvaluationFinding::new(\n"
    "                    FindingCode::NonFiniteInput,\n"
    "                    \"divide\",\n"
    "                    \"same-dimension quotient (ratio) must be finite\",\n"
    "                ));\n"
    "                return None;\n"
    "            }\n")
STEP_HEAD = ("        Some(LookupMode::Step) => {\n            if x.is_nan() {\n"
             "                findings.push(nan_table_argument(&subject_id));\n                return None;\n            }\n")
INTERP_HEAD = ("        None => {\n            if x.is_nan() {\n"
               "                findings.push(nan_table_argument(&subject_id));\n                return None;\n            }\n")
QUOTIENT = "            let ratio = left.value / right.value;\n            if !ratio.is_finite() {\n"
SI1B_MUTANTS = {
    "SI1B_I79_Q1_quotient_check_removed": [(QUOTIENT, "            let ratio = left.value / right.value;\n            if false {\n")],
    "SI1B_I79_Q2_quotient_check_infinite_only": [(QUOTIENT, "            let ratio = left.value / right.value;\n            if ratio.is_infinite() {\n")],
    "SI1B_I79_Q3_quotient_code_division_by_zero": [(
        "                    FindingCode::NonFiniteInput,\n                    \"divide\",\n                    \"same-dimension",
        "                    FindingCode::DivisionByZero,\n                    \"divide\",\n                    \"same-dimension")],
    "SI1B_I79_S1_step_nan_check_removed": [("        Some(LookupMode::Step) => {\n            if x.is_nan() {\n",
                                            "        Some(LookupMode::Step) => {\n            if false {\n")],
    "SI1B_I79_S2_step_check_all_non_finite": [("        Some(LookupMode::Step) => {\n            if x.is_nan() {\n",
                                               "        Some(LookupMode::Step) => {\n            if !x.is_finite() {\n")],
    "SI1B_I79_I1_interpolate_nan_check_removed": [("        None => {\n            if x.is_nan() {\n", "        None => {\n            if false {\n")],
    "SI1B_I79_I2_interpolate_check_all_non_finite": [("        None => {\n            if x.is_nan() {\n", "        None => {\n            if !x.is_finite() {\n")],
    "SI1B_I79_T1_nan_argument_code_out_of_range": [(
        "    EvaluationFinding::new(\n        FindingCode::NonFiniteInput,\n        subject_id,\n        \"table argument must be finite",
        "    EvaluationFinding::new(\n        FindingCode::TableOutOfRange,\n        subject_id,\n        \"table argument must be finite")],
    "SI1B_I79_T2_exact_lookup_also_blocks_nan": [(
        "        Some(LookupMode::Exact) => {\n",
        "        Some(LookupMode::Exact) => {\n            if x.is_nan() {\n                findings.push(nan_table_argument(&subject_id));\n                return None;\n            }\n")],
    "SI1B_I79_Q4_quotient_check_before_unit_check": [(
        "            if !quantity_units_match(&left, &right) {\n                findings.push(unit_mismatch(\"divide\", &left, &right));\n                return None;\n            }\n            // The ratio quantity",
        "            if !(left.value / right.value).is_finite() {\n                findings.push(EvaluationFinding::new(FindingCode::NonFiniteInput, \"divide\", \"same-dimension quotient (ratio) must be finite\"));\n                return None;\n            }\n            if !quantity_units_match(&left, &right) {\n                findings.push(unit_mismatch(\"divide\", &left, &right));\n                return None;\n            }\n            // The ratio quantity")],
    # RV104's R2 and R3, re-anchored on SI1c's arms (same inserted guard arm, verbatim).
    "SI1B_RV104_R2_overblock_dimensionless_divisor": [(
        "        (dim, Dimension::Dimensionless) => {\n",
        "        (_, Dimension::Dimensionless) if !(left.value / right.value).is_finite() => {\n"
        "            findings.push(EvaluationFinding::new(FindingCode::NonFiniteInput, \"divide\", "
        "\"same-dimension quotient (ratio) must be finite\"));\n"
        "            None\n"
        "        }\n"
        "        (dim, Dimension::Dimensionless) => {\n")],
    "SI1B_RV104_R3_overblock_derived_quotient": [(
        "            DimensionQuotient::Unique(quotient) => {\n",
        "            DimensionQuotient::Unique(_) if !(left.value / right.value).is_finite() => {\n"
        "                findings.push(EvaluationFinding::new(FindingCode::NonFiniteInput, \"divide\", "
        "\"same-dimension quotient (ratio) must be finite\"));\n"
        "                None\n"
        "            }\n"
        "            DimensionQuotient::Unique(quotient) => {\n")],
    "SI1B_RV104_R4_ratio_subject_changed": [(
        "                    FindingCode::NonFiniteInput,\n                    \"divide\",\n",
        "                    FindingCode::NonFiniteInput,\n                    \"ratio\",\n")],
    "SI1B_RV104_R5_ratio_message_changed": [(
        "                    \"divide\",\n                    \"same-dimension quotient (ratio) must be finite\",\n",
        "                    \"divide\",\n                    \"quotient must be finite\",\n")],
    "SI1B_RV104_R1_ratio_check_nan_only": [("            if !ratio.is_finite() {\n", "            if ratio.is_nan() {\n")],
    "SI1B_RV104_R6_ratio_block_continues_with_zero": [(
        RATIO_BLOCK_HEAD, RATIO_BLOCK_HEAD.replace("                return None;\n",
                                                   "                return Some(EvaluationValue::Quantity(ratio_quantity(0.0)));\n"))],
    "SI1B_RV104_S1_step_subject_changed": [(STEP_HEAD, STEP_HEAD.replace("nan_table_argument(&subject_id)", "nan_table_argument(\"lookup\")"))],
    "SI1B_RV104_S2_step_pushes_but_continues": [(STEP_HEAD, STEP_HEAD.replace("                return None;\n", ""))],
    "SI1B_RV104_I1_interpolate_nan_silent_first_row": [(INTERP_HEAD, INTERP_HEAD.replace(
        "                findings.push(nan_table_argument(&subject_id));\n                return None;\n",
        "                return Some(EvaluationValue::Quantity(Quantity { value: table.rows[0].result, "
        "dimension: table.result_dimension, unit_ref: table.result_unit_ref.trim().to_string(), "
        "unit_required: true, dimension_check_required: true }));\n"))],
    "SI1B_RV104_I2_interpolate_pushes_but_continues": [(INTERP_HEAD, INTERP_HEAD.replace("                return None;\n", ""))],
    "SI1B_RV104_N1_nan_message_changed": [(
        "        \"table argument must be finite: a NaN argument is neither inside nor outside \\\n",
        "        \"table argument is out of range \\\n")],
}

GUARD = ("            Some(rv) if !rv.value.is_finite() => {\n"
         "                if !non_finite_inputs.contains(&id) {\n"
         "                    non_finite_inputs.push(id);\n"
         "                    evaluator_findings.push(non_finite_input_finding(id));\n"
         "                }\n"
         "            }\n")
RCR_MUTANTS = {
    "N1_n4_input_check_removed": [(GUARD, "")],
    "N2_n4_keeps_the_value": [("            value: value.filter(|v| v.is_finite()),\n", "            value,\n")],
    "N3_limit_message_reverted": [(
        "        message: \"value-slot limit must be finite (NaN or ±inf after unit normalization)\"\n",
        "        message: \"value-slot limit has missing or unknown unit/dimension metadata\"\n")],
    "N4_n4_blocks_an_unreferenced_input": [(
        "        let non_finite = raw_non_finite_to_convert || value.is_some_and(|v| !v.is_finite());\n",
        "        let non_finite = raw_non_finite_to_convert || value.is_some_and(|v| !v.is_finite());\n"
        "        if non_finite && !raw_non_finite_to_convert {\n"
        "            completeness_findings.push(non_finite_input_finding(ref_id));\n"
        "        }\n")],
    "N5_raw_value_not_tested_before_normalization": [(
        "            (Some(v), Some(u)) if !v.is_finite() && u.trim() != unit_ref.trim()\n",
        "            (Some(v), Some(u)) if false && !v.is_finite() && u.trim() != unit_ref.trim()\n")],
    "N6_raw_non_finite_in_another_unit_supplied": [(
        "                evaluator_findings.push(non_finite_input_finding(ref_id));\n                (None, None)\n",
        "                evaluator_findings.push(non_finite_input_finding(ref_id));\n                (raw_value, Some(unit_ref.clone()))\n")],
    "N7_note_not_set": [("            note: if non_finite {\n", "            note: if false {\n")],
    "N8_limit_raw_check_removed": [("            if !binding.value.is_finite() {\n                return Err(non_finite_limit_finding(slot_id));\n            }\n", "")],
    "N9_limit_normalized_check_removed": [("            if !value.is_finite() {\n                return Err(non_finite_limit_finding(slot_id));\n            }\n", "")],
    "N10_formula_block_missing_returns_to_evaluator": [(
        "    if !non_finite_inputs.is_empty() {\n        return blocked_after_completeness(\n",
        "    if false {\n        return blocked_after_completeness(\n")],
}

ALL = {**{k: (EE, v) for k, v in EE_MUTANTS.items()},
       **{k: (EE, v) for k, v in SI1B_MUTANTS.items()},
       **{k: (RCR, v) for k, v in RCR_MUTANTS.items()}}


def cargo(label, crate, args):
    env = dict(os.environ, CARGO_TARGET_DIR=str(WT / "targets/i88-si1c-mut"), TMPDIR=str(S / "tmp"),
               RUSTUP_TOOLCHAIN="1.97.1", RUSTUP_AUTO_INSTALL="0", CARGO_INCREMENTAL="0", CARGO_BUILD_JOBS="4")
    log = LOGS / f"{label}.log"
    with open(log, "w") as f:
        rc = subprocess.call([str(WT / "tools/t3_cargo.sh"), "test", "--locked", "--offline"] + args,
                             cwd=crate, env=env, stdout=f, stderr=subprocess.STDOUT)
    return rc, log.read_text()


def failed(log):
    return sorted(set(re.findall(r"^---- (\S+) stdout ----", log, re.M)))


def equivalence_dumps(name, runner):
    """Dumps of a surviving mutant, compared with the candidate's."""
    out = DUMPS / name
    out.mkdir(parents=True, exist_ok=True)
    rcr = P / "core/rules/rule_check_runner"
    ee = P / "core/rules/expression_evaluator"
    for f in ("si1c_family.rs", "rv104_ee_diff.rs"):
        shutil.copy(S / "harness" / f, rcr / "tests" / f)
    shutil.copy(S / "harness/i79_point_diff.rs", ee / "tests/i79_point_diff.rs")
    os.environ.update(SI1C_EE_OUT=str(out / "si1c_ee.txt"), RV104_EE_OUT=str(out / "rv104_ee.txt"),
                      SI1C_RUN_OUT=str(out / "si1c_run.txt"), I73_DUMP_OUT=str(out / "i79_dump.tsv"),
                      I79_EXTREME_OUT=str(out / "i79_extreme.tsv"), I79_TABLE_OUT=str(out / "i79_table.tsv"))
    try:
        tests = ["--test", "si1c_family", "--test", "rv104_ee_diff"]
        cargo(f"{name}_dumps_rcr", rcr, tests + ["--", "--test-threads=1"])
        cargo(f"{name}_dumps_ee", ee, ["--test", "i79_point_diff", "--", "--test-threads=1"])
    finally:
        for f in ("si1c_family.rs", "rv104_ee_diff.rs"):
            (rcr / "tests" / f).unlink()
        (ee / "tests/i79_point_diff.rs").unlink()
    compared = {}
    for f in ("si1c_ee.txt", "rv104_ee.txt", "si1c_run.txt", "i79_dump.tsv", "i79_extreme.tsv", "i79_table.tsv"):
        mine, cand = out / f, S / "dumps/cand" / f
        if not mine.exists():
            compared[f] = "missing"
            continue
        a, b = mine.read_text().splitlines(), cand.read_text().splitlines()
        compared[f] = {"lines": len(a), "differ": sum(1 for x, y in zip(a, b) if x != y) + abs(len(a) - len(b))}
    return compared


results = {}
names = ["UNMUTATED"] + (ONLY or list(ALL))
try:
    for name in names:
        path, patches = ALL.get(name, (EE, []))
        text = ORIGINAL[path]
        for old, new in patches:
            n = text.count(old)
            assert n == 1, (name, n, old[:90])
            text = text.replace(old, new)
        path.write_text(text)
        rc_lib, log_lib = cargo(f"{name}_ee_lib", P / "core/rules/expression_evaluator", ["--lib"])
        rc_run, log_run = cargo(f"{name}_rcr", P / "core/rules/rule_check_runner", ["--test", "point_path_non_finite_run"])
        compiled = "error[E" not in log_lib and "error[E" not in log_run and "could not compile" not in log_lib + log_run
        summary = re.findall(r"^test result: .*$", log_lib + log_run, re.M)
        killed = None if name == "UNMUTATED" else (compiled and (rc_lib != 0 or rc_run != 0))
        results[name] = {"file": path.name if path == EE else "rule_check_runner/src/lib.rs", "ee_lib_rc": rc_lib,
                         "rcr_test_rc": rc_run, "compiled": compiled, "killed": killed,
                         "failed_tests": failed(log_lib) + failed(log_run), "results": summary}
        if killed is False:
            results[name]["equivalence_dumps"] = equivalence_dumps(name, path == RCR)
        path.write_text(ORIGINAL[path])
        print(name, json.dumps(results[name]), flush=True)
finally:
    for path, text in ORIGINAL.items():
        path.write_text(text)
    OUT.write_text(json.dumps(results, indent=1, sort_keys=True))
