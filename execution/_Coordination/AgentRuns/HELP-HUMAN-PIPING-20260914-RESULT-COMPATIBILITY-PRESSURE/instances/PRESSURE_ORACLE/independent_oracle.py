#!/usr/bin/env python3
"""Frozen pressure oracle: stdlib only; never imports product or solver code.

Uses exact rational checks and Decimal boundary-traction reconstruction. JSON
is an adapter-neutral expected-value protocol; a later product adapter supplies
the actual scalar output names below. Default execution verifies the oracle
itself and writes nothing. --emit writes generated evidence only beside this file.
"""
from __future__ import annotations

import argparse
from decimal import Decimal as D, localcontext
from fractions import Fraction as F
import hashlib
import json
import math
from pathlib import Path
import random
import sys

HERE = Path(__file__).resolve().parent
PRECISION = 110
PI = D("3.14159265358979323846264338327950288419716939937510582097494459230781640628620899862803482534211706798214808651")
with localcontext() as _constant_context:
    _constant_context.prec = PRECISION
    EPSILON = D(2) ** -52
    MIN_SUBNORMAL = D(2) ** -1074
EPS_FACTOR = D(128)
SUBNORMAL_ULPS = D(8)
SEED = 20260914


def dec(x):
    if isinstance(x, float):
        return D.from_float(x)
    if isinstance(x, F):
        return D(x.numerator) / D(x.denominator)
    return D(x)


def rational_oracles():
    # Work in forces/pi, areas/pi. Reconstruct Lamé coefficients from
    # two traction constraints, then apply three-dimensional axial Hooke law.
    ri2, ro2, pressure, young, nu = F(1), F(4), F(3), F(120), F(1, 4)
    wall_area_pi, fluid_area_pi = ro2 - ri2, ri2
    b = pressure / (1 / ri2 - 1 / ro2)
    a = b / ro2
    assert (a, b) == (1, 4)
    assert (a - b / ri2, a - b / ro2, a + b / ri2, a + b / ro2) == (-3, 0, 5, 2)
    assert young / (2 * (1 + nu)) == 48
    specifications = [
        ("P1_FREE_TRANSFER", F(0), F(1, 240), F(3), F(0), F(1), F(3, 2)),
        ("P2_RESTRAINED_TRANSFER", F(0), F(0), F(3, 2), F(-3, 2), F(1, 2), F(3, 2)),
        ("P3_FREE_SEPARATE_CLOSURES", F(0), F(-1, 240), F(0), F(-3), F(0), F(3, 2)),
        ("P4_FREE_THERMAL_PRESSURE", F(1, 1000), F(31, 6000), F(3), F(0), F(1), F(57, 50)),
        ("P4_RESTRAINED_THERMAL_PRESSURE", F(1, 1000), F(0), F(57, 50), F(-93, 50), F(19, 50), F(57, 50)),
    ]
    rows = []
    for name, thermal, strain, wall, effective, stress, eigen_i in specifications:
        sigma = young * (strain - thermal) + nu * (2 * a)
        wall_got = wall_area_pi * sigma
        effective_got = wall_got - pressure * fluid_area_pi
        eigen_got = wall_area_pi * (nu * 2 * a - young * thermal)
        assert (wall_got, effective_got, sigma, eigen_got) == (wall, effective, stress, eigen_i), name
        q = (-wall_got, wall_got)
        kd = (-young * wall_area_pi * strain, young * wall_area_pi * strain)
        eigen = (eigen_got, -eigen_got)
        assert tuple(k - f for k, f in zip(kd, eigen)) == q
        rows.append({"id": name, "thermal_strain_exact": str(thermal), "epsilon_z_exact": str(strain),
                     "wall_force_over_pi": str(wall), "effective_force_over_pi": str(effective),
                     "axial_membrane_pa": str(stress), "eigen_i_over_pi": str(eigen_i),
                     "wall_endpoint_over_pi": [str(-wall), str(wall)]})
    return rows


