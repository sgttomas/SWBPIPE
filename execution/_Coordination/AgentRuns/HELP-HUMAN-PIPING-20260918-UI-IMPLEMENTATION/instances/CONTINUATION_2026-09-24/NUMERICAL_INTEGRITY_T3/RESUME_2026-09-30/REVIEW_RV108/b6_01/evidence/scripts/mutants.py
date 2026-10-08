"""RV108 mutants: one edit (or one edit set) at a time to WT/rv108/mut, run the lanes, restore from WT/rv108/cand."""
import json, os, re, shutil, subprocess, sys, time, xml.etree.ElementTree as ET
from pathlib import Path
WT = Path("WT")
S = WT / "scratch/rv108_b6_01"; P = "projects/chirality-piping"
MUT, CAND, BASE = WT / "rv108/mut" / P, WT / "rv108/cand" / P, WT / "rv108/base" / P
VENV = "VENV"
TSR = "apps/desktop/src/features/results/retainedPrecision.ts"
PYR, PYC = "core/analysis_runs/retained_precision.py", "core/analysis_runs/compatibility.py"
RSC, RSR = "core/reporting/result_export/src/semantic_contract.rs", "core/reporting/result_export/src/retained_precision.rs"
CORPUS, CASES = "fixtures/results/retained_precision_cases.json", "fixtures/results/retained_precision_carrier_cases.json"
ENV = dict(os.environ, TMPDIR=str(S / "tmp"), PYTHONDONTWRITEBYTECODE="1",
           OPENPIPESTRESS_CHECKED_JSON_BIN=str(WT / "targets/rv108-b6/checked-json/release/openpipestress_jcs_ijson"),
           OPENPIPESTRESS_UNITS_BIN=str(WT / "targets/rv108-b6/units-authority/release/openpipestress_units"))
OUT = S / "out/mutants.jsonl"; LOGS = S / "logs/mut"; LOGS.mkdir(parents=True, exist_ok=True)

def sub(path, old, new):
    def f():
        t = (MUT / path).read_text(); n = t.count(old)
        assert n == 1, f"{path}: {n} matches for {old[:60]!r}"
        (MUT / path).write_text(t.replace(old, new))
    return f
def from_base(path):
    return lambda: shutil.copyfile(BASE / path, MUT / path)
def jedit(path, fn):
    def f():
        d = json.loads((MUT / path).read_text()); fn(d)
        (MUT / path).write_text(json.dumps(d, indent=2, ensure_ascii=False) + "\n")
    return f
def restore():
    for path in (TSR, PYR, PYC, RSC, RSR, CORPUS, CASES, "tests/test_retained_precision_contract.py"):
        shutil.copyfile(CAND / path, MUT / path)

def lane_ts(tag):
    out = LOGS / f"{tag}_ts.json"
    r = subprocess.run(["../../node_modules/.bin/vitest", "run", "src/features/results/retainedPrecision.test.ts", "src/features/results/retainedPrecisionIntegration.test.tsx",
                        "--reporter=json", f"--outputFile.json={out}"], cwd=MUT / "apps/desktop", env=ENV, capture_output=True, text=True)
    try:
        j = json.loads(out.read_text())
        failed = [t["fullName"] for f in j["testResults"] for t in f["assertionResults"] if t["status"] == "failed"]
        return {"rc": r.returncode, "passed": j["numPassedTests"], "failed": j["numFailedTests"], "failing": failed[:40], "suite_errors": [f["name"].split("/")[-1] for f in j["testResults"] if f.get("status") == "failed" and not f["assertionResults"]]}
    except Exception as e:
        return {"rc": r.returncode, "error": str(e), "tail": r.stdout[-800:] + r.stderr[-800:]}
def lane_py(tag):
    xml = LOGS / f"{tag}_py.xml"
    r = subprocess.run([f"{VENV}/bin/python", "-m", "pytest", "-p", "no:cacheprovider", f"--basetemp={S}/tmp/mut/bt", "-q", f"--junitxml={xml}",
                        "tests/test_retained_precision_carriers.py", "tests/test_retained_precision_contract.py", "-k", "carriers or snapshot_07 or first_failure_controls"],
                       cwd=MUT, env=ENV, capture_output=True, text=True)
    try:
        root = ET.parse(xml).getroot(); failing = []; total = 0
        for tc in root.iter("testcase"):
            total += 1
            if tc.find("failure") is not None or tc.find("error") is not None: failing.append(tc.get("name"))
        return {"rc": r.returncode, "total": total, "failed": len(failing), "failing": failing[:40]}
    except Exception as e:
        return {"rc": r.returncode, "error": str(e), "tail": r.stdout[-800:]}
