"""I86 B1-SW: the probe's input generator (read-only on committed bytes; writes JSON files).

Usage: python3 gen_inputs.py <builtin_dir> <milestone_fixture> <out_dir> <variant>...

Variants:
  a:<s>          W2b's committed input with every ring section scaled by s (OD and wall).
  b:<k>          W2b with N0's rotation about X released to one scalar spring of k N*m/rad:
                 S0 restrains UX,UY,UZ,RY,RZ rigidly; S31 (N31's UY spring) becomes the
                 RX spring at N0. Everything else is W2b's. (As built, S0 has no family, so
                 the product maps it to Guide and refuses RY and RZ: SUPPORT_INPUT_INVALID.)
  b2:<k>         b with S0's family "anchor" (the corrected construction).
  c:<name>       Item 2's milestone-replication inputs (see build_c).
  i3:<name>      Item 3's components and their three-case assembly (see build_i3).
The builtin directory holds the committed Values written by the probe's
`zz_i86_dump_builtin` (cap_maximal, w2b, w2, milestone), so (a) and (b) start from W2b's
exact committed Value.
"""
import copy
import json
import math
import sys
from fractions import Fraction

PROV = "invented_t3_b1_sw_probe_input_no_library_data"


def text(prefix, n):
    s = prefix
    while len(s) < n:
        s += "x"
    return s[:n]


def dump(path, value):
    with open(path, "w") as f:
        json.dump(value, f, indent=1, sort_keys=True, allow_nan=False)
        f.write("\n")


# ---- exact splits -----------------------------------------------------------------------

def mantissa(v):
    """v = M * 2**E exactly, with M an integer, |M| < 2**53."""
    fr = Fraction(v)
    m, e = math.frexp(v)
    M = int(m * (1 << 53))
    E = e - 53
    assert Fraction(M) * Fraction(2) ** E == fr, v
    return M, E


def split_exact(v, weights):
    """Split v into len(weights) nonzero parts proportional to `weights` (integers), all
    integer multiples of 2**E whose integer sum is M: every partial sum, in any order, is an
    integer multiple of 2**E of magnitude <= |M| < 2**53, so binary64 sums them exactly."""
    M, E = mantissa(v)
    sign = -1 if M < 0 else 1
    M = abs(M)
    total = sum(weights)
    ints = [M * w // total for w in weights]
    ints[-1] += M - sum(ints)
    assert all(i > 0 for i in ints) and sum(ints) == M
    parts = [sign * math.ldexp(i, E) for i in ints]
    assert sum(Fraction(p) for p in parts) == Fraction(v)
    acc = 0.0
    for p in parts:
        acc += p
    assert acc == v
    acc = 0.0
    for p in reversed(parts):
        acc += p
    assert acc == v
    return parts


# ---- item 1 -----------------------------------------------------------------------------

def variant_a(w2b, s):
    raw = copy.deepcopy(w2b)
    for pipe in raw["model"]["pipe_segments"]:
        sec = pipe["section"]
        sec["outside_diameter"]["value"] = sec["outside_diameter"]["value"] * s
        sec["wall_thickness"]["value"] = sec["wall_thickness"]["value"] * s
    return raw


def variant_b(w2b, k, anchor=False):
    raw = copy.deepcopy(w2b)
    sup = raw["model"]["supports"]
    assert sup[0]["node"] == "N0" and "stiffness" not in sup[0]
    sup[0]["restraints"] = ["UX", "UY", "UZ", "RY", "RZ"]
    if anchor:
        # b2: without a family, a support with fewer than six restraints maps to Guide,
        # which refuses rotational restraints (SUPPORT_INPUT_INVALID); "anchor" allows them.
        sup[0]["family"] = "anchor"
    assert sup[31]["node"] == "N31" and sup[31]["family"] == "spring"
    sup[31] = {"id": "S31", "node": "N0", "family": "spring", "restraints": ["RX"],
               "stiffness": {"dof": "RX", "value": {"value": float(k), "unit": "N*m/rad"}},
               "provenance": sup[31]["provenance"]}
    return raw


# ---- item 2 -----------------------------------------------------------------------------

def perpendicular_reference(d):
    for ref in ((0.0, 0.0, 1.0), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0)):
        c = (d[1] * ref[2] - d[2] * ref[1], d[2] * ref[0] - d[0] * ref[2], d[0] * ref[1] - d[1] * ref[0])
        n = math.sqrt(sum(x * x for x in c))
        if n > 0.5 * math.sqrt(sum(x * x for x in d)):
            return {"x": ref[0], "y": ref[1], "z": ref[2]}
    raise AssertionError(d)


