#!/usr/bin/env python3
"""RV94 repair-round mutants (scratch only). Each mutant is one exact edit (or one parsed-JSON
edit) applied to a pristine copy of the repair head in each named language's lane; that
language's committed suites run; the file is restored. A kill is a failing test only.
Usage: rv94_mutants2.py <lang: py|rs|ts> [ids...]"""
import json, os, py_compile, subprocess, sys, time

WT = "WT"
S = f"{WT}/scratch/rv94_u7_01"
PRISTINE = f"{WT}/rv94/pristine2/projects/chirality-piping"
LANE = {"py": f"{WT}/rv94/mut_py2/projects/chirality-piping", "rs": f"{WT}/rv94/mut_rs2/projects/chirality-piping", "ts": f"{WT}/rv94/mut_ts2/projects/chirality-piping"}
VENV = "REPO_ROOT/projects/chirality-piping/.venv"
TARGET = f"{WT}/targets/rv94/mut2"
CO, SC = "core/analysis_runs/compatibility.py", "core/reporting/result_export/src/semantic_contract.rs"
CASES, CORPUS = "fixtures/results/retained_precision_carrier_cases.json", "fixtures/results/retained_precision_cases.json"
R, SV = "src/features/results/", "src/services/"
TS_FILES = [R + "retainedPrecision.test.ts", R + "retainedPrecisionIntegration.test.tsx", SV + "retainedPrecisionAnalysisRun.test.ts", R + "retainedPrecisionOutputRefusal.test.tsx",
            R + "numericalResultQuality.test.ts", R + "knownSemanticLimitations.test.ts"]
D74 = "D-U7-4:ts_requires_live_native_capture"
N3 = " An invalid enum value in a not_required case's quality is refused at G7"
PROBE = "g7_not_required_quality_enum_invalid"


def d74(doc):
    return next(e for e in doc["declared_differences"] if e["id"] == D74)


def summary_forms(doc):
    return [f for f in d74(doc)["forms"] if f["subject"] == "summary"]


def set_side(lang, value):
    def fn(doc):
        for f in summary_forms(doc):
            f["expected"][lang]["summary"] = value
    return fn


def drop_summary_forms(doc):
    d74(doc)["forms"] = [f for f in d74(doc)["forms"] if f["subject"] != "summary"]


def unknown_value(doc):
    for f in summary_forms(doc):
        for lang in ("rust", "python", "typescript"):
            f["expected"][lang]["summary"] = "by_validated_class_now"


def drop_n3(doc):
    i = doc["scope"].index(N3)
    doc["scope"] = doc["scope"][:i]


def n3_ts_code(doc):
    old = "TS SOURCE_PRODUCER_CONTRACT_UNSUPPORTED, TS's G7 contract check"
    assert doc["scope"].count(old) == 1
    doc["scope"] = doc["scope"].replace(old, "TS SOURCE_NUMERICAL_CASE_INVALID, TS's G7 contract check")


def probe(doc):
    return next(m for m in doc["mutations"] if m["id"] == PROBE)


def drop_probe(doc):
    doc["mutations"] = [m for m in doc["mutations"] if m["id"] != PROBE]


def probe_reader(lang, code):
    def fn(doc):
        probe(doc)["expected_by_reader"][lang]["code"] = code
    return fn


def probe_expected(doc):
    probe(doc)["expected"]["code"] = "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED"


ALL = ("py", "rs", "ts")
M = [
    # S-1 / N-4 code (Python, Rust)
    ("R01_py_summary_from_invocation_cases", ("py",), CO, "return _classification_summary_from(validation, source, requested_basis_refs or [])",
     "return _classification_summary_from(validation, source, [{\"ref_type\": \"load_case\", \"ref_id\": c.get(\"id\")} for c in ((invocation or {}).get(\"request\", {}).get(\"model\", {}).get(\"load_cases\") or [])])"),
    ("R02_py_summary_reader_flag_only", ("py",), CO, "current = _retained_standing_from(validation, source, requested_basis_refs) == \"numerically_eligible\"", "current = validation[\"numerical_eligible\"]"),
    ("R03_py_no_refs_means_receipt_order", ("py",), CO, "requested_basis_refs or [])", "requested_basis_refs if requested_basis_refs is not None else [c[\"basis_ref\"] for c in source[\"retained_precision\"][\"body\"][\"cases\"]])"),
    ("R11_rs_summary_reader_flag_only", ("rs",), SC, "retained_standing_from(validation, source, requested_basis_refs) == \"numerically_eligible\";", "validation.numerical_eligible;"),
    ("R12_rs_summary_receipt_order", ("rs",), SC, "Ok(validation) => classification_summary_from(&validation, source, requested_basis_refs),",
     "Ok(validation) => { let _ = requested_basis_refs; let order: Vec<Value> = source[\"retained_precision\"][\"body\"][\"cases\"].as_array().map(|c| c.iter().map(|x| x[\"basis_ref\"].clone()).collect()).unwrap_or_default(); classification_summary_from(&validation, source, &order) }"),
    # S-1 declaration (the case file)
    ("R21_d74_summary_ts_side_current", ALL, CASES, set_side("typescript", "by_validated_class_current"), None),
    ("R22_d74_summary_python_side_not_current", ALL, CASES, set_side("python", "by_validated_class"), None),
    ("R23_d74_summary_rust_side_not_current", ALL, CASES, set_side("rust", "by_validated_class"), None),
    ("R24_d74_summary_forms_removed", ALL, CASES, drop_summary_forms, None),
    ("R25_d74_summary_unknown_value", ALL, CASES, unknown_value, None),
    # N-3 (scope clause and the shared probe)
    ("R26_n3_clause_removed", ALL, CASES, drop_n3, None),
    ("R27_n3_clause_ts_code_misstated", ALL, CASES, n3_ts_code, None),
    ("R31_probe_removed", ALL, CORPUS, drop_probe, None),
    ("R32_probe_ts_code", ALL, CORPUS, probe_reader("typescript", "SOURCE_NUMERICAL_CASE_INVALID"), None),
    ("R33_probe_rust_code", ALL, CORPUS, probe_reader("rust", "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED"), None),
    ("R34_probe_python_code", ALL, CORPUS, probe_reader("python", "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED"), None),
    ("R35_probe_expected_code", ALL, CORPUS, probe_expected, None),
]


