#!/usr/bin/env python3
"""Current DEL-17-06 construction, member, CSV and data-boundary controls.

E3 retires the 0.1/0.2 construction and writer APIs. Their format-specific
positive fixtures, inferred-family examples and non-JCS hash expectation no
longer describe the current source-bound 0.3 builder. The reusable input
fixture below is unchanged because the fourteen E3 byte witnesses use it.
Current packets keep the substantive privacy, witness, relational, CSV and
checksum checks; current write_materialized_members writes all nine members.
"""
from __future__ import annotations

from copy import deepcopy
import hashlib
import json
from pathlib import Path
import sys
import pytest

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.analysis_runs.compatibility import build_analysis_run_v0_3
from core.handoff.stress_neutral import package as sn
from core.security.redaction.route_control import control_route_export
from core.serialization.canonical_json.adapter import canonical_json_checked_v1, canonical_sha256_checked_v1
from schema_validation import validate_instance, validate_schema_document, walk_strings
from preview_transport_source import source, sync_headline

SCHEMA_PATH = ROOT / "schemas/stress_neutral_export.schema.json"
CSV_HEADER = ",".join(sn.CSV_COLUMNS)
FORBIDDEN_PAYLOAD_TEXT = {"real client", "asme table", "b31j", "cert" + "ified by openpipestress", "code " + "compliant", "professional " + "acceptance"}


def ref(object_type: str, value: str) -> dict[str, str]:
    return {"object_type": object_type, "ref": value}

def source_payload() -> dict[str, object]:
    return {
        "export_id": "stress-neutral:invented-del-17-06",
        "source_result_ref": ref("ResultEnvelope", "result-envelope:invented-del-17-06"),
        "source_run_ref": ref("AnalysisRun", "run:invented-del-17-06"),
        "source_model_ref": ref("Model", "model:invented-del-17-06"),
        "source_hashes": [
            {
                "algorithm": "sha256",
                "canonicalization": "deterministic_sorted_compact_json_payload_hash",
                "payload_ref": ref("ResultEnvelope", "result-envelope:invented-del-17-06"),
                "payload_scope": "source_result_envelope",
                "value": "sha256:" + "2" * 64,
            }
        ],
        "result_rows": [
            {
                "result_id": "result:invented:force:P001:axial",
                "canonical_ref": ref("Result", "result:invented:force:P001:axial"),
                "row_kind": "result_value",
                "result_family": "force",
                "load_case_ref": ref("LoadCase", "load:invented:operating"),
                "station_ref": ref("Station", "station:invented:P001:end-i"),
                "component_ref": ref("PipeElement", "pipe:invented:001"),
                "value": 12.5,
                "unit": "N",
                "dimension": "force",
                "source_result_ref": ref("Result", "result:invented:force:P001:axial"),
                "correlation_status": "canonical_id_map",
            },
            {
                "result_id": "result:invented:stress:P001:sustained",
                "canonical_ref": ref("Result", "result:invented:stress:P001:sustained"),
                "row_kind": "result_value",
                "result_family": "stress",
                "load_case_ref": ref("LoadCase", "load:invented:sustained"),
                "station_ref": ref("Station", "station:invented:P001:midspan"),
                "component_ref": ref("PipeElement", "pipe:invented:001"),
                "value": 3450.0,
                "unit": "Pa",
                "dimension": "stress",
                "source_result_ref": ref("Result", "result:invented:stress:P001:sustained"),
                "correlation_status": "canonical_id_map",
            },
        ],
        "stable_id_map": [
            {
                "canonical_ref": ref("Result", "result:invented:force:P001:axial"),
                "export_ref": ref("StressNeutralRow", "result:invented:force:P001:axial"),
                "mapping_status": "mapped",
                "loss_category": "exported",
            },
            {
                "canonical_ref": ref("Result", "result:invented:stress:P001:sustained"),
                "export_ref": ref("StressNeutralRow", "result:invented:stress:P001:sustained"),
                "mapping_status": "mapped",
                "loss_category": "exported",
            },
        ],
        "loss_report": [
            {
                "loss_id": "loss:invented:stress-neutral-exported",
                "category": "exported",
                "severity": "info",
                "affected_refs": [ref("ResultEnvelope", "result-envelope:invented-del-17-06")],
                "target_artifact_ref": ref("StressNeutralExportPackage", "stress-neutral:invented-del-17-06"),
                "reason": "Invented result rows exported to stress-neutral CSV/JSON for deterministic package testing.",
                "source_basis_ref": ref("Deliverable", "DEL-17-02"),
                "downstream_implication": "Does not define comparison pass/fail or professional reliance semantics.",
            }
        ],
        "unresolved_assumption_refs": [
            ref("Assumption", "assumption:invented-result-review"),
        ],
        "reproducibility_refs": [
            ref("AnalysisRun", "run:invented-del-17-06"),
            ref("AuditManifest", "audit-manifest:invented-del-17-06"),
        ],
    }


