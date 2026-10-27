"""Canonical comparison of a SWBPIPE model and its MBF import (DEL-17-04 Design `caepipe-mbf-import.md` §18.2).

Both models are reduced to the compared subset in SI base units, keyed by CAEPIPE names, and each scalar is given
an absolute bound derived from the printed values of the writer's MBF W and, for bends, the writer-admitted
residuals. This module is written independently of the importer: it reads the source model A directly, reads W's
printed text with its own splitting, and uses the importer's output only as the model I under comparison (its id
map gives CAEPIPE name correspondence).

Representation losses the comparison excludes (each explicit, Design §18.2):
"""

from __future__ import annotations

from dataclasses import dataclass, field
from fractions import Fraction
import math
from typing import Any, Mapping


EXCLUDED_LOSSES = (
    "y_reference on straight pipes (not in the MBF; bound imports restore it from the sidecar, unbound imports take "
    "the documented local y; a circular section's stiffness does not depend on it)",
    "support families (Design §12; restored as identity by bound imports, checked separately)",
    "labels, provenance and the members the writer does not export (Design §17 last rows)",
    "several primitives on one target (compared as their sum)",
    "zero-valued primitives (dropped)",
    "absent versus zero expansion coefficient, and absent versus zero section fields",
    "combinations (not exported)",
)
__doc__ += "".join(f"\n- {item}" for item in EXCLUDED_LOSSES) + "\n"

EPS = 2.0 ** -53
DOFS = ("UX", "UY", "UZ", "RX", "RY", "RZ")
_TO_SI = {
    "m": 1.0, "mm": 1e-3, "in": 0.0254, "ft": 0.3048,
    "N": 1.0, "kN": 1e3, "lbf": 4.4482216152605,
    "N*m": 1.0, "kN*m": 1e3, "lbf*in": 4.4482216152605 * 0.0254, "lbf*ft": 4.4482216152605 * 0.3048,
    "N/m": 1.0, "N/mm": 1e3, "kN/m": 1e3, "kN/mm": 1e6, "lbf/in": 4.4482216152605 / 0.0254,
    "Pa": 1.0, "kPa": 1e3, "MPa": 1e6, "GPa": 1e9, "psi": 4.4482216152605 / 0.00064516,
    "1/K": 1.0, "1/degC": 1.0, "1/degF": 1.8, "K": 1.0, "degC": 1.0, "C": 1.0, "degF": 5.0 / 9.0,
    "rad": 1.0, "deg": math.pi / 180.0, "1": 1.0, "none": 1.0,
}
# The same factors as exact rationals (the definitions: 1 in = 0.0254 m, 1 lbf = 4.4482216152605 N, degF intervals
# x 5/9; degrees by the binary64 pi), for summing primitives without cancellation error (Design §18.2).
_TO_SI_EXACT = {
    "m": Fraction(1), "mm": Fraction(1, 1000), "in": Fraction("0.0254"), "ft": Fraction("0.3048"),
    "N": Fraction(1), "kN": Fraction(1000), "lbf": Fraction("4.4482216152605"),
    "N*m": Fraction(1), "kN*m": Fraction(1000), "lbf*in": Fraction("4.4482216152605") * Fraction("0.0254"),
    "lbf*ft": Fraction("4.4482216152605") * Fraction("0.3048"),
    "N/m": Fraction(1), "N/mm": Fraction(1000), "kN/m": Fraction(1000), "kN/mm": Fraction(10**6),
    "lbf/in": Fraction("4.4482216152605") / Fraction("0.0254"),
    "K": Fraction(1), "degC": Fraction(1), "C": Fraction(1), "degF": Fraction(5, 9),
    "rad": Fraction(1), "deg": Fraction(math.pi) / 180,
}
_DOF_TOKEN = {**{dof: dof for dof in DOFS}, **{dof.lower(): dof for dof in DOFS},
              **{dof[0] + dof[1].lower(): dof for dof in DOFS}}
_AXIS = {"global_x": 0, "global_y": 1, "global_z": 2, "GLOBAL_X": 0, "GLOBAL_Y": 1, "GLOBAL_Z": 2,
         "RX": 0, "RY": 1, "RZ": 2, "rotation_x": 0, "rotation_y": 1, "rotation_z": 2}


def si(quantity: Mapping[str, Any]) -> float:
    return float(quantity["value"]) * _TO_SI[quantity["unit"]]


