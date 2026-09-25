"""Pure stress-neutral projection of actual physical/source-method outputs.

Projection arguments identify the supplied records, not a fresh native invocation.
No source header, quality flag, numeric value or method receipt is fabricated.
"""
from copy import deepcopy
from functools import lru_cache
import hashlib
import json
from pathlib import Path

import pytest

from core.analysis_runs.compatibility import (
    _source_contract, build_analysis_run_v0_3, PHYSICS_CONTRACT_ID,
    SOURCE_BLOCKS_CONTRACT_ID, PHYSICS_SOURCE_CONTRACT_ID,
)
from core.handoff.stress_neutral import package_v0_3 as sn
from core.serialization.canonical_json.adapter import canonical_sha256_checked_v1
from test_stress_neutral_export_package import source_payload
from test_stress_neutral_precision import prepare as prepare_precision, rehash_packet
from schema_validation import validate_instance

PROJECT = Path(__file__).resolve().parents[1]
SCHEMA = PROJECT / "schemas/stress_neutral_export.v0.3.schema.json"
ACTUAL_PATHS = [
    PROJECT / f"fixtures/results/{name}_{mode}.json"
    for name in ("physics_connected_mechanics", "physics_thermal_ui_mechanics")
    for mode in ("sparse", "dense")
] + [
    PROJECT / f"fixtures/product_preview/source_blocks/ui/{stem}-{mode}.raw.json"
    for stem in ("n05", "n06", "multicase")
    for mode in ("sparse_interactive", "dense_scrutiny")
]
COMPOSITE_DIR = PROJECT / "fixtures/product_preview/physics_source"


def arguments(raw, analysis):
    args = source_payload()
    args["source_result_ref"] = {"object_type": "ResultEnvelope", "ref": f"result-envelope:{raw['run_id']}"}
    args["source_run_ref"] = {"object_type": "AnalysisRun", "ref": raw["run_id"]}
    args["source_model_ref"] = {"object_type": "Model", "ref": raw["model_ref"]}
    args["source_hashes"] = deepcopy(analysis["analysis_run"]["hashes"])
    args["reproducibility_refs"] = [deepcopy(args["source_run_ref"])]
    _, _, table = _source_contract(raw)
    args["result_rows"] = [sn._source_row_projection(raw, row, table) for row in raw["results"]]
    args["stable_id_map"] = [{"canonical_ref": row["canonical_ref"],
        "export_ref": {"object_type": "StressNeutralRow", "ref": row["result_id"]},
        "mapping_status": "mapped", "loss_category": "exported"} for row in args["result_rows"]]
    return args


@lru_cache(maxsize=None)
def captured(path):
    raw = json.loads(Path(path).read_text())
    before = deepcopy(raw)
    analysis = build_analysis_run_v0_3(raw, input_manifest_ref={"object_type": "InputManifest", "ref": "manifest:pure-projection-test"}, input_manifest_hash="1" * 64)
    packet = sn.build_stress_neutral_export_package_v0_3(source_envelope=raw, analysis_record=analysis, **arguments(raw, analysis))
    assert raw == before
    return raw, analysis, packet


@pytest.mark.parametrize("path", ACTUAL_PATHS, ids=lambda path: path.stem)
def test_actual_physical_and_source_methods_roundtrip_all_nine_members(path, tmp_path):
    raw, analysis, packet = captured(path)
    before = deepcopy(packet)
    identity, digest, table = _source_contract(raw)
    assert identity in {PHYSICS_CONTRACT_ID, SOURCE_BLOCKS_CONTRACT_ID}
    assert packet["producer"] == raw["producer"]
    assert packet["semantic_contract"] == {"id": identity, "sha256": digest}
    assert packet["source_carrier_checksum"]["value"] == canonical_sha256_checked_v1(raw)
    assert len(packet["result_rows"]) == len(raw["results"])
    assert [item["source_row"] for item in packet["source_annotations"]] == raw["results"]
    assert [item["source_row_index"] for item in packet["source_annotations"]] == list(range(len(raw["results"])))
    by_id = {row["result_id"]: row for row in packet["result_rows"]}
    for row, annotation in zip(raw["results"], packet["source_annotations"]):
        assert by_id[row["id"]]["value"] == row["value"]
        assert by_id[row["id"]]["unit"] == row["unit"]
        assert annotation["source_row_sha256"] == canonical_sha256_checked_v1(row)
        assert annotation["source_row"].get("metadata") == row.get("metadata")
    namespace = "contract_evidence" if identity == PHYSICS_CONTRACT_ID else "source_block_recovery"
    assert packet[namespace] == raw[namespace]
    assert ("source_block_recovery" in packet) == (identity == SOURCE_BLOCKS_CONTRACT_ID)
    assert ("contract_evidence" in packet) == (identity == PHYSICS_CONTRACT_ID)
    validate_instance(json.loads(SCHEMA.read_text()), packet, instance_label=path.stem)
    sn.validate_stress_neutral_export_package_v0_3(packet)
    sn.validate_stress_neutral_export_package_v0_3(packet, source_envelope=raw, analysis_record=analysis)
    written = sn.write_materialized_members_v0_3(packet, tmp_path)
    assert [p.name for p in written] == sn.MEMBERS
    assert set(p.name for p in tmp_path.iterdir()) == set(sn.MEMBERS)
    assert sn.read_materialized_members_v0_3(tmp_path) == packet
    assert sn.read_materialized_members_v0_3(tmp_path, source_envelope=raw, analysis_record=analysis) == packet
    assert packet == before


