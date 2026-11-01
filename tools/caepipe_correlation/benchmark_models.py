"""Seeded generator of the invented CAEPIPE benchmark models, tranche 1 (DEL-17-05 Design §11, RT-V2).

From the project root:

    python3 -m tools.caepipe_correlation.benchmark_models [--seed N] [--out DIR] [--check]

Writes one SWBPIPE model file per benchmark model to `<out>/models/<model_id>.json` and the
`<out>/manifest.json` that lists each model's seed, family, parameters, sampling, series and hashes. The
default output is the tracked `validation/caepipe_benchmark/`. Every model is invented: geometry, sections,
materials and loads come from the seed and the family definitions below, never from a library, catalogue,
standard or CAEPIPE output. `--check` regenerates in memory and compares with the tracked files byte for byte
(exit 1 on any difference).

Families G1-G8 (Design §11.1): straight spans, single bends, Z and U loops, multi-plane runs, branch tees, risers,
support mixes and anchor movements. G1-G4 take their corner or series points plus a Latin hypercube drawn from
`random.Random(seed)`; G5-G8 enumerate their configurations (and G5 its tee series) and sample the rest by the
hypercube. The generator uses only `random()` from it, whose sequence for an integer seed is stable across Python
versions, and prints every float through `repr`. Its transcendental functions (exp, log, sin, cos, tan, acos) and
integer powers are correctly rounded, computed in decimal at 60 digits and rounded once, never by the platform's
libm, so the same seed gives the same bytes on every platform.
Closed-form Euler-Bernoulli values are attached to the straight members whose response has one (G1, G8); they
are the third, independent value of the comparison.

Nothing here runs either program, and nothing here records a result.
"""

from __future__ import annotations

import argparse
from decimal import Decimal, localcontext
import hashlib
import json
import math
from pathlib import Path
import random
import sys
from typing import Any, Callable


# -- correctly rounded transcendental functions (platform independent) ----------------------------------------------
_PREC = 60
_PI = Decimal("3.14159265358979323846264338327950288419716939937510582097494459230781640628620899862803")


def _taylor_sin_cos(x: Decimal) -> tuple[Decimal, Decimal]:
    """sin and cos of `x` (|x| <= pi after reduction) by their Taylor series, at the current context."""
    sin, cos, term, n = Decimal(0), Decimal(0), Decimal(1), 0
    tiny = Decimal(10) ** -(_PREC + 5)
    while True:
        if n % 2 == 0:
            cos += term if n % 4 == 0 else -term
        else:
            sin += term if n % 4 == 1 else -term
        n += 1
        term = term * x / n
        if n > 4 and abs(term) < tiny:
            return sin, cos


def _sin_cos(x: float) -> tuple[Decimal, Decimal]:
    with localcontext() as context:
        context.prec = _PREC + 10
        value = Decimal(x)
        turn = 2 * _PI
        value -= turn * (value / turn).to_integral_value()
        if value > _PI:
            value -= turn
        elif value < -_PI:
            value += turn
        return _taylor_sin_cos(value)


def cr_sin(x: float) -> float:
    return float(_sin_cos(x)[0])


def cr_cos(x: float) -> float:
    return float(_sin_cos(x)[1])


def cr_tan(x: float) -> float:
    sin, cos = _sin_cos(x)
    with localcontext() as context:
        context.prec = _PREC
        return float(sin / cos)


def cr_acos(x: float) -> float:
    if not -1.0 <= x <= 1.0:
        raise ValueError("math domain error")
    if x == 1.0:
        return 0.0
    if x == -1.0:
        return float(_PI)
    with localcontext() as context:
        context.prec = _PREC + 10
        target, y = Decimal(x), Decimal(math.acos(x))  # libm's value only seeds Newton's iteration
        for _ in range(8):
            sin, cos = _taylor_sin_cos(y)
            y += (cos - target) / sin
        return float(y)


def cr_pow(x: float, n: int) -> float:
    """x ** n for an integer n, correctly rounded (CPython's float ** calls the platform's pow)."""
    with localcontext() as context:
        context.prec = _PREC
        return float(Decimal(x) ** n)


def cr_exp(x: float) -> float:
    with localcontext() as context:
        context.prec = _PREC
        return float(Decimal(x).exp())


def cr_log(x: float) -> float:
    with localcontext() as context:
        context.prec = _PREC
        return float(Decimal(x).ln())


PROJECT = Path(__file__).resolve().parents[2]
if str(PROJECT) not in sys.path:
    sys.path.insert(0, str(PROJECT))

from core.handoff.caepipe_external.correlation import canonical_model_hash  # noqa: E402


GENERATOR_VERSION = "caepipe-benchmark-generator-1"
MANIFEST_KIND = "openpipestress.caepipe_benchmark.manifest"
MODEL_KIND = "openpipestress.caepipe_benchmark.model"
DEFAULT_SEED = 20261028
DEFAULT_OUT = PROJECT / "validation" / "caepipe_benchmark"
# The piping code every benchmark MBF is written with (Design §11.2): an ASME B31.1 edition that predates B31J, so
# OPCODE's B31J off is honoured; the current B31.1 token locks B31J on (Analysis).
PIPING_CODE = "B311-80"
REFERENCE_TEMPERATURE_C = 20.0
GRAVITY = 9.80665
SUSTAINED = "case:sustained"
EXPANSION = "case:expansion"
OPERATING = "operating"
SIX = ["UX", "UY", "UZ", "RX", "RY", "RZ"]
PROVENANCE = ("invented CAEPIPE benchmark input (DEL-17-05, RT-V2); seeded generator; not library, component, "
              "catalog, standard or code-rule data")

# Three invented temperature-independent materials (Design §11.1): E, Poisson ratio, expansion, mass density.
MATERIALS = (
    ("material:bench-a", 200.0e9, 0.30, 12.0e-6, 7850.0),
    ("material:bench-b", 195.0e9, 0.30, 16.0e-6, 7900.0),
    ("material:bench-c", 70.0e9, 0.33, 23.0e-6, 2700.0),
)
OD_RANGE = (60.0, 610.0)  # mm
DT_RANGE = (10.0, 100.0)  # D/t
DELTA_T_RANGE = (50.0, 250.0)  # K
# Inputs that cannot be held exactly equal on both sides (Design §11.3); each is carried by a series.
SHEAR = "shear_deformation"
BEND_K = "bend_flexibility_application"
BEND_SHEAR = "bend_transverse_shear"


