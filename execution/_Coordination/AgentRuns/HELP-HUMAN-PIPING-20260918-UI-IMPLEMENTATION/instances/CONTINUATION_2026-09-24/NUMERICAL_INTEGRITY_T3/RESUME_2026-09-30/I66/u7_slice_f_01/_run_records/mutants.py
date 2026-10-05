#!/usr/bin/env python3
"""I66 U7 slice F: mutants (scratch lane WT/scratch/i66_u7f/mut only). Each mutant replaces one
exact, unique snippet of a candidate file (code or the shared case file), runs the owning
suites, and restores the file. A kill is a failing test only; a compile or syntax error is
reported, never counted. Kinds: "py" (the Python retained suites), "rs" (the two Rust
retained test targets), "data" (the case file: Python and Rust carrier consumers both run).
Args: label phase [ids...]   phase: py | rs | all"""
import json, py_compile, subprocess, sys, time
WT = "WT"
S = f"{WT}/scratch/i66_u7f"
CAND = f"{WT}/f2a-u7/projects/chirality-piping"
# The Rust phase uses its own lane, so it can run beside the Python phase without sharing a file.
MUT = f"{S}/mut_rs/projects/chirality-piping" if len(sys.argv) > 2 and sys.argv[2] == "rs" else f"{S}/mut/projects/chirality-piping"
VENV = "REPO_ROOT/projects/chirality-piping/.venv"
TARGET = f"{WT}/targets/i66-u7f/mut"
PYR = "core/analysis_runs/retained_precision.py"
CO = "core/analysis_runs/compatibility.py"
RSR = "core/reporting/result_export/src/retained_precision.rs"
SC = "core/reporting/result_export/src/semantic_contract.rs"
CASES = "fixtures/results/retained_precision_carrier_cases.json"
PY_TESTS = "tests/test_retained_precision_carriers.py tests/test_retained_precision_contract.py"
PY_CARRIERS = "tests/test_retained_precision_carriers.py"
D676 = (" A numerically_eligible standing is a property of the supplied statement and its actual invocation, as the accepted "
        "reader checks them: no carrier authenticates producer origin, and none claims that a registered producer made the "
        "statement (ROOT_RULINGS_V1 \\\"RV93 on U3 grant 2: PASS; RV89 confirms the final basis re-qualified; U7 planned and ruled\\\", D-U7-6).")

M = [
    # The flags, each reverted alone.
    ("F1_py_flag_off", "py", PYR, "_IMPLEMENTATION_COMPLETE = True", "_IMPLEMENTATION_COMPLETE = False"),
    ("F2_rs_flag_off", "rs", RSR, "const IMPLEMENTATION_COMPLETE: bool = true;", "const IMPLEMENTATION_COMPLETE: bool = false;"),
    # Each reader eligibility condition dropped.
    ("C1_py_no_invocation_conjunct", "py", PYR, "eligible=_IMPLEMENTATION_COMPLETE and invocation is not None and snapshot", "eligible=_IMPLEMENTATION_COMPLETE and snapshot"),
    ("C2_py_no_mechanics_conjunct", "py", PYR, " and snapshot[\"status\"][\"mechanics\"]==\"MECHANICS_SOLVED\" and all(", " and all("),
    ("C3_py_no_case_status_conjunct", "py", PYR, " and all(c[\"status\"] in (\"selected\",\"not_required\") for c in cases)\n", "\n"),
    ("C4_rs_no_invocation_conjunct", "rs", RSR, "    let eligible = IMPLEMENTATION_COMPLETE\n        && actual_invocation.is_some()\n", "    let eligible = IMPLEMENTATION_COMPLETE\n"),
    ("C5_rs_no_mechanics_conjunct", "rs", RSR, "        && source[\"status\"][\"mechanics\"] == \"MECHANICS_SOLVED\"\n        && list(&body[\"cases\"])", "        && list(&body[\"cases\"])"),
    ("C6_rs_no_case_status_conjunct", "rs", RSR, ".all(|c| matches!(text(&c[\"status\"]), \"selected\" | \"not_required\"));", ".all(|_| true);"),
    # The carriers' use of the reader's eligibility dropped.
    ("C7_py_carrier_ignores_reader_eligibility", "py", CO, " or not validation[\"numerical_eligible\"] or list(", " or list("),
    ("C8_rs_carrier_ignores_reader_eligibility", "rs", SC, "!validation.numerical_eligible", "false"),
    # D-U7-6 and D-U7-4 in the shared file, and the v4 fields.
    ("D1_d_u7_6_sentence_removed", "data", CASES, D676, ""),
    ("D2_d_u7_4_entry_removed", "data", CASES, None, None),
    ("D3_capture_field_removed", "data", CASES, "          \"capture\": \"none\",\n", ""),
    ("D4_capture_value_other", "data", CASES, "          \"capture\": \"none\",\n", "          \"capture\": \"live\",\n"),
    ("D5_current_model_edit_path_absent", "data", CASES, "\"position\",\n                \"x\"", "\"position\",\n                \"w\""),
    ("D6_unknown_form_field", "data", CASES, "          \"capture\": \"none\",\n", "          \"capture\": \"none\",\n          \"registered\": false,\n"),
    ("D7_d_u7_4_rust_side_needs_recompute", "data", CASES, None, None),
]


