"""I68 U8-0 probe: the Python reader (core/analysis_runs/retained_precision.py) on each saved document.
Usage: python py_readers.py <archive P root> <doc.json>..."""
import json, sys, traceback, hashlib
from collections import Counter
from copy import deepcopy
root = sys.argv[1]
sys.path.insert(0, root)
from core.analysis_runs import retained_precision as rp
for path in sys.argv[2:]:
    raw = open(path, "rb").read()
    doc = json.loads(raw)
    name = path.rsplit("/", 1)[-1]
    for label, inv in (("bound", doc["invocation"]), ("unbound", None)):
        try:
            r = rp.validate_retained_precision(deepcopy(doc["source"]), deepcopy(inv))
            counts = Counter(c["class"] for c in r["classifications"])
            print(f"I68_PY {name} {label} PASS invocation_bound={r['invocation_bound']} numerical_eligible={r['numerical_eligible']} standing={r['standing']} classifications={len(r['classifications'])} classes={dict(sorted(counts.items()))} publication_sha256={r.get('publication_sha256')}")
        except rp.RetainedPrecisionError as e:
            frames = [f for f in traceback.extract_tb(e.__traceback__) if f.filename.endswith("retained_precision.py") and f.name not in ("_need",)]
            where = "; ".join(f"{f.name}:{f.lineno}" for f in frames[-3:])
            print(f"I68_PY {name} {label} FAIL gate={e.gate} code={e.code} at=[{where}]")
