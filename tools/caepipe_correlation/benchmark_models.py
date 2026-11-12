"""Seeded generator of the invented CAEPIPE benchmark models, tranches 1 and 2 (DEL-17-05, REQ-018 and AC-007;
the "Benchmark method" section of docs/RELEASE_TARGET_v1.0.md).

From the project root:

    python3 -m tools.caepipe_correlation.benchmark_models [--seed N] [--out DIR] [--check]

Writes one SWBPIPE model file per benchmark model to `<out>/models/<model_id>.json` and the
`<out>/manifest.json` that lists each model's seed, family, parameters, sampling, series and hashes. The
default output is the tracked `validation/caepipe_benchmark/`. Every model is invented: geometry, sections,
materials and loads come from the seed and the family definitions below, never from a library, catalogue,
standard or CAEPIPE output. `--check` regenerates in memory and compares with the tracked files byte for byte
(exit 1 on any difference).

Tranche 1, families G1-G8: straight spans, single bends, Z and U loops, multi-plane runs, branch tees, risers,
support mixes and anchor movements. G1-G4 take their corner or series points plus a Latin hypercube drawn from
`random.Random(seed)`; G5-G8 enumerate their configurations (and G5 its tee series) and sample the rest by the
hypercube. The generator uses only `random()` from it, whose sequence for an integer seed is stable across Python
versions, and prints every float through `repr`. Its transcendental functions (exp, log, sin, cos, tan, acos) and
integer powers are correctly rounded, computed in decimal at 60 digits and rounded once, never by the platform's
libm, so the same seed gives the same bytes on every platform.
Closed-form Euler-Bernoulli values are attached to the straight members whose response has one (G1, G8); they
are the third, independent value of the comparison.

Tranche 2, families P1-P6, is pressurised: closed straights between anchors, closed cantilevers, cantilevers with
one non-transferring terminal, single bends, Z runs and one tee, each on the exact pressure contract with its
pressure regions in the sustained case beside weight and its thermal change in a case of its own. Its families
are appended after tranche 1's, so tranche 1 consumes the same draws and its files keep their bytes. Every
tranche 2 model belongs to two series over the same model file (`PRESSURE_RUN_SERIES`): `pressure-default` and
`pressure-elongation-on`, the latter a variant run of the same file; `series_document` gives the document each run
solves and screens. Closed forms derived from the Lame closed-cylinder stresses and Hooke's law are attached per
series state where the state changes them.

SWBPIPE's side of a tranche 1 model is solved on the exact route; the side that counts for exit criterion 11 is the
Timoshenko one (`SIDES`). `exact_route_document(document, shear_model)` gives that side's document: the tracked model
restated in the form the exact route requires (`exact_route_model`), with no mechanics input changed, and the closed
forms valid for the shear model (`timoshenko_closed_forms` adds the shear term where it enters). The two shear models
form the series `exact-shear-model`; the tracked pressure-free document is the labelled second series
`pressure-free-route`. The twins are derived on demand, so the tracked files keep their bytes; the manifest records,
per model and member, the twin's canonical model hash, its closed forms, and whether the writer writes the twin's model
file byte for byte as it writes the tracked one's (`model_file_condition`), and carries the choices of that side as
they were recorded before any exact-route result existed (`EXACT_ROUTE_CHOICES`).

Nothing here runs either program, and nothing here records a result.
"""

from __future__ import annotations

import argparse
import copy
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
from core.handoff.caepipe_mbf import render_caepipe_mbf_si  # noqa: E402


GENERATOR_VERSION = "caepipe-benchmark-generator-1"
MANIFEST_KIND = "openpipestress.caepipe_benchmark.manifest"
MODEL_KIND = "openpipestress.caepipe_benchmark.model"
DEFAULT_SEED = 20261028
DEFAULT_OUT = PROJECT / "validation" / "caepipe_benchmark"
# The piping code every benchmark MBF is written with: an ASME B31.1 edition that predates B31J, so
# OPCODE's B31J off is honoured; the current B31.1 token locks B31J on (Analysis).
PIPING_CODE = "B311-80"
REFERENCE_TEMPERATURE_C = 20.0
GRAVITY = 9.80665
SUSTAINED = "case:sustained"
EXPANSION = "case:expansion"
OPERATING = "operating"
SIX = ["UX", "UY", "UZ", "RX", "RY", "RZ"]
# The text every tranche 1 model carries; it is kept so that their bytes do not change.
PROVENANCE = ("invented CAEPIPE benchmark input (DEL-17-05, RT-V2); seeded generator; not library, component, "
              "catalog, standard or code-rule data")

# Three invented temperature-independent materials: E, Poisson ratio, expansion, mass density.
MATERIALS = (
    ("material:bench-a", 200.0e9, 0.30, 12.0e-6, 7850.0),
    ("material:bench-b", 195.0e9, 0.30, 16.0e-6, 7900.0),
    ("material:bench-c", 70.0e9, 0.33, 23.0e-6, 2700.0),
)
OD_RANGE = (60.0, 610.0)  # mm
DT_RANGE = (10.0, 100.0)  # D/t
DELTA_T_RANGE = (50.0, 250.0)  # K
# Inputs that cannot be held exactly equal on both sides; each is carried by a series.
SHEAR = "shear_deformation"
BEND_K = "bend_flexibility_application"
BEND_SHEAR = "bend_transverse_shear"

# -- tranche 2: pressure ---------------------------------------------------------------------------------------------
PRESSURE_PROVENANCE = ("invented CAEPIPE benchmark input (DEL-17-05, tranche 2); seeded generator; not library, "
                       "component, catalog, standard or code-rule data")
