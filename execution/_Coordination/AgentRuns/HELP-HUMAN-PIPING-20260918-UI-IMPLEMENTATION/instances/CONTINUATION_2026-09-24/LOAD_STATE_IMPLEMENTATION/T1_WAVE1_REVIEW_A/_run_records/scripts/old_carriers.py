"""Every pre-existing carrier in fixtures/results keeps its validity and branch: base schemas vs candidate schemas."""
import glob, json, pathlib, sys
from jsonschema import Draft202012Validator
from referencing import Registry, Resource
from referencing.jsonschema import DRAFT202012
BASE = "https://openpipestress.org/schemas/"
def registry(schema_dir):
    res = []
    for p in sorted(pathlib.Path(schema_dir).iterdir()):
        if p.suffix not in {".json", ".yaml"}: continue
        try: doc = json.loads(p.read_text())
        except Exception: continue
        if not isinstance(doc, dict): continue
        r = Resource.from_contents(doc, default_specification=DRAFT202012)
        res.append((BASE + p.name, r))
        if isinstance(doc.get("$id"), str) and doc["$id"] != BASE + p.name: res.append((doc["$id"], r))
    return Registry().with_resources(res)
def outcome(reg, schema_dir, path):
    inst = json.loads(pathlib.Path(path).read_text()); name = pathlib.Path(path).name
    if "schema_version" not in inst: return None
    if "result_envelope" in inst: uri, sub, ptr, n = "results.v0.3.schema.yaml", inst["result_envelope"], "#/$defs/ResultEnvelope/oneOf", len(json.loads((pathlib.Path(schema_dir)/"results.v0.3.schema.yaml").read_text())["$defs"]["ResultEnvelope"]["oneOf"])
    elif "analysis_run" in inst and inst.get("schema_version") == "0.3.0": uri, sub, ptr, n = "analysis_run.v0.3.schema.json", inst["analysis_run"], "#/$defs/AnalysisRun/oneOf", len(json.loads((pathlib.Path(schema_dir)/"analysis_run.v0.3.schema.json").read_text())["$defs"]["AnalysisRun"]["oneOf"])
    elif inst.get("package_id") == "PKG-17" and inst.get("schema_version") == "0.3.0": uri, sub, ptr, n = "stress_neutral_export.v0.3.schema.json", inst, "#/oneOf", len(json.loads((pathlib.Path(schema_dir)/"stress_neutral_export.v0.3.schema.json").read_text())["oneOf"])
    else: return None
    if inst.get("schema_version") != "0.3.0": return None
    full = Draft202012Validator({"$ref": BASE + uri}, registry=reg).is_valid(inst)
    br = [i for i in range(n) if Draft202012Validator({"$ref": BASE + uri + ptr + f"/{i}"}, registry=reg).is_valid(sub)]
    return (full, br)
work = sys.argv[1]; base_schemas = sys.argv[2]
rb, rc = registry(base_schemas), registry(work + "/schemas")
same = diff = 0
for path in sorted(glob.glob(work + "/fixtures/results/*.json")):
    if "load_reference_source_" in path: continue
    a = outcome(rb, base_schemas, path)
    if a is None: continue
    b = outcome(rc, work + "/schemas", path)
    tag = "SAME" if a == b else "DIFF"
    same += a == b; diff += a != b
    print(tag, pathlib.Path(path).name, "base", a, "cand", b)
print("same", same, "diff", diff)
