"""The product readers on a solved hydrostatic test case (DEL-05-01 REQ-014; DEL-04-07 REQ-011).

The two documents of ``fixtures/hydrostatic_test/four_span_reader_models.json``, written by the applier's journey test:
the four-span model taken through its journey (one held test case) and its load and reference-state twin (the held test
case beside an ordinary case). Each is solved by the runner in both modes. The Python reader
(``tests/verifiers/reader_verdict.py``) and the Rust product reader (``openpipestress-runner read pressure``) admit every
envelope with and without its model, and their verdict states nothing of the test case beyond admitting its mark. Each
forged envelope is refused by both, by the code named: the mark's model binding (presence, choice, held supports,
contents) and the constant-effort binding's held set (a held case applies no held support's force; every other case,
and a test case that holds none, applies them all).

On the twin, the record of load sources states each generated self-weight entry's document magnitude as authored and the
applied magnitude with the test contents, and both readers join that entry to its contents item; three forgeries of
that rule are refused by its own code.

Needs ``OPENPIPESTRESS_RUNNER_BIN`` (``tests/conftest.py`` builds it unless ``OPENPIPESTRESS_NO_CARGO_BUILD`` is set);
skipped without it.
"""
from __future__ import annotations

from copy import deepcopy
import json
import math
import os
from pathlib import Path
import subprocess

import pytest

from tests.verifiers import reader_verdict as rv

ROOT = Path(__file__).resolve().parents[1]
MODELS = json.loads((ROOT / "fixtures/hydrostatic_test/four_span_reader_models.json").read_text(encoding="utf-8"))["models"]
MODES = ("sparse_interactive", "dense_scrutiny")
TEST_CASE = "HT-4S-TEST"
ORDINARY = "HT-4S-OPERATING"
APPLIED_LOAD = "constant_effort_support_applied_load"
SHAPE = "SOURCE_PHYSICS_EVIDENCE_INVALID"


def runner() -> str:
    path = os.environ.get("OPENPIPESTRESS_RUNNER_BIN")
    if not path or not Path(path).is_file():
        pytest.skip("OPENPIPESTRESS_RUNNER_BIN is not set")
    return path


def runner_request() -> dict:
    return {
        "request_id": "hydrostatic-test-readers", "operation": "solve",
        "operation_ref": {"ref_type": "api_operation", "ref_id": "solve"},
        "project_ref": {"ref_type": "project", "ref_id": "project:hydrostatic-test"},
        "model_ref": {"ref_type": "model", "ref_id": "hydrostatic-test"},
        "unit_system_ref": {"ref_type": "unit_system", "ref_id": "invented-si"},
        "load_basis_refs": [{"ref_type": "load_case", "ref_id": TEST_CASE}],
        "input_manifest_ref": {"ref_type": "audit_manifest", "ref_id": "hydrostatic-test-readers"},
        "requested_outputs": ["result_envelope", "audit_manifest", "diagnostics"],
        "privacy": {"local_only": True, "telemetry_allowed": False, "private_payload_redacted": True, "classification": "public_metadata"},
        "provenance": {"source_name": "invented hydrostatic test reader models",
                       "source_location": "fixtures/hydrostatic_test/four_span_reader_models.json",
                       "source_license": "project invented", "contributor": "OpenPipeStress",
                       "contributor_certification": "invented non-engineering example",
                       "redistribution_status": "invented_non_engineering_example", "review_status": "pending"},
        "professional_boundary": {"human_review_required": True, "software_makes_compliance_claim": False,
                                  "software_makes_certification_claim": False, "software_makes_sealing_claim": False,
                                  "software_makes_approval_claim": False, "software_makes_authentication_claim": False},
        "tbd_decisions": {"final_cli_command_syntax": "SETTLED_DEC_065", "package_scripts": "SETTLED_DEC_065",
                          "process_invocation": "SETTLED_DEC_065", "network_access": "SETTLED_DEC_065",
                          "filesystem_mutation_policy": "SETTLED_DEC_065", "ci_provider": "TBD", "release_matrix": "TBD",
                          "public_transport_protocol": "TBD", "external_adapter_formats": "TBD", "physical_project_container": "TBD"},
    }


_SOLVED: dict[tuple[str, str], tuple[bytes, dict]] = {}