# -- sampling ------------------------------------------------------------------------------------------------------
def latin_hypercube(rng: random.Random, points: int, dims: int) -> list[list[float]]:
    """`points` samples in [0, 1)^dims, one per stratum in every dimension (random() only, for stability)."""
    columns = []
    for _ in range(dims):
        order = list(range(points))
        for index in range(points - 1, 0, -1):  # Fisher-Yates with random() only
            other = int(rng.random() * (index + 1))
            order[index], order[other] = order[other], order[index]
        columns.append([(order[point] + rng.random()) / points for point in range(points)])
    return [[columns[dim][point] for dim in range(dims)] for point in range(points)]


def scale(unit: float, low: float, high: float, *, log: bool = False, digits: int = 4) -> float:
    """`unit` in [0, 1) mapped to [low, high] (log-uniform with `log`), rounded to `digits` significant digits."""
    value = cr_exp(cr_log(low) + unit * (cr_log(high) - cr_log(low))) if log else low + unit * (high - low)
    return float(f"{value:.{digits}g}")


# -- model building -----------------------------------------------------------------------------------------------
def q(value: float, unit: str) -> dict[str, Any]:
    return {"value": value, "unit": unit}


def _sub(a, b):
    return tuple(x - y for x, y in zip(a, b))


def _add(a, b):
    return tuple(x + y for x, y in zip(a, b))


def _mul(a, s):
    return tuple(x * s for x in a)


def _unit(a):
    norm = math.sqrt(sum(x * x for x in a))
    return tuple(x / norm for x in a)


def _perpendicular(direction) -> dict[str, float]:
    """A y_reference perpendicular to a straight along a global axis: global Y, or global X for a vertical run."""
    return {"x": 1.0, "y": 0.0, "z": 0.0} if abs(direction[1]) > 0.5 else {"x": 0.0, "y": 1.0, "z": 0.0}


class Builder:
    """One invented SWBPIPE preview model (model document 0.9.0), built node by node in metres."""

    def __init__(self, model_id: str, material: int) -> None:
        self.model_id = model_id
        self.material = MATERIALS[material]
        self.nodes: list[dict[str, Any]] = []
        self.positions: dict[str, tuple[float, float, float]] = {}
        self.pipes: list[dict[str, Any]] = []
        self.components: list[dict[str, Any]] = []
        self.supports: list[dict[str, Any]] = []
        self.sustained: list[dict[str, Any]] = []
        self.expansion: list[dict[str, Any]] = []
        self.sections: dict[str, tuple[float, float]] = {}

    def node(self, name: str, position) -> str:
        node_id = f"node:{name}"
        position = tuple(float(f"{value:.12g}") + 0.0 for value in position)
        self.positions[node_id] = position
        self.nodes.append({"id": node_id, "position": dict(zip("xyz", position)), "provenance": PROVENANCE})
        return node_id

    def pipe(self, name: str, start: str, end: str, od_mm: float, wall_mm: float, *, y_reference=None) -> str:
        pipe_id = f"pipe:{name}"
        direction = _unit(_sub(self.positions[end], self.positions[start]))
        self.pipes.append({
            "id": pipe_id, "from": start, "to": end, "material": self.material[0],
            "section": {"outside_diameter": q(od_mm / 1000.0, "m"), "wall_thickness": q(wall_mm / 1000.0, "m")},
            "y_reference": y_reference or _perpendicular(direction), "provenance": PROVENANCE,
        })
        self.sections[pipe_id] = (od_mm, wall_mm)
        return pipe_id

    def run(self, prefix: str, points: list[str], od_mm: float, wall_mm: float) -> list[str]:
        """Straight pipes joining consecutive nodes."""
        return [self.pipe(f"{prefix}{index + 1}", points[index], points[index + 1], od_mm, wall_mm)
                for index in range(len(points) - 1)]

    def bend(self, name: str, near: str, far: str, ti, radius_m: float, k: float, od_mm: float, wall_mm: float) -> str:
        """The curved bend pipe near -> far with its component; y_reference points from the chord toward the TI."""
        p, r = self.positions[near], self.positions[far]
        middle = _mul(_add(p, r), 0.5)
        toward = _unit(_sub(ti, middle))
        pipe_id = self.pipe(name, near, far, od_mm, wall_mm, y_reference=dict(zip("xyz", toward)))
        d1, d2 = _unit(_sub(ti, p)), _unit(_sub(r, ti))
        angle = cr_acos(max(-1.0, min(1.0, sum(a * b for a, b in zip(d1, d2)))))
        self.components.append({
            "id": f"component:{name}", "kind": "bend", "node": near, "provenance": PROVENANCE,
            "geometry": {"bend_pipe_ref": pipe_id, "bend_radius": q(radius_m, "m"), "bend_angle": q(angle, "rad"),
                         "bend_geometry_source_reference": "invented benchmark bend (seeded generator)"},
            "modifiers": {"flexibility_factor_user_value": q(k, "none"),
                          "source_reference": "invented benchmark value (seeded generator)"},
            "mechanics_interface": {"solver_consumption": "curved_bend_macro_element"},
        })
        return pipe_id

    def anchor(self, name: str, node: str) -> str:
        return self.support(name, node, SIX, "anchor")

    def support(self, name: str, node: str, restraints: list[str], family: str | None) -> str:
        support_id = f"support:{name}"
        entry = {"id": support_id, "node": node, "restraints": list(restraints), "provenance": PROVENANCE}
        if family is not None:
            entry["family"] = family
        self.supports.append(entry)
        return support_id

    def spring(self, name: str, node: str, dof: str, stiffness_n_per_m: float) -> str:
        support_id = f"support:{name}"
        self.supports.append({"id": support_id, "node": node, "family": "spring", "restraints": [dof],
                              "stiffness": {"dof": dof, "value": q(stiffness_n_per_m, "N/m")},
                              "provenance": PROVENANCE})
        return support_id

    def weight_per_length(self, pipe_id: str) -> float:
        od, wall = self.sections[pipe_id]
        area = math.pi / 4.0 * (cr_pow(od / 1000.0, 2) - cr_pow((od - 2.0 * wall) / 1000.0, 2))
        return -self.material[4] * area * GRAVITY

    def add_weight(self) -> None:
        for pipe in self.pipes:
            self.sustained.append({
                "id": f"load:weight:{pipe['id']}", "category": "weight",
                "target": {"type": "element", "pipe": pipe["id"]}, "direction": "global_y",
                "magnitude": q(self.weight_per_length(pipe["id"]), "N/m"), "dimension": "force_per_length",
                "provenance": PROVENANCE + "; w = -rho A g with g = 9.80665 m/s2 and A the bare metal area"})

    def add_thermal(self, delta_t: float) -> None:
        for pipe in self.pipes:
            self.expansion.append({
                "id": f"load:thermal:{pipe['id']}", "category": "thermal",
                "target": {"type": "element", "pipe": pipe["id"]}, "direction": "global_x",
                "magnitude": q(delta_t, "degC"), "dimension": "temperature_interval", "provenance": PROVENANCE})

    def force(self, name: str, node: str, direction: str, newtons: float) -> None:
        self.sustained.append({"id": f"load:{name}", "category": "concentrated_force",
                               "target": {"type": "node", "node": node}, "direction": direction,
                               "magnitude": q(newtons, "N"), "dimension": "force", "provenance": PROVENANCE})

    def moment(self, name: str, node: str, direction: str, newton_metres: float) -> None:
        self.sustained.append({"id": f"load:{name}", "category": "concentrated_moment",
                               "target": {"type": "node", "node": node}, "direction": direction,
                               "magnitude": q(newton_metres, "N*m"), "dimension": "moment", "provenance": PROVENANCE})

    def move(self, name: str, support: str, dof: str, value: float) -> None:
        translation = dof.startswith("U")
        self.expansion.append({
            "id": f"load:{name}", "category": "imposed_displacement",
            "target": {"type": "support", "support": support, "dof": dof}, "direction": dof,
            "magnitude": q(value, "m" if translation else "rad"),
            "dimension": "displacement" if translation else "rotation", "provenance": PROVENANCE})

    def model(self) -> dict[str, Any]:
        name, e, nu, alpha, _ = self.material
        cases = []
        for case_id, loads in ((SUSTAINED, self.sustained), (EXPANSION, self.expansion)):
            if loads:
                cases.append({"id": case_id, "primitive_loads": loads, "provenance": PROVENANCE})
        return {
            "schema_version": "0.9.0",
            "document_kind": "openpipestress.product_preview.model",
            "project": {"id": f"project:{self.model_id}", "name": f"project:{self.model_id}",
                        "units": {"length": "m", "force": "N", "temperature": "degC", "angle": "rad",
                                  "stress": "Pa", "pressure": "Pa"}},
            "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
                                "rule_check": "not_performed_user_rule_inputs_missing",
                                "professional_acceptance": "not_provided"},
            "nodes": self.nodes,
            "pipe_segments": self.pipes,
            "materials": [{"id": name, "elastic_modulus": q(e, "Pa"), "shear_modulus": q(e / (2.0 * (1.0 + nu)), "Pa"),
                           "thermal_expansion_coefficient": q(alpha, "1/degC"), "temperature_points": [],
                           "provenance": PROVENANCE}],
            "supports": self.supports,
            "components": self.components,
            "load_cases": cases,
            "combinations": [],
        }


