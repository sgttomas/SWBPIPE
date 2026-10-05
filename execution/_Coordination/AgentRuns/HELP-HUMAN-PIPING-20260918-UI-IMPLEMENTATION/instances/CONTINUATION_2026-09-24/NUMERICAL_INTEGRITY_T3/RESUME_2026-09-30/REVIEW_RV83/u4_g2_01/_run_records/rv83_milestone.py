"""RV83: recount the milestone fixture census and D1 clause facts (stdlib only). Run from P."""
import json, sys, hashlib
p = "fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json"
raw = open(p, "rb").read()
v = json.loads(raw)
facts = dict(values=0, objects=0, entries=0, arrays=0, elems=0, strings=0, sbytes=0, kbytes=0, maxdepth=0, max_s=0, max_k=0)
stack = [(None, v, 0)]
while stack:
    k, x, d = stack.pop()
    facts["values"] += 1; facts["maxdepth"] = max(facts["maxdepth"], d)
    if k is not None:
        facts["kbytes"] += len(k.encode()); facts["max_k"] = max(facts["max_k"], len(k.encode()))
    if isinstance(x, str):
        facts["strings"] += 1; facts["sbytes"] += len(x.encode()); facts["max_s"] = max(facts["max_s"], len(x.encode()))
    elif isinstance(x, list):
        facts["arrays"] += 1; facts["elems"] += len(x)
        stack += [(None, y, d + 1) for y in x]
    elif isinstance(x, dict):
        facts["objects"] += 1; facts["entries"] += len(x)
        stack += [(kk, y, d + 1) for kk, y in x.items()]
m = v["model"]
cases = m.get("load_cases", [])
out = {"sha256": hashlib.sha256(raw).hexdigest(), "census": facts,
       "top_keys": sorted(v.keys()), "model_keys": sorted(m.keys()),
       "schema_version": m.get("schema_version"), "document_kind": m.get("document_kind"),
       "n": len(m["nodes"]), "m": len(m["pipe_segments"]), "g": len(m["supports"]),
       "sections": len(m.get("sections", [])), "components": len(m.get("components", [])),
       "combinations": len(m.get("combinations", [])), "cases": len(cases),
       "model_materials": len(m.get("materials", [])), "request_materials": len(v.get("materials", [])),
       "temperature_points": [len(x.get("temperature_points", [])) for x in m.get("materials", []) + v.get("materials", [])],
       "supports": [{"family": s.get("family"), "restraints": s.get("restraints"), "stiffness": s.get("stiffness") is not None,
                     "hanger": "hanger" in s, "nonlinear": "nonlinear" in s} for s in m["supports"]],
       "case_keys": [sorted(c.keys()) for c in cases],
       "loads": [{"target": l["target"].get("type"), "dimension": l.get("dimension"), "category": l.get("category")} for c in cases for l in c.get("primitive_loads", [])],
       "pipe_keys": [sorted(p.keys()) for p in m["pipe_segments"]],
       "material_keys": [sorted(x.keys()) for x in m.get("materials", [])],
       "pressure_contract": "pressure_contract" in m, "reference_configurations": "reference_configurations" in m}
print(json.dumps(out, indent=1))
