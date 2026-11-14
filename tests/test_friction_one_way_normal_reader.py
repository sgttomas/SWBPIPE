"""DEL-04-04 REQ-012: a friction normal derived from a one-way support, read by the test-only Python reader
(``tests/verifiers/nonlinear_support_evidence.py``) and the Rust product reader (``openpipestress-runner read
pressure``), which must take the same verdict.

Each settled case of the frozen reference ``validation/references/friction_one_way_normal/`` is authored as the Rust
binding authors it (``core/product_physics/tests/friction_one_way_normal_source.rs``, ``document``) and solved by the
runner in both modes. Both readers admit every envelope with and without its model, and refuse by the same code:
- a published normal that is not the rule's value (doubled);
- a nonzero normal beside an inactive source (the source's own residual);
- a pulling reaction passed as a normal, which only the model's bearing sense shows (refused with the model only).

Needs ``OPENPIPESTRESS_RUNNER_BIN`` (``tests/conftest.py`` builds it unless ``OPENPIPESTRESS_NO_CARGO_BUILD`` is
set); skipped without it.
"""
from __future__ import annotations

from copy import deepcopy
import json
import os
from pathlib import Path
import subprocess

import pytest

from tests.verifiers import nonlinear_support_evidence as nse
from tests.verifiers import reader_verdict as rv

ROOT = Path(__file__).resolve().parents[1]
REFERENCES = json.loads((ROOT / "validation/references/friction_one_way_normal/references.json").read_text(encoding="utf-8"))
PROVENANCE = "invented_friction_one_way_normal_reference"
MATERIAL = "material:fown"
MODEL_SCHEMA_VERSION = "0.9.0"
MODES = ("sparse_interactive", "dense_scrutiny")
ONE_WAY, FRICTION, ANCHOR = "support:one-way-S", "support:friction-S", "support:anchor-A"
MEMBERS = (("riser", "A", "B"), ("run", "B", "S"))


def ratio(text: str) -> float:
    a, sep, b = text.partition("/")
    return float(a) / float(b) if sep else float(text)


def quantity(value: float, unit: str) -> dict:
    return {"value": value, "unit": unit}