# The exact pressure contract (DEL-04-07), with the contract's default shear model.
PRESSURE_CONTRACT = {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "timoshenko_annulus_energy_v1"}
PRESSURE_BASIS = "internal_differential_zero_external_v1"
TRANSFERS = "transfers_to_wall"
COMPENSATED = "separately_supported_or_compensated"
# Internal gauge pressure, invented: 1 to 50 bar, log-uniform, three significant digits, authored in whole pascals
# so that the writer's bar value is exact.
PRESSURE_RANGE_BAR = (1.0, 50.0)
# Inputs of the pressurised cases that no admitted public page settles for the target program, so they are not held
# equal: what its Bourdon option does to the axial growth of straight pipe (carried by the pressure-default and
# pressure-elongation-on runs of each file); whether and where it applies the end thrust of pressure, which the MBF
# cannot state because it carries no terminal closure (carried by the pressure-closure series, whose members are the
# same target-program input with SWBPIPE's terminal closure changed); and, on bends, the bend opening its Bourdon
# option may add, which SWBPIPE does not model (DEL-04-07 REQ-009 is off).
ELONGATION = "pressure_elongation_convention"
THRUST = "pressure_thrust_convention"
BEND_BOURDON = "bend_bourdon_opening"
SERIES_DEFAULT = "pressure-default"
SERIES_ELONGATION_ON = "pressure-elongation-on"
# The two runs of every tranche 2 model file. Each is a series over the same files with one parameter, the pressure
# elongation state; the second is a variant run of the file the first runs.
PRESSURE_RUN_SERIES = {
    SERIES_DEFAULT: {
        "kind": "run_state", "parameter": "pressure_elongation", "value": "off", "hypothesis": ELONGATION,
        "caepipe": "the MBF as the writer writes it: OPPRES with the Bourdon option off",
        "swbpipe": ("the tracked model document as it is, without model_settings.pressure_elongation, solved on a "
                    "build whose model document has that setting (absent, it solves with pressure elongation off)"),
        "description": "every tranche 2 model, pressure elongation off on both sides as far as the inputs state"},
    SERIES_ELONGATION_ON: {
        "kind": "run_state", "parameter": "pressure_elongation", "value": "on", "hypothesis": ELONGATION,
        "variant_of": SERIES_DEFAULT,
        "caepipe": ("the SAME MBF file, with the Bourdon option switched on by hand in the target program before the "
                    "analysis: a manual change the run's record must state"),
        "swbpipe": ("series_document(document, 'pressure-elongation-on'): the tracked model with "
                    "model_settings.pressure_elongation naming the model's elongation_load_case"),
        "description": ("every tranche 2 model, pressure elongation on; the target program forms its expansion case "
                        "from its operating and sustained cases, so a difference in that case on this run is triaged "
                        "under pressure_elongation_convention")},
}


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
    """One invented SWBPIPE preview model (model document 0.9.0), built node by node in metres. With `pressure`, the
    model takes the exact pressure contract: an E and Poisson ratio material basis, explicit zero corrosion
    allowances, and the regions of `region` in the sustained case (the expansion case lists none)."""

    def __init__(self, model_id: str, material: int, *, pressure: bool = False) -> None:
        self.model_id = model_id
        self.material = MATERIALS[material]
        self.pressure = pressure
        self.provenance = PRESSURE_PROVENANCE if pressure else PROVENANCE
        self.regions: list[dict[str, Any]] = []
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
        self.nodes.append({"id": node_id, "position": dict(zip("xyz", position)), "provenance": self.provenance})
        return node_id

    def pipe(self, name: str, start: str, end: str, od_mm: float, wall_mm: float, *, y_reference=None) -> str:
        pipe_id = f"pipe:{name}"
        direction = _unit(_sub(self.positions[end], self.positions[start]))
        self.pipes.append({
            "id": pipe_id, "from": start, "to": end, "material": self.material[0],
            "section": {"outside_diameter": q(od_mm / 1000.0, "m"), "wall_thickness": q(wall_mm / 1000.0, "m")},
            "y_reference": y_reference or _perpendicular(direction), "provenance": self.provenance,
        })
        if self.pressure:
            self.pipes[-1]["section"]["corrosion_allowance"] = q(0.0, "m")
        self.sections[pipe_id] = (od_mm, wall_mm)
        return pipe_id

    def region(self, name: str, members: list[str], pressure_pa: float, start: tuple[str, str],
               end: tuple[str, str]) -> None:
        """One pressure region of the sustained case: its member pipes in chain order and its two terminals, each
        (node, closure transfer)."""
        self.regions.append({
            "id": f"region:{name}", "member_pipe_ids": list(members), "pressure_basis": PRESSURE_BASIS,
            "pressure": q(pressure_pa, "Pa"), "provenance": self.provenance,
            "terminals": [{"node_ref": node, "closure_transfer": closure, "provenance": self.provenance}
                          for node, closure in (start, end)]})

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
            "id": f"component:{name}", "kind": "bend", "node": near, "provenance": self.provenance,
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
        entry = {"id": support_id, "node": node, "restraints": list(restraints), "provenance": self.provenance}
        if family is not None:
            entry["family"] = family
        self.supports.append(entry)
        return support_id

    def spring(self, name: str, node: str, dof: str, stiffness_n_per_m: float) -> str:
        support_id = f"support:{name}"
        self.supports.append({"id": support_id, "node": node, "family": "spring", "restraints": [dof],
                              "stiffness": {"dof": dof, "value": q(stiffness_n_per_m, "N/m")},
                              "provenance": self.provenance})
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
                "provenance": self.provenance + "; w = -rho A g with g = 9.80665 m/s2 and A the bare metal area"})

    def add_thermal(self, delta_t: float) -> None:
        for pipe in self.pipes:
            self.expansion.append({
                "id": f"load:thermal:{pipe['id']}", "category": "thermal",
                "target": {"type": "element", "pipe": pipe["id"]}, "direction": "global_x",
                "magnitude": q(delta_t, "degC"), "dimension": "temperature_interval", "provenance": self.provenance})

    def force(self, name: str, node: str, direction: str, newtons: float) -> None:
        self.sustained.append({"id": f"load:{name}", "category": "concentrated_force",
                               "target": {"type": "node", "node": node}, "direction": direction,
                               "magnitude": q(newtons, "N"), "dimension": "force", "provenance": self.provenance})

    def moment(self, name: str, node: str, direction: str, newton_metres: float) -> None:
        self.sustained.append({"id": f"load:{name}", "category": "concentrated_moment",
                               "target": {"type": "node", "node": node}, "direction": direction,
                               "magnitude": q(newton_metres, "N*m"), "dimension": "moment", "provenance": self.provenance})

    def move(self, name: str, support: str, dof: str, value: float) -> None:
        translation = dof.startswith("U")
        self.expansion.append({
            "id": f"load:{name}", "category": "imposed_displacement",
            "target": {"type": "support", "support": support, "dof": dof}, "direction": dof,
            "magnitude": q(value, "m" if translation else "rad"),
            "dimension": "displacement" if translation else "rotation", "provenance": self.provenance})

    def model(self) -> dict[str, Any]:
        name, e, nu, alpha, _ = self.material
        cases = []
        for case_id, loads in ((SUSTAINED, self.sustained), (EXPANSION, self.expansion)):
            if loads:
                cases.append({"id": case_id, "primitive_loads": loads, "provenance": self.provenance})
                if self.pressure:
                    cases[-1]["pressure_regions"] = self.regions if case_id == SUSTAINED else []
        if self.pressure:
            return {
                "schema_version": "0.9.0",
                "document_kind": "openpipestress.product_preview.model",
                "pressure_contract": dict(PRESSURE_CONTRACT),
                "project": {"id": f"project:{self.model_id}", "name": f"project:{self.model_id}",
                            "units": {"length": "m", "force": "N", "temperature": "degC", "angle": "rad",
                                      "stress": "Pa", "pressure": "Pa"}},
                "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
                                    "rule_check": "not_performed_user_rule_inputs_missing",
                                    "professional_acceptance": "not_provided"},
                "nodes": self.nodes,
                "pipe_segments": self.pipes,
                "materials": [{"id": name, "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
                               "elastic_modulus": q(e, "Pa"), "poisson_ratio": q(nu, "1"),
                               "thermal_expansion_coefficient": q(alpha, "1/degC"), "temperature_points": [],
                               "provenance": self.provenance}],
                "supports": self.supports,
                "components": self.components,
                "load_cases": cases,
                "combinations": [],
            }
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
                           "provenance": self.provenance}],
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
def closed(quantity: str, component: str, case: str, value: float, unit: str, formula: str, *,
           basis: str = "Euler-Bernoulli closed form; invented inputs", **target: str) -> dict:
    return {"quantity": quantity, "component": component, "load_case": case, **target, "value": value, "unit": unit,
            "formula": formula, "basis": basis}


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
    # Shear series: one span and OD, D/t 10, 20, 40, 100.
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


# -- tranche 2: pressurised families ---------------------------------------------------------------------------------
# A family builder of tranche 2 returns (model, closed forms valid in every series state, inexact inputs,
# {series: closed forms valid in that series state only}).
#
# Closed forms of pressure (derived here; no product code is imported for them). A straight pipe of outside radius
# r_o and inside radius r_i, internal gauge pressure p, zero external pressure, isotropic and linear elastic (E, nu):
#   Lame's thick-cylinder solution gives sigma_r = a - b/r^2 and sigma_t = a + b/r^2 with sigma_r(r_i) = -p and
#   sigma_r(r_o) = 0, so a = p r_i^2/(r_o^2 - r_i^2) and sigma_r + sigma_t = 2a = 2 p A_i/A_s at every radius, where
#   A_i = pi r_i^2 is the bore area and A_s = pi (r_o^2 - r_i^2) the wall area.
#   With both ends closed and the end load carried by the wall (both terminals transfer to the wall), axial
#   equilibrium of a cut through the closed pipe gives sigma_z A_s = p A_i, so sigma_z = p A_i/A_s.
#   Hooke's law: eps_z = (sigma_z - nu (sigma_r + sigma_t))/E = (1 - 2 nu) p A_i/(E A_s), the closed-end free
#   pressure strain, uniform along the run.
# Pressure elongation ON (the strain is applied): a run between two anchors is held at its length, so each anchor
# carries E A_s eps_z = (1 - 2 nu) p A_i; a cantilever's free end moves by eps_z L and its anchor carries nothing.
# Pressure elongation OFF (the strain is not applied): a closed region with both terminals transferring is
# self-equilibrated (its end loads cancel through the wall) and nothing else acts axially, so reactions and axial
# displacement are zero.
# Unbalanced thrust (one terminal separately supported or compensated, the other transferring): the transferring
# terminal's end load p A_i acts on the pipe at that terminal, along the member away from the region; on a
# cantilever it is statically determinate, so the anchor carries -p A_i along that direction whatever the axial
# strain law, in both series states.
# Sign convention, as for tranche 1's closed forms: a reaction is the support's action on the pipe in global axes;
# a displacement is the node's, in global axes. Every straight here runs along +X from the anchor at the origin.
PRESSURE_BASIS_TEXT = "Lame closed cylinder, axial equilibrium and Hooke's law; invented inputs"
STATICS_BASIS_TEXT = "Statics of a cantilever under an end load; invented inputs"


