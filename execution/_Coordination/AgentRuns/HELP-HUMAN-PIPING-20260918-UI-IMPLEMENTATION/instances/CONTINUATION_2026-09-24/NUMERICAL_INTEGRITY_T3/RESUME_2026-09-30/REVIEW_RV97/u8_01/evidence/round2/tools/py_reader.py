"""RV97 round 2: the Python reader's public entry on every document. Usage: py_reader.py P DOCS_DIR OUT"""
import json, sys
from pathlib import Path
P, docs, out = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3])
sys.path.insert(0, str(P))
from core.analysis_runs import retained_precision as rp
with out.open("w") as w:
    for f in sorted(docs.glob("*.json")):
        d = json.loads(f.read_text())
        try:
            r = rp.validate_retained_precision(d["source"], d["invocation"])
            rows = [[x["result_id"], x["class"], x["normalized_bits"], x["scale_bits"], x["bound_bits"]] for x in r["classifications"]]
            line = {"label": d["label"], "ok": True, "bound": r["invocation_bound"], "eligible": r["numerical_eligible"], "standing": r["standing"],
                    "publication_sha256": r.get("publication_sha256"), "classifications": rows}
        except rp.RetainedPrecisionError as e:
            line = {"label": d["label"], "ok": False, "gate": e.gate, "code": e.code}
        w.write(json.dumps(line) + "\n")
