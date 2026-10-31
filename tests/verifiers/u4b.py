"""T4-U4b reader checks of the new kinds (DEL-04-07 Design u4b-u5b-implementation.md §5.1), test-only.

Written from the design, not from the Rust reader; it shares no code with it.

- σ⁺/σ⁻ (``element_local_normal_stress_extreme``): recomputed bit for bit from the same station's and basis's
  published stress rows (§2.3; the membrane row /1e6 on a pressurized pressure-2 member), with the norm from an
  exact-integer oracle (the same as ``preview_physics_evidence.correctly_rounded_norm``, restated here so this module
  is standard library only); present exactly where its operands are.
- τ (``element_local_transverse_shear_stress_maximum``): custody, coverage (members named by
  ``TRANSVERSE_SHEAR_POISSON_OUT_OF_DOMAIN`` exempt), non-negativity and unit; bit for bit when c_b is supplied
  (pressure-2 from the evidence ν and sections, preview from the captured model).
- Directional intensified rows: bound to their ``intensified_measures`` records by key set and recomputed; the
  maximum's record follows the arc maximum's checks with the station intensified rows as its cover.
- Branch intensified rows (RT-A3 increment 1, pressure-2 only): a straight member's end rows, bound to their records
  by key set and recomputed from that end's and basis's bending rows and the record's SIF; bound to the captured
  model's tee associations (``branch_model_checks``, mirroring Rust ``u4b_evidence::branch_model_checks``).
- Toroidal hoop and radial rows (pressure-2): bit-equal by the normative sequence, exactly where R > r_o.
- The arc maximum's record: closed tokens, gap rule, midpoint, fraction and station cover, only on arcs.

Python's float operations are binary64 round to nearest; no operation below is fused.
"""
from __future__ import annotations

import math
from typing import Any, Mapping


EXTREME = "element_local_normal_stress_extreme"
SHEAR_MAX = "element_local_transverse_shear_stress_maximum"
HOOP = "pipe_toroidal_membrane_hoop_stress_v1"
RADIAL = "pipe_toroidal_radial_surface_stress_v1"
DIRECTIONAL = "curved_bend_directional_intensified_bending_stress_v1"
DIRECTIONAL_MAX = "curved_bend_directional_intensified_bending_maximum_v1"
# RT-A3 increment 1: a branch connection's intensified row (pressure-2 only).
BRANCH = "branch_intensified_bending_stress_v1"
KINDS = frozenset({EXTREME, SHEAR_MAX, HOOP, RADIAL, DIRECTIONAL, DIRECTIONAL_MAX, BRANCH})

EXTREMES_BASIS = "section_extremes_from_published_stress_rows_v1"
SHEAR_STRAIGHT_BASIS = "saint_venant_annulus_flexure_v1"
SHEAR_ARC_BASIS = "local_straight_annulus_on_arc"
HOOP_BASIS = "bore_load_transferred_toroidal_membrane_v1"
DIRECTIONAL_BASIS = "user_directional_sif_times_tangent_frame_bending_stress_v1"
BRANCH_BASIS = "user_branch_sif_times_basis_bending_stress_rows_v1"
PREVIEW_ARC_BASIS = "nominal_straight_beam_formula_on_arc_resultants"