def _corner(low: bool) -> Callable[[tuple[float, float]], float]:
    """The low or the high end of each parameter range."""
    return lambda span: span[0] if low else span[1]


def section(unit_od: float, unit_dt: float) -> tuple[float, float, float]:
    """(OD mm, D/t, wall mm) from two unit samples; OD and D/t log-uniform over the family ranges."""
    od = scale(unit_od, *OD_RANGE, log=True)
    dt = scale(unit_dt, *DT_RANGE, log=True, digits=3)
    return od, dt, float(f"{od / dt:.4g}")


def section_properties(od_mm: float, wall_mm: float) -> tuple[float, float]:
    """(A m2, I m4) of the bare annulus."""
    ro, ri = od_mm / 2000.0, (od_mm - 2.0 * wall_mm) / 2000.0
    return math.pi * (cr_pow(ro, 2) - cr_pow(ri, 2)), math.pi / 4.0 * (cr_pow(ro, 4) - cr_pow(ri, 4))


def corner_bend(builder: Builder, name: str, before: str, ti, after_direction, radius_m: float, clear_after: float,
                k: float, od: float, wall: float) -> tuple[str, str, str, Any]:
    """A bend at tangent intersection `ti`, entered from node `before` and leaving along `after_direction`.

    Returns (near tangent node, far tangent node, bend pipe, the far straight's end position)."""
    incoming = _unit(_sub(ti, builder.positions[before]))
    outgoing = _unit(after_direction)
    angle = cr_acos(max(-1.0, min(1.0, sum(a * b for a, b in zip(incoming, outgoing)))))
    tangent = radius_m * cr_tan(angle / 2.0)
    near = builder.node(f"{name}-near", _sub(ti, _mul(incoming, tangent)))
    far = builder.node(f"{name}-far", _add(ti, _mul(outgoing, tangent)))
    bend = builder.bend(f"{name}", near, far, ti, radius_m, k, od, wall)
    return near, far, bend, _add(ti, _mul(outgoing, tangent + clear_after))


# -- closed forms (Euler-Bernoulli, binary64) ------------------------------------------------------------------------
def closed(quantity: str, component: str, case: str, value: float, unit: str, formula: str, **target: str) -> dict:
    return {"quantity": quantity, "component": component, "load_case": case, **target, "value": value, "unit": unit,
            "formula": formula, "basis": "Euler-Bernoulli closed form; invented inputs"}