def delta(printed: str) -> float:
    """delta(p) = 0.5 * 10^(floor(log10|p|) - 9) for the writer's 10-significant-digit print; 0 for an exact zero."""
    value = Fraction(printed)
    if value == 0:
        return 0.0
    exponent = math.floor(math.log10(abs(float(value))))
    if Fraction(10) ** exponent > abs(value):  # guard log10 rounding at powers of ten
        exponent -= 1
    elif Fraction(10) ** (exponent + 1) <= abs(value):
        exponent += 1
    return 0.5 * 10.0 ** (exponent - 9)


def printed_bound(printed: str, factor: float = 1.0) -> float:
    """Printed directly: delta(p) + 4 eps |p|, in SI base units by the value's own factor."""
    return (delta(printed) + 4 * EPS * abs(float(printed))) * factor


# ---------------------------------------------------------------------------
# reading the writer's printed MBF (independently of the importer)
# ---------------------------------------------------------------------------
@dataclass
class Printed:
    t_ref: str
    materials: dict[str, dict[str, str]]
    sections: dict[str, tuple[str, str]]
    loads: dict[str, str]
    node_bound: dict[str, list[float]]  # CAEPIPE name -> per-axis absolute bound, metres
    coordinate_max_mm: float
    rows: list[dict[str, Any]] = field(default_factory=list)


def _fields(line: str) -> list[str]:
    out, depth, current = [], 0, ""
    for char in line:
        depth += char == "("
        depth -= char == ")"
        if char == "," and depth == 0:
            out.append(current)
            current = ""
        else:
            current += char
    return out + [current]


def read_printed(text: str) -> Printed:
    lines = text.split("\r\n")
    section, materials, sections, loads, rows = None, {}, {}, {}, []
    t_ref = "0"
    keywords = {"VERSION", "OPTIONS", "OPTEMP", "MATERIAL", "PIPE", "LOADS", "LAYOUT"}
    for line in lines[1:]:
        if line in keywords:
            section = line
            continue
        if not line:
            continue
        fields = _fields(line)
        if section == "OPTEMP":
            t_ref = fields[0]
        elif section == "MATERIAL":
            entry = materials.setdefault(fields[0], {})
            if len(fields) == 7:
                entry.update(density=fields[1], nu=fields[2])
            else:
                entry.update(E=fields[2], alpha=fields[3])
                entry.setdefault("temperatures", []).append(fields[1])
        elif section == "PIPE":
            sections[fields[0]] = (fields[1], fields[2])
        elif section == "LOADS":
            loads[fields[0]] = fields[1]
        elif section == "LAYOUT":
            row: dict[str, Any] = {"kind": fields[0][0], "node": fields[0][1:], "axes": {}, "radius": None,
                                   "M": None, "P": None, "L": None, "comments": []}
            comments = False
            for item in fields[1:]:
                if comments or item.startswith("C"):
                    row["comments"].append(item[1:] if not comments else item)
                    comments = True
                elif item[0] in "XYZ":
                    row["axes"]["XYZ".index(item[0])] = item[1:]
                elif item[0] == "B":
                    row["radius"] = item[1:]
                elif item[0] in "MP":
                    row[item[0]] = item[1:]
            for comment in row["comments"]:
                if comment.startswith("L="):
                    row["L"] = comment[2:]
            rows.append(row)
    coordinate_max = 0.0
    position: dict[str, list[float]] = {}
    for row in rows:
        if row["kind"] == "F" and row["node"] not in position:
            position[row["node"]] = [float(row["axes"].get(axis, "0")) for axis in range(3)]
            current = row["node"]
        elif row["kind"] == "F":
            current = row["node"]
        else:
            base = position[current]
            position[row["node"]] = [base[axis] + float(row["axes"].get(axis, "0")) for axis in range(3)]
            current = row["node"]
        coordinate_max = max([coordinate_max, *map(abs, position[row["node"]])])
    printed = Printed(t_ref, materials, sections, loads, {}, coordinate_max, rows)
    # Node coordinates (Design §18.2): along the path from its run start, per axis, the sum of
    # delta(p) + 2 eps C over the printed start coordinate and every printed offset, plus 4 eps C for the import.
    c_eps = 2 * EPS * coordinate_max
    bound: dict[str, list[float]] = {}
    for row in rows:
        if row["kind"] == "F":
            if row["node"] not in bound:
                bound[row["node"]] = [delta(row["axes"].get(axis, "0")) + (c_eps if axis in row["axes"] else 0.0)
                                      for axis in range(3)]
            current = row["node"]
        else:
            bound[row["node"]] = [bound[current][axis] + (delta(row["axes"][axis]) + c_eps if axis in row["axes"]
                                                          else 0.0) for axis in range(3)]
            current = row["node"]
    printed.node_bound = {name: [(value + 4 * EPS * coordinate_max) * 1e-3 for value in values]
                          for name, values in bound.items()}
    return printed


