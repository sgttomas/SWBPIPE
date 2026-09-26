#!/usr/bin/env python3
"""Non-author recomputation of reference_cases.json from its own stated inputs.

Written by the session-2 WORKING_ITEMS manager, who did not author the fixture
or its generator. Imports neither the generator nor any product/solver code.
Each expected exact value is re-derived here from first principles (partitioned
equilibrium, Euler-Bernoulli fixed-fixed stiffness, free-length ratio,
trapezoid integration, natural-length composition) and compared exactly; each
binary64 `value` is checked to be the correctly rounded projection of `exact`.
"""
from decimal import Decimal, getcontext
from fractions import Fraction as F
import json
import math
import sys

getcontext().prec = 85
PATH = sys.argv[1]
doc = json.load(open(PATH))
PI = Decimal(doc["numeric_representation"]["pi_decimal"])
failures = []
checks = 0


def ex(q):
    e = q["exact"]
    kind = e["kind"]
    if kind == "rational":
        return ("r", F(e["rational"]))
    if kind == "rational_times_pi":
        return ("pi", F(e["rational"]))
    if kind == "rational_over_pi":
        return ("over_pi", F(e["rational"]))
    return (kind, e)


def to_decimal(kind, r):
    d = Decimal(r.numerator) / Decimal(r.denominator)
    if kind == "pi":
        return d * PI
    if kind == "over_pi":
        return d / PI
    return d


def check(name, q, kind, value):
    """Compare fixture quantity with independently derived (kind, Fraction)."""
    global checks
    checks += 1
    k, r = ex(q)
    if (k, r) != (kind, value):
        failures.append(f"{name}: fixture {k}:{r} != independent {kind}:{value}")
        return
    projected = float(to_decimal(kind, value))
    if q["value"] != projected:
        failures.append(f"{name}: value {q['value']!r} is not rounded projection {projected!r}")


def inp(q):
    k, r = ex(q)
    assert k == "r", q
    return r


def check_float(name, q, expected):
    global checks
    checks += 1
    if q["exact"]["kind"] not in ("symbolic", "decimal") and "decimal" not in q:
        failures.append(f"{name}: unexpected kind {q['exact']}")
    if abs(q["value"] - float(expected)) > abs(float(expected)) * 1e-15:
        failures.append(f"{name}: {q['value']!r} vs {expected}")


geom = doc["geometry"]
od = inp(geom["authored"]["outside_diameter"])
t = inp(geom["authored"]["wall_thickness"])
ro, ri = od / 2, od / 2 - t
As = ro * ro - ri * ri  # times pi
I = (ro**4 - ri**4) / 4  # times pi
check("geometry.As", geom["derived"]["As"], "pi", As)
check("geometry.Ai", geom["derived"]["Ai"], "pi", ri * ri)
check("geometry.I", geom["derived"]["I"], "pi", I)
check("geometry.J", geom["derived"]["J"], "pi", 2 * I)
check("geometry.Z", geom["derived"]["Z"], "pi", I / ro)
# Independent identity As = t*(OD-t)
assert As == t * (od - t)

cases = doc["cases"]


def area_of(variant):
    inputs = variant["inputs"]
    if "area" in inputs:
        return ("r", inp(inputs["area"]))
    assert inputs["geometry_ref"] == geom["id"]
    return ("pi", As)


def scaled(kind, r):
    return (kind, r)


# 1. two-bar prescribed translation by partitioned equilibrium K_ff u_f = -K_fc g_c.
for vname, v in cases["prescribed_translation_two_bar"]["variants"].items():
    ak, a = area_of(v)
    i = v["inputs"]
    k1 = inp(i["E1"]) * a / inp(i["L1"])
    k2 = inp(i["E2"]) * a / inp(i["L2"])
    g0, g2 = inp(i["root_UX"]), inp(i["far_UX"])
    rhs = k1 * g0 + k2 * g2
    u1 = rhs / (k1 + k2)
    n1 = k1 * (u1 - g0)
    n2 = k2 * (g2 - u1)
    e = v["expected"]
    check(f"two_bar.{vname}.middle_UX", e["middle_UX"], "r", u1)
    check(f"two_bar.{vname}.member1_N", e["member1_N"], ak, n1)
    check(f"two_bar.{vname}.member2_N", e["member2_N"], ak, n2)
    check(f"two_bar.{vname}.root_Fx", e["root_Fx"], ak, -n1)
    check(f"two_bar.{vname}.far_Fx", e["far_Fx"], ak, n2)
    check(f"two_bar.{vname}.rhs", e["free_rhs_after_prescribed_coupling"], ak, rhs)
    # Original free equation residual when K_fc g_c is omitted.
    assert (k1 + k2) * 0 - rhs == -k1 * g0

