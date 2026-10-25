#!/usr/bin/env python3
"""The fully populated invented handoff package witness (DEL-15-01 Design
`canonical-model-export.md` §3-§4, W-6b).

`fixtures/handoff/invented_witness.swbhandoff/` is the desktop producer's exact
output for an invented model and local store (pinned by the Rust test
`witness_package_is_the_producers_output` in
`apps/desktop/src-tauri/src/handoff_package.rs`). Here:

- its manifest validates against `schemas/handoff_package.schema.json`;
- its model member validates against `schemas/model.schema.yaml`
  (`$defs/DesktopDocument`);
- `core/handoff/exporter/workflow.py` given the manifest emits no
  `EXP-HANDOFF-*` or `EXP-PRIVACY-*` diagnostic; `EXP-TARGET-MAPPING-MISSING`
  is expected, since DEL-15-02 has no target-mapping contract;
- the Python verifier twin (`tests/verifiers/handoff_package.py`) verifies it,
  and one changed byte in either member fails.

The witness records review context only; nothing in it is an acceptance.
"""

from __future__ import annotations

from hashlib import sha256
import json
from pathlib import Path
import shutil
import sys

from unittest.mock import patch

import pytest
from jsonschema import Draft202012Validator

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))
if str(ROOT / "tests") not in sys.path:
    sys.path.insert(0, str(ROOT / "tests"))

from core.handoff.exporter import build_handoff_export_workflow  # noqa: E402
from schema_validation import validate_instance  # noqa: E402
from tests.verifiers.canonical_json import canonical_json_unchecked as canonical_json  # noqa: E402
from tests.verifiers.handoff_package import HandoffVerificationError, verify_handoff_package  # noqa: E402

PACKAGE = ROOT / "fixtures" / "handoff" / "invented_witness.swbhandoff"
MANIFEST = PACKAGE / "handoff_package.json"
MODEL = PACKAGE / "model" / "model.json"
HANDOFF_SCHEMA = ROOT / "schemas" / "handoff_package.schema.json"
DESKTOP_PROFILE = {
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "$ref": "https://openpipestress.org/schemas/model.schema.yaml#/$defs/DesktopDocument",
}


def instance() -> dict:
    return json.loads(MANIFEST.read_text(encoding="utf-8"))


def manifest() -> dict:
    return instance()["handoff_package_manifest"]


def test_the_manifest_validates_against_the_handoff_schema():
    schema = json.loads(HANDOFF_SCHEMA.read_text(encoding="utf-8"))
    errors = sorted(Draft202012Validator(schema).iter_errors(instance()), key=lambda error: list(error.path))
    assert [f"{list(error.path)}: {error.message}" for error in errors] == []


def test_the_manifest_is_fully_populated_from_its_sources():
    found = manifest()
    for member in ("library_refs", "rule_pack_refs", "unresolved_assumptions", "warnings", "diagnostics",
                   "unsupported_behavior_flags", "model_state_refs", "checksums"):
        assert found[member], member
    ids = found["entity_ids"]
    for slot in ("node_ids", "component_ids", "element_ids", "load_case_ids", "model_state_ids", "mapping_id_refs"):
        assert ids[slot], slot
    assert ids["model_state_ids"] == [found["model_state_refs"][0]["model_state_ref"]["ref"]]
    assert found["model_basis"]["basis_state_ref"] == found["model_state_refs"][0]["model_state_ref"]
    # Owner decision for v0.1: the package builds on saved states and references no runs.
    assert found["analysis_run_refs"] == [] and found["result_export_refs"] == []
    units = found["units_manifest"]
    assert {units[key] for key in ("coordinate_unit", "force_unit", "moment_unit", "rotation_unit", "stress_unit")} <= {
        "unit:meter", "unit:newton", "unit:newton_meter", "unit:radian", "unit:megapascal"}
    assert found["privacy"] == {
        "classification": "private_project_data", "local_only": True, "telemetry_allowed": False,
        "private_payload_embedded": False, "protected_payload_embedded": False,
        "commercial_tool_payload_embedded": False, "redaction_refs": []}
    assert (found["redistribution_classification"], found["review_classification"]) == ("private_only", "unreviewed")
    assert instance()["handoff_contract_status"]["physical_package_container"] == "TBD"
    boundary = found["professional_boundary"]
    assert boundary["human_review_required"] is True
    assert not any(value for key, value in boundary.items() if key.startswith("software_"))


def test_the_model_member_validates_as_a_desktop_document_and_is_bound_by_hash():
    model_bytes = MODEL.read_bytes()
    document = json.loads(model_bytes)
    validate_instance(DESKTOP_PROFILE, document, schema_label="DesktopDocument", instance_label=str(MODEL.relative_to(ROOT)))
    assert canonical_json(document).encode("utf-8") == model_bytes
    found = manifest()
    assert found["model_hash"]["value"] == sha256(model_bytes).hexdigest()
    assert found["checksums"][0]["value"] == sha256(model_bytes).hexdigest()
    # Bound, not embedded: no model content member appears in the manifest, only ids.
    assert not {"nodes", "pipe_segments", "materials", "position", "elastic_modulus"} & set(walk_keys(found))
    assert sorted(node["id"] for node in document["nodes"]) == found["entity_ids"]["node_ids"]


