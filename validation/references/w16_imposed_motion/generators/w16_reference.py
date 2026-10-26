#!/usr/bin/env python3
"""W-16 independent references: an anchor (nozzle) movement on an L frame.

Standard-library Python 3 only. This generator imports, reads and runs no
product code and reads no file other than the ones it writes. Every expected
value is derived here in exact rational arithmetic:

  W16-MOTION          the nozzle anchor D moves by three translations and one
                      rotation; no load and no temperature change;
  W16-THERMAL         the same frame under a uniform temperature rise, D held;
  W16-MOTION-THERMAL  both together, in one case.

Route 1 is a six-redundant force method (complementary energy). Release the
moved anchor D, so the frame is a cantilever from the fixed anchor A. Each
redundant R_k is one action of the support on the structure at D (Fx, Fy, Fz,
Mx, My, Mz, moments about D). From statics on the released frame,

  F_ij = sum over members of the integral of
         (n_i n_j / EA + t_i t_j / GJ + my_i my_j / EI + mz_i mz_j / EI) ds,

and compatibility at D gives R = F^-1 (g - d_T): g is the imposed motion of D
and d_T the free thermal motion of the released D. The anchor A's actions
follow by equilibrium, the corner's motion by member integration (unit-load
method on A->C), and the section actions by statics. The integrands are
quadratic in s, so Simpson's rule is exact.

Route 2 is an independently written three-dimensional direct-stiffness solve
of the same frame: Euler-Bernoulli 12x12 member stiffness with St Venant
torsion, the held-member thermal end actions EA*eps, and prescribed-DOF
partitioning K_ff u_f = f_f - K_fc g. Every published reaction, nodal motion
and member end action is compared with route 1 exactly.

Every result is q * pi**k with q rational: forces and moments carry k = 1 (EA,
EI and GJ are rational multiples of pi), translations and rotations k = 0.
Decimals are printed to 40 significant digits from a Machin value of pi;
binary64 values are the correctly rounded nearest doubles, checked by rounding
with lower and upper rational bounds of pi.

Modes:
  python3 -I w16_reference.py           check: rebuild every output in memory and
                                         compare byte for byte with the committed
                                         files, SHA256SUMS included
  python3 -I w16_reference.py --write   write the outputs (intentional changes only)
"""
import argparse
import hashlib
import json
import sys
from decimal import Decimal, getcontext
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent
OUT = HERE.parent
FIXED_FILES = ("README.md", "generators/w16_reference.py")

# ----------------------------------------------------------------------------- pi
getcontext().prec = 100


def _atan_inv(x):
    x = Decimal(x)
    total, term, n, sign = Decimal(0), 1 / x, 1, 1
    x2 = x * x
    eps = Decimal(10) ** -(getcontext().prec - 5)
    while True:
        delta = term / n
        if delta < eps:
            break
        total += sign * delta
        term /= x2
        n += 2
        sign = -sign
    return total


PI_DEC = 16 * _atan_inv(5) - 4 * _atan_inv(239)  # Machin, ~95 correct digits
_PI_90 = F(str(PI_DEC)[:92])  # "3." + 90 digits, truncated
PI_LO, PI_HI = _PI_90, _PI_90 + F(1, 10 ** 90)
assert PI_LO < F(314159265358979323846, 10 ** 20) + F(1, 10 ** 20) and PI_HI > F(314159265358979323846, 10 ** 20)


def binary64(q, k):
    """Nearest double to q * pi**k, the same for both rational bounds of pi."""
    if k == 0:
        return float(q)
    lo, hi = float(q * PI_LO), float(q * PI_HI)
    if lo != hi:
        raise SystemExit(f"binary64 rounding of {q}*pi is not decided by the pi bounds")
    return lo


def decimal40(q, k):
    if q == 0:
        return "0"
    getcontext().prec = 100
    v = Decimal(q.numerator) / Decimal(q.denominator) * (PI_DEC if k else 1)
    return format(v.normalize().__format__(".39e"))


