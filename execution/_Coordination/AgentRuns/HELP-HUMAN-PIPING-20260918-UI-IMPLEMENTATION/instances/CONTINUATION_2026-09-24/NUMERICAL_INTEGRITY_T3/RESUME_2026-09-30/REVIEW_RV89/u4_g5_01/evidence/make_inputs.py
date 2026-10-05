"""RV89: sweep inputs. Fixture requests (model files wrapped as {"model": ...}) plus
RV89's own synthetic variants of the milestone and of an RV89 cap-maximal request."""
import json, os, sys, glob, copy
P = sys.argv[1]; OUT = sys.argv[2]
FX = os.path.join(P, "fixtures/product_preview")
os.makedirs(OUT, exist_ok=True)
n = 0
def put(name, value, raw_text=None):
    global n
    n += 1
    with open(os.path.join(OUT, f"{n:03d}_{name}.json"), "w") as f:
        f.write(raw_text if raw_text is not None else json.dumps(value))
files = sorted(glob.glob(FX + "/*.request.json") + glob.glob(FX + "/*/*.request.json") + glob.glob(FX + "/source_blocks/ui/*.request.json"))
files = [f for f in files if "rejected_stress_range" not in f]
files += [FX + "/invented_dec092_temperature_g_request.json"]
for f in files:
    put("fx_" + os.path.relpath(f, FX).replace("/", "__").replace(".json", ""), json.load(open(f)))
for f in [FX + "/invented_preview_model.json", FX + "/numerical_sensitive_torsion_model.json",
          P + "/core/product_physics/tests/fixtures/preview_physics_invented_model.json",
          P + "/core/product_physics/tests/fixtures/preview_physics_unicode_ids_model.json"]:
    put("model_" + os.path.basename(f).replace(".json", ""), {"model": json.load(open(f))})
put("fx_exact_pressure_connected", json.load(open(P + "/core/product_physics/tests/fixtures/exact_pressure_connected_request.json")))
M = json.load(open(FX + "/rf_skew_t_cant_off_122_r1e-04.request.json"))
def var(name, fn):
    r = copy.deepcopy(M); fn(r); put("ms_" + name, r)
var("schema_020", lambda r: r["model"].__setitem__("schema_version", "0.2.0"))
var("schema_030", lambda r: r["model"].__setitem__("schema_version", "0.3.0"))
var("two_cases", lambda r: r["model"]["load_cases"].append(dict(r["model"]["load_cases"][0], id="case2")))
var("no_cases", lambda r: r["model"].__setitem__("load_cases", []))
var("family_space", lambda r: r["model"]["supports"][1].__setitem__("family", " anchor"))
var("family_hanger_word", lambda r: r["model"]["supports"][1].__setitem__("family", "hanger"))
var("load_element", lambda r: r["model"]["load_cases"][0]["primitive_loads"][0].__setitem__("target", {"type": "element", "pipe": r["model"]["pipe_segments"][0]["id"]}))
var("load_dim_pressure", lambda r: r["model"]["load_cases"][0]["primitive_loads"][0].__setitem__("dimension", "pressure"))
var("prov_object", lambda r: r["model"]["load_cases"][0]["primitive_loads"][0].__setitem__("provenance", "{\"method\":\"x\"}"))
var("prov_ws_object", lambda r: r["model"]["load_cases"][0]["primitive_loads"][0].__setitem__("provenance", "  {\"method\":\"pipe_mass_per_length_times_explicit_axis_acceleration\"}"))
var("prov_ff_object", lambda r: r["model"]["load_cases"][0]["primitive_loads"][0].__setitem__("provenance", "\u000c{}"))
var("ctrl_key", lambda r: r["model"].__setitem__("ex\u0001tra", 1))
var("ctrl_value", lambda r: r["model"]["nodes"][0].__setitem__("provenance", "a\tb"))
var("del_value", lambda r: r["model"]["nodes"][0].__setitem__("provenance", "a\u007fb"))
var("del_key", lambda r: r["model"].__setitem__("x\u007f", 1))
var("text_128", lambda r: r["model"]["nodes"][0].__setitem__("provenance", "p" * 128))
var("text_129", lambda r: r["model"]["nodes"][0].__setitem__("provenance", "p" * 129))
var("key_129", lambda r: r["model"].__setitem__("k" * 129, 1))
var("unicode_128B", lambda r: r["model"]["nodes"][0].__setitem__("provenance", "é" * 64))
var("unicode_130B", lambda r: r["model"]["nodes"][0].__setitem__("provenance", "é" * 65))
var("bad_node_ref", lambda r: r["model"]["pipe_segments"][0].__setitem__("to", "NOPE"))
var("sections_present", lambda r: r["model"].__setitem__("sections", [{"id": "s", "name": "s", "section_type": "pipe", "properties": {}, "provenance": None}]))
var("equiv_static", lambda r: r["model"]["load_cases"][0].__setitem__("equivalent_static", {}))
def deep(r, levels):
    v = 1
    for _ in range(levels): v = [v]
    r["model"]["deep"] = v