SIGMA_PLUS_SIGN = "signed section normal-stress extreme sigma+ = N/A + hypot(My,Mz)/Z at the station (N_w/A_s on a pressurized pressure-2 member), formed from the published axial or membrane and bending stress rows of the same station and basis; at the outer-surface fibre r_o(-sigma_z, sigma_y)/hypot(sigma_y, sigma_z) of the section frame, every outer fibre tied when the bending rows are zero; not a code or equivalent stress"
SIGMA_MINUS_SIGN = SIGMA_PLUS_SIGN.replace("sigma+ = N/A + hypot", "sigma- = N/A - hypot").replace("fibre r_o(-sigma_z", "fibre -r_o(-sigma_z")
SHEAR_SIGN_EXACT = "nonnegative Saint-Venant transverse shear maximum |V|((3+2nu)ro^2+(1+2nu)ri^2)/(4(1+nu)I) of the basis annulus, on the inner surface at the two fibres normal to V; V from the published j-side section shears inside and the end-force shears at the ends; nu is the member's authored nu; torsional shear separate"
SHEAR_SIGN_PREVIEW = "nonnegative Saint-Venant transverse shear maximum |V|((3+2nu)ro^2+(1+2nu)ri^2)/(4(1+nu)I) of the section annulus, on the inner surface at the two fibres normal to V; V from the published j-side section shears inside and the end-force shears at the ends; nu = E/(2G) - 1 from the moduli the solve uses, assuming an isotropic material; torsional shear separate"
SHEAR_ARC_SUFFIX = "; on an arc the straight-annulus field is driven by the tangent-frame shears without curvature correction"
HOOP_SIGN = "tension-positive toroidal membrane hoop: the area average over the wall cut at the named section angle (intrados beta = 0, crown 90 deg, extrados 180 deg) of a complete torus in an axisymmetric membrane state under p on the basis bore; station-independent"
RADIAL_SIGN = "tension-positive radial stress of the torus section: -p at the basis bore and 0 at the outer surface"
DIRECTIONAL_SIGN = "nonnegative hypot(i_ip*Mz, i_op*My)/Z in the arc tangent frame with the user-entered directional SIFs of its intensified_measures record (i_ip on the in-plane Mz, i_op on the out-of-plane My); not a code stress; no flexibility factor, axial or torsion term; never combined"
ARC_MAXIMUM_SIGN = "nonnegative maximum |Nw/As|+hypot(My,Mz)/Z along the arc, bounded over the binary64 arc problem formed from the published end-j force and the uniform intensities; torsional shear remains separate; no code stress or equivalent stress claim"
DIRECTIONAL_MAX_SIGN = "nonnegative along-arc maximum of hypot(i_ip*Mz, i_op*My)/Z with the user-entered directional SIFs of its intensified_measures record, bounded over the binary64 arc problem formed from the published end-j force and the uniform intensities; never combined"
BRANCH_SIGN = "nonnegative i*hypot(My,Mz)/Z at the member end, formed as 1e6*hypot(i*sigma_z, i*sigma_y) from the same end's and basis's published bending stress rows, with the user-entered tee header or branch SIF i of its intensified_measures record; not a code effective section modulus; not a code stress; no flexibility factor, axial or torsion term; never combined"
ARC_TOKENS = ("arc_closed_form_section_statics", "arc_end_j_force_and_uniform_intensities_binary64_enclosures",
              "published_binary64_end_j_force_and_uniform_intensity_operand_enclosures; solution error is separate")
DIRECTIONAL_APPROXIMATION = "arc_closed_form_directional_intensified"
POISSON_OUT_OF_DOMAIN = "TRANSVERSE_SHEAR_POISSON_OUT_OF_DOMAIN"
TOROIDAL_DOMAIN = "ARC_TOROIDAL_HOOP_DOMAIN"
RECORD_KEYS = frozenset({"pipe_id", "result_id", "station_fraction", "span_index", "local_fraction", "value_lower_pa", "value_upper_pa",
                         "global_upper_bound_pa", "certified_gap_pa", "subdivisions", "approximation", "coefficient_basis", "enclosure_scope"})
STATION_RECORD_KEYS = frozenset({"pipe_id", "result_id", "location", "i_in_plane", "i_out_of_plane"})
# RT-A3 increment 1: a branch record's keys (pressure-2 adds ``section_basis``).
BRANCH_RECORD_KEYS = frozenset({"pipe_id", "result_id", "location", "component_id", "factor_role", "sif", "sif_source_reference"})
BRANCH_ROLES = ("branch_header", "branch_branch")
BRANCH_COMPONENT_KINDS = ("branch", "tee", "branch_connection")
STATIONS = ("end_i", "end_j", "quarter_1", "midspan", "quarter_3")
SEPARATOR = "; section_basis="


def correctly_rounded_norm(components: list[float]) -> float:
    """RN(sqrt(sum of squares)) of finite binary64 components, ties to even; +inf beyond range.

    Independent exact-integer oracle: align the published binary64 dyadics,
    square and sum exactly, then round the square root on the binary64 grid.
    This does not share the Rust implementation's FMA/expansion algorithm.
    """
    ratios = [float(value).as_integer_ratio() for value in components]
    denominator_exponent = max(denominator.bit_length() - 1 for _, denominator in ratios)
    square_sum = sum((numerator << (denominator_exponent - (denominator.bit_length() - 1))) ** 2
                     for numerator, denominator in ratios)
    if square_sum == 0:
        expected = 0.0
    else:
        root_exponent = (square_sum.bit_length() - 1) // 2 - denominator_exponent
        grid_exponent = max(root_exponent - 52, -1074)
        shift = denominator_exponent + grid_exponent
        if shift >= 0:
            lower = math.isqrt(square_sum) >> shift
            midpoint_square = (2 * lower + 1) ** 2 << (2 * shift)
            four_square_sum = 4 * square_sum
        else:
            scaled_square_sum = square_sum << (-2 * shift)
            lower = math.isqrt(scaled_square_sum)
            midpoint_square = (2 * lower + 1) ** 2
            four_square_sum = 4 * scaled_square_sum
        rounded = lower + int(four_square_sum > midpoint_square or (four_square_sum == midpoint_square and lower & 1))
        try:
            expected = math.ldexp(float(rounded), grid_exponent)
        except OverflowError:
            expected = math.inf
    return expected


