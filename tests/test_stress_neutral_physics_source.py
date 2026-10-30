"""Pure stress-neutral projection of actual physical/source-method outputs.

Projection arguments identify the supplied records, not a fresh native invocation.
No source header, quality flag, numeric value or method receipt is fabricated.

B7b (A4/A5): the current exact identities are pressure-2 and pressure-source-1 (`3.0.0/exact_pressure_v3`); they
take over the retired physics-1 and physics-source-1 packages, over the regenerated v3 producer twins. pressure-2
has no transport reader in Rust (SOURCE_PRESSURE_TRANSPORT_UNSUPPORTED), so its package is validated with its source.
"""
from copy import deepcopy
from functools import lru_cache
import hashlib
import csv
import io
import json
import math
import os
import struct
from pathlib import Path

import pytest

from checked_json_fixture_cache import checked_json_fixture_cache

from core.analysis_runs.compatibility import (
    _source_contract, build_analysis_run_v0_3, numerical_use_standing, PRESSURE_CONTRACT_ID,
    PRESSURE_SOURCE_CONTRACT_ID, validate_analysis_run_v0_3, verify_analysis_run_record,
)
from core.handoff.stress_neutral import package as sn
from core.serialization.canonical_json.adapter import canonical_sha256_checked_v1
from test_stress_neutral_export_package import source_payload
from test_stress_neutral_transport import rehash_packet
from schema_validation import validate_instance

PROJECT = Path(__file__).resolve().parents[1]
SCHEMA = PROJECT / "schemas/stress_neutral_export.v0.3.schema.json"
# B4/D1 (b): the preview route's source-blocks-1 captures left; the composites keep source blocks.
ACTUAL_PATHS = [
    PROJECT / f"fixtures/results/{name}_{mode}.json"
    for name in ("pressure_connected_mechanics", "pressure_thermal_ui_mechanics")
    for mode in ("sparse", "dense")
]
COMPOSITE_DIR = PROJECT / "fixtures/product_preview/pressure_source/physics_source"
PRESSURE_TRANSPORT_UNSUPPORTED = "SOURCE_PRESSURE_TRANSPORT_UNSUPPORTED"
# T4-U6 (DEL-05-03 Design §2.1, §2.5): an exact stress row's token is its recovery token followed by its
# section basis, and its id ends in that basis.
SECTION_BASES = ("nominal_section", "reduced_section")


def section_token(recovery: str, row: dict) -> bool:
    """Whether the row's basis is ``recovery`` on one section basis, the one its id names."""
    return any(row["metadata"]["basis"] == f"{recovery}; section_basis={basis}" and row["id"].endswith(f":{basis}")
               for basis in SECTION_BASES)


def bound(raw):
    """The validator's source bindings a package needs on its own: pressure-2 has no transport reader, so its
    package is read through its source; every other method also validates standalone."""
    return {"source_envelope": raw} if raw["producer"]["semantic_contract_id"] == PRESSURE_CONTRACT_ID else {}


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
    with checked_json_fixture_cache():
        raw = json.loads(Path(path).read_text())
        before = deepcopy(raw)
        analysis = build_analysis_run_v0_3(raw, input_manifest_ref={"object_type": "InputManifest", "ref": "manifest:pure-projection-test"}, input_manifest_hash="1" * 64)
        packet = sn.build_stress_neutral_export_package(source_envelope=raw, analysis_record=analysis, **arguments(raw, analysis))
        assert raw == before
        return raw, analysis, packet

