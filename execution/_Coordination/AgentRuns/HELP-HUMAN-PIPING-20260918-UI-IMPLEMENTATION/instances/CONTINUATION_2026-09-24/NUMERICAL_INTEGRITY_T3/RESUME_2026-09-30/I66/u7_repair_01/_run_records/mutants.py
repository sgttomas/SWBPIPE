#!/usr/bin/env python3
"""I66 U7 repair (RV94 S-1): mutants of the aligned summary (scratch lanes only). Each mutant
replaces one exact, unique snippet, runs the owning suite, and restores the file. A kill is a
failing test only; a compile or syntax error is reported, never counted.
Args: label phase(py|rs) [ids...]"""
import json, py_compile, subprocess, sys, time
WT = "WT"
S = f"{WT}/scratch/i66_u7r"
CAND = f"{WT}/f2a-u7/projects/chirality-piping"
VENV = "REPO_ROOT/projects/chirality-piping/.venv"
TARGET = f"{WT}/targets/i66-u7f/mut-repair"
CO = "core/analysis_runs/compatibility.py"
SC = "core/reporting/result_export/src/semantic_contract.rs"
M = [
    # Python: the condition removed (the statement's own case order decides, as the invocation's own cases did before), always Current, never Current.
    ("P1_py_condition_removed_own_cases", "py", CO,
     "def _classification_summary_from(validation: Mapping[str, Any], source: Mapping[str, Any], requested_basis_refs: list[Mapping[str, str]]) -> list[dict[str, Any]]:",
     "def _classification_summary_from(validation: Mapping[str, Any], source: Mapping[str, Any], requested_basis_refs: list[Mapping[str, str]]) -> list[dict[str, Any]]:\n"
     "    requested_basis_refs = [case[\"basis_ref\"] for case in source[\"retained_precision\"][\"body\"][\"cases\"]]"),
    ("P2_py_always_current", "py", CO,
     "    current = _retained_standing_from(validation, source, requested_basis_refs) == \"numerically_eligible\"",
     "    current = True"),
    ("P3_py_never_current", "py", CO,
     "    current = _retained_standing_from(validation, source, requested_basis_refs) == \"numerically_eligible\"",
     "    current = False"),
    ("P4_py_refs_dropped_at_entry", "py", CO,
     "    return _classification_summary_from(validation, source, requested_basis_refs or [])",
     "    return _classification_summary_from(validation, source, [])"),
    # Rust: the same.
    ("R1_rs_condition_removed_own_cases", "rs", SC,
     "    let current =\n        retained_standing_from(validation, source, requested_basis_refs) == \"numerically_eligible\";",
     "    let own: Vec<Value> = source[\"retained_precision\"][\"body\"][\"cases\"].as_array().unwrap().iter().map(|c| c[\"basis_ref\"].clone()).collect();\n"
     "    let current = retained_standing_from(validation, source, &own) == \"numerically_eligible\";"),
    ("R2_rs_always_current", "rs", SC,
     "    let current =\n        retained_standing_from(validation, source, requested_basis_refs) == \"numerically_eligible\";",
     "    let current = true;"),
    ("R3_rs_never_current", "rs", SC,
     "    let current =\n        retained_standing_from(validation, source, requested_basis_refs) == \"numerically_eligible\";",
     "    let current = false;"),
    ("R4_rs_refs_dropped_at_entry", "rs", SC,
     "        Ok(validation) => classification_summary_from(&validation, source, requested_basis_refs),",
     "        Ok(validation) => classification_summary_from(&validation, source, &[]),"),
]


def guard():
    if subprocess.run("pgrep -f memguard.sh", shell=True, capture_output=True).returncode != 0:
        print("MEMGUARD NOT RUNNING"); sys.exit(9)


def main(label, phase, only):
    mut = f"{S}/mut_{phase}/projects/chirality-piping"
    out = []
    for mid, kind, name, old, new in M:
        if kind != phase or (only and mid not in only):
            continue
        guard()
        raw = open(f"{CAND}/{name}").read()
        if raw.count(old) != 1:
            out.append({"id": mid, "status": "NOT_APPLIED", "count": raw.count(old)}); print(json.dumps(out[-1]), flush=True); continue
        target = f"{mut}/{name}"
        open(target, "w").write(raw.replace(old, new))
        t = time.time()
        try:
            if kind == "py":
                py_compile.compile(target, cfile=f"{S}/tmp/mut_compile.pyc", doraise=True)
                r = subprocess.run(f"cd {mut} && PYTHONDONTWRITEBYTECODE=1 OPENPIPESTRESS_CHECKED_JSON_BIN={WT}/targets/i52-readers/canonical_json/release/openpipestress_jcs_ijson "
                                   f"OPENPIPESTRESS_UNITS_BIN={WT}/targets/i52-readers/units/release/openpipestress_units perl -e 'alarm shift; exec @ARGV' 1800 "
                                   f"{VENV}/bin/python -m pytest -q -p no:cacheprovider -x tests/test_retained_precision_carriers.py", shell=True, capture_output=True, text=True)
                failed = [l.split(" ")[1] for l in r.stdout.splitlines() if l.startswith("FAILED ")]
                status = "KILLED" if r.returncode == 1 and failed else "SURVIVED" if r.returncode == 0 else f"ERROR_{r.returncode}"
                text = r.stdout
            else:
                r = subprocess.run(f"cd {mut}/core/reporting/result_export && env -u RUSTFLAGS CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 TMPDIR={S}/tmp perl -e 'alarm shift; exec @ARGV' 1800 "
                                   f"cargo test --locked --offline --manifest-path {mut}/core/reporting/result_export/Cargo.toml --target-dir {TARGET} --test retained_precision_carriers --test retained_precision_contract",
                                   shell=True, capture_output=True, text=True)
                text = r.stdout + r.stderr
                failed = [l.split(" ")[1] for l in text.splitlines() if l.startswith("test ") and l.endswith("FAILED")]
                compile_error = "could not compile" in text or "error[E" in text
                status = "COMPILE_ERROR" if compile_error else "KILLED" if failed else "SURVIVED" if r.returncode == 0 else f"ERROR_{r.returncode}"
        except py_compile.PyCompileError as error:
            status, failed, text = "SYNTAX_ERROR", [], str(error)
        finally:
            open(target, "w").write(raw)
        out.append({"id": mid, "status": status, "killed_by": failed[:4], "seconds": round(time.time() - t, 1)})
        print(json.dumps(out[-1]), flush=True)
    for _, kind, name, _, _ in M:
        if kind == phase:
            assert open(f"{CAND}/{name}").read() == open(f"{mut}/{name}").read(), name
    json.dump(out, open(f"{S}/logs/mutants_{label}.json", "w"), indent=1)


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2], set(sys.argv[3:]))
