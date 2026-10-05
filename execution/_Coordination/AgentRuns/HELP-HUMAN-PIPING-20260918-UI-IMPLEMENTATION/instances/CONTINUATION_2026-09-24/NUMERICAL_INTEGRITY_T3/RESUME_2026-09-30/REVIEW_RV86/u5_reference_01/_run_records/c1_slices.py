"""RV86 check 1: the U5 script's oracle slices are unmodified, uniquely anchored oracle text."""
import hashlib, re, sys
from pathlib import Path
oracle_path, script_path = sys.argv[1:3]
ob = Path(oracle_path).read_bytes(); o = ob.decode()
s = Path(script_path).read_text()
print("oracle sha256", hashlib.sha256(ob).hexdigest())
anchors = {
 "helpers_end": "def check(record):",
 "derive_end": "    rows=record['envelope']['results'];vs=record['verdicts']",
 "obs_start": "    byid={r['id']:r for r in rows};assert len(byid)==len(rows)",
 "obs_end": "    return {'mode':record['mode']",
}
pos = {}
for k, a in anchors.items():
    assert a in s, ("anchor text present in script", k)
    n = o.count(a); pos[k] = o.index(a)
    line = o[:pos[k]].count("\n") + 1
    print(f"{k}: occurrences={n} line={line} at-line-start={pos[k]==0 or o[pos[k]-1]=='\\n'}")
assert pos["helpers_end"] < pos["derive_end"] < pos["obs_start"] < pos["obs_end"]
# What the script executes from the oracle, and what it omits.
used = [(0, pos["helpers_end"]), (pos["helpers_end"], pos["derive_end"]), (pos["obs_start"], pos["obs_end"])]
omitted = [(pos["derive_end"], pos["obs_start"]), (pos["obs_end"], len(o))]
ln = lambda p: o[:p].count("\n") + 1
for a, b in used: print("executed oracle lines", ln(a), "-", ln(b) - 1)
for a, b in omitted: print("omitted oracle lines", ln(a), "-", ln(b) - 1 if b < len(o) else ln(b))
# The only glue added to executed oracle text:
glue = re.findall(r'exec\((.*?)\)\n', s)
print("exec calls:", glue)
# Tolerance constants in the glue versus the oracle's own expressions.
for expr in ["abs(F(n))/10**9", "abs(F(r['value']))/10**9", "F(2)**-64*max(abs(F(n)),F(scale))*(1+F(2)**-21)+F(2)**-53*abs(F(n))+F(2)**-1074"]:
    print("oracle has", repr(expr), expr in o)
for expr in ["abs(F(n)) / 10**9", "abs(F(r[\"value\"])) / 10**9", "F(2)**-64 * max(abs(F(n)), scale) * (1 + F(2)**-21) + F(2)**-53 * abs(F(n)) + F(2)**-1074", "verdict(lo, hi, F(0))", "F(dec(c[\"bound_bits\"]))"]:
    print("script has", repr(expr), expr in s)
# verdict rule identical to the oracle's
print("oracle verdict rule", "verdict='pass' if hi<=a else 'fail' if lo>a else 'unproved'" in o)
print("script verdict rule", 'return "pass" if hi <= allowance else "fail" if lo > allowance else "unproved"' in s)