@pytest.mark.parametrize("path", ACTUAL_PATHS, ids=lambda path: path.stem)
def test_actual_physical_and_source_methods_roundtrip_all_nine_members(path, tmp_path):
    raw, analysis, packet = captured(path)
    before = deepcopy(packet)
    identity, digest, table = _source_contract(raw)
    assert identity == PRESSURE_CONTRACT_ID
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
    assert packet["contract_evidence"] == raw["contract_evidence"]
    assert "source_block_recovery" not in packet
    validate_instance(json.loads(SCHEMA.read_text()), packet, instance_label=path.stem)
    # Rust has no pressure-2 transport reader: without its source the package is refused by Rust's name.
    with pytest.raises(ValueError, match=PRESSURE_TRANSPORT_UNSUPPORTED):
        sn.validate_stress_neutral_export_package(packet)
    sn.validate_stress_neutral_export_package(packet, source_envelope=raw)
    sn.validate_stress_neutral_export_package(packet, source_envelope=raw, analysis_record=analysis)
    with pytest.raises(ValueError, match=PRESSURE_TRANSPORT_UNSUPPORTED):
        sn.write_materialized_members(packet, tmp_path / "unbound")
    written = sn.write_materialized_members(packet, tmp_path, source_envelope=raw)
    assert [p.name for p in written] == sn.MEMBERS
    assert set(p.name for p in tmp_path.iterdir()) == set(sn.MEMBERS)
    with pytest.raises(ValueError, match=PRESSURE_TRANSPORT_UNSUPPORTED):
        sn.read_materialized_members(tmp_path)
    assert sn.read_materialized_members(tmp_path, source_envelope=raw) == packet
    assert sn.read_materialized_members(tmp_path, source_envelope=raw, analysis_record=analysis) == packet
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
            assert section_token("recovered_from_open_mechanics_stress_components", row), row["id"]
            assert rows[row["id"]]["station_ref"]["ref"] == "governing_station"
    assert len([r for r in raw["results"] if r["kind"] == "support_reaction_component_v2"]) == 12


def test_actual_composite_files_are_required_and_retain_both_namespaces(tmp_path):
    paths = sorted(COMPOSITE_DIR.glob("*.raw.json"))
    assert len(paths) >= 2, "Actual both-mode composite producer outputs are required; never substitute invented headers"
    assert any("sparse" in path.name for path in paths) and any("dense" in path.name for path in paths)
    for path in paths:
        raw, analysis, packet = captured(path)
        assert raw["producer"]["semantic_contract_id"] == PRESSURE_SOURCE_CONTRACT_ID
        assert packet["contract_evidence"] == raw["contract_evidence"]
        assert packet["source_block_recovery"] == raw["source_block_recovery"]
        assert [a["source_row"] for a in packet["source_annotations"]] == raw["results"]
        validate_instance(json.loads(SCHEMA.read_text()), packet, instance_label=path.stem)
        sn.write_materialized_members(packet, tmp_path / path.stem)
        assert sn.read_materialized_members(tmp_path / path.stem, source_envelope=raw, analysis_record=analysis) == packet
        for a, original_row in zip(packet["source_annotations"], raw["results"]):
            assert a["source_value_bits"] == struct.pack(">d", float(original_row["value"])).hex()
            row = a["source_row"]
            if row["kind"] == "pipe_elastic_normal_stress_maximum_v2":
                assert any(section_token(token, row) for token in ("retained_source_endpoint_normal_max_v1", "recovered_from_open_mechanics_stress_components")), row["id"]


@pytest.mark.parametrize("path", [ACTUAL_PATHS[0], COMPOSITE_DIR / "n05-sparse_interactive.raw.json"], ids=["pressure", "pressure_source"])
@pytest.mark.parametrize("change", ["missing", "duplicate", "wrong_index", "raw_value", "raw_metadata", "raw_hash", "wrong_id"])
def test_annotation_tampering_fails_even_after_package_rehash(change, path):
    raw, analysis, original = captured(path)
    packet = deepcopy(original)
    annotations = packet["source_annotations"]
    if change == "missing": annotations.pop()
    elif change == "duplicate": annotations[1] = deepcopy(annotations[0])
    elif change == "wrong_index": annotations[0]["source_row_index"] = True
    elif change == "raw_value": annotations[0]["source_row"]["value"] += 1
    elif change == "raw_metadata": next(a for a in annotations if isinstance(a["source_row"].get("metadata"), dict))["source_row"]["metadata"]["basis"] = "forged"
    elif change == "raw_hash": annotations[0]["source_row_sha256"] = "f" * 64
    else: annotations[0]["source_result_id"] = "unrelated"
    rehash_packet(packet)
    with pytest.raises(ValueError, match="SN-SOURCE-ANNOTATION"):
        sn.validate_stress_neutral_export_package(packet, **bound(raw))
    with pytest.raises(ValueError):
        sn.validate_stress_neutral_export_package(packet, source_envelope=raw, analysis_record=analysis)


