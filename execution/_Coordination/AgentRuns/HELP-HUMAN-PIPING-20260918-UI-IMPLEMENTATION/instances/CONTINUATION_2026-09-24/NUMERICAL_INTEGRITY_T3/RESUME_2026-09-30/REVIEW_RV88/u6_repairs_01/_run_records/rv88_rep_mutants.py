#!/usr/bin/env python3
"""RV88 (U6 repair confirmation): re-run RV88's earlier survivors and a few new
mutants against I66's repaired tests, in a scratch copy of da274dd961.
Kinds: rs (I66's result_export targets), py (I66's Python carrier tests),
schema (I66's schema test). A kill is a failing test, never a build error.
Usage: rv88_rep_mutants.py <out.json> [ids...]"""
import json, os, re, subprocess, sys

WT = "WT"
CAND = f"{WT}/rv88/r_cand/projects/chirality-piping"
MUT = f"{WT}/rv88/r_mut/projects/chirality-piping"
VENV = "REPO_ROOT/projects/chirality-piping/.venv"
ENV = dict(os.environ, PYTHONDONTWRITEBYTECODE="1", TMPDIR=f"{WT}/scratch/rv88_u6/rep/tmp", CARGO_BUILD_JOBS="4", RUST_TEST_THREADS="2",
           OPENPIPESTRESS_CHECKED_JSON_BIN=f"{WT}/targets/rv88/canonical_json/release/openpipestress_jcs_ijson",
           OPENPIPESTRESS_UNITS_BIN=f"{WT}/targets/rv88/units/release/openpipestress_units")
SC, DV = "core/reporting/result_export/src/semantic_contract.rs", "core/reporting/result_export/src/derivative.rs"
TAURI, RUNNER = "apps/desktop/src-tauri/src/lib.rs", "core/runner/headless/src/result_envelope_binding.rs"
CO, RE = "core/analysis_runs/compatibility.py", "core/analysis_runs/records.py"
SN, RES = "schemas/stress_neutral_export.v0.3.schema.json", "schemas/results.v0.3.schema.yaml"
SUCC = "openpipestress.result_semantics/0.3.0/preview-physics-retained-1"


def sn_succ(d):
    return next(b for b in d["oneOf"] if b["properties"]["producer"]["properties"]["semantic_contract_id"]["const"] == SUCC)


def res_succ(d):
    return next(b for b in d["$defs"]["ResultEnvelope"]["oneOf"] if b["properties"]["producer"]["properties"]["semantic_contract_id"]["const"] == SUCC)


M = [
    ("R01_row_guard_first_row_only", "rs", SC, "            rows.iter()\n                .any(|row| row[\"recovery_method\"] == crate::retained_precision::METHOD)",
     "            rows.first().into_iter()\n                .any(|row| row[\"recovery_method\"] == crate::retained_precision::METHOD)"),
    ("R02_doc_downgrade_ignores_null", "rs", DV, "    } else if doc[\"result_envelope\"].get(\"retained_precision\").is_some() {",
     "    } else if doc[\"result_envelope\"].get(\"retained_precision\").is_some_and(|v| !v.is_null()) {"),
    ("R05_refs_order_insensitive", "rs", SC, "        || expected.iter().zip(requested_basis_refs).any(|(a, b)| *a != b)\n",
     "        || expected.iter().any(|a| !requested_basis_refs.contains(*a))\n"),
    ("R06_member_guard_skips_legacy", "rs", SC, "    if !is_retained(source) && source.get(\"retained_precision\").is_some() {",
     "    if !is_retained(source) && source[\"schema_version\"] != \"0.1.0\" && source.get(\"retained_precision\").is_some() {"),
    ("V1_moment_unit_spelled_middle_dot", "rs", DV, "\"N*m\" | \"kN*m\" => Some(\"N*m\"),", "\"N*m\" | \"kN*m\" => Some(\"N\\u{b7}m\"),"),
    ("V2_kN_unmapped", "rs", DV, "\"N\" | \"kN\" => Some(\"N\"),", "\"N\" => Some(\"N\"),"),
    ("G1_seam_called_deep_in_tauri", "rs", TAURI, "fn solver_result_row_value(envelope: &Value, result_id: &str) -> Result<Option<(f64, String)>, &'static str> {\n",
     "fn solver_result_row_value(envelope: &Value, result_id: &str) -> Result<Option<(f64, String)>, &'static str> {\n    // open_pipe_stress_result_export::semantic_contract::retained_standing_from(\n"),
    ("G2_seam_called_in_runner_binding", "rs", RUNNER, None, "// open_pipe_stress_result_export::semantic_contract::retained_standing_from(\n"),
    ("Q02_token_helper_first_row_only", "py", CO, "row.get(\"recovery_method\") == RETAINED_METHOD for row in rows)", "row.get(\"recovery_method\") == RETAINED_METHOD for row in rows[:1])"),
    ("Q06_refs_order_insensitive", "py", CO, "list(requested_basis_refs) != expected", "(len(requested_basis_refs) != len(expected) or any(ref not in expected for ref in requested_basis_refs))"),
    ("W06_sn_successor_annotations_optional", "schema", SN, lambda d: sn_succ(d)["required"].remove("source_annotations"), None),
    ("W09_sn_successor_admits_sbr", "schema", SN, lambda d: sn_succ(d).pop("not"), None),
    ("W10_results_successor_admits_sbr", "schema", RES, lambda d: res_succ(d).pop("not"), None),
]