def cantilever_closed_forms(b: Builder, anchor: str, tip: str, pipe: str, span: float, tip_force: float,
                            delta_t: float) -> list[dict]:
    _, e, _, alpha, _ = b.material
    area, inertia = section_properties(*b.sections[pipe])
    w = b.weight_per_length(pipe)
    ei = e * inertia
    return [
        closed("displacement", "y", SUSTAINED, tip_force * cr_pow(span, 3) / (3 * ei) + w * cr_pow(span, 4) / (8 * ei), "m",
               "P L^3/(3EI) + w L^4/(8EI)", node=tip),
        closed("rotation", "z", SUSTAINED, tip_force * cr_pow(span, 2) / (2 * ei) + w * cr_pow(span, 3) / (6 * ei), "rad",
               "P L^2/(2EI) + w L^3/(6EI)", node=tip),
        closed("reaction_force", "y", SUSTAINED, -(tip_force + w * span), "N", "-(P + w L)", support=anchor),
        closed("reaction_moment", "z", SUSTAINED, -(tip_force * span + w * cr_pow(span, 2) / 2), "N*m",
               "-(P L + w L^2/2)", support=anchor),
        closed("displacement", "x", EXPANSION, alpha * delta_t * span, "m", "alpha dT L", node=tip),
        closed("reaction_force", "x", EXPANSION, 0.0, "N", "0 (free thermal growth)", support=anchor),
    ]


# -- families -------------------------------------------------------------------------------------------------------
Spec = dict[str, Any]


def g1_straight(rng: random.Random) -> list[Spec]:
    specs: list[Spec] = []
    # Shear series: one span and OD, D/t 10, 20, 40, 100 (Design §11.3).
    for dt in (10.0, 20.0, 40.0, 100.0):
        specs.append({"config": "cantilever", "sampling": "series", "series": ["shear-dt"], "span": 3.0,
                      "od": 219.1, "dt": dt, "material": 0, "delta_t": 150.0, "tip_force": -2000.0})
    # Two opposite corners of (span, OD, D/t).
    specs.append({"config": "cantilever", "sampling": "corner", "span": 1.0, "od": OD_RANGE[0], "dt": DT_RANGE[0],
                  "material": 1, "delta_t": DELTA_T_RANGE[0], "tip_force": -500.0})
    specs.append({"config": "cantilever", "sampling": "corner", "span": 12.0, "od": OD_RANGE[1], "dt": DT_RANGE[1],
                  "material": 2, "delta_t": DELTA_T_RANGE[1], "tip_force": -20000.0})
    configs = ("guided", "continuous_2", "continuous_3", "fixed_fixed")
    for config, point in zip(configs, latin_hypercube(rng, 4, 6)):
        od, dt, _ = section(point[1], point[2])
        specs.append({"config": config, "sampling": "lhs", "span": scale(point[0], 1.0, 12.0, digits=3), "od": od,
                      "dt": dt, "material": min(2, int(point[3] * 3)), "delta_t": scale(point[4], *DELTA_T_RANGE),
                      "tip_force": -scale(point[5], 200.0, 20000.0, log=True, digits=3)})
    return specs


def build_g1(model_id: str, spec: Spec) -> tuple[dict, list[dict], dict]:
    b = Builder(model_id, spec["material"])
    od, dt = spec["od"], spec["dt"]
    wall = float(f"{od / dt:.4g}")
    span, config = spec["span"], spec["config"]
    closed_forms: list[dict] = []
    if config in ("cantilever", "guided"):
        a, mid, tip = b.node("A", (0, 0, 0)), b.node("M", (span / 2, 0, 0)), b.node("B", (span, 0, 0))
        pipes = b.run("P", [a, mid, tip], od, wall)
        anchor = b.anchor("A", a)
        if config == "guided":
            b.support("B-guide", tip, ["UY", "UZ"], "guide")
            b.force("mid", mid, "global_y", spec["tip_force"])
        else:
            b.force("tip", tip, "global_y", spec["tip_force"])
    elif config == "fixed_fixed":
        a, mid, tip = b.node("A", (0, 0, 0)), b.node("M", (span / 2, 0, 0)), b.node("B", (span, 0, 0))
        pipes = b.run("P", [a, mid, tip], od, wall)
        anchor = b.anchor("A", a)
        b.anchor("B", tip)
        b.force("mid", mid, "global_z", spec["tip_force"])
    else:
        spans = 2 if config == "continuous_2" else 3
        names = []
        for index in range(2 * spans + 1):
            names.append(b.node(f"N{index}", (index * span / 2, 0, 0)))
        pipes = b.run("P", names, od, wall)
        anchor = b.anchor("A", names[0])
        b.anchor("B", names[-1])
        for index in range(2, 2 * spans, 2):
            b.support(f"V{index}", names[index], ["UY"], None)
        b.force("mid", names[1], "global_y", spec["tip_force"])
    b.add_weight()
    b.add_thermal(spec["delta_t"])
    if config == "cantilever":
        # One straight per half; the closed form is the whole span with uniform section and weight.
        closed_forms = cantilever_closed_forms(b, anchor, tip, pipes[0], span, spec["tip_force"], spec["delta_t"])
    elif config == "fixed_fixed":
        _, e, _, alpha, _ = b.material
        area, _ = section_properties(od, wall)
        closed_forms = [closed("reaction_force", "x", EXPANSION, e * area * alpha * spec["delta_t"], "N",
                               "E A alpha dT (support on pipe at the start)", support=anchor)]
    # Collinear straights along X: the expansion case is axial only, so no input is inexact there.
    inexact = {SUSTAINED: [SHEAR], EXPANSION: []}
    return b.model(), closed_forms, inexact


def g2_bends(rng: random.Random) -> list[Spec]:
    specs = []
    grid = [(rd, angle) for rd in (1.0, 1.5, 3.0, 5.0) for angle in (45.0, 90.0)]
    for (rd, angle), point in zip(grid, latin_hypercube(rng, len(grid), 8)):
        od, dt, _ = section(point[0], point[1])
        common = {"rd": rd, "angle": angle, "od": od, "dt": dt, "clear_1": scale(point[2], 1.0, 5.0, digits=3),
                  "clear_2": scale(point[3], 1.0, 5.0, digits=3), "material": min(2, int(point[4] * 3)),
                  "delta_t": scale(point[5], *DELTA_T_RANGE), "force": -scale(point[6], 200.0, 20000.0, log=True,
                                                                              digits=3)}
        k_high = float(f"{3.0 + 12.0 * point[7]:.3g}")
        for k in (1.0, k_high):
            specs.append({**common, "k": k, "sampling": "corner", "series": [f"bend-k-rd{rd:g}-a{angle:g}"]})
    return specs