def body(name: str) -> dict:
    return {"request": runner_request(), "solve": {"preview_model": {"model": MODELS[name], "materials": []}}}


def raw_solve(name: str, mode: str) -> tuple[bytes, dict]:
    """The runner's controlled output bytes and its mechanics envelope."""
    if (name, mode) not in _SOLVED:
        completed = subprocess.run([runner(), "solve", "--explicit-local-private-intent", "--solver-mode", mode, "--input", "-"],
                                   input=json.dumps(body(name)).encode(), capture_output=True, check=False)
        assert completed.stdout, completed.stderr
        envelope = json.loads(completed.stdout)["payload"]["mechanics_envelope"]
        assert envelope["status"]["mechanics"] == "MECHANICS_SOLVED", envelope["diagnostics"]
        assert any("test_condition" in case for case in envelope["contract_evidence"]["exact_cases"])
        _SOLVED[(name, mode)] = (completed.stdout, envelope)
    return _SOLVED[(name, mode)]


def solved(name: str, mode: str) -> dict:
    return deepcopy(raw_solve(name, mode)[1])


def rust_code(envelope: dict, model: dict | None) -> str | None:
    completed = subprocess.run([runner(), "read", "pressure", "--input", "-"],
                               input=json.dumps({"source": envelope, "model": model} if model is not None else {"source": envelope}),
                               capture_output=True, text=True, check=False)
    outcome = json.loads(completed.stdout)["outcome"]
    return None if outcome["ok"] else outcome["error"]["code"]


def python_code(envelope: dict, model: dict | None) -> str | None:
    outcome = rv.validate("pressure", envelope, model)
    return None if outcome["ok"] else outcome["error"]["code"]


def exact_case(envelope: dict, case_id: str) -> dict:
    return next(c for c in envelope["contract_evidence"]["exact_cases"] if c["load_case_id"] == case_id)


@pytest.mark.parametrize("mode", MODES)
@pytest.mark.parametrize("name", sorted(MODELS))
def test_both_readers_admit_the_solved_test_case_with_and_without_its_model(name: str, mode: str) -> None:
    envelope = solved(name, mode)
    for model in (None, MODELS[name]):
        assert python_code(envelope, model) is None, (name, mode, model is not None)
        assert rust_code(envelope, model) is None, (name, mode, model is not None)
        # The verdict states nothing of the test case beyond admitting the mark.
        findings = rv.dispatch(envelope, model)["standing"]["findings"]
        assert not [f for f in findings if "TEST" in f or "CASE_SHAPE" in f or "CONSTANT_EFFORT" in f], findings


def _forge(name: str, mutate) -> tuple[dict, dict]:
    envelope, model = solved(name, "sparse_interactive"), deepcopy(MODELS[name])
    mutate(envelope, model)
    return envelope, model


def _mark(envelope: dict) -> dict:
    return exact_case(envelope, TEST_CASE)["test_condition"]


def _held_test_none(envelope: dict, model: dict) -> None:
    """A test case that holds none, in the model and the mark alike, whose envelope omits the applied-load row its
    constant-effort support would publish in any unheld case."""
    case = next(c for c in model["load_cases"] if c["id"] == TEST_CASE)
    case["test_condition"]["held_supports"] = "none"
    _mark(envelope)["held_supports"] = "none"
    _mark(envelope)["held_support_refs"] = []


def _ordinary_row_dropped(envelope: dict, model: dict) -> None:
    envelope["results"] = [r for r in envelope["results"]
                           if not (r["kind"] == APPLIED_LOAD and r.get("basis_ref", {}).get("ref_id") == ORDINARY)]


def _held_row_added(envelope: dict, model: dict) -> None:
    row = deepcopy(next(r for r in envelope["results"] if r["kind"] == APPLIED_LOAD and r["basis_ref"]["ref_id"] == ORDINARY))
    row["basis_ref"]["ref_id"] = TEST_CASE
    row["id"] = row["id"].replace(ORDINARY, TEST_CASE)
    envelope["results"].append(row)


