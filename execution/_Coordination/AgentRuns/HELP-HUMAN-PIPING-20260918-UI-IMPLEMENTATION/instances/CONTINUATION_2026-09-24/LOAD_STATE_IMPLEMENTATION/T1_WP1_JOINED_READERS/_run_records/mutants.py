"""One mutant per new check of the load-reference-source-1 readers, in both languages.

Usage: python mutants.py <scratch WORKING_ROOT copy> <log dir>
Each mutant replaces one anchor, which must occur exactly once, runs the
maintained test for its language, and restores the file (byte-checked).
A mutant is killed when the maintained test fails. The worktree is never mutated.
"""
import hashlib
import os
import subprocess
import sys
from pathlib import Path

ROOT = Path(sys.argv[1]).resolve()
LOGS = Path(sys.argv[2]).resolve()
LOGS.mkdir(parents=True, exist_ok=True)
R = "core/reporting/result_export/src/load_reference_source.rs"
L = "core/reporting/result_export/src/load_reference.rs"
S = "core/reporting/result_export/src/semantic_contract.rs"
D = "core/reporting/result_export/src/derivative.rs"
PR = "core/analysis_runs/load_reference_source.py"
PL = "core/analysis_runs/load_reference_evidence.py"
PC = "core/analysis_runs/compatibility.py"

