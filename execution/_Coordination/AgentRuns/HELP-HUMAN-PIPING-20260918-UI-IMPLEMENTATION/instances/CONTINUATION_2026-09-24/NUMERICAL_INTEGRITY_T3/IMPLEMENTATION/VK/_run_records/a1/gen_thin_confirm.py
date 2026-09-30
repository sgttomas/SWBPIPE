"""ROOT's A1 ruling on THIN: K4's generator (GEN, the design's bit oracle) on
RF-RANGE-THIN-A and THIN-B, fed through GEN's own R1 adapter (`r1_adapt`,
K4's conventions: y_reference (0,0,1) unless parallel to Z, each input rounded
once, A, I, J = fl of the exact section with PI_Q).

GEN's adapter reads a section's G; R1 gives THIN's sections as E and nu. The
only change to R1's model before `r1_adapt` is therefore to state G exactly as
E/(2(1 + nu)) (THIN-B: 5/11; THIN-A: 5/11*2^200), which `references.py`'s
`parse_input` reads exactly and GEN rounds once, as R1's Model does
(G = E/(2(1+nu)), then decoded).

Usage: python3 -B gen_thin_confirm.py <path of gen_k4_vectors.py>
Standard library only; imports GEN read-only (GEN sets dont_write_bytecode).
"""
import sys
sys.dont_write_bytecode = True
import copy
import importlib.util
import json
from fractions import Fraction as Fr

spec = importlib.util.spec_from_file_location("gen_k4_vectors", sys.argv[1])
gen = importlib.util.module_from_spec(spec)
spec.loader.exec_module(gen)

ref = json.loads(gen.R1_JSON.read_text())["cases"]
for cid in ("RF-RANGE-THIN-A", "RF-RANGE-THIN-B"):
    c = ref[cid]
    model = copy.deepcopy(c["model"])
    for sid, sec in model["sections"].items():
        E = gen.r1.parse_input(sec["E"])
        nu = gen.r1.parse_input(sec.pop("nu"))
        G = E / (2 * (1 + nu))
        k = 0
        while G.numerator % 2 == 0 and G.numerator != 0:
            G /= 2
            k += 1
        sec["G"] = "%d/%d%s" % (G.numerator, G.denominator, ("*2^%d" % k) if k else "")
        assert gen.r1.parse_input(sec["G"]) == E / (2 * (1 + nu))
        print("%s section %s: G stated exactly as %s" % (cid, sid, sec["G"]))
    m, _kmem, _key_of = gen.r1_adapt(cid, c, model)
    mem = m["members"][0]
    print("%s member: %s" % (cid, {k: mem[k] for k in sorted(mem) if k in ("E", "G", "A", "Iy", "Iz", "J", "y")}))
    sel, attempts = gen.schedule_em(m)
    print("%s GEN schedule: selected %s" % (cid, sel))
    for p, what in attempts:
        print("   %d %s" % (p, what))
