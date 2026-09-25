"""Pure stress-neutral projection of actual physical/source-method outputs.

Projection arguments identify the supplied records, not a fresh native invocation.
No source header, quality flag, numeric value or method receipt is fabricated.
"""
from copy import deepcopy
from functools import lru_cache
import hashlib
import csv
import io
import json
import math
import struct
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
        assert annotation["source_value_bits"] == struct.pack(">d", float(row["value"])).hex()
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
        for a, original_row in zip(packet["source_annotations"], raw["results"]):
            assert a["source_value_bits"] == struct.pack(">d", float(original_row["value"])).hex()
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


@pytest.mark.parametrize("value,bits", [
    (0.0, "0000000000000000"), (-0.0, "8000000000000000"),
    (1.0, "3ff0000000000000"), (-1.0, "bff0000000000000"),
    (5e-324, "0000000000000001"), (-5e-324, "8000000000000001"),
])
def test_value_bits_annotation_preserves_received_binary64_without_changing_jcs(value, bits):
    # Synthetic transport vectors only, not claimed solver outputs.
    original = {"id": "transport:bits", "kind": "displacement_magnitude", "value": value, "unit": "mm", "entity_ref": "transport:node", "metadata": None, "source_result_refs": []}
    annotation = sn._source_annotations({"results": [original]})[0]
    assert set(annotation) == {"source_row_index", "source_result_id", "source_row", "source_row_sha256", "source_value_bits"}
    assert annotation["source_value_bits"] == bits
    assert annotation["source_row_sha256"] == canonical_sha256_checked_v1(original)
    assert annotation["source_row"].keys() == original.keys()
    assert annotation["source_row"]["metadata"] is None
    assert annotation["source_row"]["source_result_refs"] == []
    assert struct.pack(">d", float(original["value"])).hex() == bits
    if value == 0:
        assert math.copysign(1, annotation["source_row"]["value"]) == 1
    sn._validate_source_value_bits(annotation)
    reread = json.loads(sn.canonical_json_checked_v1(annotation))
    assert reread["source_value_bits"] == bits
    sn._validate_source_value_bits(reread)


@pytest.mark.parametrize("value,bits", [
    (0.0, "0000000000000001"), (0.0, "7ff0000000000000"),
    (0.0, "fff0000000000000"), (0.0, "7ff8000000000000"),
    (1.0, "bff0000000000000"), (1.0, "0000000000000000"),
    (-0.0, "8000000000000000"), (False, "0000000000000000"),
    (0.0, "800000000000000A"), (0.0, "800000000000000"),
    (0.0, None), (0.0, 0),
])
def test_value_bits_annotation_refuses_nonfinite_mismatch_and_noncanonical_zero(value, bits):
    with pytest.raises(ValueError, match="SN-SOURCE-ANNOTATION-VALUE-BITS"):
        sn._validate_source_value_bits({"source_row": {"value": value}, "source_value_bits": bits})