def prepared(count=2, *, values=None, diagnostic_work=False, change_source=None):
    """Invented transport statements, with the actual Rust reader and source hash."""
    raw = source()
    raw["results"] = [{"id": f"result:transport:{index:03d}", "kind": "displacement_magnitude",
        "entity_ref": "node:transport", "value": float(index) if values is None else values[index], "unit": "mm"} for index in range(count)]
    if values is None and count == 2:
        raw["results"][0]["value"], raw["results"][1]["value"] = 12.5, 3450.0
    if diagnostic_work:
        for row in raw["results"][-2:]:
            row.update(kind="nonlinear_support_free_dof_work_residual", unit="N*m", metadata={"component": "free_dof_work_residual"})
    sync_headline(raw)
    if change_source:
        change_source(raw)
    analysis = build_analysis_run_v0_3(raw, input_manifest_ref=ref("InputManifest", "manifest:transport"), input_manifest_hash="1" * 64)
    args = source_payload()
    args.update(source_result_ref=ref("ResultEnvelope", f"result-envelope:{raw['run_id']}"),
        source_run_ref=ref("AnalysisRun", raw["run_id"]), source_model_ref=ref("Model", raw["model_ref"]),
        source_hashes=deepcopy(analysis["analysis_run"]["hashes"]))
    args["reproducibility_refs"] = [deepcopy(args["source_run_ref"]), ref("AuditManifest", "audit-manifest:invented-del-17-06")]
    # Interpret each row through the already admitted source/table; no fabricated
    # table identity or unit family supplies the source reader's admission.
    _, _, table = sn._source_contract(raw)
    args["result_rows"] = [sn._source_row_projection(raw, row, table) for row in raw["results"]]
    args["stable_id_map"] = [{"canonical_ref": row["canonical_ref"],
        "export_ref": ref("StressNeutralRow", row["result_id"]), "mapping_status": "mapped", "loss_category": "exported"} for row in args["result_rows"]]
    return raw, analysis, args


def build_from_source(**kwargs):
    raw, analysis, args = prepared(**kwargs)
    return sn.build_stress_neutral_export_package(source_envelope=raw, analysis_record=analysis, **args)


def rehash_packet(packet):
    """Coherently rehash an attack without repairing its received CSV or claims."""
    members = sn.materialized_members(packet)
    checks = [item for item in packet["manifest"]["checksums"] if item["payload_ref"]["ref"] != "manifest.json"]
    for claim in checks:
        claim["value"] = hashlib.sha256(members[claim["payload_ref"]["ref"]]).hexdigest()
    checks.append(sn._checked(sn._manifest_seed(packet, checks), "manifest.json", "manifest_seed"))
    packet["manifest"]["checksums"] = checks
    packet["manifest"]["package_members"] = [{"filename": name, "checksum": deepcopy(next(c for c in checks if c["payload_ref"]["ref"] == name))} for name in sn.MEMBERS]
    packet["package_checksum"]["value"] = canonical_sha256_checked_v1(sn.package_projection(packet))