def build_g2(model_id: str, spec: Spec) -> tuple[dict, list[dict], dict]:
    b = Builder(model_id, spec["material"])
    od, wall = spec["od"], float(f"{spec['od'] / spec['dt']:.4g}")
    radius = spec["rd"] * od / 1000.0
    theta = math.radians(spec["angle"])
    tangent = radius * cr_tan(theta / 2.0)
    a = b.node("A", (0, 0, 0))
    ti = (spec["clear_1"] + tangent, 0.0, 0.0)
    near, far, _, end = corner_bend(b, "BEND1", a, ti, (cr_cos(theta), 0.0, cr_sin(theta)), radius,
                                    spec["clear_2"], spec["k"], od, wall)
    mid = b.node("M", _add(b.positions[far], _mul(_sub(end, b.positions[far]), 0.5)))
    tip = b.node("B", end)
    b.pipe("P1", a, near, od, wall)
    b.pipe("P2", far, mid, od, wall)
    b.pipe("P3", mid, tip, od, wall)
    b.anchor("A", a)
    b.anchor("B", tip)
    b.force("mid", mid, "global_y", spec["force"])
    b.add_weight()
    b.add_thermal(spec["delta_t"])
    return b.model(), [], {SUSTAINED: [SHEAR, BEND_K, BEND_SHEAR], EXPANSION: [SHEAR, BEND_K, BEND_SHEAR]}


def g3_loops(rng: random.Random) -> list[Spec]:
    specs = []
    for config in ("z", "u"):
        # Two opposite corners of (OD, D/t, R/D, k, leg lengths, delta T), then four Latin hypercube points.
        for low, material in ((True, 1), (False, 2)):
            pick = _corner(low)
            specs.append({"config": config, "sampling": "corner", "od": pick(OD_RANGE), "dt": pick(DT_RANGE),
                          "rd": pick((1.0, 5.0)), "k": pick((1.0, 10.0)), "clear_1": pick((1.0, 4.0)),
                          "height": pick((1.0, 4.0)), "width": pick((1.0, 4.0)), "material": material,
                          "delta_t": pick(DELTA_T_RANGE), "force": -1000.0})
        for point in latin_hypercube(rng, 4, 9):
            od, dt, _ = section(point[0], point[1])
            specs.append({"config": config, "sampling": "lhs", "od": od, "dt": dt,
                          "rd": scale(point[2], 1.0, 5.0, digits=3), "k": scale(point[3], 1.0, 10.0, digits=3),
                          "clear_1": scale(point[4], 1.0, 4.0, digits=3), "height": scale(point[5], 1.0, 4.0, digits=3),
                          "width": scale(point[6], 1.0, 4.0, digits=3), "material": min(2, int(point[7] * 3)),
                          "delta_t": scale(point[8], *DELTA_T_RANGE), "force": -1000.0})
    return specs


def _polyline(b: Builder, start, directions: list[tuple[float, ...]], clears: list[float], radius: float, k: float,
              od: float, wall: float, force_leg: int | None, force: float) -> tuple[str, str]:
    """Straights joined by bends along `directions`; `clears` are the straight lengths between tangent points (the
    first and last from the end nodes). A node at the middle of straight `force_leg` takes the force. Returns
    (first node, last node)."""
    first = b.node("A", start)
    current, position = first, start
    tangents = []
    for index in range(len(directions) - 1):
        d1, d2 = directions[index], directions[index + 1]
        angle = cr_acos(max(-1.0, min(1.0, sum(x * y for x, y in zip(d1, d2)))))
        tangents.append(radius * cr_tan(angle / 2.0))
    ti = _add(position, _mul(directions[0], clears[0] + tangents[0]))
    counter = 0
    for index in range(len(directions)):
        if index < len(directions) - 1:
            near, far, _, _ = corner_bend(b, f"BEND{index + 1}", current, ti, directions[index + 1], radius, 0.0, k,
                                          od, wall)
            stop = near
        else:
            stop = b.node("B", _add(b.positions[current], _mul(directions[index], clears[index])))
            far = None
        if force_leg == index:
            mid = b.node(f"F{index}", _mul(_add(b.positions[current], b.positions[stop]), 0.5))
            for start_node, end_node in ((current, mid), (mid, stop)):
                counter += 1
                b.pipe(f"P{counter}", start_node, end_node, od, wall)
            b.force("leg", mid, "global_y", force)
        else:
            counter += 1
            b.pipe(f"P{counter}", current, stop, od, wall)
        if far is None:
            return first, stop
        current = far
        if index + 2 < len(directions):
            ti = _add(b.positions[far], _mul(directions[index + 1], clears[index + 1] + tangents[index + 1]))
    raise AssertionError("unreachable")


def build_g3(model_id: str, spec: Spec) -> tuple[dict, list[dict], dict]:
    b = Builder(model_id, spec["material"])
    od, wall = spec["od"], float(f"{spec['od'] / spec['dt']:.4g}")
    radius = spec["rd"] * od / 1000.0
    x, z, mz = (1.0, 0.0, 0.0), (0.0, 0.0, 1.0), (0.0, 0.0, -1.0)
    if spec["config"] == "z":
        directions, clears = [x, z, x], [spec["clear_1"], spec["height"], spec["width"]]
        force_leg = 1
    else:
        directions = [x, z, x, mz, x]
        clears = [spec["clear_1"], spec["height"], spec["width"], spec["height"], spec["clear_1"]]
        force_leg = 2
    first, last = _polyline(b, (0.0, 0.0, 0.0), directions, clears, radius, spec["k"], od, wall, force_leg,
                            spec["force"])
    b.anchor("A", first)
    b.anchor("B", last)
    b.add_weight()
    b.add_thermal(spec["delta_t"])
    return b.model(), [], {SUSTAINED: [SHEAR, BEND_K, BEND_SHEAR], EXPANSION: [SHEAR, BEND_K, BEND_SHEAR]}


