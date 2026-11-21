#!/usr/bin/env python3
"""The reference-system fixture, tranche 1 (DEL-09-01, exit criterion 1).

Checks that the committed files are the generator's output, that the model is a current model document and a desktop
document (its data boundary stated, no authored diagnostic), that it solves on EX with every tranche-1 case solved and
nothing refused, that the generator's ledger and the authoring file's pending flags agree, that no key result carries an
expected value without a recorded basis, EQ-1/2/3/5 in both solver modes, the member states RS-OPE and RS-OPE-REF
publish, RS-CE1's applied load and support action (KR-03's identity), and KR-05's presence (the tee's rows, no values).
Invented inputs only."""

from __future__ import annotations

from fractions import Fraction
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
import reference_packages  # noqa: E402

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
    assert GENERATOR.main(["--check", "--pressure-elongation", "all"]) == 0, capsys.readouterr().out
    assert GENERATOR.generate() == GENERATOR.generate()


def test_the_check_reports_a_changed_file(tmp_path, monkeypatch, capsys):
    for name in (GENERATOR.MODEL_FILE, GENERATOR.AUTHORING_FILE, GENERATOR.KEY_RESULTS_FILE, GENERATOR.CAPTURE_FILE):
        (tmp_path / name).write_bytes((FIXTURE / name).read_bytes())
    monkeypatch.setattr(GENERATOR, "HERE", tmp_path)
    assert GENERATOR.main(["--check", "--pressure-elongation", "all"]) == 0
    (tmp_path / GENERATOR.KEY_RESULTS_FILE).write_text("{}\n", encoding="utf-8")
    assert GENERATOR.main(["--check", "--pressure-elongation", "all"]) == 1
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


def test_the_model_is_a_desktop_document_with_its_boundary_and_no_authored_diagnostic():
    """Spec §2: the fixture opens in the desktop as it is; the solver's diagnostics stay solve output."""
    desktop = {"$schema": "https://json-schema.org/draft/2020-12/schema",
               "$ref": "https://openpipestress.org/schemas/model.schema.yaml#/$defs/DesktopDocument"}
    assert MODEL["data_boundary"] == GENERATOR.DATA_BOUNDARY
    assert set(MODEL["data_boundary"]) == {"public_examples_policy", "protected_source_policy", "private_data_policy",
                                           "professional_boundary"}
    assert MODEL["diagnostics"] == []
    try:
        validate_instance(desktop, MODEL, schema_label="DesktopDocument", instance_label=GENERATOR.MODEL_FILE)
    except JsonSchemaDependencyMissing as exc:
        _skip_or_note_missing_jsonschema(exc)


def test_tranche_one_holds_its_families_and_leaves_the_pending_ones_out():
    ids = all_ids(MODEL)
    assert len(MODEL["nodes"]) == 27 and len(MODEL["pipe_segments"]) == 26
    for present in ("component:RS-B1", "component:RS-B2", "component:RS-B3", "component:RS-T1", "component:RS-F1",
                    "component:RS-V1", "component:RS-F2", "component:RS-R1", "component:RS-EJ1", "support:RS-CE1",
                    "support:RS-FR1V", "load:RS-SUS", "load:RS-OPE", "load:RS-SAM", "load:RS-OPE-REF", "combination:RS-EXP",
                    "combination:RS-dSAM"):
        assert present in ids, present
    for row in AUTHORING["pending"]:
        if not row["in_tranche_1"] and row["family"] != "RS-FR1 with RS-FR1V: lift-off friction shoe pending RT-A3 S7":
            assert not set(row["entities"]) & ids, row["family"]
    # RS-P180 carries no load: no primitive targets it and no case weighs it.
    for case in MODEL["load_cases"]:
        assert all(load["target"].get("pipe") != "pipe:RS-P180" for load in case["primitive_loads"])
        assert all("pipe:RS-P180" not in region["member_pipe_ids"] for region in case["pressure_regions"])


