"""Author the load-reference-source-1 witness requests (all inputs invented).

1. 0.4.0 unchanged-state companions of the retained-source physics_source
   witnesses n05, n06, fields and mixed: every member direct_strain_reference
   with fit none, thermal unchanged_reference, the case's own material
   selection (explicit base, or the exact point its 0.3.0 modulus_basis_ref
   named), every support active, every stored primitive listed once at factor
   1.0. Expected mechanics: the 0.3.0 physics-source-1 publication.
2. eigen_motion: a sensitive single member (soft grounded torsion spring) with
   thermal 6e-5 and fit 4e-5 eigenstrain, prescribed rigid root UY 1 mm and
   RZ 1e-4 rad, and a tip axial stop moving UX 5e-5 m. Closed-form expectations
   are in core/product_physics/src/source_receipt/load_state_tests.rs.
Output: json.dumps(indent=2) + newline.
"""
import copy, json, pathlib, sys
root = pathlib.Path(sys.argv[1])  # WORKING_ROOT
src = root / "fixtures/product_preview/physics_source"
out = root / "fixtures/product_preview/load_reference_source"
out.mkdir(exist_ok=True)
P = "invented_load_reference_source_companion_not_library_data"

def companion(request):
    request = copy.deepcopy(request)
    model = request["model"]
    assert request.get("materials", []) == [], "0.4.0 resolves model materials only"
    model["schema_version"] = "0.4.0"
    model["reference_configurations"] = [{
        "id": "reference:companion", "geometry_ref": {"kind": "authored_model_geometry"},
        "member_references": [{"pipe_ref": p["id"], "basis": {"kind": "direct_strain_reference"},
                               "fit": {"kind": "none"}, "provenance": P} for p in model["pipe_segments"]],
        "provenance": P}]
    for case in model["load_cases"]:
        point = case.pop("modulus_basis_ref", None)
        assert "modulus_basis_temperature" not in case
        def selection(pipe):
            if point is None:
                return {"kind": "explicit_base_properties", "material_ref": pipe["material"], "applicability_reference": P}
            return {"kind": "exact_point", "material_ref": pipe["material"], "point_ref": point}
        case["analysis_state"] = {
            "contract": "openpipestress.load_reference_state/1.0.0",
            "reference_configuration_ref": "reference:companion",
            "element_states": [{"pipe_ref": p["id"], "material_selection": selection(p),
                                "thermal_state": {"kind": "unchanged_reference", "provenance": P}}
                               for p in model["pipe_segments"]],
            "support_states": [{"support_ref": s["id"], "participation": {"kind": "active_model_device"}}
                               for s in model["supports"]],
            "load_sources": [{"source_ref": l["id"], "factor": 1.0} for l in case["primitive_loads"]],
            "history": {"kind": "independent_equilibrium"}, "provenance": P}
    return request

written = {}
for name in ["n05", "n06", "fields", "mixed"]:
    request = json.loads((src / f"{name}.request.json").read_text())
    path = out / f"{name}.request.json"
    path.write_text(json.dumps(companion(request), indent=2) + "\n")
    written[name] = str(path.relative_to(root))

E, NU, L = 2.0e11, 0.25, 2.0
def node(i, x):
    return {"id": i, "position": {"x": x, "y": 0, "z": 0}, "provenance": P}
eigen = {"model": {
    "document_kind": "openpipestress.product_preview.model", "schema_version": "0.4.0", "diagnostics": [],
    "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "professional_acceptance": "not_provided",
                        "rule_check": "not_performed_user_rule_inputs_missing"},
    "data_boundary": {"private_data_policy": "no_private_project_data",
                      "professional_boundary": "technical_preview_requires_human_engineering_review",
                      "protected_source_policy": "no_bundled_protected_owner_or_standards_data",
                      "public_examples_policy": "invented_or_cleared_data_only"},
    "project": {"id": "synthetic:load-reference-join", "name": "Invented join control",
                "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
    "pressure_contract": {"version": "2.0.0", "mode": "exact_straight_pressure_v2"},
    "nodes": [node("root", 0.0), node("tip", L)],
    "pipe_segments": [{"id": "member", "from": "root", "to": "tip", "material": "material",
                       "section": {"outside_diameter": {"value": 0.2, "unit": "m"}, "wall_thickness": {"value": 0.01, "unit": "m"}},
                       "y_reference": {"x": 0, "y": 1, "z": 0}, "provenance": P}],
    "materials": [{"id": "material", "provenance": P, "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
                   "elastic_modulus": {"value": E, "unit": "Pa"}, "poisson_ratio": {"value": NU, "unit": "1"}}],
    "supports": [
        {"id": "anchor", "node": "root", "family": "anchor", "restraints": ["UX", "UY", "UZ", "RY", "RZ"], "provenance": P},
        {"id": "spring", "node": "root", "family": "spring", "restraints": ["RX"],
         "stiffness": {"dof": "RX", "value": {"value": 1.0e-4, "unit": "N*m/rad"}}, "provenance": P},
        {"id": "stop", "node": "tip", "family": "line_stop", "restraints": ["UX"], "provenance": P}],
    "reference_configurations": [{"id": "reference:join", "geometry_ref": {"kind": "authored_model_geometry"},
        "member_references": [{"pipe_ref": "member", "basis": {"kind": "direct_strain_reference"},
                               "fit": {"kind": "fit_strain", "strain": {"value": 4.0e-5, "unit": "1"}}, "provenance": P}],
        "provenance": P}],
    "load_cases": [{
        "id": "case:join", "kind": "primitive_user_load", "status": "user_entered", "provenance": P,
        "primitive_loads": [{"id": "torque", "category": "concentrated_moment", "dimension": "moment", "direction": "RX",
                             "magnitude": {"value": 1.0e-8, "unit": "N*m"}, "target": {"type": "node", "node": "tip"}, "provenance": P}],
        "pressure_regions": [],
        "analysis_state": {
            "contract": "openpipestress.load_reference_state/1.0.0", "reference_configuration_ref": "reference:join",
            "element_states": [{"pipe_ref": "member", "material_selection": {"kind": "explicit_base_properties",
                                "material_ref": "material", "applicability_reference": P},
                                "thermal_state": {"kind": "explicit_interval_strain", "strain": {"value": 6.0e-5, "unit": "1"},
                                                  "interval_reference": "invented interval", "provenance": P}}],
            "support_states": [
                {"support_ref": "anchor", "participation": {"kind": "active_model_device"}, "boundary_motion": [
                    {"dof": "UY", "value": {"value": 1.0, "unit": "mm"}, "meaning": "absolute_reference_displacement"},
                    {"dof": "RZ", "value": {"value": 1.0e-4, "unit": "rad"}, "meaning": "absolute_reference_displacement"}]},
                {"support_ref": "spring", "participation": {"kind": "active_model_device"}},
                {"support_ref": "stop", "participation": {"kind": "active_model_device"}, "boundary_motion": [
                    {"dof": "UX", "value": {"value": 5.0e-5, "unit": "m"}, "meaning": "absolute_reference_displacement"}]}],
            "load_sources": [{"source_ref": "torque", "factor": 1.0}],
            "history": {"kind": "independent_equilibrium"}, "provenance": P}}],
    "combinations": [], "components": []}}
path = out / "eigen_motion.request.json"
path.write_text(json.dumps(eigen, indent=2) + "\n")
written["eigen_motion"] = str(path.relative_to(root))
print(json.dumps(written, indent=1))
