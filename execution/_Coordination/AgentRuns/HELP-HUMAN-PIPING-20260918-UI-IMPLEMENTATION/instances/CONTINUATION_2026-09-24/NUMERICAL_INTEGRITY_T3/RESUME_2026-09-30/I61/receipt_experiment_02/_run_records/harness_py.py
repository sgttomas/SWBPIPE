"""I61 disposable harness: run READER@b36739112a's Python draft validator (unchanged) on
emitted receipts, plus a jsonschema listing of every shape violation (diagnostic aid only;
the reader's own verdict is the result)."""
import json, sys, traceback
from pathlib import Path
import jsonschema

P = Path(sys.argv[1])            # READER archive P root (cwd)
files = [Path(f) for f in sys.argv[2:]]
sys.path.insert(0, str(P))
from core.analysis_runs import retained_precision as rp

schema = json.loads((P / "schemas/retained_precision_mp_v2.schema.json").read_text())
validator = jsonschema.Draft202012Validator(schema)
for f in files:
    case = json.loads(f.read_text())
    source, invocation = case["source"], case["invocation"]
    errors = sorted(validator.iter_errors(source["retained_precision"]), key=lambda e: list(e.absolute_path))
    print(f"== {case['id']}: schema violations {len(errors)}")
    for e in errors[:40]:
        print("   SCHEMA", "/".join(map(str, e.absolute_path)), "::", e.message[:220])
    try:
        result = rp._validate_draft(source, invocation)
        print("   PY RESULT standing=%s eligible=%s invocation_bound=%s classes=%d" % (result["standing"], result["numerical_eligible"], result["invocation_bound"], len(result["classifications"])))
        Path(str(f) + ".py_classes.txt").write_text("\n".join("%s|%s|%s|%s" % (c["result_id"], c["normalized_bits"], c["scale_bits"] or "null", c["class"]) for c in result["classifications"]))
    except rp.RetainedPrecisionError as e:
        print("   PY FIRST", e.gate, e.code, "detail:", (e.detail or "")[:300])
        tb = traceback.extract_tb(e.__traceback__)
        print("   PY AT", " <- ".join(f"{Path(t.filename).name}:{t.lineno}" for t in tb[-4:]))
