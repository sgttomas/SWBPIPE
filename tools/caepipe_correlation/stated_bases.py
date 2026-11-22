"""Stated bases of the benchmark comparison (DEL-17-05): what is being compared with what.

A benchmark comparison sets SWBPIPE's solve of a model document beside the print the target program made of the MBF
written from it. A stated basis is a named alternative binding or formulation evaluated on SWBPIPE's side for
specified quantities and cases: the condition under which it applies, the reason for it, how SWBPIPE realizes it, and
the independent ground on which that realization is validated in this repository (a closed form or the product
itself; never a value from the other program's output). Where the other program's convention is documented in a
cited public source, the reason says so; otherwise the basis is a comparison hypothesis and says that. Nothing of the
other program's output is used to choose, tune or validate a basis.

The rule that governs every basis: a stated basis never replaces the plain comparison. The comparison record keeps its
unadjusted results exactly as built (core/handoff/caepipe_external/benchmark.py) and shows each basis beside them,
named, with its condition, reason and ground. Every basis ends in a named outcome (OUTCOMES), never an exception: a
basis whose condition does not hold, that cannot be represented, or that the product refuses is listed with the reason
by name and compares nothing. A route writes the plain record before it attempts any basis.

The bases:
- effective_axial_force_binding (B): the binding of SWBPIPE's element end forces on pressurised members. SWBPIPE's
  global end force carries the wall axial force N_w along the member axis, which includes the closed-end pressure
  force; the like-for-like quantity beside a program that prints the pipe's element force without it is the effective
  axial force S = N_w - p A_i (rows pipe_wall_axial_force_v2 and pipe_effective_axial_force_v2 of the exact route).
  Realized on the same solve: each end force has its axial part replaced, F + s (S - N_w) t, t the member's axis at
  that end and s = -1 at end_i, +1 at end_j (the end action on the element). No second solve.
- timoshenko_shear (C): a second solve of the same document with model_settings.shear_model set to
  timoshenko_annulus_energy_v1.
- finite_anchor_stiffness (D): a second solve with each anchor replaced by six linear springs of the stiffness the
  target program's public anchor page states for its rigid anchor.
- thin_wall_elongation (E1) and straight_only_elongation (E2) are comparison hypotheses on SWBPIPE's side; no formula
  is attributed to the other program.
- thin_wall_elongation (E1), straight_only_elongation (E2), terminal_transfers_to_wall (E3): the pressure bases of the
  two pressure series (benchmark_models.PRESSURE_RUN_SERIES).
- together: one solve with every solve basis that applies to the model.
"""

from __future__ import annotations

import copy
import math
from typing import Any, Mapping

from tools.caepipe_correlation.benchmark_tranche3 import K_ROTATION, K_TRANSLATION, RIGID_SUPPORT, WINDOW_PRINT_WAIT

EFFECTIVE_AXIAL = "effective_axial_force_binding"
SHEAR = "timoshenko_shear"
ANCHOR = "finite_anchor_stiffness"
THIN = "thin_wall_elongation"
STRAIGHT = "straight_only_elongation"
TERMINAL = "terminal_transfers_to_wall"
TOGETHER = "together"
SOLVE_BASES = (SHEAR, ANCHOR, THIN, STRAIGHT, TERMINAL)
PRESSURE_BASES = (THIN, STRAIGHT, TERMINAL)

TIMOSHENKO = "timoshenko_annulus_energy_v1"
EULER_BERNOULLI = "euler_bernoulli"
TRANSFERS = "transfers_to_wall"
ELONGATION_ON = "pressure-elongation-on"
BASIS_LOAD_PREFIX = "load:stated-basis:"
SPRING_DOFS = (("UX", "N/m"), ("UY", "N/m"), ("UZ", "N/m"), ("RX", "N*m/rad"), ("RY", "N*m/rad"), ("RZ", "N*m/rad"))

# The outcome of each basis, by name.
APPLIED = "applied"
NOT_APPLICABLE = "not_applicable"            # its condition does not hold
NOT_REPRESENTABLE = "not_representable"      # its condition holds, but its realization cannot be formed
REFUSED_BY_PRODUCT = "refused_by_product"    # the product refuses the basis document, or does not solve it
NOT_COMPARED = "not_compared"                # the run is declared not screened: no basis is compared
OUTCOMES = (APPLIED, NOT_APPLICABLE, NOT_REPRESENTABLE, REFUSED_BY_PRODUCT, NOT_COMPARED)

RULE = ("a stated basis never replaces the plain comparison: the record's results are the unadjusted comparison, and "
        "each basis is shown beside them by name, with its condition, its reason and the independent ground that "
        "validates its realization; a basis is applied only where its condition holds")

