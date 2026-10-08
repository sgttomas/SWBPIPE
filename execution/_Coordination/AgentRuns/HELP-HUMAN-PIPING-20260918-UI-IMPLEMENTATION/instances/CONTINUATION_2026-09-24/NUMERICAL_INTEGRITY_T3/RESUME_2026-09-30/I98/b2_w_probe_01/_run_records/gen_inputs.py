"""I98 B2-W: the probe's inputs (read-only on committed bytes; VENV).

Usage: python gen_inputs.py <builtin dir> <I86 inputs dir> <out dir> [names...]

<builtin dir> holds the probe's `zz_i98_dump_builtin` output: U8's committed helpers
(retained_facade_tests.rs), copied verbatim, and I81's case C. <I86 inputs dir> is
R/I86/b1_w_probe_01/_run_records/inputs (SW's accepted cap-maximal components).

Names (each writes <out>/<name>.json):
- cb1_ab_canonical  the same 256 loads in the readers' canonical nodal-term order.
- cb1_ac_canonical  informational: SW's A + C (non-cancelling), 256 loads, canonical order.
- cb1_a_halfb_canonical  informational: 1·A + 0.5·B (copy nets +M/2), 256 loads, canonical order.
- cause_milestone_reversed  a cause check: the milestone's three loads authored in reverse.
- cb1_ab      W-CB1's one-case proxy: SW's case A's request with one case `case:ab` carrying
              A's 128 loads then B's 128 loads, unnetted (RV114 N-7): 2l = 256.
- cb2_case_c  W-CB2's prediction: I81's case C, byte-identical to the builtin dump.
- cb3_a       W-CB3's selected operand alone: U8's L = 0 base, byte-identical to the dump.
- cb3_<v>_b   W-CB3 candidate v: the L = 0 base with case `case:b` holding only B_v's loads.
- cb3_<v>_ab  W-CB3 candidate v's combination proxy: case `case:ab`, the milestone's three
              moments then B_v's loads.
- r7_cb1_halfb  the W-CB1 alternative's base (cases A, B; combination 1·A + 0.5·B), ordinary route only.
- r7_cb1, r7_cb2, r7_cb3_<v>  R-7's in-domain multi-case requests (ordinary route only): two
              load cases and one mechanics combination A + B (factors 1, 1).
"""
import copy
import json
import os
import sys

P98 = "invented_t3_b2_w_probe_input_no_library_data"

# W-CB3 candidates: B_v's loads on the L = 0 base's isolated node N2 (fully restrained).
CB3 = {
    "v1": [{"id": "load:n2-y", "category": "concentrated_force", "target": {"type": "node", "node": "N2"}, "direction": "global_y",
            "dimension": "force", "magnitude": {"value": 1.0, "unit": "N"}, "provenance": P98}],
    "v2": [{"id": "load:n2-rx", "category": "concentrated_moment", "target": {"type": "node", "node": "N2"}, "direction": "rotation_x",
            "dimension": "moment", "magnitude": {"value": 1.0, "unit": "N*m"}, "provenance": P98}],
}


def load(path):
    with open(path, "rb") as f:
        return json.loads(f.read())


def dump(value, path):
    # Compact, key order kept, as serde_json writes; the probe hashes its own re-serialization.
    with open(path, "w", encoding="utf-8") as f:
        json.dump(value, f, ensure_ascii=False, indent=1)
        f.write("\n")


COMPONENTS = ["UX", "UY", "UZ", "RX", "RY", "RZ"]
DIRECTIONS = {**{c: c for c in COMPONENTS}, **dict(zip(["global_x", "global_y", "global_z", "rotation_x", "rotation_y", "rotation_z"], COMPONENTS))}


def canonical_key(model, loads):
    import struct
    nodes = {n["id"]: i for i, n in enumerate(model["nodes"])}
    ordinal = {id(l): i for i, l in enumerate(loads)}
    def key(l):
        bits = struct.pack(">d", float(l["magnitude"]["value"])).hex()
        return (nodes[l["target"]["node"]], COMPONENTS.index(DIRECTIONS[l["direction"]]), l["id"].encode(), bits, ordinal[id(l)])
    return key


def case(case_id, loads, template):
    c = copy.deepcopy(template)
    c["id"] = case_id
    c["primitive_loads"] = loads
    return c


def combination(a, b):
    return {"id": "combination:ab", "label": "I98 B2-W R-7 count: A + B", "basis": "mechanics",
            "terms": [{"load_case": a, "factor": 1.0}, {"load_case": b, "factor": 1.0}], "provenance": P98}


