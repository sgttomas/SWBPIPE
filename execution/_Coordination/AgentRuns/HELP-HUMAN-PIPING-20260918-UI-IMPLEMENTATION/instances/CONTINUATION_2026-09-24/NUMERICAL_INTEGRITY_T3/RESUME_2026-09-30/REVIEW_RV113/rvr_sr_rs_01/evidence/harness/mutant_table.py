"""RV113: tabulate the mutant runs. Usage: python mutant_table.py <S dir> <out json>

For each mutant: the candidate's failing tests (RE lib unit tests, the contract test, the
public source_blocks test), the reviewer probes whose head verdict changes against the
control (NONE), and the 07m census entries whose verdict changes against the control.
"""
import json
import re
import sys

S, out = sys.argv[1], sys.argv[2]
manifest = {m["id"]: m for m in json.load(open(f"{S}/mutants/manifest.json"))}
TEST = re.compile(r"^test (\S+) \.\.\. (ok|FAILED|ignored)")


def failed(path):
    try:
        lines = open(path, errors="replace").read().splitlines()
    except FileNotFoundError:
        return None
    return sorted({m.group(1) for l in lines if (m := TEST.match(l)) and m.group(2) == "FAILED"})


def lines(path):
    try:
        return [json.loads(l) for l in open(path) if l.strip()]
    except FileNotFoundError:
        return None


def verdict(v):
    if "ok" in v:
        return {"admitted": v["ok"]["numerical_eligible"]}
    if "err" in v:
        return {"gate": v["err"]["gate"], "code": v["err"]["code"]}
    return v


base_p = lines(f"{S}/mutants/runs/NONE/probes.jsonl")
base_c = lines(f"{S}/mutants/runs/NONE/census.jsonl")
rows = []
for mid in ["NONE"] + list(manifest):
    d = f"{S}/mutants/runs/{mid}"
    run = open(f"{d}/run.log").read() if __import__("os").path.exists(f"{d}/run.log") else ""
    tests = {k: failed(f"{d}/{k}.log") for k in ("lib", "contract", "source_blocks")}
    p, c = lines(f"{d}/probes.jsonl"), lines(f"{d}/census.jsonl")
    probe_kills = [] if p is None else [
        {"id": a["id"], "control": verdict(a["bound"]), "mutant": verdict(b["bound"])}
        for a, b in zip(base_p, p) if verdict(a["bound"]) != verdict(b["bound"])]
    census_changes = [] if c is None else [
        {"id": a["id"], "control": verdict(a["bound"]), "mutant": verdict(b["bound"])}
        for a, b in zip(base_c, c) if a["bound"] != b["bound"] or a["standing"] != b["standing"]]
    candidate_kills = sorted(f"{k}::{t}" for k, ts in tests.items() if ts for t in ts)
    rows.append({
        "id": mid, "family": manifest.get(mid, {}).get("family", "control"),
        "description": manifest.get(mid, {}).get("description", "RV113_MUT unset"),
        "run_log": run.strip().splitlines(),
        "candidate_tests_failing": candidate_kills,
        "killed_by_candidate_assertion": bool(candidate_kills),
        "probe_verdict_changes": probe_kills,
        "census_07m_changes": census_changes,
    })
json.dump(rows, open(out, "w"), indent=1)
for r in rows:
    print(f'{r["id"]:5} {r["family"]:7} candidate={"KILLED" if r["killed_by_candidate_assertion"] else "survives":8} '
          f'tests={len(r["candidate_tests_failing"]):2} probes={len(r["probe_verdict_changes"]):2} census={len(r["census_07m_changes"]):2}  '
          + ", ".join(t.split("::")[-1] for t in r["candidate_tests_failing"])[:150])