def run_py(lane):
    r = subprocess.run(f"cd {lane} && TMPDIR={S}/tmp PYTHONDONTWRITEBYTECODE=1 OPENPIPESTRESS_CHECKED_JSON_BIN={WT}/targets/i52-readers/canonical_json/release/openpipestress_jcs_ijson "
                       f"OPENPIPESTRESS_UNITS_BIN={WT}/targets/i52-readers/units/release/openpipestress_units perl -e 'alarm shift; exec @ARGV' 1800 "
                       f"{VENV}/bin/python -m pytest -q -p no:cacheprovider --basetemp={S}/tmp/pymut2 -x tests/test_retained_precision_carriers.py tests/test_retained_precision_contract.py",
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
    out = f"{S}/tmp/tsmut2.json"
    if os.path.exists(out):
        os.remove(out)
    r = subprocess.run(f"cd {lane}/apps/desktop && TMPDIR={S}/tmp perl -e 'alarm shift; exec @ARGV' 1800 ../../node_modules/.bin/vitest run --maxWorkers=4 --reporter=json --outputFile={out} " + " ".join(TS_FILES),
                       shell=True, capture_output=True, text=True)
    try:
        d = json.load(open(out))
    except Exception:
        return f"ERROR_{r.returncode}", []
    failed = [a["fullName"][:120] for t in d["testResults"] for a in t["assertionResults"] if a["status"] == "failed"]
    load_errors = [t["name"].split("/")[-1] for t in d["testResults"] if t["status"] == "failed" and not any(a["status"] == "failed" for a in t["assertionResults"])]
    if failed:
        return "KILLED", failed[:2] + ([f"(+{len(failed) - 2})"] if len(failed) > 2 else [])
    return ("LOAD_ERROR" if load_errors else "SURVIVED" if r.returncode == 0 else f"ERROR_{r.returncode}"), load_errors


RUN = {"py": run_py, "rs": run_rs, "ts": run_ts}


def main(lang, only):
    out = []
    for mid, langs, name, old, new in M:
        if lang not in langs or (only and mid not in only):
            continue
        assert subprocess.run("ps -p 5387", shell=True, capture_output=True).returncode == 0, "MEMGUARD NOT RUNNING"
        lane = LANE[lang]
        raw = open(f"{PRISTINE}/{name}").read()
        assert open(f"{lane}/{name}").read() == raw, f"lane not pristine: {name}"
        if callable(old):
            doc = json.loads(raw)
            old(doc)
            mutated = json.dumps(doc, indent=2, ensure_ascii=False) + "\n"
            # the round-trip must reproduce the pristine bytes when nothing is edited
            assert json.dumps(json.loads(raw), indent=2, ensure_ascii=False) + "\n" == raw, "format round-trip"
        else:
            if raw.count(old) != 1:
                out.append({"id": mid, "lang": lang, "status": "NOT_APPLIED"}); print(json.dumps(out[-1]), flush=True); continue
            mutated = raw.replace(old, new)
        assert mutated != raw, mid
        target = f"{lane}/{name}"
        open(target, "w").write(mutated)
        t = time.time()
        try:
            if name.endswith(".py"):
                py_compile.compile(target, cfile=f"{S}/tmp/mut2_compile.pyc", doraise=True)
            status, failed = RUN[lang](lane)
        except py_compile.PyCompileError as error:
            status, failed = "SYNTAX_ERROR", [str(error)[:200]]
        finally:
            open(target, "w").write(raw)
        out.append({"id": mid, "lang": lang, "file": name, "status": status, "killed_by": failed, "seconds": round(time.time() - t, 1)})
        print(json.dumps(out[-1]), flush=True)
    for _, langs, name, _, _ in M:
        assert open(f"{PRISTINE}/{name}").read() == open(f"{LANE[lang]}/{name}").read(), name
    json.dump(out, open(f"{S}/r2/mutants2_{lang}.json", "w"), indent=1)


if __name__ == "__main__":
    main(sys.argv[1], set(sys.argv[2:]))