STATEMENTS: dict[str, dict[str, str]] = {
    EFFECTIVE_AXIAL: {
        "title": "effective axial force beside the wall axial force",
        "condition": ("a member of a pressure region in a compared case, whose axis at each end is known: a straight "
                      "member's from its nodes, a realized bend's from the straight member that meets it there, "
                      "tangent to the arc; and SWBPIPE's result carries pipe_wall_axial_force_v2 and "
                      "pipe_effective_axial_force_v2 at that end in every case where the member is pressurised"),
        "reason": ("SWBPIPE's element end force on a pressurised member is the wall's, which includes the closed-end "
                   "pressure force p A_i; the effective axial force S = N_w - p A_i is the force of the pipe as a "
                   "beam. Which of the two the other program prints as its element force is not stated in its public "
                   "documentation read here, so both bindings are shown by name"),
        "realization": ("the same solve; each bound end force F becomes F + s (S - N_w) t, with t the unit axis at that "
                        "end in the direction from end_i to end_j and s = -1 at end_i, +1 at end_j; moments and the "
                        "other rows are unchanged"),
        "ground": ("closed form: an anchored closed straight under pressure with elongation off has N_w = p A_i and "
                   "S = 0, so its effective end force along the axis is zero at both ends "
                   "(tests/test_caepipe_stated_bases.py)"),
    },
    SHEAR: {
        "title": "Timoshenko straight members",
        "condition": ("a pressure-free document (no pressure_contract) that does not select Timoshenko straight members "
                      "(it selects no shear model, or euler_bernoulli), of a run that is compared (declared screened, "
                      "or whose declared screening waits for AC-002 and VER-002 for window runs, which the window print route compares meanwhile; a capture-only run is compared "
                      "by no route), with no wind exposed spans; applied only where the product solves the basis "
                      "document"),
        "reason": ("the benchmark documents select no Timoshenko shear model, so SWBPIPE's pressure-free solve is "
                   "Euler-Bernoulli. Whether the other program's straight pipe carries shear flexibility, and with "
                   "which shape factor, is an open question (CQ-17-01-010 in "
                   "execution/PKG-17/DEL-17-01/CAEPIPE_Question_Dossier.md); this basis is a comparison hypothesis "
                   "stated on SWBPIPE's side, not a documented convention of the other program. It uses SWBPIPE's "
                   "own energy-matched annular factor kappa_E"),
        "realization": "a second solve of the same document with model_settings.shear_model = " + TIMOSHENKO,
        "ground": ("closed form: a cantilever's tip deflection under a tip force P is P L^3/(3 E I) + P L/(kappa_E G A), "
                   "kappa_E the annulus energy factor 6 (1 + nu)^2 (1 + m^2)^2 / ((7 + 14 nu + 8 nu^2)(1 + m^2)^2 + "
                   "4 m^2 (5 + 10 nu + 4 nu^2)) with m = r_i/r_o and G = E/(2 (1 + nu)) "
                   "(tests/test_caepipe_stated_bases.py)"),
    },
    ANCHOR: {
        "title": "anchors of the stated finite stiffness",
        "condition": ("a model with at least one anchor (all six components restrained) and no imposed movement of an "
                      "anchor"),
        "reason": ("SWBPIPE's anchor holds each component exactly. The target program's public anchor page states "
                   "that its rigid stiffness is set internally to a finite value: " + RIGID_SUPPORT["stated"] + ", "
                   + RIGID_SUPPORT["source"] + " (CAEPIPE-ANCHOR in "
                   "execution/PKG-17/DEL-17-01/Source_Basis_Register.md). Short, stiff models can then differ in "
                   "reactions, and show small rotations at anchors where SWBPIPE has exactly zero"),
        "realization": (f"a second solve with each anchor replaced by six linear springs at its node, "
                        f"{K_TRANSLATION!r} N/m on UX, UY, UZ and {K_ROTATION!r} N*m/rad on RX, RY, RZ ("
                        + RIGID_SUPPORT["conversion"] + "); each spring's reaction is summed into the anchor's node"),
        "ground": ("closed form: a cantilever whose root is held by springs k_t and k_r deflects at its tip under a tip "
                   "force P by P L^3/(3 E I) + P L^2/k_r + P/k_t, and its root turns by P L/k_r "
                   "(tests/test_caepipe_stated_bases.py)"),
    },
    THIN: {
        "title": "pressure elongation by the thin-wall formula at the mean diameter",
        "condition": ("the pressure-elongation-on run of a pressure series with a pressure region in an elongation "
                      "case; represented only where every member of such a region has a nominal section (no corrosion "
                      "allowance or mill tolerance other than zero), a material without temperature points whose "
                      "expansion coefficient is a nonzero finite value in 1/degC or 1/K, its other inputs in SI, and "
                      "no thermal load already on it in that case"),
        "reason": ("a comparison hypothesis on SWBPIPE's side. SWBPIPE's pressure elongation is the closed-end free "
                   "strain (1 - 2 nu) p A_i/(E A_s), the thick-wall value. The basis is the classical thin-wall "
                   "closed-end strain at the mean diameter, (1 - 2 nu) p D_m/(4 t E), the thick-wall value times "
                   "(D_m/D_i)^2. No formula is attributed to the other program"),
        "realization": ("the run's elongation-off document (model_settings.pressure_elongation removed) with, in each "
                        "elongation case, a free strain on each member of its pressure regions placed as a uniform "
                        "temperature interval dT = eps/alpha, eps = (1 - 2 nu) p D_m/(4 t E)"),
        "ground": ("closed forms on the P1 and P2 families: each anchor's axial reaction of a closed straight between "
                   "anchors changes by (1 - 2 nu) p pi D_m^2/4 from the elongation-off solve, and a free end moves by "
                   "eps L (tests/test_caepipe_stated_bases.py)"),
    },
    STRAIGHT: {
        "title": "no pressure elongation along a bend's arc",
        "condition": ("the pressure-elongation-on run of a pressure series whose regions in an elongation case hold a "
                      "realized bend; represented under THIN's section, material and load conditions"),
        "reason": ("a comparison hypothesis on SWBPIPE's side. SWBPIPE elongates every member of a pressure region "
                   "under pressure, realized arcs included. The basis applies SWBPIPE's own thick-wall elongation to "
                   "straight members only. No convention is attributed to the other program"),
        "realization": ("the run's elongation-off document with SWBPIPE's own thick-wall free strain "
                        "(1 - 2 nu) p A_i/(E A_s) placed on each straight member of the pressure regions of each "
                        "elongation case as a uniform temperature interval, and none on a realized bend"),
        "ground": ("the product itself: on a closed straight the realization equals SWBPIPE's own elongation-on "
                   "solve, and on a bend model it equals the elongation-off solve plus a solve of the same free "
                   "strain alone on the straight members (tests/test_caepipe_stated_bases.py)"),
    },
    TERMINAL: {
        "title": "both terminals transferring to the wall",
        "condition": "a pressure region with a terminal that does not transfer to the wall (closure_transfer)",
        "reason": ("the model states that terminal as separately supported or compensated, so SWBPIPE applies the "
                   "unbalanced thrust p A_i there. The MBF this writer writes carries no terminal condition: it has no "
                   "record for one"),
        "realization": "a second solve with every terminal of every pressure region transferring to the wall",
        "ground": ("closed form: a closed cantilever under pressure with elongation off carries no axial reaction at "
                   "its anchor, where the plain solve carries the thrust p A_i (tests/test_caepipe_stated_bases.py)"),
    },
    TOGETHER: {
        "title": "every applicable basis together",
        "condition": "two or more solve bases apply to the model",
        "reason": "the bases are independent statements; together they show the comparison under all of them at once",
        "realization": ("one solve with each applicable solve basis's change; where THIN and STRAIGHT both apply, the "
                        "thin-wall strain is placed on straight members only"),
        "ground": "each basis's own ground; together they add no realization of their own",
    },
}


