"""RV87 (u4_g6_02): re-derive every phase of the repaired in-build record from profile_tree.json
forms x the record's in-build atom values, and decompose the change from G6.
Usage: python3 rv87_g6r_inbuild.py <G6 _run_records> <g6r _run_records_g6r>   (stdlib only)"""
import json, os, re, sys
G, R = sys.argv[1], sys.argv[2]
def record(path):
    atoms, phases = {}, {}
    for line in open(path):
        if line.startswith("I65_G5_ATOM"):
            head, name, val, ill = line.rstrip("\n").split("\t"); atoms[name] = int(val)
        m = re.match(r"I65_G5_PHASE mode=(\w+) phase=(\w+) requested=(\d+) moving=(\d+) E_mov_plus_R=(\d+)", line)
        if m: phases[(m.group(1), m.group(2))] = int(m.group(5))
    return atoms, phases
def evaluate(tree, atoms):
    forms, exprs = tree["forms"], tree["exprs"]
    def ev_form(n):
        return sum(v if k == "1" else v * (tree["text_atoms"][k] if k in tree["text_atoms"] else atoms[k]) for k, v in forms[n].items())
    def evn(node):
        if isinstance(node, str):
            return evn(exprs[node]) if node in exprs else ev_form(node)
        return sum(evn(c) for c in node["sum"]) if "sum" in node else max(evn(c) for c in node["max"])
    return {(m, n.split(" ")[0]): sum(evn(c) for c in s["requested"]) + max(evn(c) for c in s["moving"]) + tree["R"]
            for m, ph in tree["phases"].items() for n, s in ph.items()}
t6 = json.load(open(os.path.join(G, "text_g6", "profile_tree.json"))); a6, p6 = record(os.path.join(G, "per_identity", "profile_record.test.txt"))
tr = json.load(open(os.path.join(R, "text_g6r", "profile_tree.json"))); ar, pr = record(os.path.join(R, "per_identity", "profile_record.test.txt"))
er = evaluate(tr, ar)
M = 4_026_531_840
out = {"phases": {f"{m}/{p}": {"rv87": v, "record": pr.get((m, p)), "equal": v == pr.get((m, p)), "delta_from_g6": v - p6.get((m, p), 0)} for (m, p), v in er.items()},
       "forms_changed": sorted(k for k in set(t6["forms"]) | set(tr["forms"]) if t6["forms"].get(k) != tr["forms"].get(k)),
       "atoms_changed": {k: [a6.get(k), ar.get(k)] for k in sorted(set(a6) | set(ar)) if a6.get(k) != ar.get(k)},
       "W3": {m: {"E_mov_plus_R": er[(m, "W3")], "fraction": round(er[(m, "W3")] / M, 4), "under_0.9M": int(0.9 * M) - er[(m, "W3")]} for m in ("sparse", "dense")}}
print(json.dumps(out, indent=1))