def _require(condition: Any, detail: str) -> None:
    if not condition:
        raise ValueError(f"SOURCE_U4B_EVIDENCE_INVALID: {detail}")


def bits(value: float) -> int:
    import struct
    return struct.unpack(">Q", struct.pack(">d", float(value)))[0]


# The normative sequences (stress_recovery `section_stress_quantities`), restated from the design.

def extremes(sigma_n: float, sigma_y: float, sigma_z: float) -> tuple[float, float]:
    h = correctly_rounded_norm([sigma_y, sigma_z])
    return sigma_n + h, sigma_n - h


def shear_factor(nu: float, ro: float, ri: float, inertia: float) -> float | None:
    if not (math.isfinite(nu) and -1.0 < nu < 0.5 and 0.0 < ri < ro and inertia > 0.0):
        return None
    numerator = (3.0 + 2.0 * nu) * (ro * ro) + (1.0 + 2.0 * nu) * (ri * ri)
    denominator = (4.0 * (1.0 + nu)) * inertia
    factor = numerator / denominator
    return factor if math.isfinite(factor) and factor > 0 else None


def preview_shear_factor(e: float, g: float, ro: float, ri: float, inertia: float) -> float | None:
    """c_b on preview-physics-2 from the moduli (§2.4), bit for bit with the product: the ν sequence of
    ``shear_factor`` with ν = RN(q − 1) while q = RN(E/RN(2·G)) >= 1/2; below, with no 1+ν formed,
    x = RN(RN(2·G)/E) and c_b = RN(RN(RN(2·RN(RN(r_o·r_o) + RN(r_i·r_i))) + RN(x·RN(RN(r_o − r_i)·RN(r_o + r_i))))/RN(4·I)).
    None outside -1 < ν < 1/2 (admission by the rounded ν)."""
    ratio = e / (2.0 * g)
    if not ratio < 0.5:
        return shear_factor(ratio - 1.0, ro, ri, inertia)
    if not (ratio - 1.0 > -1.0 and e > 0.0 and g > 0.0 and 0.0 < ri < ro and inertia > 0.0):
        return None
    x = (2.0 * g) / e
    if not (math.isfinite(x) and x > 2.0 / 3.0):
        return None
    factor = (2.0 * (ro * ro + ri * ri) + x * ((ro - ri) * (ro + ri))) / (4.0 * inertia)
    return factor if math.isfinite(factor) and factor > 0 else None


def shear_maximum(shear_y: float, shear_z: float, factor: float) -> float:
    return correctly_rounded_norm([shear_y, shear_z]) * factor / 1e6


def directional(i_ip: float, i_op: float, sigma_y: float, sigma_z: float) -> float:
    return 1e6 * correctly_rounded_norm([i_ip * sigma_z, i_op * sigma_y])


def hoop(p: float, ri: float, wall: float, ro: float, r: float) -> tuple[float, float, float] | None:
    if not (r > ro and ri > 0 and wall > 0 and ro > 0):
        return None
    a = p * ri / wall
    two_r = 2.0 * r
    s = ri + ro
    if not two_r - s > 0:
        return None
    return a * ((two_r - ri) / (two_r - s)), a, a * ((two_r + ri) / (two_r + s))


def pa_upward(mpa: float) -> float:
    pa = mpa * 1e6
    return 0.0 if pa == 0 else math.nextafter(pa, math.inf)


def _case(row: Mapping[str, Any]) -> str | None:
    basis = row.get("basis_ref")
    return basis.get("ref_id") if isinstance(basis, Mapping) and basis.get("ref_type") == "load_case" else None


def _basis(row: Mapping[str, Any]) -> str:
    text = (row.get("metadata") or {}).get("basis") or ""
    return text.rsplit(SEPARATOR, 1)[1] if SEPARATOR in text else ""


def _token(row: Mapping[str, Any]) -> str:
    text = (row.get("metadata") or {}).get("basis") or ""
    return text.rsplit(SEPARATOR, 1)[0] if SEPARATOR in text else text