# -- conditions ------------------------------------------------------------------------------------------------------
def _model(entry: Mapping[str, Any]) -> Mapping[str, Any]:
    return entry.get("model") or {}


def declared_compared(declaration: Mapping[str, Any] | None) -> tuple[bool, str]:
    """Whether a run's manifest declaration lets a route compare it, and why not, by name. A model without declared
    runs (None) is screened. A declared run is compared when it is declared screened, or when its declared screening
    waits for AC-002 and VER-002 for window runs (benchmark_tranche3.WINDOW_PRINT_WAIT: the hand-entry runs),
    which the window print route compares meanwhile; any other run (a dynamics capture, which waits for a modal entry of the product) is a
    reference and is compared by no route."""
    if declaration is None or declaration.get("screened") is True:
        return True, ""
    waits = str(declaration.get("screening_waits_for") or "")
    if waits.startswith(WINDOW_PRINT_WAIT):
        return True, ""
    return False, ("the run is declared not screened; its screening waits for "
                   + (waits or "a condition the declaration does not name"))


def _bend_pipes(model: Mapping[str, Any]) -> set[str]:
    """The pipes realized as arcs: each bend component's bend_pipe_ref."""
    return {str(component["geometry"]["bend_pipe_ref"]) for component in model.get("components") or []
            if isinstance(component, Mapping) and component.get("kind") == "bend"
            and isinstance(component.get("geometry"), Mapping) and component["geometry"].get("bend_pipe_ref")}


def _anchors(model: Mapping[str, Any]) -> list[Mapping[str, Any]]:
    return [support for support in model.get("supports") or [] if isinstance(support, Mapping)
            and support.get("family") == "anchor"
            and sorted(support.get("restraints") or []) == sorted(dof for dof, _ in SPRING_DOFS)]


def _elongation_cases(model: Mapping[str, Any]) -> list[str]:
    setting = (model.get("model_settings") or {}).get("pressure_elongation") or {}
    return list(setting.get("operating_load_cases") or [])


def _elongation_members(model: Mapping[str, Any]) -> dict[str, list[tuple[str, Any]]]:
    """{case: [(pipe, its region's pressure quantity)]} for every member of a pressure region of an elongation case."""
    cases = set(_elongation_cases(model))
    out: dict[str, list[tuple[str, Any]]] = {}
    for case in model.get("load_cases") or []:
        if case.get("id") not in cases:
            continue
        for region in case.get("pressure_regions") or []:
            for pipe in region.get("member_pipe_ids") or []:
                out.setdefault(case["id"], []).append((str(pipe), region.get("pressure")))
    return out


