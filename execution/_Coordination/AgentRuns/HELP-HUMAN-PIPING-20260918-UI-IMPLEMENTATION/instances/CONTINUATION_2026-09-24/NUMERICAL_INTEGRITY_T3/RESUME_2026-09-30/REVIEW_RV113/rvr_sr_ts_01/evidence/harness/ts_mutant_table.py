"""RV113: tabulate the TS mutant runs. Usage: python ts_mutant_table.py <S/ts dir> <out json>
Per mutant: whether every file loaded (no suite-level failure), the candidate's failing tests (excluding the reviewer's
probe harness), and the reviewer probes whose verdict changes against the control (NONE)."""
import json, os, sys
T, out = sys.argv[1], sys.argv[2]
manifest = {m["id"]: m for m in json.load(open(f"{T}/mutants/manifest.json"))}
def load(p): return [json.loads(l) for l in open(p) if l.strip()] if os.path.exists(p) else None
def short(v):
    if "ok" in v: return {"admitted": v["ok"]["numerical_eligible"]}
    if "err" in v: return {"gate": v["err"]["gate"], "code": v["err"]["code"]}
    return v
base = load(f"{T}/mutants/runs/NONE/probes.jsonl")
rows = []
for mid in ["NONE"] + list(manifest):
    d = f"{T}/mutants/runs/{mid}"
    rep = json.load(open(f"{d}/vitest.json")) if os.path.exists(f"{d}/vitest.json") else None
    failing, files_failed, total = [], [], 0
    if rep:
        for f in rep["testResults"]:
            name = f["name"].split("/apps/desktop/")[-1]
            if f.get("status") == "failed" and not f["assertionResults"]: files_failed.append(name)
            for a in f["assertionResults"]:
                total += 1
                if a["status"] != "passed" and "rv113Census" not in name: failing.append(f"{name.split('/')[-1]} :: {a['title']}")
    p = load(f"{d}/probes.jsonl")
    kills = [] if (p is None or base is None) else [{"id": x["id"], "control": short(x["bound"]), "mutant": short(y["bound"])}
             for x, y in zip(base, p) if short(x["bound"]) != short(y["bound"]) or x["transport"] != y["transport"]]
    rows.append({"id": mid, "family": manifest.get(mid, {}).get("family", "control"), "description": manifest.get(mid, {}).get("description", "RV113_MUT unset"),
                 "tests_collected": total, "files_failed_to_load": files_failed, "candidate_tests_failing": failing,
                 "killed_by_candidate_assertion": bool(failing) and not files_failed, "probe_verdict_changes": kills})
json.dump(rows, open(out, "w"), indent=1)
for r in rows:
    print(f'{r["id"]:5} {r["family"]:5} collected={r["tests_collected"]:4} loadfail={len(r["files_failed_to_load"])} candidate={"KILLED" if r["killed_by_candidate_assertion"] else "survives":8} '
          f'probes={len(r["probe_verdict_changes"]):2} ' + "; ".join(t.split(" :: ")[-1][:60] for t in r["candidate_tests_failing"][:3]))