def filler_bodies(prov):
    """Four anchored, connected, unloaded bodies: 18 nodes, 25 members, 4 supports.
    Body 0: 5 nodes, K5 (10 members); body 1: 5 nodes, 6 members; body 2: 4 nodes, K4 (6);
    body 3: 4 nodes, a chain (3). Each body's first node is fully restrained (one support,
    six rigid restraints, no family, as L = 0's base)."""
    shapes = [
        [(0, 0, 0), (1, 0, 0), (0, 1, 0), (0, 0, 1), (1, 1, 1)],
        [(0, 0, 0), (1, 0, 0), (1, 1, 0), (0, 1, 0), (0.5, 0.5, 1)],
        [(0, 0, 0), (1, 0, 0), (0, 1, 0), (0, 0, 1)],
        [(0, 0, 0), (1, 0, 0), (1, 1, 0), (1, 1, 1)],
    ]
    edges = [
        [(i, j) for i in range(5) for j in range(i + 1, 5)],
        [(0, 1), (1, 2), (2, 3), (3, 0), (0, 4), (2, 4)],
        [(i, j) for i in range(4) for j in range(i + 1, 4)],
        [(0, 1), (1, 2), (2, 3)],
    ]
    nodes, pipes, supports = [], [], []
    for b, (shape, es) in enumerate(zip(shapes, edges)):
        origin = (100.0 + 10.0 * b, 50.0, 0.0)
        ids = []
        for i, p in enumerate(shape):
            nid = f"F{b}:N{i}"
            ids.append(nid)
            nodes.append({"id": nid, "position": {"x": origin[0] + float(p[0]), "y": origin[1] + float(p[1]), "z": origin[2] + float(p[2])}, "provenance": prov})
        for (i, j) in es:
            d = tuple(float(shape[j][a] - shape[i][a]) for a in range(3))
            pipes.append({"id": f"F{b}:M{i}{j}", "from": ids[i], "to": ids[j], "material": "mat:1", "y_reference": perpendicular_reference(d),
                          "section": {"outside_diameter": {"value": 0.2, "unit": "m"}, "wall_thickness": {"value": 0.01, "unit": "m"}}, "provenance": prov})
        supports.append({"id": f"F{b}:anchor", "node": ids[0], "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": prov})
    assert (len(nodes), len(pipes), len(supports)) == (18, 25, 4)
    return nodes, pipes, supports


def materials(prov, points):
    pts = [{"id": f"T{i}", "provenance": prov} for i in range(points)]
    out = []
    for i in range(4):
        m = {"id": f"mat:{i}", "elastic_modulus": {"value": 200000000000.0, "unit": "Pa"}, "shear_modulus": {"value": 80000000000.0, "unit": "Pa"}, "provenance": prov}
        if points:
            m["temperature_points"] = copy.deepcopy(pts)
        out.append(m)
    return out


