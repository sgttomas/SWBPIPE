"""Item 2: the single-degree-of-freedom oscillator (DEL-04-10 AC-002 first and second items).

Mass m on a spring k to a rigid support, uniform support motion along the
spring. One free DOF, shape phi = 1 (unit maximum translation), generalized
mass m, excitation p = M R = m, so Gamma = 1 and the effective mass is m.
With w^2 = k/m and the pseudo-spectral convention:
  u = Gamma s S(f) / w^2                (relative to the support)
  spring force N = k u                  (tension positive, u positive away from the support)
  reaction at the rigid support  r = K_cf u = -k u
At the displacement extremum the velocity is zero, so these are the published
values whatever the spectrum's damping; the damping enters only through S.
"""

from decimal import Decimal
from fractions import Fraction

from numerics import dec, dsqrt, inp, num, pi, rel_diff, sci, val
import rules
from tables import RULES, T1, T2, rule_name, variant

M = 125.0
K = 200000.0
SCALE = 1.5


def build(cross):
    m, k = Fraction(M), Fraction(K)
    w2 = k / m                     # 1600 exactly
    w = dsqrt(dec(w2))             # 40 exactly
    f = w / (2 * pi())
    worst = Decimal(0)
    cases = []
    for base in (T1, T2):
        for rule in RULES:
            tab = variant(base, rule)
            readings = ["tabulated"] + (["frequency"] if base["abscissa"] == "period" and rule[0] == "linear" else [])
            for reading in readings:
                S = rules.evaluate(tab, f, reading)
                S = S if isinstance(S, Decimal) else dec(S)
                a = dec(SCALE) * S
                u = a / dec(w2)
                N = dec(k) * u
                r = -N
                # cross-check: spring force equals m times the scaled ordinate (balance of the mass)
                worst = max(worst, rel_diff(N, dec(m) * a))
                zeta = dec(base["damping"])
                info_v = 2 * zeta * N            # damper force at pseudo-velocity w u: c w u = 2 zeta k u
                info_h = dsqrt(1 + 4 * zeta * zeta) * N
                c = {
                    "table": base["id"], "rule": rule_name(rule),
                    "reading": reading,
                    "S_m_per_s2": val(S), "sS_m_per_s2": val(a),
                    "modal_displacement_u_m": val(u),
                    "spring_force_N_tension_positive": val(N),
                    "rigid_support_reaction_signed_N": val(r),
                    "combined_magnitudes": {"displacement_m": val(abs(u)),
                                            "spring_force_N": val(abs(N)),
                                            "reaction_N": val(abs(r))},
                    "information_not_adopted": {
                        "label": ("NOT the adopted value: size of the velocity-proportional term the "
                                  "method neglects (REQ-002), for a viscous damper c = 2 zeta m w in "
                                  "parallel with the spring"),
                        "damper_force_at_pseudo_velocity_N": val(info_v),
                        "ratio_to_spring_force": val(2 * zeta),
                        "harmonic_peak_of_spring_plus_damper_N": val(info_h),
                        "ratio_harmonic_peak_to_spring_force": val(dsqrt(1 + 4 * zeta * zeta)),
                    },
                }
                if base["abscissa"] == "period":
                    c["T_s"] = val(1 / f)
                cases.append(c)
    cross.append({"id": "SDOF-balance", "what": "k u against m s S(f) for every case",
                  "max_rel_diff": sci(worst), "agrees": worst < Decimal("1e-80")})
    w_check = w * w - dec(w2)
    cross.append({"id": "SDOF-omega", "what": "w = sqrt(k/m) = 40 rad/s exactly", "residual": sci(w_check),
                  "agrees": w == 40})
    return {
        "item": "2. Single-degree-of-freedom oscillator (AC-002: SDOF under each interpolation rule and the "
                "pseudo-acceleration convention; damped force and reaction at the displacement extremum)",
        "inputs": {"mass_kg": inp(M), "stiffness_N_per_m": inp(K), "scale_factor_s": inp(SCALE)},
        "derived": {"omega_squared_rad2_per_s2": num(w2), "omega_rad_per_s": val(w), "frequency_hz": val(f),
                    "period_s": val(1 / f), "participation_factor": num(Fraction(1)),
                    "effective_mass_kg": num(m), "effective_mass_fraction": num(Fraction(1))},
        "spectra": "T1 and T2 of spectrum_tables.json (damping ratio 0.05 recorded with each); the damping "
                   "enters only through the tabulated ordinates",
        "cases": cases,
    }
