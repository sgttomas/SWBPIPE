"""Independent exact-rational reference for K-D5 on the converged active set.

DEL-04-01 Design kd5-nonlinear-active-set.md section 6. For each maintained
invented request this script forms every Euler-Bernoulli 12x12 frame element
itself, in exact rationals, from the request's binary64 leaves (Przemieniecki,
Theory of Matrix Structural Analysis; McGuire, Gallagher and Ziemian, Matrix
Structural Analysis), assembles, enumerates the nonlinear support states (each
sliding support in both slip senses, each derived normal in both signs) and
solves each state exactly. It records every state that satisfies all DEC-067
conditions strictly, with its exact published rows, and every state that
satisfies them only weakly (a zero margin).

The models are axis-aligned with exact lengths and axis-aligned plane
references, so every element entry and direction cosine is exact and the
exact-element system is the p-rounded intended system. Section leaves (A, I,
J) are the normalized binary64 values the product forms from the authored
diameter and wall; the PP consumer checks them bit for bit.

This test/reference code is not imported by the product and calls no product
code. Run it to verify the maintained oracles; `--write-oracle` replaces them.
"""
import argparse
import hashlib
import json
import math
from fractions import Fraction as F
from itertools import product
from pathlib import Path

HERE = Path(__file__).resolve().parent
REQUESTS = [
    "f1_request.json",
    "f1_explicit_request.json",
    "f1_mu0_request.json",
    "f2_request.json",
    "u1_request.json",
    "u1_spring_request.json",
    "u1_rx_request.json",
]
# Normalized binary64 section leaves of the authored (OD, t) the requests use,
# as the product forms them (checked bit for bit by the PP consumer).
LEAVES = {
    (0.168, 0.007): {
        "As_m2": 0.003540574920595698,
        "I_m4": 1.1493591335983782e-05,
        "J_m4": 2.2987182671967564e-05,
    },
}
DOFS = ["UX", "UY", "UZ", "RX", "RY", "RZ"]


def exact_axes(xi, xj, yref):
    d = [b - a for a, b in zip(xi, xj)]
    nonzero = [k for k in range(3) if d[k] != 0]
    assert len(nonzero) == 1, "reference members are axis-aligned"
    length = abs(d[nonzero[0]])
    x = [v / length for v in d]
    proj = sum(a * b for a, b in zip(yref, x))
    yc = [a - proj * b for a, b in zip(yref, x)]
    ynz = [k for k in range(3) if yc[k] != 0]
    assert len(ynz) == 1, "reference plane references are axis-aligned"
    y = [v / abs(yc[ynz[0]]) for v in yc]
    z = [x[1] * y[2] - x[2] * y[1], x[2] * y[0] - x[0] * y[2], x[0] * y[1] - x[1] * y[0]]
    return [x, y, z], length


def local_stiffness(length, A, I, J, E, G):
    L = length
    k = [[F(0)] * 12 for _ in range(12)]
    axial, torsion, ei = E * A / L, G * J / L, E * I
    v12, v6, v4, v2 = 12 * ei / L**3, 6 * ei / L**2, 4 * ei / L, 2 * ei / L
    for i, j, v in [(0, 6, axial), (3, 9, torsion)]:
        k[i][i] = k[j][j] = v
        k[i][j] = k[j][i] = -v
    bz = [[v12, v6, -v12, v6], [v6, v4, -v6, v2], [-v12, -v6, v12, -v6], [v6, v2, -v6, v4]]
    by = [[v12, -v6, -v12, -v6], [-v6, v4, v6, v2], [-v12, v6, v12, v6], [-v6, v2, v6, v4]]
    for ids, block in [([1, 5, 7, 11], bz), ([2, 4, 8, 10], by)]:
        for i in range(4):
            for j in range(4):
                k[ids[i]][ids[j]] = block[i][j]
    return k


def to_global(k, R):
    T = [[F(0)] * 12 for _ in range(12)]
    for b in range(4):
        for i in range(3):
            for j in range(3):
                T[3 * b + i][3 * b + j] = R[i][j]
    KT = [[sum(k[i][m] * T[m][j] for m in range(12)) for j in range(12)] for i in range(12)]
    return [[sum(T[m][i] * KT[m][j] for m in range(12)) for j in range(12)] for i in range(12)], T


def solve(A, b):
    n = len(b)
    aug = [list(row) + [rhs] for row, rhs in zip(A, b)]
    for col in range(n):
        pivot = max(range(col, n), key=lambda r: abs(aug[r][col]))
        aug[col], aug[pivot] = aug[pivot], aug[col]
        p = aug[col][col]
        if p == 0:
            return None
        aug[col] = [x / p for x in aug[col]]
        for row in range(n):
            if row != col and aug[row][col]:
                f = aug[row][col]
                aug[row] = [a - f * c for a, c in zip(aug[row], aug[col])]
    return [row[-1] for row in aug]