@pytest.mark.parametrize("mode", ["sparse_interactive", "dense_scrutiny"])
def test_cr02_actual_mixed_signed_zeros_survive_materialize_reopen_and_bind_original(mode, tmp_path):
    path = COMPOSITE_DIR / f"mixed-{mode}.raw.json"
    raw, analysis, packet = captured(path)
    before = deepcopy(raw)
    original_bits = {row["id"]: struct.pack(">d", float(row["value"])).hex() for row in raw["results"]}
    negative_ids = {identity for identity, bits in original_bits.items() if bits == "8000000000000000"}
    positive_ids = {identity for identity, bits in original_bits.items() if bits == "0000000000000000"}
    assert negative_ids and positive_ids, "Actual mixed source must exercise both received zero signs"
    if mode == "sparse_interactive":
        assert len(negative_ids) == 31, "Retain the independent review's original signed-zero failure population"
    sn.write_materialized_members_v0_3(packet, tmp_path)
    reread = sn.read_materialized_members_v0_3(tmp_path, source_envelope=raw, analysis_record=analysis)
    validate_instance(json.loads(SCHEMA.read_text()), reread, instance_label=f"CR02 {mode}")
    assert set(p.name for p in tmp_path.iterdir()) == set(sn.MEMBERS)
    for annotation in reread["source_annotations"]:
        identity = annotation["source_result_id"]
        assert annotation["source_value_bits"] == original_bits[identity]
        restored = struct.unpack(">d", bytes.fromhex(annotation["source_value_bits"]))[0]
        assert struct.pack(">d", restored).hex() == original_bits[identity]
        if identity in negative_ids | positive_ids:
            # Raw JSON is explicitly canonical +0. Only the sidecar claims to
            # preserve the received zero sign; source_row alone does not.
            assert struct.pack(">d", float(annotation["source_row"]["value"])).hex() == "0000000000000000"
    assert raw == before
    for identities, opposite in [(negative_ids, "0000000000000000"), (positive_ids, "8000000000000000")]:
        forged = deepcopy(reread)
        target = next(a for a in forged["source_annotations"] if a["source_result_id"] in identities)
        target["source_value_bits"] = opposite
        # A changed sidecar is part of the hashed package; without rehashing it
        # fails even though +0 and -0 are numerically equal.
        with pytest.raises(ValueError, match="PACKAGE-CHECKSUM"):
            sn.validate_stress_neutral_export_package_v0_3(forged)
        rehash_packet(forged)
        # An internally consistent transport can carry either zero sign, but it
        # cannot be claimed to match the original received source after a flip.
        sn.validate_stress_neutral_export_package_v0_3(forged)
        with pytest.raises(ValueError, match="SOURCE-ANNOTATION-BINDING"):
            sn.validate_stress_neutral_export_package_v0_3(forged, source_envelope=raw, analysis_record=analysis)
    malformed = deepcopy(reread)
    malformed["source_annotations"][0].pop("source_value_bits")
    rehash_packet(malformed)
    with pytest.raises(ValueError, match="ANNOTATION-SHAPE"):
        sn.validate_stress_neutral_export_package_v0_3(malformed)
    with pytest.raises(AssertionError):
        validate_instance(json.loads(SCHEMA.read_text()), malformed, instance_label="missing CR02 sidecar")


UTF8_METHODS = [PHYSICS_CONTRACT_ID, SOURCE_BLOCKS_CONTRACT_ID, PHYSICS_SOURCE_CONTRACT_ID]


def csv_transport_row(identity, value=1.25):
    # A CSV transport vector only; it is not a claimed producer result.
    return {"result_id": identity, "canonical_ref": {"object_type": "Result", "ref": identity},
        "row_kind": "result_value", "result_family": "force",
        "load_case_ref": {"object_type": "LoadCase", "ref": 'case:温度,"a"\r\nline'},
        "station_ref": {"object_type": "Station", "ref": '端点,"i"\nnext'},
        "component_ref": {"object_type": "PipeElement", "ref": 'pipe:é\rline'},
        "value": value, "unit": "N", "dimension": "force", "correlation_status": "canonical_id_map"}