def test_authoring_marks_person_only_for_the_bend_k_the_bend_and_tee_sifs_and_their_source():
    person = {(entry["id"], path) for entry in AUTHORING["inputs"] for path, mark in entry["fields"].items()
              if mark == "person"}
    expected = {(f"component:{tag}", f"modifiers.{field}") for tag in ("RS-B1", "RS-B2", "RS-B3")
                for field in ("flexibility_factor_user_value", "sif_in_plane_user_value", "sif_out_of_plane_user_value",
                              "source_reference")}
    expected |= {("component:RS-T1", f"modifiers.{field}") for field in ("branch_header_sif_user_value",
                                                                         "branch_branch_sif_user_value",
                                                                         "source_reference")}
    assert person == expected
    assert {mark for entry in AUTHORING["inputs"] for mark in entry["fields"].values()} == {"person",
                                                                                             "agent_authorable"}
    assert AUTHORING["pending_inputs"] == []
    tee = next(c for c in MODEL["components"] if c["id"] == "component:RS-T1")
    assert (tee["kind"], tee["node"]) == ("tee", "node:RS-130")
    assert (tee["geometry"]["branch_header_pipe_ref"], tee["geometry"]["branch_branch_pipe_ref"]) == \
        ("pipe:RS-P120", "pipe:RS-P210")
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
    # RG-1, RG-2 and RG-3 meet at RS-130 with every terminal transferring to the wall, at one pressure, and every pipe
    # there is in a region: the junction is no closure (RT-A3 increment 1, D-1).
    assert not codes & {"EXACT_PRESSURE_JUNCTION_PRESSURES_DIFFER", "EXACT_PRESSURE_JUNCTION_PIPE_OUTSIDE_REGIONS"}
    cases = [case["id"] for case in MODEL["load_cases"]]
    assert cases == ["load:RS-SUS", "load:RS-OPE", "load:RS-SAM", "load:RS-OPE-REF"]
    quality = {entry["basis_ref"]["ref_id"]: entry for entry in envelope["numerical_quality"]["cases"]}
    assert set(cases) <= set(quality)
    bases = {(row["basis_ref"]["ref_type"], row["basis_ref"]["ref_id"]) for row in envelope["results"]
             if row.get("basis_ref") and row.get("kind") == "support_reaction_component_v2"}
    assert {("load_case", case) for case in cases} | {("combination", c["id"]) for c in MODEL["combinations"]} <= bases
    states = {state["load_case_id"] for state in envelope["contract_evidence"]["load_reference_states"]}
    assert states == set(cases)


# -- (d) the §10 ledger and the pending flags agree --------------------------------------------------------------------
def test_the_generator_ledger_and_the_authoring_pending_flags_agree():
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
            assert record.startswith(REFERENCE_PACKAGES) and (ROOT / record).is_file(), entry["id"]


# -- (f) EQ-1, EQ-2, EQ-3 and EQ-5 in both solver modes ---------------------------------------------------------------
# Spec §9.2; the frozen references and the conventions are in validation/references/reference_system/ (README). The
# criterion is DEC-024/DEC-026's, as validation/references/w16_imposed_motion/README.md states it: a nonzero row passes
# when |obs - exp| <= 1e-9 |exp|; a zero row when |obs| <= 1e-9 * zero_scale, the largest nonzero magnitude of its
# family in the case. Product sign convention for support_reaction_component_v2: the action of the support on the
# pipe, global axes, moments about the support's node.
# The fixture holds RS-R1 and RS-CE1: EQ-1 and EQ-2 are reference_system_r1_ce's, bound to RS-OPE-REF by
# reference_system_ope_ref, which also states EQ-3 on RS-OPE-REF - RS-SUS and the member states of RS-OPE and
# RS-OPE-REF. The tranche-1 package reference_system stays frozen as the earlier fixture's; its README states EQ-5 and
# the row conventions.
REFERENCE_PACKAGES = ("validation/references/reference_system/", "validation/references/reference_system_r1_ce/",
                      "validation/references/reference_system_ope_ref/")
REFERENCES = ROOT / "validation" / "references" / "reference_system_r1_ce" / "references.json"
OPE_REF_REFERENCES = ROOT / "validation" / "references" / "reference_system_ope_ref" / "references.json"
EQ_CASES = ("load:RS-SUS", "load:RS-OPE", "load:RS-SAM", "load:RS-OPE-REF")
SOLVER_MODES = ("sparse_interactive", "dense_scrutiny")
RELATIVE = 1e-9
FORCES, MOMENTS = ("Fx", "Fy", "Fz"), ("Mx", "My", "Mz")
SUPPORT_COMPONENT = "support_reaction_component_v2"
# Magnitude rows: recomputed from the combined components (kind -> component kinds or support components).
MAGNITUDES = {"displacement_magnitude": ("global_nodal_displacement_x", "global_nodal_displacement_y",
                                         "global_nodal_displacement_z"),
              "support_reaction_force_magnitude_v2": FORCES, "support_reaction_moment_magnitude_v2": MOMENTS}
# Case rows that are not signed component rows: maxima, extremes, intensified measures and solver records. The product
# combines none of them (COMBINATION_STRESS_MAXIMUM_UNAVAILABLE, COMBINATION_INTENSIFIED_STRESS_UNAVAILABLE).
NOT_COMPONENT_ROWS = {"branch_intensified_bending_stress_v1", "curved_bend_directional_intensified_bending_maximum_v1",
                      "curved_bend_directional_intensified_bending_stress_v1", "element_local_normal_stress_extreme",
                      "element_local_transverse_shear_stress_maximum", "pipe_elastic_normal_stress_maximum_v2",
                      "linear_solver_mode_basis"}
# Rows the combination withholds by name (a tie-rod row's key is shared by the connector's rods).
WITHHELD_KINDS = {"connector_tie_rod_force_v1"}
COMBINATION_RECORDS = {"combination_modulus_basis_record"}