def expected_values(inputs):
    """Different derivation/order from the accepted kernel formulas.

    Solve the two radial traction equations for Lamé's constants. Obtain the
    transverse trace at an interior witness, then use isotropic Hooke law for
    the axial stress. Integrate the uniform axial stress over the annulus.
    Binary64 inputs are promoted exactly; pi is independently high precision.
    """
    with localcontext() as ctx:
        ctx.prec = PRECISION
        ri, ro, e, nu, p, thermal, strain = [dec(inputs[k]) for k in
            ("ri_m", "ro_m", "E_pa", "nu", "p_pa", "thermal_strain", "epsilon_z")]
        inner_inv, outer_inv = 1 / (ri * ri), 1 / (ro * ro)
        c = p / (inner_inv - outer_inv)
        a = c * outer_inv
        ai = PI * ri * ri
        area = PI * (ro * ro - ri * ri)
        witness = (ri + ro) / 2
        sr_witness, sh_witness = a - c / witness**2, a + c / witness**2
        trace = sr_witness + sh_witness
        membrane = e * (strain - thermal) + nu * trace
        wall = membrane * area
        fluid = p * ai
        eigen_i = area * (nu * trace - e * thermal)
        values = {
            "Ai_m2": ai, "As_m2": area, "G_pa": e / (2 * (1 + nu)),
            "wall_force_n": wall, "effective_force_n": wall - fluid,
            "axial_membrane_pa": membrane,
            "eigen_i_n": eigen_i, "eigen_j_n": -eigen_i,
            "cap_i_n": -fluid, "cap_j_n": fluid,
            "inner_radial_pa": -p, "outer_radial_pa": D(0),
            "inner_hoop_pa": a + c * inner_inv,
            "outer_hoop_pa": a + c * outer_inv,
        }
        stress_scale = abs(e * strain) + abs(e * thermal) + abs(nu * trace)
        wall_scale = area * stress_scale
        scales = {
            "Ai_m2": abs(ai), "As_m2": abs(area), "G_pa": abs(values["G_pa"]),
            "wall_force_n": wall_scale, "effective_force_n": wall_scale + abs(fluid),
            "axial_membrane_pa": stress_scale,
            "eigen_i_n": area * (abs(nu * trace) + abs(e * thermal)),
            "eigen_j_n": area * (abs(nu * trace) + abs(e * thermal)),
            "cap_i_n": abs(fluid), "cap_j_n": abs(fluid),
            # Strong traction checks: cancellation in A-B/r^2 cannot enlarge
            # the permissible boundary residual to the hoop stress magnitude.
            "inner_radial_pa": abs(p), "outer_radial_pa": abs(p),
            "inner_hoop_pa": abs(a) + abs(c * inner_inv),
            "outer_hoop_pa": abs(a) + abs(c * outer_inv),
        }
        for label, r_input in inputs.get("radius_witnesses", {}).items():
            r = dec(r_input)
            radial, hoop = a - c / (r * r), a + c / (r * r)
            values[f"{label}_radial_pa"] = radial
            values[f"{label}_hoop_pa"] = hoop
            scales[f"{label}_radial_pa"] = abs(a) + abs(c / (r * r))
            scales[f"{label}_hoop_pa"] = abs(a) + abs(c / (r * r))
        checks = {}
        for key, value in values.items():
            scale = max(abs(value), scales[key])
            limit = EPS_FACTOR * EPSILON * scale + SUBNORMAL_ULPS * MIN_SUBNORMAL
            checks[key] = {"expected_decimal": str(value), "expected_f64": float(value),
                           "scale_decimal": str(scale), "absolute_limit_decimal": str(limit)}
        return checks


