"""RV97 round 2: compare the three readers on identical documents, with each other and with the
corpus's expectations. Usage: compare_parity.py R2_DIR"""
import json, sys
from collections import Counter
from pathlib import Path
d = Path(sys.argv[1])
E = json.loads((d / "expectations.json").read_text())
exp = {e["label"]: e for e in E["expectations"]}
norm = lambda rows: [[r["result_id"], r["class"], r["normalized_bits"], r["scale_bits"], r["bound_bits"]] for r in rows]
ref = {k: norm(v) for k, v in E["expected_classifications"].items()}
R = {name: {json.loads(l)["label"]: json.loads(l) for l in (d / f"logs/{name}_parity.jsonl").read_text().splitlines()} for name in ("py", "rust", "ts")}
reader_key = {"py": "python", "rust": "rust", "ts": "typescript"}
problems, tally = [], Counter()
gate_tally = {k: Counter() for k in R}
for label, e in exp.items():
    outs = {k: R[k][label] for k in R}
    if e["kind"] == "mutation":
        for k, o in outs.items():
            want = (e.get("expected_by_reader") or {}).get(reader_key[k], e["expected"])
            got = None if o["ok"] else {"gate": o["gate"], "code": o["code"]}
            gate_tally[k][f"{o.get('gate')} {o.get('code')}"] += 1
            if got != want:
                problems.append([label, k, "first failure", got, want])
        firsts = {k: (o.get("gate"), o.get("code")) for k, o in outs.items()}
        tally["mutation identical first failure in all three" if len(set(firsts.values())) == 1 else "mutation per-reader first failure differs"] += 1
        if len(set(firsts.values())) != 1:
            tally["  of which declared by expected_by_reader" if e.get("expected_by_reader") else "  of which UNDECLARED"] += 1
    else:
        want = e["expected"]
        for k, o in outs.items():
            if not o["ok"]:
                problems.append([label, k, "refused", o.get("gate"), o.get("code")]); continue
            if o["bound"] != want["invocation_bound"] or o["eligible"] != want["numerical_eligible"] or ("standing" in o and o["standing"] != want["standing"]):
                problems.append([label, k, "eligibility", [o["bound"], o["eligible"], o.get("standing")], want])
            if o["classifications"] != ref[e["classifications_of"]]:
                problems.append([label, k, "classifications differ from the base's expected_classifications"])
        pubs = {o.get("publication_sha256") for o in outs.values() if o["ok"]}
        if len(pubs) != 1:
            problems.append([label, "all", "publication_sha256 differs across readers", sorted(map(str, pubs))])
        tally[f"{e['kind']} admitted as expected by all three"] += 0 if any(p[0] == label for p in problems) else 1
print(json.dumps({"documents": len(exp), "tally": tally, "problems": problems[:50], "problem_count": len(problems)}, indent=1, default=str))
for k in R:
    print(k, "mutation first failures:", dict(sorted(gate_tally[k].items())))
# The 07l additions in detail.
for label, e in exp.items():
    if ("u8_l0" in label) or (e.get("base", "").startswith("u8_l0")):
        outs = {k: R[k][label] for k in R}
        brief = {k: (f"{o['gate']} {o['code']}" if not o["ok"] else f"ok bound={o['bound']} eligible={o['eligible']} rows={len(o['classifications'])}") for k, o in outs.items()}
        print("07L", label, json.dumps(brief))