FORGERIES = [
    # (label, document, mutation, Rust code with the model, Python code with the model)
    ("mark removed from the test case", "journey",
     lambda e, m: exact_case(e, TEST_CASE).pop("test_condition"), "SOURCE_PHYSICS_MODEL_TEST_CONDITION", None),
    ("mark put on the ordinary case", "load_state_twin",
     lambda e, m: exact_case(e, ORDINARY).__setitem__("test_condition", deepcopy(_mark(e))), "SOURCE_PHYSICS_MODEL_TEST_CONDITION", None),
    ("a held reference dropped", "journey",
     lambda e, m: _mark(e)["held_support_refs"].pop(), "SOURCE_PHYSICS_MODEL_TEST_HELD_SUPPORTS", None),
    ("a held reference added", "journey",
     lambda e, m: _mark(e)["held_support_refs"].append("support:a"), "SOURCE_PHYSICS_MODEL_TEST_HELD_SUPPORTS", None),
    ("a resolution item dropped", "journey",
     lambda e, m: _mark(e)["contents_resolution"].pop(), "SOURCE_PHYSICS_MODEL_TEST_CONTENTS", None),
    ("a density changed", "journey",
     lambda e, m: _mark(e)["contents_resolution"][0]["contents_density"].__setitem__("value", 1016.0), "SOURCE_PHYSICS_MODEL_TEST_CONTENTS", None),
    ("an applied load off the mass times the model's gravity by one ulp", "journey",
     lambda e, m: _mark(e)["contents_resolution"][0].__setitem__(
         "applied_n_per_m", math.nextafter(_mark(e)["contents_resolution"][0]["applied_n_per_m"], 0.0)),
     "SOURCE_PHYSICS_MODEL_TEST_CONTENTS", None),
    ("the held choice changed", "journey",
     lambda e, m: _mark(e).__setitem__("held_supports", "none"), "SOURCE_PHYSICS_MODEL_TEST_CONDITION", None),
    ("a held case publishes its held constant-effort support's applied load", "load_state_twin",
     _held_row_added, "SOURCE_PHYSICS_COMBINATION_MODEL_CONSTANT_EFFORT", None),
    ("a test case that holds none omits its constant-effort applied load", "journey",
     _held_test_none, "SOURCE_PHYSICS_COMBINATION_MODEL_CONSTANT_EFFORT", None),
    ("the ordinary case beside the held one omits its constant-effort applied load", "load_state_twin",
     _ordinary_row_dropped, "SOURCE_PHYSICS_COMBINATION_MODEL_CONSTANT_EFFORT", None),
]


@pytest.mark.parametrize("label, name, mutate, code, _", FORGERIES, ids=[f[0] for f in FORGERIES])
def test_each_forgery_is_refused_by_both_readers_with_the_model(label: str, name: str, mutate, code: str, _) -> None:
    envelope, model = _forge(name, mutate)
    assert rust_code(envelope, model) == code, label
    assert python_code(envelope, model) == code, label


def _entry(envelope: dict, case_id: str, index: int) -> dict:
    """The record of load sources' stored primitive entry ``index`` of ``case_id``."""
    record = next(r for r in envelope["contract_evidence"]["load_reference_states"] if r["load_case_id"] == case_id)
    return [c for c in record["contributions"] if c["owner_kind"] == "stored_primitive"][index]


@pytest.mark.parametrize("mode", MODES)
def test_the_record_of_load_sources_states_the_documents_magnitude_and_the_applied_one(mode: str) -> None:
    """On the marked case, each generated self-weight entry's authored magnitude is the document's stored value and its
    applied magnitude the factor times the mark's applied weight for that load; every other entry's applied magnitude is
    the factor times its authored one."""
    envelope, model = solved("load_state_twin", mode), MODELS["load_state_twin"]
    items = {item["load_id"]: item for item in _mark(envelope)["contents_resolution"]}
    for case_id in (TEST_CASE, ORDINARY):
        stored = {load["id"]: load["magnitude"]["value"] for case in model["load_cases"] if case["id"] == case_id
                  for load in case["primitive_loads"]}
        record = next(r for r in envelope["contract_evidence"]["load_reference_states"] if r["load_case_id"] == case_id)
        entries = [c for c in record["contributions"] if c["owner_kind"] == "stored_primitive"]
        assert entries and {c["source_id"] for c in entries} == set(stored), case_id
        for entry in entries:
            assert entry["authored_normalized_magnitude"] == stored[entry["source_id"]], entry
            base = items[entry["source_id"]]["applied_n_per_m"] if case_id == TEST_CASE else entry["authored_normalized_magnitude"]
            assert entry["applied_magnitude"] == entry["factor"] * base, entry
            assert (entry["applied_magnitude"] != entry["authored_normalized_magnitude"] * entry["factor"]) == (case_id == TEST_CASE), entry