class Rows:
    """The case rows by (case, entity, kind, component, location, basis)."""

    def __init__(self, results: list[Mapping[str, Any]]) -> None:
        self.slots: dict[tuple, Mapping[str, Any]] = {}
        for row in results:
            case = _case(row)
            if case is None:
                continue
            metadata = row.get("metadata") or {}
            key = (case, row.get("entity_ref"), row.get("kind"), metadata.get("component", ""), metadata.get("location", ""), _basis(row))
            _require(not (key in self.slots and row.get("kind") in KINDS), "duplicate row slot")
            self.slots[key] = row

    def value(self, case: str, entity: str, kind: str, component: str, location: str, basis: str) -> float | None:
        row = self.slots.get((case, entity, kind, component, location, basis))
        return None if row is None else row["value"]

    def any(self, case: str, entity: str, kind: str, location: str) -> float | None:
        for key, row in self.slots.items():
            if key[0] == case and key[1] == entity and key[2] == kind and key[4] == location and key[5] == "":
                return row["value"]
        return None


def row_semantics(row: Mapping[str, Any], exact: bool, arc: bool) -> None:
    kind = row.get("kind")
    if kind not in KINDS:
        return
    metadata = row.get("metadata") or {}
    location = metadata.get("location")
    _require(_case(row) is not None and metadata.get("coordinate_system") == "pipe_section", "new kind scope or frame")
    _require(_basis(row) in ("nominal_section", "reduced_section") if exact else SEPARATOR not in (metadata.get("basis") or ""), "new kind section basis")
    shear = SHEAR_SIGN_EXACT if exact else SHEAR_SIGN_PREVIEW
    expected = {
        EXTREME: ("MPa", {"sigma_plus", "sigma_minus"}, EXTREMES_BASIS, SIGMA_MINUS_SIGN if metadata.get("component") == "sigma_minus" else SIGMA_PLUS_SIGN, True, False),
        SHEAR_MAX: ("MPa", {"tau_v_max"}, SHEAR_ARC_BASIS if arc else SHEAR_STRAIGHT_BASIS, shear + (SHEAR_ARC_SUFFIX if arc else ""), True, False),
        HOOP: ("Pa", {"toroidal_intrados_hoop_stress", "toroidal_crown_hoop_stress", "toroidal_extrados_hoop_stress"}, HOOP_BASIS, HOOP_SIGN, True, True),
        RADIAL: ("Pa", {"toroidal_inner_radial_stress", "toroidal_outer_radial_stress"}, HOOP_BASIS, RADIAL_SIGN, True, True),
        DIRECTIONAL: ("Pa", {"intensified_bending"}, DIRECTIONAL_BASIS, DIRECTIONAL_SIGN, True, True),
        DIRECTIONAL_MAX: ("Pa", {"intensified_bending_maximum"}, DIRECTIONAL_BASIS, DIRECTIONAL_MAX_SIGN, False, True),
        BRANCH: ("Pa", {"intensified_bending"}, BRANCH_BASIS, BRANCH_SIGN, True, False),
    }[kind]
    unit, components, token, sign, at_station, arc_only = expected
    # RT-A3 increment 1: a branch row is the exact route's, at a straight member's end.
    _require(kind != BRANCH or (exact and not arc and location in ("end_i", "end_j")), f"{kind} off a straight member's end on the exact route")
    _require(not (arc_only and not arc), f"{kind} on a straight member")
    _require(exact or kind not in (HOOP, RADIAL), "toroidal rows on preview")
    _require(row.get("unit") == unit and metadata.get("component") in components and _token(row) == token
             and metadata.get("sign_convention") == sign
             and (location in STATIONS if at_station else location == "governing_station")
             and row.get("source_result_refs", []) == [], f"{kind} semantics")