def run(kind):
    if kind == "rs":
        cmd = ("perl -e 'alarm shift; exec @ARGV' 1500 cargo test --locked --offline --no-fail-fast "
               f"--target-dir {WT}/targets/rv88/r_mut --test retained_precision_carriers --test preview_physics_contract --test derivative_contract")
        r = subprocess.run(cmd, shell=True, cwd=f"{MUT}/core/reporting/result_export", env=ENV, capture_output=True, text=True)
        out = r.stdout + r.stderr
        build_error = "error[E" in out or "could not compile" in out
        failed = re.findall(r"^test (\S+) \.\.\. FAILED", out, re.M)
        return r.returncode, failed, build_error
    tests = "tests/test_retained_precision_carriers.py" if kind == "py" else "tests/test_retained_precision_schema.py"
    r = subprocess.run(f"perl -e 'alarm shift; exec @ARGV' 1500 {VENV}/bin/python -m pytest -q -p no:cacheprovider {tests}", shell=True, cwd=MUT, env=ENV, capture_output=True, text=True)
    failed = re.findall(r"^FAILED (\S+)", r.stdout, re.M)
    return r.returncode, failed, "SyntaxError" in r.stdout + r.stderr


def main():
    only = set(sys.argv[2:])
    res = []
    for mid, kind, rel, old, new in M:
        if only and mid not in only:
            continue
        if subprocess.run("pgrep -f memguard.sh", shell=True, capture_output=True).returncode != 0:
            sys.exit("MEMGUARD NOT RUNNING")
        path = f"{MUT}/{rel}"
        raw = open(f"{CAND}/{rel}").read()
        if kind == "schema":
            d = json.loads(raw); old(d); mutated = json.dumps(d, indent=2) + "\n"
        elif old is None:
            mutated = new + raw
        else:
            if raw.count(old) != 1:
                res.append({"id": mid, "status": f"NOT_APPLIED({raw.count(old)})"}); print(mid, res[-1]["status"], flush=True); continue
            mutated = raw.replace(old, new)
        open(path, "w").write(mutated)
        try:
            rc, failed, build_error = run(kind)
        finally:
            open(path, "w").write(raw)
        status = "BUILD_ERROR" if build_error else ("KILLED" if failed else ("SURVIVED" if rc == 0 else f"ERROR_{rc}"))
        res.append({"id": mid, "kind": kind, "file": rel, "status": status, "failed": failed[:4]})
        print(mid, status, failed[:2], flush=True)
        json.dump(res, open(sys.argv[1], "w"), indent=1)
    for _, kind, rel, _, _ in M:
        assert open(f"{CAND}/{rel}").read() == open(f"{MUT}/{rel}").read(), rel


if __name__ == "__main__":
    main()
