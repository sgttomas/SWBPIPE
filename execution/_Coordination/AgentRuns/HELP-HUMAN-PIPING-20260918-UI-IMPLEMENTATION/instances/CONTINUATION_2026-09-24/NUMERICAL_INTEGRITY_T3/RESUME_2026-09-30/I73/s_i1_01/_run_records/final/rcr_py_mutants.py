"""I73 S-I1 runner and Python mutants. Each patch applies exactly once to the
scratch candidate tree, the named suite runs, and the file is restored.
A mutant is killed when the suite fails (and the build compiled)."""
import json, os, pathlib, re, subprocess, sys
WT = pathlib.Path(os.environ["WT"])
S = WT / "scratch/i73_s_i1"
CAND = S / "cand/projects/chirality-piping"
RUNNER = "core/rules/rule_check_runner/src/lib.rs"
PY = "core/analysis_runs/rule_interval.py"
MUTANTS = {
  # --- runner (brief control 3c and extras) ---
  "R3c_indeterminate_collapsed_to_pass": (RUNNER, [(
    "        Truth::Indeterminate => (\n            RuleCheckStatus::RuleInputsIncomplete,",
    "        Truth::Indeterminate => (\n            RuleCheckStatus::UserRuleChecked,")]),
  "R2_interval_mode_bypassed": (RUNNER, [(
    "    if !intervals.is_empty() {\n        let formula_input = EvaluationInput {",
    "    if false && !intervals.is_empty() {\n        let formula_input = EvaluationInput {")]),
  "R3_unit_conversion_not_outward": (RUNNER, [(
    "    let down = |x: f64| -> Option<f64> {\n        let y = x.next_down();",
    "    let down = |x: f64| -> Option<f64> {\n        let y = x;"), (
    "    let up = |x: f64| -> Option<f64> {\n        let y = x.next_up();",
    "    let up = |x: f64| -> Option<f64> {\n        let y = x;")]),
  "R4_invalid_bound_bound_as_point": (RUNNER, [(
    "            Some(b) if !(b.is_finite() && b >= 0.0) => {",
    "            Some(b) if false && !(b.is_finite() && b >= 0.0) => {"), (
    "            Some(b) if b == 0.0 => (raw_value, raw_unit, note, None),",
    "            Some(b) if !(b > 0.0) || !b.is_finite() => (raw_value, raw_unit, note, None),")]),
  "R5_formula_enclosure_collapsed_in_compare": (RUNNER, [(
    "            variable_id: FORMULA_ENCLOSURE_VARIABLE.to_string(),\n            enclosure: formula.enclosure,",
    "            variable_id: FORMULA_ENCLOSURE_VARIABLE.to_string(),\n            enclosure: formula.enclosure.map(|e| Enclosure::point(e.hi)),")]),
  # --- Python reference ---
  "P1_python_outward_step_removed": (PY, [(
    "    lo, hi = math.nextafter(lo, -math.inf), math.nextafter(hi, math.inf)\n",
    "    lo, hi = lo, hi\n")]),
  "P2_python_compare_U_arm_to_T": (PY, [(
    "        return TRUE if a[1] <= b[0] else FALSE if a[0] > b[1] else INDETERMINATE",
    "        return TRUE if a[1] <= b[0] else FALSE if a[0] > b[1] else TRUE")]),
  "P3_python_interpolation_point_hull": (PY, [(
    "        segment = _segment(a0, r0, a1, r1, (_max2(x[0], a0), _min2(x[1], a1)))",
    "        def at(v):\n            return r0 if v == a0 else r1 if v == a1 else r0 + (r1 - r0) * ((v - a0) / (a1 - a0))\n        lo_, hi_ = at(_max2(x[0], a0)), at(_min2(x[1], a1))\n        segment = _outward(_min2(lo_, hi_), _max2(lo_, hi_))")]),
  "P4_python_eager_u_removed": (PY, [(
    "        indeterminate = bool(state.notes)\n",
    "        indeterminate = False\n")]),
  "P5_python_abs_straddle_inward": (PY, [(
    "    return (0.0, _max2(-lo, hi))",
    "    return (_min2(-lo, hi), _max2(-lo, hi))")]),
}
only = sys.argv[1:] or list(MUTANTS)
results = {}
for name in only:
    rel, patches = MUTANTS[name]
    path = CAND / rel
    original = path.read_text()
    text = original
    for old, new in patches:
        assert text.count(old) == 1, (name, text.count(old), old[:60])
        text = text.replace(old, new)
    path.write_text(text)
    try:
        if rel == RUNNER:
            rc = subprocess.run([str(S / "cargo_run.sh"), f"mut_{name}", str(CAND / "core/rules/rule_check_runner"),
                                 str(WT / "targets/i73-s-i1"), "test", "--locked", "--offline", "--no-fail-fast"]).returncode
        else:
            rc = subprocess.run([str(S / "run_py.sh"), f"mut_{name}", str(CAND), "tests/test_rule_interval.py"],
                                stdout=subprocess.DEVNULL).returncode
    finally:
        path.write_text(original)
    log = (S / "logs" / f"mut_{name}.log").read_text()
    compiled = "error[" not in log and "SyntaxError" not in log
    failed = sorted(set(re.findall(r"^test (\S+) \.\.\. FAILED", log, re.M)) |
                    set(re.findall(r"^FAILED (\S+)", log, re.M)))
    results[name] = {"exit": rc, "compiled": compiled, "killed": rc != 0 and compiled,
                     "failed_tests": failed[:12], "failed_count": len(failed)}
    print(name, results[name]["killed"], results[name]["failed_count"], flush=True)
(S / "mut" / "rcr_py_results.json").write_text(json.dumps(results, indent=1))
