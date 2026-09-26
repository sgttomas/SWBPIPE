#!/usr/bin/env python3
"""Reviewer's independent recomputation of reference_cases.json.

Written by the checkpoint-1 reviewer from VERIFICATION.md (git 9e8a55d) and
first-principles equations. It does NOT import or read the fixture generator,
the manager's check script, or any product code. Only the Python standard
library (fractions, decimal) is used.

Checks:
  A. every {exact, decimal, value} quantity: binary64 `value` is the correctly
     rounded projection of `exact`; `decimal` agrees with `exact` to >= 80
     significant digits (pi computed here by Machin's formula, not taken from
     the fixture);
  B. every expected quantity in the cases below is recomputed from its stated
     inputs with my own derivation and compared as an exact rational (times
     pi^k where relevant) or high-precision Decimal for exp();
  C. the stated inputs match the VERIFICATION.md control inputs;
  D. content labels: invented-data statement present; no material-library
     or code-rule vocabulary.
Exit status 1 on any failure.
"""
import json
import re
import sys
from decimal import Decimal, getcontext
from fractions import Fraction as F

getcontext().prec = 110
failures = []
checks = 0


def check(cond, msg):
    global checks
    checks += 1
    if not cond:
        failures.append(msg)


def machin_pi():
    getcontext().prec = 120

    def arctan_inv(x):
        x = Decimal(x)
        total = Decimal(0)
        term = 1 / x
        x2 = x * x
        n = 0
        while True:
            t = term / (2 * n + 1)
            if t == 0 or abs(t) < Decimal(10) ** -118:
                break
            total += t if n % 2 == 0 else -t
            term /= x2
            n += 1
        return total

    pi = 16 * arctan_inv(5) - 4 * arctan_inv(239)
    getcontext().prec = 110
    return +pi


PI = machin_pi()


def exp_dec(q):
    return (Decimal(q.numerator) / Decimal(q.denominator)).exp()


def dec(q):
    return Decimal(q.numerator) / Decimal(q.denominator)


# ---------- A: generic projection/decimal consistency ----------
def exact_decimal(ex):
    kind = ex["kind"]
    if kind == "rational":
        return dec(F(ex["rational"]))
    if kind == "rational_times_pi":
        return dec(F(ex["rational"])) * PI
    if kind == "rational_over_pi":
        return dec(F(ex["rational"])) / PI
    if kind == "symbolic":
        e = ex["expression"]
        m = re.fullmatch(r"exp\((-?\d+/\d+)\)-1", e)
        if m:
            return exp_dec(F(m.group(1))) - 1
        m = re.fullmatch(r"1-exp\((-?\d+/\d+)\)", e)
        if m:
            return 1 - exp_dec(F(m.group(1)))
    raise ValueError(ex)


def binary64(x):
    # float(str(Decimal)) is correctly rounded (Python uses David Gay's algorithm).
    return float(str(x))


def walk(o, path, out):
    if isinstance(o, dict):
        if "exact" in o and "value" in o:
            out.append((path, o))
        for k, v in o.items():
            walk(v, f"{path}.{k}", out)
    elif isinstance(o, list):
        for i, v in enumerate(o):
            walk(v, f"{path}[{i}]", out)


def sig_agree(a, b, digits):
    if a == b:
        return True
    scale = max(abs(a), abs(b))
    return abs(a - b) <= scale * Decimal(10) ** (-digits)


def exact_of(q):
    """Return (Fraction coefficient, pi_power) or ('exp', Decimal)."""
    ex = q["exact"]
    if ex["kind"] == "rational":
        return (F(ex["rational"]), 0)
    if ex["kind"] == "rational_times_pi":
        return (F(ex["rational"]), 1)
    if ex["kind"] == "rational_over_pi":
        return (F(ex["rational"]), -1)
    return ("dec", exact_decimal(ex))