def test_current_builder_is_deterministic_source_bound_and_schema_conformant():
    raw, analysis, args = prepared()
    before = deepcopy((raw, analysis, args))
    first = sn.build_stress_neutral_export_package(source_envelope=raw, analysis_record=analysis, **args)
    second = sn.build_stress_neutral_export_package(source_envelope=raw, analysis_record=analysis, **args)
    assert json.dumps(first, ensure_ascii=False, separators=(",", ":"), allow_nan=False) == json.dumps(second, ensure_ascii=False, separators=(",", ":"), allow_nan=False)
    assert (raw, analysis, args) == before
    assert first["schema_version"] == "0.3.0"
    assert first["deliverable_id"] == "DEL-17-06"
    assert first["export_profile"]["profile_id"] == "ops.stress_neutral.v3"
    assert first["export_profile"]["comparison_semantics"] == "diagnostic_export_only_no_pass_fail"
    assert first["provenance"]["source_location"] == "core/handoff/stress_neutral/package.py"
    assert [(row["result_id"], row["value"], row["unit"]) for row in first["result_rows"]] == [(row["id"], row["value"], row["unit"]) for row in raw["results"]]
    assert first["unit_system_disclosure"]["unit_system_ref"]["ref"] == "unit-system:dec-018-si-dual-display"
    assert first["unit_system_disclosure"]["result_units"] == ["mm"]
    assert first["unit_system_disclosure"]["conversion_performed"] is False
    assert first["unit_system_disclosure"]["protected_content_included"] is False
    assert first["validation_ready"] is True
    schema = json.loads(SCHEMA_PATH.read_text())
    assert validate_schema_document(schema)
    assert validate_instance(schema, first, instance_label="current native stress-neutral")


def test_current_830_rows_keep_exact_values_units_and_witness_withholding():
    raw, analysis, args = prepared(830, diagnostic_work=True)
    before = deepcopy(raw)
    packet = sn.build_stress_neutral_export_package(source_envelope=raw, analysis_record=analysis, **args)
    assert len(packet["result_rows"]) == 830
    assert [row["value"] for row in packet["result_rows"]] == [row["value"] for row in raw["results"]]
    assert [row["unit"] for row in packet["result_rows"]] == [row["unit"] for row in raw["results"]]
    assert len(packet["unit_preservation_witnesses"]) == 828
    assert sum(item["code"] == "SN-UNIT-WITNESS-WITHHELD-DIAGNOSTIC-WORK" for item in packet["diagnostics"]) == 2
    aggregate = next(item for item in packet["diagnostics"] if item["code"] == "SN-DECLARED-DIMENSION-WITNESS-UNAVAILABLE")
    assert "2 retained rows" in aggregate["message"] and "828 rows" in aggregate["message"]
    assert not any(item["code"] == "SN-UNIT-DIMENSION-MISSING" for item in packet["diagnostics"])
    exported_reason = next(item["reason"] for item in packet["loss_report"] if item["category"] == "exported")
    assert "830 received numerical rows" in exported_reason and "828 rows" in exported_reason and "2 rows" in exported_reason
    assert packet["validation_ready"] is False
    assert packet["validation_report"]["validation_status"] == "blocked"
    assert raw == before and packet["received_source_checksums"] == args["source_hashes"]
    assert list(sn.materialized_members(packet)) == sn.MEMBERS
    sn.validate_stress_neutral_export_package(packet, source_envelope=raw, analysis_record=analysis)
    validate_instance(json.loads(SCHEMA_PATH.read_text()), packet, instance_label="current 830-row packet")


def test_current_writer_roundtrips_nine_exact_checksum_bound_members(tmp_path):
    # Retired write_stress_neutral_export_package's 0.1 redaction refusal is not
    # the current 0.3 writer contract (package_v0_3.py::write_materialized_members
    # before E3). A normal current packet materializes all nine exact bytes.
    packet = build_from_source()
    members = sn.materialized_members(packet)
    paths = sn.write_materialized_members(packet, tmp_path)
    assert [path.name for path in paths] == sn.MEMBERS
    assert {path.name: path.read_bytes() for path in paths} == members
    assert sn.read_materialized_members(tmp_path) == packet
    claims = {item["payload_ref"]["ref"]: item for item in packet["manifest"]["checksums"]}
    for filename, payload in members.items():
        if filename != "manifest.json":
            assert hashlib.sha256(payload).hexdigest() == claims[filename]["value"]
        else:
            assert claims[filename]["payload_scope"] == "manifest_seed"
        if filename.endswith(".json"):
            assert not payload.endswith(b"\n")
    assert all(item["canonicalization"] == "openpipestress_jcs_ijson_v1" for filename, item in claims.items() if filename.endswith(".json"))


