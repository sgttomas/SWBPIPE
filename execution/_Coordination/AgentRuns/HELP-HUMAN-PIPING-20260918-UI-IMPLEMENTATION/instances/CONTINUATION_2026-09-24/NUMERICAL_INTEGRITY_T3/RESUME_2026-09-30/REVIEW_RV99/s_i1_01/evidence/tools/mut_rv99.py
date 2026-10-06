"""RV99 mutant runner: I73's 21 mutants (patch text read from I73's own
scripts) plus RV99's own. Each patch must apply exactly once to RV99's copy;
the file is restored afterwards and the restore is checked by sha256.

Kill criteria:
- evaluator (M*, V-evaluator): `cargo test --lib` in expression_evaluator fails
  (I73's criterion), and additionally the runner's full suite (which carries
  the shared-case and runner tests) is reported;
- runner (R*, V-runner): `cargo test --no-fail-fast` in rule_check_runner fails;
- Python (P*, PV*): `pytest tests/test_rule_interval.py` fails.
Independently, RV99's harness + checker (pass-1 cases) is run on every Rust
mutant and on every Python mutant (parity) to show whether RV99's own
oracle detects it.

usage: mut_rv99.py <I73 final run-records dir> [names...]
"""
from __future__ import annotations

import hashlib
import json
import os
import pathlib
import re
import subprocess
import sys

WT = pathlib.Path("<WT>")
S = WT / "scratch/rv99_s_i1_01"
C = WT / "rv99/projects/chirality-piping"
EE = "core/rules/expression_evaluator/src/lib.rs"
RUNNER = "core/rules/rule_check_runner/src/lib.rs"
PY = "core/analysis_runs/rule_interval.py"
VENV = "<VENV>"

rr = pathlib.Path(sys.argv[1])
os.environ.setdefault("WT", str(WT))
ns: dict = {}
exec(rr.joinpath("ee_mutants.py").read_text().split("\nonly = ")[0], ns)
I73_EE = {k: (EE, v) for k, v in ns["MUTANTS"].items()}
ns = {}
exec(rr.joinpath("rcr_py_mutants.py").read_text().split("\nonly = ")[0], ns)
I73_RP = dict(ns["MUTANTS"])

