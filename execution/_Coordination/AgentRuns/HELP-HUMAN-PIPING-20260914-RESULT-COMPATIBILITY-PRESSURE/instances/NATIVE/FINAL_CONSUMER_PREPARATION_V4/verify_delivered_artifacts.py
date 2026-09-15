#!/usr/bin/env python3
"""Independent verifier for delivered strict analysis/result/stress artifacts."""
from __future__ import annotations
import argparse
from copy import deepcopy
from hashlib import sha256
import json
import os
from pathlib import Path
import sys
from typing import Any

EXPECTED_MEMBERS = ["manifest.json", "stress_neutral_results.csv", "result_rows.json", "unit_system_disclosure.json", "unit_preservation_witnesses.json", "stable_id_map.json", "loss_report.json", "validation_report.json", "diagnostics.json"]
PROFILE = "openpipestress_jcs_ijson_v1"

def load(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))

def digest_bytes(path: Path) -> str:
    return sha256(path.read_bytes()).hexdigest()

def one(rows: list[dict[str, Any]], scope: str) -> dict[str, Any]:
    found = [row for row in rows if row.get("payload_scope") == scope]
    if len(found) != 1:
        raise AssertionError(f"CHECKSUM_CARDINALITY:{scope}:{len(found)}")
    return found[0]

def assert_checked(claim: dict[str, Any], scope: str, object_type: str, ref: str, value: str) -> None:
    actual = (claim.get("algorithm"), claim.get("canonicalization"), claim.get("payload_scope"), claim.get("payload_ref", {}).get("object_type"), claim.get("payload_ref", {}).get("ref"), claim.get("value"))
    expected = ("sha256", PROFILE, scope, object_type, ref, value)
    if actual != expected:
        raise AssertionError(f"CHECKSUM_METADATA_OR_VALUE:{scope}:{actual!r}")

def assert_checked_frontend_ref(claim: dict[str, Any], scope: str, ref_type: str, ref_id: str, value: str) -> None:
    actual = (claim.get("algorithm"), claim.get("canonicalization"), claim.get("payload_scope"), claim.get("payload_ref", {}).get("ref_type"), claim.get("payload_ref", {}).get("ref_id"), claim.get("value"))
    expected = ("sha256", PROFILE, scope, ref_type, ref_id, value)
    if actual != expected:
        raise AssertionError(f"FRONTEND_CHECKSUM_METADATA_OR_VALUE:{scope}:{actual!r}")

def target_at(doc: dict[str, Any], pointer: str) -> dict[str, Any]:
    if not pointer.startswith("/result_envelope/"):
        raise AssertionError("RESULT_TARGET_POINTER_SCOPE")
    value: Any = doc
    for token in pointer[1:].split("/"):
        value = value[int(token)] if isinstance(value, list) else value[token]
    if not isinstance(value, dict):
        raise AssertionError("RESULT_TARGET_POINTER_TYPE")
    return value