def _joined_to_another_item(envelope: dict, model: dict) -> None:
    """The second item's applied weight moved, with its own entry kept consistent; the first entry takes that weight,
    which only a reader that joins an entry to some item other than its own would admit."""
    items = _mark(envelope)["contents_resolution"]
    items[1]["applied_n_per_m"] = items[1]["applied_n_per_m"] * 1.01
    for index in (0, 1):
        entry = _entry(envelope, TEST_CASE, index)
        assert entry["source_id"] == items[index]["load_id"]
        entry["applied_magnitude"] = entry["factor"] * items[1]["applied_n_per_m"]


def _unmarked_by_the_marked_rule(envelope: dict, model: dict) -> None:
    entry = _entry(envelope, ORDINARY, 0)
    entry["applied_magnitude"] = entry["factor"] * _mark(envelope)["contents_resolution"][0]["applied_n_per_m"]


def _marked_by_the_plain_rule(envelope: dict, model: dict) -> None:
    entry = _entry(envelope, TEST_CASE, 0)
    entry["applied_magnitude"] = entry["factor"] * entry["authored_normalized_magnitude"]


@pytest.mark.parametrize("label, mutate", [
    ("an entry joined to another contents item", _joined_to_another_item),
    ("an unmarked case's entry by the marked rule", _unmarked_by_the_marked_rule),
    ("a marked case's entry by the plain rule", _marked_by_the_plain_rule),
], ids=lambda value: value if isinstance(value, str) else "")
def test_the_record_rule_on_a_marked_case_is_kept_by_both_readers(label: str, mutate) -> None:
    """The record of load sources' factor rule, refused by its own code with and without the model."""
    envelope, model = _forge("load_state_twin", mutate)
    for bound in (None, model):
        assert rust_code(envelope, bound) == "SOURCE_LOAD_REFERENCE_CONTRIBUTION_APPLIED_MAGNITUDE", (label, bound is not None)
        assert python_code(envelope, bound) == "SOURCE_LOAD_REFERENCE_CONTRIBUTION_APPLIED_MAGNITUDE", (label, bound is not None)


def test_the_mark_shape_is_refused_without_the_model() -> None:
    """The shape the result schema declares, checked by the model-free reader: a duplicate held reference, an unknown
    member, an empty resolution, another method."""
    for label, mutate, rust in [
        ("duplicate held reference", lambda m: m["held_support_refs"].append(m["held_support_refs"][0]), "SOURCE_PHYSICS_TEST_CONDITION_HELD_SUPPORT_REFS"),
        ("unknown member", lambda m: m.__setitem__("extra", True), "SOURCE_PHYSICS_TEST_CONDITION_SHAPE"),
        ("empty resolution", lambda m: m.__setitem__("contents_resolution", []), "SOURCE_PHYSICS_TEST_CONDITION_SHAPE"),
        ("another method", lambda m: m["contents_resolution"][0].__setitem__("method", "other"), "SOURCE_PHYSICS_TEST_CONDITION_CONTENTS_METHOD"),
        ("another kind", lambda m: m.__setitem__("kind", "pneumatic"), "SOURCE_PHYSICS_TEST_CONDITION_KIND"),
    ]:
        envelope = solved("journey", "sparse_interactive")
        mutate(_mark(envelope))
        assert rust_code(envelope, None) == rust, label
        assert python_code(envelope, None) == SHAPE, label


@pytest.mark.parametrize("mode", MODES)
@pytest.mark.parametrize("name", sorted(MODELS))
def test_the_qualification_adapter_admits_the_runner_output(name: str, mode: str) -> None:
    """The exact-pressure qualification adapter's transport admission (`unwrap`) of the runner's controlled output,
    whose owned-reader verdict is the runner's `read pressure` above. The controlled output of these documents is
    larger than the adapter's default byte limit, so it is read at the adapter's own maximum."""
    from tools.validation import qualification_exact_pressure as adapter
    raw, envelope = raw_solve(name, mode)
    mechanics, rows = adapter.unwrap(raw, body(name), mode, limit=adapter.MAX_OUTPUT_LIMIT)
    assert mechanics == envelope and rows == envelope["results"]
