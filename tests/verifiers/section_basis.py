"""Test-only verifier helpers for T4-U6/U8's section bases and beam theory (DEL-05-03
``section-bases-and-timoshenko.md`` §1, §2.1-§2.5 and §3.1).

Independent of product code: standard-library imports only, mirroring the Rust
reader's ``section_basis`` checks in meaning and order (it imports no product
module). What it checks, from a ``pipe_sections`` entry's captured authored values
(outside diameter D, wall t, mill tolerance c_m, corrosion allowance c_a), in the
fixed binary64 order: the nominal wall t and inner radius D/2 - t, the reduced wall
(t - c_m) - c_a and inner radius D/2 - t_r, the pressure bore on the reduced inner
radius, coincident leaves where the walls coincide, and for
``timoshenko_annulus_energy_v1`` kappa_E and A_s = kappa_E * A_n bit for bit from the
nominal radii and the member's nu (this module's own implementation of §3.1's order;
Python floats never fuse a multiply-add). The section leaves stay leaves: the
retained exact successor overlays them with its prepared bits (DEF-E).

A realized arc's entry, and only an arc's, carries ``member_kind = "realized_arc"`` (§2.5, ``marked_section``), so a
reader without the model knows every arc, including one outside every pressure region: a marked entry is
Euler-Bernoulli and the unmarked entries share one straight shear model. Where a reader has the captured model (§5),
the beam theory each entry states is bound to it: the marked entries are the model's realized arcs, a straight member
states the contract's ``pressure_contract.shear_model``, a realized arc Euler-Bernoulli (``bind_shear_model``,
``selected_model``).

Stress rows name their basis in their token and id; every stress row has its
other-basis twin with the same declared semantics, checked against the two
sections (bit-equal where the bases coincide, else the same resultant within the
64-epsilon representation guard), and the Lame rows against their own annulus.
"""
from __future__ import annotations

from collections.abc import Callable, Mapping
import math
import struct
from typing import Any

from . import component_body

NOMINAL = "nominal_section"
REDUCED = "reduced_section"
BASES = (NOMINAL, REDUCED)
GEOMETRY_BASIS = "authored_od_wall_d5_sections_v1"
# §2.5: the key and the one value of a realized arc's marker; a straight member's entry has no member_kind.
MEMBER_KIND = "member_kind"
REALIZED_ARC = "realized_arc"
SECTION_KEYS = {"pipe_id", "geometry_basis", "outside_diameter_m", "ro_m", "wall_thickness_m", "mill_tolerance_m",
                "corrosion_allowance_m", "nominal_section", "reduced_section", "pressure_bore", "beam_theory"}
ANNULUS_KEYS = ("wall_m", "ri_m", "As_m2", "I_m4", "J_m4", "Z_m3")
BORE_KEYS = {"ri_m", "Ai_m2"}
EULER_BERNOULLI = "euler_bernoulli"
TIMOSHENKO = "timoshenko_annulus_energy_v1"
STRESS_KINDS = (
    # T4-U4b (§2.2): the new per-basis kinds.
    "element_local_normal_stress_extreme", "element_local_transverse_shear_stress_maximum",
    "pipe_toroidal_membrane_hoop_stress_v1", "pipe_toroidal_radial_surface_stress_v1",
    "curved_bend_directional_intensified_bending_stress_v1", "curved_bend_directional_intensified_bending_maximum_v1",
    "element_local_axial_normal_stress", "element_local_bending_normal_stress_y", "element_local_bending_normal_stress_z",
    "element_local_torsional_shear_stress", "pipe_axial_membrane_stress_v2", "pipe_lame_radial_stress_v2",
    "pipe_lame_hoop_stress_v2", "pipe_elastic_normal_stress_maximum_v2",
)
THIN_WALL_KINDS = ("pipe_section_pressure_hoop_stress", "pipe_section_pressure_longitudinal_stress")
SEPARATOR = "; section_basis="
EPSILON = 2.0 ** -52
MIN_POSITIVE = 2.0 ** -1022  # the least positive normal binary64 (Rust f64::MIN_POSITIVE)
EXTREMUM_KEYS = {"pipe_id", "section_basis", "result_id", "approximation", "coefficient_basis", "enclosure_scope", "station_fraction",
                 "span_index", "local_fraction", "value_lower_pa", "value_upper_pa", "global_upper_bound_pa", "certified_gap_pa", "subdivisions"}


