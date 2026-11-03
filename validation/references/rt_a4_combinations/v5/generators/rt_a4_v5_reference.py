#!/usr/bin/env python3
"""RT-A4 V-5 independent references: three-axis static seismic on a cantilever.

Standard-library Python 3 only. This generator imports, reads and runs no
product code; it reads only its own directory's committed files (to hash them)
and writes `v5_cantilever.json` and `SHA256SUMS`.

Model (invented): a straight pipe cantilever of length L along the unit vector
e = (3/5, 4/5, 0), anchored at the origin, annulus D x t, modulus E. Three load
cases each carry a uniform force per length q = w_k g_k in one global
direction g_k (X, Y, Z): the static equivalent of a uniform acceleration on the
line mass. Split q into its axial part (q.e) e and transverse part
q_t = q - (q.e) e; textbook closed forms (Euler-Bernoulli, small displacement,
a circular annulus is isotropic in bending), at the tip and the anchor:

  tip displacement   (q.e) L^2 / (2 E A) e  +  q_t L^4 / (8 E I)
  tip rotation       (e x q_t) L^3 / (6 E I)
  anchor force       q L                      (magnitude; the reaction is -q L)
  anchor moment      (L/2 e) x (q L)          (magnitude; the reaction is its negative)

with A = pi (D^2 - d^2) / 4 and I = pi (D^4 - d^4) / 64, d = D - 2t. Because
e has rational direction cosines, the X and Y cases both excite the tip's X
and Y displacements, its Z rotation and the anchor's Z moment, so the absolute
sum and the SRSS differ there. Each case's components are published unsigned
(the unsigned combinations do not depend on signs). For each combination with factors f_k over the three cases
and each quantity q (a component, zero in the cases that do not excite it):

  absolute sum   A_q = sum_k |f_k q_k|
  SRSS           R_q = sqrt(sum_k (f_k q_k)^2)

and the tip displacement and anchor force and moment magnitudes are the norm
of the combination's own components (DEL-05-02 absolute-sum-srss §4).

Arithmetic: every value is an exact fractions.Fraction in which pi is replaced
by a rational PI within 1e-80 of pi (Machin's formula on integers; independent
of any floating-point pi). A square root is the integer square root of the
value scaled by 10^160, so it is within 1e-80 (relative to the scale) of the
exact root. Each published number is the nearest binary64 (Python float of
the Fraction, correctly rounded) and a 30-significant-digit decimal.

Modes:
  python3 -I rt_a4_v5_reference.py           check: rebuild in memory, compare byte for byte
  python3 -I rt_a4_v5_reference.py --write   write the outputs (intentional changes only)
"""
import argparse
import hashlib
import json
import math
import sys
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent
OUT = HERE.parent
FIXED_FILES = ("README.md", "generators/rt_a4_v5_reference.py")
DIGITS = 80