def verify_result(doc: dict[str, Any], source: dict[str, Any], canonical_hash) -> dict[str, Any]:
    env = doc["result_envelope"]
    origins = env["reproducibility"]["source_origin_bindings"]
    if len(origins) != 1:
        raise AssertionError("RESULT_ORIGIN_CARDINALITY")
    origin = origins[0]
    source_ref = origin["received_carrier_checksum"]["payload_ref"]
    scope = origin["received_carrier_checksum"]["payload_scope"]
    allowed = {"received_current_legacy_enriched_carrier": "received_current_legacy_enriched_row", "received_current_dimension_absent_carrier": "received_current_dimension_absent_row", "attested_headless_producer_carrier": "raw_source_row"}
    if scope not in allowed:
        raise AssertionError("RESULT_ORIGIN_SCOPE")
    carrier_claim = origin["received_carrier_checksum"]
    assert_checked_frontend_ref(carrier_claim, scope, source_ref["ref_type"], source_ref["ref_id"], canonical_hash(source))
    if origin.get("authentic_producer_available") is False and origin.get("original_producer_checksum") is not None:
        raise AssertionError("RESULT_FALSE_AUTHENTICITY")
    accounts = env["row_accounting"]
    annotations = env["source_annotations"]
    if len(accounts) != len(source["results"]) or len(annotations) != len(source["results"]):
        raise AssertionError("RESULT_ROW_ACCOUNTING_CARDINALITY")
    target_count = sum(len(item["values"]) for item in env["result_sets"]) + len(env["review_evidence"]) + len(env["row_disclosures"])
    if target_count != len(source["results"]):
        raise AssertionError("RESULT_TARGET_ACCOUNTING_CARDINALITY")
    seen_pointers: set[str] = set()
    witnesses_by_index = {item["source_row_index"]: item for item in env["unit_preservation_witnesses"]}
    for index, (row, account) in enumerate(zip(source["results"], accounts)):
        if (account["source_row_index"], account["source_result_id"], account["source_kind"], account["source_field_path"]) != (index, row["id"], row["kind"], f"/results/{index}"):
            raise AssertionError(f"RESULT_SOURCE_ACCOUNTING:{index}")
        pointer = account["target_field_path"]
        if pointer in seen_pointers:
            raise AssertionError("RESULT_TARGET_POINTER_DUPLICATE")
        seen_pointers.add(pointer)
        target = target_at(doc, pointer)
        if target.get("source_row_index") != index:
            raise AssertionError(f"RESULT_TARGET_INDEX:{index}")
        claim = account["received_carrier_row_checksum"]
        assert_checked_frontend_ref(claim, allowed[scope], source_ref["ref_type"], source_ref["ref_id"], canonical_hash(row))
        if account["disposition"] == "disclosed":
            if index in witnesses_by_index:
                raise AssertionError(f"RESULT_DISCLOSURE_HAS_WITNESS:{index}")
        else:
            witness = witnesses_by_index.get(index)
            if witness is None or witness["target_field_path"] != pointer or witness["target_result_ref"] != account["target_ref"] or witness["conversion_performed"] is not False:
                raise AssertionError(f"RESULT_WITNESS_BINDING:{index}")
            target_scope = "derived_quantity_row" if account["disposition"] == "exported_quantity" else "derived_review_row"
            assert_checked_frontend_ref(witness["target_row_checksum"], target_scope, "derivative_document", env["envelope_id"], canonical_hash(target))
    derivative = env["reproducibility"]["derivative_hash"]
    projection = deepcopy(doc)
    del projection["result_envelope"]["reproducibility"]["derivative_hash"]
    assert_checked_frontend_ref(derivative, "derivative_document_excludes_own_hash", "derivative_document", env["envelope_id"], canonical_hash(projection))
    return {"row_count": len(accounts), "origin_class": origin["origin_class"], "authentic_producer_available": origin["authentic_producer_available"]}

