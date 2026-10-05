"""RV86 check 4b: absolute-class edge controls on rows outside the oracle's observable checks
(the first attempt on rigid Fx was confounded by the support-norm guard, which refused both sides).
Same harness as c4_controls.py: the unchanged U5 script, the real reader, a post-validation mutation."""
import json, math, struct, sys
from pathlib import Path
sys.argv = [sys.argv[0]] + sys.argv[1:]
src = Path(__file__).with_name("c4_controls.py").read_text()
harness = src.split("results = {}")[0]  # imports, reader, run(), helpers; no controls
exec(compile(harness, "c4_controls.py", "exec"))
def ival(x): return struct.unpack('>q', struct.pack('>d', x))[0]
def fval(i): return struct.unpack('>d', struct.pack('>q', i))[0]
out = {}
doc = json.loads(Path(sparse).read_text())
for rid in ["result:force:M1:axial", "result:disp:N1:ux", "result:stress:M1:end-i:bending-normal-y"]:
    r0 = row(doc["source"], rid); unit = r0["unit"]; sign = 1.0 if r0["value"] >= 0 else -1.0
    probe = {}
    def ok(i):
        x = fval(i)
        if i not in probe: probe[i] = run([sparse], lambda s, v: set_value(s, v, rid, x), "edge4b")["result"] == ("exit", 0)
        return probe[i]
    # the published bound in SI, mapped to the published unit
    b = None
    def grab(s, v):
        global b; b = dec(cls(v, rid)["bound_bits"])
    run([sparse], grab, "edge4b_grab")
    x0 = sign * (b * 1000.0 if unit == "mm" else b / 1e6 if unit == "MPa" else b)
    lo, hi = ival(x0) - 2**12, ival(x0) + 2**12   # magnitude increases with the integer, both signs
    assert ok(lo) and not ok(hi), (rid, ok(lo), ok(hi))
    while hi - lo > 1:
        mid = (lo + hi) // 2
        if ok(mid): lo = mid
        else: hi = mid
    inside, outside = fval(lo), fval(hi)
    both_in = run([sparse, dense], lambda s, v: set_value(s, v, rid, inside), "edge4b_in")
    both_out = run([sparse, dense], lambda s, v: set_value(s, v, rid, outside), "edge4b_out")
    out[rid] = {"unit": unit, "bound_si": repr(b), "inside_raw": repr(inside), "outside_raw": repr(outside), "ulps_apart": hi - lo,
                "inside": both_in, "outside": both_out}
print(json.dumps(out, indent=1))
