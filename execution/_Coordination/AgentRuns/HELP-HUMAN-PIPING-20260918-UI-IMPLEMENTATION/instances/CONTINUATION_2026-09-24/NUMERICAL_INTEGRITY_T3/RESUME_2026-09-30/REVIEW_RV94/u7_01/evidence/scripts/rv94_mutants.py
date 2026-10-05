#!/usr/bin/env python3
"""RV94's own mutants (scratch only). Each mutant replaces one exact, unique snippet of a pristine
candidate file in a mutant lane, runs the owning committed suites, then restores the file.
A kill is a failing test only; a compile/syntax/collection error is reported, never counted.
Usage: rv94_mutants.py <phase: py|rs|ts> [ids...]"""
import json, os, py_compile, subprocess, sys, time

WT = "WT"
S = f"{WT}/scratch/rv94_u7_01"
PRISTINE = f"{WT}/rv94/pristine/projects/chirality-piping"
LANE = {"py": f"{WT}/rv94/mut_py/projects/chirality-piping", "rs": f"{WT}/rv94/mut_rs/projects/chirality-piping", "ts": f"{WT}/rv94/mut_ts/projects/chirality-piping"}
VENV = "REPO_ROOT/projects/chirality-piping/.venv"
TARGET = f"{WT}/targets/rv94/mut"
PYR, CO = "core/analysis_runs/retained_precision.py", "core/analysis_runs/compatibility.py"
RSR, SC = "core/reporting/result_export/src/retained_precision.rs", "core/reporting/result_export/src/semantic_contract.rs"
D = "apps/desktop/src/"
TRP, TRPS, NRQ = D + "features/results/retainedPrecision.ts", D + "features/results/retainedPrecisionStanding.ts", D + "features/results/numericalResultQuality.ts"
PREV, SNP, REP = D + "services/previewService.ts", D + "features/stress-neutral/StressNeutralExportPanel.tsx", D + "features/result-export/ResultExportPanel.tsx"
R, SV = "src/features/results/", "src/services/"
TS_FILES = [R + "retainedPrecision.test.ts", R + "retainedPrecisionIntegration.test.tsx", SV + "retainedPrecisionAnalysisRun.test.ts", R + "retainedPrecisionOutputRefusal.test.tsx",
            R + "numericalResultQuality.test.ts", R + "knownSemanticLimitations.test.ts", R + "resultSemantics.test.ts", SV + "analysisRunCompatibility.test.ts",
            SV + "previewService.test.ts", SV + "ruleCheckService.test.ts", R + "HistoricalRunContext.test.tsx", R + "ResultsPanel.test.tsx", R + "previewPhysicsEvidence.test.ts",
            "src/features/stress-neutral/StressNeutralExportPanel.test.tsx", "src/features/result-export/ResultExportPanel.test.tsx"]