var("depth_16", lambda r: deep(r, 14))
var("depth_17", lambda r: deep(r, 15))
var("depth_70", lambda r: deep(r, 70))
var("values_over", lambda r: r["model"].__setitem__("many", [0] * 16384))
# RV89's own cap-maximal request (independent of I65's helper): 32 nodes on a line,
# 32 members in a chain + ring closure, 32 supports x 6 restraints, 32 springs,
# 128 nodal loads, 4+4 materials x 16 temperature points.
def capmax():
    p = "rv89_invented_cap_maximal_no_library_data"
    nodes = [{"id": f"n{i:02d}", "position": {"x": float(i), "y": 0.5 * (i % 3), "z": 0.0}, "provenance": p} for i in range(32)]
    pipes = [{"id": f"p{i:02d}", "from": f"n{i:02d}", "to": f"n{(i + 1) % 32:02d}", "material": "m0",
              "section": {"outside_diameter": {"value": 0.1683, "unit": "m"}, "wall_thickness": {"value": 0.00711, "unit": "m"}}} for i in range(32)]
    sup = [{"id": f"s{i:02d}", "node": f"n{i:02d}", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "family": "anchor",
            "stiffness": {"dof": "UZ", "value": {"value": 2.5e6, "unit": "N/m"}}} for i in range(32)]
    loads = [{"id": f"l{i:03d}", "category": "weight", "target": {"type": "node", "node": f"n{i % 32:02d}"},
              "direction": ["FX", "FY", "FZ", "MX", "MY", "MZ"][i % 6], "magnitude": {"value": 10.0 + i, "unit": "N" if i % 6 < 3 else "N*m"},
              "dimension": "force" if i % 6 < 3 else "moment", "provenance": "rv89 plain text"} for i in range(128)]
    pts = [{"id": f"t{i:02d}", "temperature": {"value": 20.0 + i, "unit": "degC"}} for i in range(16)]
    mats = [{"id": f"m{i}", "elastic_modulus": {"value": 2.0e11, "unit": "Pa"}, "poisson_ratio": {"value": 0.3, "unit": "1"}, "temperature_points": pts} for i in range(4)]
    return {"model": {"schema_version": "0.1.0", "document_kind": "openpipestress.product_preview.model",
            "project": {"id": "q" * 128, "units": {"length": "m", "force": "N"}},
            "analysis_status": {"mechanics": "preview", "rule_check": "not_performed", "professional_acceptance": "not_provided"},
            "nodes": nodes, "pipe_segments": pipes, "supports": sup, "materials": mats,
            "load_cases": [{"id": "c0", "primitive_loads": loads}], "combinations": [], "components": [], "sections": []},
            "materials": mats}
C = capmax()
put("cm_at_caps", C)
def cvar(name, fn):
    r = copy.deepcopy(C); fn(r); put("cm_" + name, r)
cvar("nodes_33", lambda r: r["model"]["nodes"].append(dict(r["model"]["nodes"][0], id="n32")))
cvar("loads_129", lambda r: r["model"]["load_cases"][0]["primitive_loads"].append(dict(r["model"]["load_cases"][0]["primitive_loads"][0], id="l128")))
cvar("restraints_193", lambda r: r["model"]["supports"][0]["restraints"].append("UX"))
cvar("materials_5", lambda r: r["model"]["materials"].append(dict(r["model"]["materials"][0], id="m4")))
cvar("tpoints_17", lambda r: r["model"]["materials"][0]["temperature_points"].append({"id": "t16"}))
cvar("supports_33", lambda r: r["model"]["supports"].append({"id": "s32", "node": "n00", "restraints": []}))
cvar("members_33", lambda r: r["model"]["pipe_segments"].append(dict(r["model"]["pipe_segments"][0], id="p32")))
# raw text that is not JSON-normalized: whitespace before a provenance '{' inside the raw bytes
put("raw_text_escapes", None, raw_text=json.dumps(M).replace('"provenance": "invented', '"provenance": "\\u0020\\u0020invented', 1))
put("invalid_not_object", [1, 2, 3])
put("invalid_missing_model", {"materials": []})
print(n)