def exact_text(q, k):
    s = str(q)
    return s if k == 0 or q == 0 else f"{s}*pi"


def record(q, k):
    q = F(q)
    return {"exact": exact_text(q, k), "decimal": decimal40(q, k), "value": binary64(q, k)}


# ----------------------------------------------------------------------------- inputs (invented)
OD, WALL = F(25, 100), F(1, 100)  # m
ID = OD - 2 * WALL  # 0.23 m
E = F(200 * 10 ** 9)  # Pa
NU = F(1, 4)  # exact route: G = E / (2 (1 + nu)) = 80 GPa
G = E / (2 * (1 + NU))  # preview route input; the same value
assert G == 80 * 10 ** 9
AREA_P = (OD ** 2 - ID ** 2) / 4  # A / pi
INERTIA_P = (OD ** 4 - ID ** 4) / 64  # I / pi (Iy = Iz)
POLAR_P = 2 * INERTIA_P  # J / pi, the annulus polar moment
EA_P, EI_P, GJ_P = E * AREA_P, E * INERTIA_P, G * POLAR_P  # / pi
ALPHA, DELTA_T = F(12, 10 ** 6), F(80)
EPS_T = ALPHA * DELTA_T  # free thermal strain 6/6250
NODES = {"node:a": (F(0), F(0), F(0)), "node:c": (F(5), F(0), F(0)), "node:d": (F(5), F(4), F(0))}
PIPES = (("pipe:a-c", "node:a", "node:c", (F(0), F(1), F(0))),
         ("pipe:c-d", "node:c", "node:d", (F(-1), F(0), F(0))))
FIXED, MOVED = "node:a", "node:d"
SUPPORTS = {"support:a": FIXED, "support:d": MOVED}
# The nozzle movement of D: three translations (m) and one rotation (rad); the
# other two rotations are held at zero.
MOTION = {"UX": F(3, 1000), "UY": F(-5, 1000), "UZ": F(8, 1000), "RY": F(1, 500)}
DOFS = ("UX", "UY", "UZ", "RX", "RY", "RZ")

COMPONENTS_F = ("Fx", "Fy", "Fz", "Mx", "My", "Mz")
COMPONENTS_U = ("ux", "uy", "uz", "rx", "ry", "rz")
SECTION_COMPONENTS = ("axial_force", "shear_force_y", "shear_force_z",
                      "torsional_moment", "bending_moment_y", "bending_moment_z")
STATIONS = (F(0), F(1, 4), F(1, 2), F(3, 4), F(1))
STATION_NAMES = ("0", "1/4", "1/2", "3/4", "1")

CROSS = []


def cross_check(name, a, b):
    CROSS.append({"check": name, "agrees": a == b})
    if a != b:
        print(f"DISAGREES {name}: {a} != {b}")


# ----------------------------------------------------------------------------- vectors
def add(a, b):
    return tuple(x + y for x, y in zip(a, b))


def sub(a, b):
    return tuple(x - y for x, y in zip(a, b))


def scale(a, k):
    return tuple(x * k for x in a)


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


def cross(a, b):
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def unit(k):
    return tuple(F(1) if i == k else F(0) for i in range(3))


ZERO3 = (F(0), F(0), F(0))


def member_axes(i, j, y_ref):
    """Local x from i to j; local y the y reference less its x part; z = x * y."""
    d = sub(NODES[j], NODES[i])
    length2 = dot(d, d)
    # Members lie along a global axis, so the length is rational.
    length = sum(abs(c) for c in d)
    assert length * length == length2
    ex = scale(d, 1 / length)
    y = sub(y_ref, scale(ex, dot(y_ref, ex)))
    assert dot(y, y) == 1
    ez = cross(ex, y)
    return length, (ex, y, ez)