def g4_multiplane(rng: random.Random) -> list[Spec]:
    specs = []
    # Two opposite corners, then four Latin hypercube points.
    for low, material in ((True, 0), (False, 2)):
        pick = _corner(low)
        specs.append({"sampling": "corner", "od": pick(OD_RANGE), "dt": pick(DT_RANGE), "rd": pick((1.0, 5.0)),
                      "k": pick((1.0, 10.0)), "clear_1": pick((1.0, 4.0)), "rise": pick((1.0, 4.0)),
                      "clear_3": pick((1.0, 4.0)), "material": material, "delta_t": pick(DELTA_T_RANGE),
                      "force": -1500.0})
    for point in latin_hypercube(rng, 4, 9):
        od, dt, _ = section(point[0], point[1])
        specs.append({"sampling": "lhs", "od": od, "dt": dt, "rd": scale(point[2], 1.0, 5.0, digits=3),
                      "k": scale(point[3], 1.0, 10.0, digits=3), "clear_1": scale(point[4], 1.0, 4.0, digits=3),
                      "rise": scale(point[5], 1.0, 4.0, digits=3), "clear_3": scale(point[6], 1.0, 4.0, digits=3),
                      "material": min(2, int(point[7] * 3)), "delta_t": scale(point[8], *DELTA_T_RANGE),
                      "force": -1500.0})
    return specs


def build_g4(model_id: str, spec: Spec) -> tuple[dict, list[dict], dict]:
    b = Builder(model_id, spec["material"])
    od, wall = spec["od"], float(f"{spec['od'] / spec['dt']:.4g}")
    radius = spec["rd"] * od / 1000.0
    directions = [(1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)]
    first, last = _polyline(b, (0.0, 0.0, 0.0), directions, [spec["clear_1"], spec["rise"], spec["clear_3"]],
                            radius, spec["k"], od, wall, 2, spec["force"])
    b.anchor("A", first)
    b.anchor("B", last)
    b.add_weight()
    b.add_thermal(spec["delta_t"])
    return b.model(), [], {SUSTAINED: [SHEAR, BEND_K, BEND_SHEAR], EXPANSION: [SHEAR, BEND_K, BEND_SHEAR]}


def g5_tees(rng: random.Random) -> list[Spec]:
    specs = []
    points = latin_hypercube(rng, 6, 6)
    for index, ratio in enumerate((0.3, 0.5, 0.75, 1.0)):
        point = points[index]
        specs.append({"config": "in_plane_anchored", "sampling": "series", "series": ["tee-ratio"], "ratio": ratio,
                      "od": 323.9, "dt": 30.0, "run": 6.0, "branch": 3.0, "material": 0, "delta_t": 150.0,
                      "force": -scale(point[0], 500.0, 5000.0, digits=3)})
    for index, config in enumerate(("vertical_anchored", "vertical_free")):
        point = points[4 + index]
        od, dt, _ = section(point[0], point[1])
        specs.append({"config": config, "sampling": "lhs", "ratio": scale(point[2], 0.3, 1.0, digits=3), "od": od,
                      "dt": dt, "run": scale(point[3], 2.0, 10.0, digits=3), "branch": scale(point[4], 1.0, 4.0,
                                                                                               digits=3),
                      "material": min(2, int(point[5] * 3)), "delta_t": 100.0, "force": -1000.0})
    return specs


def build_g5(model_id: str, spec: Spec) -> tuple[dict, list[dict], dict]:
    b = Builder(model_id, spec["material"])
    od, wall = spec["od"], float(f"{spec['od'] / spec['dt']:.4g}")
    branch_od = float(f"{od * spec['ratio']:.4g}")
    branch_wall = float(f"{branch_od / spec['dt']:.4g}")
    run, branch = spec["run"], spec["branch"]
    a, tee, c = b.node("A", (0, 0, 0)), b.node("T", (run / 2, 0, 0)), b.node("C", (run, 0, 0))
    direction = (0.0, 0.0, 1.0) if spec["config"] == "in_plane_anchored" else (0.0, 1.0, 0.0)
    mid = b.node("M", _add(b.positions[tee], _mul(direction, branch / 2)))
    end = b.node("E", _add(b.positions[tee], _mul(direction, branch)))
    b.run("R", [a, tee, c], od, wall)
    b.run("BR", [tee, mid, end], branch_od, branch_wall)
    b.anchor("A", a)
    b.anchor("C", c)
    if spec["config"] == "vertical_free":
        b.force("branch-tip", end, "global_x", spec["force"])
    else:
        b.anchor("E", end)
        b.force("branch-mid", mid, "global_y" if spec["config"] == "in_plane_anchored" else "global_x", spec["force"])
    b.add_weight()
    b.add_thermal(spec["delta_t"])
    return b.model(), [], {SUSTAINED: [SHEAR], EXPANSION: [SHEAR]}


def g6_g7_supports(rng: random.Random) -> list[Spec]:
    specs = []
    for family, configs in (("G6", ("riser_guides", "riser_spring", "riser_stop")),
                            ("G7", ("run_mix", "run_springs", "run_stops"))):
        for config, point in zip(configs, latin_hypercube(rng, 3, 6)):
            od, dt, _ = section(point[0], point[1])
            specs.append({"family": family, "config": config, "sampling": "lhs", "od": od, "dt": dt,
                          "length": scale(point[2], 6.0, 16.0, digits=3),
                          "spring": scale(point[3], 1.0e5, 1.0e9, log=True, digits=3),
                          "material": min(2, int(point[4] * 3)), "delta_t": scale(point[5], *DELTA_T_RANGE),
                          "force": -2000.0})
    return specs