# ---------------------------------------------------------------------------
# canonical forms
# ---------------------------------------------------------------------------
@dataclass
class Canonical:
    values: dict[tuple, float] = field(default_factory=dict)
    sets: dict[tuple, Any] = field(default_factory=dict)


def _vec(position: Mapping[str, Any], scale: float) -> list[float]:
    return [float(position[axis]) * scale for axis in "xyz"]


def _project(y: list[float], a: list[float], b: list[float]) -> list[float]:
    chord = [b[i] - a[i] for i in range(3)]
    length = math.hypot(*chord)  # scaled norms: tiny and huge bends do not underflow or overflow
    unit = [value / length for value in chord]
    dot = sum(y[i] * unit[i] for i in range(3))
    side = [y[i] - dot * unit[i] for i in range(3)]
    norm = math.hypot(*side)
    return [value / norm for value in side]


def canonical(model: Mapping[str, Any], names: Mapping[str, str], case_kinds: Mapping[str, str], *,
              oriented: bool, writer_nu: Mapping[str, float] | None = None) -> Canonical:
    """Reduce `model` to the compared subset; `names` maps node ids to CAEPIPE names, `case_kinds` case ids to
    'sustained' or 'expansion'. `writer_nu` gives the writer's nu per material id (A's value before printing)."""
    out = Canonical()
    length_scale = _TO_SI[model["project"]["units"]["length"]]
    positions = {}
    for node in model["nodes"]:
        if node["id"] in names:
            positions[node["id"]] = _vec(node["position"], length_scale)
            for axis in range(3):
                out.values[("node", names[node["id"]], axis)] = positions[node["id"]][axis]
    materials = {material["id"]: material for material in model.get("materials") or []}
    pipes = {}
    for pipe in model["pipe_segments"]:
        ends = (names[pipe["from"]], names[pipe["to"]])
        key = ends if oriented else tuple(sorted(ends))
        pipes[pipe["id"]] = key
        material = materials[pipe["material"]]
        e = si(material["elastic_modulus"])
        if writer_nu is not None:
            nu = writer_nu[pipe["material"]]
        elif material.get("poisson_ratio") is not None:
            nu = si(material["poisson_ratio"])
        else:
            nu = e / (2 * si(material["shear_modulus"])) - 1.0
        g = si(material["shear_modulus"]) if material.get("shear_modulus") is not None else e / (2 * (1 + nu))
        alpha = si(material["thermal_expansion_coefficient"]) if material.get("thermal_expansion_coefficient") else 0.0
        for name, value in (("OD", si(pipe["section"]["outside_diameter"])), ("t", si(pipe["section"]["wall_thickness"])),
                            ("E", e), ("nu", nu), ("G", g), ("alpha", alpha)):
            out.values[("pipe", key, name)] = value
        out.sets[("pipe", key)] = pipe["material"]
    restrained: dict[str, set[str]] = {}
    supports = {}
    for support in model["supports"]:
        node = names[support["node"]]
        dofs = {_DOF_TOKEN[token] for token in support["restraints"]}
        restrained.setdefault(node, set()).update(dofs)
        supports[support["id"]] = node
        if oriented:
            out.sets[("support", support["id"])] = (node, frozenset(dofs))
    for node, dofs in restrained.items():
        out.sets[("restraints", node)] = frozenset(dofs)
    for component in model.get("components") or []:
        geometry = component["geometry"]
        pipe = next(item for item in model["pipe_segments"] if item["id"] == geometry["bend_pipe_ref"])
        key = pipes[pipe["id"]]
        a, b = positions[pipe["from"]], positions[pipe["to"]]
        y = [float(pipe["y_reference"][axis]) for axis in "xyz"]
        direction = _project(y, a, b)
        out.values[("bend", key, "R")] = si(geometry["bend_radius"])
        modifiers = component.get("modifiers") or {}
        if modifiers.get("flexibility_factor_user_value"):
            out.values[("bend", key, "k")] = si(modifiers["flexibility_factor_user_value"])
        out.sets[("bend", key, "plane")] = direction
        out.sets[("bend", key, "node")] = names[component["node"]]
    sums: dict[tuple, Fraction] = {}
    for case in model.get("load_cases") or []:
        kind = case_kinds[case["id"]]
        for load in case.get("primitive_loads") or []:
            value = Fraction(load["magnitude"]["value"]) * _TO_SI_EXACT[load["magnitude"]["unit"]]
            target = load["target"]
            category = load["category"]
            if category in ("concentrated_force", "concentrated_moment"):
                key = (kind, category, names[target["node"]], _AXIS[load["direction"]])
            elif category in ("weight", "thermal"):
                key = (kind, category, pipes[target["pipe"]])
            else:
                dof = _DOF_TOKEN[target["dof"]]
                key = (kind, category, supports[target["support"]], dof)
            sums[("load",) + key] = sums.get(("load",) + key, Fraction(0)) + value
    # Summed exactly in SI base units and rounded once; zero sums are dropped.
    out.values.update({key: float(value) for key, value in sums.items() if value != 0})
    return out


