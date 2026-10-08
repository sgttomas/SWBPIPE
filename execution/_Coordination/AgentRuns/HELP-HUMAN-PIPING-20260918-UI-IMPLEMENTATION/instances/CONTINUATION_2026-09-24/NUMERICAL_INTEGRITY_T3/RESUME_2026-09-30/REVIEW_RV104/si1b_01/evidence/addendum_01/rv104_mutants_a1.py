#!/usr/bin/env python3
"""RV104 scratch mutants (not repository content).

Applies one mutant at a time to the candidate's `expression_evaluator/src/lib.rs`
in RV104's own copy (WT/rv104/mut), runs the evaluator's lib tests and the
runner's `point_path_non_finite_run` test through the T3 cargo lock, and
records which tests fail. The file is restored after every mutant.
Usage: rv104_mutants.py WT OUT_JSON [names...]
"""
import json
import os
import re
import subprocess
import sys
from pathlib import Path

WT = Path(sys.argv[1])
OUT = Path(sys.argv[2])
ONLY = sys.argv[3:]
P = WT / "rv104/mut/projects/chirality-piping"
LIB = P / "core/rules/expression_evaluator/src/lib.rs"
SCRATCH = WT / "scratch/rv104_si1b_01"
LOGS = SCRATCH / "a1/logs/mutants"
LOGS.mkdir(parents=True, exist_ok=True)
ORIGINAL = LIB.read_text()

RATIO_BLOCK_HEAD = (
    "            if !ratio.is_finite() {\n"
    "                findings.push(EvaluationFinding::new(\n"
    "                    FindingCode::NonFiniteInput,\n"
    "                    \"divide\",\n"
    "                    \"same-dimension quotient (ratio) must be finite\",\n"
    "                ));\n"
    "                return None;\n"
    "            }\n"
)
STEP_HEAD = (
    "        Some(LookupMode::Step) => {\n"
    "            if x.is_nan() {\n"
    "                findings.push(nan_table_argument(&subject_id));\n"
    "                return None;\n"
    "            }\n"
)
INTERP_HEAD = (
    "        None => {\n"
    "            if x.is_nan() {\n"
    "                findings.push(nan_table_argument(&subject_id));\n"
    "                return None;\n"
    "            }\n"
)

MUTANTS = {
    # Site 1: the same-dimension quotient.
    "R1_ratio_check_nan_only": [
        ("            if !ratio.is_finite() {\n", "            if ratio.is_nan() {\n")],
    "R2_overblock_dimensionless_divisor": [
        ("        (dim, Dimension::Dimensionless) => Some(EvaluationValue::Quantity(Quantity {\n",
         "        (_, Dimension::Dimensionless) if !(left.value / right.value).is_finite() => {\n"
         "            findings.push(EvaluationFinding::new(FindingCode::NonFiniteInput, \"divide\", "
         "\"same-dimension quotient (ratio) must be finite\"));\n"
         "            None\n"
         "        }\n"
         "        (dim, Dimension::Dimensionless) => Some(EvaluationValue::Quantity(Quantity {\n")],
    "R3_overblock_derived_quotient": [
        ("            DimensionQuotient::Unique(quotient) => Some(EvaluationValue::Quantity(Quantity {\n",
         "            DimensionQuotient::Unique(_) if !(left.value / right.value).is_finite() => {\n"
         "                findings.push(EvaluationFinding::new(FindingCode::NonFiniteInput, \"divide\", "
         "\"same-dimension quotient (ratio) must be finite\"));\n"
         "                None\n"
         "            }\n"
         "            DimensionQuotient::Unique(quotient) => Some(EvaluationValue::Quantity(Quantity {\n")],
    "R4_ratio_subject_changed": [
        ("                    FindingCode::NonFiniteInput,\n                    \"divide\",\n",
         "                    FindingCode::NonFiniteInput,\n                    \"ratio\",\n")],
    "R5_ratio_message_changed": [
        ("                    \"divide\",\n                    \"same-dimension quotient (ratio) must be finite\",\n",
         "                    \"divide\",\n                    \"quotient must be finite\",\n")],
    "R6_ratio_block_continues_with_zero": [
        (RATIO_BLOCK_HEAD,
         RATIO_BLOCK_HEAD.replace("                return None;\n",
                                  "                return Some(EvaluationValue::Quantity(ratio_quantity(0.0)));\n"))],
    # Site 2: the step lookup.
    "S1_step_subject_changed": [
        (STEP_HEAD, STEP_HEAD.replace("nan_table_argument(&subject_id)", "nan_table_argument(\"lookup\")"))],
    "S2_step_pushes_but_continues": [
        (STEP_HEAD, STEP_HEAD.replace("                return None;\n", ""))],
    # Site 3: the interpolation.
    "I1_interpolate_nan_silent_first_row": [
        (INTERP_HEAD, INTERP_HEAD.replace(
            "                findings.push(nan_table_argument(&subject_id));\n                return None;\n",
            "                return Some(EvaluationValue::Quantity(Quantity { value: table.rows[0].result, "
            "dimension: table.result_dimension, unit_ref: table.result_unit_ref.trim().to_string(), "
            "unit_required: true, dimension_check_required: true }));\n"))],
    "I2_interpolate_pushes_but_continues": [
        (INTERP_HEAD, INTERP_HEAD.replace("                return None;\n", ""))],
    # The shared NaN finding.
    "N1_nan_message_changed": [
        ("        \"table argument must be finite: a NaN argument is neither inside nor outside \\\n",
         "        \"table argument is out of range \\\n")],
}


def cargo(label, crate, args):
    env = dict(os.environ)
    env["CARGO_TARGET_DIR"] = str(WT / "targets/rv104-mut")
    env["TMPDIR"] = str(SCRATCH / "tmp")
    log = LOGS / f"{label}.log"
    with open(log, "w") as f:
        rc = subprocess.call([str(WT / "tools/t3_cargo.sh"), "test", "--offline", "--locked"] + args,
                             cwd=crate, env=env, stdout=f, stderr=subprocess.STDOUT)
    return rc, log.read_text()


def failed(log):
    return sorted(set(re.findall(r"^---- (\S+) stdout ----", log, re.M)))


results = {}
names = ["UNMUTATED"] + (ONLY or list(MUTANTS))
try:
    for name in names:
        text = ORIGINAL
        for old, new in MUTANTS.get(name, []):
            n = text.count(old)
            assert n == 1, (name, n, old[:80])
            text = text.replace(old, new)
        LIB.write_text(text)
        rc_lib, log_lib = cargo(f"{name}_ee_lib", P / "core/rules/expression_evaluator", ["--lib"])
        rc_run, log_run = cargo(f"{name}_rcr", P / "core/rules/rule_check_runner",
                                ["--test", "point_path_non_finite_run"])
        compiled = "error[E" not in log_lib and "error[E" not in log_run
        summary = re.findall(r"^test result: .*$", log_lib, re.M)
        results[name] = {
            "ee_lib_rc": rc_lib,
            "rcr_test_rc": rc_run,
            "compiled": compiled,
            "killed": None if name == "UNMUTATED" else (compiled and (rc_lib != 0 or rc_run != 0)),
            "failed_tests": failed(log_lib) + failed(log_run),
            "ee_lib_result": summary,
        }
        print(name, json.dumps(results[name]), flush=True)
finally:
    LIB.write_text(ORIGINAL)
    OUT.write_text(json.dumps(results, indent=1, sort_keys=True))