def test_utf8_csv_transport_preserves_delimiters_quotes_and_embedded_cr_lf_scalar_order():
    identities = ['result:😀', 'result:\ue000', 'result:é', 'result:e\u0301', 'result:a,"quoted"\r\nnext', 'result:温度', '\ufeffresult:authored', 'result:\ufeff,"kept"']
    rows = [csv_transport_row(identity) for identity in identities]
    text = sn._render_utf8_csv(rows)
    wire = sn._utf8_csv_bytes(text)
    assert wire == text.encode("utf-8") and not wire.startswith(b"\xef\xbb\xbf")
    parsed = list(csv.DictReader(io.StringIO(wire.decode("utf-8"), newline=""), strict=True))
    assert [row["result_id"] for row in parsed] == sorted(identities)
    assert identities.index('result:😀') < identities.index('result:\ue000')
    assert [row["result_id"] for row in parsed].index('result:\ue000') < [row["result_id"] for row in parsed].index('result:😀'), "Scalar order differs from UTF16 code-unit order"
    assert {row["result_id"] for row in parsed} >= {'\ufeffresult:authored', 'result:\ufeff,"kept"'}, "U+FEFF within fields is data, not a file BOM"
    assert {row["result_id"] for row in parsed} >= {'result:é', 'result:e\u0301'}, "No Unicode normalization"
    for row in parsed:
        assert row["load_case_ref"] == 'case:温度,"a"\r\nline'
        assert row["station_ref"] == '端点,"i"\nnext'
        assert row["component_ref"] == 'pipe:é\rline'
    sn._validate_received_csv(text, sorted(rows, key=lambda row: row["result_id"]), utf8=True)
    with pytest.raises(ValueError, match="ROW-ORDER"):
        sn._validate_received_csv(text, rows, utf8=True)
    with pytest.raises(ValueError, match="ROW-BINDING"):
        sn._validate_received_csv(text, sorted(rows, key=lambda row: row["result_id"]))


@pytest.mark.parametrize("mutate", [
    lambda text: "\ufeff" + text,
    lambda text: text.replace("\n", "\r\n", 1),
    lambda text: text[:-1],
    lambda text: text.replace("result:温度", "result:\ud800"),
    lambda text: text.replace("result:温度", 'result:bad"quote'),
])
def test_utf8_csv_transport_rejects_bom_record_cr_surrogates_and_broken_quoting(mutate):
    text = sn._render_utf8_csv([csv_transport_row("result:温度")])
    with pytest.raises(ValueError, match="SN-CSV-"):
        sn._utf8_csv_bytes(mutate(text))


@pytest.mark.parametrize("method", UTF8_METHODS)
def test_utf8_csv_profile_is_explicit_for_each_new_method(method):
    minimal = {"producer": {"semantic_contract_id": method}, "export_profile": {"csv_encoding": "utf-8", "csv_row_order": "unicode_scalar_value_result_id"}}
    assert sn._csv_policy(minimal) == ("utf-8", "utf8_csv_record_lf_v1")
    for key, bad in [("csv_encoding", "ascii"), ("csv_encoding", "UTF-8"), ("csv_row_order", "utf16_code_units")]:
        mutated = deepcopy(minimal); mutated["export_profile"][key] = bad
        with pytest.raises(ValueError, match="CSV-PROFILE"): sn._csv_policy(mutated)
    for key in ["csv_encoding", "csv_row_order"]:
        mutated = deepcopy(minimal); del mutated["export_profile"][key]
        with pytest.raises(ValueError, match="CSV-PROFILE"): sn._csv_policy(mutated)
    minimal["producer"]["semantic_contract_id"] = sn.PRECISION_CONTRACT_ID
    with pytest.raises(ValueError, match="CSV-PROFILE"): sn._csv_policy(minimal)
    minimal["export_profile"] = {}
    assert sn._csv_policy(minimal) == ("ascii", "normalized_ascii_lf_text")