def special(mid, raw):
    data = json.loads(raw)
    if mid == "D2_d_u7_4_entry_removed":
        data["declared_differences"] = [e for e in data["declared_differences"] if not e["id"].startswith("D-U7-4")]
    elif mid == "D7_d_u7_4_rust_side_needs_recompute":
        entry = next(e for e in data["declared_differences"] if e["id"].startswith("D-U7-4"))
        for form in entry["forms"]:
            form["expected"]["rust"]["standing"] = "needs_recompute"
            form["expected"]["python"]["standing"] = "needs_recompute"
    return json.dumps(data, indent=2) + "\n"


def guard():
    if subprocess.run("pgrep -f memguard.sh", shell=True, capture_output=True).returncode != 0:
        print("MEMGUARD NOT RUNNING"); sys.exit(9)


def run_py(tests):
    r = subprocess.run(f"cd {MUT} && PYTHONDONTWRITEBYTECODE=1 OPENPIPESTRESS_CHECKED_JSON_BIN={WT}/targets/i52-readers/canonical_json/release/openpipestress_jcs_ijson "
                       f"OPENPIPESTRESS_UNITS_BIN={WT}/targets/i52-readers/units/release/openpipestress_units perl -e 'alarm shift; exec @ARGV' 1800 "
                       f"{VENV}/bin/python -m pytest -q -p no:cacheprovider -x {tests}", shell=True, capture_output=True, text=True)
    failed = [l.split(" ")[1] for l in r.stdout.splitlines() if l.startswith("FAILED ")]
    status = "KILLED" if r.returncode == 1 and failed else "SURVIVED" if r.returncode == 0 else f"ERROR_{r.returncode}"
    return status, failed[:3], (r.stdout.strip().splitlines() or [""])[-1]


def run_rs(tests):
    r = subprocess.run(f"cd {MUT}/core/reporting/result_export && env -u RUSTFLAGS CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 TMPDIR={S}/tmp perl -e 'alarm shift; exec @ARGV' 1800 "
                       f"cargo test --locked --offline --manifest-path {MUT}/core/reporting/result_export/Cargo.toml --target-dir {TARGET} {tests}",
                       shell=True, capture_output=True, text=True)
    text = r.stdout + r.stderr
    failed = [l.split(" ")[1] for l in text.splitlines() if l.startswith("test ") and l.endswith("FAILED")]
    compile_error = "could not compile" in text or "error[E" in text
    status = "COMPILE_ERROR" if compile_error else "KILLED" if failed else "SURVIVED" if r.returncode == 0 else f"ERROR_{r.returncode}"
    return status, failed[:3], [l for l in text.splitlines() if "test result" in l][-1:]


def main(label, phase, only):
    out = []
    for mid, kind, name, old, new in M:
        if only and mid not in only:
            continue
        if phase != "all" and not (kind == phase or (phase == "rs" and kind == "data") or (phase == "py" and kind == "data")):
            continue
        guard()
        raw = open(f"{CAND}/{name}").read()
        if old is None:
            mutated = special(mid, raw)
        else:
            if raw.count(old) != 1:
                out.append({"id": mid, "status": "NOT_APPLIED", "count": raw.count(old)}); print(json.dumps(out[-1]), flush=True); continue
            mutated = raw.replace(old, new)
        assert mutated != raw, mid
        target = f"{MUT}/{name}"
        open(target, "w").write(mutated)
        t = time.time()
        try:
            if kind == "py":
                py_compile.compile(target, cfile=f"{S}/tmp/mut_compile.pyc", doraise=True)
                result = {"py": run_py(PY_TESTS)}
            elif kind == "rs":
                result = {"rs": run_rs("--test retained_precision_carriers --test retained_precision_contract")}
            elif phase == "py":
                result = {"py": run_py(PY_CARRIERS)}
            else:
                result = {"rs": run_rs("--test retained_precision_carriers")}
        except py_compile.PyCompileError as error:
            result = {"py": ("SYNTAX_ERROR", [], str(error)[:200])}
        finally:
            open(target, "w").write(raw)
        out.append({"id": mid, "kind": kind, "file": name, "phase": phase,
                    **{lang: {"status": r[0], "killed_by": r[1], "summary": r[2]} for lang, r in result.items()}, "seconds": round(time.time() - t, 1)})
        print(json.dumps(out[-1]), flush=True)
    for _, _, name, _, _ in M:
        assert open(f"{CAND}/{name}").read() == open(f"{MUT}/{name}").read(), name
    json.dump(out, open(f"{S}/logs/mutants_{label}.json", "w"), indent=1)


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2], set(sys.argv[3:]))
