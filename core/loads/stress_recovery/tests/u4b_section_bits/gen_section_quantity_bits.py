#!/usr/bin/env python3
"""Expected-bit vectors for the T4-U4b section quantities in
`core/loads/stress_recovery/src/section_stress_quantities.rs` (DEL-04-07 Design
`u4b-u5b-implementation.md` sections 2.3-2.6; the hoop sequence is normative,
so readers recompute it bit for bit).

Standard library only. Independent of the Rust implementation: every
operation of each normative sequence is evaluated exactly in
`fractions.Fraction` and rounded once to nearest, ties to even, by CPython's
correctly rounded int/int division (`float(Fraction)`). `norm2(a, b)` is
RN(sqrt(a^2 + b^2)) found exactly: a binary64 candidate is moved until both
neighbouring midpoints bracket the exact sum of squares (ties to even).

Usage:  python3 -I -B gen_section_quantity_bits.py [--check]

Writes, beside this script:
  section_quantity_bits.json  the vectors (every binary64 as its 16-hex-digit
                              IEEE 754 bit pattern), the sequences in words,
                              provenance and the source binding
  SHA256SUMS                  sha256 of the JSON file
With --check, nothing is written; the file is regenerated in memory and
compared byte for byte with the committed one. The JSON is meant to be read
unmodified by the Python verifier as well as by the Rust unit test.

Sources: the binary64 members of `validation/references/t4_u4b/oracle.json`
(bound by sha256) for the reference sections, materials, resultants and hoop
parameters; seeded synthetic operands for rounding-sensitive cases; and the
near-edge hoop cases of the K2 review (D = 1.2192 m, t = 6.35 mm,
R a few ulps above r_o, where the intrados amplifies rounding by about 2R/t).
"""
import hashlib
import json
import math
import random
import struct
import sys
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent
ORACLE = HERE.parents[4] / "validation" / "references" / "t4_u4b" / "oracle.json"


def rn(q):
    """One round-to-nearest-even of an exact rational."""
    return float(q)


def bits(x):
    return "%016x" % struct.unpack("<Q", struct.pack("<d", x))[0]


def value(token):
    """The binary64 of a 16-hex-digit bit pattern."""
    return struct.unpack(">d", bytes.fromhex(token))[0]


def norm2(a, b):
    """RN(sqrt(a^2 + b^2)), exactly."""
    target = F(a) * F(a) + F(b) * F(b)
    if target == 0:
        return 0.0
    y = math.sqrt(float(target))
    for _ in range(64):
        up, down = math.nextafter(y, math.inf), math.nextafter(y, 0.0)
        hi_mid = (F(y) + F(up)) / 2
        lo_mid = (F(y) + F(down)) / 2
        if hi_mid * hi_mid < target:
            y = up
        elif lo_mid * lo_mid > target:
            y = down
        else:
            # Ties to even.
            if hi_mid * hi_mid == target and struct.unpack("<Q", struct.pack("<d", y))[0] & 1:
                y = up
            elif lo_mid * lo_mid == target and struct.unpack("<Q", struct.pack("<d", y))[0] & 1:
                y = down
            return y
    raise AssertionError("norm2 did not settle")


# ---------------------------------------------------------------------------
# The normative sequences
# ---------------------------------------------------------------------------

SEQUENCES = {
    "saint_venant_shear_factor": "c_b = RN(num/den), num = RN(RN(RN(3+2nu)*RN(ro*ro)) + RN(RN(1+2nu)*RN(ri*ri))), den = RN(RN(4*RN(1+nu))*I); 2nu and 4x are exact",
    "transverse_shear_maximum_mpa": "tau = RN(RN(norm2(V_y, V_z)*c_b)/1e6)",
    "normal_stress_extremes": "h = norm2(sigma_y, sigma_z); sigma_plus = RN(sigma_N + h); sigma_minus = RN(sigma_N - h) (MPa)",
    "normal_stress_extremes_membrane": "sigma_N = RN(pipe_axial_membrane_stress_v2/1e6) (Pa to MPa), then normal_stress_extremes",
    "directional_intensified_bending_pa": "RN(1e6*norm2(RN(i_ip*sigma_z), RN(i_op*sigma_y))) (Pa)",
    "toroidal_membrane_hoop": "a = RN(RN(p*ri)/t); s = RN(ri + ro); 2R exact; crown = a; intrados = RN(a*RN(RN(2R - ri)/RN(2R - s))); extrados = RN(a*RN(RN(2R + ri)/RN(2R + s))); refused unless R > ro (exact comparison) and RN(2R - s) > 0",
    "toroidal_radial_surface": "inner = -p, outer = +0; refused unless R > ro (exact comparison)",
}