def test_current_csv_and_json_rows_preserve_scientific_values_order_and_units():
    packet = build_from_source(values=[0.000028, 1.0])
    assert packet["csv_text"] == sn._render_utf8_csv(packet["result_rows"])
    assert packet["csv_text"].splitlines()[0] == CSV_HEADER
    packet["csv_text"].encode("ascii")
    packet["csv_text"] = packet["csv_text"].replace(",2.8e-05,mm,", ",0.000028,mm,").replace(",1,mm,", ",1.0,mm,")
    received_bytes = packet["csv_text"].encode("ascii")
    rehash_packet(packet)
    sn.validate_stress_neutral_export_package(packet)
    assert packet["csv_text"].encode("ascii") == received_bytes
    claim = next(item for item in packet["manifest"]["checksums"] if item["payload_ref"]["ref"] == "stress_neutral_results.csv")
    assert claim["value"] == hashlib.sha256(received_bytes).hexdigest()


@pytest.mark.parametrize("numeric_text", ["NaN", "Infinity", "-Infinity", "1e400", "1e-4000", "9007199254740992", "12.500000000000002", "01", "+12.5", "12.5x", ""])
def test_current_csv_refuses_nonfinite_unsafe_malformed_underflow_or_unequal_values(numeric_text):
    packet = build_from_source()
    changed = packet["csv_text"].replace(",12.5,mm,", f",{numeric_text},mm,")
    assert changed != packet["csv_text"]
    with pytest.raises(ValueError, match="SN-CSV-ROW-BINDING-MISMATCH"):
        sn._validate_received_csv(changed, packet["result_rows"])


@pytest.mark.parametrize("mutation", [
    lambda text: text.replace("\n", "\n\n", 1), lambda text: text + "\n", lambda text: text.replace("\n", "\n   \n", 1),
    lambda text: text.replace("result_id,canonical_ref", "canonical_ref,result_id", 1),
    lambda text: "\n".join(text.splitlines()[:-1]) + "\n", lambda text: text + text.splitlines()[1] + "\n",
    lambda text: text.replace(",mm,length,", ",m,length,", 1),
    lambda text: text.replace(",canonical_id_map\n", "\n", 1),
    lambda text: text.replace("correlation_status\n", "correlation_status,extra\n", 1),
])
def test_current_validator_refuses_coherently_rehashed_csv_shape_or_field_attacks(mutation):
    packet = build_from_source(); packet["csv_text"] = mutation(packet["csv_text"]); rehash_packet(packet)
    with pytest.raises(ValueError, match="SN-CSV-ROW-BINDING-MISMATCH"):
        sn.validate_stress_neutral_export_package(packet)


@pytest.mark.parametrize("mutation,expected", [
    (lambda p: p["unit_preservation_witnesses"][0].update(result_id="result:wrong"), "UNIT-WITNESS-BINDING"),
    (lambda p: p["unit_preservation_witnesses"][0]["source_quantity"].update(value=13.5), "UNIT-WITNESS-BINDING"),
    (lambda p: p["unit_preservation_witnesses"][0]["target_quantity"].update(unit="m"), "UNIT-WITNESS-BINDING"),
    (lambda p: p["unit_preservation_witnesses"][0].update(conversion_performed=True), "UNIT-WITNESS-BINDING"),
    (lambda p: p["loss_report"][0]["target_artifact_ref"].update(ref="package:wrong"), "LOSS-REPORT-BINDING"),
    (lambda p: p["source_result_ref"].update(ref="result-envelope:wrong"), "CARRIER-BINDING"),
    # The source-row annotations every current packet carries bind the run and the row identities first.
    (lambda p: p["source_run_ref"].update(ref="run:wrong"), "SOURCE-ANNOTATION-ROW-BINDING"),
    (lambda p: p["result_rows"][1].update(result_id=p["result_rows"][0]["result_id"], canonical_ref=deepcopy(p["result_rows"][0]["canonical_ref"]), source_result_ref=deepcopy(p["result_rows"][0]["source_result_ref"])), "SOURCE-ANNOTATION-COVERAGE"),
    (lambda p: p["stable_id_map"][1].update(canonical_ref=deepcopy(p["stable_id_map"][0]["canonical_ref"])), "STABLE-ID-MAP-BINDING"),
    (lambda p: p["stable_id_map"].pop(), "STABLE-ID-MAP-BINDING"),
    (lambda p: p["validation_report"].update(validation_status="blocked"), "VALIDATION-STATUS-BINDING"),
    (lambda p: p["validation_report"]["checks"][0].update(check_status="blocking", blocking_count=0), "VALIDATION-STATUS-BINDING"),
    (lambda p: p.update(schema_conformant=False), "SCHEMA-CONFORMANCE-CLAIM"),
    (lambda p: p["unit_preservation_witnesses"].pop(), "WITNESS-CATEGORY-ACCOUNTING"),
    (lambda p: p["export_profile"]["source_basis_refs"].remove(ref("ExternalReference", "fixtures/results/semantic_contract_v0_3_preview_physics_2.json")), "SEMANTIC-CONTRACT-BINDING"),
])
def test_current_recomputed_hashes_do_not_hide_relational_tamper(mutation, expected):
    packet = build_from_source(); mutation(packet)
    packet["csv_text"] = sn._render_utf8_csv(packet["result_rows"])
    rehash_packet(packet)
    with pytest.raises(ValueError, match=expected):
        sn.validate_stress_neutral_export_package(packet)


