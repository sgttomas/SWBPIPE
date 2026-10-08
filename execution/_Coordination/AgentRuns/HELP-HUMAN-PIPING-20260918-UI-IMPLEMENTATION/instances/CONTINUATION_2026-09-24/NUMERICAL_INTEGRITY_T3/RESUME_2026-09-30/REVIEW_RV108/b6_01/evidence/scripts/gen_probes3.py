"""RV108 probe batch 3: transported statements with malformed raw rows (transport reads no rows)."""
import json, sys
from copy import deepcopy
from pathlib import Path
PROOT = Path(sys.argv[1]); OUT = Path(sys.argv[2]); sys.path.insert(0, str(PROOT)); sys.path.insert(0, sys.argv[3])
import gen_probes_lib as L
corpus = json.loads((PROOT / "fixtures/results/retained_precision_cases.json").read_text()); cases = {c["id"]: c for c in corpus["cases"]}
bases = {"ordinary_prepared_synthetic": (cases["ordinary_prepared_synthetic"]["source"], cases["ordinary_prepared_synthetic"]["invocation"])}
for mode in ("sparse_interactive", "dense_scrutiny"):
    doc = json.loads((PROOT / f"fixtures/results/retained_precision_milestone_successor_{mode}.json").read_text()); bases[f"milestone_{mode}"] = (doc["source"], doc["invocation"])
probes = []
for base in bases:
    for name, fn in (("results[0]=null", lambda s: L.setp(s, ["results", 0], None)), ("results+=null", lambda s: s["results"].append(None)),
                     ("results=str", lambda s: L.setp(s, ["results"], "rows")), ("results=null", lambda s: L.setp(s, ["results"], None)),
                     ("results[0]=int", lambda s: L.setp(s, ["results", 0], 1)), ("results=[]", lambda s: L.setp(s, ["results"], []))):
        s, inv = deepcopy(bases[base][0]), deepcopy(bases[base][1]); fn(s)
        probes.append({"id": f"{base}|T.{name}", "family": "D_rows", "base": base, "source": s, "invocation": inv})
OUT.write_text(json.dumps(probes)); print(len(probes))