def reference_values(case="load:RS-SUS"):
    """W_total and the weight's moment about the origin for `case`: the r1_ce package's for the three cases it states,
    reference_system_ope_ref's binding of the same record for RS-OPE-REF."""
    refs = json.loads(REFERENCES.read_text(encoding="utf-8"))
    if case == "load:RS-OPE-REF":
        bound = json.loads(OPE_REF_REFERENCES.read_text(encoding="utf-8"))["eq1_eq2"]
        assert case in bound["cases"] and bound["bound_to"] == REFERENCES.relative_to(ROOT).as_posix()
        key = bound["key_values"]
    else:
        assert case in refs["cases"]
        key = refs["key_values"]
    return key["W_total_N"]["value"], [v["value"] for v in key["weight_moment_about_origin_N_m"]]


def row_key(row):
    """The structural pairing key from a row's published fields: never its id."""
    meta = row.get("metadata") or {}
    section = meta.get("basis", "").partition("section_basis=")[2]
    return (row["kind"], row["unit"], row["entity_ref"], meta.get("location", ""), meta.get("component", ""),
            meta.get("coordinate_system", ""), section)


def summarise(output):
    """The rows the checks read, by basis (load case or combination id), and the withheld diagnostics."""
    envelope = output["payload"]["mechanics_envelope"]
    rows = {}
    for row in envelope["results"]:
        basis = (row.get("basis_ref") or {}).get("ref_id")
        if basis:
            rows.setdefault(basis, []).append({"key": row_key(row), "value": row["value"], "id": row["id"]})
    withheld = {code["affected_refs"][0]: code["message"] for code in envelope["diagnostics"]
                if code.get("code") == "LOAD_COMBINATION_ROW_WITHHELD"}
    intensified = {case["load_case_id"]: case.get("intensified_measures") or []
                   for case in envelope["contract_evidence"].get("exact_cases", [])}
    unavailable = {code["affected_refs"][0] for code in envelope["diagnostics"]
                   if code.get("code") == "COMBINATION_INTENSIFIED_STRESS_UNAVAILABLE"}
    states = {state["load_case_id"]: state.get("members") or []
              for state in envelope["contract_evidence"].get("load_reference_states", [])}
    return {"rows": rows, "withheld": withheld, "status": envelope["status"]["mechanics"], "intensified": intensified,
            "intensified_unavailable": unavailable, "member_states": states}


@pytest.fixture(scope="module", params=SOLVER_MODES)
def mode_rows(request):
    runner = os.environ.get("OPENPIPESTRESS_RUNNER_BIN")
    if not runner:
        pytest.skip("OPENPIPESTRESS_RUNNER_BIN is not set")
    body = {"request": runner_request(), "solve": {"preview_model": {"model": MODEL, "materials": []}}}
    completed = subprocess.run([runner, "solve", "--explicit-local-private-intent", "--solver-mode", request.param,
                                "--input", "-"], input=json.dumps(body), capture_output=True, text=True, check=False)
    assert completed.returncode == 0 and completed.stdout, completed.stderr
    summary = summarise(json.loads(completed.stdout))
    assert summary["status"] == "MECHANICS_SOLVED"
    return request.param, summary


def check_row(label, observed, expected, zero_scale, failures, record):
    if expected != 0.0:
        error = abs(observed - expected) / abs(expected)
        record["relative"] = max(record.get("relative", 0.0), error)
        if error > RELATIVE:
            failures.append(f"{label}: observed {observed!r}, expected {expected!r}, relative error {error:.3e}")
    else:
        assert zero_scale > 0, label
        ratio = abs(observed) / zero_scale
        record["zero"] = max(record.get("zero", 0.0), ratio)
        if ratio > RELATIVE:
            failures.append(f"{label}: observed {observed!r} for a zero expectation, |obs|/zero_scale {ratio:.3e}")


def supports(case_rows):
    """{support id: {component: value}} from a case's support_reaction_component_v2 rows."""
    found = {}
    for row in case_rows:
        kind, _, entity, _, component, frame, _ = row["key"]
        if kind == SUPPORT_COMPONENT:
            assert frame == "global", row["id"]
            found.setdefault(entity, {})[component] = row["value"]
    return found


def resultants(case_rows):
    """The reaction resultant force and its moment about the global origin, sum of r x F + M over every support."""
    nodes = {node["id"]: tuple(node["position"][a] for a in "xyz") for node in MODEL["nodes"]}
    at = {support["id"]: nodes[support["node"]] for support in MODEL["supports"]}
    actions = supports(case_rows)
    assert set(actions) == set(at), sorted(set(actions) ^ set(at))  # every support, the spring included
    force, moment = [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]
    for support, values in sorted(actions.items()):
        assert set(values) == set(FORCES + MOMENTS), support
        f, m, r = [values[c] for c in FORCES], [values[c] for c in MOMENTS], at[support]
        lever = (r[1] * f[2] - r[2] * f[1], r[2] * f[0] - r[0] * f[2], r[0] * f[1] - r[1] * f[0])
        for i in range(3):
            force[i] += f[i]
            moment[i] += m[i] + lever[i]
    scales = (max(abs(v[c]) for v in actions.values() for c in FORCES),
              max(abs(v[c]) for v in actions.values() for c in MOMENTS))
    return force, moment, scales