def bounds(x):
    """The binary64 neighbours (lo <= x <= hi), decided exactly."""
    f = float(x)
    lo = f if F(f) <= x else math.nextafter(f, -math.inf)
    hi = f if F(f) >= x else math.nextafter(f, math.inf)
    assert F(lo) <= x <= F(hi)
    return [lo, hi]


def sqrt_bounds(sq):
    """Binary64 bounds of sqrt(sq), established by exact squared comparisons."""
    if sq == 0:
        return [0.0, 0.0]
    f = math.sqrt(float(sq))
    lo, hi = f, f
    while F(lo) ** 2 > sq:
        lo = math.nextafter(lo, -math.inf)
    while F(hi) ** 2 < sq:
        hi = math.nextafter(hi, math.inf)
    assert F(lo) ** 2 <= sq <= F(hi) ** 2
    return [lo, hi]


class Model:
    def __init__(self, request):
        m = request["model"]
        self.nodes = [n["id"] for n in m["nodes"]]
        self.pos = {n["id"]: [F(float(n["position"][a])) for a in "xyz"] for n in m["nodes"]}
        mats = {mat["id"]: mat for mat in m["materials"]}
        self.n = 6 * len(self.nodes)
        self.K = [[F(0)] * self.n for _ in range(self.n)]
        self.members = []
        self.sections = {}
        for pipe in m["pipe_segments"]:
            od = pipe["section"]["outside_diameter"]["value"]
            t = pipe["section"]["wall_thickness"]["value"]
            leaves = LEAVES[(od, t)]
            self.sections[pipe["id"]] = leaves
            mat = mats[pipe["material"]]
            E = F(float(mat["elastic_modulus"]["value"]))
            G = F(float(mat["shear_modulus"]["value"]))
            A, I, J = (F(leaves[k]) for k in ("As_m2", "I_m4", "J_m4"))
            yref = [F(float(pipe["y_reference"][a])) for a in "xyz"]
            R, length = exact_axes(self.pos[pipe["from"]], self.pos[pipe["to"]], yref)
            kl = local_stiffness(length, A, I, J, E, G)
            kg, T = to_global(kl, R)
            i, j = self.nodes.index(pipe["from"]), self.nodes.index(pipe["to"])
            dofs = list(range(6 * i, 6 * i + 6)) + list(range(6 * j, 6 * j + 6))
            for a, ga in enumerate(dofs):
                for b, gb in enumerate(dofs):
                    self.K[ga][gb] += kg[a][b]
            self.members.append((pipe["id"], kg, T, length, dofs))
            self.E, self.G = E, G
        self.rigid = {}
        self.springs = []
        self.nonlinear = []
        self.supports = []
        for s in m["supports"]:
            node = self.nodes.index(s["node"])
            self.supports.append((s["id"], node))
            if s.get("nonlinear"):
                nl = s["nonlinear"]
                d = 6 * node + DOFS.index(nl["dof"])
                entry = {"id": s["id"], "dof": d, "behavior": nl["behavior"]}
                if nl["behavior"] == "one_way":
                    entry["positive"] = nl["active_when"] == "positive_reaction"
                elif nl["behavior"] == "friction":
                    entry["mu"] = F(float(nl["friction_coefficient"]["value"]))
                    if "normal_reaction" in nl:
                        entry["normal"] = F(float(nl["normal_reaction"]["value"]))
                    else:
                        src = nl["normal_reaction_source"]
                        src_node = self.nodes.index(next(x for x in m["supports"] if x["id"] == src["support_ref"])["node"])
                        entry["source"] = 6 * src_node + DOFS.index(src["dof"])
                else:
                    raise AssertionError("reference covers one-way and friction supports")
                self.nonlinear.append(entry)
            elif s.get("hanger", {}).get("stiffness"):
                st = s["hanger"]["stiffness"]
                d = 6 * node + DOFS.index(st["dof"])
                k = F(float(st["value"]["value"]))
                self.springs.append((s["id"], d, k))
                self.K[d][d] += k
            else:
                for r in s["restraints"]:
                    self.rigid.setdefault(s["id"], []).append(6 * node + DOFS.index(r))
        self.cases = m["load_cases"]

    def force(self, case):
        f = [F(0)] * self.n
        for load in case["primitive_loads"]:
            assert load["dimension"] == "force" and load["target"]["type"] == "node"
            node = self.nodes.index(load["target"]["node"])
            axis = {"global_x": 0, "global_y": 1, "global_z": 2}[load["direction"]]
            f[6 * node + axis] += F(float(load["magnitude"]["value"]))
        return f


