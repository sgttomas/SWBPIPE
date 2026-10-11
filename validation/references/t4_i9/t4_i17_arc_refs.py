"""T4-I17 (T4-U1b): the frozen arc load-vector references consumed by
FK `arc_certificate_tests.rs` (V2 enclosure) and CB `load_vector_tests.rs`
(T4-RV8 C-1: the stable small-angle load vector).

Standard library only. Run from this directory:
    python3 -I t4_i17_arc_refs.py _run_records/rv129_ref.py > arc_load_references.txt

The reference is RV129's independent method (`rv129_ref.reference`: Decimal
120 digits, Gauss-Legendre 56 x 56 nodes, its own trig, centre-based frame,
global unit loads; no closed form shared with CB or the certificate), copied
byte for byte into `_run_records/`. Each case is also evaluated at 140 digits
with 64 x 64 nodes; e bounds the generator's own error from above as
1000 |ref_120 - ref_140| + |the 4-term binary64 residual| + 1e-100 max|ref|,
rounded up to binary64.

Output (one case per block; every float as the 16-hex-digit bits of its
binary64 value):
    case <label>
    phi <decimal phi from the reference>
    args <xi0 xi1 xi2> <xj0 xj1 xj2> <R> <y0 y1 y2> <E> <G> <A> <I> <J> <k_in> <k_out> <w0 w1 w2>
    ref <i> <t0> <t1> <t2> <t3> <e>          (12 lines; t0..t3 sum exactly to the stored reference)
    end
"""
import importlib.util
import math
import random
import struct
import sys
from decimal import Decimal, localcontext
from fractions import Fraction as Q

spec = importlib.util.spec_from_file_location("rv129_ref", sys.argv[1])
ref = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ref)


def bits(x):
    return "%016x" % struct.unpack("<Q", struct.pack("<d", float(x)))[0]


def up(q):
    f = float(q)
    if Q(f) < q:
        f = math.nextafter(f, math.inf)
    return f


def terms4(x):
    out, r = [], x
    for _ in range(4):
        t = float(r)
        out.append(t)
        r = r - Q(t)
    return out, abs(r)


def near_pi_case(rng, target):
    """RV129 rv129_near_pi.py's construction, copied: a skew chord with |d| = L
    in binary64 components and R the smallest binary64 value with 2R > L."""
    best = None
    for _ in range(200000):
        a = rng.uniform(0.2, 1.3)
        d = [0.4572 * math.cos(a), 0.4572 * math.sin(a), 0.0]
        L = Q(d[0]) ** 2 + Q(d[1]) ** 2
        half = math.sqrt(float(L)) / 2
        R = half
        for _ in range(4):
            R = math.nextafter(R, 0.0)
        while Q(R) ** 2 * 4 <= L:
            R = math.nextafter(R, math.inf)
        one_minus_s2 = 1 - L / (4 * Q(R) ** 2)
        if best is None or abs(math.log10(float(one_minus_s2)) - math.log10(target)) < best[0]:
            best = (abs(math.log10(float(one_minus_s2)) - math.log10(target)), d, R, one_minus_s2)
            if best[0] < 0.3:
                break
    _, d, R, _ = best
    return [0.0, 0.0, 0.0], [d[0], d[1], 0.0], R


def cases():
    out = [(c["label"], c["args"]) for c in ref.build_cases()]
    out += [(c["label"], c["args"]) for c in ref.build_extra_cases()]
    # near pi (RV129's skew chords; seed and targets as rv129_near_pi.py)
    rng = random.Random(1291)
    area, inertia, torsion = ref.section(0.1683, 0.00711)
    for target in (1e-12, 1e-15, 1e-17, 1e-19):
        xi, xj, R = near_pi_case(rng, target)
        d = [xj[k] - xi[k] for k in range(3)]
        out.append(("near pi 1-s^2~%g" % target, (xi, xj, R, [-d[1], d[0], 0.0], 2.03e11, 2.03e11 / 2.6,
                                                   area, inertia, torsion, 1.0, 1.0, [0.0, 0.0, -450.0])))
    # T4-RV8 C-1: a 0.3 m chord along x, y = (0, 1, 0), OD 0.168 m, wall 0.007 m,
    # E 200 GPa, G 80 GPa, k = 1, R = 0.15/sin(phi/2); global z and in-plane -y loads.
    ro, ri = 0.084, 0.084 - 0.007
    a_ = math.pi * (ro * ro - ri * ri)
    i_ = math.pi * (ro ** 4 - ri ** 4) / 4
    for label, phi in (("1e-8 rad", 1e-8), ("1e-4 rad", 1e-4), ("1e-3 rad", 1e-3), ("1 deg", math.radians(1.0)),
                       ("5 deg", math.radians(5.0)), ("15 deg", math.radians(15.0)), ("59 deg", math.radians(59.0)),
                       ("61 deg", math.radians(61.0)), ("90 deg", math.radians(90.0))):
        R = 0.15 / math.sin(phi / 2)
        for wl, w in (("global z", [0.0, 0.0, 0.3]), ("in-plane -y", [0.0, -0.3, 0.0])):
            out.append(("C-1 chord 0.3 m %s %s" % (label, wl),
                        ([0.0, 0.0, 0.0], [0.3, 0.0, 0.0], R, [0.0, 1.0, 0.0], 200e9, 80e9, a_, i_, 2 * i_,
                         1.0, 1.0, w)))
    # T15's body (s11g_tests.rs curved_body at origin 0): chord (2, 0, 0), R = sqrt 2.
    area, inertia, torsion = ref.section(0.168, 0.007)
    for k in (2.0, 1e10):
        out.append(("T15 body k=%g" % k, ([0.0, 0.0, 0.0], [2.0, 0.0, 0.0], math.sqrt(2.0), [0.0, 1.0, 0.0],
                                          200e9, 80e9, area, inertia, torsion, k, k, [0.0, 0.0, 0.3])))
    return out


def main():
    print("# T4-I17 arc load references (rv129_ref.reference, 120 digits / 56 nodes; check 140 / 64)")
    for label, args in cases():
        f1, phi, _ = ref.reference(*args)
        f2, _, _ = ref.reference(*args, prec=140, nodes=64, inner=64)
        q1 = [Q(x) for x in f1]
        q2 = [Q(x) for x in f2]
        scale = max(abs(x) for x in q1)
        print("case %s" % label)
        with localcontext() as ctx:
            ctx.prec = 20
            print("phi %s" % (+phi))
        flat = list(args[0]) + list(args[1]) + [args[2]] + list(args[3]) + list(args[4:11]) + list(args[11])
        print("args " + " ".join(bits(v) for v in flat))
        for i in range(12):
            t, resid = terms4(q1[i])
            e = up(1000 * abs(q1[i] - q2[i]) + resid + scale / Q(10) ** 100)
            print("ref %d %s %s" % (i, " ".join(bits(x) for x in t), bits(e)))
        print("end")
        sys.stdout.flush()


if __name__ == "__main__":
    main()
