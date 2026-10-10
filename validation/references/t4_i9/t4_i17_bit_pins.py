"""T4-I17 (T4-U1b): V2b bit pins of the arc load certificate, from the
design's executable specification (T4-I9 revision 01's probe,
`_run_records/arc_cert_probe_r01.py`, copied byte for byte), consumed by FK
`arc_certificate_tests.rs::matches_the_specification_bit_for_bit` and the
lemma pins there.

Standard library only. Run from this directory:
    python3 -I t4_i17_bit_pins.py _run_records/arc_cert_probe_r01.py > arc_certificate_pins.txt

Format (floats as the 16-hex-digit bits of their binary64 value):
    case <label>
    args <xi0 xi1 xi2> <xj0 xj1 xj2> <R> <y0 y1 y2> <E> <G> <A> <I> <J> <k_in> <k_out> <w0 w1 w2>
    then either   err <class>            (DESIGN_R01 section 3.5 class; 5 is followed by its rho bits)
    or            pin <i> <n> <t0..t(n-1)> <radius>   (12 lines: the midpoint's split and r-hat)
                  rho <bits>
    end
    lemma <name> <op> <n a-terms r_a> <n b-terms r_b | -> -> <n result-terms r>   (L1-L6 adversarial inputs)
"""
import importlib.util
import math
import struct
import sys
from fractions import Fraction as Q

spec = importlib.util.spec_from_file_location("probe", sys.argv[1])
p = importlib.util.module_from_spec(spec)
spec.loader.exec_module(p)


def bits(x):
    return "%016x" % struct.unpack("<Q", struct.pack("<d", float(x)))[0]


def section(od, wall):
    ro, ri = od / 2, od / 2 - wall
    return math.pi * (ro * ro - ri * ri), math.pi * (ro ** 4 - ri ** 4) / 4, 2 * (math.pi * (ro ** 4 - ri ** 4) / 4)


def arc_nodes(origin, chord_dir, plane_dir, radius, phi):
    n = math.sqrt(sum(c * c for c in chord_dir))
    dh = [c / n for c in chord_dir]
    pr = sum(plane_dir[k] * dh[k] for k in range(3))
    nr = [plane_dir[k] - pr * dh[k] for k in range(3)]
    nn = math.sqrt(sum(c * c for c in nr))
    nh = [c / nn for c in nr]
    length = 2.0 * radius * math.sin(0.5 * phi)
    xi = list(origin)
    xj = [origin[k] + length * dh[k] for k in range(3)]
    return xi, xj, [nh[k] + 0.37 * dh[k] for k in range(3)]


def cases():
    out = []
    a15, i15, j15 = section(0.168, 0.007)
    t15 = lambda k, w: ([0.0, 0.0, 0.0], [2.0, 0.0, 0.0], math.sqrt(2.0), [0.0, 1.0, 0.0], 200e9, 80e9,
                        a15, i15, j15, k, k, w)
    for k in (2.0, 1e10, 6e35, 1e36, 1e37):
        out.append(("T15 body k=%g" % k, t15(k, [0.0, 0.0, 0.3])))
    out.append(("T15 body w=1e-300 (truncated split)", t15(2.0, [0.0, 0.0, 1e-300])))
    out.append(("T15 body w=1e308 (class 9, C-3)", t15(2.0, [0.0, 0.0, 1e308])))
    out.append(("T15 body w=1e300", t15(2.0, [0.0, 0.0, 1e300])))
    out.append(("T15 body E=0 (class 2)", ([0.0, 0.0, 0.0], [2.0, 0.0, 0.0], math.sqrt(2.0), [0.0, 1.0, 0.0],
                                           0.0, 80e9, a15, i15, j15, 2.0, 2.0, [0.0, 0.0, 0.3])))
    out.append(("coincident nodes (class 3)", ([1.0, 2.0, 3.0], [1.0, 2.0, 3.0], 1.0, [0.0, 1.0, 0.0], 200e9, 80e9,
                                               a15, i15, j15, 1.0, 1.0, [0.0, 0.0, 0.3])))
    out.append(("chord 2R (class 3)", ([0.0, 0.0, 0.0], [2.0, 0.0, 0.0], 1.0, [0.0, 1.0, 0.0], 200e9, 80e9,
                                       a15, i15, j15, 1.0, 1.0, [0.0, 0.0, 0.3])))
    out.append(("y parallel to d (class 3)", ([0.0, 0.0, 0.0], [2.0, 0.0, 0.0], math.sqrt(2.0), [3.0, 0.0, 0.0],
                                              200e9, 80e9, a15, i15, j15, 1.0, 1.0, [0.0, 0.0, 0.3])))
    out.append(("NaN coordinate (class 7)", ([0.0, float("nan"), 0.0], [2.0, 0.0, 0.0], math.sqrt(2.0),
                                             [0.0, 1.0, 0.0], 200e9, 80e9, a15, i15, j15, 1.0, 1.0, [0.0, 0.0, 0.3])))
    steel = (2.03e11, 2.03e11 / 2.6)
    aL, iL, jL = section(0.1683, 0.00711)
    def lline(label, phi, origin=(3.0, 0.0, 0.0), chord=(1.0, 0.0, 0.0), plane=(0.0, -1.0, 0.0), R=0.2286,
              k=(1.0, 1.0), w=(0.0, 0.0, -450.0), sec=(aL, iL, jL)):
        xi, xj, y = arc_nodes(origin, chord, plane, R, phi)
        out.append((label, (xi, xj, R, y, steel[0], steel[1], sec[0], sec[1], sec[2], k[0], k[1], list(w))))
    lline("L line 90 deg", math.pi / 2)
    lline("L line 1 deg", math.radians(1.0))
    lline("L line 1e-4 rad", 1e-4)
    lline("L line 1e-8 rad", 1e-8)
    lline("L line 170 deg", math.radians(170.0))
    lline("L line pi-1e-6", math.pi - 1e-6)
    utm = (7.3e6 + 0.123456789, 4.6e6 - 7.654321, 312.5 + 1.0 / 3.0)
    lline("UTM skew 45 deg", math.radians(45.0), origin=utm, chord=(0.6, -0.3, 0.74), plane=(0.2, 0.9, -0.1))
    lline("UTM skew 7 deg in-plane", math.radians(7.0), origin=utm, chord=(0.1, 0.99, 0.0), plane=(1.0, 0.0, 0.3),
          w=(-450.0, 0.0, 0.0))
    lline("three-component w 100 deg", math.radians(100.0), chord=(0.5, 0.5, 0.5), plane=(0.0, 1.0, -1.0),
          w=(123.4, -56.7, -890.1))
    lline("k_in=3 k_out=0.5 R=25 w=1e9", math.radians(20.0), R=25.0, k=(3.0, 0.5), w=(0.0, 1e9, 0.0),
          sec=section(1.2192, 0.00635))
    lline("k=1e-3 90 deg", math.pi / 2, k=(1e-3, 1e-3))
    lline("0.3 m chord 1e-4 rad", 1e-4, R=0.15 / math.sin(0.5e-4), w=(0.0, 0.0, 0.3))
    return out