def shear_factor(nu, ro, ri, inertia):
    num = rn(F(rn(F(rn(3 + 2 * F(nu))) * F(rn(F(ro) * F(ro))))) + F(rn(F(rn(1 + 2 * F(nu))) * F(rn(F(ri) * F(ri))))))
    den = rn(F(rn(4 * F(rn(1 + F(nu))))) * F(inertia))
    return rn(F(num) / F(den))


def shear_factor_reassociated(nu, ro, ri, inertia):
    """(3+2nu)*ro*ro chained left to right: a different order, for the record."""
    num = rn(F(rn(F(rn(F(rn(3 + 2 * F(nu))) * F(ro))) * F(ro))) + F(rn(F(rn(1 + 2 * F(nu))) * F(rn(F(ri) * F(ri))))))
    den = rn(F(rn(4 * F(rn(1 + F(nu))))) * F(inertia))
    return rn(F(num) / F(den))


def shear_maximum(v_y, v_z, factor):
    return rn(F(rn(F(norm2(v_y, v_z)) * F(factor))) / 1000000)


def extremes(sigma_n, sigma_y, sigma_z):
    h = norm2(sigma_y, sigma_z)
    return rn(F(sigma_n) + F(h)), rn(F(sigma_n) - F(h))


def intensified(i_ip, i_op, sigma_y, sigma_z):
    return rn(1000000 * F(norm2(rn(F(i_ip) * F(sigma_z)), rn(F(i_op) * F(sigma_y)))))


def hoop(p, ri, t, ro, R):
    assert R > ro
    a = rn(F(rn(F(p) * F(ri))) / F(t))
    two_r = 2 * R
    s = rn(F(ri) + F(ro))
    den_i = rn(F(two_r) - F(s))
    assert den_i > 0
    intrados = rn(F(a) * F(rn(F(rn(F(two_r) - F(ri))) / F(den_i))))
    extrados = rn(F(a) * F(rn(F(rn(F(two_r) + F(ri))) / F(rn(F(two_r) + F(s))))))
    return intrados, a, extrados


def hoop_exact(p, ri, t, ro, R):
    """The closed form at the binary64 operands, exactly (conditioning record)."""
    a = F(p) * F(ri) / F(t)
    return a * (2 * F(R) - F(ri)) / (2 * F(R) - F(ri) - F(ro))


# ---------------------------------------------------------------------------
# Vectors
# ---------------------------------------------------------------------------

STATIONS = ("end_i", "quarter_1", "midspan", "quarter_3", "end_j")


def h(x):
    return bits(x)


