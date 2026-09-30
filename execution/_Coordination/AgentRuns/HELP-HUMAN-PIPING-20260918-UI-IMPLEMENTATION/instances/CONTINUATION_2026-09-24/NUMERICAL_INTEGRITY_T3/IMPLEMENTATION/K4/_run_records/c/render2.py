import json, sys
sys.dont_write_bytecode = True
sys.path.insert(0, ".")
import mutants as M
from mutants import MUTANTS
label = {}
for k, v in vars(M).items():
    if isinstance(v, str) and "::" in v and k.isupper():
        label[v] = k
label.setdefault("itest::s11_site_table", "S11")
res = {json.loads(l)["name"]: json.loads(l) for l in open("results.jsonl")}
ev = {json.loads(l)["name"]: json.loads(l)["moved"] for l in open("evidence.jsonl")}
rows = ["| Mutant | Edit | Result | Killed by | Controls that move (* false claim) |", "|---|---|---|---|---|"]
for name, spec in MUTANTS.items():
    r = res[name]
    kills = r.get("killed_by", [])
    k = ", ".join(label.get(t, t.split("::")[-1]) for t in kills) or "—"
    if name.startswith("NONE"):
        rs = "control: passes" if not kills else "CONTROL FAILS"
    elif spec.get("expect") == "guard":
        ctrl = [x for x in r["ran"] if x["test"].endswith("every_control_follows_gens_schedule_and_r7s_expectations_honestly")]
        rs = "guard: no control moves" if ctrl and ctrl[0]["ok"] else "guard: CONTROL MOVES"
    else:
        rs = r["result"].lower()
    e = ev.get(name)
    if e is None:
        es = ""
    elif not e and name in ("R7-M4", "R7-M8", "K4-M40"):
        es = "(operand %s unresolved; the list stops)" % ("PT-A" if name == "R7-M4" else "PTF-A")
    elif not e:
        es = "none"
    else:
        es = ", ".join(x.split(":")[0] + "→" + x.split(" ")[1] + ("*" if "DISHONEST" in x else "") for x in e)
    rows.append("| %s | %s | %s | %s | %s |" % (name, spec["what"].replace("|", "/"), rs, k, es))
legend = sorted(set(label[t] + " = " + t.split("::")[-1] for r in res.values() for t in r.get("killed_by", []) if t in label))
open("table2.md", "w").write("\n".join(rows) + "\n\nKilling tests: " + "; ".join(legend) + "\n")