def test_eq1_eq2_reactions_balance_the_independent_weight(mode_rows):
    """EQ-1: sum of vertical reactions = W_total; EQ-2: horizontal resultant zero, moment resultant = -M_weight.
    RS-CE1 is one support among all, its action counted once; the negative controls of the references (RS-P150's
    weight kept beside RS-R1's, RS-R1 omitted, RS-CE1 counted twice or left out) each miss."""
    mode, summary = mode_rows
    negative = json.loads(REFERENCES.read_text(encoding="utf-8"))["negative_controls"]
    failures, eq1, eq2 = [], {}, {}
    for case in EQ_CASES:
        weight, weight_moment = reference_values(case)
        force, moment, (force_scale, moment_scale) = resultants(summary["rows"][case])
        for label, control in negative.items():
            wrong = (control.get("W_N") or control["sum_Fz_N"])["value"]
            assert abs(force[2] - wrong) > 1e3 * RELATIVE * abs(wrong), (mode, case, label, force[2], wrong)
        check_row(f"EQ-1 {case} sum Fz", force[2], weight, force_scale, failures, eq1)
        for i, axis in enumerate("XY"):
            check_row(f"EQ-2 {case} sum F{axis.lower()}", force[i], 0.0, force_scale, failures, eq2)
        for i, axis in enumerate("XYZ"):
            check_row(f"EQ-2 {case} moment about the origin M{axis.lower()}", moment[i], -weight_moment[i],
                      moment_scale, failures, eq2)
    print(f"{mode} EQ-1 {eq1} EQ-2 {eq2}")  # largest relative error and zero-row ratio (pytest -s)
    assert not failures, failures


def test_rs_ce1_applies_and_publishes_its_entered_constant_load_in_every_case(mode_rows):
    """KR-03's identity for RS-CE1 (DEC-049): in every case, its applied-load row and its published support action Fz
    are its entered constant_load, and its other five components are zero. RS-SUS, RS-OPE and RS-SAM are compared with
    the frozen reference_system_r1_ce, which also gives the supports other than RS-CE1 W less it; RS-OPE-REF, which that
    package does not name, is compared with the entered load alone, the identity needing nothing else."""
    mode, summary = mode_rows
    refs = json.loads(REFERENCES.read_text(encoding="utf-8"))
    ce = refs["constant_effort"]
    record = next(s for s in MODEL["supports"] if s["id"] == ce["support"])
    assert (record["family"], record["node"], record["restraints"]) == ("constant_effort_support", ce["node"], ["UZ"])
    assert record["hanger"]["constant_load"] == {"value": float(ce["constant_load_N"]), "unit": "N"}
    entered = record["hanger"]["constant_load"]["value"]
    others_expected = ce["other_supports_sum_Fz_N"]["value"]
    assert [c["id"] for c in MODEL["load_cases"]] == [*ce["cases"], "load:RS-OPE-REF"]
    for case in [*ce["cases"], "load:RS-OPE-REF"]:
        rows = summary["rows"][case]
        applied = [row for row in rows if row["key"][0] == "constant_effort_support_applied_load"
                   and row["key"][2] == ce["support"]]
        action = supports(rows)[ce["support"]]
        if case in ce["cases"]:
            assert [row["value"] for row in applied] == [float(ce["applied_load_N"])], (mode, case, applied)
            expected = dict(zip(FORCES + MOMENTS, [float(v) for v in ce["support_action_on_pipe_N"]]
                                + [float(v) for v in ce["support_action_moment_about_node_N_m"]]))
            others = sum(values["Fz"] for support, values in supports(rows).items() if support != ce["support"])
            assert abs(others - others_expected) <= RELATIVE * others_expected, (mode, case, others, others_expected)
        else:
            assert [row["value"] for row in applied] == [entered], (mode, case, applied)
            expected = dict(zip(FORCES + MOMENTS, [0.0, 0.0, entered, 0.0, 0.0, 0.0]))
        assert action == expected, (mode, case, action)


def test_eq5_the_anchor_movement_term_is_self_equilibrated(mode_rows):
    mode, summary = mode_rows
    force, moment, (force_scale, moment_scale) = resultants(summary["rows"]["combination:RS-dSAM"])
    failures, eq5 = [], {}
    for i, axis in enumerate("xyz"):
        check_row(f"EQ-5 RS-dSAM sum F{axis}", force[i], 0.0, force_scale, failures, eq5)
        check_row(f"EQ-5 RS-dSAM moment about the origin M{axis}", moment[i], 0.0, moment_scale, failures, eq5)
    print(f"{mode} EQ-5 {eq5}")
    assert not failures, failures


