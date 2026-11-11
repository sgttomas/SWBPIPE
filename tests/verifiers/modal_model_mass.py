"""Independent model-derived mass assurance for modal result documents (DEL-04-09
design, "Selected mass assurance: next
modal unit after CE"; §1.3, §1.5, §2.1 to §2.4).

The payload relations of `modal_evidence` check totals and participation against the
document's own formed M. They cannot see a coherently wrong M whose totals and
participation were resealed to agree with it. This leaf derives the mass a bound
request's model implies, from the authored request alone, and holds the document to
it:

- the full symmetric global M, entry by entry over the union of derived and published
  upper entries (a missing entry is 0), so rotary, Rayleigh, torsional and
  translation-rotation coupling terms invisible to a·M·R_k are checked;
- the restraint partition (the free DOFs), the directional totals T_k = R_kᵀMR_k, the
  free-DOF masses R_f,kᵀM_ffR_f,k and the mass centre from M's first moments;
- from the derived M and the PUBLISHED shapes: m_i = φᵀM_ffφ, p_k = (M R_k)_f, Γ,
  m_eff, the fractions against T_k and their cumulative sums.

**Independence.** It decodes the request's model itself (project length unit, shared
section binding, route, node order and coordinates, pipe ends, component mapping,
supports and the named state case) through the unit catalog (`units`) only, and forms
M from the Design's formulas: no producer preparation, self-weight, eligibility, mass,
kernel or other reader code is imported or run, and no producer-authored mass recipe
or normalized leaf is read. The product's source was read only for its contracts
(which inputs it admits or refuses, which members bear mass) and for the operation
shapes its rounding model covers (below).

**Arithmetic and the comparison.** Every quantity is an interval with exact rational
ends (`modal_model_mass_interval`). Each derived entry carries two enclosures:

- `exact`: the Design formula's real value from the authored binary64 inputs;
- `envelope`: every value the producer's stated arithmetic can form from the same
  inputs: its binary64 section quantities (each binary64 or `Scaled` operation of
  `stable_mass_per_length`'s source areas and of §2.2's j̄ form rounding by at most
  u = 2⁻⁵³, simulated on intervals so cancellation is covered), its body mass
  fl(W/a), its double-word element formation and assembly (at most 2⁻⁹⁰ of the
  entry's absolute term sum, `DD_RELATIVE`), and the one final binary64 rounding.

A published value is consistent when it lies in its envelope; an established
contradiction is a published value outside every value the envelope admits; a value
the rounding model does not cover (an overflow, a divisor that may vanish, a flagged
or outside-the-profile published value that the envelope meets) is unresolved, never
passed. No tolerance is selected here and no current threshold changes: the envelope
is the rounding model, and its premises are stated beside it.

**Coverage, and what is scoped.** Covered: PF and EX sections (metal on t − c_m or t,
contents on the bore of t − c_m or (t − c_m) − c_a, insulation, with no zero default
for a missing density or half an insulation pair); straight members (tee legs,
geometry-only bends and annotation-only expansion joints are straight pipe); realized
arcs as two rigid half-arcs; the five rigid-family component kinds with self-weight's
eligibility, mapping conflicts and pipe-mass fallback; objective connectors' replaced
spans, tie rods and supports as massless; the axis restraint partition (rigid
families, springs and hangers free, constant-effort supports absent); and a named
state case's nonlinear supports, whose partition from the REPORTED states is checked
but stays conditional on their qualification (scoped, never passed as unconditional
assurance). Refused producer domains (skew records in modal-1, snubbers, Timoshenko,
condition contents on the state case, gap supports, mapping
conflicts and the like) make a solved document a contradiction (`RefusedDomain`).
A basis this reader does not cover (`Uncovered`) is scoped unsupported by name. This
verifies the entered model mass only: not omitted real-world masses, discretization
(R3), K, the eigenpairs, the state qualification, Bessel or the recording and
resource conditions.

**The subject census** (Design §5.3.2, `census`) is a separate structural pass over the
same model: which members, arcs, component bodies and supports the run reads, by the
subject rule stated there. It reads no numerical enclosure, so an unresolved or
out-of-model mass quantity never drops a subject; a mass refusal stays the mass
relation's failure and never changes the list.

**Resources.** The work is linear in the model's members and in the document's mass
entries, and proportional to modes × entries for participation; every list it walks
is already in memory. `work_units` states the estimate from the input counts before
any formation, and a caller-supplied `budget` makes the check fallible: an estimate
above it is scoped before the work starts. No production budget is selected here.
"""
from __future__ import annotations

from fractions import Fraction
import json
import math
from typing import Any

from tests.verifiers import units
from tests.verifiers.modal_model_mass_interval import (
    DD_ABSOLUTE, DD_RELATIVE, DD_RESOLUTION_LIMIT, below_floor, dd_bound, LARGEST, NORMAL_MIN, ONE, PROFILE_FLOOR, U, ZERO, Iv, OutsideModel, Undecidable, asin_and_cos, binary64,
    exact, pi, published_image, relative, scaled, sqrt,
)

# The model document version this reader's basis covers (named only here).
MODEL_VERSION = "0.9.0"
UNRESOLVED = "unresolved: "
COMPONENTS = ("ux", "uy", "uz", "rx", "ry", "rz")
DIRECTIONS = ("x", "y", "z")
DOF_TOKENS = {spelling: index for index, name in enumerate(("UX", "UY", "UZ", "RX", "RY", "RZ"))
              for spelling in (name, name[0] + name[1].lower(), name.lower())}
RIGID_FAMILY = ("valve", "flange", "reducer", "rigid", "specialty")
BEND_KINDS = ("bend", "elbow")
GEOMETRY_ONLY = "mechanics_geometry_only"
NOT_SOLVER_CONSUMED = "not_solver_consumed"
CURVED_BEND = "curved_bend_macro_element"
TIMOSHENKO = "timoshenko_annulus_energy_v1"
COMPONENT_WEIGHT_METHOD = "component_weight_at_center_of_gravity_global_offset/v1"
BEHAVIORS = {"one_way": "one_way", "one-way": "one_way", "oneway": "one_way", "lift_off": "lift_off", "lift-off": "lift_off",
             "liftoff": "lift_off", "gap": "gap", "friction": "friction"}
# The curved-bend admission's binary64 plane test (DEC-070), a contract value of the
# producer's admission, not a tolerance of this reader.
CURVED_BEND_PLANE_TOLERANCE = 1.0e-9


class RefusedDomain(Exception):
    """The model is one the producer refuses by a stated rule, so a solved document of
    it is a contradiction."""


class Uncovered(Exception):
    """The model needs a basis this reader does not cover: scoped unsupported."""


class Contradiction(Exception):
    """An established contradiction between the document and the model's mass."""


class ReaderSelfCheck(Exception):
    """The reader's own cross-checks disagree (a defect of this reader): unresolved."""


# --- decoding --------------------------------------------------------------------------------

def _obj(value, what) -> dict:
    if not isinstance(value, dict):
        raise RefusedDomain(f"{what}: not an object")
    return value


def _list(value, what) -> list:
    if not isinstance(value, list):
        raise RefusedDomain(f"{what}: not an array")
    return value


def _text(value, what) -> str:
    if not isinstance(value, str):
        raise RefusedDomain(f"{what}: not a string")
    return value


def _finite(value, what) -> float:
    if type(value) not in (int, float) or not math.isfinite(value):
        raise RefusedDomain(f"{what}: not a finite number")
    return float(value)


def _convert(value, unit, dimension, what) -> float:
    """One authored value in its dimension's canonical unit, through the catalog."""
    value = _finite(value, f"{what}/value")
    unit = _text(unit, f"{what}/unit")
    try:
        return units.to_canonical(value, unit, dimension)
    except ValueError as error:
        raise RefusedDomain(f"{what}: {error}") from None


def _quantity(q, dimension, what) -> float:
    q = _obj(q, what)
    return _convert(q.get("value"), q.get("unit"), dimension, what)


def _optional(section, key):
    """A section member: absent or null means no value (never zero)."""
    value = section.get(key)
    return None if value is None else value


def _dof(token, what) -> int:
    if not isinstance(token, str) or token not in DOF_TOKENS:
        raise RefusedDomain(f"{what}: `{token}` is not a frame DOF")
    return DOF_TOKENS[token]


def _nonblank(value):
    return value if isinstance(value, str) and value.strip() else None


def _consumption(component, default=None):
    interface = component.get("mechanics_interface")
    if isinstance(interface, dict) and isinstance(interface.get("solver_consumption"), str):
        return interface["solver_consumption"]
    return default


# --- the producer's section quantities, enclosed ------------------------------------------------

class Section:
    """m̄ (with its three categories), j̄ and ī: each as (exact, envelope)."""

    __slots__ = ("mass", "metal", "contents", "insulation", "polar", "bending")