def vectors():
    raw = ORACLE.read_bytes()
    oracle = json.loads(raw)
    out = []
    record = {"oracle_sha256": hashlib.sha256(raw).hexdigest()}
    reassociation_differs = 0
    factors = 0
    for cid, case in oracle["cases"].items():
        q = case["quantities"]
        for mid, derived in case["derived"].items():
            nu = derived["nu"][1]
            sif = derived.get("directional_sif")
            for basis, sec in derived["section_bases"].items():
                ro, ri, inertia, area, z = (sec[k][1] for k in ("ro", "ri", "I", "A", "Z"))
                cb = shear_factor(nu, ro, ri, inertia)
                factors += 1
                if shear_factor_reassociated(nu, ro, ri, inertia) != cb:
                    reassociation_differs += 1
                base = f"{cid}/{mid}/{basis}"
                out.append({"id": f"c_b/{base}", "quantity": "saint_venant_shear_factor",
                            "inputs": {"nu": h(nu), "outer_radius": h(ro), "inner_radius": h(ri), "second_moment": h(inertia)},
                            "expected": {"c_b": h(cb)}})
                for st in STATIONS:
                    r = q["resultants_tangent_frame_j_side"][mid][st]
                    vy, vz = r["V_y"][1], r["V_z"][1]
                    out.append({"id": f"tau/{base}/{st}", "quantity": "transverse_shear_maximum_mpa",
                                "inputs": {"shear_y": h(vy), "shear_z": h(vz), "c_b": h(cb)},
                                "expected": {"tau_v_max": h(shear_maximum(vy, vz, cb))}})
                    sy, sz = r["M_y"][1] / z / 1e6, r["M_z"][1] / z / 1e6
                    if "N_w" in r:
                        membrane = r["N_w"][1] / area
                        sn = rn(F(membrane) / 1000000)
                        plus, minus = extremes(sn, sy, sz)
                        out.append({"id": f"sigma_membrane/{base}/{st}", "quantity": "normal_stress_extremes_membrane",
                                    "inputs": {"membrane_pa": h(membrane), "sigma_y": h(sy), "sigma_z": h(sz)},
                                    "expected": {"sigma_n": h(sn), "sigma_plus": h(plus), "sigma_minus": h(minus)}})
                    else:
                        sn = r["N"][1] / area / 1e6
                        plus, minus = extremes(sn, sy, sz)
                        out.append({"id": f"sigma_axial/{base}/{st}", "quantity": "normal_stress_extremes",
                                    "inputs": {"sigma_n": h(sn), "sigma_y": h(sy), "sigma_z": h(sz)},
                                    "expected": {"sigma_plus": h(plus), "sigma_minus": h(minus)}})
                    if sif:
                        i_ip, i_op = sif["i_ip"][1], sif["i_op"][1]
                        out.append({"id": f"intensified/{base}/{st}", "quantity": "directional_intensified_bending_pa",
                                    "inputs": {"in_plane_factor": h(i_ip), "out_of_plane_factor": h(i_op), "sigma_y": h(sy), "sigma_z": h(sz)},
                                    "expected": {"intensified": h(intensified(i_ip, i_op, sy, sz))}})
        for mid, by_basis in q.get("toroidal_membrane_hoop", {}).items():
            for basis, hp in by_basis.items():
                p = hp["parameters"]
                args = [p[k][1] for k in ("p", "ri", "t", "ro", "R")]
                out.append(hoop_vector(f"hoop/{cid}/{mid}/{basis}", *args))
    # Codex's case (K2 review): the normative order gives ...73, a
    # reassociated order ...72.
    codex = next(v for v in out if v["id"] == "c_b/U4B-L90-P/pipe:S1/reduced_section")
    assert codex["expected"]["c_b"] == "4080cfbe845b9c73", codex
    i = codex["inputs"]
    alt = shear_factor_reassociated(*(value(i[k]) for k in ("nu", "outer_radius", "inner_radius", "second_moment")))
    assert bits(alt) == "4080cfbe845b9c72", bits(alt)

    # Seeded rounding-sensitive synthetic operands, nonzero nu.
    rng = random.Random(20261017)
    for n in range(60):
        nu = rng.choice([rng.uniform(-0.95, 0.495), 0.27, 0.31, 0.29, 0.26, 0.28, 0.3])
        ro = rng.uniform(0.01, 0.8)
        ri = ro * rng.uniform(0.55, 0.995)
        inertia = math.pi * (ro ** 4 - ri ** 4) / 4
        cb = shear_factor(nu, ro, ri, inertia)
        factors += 1
        if shear_factor_reassociated(nu, ro, ri, inertia) != cb:
            reassociation_differs += 1
        out.append({"id": f"c_b/synthetic/{n}", "quantity": "saint_venant_shear_factor",
                    "inputs": {"nu": h(nu), "outer_radius": h(ro), "inner_radius": h(ri), "second_moment": h(inertia)},
                    "expected": {"c_b": h(cb)}})
        vy, vz = rng.uniform(-5e4, 5e4), rng.uniform(-5e4, 5e4)
        out.append({"id": f"tau/synthetic/{n}", "quantity": "transverse_shear_maximum_mpa",
                    "inputs": {"shear_y": h(vy), "shear_z": h(vz), "c_b": h(cb)},
                    "expected": {"tau_v_max": h(shear_maximum(vy, vz, cb))}})
        sy, sz = rng.uniform(-200, 200), rng.uniform(-200, 200)
        # sigma_N near -h and near +h exercises the cancellation in sigma_minus/plus.
        hn = norm2(sy, sz)
        for tag, sn in (("near_minus", math.nextafter(hn, 0.0)), ("near_plus", -math.nextafter(hn, 0.0)), ("free", rng.uniform(-100, 100))):
            plus, minus = extremes(sn, sy, sz)
            out.append({"id": f"sigma_axial/synthetic/{n}/{tag}", "quantity": "normal_stress_extremes",
                        "inputs": {"sigma_n": h(sn), "sigma_y": h(sy), "sigma_z": h(sz)},
                        "expected": {"sigma_plus": h(plus), "sigma_minus": h(minus)}})
        membrane = rng.uniform(-1e8, 1e8)
        sn = rn(F(membrane) / 1000000)
        plus, minus = extremes(sn, sy, sz)
        out.append({"id": f"sigma_membrane/synthetic/{n}", "quantity": "normal_stress_extremes_membrane",
                    "inputs": {"membrane_pa": h(membrane), "sigma_y": h(sy), "sigma_z": h(sz)},
                    "expected": {"sigma_n": h(sn), "sigma_plus": h(plus), "sigma_minus": h(minus)}})
        i_ip, i_op = rng.uniform(1.0, 4.0), rng.uniform(1.0, 4.0)
        out.append({"id": f"intensified/synthetic/{n}", "quantity": "directional_intensified_bending_pa",
                    "inputs": {"in_plane_factor": h(i_ip), "out_of_plane_factor": h(i_op), "sigma_y": h(sy), "sigma_z": h(sz)},
                    "expected": {"intensified": h(intensified(i_ip, i_op, sy, sz))}})

    # The near-edge hoop (K2 review): D = 1.2192 m, t = 6.35 mm.
    ro = 1.2192 / 2
    t = 0.00635
    ri = ro - t
    worst_amplification = 0.0
    for p in (1.0e6, 7.5e6, -2.5e5):
        r = ro
        for step in range(1, 6):
            r = math.nextafter(r, math.inf)
            vector = hoop_vector(f"hoop/near_edge/p{p:g}/ro_plus_{step}ulp", p, ri, t, ro, r)
            out.append(vector)
            exact = hoop_exact(p, ri, t, ro, r)
            got = value(vector["expected"]["intrados"])
            amplification = float(abs(F(got) - exact) / abs(exact)) / (2 ** -53)
            worst_amplification = max(worst_amplification, amplification)
        for factor in (1.000001, 1.01, 1.5, 3.0):
            out.append(hoop_vector(f"hoop/near_edge/p{p:g}/R_{factor:g}ro", p, ri, t, ro, ro * factor))
    record["hoop_near_edge_worst_intrados_error_units_of_2^-53"] = round(worst_amplification, 3)
    record["c_b_vectors"] = factors
    record["c_b_vectors_where_the_reassociated_order_differs"] = reassociation_differs
    return out, record