def member_stations(rows: Rows, case: str, pipe: str, bases: list[str], factors: Mapping[str, float | None],
                    exempt: bool, toroidal: Mapping[str, tuple] | None) -> dict[str, list[float]]:
    """σ±, τ, hoop and radial of one member; returns each basis's station cover (Pa, rounded upward)."""
    cover: dict[str, list[float]] = {}
    for basis in bases:
        for location in STATIONS:
            y = rows.value(case, pipe, "element_local_bending_normal_stress_y", "bending_normal_stress_y", location, basis)
            z = rows.value(case, pipe, "element_local_bending_normal_stress_z", "bending_normal_stress_z", location, basis)
            n = rows.value(case, pipe, "element_local_axial_normal_stress", "axial_normal_stress", location, basis)
            if n is None:
                membrane = rows.value(case, pipe, "pipe_axial_membrane_stress_v2", "axial_membrane_stress", location, basis)
                n = None if membrane is None else membrane / 1e6
            plus = rows.value(case, pipe, EXTREME, "sigma_plus", location, basis)
            minus = rows.value(case, pipe, EXTREME, "sigma_minus", location, basis)
            if n is not None and y is not None and z is not None:
                want = extremes(n, y, z)
                _require(plus is not None and minus is not None and bits(plus) == bits(want[0]) and bits(minus) == bits(want[1]), "sigma extreme value or coverage")
                cover.setdefault(basis, []).append(pa_upward(max(abs(plus), abs(minus))))
            else:
                _require(plus is None and minus is None, "sigma extreme without operands")
            tau = rows.value(case, pipe, SHEAR_MAX, "tau_v_max", location, basis)
            vy = rows.any(case, pipe, "element_local_shear_force_y", location)
            vz = rows.any(case, pipe, "element_local_shear_force_z", location)
            if exempt or vy is None or vz is None:
                _require(tau is None, "transverse shear coverage")
            else:
                _require(tau is not None and tau >= 0, "transverse shear coverage or sign")
                factor = factors.get(basis)
                if factor is not None:
                    _require(bits(tau) == bits(shear_maximum(vy, vz, factor)), "transverse shear value")
            values = [rows.value(case, pipe, HOOP, c, location, basis) for c in ("toroidal_intrados_hoop_stress", "toroidal_crown_hoop_stress", "toroidal_extrados_hoop_stress")]
            radial = [rows.value(case, pipe, RADIAL, c, location, basis) for c in ("toroidal_inner_radial_stress", "toroidal_outer_radial_stress")]
            operands = (toroidal or {}).get(basis)
            expected = hoop(*operands) if operands is not None else None
            if expected is None:
                _require(all(v is None for v in values + radial), "toroidal coverage")
            else:
                p = operands[0]
                _require(all(v is not None and bits(v) == bits(w) for v, w in zip(values, expected)), "toroidal hoop value")
                _require(radial[0] is not None and bits(radial[0]) == bits(-p) and radial[1] is not None and bits(radial[1]) == bits(0.0), "toroidal radial value")
    return cover


def maximum_record(record: Mapping[str, Any], row: Mapping[str, Any], approximation: str, cover: list[float]) -> None:
    _require(record.get("approximation") == approximation and record.get("coefficient_basis") == ARC_TOKENS[1]
             and record.get("enclosure_scope") == ARC_TOKENS[2] and record.get("span_index") == 0 and type(record.get("span_index")) is int
             and type(record.get("subdivisions")) is int and 0 <= record["subdivisions"] <= 131072, "arc maximum tokens")
    lower, upper, glob, gap = (record.get(k) for k in ("value_lower_pa", "value_upper_pa", "global_upper_bound_pa", "certified_gap_pa"))
    fraction = record.get("station_fraction")
    _require(all(isinstance(v, (int, float)) and math.isfinite(v) for v in (lower, upper, glob, gap, fraction)), "arc maximum numbers")
    _require(0 <= fraction <= 1 and bits(record.get("local_fraction")) == bits(fraction), "arc maximum fraction")
    _require(lower >= 0 and upper >= lower and glob == upper and gap >= 0 and gap <= 1e-12 + 1e-12 * lower and gap >= glob - lower
             and row.get("value") == lower + 0.5 * (upper - lower), "arc maximum bounds")
    _require(len(cover) == len(STATIONS) and all(upper >= c for c in cover), "arc maximum station cover")