RV99 = {
    # evaluator: comparisons
    "V1_less_than_strictness_lost": (EE, [(
        "        ComparisonOperator::LessThan => {\n            if a.hi < b.lo {",
        "        ComparisonOperator::LessThan => {\n            if a.hi <= b.lo {")]),
    "V2_ge_true_uses_upper_end": (EE, [(
        "        ComparisonOperator::GreaterThanOrEqual => {\n            if a.lo >= b.hi {",
        "        ComparisonOperator::GreaterThanOrEqual => {\n            if a.hi >= b.hi {")]),
    "V3_equal_true_for_equal_ranges": (EE, [(
        "    if a.is_point() && b.is_point() && a.lo == b.lo {",
        "    if a.lo == b.lo && a.hi == b.hi {")]),
    "V4_kleene_false_or_unknown_is_false": (EE, [(
        "            (Truth::False, Truth::False) => Truth::False,\n            (Truth::False, Truth::Indeterminate)\n            | (Truth::Indeterminate, Truth::False)",
        "            (Truth::False, Truth::False) | (Truth::False, Truth::Indeterminate) => Truth::False,\n            (Truth::Indeterminate, Truth::False)")]),
    "V5_select_unknown_boolean_takes_then": (EE, [(
        "                        Some(IValue::Boolean(if then_truth == else_truth {\n                            then_truth\n                        } else {\n                            Truth::Indeterminate\n                        }))",
        "                        Some(IValue::Boolean(if then_truth == else_truth {\n                            then_truth\n                        } else {\n                            then_truth\n                        }))")]),
    "V6_step_governing_row_strict": (EE, [(
        "        if row.argument <= x {\n            index = candidate;",
        "        if row.argument < x {\n            index = candidate;")]),
    "V7_interpolation_first_segment_only": (EE, [(
        "        joined = Some(match joined {\n            Some(enclosure) => interval_hull(enclosure, segment),\n            None => segment,\n        });\n    }\n    joined",
        "        joined = Some(match joined {\n            Some(enclosure) => interval_hull(enclosure, segment),\n            None => segment,\n        });\n        break;\n    }\n    joined")]),
    "V8_bound_enclosure_not_outward": (EE, [(
        "    outward(value - bound, value + bound)\n}",
        "    Some(Enclosure {\n        lo: value - bound,\n        hi: value + bound,\n    })\n}")]),
    "V9_divisor_zero_end_not_refused": (EE, [(
        "    e.lo <= 0.0 && 0.0 <= e.hi",
        "    e.lo < 0.0 && 0.0 < e.hi")]),
    "V14_table_range_note_not_eager": (EE, [(
        "        value.map(|value| value.into_public(!notes.is_empty()))",
        "        value.map(|value| value.into_public(notes.iter().any(|n| n.code != IntervalNoteCode::TableArgumentRange)))")]),
    # runner
    "V10_runner_all_fail_reported_as_pass": (RUNNER, [(
        "        Truth::False => (\n            RuleCheckStatus::UserRuleFailed,\n            RULE_INTERVAL_ALL_FAIL,",
        "        Truth::False => (\n            RuleCheckStatus::UserRuleChecked,\n            RULE_INTERVAL_ALL_PASS,")]),
    "V11_runner_offset_step_inward": (RUNNER, [(
        "    let lo = down(lo - t.offset)?;",
        "    let lo = up(lo - t.offset)?;")]),
    "V12_runner_subnormal_bound_as_point": (RUNNER, [(
        "            Some(b) if b == 0.0 => (raw_value, raw_unit, note, None),",
        "            Some(b) if b < 1e-300 => (raw_value, raw_unit, note, None),")]),
    "V13_runner_limit_compare_uses_point": (RUNNER, [(
        "            enclosure: formula.enclosure,\n        }],",
        "            enclosure: formula.enclosure.map(|e| Enclosure::point(e.lo)),\n        }],")]),
    # python
    "PV1_python_less_than_strictness_lost": (PY, [(
        "        return TRUE if a[1] < b[0] else FALSE if a[0] >= b[1] else INDETERMINATE",
        "        return TRUE if a[1] <= b[0] else FALSE if a[0] >= b[1] else INDETERMINATE")]),
    "PV2_python_or_false_unknown_is_false": (PY, [(
        "    return FALSE if a == b == FALSE else INDETERMINATE",
        "    return FALSE if FALSE in (a, b) else INDETERMINATE")]),
}

ALL = {}
ALL.update(I73_EE)
ALL.update(I73_RP)
ALL.update(RV99)
env_base = dict(os.environ)


def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()


def cargo(label, crate, *args, extra_env=None):
    env = dict(env_base)
    if extra_env:
        env.update(extra_env)
    return subprocess.run([str(S / "rc.sh"), str(C / crate), "cand", label, *args], env=env).returncode


def harness(label):
    rc = cargo(label, "core/rules/rule_check_runner", "test", "--locked", "--offline", "--release",
               "--test", "rv99_harness",
               extra_env={"RV99_CASES": str(S / "rv99_cases.json"), "RV99_OUT": str(S / f"mut/{label}.json")})
    if rc != 0:
        return {"harness_rc": rc}
    out = subprocess.run([f"{VENV}/bin/python", "-B", str(S / "rv99_check.py"), str(S / "rv99_cases.json"),
                          str(S / f"mut/{label}.json"), str(C / "core/analysis_runs")],
                         capture_output=True, text=True).stdout
    m = re.search(r"VIOLATIONS: (\{.*\})", out)
    viol = eval(m.group(1)) if m else {"checker": "no output"}
    for k in ("runner_note_format", "runner_zero_bound_differs"):  # pass-1 baseline artefacts
        viol.pop(k, None)
    os.remove(S / f"mut/{label}.json")
    return {"rv99_violations": viol}