def test_coherently_rehashed_annotation_is_not_an_authenticity_claim():
    # Standalone transport needs a method with a transport reader: pressure-source-1's (pressure-2 has none).
    raw, _, original = captured(COMPOSITE_DIR / "n05-sparse_interactive.raw.json")
    packet = deepcopy(original)
    # A nonnumeric raw annotation can be internally consistent but differs from
    # supplied original source. Standalone transport cannot authenticate it.
    row = packet["source_annotations"][0]["source_row"]
    row["retained_note"] = "invented transport mutation"
    packet["source_annotations"][0]["source_row_sha256"] = canonical_sha256_checked_v1(row)
    rehash_packet(packet)
    sn.validate_stress_neutral_export_package(packet)
    with pytest.raises(ValueError, match="ANNOTATION-BINDING"):
        sn.validate_stress_neutral_export_package(packet, source_envelope=raw)


@pytest.mark.parametrize("path", [ACTUAL_PATHS[0], COMPOSITE_DIR / "n05-sparse_interactive.raw.json"], ids=["pressure", "pressure_source"])
def test_method_namespace_removal_substitution_and_relabel_are_rejected(path):
    raw, _, original = captured(path)
    field = "contract_evidence" if raw["producer"]["semantic_contract_id"] == PRESSURE_CONTRACT_ID else "source_block_recovery"
    for bad_value in [None, False, {}, []]:
        packet = deepcopy(original); packet[field] = bad_value; rehash_packet(packet)
        with pytest.raises((ValueError, KeyError, TypeError)):
            sn.validate_stress_neutral_export_package(packet, **bound(raw))
        with pytest.raises(AssertionError):
            validate_instance(json.loads(SCHEMA.read_text()), packet, instance_label="invalid namespace")
    packet = deepcopy(original)
    packet["producer"]["semantic_contract_id"] = "openpipestress.result_semantics/0.3.0/unknown-method"
    # The CSV wire policy is method-bound, so an unknown method cannot even be
    # re-materialized; the materializer and the validator must each refuse it.
    with pytest.raises(ValueError, match="SN-CSV-PROFILE-MISMATCH"): rehash_packet(deepcopy(packet))
    with pytest.raises(ValueError, match="SOURCE_PRODUCER_CONTRACT_UNSUPPORTED"): sn.validate_stress_neutral_export_package(packet)
    with pytest.raises(AssertionError): validate_instance(json.loads(SCHEMA.read_text()), packet, instance_label="unknown method")
    packet = deepcopy(original); packet["semantic_contract"]["sha256"] = "f" * 64; rehash_packet(packet)
    with pytest.raises(ValueError, match="SEMANTIC-CONTRACT"):
        sn.validate_stress_neutral_export_package(packet, **bound(raw))
    with pytest.raises(AssertionError): validate_instance(json.loads(SCHEMA.read_text()), packet, instance_label="wrong method hash")


def test_new_method_privacy_professional_and_member_guards_stay_closed():
    raw, _, original = captured(ACTUAL_PATHS[0])
    for group, key in [("privacy", "private_payload_embedded"), ("privacy", "protected_payload_embedded"), ("professional_boundary", "software_makes_solver_validation_claim"), ("professional_boundary", "software_makes_approval_claim")]:
        packet = deepcopy(original); packet[group][key] = True; rehash_packet(packet)
        with pytest.raises(ValueError, match="BOUNDARY-VIOLATION"):
            sn.validate_stress_neutral_export_package(packet, **bound(raw))
    members = sn.materialized_members(original)
    members["extra.json"] = b"{}"
    with pytest.raises(ValueError, match="INVENTORY"):
        sn.reconstruct_materialized_members(members)