def states_for(entry):
    if entry["behavior"] == "one_way":
        return [("active", None, None), ("inactive", None, None)]
    out = [("inactive", None, None), ("sticking", None, None)]
    for direction in (1, -1):
        for beta in ((1, -1) if "source" in entry and entry["mu"] != 0 else (None,)):
            out.append(("sliding", direction, beta))
    return out


def solve_state(model, f, combo):
    n = model.n
    constrained = {d: F(0) for dofs in model.rigid.values() for d in dofs}
    friction_rows = {}
    for entry, (state, direction, beta) in zip(model.nonlinear, combo):
        d = entry["dof"]
        if state in ("active", "sticking"):
            constrained[d] = F(0)
        elif state == "sliding" and entry["mu"] != 0:
            friction_rows[d] = (entry, direction, beta)
    free = [i for i in range(n) if i not in constrained]
    A, b = [], []
    for i in free:
        row = [model.K[i][j] for j in free]
        rhs = f[i] - sum(model.K[i][j] * v for j, v in constrained.items())
        if i in friction_rows:
            entry, direction, beta = friction_rows[i]
            if "source" in entry:
                # F = -dir*mu*beta*(K_n u - f_n), moved to the left-hand side.
                c = -direction * entry["mu"] * beta
                src = entry["source"]
                row = [r - c * model.K[src][j] for r, j in zip(row, free)]
                rhs = rhs + c * (-f[src] + sum(model.K[src][j] * v for j, v in constrained.items()))
            else:
                rhs = rhs + (-direction * entry["mu"] * abs(entry["normal"]))
        A.append(row)
        b.append(rhs)
    x = solve(A, b)
    if x is None:
        return None
    u = [F(0)] * n
    for d, v in constrained.items():
        u[d] = v
    for i, v in zip(free, x):
        u[i] = v
    r = [sum(model.K[i][j] * u[j] for j in range(n)) - f[i] for i in range(n)]
    return u, r


def classify(model, combo, u, r):
    """'strict', 'weak' or None for one state against every DEC-067 condition."""
    weak = False
    for entry, (state, direction, beta) in zip(model.nonlinear, combo):
        d = entry["dof"]
        if entry["behavior"] == "one_way":
            s = 1 if entry["positive"] else -1
            value = s * r[d] if state == "active" else s * u[d]
            if value < 0:
                return None
            weak |= value == 0
            continue
        normal = abs(r[entry["source"]]) if "source" in entry else entry["normal"]
        if state == "inactive":
            if normal > 0:
                return None
            # A derived inactive support is weakly active (R_n = 0 exactly);
            # an explicit N <= 0 is an input decision.
            weak |= "source" in entry
        elif state == "sticking":
            margin = entry["mu"] * normal - abs(r[d])
            if normal <= 0 or margin < 0:
                return None
            weak |= margin == 0
        else:
            if normal <= 0 or direction * u[d] < 0:
                return None
            if "source" in entry and entry["mu"] != 0 and beta * r[entry["source"]] < 0:
                return None
            weak |= direction * u[d] == 0
            if "source" in entry and entry["mu"] != 0:
                weak |= r[entry["source"]] == 0
    return "weak" if weak else "strict"