MUTANTS = [
    # Rust
    ("RS-J0-IDENTITY", R, 'source["producer"]["semantic_contract_id"] == CONTRACT_ID\n            && source["formulation_basis"]["profile_id"] == PROFILE,', "true,"),
    ("RS-S1-RECEIPT-REQUIRED", L, 'source.get("source_block_recovery").is_some(),\n            "JOIN_RECEIPT_REQUIRED",', 'true,\n            "JOIN_RECEIPT_REQUIRED",'),
    ("RS-S7-RECOVERY-METHOD-KEY", L, '.chain(joined.then_some("recovery_method"))', ".chain(None)"),
    ("RS-S10-SELECTED-RECOVERY", L, 'recovery["status"] == "selected" && recovery["method"] == EXACT_METHOD,', "true,"),
    ("RS-S10-SELECTED-MODE", L, '(selected\n            && matches!(\n                solve["requested_mode"].as_str(),\n                Some("sparse_interactive" | "dense_scrutiny")\n            )))', "(selected))"),
    ("RS-S10B-RECOVERY-METHOD", L, 'case["recovery_method"] == record["solve"]["recovery_method"],', "true,"),
    ("RS-S10B-SELECTION-REQUIRED", L, "records.iter().any(|r| is_selected(r)),", "true,"),
    ("RS-S13-SELECTED-HIT", L, "            require(\n                hits.len() == 1", '            require(\n                name == "JOIN_SELECTED_DIAGNOSTIC" || hits.len() == 1'),
    ("RS-S13-SELECTED-COUNT", L, 'diagnostics.iter().filter(|d| d["code"] == SELECTED).count() == selected,', "true,"),
    ("RS-R1-POLICY", R, 'require(body["policy"] == POLICY, "JOIN_RECEIPT_POLICY")?;', 'require(true, "JOIN_RECEIPT_POLICY")?;'),
    ("RS-R2-SHAPE", R, "crate::physics_source::validate_receipt_shape(&shaped).is_ok(),", "true,"),
    ("RS-R3-RECEIPT-HASH", R, 'receipt["receipt_sha256"] == domain_hash("source_blocks_receipt_v1", body)?,', "true,"),
    ("RS-R4-PUBLICATION-HASH", R, 'body["publication_sha256"]\n                == domain_hash("source_blocks_publication_v1", &publication)?,', "true,"),
    ("RS-R5-CASE-ORDER", R, "    require(\n        cases\n            .iter()", "    require(\n        true || cases\n            .iter()"),
    ("RS-R6-RECOVERY-METHOD", R, 'case["selected_method"] == exact["recovery_method"],', "true,"),
    ("RS-R6-REQUESTED-MODE", R, 'case["requested_mode"] == record["solve"]["requested_mode"],', "true,"),
    ("RS-R7-PHYSICAL-HASH", R, 'case["physical_evidence_sha256"] == domain_hash(CASE_EVIDENCE_DOMAIN, &proof)?,', "true,"),
    ("RS-R7-RECORD-BINDING", R, '            "load_reference_state": record,\n', ""),
    ("RS-J3-PROJECTED-RAW", R, "crate::physics_source::validate(&projected, None).map(|_| ())", "Ok::<(), String>(())"),
    ("RS-J3-PROJECTED-TRANSPORT", R, "crate::physics_source::validate_transport_metadata(&projected)", "Ok::<(), String>(())"),
    ("RS-TABLE-HASH", R, "if format!(\"{:x}\", Sha256::digest(bytes)) != TABLE_SHA256 {", "if false {"),
    ("RS-DISPATCH-PROFILE", S, "Some(LOAD_REFERENCE_SOURCE_ID) => LOAD_REFERENCE_SOURCE_PROFILE,", "Some(LOAD_REFERENCE_SOURCE_ID) => LOAD_REFERENCE_PROFILE,"),
    ("RS-DISPATCH-CARRIER", S, '(p["semantic_contract_id"] == LOAD_REFERENCE_ID\n                || p["semantic_contract_id"] == LOAD_REFERENCE_SOURCE_ID)', '(p["semantic_contract_id"] == LOAD_REFERENCE_ID)'),
    ("RS-DISPATCH-VALIDATE", S, "            validate_load_reference_source_evidence(source)?;", "            let _ = validate_load_reference_source_evidence(source);"),
    ("RS-DERIVATIVE-RECEIPT", D, '                    | crate::semantic_contract::LOAD_REFERENCE_SOURCE_ID\n            )\n        ) {\n            e["source_block_recovery"]', '            )\n        ) {\n            e["source_block_recovery"]'),
    # Python
    ("PY-J0-IDENTITY", PR, '_require(lr._eq(lr._get(lr._get(source, "producer"), "semantic_contract_id"), CONTRACT_ID) and lr._eq(lr._get(lr._get(source, "formulation_basis"), "profile_id"), PROFILE), "JOIN_IDENTITY")', '_require(True, "JOIN_IDENTITY")'),
    ("PY-S1-RECEIPT-REQUIRED", PL, '_require(isinstance(source, Mapping) and "source_block_recovery" in source, "JOIN_RECEIPT_REQUIRED")', '_require(True, "JOIN_RECEIPT_REQUIRED")'),
    ("PY-S7-RECOVERY-METHOD-KEY", PL, 'case_keys = CASE_KEYS + (["recovery_method"] if joined else [])', "case_keys = CASE_KEYS"),
    ("PY-S10-SELECTED-RECOVERY", PL, '_require(_eq(recovery["status"], "selected") and _eq(recovery["method"], EXACT_METHOD), "SOURCE_RECOVERY")', '_require(True, "SOURCE_RECOVERY")'),
    ("PY-S10-SELECTED-MODE", PL, 'or (selected and pair[0] in {"sparse_interactive", "dense_scrutiny"}))', "or selected)"),
    ("PY-S10B-RECOVERY-METHOD", PL, '_require(_same(case["recovery_method"], _get(record["solve"], "recovery_method")), "JOIN_RECOVERY_METHOD")', '_require(True, "JOIN_RECOVERY_METHOD")'),
    ("PY-S10B-SELECTION-REQUIRED", PL, '_require(any(_is_selected(record) for record in records), "JOIN_SELECTION_REQUIRED")', '_require(True, "JOIN_SELECTION_REQUIRED")'),
    ("PY-S13-SELECTED-HIT", PL, "            _require(len(hits) == 1 and", '            _require(name == "JOIN_SELECTED_DIAGNOSTIC" or len(hits) == 1 and'),
    ("PY-S13-SELECTED-COUNT", PL, '_require(sum(1 for d in diagnostics if _eq(_get(d, "code"), SELECTED)) == selected, "JOIN_SELECTED_DIAGNOSTIC")', '_require(True, "JOIN_SELECTED_DIAGNOSTIC")'),
    ("PY-R1-POLICY", PR, '_require(lr._eq(lr._get(body, "policy"), POLICY), "JOIN_RECEIPT_POLICY")', '_require(True, "JOIN_RECEIPT_POLICY")'),
    ("PY-R2-SHAPE", PR, "        validate_receipt_shape(shaped)\n", "        pass\n"),
    ("PY-R3-RECEIPT-HASH", PR, '_require(receipt["receipt_sha256"] == domain_hash("source_blocks_receipt_v1", body), "JOIN_RECEIPT_HASH")', '_require(True, "JOIN_RECEIPT_HASH")'),
    ("PY-R4-PUBLICATION-HASH", PR, '_require(body["publication_sha256"] == domain_hash("source_blocks_publication_v1", publication), "JOIN_PUBLICATION_HASH")', '_require(True, "JOIN_PUBLICATION_HASH")'),
    ("PY-R5-CASE-ORDER", PR, '_require([case["basis_ref"]["ref_id"] for case in cases] == exact_ids and [record["load_case_id"] for record in records] == exact_ids, "JOIN_CASE_ORDER")', '_require(True, "JOIN_CASE_ORDER")'),
    ("PY-R6-RECOVERY-METHOD", PR, '_require(case["selected_method"] == exact_case["recovery_method"], "JOIN_RECOVERY_METHOD")', '_require(True, "JOIN_RECOVERY_METHOD")'),
    ("PY-R6-REQUESTED-MODE", PR, '_require(case["requested_mode"] == record["solve"]["requested_mode"], "JOIN_REQUESTED_MODE")', '_require(True, "JOIN_REQUESTED_MODE")'),
    ("PY-R7-PHYSICAL-HASH", PR, '_require(case["physical_evidence_sha256"] == domain_hash(CASE_EVIDENCE_DOMAIN, proof), "JOIN_PHYSICAL_CASE_HASH")', '_require(True, "JOIN_PHYSICAL_CASE_HASH")'),
    ("PY-R7-RECORD-BINDING", PR, ', "load_reference_state": record}', "}"),
    ("PY-J3-PROJECTED-RAW", PR, "            validate_physics_source(projected)\n", "            pass\n"),
    ("PY-J3-PROJECTED-TRANSPORT", PR, "            validate_transport_metadata(projected)\n", "            pass\n"),
    ("PY-TABLE-HASH", PR, "if hashlib.sha256(data).hexdigest() != TABLE_SHA256:", "if False:"),
    ("PY-DISPATCH-PROFILE", PC, "LOAD_REFERENCE_SOURCE_PROFILE if joined else", "LOAD_REFERENCE_PROFILE if joined else"),
    ("PY-DISPATCH-VALIDATE", PC, "            validate_load_reference_source_evidence(source)\n", "            pass\n"),
    ("PY-ANALYSIS-RUN-EVIDENCE", PC, 'if contract_id in {PHYSICS_SOURCE_CONTRACT_ID, LOAD_REFERENCE_SOURCE_CONTRACT_ID}:\n        envelope["analysis_run"]["contract_evidence"]', 'if contract_id == PHYSICS_SOURCE_CONTRACT_ID:\n        envelope["analysis_run"]["contract_evidence"]'),
    ("PY-ANALYSIS-RUN-RECEIPT", PC, 'if contract_id in {SOURCE_BLOCKS_CONTRACT_ID, PHYSICS_SOURCE_CONTRACT_ID, LOAD_REFERENCE_SOURCE_CONTRACT_ID}:\n        envelope["analysis_run"]["source_block_recovery"]', 'if contract_id in {SOURCE_BLOCKS_CONTRACT_ID, PHYSICS_SOURCE_CONTRACT_ID}:\n        envelope["analysis_run"]["source_block_recovery"]'),
]