def _section(pipe_id, section, exact_route, label) -> Section:
    """Design §2.1 and §2.2 on the route's basis. `exact` intervals are the Design
    formula's value; `envelope` intervals enclose every value the producer's binary64
    and `Scaled` operations can form from the same authored inputs (the operation
    shapes of `annulus_geometry` and `stable_mass_per_length`, and §2.2's j̄ written
    through the wall, each operation rounding by at most u)."""
    what = f"{label} {pipe_id}"
    od = _quantity(section.get("outside_diameter"), "length", f"{what} outside_diameter")
    t = _quantity(section.get("wall_thickness"), "length", f"{what} wall_thickness")
    mill_q = _optional(section, "mill_tolerance")
    mill = 0.0 if mill_q is None else _quantity(mill_q, "length", f"{what} mill_tolerance")
    allowance_q = section.get("corrosion_allowance", None) if "corrosion_allowance" in section else None
    if "corrosion_allowance" in section and section["corrosion_allowance"] is None:
        raise RefusedDomain(f"{what}: a present null corrosion_allowance is refused")
    allowance = None if allowance_q is None else _quantity(allowance_q, "length", f"{what} corrosion_allowance")
    if not od > 0.0:
        raise RefusedDomain(f"{what}: outside diameter must be positive")
    if not t > 0.0:
        raise RefusedDomain(f"{what}: wall thickness must be positive")
    if mill < 0.0:
        raise RefusedDomain(f"{what}: mill deduction must be nonnegative")
    density_q = _optional(section, "material_density")
    if density_q is None:
        raise RefusedDomain(f"{what}: explicit material density required (no zero default)")
    density = _quantity(density_q, "density", f"{what} material_density")
    if not density > 0.0:
        raise RefusedDomain(f"{what}: material density must be positive")
    contents_q = _optional(section, "contents_density")
    contents = None if contents_q is None else _quantity(contents_q, "density", f"{what} contents_density")
    thickness_q, insulation_q = _optional(section, "insulation_thickness"), _optional(section, "insulation_density")
    if (thickness_q is None) != (insulation_q is None):
        raise RefusedDomain(f"{what}: insulation mass requires both thickness and density (no zero default)")
    ins_t = None if thickness_q is None else _quantity(thickness_q, "length", f"{what} insulation_thickness")
    ins_rho = None if insulation_q is None else _quantity(insulation_q, "density", f"{what} insulation_density")

    # The walls, in the producer's binary64 order (admission predicates evaluated as it does).
    if exact_route:
        if allowance is None:
            raise RefusedDomain(f"{what}: explicit corrosion allowance required on an exact-contract section")
        if allowance < 0.0:
            raise RefusedDomain(f"{what}: corrosion allowance must be nonnegative")
        metal_wall_f, bore_wall_f = t, (t - mill) - allowance
        metal_wall_x = exact(t)
        bore_wall_x = exact(t) - exact(mill) - exact(allowance)
        metal_wall_e = Iv(t)
        bore_wall_e = binary64(binary64(Iv(t) - Iv(mill)) - Iv(allowance))
    else:
        metal_wall_f = bore_wall_f = t - mill
        metal_wall_x = bore_wall_x = exact(t) - exact(mill)
        metal_wall_e = bore_wall_e = binary64(Iv(t) - Iv(mill))

    def valid_wall(wall_f):
        return math.isfinite(wall_f) and wall_f > 0.0 and not wall_f > od and not wall_f > od - wall_f

    if not valid_wall(metal_wall_f):
        raise RefusedDomain(f"{what}: invalid source area input (metal wall)")
    if contents is not None and not valid_wall(bore_wall_f):
        raise RefusedDomain(f"{what}: invalid source area input (bore wall)")
    if contents is not None and contents < 0.0:
        raise RefusedDomain(f"{what}: contents density must be nonnegative")
    if ins_t is not None and (ins_t < 0.0 or ins_rho < 0.0):
        raise RefusedDomain(f"{what}: insulation thickness and density must be nonnegative")

    PI = pi()
    PI_F = relative(PI, U)  # the binary64 constant fl(π), within u of π
    OD = Iv(od)
    s = Section()
    # Metal: ρ·π·w·(D − w) (exact), the producer's Scaled π·w·(D − w).
    w_x, w_e = Iv(metal_wall_x), metal_wall_e
    area_x = PI * w_x * (OD - w_x)
    area_e = binary64(scaled(scaled(PI_F * w_e) * scaled(OD - w_e, exact(od))), f"{what} metal area")
    metal = (Iv(density) * area_x, binary64(Iv(density) * area_e, f"{what} metal mass"))
    total_x, total_e = metal
    contents_pair = (ZERO, ZERO)
    if contents is not None:
        b_x, b_e = Iv(bore_wall_x), bore_wall_e
        ri_x = OD / Iv(2) - b_x
        ri_e = scaled(OD / Iv(2) - b_e, exact(od))
        bore_x = PI * ri_x.square()
        bore_e = binary64(scaled(scaled(PI_F * ri_e) * ri_e), f"{what} bore area")
        contents_pair = (Iv(contents) * bore_x, binary64(Iv(contents) * bore_e, f"{what} contents mass"))
        total_x, total_e = total_x + contents_pair[0], binary64(total_e + contents_pair[1], f"{what} mass per length")
    insulation_pair = (ZERO, ZERO)
    if ins_t is not None:
        T = Iv(ins_t)
        ins_area_x = PI * T * (OD + T)
        ins_area_e = binary64(scaled(scaled(PI_F * T) * scaled(OD + T, exact(od))), f"{what} insulation area")
        insulation_pair = (Iv(ins_rho) * ins_area_x, binary64(Iv(ins_rho) * ins_area_e, f"{what} insulation mass"))
        total_x, total_e = total_x + insulation_pair[0], binary64(total_e + insulation_pair[1], f"{what} mass per length")
    if not total_x.excludes_zero() or total_x.hi <= 0:
        raise RefusedDomain(f"{what}: total positive mass is outside the representable range")
    s.mass, s.metal, s.contents, s.insulation = (total_x, total_e), metal, contents_pair, insulation_pair

    # j̄ = ρ_m·J_metal + ρ_ins·J_ins (contents excluded, D-e); J of an annulus π(Do⁴ − Di⁴)/32.
    inner_x = OD - Iv(2) * w_x
    polar_x = Iv(density) * PI * (OD.square().square() - inner_x.square().square()) / Iv(32)
    inner_e = binary64(OD - Iv(2) * w_e, f"{what} inner diameter")
    sum_e = binary64(binary64(OD.square()) + binary64(inner_e.square()))
    metal_j_e = binary64(binary64(binary64(PI_F / Iv(8) * sum_e) * binary64(OD - w_e)) * w_e, f"{what} polar inertia")
    polar_e = binary64(Iv(density) * metal_j_e)
    if ins_t is not None:
        T = Iv(ins_t)
        outer_x = OD + Iv(2) * T
        polar_x = polar_x + Iv(ins_rho) * PI * (outer_x.square().square() - OD.square().square()) / Iv(32)
        outer_e = binary64(OD + Iv(2) * T)
        sum2_e = binary64(binary64(outer_e.square()) + binary64(OD.square()))
        ins_j_e = binary64(binary64(binary64(PI_F / Iv(16) * sum2_e) * binary64(outer_e + OD)) * T, f"{what} insulation inertia")
        polar_e = binary64(polar_e + binary64(Iv(ins_rho) * ins_j_e))
    bending_e = polar_e / Iv(2)
    if bending_e.mig() < 2 * NORMAL_MIN:
        bending_e = binary64(bending_e)
    s.polar, s.bending = (polar_x, polar_e), (polar_x / Iv(2), bending_e)
    # The reader's own cross-check: the producer-shaped envelope holds the Design value.
    for name, (x, e) in (("m̄", s.mass), ("j̄", s.polar), ("ī", s.bending)):
        if not e.contains_interval(x):
            raise ReaderSelfCheck(f"{what}: the envelope of {name} does not hold its Design value")
    return s


# --- element masses ------------------------------------------------------------------------------

class Accumulator:
    """Global upper entries: exact enclosure, producer envelope (before the final
    rounding), the κ-weighted absolute term sum and the contribution count the
    double-word bound (`dd_bound`) scales with."""

    def __init__(self):
        self.entries: dict[tuple[int, int], list] = {}

    def add(self, a: int, b: int, exact_term: Iv, envelope_term: Iv, conditioning: Fraction = Fraction(1),
            scale: Fraction | None = None, outside: str | None = None):
        """`scale` is the absolute term sum of the producer's formation of this
        contribution (default: the term's own magnitude, exact where no frame rotation
        or cancellation intervenes, as for a body's products)."""
        if a > b:
            a, b = b, a
        scale = conditioning * (envelope_term.mag() if scale is None else scale)
        held = self.entries.get((a, b))
        if held is None:
            self.entries[(a, b)] = [exact_term, envelope_term, scale, 1, conditioning, outside]
        else:
            held[0] = held[0] + exact_term
            held[1] = held[1] + envelope_term
            held[2] += scale
            held[3] += 1
            held[4] = max(held[4], conditioning)
            held[5] = held[5] or outside


def _cross(e):
    """[e]×, the matrix of v ↦ e × v."""
    return ((ZERO, -e[2], e[1]), (e[2], ZERO, -e[0]), (-e[1], e[0], ZERO))


HERMITE = ((156, (22, 1), 54, (-13, 1)), ((22, 1), (4, 2), (13, 1), (-3, 2)), (54, (13, 1), 156, (-22, 1)),
           ((-13, 1), (-3, 2), (-22, 1), (4, 2)))
RAYLEIGH = ((36, (3, 1), -36, (3, 1)), ((3, 1), (4, 2), (-3, 1), (-1, 2)), (-36, (-3, 1), 36, (-3, 1)),
            ((3, 1), (-1, 2), (-3, 1), (4, 2)))


def _coefficient(entry, length: Iv) -> Iv:
    if isinstance(entry, tuple):
        c, power = entry
        out = Iv(c)
        for _ in range(power):
            out = out * length
        return out
    return Iv(entry)


def _straight_blocks(xi, xj, mbar: Iv, ibar: Iv, jbar: Iv, geometry, floors: list):
    """The 12×12 consistent mass of Design §2.2 in global axes, frame-free (a circular
    section's mass does not depend on the member's y/z orientation): transverse
    displacement d = (I − eeᵀ)u and slope s = θ × e = −[e]×θ (so that along +x the planes
    act on [u_y, θ_z] and [u_z, −θ_y]); Hermitian translation m̄L/420·H and Rayleigh
    ī/(30L)·R on (d_i, s_i, d_j, s_j), linear axial m̄L and torsional j̄L. Returns
    ({(r, c): Iv}, {(r, c): scale}) for r ≤ c over local DOFs 0..11: the scale bounds
    the absolute terms the producer's frame rotation sums for that entry (|R| ≤ 1, so
    the member-axis 3×3 block's absolute entries: |axial| + 2|plane| on a translation
    or rotation pair, 2|plane| on a coupling), which an entry cancelling to zero in
    this frame-free form does not show."""
    length, e = geometry
    proj = [[(ONE if a == b else ZERO) - e[a] * e[b] for b in range(3)] for a in range(3)]
    axis = [[e[a] * e[b] for b in range(3)] for a in range(3)]
    cross = _cross(e)
    transverse = mbar * length / Iv(420)
    rotary = ibar / (Iv(30) * length)
    floors.extend(f"straight member {what}" for what, x in (("m̄L/420", transverse), ("ī/(30L)", rotary), ("L", length))
                  if below_floor(x))
    out: dict[tuple[int, int], Iv] = {}
    scales: dict[tuple[int, int], Fraction] = {}

    def put(r, c, value, scale):
        if r > c:
            r, c = c, r
        out[(r, c)] = out.get((r, c), ZERO) + value
        scales[(r, c)] = scales.get((r, c), Fraction(0)) + scale

    for g in range(4):
        for h in range(4):
            parts = (transverse * _coefficient(HERMITE[g][h], length), rotary * _coefficient(RAYLEIGH[g][h], length))
            floors.extend("straight member plane coefficient" for part in parts if below_floor(part))
            coef = parts[0] + parts[1]
            node_g, node_h = g // 2, h // 2
            slope_g, slope_h = g % 2 == 1, h % 2 == 1
            for a in range(3):
                for b in range(3):
                    if not slope_g and not slope_h:
                        geo = proj[a][b]
                    elif not slope_g and slope_h:
                        geo = -cross[a][b]
                    elif slope_g and not slope_h:
                        geo = cross[a][b]
                    else:
                        geo = proj[a][b]
                    r = 6 * node_g + (3 if slope_g else 0) + a
                    c = 6 * node_h + (3 if slope_h else 0) + b
                    if r <= c:
                        put(r, c, coef * geo, 2 * coef.mag())
    third, sixth = Iv(Fraction(1, 3)), Iv(Fraction(1, 6))
    for node_g in range(2):
        for node_h in range(2):
            factor = third if node_g == node_h else sixth
            for a in range(3):
                for b in range(3):
                    for offset, q in ((0, mbar), (3, jbar)):
                        r, c = 6 * node_g + offset + a, 6 * node_h + offset + b
                        if r <= c:
                            if below_floor(q * length * factor):
                                floors.append("straight member axial or torsional coefficient")
                            put(r, c, q * length * factor * axis[a][b], (q * length * factor).mag())
    return out, scales


