"""B6 (I83) mutants: one edit each, applied to the mutant tree M (a copy of the head candidate), run in the
named lanes, then restored from the head file. Usage: mutants.py <M P root> <head P root> <out.jsonl> [ids...]
A kill is a failing test; the failing test ids are recorded."""
import json, os, re, shutil, subprocess, sys, time
from pathlib import Path
M, HEAD, OUT = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3])
ONLY = set(sys.argv[4:])
WT = os.environ["WT"]; S = os.environ["S"]; VENV = os.environ["VENV"]; LOCK = f"{WT}/guard/cargo_job.lock"
CORPUS = "fixtures/results/retained_precision_cases.json"
TSR = "apps/desktop/src/features/results/retainedPrecision.ts"
PYR = "core/analysis_runs/retained_precision.py"
PYC = "core/analysis_runs/compatibility.py"
G7 = lambda code: {"gate": "G7", "code": code}

def corpus_edit(fn):
    def apply():
        p = M / CORPUS; c = json.loads(p.read_text()); fn(c); p.write_text(json.dumps(c, indent=2) + "\n")
    return apply

def text_edit(rel, old, new):
    def apply():
        p = M / rel; t = p.read_text(); assert t.count(old) == 1, (rel, old[:80]); p.write_text(t.replace(old, new))
    return apply

def swap(c, i, j): c["mutations"][i], c["mutations"][j] = c["mutations"][j], c["mutations"][i]
def repoint(c):
    m = c["mutations"][290]; assert m["id"] == "g7_quality_status_invalid"
    m["edits"] = [{"path": ["formulation_basis", "limitations"], "op": "set", "value": []}]; m["expected"] = G7("SOURCE_FORMULATION_BASIS_UNSUPPORTED")

