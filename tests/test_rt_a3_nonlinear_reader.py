"""RT-A3 increment 4 (DEL-04-07 Design ``route-unification-r4.md`` §2.4 and §3.5 S5; ``pressure-contract.md``,
"Nonlinear supports on EX (RT-A3 increment 4)"): the test-only Python pressure-2 reader
(``tests/verifiers/pressure_2.py`` with ``nonlinear_support_evidence.py``) and the Rust product reader
(``openpipestress-runner read pressure``) take the same verdict on the exact route's nonlinear support results.

Each case of the frozen references (``validation/references/rt_a3_nonlinear/``, 20 of 21; G1-P0 has no load and is
refused by the product) is authored as the Rust binding authors it (``core/product_physics/tests/
rt_a3_nonlinear_references.rs``, ``document``) and solved by the runner in both modes. Both readers admit all 40
envelopes with and without their model, and refuse the binding's forged envelopes by the same code (its negative
controls, mirrored here), with pinned limits: a contact scaled consistently, magnitude included, is lawful and
self-consistent and is refused only by the references; and a non-Passed inactive reaction is inspection evidence with
no published bound, so a consistently forged magnitude is admitted (the nl3 review's B1).

Needs ``OPENPIPESTRESS_RUNNER_BIN`` (``tests/conftest.py`` builds it unless ``OPENPIPESTRESS_NO_CARGO_BUILD`` is
set); skipped without it.
"""
from __future__ import annotations

from copy import deepcopy
import json
import math
import os
from pathlib import Path
import subprocess

import pytest

from tests.verifiers import nonlinear_support_evidence as nse
from tests.verifiers import reader_verdict as rv

ROOT = Path(__file__).resolve().parents[1]
REFERENCES = json.loads((ROOT / "validation/references/rt_a3_nonlinear/references.json").read_text(encoding="utf-8"))
# The RT-A3 increment 4 review's (nl3) loaded B1 counterexample: untouched sparse producer output and its model.
B1_LOADED = ROOT / "core/product_physics/tests/fixtures/rt_a3_nonlinear/b1_loaded_inactive_reaction.json"
PROVENANCE = "invented_rt_a3_nonlinear_reference"
MATERIAL = "material:rt-a3-nl"
MODEL_SCHEMA_VERSION = "0.9.0"
MODES = ("sparse_interactive", "dense_scrutiny")
INEXPRESSIBLE = ("G1-P0",)
PASSED = "NUMERICAL_INTEGRITY_CHECKS_PASSED"
SENSITIVE = "NUMERICAL_INTEGRITY_SENSITIVE"
UNQUALIFIED = "NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED"


# -- the documents (the Rust binding's `document`) --------------------------------------------------------------------
def ratio(text: str) -> float:
    a, sep, b = text.partition("/")
    return float(a) / float(b) if sep else float(text)


def quantity(value: float, unit: str) -> dict:
    return {"value": value, "unit": unit}


def node_id(name: str) -> str:
    return f"node:{name}"