def published(model, combo, u, r, f):
    """Exact published rows of one state: member groups, support groups, signed slots."""
    groups, signed = {}, {}
    for pipe_id, kg, T, length, dofs in model.members:
        ue = [u[d] for d in dofs]
        fg = [sum(kg[a][b] * ue[b] for b in range(12)) for a in range(12)]
        vl = [sum(T[a][b] * fg[b] for b in range(12)) for a in range(12)]
        locations = {"end_i": vl[:6], "end_j": vl[6:]}
        for name, frac in zip(["cut_i", "quarter_1", "midspan", "quarter_3", "cut_j"], [F(0), F(1, 4), F(1, 2), F(3, 4), F(1)]):
            x = length * frac
            v = [-z for z in vl[:6]]
            v[4] -= x * vl[2]
            v[5] += x * vl[1]
            locations[name] = v
        for loc, v in locations.items():
            for kind, rows in [("axial", v[0:1]), ("shear", v[1:3]), ("torsion", v[3:4]), ("bending", v[4:6])]:
                groups.setdefault(pipe_id, {}).setdefault(loc, {})[kind] = sqrt_bounds(sum(x * x for x in rows))
    slots = {}
    for entry, (state, direction, beta) in zip(model.nonlinear, combo):
        d = entry["dof"]
        if state in ("active", "sticking"):
            value = r[d]
        elif state == "sliding" and entry["mu"] != 0:
            value = r[d]  # = the applied friction force at the free row
        else:
            value = F(0)
        slots[entry["id"]] = (d, value)
        signed[entry["id"]] = {"slot": DOFS[d % 6], "value": bounds(value)}
    for support_id, node in model.supports:
        rows = [F(0)] * 6
        for d in model.rigid.get(support_id, []):
            rows[d % 6] = r[d]
        for sid, d, k in model.springs:
            if sid == support_id:
                rows[d % 6] = -k * u[d]
        if support_id in slots:
            d, value = slots[support_id]
            rows[d % 6] = value
        force_sq = sum(x * x for x in rows[:3])
        moment_sq = sum(x * x for x in rows[3:])
        groups[support_id] = {"node": {
            "support_force": sqrt_bounds(force_sq),
            "support_moment": sqrt_bounds(moment_sq),
            "support_force_magnitude": sqrt_bounds(force_sq),
            "support_moment_magnitude": sqrt_bounds(moment_sq),
        }}
        signed.setdefault(support_id, {})["components"] = [bounds(x) for x in rows]
    return groups, signed


def reference(name):
    request_bytes = (HERE / name).read_bytes()
    request = json.loads(request_bytes)
    model = Model(request)
    cases = {}
    for case in model.cases:
        f = model.force(case)
        qualifying = []
        for combo in product(*[states_for(e) for e in model.nonlinear]):
            solved = solve_state(model, f, combo)
            if solved is None:
                continue
            u, r = solved
            kind = classify(model, combo, u, r)
            if kind is None:
                continue
            groups, signed = published(model, combo, u, r, f)
            states = {}
            for entry, (state, direction, beta) in zip(model.nonlinear, combo):
                states[entry["id"]] = state
            record = {"standing": kind, "states": states, "groups": groups, "signed": signed}
            # The derived coupling gain c*g of a sliding derived-normal support.
            for entry, (state, direction, beta) in zip(model.nonlinear, combo):
                if state == "sliding" and "source" in entry and entry["mu"] != 0:
                    record.setdefault("coupling", {})[entry["id"]] = coupling_gain(model, combo, entry, direction, beta)
            qualifying.append(record)
        strict = [q for q in qualifying if q["standing"] == "strict"]
        cases[case["id"]] = {"strict_states": strict, "weak_states": [q["states"] for q in qualifying if q["standing"] == "weak"]}
    return {
        "request": name,
        "request_sha256": hashlib.sha256(request_bytes).hexdigest(),
        "basis": "exact rational solve of independently formed Euler-Bernoulli frames, every support state enumerated",
        "normalized_sections": model.sections,
        "elastic_modulus_pa": float(model.E),
        "shear_modulus_pa": float(model.G),
        "cases": cases,
    }


def coupling_gain(model, combo, entry, direction, beta):
    """c*g for a sliding derived-normal support: g = dR_n/dF of the symmetric system."""
    others = [(s, dr, bt) if e is not entry else ("free", None, None) for e, (s, dr, bt) in zip(model.nonlinear, combo)]
    n = model.n
    constrained = {d: F(0) for dofs in model.rigid.values() for d in dofs}
    for e, (state, _, _) in zip(model.nonlinear, others):
        if state in ("active", "sticking"):
            constrained[e["dof"]] = F(0)
    free = [i for i in range(n) if i not in constrained]
    b = [F(1) if i == entry["dof"] else F(0) for i in free]
    z = solve([[model.K[i][j] for j in free] for i in free], b)
    zf = dict(zip(free, z))
    g = sum(model.K[entry["source"]][j] * v for j, v in zf.items())
    c = -direction * entry["mu"] * beta
    return float(c * g)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--write-oracle", action="store_true", help="Replace the maintained oracles.")
    args = parser.parse_args()
    for name in REQUESTS:
        oracle = reference(name)
        path = HERE / name.replace("_request.json", "_oracle.json")
        encoded = json.dumps(oracle, indent=1, sort_keys=True) + "\n"
        if args.write_oracle:
            path.write_text(encoded)
        else:
            assert json.loads(path.read_text()) == json.loads(encoded), f"{path.name} differs from the exact reference"
        print(f"{name}: sha256={oracle['request_sha256']}; cases=" + ", ".join(
            f"{cid}:{len(c['strict_states'])} strict/{len(c['weak_states'])} weak" for cid, c in oracle["cases"].items()))


if __name__ == "__main__":
    main()