def test_eq3_expansion_equals_rs_ope_ref_minus_sustained_on_every_component_row(mode_rows):
    """EQ-3 on RS-EXP's operands (spec §8.2, §9.2): the combination's algebra, row by row. It establishes nothing about
    either solve."""
    mode, summary = mode_rows
    combination = next(c for c in MODEL["combinations"] if c["id"] == "combination:RS-EXP")
    operands = json.loads(OPE_REF_REFERENCES.read_text(encoding="utf-8"))["eq3"]
    assert (combination["minuend_id"], combination["subtrahend_id"]) == (operands["minuend"], operands["subtrahend"])
    by_key = {}
    for case in (operands["minuend"], operands["subtrahend"], "combination:RS-EXP"):
        keyed = {}
        for row in summary["rows"][case]:
            keyed.setdefault(row["key"], []).append(row)
        by_key[case] = keyed
    minuend, sus, exp = by_key[operands["minuend"]], by_key[operands["subtrahend"]], by_key["combination:RS-EXP"]
    exp = {key: rows for key, rows in exp.items() if key[0] not in COMBINATION_RECORDS}
    # Coverage: every case component row is combined once, except the withheld (listed) and non-component rows.
    case_keys = {key for key in minuend if key[0] not in NOT_COMPONENT_ROWS}
    assert case_keys == {key for key in sus if key[0] not in NOT_COMPONENT_ROWS}
    withheld = {key for key in case_keys if key[0] in WITHHELD_KINDS}
    assert withheld and all(len(minuend[key]) > 1 for key in withheld)  # the rods share one key
    assert set(exp) == case_keys - withheld, sorted(set(exp) ^ (case_keys - withheld))[:10]
    message = summary["withheld"]["combination:RS-EXP"]
    withheld_ids = sorted(row["id"] for key in withheld for row in minuend[key])
    assert all(row_id.removeprefix("result:") in message for row_id in withheld_ids), (withheld_ids, message)
    # Expected values: RS-OPE-REF - RS-SUS for signed rows; the norm of the combined components for magnitude rows.
    expected = {}
    for key, rows in exp.items():
        assert len(rows) == 1 and len(minuend[key]) == 1 and len(sus[key]) == 1, key
        if key[0] not in MAGNITUDES:
            expected[key] = minuend[key][0]["value"] - sus[key][0]["value"]
    for key in exp:
        kind = key[0]
        if kind in MAGNITUDES:
            parts = MAGNITUDES[kind]
            if kind == "displacement_magnitude":
                comps = [exp[(p, key[1], key[2], "node", f"nodal_displacement_{p[-1]}", "global", "")][0]["value"]
                         for p in parts]
            else:
                comps = [exp[(SUPPORT_COMPONENT, key[1], key[2], key[3], c, key[5], key[6])][0]["value"]
                         for c in parts]
            expected[key] = sum(v * v for v in comps) ** 0.5
    # Zero scales: the largest nonzero expected magnitude of the row's family (its unit) in RS-EXP.
    scales = {}
    for key, value in expected.items():
        scales[key[1]] = max(scales.get(key[1], 0.0), abs(value))
    failures, eq3 = [], {}
    for key, rows in sorted(exp.items()):
        check_row(f"EQ-3 {rows[0]['id']}", rows[0]["value"], expected[key], scales[key[1]], failures, eq3)
    eq3["rows"], eq3["withheld"] = len(exp), withheld_ids
    print(f"{mode} EQ-3 {eq3}")
    assert not failures, failures[:20]


# The member states of RS-OPE and RS-OPE-REF (spec §4.1, §8.1) against reference_system_ope_ref, read from the published
# load reference state (contract_evidence.load_reference_states[].members). The tolerance is a priori, from rounding:
# each published quantity is formed from the decimal inputs by at most about ten binary64 operations (degC to K,
# interpolation, E/[2(1+nu)], the secant stretches and their ratio less one), each within 2^-53 relative; the one
# cancellation, a stretch near 1 less 1, amplifies that by at most 1/0.00228, under 440. So the relative error is below
# 10 * 1.2e-16 * 440, about 5e-13; 1e-12 bounds it. The two cases' moduli differ by 5.85 percent and a missing thermal
# strain by 100 percent, each far outside it. A zero expectation (RS-P180's strain) is exact.
MEMBER_STATE_RELATIVE = 1e-12
# Spec §3.2: RS-P180 is replaced by RS-EJ1 and "carries no load or result", so the published load reference state
# holds the other 25 members; the reference's RS-P180 row (zero eigenstrain) has no published counterpart.
UNPUBLISHED_MEMBERS = {"pipe:RS-P180"}