def test_physics_shape_definitions_match_published_result_schema():
    current = json.loads(SCHEMA.read_text())
    result = json.loads((PROJECT / "schemas/results.v0.3.schema.yaml").read_text())
    for key in ["ExactRetainedContractEvidence", "PhysicsGeometry", "PhysicsMaterial"]:
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
    # K-D5 (DEL-04-01 kd5-member-relative-bound.md §§3.1, 3.4): the composite `mixed` source's CHECKS_PASSED
    # case now publishes its exactly-zero certified rows as +0 (its 31 received -0 rows are gone). B4/D1 (b)
    # retired the source-block multicase capture; the actual producer source with both received zero signs is
    # the preview route's output for the same request (PP `tests/source_block_recovery.rs`, OPS_W1_CONTROL_RAW_DIR).
    path = PROJECT / f"fixtures/product_preview/w1_controls/ui/multicase-{mode}.raw.json"
    raw, analysis, packet = captured(path)
    before = deepcopy(raw)
    original_bits = {row["id"]: struct.pack(">d", float(row["value"])).hex() for row in raw["results"]}
    negative_ids = {identity for identity, bits in original_bits.items() if bits == "8000000000000000"}
    positive_ids = {identity for identity, bits in original_bits.items() if bits == "0000000000000000"}
    assert negative_ids and positive_ids, "Actual mixed source must exercise both received zero signs"
    # T4-U4b adds the five sigma- rows of the unloaded member's stations: RN(-0 - 0) = -0 (sigma+ = -0 + 0 = +0).
    assert len(negative_ids) == 57, "the capture's received -0 population"
    sn.write_materialized_members(packet, tmp_path)
    reread = sn.read_materialized_members(tmp_path, source_envelope=raw, analysis_record=analysis)
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
            sn.validate_stress_neutral_export_package(forged)
        rehash_packet(forged)
        # An internally consistent transport can carry either zero sign, but it
        # cannot be claimed to match the original received source after a flip.
        sn.validate_stress_neutral_export_package(forged)
        with pytest.raises(ValueError, match="SOURCE-ANNOTATION-BINDING"):
            sn.validate_stress_neutral_export_package(forged, source_envelope=raw, analysis_record=analysis)
    malformed = deepcopy(reread)
    malformed["source_annotations"][0].pop("source_value_bits")
    rehash_packet(malformed)
    with pytest.raises(ValueError, match="ANNOTATION-SHAPE"):
        sn.validate_stress_neutral_export_package(malformed)
    with pytest.raises(AssertionError):
        validate_instance(json.loads(SCHEMA.read_text()), malformed, instance_label="missing CR02 sidecar")


UTF8_METHODS = [PRESSURE_CONTRACT_ID, PRESSURE_SOURCE_CONTRACT_ID]


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
    sn._validate_received_csv(text, sorted(rows, key=lambda row: row["result_id"]))
    with pytest.raises(ValueError, match="ROW-ORDER"):
        sn._validate_received_csv(text, rows)


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
    # B1: the retired precision-1 identity has no CSV profile, with or without the UTF-8 members.
    minimal["producer"]["semantic_contract_id"] = "openpipestress.result_semantics/0.3.0/precision-1"
    with pytest.raises(ValueError, match="CSV-PROFILE"): sn._csv_policy(minimal)
    minimal["export_profile"] = {}
    with pytest.raises(ValueError, match="CSV-PROFILE"): sn._csv_policy(minimal)