# ----------------------------------------------------------------------------- linear algebra
def solve(a, y):
    n = len(a)
    m = [list(row) + [y[i]] for i, row in enumerate(a)]
    for c in range(n):
        p = next(r for r in range(c, n) if m[r][c] != 0)
        m[c], m[p] = m[p], m[c]
        for r in range(n):
            if r != c and m[r][c] != 0:
                k = m[r][c] / m[c][c]
                m[r] = [x - k * z for x, z in zip(m[r], m[c])]
    return [m[i][n] / m[i][i] for i in range(n)]


def matvec(a, x):
    return [sum(r * v for r, v in zip(row, x)) for row in a]


def transpose(a):
    return [list(r) for r in zip(*a)]


def matmul(a, b):
    bt = transpose(b)
    return [[sum(x * y for x, y in zip(r, col)) for col in bt] for r in a]


# ----------------------------------------------------------------------------- route 1: force method
def section_global(point, force, moment, at):
    """Action on the +x face of the i-side portion at `point`, from the j-side
    portion's equilibrium under (force, moment about `at`) applied beyond it."""
    return force, add(moment, cross(sub(at, point), force))


def local(axes, force, moment):
    ex, ey, ez = axes
    return {"axial_force": dot(force, ex), "shear_force_y": dot(force, ey), "shear_force_z": dot(force, ez),
            "torsional_moment": dot(moment, ex), "bending_moment_y": dot(moment, ey),
            "bending_moment_z": dot(moment, ez)}


def energy_density(a, b):
    """The complementary-energy cross term of two local section states, / pi."""
    return (a["axial_force"] * b["axial_force"] / EA_P + a["torsional_moment"] * b["torsional_moment"] / GJ_P
            + a["bending_moment_y"] * b["bending_moment_y"] / EI_P
            + a["bending_moment_z"] * b["bending_moment_z"] / EI_P)


def integrate(pipes, state_a, state_b):
    """Sum over `pipes` of the integral of energy_density over the length;
    each state maps (pipe, point) to a local section state. Simpson is exact
    for these quadratic integrands."""
    total = F(0)
    for pid, i, j, y_ref in pipes:
        length, axes = member_axes(i, j, y_ref)
        values = []
        for s in (F(0), F(1, 2), F(1)):
            point = add(NODES[i], scale(sub(NODES[j], NODES[i]), s))
            values.append(energy_density(state_a(pid, axes, point), state_b(pid, axes, point)))
        total += length / 6 * (values[0] + 4 * values[1] + values[2])
    return total


def load_state(force, moment, at):
    return lambda pid, axes, point: local(axes, *section_global(point, force, moment, at))


def redundant_unit(k):
    """Unit redundant k at D: a force (k < 3) or a moment (k >= 3)."""
    force = unit(k) if k < 3 else ZERO3
    moment = unit(k - 3) if k >= 3 else ZERO3
    return force, moment


def flexibility():
    states = [load_state(*redundant_unit(k), NODES[MOVED]) for k in range(6)]
    return [[integrate(PIPES, states[i], states[j]) for j in range(6)] for i in range(6)]


def imposed_vector(motion):
    return [motion.get(d, F(0)) for d in DOFS]


def thermal_free_motion(eps, frm, to):
    """Free thermal motion of `to` relative to `frm` on the released tree."""
    return list(scale(sub(NODES[to], NODES[frm]), eps)) + [F(0)] * 3