class BasisError(ValueError):
    """A section-basis refusal; ``str(error)`` is the detail."""


def _require(ok: bool, detail: str) -> None:
    if not ok:
        raise BasisError(detail)


def _number(value: Any) -> float:
    _require(type(value) in (int, float) and math.isfinite(value), "nonfinite section value")
    return float(value)


def hyphenated(basis: str) -> str:
    return basis.replace("_", "-")


def other(basis: str) -> str:
    return REDUCED if basis == NOMINAL else NOMINAL


def split_token(token: Any) -> tuple[str, str] | None:
    """A basis-named token's (recovery token, section basis), else None."""
    if not isinstance(token, str) or SEPARATOR not in token:
        return None
    recovery, basis = token.rsplit(SEPARATOR, 1)
    if basis not in BASES or not recovery or SEPARATOR in recovery:
        return None
    return recovery, basis


def id_basis(row_id: Any) -> str | None:
    if not isinstance(row_id, str):
        return None
    return next((b for b in BASES if row_id.endswith(":" + b) or row_id.endswith(":" + hyphenated(b))), None)


def twin_id(row_id: str) -> str | None:
    for b in BASES:
        for source, target in ((":" + b, ":" + other(b)), (":" + hyphenated(b), ":" + hyphenated(other(b)))):
            if row_id.endswith(source):
                return row_id[: -len(source)] + target
    return None


def row_basis(row: Mapping[str, Any]) -> tuple[str, str]:
    metadata = row.get("metadata") if isinstance(row.get("metadata"), Mapping) else {}
    parsed = split_token(metadata.get("basis"))
    _require(parsed is not None and id_basis(row.get("id")) == parsed[1], "stress row section basis")
    return parsed  # type: ignore[return-value]


def reduced_wall(wall: float, mill_tolerance: float, corrosion_allowance: float) -> float:
    """§1's fixed order, t_r = (t - c_m) - c_a."""
    return (wall - mill_tolerance) - corrosion_allowance


def timoshenko_shear_factor(ri: float, ro: float, nu: float) -> float:
    """§3.1: kappa_E in its fixed binary64 order (not Cowper, not 5/6)."""
    m = ri / ro
    m2 = m * m
    a = 1.0 + m2
    b = 1.0 - m2
    n1 = 1.0 + nu
    c = (8.0 * nu + 14.0) * nu + 7.0
    d = c * (b * b) + 48.0 * (m2 * (n1 * n1))
    return (6.0 * (n1 * n1)) * (a * a) / d


def _annulus(value: Any, od: float, wall: float) -> dict[str, float]:
    _require(isinstance(value, Mapping) and set(value) == set(ANNULUS_KEYS), "section shape")
    leaves = {key: _number(value[key]) for key in ANNULUS_KEYS}
    _require(all(v > 0 for v in leaves.values()), "section range")
    _require(leaves["wall_m"] == wall and wall < od * 0.5 and leaves["ri_m"] == od * 0.5 - wall and leaves["ri_m"] < od * 0.5, "section source binding")
    return leaves


def arc_marker(value: Any) -> bool:
    """§2.5: an entry's realized-arc marker: True for ``member_kind = "realized_arc"``, False without
    ``member_kind``; any other ``member_kind`` is refused by name (Rust ``SOURCE_PHYSICS_MEMBER_KIND``)."""
    if not isinstance(value, Mapping) or MEMBER_KIND not in value:
        return False
    _require(value[MEMBER_KIND] == REALIZED_ARC, "member kind")
    return True


def section(value: Any) -> dict[str, Any]:
    """One ``pipe_sections`` entry with exactly ``SECTION_KEYS`` against its captured authored values; a marked entry
    is read by ``marked_section``."""
    return _section(value, False)