def fixtures():
    base = dict(ri_m=1., ro_m=2., E_pa=120., nu=.25, p_pa=3., thermal_strain=0., epsilon_z=0.)
    cases = []
    rational = rational_oracles()
    for row in rational:
        inp = dict(base, thermal_strain=float(F(row["thermal_strain_exact"])),
                   epsilon_z=float(F(row["epsilon_z_exact"])), radius_witnesses={"midwall": 1.5})
        cases.append({"id": row["id"], "inputs": inp, "checks": expected_values(inp)})
    specials = [
        ("ZERO_ALL_LOADS", dict(p_pa=0., epsilon_z=0., thermal_strain=0.)),
        ("THERMAL_ONLY", dict(p_pa=0., thermal_strain=1 / 1024, epsilon_z=0.)),
        ("NU_ZERO", dict(nu=0., thermal_strain=0., epsilon_z=0.)),
        ("PRESSURE_REVERSAL", dict(p_pa=-3.)),
        ("THERMAL_REVERSAL", dict(p_pa=0., thermal_strain=-1 / 1024)),
        ("AUXETIC", dict(nu=-.5)),
        ("NU_INSIDE_UPPER_BOUND", dict(nu=math.nextafter(.5, 0.))),
        ("NU_INSIDE_LOWER_BOUND", dict(E_pa=1e-10, nu=math.nextafter(-1., 0.))),
        ("ADJACENT_RADII", dict(ro_m=math.nextafter(1., math.inf))),
        ("THIN_DYADIC_WALL", dict(ro_m=1. + 2**-30, p_pa=-3.)),
        ("SMALL_GEOMETRY", dict(ri_m=2**-200, ro_m=2**-199, E_pa=2**200, p_pa=2**100)),
        ("LARGE_GEOMETRY", dict(ri_m=2**200, ro_m=2**201, E_pa=2**-200, p_pa=2**-100)),
        ("SUBNORMAL_PRESSURE", dict(p_pa=math.ulp(0.))),
    ]
    for name, changes in specials:
        inp = dict(base, **changes)
        cases.append({"id": name, "inputs": inp, "checks": expected_values(inp)})
    rng = random.Random(SEED)
    for index in range(96):
        ri = math.ldexp(rng.randint(1, 16), rng.randint(-20, 12))
        ro = ri * (1 + math.ldexp(rng.randint(1, 8), -4))
        inp = dict(ri_m=ri, ro_m=ro, E_pa=math.ldexp(rng.randint(1, 16), rng.randint(-5, 35)),
                   nu=rng.randint(-24, 15) / 32, p_pa=math.ldexp(rng.randint(-16, 16), rng.randint(-10, 20)),
                   thermal_strain=rng.randint(-16, 16) / 2**18, epsilon_z=rng.randint(-16, 16) / 2**18,
                   radius_witnesses={"quarterwall": ri + (ro - ri) / 4, "midwall": (ri + ro) / 2})
        cases.append({"id": f"DYADIC_{index:03d}", "inputs": inp, "checks": expected_values(inp)})
    return {"format": "private-pressure-oracle-v1", "precision_digits": PRECISION,
            "seed": SEED, "comparison": {"epsilon_multiplier": 128, "subnormal_ulp_floor": 8,
            "equation": "abs(actual - expected_decimal) <= 128 * 2^-52 * declared_scale + 8 * 2^-1074",
            "scope": "floating-point arithmetic only; no engineering acceptance threshold"},
            "rational_cases": rational, "cases": cases}


def compare_scalar(actual, check):
    if isinstance(actual, bool) or not isinstance(actual, (int, float)) or not math.isfinite(actual):
        return False
    with localcontext() as ctx:
        ctx.prec = PRECISION
        return abs(dec(actual) - D(check["expected_decimal"])) <= D(check["absolute_limit_decimal"])


def compare_actuals(actuals, expected):
    errors = []
    ids = {case["id"] for case in expected["cases"]}
    if set(actuals) != ids:
        errors.append({"case_ids": {"missing": sorted(ids - set(actuals)), "extra": sorted(set(actuals) - ids)}})
    for case in expected["cases"]:
        got = actuals.get(case["id"], {})
        for key, check in case["checks"].items():
            if key not in got or not compare_scalar(got[key], check):
                errors.append({"id": case["id"], "quantity": key, "actual": got.get(key), "expected": check})
    return errors