def walk_keys(value):
    if isinstance(value, dict):
        for key, item in value.items():
            yield key
            yield from walk_keys(item)
    elif isinstance(value, list):
        for item in value:
            yield from walk_keys(item)


def test_the_export_workflow_finds_no_handoff_or_privacy_finding():
    """The workflow's own diagnostics, as it hands them to its route control
    (which withholds their codes from the controlled export)."""
    from core.handoff.exporter import workflow

    seen: list[dict] = []
    original = workflow.control_route_export

    def capture(payload, **options):
        seen.extend(dict(item) for item in options.get("source_findings", ()))
        return original(payload, **options)

    with patch.object(workflow, "control_route_export", side_effect=capture):
        build_handoff_export_workflow(export_workflow_id="export:invented-witness", handoff_package=instance())
    codes = sorted({item["code"] for item in seen})
    assert "EXP-TARGET-MAPPING-MISSING" in codes
    assert [code for code in codes if code.startswith(("EXP-HANDOFF-", "EXP-PRIVACY-"))] == []


def test_the_python_verifier_twin_verifies_the_witness():
    verified = verify_handoff_package(PACKAGE)
    assert verified["model_hash"] == f"sha256:{sha256(MODEL.read_bytes()).hexdigest()}"
    assert verified["export_hash"] == f"sha256:{manifest()['checksums'][1]['value']}"