def pressure_pa(bar: float) -> float:
    """A bar value of three significant digits in whole pascals (exact in binary64 and in the writer's bar)."""
    return float(round(bar * 1.0e5))


def pressure_areas(od_mm: float, wall_mm: float) -> tuple[float, float]:
    """(A_i bore area m2, A_s wall area m2)."""
    ro, ri = od_mm / 2000.0, (od_mm - 2.0 * wall_mm) / 2000.0
    return math.pi * cr_pow(ri, 2), math.pi * (cr_pow(ro, 2) - cr_pow(ri, 2))


def _pressure_inexact(*, bends: bool, shear: bool = True) -> dict[str, list[str]]:
    sustained = [SHEAR] if shear else []
    expansion: list[str] = []
    if bends:
        sustained += [BEND_K, BEND_SHEAR, BEND_BOURDON]
        expansion = [SHEAR, BEND_K, BEND_SHEAR]
    return {SUSTAINED: [*sustained, ELONGATION, THRUST], EXPANSION: expansion}


def _straight_spec(point: list[float], config: str, sampling: str) -> Spec:
    """A straight run's parameters from six unit samples: span, OD, D/t, material, pressure, delta T."""
    od, dt, _ = section(point[1], point[2])
    return {"config": config, "sampling": sampling, "span": scale(point[0], 1.0, 12.0, digits=3), "od": od, "dt": dt,
            "material": min(2, int(point[3] * 3)),
            "pressure_bar": scale(point[4], *PRESSURE_RANGE_BAR, log=True, digits=3),
            "delta_t": scale(point[5], *DELTA_T_RANGE)}


def p1_closed_straights(rng: random.Random) -> list[Spec]:
    specs = []
    # Two opposite corners of (span, OD, D/t, pressure, delta T), then four Latin hypercube points.
    for low, material in ((True, 1), (False, 2)):
        pick = _corner(low)
        specs.append({"config": "anchored", "sampling": "corner", "span": pick((1.0, 12.0)), "od": pick(OD_RANGE),
                      "dt": pick(DT_RANGE), "material": material, "pressure_bar": pick(PRESSURE_RANGE_BAR),
                      "delta_t": pick(DELTA_T_RANGE)})
    specs += [_straight_spec(point, "anchored", "lhs") for point in latin_hypercube(rng, 4, 6)]
    return specs


def build_p1(model_id: str, spec: Spec) -> tuple[dict, list[dict], dict, dict]:
    """One member between two anchors, closed at both (both terminals transfer to the wall)."""
    b = Builder(model_id, spec["material"], pressure=True)
    od, wall = spec["od"], float(f"{spec['od'] / spec['dt']:.4g}")
    span, p = spec["span"], pressure_pa(spec["pressure_bar"])
    a, end = b.node("A", (0, 0, 0)), b.node("B", (span, 0, 0))
    pipe = b.pipe("P1", a, end, od, wall)
    anchor_a, anchor_b = b.anchor("A", a), b.anchor("B", end)
    b.region("R1", [pipe], p, (a, TRANSFERS), (end, TRANSFERS))
    b.add_weight()
    b.add_thermal(spec["delta_t"])
    _, e, nu, alpha, _ = b.material
    bore, wall_area = pressure_areas(od, wall)
    thrust = (1.0 - 2.0 * nu) * p * bore
    every = [closed("reaction_force", "x", EXPANSION, e * wall_area * alpha * spec["delta_t"], "N",
                    "E A_s alpha dT (support on pipe at the start)", support=anchor_a)]
    by_state = {
        SERIES_DEFAULT: [
            closed("reaction_force", "x", SUSTAINED, 0.0, "N", "0 (closed region, elongation off)",
                   basis=PRESSURE_BASIS_TEXT, support=anchor_a),
            closed("reaction_force", "x", SUSTAINED, 0.0, "N", "0 (closed region, elongation off)",
                   basis=PRESSURE_BASIS_TEXT, support=anchor_b)],
        SERIES_ELONGATION_ON: [
            closed("reaction_force", "x", SUSTAINED, thrust, "N", "(1 - 2 nu) p A_i (support on pipe at the start)",
                   basis=PRESSURE_BASIS_TEXT, support=anchor_a),
            closed("reaction_force", "x", SUSTAINED, -thrust, "N", "-(1 - 2 nu) p A_i (support on pipe at the end)",
                   basis=PRESSURE_BASIS_TEXT, support=anchor_b)],
    }
    return b.model(), every, _pressure_inexact(bends=False), by_state


def p2_p3_cantilevers(rng: random.Random) -> list[Spec]:
    """Four geometries, each built closed (P2) and with one terminal separately supported or compensated (P3). The
    first two pairs are pressure-closure series: one target-program input, SWBPIPE's anchor terminal changed."""
    points = latin_hypercube(rng, 4, 6)
    closed_specs, open_specs = [], []
    for index, point in enumerate(points):
        base = _straight_spec(point, "closed", "lhs")
        series = [f"pressure-closure-{index + 1:02d}"] if index < 2 else []
        closed_specs.append({**base, "family": "P2", "series": series, "anchor_closure": TRANSFERS})
        open_config = "open_at_anchor" if index < 2 else "open_at_free_end"
        open_specs.append({**base, "family": "P3", "config": open_config, "series": series,
                           "anchor_closure": TRANSFERS if open_config == "open_at_free_end" else COMPENSATED})
    return closed_specs + open_specs


def build_p2_p3(model_id: str, spec: Spec) -> tuple[dict, list[dict], dict, dict]:
    """A cantilever along +X: anchor at A, one member, the free end B capped or open."""
    b = Builder(model_id, spec["material"], pressure=True)
    od, wall = spec["od"], float(f"{spec['od'] / spec['dt']:.4g}")
    span, p = spec["span"], pressure_pa(spec["pressure_bar"])
    a, tip = b.node("A", (0, 0, 0)), b.node("B", (span, 0, 0))
    pipe = b.pipe("P1", a, tip, od, wall)
    anchor = b.anchor("A", a)
    free_closure = COMPENSATED if spec["config"] == "open_at_free_end" else TRANSFERS
    b.region("R1", [pipe], p, (a, spec["anchor_closure"]), (tip, free_closure))
    b.add_weight()
    b.add_thermal(spec["delta_t"])
    _, e, nu, alpha, _ = b.material
    bore, wall_area = pressure_areas(od, wall)
    every = [closed("displacement", "x", EXPANSION, alpha * spec["delta_t"] * span, "m", "alpha dT L", node=tip),
             closed("reaction_force", "x", EXPANSION, 0.0, "N", "0 (free thermal growth)", support=anchor)]
    by_state: dict[str, list[dict]] = {SERIES_DEFAULT: [], SERIES_ELONGATION_ON: []}
    if spec["config"] == "closed":
        every.append(closed("reaction_force", "x", SUSTAINED, 0.0, "N",
                            "0 (closed region on a cantilever: self-equilibrated, free axial strain)",
                            basis=PRESSURE_BASIS_TEXT, support=anchor))
        by_state[SERIES_DEFAULT].append(closed("displacement", "x", SUSTAINED, 0.0, "m",
                                               "0 (closed region, elongation off)", basis=PRESSURE_BASIS_TEXT,
                                               node=tip))
        by_state[SERIES_ELONGATION_ON].append(closed(
            "displacement", "x", SUSTAINED, (1.0 - 2.0 * nu) * p * bore * span / (e * wall_area), "m",
            "(1 - 2 nu) p A_i L/(E A_s)", basis=PRESSURE_BASIS_TEXT, node=tip))
    elif spec["config"] == "open_at_anchor":
        # The free end's cap transfers p A_i to the wall along +X; the anchor holds it.
        every.append(closed("reaction_force", "x", SUSTAINED, -p * bore, "N",
                            "-p A_i (the capped free end's thrust along +X, held by the anchor)",
                            basis=STATICS_BASIS_TEXT, support=anchor))
    else:
        # The anchor end's cap transfers p A_i to the wall along -X at the anchor; the anchor holds it.
        every.append(closed("reaction_force", "x", SUSTAINED, p * bore, "N",
                            "p A_i (the capped anchor end's thrust along -X, held by the anchor)",
                            basis=STATICS_BASIS_TEXT, support=anchor))
    return b.model(), every, _pressure_inexact(bends=False), by_state