@pytest.mark.parametrize("mutation,expected", [
    (lambda p: p["manifest"]["checksums"][0].update(algorithm="sha512"), "CHECKSUM-METADATA"),
    (lambda p: p["manifest"]["checksums"][1]["payload_ref"].update(ref="../escape.csv"), "BIJECTION"),
    (lambda p: p["manifest"]["checksums"].append(deepcopy(p["manifest"]["checksums"][0])), "STRICT-INVENTORY"),
    (lambda p: p["manifest"]["checksums"].pop(), "STRICT-INVENTORY"),
    (lambda p: p["manifest"]["package_members"][0].update(checksum=deepcopy(p["manifest"]["checksums"][1])), "MEMBER-CHECKSUM-BINDING"),
    (lambda p: p["package_checksum"].update(payload_scope="member_payload"), "PACKAGE-CHECKSUM-METADATA"),
])
def test_current_inventory_checksum_metadata_and_binding_attacks_refuse(mutation, expected):
    packet = build_from_source(); mutation(packet)
    with pytest.raises(ValueError, match=expected):
        sn.validate_stress_neutral_export_package(packet)


def test_current_package_and_member_tampering_is_detected():
    packet = build_from_source(); packet["csv_text"] = packet["csv_text"].replace(",12.5,", ",13.5,")
    with pytest.raises(ValueError, match="CHECKSUM"):
        sn.validate_stress_neutral_export_package(packet)
    packet = build_from_source(); packet["package_checksum"]["value"] = "0" * 64
    with pytest.raises(ValueError, match="SN-PACKAGE-CHECKSUM-MISMATCH"):
        sn.validate_stress_neutral_export_package(packet)
    packet = build_from_source(); members = sn.materialized_members(packet); members["extra.json"] = b"{}"
    with pytest.raises(ValueError, match="INVENTORY"):
        sn.reconstruct_materialized_members(members)


@pytest.mark.parametrize("profile", [{"profile_id": "ops.stress_neutral.v1"}, {"profile_version": "0.2.0"}])
def test_current_builder_refuses_old_or_contradictory_profile_identity(profile):
    raw, analysis, args = prepared(); args["export_profile"] = profile
    with pytest.raises(ValueError, match="SN-0.3-PROFILE-IDENTITY-MISMATCH"):
        sn.build_stress_neutral_export_package(source_envelope=raw, analysis_record=analysis, **args)


@pytest.mark.parametrize("version", ["0.1.0", "0.2.0"])
def test_current_reader_and_schema_refuse_old_format_statements(version):
    packet = build_from_source(); packet["schema_version"] = version
    with pytest.raises(ValueError, match="SN-VERSION-UNSUPPORTED"):
        sn.validate_stress_neutral_export_package(packet)
    with pytest.raises(AssertionError):
        validate_instance(json.loads(SCHEMA_PATH.read_text()), packet, instance_label="retired stress-neutral version")