def copies_loads(ms, case_prefix, copies, total, scale_of, weights_of):
    """`total` moments over `copies` milestone copies, three rotational DOFs at each copy's N1.
    Copy k's net on each DOF is exactly scale_of(k) times the milestone's (scale a power of two
    or its negative). Slot (k, dof) gets n parts with exact integer-mantissa splits."""
    loads = ms["model"]["load_cases"][0]["primitive_loads"]
    base = {l["direction"]: l["magnitude"]["value"] for l in loads}
    slots = [(k, d) for k in range(copies) for d in ("RX", "RY", "RZ")]
    per = [total // len(slots)] * len(slots)
    for i in range(total - sum(per)):
        per[i] += 1
    out = []
    for (k, d), n in zip(slots, per):
        net = base[d] * scale_of(k)
        assert Fraction(net) == Fraction(base[d]) * Fraction(scale_of(k))
        for j, part in enumerate(split_exact(net, weights_of(k, d, n))):
            out.append({"id": f"{case_prefix}C{k}:{d}:{j}", "category": "concentrated_moment", "target": {"type": "node", "node": f"C{k}:N1"},
                        "direction": d, "magnitude": {"value": part, "unit": "N*m"}, "dimension": "moment", "provenance": PROV})
    assert len(out) == total
    return out


def build_model(ms, copies, points, case_loads, project_len=128):
    """The milestone body replicated `copies` times (disconnected, unchanged geometry and
    supports, offset by whole metres along X), plus the filler, at D1's count caps."""
    m = ms["model"]
    nodes, pipes, supports = [], [], []
    for k in range(copies):
        dx = 10.0 * k
        for n in m["nodes"]:
            p = n["position"]
            nodes.append({"id": f"C{k}:{n['id']}", "position": {"x": p["x"] + dx, "y": p["y"], "z": p["z"]}, "provenance": n["provenance"]})
        for pipe in m["pipe_segments"]:
            q = copy.deepcopy(pipe)
            q["id"], q["from"], q["to"], q["material"] = f"C{k}:{pipe['id']}", f"C{k}:{pipe['from']}", f"C{k}:{pipe['to']}", "mat:0"
            pipes.append(q)
        for s in m["supports"]:
            q = copy.deepcopy(s)
            q["id"], q["node"] = f"C{k}:{s['id']}", f"C{k}:{s['node']}"
            supports.append(q)
    fn, fp, fs = filler_bodies(PROV)
    nodes += fn
    pipes += fp
    supports += fs
    assert (len(nodes), len(pipes), len(supports)) == (32, 32, 32), (len(nodes), len(pipes), len(supports))
    model = {
        "schema_version": m["schema_version"],
        "document_kind": m["document_kind"],
        "analysis_status": copy.deepcopy(m["analysis_status"]),
        "project": {"id": text("invented:t3-b1-sw:", project_len), "units": copy.deepcopy(m["project"]["units"])},
        "nodes": nodes,
        "pipe_segments": pipes,
        "materials": materials(PROV, points),
        "supports": supports,
        "load_cases": case_loads,
        "combinations": [],
    }
    return {"model": model, "materials": materials(PROV, points)}


def one_case(case_id, loads):
    return {"id": case_id, "label": "I86 B1-SW probe case", "kind": "primitive_user_load", "primitive_loads": loads, "provenance": PROV}


def equal(k, d, n):
    return [1] * n


def build_c(ms, name):
    if name == "c1":
        # 7 copies (the support cap), 128 moments with each copy's net exactly the milestone's,
        # 4 + 4 materials with 16 temperature points, a 128-byte project id.
        loads = copies_loads(ms, "", 7, 128, lambda k: 1.0, equal)
        return build_model(ms, 7, 16, [one_case("case", loads)])
    if name == "c2":
        # c1 without temperature points (isolates them if c1 does not publish).
        loads = copies_loads(ms, "", 7, 128, lambda k: 1.0, equal)
        return build_model(ms, 7, 0, [one_case("case", loads)])
    raise SystemExit(f"unknown c variant {name}")


def escape_every_provenance(v):
    if isinstance(v, dict):
        for k, x in v.items():
            if k == "provenance" and isinstance(x, str):
                v[k] = x + ' q"b\\'
            else:
                escape_every_provenance(x)
    elif isinstance(v, list):
        for x in v:
            escape_every_provenance(x)


def depth_16_value():
    deep = 1
    for _ in range(14):
        deep = [deep]
    return deep


def stressed(raw):
    raw = copy.deepcopy(raw)
    escape_every_provenance(raw)
    raw["model"]["unknown_depth_witness"] = depth_16_value()
    return raw


def i3_sets(ms, copies):
    """Three distinct 128-moment sets on item 2's model:
    A: each copy's net exactly the milestone's, equal parts;
    B: each copy's net exactly minus the milestone's, parts weighted 1..n;
    C: copy k's net exactly 2**(k-3) times the milestone's, parts weighted n..1 plus one."""
    a = copies_loads(ms, "a:", copies, 128, lambda k: 1.0, equal)
    b = copies_loads(ms, "b:", copies, 128, lambda k: -1.0, lambda k, d, n: list(range(1, n + 1)))
    c = copies_loads(ms, "c:", copies, 128, lambda k: math.ldexp(1.0, k - 3), lambda k, d, n: [n - i + 1 for i in range(n)])
    return {"case:a": a, "case:b": b, "case:c": c}


def build_i3(ms, name, out):
    """i3:<base> writes the three stressed one-case components and the stressed three-case
    assembly, all on the c-variant <base>'s model."""
    points = 16 if name == "c1" else 0
    sets = i3_sets(ms, 7)
    written = []
    for cid, loads in sets.items():
        raw = stressed(build_model(ms, 7, points, [one_case(cid, loads)]))
        path = f"{out}/i3_{name}_{cid.replace(':', '_')}.json"
        dump(path, raw)
        written.append(path)
    raw = stressed(build_model(ms, 7, points, [one_case(cid, loads) for cid, loads in sets.items()]))
    path = f"{out}/i3_{name}_three_case.json"
    dump(path, raw)
    written.append(path)
    return written


def main():
    builtin, fixture, out = sys.argv[1:4]
    with open(fixture) as f:
        ms = json.load(f)
    w2b = None
    for v in sys.argv[4:]:
        kind, arg = v.split(":", 1)
        if kind in ("a", "b", "b2") and w2b is None:
            with open(f"{builtin}/builtin_w2b.json") as f:
                w2b = json.load(f)
        if kind == "a":
            s = float(arg)
            dump(f"{out}/a_s{arg}.json", variant_a(w2b, s))
            print(f"{out}/a_s{arg}.json")
        elif kind == "b":
            dump(f"{out}/b_k{arg}.json", variant_b(w2b, float(arg)))
            print(f"{out}/b_k{arg}.json")
        elif kind == "b2":
            dump(f"{out}/b2_k{arg}.json", variant_b(w2b, float(arg), anchor=True))
            print(f"{out}/b2_k{arg}.json")
        elif kind == "c":
            dump(f"{out}/{arg}.json", build_c(ms, arg))
            print(f"{out}/{arg}.json")
        elif kind == "i3":
            for p in build_i3(ms, arg, out):
                print(p)
        else:
            raise SystemExit(f"unknown variant {v}")


if __name__ == "__main__":
    main()