def main(path):
    d = json.load(open(path))
    # D. labels
    check("Invented" in d.get("numbers", ""), "numbers label must state invented data")
    check("No operational material/code data" in d.get("numbers", ""), "no operational data statement")
    blob = json.dumps(d).lower()
    for word in ["asme", "b31", "astm", "a106", "a53", "a312", "sa-", "carbon steel", "stainless",
                 "allowable", "sif", "stress intensification", "code credit", "grade b", "en 13480"]:
        # word-bounded: first run flagged substrings of a sha256 and "classification"
        check(re.search(r"(?<![0-9a-z])" + re.escape(word) + r"(?![0-9a-z])", blob) is None,
              f"library/code vocabulary present: {word!r}")
    # fixture pi string vs Machin
    check(sig_agree(Decimal(d["numeric_representation"]["pi_decimal"]), PI, 95), "pi_decimal digits")

    # A.
    qs = []
    walk(d, "", qs)
    for p, q in qs:
        x = exact_decimal(q["exact"])
        check(binary64(x) == q["value"] or (x == 0 and q["value"] == 0.0),
              f"{p}: value {q['value']!r} is not correctly rounded projection {binary64(x)!r}")
        if "decimal" in q:
            check(sig_agree(Decimal(q["decimal"]), x, 80), f"{p}: decimal disagrees with exact")
    nq = len(qs)

    C = d["cases"]
    g = d["geometry"]

    def ex(q):
        return exact_of(q)

    def expect(p, q, coeff, pipow=0):
        got = ex(q)
        check(got == (F(coeff), pipow), f"{p}: fixture {got} != reviewer {(F(coeff), pipow)}")

    def expect_dec(p, q, val):
        got = ex(q)
        check(got[0] == "dec" and sig_agree(got[1], val, 90), f"{p}: exp value mismatch")

    # geometry, from OD and wall only
    OD, t = F(1, 5), F(1, 100)
    check(ex(g["authored"]["outside_diameter"]) == (OD, 0), "OD input")
    check(ex(g["authored"]["wall_thickness"]) == (t, 0), "wall input")
    ro, ri = OD / 2, OD / 2 - t
    As = ro**2 - ri**2            # times pi
    I = (ro**4 - ri**4) / 4       # times pi
    gd = g["derived"]
    expect("ro", gd["ro"], ro)
    expect("ri", gd["ri"], ri)
    expect("As", gd["As"], As, 1)
    expect("Ai", gd["Ai"], ri**2, 1)
    expect("I", gd["I"], I, 1)
    expect("J", gd["J"], 2 * I, 1)
    expect("Z", gd["Z"], I / ro, 1)
    check(As == t * (OD - t), "annulus identity As=t(OD-t)")

    # variants: generic area 1/1000 (pi^0) or annulus As (pi^1)
    def areas(case):
        v = case["variants"]
        out = [("generic_reviewed", v["generic_reviewed"], F(1, 1000), 0)]
        if "annular_companion" in v:
            out.append(("annular_companion", v["annular_companion"], As, 1))
        return out

    # ---- control 1: two bars in series; my own stiffness solve ----
    case = C["prescribed_translation_two_bar"]
    for name, var, A, pp in areas(case):
        i = var["inputs"]
        E1, E2 = F(200 * 10**9), F(100 * 10**9)
        L1 = L2 = F(1)
        g0, g2 = F(1, 10000), F(0)
        check(ex(i["E1"]) == (E1, 0) and ex(i["E2"]) == (E2, 0), f"c1 {name} E inputs (VERIFICATION: 200/100 GPa)")
        check(ex(i["root_UX"]) == (g0, 0) and ex(i["far_UX"]) == (g2, 0), f"c1 {name} motion inputs")
        if name == "generic_reviewed":
            check(ex(i["area"]) == (A, 0), "c1 area 0.001")
        k1, k2 = E1 * A / L1, E2 * A / L2   # times pi^pp
        # free equation: (k1+k2) u1 = k1 g0 + k2 g2  (pi cancels)
        u1 = (k1 * g0 + k2 * g2) / (k1 + k2)
        N1 = k1 * (u1 - g0)
        N2 = k2 * (g2 - u1)
        e = var["expected"]
        p = f"c1.{name}"
        expect(p + ".middle_UX", e["middle_UX"], u1)
        expect(p + ".member1_N", e["member1_N"], N1, pp)
        expect(p + ".member2_N", e["member2_N"], N2, pp)
        # support-on-member at root = -N1 (member pushes root), far = +N2
        expect(p + ".root_Fx", e["root_Fx"], -N1, pp)
        expect(p + ".far_Fx", e["far_Fx"], N2, pp)
        expect(p + ".rhs", e["free_rhs_after_prescribed_coupling"], k1 * g0 + k2 * g2, pp)
        check(N1 == N2, "c1 series force equality")
    expect("c1.wrong", case["wrong_result_discriminators"]["omitted_Kfc_gc"]["middle_UX"], 0)
    # VERIFICATION: middle 1/15000, forces +-20000/3
    check(u1 == F(1, 15000), "c1 matches VERIFICATION 1/15000")

    case = C["prescribed_translation_all_fixed"]
    for name, var, A, pp in areas(case):
        E = F(200 * 10**9)
        N = E * A * (F(0) - F(1, 10000)) / 1
        e = var["expected"]
        p = f"c1fixed.{name}"
        expect(p + ".N", e["N"], N, pp)
        expect(p + ".root", e["root_Fx"], -N, pp)
        expect(p + ".far", e["far_Fx"], N, pp)

    # ---- control 2: beam end rotation, Euler-Bernoulli stiffness ----
    def beam_K(EI, L):
        return [
            [12 * EI / L**3, 6 * EI / L**2, -12 * EI / L**3, 6 * EI / L**2],
            [6 * EI / L**2, 4 * EI / L, -6 * EI / L**2, 2 * EI / L],
            [-12 * EI / L**3, -6 * EI / L**2, 12 * EI / L**3, -6 * EI / L**2],
            [6 * EI / L**2, 2 * EI / L, -6 * EI / L**2, 4 * EI / L],
        ]

    case = C["prescribed_rotation_all_fixed"]
    for name, var, A, pp in areas(case):
        L, th = F(2), F(1, 1000)
        EI = F(2000) if name == "generic_reviewed" else F(200 * 10**9) * I
        epp = 0 if name == "generic_reviewed" else 1
        expect(f"c2.{name}.EI", var["inputs"]["EI"], EI, epp)
        K = beam_K(EI, L)
        u = [0, th, 0, 0]
        f = [sum(K[r][c] * u[c] for c in range(4)) for r in range(4)]
        e = var["expected"]
        for key, val in zip(["root_Fy", "root_Mz", "far_Fy", "far_Mz"], f):
            expect(f"c2.{name}.{key}", e[key], val, epp)
        # force and moment balance about root
        check(f[0] + f[2] == 0 and f[1] + f[3] + f[2] * L == 0, f"c2 {name} equilibrium")
        if name == "generic_reviewed":
            check(f == [3, 4, -3, 2], "c2 VERIFICATION [3,4,-3,2]")

    case = C["prescribed_rotation_free_tip"]
    for name, var, A, pp in areas(case):
        L, th = F(2), F(1, 1000)
        e = var["expected"]
        expect(f"c2free.{name}.tip_UY", e["tip_UY"], L * th)
        expect(f"c2free.{name}.tip_RZ", e["tip_RZ"], th)
        for key in ["root_Fy", "root_Mz", "wall_N"]:
            expect(f"c2free.{name}.{key}", e[key], 0)
        # free-tip condensation check: tip DOFs solve K_ff u_f = -K_fc u_c
        EI = F(2000)
        K = beam_K(EI, L)
        # u_c=[0,th] (root), solve 2x2 for [v2, th2]
        a, b, c_, d_ = K[2][2], K[2][3], K[3][2], K[3][3]
        r1 = -(K[2][0] * 0 + K[2][1] * th)
        r2 = -(K[3][0] * 0 + K[3][1] * th)
        det = a * d_ - b * c_
        v2 = (r1 * d_ - b * r2) / det
        th2 = (a * r2 - c_ * r1) / det
        check(v2 == L * th and th2 == th, "c2 free tip by condensation")
    w = case["wrong_result_discriminators"]
    expect("c2free.wrong0", w["missing_rotation_coupling"]["wrong_tip_UY"], 0)

    # ---- control 3: shared material, different member states ----
    case = C["shared_material_parallel"]
    for name, var, A, pp in areas(case):
        Ec, Eh, ec, eh, L = F(200 * 10**9), F(100 * 10**9), F(0), F(1, 1000), F(1)
        # Ec A (u/L - ec) + Eh A (u/L - eh) = 0
        u = L * (Ec * ec + Eh * eh) / (Ec + Eh)
        Nc = Ec * A * (u / L - ec)
        Nh = Eh * A * (u / L - eh)
        e = var["expected"]
        p = f"c3par.{name}"
        expect(p + ".tip", e["tip_UX"], u)
        expect(p + ".Nc", e["cold_N"], Nc, pp)
        expect(p + ".Nh", e["hot_N"], Nh, pp)
        expect(p + ".root", e["total_root_Fx"], 0)
    # material-id-only override: both members share one E -> u = L*eh/2
    expect("c3par.wrong", case["wrong_result_discriminators"]["material_ID_only_E_override"]["wrong_tip_UX"], F(1, 2000))

    case = C["shared_material_serial_companion"]
    for name, var, A, pp in areas(case):
        E1, E2, e1, e2, L1, L2 = F(200 * 10**9), F(100 * 10**9), F(0), F(1, 1000), F(1), F(1)
        # compatibility: total elongation 0 = N L1/(E1A)+e1 L1 + N L2/(E2A)+e2 L2
        N = -(e1 * L1 + e2 * L2) / (L1 / (E1 * A) + L2 / (E2 * A))
        um = N * L1 / (E1 * A) + e1 * L1
        e = var["expected"]
        p = f"c3ser.{name}"
        expect(p + ".um", e["middle_UX"], um)
        expect(p + ".N1", e["member1_N"], N, pp)
        expect(p + ".N2", e["member2_N"], N, pp)
        expect(p + ".root", e["root_Fx"], -N, pp)
        expect(p + ".far", e["far_Fx"], N, pp)
        if name == "annular_companion":
            expect(p + ".G1", e["G1"], E1 / (2 * (1 + F(3, 10))))
            expect(p + ".G2", e["G2"], E2 / (2 * (1 + F(1, 4))))
    expect("c3ser.wrong", case["wrong_result_discriminators"]["material_ID_only_E_override"]["wrong_middle_UX"], F(-1, 2000))

    # ---- control 4: thermal datum ratio ----
    case = C["thermal_datum_ratio"]
    Tm, Ti, T = F(20), F(50), F(150)          # degC; differences are K
    a_i, a_o = F(12, 10**6), F(16, 10**6)
    lam = lambda a, t_: 1 + a * (t_ - Tm)
    eps = lam(a_o, T) / lam(a_i, Ti) - 1
    check(eps == F(43, 25009), "c4 43/25009 (VERIFICATION)")
    for name, var, A, pp in areas(case):
        i = var["inputs"]
        check(ex(i["alpha_install"]) == (a_i, 0) and ex(i["alpha_operating"]) == (a_o, 0), f"c4 {name} alphas")
        pts = i["table_points"]
        check([ex(p_["temperature"])[0] for p_ in pts] == [20, 50, 150], f"c4 {name} table temps")
        check(ex(pts[1]["alpha"]) == (a_i, 0) and ex(pts[2]["alpha"]) == (a_o, 0), f"c4 {name} table alphas")
        e = var["expected"]
        p = f"c4.{name}"
        expect(p + ".di", e["dilation_install"], a_i * (Ti - Tm))
        expect(p + ".do", e["dilation_operating"], a_o * (T - Tm))
        expect(p + ".stretch", e["thermal_stretch"], 1 + eps)
        expect(p + ".strain", e["thermal_strain"], eps)
        expect(p + ".TmK", e["datum_temperature_K"], Tm + F(27315, 100))
        expect(p + ".TiK", e["installation_temperature_K"], Ti + F(27315, 100))
        expect(p + ".TK", e["operating_temperature_K"], T + F(27315, 100))
        if name == "annular_companion":
            E, L = F(200 * 10**9), F(1)
            expect(p + ".free_tip", e["free_tip_UX"], L * eps)
            expect(p + ".free_N", e["free_wall_N"], 0)
            expect(p + ".fixed_N", e["fixed_wall_N"], -E * As * eps, 1)
            expect(p + ".fixed_root", e["fixed_root_Fx"], E * As * eps, 1)
            expect(p + ".fixed_far", e["fixed_far_Fx"], -E * As * eps, 1)
    w = case["wrong_result_discriminators"]
    expect("c4.wrong1", w["alpha_hot_times_operating_minus_install"], a_o * (T - Ti))
    expect("c4.wrong2", w["subtract_datum_dilations"], a_o * (T - Tm) - a_i * (Ti - Tm))

    case = C["constant_alpha_interval"]
    var = case["variants"]["generic_reviewed"]
    expect("c4const.strain", var["expected"]["thermal_strain"], F(1, 10**5) * 80)
    expect("c4const.stretch", var["expected"]["thermal_stretch"], 1 + F(1, 10**5) * 80)

    # ---- control 5: coefficient definitions ----
    case = C["coefficient_definition"]
    var = case["variants"]["generic_reviewed"]
    a0, a1, t0, t1 = F(10, 10**6), F(20, 10**6), F(20), F(120)
    alpha = lambda t_: a0 + (a1 - a0) * (t_ - t0) / (t1 - t0)
    # exact polynomial antiderivative of the linear coefficient
    Aint = lambda lo, hi: (alpha(lo) + alpha(hi)) * (hi - lo) / 2
    lam_d = lambda t_: 1 + Aint(t0, t_)
    e = var["expected"]
    expect("c5.int", e["integral"], Aint(t0, t1))
    expect("c5.int1", e["first_half_integral"], Aint(t0, F(70)))
    expect("c5.int2", e["second_half_integral"], Aint(F(70), t1))
    expect("c5.dfwd", e["datum_length_strain_forward"], lam_d(t1) / lam_d(t0) - 1)
    expect("c5.drev", e["datum_length_strain_reverse"], lam_d(t0) / lam_d(t1) - 1)
    expect("c5.d1", e["datum_length_strain_first_half"], lam_d(F(70)) / lam_d(t0) - 1)
    expect("c5.d2", e["datum_length_strain_second_half"], lam_d(t1) / lam_d(F(70)) - 1)
    expect_dec("c5.lfwd", e["current_length_strain_forward"], exp_dec(Aint(t0, t1)) - 1)
    expect_dec("c5.lrev", e["current_length_strain_reverse"], exp_dec(-Aint(t0, t1)) - 1)
    expect_dec("c5.l1", e["current_length_strain_first_half"], exp_dec(Aint(t0, F(70))) - 1)
    expect_dec("c5.l2", e["current_length_strain_second_half"], exp_dec(Aint(F(70), t1)) - 1)
    check(Aint(t0, t1) == F(3, 2000), "c5 VERIFICATION 0.0015")
    w = case["wrong_result_discriminators"]
    expect("c5.wrong", w["endpoint_alpha_times_interval"], a1 * (t1 - t0))
    check(F(1, 500) == a1 * (t1 - t0), "c5 wrong 0.002 (VERIFICATION)")

    # ---- control 6: signed fit ----
    case = C["signed_fit_states"]
    L, dL, alpha_c = F(10), F(-2, 1000), F(1, 10**5)
    Ec, Eh = F(200 * 10**9), F(150 * 10**9)
    lam_fit = 1 + dL / L
    for name, var, A, pp in areas(case):
        i = var["inputs"]
        check(ex(i["signed_fit_length_change"]) == (dL, 0) and ex(i["L"]) == (L, 0), f"c6 {name} fit inputs")
        check(ex(i["cold_E"]) == (Ec, 0) and ex(i["hot_E"]) == (Eh, 0), f"c6 {name} E inputs")
        states = {"cold": (Ec, F(0)), "hot": (Eh, alpha_c * 80), "return": (Ec, F(0))}
        for sname, (E, eth) in states.items():
            e = var["expected"][sname]
            p = f"c6.{name}.{sname}"
            star = lam_fit * (1 + eth) - 1
            expect(p + ".E", e["E"], E)
            expect(p + ".eth", e["thermal_strain"], eth)
            expect(p + ".fit", e["fit_strain"], dL / L)
            expect(p + ".star", e["total_eigenstrain"], star)
            # both ends fixed: u_j-u_i = 0 -> N = -E A eps*
            N = -E * A * star
            expect(p + ".N", e["fixed_wall_N"], N, pp)
            expect(p + ".root", e["fixed_root_Fx"], -N, pp)
            expect(p + ".far", e["fixed_far_Fx"], N, pp)
            expect(p + ".tip", e["released_tip_UX"], L * star)
            expect(p + ".relN", e["released_wall_N"], 0)
        b = var["baselines"]
        expect(f"c6.{name}.nofit_cold", b["no_fit_cold_N"], 0)
        expect(f"c6.{name}.nofit_hot", b["no_fit_hot_N"], -Eh * A * alpha_c * 80, pp)
        expect(f"c6.{name}.cutlong", b["cut_long_cold_N"], -Ec * A * (-dL / L), pp)
        if name == "generic_reviewed":
            check(-Ec * A * (dL / L) == 40000 and -Eh * A * (lam_fit * (1 + alpha_c * 80) - 1) == -89976,
                  "c6 VERIFICATION +40000/-89976")
            check(lam_fit * (1 + alpha_c * 80) - 1 == F(59984, 10**8), "c6 eps* 0.00059984")
    w = case["wrong_result_discriminators"]
    A = F(1, 1000)
    expect("c6.w.add", w["hot_additive_strains"]["wrong_generic_N"], -Eh * A * (dL / L + alpha_c * 80))
    expect("c6.w.coldE", w["hot_uses_cold_E"]["wrong_generic_N"], -Ec * A * (lam_fit * (1 + alpha_c * 80) - 1))
    expect("c6.w.twice", w["fit_applied_twice"]["wrong_cold_generic_N"], -Ec * A * (2 * dL / L))
    expect("c6.w.sign", w["sign_flipped"]["wrong_cold_generic_N"], -Ec * A * (-dL / L))

    # ---- control 9: persistent source once ----
    case = C["persistent_source_once"]
    for name, var, A_, pp in areas(case):
        pre, wt, ind = F(100), F(-1000), F(200)
        k = F(1000) if name == "generic_reviewed" else F(200 * 10**9) * As / 1
        kpp = 0 if name == "generic_reviewed" else 1
        expect(f"c9.{name}.k", var["inputs"]["structure_stiffness"], k, kpp)
        e = var["expected"]
        rhs = pre + wt + ind
        expect(f"c9.{name}.rhs", e["combined_rhs"], rhs)
        expect(f"c9.{name}.u", e["combined_displacement"], rhs / k, -kpp)
        expect(f"c9.{name}.R", e["root_Fx"], -rhs)
        expect(f"c9.{name}.W", e["weight_total_rhs"], pre + wt)
        expect(f"c9.{name}.add", e["additional_total_rhs"], pre + ind)
        expect(f"c9.{name}.naive", e["naive_total_sum_rhs"], (pre + wt) + (pre + ind))
        expect(f"c9.{name}.naiveu", e["naive_total_sum_displacement"], ((pre + wt) + (pre + ind)) / k, -kpp)
        expect(f"c9.{name}.two", e["two_distinct_equal_actions_rhs"], F(200) + F(200))
        expect(f"c9.{name}.excl", e["excluded_unreferenced_action_rhs"], rhs)
    check(F(-700) / 1000 == F(-7, 10), "c9 VERIFICATION u=-0.7")

    print(json.dumps({"quantities_scanned": nq, "checks": checks, "failures": failures}, indent=1))
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1]))