for vname, v in cases["prescribed_translation_all_fixed"]["variants"].items():
    ak, a = area_of(v)
    i = v["inputs"]
    n = inp(i["E"]) * a * (inp(i["far_UX"]) - inp(i["root_UX"])) / inp(i["L"])
    e = v["expected"]
    check(f"all_fixed.{vname}.N", e["N"], ak, n)
    check(f"all_fixed.{vname}.root_Fx", e["root_Fx"], ak, -n)
    check(f"all_fixed.{vname}.far_Fx", e["far_Fx"], ak, n)

# 2. rotation: fixed-fixed Euler-Bernoulli stiffness rows [12,6L,-12,6L; 6L,4L2,-6L,2L2]*EI/L^3
for vname, v in cases["prescribed_rotation_all_fixed"]["variants"].items():
    i = v["inputs"]
    L = inp(i["L"])
    th = inp(i["root_RZ"])
    if "EI" in i and i["EI"]["exact"]["kind"] == "rational":
        ek, EI = "r", inp(i["EI"])
    else:
        ek, EI = "pi", inp(i["E"]) * I
        check(f"rot_fixed.{vname}.EI", i["EI"], "pi", EI)
    c = EI / L**3
    # member end forces from d=[0, th, 0, 0]
    fyi, mzi, fyj, mzj = c * 6 * L * th, c * 4 * L * L * th, -c * 6 * L * th, c * 2 * L * L * th
    e = v["expected"]
    check(f"rot_fixed.{vname}.root_Fy", e["root_Fy"], ek, fyi)
    check(f"rot_fixed.{vname}.root_Mz", e["root_Mz"], ek, mzi)
    check(f"rot_fixed.{vname}.far_Fy", e["far_Fy"], ek, fyj)
    check(f"rot_fixed.{vname}.far_Mz", e["far_Mz"], ek, mzj)
    assert fyi + fyj == 0 and mzi + mzj + fyj * L == 0  # force and root-moment balance

for vname, v in cases["prescribed_rotation_free_tip"]["variants"].items():
    i = v["inputs"]
    L, th = inp(i["L"]), inp(i["root_RZ"])
    # free tip: solve K_ff [v2, th2] = -K_fc [0, th]; rigid rotation results.
    # EI/L^3 cancels. K_jj = [[12,-6L],[-6L,4L^2]]; K_ji = [[-12,-6L],[6L,2L^2]].
    k = [[12, -6 * L], [-6 * L, 4 * L * L]]
    b = [-(-6 * L * th), -(2 * L * L * th)]  # -K_ji [0, th]
    det = k[0][0] * k[1][1] - k[0][1] * k[1][0]
    v2 = (b[0] * k[1][1] - k[0][1] * b[1]) / det
    t2 = (k[0][0] * b[1] - k[1][0] * b[0]) / det
    e = v["expected"]
    check(f"rot_free.{vname}.tip_UY", e["tip_UY"], "r", v2)
    check(f"rot_free.{vname}.tip_RZ", e["tip_RZ"], "r", t2)
    fyi = 6 * L * th - 12 * v2 + 6 * L * t2
    mzi = 4 * L * L * th - 6 * L * v2 + 2 * L * L * t2
    check(f"rot_free.{vname}.root_Fy", e["root_Fy"], "r", fyi)
    check(f"rot_free.{vname}.root_Mz", e["root_Mz"], "r", mzi)
    check(f"rot_free.{vname}.wall_N", e["wall_N"], "r", F(0))
    w = cases["prescribed_rotation_free_tip"]["wrong_result_discriminators"]
    # Treating the entered angle as angle/L: tip UY = L*(th/L), tip RZ = th/L.
    check(f"rot_free.wrong_len_norm_tipUY", w["length_normalized_angle"]["wrong_tip_UY"], "r", L * (th / L))
    check(f"rot_free.wrong_len_norm_tipRZ", w["length_normalized_angle"]["wrong_tip_RZ"], "r", th / L)

