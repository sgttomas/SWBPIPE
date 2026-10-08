"""I99 B3-W input generator (system python3, standard library; reads committed bytes only).
usage: gen_inputs.py <archive P root> <out dir>
Every input is written compact (json.dumps(separators=(',',':'))), as the milestone fixture is."""
import copy, json, sys, pathlib

P = pathlib.Path(sys.argv[1]); OUT = pathlib.Path(sys.argv[2]); OUT.mkdir(parents=True, exist_ok=True)
MILESTONE = P / "fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json"
PS = P / "fixtures/product_preview/physics_source"

def write(name, value):
    (OUT / f"{name}.json").write_text(json.dumps(value, separators=(",", ":"), ensure_ascii=False))

def copy_committed(name, src):
    (OUT / f"{name}.json").write_bytes(src.read_bytes())

base = json.loads(MILESTONE.read_text())
PROV = base["model"]["nodes"][0]["provenance"]

def exact(m):
    """The milestone authored as 0.3.0 exact: the exact contract, the common E/nu basis with
    E = 2e11 Pa and nu = 0.25 (no shear modulus: G is derived, 2e11/(2*1.25) = 8e10 exactly,
    the 0.1.0 milestone's G), and explicitly empty pressure regions on every case."""
    m = copy.deepcopy(m)
    md = m["model"]
    md["schema_version"] = "0.3.0"
    md["pressure_contract"] = {"version": "2.0.0", "mode": "exact_straight_pressure_v2"}
    for mat in md["materials"]:
        del mat["shear_modulus"]
        mat["constitutive_basis"] = "homogeneous_isotropic_E_nu_v1"
        mat["poisson_ratio"] = {"value": 0.25, "unit": "1"}
    for case in md["load_cases"]:
        case["pressure_regions"] = []
    return m

def legacy3(m):
    """The milestone authored as 0.3.0 legacy_pressure_v1 with zero pressure (no pressure
    primitive, no regions); materials unchanged (E and G)."""
    m = copy.deepcopy(m)
    m["model"]["schema_version"] = "0.3.0"
    m["model"]["pressure_contract"] = {"version": "1.0.0", "mode": "legacy_pressure_v1"}
    return m

def force(lid, node, direction, value):
    return {"id": lid, "category": "concentrated_force", "target": {"type": "node", "node": node}, "direction": direction,
            "magnitude": {"value": value, "unit": "N"}, "dimension": "force", "provenance": PROV}

# Item 2's second cases (each on the milestone's own model and body).
SECOND = {
    # an axial force at N1 along the member's exact direction (1, 2, 2)/3, 3 N in all
    "axial": [force("load:b:0", "N1", "global_x", 1.0), force("load:b:1", "N1", "global_y", 2.0), force("load:b:2", "N1", "global_z", 2.0)],
    # a force on N0's rigidly restrained translation (UX): no free load term
    "anchor": [force("load:b:0", "N0", "global_x", 1.0)],
    # variant 3: a lateral force at N1, F = (0, 1, 1) N, whose moment about N0, r x F =
    # (0, -1, 1) N*m with r = (1, 2, 2), has no global-X component (the soft RX spring)
    "lateral": [force("load:b:0", "N1", "global_y", 1.0), force("load:b:1", "N1", "global_z", 1.0)],
}
def second_case(key):
    return {"id": "case:b", "label": f"I99 B3-W second case ({key})", "kind": "primitive_user_load",
            "primitive_loads": SECOND[key], "provenance": PROV}

def mixed(key):
    m = copy.deepcopy(base)
    m["model"]["load_cases"].append(second_case(key))
    return exact(m)

def twin(key):
    """The one-case 0.1.0 preview twin of the second case (U8-0's proxy method): the
    milestone with case B's loads in place of its own (same model, E and G explicit)."""
    m = copy.deepcopy(base)
    c = m["model"]["load_cases"][0]
    c["primitive_loads"] = copy.deepcopy(SECOND[key])
    return m

wanted = sys.argv[3:] or ["item1", "item2", "item3", "item4"]
if "item1" in wanted:
    write("m3x", exact(base))
    copy_committed("n05", PS / "n05.request.json")
    copy_committed("n06", PS / "n06.request.json")
if "item2" in wanted:
    for key in ["axial", "anchor", "lateral"]:
        write(f"m3x_mix_{key}", mixed(key))
        write(f"m1_twin_{key}", twin(key))
if "item3" in wanted:
    write("m3l", legacy3(base))
if "item4" in wanted:
    copy_committed("fields", PS / "fields.request.json")
for p in sorted(OUT.glob("*.json")):
    print(p.name)