@pytest.mark.parametrize("mode", ["sparse_interactive", "dense_scrutiny"])
def test_utf8_actual_unicode_source_roundtrips_exact_nine_members_and_value_bits(mode, tmp_path):
    path = COMPOSITE_DIR / f"n05_unicode-{mode}.raw.json"
    assert path.exists(), "Requires actual Unicode producer capture, not relabelled source headers"
    raw, analysis, packet = captured(path)
    original_request = json.loads((COMPOSITE_DIR / "n05_unicode.request.json").read_text())
    from tests.verifiers.source_blocks import domain_hash
    assert original_request["model"]["project"]["id"] == raw["model_ref"]
    assert raw["source_block_recovery"]["body"]["invocation"]["value"] == domain_hash("source_blocks_invocation_v1", {"request": original_request, "solver_mode": mode})
    assert any(not row["id"].isascii() for row in raw["results"])
    assert packet["export_profile"]["csv_encoding"] == "utf-8"
    assert packet["export_profile"]["csv_row_order"] == "unicode_scalar_value_result_id"
    members = sn.materialized_members(packet)
    assert list(members) == sn.MEMBERS
    wire = members["stress_neutral_results.csv"]
    assert wire == packet["csv_text"].encode("utf-8") and not wire.startswith(b"\xef\xbb\xbf")
    assert any(byte > 127 for byte in wire)
    checksum = next(c for c in packet["manifest"]["checksums"] if c["payload_ref"]["ref"] == "stress_neutral_results.csv")
    assert checksum["canonicalization"] == "utf8_csv_record_lf_v1"
    assert checksum["value"] == hashlib.sha256(wire).hexdigest()
    sn.write_materialized_members(packet, tmp_path)
    reopened = sn.read_materialized_members(tmp_path, source_envelope=raw, analysis_record=analysis)
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
            sn.reconstruct_materialized_members(corrupted, source_envelope=raw)
    bad = deepcopy(packet); bad["export_profile"]["csv_encoding"] = "ascii"
    bad["package_checksum"]["value"] = canonical_sha256_checked_v1(sn.package_projection(bad))
    with pytest.raises(ValueError, match="CSV-PROFILE"):
        sn.validate_stress_neutral_export_package(bad)
    with pytest.raises(AssertionError): validate_instance(json.loads(SCHEMA.read_text()), bad, instance_label="wrong UTF8 label")
    bad = deepcopy(packet)
    for claim in bad["manifest"]["checksums"]:
        if claim["payload_ref"]["ref"] == "stress_neutral_results.csv": claim["canonicalization"] = "normalized_ascii_lf_text"
    for member in bad["manifest"]["package_members"]:
        if member["filename"] == "stress_neutral_results.csv": member["checksum"]["canonicalization"] = "normalized_ascii_lf_text"
    bad["package_checksum"]["value"] = canonical_sha256_checked_v1(sn.package_projection(bad))
    with pytest.raises(ValueError, match="CHECKSUM-METADATA"):
        sn.validate_stress_neutral_export_package(bad)
    with pytest.raises(AssertionError): validate_instance(json.loads(SCHEMA.read_text()), bad, instance_label="wrong UTF8 checksum label")


# b7-plan E14 deleted the DEC-050/053 parity observation (supported-source-037, the only
# `diagnostic_relative_ratio` row of any current table), so the composite raw no longer publishes
# `result:sparse-live:dense-parity-relative-delta` and its ratio-witness test was retired with it.
# The table-override protection below stays live on a surviving nonzero composite witness.
NEW_METHOD_WITNESS_ID = "result:disp:independent-root:rx"


@pytest.mark.parametrize("change", ["source_dimension", "target_dimension", "value", "contradiction", "unknown_semantic", "missing_semantic"])
def test_new_method_witness_and_category_cannot_override_table(change):
    raw, analysis, original = captured(COMPOSITE_DIR / "fields-dense_scrutiny.raw.json")
    packet = deepcopy(original)
    witness = next(w for w in packet["unit_preservation_witnesses"] if w["result_id"] == NEW_METHOD_WITNESS_ID)
    assert witness["source_quantity"]["value"] != 0.0
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
            "source": {"object_type": "StressNeutralResultRow", "ref": NEW_METHOD_WITNESS_ID},
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
        sn.validate_stress_neutral_export_package(packet)
    with pytest.raises(ValueError, match="WITNESS-BINDING"):
        sn.validate_stress_neutral_export_package(packet, source_envelope=raw, analysis_record=analysis)