def _frame_conditioning(xi, xj, y, what) -> Fraction:
    """An upper bound on 1 + κ, κ = |y|/|y⊥| with y⊥ the part of the y reference normal
    to the chord: the producer forms the member frame from y in double word, so its
    formation error scales with it (`dd_bound`). A y reference parallel to the chord
    is refused by the producer's frame; one parallel only within rounding is
    outside this reader's basis."""
    d = [Iv(xj[k]) - Iv(xi[k]) for k in range(3)]
    yy = [Iv(v) for v in y]
    dd_ = d[0].square() + d[1].square() + d[2].square()
    yd = yy[0] * d[0] + yy[1] * d[1] + yy[2] * d[2]
    y2 = yy[0].square() + yy[1].square() + yy[2].square()
    # |y⊥|² = |y|² − (y·d)²/|d|², bounded below by the exact form's enclosure.
    perp2 = y2 - yd.square() / dd_
    if perp2.hi <= 0:
        raise RefusedDomain(f"{what}: y_reference is parallel to the chord")
    if perp2.lo <= 0:
        raise Uncovered(f"{what}: y_reference is parallel to the chord within rounding; the frame's conditioning is not bounded")
    return Fraction(1) + (sqrt(y2) / sqrt(perp2)).hi


def _straight_geometry(xi, xj):
    d = [Iv(xj[k]) - Iv(xi[k]) for k in range(3)]
    squared = d[0].square() + d[1].square() + d[2].square()
    if not squared.excludes_zero():
        raise RefusedDomain("a straight member has zero length")
    length = sqrt(squared)
    return length, [d[k] / length for k in range(3)]


def _rigid_block(mass: Iv, first: list, second: list):
    """The 6×6 spatial mass [[m·I, −[S]×], [[S]×, J]] with S the first moment and J the
    second moment about the node (upper entries)."""
    out = {}
    for a in range(3):
        out[(a, a)] = mass
    cross = _cross(first)
    for a in range(3):
        for b in range(3):
            out[(a, 3 + b)] = -cross[a][b]
    for a in range(3):
        for b in range(a, 3):
            out[(3 + a, 3 + b)] = second[a][b]
    return out


def _floor_reason(floors: list, owner: str) -> str | None:
    """The premise breach carried by every entry an element contributes to, or None.

    A nonzero double-word intermediate that may lie below the kernel's low-word floor
    (2^-969, `LOCAL_ENTRY_MIN`) leaves the relative rounding premise; the element's
    entries become undecidable, while every other entry is still compared."""
    if not floors:
        return None
    return (f"{owner}: a double-word intermediate ({floors[0]}) may lie below the low-word floor 2^-969, "
            "outside the rounding model")


def _body_block(mass: Iv, cog, floors: list) -> dict:
    """A point mass at the centre of gravity r from its node:
    [[m·I, −m[r]×], [m[r]×, m(|r|²I − rrᵀ)]]; its own inertia is unentered, so zero."""
    r = [Iv(v) for v in cog]
    first = [mass * r[a] for a in range(3)]
    # The producer forms m, m·r_a and (m·r_a)·r_b in double word (`component_mass`).
    if any(below_floor(x) for x in [mass, *first, *(first[a] * r[b] for a in range(3) for b in range(3))]):
        floors.append("component body")
    squared = r[0].square() + r[1].square() + r[2].square()
    second = [[(mass * squared if a == b else ZERO) - mass * r[a] * r[b] for b in range(3)] for a in range(3)]
    return _rigid_block(mass, first, second)


def _arc_lumps(xi, xj, radius, y, mbar: Iv, ibar: Iv, jbar: Iv, trig, floors: list):
    """two rigid half-arcs (half-angle α, mass m̄Rα each) about their end
    nodes, with first moment ∫m̄ρ ds and second moment ∫m̄(|ρ|²I − ρρᵀ) ds + ∫(j̄ttᵀ +
    ī(I − ttᵀ)) ds. In the half's own frame (e1 radial outward at the node, e2 the
    tangent into the half, e3 = e1 × e2), ρ(θ) = R((cos θ − 1)e1 + sin θ e2), θ ∈ [0, α]:
    S = m̄R²(sin α − α, 1 − cos α, 0); Jc = m̄R³[[α/2 − sc/2, 1 − c − s²/2, 0],
    [·, 3α/2 + sc/2 − 2s, 0], [0, 0, 2(α − s)]]; the section term R[j̄·Tt + ī(αI − Tt)]
    with Tt = [[α/2 − sc/2, −s²/2, 0], [·, α/2 + sc/2, 0], [0, 0, 0]] (s = sin α,
    c = cos α). The arc bows toward the pipe's y_reference projected normal to the chord."""
    alpha, s, c, chord_unit, bow = trig
    R = Iv(radius)
    half = Iv(Fraction(1, 2))
    lumps = []
    for side in (0, 1):
        sign = Iv(-1) if side == 0 else ONE
        e1 = [sign * chord_unit[k] * s + bow[k] * c for k in range(3)]
        e2 = [-(sign * chord_unit[k] * c) + bow[k] * s for k in range(3)]
        e3 = [e1[1] * e2[2] - e1[2] * e2[1], e1[2] * e2[0] - e1[0] * e2[2], e1[0] * e2[1] - e1[1] * e2[0]]
        frame = (e1, e2, e3)
        mass = mbar * R * alpha
        r2 = R * R
        sc = s * c
        local_first = [mbar * r2 * (s - alpha), mbar * r2 * (ONE - c), ZERO]
        r3 = r2 * R
        jc = [[mbar * r3 * (alpha * half - sc * half), mbar * r3 * (ONE - c - s.square() * half), ZERO],
              [None, mbar * r3 * (Iv(Fraction(3, 2)) * alpha + sc * half - Iv(2) * s), ZERO],
              [None, None, mbar * r3 * Iv(2) * (alpha - s)]]
        tt = [[alpha * half - sc * half, -(s.square() * half), ZERO], [None, alpha * half + sc * half, ZERO], [None, None, ZERO]]
        local = [[None] * 3 for _ in range(3)]
        for a in range(3):
            for b in range(a, 3):
                section = R * (jbar * tt[a][b] + ibar * ((alpha if a == b else ZERO) - tt[a][b]))
                local[a][b] = jc[a][b] + section
                local[b][a] = local[a][b]
        for x in [mass, *local_first, *(local[p][q] for p in range(3) for q in range(3)),
                  *(tt[p][q] for p in range(3) for q in range(p, 3)),
                  *(jc[p][q] for p in range(3) for q in range(p, 3))]:
            if below_floor(x):
                floors.append("arc half-lump local entry")
        first = [frame[0][k] * local_first[0] + frame[1][k] * local_first[1] for k in range(3)]
        second = [[None] * 3 for _ in range(3)]
        for a in range(3):
            for b in range(a, 3):
                total = ZERO
                for p in range(3):
                    for q in range(3):
                        if local[p][q].lo == 0 and local[p][q].hi == 0:
                            continue
                        total = total + frame[p][a] * local[p][q] * frame[q][b]
                second[a][b] = total
        # The producer turns S and J to global axes as Σ S_k·e_k and Σ J_pq·e_p·e_qᵀ
        # (|e| ≤ 1): their absolute term sums bound each turned entry's scale.
        first_scale = sum((v.mag() for v in local_first), Fraction(0))
        second_scale = sum((local[p][q].mag() for p in range(3) for q in range(3)), Fraction(0))
        block = _rigid_block(mass, first, second)
        scales = {key: (mass.mag() if key[1] < 3 else first_scale if key[0] < 3 else second_scale) for key in block}
        lumps.append((block, scales))
    return lumps


def _arc_trig(xi, xj, radius, y, what):
    """α, sin α, cos α, the chord unit and the bow unit, enclosed (sin α = c/(2R))."""
    chord = [Iv(xj[k]) - Iv(xi[k]) for k in range(3)]
    chord_length = sqrt(chord[0].square() + chord[1].square() + chord[2].square())
    if not chord_length.excludes_zero():
        raise RefusedDomain(f"{what}: bend span chord length must be positive")
    s = chord_length / (Iv(2) * Iv(radius))
    if s.lo >= 1:
        raise RefusedDomain(f"{what}: the bend radius cannot span the chord")
    if s.hi >= 1:
        raise Uncovered(f"{what}: the bend radius spans the chord only within rounding; the half-angle is not enclosed")
    alpha, cosine = asin_and_cos(s)
    unit = [chord[k] / chord_length for k in range(3)]
    yy = [Iv(v) for v in y]
    axial = yy[0] * unit[0] + yy[1] * unit[1] + yy[2] * unit[2]
    normal = [yy[k] - axial * unit[k] for k in range(3)]
    size = sqrt(normal[0].square() + normal[1].square() + normal[2].square())
    if not size.excludes_zero():
        raise RefusedDomain(f"{what}: y_reference is parallel to the chord")
    bow = [normal[k] / size for k in range(3)]
    return alpha, s, cosine, unit, bow


# --- the model ---------------------------------------------------------------------------------

class ModelMass:
    """The model-derived mass basis of one bound request."""

    def __init__(self):
        self.route = None
        self.node_ids: list[str] = []
        self.coordinates: list[tuple[float, float, float]] = []
        self.restrained: set[int] = set()
        self.nonlinear: dict[str, tuple[str, int]] = {}
        self.state_case = None
        self.entries: dict[tuple[int, int], list] = {}
        self.categories = {"metal": ZERO, "contents": ZERO, "insulation": ZERO, "component_bodies": ZERO}
        self.bodies: list[str] = []
        # The component id of each body, by the member it replaces (for the subject
        # census's self-check only; no arithmetic reads it).
        self.body_components: dict[str, str] = {}
        self.arcs: list[str] = []
        self.straights: list[str] = []
        self.fallbacks: list[str] = []
        self.notes: list[str] = []

    # The published entry's envelope: the double-word bound, then the final rounding.
    def rounded(self, key) -> Iv:
        held = self.entries.get(key)
        if held is None:
            return ZERO
        if held[5]:
            raise OutsideModel(f"mass entry {key}: {held[5]}")
        if (held[3] + 1) * held[4] >= DD_RESOLUTION_LIMIT:
            raise OutsideModel(f"mass entry {key}: the frame conditioning puts the double-word premise at binary64 resolution")
        return binary64(held[1].widen(dd_bound(held[3], held[2])), f"mass entry {key}")

    def exact_entry(self, key) -> Iv:
        held = self.entries.get(key)
        return ZERO if held is None else held[0]