def marked_section(value: Any) -> tuple[dict[str, Any], bool]:
    """One ``pipe_sections`` entry and its realized-arc marker (``arc_marker``): a marked entry carries
    ``SECTION_KEYS`` and ``member_kind``, an unmarked one exactly ``SECTION_KEYS``."""
    arc = arc_marker(value)
    return _section(value, arc), arc


def _section(value: Any, marked: bool) -> dict[str, Any]:
    _require(isinstance(value, Mapping) and set(value) == SECTION_KEYS | ({MEMBER_KIND} if marked else set()), "section shape")
    _require(isinstance(value["pipe_id"], str) and bool(value["pipe_id"]) and value["geometry_basis"] == GEOMETRY_BASIS, "section basis")
    od, ro, wall = (_number(value[k]) for k in ("outside_diameter_m", "ro_m", "wall_thickness_m"))
    mill, allowance = _number(value["mill_tolerance_m"]), _number(value["corrosion_allowance_m"])
    _require(od > 0 and ro > 0 and wall > 0 and mill >= 0 and allowance >= 0, "section range")
    _require(ro == od * 0.5, "section source binding")
    t_r = reduced_wall(wall, mill, allowance)
    _require(mill + allowance < wall and t_r > 0, "section allowance")
    nominal = _annulus(value["nominal_section"], od, wall)
    reduced = _annulus(value["reduced_section"], od, t_r)
    bore = value["pressure_bore"]
    _require(isinstance(bore, Mapping) and set(bore) == BORE_KEYS, "section shape")
    _require(_number(bore["Ai_m2"]) > 0 and _number(bore["ri_m"]) == reduced["ri_m"], "section bases")
    _require(nominal["wall_m"] != reduced["wall_m"] or nominal == reduced, "section bases")
    beam = value["beam_theory"]
    if isinstance(beam, Mapping) and beam.get("model") == EULER_BERNOULLI and set(beam) == {"model"}:
        parsed: dict[str, Any] = {"model": EULER_BERNOULLI}
    elif isinstance(beam, Mapping) and beam.get("model") == TIMOSHENKO and set(beam) == {"model", "shear_factor", "shear_area_m2"}:
        parsed = {"model": TIMOSHENKO, "shear_factor": _number(beam["shear_factor"]), "shear_area_m2": _number(beam["shear_area_m2"])}
    else:
        raise BasisError("beam theory")
    return {"od": od, "ro": ro, "wall": wall, "mill": mill, "allowance": allowance, NOMINAL: nominal, REDUCED: reduced,
            "bore": {"ri_m": bore["ri_m"], "Ai_m2": bore["Ai_m2"]}, "beam": parsed}


def coincide(sections: Mapping[str, Any]) -> bool:
    return sections[NOMINAL] == sections[REDUCED]


def bits(value: float) -> int:
    return struct.unpack(">Q", struct.pack(">d", value))[0]


def bit_equal(a: Any, b: Any) -> bool:
    """T4-U6 (§2.5, A2): structural equality whose numbers compare by their binary64 bits, the one equality of twins
    and links in the Python verifiers and the Rust readers (``section_basis::bit_equal``): -0.0 is not 0.0, and the
    same binary64 number in another JSON form (``1e6`` or ``1000000``, ``0`` or ``0.0``) is the same number. A
    non-finite or non-number leaf is unequal to a number."""
    numbers = (int, float)
    if type(a) in numbers and type(b) in numbers:
        return math.isfinite(a) and math.isfinite(b) and bits(a) == bits(b)
    if type(a) is list and type(b) is list:
        return len(a) == len(b) and all(bit_equal(x, y) for x, y in zip(a, b))
    if isinstance(a, Mapping) and isinstance(b, Mapping):
        return a.keys() == b.keys() and all(bit_equal(a[k], b[k]) for k in a)
    return type(a) is type(b) and a == b


def beam_theory(sections: Mapping[str, Any], nu: float, arc: bool) -> None:
    beam = sections["beam"]
    if beam["model"] == EULER_BERNOULLI:
        return
    _require(not arc, "arc beam theory")
    kappa = timoshenko_shear_factor(sections[NOMINAL]["ri_m"], sections["ro"], nu)
    _require(math.isfinite(kappa) and kappa > 0 and bits(beam["shear_factor"]) == bits(kappa)
             and bits(beam["shear_area_m2"]) == bits(kappa * sections[NOMINAL]["As_m2"]), "timoshenko shear factor")