# 3. parallel shared material: N_c + N_h = 0 with N = E A (u/L - eps)
for vname, v in cases["shared_material_parallel"]["variants"].items():
    ak, a = area_of(v)
    i = v["inputs"]
    L = inp(i["member_length"])
    Ec, ec = inp(i["cold"]["E"]), inp(i["cold"]["thermal_strain"])
    Eh, eh = inp(i["hot"]["E"]), inp(i["hot"]["thermal_strain"])
    u = L * (Ec * ec + Eh * eh) / (Ec + Eh)
    e = v["expected"]
    check(f"parallel.{vname}.tip_UX", e["tip_UX"], "r", u)
    check(f"parallel.{vname}.cold_N", e["cold_N"], ak, Ec * a * (u / L - ec))
    check(f"parallel.{vname}.hot_N", e["hot_N"], ak, Eh * a * (u / L - eh))
    check(f"parallel.{vname}.total", e["total_root_Fx"], "r", F(0))
    wrong = L * (Ec * ec + Ec * eh) / (Ec + Ec)
    check("parallel.wrong_id_only", cases["shared_material_parallel"]["wrong_result_discriminators"]["material_ID_only_E_override"]["wrong_tip_UX"], "r", wrong)

for vname, v in cases["shared_material_serial_companion"]["variants"].items():
    ak, a = area_of(v)
    i = v["inputs"]
    L1, L2, E1, E2 = inp(i["L1"]), inp(i["L2"]), inp(i["E1"]), inp(i["E2"])
    e1, e2 = inp(i["epsilon1"]), inp(i["epsilon2"])
    # both ends fixed at zero: sum of elongations zero; N common.
    flex = L1 / E1 + L2 / E2  # per area
    n_over_a = -(e1 * L1 + e2 * L2) / flex
    u1 = n_over_a * L1 / E1 + e1 * L1
    e = v["expected"]
    check(f"serial.{vname}.middle_UX", e["middle_UX"], "r", u1)
    check(f"serial.{vname}.member1_N", e["member1_N"], ak, n_over_a * a)
    check(f"serial.{vname}.member2_N", e["member2_N"], ak, n_over_a * a)
    check(f"serial.{vname}.root_Fx", e["root_Fx"], ak, -n_over_a * a)
    check(f"serial.{vname}.far_Fx", e["far_Fx"], ak, n_over_a * a)
    if "nu1" in i:
        check(f"serial.{vname}.G1", e["G1"], "r", E1 / (2 * (1 + inp(i["nu1"]))))
        check(f"serial.{vname}.G2", e["G2"], "r", E2 / (2 * (1 + inp(i["nu2"]))))
    flexw = L1 / E1 + L2 / E1
    wrong = -(e1 * L1 + e2 * L2) / flexw * L1 / E1 + e1 * L1
    check("serial.wrong_id_only", cases["shared_material_serial_companion"]["wrong_result_discriminators"]["material_ID_only_E_override"]["wrong_middle_UX"], "r", wrong)

# 4. thermal datum ratio from piecewise-linear secant coefficient table.
def kelvin(q):
    assert q["unit"] == "degC", q
    return inp(q) + F(27315, 100)


def interp(points, T):
    for (t0, a0), (t1, a1) in zip(points, points[1:]):
        if t0 <= T <= t1:
            return a0 + (a1 - a0) * (T - t0) / (t1 - t0)
    raise ValueError("outside")