M = [
    # --- the flags, each reverted alone
    ("M01_py_flag_off", "py", PYR, "_IMPLEMENTATION_COMPLETE = True", "_IMPLEMENTATION_COMPLETE = False"),
    ("M11_rs_flag_off", "rs", RSR, "const IMPLEMENTATION_COMPLETE: bool = true;", "const IMPLEMENTATION_COMPLETE: bool = false;"),
    ("M21_ts_flag_off", "ts", TRP, "const SUMMARY_COVERAGE_COMPLETE = true;", "const SUMMARY_COVERAGE_COMPLETE = false;"),
    # --- reader conditions, per language
    ("M02_py_reader_no_invocation", "py", PYR, "eligible=_IMPLEMENTATION_COMPLETE and invocation is not None and snapshot", "eligible=_IMPLEMENTATION_COMPLETE and snapshot"),
    ("M03_py_reader_selected_only", "py", PYR, "c[\"status\"] in (\"selected\",\"not_required\") for c in cases)", "c[\"status\"] in (\"selected\",) for c in cases)"),
    ("M04_py_reader_admits_unavailable", "py", PYR, "c[\"status\"] in (\"selected\",\"not_required\") for c in cases)", "c[\"status\"] in (\"selected\",\"not_required\",\"unavailable\") for c in cases)"),
    ("M09_py_reader_no_mechanics", "py", PYR, " and snapshot[\"status\"][\"mechanics\"]==\"MECHANICS_SOLVED\" and all(", " and all("),
    ("M12_rs_reader_no_invocation", "rs", RSR, "    let eligible = IMPLEMENTATION_COMPLETE\n        && actual_invocation.is_some()\n", "    let eligible = IMPLEMENTATION_COMPLETE\n"),
    ("M13_rs_reader_selected_only", "rs", RSR, ".all(|c| matches!(text(&c[\"status\"]), \"selected\" | \"not_required\"));", ".all(|c| matches!(text(&c[\"status\"]), \"selected\"));"),
    ("M17_rs_reader_admits_unavailable", "rs", RSR, ".all(|c| matches!(text(&c[\"status\"]), \"selected\" | \"not_required\"));", ".all(|c| matches!(text(&c[\"status\"]), \"selected\" | \"not_required\" | \"unavailable\"));"),
    ("M22_ts_reader_no_invocation", "ts", TRP, "SUMMARY_COVERAGE_COMPLETE && actual !== undefined && ", "SUMMARY_COVERAGE_COMPLETE && "),
    ("M23_ts_reader_selected_only", "ts", TRP, "['selected', 'not_required'].includes(c.status)", "['selected'].includes(c.status)"),
    ("M36_ts_reader_admits_unavailable", "ts", TRP, "['selected', 'not_required'].includes(c.status)", "['selected', 'not_required', 'unavailable'].includes(c.status)"),
    # --- carrier conditions, per language
    ("M05_py_carrier_no_requested", "py", CO, " or list(requested_basis_refs) != expected\n", "\n"),
    ("M06_py_carrier_no_nr_ordinary", "py", CO, " or not _not_required_cases_ordinarily_eligible(source, cases)):", "):"),
    ("M07_py_summary_reader_only", "py", CO, "current = _retained_standing_from(validation, source, requested) == \"numerically_eligible\"", "current = validation[\"numerical_eligible\"]"),
    ("M14_rs_carrier_no_requested", "rs", SC, "        || expected.len() != requested_basis_refs.len()\n        || expected.iter().zip(requested_basis_refs).any(|(a, b)| *a != b)\n", ""),
    ("M15_rs_carrier_no_nr_ordinary", "rs", SC, "        || !not_required_cases_ordinarily_eligible(source, cases)\n", ""),
    ("M16_rs_summary_reader_only", "rs", SC, "let current = retained_standing_from(validation, source, &requested) == \"numerically_eligible\";", "let current = validation.numerical_eligible;"),
    ("M33_ts_carrier_no_requested", "ts", TRPS, "\n    || cases.length !== requested.length || cases.some((c, index) => !sameJson(c?.basis_ref, requested[index]))", ""),
    ("M34_ts_carrier_no_nr_ordinary", "ts", TRPS, "\n    || !notRequiredCasesOrdinarilyEligible(source, cases)) return \"needs_recompute\";", ") return \"needs_recompute\";"),
    # --- TS N-2 (live capture binding)
    ("M24_ts_live_null_passes", "ts", TRPS, "if (registration.live?.(model) !== true)", "if (registration.live?.(model) === false)"),
    ("M25_ts_live_check_removed", "ts", TRPS, "  if (registration.live?.(model) !== true) return { standing: \"needs_recompute\", eligible: false, findings: [RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED] };\n", ""),
    ("M26_ts_preview_always_live", "ts", PREV, "model => hasNativeMechanicsInvocation(source, model as PreviewModel)", "() => true"),
    ("M37_ts_live_ignores_model", "ts", PREV, "model => hasNativeMechanicsInvocation(source, model as PreviewModel)", "() => hasNativeMechanicsInvocation(source, capture.callerModel)"),
    # --- TS N-5 (explicit panel gates)
    ("M27_ts_stress_gate_removed", "ts", SNP, "|| !hasCurrentSourceContract(result) || loadReferenceOutputRefusal(result) !== null || !numericalResultStanding", "|| !hasCurrentSourceContract(result) || !numericalResultStanding"),
    ("M28_ts_export_gate_removed", "ts", REP, " || loadReferenceOutputRefusal(result) !== null\n", "\n"),
    # --- TS token/status (RV92 N-8)
    ("M30_ts_status_is_token", "ts", NRQ, "numerically_eligible: \"integrity_checked\"", "numerically_eligible: \"numerically_eligible\""),
    ("M35_ts_refused_token_as_needs_recompute", "ts", TRPS, "if (registration.outcome.error !== null) return { standing: \"unsupported\",", "if (registration.outcome.error !== null) return { standing: \"needs_recompute\","),
    # --- TS withheld fix (e5e1693ceb)
    ("M32_ts_withheld_ignores_live", "ts", TRPS, "retainedPrecisionStanding(source!, model).eligible ? requestedRefs(model) : []", "requestedRefs(model)"),
]


def guard():
    if subprocess.run("ps -p 5387", shell=True, capture_output=True).returncode != 0:
        print("MEMGUARD NOT RUNNING"); sys.exit(9)