def realized_arcs(model: Any) -> set[str]:
    """§3.6: the realized arcs of a captured model document, as the producer realizes them: the pipes named by
    ``geometry.bend_pipe_ref`` of its ``bend`` or ``elbow`` components whose ``mechanics_interface.solver_consumption``
    is ``curved_bend_macro_element``. The evidence marks each arc's ``pipe_sections`` entry and region geometry (§2.5);
    the model names which pipes those are."""
    components = model.get("components") if isinstance(model, Mapping) else None
    arcs: set[str] = set()
    for component in components if isinstance(components, list) else []:
        if not isinstance(component, Mapping) or component.get("kind") not in ("bend", "elbow"):
            continue
        interface, geometry = component.get("mechanics_interface"), component.get("geometry")
        ref = geometry.get("bend_pipe_ref") if isinstance(geometry, Mapping) else None
        if isinstance(interface, Mapping) and interface.get("solver_consumption") == "curved_bend_macro_element" and isinstance(ref, str) and ref.strip():
            arcs.add(ref)
    return arcs


def selected_model(shear_model: Any, arc: bool) -> Any:
    """§3.6, §5: a realized arc is Euler-Bernoulli whatever the document selects; a straight member takes the
    contract's ``pressure_contract.shear_model``."""
    return EULER_BERNOULLI if arc else shear_model


def bind_shear_model(evidence: Any, model: Any) -> None:
    """§5 (pressure-2, against the captured model; Rust ``bind_shear_model``): each exact case's ``pipe_sections`` entry
    and each pressure region's geometry entry states the beam theory the model's contract selects for that member.
    §2.5: the entries the evidence marks are exactly the model's realized arcs. Arcs are the model's realized arcs and
    the arcs the region geometry marks. Refusals carry the Rust codes: ``SOURCE_PHYSICS_ARC_MARKER`` for a realized
    arc's unmarked entry or a straight member's marked entry, ``SOURCE_PHYSICS_MEMBER_KIND`` for another
    ``member_kind``, ``SOURCE_PHYSICS_ARC_BEAM_THEORY`` for an arc, ``SOURCE_PHYSICS_SHEAR_MODEL_CONTRACT`` for a
    straight member or a contract naming no admitted model."""
    items = lambda value: value if isinstance(value, list) else []
    contract = model.get("pressure_contract") if isinstance(model, Mapping) else None
    shear_model = contract.get("shear_model") if isinstance(contract, Mapping) else None
    _require(shear_model in (EULER_BERNOULLI, TIMOSHENKO), "SOURCE_PHYSICS_SHEAR_MODEL_CONTRACT")
    evidence = evidence if isinstance(evidence, Mapping) else {}
    regions = [region for region in items(evidence.get("pressure")) if isinstance(region, Mapping)]
    realized = realized_arcs(model)
    arcs = realized | {g.get("pipe_id") for region in regions for g in items(region.get("geometry")) if isinstance(g, Mapping) and MEMBER_KIND in g}
    entries = [s for case in items(evidence.get("exact_cases")) if isinstance(case, Mapping) for s in items(case.get("pipe_sections"))]
    entries += [g for region in regions for g in items(region.get("geometry"))]
    for entry in entries:
        entry = entry if isinstance(entry, Mapping) else {}
        _require(MEMBER_KIND not in entry or entry[MEMBER_KIND] == REALIZED_ARC, "SOURCE_PHYSICS_MEMBER_KIND")
        _require((MEMBER_KIND in entry) == (entry.get("pipe_id") in realized), "SOURCE_PHYSICS_ARC_MARKER")
        arc = entry.get("pipe_id") in arcs
        beam = entry.get("beam_theory")
        stated = beam.get("model") if isinstance(beam, Mapping) else None
        _require(stated == selected_model(shear_model, arc), "SOURCE_PHYSICS_ARC_BEAM_THEORY" if arc else "SOURCE_PHYSICS_SHEAR_MODEL_CONTRACT")