def force_method(case_id, motion, thermal):
    eps = EPS_T if thermal else F(0)
    g = imposed_vector(motion)
    d_t = thermal_free_motion(eps, FIXED, MOVED)
    flex = flexibility()
    redundants = solve(flex, [g[i] - d_t[i] for i in range(6)])  # / pi
    force, moment = tuple(redundants[:3]), tuple(redundants[3:])
    at_d = NODES[MOVED]
    # The fixed anchor A by equilibrium of the whole frame.
    force_a = scale(force, -1)
    moment_a = scale(add(moment, cross(sub(at_d, NODES[FIXED]), force)), -1)
    reactions = {"support:a": dict(zip(COMPONENTS_F, force_a + moment_a)),
                 "support:d": dict(zip(COMPONENTS_F, force + moment))}
    actual = load_state(force, moment, at_d)
    # The corner C by member integration on A->C (unit-load method).
    a_c = [p for p in PIPES if p[0] == "pipe:a-c"]
    c_t = thermal_free_motion(eps, FIXED, "node:c")
    corner = [c_t[k] + integrate(a_c, load_state(*redundant_unit(k), NODES["node:c"]), actual) for k in range(6)]
    # D from C's motion carried rigidly plus member C->D as a cantilever from C:
    # compatibility self-check.
    c_d = [p for p in PIPES if p[0] == "pipe:c-d"]
    rel = [integrate(c_d, load_state(*redundant_unit(k), at_d), actual) for k in range(6)]
    arm = sub(at_d, NODES["node:c"])
    carried = add(tuple(corner[:3]), cross(tuple(corner[3:]), arm))
    d_tree = list(add(add(carried, scale(arm, eps)), tuple(rel[:3]))) + list(add(tuple(corner[3:]), tuple(rel[3:])))
    cross_check(f"{case_id}: compatibility at D (force method, then tree integration)", d_tree, g)
    nodes = {"node:a": dict(zip(COMPONENTS_U, [F(0)] * 6)),
             "node:c": dict(zip(COMPONENTS_U, corner)),
             "node:d": dict(zip(COMPONENTS_U, g))}
    sections = {}
    for pid, i, j, y_ref in PIPES:
        _, axes = member_axes(i, j, y_ref)
        rows = []
        for s in STATIONS:
            point = add(NODES[i], scale(sub(NODES[j], NODES[i]), s))
            rows.append(actual(pid, axes, point))
        sections[pid] = rows
    return reactions, nodes, sections, {"redundants": redundants}


# ----------------------------------------------------------------------------- route 2: direct stiffness
def element_local_stiffness(length):
    """Euler-Bernoulli 3D frame member, local DOFs (u, v, w, tx, ty, tz) at i then j, / pi."""
    L = length
    ea, gj, ei = EA_P / L, GJ_P / L, EI_P
    k = [[F(0)] * 12 for _ in range(12)]

    def put(r, c, v):
        k[r][c] = F(v)
        k[c][r] = F(v)
    put(0, 0, ea); put(6, 6, ea); put(0, 6, -ea)
    put(3, 3, gj); put(9, 9, gj); put(3, 9, -gj)
    # bending in the local x-y plane: v and theta_z
    for (r, c, v) in ((1, 1, 12 * ei / L ** 3), (1, 5, 6 * ei / L ** 2), (1, 7, -12 * ei / L ** 3), (1, 11, 6 * ei / L ** 2),
                      (5, 5, 4 * ei / L), (5, 7, -6 * ei / L ** 2), (5, 11, 2 * ei / L),
                      (7, 7, 12 * ei / L ** 3), (7, 11, -6 * ei / L ** 2), (11, 11, 4 * ei / L)):
        put(r, c, v)
    # bending in the local x-z plane: w and theta_y (dw/dx = -theta_y)
    for (r, c, v) in ((2, 2, 12 * ei / L ** 3), (2, 4, -6 * ei / L ** 2), (2, 8, -12 * ei / L ** 3), (2, 10, -6 * ei / L ** 2),
                      (4, 4, 4 * ei / L), (4, 8, 6 * ei / L ** 2), (4, 10, 2 * ei / L),
                      (8, 8, 12 * ei / L ** 3), (8, 10, 6 * ei / L ** 2), (10, 10, 4 * ei / L)):
        put(r, c, v)
    return k


def transformation(axes):
    t = [[F(0)] * 12 for _ in range(12)]
    for blk in range(4):
        for r in range(3):
            for c in range(3):
                t[3 * blk + r][3 * blk + c] = axes[r][c]
    return t