def p4_bends(rng: random.Random) -> list[Spec]:
    specs = []
    grid = ((1.5, 90.0), (3.0, 45.0))
    for (rd, angle), point in zip(grid, latin_hypercube(rng, len(grid), 8)):
        od, dt, _ = section(point[0], point[1])
        common = {"rd": rd, "angle": angle, "od": od, "dt": dt, "clear_1": scale(point[2], 1.0, 5.0, digits=3),
                  "clear_2": scale(point[3], 1.0, 5.0, digits=3), "material": min(2, int(point[4] * 3)),
                  "pressure_bar": scale(point[5], *PRESSURE_RANGE_BAR, log=True, digits=3),
                  "delta_t": scale(point[6], *DELTA_T_RANGE)}
        k_high = float(f"{3.0 + 12.0 * point[7]:.3g}")
        for k in (1.0, k_high):
            specs.append({**common, "k": k, "sampling": "corner", "series": [f"pressure-k-rd{rd:g}-a{angle:g}"]})
    return specs


def build_p4(model_id: str, spec: Spec) -> tuple[dict, list[dict], dict, dict]:
    """An anchored L in the X-Z plane: straight, realized bend, straight, one closed region over all three."""
    b = Builder(model_id, spec["material"], pressure=True)
    od, wall = spec["od"], float(f"{spec['od'] / spec['dt']:.4g}")
    radius = spec["rd"] * od / 1000.0
    theta = math.radians(spec["angle"])
    tangent = radius * cr_tan(theta / 2.0)
    a = b.node("A", (0, 0, 0))
    ti = (spec["clear_1"] + tangent, 0.0, 0.0)
    near, far, bend, end = corner_bend(b, "BEND1", a, ti, (cr_cos(theta), 0.0, cr_sin(theta)), radius,
                                       spec["clear_2"], spec["k"], od, wall)
    tip = b.node("B", end)
    first = b.pipe("P1", a, near, od, wall)
    last = b.pipe("P2", far, tip, od, wall)
    b.anchor("A", a)
    b.anchor("B", tip)
    b.region("R1", [first, bend, last], pressure_pa(spec["pressure_bar"]), (a, TRANSFERS), (tip, TRANSFERS))
    b.add_weight()
    b.add_thermal(spec["delta_t"])
    return b.model(), [], _pressure_inexact(bends=True), {SERIES_DEFAULT: [], SERIES_ELONGATION_ON: []}


def p5_z(rng: random.Random) -> list[Spec]:
    specs = []
    for point in latin_hypercube(rng, 1, 10):
        od, dt, _ = section(point[0], point[1])
        common = {"config": "z", "sampling": "lhs", "od": od, "dt": dt, "rd": scale(point[2], 1.0, 5.0, digits=3),
                  "clear_1": scale(point[3], 1.0, 4.0, digits=3), "height": scale(point[4], 1.0, 4.0, digits=3),
                  "width": scale(point[5], 1.0, 4.0, digits=3), "material": min(2, int(point[6] * 3)),
                  "pressure_bar": scale(point[7], *PRESSURE_RANGE_BAR, log=True, digits=3),
                  "delta_t": scale(point[8], *DELTA_T_RANGE)}
        k_high = float(f"{3.0 + 12.0 * point[9]:.3g}")
        for k in (1.0, k_high):
            specs.append({**common, "k": k, "series": ["pressure-k-z"]})
    return specs


def build_p5(model_id: str, spec: Spec) -> tuple[dict, list[dict], dict, dict]:
    """An anchored Z in the X-Z plane, one closed region over every member."""
    b = Builder(model_id, spec["material"], pressure=True)
    od, wall = spec["od"], float(f"{spec['od'] / spec['dt']:.4g}")
    radius = spec["rd"] * od / 1000.0
    x, z = (1.0, 0.0, 0.0), (0.0, 0.0, 1.0)
    first, last = _polyline(b, (0.0, 0.0, 0.0), [x, z, x], [spec["clear_1"], spec["height"], spec["width"]], radius,
                            spec["k"], od, wall, None, 0.0)
    b.anchor("A", first)
    b.anchor("B", last)
    # The members in chain order: _polyline adds each straight after the bend that ends it.
    by_start = {pipe["from"]: pipe["id"] for pipe in b.pipes}
    chain, node = [], first
    while node != last:
        pipe_id = by_start[node]
        chain.append(pipe_id)
        node = next(pipe["to"] for pipe in b.pipes if pipe["id"] == pipe_id)
    b.region("R1", chain, pressure_pa(spec["pressure_bar"]), (first, TRANSFERS), (last, TRANSFERS))
    b.add_weight()
    b.add_thermal(spec["delta_t"])
    return b.model(), [], _pressure_inexact(bends=True), {SERIES_DEFAULT: [], SERIES_ELONGATION_ON: []}


def p6_tee(rng: random.Random) -> list[Spec]:
    point = latin_hypercube(rng, 1, 8)[0]
    od, dt, _ = section(point[0], point[1])
    return [{"config": "anchored_legs", "sampling": "lhs", "ratio": scale(point[2], 0.3, 1.0, digits=3), "od": od,
             "dt": dt, "run": scale(point[3], 2.0, 10.0, digits=3), "branch": scale(point[4], 1.0, 4.0, digits=3),
             "material": min(2, int(point[5] * 3)),
             "pressure_bar": scale(point[6], *PRESSURE_RANGE_BAR, log=True, digits=3),
             "delta_t": scale(point[7], *DELTA_T_RANGE)}]


def build_p6(model_id: str, spec: Spec) -> tuple[dict, list[dict], dict, dict]:
    """A tee with three anchored legs, one member and one closed region per leg, meeting at a pressure junction."""
    b = Builder(model_id, spec["material"], pressure=True)
    od, wall = spec["od"], float(f"{spec['od'] / spec['dt']:.4g}")
    branch_od = float(f"{od * spec['ratio']:.4g}")
    branch_wall = float(f"{branch_od / spec['dt']:.4g}")
    run, branch = spec["run"], spec["branch"]
    a, tee, c = b.node("A", (0, 0, 0)), b.node("T", (run / 2, 0, 0)), b.node("C", (run, 0, 0))
    end = b.node("E", (run / 2, 0.0, branch))
    west, east = b.run("R", [a, tee, c], od, wall)
    leg = b.pipe("BR1", tee, end, branch_od, branch_wall)
    for name, node in (("A", a), ("C", c), ("E", end)):
        b.anchor(name, node)
    p = pressure_pa(spec["pressure_bar"])
    b.region("WEST", [west], p, (a, TRANSFERS), (tee, TRANSFERS))
    b.region("EAST", [east], p, (tee, TRANSFERS), (c, TRANSFERS))
    b.region("BRANCH", [leg], p, (tee, TRANSFERS), (end, TRANSFERS))
    b.add_weight()
    b.add_thermal(spec["delta_t"])
    inexact = _pressure_inexact(bends=False)
    inexact[EXPANSION] = [SHEAR]
    return b.model(), [], inexact, {SERIES_DEFAULT: [], SERIES_ELONGATION_ON: []}


TRANCHE_2 = ("P1", "P2-P3", "P4", "P5", "P6")


def elongation_on_model(model: dict[str, Any], load_case: str) -> dict[str, Any]:
    """`model` with model_settings.pressure_elongation naming `load_case`; the argument is not changed."""
    out = copy.deepcopy(model)
    settings = out.setdefault("model_settings", {})
    settings["pressure_elongation"] = {"operating_load_cases": [load_case]}
    return out