# ---------------------------------------------------------------------------
# tolerances (Design §18.2)
# ---------------------------------------------------------------------------
def bend_residuals(model: Mapping[str, Any], bend_pipe_id: str) -> dict[str, float]:
    """A's writer-admitted bend residuals, with the writer's own formulas (si_writer.tangent_intersection):
    tangent-length mismatches |t_i - R tan(theta/2)|, the intersection gap and the orientation misalignment
    (sine of the angle between the projected y_reference and the direction from the chord middle to the TI)."""
    scale = _TO_SI[model["project"]["units"]["length"]]
    nodes = {node["id"]: _vec(node["position"], scale) for node in model["nodes"]}
    pipes = {pipe["id"]: pipe for pipe in model["pipe_segments"]}
    bend = pipes[bend_pipe_id]
    component = next(item for item in model["components"] if item["geometry"]["bend_pipe_ref"] == bend_pipe_id)
    radius = si(component["geometry"]["bend_radius"])

    def far_end(tangent: str) -> list[float]:
        straight = next(pipe for pipe in pipes.values() if pipe["id"] != bend_pipe_id and tangent in (pipe["from"], pipe["to"]))
        return nodes[straight["to"] if straight["from"] == tangent else straight["from"]]

    p, q = nodes[bend["from"]], nodes[bend["to"]]
    incoming = [p[i] - far_end(bend["from"])[i] for i in range(3)]
    outgoing = [far_end(bend["to"])[i] - q[i] for i in range(3)]
    chord = [q[i] - p[i] for i in range(3)]
    unit = lambda v: [x / math.hypot(*v) for x in v]  # noqa: E731
    d1, d2 = unit(incoming), unit(outgoing)
    cosine = max(-1.0, min(1.0, sum(d1[i] * d2[i] for i in range(3))))
    det = 1.0 - cosine * cosine
    b1, b2 = sum(d1[i] * chord[i] for i in range(3)), sum(d2[i] * chord[i] for i in range(3))
    t1, t2 = (b1 - cosine * b2) / det, (b2 - cosine * b1) / det
    ti = [p[i] + t1 * d1[i] for i in range(3)]
    gap = math.hypot(*(q[i] - t2 * d2[i] - ti[i] for i in range(3)))
    tangent = radius * math.tan(math.acos(cosine) / 2.0)
    side = _project([float(bend["y_reference"][axis]) for axis in "xyz"], p, q)
    middle = [(p[i] + q[i]) / 2.0 for i in range(3)]
    toward = unit([ti[i] - middle[i] for i in range(3)])
    cross = [side[1] * toward[2] - side[2] * toward[1], side[2] * toward[0] - side[0] * toward[2],
             side[0] * toward[1] - side[1] * toward[0]]
    return {"tangent_near": abs(t1 - tangent), "tangent_far": abs(t2 - tangent), "gap": gap,
            "orientation": math.hypot(*cross), "theta": math.acos(cosine),
            "radius": radius, "ti": ti, "legs": (math.hypot(*incoming) + t1, math.hypot(*outgoing) + t2)}