def build_g6_g7(model_id: str, spec: Spec) -> tuple[dict, list[dict], dict]:
    b = Builder(model_id, spec["material"])
    od, wall = spec["od"], float(f"{spec['od'] / spec['dt']:.4g}")
    length, config = spec["length"], spec["config"]
    if config.startswith("riser"):
        # A vertical riser on an anchor, guided along its height, turning into a horizontal run to an anchor.
        radius = 1.5 * od / 1000.0
        points = [b.node(f"R{index}", (0.0, index * length / 4.0, 0.0)) for index in range(4)]
        ti = (0.0, length, 0.0)
        near, far, _, end = corner_bend(b, "BEND1", points[-1], ti, (1.0, 0.0, 0.0), radius, 3.0, 1.0, od, wall)
        tip = b.node("B", end)
        b.run("P", [*points, near], od, wall)
        b.pipe("H1", far, tip, od, wall)
        b.anchor("A", points[0])
        b.anchor("B", tip)
        b.support("G1", points[1], ["UX", "UZ"], "guide")
        if config == "riser_spring":
            b.spring("S2", points[2], "UY", spec["spring"])
        else:
            b.support("G2", points[2], ["UX", "UZ"], "guide")
        if config == "riser_stop":
            b.support("L3", points[3], ["UY"], "line_stop")
        else:
            b.force("riser", points[3], "global_x", spec["force"])
        inexact = {SUSTAINED: [SHEAR, BEND_K, BEND_SHEAR], EXPANSION: [SHEAR, BEND_K, BEND_SHEAR]}
    else:
        # A horizontal run along X between anchors with intermediate supports of each linear kind.
        points = [b.node(f"N{index}", (index * length / 6.0, 0.0, 0.0)) for index in range(7)]
        b.run("P", points, od, wall)
        b.anchor("A", points[0])
        b.anchor("B", points[-1])
        if config == "run_mix":
            b.support("V2", points[2], ["UY"], None)
            b.support("G3", points[3], ["UY", "UZ"], "guide")
            b.spring("S4", points[4], "UY", spec["spring"])
            b.force("lateral", points[5], "global_z", spec["force"])
        elif config == "run_springs":
            b.spring("S2", points[2], "UY", spec["spring"])
            b.spring("S4", points[4], "UZ", spec["spring"])
            b.force("vertical", points[3], "global_y", spec["force"])
        else:
            b.support("L2", points[2], ["UX"], "line_stop")
            b.support("V4", points[4], ["UY"], None)
            b.force("lateral", points[3], "global_z", spec["force"])
        inexact = {SUSTAINED: [SHEAR], EXPANSION: []}
    b.add_weight()
    b.add_thermal(spec["delta_t"])
    return b.model(), [], inexact


def g8_movements(rng: random.Random) -> list[Spec]:
    specs = []
    configs = (("move_uy", "UY"), ("move_uz", "UZ"), ("rotate_rz", "RZ"), ("l_general", None))
    for (config, dof), point in zip(configs, latin_hypercube(rng, 4, 6)):
        od, dt, _ = section(point[0], point[1])
        specs.append({"config": config, "dof": dof, "sampling": "lhs", "od": od, "dt": dt,
                      "span": scale(point[2], 2.0, 10.0, digits=3),
                      "movement": scale(point[3], 0.002, 0.05, digits=3),
                      "rotation": scale(point[4], 0.001, 0.02, digits=3), "material": min(2, int(point[5] * 3))})
    return specs


def build_g8(model_id: str, spec: Spec) -> tuple[dict, list[dict], dict]:
    b = Builder(model_id, spec["material"])
    od, wall = spec["od"], float(f"{spec['od'] / spec['dt']:.4g}")
    span = spec["span"]
    _, e, _, _, _ = b.material
    _, inertia = section_properties(od, wall)
    ei = e * inertia
    closed_forms: list[dict] = []
    if spec["config"] != "l_general":
        a, mid, tip = b.node("A", (0, 0, 0)), b.node("M", (span / 2, 0, 0)), b.node("B", (span, 0, 0))
        b.run("P", [a, mid, tip], od, wall)
        anchor_a, anchor_b = b.anchor("A", a), b.anchor("B", tip)
        dof = spec["dof"]
        if dof == "RZ":
            phi = spec["rotation"]
            b.move("b-rz", anchor_b, "RZ", phi)
            closed_forms = [
                closed("reaction_moment", "z", EXPANSION, 4 * ei * phi / span, "N*m", "4EI phi/L", support=anchor_b),
                closed("reaction_moment", "z", EXPANSION, 2 * ei * phi / span, "N*m", "2EI phi/L", support=anchor_a),
                closed("reaction_force", "y", EXPANSION, -6 * ei * phi / cr_pow(span, 2), "N", "-6EI phi/L^2",
                       support=anchor_b),
                closed("rotation", "z", EXPANSION, -phi / 4, "rad",
                       "-phi/4 at midspan (v = phi (x^3/L^2 - x^2/L))", node=mid),
            ]
        else:
            delta = spec["movement"]
            b.move(f"b-{dof.lower()}", anchor_b, dof, delta)
            axis = dof[1].lower()
            closed_forms = [
                closed("reaction_force", axis, EXPANSION, 12 * ei * delta / cr_pow(span, 3), "N", "12EI delta/L^3",
                       support=anchor_b),
                closed("reaction_force", axis, EXPANSION, -12 * ei * delta / cr_pow(span, 3), "N", "-12EI delta/L^3",
                       support=anchor_a),
                closed("displacement", axis, EXPANSION, delta / 2, "m", "delta/2 at midspan", node=mid),
            ]
    else:
        radius = 1.5 * od / 1000.0
        a = b.node("A", (0, 0, 0))
        near, far, _, end = corner_bend(b, "BEND1", a, (span + radius, 0.0, 0.0), (0.0, 0.0, 1.0), radius,
                                        span / 2, 1.0, od, wall)
        tip = b.node("B", end)
        b.pipe("P1", a, near, od, wall)
        b.pipe("P2", far, tip, od, wall)
        b.anchor("A", a)
        anchor_b = b.anchor("B", tip)
        for dof, value in (("UX", spec["movement"]), ("UY", -spec["movement"] / 2), ("UZ", spec["movement"] / 4),
                           ("RY", spec["rotation"])):
            b.move(f"b-{dof.lower()}", anchor_b, dof, value)
    b.add_weight()
    bends = [BEND_K, BEND_SHEAR] if spec["config"] == "l_general" else []
    return b.model(), closed_forms, {SUSTAINED: [SHEAR, *bends], EXPANSION: [SHEAR, *bends]}


