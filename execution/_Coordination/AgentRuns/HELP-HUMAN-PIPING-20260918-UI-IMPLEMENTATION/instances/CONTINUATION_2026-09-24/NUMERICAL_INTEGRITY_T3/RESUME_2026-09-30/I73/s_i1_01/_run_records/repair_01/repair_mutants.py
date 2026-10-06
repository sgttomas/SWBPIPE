"""I73 S-I1 repair round 01: every mutant on the repaired candidate.

I73's 21 (patch texts from ee_mutants.py and rcr_py_mutants.py; R4's two
anchors re-pointed at the repaired `Some(Some(b))` arms, same semantics),
RV99's 16 (patch texts from RV99's mut_rv99.py; V12 re-pointed the same way),
and the repair round's own R6, R7, P6, P7. Each patch must apply exactly once.
Kill criteria (I73's and RV99's): evaluator mutants fail `cargo test --lib` in
expression_evaluator; runner mutants fail the full runner suite; Python mutants
fail tests/test_rule_interval.py. V3 and V9 are also run against the runner's
shared-case test. Each file is restored after its run."""
import json, os, pathlib, re, shutil, subprocess, sys
WT = pathlib.Path(os.environ["WT"])
S = WT / "scratch/i73_s_i1"
CAND = S / "cand/projects/chirality-piping"
EE = "core/rules/expression_evaluator/src/lib.rs"
RUNNER = "core/rules/rule_check_runner/src/lib.rs"
PY = "core/analysis_runs/rule_interval.py"

def load(path, split):
    ns = {"__name__": "loaded", "os": os, "pathlib": pathlib}
    os.environ.setdefault("WT", str(WT))
    src = path.read_text().split(split)[0]
    exec(src, ns)
    return ns["MUTANTS"]

ee = load(S / "mut/ee_mutants.py", "\nonly = ")
rp = load(S / "mut/rcr_py_mutants.py", "\nonly = ")
MUTANTS = {}
for name, patches in ee.items():
    MUTANTS[name] = (EE, patches)
for name, (rel, patches) in rp.items():
    MUTANTS[name] = (rel, patches)
# R4 re-pointed at the repaired arms (same semantics: invalid b accepted, bound as a point).
MUTANTS["R4_invalid_bound_bound_as_point"] = (RUNNER, [(
    "            Some(Some(b)) if !(b.is_finite() && b >= 0.0) => {",
    "            Some(Some(b)) if false && !(b.is_finite() && b >= 0.0) => {"), (
    "            Some(Some(b)) if b == 0.0 => (raw_value, raw_unit, note, None),",
    "            Some(Some(b)) if !(b > 0.0) || !b.is_finite() => (raw_value, raw_unit, note, None),")])
rv = {}
exec((S / "mut/rv99_mutants_patches.py").read_text(), rv)
for name, (which, patches) in rv["RV99"].items():
    MUTANTS[name] = ({"EE": EE, "RUNNER": RUNNER, "PY": PY}[which], patches)
MUTANTS["V12_runner_subnormal_bound_as_point"] = (RUNNER, [(
    "            Some(Some(b)) if b == 0.0 => (raw_value, raw_unit, note, None),",
    "            Some(Some(b)) if b < 1e-300 => (raw_value, raw_unit, note, None),")])
# The repair round's own.
MUTANTS["R6_downgraded_check_keeps_interval_code"] = (RUNNER, [(
    "    let downgraded = status != RuleCheckStatus::RuleInputsIncomplete\n",
    "    let downgraded = false && status != RuleCheckStatus::RuleInputsIncomplete\n")])
MUTANTS["R7_duplicate_bounds_last_wins"] = (RUNNER, [(
    "                    .and_modify(|entry| *entry = None)",
    "                    .and_modify(|entry| *entry = Some(b.absolute_bound))")])
MUTANTS["P6_python_invalid_bound_as_point"] = (PY, [(
    '        elif item.get("bound") is not None and item["bound"] != 0.0:',
    '        elif item.get("bound") is not None and item["bound"] > 0.0:')])
MUTANTS["P7_python_enclosure_unvalidated"] = (PY, [(
    "            if not (math.isfinite(lo) and math.isfinite(hi)):\n                findings.append((\"NonFiniteInput\", variable_id))",
    "            if False:\n                findings.append((\"NonFiniteInput\", variable_id))"), (
    "            if lo > hi:\n                findings.append((\"InvalidReference\", variable_id))",
    "            if False:\n                findings.append((\"InvalidReference\", variable_id))")])

def run(name, rel, args_kind):
    if rel == PY:
        return subprocess.run([str(S / "run_py.sh"), f"rep_mut_{name}", str(CAND), "tests/test_rule_interval.py"],
                              stdout=subprocess.DEVNULL).returncode, f"rep_mut_{name}"
    if rel == EE and args_kind == "lib":
        crate, extra, label = "core/rules/expression_evaluator", ["--lib"], f"rep_mut_{name}"
    elif rel == EE and args_kind == "cases":
        crate, extra, label = "core/rules/rule_check_runner", ["--test", "rule_interval_cases"], f"rep_mut_{name}_cases"
    else:
        crate, extra, label = "core/rules/rule_check_runner", ["--no-fail-fast"], f"rep_mut_{name}"
    rc = subprocess.run([str(S / "cargo_run.sh"), label, str(CAND / crate), str(WT / "targets/i73-s-i1"),
                         "test", "--locked", "--offline", *extra]).returncode
    return rc, label

only = sys.argv[1:] or list(MUTANTS)
results = {}
for name in only:
    rel, patches = MUTANTS[name]
    path = CAND / rel
    original = path.read_text()
    text = original
    for old, new in patches:
        assert text.count(old) == 1, (name, text.count(old), old[:70])
        text = text.replace(old, new)
    path.write_text(text)
    try:
        kinds = ["lib", "cases"] if name.startswith(("V3_", "V9_")) else ["lib"]
        entry = {}
        for kind in kinds:
            rc, label = run(name, rel, kind)
            log = (S / "logs" / f"{label}.log").read_text()
            compiled = "error[" not in log and "SyntaxError" not in log
            failed = sorted(set(re.findall(r"^test (\S+) \.\.\. FAILED", log, re.M)) |
                            set(re.findall(r"^FAILED (\S+)", log, re.M)))
            entry[kind] = {"exit": rc, "compiled": compiled, "killed": rc != 0 and compiled,
                           "failed_count": len(failed), "failed_tests": failed[:10]}
    finally:
        path.write_text(original)
    results[name] = entry
    print(name, {k: (v["killed"], v["failed_count"]) for k, v in entry.items()}, flush=True)
(S / "mut" / "repair_results.json").write_text(json.dumps(results, indent=1))