def main():
    builtin, i86, out = sys.argv[1:4]
    names = sys.argv[4:]
    a = load(os.path.join(i86, "i3_c1_case_a.json"))
    b = load(os.path.join(i86, "i3_c1_case_b.json"))
    two_a = load(os.path.join(builtin, "builtin_two_body_case_a.json"))
    two_b = load(os.path.join(builtin, "builtin_two_body_case_b.json"))
    l0 = load(os.path.join(builtin, "builtin_l0_isolated_node.json"))
    for name in names:
        if name == "cb1_ab":
            v = copy.deepcopy(a)
            ca, cb = a["model"]["load_cases"][0], b["model"]["load_cases"][0]
            assert {k: x for k, x in a["model"].items() if k != "load_cases"} == {k: x for k, x in b["model"].items() if k != "load_cases"}
            v["model"]["load_cases"] = [case("case:ab", ca["primitive_loads"] + cb["primitive_loads"], ca)]
        elif name in ("cb2_case_c", "cb3_a"):
            # Byte copies of the builtin dumps (I81's case C; U8's L = 0 base).
            src = "builtin_case_c.json" if name == "cb2_case_c" else "builtin_l0_isolated_node.json"
            with open(os.path.join(builtin, src), "rb") as f, open(os.path.join(out, name + ".json"), "wb") as g:
                g.write(f.read())
            print(name, "copied")
            continue
        elif name == "cb1_ab_canonical":
            # The same 256 loads, authored in the readers' canonical nodal-term order (G8:
            # node, component, source id bytes, value bits, ordinal), not A's then B's.
            v = copy.deepcopy(a)
            ca, cb = a["model"]["load_cases"][0], b["model"]["load_cases"][0]
            loads = ca["primitive_loads"] + cb["primitive_loads"]
            v["model"]["load_cases"] = [case("case:ab", sorted(loads, key=canonical_key(a["model"], loads)), ca)]
        elif name == "cb1_ac_canonical":
            # Informational (not the brief's W-CB1): SW's A + C, whose nets do not cancel
            # (copy k: (1 + 2^(k-3))·M), 256 loads in the canonical nodal-term order.
            c = load(os.path.join(i86, "i3_c1_case_c.json"))
            assert {k: x for k, x in a["model"].items() if k != "load_cases"} == {k: x for k, x in c["model"].items() if k != "load_cases"}
            v = copy.deepcopy(a)
            ca, cc = a["model"]["load_cases"][0], c["model"]["load_cases"][0]
            loads = ca["primitive_loads"] + cc["primitive_loads"]
            v["model"]["load_cases"] = [case("case:ac", sorted(loads, key=canonical_key(a["model"], loads)), ca)]
        elif name == "cb1_a_halfb_canonical":
            # Informational: the proxy of 1·A + 0.5·B (each B term times 0.5, exact in binary64;
            # copy nets +M/2), 256 loads in the canonical nodal-term order.
            v = copy.deepcopy(a)
            ca, cb = a["model"]["load_cases"][0], copy.deepcopy(b["model"]["load_cases"][0])
            for l in cb["primitive_loads"]:
                x = l["magnitude"]["value"]
                assert (x * 0.5) * 2.0 == x
                l["magnitude"]["value"] = x * 0.5
            loads = ca["primitive_loads"] + cb["primitive_loads"]
            v["model"]["load_cases"] = [case("case:a-half-b", sorted(loads, key=canonical_key(a["model"], loads)), ca)]
        elif name == "cause_milestone_reversed":
            # A cause check (not a witness): the milestone with its three loads authored in
            # reverse (RZ, RY, RX), out of the canonical nodal-term order.
            v = load(os.path.join(builtin, "builtin_milestone.json"))
            v["model"]["load_cases"][0]["primitive_loads"] = list(reversed(v["model"]["load_cases"][0]["primitive_loads"]))
        elif name.startswith("cb3_") and name.endswith("_ab"):
            key = name[4:-3]
            v = copy.deepcopy(l0)
            c0 = l0["model"]["load_cases"][0]
            v["model"]["load_cases"] = [case("case:ab", c0["primitive_loads"] + CB3[key], c0)]
        elif name.startswith("cb3_") and name.endswith("_b"):
            key = name[4:-2]
            v = copy.deepcopy(l0)
            c0 = l0["model"]["load_cases"][0]
            v["model"]["load_cases"] = [case("case:b", CB3[key], c0)]
        elif name == "r7_cb1":
            v = copy.deepcopy(a)
            v["model"]["load_cases"] = [copy.deepcopy(a["model"]["load_cases"][0]), copy.deepcopy(b["model"]["load_cases"][0])]
            v["model"]["combinations"] = [combination("case:a", "case:b")]
        elif name == "r7_cb1_halfb":
            # The informational W-CB1 alternative's base: cases A and B and 1·A + 0.5·B.
            v = copy.deepcopy(a)
            v["model"]["load_cases"] = [copy.deepcopy(a["model"]["load_cases"][0]), copy.deepcopy(b["model"]["load_cases"][0])]
            comb = combination("case:a", "case:b")
            comb["terms"][1]["factor"] = 0.5
            comb["label"] = "I98 B2-W R-7 count: A + 0.5 B"
            v["model"]["combinations"] = [comb]
        elif name == "r7_cb2":
            v = copy.deepcopy(two_a)
            c0 = two_a["model"]["load_cases"][0]
            v["model"]["load_cases"] = [case("case:a", c0["primitive_loads"], c0), case("case:b", two_b["model"]["load_cases"][0]["primitive_loads"], c0)]
            v["model"]["combinations"] = [combination("case:a", "case:b")]
        elif name.startswith("r7_cb3_"):
            key = name[7:]
            v = copy.deepcopy(l0)
            c0 = l0["model"]["load_cases"][0]
            v["model"]["load_cases"] = [case("case:a", c0["primitive_loads"], c0), case("case:b", CB3[key], c0)]
            v["model"]["combinations"] = [combination("case:a", "case:b")]
        else:
            raise SystemExit(f"unknown input {name}")
        dump(v, os.path.join(out, name + ".json"))
        print(name, "written")


if __name__ == "__main__":
    main()