def arctan_inverse(x, scale):
    """atan(1/x) * scale, by the alternating series on integers."""
    total, term, n, sign = 0, scale // x, 1, 1
    x2 = x * x
    while term:
        total += sign * (term // n)
        term //= x2
        n += 2
        sign = -sign
    return total


def machin_pi():
    scale = 10 ** (DIGITS + 10)
    pi = 16 * arctan_inverse(5, scale) - 4 * arctan_inverse(239, scale)
    return F(pi, scale)


PI = machin_pi()
ROOT_SCALE = 10 ** 160


def sqrt(value):
    assert value >= 0
    n = value * ROOT_SCALE * ROOT_SCALE
    return F(math.isqrt(n.numerator // n.denominator), ROOT_SCALE)


def number(value):
    value = F(value)
    return {"binary64": float(value), "decimal": format_decimal(value)}


def format_decimal(value):
    if value == 0:
        return "0"
    sign = "-" if value < 0 else ""
    value = abs(value)
    exponent = math.floor(math.log10(float(value)))
    while value >= F(10) ** (exponent + 1):
        exponent += 1
    while value < F(10) ** exponent:
        exponent -= 1
    digits = round(value / F(10) ** (exponent - 29))
    text = str(digits)
    return f"{sign}{text[0]}.{text[1:]}e{exponent}"


INPUTS = {
    "length_m": F(5),
    "outside_diameter_m": F(1, 5),
    "wall_thickness_m": F(1, 100),
    "elastic_modulus_Pa": F(200_000_000_000),
    "poisson_ratio": F(3, 10),
    "w_N_per_m": {"x": F(1500), "y": F(2000), "z": F(2500)},
}
CASES = [("case:EQX", "x"), ("case:EQY", "y"), ("case:EQZ", "z")]
COMBINATIONS = [
    {"id": "comb:SRSS", "basis": "srss", "factors": [F(1), F(1), F(1)]},
    {"id": "comb:ABS", "basis": "absolute_sum", "factors": [F(1), F(1), F(1)]},
    {"id": "comb:SRSS-F", "basis": "srss", "factors": [F(1, 2), F(2), F(3, 2)]},
    {"id": "comb:ABS-F", "basis": "absolute_sum", "factors": [F(1, 2), F(-2), F(3, 2)]},
]
# The quantities: (name, entity, kind token used by the test, component).
AXIS = (F(3, 5), F(4, 5), F(0))
QUANTITIES = [
    ("tip_ux", "node:tip", "global_nodal_displacement_x"),
    ("tip_uy", "node:tip", "global_nodal_displacement_y"),
    ("tip_uz", "node:tip", "global_nodal_displacement_z"),
    ("tip_rx", "node:tip", "global_nodal_rotation_x"),
    ("tip_ry", "node:tip", "global_nodal_rotation_y"),
    ("tip_rz", "node:tip", "global_nodal_rotation_z"),
    ("anchor_fx", "support:root", "Fx"),
    ("anchor_fy", "support:root", "Fy"),
    ("anchor_fz", "support:root", "Fz"),
    ("anchor_mx", "support:root", "Mx"),
    ("anchor_my", "support:root", "My"),
    ("anchor_mz", "support:root", "Mz"),
]


def cross(a, b):
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def case_values(axis):
    """Each quantity's unsigned closed-form value in the case loaded along `axis`."""
    L, D, t, E = (INPUTS[k] for k in ("length_m", "outside_diameter_m", "wall_thickness_m", "elastic_modulus_Pa"))
    d = D - 2 * t
    area = PI * (D ** 2 - d ** 2) / 4
    inertia = PI * (D ** 4 - d ** 4) / 64
    w = INPUTS["w_N_per_m"][axis]
    q = tuple(w if a == axis else F(0) for a in "xyz")
    e = AXIS
    axial = sum(qi * ei for qi, ei in zip(q, e))
    transverse = tuple(qi - axial * ei for qi, ei in zip(q, e))
    tip = tuple(axial * L ** 2 / (2 * E * area) * ei + ti * L ** 4 / (8 * E * inertia) for ei, ti in zip(e, transverse))
    rotation = tuple(c * L ** 3 / (6 * E * inertia) for c in cross(e, transverse))
    force = tuple(qi * L for qi in q)
    moment = cross(tuple(ei * L / 2 for ei in e), force)
    values = tip + rotation + force + moment
    return {name: abs(v) for (name, _, _), v in zip(QUANTITIES, values)}


def combine(basis, factors, values):
    if basis == "absolute_sum":
        return sum(abs(f * v) for f, v in zip(factors, values))
    return sqrt(sum((f * v) ** 2 for f, v in zip(factors, values)))


def build():
    per_case = {case: case_values(axis) for case, axis in CASES}
    combinations = []
    for combination in COMBINATIONS:
        rows = {}
        for name, _, _ in QUANTITIES:
            rows[name] = combine(combination["basis"], combination["factors"], [per_case[c][name] for c, _ in CASES])
        rows["tip_displacement_magnitude"] = sqrt(rows["tip_ux"] ** 2 + rows["tip_uy"] ** 2 + rows["tip_uz"] ** 2)
        rows["anchor_force_magnitude"] = sqrt(rows["anchor_fx"] ** 2 + rows["anchor_fy"] ** 2 + rows["anchor_fz"] ** 2)
        rows["anchor_moment_magnitude"] = sqrt(rows["anchor_mx"] ** 2 + rows["anchor_my"] ** 2 + rows["anchor_mz"] ** 2)
        combinations.append({
            "id": combination["id"],
            "basis": combination["basis"],
            "terms": [{"load_case": c, "factor": number(f)} for (c, _), f in zip(CASES, combination["factors"])],
            "rows": {name: number(value) for name, value in rows.items()},
        })
    return {
        "reference_id": "rt_a4_combinations/v5_cantilever",
        "generator": "validation/references/rt_a4_combinations/v5/generators/rt_a4_v5_reference.py",
        "claim_fence": "invented inputs; independent closed forms; no product code; F-PIP-2",
        "inputs": {
            "length_m": number(INPUTS["length_m"]),
            "axis_unit_vector": [number(c) for c in AXIS],
            "outside_diameter_m": number(INPUTS["outside_diameter_m"]),
            "wall_thickness_m": number(INPUTS["wall_thickness_m"]),
            "elastic_modulus_Pa": number(INPUTS["elastic_modulus_Pa"]),
            "poisson_ratio": number(INPUTS["poisson_ratio"]),
            "w_N_per_m": {axis: number(w) for axis, w in INPUTS["w_N_per_m"].items()},
        },
        "quantities": [{"name": n, "entity": e, "kind_or_component": k} for n, e, k in QUANTITIES],
        "cases": [{"id": c, "axis": a, "unsigned_values": {n: number(v) for n, v in per_case[c].items()}} for c, a in CASES],
        "combinations": combinations,
        "criterion": "relative 1e-9 against the binary64 value; an exactly zero expectation is compared with 1e-9 times the quantity family's largest case value",
    }


def outputs():
    references = json.dumps(build(), indent=1, ensure_ascii=False) + "\n"
    files = {"v5_cantilever.json": references}
    sums = []
    for name in sorted(FIXED_FILES + ("v5_cantilever.json",)):
        data = files[name].encode() if name in files else (OUT / name).read_bytes()
        sums.append(f"{hashlib.sha256(data).hexdigest()}  {name}\n")
    files["SHA256SUMS"] = "".join(sums)
    return files


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--write", action="store_true")
    args = parser.parse_args()
    files = outputs()
    if args.write:
        for name, text in files.items():
            (OUT / name).write_text(text, encoding="utf-8")
        return 0
    for name, text in files.items():
        if (OUT / name).read_text(encoding="utf-8") != text:
            print(f"differs: {name}", file=sys.stderr)
            return 1
    print("rt_a4 V-5 references: every output matches")
    return 0


if __name__ == "__main__":
    sys.exit(main())
