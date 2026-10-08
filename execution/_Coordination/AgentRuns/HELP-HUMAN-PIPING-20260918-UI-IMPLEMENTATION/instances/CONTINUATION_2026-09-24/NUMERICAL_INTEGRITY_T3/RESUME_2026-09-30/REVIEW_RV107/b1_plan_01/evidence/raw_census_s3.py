"""RV107: approximate D1.9 raw census (retained_memory.rs borrowed_value_census semantics:
every JSON value counted, string bytes summed, key bytes summed, array elements summed)
for the law tests' cap_maximal input and three-case variants at S3's caps. Read-only."""
import json, math, copy
P = "invented_t3_g5_cap_maximal_input_no_library_data"
def text(prefix, n): return (prefix + "x" * n)[:n]
def cap_maximal(cases=1, distinct_ids=True, l=128):
    nodes = [{"id": f"N{i}", "position": {"x": 10*math.cos(2*math.pi*i/32), "y": 10*math.sin(2*math.pi*i/32), "z": 0.0}, "provenance": P} for i in range(32)]
    pipes = [{"id": f"M{i}", "from": f"N{i}", "to": f"N{(i+1)%32}", "material": "mat:0", "y_reference": {"x": 0.0, "y": 0.0, "z": 1.0},
              "section": {"outside_diameter": {"value": 0.2, "unit": "m"}, "wall_thickness": {"value": 0.01, "unit": "m"}}, "provenance": P} for i in range(32)]
    supports = [{"id": f"S{i}", "node": f"N{i}", "restraints": ["UX","UY","UZ","RX","RY","RZ"],
                 "stiffness": {"dof": "UY", "value": {"value": 1.0e6, "unit": "N/m"}}, "provenance": P} for i in range(32)]
    def loads(k):
        return [{"id": (f"C{k}L{i}" if distinct_ids else f"L{i}"), "category": "concentrated_force", "target": {"type": "node", "node": f"N{i%32}"},
                 "direction": "global_y" if i % 2 == 0 else "rotation_x", "magnitude": {"value": 1.0, "unit": "N" if i % 2 == 0 else "N*m"},
                 "dimension": "force" if i % 2 == 0 else "moment", "provenance": P} for i in range(l)]
    points = [{"id": f"T{i}", "provenance": P} for i in range(16)]
    materials = [{"id": f"mat:{i}", "elastic_modulus": {"value": 2.0e11, "unit": "Pa"}, "shear_modulus": {"value": 8.0e10, "unit": "Pa"},
                  "temperature_points": points, "provenance": P} for i in range(4)]
    return {"model": {"schema_version": "0.1.0", "document_kind": "openpipestress.product_preview.model",
            "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
            "project": {"id": text("project:", 128), "units": {"length": "m", "force": "N"}},
            "nodes": nodes, "pipe_segments": pipes, "materials": materials, "supports": supports,
            "load_cases": [{"id": ("case" if k == 0 else f"case-{k}"), "primitive_loads": loads(k), "provenance": P} for k in range(cases)],
            "combinations": []}, "materials": materials}
def escape(v):
    if isinstance(v, dict):
        for k, x in v.items():
            if k == "provenance" and isinstance(x, str): v[k] = x + ' q"b\\'
            else: escape(x)
    elif isinstance(v, list):
        for x in v: escape(x)
def census(v):
    f = dict(values=0, string_bytes=0, key_bytes=0, array_elements=0)
    def walk(x, key=None):
        f["values"] += 1
        if key is not None: f["key_bytes"] += len(key.encode())
        if isinstance(x, str): f["string_bytes"] += len(x.encode())
        elif isinstance(x, list):
            f["array_elements"] += len(x)
            for y in x: walk(y)
        elif isinstance(x, dict):
            for k, y in x.items(): walk(y, k)
    walk(v); return f
CAPS = dict(values=16384, string_bytes=65536, key_bytes=65536, array_elements=32768)
for label, cases, esc in [("c1 (law cap_maximal)", 1, False), ("c1 escaped (W2-style)", 1, True), ("c3 x 128 loads", 3, False), ("c3 x 128 loads escaped", 3, True)]:
    v = cap_maximal(cases); 
    if esc: escape(v)
    f = census(v)
    print(f"{label:26s} " + "  ".join(f"{k}={f[k]:,}/{CAPS[k]:,}{' OVER' if f[k] > CAPS[k] else ''}" for k in CAPS))