def direct_stiffness(motion, thermal):
    eps = EPS_T if thermal else F(0)
    ids = list(NODES)
    index = {n: 6 * k for k, n in enumerate(ids)}
    n = 6 * len(ids)
    K = [[F(0)] * n for _ in range(n)]
    P = [F(0)] * n
    data = []
    for pid, i, j, y_ref in PIPES:
        length, axes = member_axes(i, j, y_ref)
        k = element_local_stiffness(length)
        t = transformation(axes)
        kg = matmul(matmul(transpose(t), k), t)
        fef = [F(0)] * 12
        fef[0], fef[6] = EA_P * eps, -EA_P * eps  # held member, node-on-element end actions
        dofs = [index[i] + d for d in range(6)] + [index[j] + d for d in range(6)]
        fg = matvec(transpose(t), fef)
        for a in range(12):
            P[dofs[a]] -= fg[a]
            for b in range(12):
                K[dofs[a]][dofs[b]] += kg[a][b]
        data.append((pid, dofs, k, t, fef))
    restrained = sorted(index[FIXED] + d for d in range(6)) + sorted(index[MOVED] + d for d in range(6))
    U = [F(0)] * n
    for d, name in enumerate(DOFS):
        U[index[MOVED] + d] = motion.get(name, F(0))
    free = [d for d in range(n) if d not in restrained]
    rhs = [P[f] - sum(K[f][r] * U[r] for r in restrained) for f in free]
    for f, v in zip(free, solve([[K[f][g] for g in free] for f in free], rhs)):
        U[f] = v
    ends, total = {}, [F(0)] * n
    for pid, dofs, k, t, fef in data:
        fl = [x + y for x, y in zip(matvec(k, matvec(t, [U[d] for d in dofs])), fef)]
        ends[pid] = fl
        fg = matvec(transpose(t), fl)
        for a in range(12):
            total[dofs[a]] += fg[a]
    for f in free:
        assert total[f] == 0
    motions = {nd: dict(zip(COMPONENTS_U, U[index[nd]:index[nd] + 6])) for nd in ids}
    reactions = {sup: dict(zip(COMPONENTS_F, total[index[nd]:index[nd] + 6])) for sup, nd in SUPPORTS.items()}
    return motions, reactions, ends


def check_direct_stiffness(case_id, motion, thermal, reactions, nodes, sections):
    m, r, ends = direct_stiffness(motion, thermal)
    for nd in NODES:
        for comp in COMPONENTS_U:
            cross_check(f"{case_id}: {nd} {comp} direct stiffness", m[nd][comp], nodes[nd][comp])
    for sup in SUPPORTS:
        for comp in COMPONENTS_F:
            cross_check(f"{case_id}: {sup} {comp} direct stiffness", r[sup][comp], reactions[sup][comp])
    for pid, rows in sections.items():
        fl = ends[pid]
        for slot, comp in enumerate(SECTION_COMPONENTS):
            cross_check(f"{case_id}: {pid} end action {comp} end_i direct stiffness", fl[slot], -rows[0][comp])
            cross_check(f"{case_id}: {pid} end action {comp} end_j direct stiffness", fl[6 + slot], rows[-1][comp])


# ----------------------------------------------------------------------------- publication
def family(unit_name):
    return {"N": "force", "N*m": "moment", "m": "translation", "rad": "rotation"}[unit_name]


def publish(case_id, reactions, nodes, sections):
    rows = []
    for sup in sorted(reactions):
        for comp in COMPONENTS_F:
            unit_name = "N" if comp.startswith("F") else "N*m"
            rows.append(dict(quantity="support_reaction", entity=sup, component=comp, unit=unit_name,
                             **record(reactions[sup][comp], 1)))
    for nd in NODES:
        for comp in COMPONENTS_U:
            unit_name = "m" if comp.startswith("u") else "rad"
            rows.append(dict(quantity="nodal_motion", entity=nd, component=comp, unit=unit_name,
                             **record(nodes[nd][comp], 0)))
    for pipe in sorted(sections):
        for name, sec in zip(STATION_NAMES, sections[pipe]):
            for comp in SECTION_COMPONENTS:
                unit_name = "N" if comp in ("axial_force", "shear_force_y", "shear_force_z") else "N*m"
                rows.append(dict(quantity="section_action", entity=pipe, station=name, component=comp,
                                 unit=unit_name, **record(sec[comp], 1)))
    scales = {}
    for fam, (unit_name, k) in (("force", ("N", 1)), ("moment", ("N*m", 1)), ("translation", ("m", 0)),
                                ("rotation", ("rad", 0))):
        mags = [abs(F(r["exact"].replace("*pi", ""))) for r in rows if family(r["unit"]) == fam]
        top = max(mags)
        assert top > 0, (case_id, fam)
        scales[fam] = dict(unit=unit_name, **record(top, k))
    return rows, scales