def condition(entry: Mapping[str, Any], name: str,
              declaration: Mapping[str, Any] | None = None) -> tuple[bool, str]:
    """Whether a solve basis applies to a benchmark document, and, when it does not, why, by name. `declaration` is
    the run's manifest declaration, where the route has it (declared_compared)."""
    model = _model(entry)
    contract = model.get("pressure_contract")
    if name == SHEAR:
        if contract is not None:
            return False, (f"the document is on the exact pressure route, whose shear model is its pressure_contract's "
                           f"({contract.get('shear_model', EULER_BERNOULLI)}); the basis is the pressure-free "
                           "selection")
        selected = (model.get("model_settings") or {}).get("shear_model")
        if selected not in (None, EULER_BERNOULLI):
            return False, f"the document already selects {selected}"
        if entry.get("capture_only"):
            return False, "a capture-only run: no route compares it"
        compared, why = declared_compared(declaration)
        if not compared:
            return False, why
        for case in model.get("load_cases") or []:
            wind = ((case.get("equivalent_static") or {}).get("wind") or {})
            if wind.get("exposed_spans"):
                return False, (f"{case.get('id')} lists wind exposed spans, which the product refuses on a Timoshenko "
                               "straight member by name (docs/RELEASE_LIMITATIONS_v1.0.md)")
        return True, ""
    if name == ANCHOR:
        anchors = _anchors(model)
        if not anchors:
            return False, "the model has no anchor"
        ids = {str(anchor["id"]) for anchor in anchors}
        for case in model.get("load_cases") or []:
            for load in case.get("primitive_loads") or []:
                target = load.get("target") or {}
                if target.get("type") == "support" and target.get("support") in ids:
                    return False, (f"{load.get('id')} imposes a movement on anchor {target.get('support')}; a spring's "
                                   "base movement is not established as the other program's anchor movement")
        return True, ""
    if name in (THIN, STRAIGHT):
        if contract is None or entry.get("series_run") != ELONGATION_ON or not _elongation_cases(model):
            return False, "not the pressure-elongation-on run of a pressure series"
        members = _elongation_members(model)
        if not members:
            return False, "no pressure region in an elongation case"
        if name == STRAIGHT and not any(pipe in _bend_pipes(model) for items in members.values()
                                        for pipe, _ in items):
            return False, ("no realized bend in a pressure region of an elongation case: SWBPIPE elongates straight "
                           "members only here, so the basis would equal the unadjusted solve")
        return True, ""
    if name == TERMINAL:
        if contract is None:
            return False, "no pressure region"
        for case in model.get("load_cases") or []:
            for region in case.get("pressure_regions") or []:
                if any(terminal.get("closure_transfer") != TRANSFERS for terminal in region.get("terminals") or []):
                    return True, ""
        return False, "every terminal of every pressure region transfers to the wall"
    raise ValueError(f"unknown solve basis {name!r}")


def applicable(entry: Mapping[str, Any], declaration: Mapping[str, Any] | None = None) -> dict[str, tuple[bool, str]]:
    return {name: condition(entry, name, declaration) for name in SOLVE_BASES}


# -- realizations ----------------------------------------------------------------------------------------------------
class NotRepresentable(ValueError):
    """A basis whose condition holds cannot be realized for this document; the message says why."""


_LENGTH = {"m": 1.0, "mm": 1.0e-3}
_PER_DEGREE = ("1/degC", "1/K")  # an interval of one kelvin is an interval of one degree Celsius


def _section(model: Mapping[str, Any], pipe_id: str) -> tuple[Mapping[str, Any], Mapping[str, Any]]:
    pipe = next((item for item in model.get("pipe_segments") or [] if item.get("id") == pipe_id), None)
    if pipe is None:
        raise NotRepresentable(f"{pipe_id} is not a pipe segment of the model")
    material = next((item for item in model.get("materials") or [] if item.get("id") == pipe.get("material")), None)
    if material is None:
        raise NotRepresentable(f"{pipe_id}'s material is not in the model")
    return pipe, material


def _si(quantity: Any, unit: str, what: str) -> float:
    """A finite quantity in exactly `unit`, else NotRepresentable naming `what`."""
    if not isinstance(quantity, Mapping) or quantity.get("unit") != unit:
        raise NotRepresentable(f"{what} is not given in {unit} ({quantity!r})")
    value = quantity.get("value")
    if isinstance(value, bool) or not isinstance(value, (int, float)) or not math.isfinite(value):
        raise NotRepresentable(f"{what} is not a finite number ({quantity!r})")
    return float(value)


def _length_is_zero(quantity: Any, what: str) -> bool:
    """Whether an optional length (corrosion allowance, mill tolerance) is absent or zero; NotRepresentable when it is
    not a length."""
    if quantity is None:
        return True
    if not isinstance(quantity, Mapping) or quantity.get("unit") not in _LENGTH:
        raise NotRepresentable(f"{what} is not a length in m or mm ({quantity!r})")
    value = quantity.get("value")
    if isinstance(value, bool) or not isinstance(value, (int, float)) or not math.isfinite(value):
        raise NotRepresentable(f"{what} is not a finite number ({quantity!r})")
    return value == 0