def agree(a: float, b: float) -> bool:
    """The 64-epsilon representation guard on the E17 scale."""
    return math.isfinite(a) and math.isfinite(b) and abs(a - b) <= 64 * EPSILON * max(abs(a), abs(b), MIN_POSITIVE)


DIVISOR = {"element_local_axial_normal_stress": "As_m2", "pipe_axial_membrane_stress_v2": "As_m2",
           "element_local_bending_normal_stress_y": "Z_m3", "element_local_bending_normal_stress_z": "Z_m3",
           "element_local_torsional_shear_stress": "J_m4"}


def twin_values(kind: str, nominal: float, reduced: float, sections: Mapping[str, Any]) -> None:
    """Rust ``twin_values``: bit-equal where the bases coincide (a signed zero is not its twin's zero); else the same
    resultant for the linear kinds and bit-equal Lame radial values."""
    if coincide(sections):
        _require(bits(nominal) == bits(reduced), "stress basis section")
        return
    if kind in DIVISOR:
        key = DIVISOR[kind]
        _require(agree(nominal * sections[NOMINAL][key], reduced * sections[REDUCED][key]), "stress basis section")
    elif kind == "pipe_lame_radial_stress_v2":
        _require(bits(nominal) == bits(reduced), "stress basis section")


def lame_station(values: Mapping[str, float], annulus: Mapping[str, float], p: float) -> None:
    keys = ("lame_inner_radial_stress", "lame_outer_radial_stress", "lame_inner_hoop_stress", "lame_outer_hoop_stress")
    _require(all(k in values for k in keys), "pressure station coverage")
    bore = 2.0 * (p * ((math.pi * annulus["ri_m"]) * annulus["ri_m"]))
    _require(values["lame_inner_radial_stress"] == -p and values["lame_outer_radial_stress"] == 0
             and agree(values["lame_outer_hoop_stress"] * annulus["As_m2"], bore)
             and agree(values["lame_inner_hoop_stress"], values["lame_outer_hoop_stress"] + p), "stress basis section")


def twins(rows: Mapping[str, Any], sections: Mapping[tuple[str, str], Any], pressures: Mapping[tuple[str, str], float],
          order: Callable[[Mapping[str, Any], Mapping[str, Any]], None] | None = None) -> None:
    """§2.1, §2.5: both families of every stress signature. Each nominal row's twin is the row of the same id with the
    reduced segment, the same recovery token and the same declared semantics; each pair is checked against its member's
    two sections, and each straight member's Lame rows against each basis's own annulus and its region's pressure
    (``pressures`` by (case, member)). ``order(row, twin)``, when given, is the caller's row-order check on each
    nominal row and its twin, made after the twin is matched and before the pair's values are compared."""
    stress = [row for row in rows.values() if row.get("kind") in STRESS_KINDS]
    pairs = 0
    lame: dict[tuple[str, str, str, str], dict[str, float]] = {}
    for row in stress:
        token, basis = row_basis(row)
        cid, pid, metadata = row["basis_ref"]["ref_id"], row["entity_ref"], row["metadata"]
        _require((cid, pid) in sections, "stress row member scope")
        if row["kind"] in ("pipe_lame_radial_stress_v2", "pipe_lame_hoop_stress_v2"):
            values = lame.setdefault((cid, pid, metadata["location"], basis), {})
            _require(metadata["component"] not in values, "duplicate physical result slot")
            values[metadata["component"]] = row["value"]
        if basis != NOMINAL:
            continue
        twin = rows.get(twin_id(row["id"]) or "")
        _require(isinstance(twin, Mapping) and row_basis(twin) == (token, REDUCED)
                 and all(twin.get(k) == row.get(k) for k in ("kind", "unit", "entity_ref", "basis_ref"))
                 and all(twin["metadata"].get(k) == metadata.get(k) for k in ("component", "coordinate_system", "location", "sign_convention")), "stress basis twin")
        if order is not None:
            order(row, twin)
        twin_values(row["kind"], row["value"], twin["value"], sections[cid, pid])
        pairs += 1
    _require(2 * pairs == len(stress), "stress basis twin")
    for (cid, pid, _location, basis), values in lame.items():
        _require((cid, pid) in pressures, "unbound pressure row")
        lame_station(values, sections[cid, pid][basis], pressures[cid, pid])