def hoop_vector(vid, p, ri, t, ro, R):
    intrados, crown, extrados = hoop(p, ri, t, ro, R)
    return {"id": vid, "quantity": "toroidal_membrane_hoop",
            "inputs": {"pressure": h(p), "inner_radius": h(ri), "wall": h(t), "outer_radius": h(ro), "bend_radius": h(R)},
            "expected": {"intrados": h(intrados), "crown": h(crown), "extrados": h(extrados),
                         "radial_inner": h(-p), "radial_outer": h(0.0)}}


def generate():
    out, record = vectors()
    document = {
        "schema": "u4b.section_quantity_bits/1",
        "purpose": "Expected binary64 bits of the T4-U4b section quantities (u4b-u5b-implementation.md sections 2.3-2.6), for the stress_recovery unit test and the Python verifier",
        "claim_fence": "Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).",
        "provenance": {
            "generator": "core/loads/stress_recovery/tests/u4b_section_bits/gen_section_quantity_bits.py (standard library)",
            "method": "every operation of the normative sequence evaluated exactly in fractions.Fraction and rounded once to nearest, ties to even; norm2 = RN(sqrt(a^2 + b^2)) exactly; independent of the Rust implementation",
            "oracle": "validation/references/t4_u4b/oracle.json (binary64 members only)",
            **record,
        },
        "encoding": "every binary64 is its 16-hex-digit IEEE 754 bit pattern",
        "sequences": SEQUENCES,
        "vectors": out,
    }
    text = json.dumps(document, indent=1, sort_keys=False) + "\n"
    sums = f"{hashlib.sha256(text.encode()).hexdigest()}  section_quantity_bits.json\n"
    return {"section_quantity_bits.json": text, "SHA256SUMS": sums}, record


def main():
    files, record = generate()
    print(json.dumps(record))
    if "--check" in sys.argv[1:]:
        bad = [n for n, text in files.items() if (HERE / n).read_text() != text]
        if bad:
            print("MISMATCH:", ", ".join(bad))
            sys.exit(1)
        print("check: all files match")
        return
    for name, text in files.items():
        (HERE / name).write_text(text)
    print("wrote", ", ".join(sorted(files)))


if __name__ == "__main__":
    main()