@pytest.mark.parametrize("mode", ["sparse_interactive", "dense_scrutiny"])
def test_utf8_actual_unicode_source_roundtrips_exact_nine_members_and_value_bits(mode, tmp_path):
    path = COMPOSITE_DIR / f"n05_unicode-{mode}.raw.json"
    assert path.exists(), "Requires actual Unicode producer capture, not relabelled source headers"
    raw, analysis, packet = captured(path)
    original_request = json.loads((COMPOSITE_DIR / "n05_unicode.request.json").read_text())
    from core.analysis_runs.source_blocks import domain_hash
    assert original_request["model"]["project"]["id"] == raw["model_ref"]
    assert raw["source_block_recovery"]["body"]["invocation"]["value"] == domain_hash("source_blocks_invocation_v1", {"request": original_request, "solver_mode": mode})
    assert any(not row["id"].isascii() for row in raw["results"])
    assert packet["export_profile"]["csv_encoding"] == "utf-8"
    assert packet["export_profile"]["csv_row_order"] == "unicode_scalar_value_result_id"
    members = sn.materialized_members_v0_3(packet)
    assert list(members) == sn.MEMBERS
    wire = members["stress_neutral_results.csv"]
    assert wire == packet["csv_text"].encode("utf-8") and not wire.startswith(b"\xef\xbb\xbf")
    assert any(byte > 127 for byte in wire)
    checksum = next(c for c in packet["manifest"]["checksums"] if c["payload_ref"]["ref"] == "stress_neutral_results.csv")
    assert checksum["canonicalization"] == "utf8_csv_record_lf_v1"
    assert checksum["value"] == hashlib.sha256(wire).hexdigest()
    sn.write_materialized_members_v0_3(packet, tmp_path)
    reopened = sn.read_materialized_members_v0_3(tmp_path, source_envelope=raw, analysis_record=analysis)
    validate_instance(json.loads(SCHEMA.read_text()), reopened, instance_label=f"UTF8 actual {mode}")
    assert reopened == packet
    raw_by_id = {row["id"]: row for row in raw["results"]}
    assert [row["result_id"] for row in reopened["result_rows"]] == sorted(raw_by_id)
    for annotation in reopened["source_annotations"]:
        original = raw_by_id[annotation["source_result_id"]]
        assert annotation["source_value_bits"] == struct.pack(">d", float(original["value"])).hex()
        assert annotation["source_row"].get("metadata") == original.get("metadata")
    for bad_wire in [b"\xff" + wire, b"\xc0\x80" + wire, b"\xed\xa0\x80" + wire, b"\xe2\x82", b"\xef\xbb\xbf" + wire, wire.replace(b"\n", b"\r\n", 1)]:
        corrupted = dict(members); corrupted["stress_neutral_results.csv"] = bad_wire
        with pytest.raises(ValueError, match="SN-CSV-"):
            sn.reconstruct_materialized_members_v0_3(corrupted, source_envelope=raw)
    bad = deepcopy(packet); bad["export_profile"]["csv_encoding"] = "ascii"
    bad["package_checksum"]["value"] = canonical_sha256_checked_v1(sn.package_projection(bad))
    with pytest.raises(ValueError, match="CSV-PROFILE"):
        sn.validate_stress_neutral_export_package_v0_3(bad)
    with pytest.raises(AssertionError): validate_instance(json.loads(SCHEMA.read_text()), bad, instance_label="wrong UTF8 label")
    bad = deepcopy(packet)
    for claim in bad["manifest"]["checksums"]:
        if claim["payload_ref"]["ref"] == "stress_neutral_results.csv": claim["canonicalization"] = "normalized_ascii_lf_text"
    for member in bad["manifest"]["package_members"]:
        if member["filename"] == "stress_neutral_results.csv": member["checksum"]["canonicalization"] = "normalized_ascii_lf_text"
    bad["package_checksum"]["value"] = canonical_sha256_checked_v1(sn.package_projection(bad))
    with pytest.raises(ValueError, match="CHECKSUM-METADATA"):
        sn.validate_stress_neutral_export_package_v0_3(bad)
    with pytest.raises(AssertionError): validate_instance(json.loads(SCHEMA.read_text()), bad, instance_label="wrong UTF8 checksum label")


DENSE_DIAGNOSTIC_RATIO_ID = "result:sparse-live:dense-parity-relative-delta"