# E3's equal-input Python construction basis: complete package-file witnesses, whose insertion-order file
# encoding is distinct from the checked member encoding. B7b replaced the 14 v2 witnesses (load_reference_* and
# load_reference_source_*) by their v3 twins: pressure-2 (pressure_load_state_*) and pressure-source-1
# (pressure_source_load_state_*) over the regenerated load/reference-state producer fixtures.
E3_PACKAGE_NAMES = [
    f"{family}_{mode}"
    for family in ("pressure_load_state_connected", "pressure_load_state_pressure",
        "pressure_source_load_state_eigen_motion", "pressure_source_load_state_fields",
        "pressure_source_load_state_mixed", "pressure_source_load_state_n05", "pressure_source_load_state_n06")
    for mode in ("sparse", "dense")
]
E3_RAW_FOLDERS = {"pressure_source_load_state": "pressure_source/load_reference_source", "pressure_load_state": "pressure/load_reference"}


@pytest.mark.parametrize("name", E3_PACKAGE_NAMES)
def test_current_native_builder_matches_committed_package_file_bytes_on_identical_inputs(name):
    stem, mode = name.rsplit("_", 1)
    family = "pressure_source_load_state" if stem.startswith("pressure_source_load_state_") else "pressure_load_state"
    case = stem.removeprefix(family + "_")
    solver_mode = "sparse_interactive" if mode == "sparse" else "dense_scrutiny"
    raw = json.loads((PROJECT / f"fixtures/product_preview/{E3_RAW_FOLDERS[family]}/{case}-{solver_mode}.raw.json").read_text())
    analysis = json.loads((PROJECT / f"fixtures/results/{name}.analysis_run.json").read_text())
    packet = sn.build_stress_neutral_export_package(source_envelope=raw, analysis_record=analysis, **arguments(raw, analysis))
    file_bytes = (json.dumps(packet, ensure_ascii=False, separators=(",", ":"), allow_nan=False) + "\n").encode("utf-8")
    path = PROJECT / f"fixtures/results/{name}.stress_neutral.json"
    # Regenerated only when a bound input moves (a table pin, the analysis record); never to silence a builder change.
    if os.environ.get("STRESS_NEUTRAL_WRITE_FIXTURES") == "1":
        path.write_bytes(file_bytes)
    assert file_bytes == path.read_bytes()
    assert list(sn.materialized_members(packet)) == sn.MEMBERS


# Epoch 8 (DEL-04-07 Design desktop-exact-outputs, section 6.2): the plain exact twins, the desktop builders'
# equal-input oracle for eligible plain results, which no load/reference-state twin covers. Python's AnalysisRun
# record and package over committed plain raws: pressure-2's connected mechanics with its request, and
# pressure-source-1's n05 raw control with its request (whose registered invocation makes it eligible).
PLAIN_MODES = {"sparse": "sparse_interactive", "dense": "dense_scrutiny"}
PLAIN_TWINS = {
    **{f"pressure_plain_connected_{short}": (f"fixtures/results/pressure_connected_mechanics_{short}.json",
        "core/product_physics/tests/fixtures/exact_pressure_connected_request.json", mode)
       for short, mode in PLAIN_MODES.items()},
    **{f"pressure_source_plain_n05_{short}": (f"fixtures/product_preview/pressure_source/physics_source/n05-{mode}.raw.json",
        "fixtures/product_preview/pressure_source/physics_source/n05.request.json", mode)
       for short, mode in PLAIN_MODES.items()},
}


def _plain_twin_bytes(value) -> bytes:
    """Compact bytes plus a newline, as the load/reference-state carriers and package witnesses are written."""
    return (json.dumps(value, ensure_ascii=False, separators=(",", ":"), allow_nan=False) + "\n").encode("utf-8")