def series_document(document: dict[str, Any], series: str) -> dict[str, Any]:
    """The benchmark document one run of a tranche 2 model solves and screens: for `pressure-default` the tracked
    model as it is, for `pressure-elongation-on` the tracked model with the setting naming its elongation_load_case;
    either way its closed forms are those valid in every state plus those of that series state. Deterministic, and
    nothing is written to the tracked set."""
    if series not in PRESSURE_RUN_SERIES or series not in document.get("series", []):
        raise ValueError(f"{document.get('model_id')} has no {series!r} run")
    out = copy.deepcopy(document)
    out.pop("closed_form_by_series", None)
    out["series_run"] = series
    out["closed_form"] = [*document["closed_form"], *document["closed_form_by_series"][series]]
    if series == SERIES_ELONGATION_ON:
        out["model"] = elongation_on_model(document["model"], document["elongation_load_case"])
    return out


# -- tranche 1 on the exact route: the exact-route twin -------------------------------------------------------------
# SWBPIPE's side of a tranche 1 model is solved on the exact route (the SWBPIPE Agent 0, 12 November 2026): the twin
# below restates the tracked, pressure-free model in the form the exact route requires, changing no mechanics input.
# The tracked files are not changed; the twin is derived on demand, and the manifest records its canonical model hash
# per shear model. The pressure-free solve of the tracked file is kept as a second, labelled series.
TIMOSHENKO = "timoshenko_annulus_energy_v1"
EULER_BERNOULLI = "euler_bernoulli"
EXACT_SHEAR_SERIES = "exact-shear-model"
# The members of the one-parameter series, in order: the first is the side used for the screen.
EXACT_SHEAR_MEMBERS = (TIMOSHENKO, EULER_BERNOULLI)
SIDE_EXACT = "exact-route-timoshenko"
SIDE_PRESSURE_FREE = "pressure-free"
PRESSURE_FREE_SERIES = "pressure-free-route"
TWIN_PROVENANCE = ("exact-route restatement of an invented CAEPIPE benchmark input (DEL-17-05, tranche 1); seeded "
                   "generator; not library, component, catalog, standard or code-rule data")
LOAD_STATE_CONTRACT = "openpipestress.load_reference_state/1.0.0"
TWIN_REFERENCE = "reference:installed"
TIMOSHENKO_BASIS_TEXT = ("Timoshenko closed form with the energy-matched annulus shear factor kappa_E and G = E/(2(1 + "
                         "nu)) as DEL-05-03 states them; invented inputs")
# The binding records of the writer's identity file: a stored result binds to SWBPIPE's entities through these.
BINDING_MAPS = ("node_map", "element_map", "support_map", "load_case_map")
# The choices of the exact-route side, recorded before any exact-route result existed (see EXACT_ROUTE_CHOICES).
CHOICES_DATE = "2026-11-12"
SHEAR_SOURCE = {
    "title": "Fiber Reinforced Plastic Piping (FRP), section \"Stiffness matrix\" (the target program's public "
             "technical reference)",
    "url": "https://docs.sstcae.com/tech_manual/fiber_reinforced_plastic_piping__frp_.htm",
    "fetched": "2026-11-12", "http_status": 200, "bytes": 30147, "sha256_prefix": "c4ab34330fb7f8b6",
    "states": ("the stiffness matrix of a pipe is calculated with an axial term L/EA, a shear term (shape factor x "
               "L/GA), a bending term L/EI and a torsion term, and for an isotropic material G = E/2(1 + Poisson's "
               "ratio): the public description of the pipe element includes a transverse shear flexibility term"),
}
EXACT_ROUTE_CHOICES = {
    "date": CHOICES_DATE,
    "decided_by": "the SWBPIPE Agent 0 decided the route (12 November 2026); the inputs below were recorded by the "
                  "CAEPIPE interoperation Type 1",
    "recorded_before": ("These choices were recorded before any exact-route result for these models existed, and so "
                        "before any exact-route result was compared with, or looked at beside, a value of the target "
                        "program. The pressure-free route's results for all 60 models had already been seen, screened "
                        "against the target program's results, before they were recorded; so the shear model rests on "
                        "the public source below, and the other value of that one input is run as a declared series "
                        "member."),
    "shear_model": {
        "side_used_for_the_screen": TIMOSHENKO,
        "source": SHEAR_SOURCE,
        "not_settled_by_the_source": [
            "the value of the shape factor",
            "whether the shear term applies to steel pipe as well as to fibre reinforced plastic pipe (DEL-17-01 "
            "question CQ-17-01-010, open)",
        ],
        "consequence": ("SWBPIPE's coefficient is its own energy-matched annulus factor kappa_E (DEL-05-03); "
                        "shear_deformation stays a named inexact input on every case that names it, in the 5e-3 "
                        "class; nothing is fitted and the shear coefficient is not a free parameter"),
        "series": {"name": EXACT_SHEAR_SERIES, "members": list(EXACT_SHEAR_MEMBERS),
                   "note": (f"{EULER_BERNOULLI} is the declared second member, declared before any run and run once; "
                            "it shows the size of the input and is not the side used for the screen")},
    },
    "other_inputs": {
        "mechanics": ("geometry, sections, E, G (as the Poisson's ratio it was formed from), expansion coefficient, "
                      "loads, restraints, spring rates, imposed movements, bend radius, angle and flexibility factor "
                      "are the tracked values unchanged"),
        "corrosion_allowance": "0 m on every pipe section (the exact contract requires it stated; the models are bare "
                               "pipe)",
        "pressure_regions": "an explicit empty list on every load case (no tranche 1 model has pressure)",
        "material": ("constitutive_basis homogeneous_isotropic_E_nu_v1 with Poisson's ratio the value the tracked G "
                     "was formed from (G = E/(2(1 + nu)) in this generator); the tracked shear_modulus is not "
                     "restated, since the exact route takes G only from E and nu"),
        "anchor_movements": ("the same values, stated as boundary motion (absolute reference displacement) of the "
                             "moved support in the case's analysis_state, the only form the exact route takes them "
                             "in; the document then carries the load/reference-state group: one reference "
                             "configuration of the authored geometry with a direct-strain basis and no fit, every "
                             "pipe on its base properties with an unchanged-reference thermal state, every support an "
                             "active model device, every stored load a source of factor 1"),
        "thermal_change": ("the same temperature interval per pipe, as the tracked primitive thermal load, which the "
                           "exact route takes as it is"),
        "bend_modifiers": ("none added: the exact route requires no stress intensification factor on a bend (it "
                           "admits the bend with the same warning as the pressure-free route), so the rule 'stated "
                           "as 1 where the route requires it' adds nothing"),
        "anything_else": ("set to the value that restates the tracked model; a requirement that cannot be met without "
                          "a value that changes mechanics leaves the model refused, with the refusal's code, and that "
                          "refusal is a finding"),
    },
    "model_file_condition": ("the stored results of the target program are bound by hash to the model file written "
                             "from the tracked document; a twin is a second SWBPIPE side of a stored result only if "
                             "the writer, given the twin with the same piping code and reference temperature, writes "
                             "the same model file bytes, with every differing line listed otherwise; a twin that "
                             "fails this is not compared"),
}
SIDES = {
    SIDE_EXACT: {
        "route": "exact", "pressure_contract": {**PRESSURE_CONTRACT, "shear_model": TIMOSHENKO},
        "counts_for": "exit criterion 11 of docs/RELEASE_TARGET_v1.0.md",
        "document": f"exact_route_document(document, {TIMOSHENKO!r})", "series": EXACT_SHEAR_SERIES,
        "applies_to": "tranche 1; tranche 2 is authored on the exact route with this shear model",
    },
    SIDE_PRESSURE_FREE: {
        "route": "pressure_free", "label": "second series: the tracked document as it is, solved on the "
                                           "pressure-free route (an Euler-Bernoulli frame)",
        "counts_for": None, "document": "the tracked model document as it is", "series": PRESSURE_FREE_SERIES,
        "applies_to": "tranche 1",
    },
}