def bounds(model: Mapping[str, Any], writer: Any, printed: Printed, names: Mapping[str, str],
           reference: Canonical, *, residuals: bool = True) -> dict[tuple, float]:
    """Absolute bound per compared scalar of `reference` (A's canonical form)."""
    out: dict[tuple, float] = {}
    for key in reference.values:
        if key[0] == "node" and key[1] in printed.node_bound:  # tangent nodes nA, nB: from their bend, below
            out[key] = printed.node_bound[key[1]][key[2]]
    element = writer.element_map
    authored_both = {material["id"]: material.get("poisson_ratio") is not None and
                     material.get("shear_modulus") is not None for material in model.get("materials") or []}
    pipe_keys = {}
    for pipe in model["pipe_segments"]:
        ends = (names[pipe["from"]], names[pipe["to"]])
        pipe_keys[pipe["id"]] = [key for key in (ends, tuple(sorted(ends))) if ("pipe", key, "OD") in reference.values][0]
    for pipe_id, key in pipe_keys.items():
        entry = element[pipe_id]
        od, t = printed.sections[entry["section"]]
        material = printed.materials[entry["material"]]
        out[("pipe", key, "OD")] = printed_bound(od, 1e-3)
        out[("pipe", key, "t")] = printed_bound(t, 1e-3)
        out[("pipe", key, "E")] = printed_bound(material["E"], 1e6)
        out[("pipe", key, "nu")] = printed_bound(material["nu"])
        out[("pipe", key, "alpha")] = printed_bound(material["alpha"])
        e, nu = float(material["E"]), float(material["nu"])
        relative = delta(material["E"]) / e + delta(material["nu"]) / (1 + nu) + 8 * EPS
        mat_id = next(pipe["material"] for pipe in model["pipe_segments"] if pipe["id"] == pipe_id)
        if authored_both.get(mat_id):
            relative += 1e-9  # the writer's MBF-SI-SHEAR-MODULUS-NOT-EXPORTED agreement threshold
        out[("pipe", key, "G")] = relative * abs(reference.values[("pipe", key, "G")])
        # Weight per length: delta(rho)/rho + (|dA/dOD| delta(OD) + |dA/dt| delta(t))/A + 8 eps, relative.
        density = material["density"]
        if float(density) != 0:
            odv, tv = float(od), float(t)
            area = math.pi / 4.0 * (odv**2 - (odv - 2 * tv) ** 2)
            weight_relative = (delta(density) / float(density)
                               + (math.pi * tv * delta(od) + math.pi * (odv - 2 * tv) * delta(t)) / area + 8 * EPS)
            for load_key, value in reference.values.items():
                if load_key[:3] == ("load", "sustained", "weight") and load_key[3] == key:
                    out[load_key] = weight_relative * abs(value)
        load_name = entry["load"]
        t1 = printed.loads[load_name]
        for load_key in reference.values:
            if load_key[:3] == ("load", "expansion", "thermal") and load_key[3] == key:
                out[load_key] = delta(t1) + delta(printed.t_ref) + 4 * EPS * abs(float(t1))
    # Forces, moments, displacements and rotations: printed directly, at the node's row comment.
    comment_values: dict[tuple, str] = {}
    for row in printed.rows:
        for comment in row["comments"]:
            for code, category in (("FOR(", "concentrated_force"), ("MOM(", "concentrated_moment"),
                                   ("D(", "imposed_displacement_U"), ("ROT(", "imposed_displacement_R")):
                if comment.startswith(code):
                    for axis, value in enumerate(comment[len(code):-1].split(",")):
                        comment_values[(category, row["node"], axis)] = value
    for key, value in reference.values.items():
        if key[0] != "load" or key[2] not in ("concentrated_force", "concentrated_moment", "imposed_displacement"):
            continue
        if key[2] == "imposed_displacement":
            dof = key[4]
            printed_value = comment_values[(f"imposed_displacement_{dof[0]}", key[3], DOFS.index(dof) % 3)]
            out[key] = printed_bound(printed_value, 1e-3 if dof[0] == "U" else math.pi / 180.0)
        else:
            out[key] = printed_bound(comment_values[(key[2], key[3], key[4])])
    # Bends: R, k printed directly; tangent nodes and the bend plane by the derivation below.
    for pipe_id, entry in element.items():
        bend = entry.get("bend")
        if not bend:
            continue
        key = pipe_keys[pipe_id]
        row = next(row for row in printed.rows if row["node"] == bend["caepipe_row_node"] and row["radius"])
        out[("bend", key, "R")] = printed_bound(row["radius"], 1e-3)
        factor = next(comment[3:] for comment in row["comments"] if comment.startswith("FF="))
        out[("bend", key, "k")] = printed_bound(factor)
        res = bend_residuals(model, pipe_id)
        r, theta = res["radius"], res["theta"]
        ti_bound = max(printed.node_bound[bend["caepipe_row_node"]])
        previous = printed.rows[printed.rows.index(row) - 1]["node"]
        following = printed.rows[printed.rows.index(row) + 1]["node"]
        legs = min(res["legs"])
        ends = max(sum(printed.node_bound[name]) for name in (previous, bend["caepipe_row_node"], following))
        d_theta = 2 * ends / legs
        tangent_bound = (ti_bound + delta(row["radius"]) * 1e-3 * math.tan(theta / 2)
                         + r / math.cos(theta / 2) ** 2 * d_theta / 2)
        for suffix, residual in (("A", res["tangent_near"]), ("B", res["tangent_far"])):
            name = bend["caepipe_row_node"] + suffix
            extra = (residual + res["gap"]) if residuals else 0.0
            for axis in range(3):
                out[("node", name, axis)] = tangent_bound + extra + 4 * EPS * printed.coordinate_max_mm * 1e-3
        # Bend plane (angle, rad): a shift b of the chord ends or the TI turns the projected direction by at most
        # 2 (b_TI + b_N + b_F) / min(|TI - chord middle|, chord); plus A's orientation residual.
        chord = 2 * r * math.sin(theta / 2) if theta else 0.0
        sagitta = r * (1 / math.cos(theta / 2) - math.cos(theta / 2))
        out[("bend", key, "plane")] = (2 * (ti_bound + 2 * (tangent_bound + (res["tangent_near"] + res["gap"]
                                                                              if residuals else 0.0)))
                                       / min(sagitta, chord) + 8 * EPS
                                       + (res["orientation"] if residuals else 0.0))
    return out