def _alpha(material: Mapping[str, Any]) -> float:
    quantity = material.get("thermal_expansion_coefficient")
    if not isinstance(quantity, Mapping) or quantity.get("unit") not in _PER_DEGREE:
        raise NotRepresentable(f"{material.get('id')}'s expansion coefficient is absent or not in 1/degC or 1/K "
                               f"({quantity!r}); the strain is placed as a temperature interval")
    value = quantity.get("value")
    if isinstance(value, bool) or not isinstance(value, (int, float)) or not math.isfinite(value) or value == 0:
        raise NotRepresentable(f"{material.get('id')}'s expansion coefficient is zero or not finite ({quantity!r}); "
                               "a strain cannot be placed as a temperature interval")
    return float(value)


def representation(entry: Mapping[str, Any], name: str) -> str:
    """Why a basis whose condition holds cannot be realized for this document, or ''."""
    if name not in (THIN, STRAIGHT):
        return ""
    model = _model(entry)
    try:
        for case_id, items in _elongation_members(model).items():
            case = next(item for item in model.get("load_cases") or [] if item.get("id") == case_id)
            heated = {str((load.get("target") or {}).get("pipe")) for load in case.get("primitive_loads") or []
                      if load.get("category") == "thermal"}
            for pipe_id, pressure in items:
                pipe, material = _section(model, pipe_id)
                section = pipe.get("section") or {}
                if not (_length_is_zero(section.get("corrosion_allowance"), f"{pipe_id}'s corrosion allowance")
                        and _length_is_zero(section.get("mill_tolerance"), f"{pipe_id}'s mill tolerance")):
                    return f"{pipe_id} has a corrosion allowance or mill tolerance: its bore is not the nominal one"
                if material.get("temperature_points"):
                    return f"{material.get('id')} has temperature points: alpha is not one constant"
                if pipe_id in heated:
                    return f"{case_id} already heats {pipe_id}"
                _alpha(material)
                free_strain(model, pipe_id, _si(pressure, "Pa", f"{pipe_id}'s pressure in {case_id}"),
                            thin_wall=name == THIN)
    except NotRepresentable as why:
        return str(why)
    return ""


def free_strain(model: Mapping[str, Any], pipe_id: str, pressure_pa: float, *, thin_wall: bool) -> float:
    """The closed-end free pressure strain of a member: thick-wall (1 - 2 nu) p A_i/(E A_s), or thin-wall at the mean
    diameter (1 - 2 nu) p D_m/(4 t E)."""
    pipe, material = _section(model, pipe_id)
    od = _si((pipe.get("section") or {}).get("outside_diameter"), "m", f"{pipe_id}'s outside diameter")
    t = _si((pipe.get("section") or {}).get("wall_thickness"), "m", f"{pipe_id}'s wall thickness")
    e = _si(material.get("elastic_modulus"), "Pa", f"{material.get('id')}'s elastic modulus")
    nu = _si(material.get("poisson_ratio"), "1", f"{material.get('id')}'s Poisson's ratio")
    di = od - 2.0 * t
    if not (0.0 < t and 0.0 < di and e > 0.0):
        raise NotRepresentable(f"{pipe_id}'s section or modulus does not give a closed-end strain")
    if thin_wall:
        return (1.0 - 2.0 * nu) * pressure_pa * (od - t) / (4.0 * t * e)
    return (1.0 - 2.0 * nu) * pressure_pa * di * di / (e * (od * od - di * di))


def _place_strains(model: dict[str, Any], members: Mapping[str, list[tuple[str, Any]]], *, thin_wall: bool,
                   straight_only: bool) -> None:
    bends = _bend_pipes(model)
    for case in model["load_cases"]:
        for pipe_id, pressure in members.get(case["id"], []):
            if straight_only and pipe_id in bends:
                continue
            _, material = _section(model, pipe_id)
            alpha = _alpha(material)
            strain = free_strain(model, pipe_id, _si(pressure, "Pa", f"{pipe_id}'s pressure"), thin_wall=thin_wall)
            case.setdefault("primitive_loads", []).append({
                "category": "thermal", "dimension": "temperature_interval", "direction": "global_x",
                "id": f"{BASIS_LOAD_PREFIX}{'thin' if thin_wall else 'thick'}-strain:{pipe_id}",
                "magnitude": {"unit": "degC", "value": strain / alpha},
                "provenance": ("stated basis of the benchmark comparison: the closed-end free pressure strain "
                               f"{'(1 - 2 nu) p D_m/(4 t E)' if thin_wall else '(1 - 2 nu) p A_i/(E A_s)'} placed as "
                               "a temperature interval dT = eps/alpha"),
                "target": {"pipe": pipe_id, "type": "element"}})