names = sys.argv[2:] or list(ALL)
(S / "mut").mkdir(exist_ok=True)
results = {}
resfile = S / "mut/results.json"
if resfile.exists():
    results = json.loads(resfile.read_text())
for name in names:
    rel, patches = ALL[name]
    path = C / rel
    before = sha(path)
    original = path.read_text()
    text = original
    for old, new in patches:
        n = text.count(old)
        assert n == 1, (name, n, old[:70])
        text = text.replace(old, new)
    path.write_text(text)
    r = {"file": rel}
    try:
        if rel == EE:
            rc = cargo(f"mut_{name}_lib", "core/rules/expression_evaluator", "test", "--locked", "--offline", "--lib")
            log = (S / f"logs/mut_{name}_lib.txt").read_text()
            r["suite"] = "expression_evaluator --lib"
            r["compiled"] = "error[" not in log
            r["killed"] = rc != 0 and r["compiled"]
            r["failed_tests"] = sorted(set(re.findall(r"^test (\S+) \.\.\. FAILED", log, re.M)))
            rc2 = cargo(f"mut_{name}_runner", "core/rules/rule_check_runner", "test", "--locked", "--offline", "--no-fail-fast")
            log2 = (S / f"logs/mut_{name}_runner.txt").read_text()
            r["runner_suite_failed_tests"] = sorted(set(re.findall(r"^test (\S+) \.\.\. FAILED", log2, re.M)))
            r.update(harness(f"h_{name}"))
        elif rel == RUNNER:
            rc = cargo(f"mut_{name}", "core/rules/rule_check_runner", "test", "--locked", "--offline", "--no-fail-fast")
            log = (S / f"logs/mut_{name}.txt").read_text()
            r["suite"] = "rule_check_runner (all tests)"
            r["compiled"] = "error[" not in log
            r["killed"] = rc != 0 and r["compiled"]
            r["failed_tests"] = sorted(set(re.findall(r"^test (\S+) \.\.\. FAILED", log, re.M)))
            r.update(harness(f"h_{name}"))
        else:
            env = dict(env_base, PYTHONDONTWRITEBYTECODE="1", OPENPIPESTRESS_CHECKED_JSON_BIN=str(S / "no_such_bin"),
                       OPENPIPESTRESS_UNITS_BIN=str(S / "no_such_bin"), TMPDIR=str(S / "tmp"), PATH="/usr/bin:/bin")
            p = subprocess.run([f"{VENV}/bin/python", "-B", "-m", "pytest", "-q", "-p", "no:cacheprovider",
                                "tests/test_rule_interval.py"], cwd=C, env=env, capture_output=True, text=True)
            (S / f"logs/mut_{name}.txt").write_text(p.stdout + p.stderr)
            r["suite"] = "pytest tests/test_rule_interval.py"
            r["compiled"] = "SyntaxError" not in p.stdout + p.stderr
            r["killed"] = p.returncode != 0 and r["compiled"]
            r["failed_tests"] = sorted(set(re.findall(r"^FAILED (\S+)", p.stdout, re.M)))[:12]
            r["failed_count"] = len(set(re.findall(r"^FAILED (\S+)", p.stdout, re.M)))
            # RV99 parity check against the (unmutated) Rust pass-1 output
            out = subprocess.run([f"{VENV}/bin/python", "-B", str(S / "rv99_check.py"), str(S / "rv99_cases.json"),
                                  str(S / "rv99_out.json"), str(C / "core/analysis_runs"), "--no-exact"],
                                 capture_output=True, text=True).stdout
            m = re.search(r"VIOLATIONS: (\{.*\})", out)
            viol = eval(m.group(1)) if m else {"checker": "no output"}
            for k in ("runner_note_format", "runner_zero_bound_differs"):
                viol.pop(k, None)
            r["rv99_violations"] = viol
    finally:
        path.write_text(original)
    r["restored"] = sha(path) == before
    results[name] = r
    resfile.write_text(json.dumps(results, indent=1))
    print(name, "killed=", r.get("killed"), "n_failed=", len(r.get("failed_tests", [])),
          "rv99=", r.get("rv99_violations"), "restored=", r["restored"], flush=True)