def lemma_pins():
    """Adversarial lemma inputs (a2_checks.py's and RV129's): the operand balls
    and the result's midpoint split and radius."""
    B, rnd = p.Ball, p.rnd
    rows = []
    def ball(b):
        terms, truncated = p.split_binary64(b.m)
        assert not truncated
        return "%d %s %s" % (len(terms), " ".join(bits(t) for t in terms), bits(b.r))
    def emit(name, op, out, a, b=None):
        rows.append("lemma %s %s %s %s -> %s" % (name, op, ball(a), ball(b) if b is not None else "-", ball(out)))
    a, b = B(rnd(Q(5, 11)), 1e-30), B(rnd(Q(3, 7)), 1e-25)
    emit("L1_add", "add", p.b_add(a, b), a, b)
    emit("L1_sub", "sub", p.b_sub(a, b), a, b)
    emit("L2_mul", "mul", p.b_mul(a, b), a, b)
    emit("L3_div", "div", p.b_div(a, b), a, b)
    mb = rnd(Q(3, 7))
    an, bn = B(rnd(Q(5, 11)), 0.0), B(mb, float(Q(mb) * (1 - Q(1, 2 ** 40))))
    emit("L3_div_near_singular", "div", p.b_div(an, bn), an, bn)
    s2 = B(rnd(Q(2)), 1e-20)
    emit("L4_sqrt_2", "sqrt", p.b_sqrt(s2), s2)
    for name, t in (("L5_atan_third_wide", B(rnd(Q(1, 3)), 0.3)), ("L5_atan_third_narrow", B(rnd(Q(1, 3)), 1e-35)),
                    ("L5_atan_1e5", B(rnd(Q(10 ** 5)), 1e-25))):
        emit(name, "atan", p.b_atan(t), t)
    emit("L6_half", "half", p.b_pow2(a, -1), a)
    return rows


def main():
    print("# T4-I17 V2b pins from arc_cert_probe_r01.py (DESIGN_R01 sections 3.2-3.3)")
    for label, args in cases():
        print("case %s" % label)
        flat = list(args[0]) + list(args[1]) + [args[2]] + list(args[3]) + list(args[4:11]) + list(args[11])
        print("args " + " ".join(bits(v) for v in flat))
        try:
            balls, rho = p.certify(*args)
        except p.CertificateFailure as error:
            if error.klass == 5:
                print("err 5 %s" % bits(p.invert.last_rho))
            else:
                print("err %d" % error.klass)
            print("end")
            continue
        for i, b in enumerate(balls):
            terms, _ = p.split_binary64(b.m)
            print("pin %d %d %s %s" % (i, len(terms), " ".join(bits(t) for t in terms), bits(b.r)))
        print("rho %s" % bits(rho))
        print("end")
        sys.stdout.flush()
    for row in lemma_pins():
        print(row)


if __name__ == "__main__":
    main()