@pytest.mark.parametrize("member", ["model/model.json", "handoff_package.json"])
def test_one_changed_byte_in_either_member_fails(tmp_path, member):
    copy = tmp_path / "invented_witness.swbhandoff"
    shutil.copytree(PACKAGE, copy)
    target = copy / member
    original = target.read_bytes()
    text = original.decode("utf-8")
    positions = set(range(0, len(original), max(1, len(original) // 200))) | {len(original) - 1}
    for checksum in manifest()["checksums"]:
        at = text.find(checksum["value"])
        if at >= 0:
            positions |= set(range(at, at + len(checksum["value"])))
    for position in sorted(positions):
        tampered = bytearray(original)
        tampered[position] ^= 0x01
        target.write_bytes(bytes(tampered))
        with pytest.raises(HandoffVerificationError):
            verify_handoff_package(copy)
    target.write_bytes(original)
    assert verify_handoff_package(copy)


def test_the_verifier_refuses_extra_members_and_tolerates_finder_metadata(tmp_path):
    copy = tmp_path / "invented_witness.swbhandoff"
    shutil.copytree(PACKAGE, copy)
    (copy / ".DS_Store").write_bytes(b"invented")
    assert verify_handoff_package(copy)
    (copy / "extra.json").write_text("{}", encoding="utf-8")
    with pytest.raises(HandoffVerificationError):
        verify_handoff_package(copy)


# --- Review round 1 (Codex; scratch/rep-w6b-review/python-regressions.patch),
# built here from the committed producer outputs ------------------------------

from tests.verifiers.canonical_json import canonical_json_unchecked, parse_json  # noqa: E402
from tests.verifiers.handoff_package import checksum_inventory, export_hash  # noqa: E402

BINARY64 = ROOT / "fixtures" / "handoff" / "invented_binary64_witness.swbhandoff"
VECTORS = ROOT / "fixtures" / "handoff" / "canonical_text_vectors.json"


def _reseal(directory: Path, edit_manifest, model_text: str | None = None) -> None:
    """Rewrites a copied package as a forger would: edit the manifest (and the
    model member), then recompute the export hash and the checksum inventory so
    that every hash in it is consistent."""
    model_bytes = model_text.encode("utf-8") if model_text is not None else (directory / "model" / "model.json").read_bytes()
    if model_text is not None:
        (directory / "model" / "model.json").write_bytes(model_bytes)
    model_hex = sha256(model_bytes).hexdigest()
    package = json.loads((directory / "handoff_package.json").read_text(encoding="utf-8"))
    inner = package["handoff_package_manifest"]
    edit_manifest(inner, model_hex)
    sealed = {key: value for key, value in inner.items() if key != "checksums"}
    sealed["checksums"] = checksum_inventory(inner["model_hash"]["payload_ref"], inner["package_identity"]["handoff_package_id"], model_hex, export_hash(inner))
    package["handoff_package_manifest"] = sealed
    (directory / "handoff_package.json").write_text(canonical_json_unchecked(package), encoding="utf-8")


def _schema_valid(directory: Path) -> None:
    schema = json.loads(HANDOFF_SCHEMA.read_text(encoding="utf-8"))
    Draft202012Validator(schema).validate(json.loads((directory / "handoff_package.json").read_text(encoding="utf-8")))


@pytest.mark.parametrize("package", [PACKAGE, BINARY64], ids=["witness", "binary64"])
def test_produced_packages_are_schema_valid_and_verify(package):
    _schema_valid(package)
    assert verify_handoff_package(package)


def test_python_accepts_the_produced_unchecked_binary64_model():
    """Finding 6: a model holding 1e20 (outside the checked profile) is W-6a's
    bytes; the twin reads it with the same unchecked serializer."""
    text = (BINARY64 / "model" / "model.json").read_text(encoding="utf-8")
    assert "100000000000000000000" in text
    assert verify_handoff_package(BINARY64)["model_hash"] == f"sha256:{sha256(text.encode('utf-8')).hexdigest()}"


def test_the_unchecked_serializer_matches_the_rust_vectors():
    vectors = json.loads(VECTORS.read_text(encoding="utf-8"))["vectors"]
    assert len(vectors) >= 30
    for vector in vectors:
        text = canonical_json_unchecked(parse_json(vector["input_json"]))
        assert text == vector["canonical_text"], vector["input_json"]
        assert sha256(text.encode("utf-8")).hexdigest() == vector["sha256"], vector["input_json"]
    cases = json.loads((ROOT / "fixtures" / "canonical_hash" / "cases.json").read_text(encoding="utf-8"))["cases"]
    for case in cases:
        assert canonical_json_unchecked(parse_json(case["input_json"])) == case["expected_canonical"], case["case_id"]


def _forged_subject(manifest, _model_hex):
    manifest["model_hash"]["payload_ref"] = {"object_type": "RulePack", "ref": "rule-pack:unrelated"}


def _substituted_keep_identity(manifest, model_hex):
    # Every hash names the substituted bytes; the identity and inventories stay the original model's.
    for checksum in (manifest["model_hash"], manifest["model_basis"]["hash_refs"][0], manifest["model_state_refs"][0]["hash_refs"][0]):
        checksum["value"] = model_hex
    for key, prefix in (("handoff_package_id", "handoff"), ("manifest_id", "manifest")):
        project = manifest["package_identity"][key].split(":")[1]
        manifest["package_identity"][key] = f"{prefix}:{project}:{model_hex[:12]}"


def _substituted_model_text() -> str:
    other = parse_json(MODEL.read_text(encoding="utf-8"))
    other["project"]["id"] = "project:unrelated-model"
    other["nodes"][0]["id"] = "node:unrelated-node"
    return canonical_json_unchecked(other)


def _substituted_hash_only(manifest, model_hex):
    manifest["model_hash"]["value"] = model_hex


@pytest.mark.parametrize("forgery", ["forged-subject", "substituted-model", "substituted-model-hash-only", "rule-pack-subject", "library-subject"])
def test_semantically_forged_subjects_are_rejected(tmp_path, forgery):
    """Finding 3: schema-valid packages whose hashes are all consistent but whose
    subjects are not the model member's are refused."""
    copy = tmp_path / "forged.swbhandoff"
    shutil.copytree(PACKAGE, copy)
    if forgery == "forged-subject":
        _reseal(copy, _forged_subject)
    elif forgery == "substituted-model":
        _reseal(copy, _substituted_keep_identity, _substituted_model_text())
    elif forgery == "substituted-model-hash-only":
        _reseal(copy, _substituted_hash_only, _substituted_model_text())
    elif forgery == "rule-pack-subject":
        _reseal(copy, lambda manifest, _: manifest["rule_pack_refs"][0]["checksum"].update(payload_ref={"object_type": "RulePack", "ref": "unrelated_pack"}))
    else:
        _reseal(copy, lambda manifest, _: manifest["library_refs"][0]["checksum"].update(payload_ref={"object_type": "Model", "ref": "project:slots"}))
    _schema_valid(copy)
    with pytest.raises(HandoffVerificationError):
        verify_handoff_package(copy)


@pytest.mark.parametrize("change", ["member", "inventory-label", "extra-inventory"])
def test_obvious_substitution_and_forged_inventory_refused(tmp_path, change):
    copy = tmp_path / "copy.swbhandoff"
    shutil.copytree(PACKAGE, copy)
    if change == "member":
        (copy / "model" / "model.json").write_text("{}", encoding="utf-8")
    else:
        package = json.loads((copy / "handoff_package.json").read_text(encoding="utf-8"))
        inventory = package["handoff_package_manifest"]["checksums"]
        if change == "inventory-label":
            inventory[0]["payload_ref"]["label"] = "other-model.json"
        else:
            inventory.append(dict(inventory[0]))
        (copy / "handoff_package.json").write_text(canonical_json_unchecked(package), encoding="utf-8")
    with pytest.raises(HandoffVerificationError):
        verify_handoff_package(copy)


def test_review2_empty_basis_state_id_is_refused(tmp_path):
    copy = tmp_path / "empty-state.swbhandoff"
    shutil.copytree(PACKAGE, copy)
    def mutate(manifest, _):
        for ref in (manifest["model_basis"]["basis_state_ref"],
                    manifest["model_state_refs"][0]["model_state_ref"],
                    manifest["model_state_refs"][0]["hash_refs"][1]["payload_ref"]):
            ref["ref"] = ""
        manifest["entity_ids"]["model_state_ids"] = [""]
    _reseal(copy, mutate)
    with pytest.raises(HandoffVerificationError):
        verify_handoff_package(copy)