for vname, v in cases["thermal_datum_ratio"]["variants"].items():
    i = v["inputs"]
    Tm, Ti, To = kelvin(i["datum_temperature"]), kelvin(i["installation_temperature"]), kelvin(i["operating_temperature"])
    pts = [(kelvin(p["temperature"]), inp(p["alpha"])) for p in i["table_points"]]
    ai, ao = interp(pts, Ti), interp(pts, To)
    assert ai == inp(i["alpha_install"]) and ao == inp(i["alpha_operating"])
    di, do = ai * (Ti - Tm), ao * (To - Tm)
    eps = (1 + do) / (1 + di) - 1
    e = v["expected"]
    check(f"datum.{vname}.dilation_install", e["dilation_install"], "r", di)
    check(f"datum.{vname}.dilation_operating", e["dilation_operating"], "r", do)
    check(f"datum.{vname}.stretch", e["thermal_stretch"], "r", 1 + eps)
    check(f"datum.{vname}.strain", e["thermal_strain"], "r", eps)
    check(f"datum.{vname}.Tm", e["datum_temperature_K"], "r", Tm)
    check(f"datum.{vname}.Ti", e["installation_temperature_K"], "r", Ti)
    check(f"datum.{vname}.To", e["operating_temperature_K"], "r", To)
    if "fixed_wall_N" in e:
        L, E = inp(i["L"]), inp(i["E"])
        check(f"datum.{vname}.free_tip", e["free_tip_UX"], "r", L * eps)
        check(f"datum.{vname}.free_N", e["free_wall_N"], "r", F(0))
        check(f"datum.{vname}.fixed_N", e["fixed_wall_N"], "pi", -E * As * eps)
        check(f"datum.{vname}.fixed_root", e["fixed_root_Fx"], "pi", E * As * eps)
        check(f"datum.{vname}.fixed_far", e["fixed_far_Fx"], "pi", -E * As * eps)
w = cases["thermal_datum_ratio"]["wrong_result_discriminators"]
check("datum.wrong_hot_alpha", w["alpha_hot_times_operating_minus_install"], "r", F(16, 1000000) * 100)
check("datum.wrong_subtract", w["subtract_datum_dilations"], "r", F(13, 6250) - F(9, 25000))

# 5. coefficient definitions: exact trapezoid integral; log uses exp.
v = cases["coefficient_definition"]["variants"]["generic_reviewed"]
i = v["inputs"]
pts = [(kelvin(p["temperature"]), inp(p["coefficient"])) for p in i["linear_coefficient_points"]]


def integ(a, b):
    lo, hi, s = (a, b, 1) if a <= b else (b, a, -1)
    return s * (interp(pts, lo) + interp(pts, hi)) * (hi - lo) / 2


Tm, Ti, To = kelvin(i["datum_temperature"]), kelvin(i["installation_temperature"]), kelvin(i["operating_temperature"])
Th = (Ti + To) / 2
e = v["expected"]
check("coef.integral", e["integral"], "r", integ(Ti, To))
check("coef.first_half", e["first_half_integral"], "r", integ(Ti, Th))
check("coef.second_half", e["second_half_integral"], "r", integ(Th, To))
lam = lambda T: 1 + integ(Tm, T)
check("coef.datum_fwd", e["datum_length_strain_forward"], "r", lam(To) / lam(Ti) - 1)
check("coef.datum_rev", e["datum_length_strain_reverse"], "r", lam(Ti) / lam(To) - 1)
check("coef.datum_first", e["datum_length_strain_first_half"], "r", lam(Th) / lam(Ti) - 1)
check("coef.datum_second", e["datum_length_strain_second_half"], "r", lam(To) / lam(Th) - 1)
dexp = lambda r: (Decimal(r.numerator) / Decimal(r.denominator)).exp() - 1
check_float("coef.log_fwd", e["current_length_strain_forward"], dexp(integ(Ti, To)))
check_float("coef.log_rev", e["current_length_strain_reverse"], dexp(integ(To, Ti)))
check_float("coef.log_first", e["current_length_strain_first_half"], dexp(integ(Ti, Th)))
check_float("coef.log_second", e["current_length_strain_second_half"], dexp(integ(Th, To)))
# split composition identity for both definitions
assert (lam(Th) / lam(Ti)) * (lam(To) / lam(Th)) == lam(To) / lam(Ti)
w = cases["coefficient_definition"]["wrong_result_discriminators"]
check("coef.wrong_endpoint", w["endpoint_alpha_times_interval"], "r", interp(pts, To) * (To - Ti))

v = cases["constant_alpha_interval"]["variants"]["generic_reviewed"]
eps = inp(v["inputs"]["alpha"]) * inp(v["inputs"]["deltaT"])
check("constant.strain", v["expected"]["thermal_strain"], "r", eps)
check("constant.stretch", v["expected"]["thermal_stretch"], "r", 1 + eps)