def _without_elongation(model: dict[str, Any]) -> None:
    settings = model.get("model_settings") or {}
    settings.pop("pressure_elongation", None)
    if not settings:
        model.pop("model_settings", None)


def realize(entry: Mapping[str, Any], names: tuple[str, ...], *, k_translation: float = K_TRANSLATION,
            k_rotation: float = K_ROTATION,
            declaration: Mapping[str, Any] | None = None) -> tuple[dict[str, Any], dict[str, str]]:
    """The document SWBPIPE solves under the named solve bases (each of which must apply), and the map from each new
    support id to the support whose node it stands for. The argument is not changed. The stiffnesses are parameters
    only so that the mechanism can be examined on a closed form; the comparison uses the stated ones. Raises
    NotRepresentable when a basis cannot be realized for this document."""
    for name in names:
        applies, why = condition(entry, name, declaration)
        if not applies:
            raise ValueError(f"{name} does not apply: {why}")
        problem = representation(entry, name)
        if problem:
            raise NotRepresentable(f"{name}: {problem}")
    out = copy.deepcopy(dict(entry))
    model = out["model"]
    aliases: dict[str, str] = {}
    if SHEAR in names:
        model.setdefault("model_settings", {})["shear_model"] = TIMOSHENKO
    if ANCHOR in names:
        supports = []
        for support in model["supports"]:
            if support in _anchors(model):
                for dof, unit in SPRING_DOFS:
                    spring_id = f"{support['id']}:stated-stiffness:{dof}"
                    aliases[spring_id] = str(support["id"])
                    supports.append({"family": "spring", "id": spring_id, "node": support["node"],
                                     "provenance": "stated basis of the benchmark comparison: " + RIGID_SUPPORT["stated"],
                                     "restraints": [dof],
                                     "stiffness": {"dof": dof, "value": {
                                         "unit": unit, "value": k_translation if unit == "N/m" else k_rotation}}})
            else:
                supports.append(support)
        model["supports"] = supports
    if THIN in names or STRAIGHT in names:
        members = _elongation_members(model)
        _without_elongation(model)
        _place_strains(model, members, thin_wall=THIN in names, straight_only=STRAIGHT in names)
    if TERMINAL in names:
        for case in model["load_cases"]:
            for region in case.get("pressure_regions") or []:
                for terminal in region.get("terminals") or []:
                    terminal["closure_transfer"] = TRANSFERS
    out["stated_bases"] = list(names)
    # Closed forms belong to the plain model; a basis solve is another model, so it carries none.
    out["closed_form"] = []
    return out, aliases


def shear_model_of(entry: Mapping[str, Any]) -> str:
    """The shear model a document's solve uses, by name."""
    model = _model(entry)
    contract = model.get("pressure_contract")
    if contract is not None:
        return f"{contract.get('shear_model', EULER_BERNOULLI)} (pressure_contract)"
    selected = (model.get("model_settings") or {}).get("shear_model")
    return f"{selected} (model_settings)" if selected else f"{EULER_BERNOULLI} (no shear model selected)"


# -- the effective axial force binding (B) ---------------------------------------------------------------------------
def _unit(vector: tuple[float, float, float]) -> tuple[float, float, float] | None:
    norm = math.sqrt(sum(c * c for c in vector))
    return None if not norm > 0 else tuple(c / norm for c in vector)  # type: ignore[return-value]


def member_end_axes(model: Mapping[str, Any]) -> tuple[dict[tuple[str, str], tuple[float, float, float]],
                                                       dict[str, str]]:
    """The unit axis at each member end, in the direction from end_i to end_j, and why an end has none.

    A straight member's axis is from its from node to its to node. A realized bend's axis at an end is the axis of the
    one straight member that meets it there, taken only when that straight makes half the bend angle with the bend's
    chord (the arc is tangent to it), within 1e-9 rad."""
    nodes = {node["id"]: tuple(float(node["position"][axis]) for axis in "xyz") for node in model.get("nodes") or []}
    pipes = {pipe["id"]: pipe for pipe in model.get("pipe_segments") or []}
    bends = {str(component["geometry"]["bend_pipe_ref"]): component for component in model.get("components") or []
             if component.get("kind") == "bend"}
    axes: dict[tuple[str, str], tuple[float, float, float]] = {}
    missing: dict[str, str] = {}

    def chord(pipe: Mapping[str, Any]) -> tuple[float, float, float] | None:
        a, b = nodes.get(pipe["from"]), nodes.get(pipe["to"])
        return None if a is None or b is None else _unit(tuple(b[k] - a[k] for k in range(3)))

    for pipe_id, pipe in pipes.items():
        if pipe_id in bends:
            continue
        axis = chord(pipe)
        if axis is None:
            missing[pipe_id] = "its nodes do not give an axis"
        else:
            axes[(pipe_id, "end_i")] = axes[(pipe_id, "end_j")] = axis
    for pipe_id, component in bends.items():
        pipe = pipes.get(pipe_id)
        through = chord(pipe) if pipe else None
        angle = (component.get("geometry") or {}).get("bend_angle") or {}
        if through is None or angle.get("unit") != "rad":
            missing[pipe_id] = "the bend's chord or angle is not known"
            continue
        half = float(angle["value"]) / 2.0
        for end, node in (("end_i", pipe["from"]), ("end_j", pipe["to"])):
            meeting = [other for other_id, other in pipes.items() if other_id not in bends and other_id != pipe_id
                       and node in (other["from"], other["to"])]
            if len(meeting) != 1 or chord(meeting[0]) is None:
                missing[pipe_id] = f"no single straight member meets it at {node}"
                break
            straight = chord(meeting[0])
            # Oriented along the bend's travel: into the bend at end_i, out of it at end_j.
            sign = 1.0 if (meeting[0]["to"] == node) == (end == "end_i") else -1.0
            tangent = tuple(sign * c for c in straight)
            cosine = max(-1.0, min(1.0, sum(tangent[k] * through[k] for k in range(3))))
            if abs(math.acos(cosine) - half) > 1e-9:
                missing[pipe_id] = f"the straight member at {node} is not tangent to the arc"
                break
            axes[(pipe_id, end)] = tangent  # type: ignore[assignment]
        else:
            continue
        axes.pop((pipe_id, "end_i"), None)
        axes.pop((pipe_id, "end_j"), None)
    return axes, missing