def directional_records(case: str, records: list[Mapping[str, Any]], rows_by_id: Mapping[str, Mapping[str, Any]],
                        rows: Rows, arcs: set[str], with_basis: bool) -> set[str]:
    """The directional station and maximum records (by key set); returns their bound row ids."""
    extra = {"section_basis"} if with_basis else set()
    bound: set[str] = set()
    stations: dict[tuple[str, str], list[float]] = {}
    factors: dict[str, tuple[int, int]] = {}
    maxima = []
    for record in records:
        keys = set(record) if isinstance(record, Mapping) else set()
        station = keys == STATION_RECORD_KEYS | extra
        maximum = keys == RECORD_KEYS | {"i_in_plane", "i_out_of_plane"} | extra
        if not (station or maximum):
            continue
        pipe, rid = record["pipe_id"], record["result_id"]
        _require(pipe in arcs, "directional record on a straight member")
        row = rows_by_id.get(rid)
        _require(row is not None and rid not in bound, "directional record binding")
        bound.add(rid)
        basis = record["section_basis"] if with_basis else ""
        i_ip, i_op = record["i_in_plane"], record["i_out_of_plane"]
        _require(all(isinstance(v, (int, float)) and math.isfinite(v) and v > 0 for v in (i_ip, i_op)), "directional factors")
        _require(factors.setdefault(pipe, (bits(i_ip), bits(i_op))) == (bits(i_ip), bits(i_op)), "directional factors differ")
        _require(_case(row) == case and row.get("entity_ref") == pipe and _basis(row) == basis, "directional row binding")
        if station:
            location = record["location"]
            _require(row.get("kind") == DIRECTIONAL and (row.get("metadata") or {}).get("location") == location, "directional row binding")
            y = rows.value(case, pipe, "element_local_bending_normal_stress_y", "bending_normal_stress_y", location, basis)
            z = rows.value(case, pipe, "element_local_bending_normal_stress_z", "bending_normal_stress_z", location, basis)
            _require(y is not None and z is not None and bits(row["value"]) == bits(directional(i_ip, i_op, y, z)), "directional value")
            stations.setdefault((pipe, basis), []).append(row["value"])
        else:
            _require(row.get("kind") == DIRECTIONAL_MAX, "directional maximum binding")
            maxima.append((pipe, basis, record, row))
    for pipe, basis, record, row in maxima:
        maximum_record(record, row, DIRECTIONAL_APPROXIMATION, stations.get((pipe, basis), []))
    _require(all(len(values) == len(STATIONS) for values in stations.values()), "directional coverage")
    return bound


def _number(value: Any) -> bool:
    """A JSON number as Rust's ``as_f64`` reads it (a boolean is not one)."""
    return isinstance(value, (int, float)) and not isinstance(value, bool)


def _nonempty(value: Any) -> bool:
    return isinstance(value, str) and value != ""


def branch_records(case: str, records: list[Any], rows_by_id: Mapping[str, Mapping[str, Any]], rows: Rows) -> set[str]:
    """RT-A3 increment 1 (Rust ``u4b_evidence::branch_records``): the branch records of one pressure-2 case, told apart
    from the directional records by key set (``BRANCH_RECORD_KEYS`` and ``section_basis``); each bound to one branch row
    of its member, end, basis and case and recomputed bit for bit from that end's and basis's bending rows and the
    record's SIF. Returns the bound row ids."""
    bound: set[str] = set()
    for record in records:
        if not (isinstance(record, Mapping) and set(record) == BRANCH_RECORD_KEYS | {"section_basis"}):
            continue
        pipe, rid, location = record["pipe_id"], record["result_id"], record["location"]
        _require(isinstance(pipe, str) and isinstance(rid, str) and location in ("end_i", "end_j"), "branch record shape")
        _require(_nonempty(record["component_id"]) and record["factor_role"] in BRANCH_ROLES
                 and _nonempty(record["sif_source_reference"]), "branch record shape")
        sif = record["sif"]
        _require(_number(sif) and math.isfinite(sif) and sif > 0, "branch factor range")
        basis = record["section_basis"] if isinstance(record["section_basis"], str) else "-"
        row = rows_by_id.get(rid)
        _require(row is not None, "branch record result missing")
        _require(rid not in bound, "branch record duplicate")
        bound.add(rid)
        _require(_case(row) == case and row.get("entity_ref") == pipe and _basis(row) == basis and row.get("kind") == BRANCH
                 and (row.get("metadata") or {}).get("location") == location, "branch row binding")
        y = rows.value(case, pipe, "element_local_bending_normal_stress_y", "bending_normal_stress_y", location, basis)
        z = rows.value(case, pipe, "element_local_bending_normal_stress_z", "bending_normal_stress_z", location, basis)
        _require(y is not None and z is not None, "branch operands")
        value = row.get("value")
        _require(_number(value) and bits(value) == bits(directional(sif, sif, y, z)), "branch value")
    return bound


def twin_order(results: list[Mapping[str, Any]], arcs: set[str]) -> None:
    """D-5 row order (section-bases-and-timoshenko.md §2.5) for the U4b families and the arc maxima on pressure-2:
    each nominal row is immediately followed by its reduced twin (same case, entity, kind, component and location)."""
    for position, row in enumerate(results):
        kind = row.get("kind")
        if not (kind in KINDS or (kind == "pipe_elastic_normal_stress_maximum_v2" and row.get("entity_ref") in arcs)):
            continue
        if _basis(row) != "nominal_section":
            continue
        following = results[position + 1] if position + 1 < len(results) else None
        metadata = row.get("metadata") or {}
        twin = isinstance(following, Mapping) and _basis(following) == "reduced_section" and _case(following) == _case(row) and all(
            following.get(k) == row.get(k) for k in ("kind", "entity_ref")) and all(
            (following.get("metadata") or {}).get(k) == metadata.get(k) for k in ("component", "location"))
        _require(twin, "basis twin order")