def _poisson_ratio(material: dict[str, Any]) -> float:
    """The Poisson's ratio this generator formed the tracked material's G from; refuses a G not formed so."""
    for name, e, nu, _, _ in MATERIALS:
        if name == material["id"]:
            if (material["elastic_modulus"]["value"], material["shear_modulus"]["value"]) != (e, e / (2.0 * (1.0 + nu))):
                raise ValueError(f"{name}: the tracked E and G are not the generator's")
            return nu
    raise ValueError(f"{material['id']} is not one of the generator's materials")


def _load_state_restatement(model: dict[str, Any]) -> None:
    """Move every support-targeted imposed displacement into its case's analysis_state as boundary motion, adding the
    load/reference-state group that form needs (in place)."""
    pipes = [pipe["id"] for pipe in model["pipe_segments"]]
    model["reference_configurations"] = [{
        "id": TWIN_REFERENCE, "geometry_ref": {"kind": "authored_model_geometry"}, "provenance": TWIN_PROVENANCE,
        "member_references": [{"pipe_ref": pipe, "basis": {"kind": "direct_strain_reference"}, "fit": {"kind": "none"},
                               "provenance": TWIN_PROVENANCE} for pipe in pipes]}]
    material = {pipe["id"]: pipe["material"] for pipe in model["pipe_segments"]}
    for case in model["load_cases"]:
        motion: dict[str, list[dict[str, Any]]] = {}
        kept = []
        for load in case["primitive_loads"]:
            if load["category"] == "imposed_displacement":
                motion.setdefault(load["target"]["support"], []).append({
                    "dof": load["target"]["dof"], "value": dict(load["magnitude"]),
                    "meaning": "absolute_reference_displacement"})
            else:
                kept.append(load)
        case["primitive_loads"] = kept
        states = []
        for support in model["supports"]:
            state: dict[str, Any] = {"support_ref": support["id"], "participation": {"kind": "active_model_device"}}
            if support["id"] in motion:
                state["boundary_motion"] = motion.pop(support["id"])
            states.append(state)
        if motion:
            raise ValueError(f"{case['id']}: imposed displacement on an unknown support {sorted(motion)}")
        case["analysis_state"] = {
            "contract": LOAD_STATE_CONTRACT, "reference_configuration_ref": TWIN_REFERENCE,
            "element_states": [{"pipe_ref": pipe,
                                "material_selection": {"kind": "explicit_base_properties", "material_ref": material[pipe],
                                                       "applicability_reference": TWIN_PROVENANCE},
                                "thermal_state": {"kind": "unchanged_reference", "provenance": TWIN_PROVENANCE}}
                               for pipe in pipes],
            "support_states": states,
            "load_sources": [{"source_ref": load["id"], "factor": 1.0} for load in kept],
            "history": {"kind": "independent_equilibrium"}, "provenance": TWIN_PROVENANCE}


def exact_route_model(model: dict[str, Any], shear_model: str) -> dict[str, Any]:
    """The exact-route twin of a tracked tranche 1 model for `shear_model`: the exact pressure contract, a zero
    corrosion allowance on every section, an explicit empty pressure_regions on every case, an E and nu material, and
    any anchor movement as boundary motion. No mechanics input changes; the argument is not changed."""
    if shear_model not in EXACT_SHEAR_MEMBERS:
        raise ValueError(f"unknown shear model {shear_model!r}")
    if "pressure_contract" in model:
        raise ValueError("the twin is formed from a pressure-free model")
    out = copy.deepcopy(model)
    out["pressure_contract"] = {**PRESSURE_CONTRACT, "shear_model": shear_model}
    for pipe in out["pipe_segments"]:
        pipe["section"]["corrosion_allowance"] = q(0.0, "m")
    out["materials"] = [{
        "id": material["id"], "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
        "elastic_modulus": dict(material["elastic_modulus"]), "poisson_ratio": q(_poisson_ratio(material), "1"),
        **{key: copy.deepcopy(value) for key, value in material.items()
           if key not in ("id", "elastic_modulus", "shear_modulus")}} for material in out["materials"]]
    for case in out["load_cases"]:
        case["pressure_regions"] = []
    if any(load["category"] == "imposed_displacement" for case in out["load_cases"] for load in case["primitive_loads"]):
        _load_state_restatement(out)
    return out


def annulus_shear_factor(r_o: float, r_i: float, nu: float) -> float:
    """kappa_E, the energy-matched shear factor of the annulus, in DEL-05-03's operation order (no product code)."""
    m = r_i / r_o
    m2 = m * m
    a, b, n1 = 1.0 + m2, 1.0 - m2, 1.0 + nu
    c = (8.0 * nu + 14.0) * nu + 7.0
    d = c * (b * b) + 48.0 * (m2 * (n1 * n1))
    return (6.0 * (n1 * n1)) * (a * a) / d


def _beam(document: dict[str, Any]) -> dict[str, float]:
    """E, G, I, the shear area G A_s and the bending/shear ratio of a model's (uniform) straight section, in SI."""
    model = document["model"]
    pipe = model["pipe_segments"][0]
    od, wall = pipe["section"]["outside_diameter"]["value"], pipe["section"]["wall_thickness"]["value"]
    if any((p["section"]["outside_diameter"]["value"], p["section"]["wall_thickness"]["value"]) != (od, wall)
           for p in model["pipe_segments"]):
        raise ValueError(f"{document['model_id']}: a closed form needs one section")
    material = model["materials"][0]
    e, nu = material["elastic_modulus"]["value"], _poisson_ratio(material)
    g = e / (2.0 * (1.0 + nu))
    r_o = od / 2.0
    r_i = r_o - wall
    area = math.pi * (r_o * r_o - r_i * r_i)
    inertia = math.pi / 4.0 * (cr_pow(r_o, 4) - cr_pow(r_i, 4))
    shear_area = annulus_shear_factor(r_o, r_i, nu) * area
    return {"E": e, "G": g, "I": inertia, "GAs": g * shear_area, "EI": e * inertia}


def timoshenko_closed_forms(document: dict[str, Any]) -> list[dict]:
    """The tracked closed forms of a tranche 1 model restated for Timoshenko straight members: each value that the
    shear term changes is derived here (no product code); every other closed form is carried unchanged.

    Cantilever under a tip load P and a uniform load w (G1): the section rotation and the reactions are statically
    determinate and unchanged; the tip deflection gains the shear term (P L + w L^2/2)/(G A_s).
    Beam fixed at both ends, end B moved by delta or turned by phi (G8), with phi_s = 12 E I/(G A_s L^2): the end
    shear is 12 E I delta/(L^3 (1 + phi_s)) and the midspan deflection stays delta/2; under phi the end moments are
    (4 + phi_s) E I phi/(L (1 + phi_s)) at B and (2 - phi_s) E I phi/(L (1 + phi_s)) at A, the end shear
    6 E I phi/(L^2 (1 + phi_s)), and the midspan section rotation -phi (1 - 2 phi_s)/(4 (1 + phi_s)). Axial closed
    forms carry no shear term."""
    out = []
    family, parameters = document["family"], document["parameters"]
    for entry in document["closed_form"]:
        entry = dict(entry)
        formula = entry["formula"]
        if family == "G1" and parameters["config"] == "cantilever" and formula == "P L^3/(3EI) + w L^4/(8EI)":
            beam, span, tip = _beam(document), parameters["span"], parameters["tip_force"]
            builder = Builder(document["model_id"], next(index for index, item in enumerate(MATERIALS)
                                                        if item[0] == document["model"]["materials"][0]["id"]))
            builder.sections["pipe"] = (parameters["od"], float(f"{parameters['od'] / parameters['dt']:.4g}"))
            w = builder.weight_per_length("pipe")
            entry.update(value=entry["value"] + (tip * span + w * cr_pow(span, 2) / 2) / beam["GAs"],
                         formula="P L^3/(3EI) + w L^4/(8EI) + (P L + w L^2/2)/(G A_s)", basis=TIMOSHENKO_BASIS_TEXT)
        elif family == "G8" and parameters["config"] != "l_general" and (
                entry["quantity"].startswith("reaction")
                or (parameters["config"] == "rotate_rz" and entry["quantity"] == "rotation")):
            beam, span = _beam(document), parameters["span"]
            phi_s = 12.0 * beam["EI"] / (beam["GAs"] * span * span)
            factor = {
                "12EI delta/L^3": 1.0 / (1.0 + phi_s), "-12EI delta/L^3": 1.0 / (1.0 + phi_s),
                "-6EI phi/L^2": 1.0 / (1.0 + phi_s),
                "4EI phi/L": (4.0 + phi_s) / (4.0 * (1.0 + phi_s)), "2EI phi/L": (2.0 - phi_s) / (2.0 * (1.0 + phi_s)),
                "-phi/4 at midspan (v = phi (x^3/L^2 - x^2/L))": (1.0 - 2.0 * phi_s) / (1.0 + phi_s),
            }[formula]
            restated = {
                "12EI delta/L^3": "12EI delta/(L^3 (1 + phi_s))", "-12EI delta/L^3": "-12EI delta/(L^3 (1 + phi_s))",
                "-6EI phi/L^2": "-6EI phi/(L^2 (1 + phi_s))", "4EI phi/L": "(4 + phi_s) EI phi/(L (1 + phi_s))",
                "2EI phi/L": "(2 - phi_s) EI phi/(L (1 + phi_s))",
                "-phi/4 at midspan (v = phi (x^3/L^2 - x^2/L))": "-phi (1 - 2 phi_s)/(4 (1 + phi_s)) at midspan",
            }[formula]
            entry.update(value=entry["value"] * factor, formula=f"{restated}, phi_s = 12 E I/(G A_s L^2)",
                         basis=TIMOSHENKO_BASIS_TEXT)
        out.append(entry)
    return out


