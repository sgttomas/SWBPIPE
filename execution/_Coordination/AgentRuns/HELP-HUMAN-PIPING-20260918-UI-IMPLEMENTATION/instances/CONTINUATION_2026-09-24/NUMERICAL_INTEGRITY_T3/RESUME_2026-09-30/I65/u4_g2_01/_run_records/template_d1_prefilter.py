"""I65 U4 G2: heuristic D1 reachability pre-filter for the T08 template inventory.

Adds the enclosing function name to each row of template_inventory.out.json and
marks a row EXCLUDED_BY_D1 when its diagnostic code or enclosing function names
a family that D1 removes (DOMAIN.md D1.3-D1.8). Everything else is IN_D1_CANDIDATE.
This is a lead for G3, not a reachability proof: G3 confirms each exclusion by
call path and attaches multiplicity to every candidate.
Usage: python3 template_d1_prefilter.py <repo-relative root> <inventory json>
"""
import json, os, re, sys

root, inv_path = sys.argv[1], sys.argv[2]
inv = json.load(open(inv_path))
EXCLUDE = re.compile(r"pressure|nonlinear|component|combination|hanger|constant_effort|thermal|wind|"
                     r"seismic|equivalent_static|load_state|reference_config|expansion_joint|expansion_law|"
                     r"bend|curved|user_stiffness|imposed|friction|gap|contact|modulus_basis|temperature_point|"
                     r"interpolat|membrane|historical", re.I)
fn_cache = {}
def enclosing(path, line):
    if path not in fn_cache:
        lines = open(os.path.join(root, path), encoding="utf-8").read().split("\n")
        fns = [(i + 1, m.group(1)) for i, l in enumerate(lines)
               for m in [re.match(r"\s*(?:pub(?:\([a-z]+\))?\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)", l)] if m]
        fn_cache[path] = fns
    best = "?"
    for ln, name in fn_cache[path]:
        if ln <= line:
            best = name
        else:
            break
    return best
out, counts = [], {"IN_D1_CANDIDATE": 0, "EXCLUDED_BY_D1": 0}
for r in inv["rows"]:
    fn = enclosing(r["file"], r["line"])
    text = " ".join([fn, r.get("code", ""), r.get("template", "")])
    status = "EXCLUDED_BY_D1" if EXCLUDE.search(text) else "IN_D1_CANDIDATE"
    counts[status] += 1
    out.append({"file": r["file"], "line": r["line"], "kind": r["kind"], "fn": fn,
                "code": r.get("code"), "status": status,
                "literal_bytes": r.get("literal_bytes"),
                "placeholders": [p["spec"] for p in r.get("placeholders", [])]})
print(json.dumps({"counts": counts, "rows": out}, indent=1))