DIRECTIONAL_MAX_UNAVAILABLE = "DIRECTIONAL_INTENSIFIED_MAXIMUM_UNAVAILABLE"


def directional_arcs(model: Mapping[str, Any]) -> dict[str, tuple[float, float]]:
    """The model's realized arcs with the directional pair (§2.6): a bend or elbow consumed as
    curved_bend_macro_element with a nonempty bend_pipe_ref, both factors dimensionless ("1" or "none"), finite and
    above zero; the pair as authored. Any other placement of the pair is refused (the producer blocks it)."""
    arcs: dict[str, tuple[float, float]] = {}
    # As the Rust reader reads it: a non-array `components` lists nothing, and an entry or `modifiers` that is
    # not an object carries no pair; G8 binds the invocation's model shape itself.
    components = model.get("components")
    for component in components if isinstance(components, list) else []:
        modifiers = component.get("modifiers") if isinstance(component, Mapping) else None
        modifiers = modifiers if isinstance(modifiers, Mapping) else {}
        ip, op = modifiers.get("sif_in_plane_user_value"), modifiers.get("sif_out_of_plane_user_value")
        if ip is None and op is None:
            continue
        def factor(q: Any) -> float | None:
            value = q.get("value") if isinstance(q, Mapping) else None
            ok = isinstance(value, (int, float)) and not isinstance(value, bool) and math.isfinite(value) and value > 0 and q.get("unit") in ("1", "none")
            return float(value) if ok else None
        realized = component.get("kind") in ("bend", "elbow") and (component.get("mechanics_interface") or {}).get("solver_consumption") == "curved_bend_macro_element"
        pipe = (component.get("geometry") or {}).get("bend_pipe_ref")
        pair = (factor(ip), factor(op))
        _require(realized and isinstance(pipe, str) and pipe.strip() and None not in pair and pipe not in arcs, "directional pair outside a realized arc")
        arcs[pipe] = pair  # type: ignore[assignment]
    return arcs


def directional_model_checks(source: Mapping[str, Any], model: Mapping[str, Any], exact: bool) -> None:
    """Review item 1: in every case every realized arc with the model's pair has its five station rows on every basis,
    records with the model's factors bit for bit, and the maximum row or its named unavailable outcome; directional
    rows or records on any other member are refused."""
    if model.get("components") is None:
        return  # G2's convention: a model without components states nothing about the directional pair
    results = source.get("results") or []
    diagnostics = source.get("diagnostics") or []
    solved = (source.get("status") or {}).get("mechanics") == "MECHANICS_SOLVED"
    arcs = directional_arcs(model) if solved else {}
    for row in results:
        if row.get("kind") in (DIRECTIONAL, DIRECTIONAL_MAX):
            _require(row.get("entity_ref") in arcs, "directional row outside the model's arcs with the pair")
    rows = Rows(results)
    bases = ("nominal_section", "reduced_section") if exact else ("",)
    suffix = {"nominal_section": ":maximum:nominal-section", "reduced_section": ":maximum:reduced-section", "": ":maximum"}
    for case in (source.get("contract_evidence") or {}).get("exact_cases" if exact else "preview_cases") or []:
        cid = case["load_case_id"]
        for record in case.get("intensified_measures") or []:
            if "i_in_plane" not in record and "i_out_of_plane" not in record:
                continue
            pair = arcs.get(record.get("pipe_id"))
            _require(pair is not None, "directional record outside the model's arcs with the pair")
            _require(bits(record["i_in_plane"]) == bits(pair[0]) and bits(record["i_out_of_plane"]) == bits(pair[1]), "directional factors differ from the model")
        for pipe in arcs:
            for basis in bases:
                _require(all(rows.value(cid, pipe, DIRECTIONAL, "intensified_bending", s, basis) is not None for s in STATIONS), "directional station coverage")
                maximum = rows.value(cid, pipe, DIRECTIONAL_MAX, "intensified_bending_maximum", "governing_station", basis) is not None
                unavailable = any(d.get("code") == DIRECTIONAL_MAX_UNAVAILABLE and (d.get("affected_refs") or [None, None])[:2] == [cid, pipe]
                                  and str(d.get("id", "")).endswith(suffix[basis]) for d in diagnostics)
                _require(maximum or unavailable, "directional maximum or its unavailable outcome")


def poisson_exempt(diagnostics: list[Mapping[str, Any]], case: str) -> set[str]:
    return {d["affected_refs"][1] for d in diagnostics if d.get("code") == POISSON_OUT_OF_DOMAIN
            and isinstance(d.get("affected_refs"), list) and len(d["affected_refs"]) >= 2 and d["affected_refs"][0] == case}


