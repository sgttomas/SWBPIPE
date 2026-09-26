"""Per-case accept/refuse agreement of the three recorded reader logs."""
import json, sys
from pathlib import Path
root = Path(sys.argv[1])
def load(p): return {e["id"]: e for e in json.loads(p.read_text())}
for ident, ts_name in (("lr", "ts_outcomes.json"), ("lrs", "ts_source_outcomes.json")):
    rust = {**load(root/ident/"rust_outcomes.json"), **load(root/ident/"rust_table_transport_outcomes.json")}
    py = load(root/ident/"python_outcomes.json"); ts = load(root/"ts"/ts_name)
    assert set(rust) == set(py) == set(ts), (ident, len(rust), len(py), len(ts))
    agree = disagree = 0; kinds = {}
    for cid in ts:
        for key in ("dispatch", "validator", "joined", "lr", "ps", "table", "transport"):
            vals = [log[cid].get(key) for log in (rust, py, ts)]
            if any(v is None for v in vals): continue
            classes = {v == "accept" for v in vals}
            kinds[key] = kinds.get(key, 0) + 1
            if len(classes) == 1: agree += 1
            else: disagree += 1; print("DISAGREE", ident, cid, key, vals)
    print(f"{ident}: {len(ts)} entries; {agree} compared outcomes agree, {disagree} disagree; per key {kinds}")
