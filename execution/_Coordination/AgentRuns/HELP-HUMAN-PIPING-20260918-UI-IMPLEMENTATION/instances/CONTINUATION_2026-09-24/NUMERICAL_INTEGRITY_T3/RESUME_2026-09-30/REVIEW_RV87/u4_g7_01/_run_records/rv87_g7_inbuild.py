"""RV87 (u4_g7_01): the G7 record re-derived from G7's profile_tree forms x the record's in-build
atoms (phases, both modes), and compared with G6r's; and G7's tree forms against G6r's.
Usage: python3 rv87_g7_inbuild.py <g6r _run_records_g6r> <g7 pass_a>   (stdlib only)"""
import json, os, re, sys
R6, A7 = sys.argv[1], sys.argv[2]
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
    return {f"{m}/{n.split(' ')[0]}": sum(evn(c) for c in s["requested"]) + max(evn(c) for c in s["moving"]) + tree["R"]
            for m, ph in tree["phases"].items() for n, s in ph.items()}, {k: ev_form(k) for k in forms}
t6 = json.load(open(os.path.join(R6, "text_g6r", "profile_tree.json"))); a6, p6 = record(os.path.join(R6, "per_identity", "profile_record.test.txt"))
t7 = json.load(open(os.path.join(A7, "text_g7", "profile_tree.json"))); a7, p7 = record(os.path.join(A7, "profile_record.test.txt"))
e7, f7 = evaluate(t7, a7); e6, f6 = evaluate(t6, a6)
M = 4_026_531_840
out = {"g7_phases_equal_record": all(e7[k] == p7[(k.split('/')[0], k.split('/')[1])] for k in e7),
       "g7_minus_g6r": {k: e7[k] - e6[k] for k in e7},
       "forms_changed": {k: {"g6r": t6["forms"].get(k), "g7": t7["forms"].get(k)} for k in set(t6["forms"]) | set(t7["forms"]) if t6["forms"].get(k) != t7["forms"].get(k)},
       "atoms_changed": {k: [a6.get(k), a7.get(k)] for k in set(a6) | set(a7) if a6.get(k) != a7.get(k)},
       "T17_stages_g7": {k: f7[k] for k in f7 if k.startswith("T17_V")},
       "W3": {m: [e7[f"{m}/W3"], round(e7[f"{m}/W3"] / M, 4), int(0.9 * M) - e7[f"{m}/W3"]] for m in ("sparse", "dense")}}
print(json.dumps(out, indent=1))