CASES = (("W16-MOTION", True, False), ("W16-THERMAL", False, True), ("W16-MOTION-THERMAL", True, True))


def inputs_record():
    return {
        "section": {"outside_diameter_m": str(OD), "wall_thickness_m": str(WALL), "inside_diameter_m": str(ID),
                    "elastic_modulus_Pa": str(E), "shear_modulus_Pa_preview_route": str(G),
                    "poisson_ratio_exact_route": str(NU), "area_m2": f"{AREA_P}*pi",
                    "second_moment_m4": f"{INERTIA_P}*pi", "polar_moment_m4": f"{POLAR_P}*pi",
                    "EA_N": f"{EA_P}*pi", "EI_N_m2": f"{EI_P}*pi", "GJ_N_m2": f"{GJ_P}*pi"},
        "geometry": {"nodes": {k: [int(c) for c in v] for k, v in NODES.items()},
                     "pipes": {pid: {"from": i, "to": j, "y_reference": [int(c) for c in y]}
                               for pid, i, j, y in PIPES},
                     "supports": {sup: {"node": nd, "restraints": list(DOFS)} for sup, nd in SUPPORTS.items()}},
    }


def build_cases():
    cases, by_id = [], {}
    for case_id, moving, thermal in CASES:
        motion = MOTION if moving else {}
        reactions, nodes, sections, key = force_method(case_id, motion, thermal)
        check_direct_stiffness(case_id, motion, thermal, reactions, nodes, sections)
        by_id[case_id] = (reactions, nodes, sections)
        rows, scales = publish(case_id, reactions, nodes, sections)
        inputs = inputs_record()
        if moving:
            inputs["support_motion"] = {"support": "support:d", "node": MOVED,
                                        "dofs": [{"dof": d, "value": str(v), "unit": "m" if d.startswith("U") else "rad"}
                                                 for d, v in MOTION.items()],
                                        "other_dofs": "held at 0"}
        if thermal:
            inputs["thermal"] = {"alpha_per_K": str(ALPHA), "delta_T_K": str(DELTA_T), "free_strain": str(EPS_T),
                                 "applied_to": [p[0] for p in PIPES]}
        values = {f"redundant_{c}": record(v, 1) for c, v in zip(COMPONENTS_F, key["redundants"])}
        values.update({f"corner_{c}": record(nodes["node:c"][c], 0) for c in COMPONENTS_U})
        cases.append({"id": case_id,
                      "method": "six-redundant force method (complementary energy), release D, R = F^-1 (g - d_T); "
                                "A by equilibrium; corner by unit-load member integration; sections by statics",
                      "inputs": inputs, "key_values": values, "zero_scales": scales, "rows": rows})
    # Superposition: for fixed stiffness and boundary partition, both together
    # equal the sum of the two cases, row for row.
    both, motion, thermal = by_id["W16-MOTION-THERMAL"], by_id["W16-MOTION"], by_id["W16-THERMAL"]
    for part in range(3):
        for entity, value in both[part].items():
            if part < 2:
                for comp, v in value.items():
                    cross_check(f"superposition: {entity} {comp}", v, motion[part][entity][comp] + thermal[part][entity][comp])
            else:
                for s, row in enumerate(value):
                    for comp, v in row.items():
                        cross_check(f"superposition: {entity} station {STATION_NAMES[s]} {comp}", v,
                                    motion[part][entity][s][comp] + thermal[part][entity][s][comp])
    return cases