def test_physics_support_moments_and_maxima_keep_distinct_source_semantics():
    raw, _, packet = captured(ACTUAL_PATHS[0])
    rows = {row["result_id"]: row for row in packet["result_rows"]}
    for annotation in packet["source_annotations"]:
        row = annotation["source_row"]
        if row["kind"] == "support_reaction_component_v2":
            moment = row["metadata"]["component"].startswith("M")
            assert rows[row["id"]]["result_family"] == "reaction"
            assert rows[row["id"]]["dimension"] == ("moment" if moment else "force")
            assert rows[row["id"]]["unit"] == ("N*m" if moment else "N")
        if row["kind"] == "pipe_elastic_normal_stress_maximum_v2":
            assert rows[row["id"]]["result_family"] == "stress"
            assert row["metadata"]["basis"] == "recovered_from_open_mechanics_stress_components"
            assert rows[row["id"]]["station_ref"]["ref"] == "governing_station"
    assert len([r for r in raw["results"] if r["kind"] == "support_reaction_component_v2"]) == 12


def test_actual_composite_files_are_required_and_retain_both_namespaces(tmp_path):
    paths = sorted(COMPOSITE_DIR.glob("*.raw.json"))
    assert len(paths) >= 2, "Actual both-mode composite producer outputs are required; never substitute invented headers"
    assert any("sparse" in path.name for path in paths) and any("dense" in path.name for path in paths)
    for path in paths:
        raw, analysis, packet = captured(path)
        assert raw["producer"]["semantic_contract_id"] == PHYSICS_SOURCE_CONTRACT_ID
        assert packet["contract_evidence"] == raw["contract_evidence"]
        assert packet["source_block_recovery"] == raw["source_block_recovery"]
        assert [a["source_row"] for a in packet["source_annotations"]] == raw["results"]
        validate_instance(json.loads(SCHEMA.read_text()), packet, instance_label=path.stem)
        sn.write_materialized_members_v0_3(packet, tmp_path / path.stem)
        assert sn.read_materialized_members_v0_3(tmp_path / path.stem, source_envelope=raw, analysis_record=analysis) == packet
        for a in packet["source_annotations"]:
            row = a["source_row"]
            if row["kind"] == "pipe_elastic_normal_stress_maximum_v2":
                assert row["metadata"]["basis"] in {"retained_source_endpoint_normal_max_v1", "recovered_from_open_mechanics_stress_components"}


@pytest.mark.parametrize("change", ["missing", "duplicate", "wrong_index", "raw_value", "raw_metadata", "raw_hash", "wrong_id"])
def test_annotation_tampering_fails_even_after_package_rehash(change):
    raw, analysis, original = captured(ACTUAL_PATHS[0])
    packet = deepcopy(original)
    annotations = packet["source_annotations"]
    if change == "missing": annotations.pop()
    elif change == "duplicate": annotations[1] = deepcopy(annotations[0])
    elif change == "wrong_index": annotations[0]["source_row_index"] = True
    elif change == "raw_value": annotations[0]["source_row"]["value"] += 1
    elif change == "raw_metadata": annotations[0]["source_row"]["metadata"]["basis"] = "forged"
    elif change == "raw_hash": annotations[0]["source_row_sha256"] = "f" * 64
    else: annotations[0]["source_result_id"] = "unrelated"
    rehash_packet(packet)
    with pytest.raises(ValueError, match="SN-SOURCE-ANNOTATION"):
        sn.validate_stress_neutral_export_package_v0_3(packet)
    with pytest.raises(ValueError):
        sn.validate_stress_neutral_export_package_v0_3(packet, source_envelope=raw, analysis_record=analysis)


def test_coherently_rehashed_annotation_is_not_an_authenticity_claim():
    raw, _, original = captured(ACTUAL_PATHS[0])
    packet = deepcopy(original)
    # A nonnumeric raw annotation can be internally consistent but differs from
    # supplied original source. Standalone transport cannot authenticate it.
    row = packet["source_annotations"][0]["source_row"]
    row["retained_note"] = "invented transport mutation"
    packet["source_annotations"][0]["source_row_sha256"] = canonical_sha256_checked_v1(row)
    rehash_packet(packet)
    sn.validate_stress_neutral_export_package_v0_3(packet)
    with pytest.raises(ValueError, match="ANNOTATION-BINDING"):
        sn.validate_stress_neutral_export_package_v0_3(packet, source_envelope=raw)