def work_units(request: Any, view: Any = None) -> int:
    """An input-derived bound on the reader's interval operations, stated before any
    formation: each distributed member's 12×12 (or two 6×6 lumps) with its section, each
    body, each document mass entry against its envelope, and per mode its m, three p_k
    and their quotients over the entries."""
    model = request.get("model") if isinstance(request, dict) else None
    model = model if isinstance(model, dict) else {}
    pipes = len(model.get("pipe_segments") or []) if isinstance(model.get("pipe_segments"), list) else 0
    components = len(model.get("components") or []) if isinstance(model.get("components"), list) else 0
    supports = len(model.get("supports") or []) if isinstance(model.get("supports"), list) else 0
    nodes = len(model.get("nodes") or []) if isinstance(model.get("nodes"), list) else 0
    entries = 144 * pipes + 21 * components
    modes = 0
    if view is not None:
        entries += len(view.mass or [])
        modes = len(view.modes)
    return 2_000 * pipes + 500 * components + 10 * supports + 36 * nodes + 8 * entries + modes * 8 * entries


def derive(request: Any) -> ModelMass:
    """The model-derived mass of a bound request (module docs). Raises `RefusedDomain`,
    `Uncovered`, `OutsideModel`, `Undecidable` or `ReaderSelfCheck`."""
    request = _obj(request, "request")
    model = _obj(request.get("model"), "request/model")
    out = ModelMass()
    if model.get("schema_version") != MODEL_VERSION:
        raise RefusedDomain(f"model schema_version: only {MODEL_VERSION} is read")
    # Route (D-r): the exact contract's presence, never inferred.
    exact_route = "pressure_contract" in model
    if exact_route:
        contract = _obj(model["pressure_contract"], "pressure_contract")
        if contract.get("version") != "3.0.0" or contract.get("mode") != "exact_pressure_v3":
            raise RefusedDomain("pressure_contract: only 3.0.0 exact_pressure_v3 is admitted")
        if contract.get("shear_model") == TIMOSHENKO:
            raise RefusedDomain("Timoshenko straights are refused by name (MODAL_TIMOSHENKO_MASS_UNSUPPORTED)")
        if contract.get("shear_model") != "euler_bernoulli":
            raise RefusedDomain("pressure_contract.shear_model is not a known beam theory")
    else:
        settings = model.get("model_settings")
        if isinstance(settings, dict) and settings.get("shear_model") == TIMOSHENKO:
            raise RefusedDomain("Timoshenko straights are refused by name (MODAL_TIMOSHENKO_MASS_UNSUPPORTED)")
    out.route = "exact_pressure" if exact_route else "pressure_free"

    # Nodes, in model order, in the project length unit.
    project = _obj(model.get("project"), "project")
    project_units = project.get("units")
    length_unit = project_units.get("length") if isinstance(project_units, dict) else None
    if not isinstance(length_unit, str):
        raise RefusedDomain("project.units.length: required for node coordinates")
    index: dict[str, int] = {}
    for i, node in enumerate(_list(model.get("nodes"), "nodes")):
        node = _obj(node, f"nodes/{i}")
        node_id = _text(node.get("id"), f"nodes/{i}/id")
        if node_id in index:
            raise RefusedDomain(f"node `{node_id}` is duplicated")
        position = _obj(node.get("position"), f"nodes/{i}/position")
        coordinates = tuple(_convert(position.get(axis), length_unit, "length", f"node {node_id} {axis}") for axis in "xyz")
        index[node_id] = i
        out.node_ids.append(node_id)
        out.coordinates.append(coordinates)

    # Pipes, with the shared-section binding resolved (inline OD/wall must be the
    # shared section's exact authored quantities).
    pipes: dict[str, dict] = {}
    order: list[str] = []
    shared = model.get("sections") if isinstance(model.get("sections"), list) else []
    for i, pipe in enumerate(_list(model.get("pipe_segments"), "pipe_segments")):
        pipe = _obj(pipe, f"pipe_segments/{i}")
        pipe_id = _text(pipe.get("id"), f"pipe_segments/{i}/id")
        if pipe_id in pipes:
            raise RefusedDomain(f"pipe `{pipe_id}` is duplicated")
        for end in ("from", "to"):
            if _text(pipe.get(end), f"pipe {pipe_id} {end}") not in index:
                raise RefusedDomain(f"pipe {pipe_id}: end node `{pipe.get(end)}` is not in the model")
        section = _obj(pipe.get("section"), f"pipe {pipe_id} section")
        reference = pipe.get("section_ref")
        if reference is not None:
            _bind_shared_section(pipe_id, section, reference, shared)
        pipes[pipe_id] = pipe
        order.append(pipe_id)

    components = _list(model.get("components", []), "components")
    seen_components = set()
    for i, component in enumerate(components):
        component = _obj(component, f"components/{i}")
        component_id = _text(component.get("id"), f"components/{i}/id")
        if component_id in seen_components:
            raise RefusedDomain(f"component `{component_id}` is duplicated")
        seen_components.add(component_id)
        _text(component.get("kind"), f"component {component_id} kind")

    # Objective connectors' replaced spans (exact v3 contract only) carry no mass.
    replaced: dict[str, str] = {}
    if exact_route:
        for component in components:
            if component.get("kind") != "expansion_joint" or component.get("objective_connector") is None:
                continue
            connector = _obj(component["objective_connector"], f"component {component['id']} objective_connector")
            topology = connector.get("topology")
            span = topology.get("span_ref") if isinstance(topology, dict) else None
            if not isinstance(span, str) or span not in pipes:
                raise RefusedDomain(f"objective connector {component['id']}: replaces_span names no pipe of the model")
            if span in replaced:
                raise RefusedDomain(f"objective connector {component['id']}: pipe {span} is already replaced")
            replaced[span] = component["id"]
    selected = [pipe_id for pipe_id in order if pipe_id not in replaced]

    # Realized arcs: a bend or elbow whose consumption (default geometry-only) is the
    # curved-bend macro-element, naming its span.
    curved: dict[str, dict] = {}
    for component in components:
        if component.get("kind") not in BEND_KINDS or _consumption(component, GEOMETRY_ONLY) != CURVED_BEND:
            continue
        geometry = component.get("geometry")
        pipe_ref = _nonblank(geometry.get("bend_pipe_ref")) if isinstance(geometry, dict) else None
        if pipe_ref is None:
            raise RefusedDomain(f"bend {component['id']}: the curved-bend macro-element needs geometry.bend_pipe_ref")
        if pipe_ref in curved:
            raise Uncovered(f"pipe {pipe_ref} is named by two curved-bend components")
        if pipe_ref in replaced:
            raise RefusedDomain(f"pipe {pipe_ref} is a realized curved span and a connector's replaced span")
        curved[pipe_ref] = component

    bodies = _component_bodies(model, components, pipes, selected, curved, replaced, request, out)
    acceleration = None
    if "weight_to_mass_acceleration" in request:
        acceleration = _quantity(request["weight_to_mass_acceleration"], "acceleration", "weight_to_mass_acceleration")
        if not acceleration > 0.0:
            raise RefusedDomain("weight_to_mass_acceleration must be a finite positive acceleration")
    if bodies and acceleration is None:
        raise RefusedDomain("component bodies enter the mass, but the request names no weight_to_mass_acceleration")
    if bodies:
        _applied_acceleration_agreement(model, acceleration)

    accumulator = Accumulator()
    for pipe_id in selected:
        pipe = pipes[pipe_id]
        if pipe_id in bodies:
            continue
        if not isinstance(pipe.get("y_reference"), dict):
            raise RefusedDomain(f"pipe {pipe_id}: it has no y_reference")
        y = tuple(_finite(pipe["y_reference"].get(axis), f"pipe {pipe_id} y_reference {axis}") for axis in "xyz")
        section = _section(pipe_id, pipe["section"], exact_route, "pipe")
        i, j = index[pipe["from"]], index[pipe["to"]]
        xi, xj = out.coordinates[i], out.coordinates[j]
        if xi == xj:
            raise RefusedDomain(f"pipe {pipe_id}: a member has zero length")
        conditioning = _frame_conditioning(xi, xj, y, f"pipe {pipe_id}")
        if pipe_id in curved:
            radius = _arc_admission(curved[pipe_id], pipe_id, pipes, xi, xj, y)
            geometry = _arc_trig(xi, xj, radius, y, f"bend {curved[pipe_id]['id']}")
            pair, floors = [], []
            for qs in ((section.mass[0], section.bending[0], section.polar[0]), (section.mass[1], section.bending[1], section.polar[1])):
                pair.append(_arc_lumps(xi, xj, radius, y, *qs, geometry, floors))
            outside = _floor_reason(floors, f"pipe {pipe_id}")
            for side, node in ((0, i), (1, j)):
                block, _ = pair[0][side]
                envelope, scales = pair[1][side]
                for (r, c), value in block.items():
                    accumulator.add(6 * node + r, 6 * node + c, value, envelope[(r, c)], conditioning, scales[(r, c)],
                                    outside)
            arc_length = Iv(2) * Iv(radius) * geometry[0]
            out.arcs.append(pipe_id)
        else:
            geometry = _straight_geometry(xi, xj)
            floors = []
            blocks = [_straight_blocks(xi, xj, section.mass[k], section.bending[k], section.polar[k], geometry, floors)
                      for k in (0, 1)]
            outside = _floor_reason(floors, f"pipe {pipe_id}")
            nodes = (i, j)
            for (r, c), value in blocks[0][0].items():
                accumulator.add(6 * nodes[r // 6] + r % 6, 6 * nodes[c // 6] + c % 6, value, blocks[1][0][(r, c)],
                                conditioning, blocks[1][1][(r, c)], outside)
            arc_length = geometry[0]
            out.straights.append(pipe_id)
        for name, pair in (("metal", section.metal), ("contents", section.contents), ("insulation", section.insulation)):
            out.categories[name] = out.categories[name] + pair[0] * arc_length
    for pipe_id, body in bodies.items():
        node, weight, cog = body
        _body_range(weight, acceleration, cog, pipe_id)
        mass_x = Iv(weight) / Iv(acceleration)            # W/a
        mass_e = binary64(mass_x, f"component body on {pipe_id}")  # the producer's fl(W/a)
        floors = []
        exact_block, envelope_block = (_body_block(m, cog, floors) for m in (mass_x, mass_e))
        outside = _floor_reason(floors, f"the component body on {pipe_id}")
        for (r, c), value in exact_block.items():
            accumulator.add(6 * node + r, 6 * node + c, value, envelope_block[(r, c)], outside=outside)
        out.categories["component_bodies"] = out.categories["component_bodies"] + mass_x
        out.bodies.append(pipe_id)
    out.entries = accumulator.entries
    # The envelope must hold the Design value entry by entry (a reader self-check).
    for key, (x, e, *_) in out.entries.items():
        if not e.contains_interval(x):
            raise ReaderSelfCheck(f"mass entry {key}: the envelope does not hold the Design value")

    _partition(model, index, out)
    _state(request, model, index, out)
    return out


def _bind_shared_section(pipe_id, section, reference, shared):
    if not isinstance(reference, str) or not reference.strip():
        raise RefusedDomain(f"pipe {pipe_id}: a bound section requires an explicit nonblank identity")
    matches = [s for s in shared if isinstance(s, dict) and s.get("id") == reference]
    if len(matches) != 1:
        raise RefusedDomain(f"pipe {pipe_id}: bound section `{reference}` must exist exactly once")
    record = matches[0]
    properties = record.get("properties")
    if record.get("section_type") != "pipe" or not isinstance(properties, dict) or any(
            key not in ("outside_diameter", "wall_thickness") for key in properties):
        raise RefusedDomain(f"pipe {pipe_id}: binding requires a pipe section with only outside_diameter and wall_thickness")
    for key in ("outside_diameter", "wall_thickness"):
        quantity = properties.get(key)
        _quantity(quantity, "length", f"section {reference} {key}")
        inline = section.get(key)
        if not isinstance(inline, dict) or inline.get("value") != quantity.get("value") or inline.get("unit") != quantity.get("unit") \
                or type(inline.get("value")) is bool:
            raise RefusedDomain(f"pipe {pipe_id}: inline {key} contradicts shared section `{reference}` (SECTION_REFERENCE_CACHE_STALE)")


def _cog(component, what):
    """The entered centre of gravity in metres; None when it is not a finite length
    vector; a nonzero component that normalizes to zero is a range refusal."""
    geometry = component.get("geometry")
    vector = geometry.get("center_of_gravity") if isinstance(geometry, dict) else None
    if not isinstance(vector, dict):
        return None
    entered = [vector.get(axis) for axis in "xyz"]
    if any(type(v) not in (int, float) or not math.isfinite(v) for v in entered) or not isinstance(vector.get("unit"), str):
        return None
    try:
        r = [units.to_canonical(v, vector["unit"], "length") for v in entered]
    except ValueError:
        return None
    if any(e != 0 and n == 0.0 for e, n in zip(entered, r)):
        raise RefusedDomain(f"{what}: a nonzero centre-of-gravity component normalizes to zero")
    return tuple(r)


def _weight(component):
    geometry = component.get("geometry")
    q = geometry.get("weight") if isinstance(geometry, dict) else None
    if not isinstance(q, dict) or type(q.get("value")) not in (int, float) or not isinstance(q.get("unit"), str):
        return None
    try:
        w = units.to_canonical(q["value"], q["unit"], "force")
    except ValueError:
        return None
    return w if math.isfinite(w) and w > 0.0 else None


def _component_bodies(model, components, pipes, selected, curved, replaced, request, out):
    """Self-weight's eligibility (W-15 slice b), by its stated rules: a
    mapping conflict refuses; `not_solver_consumed` keeps the pipe mass silently; an
    unmet condition (consumption undeclared or not geometry-only, the member not ending
    at the node, no finite positive weight, no finite centre of gravity, in that order)
    keeps the pipe mass with its disclosure; otherwise the body W/a at its centre of
    gravity replaces the member's distributed mass."""
    rigid = [c for c in components if c.get("kind") in RIGID_FAMILY]
    by_member: dict[str, list[str]] = {}
    for component in rigid:
        geometry = component.get("geometry")
        pipe_ref = _nonblank(geometry.get("rigid_pipe_ref")) if isinstance(geometry, dict) else None
        if pipe_ref is None or pipe_ref not in pipes:
            continue
        by_member.setdefault(pipe_ref, []).append(component["id"])
        if pipe_ref in curved:
            raise RefusedDomain(f"component {component['id']} maps realized curved span {pipe_ref} (mapping conflict)")
        if pipe_ref in replaced:
            raise RefusedDomain(f"component {component['id']} maps replaced span {pipe_ref} (mapping conflict)")
    for member, ids in by_member.items():
        if len(ids) > 1:
            raise RefusedDomain(f"member {member} is mapped by more than one rigid-family component ({', '.join(ids)})")
    selected_set = set(selected)
    bodies: dict[str, tuple[int, float, tuple]] = {}
    node_index = {node_id: i for i, node_id in enumerate(out.node_ids)}
    for component in rigid:
        geometry = component.get("geometry")
        pipe_ref = _nonblank(geometry.get("rigid_pipe_ref")) if isinstance(geometry, dict) else None
        if pipe_ref is None or pipe_ref not in selected_set:
            continue
        consumption = _consumption(component)
        if consumption == NOT_SOLVER_CONSUMED:
            continue
        pipe = pipes[pipe_ref]
        weight = _weight(component)
        cog = _cog(component, f"component {component['id']}")
        node = component.get("node")
        if consumption is None:
            unmet = "it declares no solver_consumption"
        elif consumption != GEOMETRY_ONLY:
            unmet = f"its solver_consumption is {consumption}"
        elif pipe.get("from") != node and pipe.get("to") != node:
            unmet = f"its mapped member {pipe_ref} does not end at its node"
        elif weight is None:
            unmet = "its weight is missing, not a force, or not finite and positive"
        elif cog is None:
            unmet = "its centre of gravity is missing, not a length vector, or not finite"
        else:
            unmet = None
        if unmet is not None:
            out.fallbacks.append(f"component {component['id']} keeps member {pipe_ref}'s pipe mass: {unmet}")
            continue
        # The wrench's moment range rule (gravity along +z): a nonzero lever whose
        # product with W is zero or not finite is refused, never dropped.
        for lever in (cog[1], cog[0]):
            product = lever * weight
            if lever != 0.0 and (product == 0.0 or not math.isfinite(product)):
                raise RefusedDomain(f"component {component['id']}: its weight moment is outside the representable range")
        if node not in node_index:
            raise RefusedDomain(f"component {component['id']} names no node")
        bodies[pipe_ref] = (node_index[node], weight, cog)
        out.body_components[pipe_ref] = component["id"]
    return bodies


def _body_range(weight, a, cog, pipe_id):
    """four-part binary64 premise, in binary64 and in the order the
    producer evaluates it (`component_range`): first = m·r_i, then first·r_j, each
    zero or normal, and a nonzero exact product that rounds to zero is out of range;
    then the rotational diagonal's sums are finite. The producer refuses a body
    outside it (MODAL_COMPONENT_MASS_OUT_OF_RANGE)."""
    def normal(x):
        return math.isfinite(x) and abs(x) >= 2.0 ** -1022

    def ok(x):
        return x == 0.0 or normal(x)

    refused = RefusedDomain(f"body on {pipe_id}: it leaves the binary64 range premise (MODAL_COMPONENT_MASS_OUT_OF_RANGE)")
    if not (normal(weight) and weight > 0 and normal(a) and a > 0):
        raise refused
    m = weight / a
    if not normal(m) or not normal(a * m) or not all(math.isfinite(r) for r in cog):
        raise refused
    for i in range(3):
        first = m * cog[i]
        if not ok(first) or (first == 0.0 and cog[i] != 0.0):
            raise refused
        for j in range(3):
            second = first * cog[j]
            if not ok(second) or (second == 0.0 and first != 0.0 and cog[j] != 0.0):
                raise refused
    for i in range(3):
        j, k = (i + 1) % 3, (i + 2) % 3
        if not math.isfinite(m * cog[j] * cog[j] + m * cog[k] * cog[k]):
            raise refused


def _applied_acceleration_agreement(model, acceleration):
    """an applied component-weight primitive's recorded acceleration
    must equal a, unless that primitive is stale or overridden. Staleness is not decided
    here, so only a disagreeing record is scoped."""
    for case in model.get("load_cases") or []:
        if not isinstance(case, dict):
            continue
        for load in case.get("primitive_loads") or []:
            provenance = load.get("provenance") if isinstance(load, dict) else None
            if not isinstance(provenance, str) or COMPONENT_WEIGHT_METHOD not in provenance:
                continue
            try:
                evidence = json.loads(provenance)
            except ValueError:
                continue
            if not isinstance(evidence, dict) or evidence.get("method") != COMPONENT_WEIGHT_METHOD:
                continue
            recorded = evidence.get("normalized_acceleration_m_per_s2")
            if type(recorded) in (int, float) and math.isfinite(recorded) and abs(float(recorded)) == acceleration:
                continue
            raise Uncovered(f"applied component-weight primitive {load.get('id')} records an acceleration other than a; "
                            "whether it is current (and so refuses the request) is not decided here")


def _arc_admission(component, pipe_id, pipes, xi, xj, y) -> float:
    """The curved-bend admission rules the arc's mass depends on (DEC-070, by contract):
    the component node is an end, a positive flexibility factor and radius, a radius
    that spans the chord, a y_reference not parallel to it (its binary64 plane test)."""
    what = f"bend {component['id']}"
    pipe = pipes[pipe_id]
    if component.get("node") not in (pipe.get("from"), pipe.get("to")):
        raise RefusedDomain(f"{what}: component node must be an endpoint of the referenced bend span")
    modifiers = component.get("modifiers")
    factor = modifiers.get("flexibility_factor_user_value") if isinstance(modifiers, dict) else None
    if not isinstance(factor, dict) or type(factor.get("value")) not in (int, float) or not (
            math.isfinite(factor["value"]) and factor["value"] > 0):
        raise RefusedDomain(f"{what}: a finite positive flexibility factor is required")
    geometry = component.get("geometry")
    radius_q = geometry.get("bend_radius") if isinstance(geometry, dict) else None
    if radius_q is None:
        raise RefusedDomain(f"{what}: geometry.bend_radius is required")
    radius = _quantity(radius_q, "length", f"{what} bend_radius")
    if not radius > 0.0:
        raise RefusedDomain(f"{what}: the bend radius must be a finite positive length")
    chord = [xj[k] - xi[k] for k in range(3)]
    chord_length = math.sqrt(chord[0] * chord[0] + chord[1] * chord[1] + chord[2] * chord[2])
    if not (math.isfinite(chord_length) and chord_length > 0.0):
        raise RefusedDomain(f"{what}: bend span chord length must be positive")
    if radius <= 0.5 * chord_length:
        raise RefusedDomain(f"{what}: the bend radius cannot span the chord")
    unit = [c / chord_length for c in chord]
    axial = y[0] * unit[0] + y[1] * unit[1] + y[2] * unit[2]
    plane = [y[k] - axial * unit[k] for k in range(3)]
    if not math.sqrt(plane[0] * plane[0] + plane[1] * plane[1] + plane[2] * plane[2]) > CURVED_BEND_PLANE_TOLERANCE:
        raise RefusedDomain(f"{what}: y_reference is parallel to the bend span chord")
    return radius


# The axis of each DOF token the producer recognises (`dof_axis`), translations and rotations,
# and the (translation, axis) DOF it names.
_TOKEN_AXIS = {token: axis for axis, letter in enumerate("XYZ") for kind in "UR"
               for token in (kind + letter, kind + letter.lower(), kind.lower() + letter.lower())}
_TOKEN_DOF = {token: (token[0] in "Uu", axis) for token, axis in _TOKEN_AXIS.items()}


def _direction_axis(support, support_id, restraints):
    """The producer's admission of a `direction` record (`supports_findings`), then its
    axis: None when absent. Refused, as the producer refuses it before lowering: a
    direction that is not three finite numbers or is zero; on a family other than
    line_stop, spring or snubber; beside a nonlinear member; on a record whose
    restraints and stiffness do not name exactly one DOF, or name a rotational one.
    A skewed direction is admitted statically but refused by modal-1."""
    if "direction" not in support:  # a present null is refused, as `deserialize_direction` keeps it
        return None
    direction = support["direction"]

    def refused(reason):
        return RefusedDomain(f"support {support_id}: its direction {reason}, which the producer refuses")

    d = [direction.get(axis) for axis in "xyz"] if isinstance(direction, dict) else None
    if d is None or not all(type(v) in (int, float) and math.isfinite(v) for v in d):
        raise refused("is not three finite numbers (SUPPORT_DIRECTION_INVALID)")
    if all(v == 0 for v in d):
        raise refused("is zero (SUPPORT_DIRECTION_ZERO)")
    if support.get("family") not in ("line_stop", "spring", "snubber"):
        raise refused("is on a family other than line_stop, spring or snubber (SUPPORT_DIRECTION_RECORD_UNSUPPORTED)")
    if support.get("nonlinear") is not None:
        raise refused("is beside a nonlinear member (SUPPORT_DIRECTION_RECORD_UNSUPPORTED)")
    hanger = support.get("hanger")
    stiffness = support.get("stiffness") or (hanger.get("stiffness") if isinstance(hanger, dict) else None)
    tokens = list(restraints) + ([stiffness.get("dof")] if isinstance(stiffness, dict) else [])
    dofs = []
    for token in tokens:
        if token not in _TOKEN_DOF:
            raise refused("is on a record naming a token that is not a DOF (SUPPORT_DIRECTION_RECORD_UNSUPPORTED)")
        if _TOKEN_DOF[token] not in dofs:
            dofs.append(_TOKEN_DOF[token])
    if len(dofs) != 1:
        raise refused("is on a record acting on other than exactly one DOF (SUPPORT_DIRECTION_RECORD_UNSUPPORTED)")
    if not dofs[0][0]:
        raise refused("is on a rotational DOF (SUPPORT_DIRECTION_ROTATIONAL_UNSUPPORTED)")
    if sum(1 for v in d if v != 0) > 1:
        raise RefusedDomain(f"support {support_id}: modal-1 refuses skew records (MODAL_DOCUMENT_SKEW_RESTRAINT_UNSUPPORTED)")
    return next(k for k in range(3) if d[k] != 0)


def _partition(model, index, out):
    """The axis restraint partition: a rigid support restrains its listed
    DOFs; a spring or (variable) spring hanger leaves its DOF free; a constant-effort
    support is absent; a nonlinear support with no restraints and no stiffness enters
    only through the state case. A skewed record (modal-1 refuses skew before
    projection) and a snubber (refused on every static solve this source has) are
    refused domains."""
    for i, support in enumerate(_list(model.get("supports"), "supports")):
        support = _obj(support, f"supports/{i}")
        support_id = _text(support.get("id"), f"supports/{i}/id")
        family = support.get("family")
        if family == "snubber":
            raise RefusedDomain(f"support {support_id}: a snubber is refused on this source (SNUBBER_DYNAMIC_ONLY_UNSUPPORTED)")
        restraints_authored = _list(support.get("restraints"), f"support {support_id} restraints")
        axis = _direction_axis(support, support_id, restraints_authored)
        if axis is not None:
            # An admitted, exactly axis-aligned direction is lowered as the producer does
            # (`lower_axis_aligned_directions`): each DOF token on another axis becomes
            # the direction's translation token.
            restraints_authored = ["U" + "XYZ"[axis] if _TOKEN_AXIS.get(t, axis) != axis else t
                                   for t in restraints_authored]
        hanger = support.get("hanger")
        hanger_type = hanger.get("hanger_type") if isinstance(hanger, dict) else None
        hanger_type = hanger_type.strip() if isinstance(hanger_type, str) and hanger_type.strip() else (
            family.strip() if isinstance(family, str) else None)
        stiffness = support.get("stiffness") or (hanger.get("stiffness") if isinstance(hanger, dict) else None)
        nonlinear = support.get("nonlinear")
        restraints = restraints_authored
        if nonlinear is not None and not restraints and stiffness is None:
            continue
        if hanger_type == "constant_effort_support":
            continue
        node = support.get("node")
        if node not in index:
            raise RefusedDomain(f"support {support_id}: its node is not in the model")
        if family == "spring" or hanger_type in ("variable_spring_hanger", "spring_hanger"):
            continue
        for token in restraints:
            out.restrained.add(6 * index[node] + _dof(token, f"support {support_id} restraint"))
    for i, support in enumerate(model.get("supports")):
        nonlinear = support.get("nonlinear")
        if nonlinear is None:
            continue
        nonlinear = _obj(nonlinear, f"support {support['id']} nonlinear")
        behavior = BEHAVIORS.get(nonlinear.get("behavior"))
        if behavior is None:
            raise RefusedDomain(f"support {support['id']}: unknown nonlinear behaviour")
        if support.get("node") not in index:
            raise RefusedDomain(f"support {support['id']}: its node is not in the model")
        out.nonlinear[support["id"]] = (behavior, 6 * index[support["node"]] + _dof(nonlinear.get("dof"), f"support {support['id']} dof"))


def _state(request, model, index, out):
    """The named state case (D-s, D-w): condition contents on it are refused, never
    replaced by section contents; a gap support is refused on this source (no static
    gap qualification, `modal/state.rs`). Nonlinear supports on the exact route are
    solved statically since RT-A3 S6 and enter like any other reported state."""
    case_id = request.get("state_case")
    if out.nonlinear:
        if any(behavior == "gap" for behavior, _ in out.nonlinear.values()):
            raise RefusedDomain("a model with a gap support is refused until static gap qualification lands (RT-A3 R-3)")
        if case_id is None:
            raise RefusedDomain("a model with a nonlinear support requires a state case (MODAL_STATE_CASE_REQUIRED)")
    if case_id is None:
        return
    cases = [c for c in (model.get("load_cases") or []) if isinstance(c, dict) and c.get("id") == case_id]
    if len(cases) != 1:
        raise RefusedDomain(f"state case `{case_id}` does not name exactly one load case")
    use = cases[0].get("condition_use")
    if isinstance(use, dict) and use.get("contents_density") is not None:
        raise RefusedDomain(f"state case `{case_id}` uses condition contents (LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED); "
                            "they are refused, never replaced by section contents")
    out.state_case = case_id


# --- the subject census ---------------------------------------------------------

class Census:
    """The subjects a bound request's model gives a run, each list in model order: the
    straight members and arcs by pipe id, the component bodies by component id, every
    support record by id, and the nonlinear supports (those the supports table lists)."""

    def __init__(self):
        self.straights: list[str] = []
        self.arcs: list[str] = []
        self.bodies: list[str] = []
        self.supports: list[str] = []
        self.nonlinear: list[str] = []

    def subjects(self) -> list[dict]:
        """The census list: `{kind, id}` sorted by (kind, id) in code-point order."""
        pairs = ([("straight_member", i) for i in self.straights] + [("arc", i) for i in self.arcs]
                 + [("component_body", i) for i in self.bodies] + [("support", i) for i in self.supports])
        return [{"kind": kind, "id": subject} for kind, subject in sorted(pairs)]


def _cog_entered(component) -> bool:
    """An entered centre of gravity: a finite length vector. One whose nonzero component
    normalizes to zero is entered (its range refusal is the mass relation's)."""
    try:
        return _cog(component, f"component {component['id']}") is not None
    except RefusedDomain:
        return True


def census(request: Any) -> Census:
    """The subject rule of Design §5.3.2, from the bound request's model alone and from
    its structure only (no quantity is enclosed, so no subject is ever dropped because
    a mass value is unresolved or outside the rounding model):

    1. the members are the `pipe_segments` (ids unique) less the spans the exact
       contract's objective connectors replace;
    2. a member whose distributed mass a rigid-family body replaces (self-weight's
       eligibility: the mapping, `mechanics_geometry_only`, the node at an end of the
       span, a finite positive weight and an entered centre of gravity) is the
       `component_body` of that component id, and is not listed as a member;
    3. any other member is an `arc` when a curved-bend component realizes it, else a
       `straight_member`;
    4. every `model.supports` record is a `support`; those with a present, non-null
       `nonlinear` member are the nonlinear supports.

    Raises `RefusedDomain` or `Uncovered` for a structure it cannot decide."""
    request = _obj(request, "request")
    model = _obj(request.get("model"), "request/model")
    out = Census()
    if model.get("schema_version") != MODEL_VERSION:
        raise RefusedDomain(f"model schema_version: only {MODEL_VERSION} is read")
    exact_route = "pressure_contract" in model
    if exact_route:
        contract = _obj(model["pressure_contract"], "pressure_contract")
        if contract.get("version") != "3.0.0" or contract.get("mode") != "exact_pressure_v3":
            raise RefusedDomain("pressure_contract: only 3.0.0 exact_pressure_v3 is admitted")
    pipes: dict[str, dict] = {}
    for i, pipe in enumerate(_list(model.get("pipe_segments"), "pipe_segments")):
        pipe = _obj(pipe, f"pipe_segments/{i}")
        pipe_id = _text(pipe.get("id"), f"pipe_segments/{i}/id")
        if pipe_id in pipes:
            raise RefusedDomain(f"pipe `{pipe_id}` is duplicated")
        pipes[pipe_id] = pipe
    components = _list(model.get("components", []), "components")
    seen = set()
    for i, component in enumerate(components):
        component = _obj(component, f"components/{i}")
        component_id = _text(component.get("id"), f"components/{i}/id")
        if component_id in seen:
            raise RefusedDomain(f"component `{component_id}` is duplicated")
        seen.add(component_id)

    # 1. The exact contract's replaced spans are no members.
    replaced: set[str] = set()
    if exact_route:
        for component in components:
            if component.get("kind") != "expansion_joint" or component.get("objective_connector") is None:
                continue
            connector = _obj(component["objective_connector"], f"component {component['id']} objective_connector")
            topology = connector.get("topology")
            span = topology.get("span_ref") if isinstance(topology, dict) else None
            if not isinstance(span, str) or span not in pipes:
                raise RefusedDomain(f"objective connector {component['id']}: replaces_span names no pipe of the model")
            if span in replaced:
                raise RefusedDomain(f"objective connector {component['id']}: pipe {span} is already replaced")
            replaced.add(span)
    members = [pipe_id for pipe_id in pipes if pipe_id not in replaced]

    # 3. The realized arcs.
    curved: set[str] = set()
    for component in components:
        if component.get("kind") not in BEND_KINDS or _consumption(component, GEOMETRY_ONLY) != CURVED_BEND:
            continue
        geometry = component.get("geometry")
        pipe_ref = _nonblank(geometry.get("bend_pipe_ref")) if isinstance(geometry, dict) else None
        if pipe_ref is None:
            raise RefusedDomain(f"bend {component['id']}: the curved-bend macro-element needs geometry.bend_pipe_ref")
        if pipe_ref in curved:
            raise Uncovered(f"pipe {pipe_ref} is named by two curved-bend components")
        if pipe_ref in replaced:
            raise RefusedDomain(f"pipe {pipe_ref} is a realized curved span and a connector's replaced span")
        curved.add(pipe_ref)

    # 2. The component bodies, by self-weight's eligibility.
    rigid = [c for c in components if c.get("kind") in RIGID_FAMILY]
    mapped: dict[str, list[str]] = {}
    for component in rigid:
        geometry = component.get("geometry")
        pipe_ref = _nonblank(geometry.get("rigid_pipe_ref")) if isinstance(geometry, dict) else None
        if pipe_ref is None or pipe_ref not in pipes:
            continue
        mapped.setdefault(pipe_ref, []).append(component["id"])
        if pipe_ref in curved or pipe_ref in replaced:
            raise RefusedDomain(f"component {component['id']} maps {pipe_ref}, a realized curved or replaced span (mapping conflict)")
    for member, ids in mapped.items():
        if len(ids) > 1:
            raise RefusedDomain(f"member {member} is mapped by more than one rigid-family component ({', '.join(ids)})")
    body_of: dict[str, str] = {}
    for component in rigid:
        pipe_ref = _nonblank(component["geometry"].get("rigid_pipe_ref")) if isinstance(component.get("geometry"), dict) else None
        if pipe_ref is None or pipe_ref not in pipes or pipe_ref in replaced:
            continue
        pipe = pipes[pipe_ref]
        node = component.get("node")
        if (_consumption(component) == GEOMETRY_ONLY and node in (pipe.get("from"), pipe.get("to"))
                and _weight(component) is not None and _cog_entered(component)):
            body_of[pipe_ref] = component["id"]
    for pipe_id in members:
        if pipe_id in body_of:
            out.bodies.append(body_of[pipe_id])
        elif pipe_id in curved:
            out.arcs.append(pipe_id)
        else:
            out.straights.append(pipe_id)

    # 4. Every support record; the nonlinear ones are the supports table's.
    for i, support in enumerate(_list(model.get("supports"), "supports")):
        support = _obj(support, f"supports/{i}")
        support_id = _text(support.get("id"), f"supports/{i}/id")
        out.supports.append(support_id)
        if support.get("nonlinear") is not None:
            out.nonlinear.append(support_id)
    return out


def census_self_check(model: ModelMass, subjects: Census) -> None:
    """The mass pass's formed members and bodies against the census pass's (a reader
    self-check: a disagreement is this reader's defect, never a document verdict)."""
    formed = (sorted(model.straights), sorted(model.arcs), sorted(model.body_components[p] for p in model.bodies))
    listed = (sorted(subjects.straights), sorted(subjects.arcs), sorted(subjects.bodies))
    for kind, a, b in zip(("straight members", "arcs", "component bodies"), formed, listed):
        if a != b:
            raise ReaderSelfCheck(f"the mass pass formed {kind} {a}, the subject census lists {b}")


# --- comparison --------------------------------------------------------------------------------

def _judge(what, vo, expected: Iv, signed=False):
    """A published value object against its envelope. Returns None when consistent at a
    published point, an unresolved text when its published set meets the envelope but
    is not a point, and raises `Contradiction` when no published value can lie in it."""
    if vo.value is not None and not vo.below:
        if expected.contains(vo.value):
            return None
        raise Contradiction(f"{what}: published {vo.value!r} is outside the model-derived envelope "
                            f"[{float(expected.lo)!r}, {float(expected.hi)!r}]")
    if vo.below:
        # A flagged value is a nonzero real of magnitude below 2^-1022 (open interval).
        meets = expected.lo < NORMAL_MIN and expected.hi > 0
        if signed:
            meets = meets or (expected.hi > -NORMAL_MIN and expected.lo < 0)
        if not meets:
            raise Contradiction(f"{what}: flagged below the normal range, but the model-derived value is "
                                f"[{float(expected.lo)!r}, {float(expected.hi)!r}]")
        return UNRESOLVED + f"{what}: flagged below the normal range; undecidable against the model on the published bounds"
    if vo.undefined == "outside_canonical_profile":
        big = Iv(PROFILE_FLOOR, LARGEST)
        if not (expected.meets(big) or (signed and expected.meets(-big))):
            raise Contradiction(f"{what}: published outside the checked profile, but the model-derived value is "
                                f"[{float(expected.lo)!r}, {float(expected.hi)!r}]")
        return UNRESOLVED + f"{what}: outside the checked profile; undecidable against the model on the published bounds"
    if vo.undefined == "not_finite":
        if expected.hi < LARGEST and expected.lo > -LARGEST:
            raise Contradiction(f"{what}: published not finite, but the model-derived value is finite")
        return UNRESOLVED + f"{what}: not finite; undecidable against the model"
    return None  # a claim-null: the payload relation that consumes it decides it


def _guarded(items: list, what: str, check) -> None:
    """One comparison: a contradiction propagates; a value outside the rounding model or
    an undefined interval operation leaves this comparison unresolved, and the others
    still run."""
    try:
        text = check()
    except (OutsideModel, Undecidable) as error:
        items.append(UNRESOLVED + f"{what}: {error}")
        return
    if text:
        items.append(text)


def _free_list(view):
    return [f is not None for f in view.free]


def _bind_nodes(model: ModelMass, view):
    if list(view.node_ids) != model.node_ids:
        raise Contradiction("nodes: the document's node order is not the bound model's")
    for n, (published, derived) in enumerate(zip(view.nodes, model.coordinates)):
        if any(p is None or p != d or math.copysign(1.0, p) != math.copysign(1.0, d) for p, d in zip(published, derived)):
            raise Contradiction(f"node {model.node_ids[n]}: its coordinates are not the bound model's in metres")
    if view.route != model.route:
        raise Contradiction(f"basis route {view.route} is not the bound model's {model.route}")


def _state_rigid(behavior, state):
    """Design §1.3's table: True rigid, False free, None not admitted."""
    if behavior == "friction":
        return {"sticking": True, "sliding": False, "inactive": False}.get(state)
    return {"active": True, "inactive": False}.get(state)


def _bind_partition(model: ModelMass, view):
    """The document's restrained DOFs against the model's partition: the axis
    restraints, augmented by the reported rigid states of a linearized state case."""
    restrained_doc = {dof for dof, f in enumerate(view.free) if f is None}
    items = []
    if model.nonlinear:
        if view.linearized_restrained is None:
            raise Contradiction("supports: the model has nonlinear supports, but no state case is linearized")
        rows = {row[0]: row for row in view.support_rows}
        if set(rows) != set(model.nonlinear) or len(rows) != len(view.support_rows):
            raise Contradiction("supports: the reported supports are not the model's nonlinear supports, each once")
        augmented = set(model.restrained)
        for support_id, (behavior, dof) in model.nonlinear.items():
            _, row_behavior, row_dof, state, treatment = rows[support_id]
            if row_behavior != behavior or row_dof != dof:
                raise Contradiction(f"supports: {support_id} is not the model's {behavior} support on DOF {dof}")
            rigid = _state_rigid(behavior, state)
            if rigid is None:
                raise Contradiction(f"supports: {support_id} ({behavior}) state {state} is not in the modal state table")
            if rigid:
                expected = "already_restrained" if dof in model.restrained else "restrained"
                augmented.add(dof)
            else:
                expected = "free_frozen_friction" if behavior == "friction" and state == "sliding" else "free"
            if treatment != expected:
                raise Contradiction(f"supports: {support_id} in state {state} is treated {treatment}, not {expected}")
        if restrained_doc != augmented:
            raise Contradiction("partition: the restrained DOFs are not the model's axis restraints with the reported rigid states")
        items.append("partition: the free DOFs follow the REPORTED nonlinear support states; that they are the converged, "
                     "confirmed states of the named case's static solve is not established here, so the free-DOF masses, "
                     "m_i and p_k are conditional on that qualification")
    else:
        if view.support_rows:
            raise Contradiction("supports: support states are reported, but the model has no nonlinear support")
        if restrained_doc != model.restrained:
            raise Contradiction("partition: the restrained DOFs are not the model's axis restraints")
    return items


def _rounded(model: ModelMass):
    """Every derived upper entry's published envelope; a value outside the rounding
    model leaves that entry (and what uses it) unresolved."""
    out, outside = {}, []
    for key in model.entries:
        try:
            out[key] = model.rounded(key)
        except OutsideModel as error:
            outside.append((key, str(error)))
    return out, outside


def _totals(rounded, free=None):
    """T_k (or the free-DOF mass) over the rounded-entry envelopes: the producer's
    double-word sum (its terms here are entries counted 1 or 2 times), rounded once."""
    out = []
    for k in range(3):
        total, magnitude, count = ZERO, Fraction(0), 0
        for (a, b), value in rounded.items():
            if a % 6 != k or b % 6 != k or (free is not None and not (free[a] and free[b])):
                continue
            term = value if a == b else value * Iv(2)
            total = total + term
            magnitude += term.mag()
            count += 1
        out.append(binary64(total.widen(dd_bound(count, magnitude)), "a directional total"))
    return out


def _full_row_sums(rounded, n_dofs: int):
    """(M R_k) over all DOFs for k = x, y, z, each row widened by its double-word
    accumulation allowance (`dd_bound` over the row's term count and absolute term
    sum), so every use of a row covers the producer's row error."""
    rows = [[ZERO] * n_dofs for _ in range(3)]
    mags = [[Fraction(0)] * n_dofs for _ in range(3)]
    counts = [[0] * n_dofs for _ in range(3)]
    for (a, b), value in rounded.items():
        if b % 6 < 3:
            k = b % 6
            rows[k][a] = rows[k][a] + value
            mags[k][a] += value.mag()
            counts[k][a] += 1
        if a != b and a % 6 < 3:
            k = a % 6
            rows[k][b] = rows[k][b] + value
            mags[k][b] += value.mag()
            counts[k][b] += 1
    rows = [[rows[k][a].widen(dd_bound(counts[k][a], mags[k][a])) for a in range(n_dofs)] for k in range(3)]
    return rows, mags


def mass_items(model: ModelMass, view) -> list[str]:
    """The model-derived mass relation: node, route and partition binding; every upper
    entry over the union of derived and published entries; T_k; the free-DOF masses;
    the mass centre. Raises `Contradiction`."""
    _bind_nodes(model, view)
    items = _bind_partition(model, view)
    n_dofs = 6 * len(model.node_ids)
    rounded, outside = _rounded(model)
    for key, error in outside:
        items.append(UNRESOLVED + f"mass entry {key}: {error}")
    if view.mass is None:
        items.append("mass_matrix withheld: the formed entries are not compared with the model's")
    else:
        published = {(a, b): vo for a, b, vo in view.mass}
        for key in sorted(set(published) | set(model.entries)):
            if key not in rounded and key in model.entries:
                continue  # outside the rounding model: already unresolved
            expected = rounded.get(key, ZERO)
            vo = published.get(key)
            what = f"mass entry ({view.dof_ids[key[0]]}, {view.dof_ids[key[1]]})"
            if vo is None:
                if not expected.contains(0):
                    raise Contradiction(f"{what}: the model forms [{float(expected.lo)!r}, {float(expected.hi)!r}], "
                                        "but the document publishes no entry (0)")
                continue
            _guarded(items, what, lambda: _judge(what, vo, expected, signed=key[0] != key[1]))
    free = _free_list(view)
    open_keys = [key for key, _ in outside]
    for name, derived, published, scope in (("T", _totals(rounded), view.totals, None),
                                            ("free-DOF mass", _totals(rounded, free), view.free_totals, free)):
        for k in range(3):
            what = f"{name} {DIRECTIONS[k]} against the model"
            if any(a % 6 == k and b % 6 == k and (scope is None or (scope[a] and scope[b])) for a, b in open_keys):
                items.append(UNRESOLVED + f"{what}: an entry it sums is outside the rounding model")
                continue
            _guarded(items, what, lambda: _judge(what, published[k], derived[k]))
    if any(a % 6 < 3 or b % 6 < 3 for a, b in open_keys):
        items.append(UNRESOLVED + "the mass centre: an entry of a translational column is outside the rounding model")
    else:
        _guarded(items, "the mass centre", lambda: _centre(model, view, rounded, n_dofs, items))
    # The category totals are an internal cross-check of this reader: in real arithmetic
    # T_k = metal + contents + insulation + bodies, whatever the direction.
    categories = model.categories["metal"] + model.categories["contents"] + model.categories["insulation"] + \
        model.categories["component_bodies"]
    for k in range(3):
        exact_total = ZERO
        for (a, b), held in model.entries.items():
            if a % 6 == k and b % 6 == k:
                exact_total = exact_total + (held[0] if a == b else held[0] * Iv(2))
        if not exact_total.meets(categories):
            raise ReaderSelfCheck(f"T_{DIRECTIONS[k]}: the derived entries and the category totals disagree")
    return items


def _centre(model: ModelMass, view, rounded, n_dofs, items):
    """The mass centre from M's first moments: with F = M R_k about a point
    p, the resultant moment is T_k·(c − p) × e_k, so c_x and c_y come from R_z and c_z
    from R_x. The producer takes p̃ = the node centroid (double word, rounded), divides
    the double-word moments by the binary64 totals and publishes fl(p̃ + fl(d))."""
    totals = _totals(rounded)
    published = view.mass_center
    positive = all(t.lo > 0 for t in totals)
    maybe_positive = all(t.hi > 0 for t in totals)
    if published is None:
        if positive:
            raise Contradiction("mass centre: not published, but the model carries translational mass in every direction")
        return None
    if not maybe_positive:
        raise Contradiction("mass centre: published, but the model carries no translational mass in some direction")
    if not positive:
        return UNRESOLVED + "mass centre: a total may be zero within rounding"
    rows, _ = _full_row_sums(rounded, n_dofs)
    count = len(model.node_ids)
    mean = [Iv(sum((exact(c[k]) for c in model.coordinates), Fraction(0)) / count) for k in range(3)]
    spread = [sum((abs(exact(c[k])) for c in model.coordinates), Fraction(0)) / count for k in range(3)]
    point = [binary64(relative(mean[k].widen(dd_bound(count, spread[k]) + DD_ABSOLUTE), DD_RELATIVE)) for k in range(3)]

    def moment(k):
        out, magnitude = [ZERO, ZERO, ZERO], [Fraction(0)] * 3
        for n in range(count):
            d = [Iv(model.coordinates[n][a]) - point[a] for a in range(3)]
            f = [rows[k][6 * n + a] for a in range(3)]
            terms = ((d[1] * f[2], -(d[2] * f[1])), (d[2] * f[0], -(d[0] * f[2])), (d[0] * f[1], -(d[1] * f[0])))
            for a in range(3):
                rotational = rows[k][6 * n + 3 + a]
                out[a] = out[a] + terms[a][0] + terms[a][1] + rotational
                magnitude[a] += terms[a][0].mag() + terms[a][1].mag() + rotational.mag()
        return out, magnitude

    (mz, mz_mag), (mx, mx_mag) = moment(2), moment(0)
    offsets = ((-mz[1], mz_mag[1], totals[2]), (mz[0], mz_mag[0], totals[2]), (mx[1], mx_mag[1], totals[0]))
    for k, (numerator, magnitude, total) in enumerate(offsets):
        # Double-word error of the moment (against its absolute terms) and of the quotient,
        # each with its underflow term: two products per node and component, one division.
        quotient = numerator.widen(dd_bound(3 * count, magnitude) + 2 * count * DD_ABSOLUTE) / total
        d = binary64(relative(quotient.widen(DD_ABSOLUTE * max(Fraction(1), 1 / total.lo)), DD_RELATIVE))
        expected = binary64(point[k] + d)
        what = f"mass centre {DIRECTIONS[k]}"
        _guarded(items, what, lambda: _judge(what, published[k], expected, signed=True))
    return None


def participation_items(model: ModelMass, view) -> list[str]:
    """m_i, Γ, m_eff, the fractions and their cumulative sums from the model-derived M
    and the published shapes (D-v): m = φᵀM_ffφ summed exactly and published
    once; p_k = (M R_k)_f, each row a double-word sum rounded to binary64, so the
    restrained-point coupling M_fc stays in it; φᵀp_k summed exactly; Γ = φᵀp_k/m,
    m_eff = (φᵀp_k)²/m and the fraction (φᵀp_k)²/(m·T_k) in double word, each published
    once; the cumulative sums a double-word accumulation of the published fractions,
    rounded once."""
    items: list[str] = []
    # The quantities are only meaningful on the model's own node basis and partition; when
    # the document contradicts either, `mass` fails and participation is not recomputed.
    try:
        _bind_nodes(model, view)
        _bind_partition(model, view)
    except Contradiction:
        return ["model-derived participation not checked: the node basis or restraint partition is contradicted (mass)"]
    if model.nonlinear:
        items.append("p_k and m_i use the free DOFs of the REPORTED nonlinear support states, so they are conditional "
                     "on those states' qualification, which this reader does not establish")
    if view.shapes is None:
        return items + ["shapes withheld: m_i, Γ, m_eff and the fractions are not recomputed from the model"]
    rounded, outside = _rounded(model)
    free = _free_list(view)
    # An entry outside the rounding model leaves a mode's quantity undecidable only where
    # that quantity uses it (below); every other quantity is still compared.
    open_keys = [key for key, _ in outside]
    n_dofs = 6 * len(model.node_ids)
    rows, mags = _full_row_sums(rounded, n_dofs)
    try:
        p_rows = [[binary64(rows[k][a], "a row of p_k") if free[a] else None
                   for a in range(n_dofs)] for k in range(3)]
        totals = _totals(rounded)
    except OutsideModel as error:
        return [UNRESOLVED + f"p_k or T_k: {error}"]
    cumulative = [ZERO, ZERO, ZERO]
    cumulative_open = [False, False, False]
    for m_index, mode in enumerate(view.modes):
        label = m_index + 1
        phi = [None if vo.value is None else Iv(vo.value) for vo in view.shapes[m_index]]
        if any(v is None for v in phi):
            items.append(UNRESOLVED + f"mode {label}: a shape component is not published")
            cumulative_open = [True, True, True]
            continue
        nonzero = [not (v.lo == 0 and v.hi == 0) for v in phi]
        # m_i sums the free block over the shape's nonzero components; φᵀp_k the rows of
        # those components over column direction k; the fraction also T_k.
        m_open = any(free[a] and free[b] and nonzero[a] and nonzero[b] for a, b in open_keys)
        if m_open:
            items.append(UNRESOLVED + f"mode {label}: m, Γ, m_eff and the fractions use a mass entry outside the "
                         "rounding model")
            cumulative_open = [True, True, True]
            continue
        generalized = ZERO
        for (a, b), value in rounded.items():
            if free[a] and free[b] and nonzero[a] and nonzero[b]:
                generalized = generalized + phi[a] * value * phi[b] * (ONE if a == b else Iv(2))
        what = f"mode {label}: m against the model"
        _guarded(items, what, lambda: _judge(what, mode["generalized_mass"], published_image(generalized)))
        wide = relative(generalized, DD_RELATIVE)
        for k in range(3):
            direction = DIRECTIONS[k]
            row = view.participation[m_index][k]
            if any((free[a] and nonzero[a] and b % 6 == k) or (free[b] and nonzero[b] and a % 6 == k)
                   for a, b in open_keys):
                items.append(UNRESOLVED + f"mode {label} along {direction}: φᵀp_k uses a mass entry outside the "
                             "rounding model")
                cumulative_open[k] = True
                continue
            t_open = any(a % 6 == k and b % 6 == k for a, b in open_keys)
            q = ZERO
            for a in range(n_dofs):
                if free[a] and nonzero[a]:
                    q = q + phi[a] * p_rows[k][a]
            try:
                gamma = published_image(relative(q / wide, DD_RELATIVE * 2))
                effective = published_image(relative(q.square() / wide, DD_RELATIVE * 3))
                fraction = None if t_open else \
                    published_image(relative(q.square() / (wide * totals[k]), DD_RELATIVE * 4))
            except (Undecidable, OutsideModel) as error:
                items.append(UNRESOLVED + f"mode {label} along {direction}: {error}")
                cumulative_open[k] = True
                continue
            for name, vo, expected, signed in (("Γ", row["gamma"], gamma, True), ("m_eff", row["effective"], effective, False)):
                what = f"mode {label}: {name} along {direction} against the model"
                _guarded(items, what, lambda: _judge(what, vo, expected, signed))
            fraction_vo = row["fraction"]
            if t_open:
                items.append(UNRESOLVED + f"mode {label}: the fraction along {direction} uses T_{direction}, which sums "
                             "a mass entry outside the rounding model")
                cumulative_open[k] = True
                continue
            if fraction_vo.undefined == "total_mass_zero":
                if totals[k].excludes_zero():
                    raise Contradiction(f"mode {label}: the fraction along {direction} claims T_k = 0, but the model's T_k is not 0")
                cumulative_open[k] = True
                continue
            what = f"mode {label}: fraction along {direction} against the model"
            _guarded(items, what, lambda: _judge(what, fraction_vo, fraction))
            cumulative[k] = cumulative[k] + fraction
            if cumulative_open[k]:
                continue
            what = f"mode {label}: cumulative fraction along {direction} against the model"
            _guarded(items, what, lambda: _judge(what, row["cumulative"],
                                                 binary64(relative(cumulative[k], DD_RELATIVE * (label + 1)))))
    return items