def pressurised_members(model: Mapping[str, Any]) -> dict[str, set[str]]:
    """{case: members of its pressure regions}."""
    return {str(case.get("id")): {str(pipe) for region in case.get("pressure_regions") or []
                                  for pipe in region.get("member_pipe_ids") or []}
            for case in model.get("load_cases") or []}


# -- the comparison under each basis ---------------------------------------------------------------------------------
def _statement(name: str) -> dict[str, str]:
    return {"name": name, **STATEMENTS[name]}


def _key(row: Mapping[str, Any]) -> tuple:
    return (row.get("case"), row.get("kind"), row.get("entity"), row.get("end"), row.get("component"))


def _counts(rows: list[Mapping[str, Any]]) -> dict[str, Any]:
    from core.handoff.caepipe_external.benchmark import FINDING, UNMATCHED, WITHIN

    counts: dict[str, Any] = {name: sum(1 for row in rows if row["classification"] == name)
                              for name in (WITHIN, FINDING, UNMATCHED)}
    counts["zero_against_zero"] = {name: sum(1 for row in rows if "zero_against_zero" in row
                                             and row["classification"] == name) for name in (WITHIN, FINDING)}
    return counts


def _axial_block(entry: Mapping[str, Any], record_of, plain_rows: list[Mapping[str, Any]]) -> dict[str, Any] | None:
    """The effective axial force binding's rows beside the wall binding's, for the members pressurised in each
    compared case of one solve (the operating case: in any of the cases it sums), or None when no member is
    pressurised in any case. `record_of(end_force_binding)` builds the record of that solve. The binding is applied
    only where at least one end force was bound under it."""
    model = _model(entry)
    pressurised = pressurised_members(model)
    if not any(pressurised.values()):
        return None
    axes, missing = member_end_axes(model)
    record = record_of({"axes": axes, "pressurised": pressurised})
    by_case = {str(case["case"]): set().union(*(pressurised.get(str(c)) or set() for c in case["swbpipe_cases"]))
               for case in record.get("load_cases") or [] if case.get("swbpipe_cases")}
    plain = {_key(row): row for row in plain_rows}
    rows = []
    for row in record["results"]:
        if row.get("kind") == "element_force" and row.get("entity") in by_case.get(str(row.get("case")), set()):
            twin = plain.get(_key(row)) or {}
            rows.append({**row, "wall_binding": {"swbpipe": (twin.get("values") or {}).get("swbpipe"),
                                                 "classification": twin.get("classification")}})
    members = sorted(set().union(*by_case.values())) if by_case else []
    bound = sum(1 for row in rows if (row.get("values") or {}).get("swbpipe") is not None)
    block = {**_statement(EFFECTIVE_AXIAL), "members": members,
             "members_by_case": {case: sorted(found) for case, found in sorted(by_case.items())},
             "axis_not_known": {member: why for member, why in sorted(missing.items()) if member in members},
             "counts": _counts(rows), "results": rows}
    if not bound:
        return {**block, "applied": False, "outcome": NOT_APPLICABLE,
                "not_applied": "no end force of a member pressurised in a compared case was bound under it"}
    return {**block, "applied": True, "outcome": APPLIED}