TS_CASE_LOOP = "  for (const c of q.cases) {\n    if (!exact(c, ['basis_ref',"
MUTANTS = [
    ("N0", "no mutation (the unmutated head, every lane)", lambda: None, ["py", "pyc", "ts", "rs"]),
    # Item 4 (RV94 N-5) and the per-reader pins: corpus mutants.
    ("C1", "277's expected_by_reader.python changed (R34)", corpus_edit(lambda c: c["mutations"][277]["expected_by_reader"].__setitem__("python", G7("SOURCE_NUMERICAL_QUALITY_INVALID"))), ["py", "ts"]),
    ("C2", "139's expected_by_reader.python changed (R34)", corpus_edit(lambda c: c["mutations"][139]["expected_by_reader"].__setitem__("python", G7("SOURCE_PREVIEW_PHYSICS_EXTREMA_BOUNDS"))), ["py", "ts"]),
    ("C3", "277's shared expected changed (R35)", corpus_edit(lambda c: c["mutations"][277].__setitem__("expected", G7("SOURCE_PRODUCER_CONTRACT_UNSUPPORTED"))), ["py", "ts"]),
    # Item 1 and the 07m slice: self-consistent corpus mutants the per-entry tests cannot see.
    ("C4", "mutations 277 and 286 swapped (each entry self-consistent)", corpus_edit(lambda c: swap(c, 277, 286)), ["py", "ts", "rs"]),
    ("C5", "entry 290 re-pointed self-consistently to the formulation class", corpus_edit(repoint), ["py", "ts", "rs"]),
    ("C6", "the last 07m entry dropped", corpus_edit(lambda c: c["mutations"].pop()), ["py", "ts", "rs"]),
    # Item 2: the TS reader's G7 header code.
    ("T1", "G7 header refusal back to SOURCE_PRODUCER_CONTRACT_UNSUPPORTED", text_edit(TSR, "throw new RetainedPrecisionError(gate, baseHeaderCode(base));", "throw new RetainedPrecisionError(gate, 'SOURCE_PRODUCER_CONTRACT_UNSUPPORTED');"), ["ts"]),
    ("T2", "the case loop removed", text_edit(TSR, TS_CASE_LOOP, "  for (const c of [] as Obj[]) {\n    if (!exact(c, ['basis_ref',"), ["ts"]),
    ("T3", "the case rule's exact-keys part dropped", text_edit(TSR, "    if (!exact(c, ['basis_ref', 'structural_status', 'solve_quality', 'model_matrix_fidelity', 'accuracy_evidence', 'evidence_refs'])\n      || !exact(c.basis_ref", "    if (!isObj(c)\n      || !exact(c.basis_ref"), ["ts"]),
    ("T3b", "the case rule's structural_status part dropped", text_edit(TSR, "      || !['passive_model_basis', 'physical_mechanism_witnessed', 'negative_energy_witnessed', 'numerically_unresolved'].includes(c.structural_status)\n", ""), ["ts"]),
    ("T3c", "the case rule's model_matrix_fidelity part dropped", text_edit(TSR, "      || !['represented_equations_retained', 'assembly_loss_detected', 'assembly_uncertainty', 'not_assessed'].includes(c.model_matrix_fidelity)\n", ""), ["ts"]),
    ("T4", "the evidence_refs part dropped", text_edit(TSR, "      || !Array.isArray(c.evidence_refs) || !c.evidence_refs.every(text)) return 'SOURCE_NUMERICAL_CASE_INVALID';", "      ) return 'SOURCE_NUMERICAL_CASE_INVALID';"), ["ts"]),
    ("T5", "the quality-level check dropped", text_edit(TSR, "|| !QUALITY_STATUSES.includes(q.status) || !Array.isArray(q.cases)) return 'SOURCE_NUMERICAL_QUALITY_INVALID';", "|| !Array.isArray(q.cases)) return 'SOURCE_NUMERICAL_QUALITY_INVALID';"), ["ts"]),
    ("T6", "the formulation check dropped", text_edit(TSR, "    || !f.limitations.every(text)) return 'SOURCE_FORMULATION_BASIS_UNSUPPORTED';", "    || !f.limitations.every(text)) return 'SOURCE_PRODUCER_CONTRACT_UNSUPPORTED';"), ["ts"]),
    ("T7", "the evidence-required check dropped", text_edit(TSR, "  if (!isObj(p.contract_evidence)) return 'SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED';", "  if (false) return 'SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED';"), ["ts"]),
    ("T8", "the source_block_recovery check dropped", text_edit(TSR, "  if (Object.hasOwn(p, 'source_block_recovery')) return 'SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN';", "  if (false) return 'SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN';"), ["ts"]),
    ("T9", "contract_evidence and source_block_recovery checks in Rust's order", text_edit(TSR, "  if (!isObj(p.contract_evidence)) return 'SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED';\n  if (Object.hasOwn(p, 'source_block_recovery')) return 'SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN';", "  if (Object.hasOwn(p, 'source_block_recovery')) return 'SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN';\n  if (!isObj(p.contract_evidence)) return 'SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED';"), ["ts"]),
    ("T10", "the carrier_evidence branch dropped", text_edit(TSR, "  if (Object.hasOwn(p, 'carrier_evidence')) return 'SOURCE_PRODUCER_CONTRACT_UNSUPPORTED';\n", ""), ["ts"]),
    # Item 3: the Python transport validator.
    ("X1", "transport dispatch refuses again (the old F-U6b-2 behaviour)", text_edit(PYC, "        _retained_transport(source)\n        return PREVIEW", "        raise ValueError(\"SOURCE_PRODUCER_CONTRACT_UNSUPPORTED\")\n        return PREVIEW"), ["pyc"]),
    ("X2", "transport skips the receipt digest", text_edit(PYR, '        _need(_hash("retained_precision_receipt_mp_v2",body)==receipt["receipt_sha256"],gate,"RECEIPT_MISMATCH")', '        _need(not raw or _hash("retained_precision_receipt_mp_v2",body)==receipt["receipt_sha256"],gate,"RECEIPT_MISMATCH")'), ["pyc"]),
    ("X3", "transport skips the base transport metadata (G7)", text_edit(PYR, '            gate="G7";_transport_g7(snapshot)', '            gate="G7"'), ["pyc"]),
    ("X4", "transport reads eligible", text_edit(PYR, 'return {"invocation_bound":False,"numerical_eligible":False,"standing":"needs_recompute","publication_sha256":body["publication_sha256"],"classifications":[]}', 'return {"invocation_bound":False,"numerical_eligible":True,"standing":"needs_recompute","publication_sha256":body["publication_sha256"],"classifications":[]}'), ["pyc"]),
    ("X5", "transport requires the raw rows", text_edit(PYR, '(not raw or (type(snapshot.get("results")) is list', '(True and (type(snapshot.get("results")) is list'), ["pyc"]),
    ("X6", "transport checks the publication digest", text_edit(PYR, '        if raw:_need(_hash("retained_precision_publication_mp_v2"', '        if True:_need(_hash("retained_precision_publication_mp_v2"'), ["pyc"]),
    ("X7", "the carrier keeps the code, not the base text", text_edit(PYC, "        return validate_retained_precision_transport(source)\n    except RetainedPrecisionError as error:\n        raise ValueError(error.detail or error.code) from error", "        return validate_retained_precision_transport(source)\n    except RetainedPrecisionError as error:\n        raise ValueError(error.code) from error"), ["pyc"]),
    ("X8", "transport skips G2", text_edit(PYR, 'gate="G2";_encoding(receipt,schema);_need(not _negative_zero(receipt),gate,"ENCODING_MISMATCH");', 'gate="G2";raw and _encoding(receipt,schema);_need(not raw or not _negative_zero(receipt),gate,"ENCODING_MISMATCH");'), ["pyc"]),
]