def test_new_method_diagnostic_ratio_witness_follows_exact_table_target():
    raw, analysis, packet = captured(COMPOSITE_DIR / "fields-dense_scrutiny.raw.json")
    original = next(row for row in raw["results"] if row["id"] == DENSE_DIAGNOSTIC_RATIO_ID)
    identity, _, table = _source_contract(raw)
    semantic, _ = sn._semantic(original, table)
    assert identity == PHYSICS_SOURCE_CONTRACT_ID
    assert semantic["signature_id"] == "supported-source-037"
    assert semantic["category"] == "diagnostic_relative_ratio"
    assert semantic["source_physical_semantic_dimension"] == "dimensionless"
    assert semantic["derivative_target_dimension"] == "ratio"
    assert semantic["governing_ratio_eligible"] is False
    assert sn.UNIT_DIMENSIONS["unitless"] == "dimensionless", "The generic unit catalog must not be changed"
    row = next(row for row in packet["result_rows"] if row["result_id"] == DENSE_DIAGNOSTIC_RATIO_ID)
    assert row["result_family"] == "other" and row["dimension"] == "ratio"
    witness = next(w for w in packet["unit_preservation_witnesses"] if w["result_id"] == DENSE_DIAGNOSTIC_RATIO_ID)
    assert witness["source_quantity"] == witness["target_quantity"] == {"value": original["value"], "unit": original["unit"], "dimension": "ratio"}
    assert witness["conversion_performed"] is False
    assert not any(d["source"]["ref"] == DENSE_DIAGNOSTIC_RATIO_ID for d in packet["diagnostics"] if d["code"].startswith("SN-UNIT-WITNESS-WITHHELD"))
    assert packet["professional_boundary"]["software_makes_compliance_claim"] is False
    sn.validate_stress_neutral_export_package_v0_3(packet)
    sn.validate_stress_neutral_export_package_v0_3(packet, source_envelope=raw, analysis_record=analysis)


@pytest.mark.parametrize("change", ["source_dimension", "target_dimension", "value", "contradiction", "unknown_semantic", "missing_semantic"])
def test_new_method_diagnostic_witness_and_category_cannot_override_table(change):
    raw, analysis, original = captured(COMPOSITE_DIR / "fields-dense_scrutiny.raw.json")
    packet = deepcopy(original)
    witness = next(w for w in packet["unit_preservation_witnesses"] if w["result_id"] == DENSE_DIAGNOSTIC_RATIO_ID)
    if change == "source_dimension":
        witness["source_quantity"]["dimension"] = "dimensionless"
    elif change == "target_dimension":
        witness["target_quantity"]["dimension"] = "dimensionless"
    elif change == "value":
        witness["source_quantity"]["value"] *= 2
    else:
        packet["unit_preservation_witnesses"].remove(witness)
        packet["diagnostics"].append({
            "code": sn.WITHHOLDING_CODES[change], "class": "unit_preservation_witness", "severity": "blocking",
            "source": {"object_type": "StressNeutralResultRow", "ref": DENSE_DIAGNOSTIC_RATIO_ID},
            "affected_object": {"object_type": "StressNeutralUnitWitness", "ref": witness["witness_id"]},
            "message": "Deliberately forged table-override category for a transport refusal test.",
            "remediation": "Retain the actual source/table interpretation.", "provenance": deepcopy(packet["provenance"]),
        })
        packet["validation_ready"] = False
        packet["validation_report"]["validation_status"] = "blocked"
        check = packet["validation_report"]["checks"][0]
        check.update(check_status="blocking", blocking_count=check["blocking_count"] + 1, diagnostic_count=check["diagnostic_count"] + 1)
    rehash_packet(packet)
    expected = "UNIT-WITNESS-BINDING" if change in {"source_dimension", "target_dimension", "value"} else "WITNESS-CATEGORY-ACCOUNTING"
    with pytest.raises(ValueError, match=expected):
        sn.validate_stress_neutral_export_package_v0_3(packet)
    with pytest.raises(ValueError, match="WITNESS-BINDING"):
        sn.validate_stress_neutral_export_package_v0_3(packet, source_envelope=raw, analysis_record=analysis)