def main() -> None:
    p = argparse.ArgumentParser()
    p.add_argument("--working-root", required=True, type=Path)
    p.add_argument("--checked-json-bin", required=True, type=Path)
    p.add_argument("--case-dir", required=True, type=Path)
    p.add_argument("--source-request", required=True, type=Path)
    p.add_argument("--source-output", required=True, type=Path)
    p.add_argument("--stress-members", required=True, type=Path)
    p.add_argument("--composition-receipt", required=True, type=Path)
    p.add_argument("--delivery-receipt", type=Path)
    p.add_argument("--report", required=True, type=Path)
    a = p.parse_args()
    root, checked, case_dir = a.working_root.resolve(), a.checked_json_bin.resolve(), a.case_dir.resolve()
    if not checked.is_file():
        raise SystemExit(f"CHECKED-JSON-AUTHORITY-MISSING:{checked}")
    os.environ["OPENPIPESTRESS_CHECKED_JSON_BIN"] = str(checked)
    sys.path[:0] = [str(root), str(root / "tests")]
    from schema_validation import validate_instance  # noqa: E402
    from core.serialization.canonical_json.adapter import canonical_json_checked_v1, canonical_sha256_checked_v1  # noqa: E402
    from core.analysis_runs.compatibility import verify_analysis_run_record  # noqa: E402
    from core.handoff.stress_neutral import materialized_members_v0_2, validate_stress_neutral_export_package_v0_2  # noqa: E402

    files = {"analysis": case_dir / "analysis_run_v0_2.json", "result": case_dir / "result_export_v0_2.json", "stress": case_dir / "stress_neutral_v0_2.json"}
    docs = {name: load(path) for name, path in files.items()}
    source_request = load(a.source_request)
    source_envelope = load(a.source_output)["mechanics_envelope"]
    source_model = source_request["solve"]["preview_model"]["model"]
    schemas = {"analysis": root / "schemas/analysis_run.schema.json", "result": root / "schemas/results.schema.yaml", "stress": root / "schemas/stress_neutral_export.schema.json"}
    for name, schema_path in schemas.items():
        validate_instance(load(schema_path), docs[name], schema_label=str(schema_path), instance_label=str(files[name]))
    if verify_analysis_run_record(docs["analysis"]) != "match":
        raise AssertionError("ANALYSIS_RECORD_CHECKSUM")
    run = docs["analysis"]["analysis_run"]
    input_manifest_evidence = load(case_dir / "input_manifest.json")
    manifest = input_manifest_evidence["manifest"]
    manifest_digest = canonical_sha256_checked_v1(manifest)
    if input_manifest_evidence["manifest_sha256"] != manifest_digest or input_manifest_evidence["canonical_bytes"] != canonical_json_checked_v1(manifest):
        raise AssertionError("INPUT_MANIFEST_OLD_PROFILE_HASH_BINDING")
    manifest_hashes = run["reproducibility"]["input_manifest_hashes"]
    if len(manifest_hashes) != 1 or manifest_hashes[0] != {"algorithm": "sha256", "canonicalization": "rfc8785_jcs", "payload_ref": input_manifest_evidence["manifest_ref"], "payload_scope": "input_manifest", "value": manifest_digest}:
        raise AssertionError("ANALYSIS_INPUT_MANIFEST_BINDING")
    if len(run["result_refs"]) != len(source_envelope["results"]):
        raise AssertionError("ANALYSIS_ROW_CARDINALITY")
    received = one(run["hashes"], "received_result")
    assert_checked(received, "received_result", "ResultEnvelope", f"result-envelope:{source_envelope['run_id']}", canonical_sha256_checked_v1(source_envelope))
    for index, (row, ref) in enumerate(zip(source_envelope["results"], run["result_refs"])):
        if ref["source_row_index"] != index or ref["result_ref"] != {"object_type": "Result", "ref": row["id"]}:
            raise AssertionError(f"ANALYSIS_RESULT_REF:{index}")
        assert_checked(one(ref["hash_refs"], "result_row"), "result_row", "Result", row["id"], canonical_sha256_checked_v1(row))
    result_summary = verify_result(docs["result"], source_envelope, canonical_sha256_checked_v1)
    model_hash = docs["result"]["result_envelope"]["reproducibility"]["model_hash"]
    if (model_hash.get("algorithm"), model_hash.get("canonicalization"), model_hash.get("payload_ref"), model_hash.get("value")) != ("sha256", PROFILE, {"ref_type": "model_payload", "ref_id": source_envelope["model_ref"]}, canonical_sha256_checked_v1(source_model)):
        raise AssertionError("RESULT_MODEL_HASH_BINDING")
    if docs["stress"]["received_source_checksums"] != run["hashes"]:
        raise AssertionError("STRESS_RECEIVED_METADATA_BINDING")
    validate_stress_neutral_export_package_v0_2(docs["stress"])
    tampered = deepcopy(docs["stress"])
    tampered["result_rows"][0]["value"] = tampered["result_rows"][0]["value"] + 1
    try:
        validate_stress_neutral_export_package_v0_2(tampered)
    except (AssertionError, ValueError) as exc:
        tamper_rejection = str(exc)
    else:
        raise AssertionError("STRESS_COVERED_FIELD_TAMPER_ACCEPTED")
    if len(docs["stress"]["result_rows"]) != len(source_envelope["results"]):
        raise AssertionError("STRESS_SOURCE_ROW_CARDINALITY")
    if {row["result_id"] for row in docs["stress"]["result_rows"]} != {row["id"] for row in source_envelope["results"]}:
        raise AssertionError("STRESS_SOURCE_ROW_IDENTITY")
    contract = load(root / "fixtures/results/semantic_contract_v0_2.json")["rows"]
    contract_by_signature = {(row["kind"], row["unit"], row.get("component")): row for row in contract}
    eligible_ids, solver_mode_ids = set(), set()
    for row in source_envelope["results"]:
        semantic = contract_by_signature.get((row["kind"], row["unit"], (row.get("metadata") or {}).get("component")))
        if semantic is None:
            semantic = contract_by_signature.get((row["kind"], row["unit"], None))
        if semantic is None:
            raise AssertionError(f"STRESS_SEMANTIC_ELIGIBILITY_UNAVAILABLE:{row['id']}")
        if semantic["category"] != "diagnostic_work":
            eligible_ids.add(row["id"])
        if semantic["category"] == "solver_mode":
            solver_mode_ids.add(row["id"])
    witness_ids = {item["result_id"] for item in docs["stress"]["unit_preservation_witnesses"]}
    if witness_ids != eligible_ids:
        raise AssertionError("STRESS_DERIVED_WITNESS_ELIGIBILITY")
    withheld_ids = {item.get("source", {}).get("ref") for item in docs["stress"]["diagnostics"] if item.get("code") == "SN-UNIT-WITNESS-WITHHELD-DIAGNOSTIC-WORK"}
    if solver_mode_ids & withheld_ids:
        raise AssertionError("STRESS_SOLVER_MODE_MISLABELLED_DIAGNOSTIC_WORK")
    expected_member_bytes = materialized_members_v0_2(docs["stress"])
    actual_names = sorted(item.name for item in a.stress_members.iterdir() if item.is_file())
    if actual_names != sorted(EXPECTED_MEMBERS):
        raise AssertionError(f"STRESS_MATERIALIZED_INVENTORY:{actual_names}")
    for name in EXPECTED_MEMBERS:
        if (a.stress_members / name).read_bytes() != expected_member_bytes[name]:
            raise AssertionError(f"STRESS_MATERIALIZED_BYTES:{name}")
    receipt = load(a.composition_receipt)
    if receipt["captured_request"]["sha256"] != digest_bytes(a.source_request) or receipt["captured_output"]["sha256"] != digest_bytes(a.source_output):
        raise AssertionError("COMPOSITION_RECEIPT_SOURCE_BYTES")
    for name, filename in (("analysis_run_v0_2.json", files["analysis"]), ("result_export_v0_2.json", files["result"]), ("stress_neutral_v0_2.json", files["stress"])):
        if receipt["artifact_sha256"][name] != digest_bytes(filename):
            raise AssertionError(f"COMPOSITION_RECEIPT_BYTES:{name}")
    delivery = None
    if a.delivery_receipt:
        delivery = load(a.delivery_receipt)
        for item in delivery.get("artifacts", []):
            delivered = Path(item["path"])
            if digest_bytes(delivered) != item["sha256"]:
                raise AssertionError(f"DELIVERY_RECEIPT_BYTES:{delivered}")
    report = {"status": "PASS", "checked_authority": {"path": str(checked), "sha256": digest_bytes(checked), "profile": PROFILE}, "schemas": {name: str(path) for name, path in schemas.items()}, "analysis": {"run_id": run["run_id"], "row_count": len(run["result_refs"]), "record_checksum": "match", "received_result_checksum": "match", "row_checksums": "match", "input_manifest_old_profile": "rfc8785_jcs_unchanged_and_verified"}, "result": {**result_summary, "model_hash": "verified_without_profile_relabel"}, "stress": {"row_count": len(docs["stress"]["result_rows"]), "derived_witness_eligible_count": len(eligible_ids), "solver_mode_discrete_ids": sorted(solver_mode_ids), "member_inventory": EXPECTED_MEMBERS, "manifest_seed": "verified_by_owning_validator", "complete_package_checksum": "verified_by_owning_validator", "materialized_member_bytes": "match_owning_product_api", "covered_field_tamper_rejected": tamper_rejection}, "composition_receipt": str(a.composition_receipt.resolve()), "delivery_receipt": str(a.delivery_receipt.resolve()) if a.delivery_receipt else None}
    a.report.parent.mkdir(parents=True, exist_ok=True)
    a.report.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps({"status": "PASS", "report": str(a.report.resolve())}, sort_keys=True))

if __name__ == "__main__":
    main()