@pytest.mark.parametrize("name", list(PLAIN_TWINS))
def test_plain_exact_twins_are_reproduced_byte_for_byte(name):
    raw_path, request_path, mode = PLAIN_TWINS[name]
    raw = json.loads((PROJECT / raw_path).read_text())
    request = json.loads((PROJECT / request_path).read_text())
    before = deepcopy(raw)
    identity = _source_contract(raw)[0]
    assert identity == (PRESSURE_SOURCE_CONTRACT_ID if name.startswith("pressure_source_") else PRESSURE_CONTRACT_ID)
    assert "load_reference_states" not in raw["contract_evidence"]
    assert request["model"]["project"]["id"] == raw["model_ref"]
    bases = [case["basis_ref"] for case in raw["numerical_quality"]["cases"]]
    assert [base["ref_id"] for base in bases] == [case["id"] for case in request["model"]["load_cases"]]
    # The inputs are eligible plain results: pressure-2 on its own, pressure-source-1 with its registered invocation.
    invocation = {"request": request, "solver_mode": mode} if identity == PRESSURE_SOURCE_CONTRACT_ID else None
    assert numerical_use_standing(raw, bases, invocation) == "numerically_eligible"
    record = build_analysis_run_v0_3(raw, input_manifest_ref={"object_type": "InputManifest", "ref": f"manifest:{name}"}, input_manifest_hash="1" * 64)
    validate_analysis_run_v0_3(record, raw)
    assert verify_analysis_run_record(record) == "match"
    packet = sn.build_stress_neutral_export_package(source_envelope=raw, analysis_record=record, **arguments(raw, record))
    assert raw == before
    validate_instance(json.loads(SCHEMA.read_text()), packet, instance_label=name)
    sn.validate_stress_neutral_export_package(packet, source_envelope=raw, analysis_record=record)
    sn.validate_stress_neutral_export_package(packet, **bound(raw))
    assert packet["contract_evidence"] == raw["contract_evidence"]
    assert packet.get("source_block_recovery") == raw.get("source_block_recovery")
    assert list(sn.materialized_members(packet)) == sn.MEMBERS
    # Written only when a bound input moves (a raw, a table pin); never to silence a builder change.
    for suffix, value in (("analysis_run", record), ("stress_neutral", packet)):
        data = _plain_twin_bytes(value)
        path = PROJECT / f"fixtures/results/{name}.{suffix}.json"
        if os.environ.get("STRESS_NEUTRAL_WRITE_FIXTURES") == "1":
            path.write_bytes(data)
        assert path.read_bytes() == data, suffix


# Epoch 8 (DEL-04-07 Design desktop-exact-outputs, section 6.2: "the Python validator test reads the desktop-shaped
# members through its own checks"). The desktop-built package and the desktop 0.3 record it binds, for the plain
# inputs, are written by the desktop oracle (apps/desktop/src/features/result-export/exactOutputTwins.test.tsx,
# DESKTOP_STRESS_NEUTRAL_WRITE_FIXTURES=1) and read here by Python's own validators with the committed source raw.
@pytest.mark.parametrize("name", list(PLAIN_TWINS))
def test_desktop_shaped_plain_packages_pass_the_python_validators(name):
    raw_path, request_path, _mode = PLAIN_TWINS[name]
    raw = json.loads((PROJECT / raw_path).read_text())
    request = json.loads((PROJECT / request_path).read_text())
    record = json.loads((PROJECT / f"apps/desktop/src/test-support/desktop-shaped/desktop_shaped_{name}.analysis_run.json").read_text())
    packet = json.loads((PROJECT / f"apps/desktop/src/test-support/desktop-shaped/desktop_shaped_{name}.stress_neutral.json").read_text())
    before = deepcopy((raw, record, packet))
    # The desktop record, against its source.
    validate_analysis_run_v0_3(record, raw)
    assert verify_analysis_run_record(record) == "match"
    expected_basis_refs = [{"object_type": "LoadCase", "ref": case["id"]} for case in request["model"]["load_cases"]]
    assert record["analysis_run"]["load_basis_refs"] == expected_basis_refs
    # The desktop package, through the schema and Python's package validator: with its source and record, and on its
    # own (pressure-2 is read through its source; pressure-source-1 standalone).
    validate_instance(json.loads(SCHEMA.read_text()), packet, instance_label=f"desktop_shaped_{name}")
    sn.validate_stress_neutral_export_package(packet, source_envelope=raw, analysis_record=record, expected_basis_refs=expected_basis_refs)
    sn.validate_stress_neutral_export_package(packet, **bound(raw))
    assert packet["contract_evidence"] == raw["contract_evidence"]
    assert packet.get("source_block_recovery") == raw.get("source_block_recovery")
    assert (raw, record, packet) == before