def exact_route_document(document: dict[str, Any], shear_model: str) -> dict[str, Any]:
    """The benchmark document of a tranche 1 model's exact-route side for one member of the exact-shear-model series:
    the tracked document with its model replaced by the exact-route twin and the closed forms valid for that shear
    model (Euler-Bernoulli: the tracked ones; Timoshenko: `timoshenko_closed_forms`). Deterministic; nothing is
    written to the tracked set."""
    if document.get("tranche") == "2":
        raise ValueError(f"{document.get('model_id')} is a tranche 2 model, authored on the exact route")
    out = copy.deepcopy(document)
    out["model"] = exact_route_model(document["model"], shear_model)
    out["series_run"] = EXACT_SHEAR_SERIES
    out["route"] = "exact"
    out["shear_model"] = shear_model
    out["closed_form"] = (timoshenko_closed_forms(document) if shear_model == TIMOSHENKO
                          else copy.deepcopy(document["closed_form"]))
    return out


def _write_model_file(document: dict[str, Any], model: dict[str, Any]):
    """The writer's output for `model` with the piping code and reference temperature run_pilot passes."""
    return render_caepipe_mbf_si(model, title=document["model_id"],
                                 reference_temperature_c=document.get("reference_temperature_c", 20.0),
                                 piping_code=document["piping_code"])


def model_file_differences(pressure_free: str, twin: str) -> list[dict[str, Any]]:
    """Every line where two model files differ, each with the record (the keyword line above it) it belongs to."""
    a, b = pressure_free.split("\r\n"), twin.split("\r\n")
    out, record = [], None
    for index in range(max(len(a), len(b))):
        left = a[index] if index < len(a) else None
        right = b[index] if index < len(b) else None
        line = left if left is not None else right
        if line and "," not in line and line.replace("_", "").isalnum() and line.upper() == line:
            record = line
        if left != right:
            out.append({"line": index + 1, "record": record, "pressure_free": left, "exact": right})
    return out


def model_file_condition(document: dict[str, Any], shear_model: str) -> dict[str, Any]:
    """Whether the writer writes the twin's model file byte for byte as it writes the tracked document's."""
    tracked = _write_model_file(document, document["model"])
    twin = _write_model_file(document, exact_route_model(document["model"], shear_model))
    if twin.status != "written":
        return {"writer_status": twin.status, "equal_to_pressure_free": False,
                "writer_refusals": sorted({item["code"] for item in twin.diagnostics if item["severity"] == "error"})}
    differences = model_file_differences(tracked.text, twin.text)
    tracked_record, twin_record = tracked.to_dict(), twin.to_dict()
    return {"writer_status": "written", "equal_to_pressure_free": not differences, "differences": differences,
            "binding_maps_equal": all(tracked_record[name] == twin_record[name] for name in BINDING_MAPS),
            "identity_file_records_differing": sorted(key for key in set(tracked_record) | set(twin_record)
                                                      if tracked_record.get(key) != twin_record.get(key))}


def _closed_form_sha256(closed_forms: list[dict]) -> str:
    return hashlib.sha256(_dump(closed_forms).encode("ascii")).hexdigest()


def exact_route_entry(document: dict[str, Any]) -> dict[str, Any]:
    """The manifest's record of a tranche 1 model's exact-route twin: per shear model, the twin's canonical model
    hash, its closed forms and the model file condition."""
    forms = ["pressure_contract", "corrosion_allowance_zero", "pressure_regions_empty", "material_E_nu"]
    if any(load["category"] == "imposed_displacement" for case in document["model"]["load_cases"]
           for load in case["primitive_loads"]):
        forms.append("boundary_motion_in_load_reference_state")
    members = []
    for shear_model in EXACT_SHEAR_MEMBERS:
        twin = exact_route_document(document, shear_model)
        condition = model_file_condition(document, shear_model)
        if not condition.get("differences", True):
            condition.pop("differences")
        members.append({"shear_model": shear_model, "side": SIDE_EXACT if shear_model == TIMOSHENKO else None,
                        "swbpipe_model_hash": canonical_model_hash(twin["model"]),
                        "closed_forms": len(twin["closed_form"]),
                        "closed_form_sha256": _closed_form_sha256(twin["closed_form"]),
                        "model_file": condition})
    return {"series": EXACT_SHEAR_SERIES, "forms": forms,
            "comparable_with_stored_results": all(member["model_file"]["equal_to_pressure_free"] for member in members),
            "members": members}


EXACT_SHEAR_SERIES_DEFINITION = {
    "kind": "shear_model", "parameter": "pressure_contract.shear_model", "hypothesis": SHEAR,
    "values": [{"value": TIMOSHENKO, "role": f"the side used for the screen ({SIDE_EXACT})"},
               {"value": EULER_BERNOULLI, "role": "declared second member; shows the size of the input"}],
    "swbpipe": "exact_route_document(document, value): the exact-route twin of the tracked tranche 1 model",
    "caepipe": "the model file written from the tracked document, unchanged, where the twin's model file equals it",
    "description": ("every tranche 1 model on the exact route under each shear model; a model whose twin fails the "
                    "model file condition is listed with comparable_with_stored_results false and is not compared"),
}
PRESSURE_FREE_SERIES_DEFINITION = {
    "kind": "route", "parameter": "route", "value": "pressure_free",
    "swbpipe": "the tracked tranche 1 model document as it is, solved on the pressure-free route",
    "caepipe": "the model file written from the tracked document",
    "description": ("the second, labelled series: SWBPIPE's pressure-free route (an Euler-Bernoulli frame); it does "
                    "not count for exit criterion 11"),
}


FAMILIES: tuple[tuple[str, str, Callable[[random.Random], list[Spec]], Callable[..., Any]], ...] = (
    ("G1", "straight spans: cantilever, guided, continuous, fixed-fixed", g1_straight, build_g1),
    ("G2", "single bends: anchored L, R/D 1-5, 45 and 90 degrees, user k", g2_bends, build_g2),
    ("G3", "Z and U loops", g3_loops, build_g3),
    ("G4", "multi-plane runs", g4_multiplane, build_g4),
    ("G5", "branch tees", g5_tees, build_g5),
    ("G6-G7", "risers and support mixes: anchors, guides, line stops, vertical supports, springs", g6_g7_supports,
     build_g6_g7),
    ("G8", "anchor displacement and rotation", g8_movements, build_g8),
    # Tranche 2, appended after tranche 1 so that tranche 1 consumes the same draws.
    ("P1", "pressurised closed straights between anchors, one member, both terminals transferring",
     p1_closed_straights, build_p1),
    ("P2-P3", "pressurised cantilevers, one member: closed (P2), or with one terminal separately supported or "
              "compensated so that an unbalanced thrust acts (P3)", p2_p3_cantilevers, build_p2_p3),
    ("P4", "pressurised single bends: anchored L, one closed region, user k 1 and a higher k", p4_bends, build_p4),
    ("P5", "pressurised anchored Z, one closed region, user k 1 and a higher k", p5_z, build_p5),
    ("P6", "pressurised tee: three anchored legs, one closed region per leg meeting at a pressure junction", p6_tee,
     build_p6),
)