def run_py(lane):
    r = subprocess.run(f"cd {lane} && TMPDIR={S}/tmp PYTHONDONTWRITEBYTECODE=1 OPENPIPESTRESS_CHECKED_JSON_BIN={WT}/targets/i52-readers/canonical_json/release/openpipestress_jcs_ijson "
                       f"OPENPIPESTRESS_UNITS_BIN={WT}/targets/i52-readers/units/release/openpipestress_units perl -e 'alarm shift; exec @ARGV' 1800 "
                       f"{VENV}/bin/python -m pytest -q -p no:cacheprovider --basetemp={S}/tmp/pymut -x tests/test_retained_precision_carriers.py tests/test_retained_precision_contract.py",
                       shell=True, capture_output=True, text=True)
    failed = [l.split(" ")[1] for l in r.stdout.splitlines() if l.startswith("FAILED ")]
    return ("KILLED" if r.returncode == 1 and failed else "SURVIVED" if r.returncode == 0 else f"ERROR_{r.returncode}"), failed[:3]


def run_rs(lane):
    r = subprocess.run(f"cd {lane}/core/reporting/result_export && env -u RUSTFLAGS CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 TMPDIR={S}/tmp perl -e 'alarm shift; exec @ARGV' 1800 "
                       f"cargo test --locked --offline --manifest-path {lane}/core/reporting/result_export/Cargo.toml --target-dir {TARGET} --test retained_precision_carriers --test retained_precision_contract",
                       shell=True, capture_output=True, text=True)
    text = r.stdout + r.stderr
    failed = [l.split(" ")[1] for l in text.splitlines() if l.startswith("test ") and l.endswith("FAILED")]
    if "could not compile" in text or "error[E" in text:
        return "COMPILE_ERROR", []
    return ("KILLED" if failed else "SURVIVED" if r.returncode == 0 else f"ERROR_{r.returncode}"), failed[:3]


def run_ts(lane):
    out = f"{S}/tmp/tsmut.json"
    r = subprocess.run(f"cd {lane}/apps/desktop && TMPDIR={S}/tmp perl -e 'alarm shift; exec @ARGV' 1800 ../../node_modules/.bin/vitest run --maxWorkers=4 --reporter=json --outputFile={out} " + " ".join(TS_FILES),
                       shell=True, capture_output=True, text=True)
    try:
        d = json.load(open(out))
    except Exception:
        return f"ERROR_{r.returncode}", []
    failed = [a["fullName"][:120] for t in d["testResults"] for a in t["assertionResults"] if a["status"] == "failed"]
    load_errors = [t["name"].split("/")[-1] for t in d["testResults"] if t["status"] == "failed" and not any(a["status"] == "failed" for a in t["assertionResults"])]
    if failed:
        return "KILLED", failed[:3] + [f"(+{len(failed) - 3})"] if len(failed) > 3 else failed
    return ("LOAD_ERROR" if load_errors else "SURVIVED" if r.returncode == 0 else f"ERROR_{r.returncode}"), load_errors


def main(phase, only):
    out = []
    for mid, kind, name, old, new in M:
        if kind != phase or (only and mid not in only):
            continue
        guard()
        lane = LANE[kind]
        raw = open(f"{PRISTINE}/{name}").read()
        assert open(f"{lane}/{name}").read() == raw, f"lane not pristine: {name}"
        if raw.count(old) != 1:
            out.append({"id": mid, "status": "NOT_APPLIED", "count": raw.count(old)}); print(json.dumps(out[-1]), flush=True); continue
        target = f"{lane}/{name}"
        open(target, "w").write(raw.replace(old, new))
        t = time.time()
        try:
            if kind == "py":
                py_compile.compile(target, cfile=f"{S}/tmp/mut_compile.pyc", doraise=True)
                status, failed = run_py(lane)
            elif kind == "rs":
                status, failed = run_rs(lane)
            else:
                status, failed = run_ts(lane)
        except py_compile.PyCompileError as error:
            status, failed = "SYNTAX_ERROR", [str(error)[:200]]
        finally:
            open(target, "w").write(raw)
        out.append({"id": mid, "kind": kind, "file": name, "status": status, "killed_by": failed, "seconds": round(time.time() - t, 1)})
        print(json.dumps(out[-1]), flush=True)
    for _, kind, name, _, _ in M:
        if kind == phase:
            assert open(f"{PRISTINE}/{name}").read() == open(f"{LANE[kind]}/{name}").read(), name
    json.dump(out, open(f"{S}/mutants_rv94_{phase}.json", "w"), indent=1)


if __name__ == "__main__":
    main(sys.argv[1], set(sys.argv[2:]))