def run(cmd, cwd, log):
    with open(log, "w") as f:
        return subprocess.run(cmd, cwd=cwd, stdout=f, stderr=subprocess.STDOUT, env=dict(os.environ)).returncode

def lane(name, mid):
    log = f"{S}/logs/mutants/{mid}_{name}.log"
    if name in ("py", "pyc"):
        files = ["tests/test_retained_precision_carriers.py"] if name == "pyc" else ["tests/test_retained_precision_contract.py", "-k", "snapshot_07 or (first_failure_controls and g7)"]
        rc = run(["/usr/bin/lockf", "-k", LOCK, f"{VENV}/bin/python", "-m", "pytest", "-p", "no:cacheprovider", f"--basetemp={S}/tmp/mut/{mid}", "-q", "-rf", *files], M, log)
        failed = re.findall(r"^FAILED (\S+)", open(log).read(), re.M)
    elif name == "ts":
        rc = run(["/usr/bin/lockf", "-k", LOCK, "../../node_modules/.bin/vitest", "run", "src/features/results/retainedPrecision.test.ts", "--reporter=json", f"--outputFile.json={log}.json"], M / "apps/desktop", log)
        res = json.load(open(f"{log}.json")) if os.path.exists(f"{log}.json") else {"testResults": []}
        failed = [a["fullName"] for r in res["testResults"] for a in r["assertionResults"] if a["status"] == "failed"]
    else:
        env_rs = ["env", "-u", "RUSTFLAGS", "-u", "CARGO_ENCODED_RUSTFLAGS", "CARGO_BUILD_JOBS=4", "RUST_TEST_THREADS=2", f"CARGO_TARGET_DIR={WT}/targets/i83-b6/rx-mut"]
        rc = run([*env_rs, f"{WT}/tools/t3_cargo.sh", "test", "--locked", "--offline", "--no-fail-fast", "--test", "retained_precision_contract"], M / "core/reporting/result_export", log)
        failed = re.findall(r"^test (\S+) \.\.\. FAILED", open(log).read(), re.M)
    return {"lane": name, "rc": rc, "killed": rc != 0, "failed": failed}

os.makedirs(f"{S}/logs/mutants", exist_ok=True); os.makedirs(f"{S}/tmp/mut", exist_ok=True)
with open(OUT, "a") as out:
    for mid, what, apply, lanes in MUTANTS:
        if ONLY and mid not in ONLY: continue
        touched = set() if mid == "N0" else {CORPUS} if mid.startswith("C") else {TSR} if mid.startswith("T") else {PYR, PYC}
        apply()
        try:
            results = [lane(name, mid) for name in lanes]
        finally:
            for rel in touched: shutil.copyfile(HEAD / rel, M / rel)
        rec = {"id": mid, "mutation": what, "results": results, "utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())}
        out.write(json.dumps(rec) + "\n"); out.flush()
        print(mid, [(r["lane"], "killed" if r["killed"] else "SURVIVED", len(r["failed"])) for r in results], flush=True)