def test_current_schema_keeps_unknown_and_missing_field_guards():
    schema = json.loads(SCHEMA_PATH.read_text()); valid = build_from_source()
    for path, field, remove in [
        (["export_profile"], "unknown", False), (["result_rows", 0], "unit", True),
        (["unit_system_disclosure"], "unknown", False), (["stable_id_map", 0], "mapping_status", True),
        (["loss_report", 0], "reason", True), (["validation_report"], "unknown", False),
        (["provenance"], "unknown", False), (["received_source_checksums", 0], "unknown", False),
        (["privacy"], "unknown", False), (["professional_boundary"], "unknown", False),
    ]:
        changed = deepcopy(valid); target = changed
        for part in path: target = target[part]
        if remove: del target[field]
        else: target[field] = True
        with pytest.raises(AssertionError):
            validate_instance(schema, changed, instance_label=f"negative current field {path}/{field}")


def test_assumption_reproducibility_refs_are_copied_into_current_transport():
    raw, analysis, args = prepared()
    packet = sn.build_stress_neutral_export_package(source_envelope=raw, analysis_record=analysis, **args)
    transport = json.loads(sn.materialized_members(packet)["manifest.json"])
    for key in ("unresolved_assumption_refs", "reproducibility_refs"):
        assert packet[key] == args[key] == transport["package_metadata"][key]
    args["unresolved_assumption_refs"][0]["ref"] = "mutated-after-build"
    assert packet["unresolved_assumption_refs"][0]["ref"] == "assumption:invented-result-review"
    assert packet["professional_boundary"]["software_creates_professional_reliance_record"] is False


def test_current_boundary_diagnostics_keep_missing_data_privacy_and_authority_controls():
    packet = build_from_source(); rows = deepcopy(packet["result_rows"]); rows[0]["unit"] = ""
    profile = deepcopy(packet["export_profile"]); profile["free_metadata"] = {"unsafe_label": "cert" + "ified comparison export"}; profile["source_basis_refs"] = []
    findings = sn._boundary_diagnostics(result_rows=rows, stable_id_map=[], loss_report=[], export_profile=profile,
        privacy={**packet["privacy"], "protected_payload_embedded": True}, package_ref=ref("StressNeutralExportPackage", packet["export_id"]))
    codes = {item["code"] for item in findings}
    assert {"SN-UNIT-DIMENSION-MISSING", "SN-STABLE-ID-MAP-MISSING", "SN-LOSS-REPORT-MISSING", "SN-SOURCE-BASIS-REFS-MISSING", "SN-PRIVACY-BOUNDARY-VIOLATION", "SN-PROFILE-AUTHORITY-TERM"} <= codes
    assert all(item["severity"] == "blocking" for item in findings)
    controlled = control_route_export(packet, route_id="REXC-CORE-005", export_context="downstream_tool", require_lossless_materialization=True, source_findings=findings)
    assert controlled.blocked is True and controlled.payload is None
    assert controlled.summary["source_blocking_count"] == len(findings)


def test_current_builder_retains_blocking_authority_diagnostic_without_false_claims():
    raw, analysis, args = prepared(); args["export_profile"] = {"free_metadata": {"unsafe_label": "cert" + "ified comparison export"}}
    packet = sn.build_stress_neutral_export_package(source_envelope=raw, analysis_record=analysis, **args)
    assert "SN-PROFILE-AUTHORITY-TERM" in {item["code"] for item in packet["diagnostics"]}
    assert packet["validation_ready"] is False and packet["validation_report"]["validation_status"] == "blocked"
    assert packet["professional_boundary"]["software_makes_compliance_claim"] is False
    assert packet["professional_boundary"]["software_creates_professional_reliance_record"] is False
    # Free metadata is represented by the existing blocking diagnostic. It is
    # not a new direct-writer refusal; the writer's current flag guards below
    # are exercised with checksums coherent, before any filesystem writes.
    controlled = control_route_export(packet, route_id="REXC-CORE-005", export_context="downstream_tool", require_lossless_materialization=True, source_findings=packet["diagnostics"])
    assert controlled.blocked and controlled.summary["source_blocking_count"] >= 1


@pytest.mark.parametrize("field", ["private_payload_embedded", "protected_payload_embedded", "commercial_tool_payload_embedded", "telemetry_allowed"])
def test_current_builder_refuses_disallowed_privacy_flags(field):
    raw, analysis, args = prepared(); args["privacy"] = {field: True}
    with pytest.raises(ValueError, match="SN-PRIVACY-BOUNDARY-VIOLATION"):
        sn.build_stress_neutral_export_package(source_envelope=raw, analysis_record=analysis, **args)


