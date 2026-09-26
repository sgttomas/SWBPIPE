"""Reviewer's independent schema preservation walk: base (c1e130818) vs candidate."""
import json, sys, yaml, pathlib
base_dir, cand_dir = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
def load(p):
    t = p.read_text()
    return json.loads(t)
report = {}
for name in ["results.v0.3.schema.yaml", "analysis_run.v0.3.schema.json", "stress_neutral_export.v0.3.schema.json"]:
    b, c = load(base_dir / name), load(cand_dir / name)
    appended, added_keys, changed = [], [], []
    def walk(x, y, ptr):
        if isinstance(x, dict) and isinstance(y, dict):
            for k in x:
                if k not in y:
                    changed.append((ptr + "/" + k, "REMOVED"))
                else:
                    walk(x[k], y[k], ptr + "/" + k)
            for k in y:
                if k not in x:
                    added_keys.append(ptr + "/" + k)
        elif isinstance(x, list) and isinstance(y, list):
            if len(y) < len(x):
                changed.append((ptr, "SHORTENED")); return
            for i, v in enumerate(x):
                walk(v, y[i], f"{ptr}/{i}")
            if len(y) > len(x):
                appended.append((ptr, len(x), len(y)))
        elif x != y or type(x) != type(y):
            changed.append((ptr, json.dumps(x)[:120], json.dumps(y)[:160]))
    walk(b, c, "")
    report[name] = {"appended_lists": appended, "added_keys": added_keys, "changed": changed}
print(json.dumps(report, indent=1))
