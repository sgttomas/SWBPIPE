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
from tests.verifiers.canonical_json import canonical_json  # noqa: E402
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