@pytest.mark.parametrize("group,field,value,code", [
    ("privacy", "local_only", False, "SN-PRIVACY-BOUNDARY-VIOLATION"),
    *(("privacy", field, True, "SN-PRIVACY-BOUNDARY-VIOLATION") for field in ("private_payload_embedded", "protected_payload_embedded", "commercial_tool_payload_embedded", "telemetry_allowed")),
    ("professional_boundary", "human_review_required", False, "SN-PROFESSIONAL-BOUNDARY-VIOLATION"),
    *(("professional_boundary", field, True, "SN-PROFESSIONAL-BOUNDARY-VIOLATION") for field in ("software_makes_release_claim", "software_makes_external_compatibility_claim", "software_makes_solver_validation_claim", "software_makes_compliance_claim", "software_makes_certification_claim", "software_makes_sealing_claim", "software_makes_approval_claim", "software_creates_professional_reliance_record")),
])
def test_current_writer_refuses_coherently_hashed_boundary_attacks_before_writes(tmp_path, group, field, value, code):
    packet = build_from_source(); packet[group][field] = value; rehash_packet(packet)
    assert canonical_sha256_checked_v1(sn.package_projection(packet)) == packet["package_checksum"]["value"]
    output = tmp_path / "should-not-be-created"
    with pytest.raises(ValueError, match=code):
        sn.write_materialized_members(packet, output)
    assert not output.exists() and list(tmp_path.iterdir()) == []


def test_current_source_basis_refs_keep_result_export_run_comparison_contracts():
    packet = build_from_source()
    assert {("Deliverable", name) for name in ("DEL-08-04", "DEL-14-02", "DEL-14-05", "DEL-17-02")} <= {(item["object_type"], item["ref"]) for item in packet["export_profile"]["source_basis_refs"]}
    raw, analysis, args = prepared(); args["export_profile"] = {"source_basis_refs": [ref("Deliverable", "DEL-17-02")]}
    blocked = sn.build_stress_neutral_export_package(source_envelope=raw, analysis_record=analysis, **args)
    assert "SN-SOURCE-BASIS-REFS-MISSING" in {item["code"] for item in blocked["diagnostics"]}
    assert blocked["validation_report"]["validation_status"] == "blocked"


def test_current_invented_packet_contains_no_private_or_protected_payload_text():
    text = "\n".join(walk_strings(build_from_source())).lower()
    for forbidden in FORBIDDEN_PAYLOAD_TEXT: assert forbidden not in text


def test_current_category_reassignment_cannot_be_hidden_by_rehashing():
    packet = build_from_source(count=4, diagnostic_work=True)
    finding = next(item for item in packet["diagnostics"] if item["code"] == "SN-UNIT-WITNESS-WITHHELD-DIAGNOSTIC-WORK")
    finding["source"]["ref"] = packet["unit_preservation_witnesses"][0]["result_id"]
    rehash_packet(packet)
    with pytest.raises(ValueError, match="WITNESS-CATEGORY-ACCOUNTING"):
        sn.validate_stress_neutral_export_package(packet)


def test_current_schema_keeps_nested_provenance_shape_without_weakening_top_level():
    packet = build_from_source()
    packet["result_rows"][0]["provenance"] = {"source_name": "invented nested record",
        "source_location": "invented:test-record", "source_license": "invented", "extra": True}
    rehash_packet(packet)
    schema = json.loads(SCHEMA_PATH.read_text())
    validate_instance(schema, packet, instance_label="unchanged current nested provenance admission")
    packet["provenance"]["extra"] = True
    with pytest.raises(AssertionError):
        validate_instance(schema, packet, instance_label="unchanged strict top-level provenance refusal")


if __name__ == "__main__":
    test_current_builder_is_deterministic_source_bound_and_schema_conformant()
    test_current_csv_and_json_rows_preserve_scientific_values_order_and_units()
    test_current_boundary_diagnostics_keep_missing_data_privacy_and_authority_controls()
    test_current_builder_retains_blocking_authority_diagnostic_without_false_claims()
    test_current_source_basis_refs_keep_result_export_run_comparison_contracts()
    test_current_invented_packet_contains_no_private_or_protected_payload_text()
