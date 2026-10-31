#!/usr/bin/env python3
"""The reference-system fixture, tranche 1 (DEL-09-01 `Work/reference-system.md`, exit criterion 1).

Checks that the committed files are the generator's output, that the model is a current model document, that it
solves on EX with every tranche-1 case solved and nothing refused, that the spec's §10 ledger and the authoring
file's pending flags agree, and that no key result carries an expected value without a recorded basis. Invented
inputs only."""

from __future__ import annotations

import importlib.util
import json
import os
from pathlib import Path
import re
import subprocess
import sys

import pytest


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT / "tests") not in sys.path:
    sys.path.insert(0, str(ROOT / "tests"))

from schema_validation import JsonSchemaDependencyMissing, _skip_or_note_missing_jsonschema, validate_instance  # noqa: E402

FIXTURE = ROOT / "fixtures" / "reference_system"
SCHEMA = ROOT / "schemas" / "model.schema.yaml"


def load_generator():
    spec = importlib.util.spec_from_file_location("generate_reference_system", FIXTURE / "generate_reference_system.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


GENERATOR = load_generator()


def read(name):
    return json.loads((FIXTURE / name).read_text(encoding="utf-8"))


MODEL = read(GENERATOR.MODEL_FILE)
AUTHORING = read(GENERATOR.AUTHORING_FILE)
KEY_RESULTS = read(GENERATOR.KEY_RESULTS_FILE)


def all_ids(value, found=None):
    found = set() if found is None else found
    if isinstance(value, dict):
        if isinstance(value.get("id"), str):
            found.add(value["id"])
        for item in value.values():
            all_ids(item, found)
    elif isinstance(value, list):
        for item in value:
            all_ids(item, found)
    return found


# -- (a) the committed files are the generator's output ---------------------------------------------------------------
def test_the_generator_check_passes_and_is_deterministic(capsys):
    assert GENERATOR.main(["--check"]) == 0, capsys.readouterr().out
    assert GENERATOR.generate() == GENERATOR.generate()


def test_the_check_reports_a_changed_file(tmp_path, monkeypatch, capsys):
    for name in (GENERATOR.MODEL_FILE, GENERATOR.AUTHORING_FILE, GENERATOR.KEY_RESULTS_FILE, GENERATOR.CAPTURE_FILE):
        (tmp_path / name).write_bytes((FIXTURE / name).read_bytes())
    monkeypatch.setattr(GENERATOR, "HERE", tmp_path)
    assert GENERATOR.main(["--check"]) == 0
    (tmp_path / GENERATOR.KEY_RESULTS_FILE).write_text("{}\n", encoding="utf-8")
    assert GENERATOR.main(["--check"]) == 1
    assert capsys.readouterr().out.split() == ["differs:", GENERATOR.KEY_RESULTS_FILE]


def test_a_weight_input_change_makes_the_self_weight_capture_stale(monkeypatch):
    """W is the product generator's captured output; a generation whose weight inputs moved refuses."""
    monkeypatch.setitem(GENERATOR.SECTIONS, "S270", (270, 9.5, 1.0, 1.5, 60, 140, 820))
    with pytest.raises(SystemExit, match="stale"):
        GENERATOR.generate()


def test_the_version_token_is_not_a_weight_input(monkeypatch):
    """The 0.9.0 ripple bumps one constant and re-runs the generator without a new capture."""
    monkeypatch.setattr(GENERATOR, "MODEL_DOCUMENT_VERSION", "9.9.9")
    model = json.loads(GENERATOR.generate()[GENERATOR.MODEL_FILE])
    assert model["schema_version"] == "9.9.9"
    assert model["load_cases"] == MODEL["load_cases"]


# -- (b) a current model document on EX -------------------------------------------------------------------------------
def test_the_model_validates_as_the_current_model_document():
    schema = json.loads(SCHEMA.read_text(encoding="utf-8"))
    assert schema["properties"]["schema_version"]["enum"] == [MODEL["schema_version"]]
    assert MODEL["schema_version"] == GENERATOR.MODEL_DOCUMENT_VERSION
    assert MODEL["document_kind"] == "openpipestress.product_preview.model"
    assert MODEL["pressure_contract"] == {"version": "3.0.0", "mode": "exact_pressure_v3",
                                          "shear_model": "euler_bernoulli"}
    try:
        validate_instance(schema, MODEL, schema_label=str(SCHEMA), instance_label=GENERATOR.MODEL_FILE)
    except JsonSchemaDependencyMissing as exc:
        _skip_or_note_missing_jsonschema(exc)


def test_tranche_one_holds_its_families_and_leaves_the_pending_ones_out():
    ids = all_ids(MODEL)
    assert len(MODEL["nodes"]) == 27 and len(MODEL["pipe_segments"]) == 26
    for present in ("component:RS-B1", "component:RS-B2", "component:RS-B3", "component:RS-F1", "component:RS-V1",
                    "component:RS-F2", "component:RS-EJ1", "support:RS-FR1V", "load:RS-SUS", "load:RS-OPE",
                    "load:RS-SAM", "combination:RS-EXP", "combination:RS-dSAM"):
        assert present in ids, present
    for row in AUTHORING["pending"]:
        if not row["in_tranche_1"] and row["family"] != "RS-FR1 with RS-FR1V: lift-off friction shoe pending RT-A3 S7":
            assert not set(row["entities"]) & ids, row["family"]
    # RS-P180 carries no load: no primitive targets it and no case weighs it.
    for case in MODEL["load_cases"]:
        assert all(load["target"].get("pipe") != "pipe:RS-P180" for load in case["primitive_loads"])
        assert all("pipe:RS-P180" not in region["member_pipe_ids"] for region in case["pressure_regions"])


def test_authoring_marks_person_only_for_the_bend_k_sifs_and_their_source():
    person = {(entry["id"], path) for entry in AUTHORING["inputs"] for path, mark in entry["fields"].items()
              if mark == "person"}
    expected = {(f"component:{tag}", f"modifiers.{field}") for tag in ("RS-B1", "RS-B2", "RS-B3")
                for field in ("flexibility_factor_user_value", "sif_in_plane_user_value", "sif_out_of_plane_user_value",
                              "source_reference")}
    assert person == expected
    assert {mark for entry in AUTHORING["inputs"] for mark in entry["fields"].values()} == {"person",
                                                                                             "agent_authorable"}
    tee = AUTHORING["pending_inputs"][0]
    assert tee["fields"]["modifiers.branch_header_sif_user_value"] == "person"
    model_bends = {c["id"]: c for c in MODEL["components"] if c["kind"] == "bend"}
    for bend in AUTHORING["bends"]:
        component = model_bends[bend["component"]]
        assert component["node"] == bend["creates"]["tangent_nodes"][0]
        assert component["geometry"]["bend_pipe_ref"] == bend["creates"]["arc_pipe"]
        arc = next(p for p in MODEL["pipe_segments"] if p["id"] == bend["creates"]["arc_pipe"])
        assert [arc["from"], arc["to"]] == bend["creates"]["tangent_nodes"]
        assert arc["y_reference"] == bend["arc_y_reference"]


# -- (c) the EX solve -------------------------------------------------------------------------------------------------
def runner_request():
    return {
        "request_id": "reference-system-tranche-1",
        "operation": "solve",
        "operation_ref": {"ref_type": "api_operation", "ref_id": "solve"},
        "project_ref": {"ref_type": "project", "ref_id": MODEL["project"]["id"]},
        "model_ref": {"ref_type": "model", "ref_id": "reference-system"},
        "unit_system_ref": {"ref_type": "unit_system", "ref_id": "invented-si"},
        "load_basis_refs": [{"ref_type": "load_case", "ref_id": case["id"]} for case in MODEL["load_cases"]],
        "input_manifest_ref": {"ref_type": "audit_manifest", "ref_id": "reference-system-tranche-1"},
        "requested_outputs": ["result_envelope", "audit_manifest", "diagnostics"],
        "privacy": {"local_only": True, "telemetry_allowed": False, "private_payload_redacted": True,
                    "classification": "public_metadata"},
        "provenance": {"source_name": "invented reference-system fixture",
                       "source_location": "fixtures/reference_system/reference_system.json",
                       "source_license": "project invented", "contributor": "OpenPipeStress",
                       "contributor_certification": "invented non-engineering example",
                       "redistribution_status": "invented_non_engineering_example", "review_status": "pending"},
        "professional_boundary": {"human_review_required": True, "software_makes_compliance_claim": False,
                                  "software_makes_certification_claim": False,
                                  "software_makes_sealing_claim": False, "software_makes_approval_claim": False,
                                  "software_makes_authentication_claim": False},
        "tbd_decisions": {"final_cli_command_syntax": "SETTLED_DEC_065", "package_scripts": "SETTLED_DEC_065",
                          "process_invocation": "SETTLED_DEC_065", "network_access": "SETTLED_DEC_065",
                          "filesystem_mutation_policy": "SETTLED_DEC_065", "ci_provider": "TBD",
                          "release_matrix": "TBD", "public_transport_protocol": "TBD",
                          "external_adapter_formats": "TBD", "physical_project_container": "TBD"},
    }


@pytest.fixture(scope="module")
def solved():
    runner = os.environ.get("OPENPIPESTRESS_RUNNER_BIN")
    if not runner:
        pytest.skip("OPENPIPESTRESS_RUNNER_BIN is not set")
    body = {"request": runner_request(), "solve": {"preview_model": {"model": MODEL, "materials": []}}}
    completed = subprocess.run([runner, "solve", "--explicit-local-private-intent", "--input", "-"],
                               input=json.dumps(body), capture_output=True, text=True, check=False)
    assert completed.stdout, completed.stderr
    return completed.returncode, json.loads(completed.stdout)


BLOCKING = {"blocking", "error", "fatal"}


def test_every_tranche_one_case_solves_on_ex_with_nothing_refused(solved):
    code, output = solved
    payload = output["payload"]
    envelope = payload["mechanics_envelope"]
    diagnostics = envelope["diagnostics"] + payload.get("diagnostics", []) + output.get("findings", [])
    refused = [(d.get("code"), d.get("message")) for d in diagnostics if d.get("severity") in BLOCKING]
    assert not refused, refused
    assert code == 0 and not output.get("blocked")
    assert envelope["status"]["mechanics"] == "MECHANICS_SOLVED"
    # No pending family's refusal code appears, and no self-weight lineage diagnostic (W is fresh).
    codes = {d.get("code") for d in diagnostics}
    pending_codes = {code for row in AUTHORING["pending"] for code in re.findall(r"`([A-Z][A-Z0-9_]+)`",
                                                                                 row["expected_until_then"])}
    assert not codes & pending_codes, codes & pending_codes
    assert not {c for c in codes if c and c.startswith("SELF_WEIGHT")}
    cases = [case["id"] for case in MODEL["load_cases"]]
    assert cases == ["load:RS-SUS", "load:RS-OPE", "load:RS-SAM"]
    quality = {entry["basis_ref"]["ref_id"]: entry for entry in envelope["numerical_quality"]["cases"]}
    assert set(cases) <= set(quality)
    bases = {(row["basis_ref"]["ref_type"], row["basis_ref"]["ref_id"]) for row in envelope["results"]
             if row.get("basis_ref") and row.get("kind") == "support_reaction_component_v2"}
    assert {("load_case", case) for case in cases} | {("combination", c["id"]) for c in MODEL["combinations"]} <= bases
    states = {state["load_case_id"] for state in envelope["contract_evidence"]["load_reference_states"]}
    assert states == set(cases)


# -- (d) the §10 ledger and the pending flags agree --------------------------------------------------------------------
def test_the_generator_ledger_and_the_authoring_pending_flags_agree():
    # The specification's §10 table is compared with these by tools/validation/test_reference_system_ledger.py,
    # which runs in the repository checks (the product checkout carries no execution/ records).
    authored = [(row["family"], row["state"], row["row_lane"], row["expected_until_then"])
                for row in AUTHORING["pending"]]
    assert authored == [tuple(row) for row in GENERATOR.LEDGER]
    for row in AUTHORING["pending"]:
        assert row["in_tranche_1"] == row["state"].startswith("in tranche 1"), row["family"]


# -- (e) no expected value without a recorded basis --------------------------------------------------------------------
def test_every_key_result_is_named_with_a_basis_and_no_unrecorded_expected_value():
    ids = [entry["id"] for entry in KEY_RESULTS["results"]]
    assert ids == [f"KR-{n:02d}" for n in range(1, 11)] + [f"EQ-{n}" for n in range(1, 8)]
    for entry in KEY_RESULTS["results"]:
        assert entry["basis"] in {"identity", "closed_form", "verified_solve"}, entry["id"]
        assert entry["description"] and entry["cases"] and entry["spec_basis"], entry["id"]
        expected = entry["expected"]
        if expected is not None:
            record = expected.get("basis_record", "") if isinstance(expected, dict) else ""
            assert record.startswith("validation/references/reference_system/") and (ROOT / record).is_file(), \
                entry["id"]