@pytest.mark.parametrize("path", [ACTUAL_PATHS[0], ACTUAL_PATHS[4]], ids=["physics", "source_blocks"])
def test_method_namespace_removal_substitution_and_relabel_are_rejected(path):
    raw, _, original = captured(path)
    field = "contract_evidence" if raw["producer"]["semantic_contract_id"] == PHYSICS_CONTRACT_ID else "source_block_recovery"
    for bad_value in [None, False, {}, []]:
        packet = deepcopy(original); packet[field] = bad_value; rehash_packet(packet)
        with pytest.raises((ValueError, KeyError, TypeError)):
            sn.validate_stress_neutral_export_package_v0_3(packet)
        with pytest.raises(AssertionError):
            validate_instance(json.loads(SCHEMA.read_text()), packet, instance_label="invalid namespace")
    packet = deepcopy(original)
    packet["producer"]["semantic_contract_id"] = "openpipestress.result_semantics/0.3.0/unknown-method"
    rehash_packet(packet)
    with pytest.raises(ValueError): sn.validate_stress_neutral_export_package_v0_3(packet)
    with pytest.raises(AssertionError): validate_instance(json.loads(SCHEMA.read_text()), packet, instance_label="unknown method")
    packet = deepcopy(original); packet["semantic_contract"]["sha256"] = "f" * 64; rehash_packet(packet)
    with pytest.raises(ValueError, match="SEMANTIC-CONTRACT"):
        sn.validate_stress_neutral_export_package_v0_3(packet)
    with pytest.raises(AssertionError): validate_instance(json.loads(SCHEMA.read_text()), packet, instance_label="wrong method hash")


def test_new_method_privacy_professional_and_member_guards_stay_closed():
    _, _, original = captured(ACTUAL_PATHS[0])
    for group, key in [("privacy", "private_payload_embedded"), ("privacy", "protected_payload_embedded"), ("professional_boundary", "software_makes_solver_validation_claim"), ("professional_boundary", "software_makes_approval_claim")]:
        packet = deepcopy(original); packet[group][key] = True; rehash_packet(packet)
        with pytest.raises(ValueError, match="BOUNDARY-VIOLATION"):
            sn.validate_stress_neutral_export_package_v0_3(packet)
    members = sn.materialized_members_v0_3(original)
    members["extra.json"] = b"{}"
    with pytest.raises(ValueError, match="INVENTORY"):
        sn.reconstruct_materialized_members_v0_3(members)


def test_precision_package_and_all_nine_member_bytes_are_unchanged():
    raw, analysis, args = prepare_precision()
    packet = sn.build_stress_neutral_export_package_v0_3(source_envelope=raw, analysis_record=analysis, **args)
    assert packet["package_checksum"]["value"] == "3677f468fb9dde4683de9fd5d9483fcce135cd4050d2f3fa7fa24d369a97359e"
    expected = {
        "manifest.json": "244baa3ba952493f7a2dcd5eceef478cf9b1d38d08794520a8e33e66c0cec601",
        "stress_neutral_results.csv": "e8d604f65489413da7f421f0f2d95432c4f7df16a7f8566c5902ececbaefb59d",
        "result_rows.json": "eb752adaefb92719b5563322d00369f3657974e787f39da7f6dde3a3eb10b876",
        "unit_system_disclosure.json": "3ad54b12cfc3081b51a2b394834826cbe4549e3c74d1dd8a6fa935d5a1a831e3",
        "unit_preservation_witnesses.json": "1824cf8af549dea6d204e4a53fd2b4d30c676148d5253ad289dc5009c5547fc1",
        "stable_id_map.json": "5feea93687527da9da108480548fd575c76132655a15793371374cdf419c17e6",
        "loss_report.json": "c720c6fb2cfeb516cf6d28637c68b0b54147fc41cec5b1da412a3f60006c2d4e",
        "validation_report.json": "65be20ef22800df89d862a810a2da45bd6fa2ab1be3d01b5a8e399219f805d13",
        "diagnostics.json": "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945",
    }
    assert {name: hashlib.sha256(data).hexdigest() for name, data in sn.materialized_members_v0_3(packet).items()} == expected
    assert not {"contract_evidence", "source_block_recovery", "source_annotations"} & set(packet)


def test_physics_shape_definitions_match_published_result_schema():
    current = json.loads(SCHEMA.read_text())
    result = json.loads((PROJECT / "schemas/results.v0.3.schema.yaml").read_text())
    for key in ["PhysicsContractEvidence", "PhysicsGeometry", "PhysicsMaterial"]:
        assert current["$defs"][key] == result["$defs"][key]
