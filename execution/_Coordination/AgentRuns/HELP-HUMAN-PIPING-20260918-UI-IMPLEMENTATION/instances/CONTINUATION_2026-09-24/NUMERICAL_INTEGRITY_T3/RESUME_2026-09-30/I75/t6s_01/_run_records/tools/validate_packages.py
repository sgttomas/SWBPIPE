"""I75 scratch: validate dumped stress-neutral packages under the committed schemas (registry helper)."""
import json, sys, pathlib
P = pathlib.Path(sys.argv[1]); out = pathlib.Path(sys.argv[2])
sys.path.insert(0, str(P / "tests"))
from schema_validation import validate_instance
for name in ["stress_neutral_export.v0.3.schema.json", "stress_neutral_export.schema.json"]:
    schema = json.loads((P / "schemas" / name).read_text())
    for f in sorted(out.glob("sn_*.json")):
        inst = json.loads(f.read_text())
        try:
            validate_instance(schema, inst, schema_label=name, instance_label=f.name)
            print("VALID", name, f.name)
        except AssertionError as e:
            print("INVALID", name, f.name, str(e)[:2000])
    # Which oneOf branch matched (v0.3 file only)
    if name.endswith("v0.3.schema.json"):
        from jsonschema import Draft202012Validator
        from referencing import Registry, Resource
        from referencing.jsonschema import DRAFT202012
        from urllib.parse import urljoin
        res = {}
        for path in (P / "schemas").glob("*.schema.*"):
            try: c = json.loads(path.read_text())
            except Exception: continue
            r = Resource.from_contents(c, default_specification=DRAFT202012)
            urls = {path.resolve().as_uri(), urljoin(schema.get("$id", ""), path.name)}
            if "$id" in c: urls.add(c["$id"])
            res.update({u: r for u in urls})
        reg = Registry().with_resources(res.items())
        for f in sorted(out.glob("sn_*.json")):
            inst = json.loads(f.read_text())
            rest = {k: v for k, v in schema.items() if k not in ("oneOf", "$defs", "$id", "$schema")}
            def branch(br):
                return {"$schema": schema["$schema"], "$id": schema.get("$id", ""), "$defs": schema["$defs"], "allOf": [rest, br]}
            hits = [i for i, br in enumerate(schema["oneOf"]) if not list(Draft202012Validator(branch(br), registry=reg).iter_errors(inst))]
            print("BRANCHES", f.name, hits, "of", len(schema["oneOf"]))
