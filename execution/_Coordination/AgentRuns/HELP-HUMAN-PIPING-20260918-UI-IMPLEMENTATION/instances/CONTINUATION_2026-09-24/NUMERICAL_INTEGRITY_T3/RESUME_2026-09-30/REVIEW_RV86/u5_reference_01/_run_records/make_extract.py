"""RV86: derive the U5 oracle inputs from I50's captured log (sha256 5ced66b5..., 1,059,560 bytes).
Keeps, per I50_RECORD line and in log order, only the four fields U5's reuse of the oracle reads:
mode, request (equal to the committed fixture sha256 2aa51bee...), source (I50's captured source map)
and facts (I50's represented section facts). Usage: make_extract.py I50_LOG OUT"""
import hashlib, json, sys
from pathlib import Path
log, out = sys.argv[1:3]
assert hashlib.sha256(Path(log).read_bytes()).hexdigest() == "5ced66b5e030db563298dac2d4385345a0d3fcd2c05514f86df0b43c18270b69"
recs = [json.loads(l.split("I50_RECORD ", 1)[1]) for l in Path(log).read_text().splitlines() if "I50_RECORD " in l]
assert [r["mode"] for r in recs] == ["sparse_interactive", "dense_scrutiny"]
Path(out).write_text("\n".join("I50_RECORD " + json.dumps({k: r[k] for k in ["mode", "request", "source", "facts"]}, sort_keys=True, separators=(",", ":")) for r in recs) + "\n")
print(hashlib.sha256(Path(out).read_bytes()).hexdigest())