def preview_arcs(results: list[Mapping[str, Any]]) -> set[str]:
    return {row.get("entity_ref") for row in results if _case(row) is not None and (row.get("metadata") or {}).get("basis") == PREVIEW_ARC_BASIS}


def branch_model_checks(source: Mapping[str, Any], model: Mapping[str, Any]) -> None:
    """RT-A3 increment 1 (Rust ``u4b_evidence::branch_model_checks``): the branch rows bound to the captured model by
    the producer's association rule. A ``branch``, ``tee`` or ``branch_connection`` component whose
    ``mechanics_interface.solver_consumption`` is absent or ``mechanics_geometry_only`` associates its header pipe with
    its positive finite ``branch_header_sif_user_value`` and its branch pipe with its ``branch_branch_sif_user_value``,
    at each end of that pipe at the component's node, where the pipe is not a realized arc. In every solved case each
    association has its record on both bases with the model's component, side and factor bit for bit, and no other
    branch row or record is published."""
    if model.get("components") is None or (source.get("status") or {}).get("mechanics") != "MECHANICS_SOLVED":
        return  # G2's convention: a model without components states nothing about the branch factors

    def items(value: Any) -> list[Any]:
        return value if isinstance(value, list) else []

    def field(value: Any, *path: str) -> Any:
        for key in path:
            value = value.get(key) if isinstance(value, Mapping) else None
        return value

    def text(value: Any) -> str | None:
        return value if isinstance(value, str) else None

    pipes = items(model.get("pipe_segments"))
    for case in items((source.get("contract_evidence") or {}).get("exact_cases")):
        arcs = {s.get("pipe_id") for s in items(field(case, "pipe_sections"))
                if isinstance(s, Mapping) and s.get("member_kind") == "realized_arc" and isinstance(s.get("pipe_id"), str)}
        # (pipe, location, component) -> (role, pipe, factor bits)
        expected: dict[tuple[str, str, str], tuple[str, str, int]] = {}
        for component in items(model.get("components")):
            consumption = field(component, "mechanics_interface", "solver_consumption")
            consumption = consumption if isinstance(consumption, str) else "mechanics_geometry_only"
            if field(component, "kind") not in BRANCH_COMPONENT_KINDS or consumption != "mechanics_geometry_only":
                continue
            cid, node = text(field(component, "id")), text(field(component, "node"))
            if cid is None or node is None:
                continue
            header = text(field(component, "geometry", "branch_header_pipe_ref"))
            for pipe in pipes:
                pipe_id = text(field(pipe, "id"))
                if pipe_id is None:
                    continue
                if header == pipe_id:
                    role, value = "branch_header", field(component, "modifiers", "branch_header_sif_user_value", "value")
                elif text(field(component, "geometry", "branch_branch_pipe_ref")) == pipe_id:
                    role, value = "branch_branch", field(component, "modifiers", "branch_branch_sif_user_value", "value")
                else:
                    continue
                if not (_number(value) and math.isfinite(value) and value > 0) or pipe_id in arcs:
                    continue
                for location, end in (("end_i", "from"), ("end_j", "to")):
                    if text(field(pipe, end)) == node:
                        expected[pipe_id, location, cid] = (role, pipe_id, bits(value))
        case_id = text(field(case, "load_case_id"))
        _require(case_id is not None, "branch model case")
        seen: set[tuple[str, str, str, str]] = set()
        for record in items(field(case, "intensified_measures")):
            if not (isinstance(record, Mapping) and "factor_role" in record):
                continue
            key = (text(record.get("pipe_id")) or "", text(record.get("location")) or "", text(record.get("component_id")) or "")
            want = expected.get(key)
            _require(want is not None, "branch record outside the model's tee associations")
            sif = record.get("sif")
            _require(record.get("factor_role") == want[0] and _number(sif) and bits(sif) == want[2], "branch factors differ from the model")
            seen.add((*key, text(record.get("section_basis")) or ""))
        for key in expected:
            for basis in ("nominal_section", "reduced_section"):
                _require((*key, basis) in seen, "branch record coverage")
        _require(len(seen) == 2 * len(expected), "branch record outside the model's tee associations")
        associated = {want[1] for want in expected.values()}
        for row in items(source.get("results")):
            if isinstance(row, Mapping) and row.get("kind") == BRANCH and field(row, "basis_ref", "ref_id") == case_id:
                _require(text(row.get("entity_ref")) in associated, "branch row outside the model's tee associations")