@dataclass
class Comparison:
    failures: list[str]
    r_in: float
    compared: int


def compare(reference: Canonical, imported: Canonical, limits: Mapping[tuple, float]) -> Comparison:
    failures = []
    r_in = 0.0
    for key in sorted(set(reference.values) | set(imported.values), key=repr):
        if key not in reference.values or key not in imported.values:
            failures.append(f"{key}: present in only one model")
            continue
        a, b = reference.values[key], imported.values[key]
        limit = limits.get(key)
        if limit is None:
            failures.append(f"{key}: no derived bound")
            continue
        if abs(a - b) > limit:
            failures.append(f"{key}: |{a!r} - {b!r}| = {abs(a - b)!r} > {limit!r}")
        if a != 0:
            r_in = max(r_in, limit / abs(a))
    for key in sorted(set(reference.sets) | set(imported.sets), key=repr):
        a, b = reference.sets.get(key), imported.sets.get(key)
        if key[0] == "pipe":
            continue  # the material identity is compared through E, nu, G and alpha
        if key[-1] == "plane":
            if a is None or b is None:
                failures.append(f"{key}: present in only one model")
                continue
            angle = math.asin(min(1.0, math.sqrt(sum(value * value for value in (
                a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])))))
            if sum(a[i] * b[i] for i in range(3)) <= 0 or angle > limits[key]:
                failures.append(f"{key}: bend planes differ by {angle!r} rad > {limits[key]!r}")
        elif a != b:
            failures.append(f"{key}: {a!r} != {b!r}")
    return Comparison(failures, r_in, len(reference.values))


def _unoriented_key(key: tuple) -> tuple:
    if key[0] in ("pipe", "bend"):
        return (key[0], tuple(sorted(key[1])), *key[2:])
    if key[0] == "load" and isinstance(key[3], tuple):
        return key[:3] + (tuple(sorted(key[3])),) + key[4:]
    return key


def unoriented(form: Canonical, limits: Mapping[tuple, float]) -> tuple[Canonical, dict[tuple, float]]:
    """A canonical form and its bounds with pipe ends unordered and per-support identities dropped (unbound)."""
    return (Canonical({_unoriented_key(key): value for key, value in form.values.items()},
                      {_unoriented_key(key): value for key, value in form.sets.items() if key[0] != "support"}),
            {_unoriented_key(key): value for key, value in limits.items()})
