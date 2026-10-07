"""Python transport dispatch (compatibility._source_contract(check_receipt=False)) on every RV92 probe.
Usage: transport_py.py <P root> <probe dir> <out.json>"""
import json, sys
from pathlib import Path
P, D, OUT = Path(sys.argv[1]), Path(sys.argv[2]), sys.argv[3]
sys.path.insert(0, str(P))
from core.analysis_runs import compatibility as c
out = {}
for item in json.loads((D / "index.json").read_text()):
    probe = json.loads((D / item["file"]).read_text())
    try:
        c._source_contract(probe["source"], check_receipt=False); r = "ok"
    except ValueError as error:
        r = str(error)
    except Exception as error:
        r = f"EXC {type(error).__name__}"
    out[probe["id"]] = r
json.dump(out, open(OUT, "w"), indent=0, sort_keys=True)
print(len(out))