def member_state_misses(case, published, expected):
    """Every difference between a case's published member states and the reference's, by member."""
    misses = []
    assert len({member["pipe_id"] for member in published}) == len(published), (case, "a member published twice")
    by_pipe = {member["pipe_id"]: member for member in published}
    expected = [row for row in expected if row["member"] not in UNPUBLISHED_MEMBERS]
    assert len(expected) == 25 and sorted(by_pipe) == sorted(row["member"] for row in expected), (case, sorted(by_pipe))

    def close(what, observed, reference):
        if reference == 0.0:
            ok = observed == 0.0
        else:
            ok = observed is not None and abs(observed - reference) <= MEMBER_STATE_RELATIVE * abs(reference)
        if not ok:
            misses.append(f"{case} {what}: published {observed!r}, reference {reference!r}")

    for row in expected:
        pipe, member = row["member"], by_pipe[row["member"]]
        points = member["consumed_material_points"]
        if [p["point_id"] for p in points] != [p["point_id"] for p in row["consumed_material_points"]]:
            misses.append(f"{case} {pipe} consumed points: published {[p['point_id'] for p in points]}")
        else:
            for got, want in zip(points, row["consumed_material_points"]):
                close(f"{pipe} {want['point_id']} E", got["E_pa"], want["E_Pa"]["value"])
                close(f"{pipe} {want['point_id']} T", got["temperature_k"], float(Fraction(want["temperature_K"])))
        if member["material_selection_kind"] != row["material_selection_kind"]:
            misses.append(f"{case} {pipe} selection: published {member['material_selection_kind']!r}")
        if row["interpolation_fraction"] is not None:
            close(f"{pipe} interpolation fraction", member["interpolation_fraction"], row["interpolation_fraction"]["value"])
        close(f"{pipe} E", member["selected_E_pa"], row["E_Pa"]["value"])
        close(f"{pipe} nu", member["selected_nu"], row["nu"]["value"])
        close(f"{pipe} G", member["derived_G_pa"], row["G_Pa"]["value"])
        close(f"{pipe} thermal strain", member["thermal_strain"], row["thermal_strain"]["value"])
        close(f"{pipe} total eigenstrain", member["total_eigenstrain"], row["thermal_strain"]["value"])
        operating = row["operating_temperature_K"]
        if operating is None:
            if member["operating_temperature_k"] is not None:
                misses.append(f"{case} {pipe} operating temperature: published {member['operating_temperature_k']!r}")
        else:
            close(f"{pipe} operating temperature", member["operating_temperature_k"], float(Fraction(operating)))
        override = member.get("analysis_basis_override")
        reason = override and override.get("reason")
        if reason != row["analysis_basis_override_reason"]:
            misses.append(f"{case} {pipe} analysis-basis reason: published {reason!r}")
    return misses


def test_rs_ope_and_rs_ope_ref_publish_the_member_states_the_references_derive(mode_rows):
    """Each member's consumed point(s), E, nu, G, thermal strain and analysis-basis reason in RS-OPE and RS-OPE-REF
    equal the independent derivation: one thermal strain, two moduli."""
    mode, summary = mode_rows
    reference = json.loads(OPE_REF_REFERENCES.read_text(encoding="utf-8"))["member_states"]
    misses = []
    for case in ("load:RS-OPE", "load:RS-OPE-REF"):
        misses += member_state_misses(case, summary["member_states"][case], reference[case])
    assert not misses, (mode, misses[:20])


def test_the_member_state_check_tells_the_two_cases_apart():
    """The check discriminates: RS-OPE's published states, read as RS-OPE-REF's, miss on every hot member's modulus,
    shear modulus, consumed point and reason; a state with no thermal strain misses on its strain."""
    reference = json.loads(OPE_REF_REFERENCES.read_text(encoding="utf-8"))["member_states"]

    def as_published(row):
        return {"pipe_id": row["member"], "material_selection_kind": row["material_selection_kind"],
                "consumed_material_points": [{"point_id": p["point_id"], "E_pa": p["E_Pa"]["value"],
                                              "temperature_k": float(Fraction(p["temperature_K"]))}
                                             for p in row["consumed_material_points"]],
                "interpolation_fraction": row["interpolation_fraction"] and row["interpolation_fraction"]["value"],
                "selected_E_pa": row["E_Pa"]["value"], "selected_nu": row["nu"]["value"],
                "derived_G_pa": row["G_Pa"]["value"], "thermal_strain": row["thermal_strain"]["value"],
                "total_eigenstrain": row["thermal_strain"]["value"],
                "operating_temperature_k": row["operating_temperature_K"] and float(Fraction(row["operating_temperature_K"])),
                "analysis_basis_override": row["analysis_basis_override_reason"]
                and {"reason": row["analysis_basis_override_reason"]}}

    def published(case):
        return [as_published(r) for r in reference[case] if r["member"] not in UNPUBLISHED_MEMBERS]

    for case in ("load:RS-OPE", "load:RS-OPE-REF"):
        assert member_state_misses(case, published(case), reference[case]) == []
    hot_modulus = member_state_misses("load:RS-OPE-REF", published("load:RS-OPE"), reference["load:RS-OPE-REF"])
    for what in (" E:", " G:", "consumed points", "analysis-basis reason"):
        assert sum(what in m for m in hot_modulus) >= 25, what
    cold = [dict(r, thermal_strain=0.0, total_eigenstrain=0.0) for r in published("load:RS-OPE-REF")]
    no_strain = member_state_misses("load:RS-OPE-REF", cold, reference["load:RS-OPE-REF"])
    assert sum("thermal strain" in m for m in no_strain) == 25