def mutation_witnesses(expected):
    by_id = {case["id"]: case for case in expected["cases"]}
    c = by_id["P2_RESTRAINED_TRANSFER"]["checks"]
    p = math.pi * 3
    mutations = [
        ("omit_2nuP", "wall_force_n", 0.),
        ("double_2nuP", "wall_force_n", p),
        ("flip_Poisson_sign", "wall_force_n", -p / 2),
        ("subtract_cap_from_wall", "wall_force_n", -p / 2),
        ("mean_radius_fluid_area", "cap_j_n", 3 * math.pi * 1.5**2),
        ("double_longitudinal_stress", "axial_membrane_pa", 1.5),
        ("omit_fluid_effective_term", "effective_force_n", p / 2),
        ("double_cap", "cap_j_n", 2 * p),
        ("flip_eigen_sign", "eigen_i_n", -p / 2),
    ]
    out = []
    for name, field, wrong in mutations:
        killed = not compare_scalar(wrong, c[field])
        assert killed, name
        out.append({"mutation": name, "case": "P2_RESTRAINED_TRANSFER", "field": field,
                    "mutant_value": wrong, "killed": killed})
    # Material interpolation is upstream, but an independently supplied selected
    # midpoint pair must derive the value 2160/31, never interpolated endpoint G.
    mid = expected_values(dict(ri_m=1., ro_m=2., E_pa=180., nu=7 / 24,
                              p_pa=3., thermal_strain=0., epsilon_z=0.))["G_pa"]
    assert not compare_scalar(69., mid)
    out.append({"mutation": "independent_G_interpolation", "selected_pair": [180., 7 / 24],
                "exact_G": "2160/31", "mutant_value": 69., "killed": True,
                "qualification": "selected pure material pair only; interpolation dispatch deferred"})
    assert not compare_scalar(0., c["inner_radial_pa"])
    thin = by_id["ADJACENT_RADII"]["checks"]
    assert not compare_scalar(0., thin["inner_radial_pa"])
    out.append({"mutation": "thin_wall_boundary_cancellation_to_zero", "case": "ADJACENT_RADII",
                "field": "inner_radial_pa", "mutant_value": 0., "killed": True})
    return out


def self_check(expected):
    all_actuals = {c["id"]: {k: v["expected_f64"] for k, v in c["checks"].items()} for c in expected["cases"]}
    assert not compare_actuals(all_actuals, expected)
    checks = 0
    superposition_checks = 0
    for c in expected["cases"]:
        inp = c["inputs"]
        rev = dict(inp, p_pa=-inp["p_pa"], thermal_strain=-inp["thermal_strain"], epsilon_z=-inp["epsilon_z"])
        reverse = expected_values(rev)
        for key in c["checks"]:
            if key in ("Ai_m2", "As_m2", "G_pa"):
                continue
            assert compare_scalar(-reverse[key]["expected_f64"], c["checks"][key]), (c["id"], key)
            checks += 1
        for factor in (-2., 0., 2.):
            scaled = dict(inp, p_pa=factor * inp["p_pa"], thermal_strain=factor * inp["thermal_strain"], epsilon_z=factor * inp["epsilon_z"])
            scale_results = expected_values(scaled)
            for key in c["checks"]:
                if key in ("Ai_m2", "As_m2", "G_pa"):
                    continue
                # The zero case and possible binary64 subnormal rounding are
                # compared against the transformed oracle's declared scale.
                assert compare_scalar(factor * c["checks"][key]["expected_f64"], scale_results[key]), (c["id"], key)
                checks += 1
        parts = []
        for active in ("p_pa", "thermal_strain", "epsilon_z"):
            part = dict(inp, p_pa=0., thermal_strain=0., epsilon_z=0.)
            part[active] = inp[active]
            parts.append(expected_values(part))
        for key, check in c["checks"].items():
            if key in ("Ai_m2", "As_m2", "G_pa"):
                continue
            total = math.fsum(part[key]["expected_f64"] for part in parts)
            assert compare_scalar(total, check), (c["id"], key, "superposition")
            superposition_checks += 1
    mutations = mutation_witnesses(expected)
    return {"status": "PASS", "scope": "independent oracle self-verification; no production execution",
            "scalar_case_count": len(expected["cases"]), "rational_state_count": len(expected["rational_cases"]),
            "metamorphic_scalar_checks": checks, "mutations": mutations,
            "superposition_scalar_checks": superposition_checks,
            "python_version": sys.version, "oracle_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--emit", action="store_true", help="Write generated frozen fixtures and self-check output beside script")
    parser.add_argument("--check-actuals", type=Path, help="Read adapter-neutral JSON mapping case id to scalar quantity values")
    args = parser.parse_args()
    expected = fixtures()
    report = self_check(expected)
    if args.check_actuals:
        errors = compare_actuals(json.loads(args.check_actuals.read_text()), expected)
        report["production_comparison"] = {"status": "FAIL" if errors else "PASS", "errors": errors}
        print(json.dumps(report, indent=2, allow_nan=False))
        return 1 if errors else 0
    if args.emit:
        (HERE / "FROZEN_EXPECTATIONS.json").write_text(json.dumps(expected, indent=2, allow_nan=False) + "\n")
        (HERE / "ORACLE_SELF_CHECK.json").write_text(json.dumps(report, indent=2, allow_nan=False) + "\n")
    print(json.dumps(report, indent=2, allow_nan=False))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
