"""Author invented 0.4.0 load/reference-state request fixtures (no library data)."""
import json, pathlib, sys
out = pathlib.Path(sys.argv[1])
PROV = "invented_load_reference_fixture_not_library_data"
def q(v, u): return {"value": v, "unit": u}
def node(i, x): return {"id": i, "position": {"x": x, "y": 0, "z": 0}, "provenance": PROV}
def pipe(i, a, b): return {"id": i, "from": a, "to": b, "material": "material:shared",
    "section": {"outside_diameter": q(200, "mm"), "wall_thickness": q(10, "mm")},
    "y_reference": {"x": 0, "y": 1, "z": 0}, "provenance": PROV}
def anchor(i, n): return {"id": i, "node": n, "family": "anchor", "restraints": ["UX","UY","UZ","RX","RY","RZ"], "provenance": PROV}
def base(pid, nodes, pipes, supports, material, references, cases):
    return {"model": {"schema_version": "0.4.0", "document_kind": "openpipestress.product_preview.model",
        "pressure_contract": {"version": "2.0.0", "mode": "exact_straight_pressure_v2"},
        "project": {"id": pid, "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": nodes, "pipe_segments": pipes, "supports": supports, "components": [], "materials": [material],
        "reference_configurations": [{"id": "reference:installed", "label": "Invented installed reference",
            "geometry_ref": {"kind": "authored_model_geometry"}, "member_references": references, "provenance": PROV}],
        "load_cases": cases, "combinations": []}, "materials": []}
material = {"id": "material:shared", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
    "elastic_modulus": q(200, "GPa"), "poisson_ratio": q(0.3, "1"), "provenance": PROV,
    "temperature_points": [
        {"id": "point:cold", "temperature": q(20, "degC"), "elastic_modulus": q(200, "GPa"), "poisson_ratio": q(0.3, "1"), "provenance": PROV},
        {"id": "point:hot", "temperature": q(150, "degC"), "elastic_modulus": q(150, "GPa"), "poisson_ratio": q(0.3, "1"), "provenance": PROV}],
    "expansion_laws": [{"id": "law:secant", "definition": "engineering_secant", "datum_temperature": q(20, "degC"),
        "data": {"kind": "constant", "coefficient": q(1.2e-5, "1/K")}, "provenance": PROV}]}
def element(p, t, point):
    return {"pipe_ref": p, "operating_temperature": q(t, "degC"),
        "material_selection": {"kind": "exact_point", "material_ref": "material:shared", "point_ref": point},
        "thermal_state": {"kind": "free_length_state", "expansion_law_ref": "law:secant"}}
def state(elements, supports, sources):
    return {"contract": "openpipestress.load_reference_state/1.0.0", "reference_configuration_ref": "reference:installed",
        "element_states": elements, "support_states": supports, "load_sources": sources,
        "history": {"kind": "independent_equilibrium"}, "provenance": PROV}
force = lambda i, v: {"id": i, "category": "concentrated_force", "target": {"type": "node", "node": "node:middle"},
    "direction": "global_y", "magnitude": q(v, "N"), "dimension": "force", "provenance": PROV}
motion = [{"support_ref": "support:root", "participation": {"kind": "active_model_device"},
    "boundary_motion": [{"dof": "UX", "value": q(0.5, "mm"), "meaning": "absolute_reference_displacement"},
                        {"dof": "RZ", "value": q(0.001, "rad"), "meaning": "absolute_reference_displacement"}]},
    {"support_ref": "support:far", "participation": {"kind": "active_model_device"}}]
refs = [{"pipe_ref": "pipe:first", "basis": {"kind": "temperature_reference", "installation_temperature": q(20, "degC")}, "fit": {"kind": "none"}, "provenance": PROV},
        {"pipe_ref": "pipe:second", "basis": {"kind": "temperature_reference", "installation_temperature": q(20, "degC")},
         "fit": {"kind": "natural_length_change", "length_change": q(-1, "mm")}, "provenance": PROV}]
cases = []
for cid, t, point, factor in [("case:cold", 20, "point:cold", 1.0), ("case:hot", 150, "point:hot", 1.5)]:
    cases.append({"id": cid, "provenance": PROV, "pressure_regions": [],
        "primitive_loads": [force(f"load:transverse:{cid}", -1000.0), force(f"load:stored-unused:{cid}", 999.0)],
        "analysis_state": state([element("pipe:first", t, point), element("pipe:second", t, point)], motion,
            [{"source_ref": f"load:transverse:{cid}", "factor": factor}])})
a = base("project:load-reference-connected", [node("node:root", 0), node("node:middle", 1), node("node:far", 2)],
    [pipe("pipe:first", "node:root", "node:middle"), pipe("pipe:second", "node:middle", "node:far")],
    [anchor("support:root", "node:root"), anchor("support:far", "node:far")], material, refs, cases)
region = [{"id": "region:closed", "member_pipe_ids": ["pipe:first"], "pressure_basis": "internal_differential_zero_external_v1",
    "pressure": q(2, "MPa"), "terminals": [
        {"node_ref": "node:root", "closure_transfer": "transfers_to_wall", "provenance": PROV},
        {"node_ref": "node:middle", "closure_transfer": "transfers_to_wall", "provenance": PROV}], "provenance": PROV}]
pcases = []
for cid, t, point in [("case:cold-pressure", 20, "point:cold"), ("case:hot-pressure", 150, "point:hot")]:
    pcases.append({"id": cid, "provenance": PROV, "pressure_regions": region, "primitive_loads": [],
        "analysis_state": state([element("pipe:first", t, point)],
            [{"support_ref": "support:root", "participation": {"kind": "active_model_device"}},
             {"support_ref": "support:far", "participation": {"kind": "active_model_device"}}], [])})
b = base("project:load-reference-pressure", [node("node:root", 0), node("node:middle", 1)],
    [pipe("pipe:first", "node:root", "node:middle")],
    [anchor("support:root", "node:root"), anchor("support:far", "node:middle")], material, refs[:1], pcases)
for name, doc in [("connected", a), ("pressure", b)]:
    (out / f"{name}.request.json").write_text(json.dumps(doc, indent=2) + "\n")
print("written")