def coverage(value: Any) -> list[tuple[set[str], bool]]:
    """``stress_maximum_coverage``: the nominal entry then the reduced entry."""
    _require(isinstance(value, list) and len(value) == 2, "maximum coverage")
    out = []
    for entry, basis in zip(value, BASES):
        _require(isinstance(entry, Mapping) and set(component_body.coverage_without_bodies(entry)) == {"section_basis", "complete", "unavailable_pipe_ids"} and entry["section_basis"] == basis, "maximum coverage")
        ids = entry["unavailable_pipe_ids"]
        _require(isinstance(ids, list) and all(isinstance(i, str) and i for i in ids) and len(set(ids)) == len(ids) and type(entry["complete"]) is bool, "maximum coverage values")
        _require(entry["complete"] == (not ids), "maximum coverage contradiction")
        out.append((set(ids), entry["complete"]))
    return out


def extremum_basis(value: Mapping[str, Any]) -> str:
    basis = value.get("section_basis") if isinstance(value, Mapping) else None
    _require(basis in BASES, "extrema shape")
    return basis  # type: ignore[return-value]


def extrema_twins(nominal: Mapping[str, Any], reduced: Mapping[str, Any], sections: Mapping[str, Any]) -> None:
    """Rust ``extrema_twins``: on coincident bases the reduced extremum is the nominal one but for its row and basis,
    its numbers bit for bit (``bit_equal``)."""
    if coincide(sections):
        strip = lambda v: {k: x for k, x in v.items() if k not in ("result_id", "section_basis")}
        _require(bit_equal(strip(nominal), strip(reduced)), "stress basis section")
    elif sections[REDUCED]["As_m2"] <= sections[NOMINAL]["As_m2"] and sections[REDUCED]["Z_m3"] <= sections[NOMINAL]["Z_m3"]:
        _require(reduced["value_upper_pa"] >= nominal["value_lower_pa"], "stress basis section")


def headlines(summary: Any, rows: Mapping[str, Any], complete: Mapping[str, bool], bodies: Any = frozenset()) -> None:
    """§2.2: ``max_open_formula_stress`` null, both basis keys present, each non-null headline on complete coverage of
    its basis, bound to a maximum row of its own basis (its value bit for bit, ``bit_equal``) and not below any
    maximum of that basis."""
    _require(isinstance(summary, Mapping) and "max_open_formula_stress" in summary and summary["max_open_formula_stress"] is None
             and all(f"max_open_formula_stress_{b}" in summary for b in BASES), "stress maximum headline bases")
    # W-15 slice d (DEL-03-05 Design §5): the headlines range over the non-body members.
    maxima = [row for row in rows.values() if row.get("kind") == "pipe_elastic_normal_stress_maximum_v2" and row.get("entity_ref") not in bodies]
    for basis in BASES:
        headline = summary[f"max_open_formula_stress_{basis}"]
        _require(headline is None or not isinstance(headline, Mapping) or headline.get("location_ref") not in bodies, "stress maximum headline names a component body")
        if not complete[basis]:
            _require(headline is None, "incomplete stress maximum headline")
            continue
        if headline is None:
            continue
        _require(isinstance(headline, Mapping) and set(headline) == {"value", "unit", "location_ref", "result_ref"}, "stress maximum headline")
        own = [row for row in maxima if (split_token((row.get("metadata") or {}).get("basis")) or ("", ""))[1] == basis]
        selected = rows.get(headline.get("result_ref"), {})
        _require(bool(own) and selected in own and bit_equal(headline.get("value"), selected["value"]) and selected["value"] == max(row["value"] for row in own)
                 and headline.get("unit") == selected["unit"] and headline.get("location_ref") == selected["entity_ref"], "stress maximum headline binding")
