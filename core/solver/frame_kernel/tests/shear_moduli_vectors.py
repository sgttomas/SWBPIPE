"""Vectors for `shear::annulus_energy_shear_factor_of_moduli` (shear_moduli_factor.rs).

Each line holds ri, ro, E, G as binary64 bits, then either the factor's expected
bits (its fixed order, reproduced here in binary64, which Python floats are, with
no fused multiply-add) and the exact contract value kappa* as a double-double
(hi, lo), or `refused` with the refusal's kind. kappa* is the module's contract
form 6(1+nu)^2(1+m^2)^2 / ((7+14nu+8nu^2)(1+m^2)^2 + 4m^2(5+10nu+4nu^2)) at the
exact rationals m = ri/ro and nu = E/(2G) - 1 of the binary64 inputs.

The grid spans E/(2G) from 1.4 to 1e-300: within ulps of 1 (nu near 0), both
Sterbenz edges of 1 - x (E/(2G) near 1/2 and the domain edge 3/2), 2^-13 and
2^-54, and both sides of the t*t overflow near 7.458e-155, on solid, ordinary,
thin and binary-exact annuli and four shear moduli.

    python3 -I shear_moduli_vectors.py          # rewrite the vectors
    python3 -I shear_moduli_vectors.py --check  # compare with the committed file
"""
import math
import random
import struct
import sys
from fractions import Fraction
from pathlib import Path

OUT = Path(__file__).with_name("shear_moduli_vectors.txt")
HEADER = (
    "# annulus_energy_shear_factor_of_moduli vectors (shear_moduli_vectors.py): ri ro E G, binary64 bits in hex,\n"
    "# then the factor's bits and the exact contract kappa* as hi lo, or `refused <kind>`.\n"
)
SECTIONS = [(0.0, 0.1), (0.05, 0.1), (0.09, 0.1), (0.099, 0.1), (0.75, 1.0), (90.0, 100.0), (0.0925, 0.1016)]
# Pa: steel-like, unit, and the near-minus-one review model's G (3e9 * 2^51 Pa).
SHEAR_MODULI = [8e10, 7.7e10, 1.0, 6.755399441055744e24]


def bits(value):
    return struct.unpack("<Q", struct.pack("<d", value))[0]


def hexbits(value):
    return f"{bits(value):016x}"


def ratios():
    edges = [
        1.4, 1.5, 1.5 - 2**-52, 1.25,
        1.0 + 2**-52, 1.0, 1.0 - 2**-53, 1.0 - 2**-52, 1.0 + 2**-51, 1.0 - 2**-51,
        0.9, 0.75, 0.5 + 2**-53, 0.5, 0.5 - 2**-54, 0.5 - 2**-53, 0.4, 0.25,
        2**-13 * (1 + 2**-52), 2**-13, 2**-13 * (1 - 2**-53), 1e-5, 1e-10,
        1 / (3 * 2**52), 2**-54 * (1 + 2**-52), 2**-54, 2**-54 * (1 - 2**-53), 1e-17, 1e-20,
        1e-50, 1e-100, 1e-150, 7.5e-155, 7.46e-155, 7.458e-155, 7.457e-155, 7.45e-155, 1e-200, 1e-300,
    ]
    rng = random.Random(20261027)
    return edges + [10 ** rng.uniform(-300, math.log10(1.49)) for _ in range(40)]


def factor(ri, ro, e, g):
    """The kernel's order (shear.rs), or the refusal's kind."""
    if not all(math.isfinite(v) for v in (ri, ro, e, g)):
        return "NonFiniteInput"
    if ro <= 0.0:
        return "NonPositiveOuterRadius"
    if not 0.0 <= ri < ro:
        return "InnerRadiusOutOfRange"
    if e <= 0.0 or g <= 0.0:
        return "NonPositiveModulus"
    if e / (2.0 * g) >= 1.5:
        return "PoissonRatioOutOfRange"
    m = ri / ro
    m2 = m * m
    a = 1.0 + m2
    b = 1.0 - m2
    x = (2.0 * g) / e
    t = 1.0 - x
    d = (7.0 + t * t) * (b * b) + 48.0 * m2
    if not (math.isfinite(d) and d >= sys.float_info.min):
        return "NumericalRange"
    kappa = (6.0 * (a * a)) / d
    if not (math.isfinite(kappa) and kappa >= sys.float_info.min):
        return "NumericalRange"
    return kappa


def contract(ri, ro, e, g):
    m = Fraction(ri) / Fraction(ro)
    nu = Fraction(e) / (2 * Fraction(g)) - 1
    s = (1 + m * m) ** 2
    return 6 * (1 + nu) ** 2 * s / ((7 + 14 * nu + 8 * nu * nu) * s + 4 * m * m * (5 + 10 * nu + 4 * nu * nu))


def double_double(value):
    hi = float(value)
    lo = float(value - Fraction(hi))
    return hi, lo


def lines():
    out = []
    for ri, ro in SECTIONS:
        for g in SHEAR_MODULI:
            for r in ratios():
                e = (2.0 * g) * r
                if not (math.isfinite(e) and e > 0.0):
                    continue
                kappa = factor(ri, ro, e, g)
                head = " ".join(hexbits(v) for v in (ri, ro, e, g))
                if isinstance(kappa, str):
                    out.append(f"{head} refused {kappa}")
                    continue
                hi, lo = double_double(contract(ri, ro, e, g))
                out.append(f"{head} {hexbits(kappa)} {hexbits(hi)} {hexbits(lo)}")
    return HEADER + "\n".join(out) + "\n"


def main():
    text = lines()
    if sys.argv[1:] == ["--check"]:
        if OUT.read_text() != text:
            sys.exit(f"{OUT.name} is stale: rerun {Path(__file__).name}")
        print(f"{OUT.name}: current")
    elif sys.argv[1:]:
        sys.exit(__doc__)
    else:
        OUT.write_text(text)
        print(f"{OUT.name}: {text.count(chr(10)) - 2} vectors")


if __name__ == "__main__":
    main()