def document(case: dict) -> dict:
    model = REFERENCES["model"]
    section = model["section"]
    nodes = [{"id": f"node:{name}", "position": dict(zip("xyz", map(float, xyz))), "provenance": PROVENANCE}
             for name, xyz in model["nodes"].items()]
    pipes = [{"id": pid, "from": f"node:{a}", "to": f"node:{b}",
              "section": {"outside_diameter": quantity(ratio(section["D"]["exact"]), "m"),
                          "wall_thickness": quantity(ratio(section["t"]["exact"]), "m"),
                          "mill_tolerance": quantity(0.0, "m"), "corrosion_allowance": quantity(0.0, "m")},
              "material": MATERIAL, "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0}, "provenance": PROVENANCE}
             for pid, a, b in MEMBERS]
    supports = [
        {"id": ANCHOR, "node": "node:A", "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROVENANCE},
        {"id": ONE_WAY, "node": "node:S", "family": "nonlinear", "restraints": [],
         "nonlinear": {"behavior": "one_way", "dof": "UZ", "initial_state": "active", "active_when": "positive_reaction"}, "provenance": PROVENANCE},
        {"id": FRICTION, "node": "node:S", "family": "nonlinear", "restraints": [],
         "nonlinear": {"behavior": "friction", "dof": "UX", "initial_state": "sticking",
                       "friction_coefficient": quantity(ratio(model["friction_coefficient"]["exact"]), "1"),
                       "normal_reaction_source": {"support_ref": ONE_WAY, "dof": "UZ"}}, "provenance": PROVENANCE},
    ]
    loads = [{"id": f"load:{k}", "category": "concentrated_force", "target": {"type": "node", "node": f"node:{load['node']}"},
              "direction": {"FX": "global_x", "FZ": "global_z"}[load["component"]],
              "magnitude": quantity(ratio(load["value"]["exact"]), "N"), "dimension": "force", "provenance": PROVENANCE}
             for k, load in enumerate(case["primitive_loads"])]
    return {"model": {"schema_version": MODEL_SCHEMA_VERSION, "document_kind": "openpipestress.product_preview.model",
                      "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"},
                      "project": {"id": "project:friction-one-way-normal",
                                  "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
                      "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing",
                                          "professional_acceptance": "not_provided"},
                      "nodes": nodes, "pipe_segments": pipes,
                      "materials": [{"id": MATERIAL, "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
                                     "elastic_modulus": quantity(ratio(model["material"]["E"]["exact"]), "Pa"),
                                     "poisson_ratio": quantity(ratio(model["material"]["nu"]["exact"]), "1"), "provenance": PROVENANCE}],
                      "supports": supports, "components": [],
                      "load_cases": [{"id": "case:main", "primitive_loads": loads, "pressure_regions": [], "provenance": PROVENANCE}],
                      "combinations": []},
            "materials": []}


def runner() -> str:
    path = os.environ.get("OPENPIPESTRESS_RUNNER_BIN")
    if not path or not Path(path).is_file():
        pytest.skip("OPENPIPESTRESS_RUNNER_BIN is not set")
    return path


def runner_request() -> dict:
    return {
        "request_id": "friction-one-way-normal-reader", "operation": "solve",
        "operation_ref": {"ref_type": "api_operation", "ref_id": "solve"},
        "project_ref": {"ref_type": "project", "ref_id": "project:friction-one-way-normal"},
        "model_ref": {"ref_type": "model", "ref_id": "friction-one-way-normal"},
        "unit_system_ref": {"ref_type": "unit_system", "ref_id": "invented-si"},
        "load_basis_refs": [{"ref_type": "load_case", "ref_id": "case:main"}],
        "input_manifest_ref": {"ref_type": "audit_manifest", "ref_id": "friction-one-way-normal-reader"},
        "requested_outputs": ["result_envelope", "audit_manifest", "diagnostics"],
        "privacy": {"local_only": True, "telemetry_allowed": False, "private_payload_redacted": True, "classification": "public_metadata"},
        "provenance": {"source_name": "invented friction one-way normal references",
                       "source_location": "validation/references/friction_one_way_normal/references.json",
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


def solve(doc: dict, mode: str) -> dict:
    body = {"request": runner_request(), "solve": {"preview_model": doc}}
    completed = subprocess.run([runner(), "solve", "--explicit-local-private-intent", "--solver-mode", mode, "--input", "-"],
                               input=json.dumps(body), capture_output=True, text=True, check=False)
    assert completed.stdout, completed.stderr
    envelope = json.loads(completed.stdout)["payload"]["mechanics_envelope"]
    assert envelope["status"]["mechanics"] == "MECHANICS_SOLVED", envelope["diagnostics"]
    return envelope


def rust_code(envelope: dict, model: dict | None) -> str | None:
    completed = subprocess.run([runner(), "read", "pressure", "--input", "-"],
                               input=json.dumps({"source": envelope, "model": model}), capture_output=True, text=True, check=False)
    outcome = json.loads(completed.stdout)["outcome"]
    return None if outcome["ok"] else outcome["error"]["code"]


def python_code(envelope: dict, model: dict | None) -> str | None:
    outcome = rv.validate("pressure", envelope, model)
    return None if outcome["ok"] else outcome["error"]["code"]


def codes(envelope: dict, model: dict) -> tuple:
    """Both readers' codes, without and with the model; the two readers must agree."""
    python = (python_code(envelope, None), python_code(envelope, model))
    rust = (rust_code(envelope, None), rust_code(envelope, model))
    assert python == rust, (python, rust)
    return python


def case(cid: str) -> dict:
    return next(c for c in REFERENCES["cases"] if c["id"] == cid)


@pytest.fixture(scope="module")
def solved() -> dict:
    out = {}
    for c in REFERENCES["cases"]:
        if c["strict confirmation"]["establishes"]:
            doc = document(c)
            for mode in MODES:
                out[(c["id"], mode)] = (doc, solve(doc, mode))
    assert len(out) == 24
    return out


def test_every_settled_envelope_is_admitted_by_both_readers_with_and_without_its_model(solved):
    for (cid, mode), (doc, envelope) in solved.items():
        assert codes(envelope, doc["model"]) == (None, None), (cid, mode)


def rows(envelope: dict, kind: str, entity: str) -> list:
    found = [r for r in envelope["results"] if r["kind"] == kind and r["entity_ref"] == entity]
    assert found, (kind, entity)
    return found


def test_a_normal_the_rule_does_not_give_is_refused_by_both_readers(solved):
    normal = "nonlinear_support_friction_normal_reaction_derived"
    # A doubled normal.
    doc, envelope = solved[("SLIDE-POS-B", "sparse_interactive")]
    forged = deepcopy(envelope)
    for r in rows(forged, normal, FRICTION):
        r["value"] = 2.0 * r["value"]
    assert codes(forged, doc["model"]) == ("SOURCE_PHYSICS_NONLINEAR_NORMAL",) * 2
    # The inactive source's residual passed as the normal (sparse LIFT-0 publishes a nonzero one).
    doc, envelope = solved[("LIFT-0", "sparse_interactive")]
    forged = deepcopy(envelope)
    residual = [r for r in rows(forged, "support_reaction_component_v2", ONE_WAY) if r["metadata"]["component"] == "Fz"][0]["value"]
    assert residual != 0.0
    for r in rows(forged, normal, FRICTION):
        r["value"] = abs(residual)
    assert codes(forged, doc["model"]) == ("SOURCE_PHYSICS_NONLINEAR_NORMAL",) * 2
    # A pulling reaction passed as a normal: the model's opposite bearing sense.
    doc, envelope = solved[("SLIDE-POS-B", "dense_scrutiny")]
    model = deepcopy(doc["model"])
    model["supports"][1]["nonlinear"]["active_when"] = "negative_reaction"
    assert codes(envelope, model) == (None, "SOURCE_PHYSICS_NONLINEAR_MODEL_NORMAL")


def test_the_bearing_rule_from_both_sides():
    """The Python twin's model rule alone (``_bearing_normal_bound``; Rust ``bearing_normal_bound``)."""
    item = nse._Input(ONE_WAY, "node:S", 2)
    item.behavior, item.sense = "one_way", 1.0

    def published(state: str, reaction: float):
        return [nse._Support(ONE_WAY, 2, state, 0.0, reaction, None, None)]

    bits = nse._bits
    assert nse._bearing_normal_bound([item], published("active", 5.0), ONE_WAY, 2, 5.0)
    assert not nse._bearing_normal_bound([item], published("active", -5.0), ONE_WAY, 2, 5.0)
    assert nse._bearing_normal_bound([item], published("active", -5.0), ONE_WAY, 2, 0.0)
    assert not nse._bearing_normal_bound([item], published("inactive", 3.6e-12), ONE_WAY, 2, 3.6e-12)
    assert nse._bearing_normal_bound([item], published("inactive", 3.6e-12), ONE_WAY, 2, 0.0)
    assert not nse._bearing_normal_bound([item], published("active", 5.0), ONE_WAY, 0, 5.0)
    item.sense = -1.0
    assert nse._bearing_normal_bound([item], published("active", -5.0), ONE_WAY, 2, 5.0)
    # A linear source (not a nonlinear input) is not this rule's: unchanged.
    assert nse._bearing_normal_bound([item], published("active", 5.0), "support:linear", 2, 123.0)
    assert bits(0.0) != bits(-0.0)
