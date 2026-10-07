"""I79 T3-SI1b mutants. Each patch must apply exactly once to the candidate
`expression_evaluator/src/lib.rs` (head da0758064e). A mutant is killed when
the evaluator's `cargo test --lib` fails (it must still compile); the
rule_check_runner regression file `point_path_non_finite_run` is also run
against every mutant and its outcome recorded. One mutant tree is patched,
run and restored in turn. Scratch harness; results go to the RETURN."""
import json, os, pathlib, re, shutil, subprocess, sys

WT = pathlib.Path(os.environ["WT"])
S = WT / "scratch/i79_si1b"
CAND = S / os.environ.get("I79_CAND_TREE", "cand") / "projects/chirality-piping"
RESULTS = S / os.environ.get("I79_MUT_RESULTS", "mut_results.json")
MUT = S / "mut/projects/chirality-piping"
TARGET = WT / "targets/i79-si1b-mut"

QUOTIENT = "            let ratio = left.value / right.value;\n            if !ratio.is_finite() {\n"
STEP = "        Some(LookupMode::Step) => {\n            if x.is_nan() {\n"
INTERP = "        None => {\n            if x.is_nan() {\n"
MUTANTS = {
    # Site 1: the same-dimension quotient.
    "Q1_quotient_check_removed": [(QUOTIENT, "            let ratio = left.value / right.value;\n            if false {\n")],
    "Q2_quotient_check_infinite_only": [(QUOTIENT, "            let ratio = left.value / right.value;\n            if ratio.is_infinite() {\n")],
    "Q3_quotient_code_division_by_zero": [(
        "                    FindingCode::NonFiniteInput,\n                    \"divide\",",
        "                    FindingCode::DivisionByZero,\n                    \"divide\",")],
    "Q4_quotient_check_before_unit_check": [(
        "            if !quantity_units_match(&left, &right) {\n                findings.push(unit_mismatch(\"divide\", &left, &right));\n                return None;\n            }\n            // The ratio quantity",
        "            if !(left.value / right.value).is_finite() {\n                findings.push(EvaluationFinding::new(FindingCode::NonFiniteInput, \"divide\", \"same-dimension quotient (ratio) must be finite\"));\n                return None;\n            }\n            if !quantity_units_match(&left, &right) {\n                findings.push(unit_mismatch(\"divide\", &left, &right));\n                return None;\n            }\n            // The ratio quantity")],
    # Site 2: the step lookup.
    "S1_step_nan_check_removed": [(STEP, "        Some(LookupMode::Step) => {\n            if false {\n")],
    "S2_step_check_all_non_finite": [(STEP, "        Some(LookupMode::Step) => {\n            if !x.is_finite() {\n")],
    # Site 3: the interpolation.
    "I1_interpolate_nan_check_removed": [(INTERP, "        None => {\n            if false {\n")],
    "I2_interpolate_check_all_non_finite": [(INTERP, "        None => {\n            if !x.is_finite() {\n")],
    # The shared table finding, and the exact lookup it must not reach.
    "T1_nan_argument_code_out_of_range": [(
        "    EvaluationFinding::new(\n        FindingCode::NonFiniteInput,\n        subject_id,\n        \"table argument must be finite",
        "    EvaluationFinding::new(\n        FindingCode::TableOutOfRange,\n        subject_id,\n        \"table argument must be finite")],
    # RV104's three survivors (its `rv104_mutants.py`, patches verbatim), added
    # in repair round 01: they must now be killed.
    "RV104_R2_overblock_dimensionless_divisor": [(
        "        (dim, Dimension::Dimensionless) => Some(EvaluationValue::Quantity(Quantity {\n",
        "        (_, Dimension::Dimensionless) if !(left.value / right.value).is_finite() => {\n"
        "            findings.push(EvaluationFinding::new(FindingCode::NonFiniteInput, \"divide\", "
        "\"same-dimension quotient (ratio) must be finite\"));\n"
        "            None\n"
        "        }\n"
        "        (dim, Dimension::Dimensionless) => Some(EvaluationValue::Quantity(Quantity {\n")],
    "RV104_R3_overblock_derived_quotient": [(
        "            DimensionQuotient::Unique(quotient) => Some(EvaluationValue::Quantity(Quantity {\n",
        "            DimensionQuotient::Unique(_) if !(left.value / right.value).is_finite() => {\n"
        "                findings.push(EvaluationFinding::new(FindingCode::NonFiniteInput, \"divide\", "
        "\"same-dimension quotient (ratio) must be finite\"));\n"
        "                None\n"
        "            }\n"
        "            DimensionQuotient::Unique(quotient) => Some(EvaluationValue::Quantity(Quantity {\n")],
    "RV104_R6_ratio_block_continues_with_zero": [(
        "                    \"same-dimension quotient (ratio) must be finite\",\n                ));\n                return None;\n",
        "                    \"same-dimension quotient (ratio) must be finite\",\n                ));\n                return Some(EvaluationValue::Quantity(ratio_quantity(0.0)));\n")],
    "T2_exact_lookup_also_blocks_nan": [(
        "        Some(LookupMode::Exact) => {\n",
        "        Some(LookupMode::Exact) => {\n            if x.is_nan() {\n                findings.push(nan_table_argument(&subject_id));\n                return None;\n            }\n")],
}

def run(label, crate, *args):
    log = S / "logs" / f"{label}.log"
    rc = subprocess.run([str(S / "cargo_run.sh"), label, str(crate), str(TARGET), *args]).returncode
    text = log.read_text()
    return rc, text

if MUT.exists():
    shutil.rmtree(S / "mut")
MUT.mkdir(parents=True)
for part in ("core", "examples"):
    shutil.copytree(CAND / part, MUT / part)
lib = MUT / "core/rules/expression_evaluator/src/lib.rs"
original = lib.read_text()
assert original == (CAND / "core/rules/expression_evaluator/src/lib.rs").read_text()

results = {}
only = sys.argv[1:] or list(MUTANTS)
for name in ["UNMUTATED"] + only:
    text = original
    for old, new in MUTANTS.get(name, []):
        n = text.count(old)
        assert n == 1, (name, n, old[:70])
        text = text.replace(old, new)
    lib.write_text(text)
    try:
        rc_lib, log_lib = run(f"mut_{name}_ee_lib", MUT / "core/rules/expression_evaluator",
                              "test", "--offline", "--locked", "--lib")
        rc_run, log_run = run(f"mut_{name}_rcr", MUT / "core/rules/rule_check_runner",
                              "test", "--offline", "--locked", "--test", "point_path_non_finite_run")
    finally:
        lib.write_text(original)
    compiled = "error[" not in log_lib and "error[" not in log_run
    failed = sorted(set(re.findall(r"^test (\S+) \.\.\. FAILED", log_lib + log_run, re.M)))
    passed_line = re.findall(r"^test result: .*$", log_lib, re.M)
    results[name] = {"ee_lib_rc": rc_lib, "rcr_regression_rc": rc_run, "compiled": compiled,
                     "killed": (rc_lib != 0) and compiled if name != "UNMUTATED" else None,
                     "failed_tests": failed, "ee_lib_result": passed_line[:1]}
    print(name, json.dumps(results[name]), flush=True)
RESULTS.write_text(json.dumps(results, indent=1))