def document(cases):
    return {
        "document": "openpipestress.w16_imposed_motion_references",
        "status": "candidate frozen references; derived and cross-checked by the author TASK, not yet independently refuted",
        "units": {"force": "N", "moment": "N*m", "translation": "m", "rotation": "rad"},
        "theory": "small-displacement linear-elastic Euler-Bernoulli space frame, no shear deformation, St Venant "
                  "torsion GJ with J = 2I; annular section, Iy = Iz = I",
        "conventions": {
            "support_reaction": "global action of the support on the structure at its node (moments about that node)",
            "nodal_motion": "global translation and right-handed rotation",
            "section_action": "element-local action on the +x face of the i-side portion at station s/L (local x from i to j); "
                              "a node-on-element end action is -section(0) at end_i and +section(1) at end_j",
            "local_axes": "x from i to j; y = y_reference less its x part; z = x cross y",
            "exact": "rational q, or q*pi",
        },
        "criterion": "|observed - expected| <= 1e-9 * |expected|; an exactly zero expectation uses 1e-9 * zero_scale "
                     "of its family (largest nonzero magnitude of that family in the case)",
        "crosscheck": list(CROSS),
        "cases": cases,
    }


def dumps(obj):
    def inline(o):
        return json.dumps(o, ensure_ascii=False, separators=(", ", ": "))

    def emit(o, level):
        pad = " " * level
        if isinstance(o, dict) and ("exact" in o or "check" in o or "dof" in o):
            return inline(o)
        if isinstance(o, dict):
            items = [f'{pad} {json.dumps(k)}: {emit(v, level + 1)}' for k, v in o.items()]
            return "{\n" + ",\n".join(items) + "\n" + pad + "}" if items else "{}"
        if isinstance(o, list) and o and all(isinstance(x, dict) for x in o):
            items = [f"{pad} {emit(v, level + 1)}" for v in o]
            return "[\n" + ",\n".join(items) + "\n" + pad + "]"
        return inline(o)
    return emit(obj, 0) + "\n"


def sha256sums(files):
    lines = [f"{hashlib.sha256(data).hexdigest()}  {path}" for path, data in sorted(files.items())]
    return ("\n".join(lines) + "\n").encode()


def build():
    CROSS.clear()
    cases = build_cases()
    doc = document(cases)
    text = dumps(doc)
    assert json.loads(text) == json.loads(json.dumps(doc))
    files = {"references.json": text.encode()}
    for fixed in FIXED_FILES:
        p = OUT / fixed
        if p.exists():
            files[fixed] = p.read_bytes()
    files["SHA256SUMS"] = sha256sums({k: v for k, v in files.items() if k != "SHA256SUMS"})
    return files, cases


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--write", action="store_true")
    args = parser.parse_args()
    files, cases = build()
    bad = [c for c in CROSS if not c["agrees"]]
    print(f"{len(cases)} cases, {sum(len(c['rows']) for c in cases)} rows")
    print(f"cross-checks: {len(CROSS)} recorded, {len(bad)} disagreeing")
    for c in cases:
        print(c["id"], ", ".join(f"{k} {v['value']:.6g}" for k, v in c["key_values"].items()))
    if args.write:
        if bad:
            print("not written: a cross-check disagrees")
            return 1
        for path, data in files.items():
            if path in FIXED_FILES:
                continue
            target = OUT / path
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(data)
        print(f"wrote {len(files) - len([f for f in FIXED_FILES if f in files])} files")
        return 0
    failures = [p for p, d in files.items() if not (OUT / p).exists() or (OUT / p).read_bytes() != d]
    committed = {str(p.relative_to(OUT)) for p in OUT.rglob("*") if p.is_file() and "__pycache__" not in p.parts}
    extra = sorted(committed - set(files))
    if failures or extra or bad:
        print("MISMATCH: " + ", ".join(failures + [f"unexpected {e}" for e in extra]))
        return 1
    print(f"verified {len(files)} files byte for byte (SHA256SUMS included)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