def lane_rs(tag):
    log = LOGS / f"{tag}_rs.log"
    env = {k: v for k, v in ENV.items() if k not in ("RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS")}
    env.update(CARGO_BUILD_JOBS="4", RUST_TEST_THREADS="2", CARGO_TARGET_DIR=str(WT / "targets/rv108-b6/rx-mut"))
    with log.open("w") as fh:
        r = subprocess.run([str(WT / "tools/t3_cargo.sh"), "test", "--locked", "--offline", "--no-fail-fast", "--test", "retained_precision_contract", "--test", "retained_precision_carriers"],
                           cwd=MUT / "core/reporting/result_export", env=env, stdout=fh, stderr=subprocess.STDOUT)
    text = log.read_text(errors="replace")
    failing = re.findall(r"^test (\S+) \.\.\. FAILED", text, re.M); passed = len(re.findall(r"^test \S+ \.\.\. ok", text, re.M))
    return {"rc": r.returncode, "passed": passed, "failed": len(failing), "failing": failing[:40], "compile_error": "error[E" in text or "could not compile" in text}
def lane_probe(tag):
    out = S / f"out/mut_{tag}_probe.jsonl"
    env = dict(ENV, RV108_IN=str(S / "probes/mut_probe_subset.json"), RV108_OUT=str(out))
    subprocess.run(["../../node_modules/.bin/vitest", "run", "src/features/results/zzRv108Probe.test.ts"], cwd=MUT / "apps/desktop", env=env, capture_output=True)
    ref = {json.loads(l)["id"]: json.loads(l) for l in open(S / "out/ts_cand_probes.jsonl")}
    diffs = []
    for l in open(out):
        x = json.loads(l)
        if x["raw_inv"] != ref[x["id"]]["raw_inv"]: diffs.append((x["id"], x["raw_inv"].get("code"), ref[x["id"]]["raw_inv"].get("code")))
    return {"differences_from_head": len(diffs), "examples": diffs[:6]}