def document(case: dict) -> dict:
    model, m = REFERENCES["model"], case["model"]
    section = model["section"]
    nodes = [{"id": node_id(name), "position": dict(zip("xyz", map(ratio, xyz))), "provenance": PROVENANCE}
             for name, xyz in m["nodes_m"].items()]
    components, pipes = [], []
    for member in m["members"]:
        if member["kind"] == "straight":
            y_reference = {"x": 0.0, "y": 1.0, "z": 0.0}
        else:
            assert member["kind"] == "realized_arc" and member["angle"] == "pi/2"
            a, b = ([ratio(v) for v in m["nodes_m"][member[end]]] for end in ("from", "to"))
            c = [ratio(v) for v in member["center_m"]]
            toward = [0.5 * (a[k] + b[k]) - c[k] for k in range(3)]
            components.append({"id": f"component:{member['id']}", "kind": "bend", "node": node_id(member["from"]),
                               "geometry": {"bend_pipe_ref": member["id"], "bend_radius": quantity(ratio(member["radius_m"]), "m"),
                                            "bend_angle": quantity(math.pi / 2, "rad"),
                                            "bend_plane_orientation": "plane of the span chord and the pipe y_reference; the arc bows toward +y_reference"},
                               "modifiers": {"flexibility_factor_user_value": quantity(ratio(member["flexibility_factor_k"]), "none")},
                               "mechanics_interface": {"solver_consumption": "curved_bend_macro_element"}, "provenance": PROVENANCE})
            y_reference = dict(zip("xyz", toward))
        pipes.append({"id": member["id"], "from": node_id(member["from"]), "to": node_id(member["to"]),
                      "section": {"outside_diameter": quantity(ratio(section["D_m"]), "m"), "wall_thickness": quantity(ratio(section["t_m"]), "m"),
                                  "mill_tolerance": quantity(ratio(section["c_m_m"]), "m"), "corrosion_allowance": quantity(ratio(section["c_a_m"]), "m")},
                      "material": MATERIAL, "y_reference": y_reference, "provenance": PROVENANCE})
    anchor = m["anchor"]
    supports = [{"id": anchor["id"], "node": node_id(anchor["node"]), "family": "anchor",
                 "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROVENANCE}]
    supports += [{"id": r["id"], "node": node_id(r["node"]), "family": "line_stop", "restraints": [r["dof"]], "provenance": PROVENANCE}
                 for r in m["linear_restraints"]]
    sign = {"+": "positive", "-": "negative"}
    for s in m["nonlinear_supports"]:
        if s["kind"] == "gap":
            nonlinear = {"behavior": "gap", "dof": s["dof"], "initial_state": "inactive",
                         "closes_when": f"{sign[s['closes']]}_displacement", "gap": quantity(ratio(s["clearance_m"]), "m")}
        elif s["kind"] == "one_way":
            nonlinear = {"behavior": "one_way", "dof": s["dof"], "initial_state": "active", "active_when": f"{sign[s['sense']]}_reaction"}
        else:
            assert s["kind"] == "friction"
            nonlinear = {"behavior": "friction", "dof": s["dof"], "initial_state": "sticking", "friction_coefficient": quantity(ratio(s["mu"]), "1")}
            normal = s["normal"]
            if normal["kind"] == "explicit":
                nonlinear["normal_reaction"] = quantity(ratio(normal["value_N"]), "N")
            else:
                source = next(r for r in m["linear_restraints"] if r["node"] == normal["source_node"] and r["dof"] == normal["source_dof"])
                nonlinear["normal_reaction_source"] = {"support_ref": source["id"], "dof": normal["source_dof"]}
        supports.append({"id": s["id"], "node": node_id(s["node"]), "family": "nonlinear", "restraints": [],
                         "nonlinear": nonlinear, "provenance": PROVENANCE})
    lc = case["load_case"]
    p = ratio(lc["p_Pa"])
    if lc["regions"] == "one region over every member":
        r = m["region"]
        regions = [{"id": "region:all", "member_pipe_ids": r["members"], "pressure_basis": "internal_differential_zero_external_v1",
                    "pressure": quantity(p, "Pa"),
                    "terminals": [{"node_ref": node_id(n), "closure_transfer": t, "provenance": PROVENANCE} for n, t in r["terminals"].items()],
                    "provenance": PROVENANCE}]
    else:
        assert p == 0.0
        regions = []
    directions = {"FX": "global_x", "FY": "global_y", "FZ": "global_z"}
    loads = [{"id": f"load:{n}:{c}", "category": "concentrated_force", "target": {"type": "node", "node": node_id(n)},
              "direction": directions[c], "magnitude": quantity(ratio(v), "N"), "dimension": "force", "provenance": PROVENANCE}
             for n, by in lc["nodal_loads"].items() for c, v in by.items()]
    shear = {"euler_bernoulli": "euler_bernoulli", "timoshenko": "timoshenko_annulus_energy_v1"}[m["shear_model"]]
    return {"model": {"schema_version": MODEL_SCHEMA_VERSION, "document_kind": "openpipestress.product_preview.model",
                      "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": shear},
                      "project": {"id": "project:rt-a3-nonlinear", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
                      "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
                      "nodes": nodes, "pipe_segments": pipes,
                      "materials": [{"id": MATERIAL, "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
                                     "elastic_modulus": quantity(ratio(model["material"]["E_Pa"]), "Pa"), "poisson_ratio": quantity(0.3, "1"), "provenance": PROVENANCE}],
                      "supports": supports, "components": components,
                      "load_cases": [{"id": "case:main", "primitive_loads": loads, "pressure_regions": regions, "provenance": PROVENANCE}],
                      "combinations": []},
            "materials": []}


# -- the runner ------------------------------------------------------------------------------------------------------
def runner() -> str:
    path = os.environ.get("OPENPIPESTRESS_RUNNER_BIN")
    if not path or not Path(path).is_file():
        pytest.skip("OPENPIPESTRESS_RUNNER_BIN is not set")
    return path


def runner_request() -> dict:
    return {
        "request_id": "rt-a3-nonlinear-reader", "operation": "solve",
        "operation_ref": {"ref_type": "api_operation", "ref_id": "solve"},
        "project_ref": {"ref_type": "project", "ref_id": "project:rt-a3-nonlinear"},
        "model_ref": {"ref_type": "model", "ref_id": "rt-a3-nonlinear"},
        "unit_system_ref": {"ref_type": "unit_system", "ref_id": "invented-si"},
        "load_basis_refs": [{"ref_type": "load_case", "ref_id": "case:main"}],
        "input_manifest_ref": {"ref_type": "audit_manifest", "ref_id": "rt-a3-nonlinear-reader"},
        "requested_outputs": ["result_envelope", "audit_manifest", "diagnostics"],
        "privacy": {"local_only": True, "telemetry_allowed": False, "private_payload_redacted": True, "classification": "public_metadata"},
        "provenance": {"source_name": "invented RT-A3 nonlinear references", "source_location": "validation/references/rt_a3_nonlinear/references.json",
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
    assert envelope["formulation_basis"]["profile_id"] == "exact_pressure_v3"
    return envelope


def rust_code(envelope: dict, model: dict | None) -> str | None:
    completed = subprocess.run([runner(), "read", "pressure", "--input", "-"],
                               input=json.dumps({"source": envelope, "model": model}), capture_output=True, text=True, check=False)
    outcome = json.loads(completed.stdout)["outcome"]
    return None if outcome["ok"] else outcome["error"]["code"]


def python_code(envelope: dict, model: dict | None) -> str | None:
    outcome = rv.validate("pressure", envelope, model)
    return None if outcome["ok"] else outcome["error"]["code"]


def codes(envelope: dict, doc: dict) -> dict:
    """Both readers' codes, without and with the model; the two readers must agree."""
    python = (python_code(envelope, None), python_code(envelope, doc["model"]))
    rust = (rust_code(envelope, None), rust_code(envelope, doc["model"]))
    assert python == rust, (python, rust)
    return python


@pytest.fixture(scope="module")
def solved() -> dict:
    out = {}
    for case in REFERENCES["cases"]:
        if case["id"] in INEXPRESSIBLE:
            continue
        doc = document(case)
        for mode in MODES:
            out[(case["id"], mode)] = (doc, solve(doc, mode))
    assert len(out) == 2 * (21 - len(INEXPRESSIBLE))
    return out


def test_every_reference_envelope_is_admitted_by_both_readers_with_and_without_its_model(solved):
    for (cid, mode), (doc, envelope) in solved.items():
        assert codes(envelope, doc) == (None, None), (cid, mode)


# -- the forgeries (the Rust binding's negative controls) -------------------------------------------------------------
def set_rows(envelope: dict, kind: str, entity: str, edit) -> None:
    hit = 0
    for row in envelope["results"]:
        if row["kind"] == kind and row["entity_ref"] == entity:
            edit(row)
            hit += 1
    assert hit, (kind, entity)


def scale_support(envelope: dict, support: str, component: str, factor: float, magnitude: bool) -> None:
    kinds = ["nonlinear_support_final_reaction", "support_reaction_component_v2"] + (["support_reaction_force_magnitude_v2"] if magnitude else [])
    for kind in kinds:
        def edit(row):
            if row["kind"] != "support_reaction_component_v2" or row["metadata"]["component"] == component:
                row["value"] = factor * row["value"]
        set_rows(envelope, kind, support, edit)


def restate(envelope: dict, support: str, old: str, new: str, code: float, review: bool) -> None:
    for kind in ("nonlinear_support_active_set_state_code", "nonlinear_support_final_reaction", "nonlinear_support_final_displacement"):
        def edit(row):
            if row["kind"] == "nonlinear_support_active_set_state_code":
                row["value"] = code
            row["metadata"]["basis"] = row["metadata"]["basis"].replace(f"final_state={old}", f"final_state={new}")
        set_rows(envelope, kind, support, edit)
    if review:
        for d in envelope["diagnostics"]:
            if d["code"] == "NONLINEAR_SUPPORT_STATE_REVIEW" and d["affected_refs"][0] == support:
                d["message"] = d["message"].replace(f" in {old} state", f" in {new} state")


def relabel_passed(envelope: dict, old: str, quality: bool, line: bool) -> None:
    for d in envelope["diagnostics"]:
        if d["code"] == old:
            d["code"], d["severity"] = PASSED, "info"
            if line:
                d["message"] += " Converged support state: established at 128-bit precision by one formation-checked solve; (forged)"
    if quality:
        q = envelope["numerical_quality"]
        q["status"] = "checks_passed"
        for case in q["cases"]:
            case.update(solve_quality="checks_passed", structural_status="passive_model_basis",
                        model_matrix_fidelity="represented_equations_retained", accuracy_evidence="not_claimed")


def both(code: str) -> tuple:
    return (f"SOURCE_PHYSICS_NONLINEAR_{code}",) * 2


def model_only(code: str) -> tuple:
    return (None, f"SOURCE_PHYSICS_NONLINEAR_{code}")


def forged(solved, cid: str, edit) -> tuple:
    doc, envelope = solved[(cid, "dense_scrutiny")]
    envelope = deepcopy(envelope)
    edit(envelope)
    return codes(envelope, doc)


def test_forged_contacts_states_friction_and_standings_are_refused_by_both_readers(solved):
    # 1. G1's contact halved (reaction and slot), and with its magnitude too (the pinned limit).
    assert forged(solved, "G1", lambda e: scale_support(e, "support:GAP", "Fx", 0.5, False)) == both("MAGNITUDE")
    assert forged(solved, "G1", lambda e: scale_support(e, "support:GAP", "Fx", 0.5, True)) == (None, None)
    # 2. G1's closed gap stated inactive; with its state diagnostic too; G2's open gap stated active.
    assert forged(solved, "G1", lambda e: restate(e, "support:GAP", "active", "inactive", 0.0, False)) == both("STATE_REVIEW")
    # Limit: with its state diagnostic too, a non-Passed inactive force is inspection evidence with no published bound.
    assert forged(solved, "G1", lambda e: restate(e, "support:GAP", "active", "inactive", 0.0, True)) == (None, None)
    assert forged(solved, "G2", lambda e: restate(e, "support:GAP", "inactive", "active", 1.0, True)) == model_only("MODEL_COMPLEMENTARITY")
    # 3. F1-SLIDE's friction at 1.1 muN; with its magnitude; turned along the slip; its derived normal doubled.
    assert forged(solved, "F1-SLIDE", lambda e: scale_support(e, "support:FRICTION", "Fz", 1.1, False)) == both("MAGNITUDE")
    assert forged(solved, "F1-SLIDE", lambda e: scale_support(e, "support:FRICTION", "Fz", 1.1, True)) == model_only("MODEL_FRICTION_CAPACITY")
    assert forged(solved, "F1-SLIDE", lambda e: scale_support(e, "support:FRICTION", "Fz", -1.0, False)) == both("SLIP")

    def double_normal(e):
        set_rows(e, "nonlinear_support_friction_normal_reaction_derived", "support:FRICTION", lambda r: r.update(value=2.0 * r["value"]))
    assert forged(solved, "F1-SLIDE", double_normal) == both("NORMAL")
    # 4. G1 relabelled Passed; with its quality record and the evidence line.
    assert forged(solved, "G1", lambda e: relabel_passed(e, UNQUALIFIED, False, False)) == both("STANDING")
    assert forged(solved, "G1", lambda e: relabel_passed(e, UNQUALIFIED, True, True)) == both("GAP_STANDING")
    # 5. F1-P0-SLIDELOAD relabelled Passed; with its quality record and the evidence line.
    assert forged(solved, "F1-P0-SLIDELOAD", lambda e: relabel_passed(e, SENSITIVE, False, False)) == both("STANDING")
    assert forged(solved, "F1-P0-SLIDELOAD", lambda e: relabel_passed(e, SENSITIVE, True, True)) == both("FRICTION_NORMAL_STANDING")


# -- the review's blocking reader findings (the Rust binding's `reader_binds_inactive_residuals_support_nodes_and_standings`)
def inclined_gap_document() -> dict:
    """The review's inclined-gap document (B1): G2 with its tip at (1000, 1000/3, 1000/7) m, its +UX gap open by
    0.001 m, no pressure region and one tip global-Z load of 1e-6 N."""
    doc = document(next(c for c in REFERENCES["cases"] if c["id"] == "G2"))
    model = doc["model"]
    tip = model["pipe_segments"][0]["to"]
    for node in model["nodes"]:
        if node["id"] == tip:
            node["position"] = {"x": 1000.0, "y": 1000.0 / 3.0, "z": 1000.0 / 7.0}
    for support in model["supports"]:
        if support.get("nonlinear") is not None:
            support["nonlinear"]["gap"] = quantity(0.001, "m")
    case = model["load_cases"][0]
    case["pressure_regions"] = []
    case["primitive_loads"] = [{"id": "load:tip", "category": "concentrated_force", "target": {"type": "node", "node": tip},
                                "direction": "global_z", "magnitude": quantity(1e-6, "N"), "dimension": "force", "provenance": PROVENANCE}]
    return doc


def one_value(envelope: dict, kind: str, entity: str) -> float:
    found = [r for r in envelope["results"] if r["kind"] == kind and r["entity_ref"] == entity]
    assert len(found) == 1, (kind, entity)
    return found[0]["value"]


def set_code(envelope: dict, old: str, new: str) -> None:
    hit = [d for d in envelope["diagnostics"] if d["code"] == old]
    assert len(hit) == 1, old
    hit[0]["code"] = new


def set_quality(envelope: dict, tokens: tuple) -> None:
    for case in envelope["numerical_quality"]["cases"]:
        case.update(zip(("structural_status", "solve_quality", "model_matrix_fidelity", "accuracy_evidence"), tokens))


def test_inactive_residuals_support_nodes_and_standings_are_bound_by_both_readers(solved):
    # B1. The untouched producer output, both modes: admitted as inspection evidence beside the producer's own compliant
    # free-row evaluation record (dense: far above 1e-9 of the largest support component).
    doc = inclined_gap_document()
    dense = None
    for mode in MODES:
        envelope = solve(doc, mode)
        reaction = one_value(envelope, "nonlinear_support_final_reaction", "support:GAP")
        assert codes(envelope, doc) == (None, None), mode
        if mode == "dense_scrutiny":
            largest = max(abs(r["value"]) for r in envelope["results"]
                          if r["kind"] == "support_reaction_component_v2" and r["metadata"]["component"] in ("Fx", "Fy", "Fz"))
            assert abs(reaction) > 1e-9 * largest, (reaction, largest)
            dense = envelope

    def inclined(edit) -> tuple:
        envelope = deepcopy(dense)
        edit(envelope)
        return codes(envelope, doc)

    # The nl3 review's loaded counterexample, untouched: |R| above the compliant record (different evaluations, no
    # published denominator), admitted with and without its model; and its model solved now, sparse.
    loaded = json.loads(B1_LOADED.read_text(encoding="utf-8"))
    source, loaded_doc = loaded["source"], {"model": loaded["model"], "materials": []}
    reaction = one_value(source, "nonlinear_support_final_reaction", "support:GAP")
    record = one_value(source, "nonlinear_support_observed_free_dof_force_residual", "nonlinear_supports")
    assert (reaction, record) == (-3.2445904686942417e-13, 2.0991282984831466e-13) and abs(reaction) > record
    assert codes(source, loaded_doc) == (None, None)
    assert codes(solve(loaded_doc, "sparse_interactive"), loaded_doc) == (None, None)

    # Forged-force controls under the revised rule. A non-numeric inactive reaction and slot: refused by the outer
    # number rules on both readers (their outer codes differ, outside the parity claim), and inside the module by
    # MAGNITUDE before the finiteness backstop.
    not_a_number = deepcopy(dense)
    set_rows(not_a_number, "nonlinear_support_final_reaction", "support:GAP", lambda r: r.update(value=None))
    set_rows(not_a_number, "support_reaction_component_v2", "support:GAP",
             lambda r: r.update(value=None) if r["metadata"]["component"] == "Fx" else None)
    assert rust_code(not_a_number, None) == rust_code(not_a_number, doc["model"]) == "SOURCE_PHYSICS_NUMBER_INVALID"
    assert python_code(not_a_number, None) == python_code(not_a_number, doc["model"]) == "SOURCE_PHYSICS_EVIDENCE_INVALID"
    with pytest.raises(ValueError, match="SOURCE_PHYSICS_NONLINEAR_MAGNITUDE"):
        nse.check(not_a_number)
    # Limit: the inactive force doubled with slot and magnitude.
    assert inclined(lambda e: scale_support(e, "support:GAP", "Fx", 2.0, True)) == (None, None)
    assert inclined(lambda e: set_rows(e, "nonlinear_support_final_reaction", "support:GAP",
                                       lambda r: r.update(unit="N*m"))) == both("UNIT")
    # The record in the wrong family: refused by the outer validators before this module (Rust
    # SOURCE_UNIT_CONTRADICTION; outer codes are outside the parity claim).
    wrong_family = deepcopy(dense)
    set_rows(wrong_family, "nonlinear_support_observed_free_dof_force_residual", "nonlinear_supports",
             lambda r: r.update(unit="N*m"))
    assert rust_code(wrong_family, None) == rust_code(wrong_family, doc["model"]) == "SOURCE_UNIT_CONTRADICTION"
    assert python_code(wrong_family, None) is not None and python_code(wrong_family, doc["model"]) is not None
    assert inclined(lambda e: set_rows(e, "nonlinear_support_observed_free_dof_force_residual", "nonlinear_supports",
                                       lambda r: r.update(value=-1.0))) == both("RESIDUAL")
    def duplicate_record(e):
        row = deepcopy(next(r for r in e["results"] if r["kind"] == "nonlinear_support_observed_free_dof_force_residual"))
        row["id"] += "-duplicate"
        e["results"].append(row)
    assert inclined(duplicate_record) == both("RESIDUAL")

    def non_compliant(e):
        set_rows(e, "nonlinear_support_observed_free_dof_force_residual", "nonlinear_supports",
                 lambda r: r["metadata"].update(basis=r["metadata"]["basis"].replace("observed_compliance=true", "observed_compliance=false")))
    assert inclined(non_compliant) == both("RESIDUAL")
    assert inclined(lambda e: e.update(results=[r for r in e["results"]
                                                if r["kind"] != "nonlinear_support_observed_free_dof_force_residual"])) == both("RESIDUAL")

    def minus_zero(e):
        set_rows(e, "nonlinear_support_final_reaction", "support:REST", lambda r: r.update(value=-0.0))
        set_rows(e, "support_reaction_component_v2", "support:REST",
                 lambda r: r.update(value=-0.0) if r["metadata"]["component"] == "Fz" else None)
    assert forged(solved, "O1-LIFT", minus_zero) == both("PASSED_ZERO")

    # B2. The deciding displacement is the support node's published row; no ground moves on S5.
    g2_doc, _ = solved[("G2", "dense_scrutiny")]
    clearance = next(s for s in g2_doc["model"]["supports"] if s["id"] == "support:GAP")["nonlinear"]["gap"]["value"] * 1000.0

    def false_contact(e):
        restate(e, "support:GAP", "inactive", "active", 1.0, True)
        set_rows(e, "nonlinear_support_final_displacement", "support:GAP", lambda r: r.update(value=clearance))
    assert forged(solved, "G2", false_contact) == both("DISPLACEMENT")

    def invented_ground(e):
        restate(e, "support:GAP", "inactive", "active", 1.0, True)
        row = deepcopy(next(r for r in e["results"] if r["kind"] == "nonlinear_support_final_displacement" and r["entity_ref"] == "support:GAP"))
        row.update(id=row["id"] + "-relative", kind="nonlinear_support_final_relative_displacement", value=clearance)
        e["results"].append(row)
    assert forged(solved, "G2", invented_ground) == both("RELATIVE_DISPLACEMENT")
    assert forged(solved, "G2", lambda e: set_rows(e, "nonlinear_support_final_displacement", "support:GAP",
                                                    lambda r: r.update(value=0.0))) == model_only("MODEL_DISPLACEMENT")
    assert forged(solved, "F1-SLIDE", lambda e: set_rows(e, "nonlinear_support_final_displacement", "support:FRICTION",
                                                          lambda r: r.update(value=1000.0 * r["value"]))) == both("DISPLACEMENT")

    # B3. The whole code-to-quality assignment, unknown codes and gap precedence.
    assert forged(solved, "F1-P0-SLIDELOAD", lambda e: set_code(e, SENSITIVE, UNQUALIFIED)) == both("STANDING")
    assert forged(solved, "F1-P0-SLIDELOAD", lambda e: set_code(e, SENSITIVE, "NUMERICAL_INTEGRITY_FAILED")) == both("STANDING")
    assert forged(solved, "F1-P0-SLIDELOAD", lambda e: set_quality(
        e, ("passive_model_basis", "sensitive", "represented_equations_retained", "unresolved"))) == both("STANDING")
    assert forged(solved, "F1-P0-SLIDELOAD", lambda e: e["numerical_quality"]["cases"][0].update(evidence_refs=[])) == both("STANDING")
    assert forged(solved, "F1-P0-SLIDELOAD",
                  lambda e: set_code(e, SENSITIVE, "NUMERICAL_INTEGRITY_SENSITIVE_ENOUGH")) == both("STANDING_CODE")
    assert forged(solved, "G1", lambda e: set_code(e, UNQUALIFIED, SENSITIVE)) == both("STANDING")

    def gap_sensitive(e):
        set_code(e, UNQUALIFIED, SENSITIVE)
        set_quality(e, ("passive_model_basis", "sensitive", "represented_equations_retained", "not_claimed"))
    assert forged(solved, "G1", gap_sensitive) == both("GAP_STANDING")

    def unresolved(e):
        set_code(e, SENSITIVE, "NUMERICAL_INTEGRITY_UNRESOLVED")
        set_quality(e, ("numerically_unresolved", "unresolved", "assembly_uncertainty", "unresolved"))
    # Limit (kd5 §2.4, preserved earlier reasons): a consistent non-Passed relabel of a non-gap case.
    assert forged(solved, "F1-P0-SLIDELOAD", unresolved) == (None, None)


# -- combinations: #98's EX gate and decision OQ-1, consumed unchanged ----------------------------------------------
GATE_CODE = "NONLINEAR_COMBINATION_REQUIRES_SOLVE"
COMBINATION_GATE = "SOURCE_PHYSICS_COMBINATION_GATE"
OQ1_SIGN = ("positive value follows explicit user linear combination of matching source result sign conventions; "
            "a solved state plus derived differences or bounds; not an equilibrium state")


def two_case_document(first: str, second: str, combinations) -> dict:
    """The Rust binding's ``two_case_document``: ``first``'s model with ``first``'s and ``second``'s load cases."""
    cases = {c["id"]: c for c in REFERENCES["cases"]}
    doc, other = document(cases[first]), document(cases[second])
    pair = []
    for source, name in ((doc, first), (other, second)):
        case = deepcopy(source["model"]["load_cases"][0])
        case["id"] = f"case:{name}"
        for load in case["primitive_loads"]:
            load["id"] = f"{load['id']}:{name}"
        pair.append(case)
    doc["model"]["load_cases"] = pair
    doc["model"]["combinations"] = combinations(f"case:{first}", f"case:{second}")
    return doc


def comb(cid: str, basis: str, *terms) -> dict:
    return {"id": cid, "basis": basis, "provenance": PROVENANCE,
            "terms": [{"load_case" if kind == "case" else "combination": operand, "factor": factor} for kind, operand, factor in terms]}


def gate(envelope: dict, cid: str) -> dict:
    return next(r for r in envelope["contract_evidence"]["exact_combinations"] if r["combination_id"] == cid)["gate"]


def combination_rows(envelope: dict, cid: str) -> list:
    return [r for r in envelope["results"] if r.get("basis_ref") == {"ref_type": "combination", "ref_id": cid}
            and r["kind"] != "combination_modulus_basis_record"]


def test_flat_mechanics_combinations_are_withheld_and_oq1_is_admitted_by_both_readers():
    """EX's ``NONLINEAR_COMBINATION_REQUIRES_SOLVE`` (#98) on an admitted nonlinear model: every flat mechanics
    combination (the singleton included) is withheld; decision OQ-1's one solved case at factor exactly 1 plus
    κ-zero combinations is admitted and nothing broader. Both readers admit the envelope with and without its model
    and refuse the same forged gates by the same code."""
    def combinations(s: str, o: str) -> list:
        return [
            {"id": "D", "basis": "result_state_subtraction", "minuend_id": o, "subtrahend_id": s, "provenance": PROVENANCE},
            comb("E", "srss", ("comb", "D", 1.0)),
            comb("occ", "mechanics", ("case", s, 1.0), ("comb", "E", 1.0)),
            comb("occ-minus", "mechanics", ("comb", "E", -1.0), ("case", s, 1.0), ("comb", "D", 0.5)),
            comb("factor", "mechanics", ("case", s, 1.5), ("comb", "E", 1.0)),
            comb("two-cases", "mechanics", ("case", s, 1.0), ("case", o, 1.0), ("comb", "E", 1.0)),
            {"id": "R", "basis": "range_envelope", "operand_ids": [s, o], "mode": "max_abs", "provenance": PROVENANCE},
            comb("kappa-one", "mechanics", ("case", s, 1.0), ("comb", "R", 1.0)),
            comb("srss-case", "srss", ("case", o, 1.0)),
            comb("undefined", "mechanics", ("case", s, 1.0), ("comb", "srss-case", 1.0)),
            comb("singleton", "mechanics", ("case", s, 1.0)),
            comb("flat", "mechanics", ("case", s, 1.0), ("case", o, 1.0)),
            comb("half", "mechanics", ("case", s, 0.5), ("case", o, 0.5)),
            comb("abs", "absolute_sum", ("comb", "D", 1.0), ("case", o, 1.0)),
        ]
    doc = two_case_document("O1-LIFT", "O1-CONTACT", combinations)
    admitted, withheld = {"withheld": False, "reason": None}, {"withheld": True, "reason": GATE_CODE}
    for mode in MODES:
        envelope = solve(doc, mode)
        for cid in ("D", "E", "occ", "occ-minus", "R", "srss-case", "abs"):
            assert gate(envelope, cid) == admitted, (mode, cid)
        for cid in ("factor", "two-cases", "kappa-one", "undefined", "singleton", "flat", "half"):
            assert gate(envelope, cid) == withheld, (mode, cid)
            assert combination_rows(envelope, cid) == [], (mode, cid)
        for cid in ("occ", "occ-minus"):
            rows = combination_rows(envelope, cid)
            assert rows and all(r["metadata"]["sign_convention"] == OQ1_SIGN for r in rows), (mode, cid)
        assert any(d["code"] == GATE_CODE for d in envelope["diagnostics"]), mode
        assert codes(envelope, doc) == (None, None), mode
        for cid, reason in (("occ", GATE_CODE), ("singleton", None), ("flat", None), ("half", None), ("factor", None),
                            ("flat", "COMBINATION_INSTALLED_REFERENCE_REQUIRES_SOLVE"),
                            ("flat", "CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE")):
            tampered = deepcopy(envelope)
            next(r for r in tampered["contract_evidence"]["exact_combinations"] if r["combination_id"] == cid)["gate"] = \
                {"withheld": reason is not None, "reason": reason}
            assert codes(tampered, doc) == (COMBINATION_GATE, COMBINATION_GATE), (mode, cid, reason)


def test_every_reference_envelope_validates_against_the_pressure_2_result_schema(solved):
    """No new reason code or row kind is published: the nonlinear support kinds, their diagnostics, the gap standing and
    the combination gate are already in ``results_pressure_2.v0.3.schema.json``, and every reference envelope (and the
    combination document's, gate-withheld) validates."""
    from tests.test_rt_a3_constant_effort_schema import _pressure_schema, _validate
    schema = _pressure_schema()
    for (cid, mode), (_, envelope) in solved.items():
        _validate(schema, envelope)
    doc = two_case_document("O1-LIFT", "O1-CONTACT", lambda s, o: [comb("flat", "mechanics", ("case", s, 1.0), ("case", o, 1.0))])
    envelope = solve(doc, "sparse_interactive")
    assert gate(envelope, "flat") == {"withheld": True, "reason": GATE_CODE}
    _validate(schema, envelope)