FAMILIES: tuple[tuple[str, str, Callable[[random.Random], list[Spec]], Callable[..., Any]], ...] = (
    ("G1", "straight spans: cantilever, guided, continuous, fixed-fixed", g1_straight, build_g1),
    ("G2", "single bends: anchored L, R/D 1-5, 45 and 90 degrees, user k", g2_bends, build_g2),
    ("G3", "Z and U loops", g3_loops, build_g3),
    ("G4", "multi-plane runs", g4_multiplane, build_g4),
    ("G5", "branch tees", g5_tees, build_g5),
    ("G6-G7", "risers and support mixes: anchors, guides, line stops, vertical supports, springs", g6_g7_supports,
     build_g6_g7),
    ("G8", "anchor displacement and rotation", g8_movements, build_g8),
)

SERIES = {
    "shear-dt": {"swept": "dt", "hypothesis": SHEAR,
                 "description": "cantilevers of one span and OD with D/t 10, 20, 40, 100"},
    "tee-ratio": {"swept": "ratio", "hypothesis": "branch_flexibility",
                  "description": "anchored in-plane tees with branch/run OD ratio 0.3, 0.5, 0.75, 1.0"},
}


def _dump(value: Any) -> str:
    return json.dumps(value, indent=2, sort_keys=True, ensure_ascii=True, allow_nan=False) + "\n"


def generate(seed: int = DEFAULT_SEED) -> dict[str, str]:
    """{relative path: file text} of the whole benchmark set for `seed`."""
    rng = random.Random(seed)
    files: dict[str, str] = {}
    entries = []
    series: dict[str, dict[str, Any]] = {}
    for family, description, sample, build in FAMILIES:
        for index, spec in enumerate(sample(rng)):
            sub_family = spec.get("family", family)
            model_id = f"cpb-{sub_family.lower()}-{index + 1:02d}"
            if family == "G6-G7":
                model_id = f"cpb-{sub_family.lower()}-{index % 3 + 1:02d}"
            model, closed_forms, inexact = build(model_id, spec)
            operating = sorted(set(inexact.get(SUSTAINED, [])) | set(inexact.get(EXPANSION, [])))
            parameters = {key: value for key, value in spec.items() if key not in ("sampling", "series", "family")}
            document = {
                "document_kind": MODEL_KIND,
                "model_id": model_id,
                "family": sub_family,
                "family_description": description,
                "generator": {"version": GENERATOR_VERSION, "seed": seed},
                "sampling": spec["sampling"],
                "parameters": parameters,
                "series": spec.get("series", []),
                "piping_code": PIPING_CODE,
                "reference_temperature_c": REFERENCE_TEMPERATURE_C,
                "inexact_inputs": {SUSTAINED: inexact.get(SUSTAINED, []), EXPANSION: inexact.get(EXPANSION, []),
                                   OPERATING: operating},
                "closed_form": closed_forms,
                "provenance": {"origin": "invented", "license": "project-original",
                               "redistribution_status": "public_permissive", "produced_by_caepipe": False,
                               "contributor_note": PROVENANCE},
                "model": model,
            }
            text = _dump(document)
            relative = f"models/{model_id}.json"
            files[relative] = text
            for name in spec.get("series", []):
                series.setdefault(name, {**SERIES.get(name, {"swept": "k", "hypothesis": BEND_K,
                                                             "description": "one bend geometry with k = 1 and a "
                                                                            "higher k"}), "members": []})
                series[name]["members"].append({"model_id": model_id, "value": spec.get(series[name]["swept"])})
            entries.append({"model_id": model_id, "family": sub_family, "sampling": spec["sampling"],
                            "parameters": parameters, "series": spec.get("series", []), "file": relative,
                            "model_hash": canonical_model_hash(model),
                            "file_sha256": hashlib.sha256(text.encode("ascii")).hexdigest()})
    files["manifest.json"] = _dump({
        "document_kind": MANIFEST_KIND,
        "generator": {"version": GENERATOR_VERSION, "module": "tools/caepipe_correlation/benchmark_models.py",
                      "seed": seed, "rng": "random.Random(seed).random() only"},
        "tranche": "1",
        "requirement": "RT-V2 (docs/RELEASE_TARGET_v1.0.md section 3); DEL-17-05 Design section 11",
        "piping_code": PIPING_CODE,
        "materials": [{"id": name, "elastic_modulus_pa": e, "poisson_ratio": nu, "thermal_expansion_per_k": alpha,
                       "density_kg_per_m3": rho} for name, e, nu, alpha, rho in MATERIALS],
        "series": dict(sorted(series.items())),
        "models": entries,
        "provenance": {"origin": "invented", "produced_by_caepipe": False,
                       "note": "invented inputs only; no result of either program is recorded here"},
    })
    return files


def write(files: dict[str, str], out: Path) -> None:
    (out / "models").mkdir(parents=True, exist_ok=True)
    for stale in (out / "models").glob("*.json"):
        if f"models/{stale.name}" not in files:
            stale.unlink()
    for relative, text in files.items():
        (out / relative).write_bytes(text.encode("ascii"))


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--seed", type=int, default=DEFAULT_SEED)
    parser.add_argument("--out", type=Path, default=DEFAULT_OUT)
    parser.add_argument("--check", action="store_true", help="compare with the files in --out; write nothing")
    args = parser.parse_args(argv)
    files = generate(args.seed)
    if args.check:
        stored = {f"models/{path.name}" for path in (args.out / "models").glob("*.json")}
        differ = sorted(relative for relative, text in files.items()
                        if not (args.out / relative).is_file() or (args.out / relative).read_bytes() != text.encode())
        extra = sorted(stored - set(files))
        for relative in differ + extra:
            print(f"differs: {relative}")
        return 1 if differ or extra else 0
    write(files, args.out)
    print(f"{len(files) - 1} models and manifest.json written to {args.out}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