def compare_bases(entry: Mapping[str, Any], *, plain_payload: Mapping[str, Any], plain_record: Mapping[str, Any],
                  mbf_result: Any, mbf_text: str, parsed_caepipe: Mapping[str, Any] | None,
                  solve, record_options: Mapping[str, Any] | None = None,
                  declaration: Mapping[str, Any] | None = None) -> dict[str, Any]:
    """Every stated basis beside the plain comparison of one benchmark document, after the plain record is written.

    `solve(document, names)` solves SWBPIPE's side of a basis document in a solve folder of its own and returns
    (payload or None, error, solve record). The plain record is not changed. Each basis ends in a named outcome
    (OUTCOMES) and never raises: one whose condition does not hold, that cannot be represented, or that the product
    refuses is listed with the reason and compares nothing. `record_options` are passed to every record built here as
    they were to the plain one; `declaration` is the run's manifest declaration (declared_compared)."""
    from core.handoff.caepipe_external.benchmark import build_benchmark_record

    def record(document: Mapping[str, Any], payload: Mapping[str, Any], **extra: Any) -> dict[str, Any]:
        return build_benchmark_record(document, swbpipe_result_payload=payload, mbf_result=mbf_result,
                                      mbf_text=mbf_text, parsed_caepipe=parsed_caepipe,
                                      **dict(record_options or {}), **extra)

    plain = {"shear_model": shear_model_of(entry),
             "axial_binding": ("wall: SWBPIPE's element_global_end_force_v1, whose axial part on a pressurised member "
                               "is the wall force N_w"),
             "anchors": "ideal: every restrained component held exactly",
             "note": "the record's results are this comparison, unadjusted"}
    compared, why = declared_compared(declaration)
    if not compared:
        return {"rule": RULE, "plain": plain,
                "bases": [{**_statement(name), "applied": False, "outcome": NOT_COMPARED, "not_applied": why}
                          for name in (EFFECTIVE_AXIAL, *SOLVE_BASES, TOGETHER)]}

    def unrepresentable(name: str, error: BaseException, **extra: Any) -> dict[str, Any]:
        return {**_statement(name), "applied": False, "outcome": NOT_REPRESENTABLE, **extra,
                "not_applied": f"{type(error).__name__}: {error}"}

    bases: list[dict[str, Any]] = []
    try:
        axial = _axial_block(entry, lambda binding: record(entry, plain_payload, end_force_binding=binding),
                             plain_record["results"])
        bases.append(axial or {**_statement(EFFECTIVE_AXIAL), "applied": False, "outcome": NOT_APPLICABLE,
                               "not_applied": "no member of a pressure region in a compared case"})
    except Exception as error:  # an optional basis never costs the plain record
        bases.append(unrepresentable(EFFECTIVE_AXIAL, error))
    try:
        conditions = applicable(entry, declaration)
    except Exception as error:
        conditions = {name: (False, f"{type(error).__name__}: {error}") for name in SOLVE_BASES}
    problems = {name: (representation(entry, name) if conditions[name][0] else "") for name in SOLVE_BASES}
    usable = [name for name in SOLVE_BASES if conditions[name][0] and not problems[name]]
    runs = [(name, (name,)) for name in SOLVE_BASES] + [(TOGETHER, tuple(usable))]
    for name, names in runs:
        if name != TOGETHER and not conditions[name][0]:
            bases.append({**_statement(name), "applied": False, "outcome": NOT_APPLICABLE,
                          "not_applied": conditions[name][1]})
            continue
        if name != TOGETHER and problems[name]:
            bases.append({**_statement(name), "applied": False, "outcome": NOT_REPRESENTABLE,
                          "not_applied": problems[name]})
            continue
        if name == TOGETHER and len(names) < 2:
            bases.append({**_statement(name), "applied": False, "outcome": NOT_APPLICABLE,
                          "not_applied": f"{len(names)} solve basis applies; nothing to combine"})
            continue
        try:
            document, aliases = realize(entry, names, declaration=declaration)
        except Exception as error:
            bases.append(unrepresentable(name, error, bases_combined=list(names)))
            continue
        block: dict[str, Any] = {**_statement(name), "bases_combined": list(names),
                                 "shear_model": shear_model_of(document)}
        try:
            payload, error, solved = solve(document, names)
        except Exception as raised:
            payload, error, solved = None, f"{type(raised).__name__}: {raised}", {}
        block["swbpipe_solve"] = solved
        if error or payload is None:
            block.update(applied=False, outcome=REFUSED_BY_PRODUCT, solved_by_product=False,
                         refusal=error or "no result",
                         not_applied="no comparison under this basis: SWBPIPE did not solve it")
            bases.append(block)
            continue
        try:
            basis_record = record(document, payload, support_aliases=aliases)
            block.update(applied=True, outcome=APPLIED, solved_by_product=True, summary=basis_record["summary"],
                         results=basis_record["results"], element_binding=basis_record["element_binding"])
            axial = _axial_block(document, lambda binding, d=document, p=payload: record(
                d, p, support_aliases=aliases, end_force_binding=binding), basis_record["results"])
            if axial is not None:
                block["effective_axial_force_binding"] = {key: axial[key] for key in (
                    "applied", "outcome", "members", "members_by_case", "axis_not_known", "counts", "results")
                    if key in axial}
        except Exception as raised:
            block.update(applied=False, outcome=NOT_REPRESENTABLE, solved_by_product=True,
                         not_applied=f"the basis solve's comparison could not be formed: "
                                     f"{type(raised).__name__}: {raised}")
            for key in ("summary", "results", "element_binding"):
                block.pop(key, None)
        bases.append(block)
    return {"rule": RULE, "plain": plain, "bases": bases}