def test_rs_ope_ref_states_the_specification_s_reason_and_the_references_take_its_inputs():
    """The fixture's RS-OPE-REF carries, on each of its 25 members with an operating temperature, the reason the
    specification states (§8.1) and the record provenance; RS-P180 states none; the references hold the same reason."""
    spec = (FIXTURE / "SPECIFICATION.md").read_text(encoding="utf-8")
    reference = json.loads(OPE_REF_REFERENCES.read_text(encoding="utf-8"))["member_states"]["load:RS-OPE-REF"]
    reasons = {row["analysis_basis_override_reason"] for row in reference} - {None}
    assert len(reasons) == 1
    (reason,) = reasons
    assert f"`{reason}`" in re.sub(r"\s*\n\s*", " ", spec)
    case = next(c for c in MODEL["load_cases"] if c["id"] == "load:RS-OPE-REF")
    states = case["analysis_state"]["element_states"]
    hot = [s for s in states if "operating_temperature" in s]
    assert len(hot) == 25 and len(states) == 26
    for state in hot:
        assert state["analysis_basis_override"] == {"reason": reason, "provenance": GENERATOR.PROVENANCE}, state["pipe_ref"]
    assert all("analysis_basis_override" not in s for s in states if s not in hot)


# KR-05 (spec §9.1): presence is an identity, checked here; its values come later from a verified solve, so no value is
# read. The tee RS-T1 at RS-130 names RS-P120 (header, which ends there: end_j) and RS-P210 (branch, which starts there:
# end_i); each side publishes one row per section basis, bound to its intensified_measures record.
BRANCH_KIND = "branch_intensified_bending_stress_v1"
KR05_ENDS = {("pipe:RS-P120", "end_j"): ("branch_header", 2.4), ("pipe:RS-P210", "end_i"): ("branch_branch", 2.0)}
SECTION_BASES = ("nominal_section", "reduced_section")


def test_kr05_the_tee_publishes_its_header_and_branch_rows_on_both_section_bases(mode_rows):
    mode, summary = mode_rows
    tee = next(c for c in MODEL["components"] if c["id"] == "component:RS-T1")
    pipes = {p["id"]: p for p in MODEL["pipe_segments"]}
    for (pipe, location), _ in KR05_ENDS.items():
        assert pipes[pipe]["to" if location == "end_j" else "from"] == tee["node"], pipe
    expected = {(BRANCH_KIND, "Pa", pipe, location, "intensified_bending", "pipe_section", basis)
                for pipe, location in KR05_ENDS for basis in SECTION_BASES}
    found = {}
    for case in ("load:RS-SUS", "load:RS-OPE"):
        branch_rows = [row for row in summary["rows"][case] if row["key"][0] == BRANCH_KIND]
        keys = [row["key"] for row in branch_rows]
        assert sorted(keys) == sorted(expected), (mode, case, keys)
        # Exactly one evidence record per row, each bound by its result_id to the row of its own member end and
        # section basis, with the tee's role and SIF; no record may be duplicated or point elsewhere.
        records = [r for r in summary["intensified"][case] if r.get("component_id") == tee["id"]]
        assert len(records) == len(expected), (mode, case, records)
        row_by_id = {row["id"]: row for row in branch_rows}
        assert len(row_by_id) == len(branch_rows) == len({r["result_id"] for r in records}), (mode, case)
        for record in records:
            row = row_by_id.get(record["result_id"])
            assert row is not None, (mode, case, record)
            pipe, location, basis = record["pipe_id"], record["location"], record["section_basis"]
            assert row["key"] == (BRANCH_KIND, "Pa", pipe, location, "intensified_bending", "pipe_section", basis), \
                (mode, case, record, row["key"])
            assert (record["factor_role"], record["sif"]) == KR05_ENDS[(pipe, location)], (mode, case, record)
        found[case] = len(keys)
    # RS-EXP is listed for KR-05 "where published": the combination withholds intensified rows by name.
    exp_keys = [row["key"] for row in summary["rows"]["combination:RS-EXP"] if row["key"][0] == BRANCH_KIND]
    if not exp_keys:
        assert "combination:RS-EXP" in summary["intensified_unavailable"], mode
    else:
        assert sorted(exp_keys) == sorted(expected), (mode, exp_keys)
    found["combination:RS-EXP"] = len(exp_keys)
    print(f"{mode} KR-05 presence rows {found}")


def test_kr05_names_its_presence_check_and_no_expected_value():
    entry = next(e for e in KEY_RESULTS["results"] if e["id"] == "KR-05")
    assert entry["expected"] is None and entry["basis"] == "verified_solve"
    assert entry["check"]["kind"] == "presence" and entry["check"]["test"] == "tests/test_reference_system_fixture.py"