# 6. signed fit composition.
for vname, v in cases["signed_fit_states"]["variants"].items():
    ak, a = area_of(v)
    i = v["inputs"]
    L = inp(i["L"])
    fit = inp(i["signed_fit_length_change"]) / L
    alpha = inp(i["alpha"])
    Tm = kelvin(i["coefficient_datum_temperature"])
    Tinst = kelvin(i["installation_temperature"])
    Thot = kelvin(i["hot_temperature"])
    for state, T, E in [("cold", Tinst, inp(i["cold_E"])), ("hot", Thot, inp(i["hot_E"])), ("return", Tinst, inp(i["cold_E"]))]:
        eth = (1 + alpha * (T - Tm)) / (1 + alpha * (Tinst - Tm)) - 1
        star = (1 + fit) * (1 + eth) - 1
        n = -E * a * star
        e = v["expected"][state]
        check(f"fit.{vname}.{state}.E", e["E"], "r", E)
        check(f"fit.{vname}.{state}.eth", e["thermal_strain"], "r", eth)
        check(f"fit.{vname}.{state}.fit", e["fit_strain"], "r", fit)
        check(f"fit.{vname}.{state}.star", e["total_eigenstrain"], "r", star)
        check(f"fit.{vname}.{state}.N", e["fixed_wall_N"], ak, n)
        check(f"fit.{vname}.{state}.root", e["fixed_root_Fx"], ak, -n)
        check(f"fit.{vname}.{state}.far", e["fixed_far_Fx"], ak, n)
        check(f"fit.{vname}.{state}.released_tip", e["released_tip_UX"], "r", L * star)
        check(f"fit.{vname}.{state}.released_N", e["released_wall_N"], "r", F(0))
    b = v["baselines"]
    eth_hot = alpha * (Thot - Tinst)
    check(f"fit.{vname}.no_fit_cold", b["no_fit_cold_N"], "r", F(0))
    check(f"fit.{vname}.no_fit_hot", b["no_fit_hot_N"], ak, -inp(i["hot_E"]) * a * eth_hot)
    check(f"fit.{vname}.cut_long", b["cut_long_cold_N"], ak, -inp(i["cold_E"]) * a * (-fit))
w = cases["signed_fit_states"]["wrong_result_discriminators"]
check("fit.wrong_additive", w["hot_additive_strains"]["wrong_generic_N"], "r", -F(150000000000) * F(1, 1000) * (F(-1, 5000) + F(1, 1250)))
check("fit.wrong_coldE", w["hot_uses_cold_E"]["wrong_generic_N"], "r", -F(200000000000) * F(1, 1000) * F(3749, 6250000))
check("fit.wrong_twice", w["fit_applied_twice"]["wrong_cold_generic_N"], "r", -F(200000000000) * F(1, 1000) * 2 * F(-1, 5000))
check("fit.wrong_sign", w["sign_flipped"]["wrong_cold_generic_N"], "r", -F(200000000000) * F(1, 1000) * F(1, 5000))

# 9. persistent source once.
for vname, v in cases["persistent_source_once"]["variants"].items():
    i = v["inputs"]
    p, wgt, ind = inp(i["preload"]), inp(i["weight"]), inp(i["independent_action"])
    if i["structure_stiffness"]["exact"]["kind"] == "rational":
        kk, k = "r", inp(i["structure_stiffness"])
        dk = "r"
    else:
        kk, k = "pi", inp(i["E"]) * As / inp(i["L"])
        check(f"source.{vname}.k", i["structure_stiffness"], "pi", k)
        dk = "over_pi"
    e = v["expected"]
    rhs = p + wgt + ind
    check(f"source.{vname}.rhs", e["combined_rhs"], "r", rhs)
    check(f"source.{vname}.u", e["combined_displacement"], dk, rhs / k)
    check(f"source.{vname}.root", e["root_Fx"], "r", -rhs)
    check(f"source.{vname}.w_total", e["weight_total_rhs"], "r", p + wgt)
    check(f"source.{vname}.a_total", e["additional_total_rhs"], "r", p + ind)
    check(f"source.{vname}.naive", e["naive_total_sum_rhs"], "r", 2 * p + wgt + ind)
    check(f"source.{vname}.naive_u", e["naive_total_sum_displacement"], dk, (2 * p + wgt + ind) / k)
    check(f"source.{vname}.two_equal", e["two_distinct_equal_actions_rhs"], "r", ind + ind)
    check(f"source.{vname}.excluded", e["excluded_unreferenced_action_rhs"], "r", rhs)

print(json.dumps({"checks": checks, "failures": failures}, indent=1))
sys.exit(1 if failures else 0)