SERIES = {
    "shear-dt": {"swept": "dt", "hypothesis": SHEAR,
                 "description": "cantilevers of one span and OD with D/t 10, 20, 40, 100"},
    "tee-ratio": {"swept": "ratio", "hypothesis": "branch_flexibility",
                  "description": "anchored in-plane tees with branch/run OD ratio 0.3, 0.5, 0.75, 1.0"},
    **{f"pressure-closure-{index:02d}": {
        "swept": "anchor_closure", "hypothesis": THRUST,
        "description": ("one pressurised cantilever with its anchor terminal transferring to the wall (closed) and "
                        "separately supported or compensated (open): the same target-program input, since the MBF "
                        "carries no terminal closure, with SWBPIPE's end thrust changed")} for index in (1, 2)},
}
TRANCHES = {
    "1": {"families": ["G1", "G2", "G3", "G4", "G5", "G6", "G7", "G8"], "description": "linear static models"},
    "2": {"families": ["P1", "P2", "P3", "P4", "P5", "P6"],
          "description": ("pressurised linear static models on the exact pressure contract, each run in the "
                          "pressure-default and pressure-elongation-on series; no model document carries "
                          "model_settings")},
}


def _dump(value: Any) -> str:
    return json.dumps(value, indent=2, sort_keys=True, ensure_ascii=True, allow_nan=False) + "\n"


def layout_rows(document: dict[str, Any]) -> int:
    """The writer's layout row count for a model, which must be written without refusal."""
    result = render_caepipe_mbf_si(document["model"], title=document["model_id"],
                                   reference_temperature_c=document["reference_temperature_c"],
                                   piping_code=document["piping_code"])
    if result.status != "written":
        raise ValueError(f"{document['model_id']}: the writer refused it: "
                         f"{sorted({item['code'] for item in result.diagnostics if item['severity'] == 'error'})}")
    return result.layout_row_count


def generate(seed: int = DEFAULT_SEED) -> dict[str, str]:
    """{relative path: file text} of the whole benchmark set for `seed`."""
    rng = random.Random(seed)
    files: dict[str, str] = {}
    entries = []
    series: dict[str, dict[str, Any]] = {}
    counters: dict[str, int] = {}
    for family, description, sample, build in FAMILIES:
        pressurised = family in TRANCHE_2
        for spec in sample(rng):
            sub_family = spec.get("family", family)
            counters[sub_family] = counters.get(sub_family, 0) + 1
            model_id = f"cpb-{sub_family.lower()}-{counters[sub_family]:02d}"
            built = build(model_id, spec)
            model, closed_forms, inexact = built[:3]
            operating = sorted(set(inexact.get(SUSTAINED, [])) | set(inexact.get(EXPANSION, [])))
            parameters = {key: value for key, value in spec.items() if key not in ("sampling", "series", "family")}
            member_of = [*(PRESSURE_RUN_SERIES if pressurised else ()), *spec.get("series", [])]
            document = {
                "document_kind": MODEL_KIND,
                "model_id": model_id,
                "family": sub_family,
                "family_description": description,
                "generator": {"version": GENERATOR_VERSION, "seed": seed},
                "sampling": spec["sampling"],
                "parameters": parameters,
                "series": member_of,
                "piping_code": PIPING_CODE,
                "reference_temperature_c": REFERENCE_TEMPERATURE_C,
                "inexact_inputs": {SUSTAINED: inexact.get(SUSTAINED, []), EXPANSION: inexact.get(EXPANSION, []),
                                   OPERATING: operating},
                "closed_form": closed_forms,
                "provenance": {"origin": "invented", "license": "project-original",
                               "redistribution_status": "public_permissive", "produced_by_caepipe": False,
                               "contributor_note": PRESSURE_PROVENANCE if pressurised else PROVENANCE},
                "model": model,
            }
            if pressurised:
                document["tranche"] = "2"
                # The load case that carries pressure: the one model_settings.pressure_elongation names in the
                # pressure-elongation-on run.
                document["elongation_load_case"] = SUSTAINED
                document["closed_form_by_series"] = built[3]
            text = _dump(document)
            relative = f"models/{model_id}.json"
            files[relative] = text
            for name in spec.get("series", []):
                series.setdefault(name, {**SERIES.get(name, {"swept": "k", "hypothesis": BEND_K,
                                                             "description": "one bend geometry with k = 1 and a "
                                                                            "higher k"}), "members": []})
                series[name]["members"].append({"model_id": model_id, "value": spec.get(series[name]["swept"])})
            entry = {"model_id": model_id, "family": sub_family, "tranche": "2" if pressurised else "1",
                     "sampling": spec["sampling"], "parameters": parameters, "series": member_of, "file": relative,
                     "model_hash": canonical_model_hash(model),
                     "file_sha256": hashlib.sha256(text.encode("ascii")).hexdigest()}
            if pressurised:
                entry["layout_row_count"] = layout_rows(document)
                entry["elongation_load_case"] = SUSTAINED
                on_model = elongation_on_model(model, SUSTAINED)
                entry["runs"] = [
                    {"series": SERIES_DEFAULT, "file": relative, "variant_of": None,
                     "swbpipe_model_hash": canonical_model_hash(model),
                     "closed_forms": len(closed_forms) + len(built[3][SERIES_DEFAULT])},
                    {"series": SERIES_ELONGATION_ON, "file": relative, "variant_of": SERIES_DEFAULT,
                     "caepipe_manual_change": "the Bourdon option switched on by hand before the analysis",
                     "swbpipe_model_hash": canonical_model_hash(on_model),
                     "swbpipe_model_settings": on_model["model_settings"],
                     "closed_forms": len(closed_forms) + len(built[3][SERIES_ELONGATION_ON])},
                ]
                for name in PRESSURE_RUN_SERIES:
                    series.setdefault(name, {**PRESSURE_RUN_SERIES[name], "members": []})
                    series[name]["members"].append({"model_id": model_id, "value": PRESSURE_RUN_SERIES[name]["value"],
                                                    "elongation_load_case": SUSTAINED})
            else:
                entry["exact_route"] = exact_route_entry(document)
                series.setdefault(PRESSURE_FREE_SERIES, {**PRESSURE_FREE_SERIES_DEFINITION, "members": []})
                series[PRESSURE_FREE_SERIES]["members"].append({"model_id": model_id, "value": "pressure_free",
                                                                "swbpipe_model_hash": entry["model_hash"]})
                series.setdefault(EXACT_SHEAR_SERIES, {**EXACT_SHEAR_SERIES_DEFINITION, "members": []})
                for member in entry["exact_route"]["members"]:
                    series[EXACT_SHEAR_SERIES]["members"].append({
                        "model_id": model_id, "value": member["shear_model"],
                        "swbpipe_model_hash": member["swbpipe_model_hash"]})
            entries.append(entry)
    files["manifest.json"] = _dump({
        "document_kind": MANIFEST_KIND,
        "generator": {"version": GENERATOR_VERSION, "module": "tools/caepipe_correlation/benchmark_models.py",
                      "seed": seed, "rng": "random.Random(seed).random() only"},
        "tranches": TRANCHES,
        "requirement": ("DEL-17-05 REQ-018 and AC-007 (execution/PKG-17/DEL-17-05/ScopeOfWork.md); the Benchmark "
                        "method section of docs/RELEASE_TARGET_v1.0.md"),
        "piping_code": PIPING_CODE,
        "materials": [{"id": name, "elastic_modulus_pa": e, "poisson_ratio": nu, "thermal_expansion_per_k": alpha,
                       "density_kg_per_m3": rho} for name, e, nu, alpha, rho in MATERIALS],
        "swbpipe_sides": SIDES,
        "exact_route_choices": EXACT_ROUTE_CHOICES,
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
