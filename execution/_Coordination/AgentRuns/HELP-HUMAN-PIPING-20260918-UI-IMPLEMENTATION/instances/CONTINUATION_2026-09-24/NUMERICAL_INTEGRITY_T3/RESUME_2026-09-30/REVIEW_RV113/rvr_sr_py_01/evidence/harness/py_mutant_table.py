"""RV113 (RV-R): the Python mutant table. Usage: py_mutant_table.py <runs dir> <MUTANTS.json> <head probes v5> <head probes fg> <out.json>
Per mutant: the failing tests (from junit), each failure's exception type (a kill counts only if an
AssertionError fails a test, never an error or a collection failure), and the probes whose verdict
(bound, unbound or transport) differs from the control run's."""
import json
import os
import sys
import xml.etree.ElementTree as ET

runs, mutants, hv5, hfg, out = sys.argv[1:6]
table = json.load(open(mutants))


def junit(path):
    res = {"tests": 0, "failed": [], "errors": [], "assertion_failures": []}
    if not os.path.exists(path):
        res["missing"] = True
        return res
    for tc in ET.parse(path).getroot().iter("testcase"):
        res["tests"] += 1
        name = f'{tc.get("classname").rsplit(".", 1)[-1]}::{tc.get("name")}'
        for child in tc:
            if child.tag == "failure":
                res["failed"].append(name)
                msg = (child.get("message") or "") + "\n" + (child.text or "")
                if "AssertionError" in msg or msg.lstrip().startswith("assert"):
                    res["assertion_failures"].append(name)
            elif child.tag == "error":
                res["errors"].append(name)
    return res


def lines(path):
    return {json.loads(l)["id"]: json.loads(l) for l in open(path)} if os.path.exists(path) else None


def short(v):
    if v is None:
        return None
    if "ok" in v:
        return ("admitted", v["ok"]["numerical_eligible"], v["ok"]["standing"])
    if "err" in v:
        return (v["err"]["gate"], v["err"]["code"])
    return ("escape", v.get("escape"))


ctrl = {"v5": lines(f"{runs}/NONE/probes_v5.jsonl"), "fg": lines(f"{runs}/NONE/probes_fg.jsonl")}
head = {"v5": lines(hv5), "fg": lines(hfg)}
control_equals_head = {k: (ctrl[k] is not None and head[k] is not None and
                           all(ctrl[k][i][m] == head[k][i][m] for i in head[k] for m in ("bound", "unbound", "transport")))
                       for k in ctrl}
rows = []
for mid in ["NONE"] + [m["id"] for m in table]:
    d = f"{runs}/{mid}"
    j = junit(f"{d}/pytest.xml")
    moved = []
    for k in ("v5", "fg"):
        got = lines(f"{d}/probes_{k}.jsonl")
        if got is None:
            continue
        for pid, line in got.items():
            for m in ("bound", "unbound", "transport"):
                if short(line[m]) != short(ctrl[k][pid][m]):
                    moved.append({"probe": pid, "verdict": m, "control": short(ctrl[k][pid][m]), "mutant": short(line[m])})
    what = next((m["what"] for m in table if m["id"] == mid), "control (RV113_MUT unset)")
    killed = bool(j["assertion_failures"])
    rows.append({"id": mid, "what": what, "tests": j["tests"], "failed": j["failed"], "errors": j["errors"],
                 "killed_by_assertion": killed, "assertion_failures": j["assertion_failures"],
                 "probes_moved": len(moved), "probe_moves": moved})
json.dump({"control_equals_head_probes": control_equals_head, "rows": rows}, open(out, "w"), indent=1)
print(json.dumps({"control_equals_head_probes": control_equals_head}))
for r in rows:
    print(f'{r["id"]:5} tests={r["tests"]:3} failed={len(r["failed"]):2} errors={len(r["errors"])} killed={r["killed_by_assertion"]!s:5} probes_moved={r["probes_moved"]:3} | {", ".join(sorted(set(x.split("::")[1].split("[")[0] for x in r["failed"])))[:200]}')