LANES = {"ts": lane_ts, "py": lane_py, "rs": lane_rs, "probe": lane_probe}
G7_LIST = "['not_claimed', 'reference_verified', 'unresolved']"
MUTANTS = [
  ("N0", "no mutation (head)", [], ["ts", "py", "rs", "probe"]),
  # TS reader (baseHeaderCode and the G7 line)
  ("RT1", "TS: drop the value_representation sub-check of the quality branch", [sub(TSR, " || q.value_representation !== 'finite_binary64'\n", "\n")], ["ts", "probe"]),
  ("RT2", "TS: formulation branch without its exact-keys sub-check", [sub(TSR, "if (!exact(f, ['profile_id', 'limitations']) || f.profile_id", "if (!isObj(f) || f.profile_id")], ["ts", "probe"]),
  ("RT3", "TS: contract_evidence branch weakened to null/absent only", [sub(TSR, "if (!isObj(p.contract_evidence)) return", "if (p.contract_evidence == null) return")], ["ts", "probe"]),
  ("RT4", "TS: case branch without the evidence_refs array sub-check", [sub(TSR, "|| !Array.isArray(c.evidence_refs) || !c.evidence_refs.every(text))", "|| !c.evidence_refs.every(text))")], ["ts", "probe"]),
  ("RT5", "TS: G7 reads the unprojected successor (baseHeaderCode(s))", [sub(TSR, "throw new RetainedPrecisionError(gate, baseHeaderCode(base));", "throw new RetainedPrecisionError(gate, baseHeaderCode(s));")], ["ts"]),
  ("RT6", "TS: carrier_evidence branch returns Rust's base code", [sub(TSR, "if (Object.hasOwn(p, 'carrier_evidence')) return 'SOURCE_PRODUCER_CONTRACT_UNSUPPORTED';", "if (Object.hasOwn(p, 'carrier_evidence')) return 'SOURCE_PREVIEW_PHYSICS_FOREIGN_METHOD_EVIDENCE';")], ["ts", "probe"]),
  ("RT7", "TS: formulation branch without the limitations-element sub-check", [sub(TSR, "|| !f.limitations.length\n    || !f.limitations.every(text)) return", "|| !f.limitations.length) return")], ["ts", "probe"]),
  ("RT8", "TS: quality branch without its exact-keys sub-check", [sub(TSR, "if (!exact(q, ['value_representation', 'publication_quantization', 'integrity_policy', 'status', 'cases']) || q.value_representation", "if (!isObj(q) || q.value_representation")], ["ts", "probe"]),
  ("RT9", "TS: case branch without its exact-keys sub-check", [sub(TSR, "if (!exact(c, ['basis_ref', 'structural_status', 'solve_quality', 'model_matrix_fidelity', 'accuracy_evidence', 'evidence_refs'])\n", "if (!isObj(c)\n")], ["ts"]),
  ("RT10", "TS: accuracy_evidence vocabulary widened by 'estimated'", [sub(TSR, "|| !" + G7_LIST + ".includes(c.accuracy_evidence)", "|| !['not_claimed', 'reference_verified', 'unresolved', 'estimated'].includes(c.accuracy_evidence)")], ["ts"]),
  ("RT11", "TS: QUALITY_STATUSES widened by 'estimated'", [sub(TSR, "const QUALITY_STATUSES = ['not_assessed', 'checks_passed', 'sensitive', 'unresolved', 'failed'];", "const QUALITY_STATUSES = ['not_assessed', 'checks_passed', 'sensitive', 'unresolved', 'failed', 'estimated'];")], ["ts"]),
  # Python reader and carrier
  ("PM1", "Py: transport G7 code is always the fallback", [sub(PYR, 'error=RetainedPrecisionError("G7",match.group(0) if match else "SOURCE_PREVIEW_PHYSICS_INVALID");error.detail=text\n        raise error from exc\n\n\ndef _validate_draft', 'error=RetainedPrecisionError("G7","SOURCE_PREVIEW_PHYSICS_INVALID");error.detail=text\n        raise error from exc\n\n\ndef _validate_draft')], ["py"]),
  ("PM2", "Py: transport G7 drops the base text (detail None)", [sub(PYR, 'if match else "SOURCE_PREVIEW_PHYSICS_INVALID");error.detail=text\n        raise error from exc\n\n\ndef _validate_draft', 'if match else "SOURCE_PREVIEW_PHYSICS_INVALID");error.detail=None\n        raise error from exc\n\n\ndef _validate_draft')], ["py"]),
  ("PM3", "Py: transport projection keeps the receipt member", [sub(PYR, '    projected=deepcopy(snapshot);del projected["retained_precision"]\n    projected["producer"]', '    projected=deepcopy(snapshot)\n    projected["producer"]')], ["py"]),
  ("PM4", "Py: transport G7 runs the raw base dispatch", [sub(PYR, "try:_source_contract(projected,check_receipt=False)", "try:_source_contract(projected)")], ["py"]),
  ("PM5", "Py: transport G7 lets the base ValueError escape to the fallback", [sub(PYR, "    try:_source_contract(projected,check_receipt=False)\n    except ValueError as exc:", "    try:_source_contract(projected,check_receipt=False)\n    except KeyError as exc:")], ["py"]),
  ("PM6", "Py: G1 row-shape condition inverted (transport requires rows, raw skips them)", [sub(PYR, "(not raw or (type(snapshot.get(\"results\")) is list", "(raw or (type(snapshot.get(\"results\")) is list")], ["py"]),
  ("PM7", "Py: raw path checks the publication digest only with an invocation", [sub(PYR, 'if raw:_need(_hash("retained_precision_publication_mp_v2"', 'if invocation is not None:_need(_hash("retained_precision_publication_mp_v2"')], ["py"]),
  ("PM8", "Py: transport dispatch reports the base preview-physics-1 contract", [sub(PYC, "        _retained_transport(source)\n        return PREVIEW_PHYSICS_RETAINED_CONTRACT_ID, PREVIEW_PHYSICS_RETAINED_CONTRACT_SHA256, _PREVIEW_PHYSICS_RETAINED_CONTRACT_PATH", "        _retained_transport(source)\n        return PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_CONTRACT_SHA256, _PREVIEW_PHYSICS_CONTRACT_PATH")], ["py"]),
  ("PM9", "Py: transport carrier swallows reader refusals", [sub(PYC, "        return validate_retained_precision_transport(source)\n    except RetainedPrecisionError as error:\n        raise ValueError(error.detail or error.code) from error", "        return validate_retained_precision_transport(source)\n    except RetainedPrecisionError:\n        return {}")], ["py"]),
  ("PM10", "Py test helper: _expected reads Rust's per-reader field", [sub("tests/test_retained_precision_contract.py", 'return mutation.get("expected_by_reader", {}).get("python", mutation["expected"])', 'return mutation.get("expected_by_reader", {}).get("rust", mutation["expected"])')], ["py"]),
  # Rust src (unchanged by B6; mutated to show the new Rust tests guard it)
  ("RM1", "Rust: for_source_metadata admits a successor without the reader's transport checks", [sub(RSC, "        crate::retained_precision::validate_transport_metadata(source).map_err(retained_error)?;\n", "")], ["rs"]),
  ("RM2", "Rust: validate_transport_metadata skips G1", [sub(RSR, "    g0(source)?;\n    g1(source, false)?;\n", "    g0(source)?;\n")], ["rs"]),
  ("RM3", "Rust: base header reports a bad case as SOURCE_NUMERICAL_QUALITY_INVALID", [sub(RSC, '                    return Err("SOURCE_NUMERICAL_CASE_INVALID".into());', '                    return Err("SOURCE_NUMERICAL_QUALITY_INVALID".into());')], ["rs"]),
  # Data (corpus and case file)
  ("DM1", "Case file: the transport scope sentence reverted to name Python F-U6b-2's code", [jedit(CASES, lambda d: d.__setitem__("scope", d["scope"].replace("Rust and Python the reader's G0 code or their base header code, Python's transport dispatch running the reader's transport validator since B6, F-U6b-2", "Rust the reader's G0 code or its base header code; Python F-U6b-2's code")))], ["ts", "py", "rs"]),
  ("DM2", "Corpus: entry 277 renamed", [jedit(CORPUS, lambda d: d["mutations"][277].__setitem__("id", "g7_not_required_quality_enum_invalid_rv108"))], ["ts", "py", "rs"]),
  ("DM3", "Corpus: entry 290 re-expected as a case defect", [jedit(CORPUS, lambda d: d["mutations"][290].__setitem__("expected", {"gate": "G7", "code": "SOURCE_NUMERICAL_CASE_INVALID"}))], ["ts", "py", "rs"]),
  ("DM4", "Corpus: entry 293 duplicated at the end", [jedit(CORPUS, lambda d: d["mutations"].append(dict(d["mutations"][293], id="g7_source_block_recovery_present_rv108")))], ["ts", "py", "rs"]),
  ("DM5", "Corpus: 277's TS expectation reverted to the base value", [jedit(CORPUS, lambda d: d["mutations"][277]["expected_by_reader"].__setitem__("typescript", {"gate": "G7", "code": "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED"}))], ["ts", "py", "rs"]),
  ("DM6", "Case file: the F-U6b-2 declared difference restored from base", [jedit(CASES, lambda d: d["declared_differences"].insert(1, next(e for e in json.loads((BASE / CASES).read_text())["declared_differences"] if e["id"].startswith("F-U6b-2"))))], ["ts", "py", "rs"]),
  # Head tests against base code (the new tests fail at base where they should)
  ("BT", "Head tests with BASE's TS reader", [from_base(TSR)], ["ts"]),
  ("BP", "Head tests with BASE's Python reader and carrier", [from_base(PYR), from_base(PYC)], ["py"]),
  ("BC", "Head tests with BASE's corpus and case file", [from_base(CORPUS), from_base(CASES)], ["ts", "py", "rs"]),
]
only = sys.argv[1:]
restore()
ALLOW = set(os.environ["LANES"].split(",")) if os.environ.get("LANES") else None
for mid, desc, edits, lanes in MUTANTS:
    if only and mid not in only: continue
    lanes = [l for l in lanes if ALLOW is None or l in ALLOW]
    if not lanes and not os.environ.get("DRY"): continue
    t0 = time.time(); rec = {"id": mid, "description": desc, "lanes": {}}
    try:
        for e in edits: e()
        for lane in ([] if os.environ.get("DRY") else lanes): rec["lanes"][lane] = LANES[lane](mid)
    except AssertionError as e:
        rec["apply_error"] = str(e)
    finally:
        restore()
    rec["seconds"] = round(time.time() - t0)
    if not os.environ.get("DRY"):
        with OUT.open("a") as fh: fh.write(json.dumps(rec) + "\n")
    print(mid, json.dumps({k: {kk: vv for kk, vv in v.items() if kk in ("failed", "differences_from_head", "rc")} for k, v in rec["lanes"].items()}), rec.get("apply_error", ""), flush=True)
print("done")