ENV = dict(os.environ, PYTHONDONTWRITEBYTECODE="1")
RUST = ["cargo", "+1.97.1", "test", "--locked", "--offline", "-j", "2", "--test", "load_reference_source_contract"]
PYTHON = [sys.executable, "-m", "pytest", "-x", "-q", "-p", "no:cacheprovider", "tests/test_load_reference_source_readers.py"]
only = set(sys.argv[3:])
summary = []
for mid, rel, anchor, replacement in MUTANTS:
    if only and mid not in only:
        continue
    path = ROOT / rel
    original = path.read_bytes()
    text = original.decode()
    assert text.count(anchor) == 1, (mid, text.count(anchor))
    path.write_text(text.replace(anchor, replacement))
    try:
        if mid.startswith("RS-"):
            run = subprocess.run(RUST, cwd=ROOT / "core/reporting/result_export", env=ENV, capture_output=True, text=True)
        else:
            run = subprocess.run(PYTHON, cwd=ROOT, env=ENV, capture_output=True, text=True)
    finally:
        path.write_bytes(original)
    assert hashlib.sha256(path.read_bytes()).hexdigest() == hashlib.sha256(original).hexdigest()
    (LOGS / f"{mid}.log").write_text(run.stdout[-20000:] + "\n--- stderr ---\n" + run.stderr[-20000:])
    killed = run.returncode != 0
    failing = [line for line in (run.stdout + run.stderr).splitlines() if "panicked at" in line or line.startswith("FAILED") or "error[" in line]
    summary.append(f"{mid}\t{'KILLED' if killed else 'SURVIVED'}\t{failing[:2]}")
    print(summary[-1], flush=True)
(LOGS / "summary.txt").write_text("\n".join(summary) + "\n")