def test_eq1_eq2_expected_values_are_the_frozen_references():
    weight, weight_moment = reference_values()
    expected = {entry["id"]: entry["expected"] for entry in KEY_RESULTS["results"]}
    assert expected["EQ-1"]["value"] == weight and expected["EQ-2"]["value"] == weight_moment
    assert reference_values("load:RS-OPE-REF") == (weight, weight_moment)
    refs = json.loads(REFERENCES.read_text(encoding="utf-8"))
    assert expected["EQ-1"]["exact"] == refs["key_values"]["W_total_N"]["exact"]
    assert expected["EQ-1"]["basis_record"] == expected["EQ-2"]["basis_record"] == REFERENCES.relative_to(ROOT).as_posix()
    for rid in ("EQ-1", "EQ-2"):
        assert expected[rid]["cases"] == ["RS-SUS", "RS-OPE", "RS-OPE-REF", "RS-SAM"], rid
        assert expected[rid]["case_binding"] == {"RS-OPE-REF": OPE_REF_REFERENCES.relative_to(ROOT).as_posix()}, rid
    for rid in ("EQ-3", "EQ-5"):
        assert expected[rid]["identity"] and expected[rid]["basis_record"].endswith("README.md"), rid
    assert expected["EQ-3"]["cases"] == ["RS-EXP", "RS-OPE-REF", "RS-SUS"]
    assert expected["EQ-3"]["basis_record"].startswith("validation/references/reference_system_ope_ref/")
    check = next(e for e in KEY_RESULTS["results"] if e["id"] == "KR-03")
    assert check["expected"] is None and check["check"]["kind"] == "identity"


def test_the_fixture_holds_the_inputs_the_references_derive_from():
    """The references read no fixture: the fixture's weighted members, the members a component's weight replaces, and
    each rigid-family component's weight, centre of gravity and node are the ones they derive."""
    refs = json.loads(REFERENCES.read_text(encoding="utf-8"))
    by_member = {row["member"]: row for row in refs["members"]}
    for case in MODEL["load_cases"]:
        lines = sorted(load["target"]["pipe"] for load in case["primitive_loads"] if load["target"]["type"] == "element")
        assert lines == refs["self_weight_inventory"]["weighted_members"], case["id"]
        assert len(case["primitive_loads"]) == refs["self_weight_inventory"]["per_case"]["total"], case["id"]
    replaced = {c["geometry"]["rigid_pipe_ref"]: c["id"] for c in MODEL["components"] if "rigid_pipe_ref" in c.get("geometry", {})}
    assert replaced == {f"pipe:{row['member']}": row["by"] for row in by_member.values() if "by" in row}
    nodes = {node["id"]: node["position"] for node in MODEL["nodes"]}
    for comp in refs["components"]:
        record = next(c for c in MODEL["components"] if c["id"] == comp["component"])
        assert record["node"] == comp["node"] and float(record["geometry"]["weight"]["value"]) == comp["weight_N"]["value"]
        cog = record["geometry"]["center_of_gravity"]
        assert [cog[a] for a in "xyz"] == [float(Fraction(v)) for v in comp["center_of_gravity_from_node_m"]], comp
        assert [nodes[comp["node"]][a] + cog[a] for a in "xyz"] == pytest.approx(
            [float(Fraction(v)) for v in comp["position_m"]], rel=1e-15, abs=1e-15)
    reducer = refs["reducer"]
    record = next(c for c in MODEL["components"] if c["id"] == reducer["component"])
    offset = record["geometry"]["eccentric_offset"]
    assert (offset["unit"], [offset[a] for a in "xyz"]) == ("mm", [float(v) for v in reducer["eccentric_offset_mm"]])
    assert [record["geometry"][k]["value"] for k in ("end_a_size", "end_b_size")] == [int(v) for v in reducer["end_sizes_mm"]]
    assert record["modifiers"]["stiffness_scaling_user_value"]["value"] == float(reducer["stiffness_scale"])


def test_the_reference_generator_checks_pass(tmp_path):
    """The adopted references' checks (RS-OPE-REF's and RS-R1/RS-CE1's), in place; and the frozen tranche-1 package's, on the tranche-1 inputs it was
    generated from (tests/reference_packages.py stages them)."""
    for name in ("reference_system_ope_ref", "reference_system_r1_ce", "reference_system"):
        problems = reference_packages.check_problems(name, 0, tmp_path / name / "check_output")
        assert not problems, problems


def test_the_rs_ope_ref_reference_generator_is_independent_of_the_product():
    """reference_system_ope_ref's generator imports only the standard library, makes no dynamic import or evaluation,
    and names no product path: its inputs are the specification's statements, transcribed."""
    import ast
    tree = ast.parse((ROOT / "validation" / "references" / "reference_system_ope_ref" / "generators" /
                      "rs_ope_ref_reference.py").read_text(encoding="utf-8"))
    modules, called, paths = set(), set(), set()
    for node in ast.walk(tree):
        if isinstance(node, ast.Import):
            modules |= {alias.name.split(".")[0] for alias in node.names}
        elif isinstance(node, ast.ImportFrom):
            assert node.level == 0
            modules.add(node.module.split(".")[0])
        elif isinstance(node, ast.Call):
            func = node.func
            name = func.id if isinstance(func, ast.Name) else func.attr if isinstance(func, ast.Attribute) else None
            if name in {"__import__", "import_module", "exec", "eval", "spec_from_file_location"}:
                called.add(name)
        elif isinstance(node, ast.Constant) and isinstance(node.value, str) and not re.search(r"\s", node.value):
            if set(re.split(r"[\\/]+", node.value)) & {"core", "apps", "tests", "tools", "fixtures", "schemas"}:
                paths.add(node.value)
    assert modules and modules <= set(sys.stdlib_module_names), sorted(modules - set(sys.stdlib_module_names))
    assert not called and not paths, (called, paths)
