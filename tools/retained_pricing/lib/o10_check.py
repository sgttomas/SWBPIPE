"""O-10: after a run, each O10_LOOPS_LAST rule (a header rule appended last) may have matched only the loops it was
written for. Reads the chain's SQ2_RULES.json (o10_loops_last) and a point's looplog.json; prints each rule's matched
functions and exits 3 if any match lies outside the rule's listed functions. Usage: o10_check.py <chain dir> <run dir>..."""
import json, os, re, sys
chain, runs = sys.argv[1], sys.argv[2:]
rules = json.load(open(os.path.join(chain, "SQ2_RULES.json"))).get("o10_loops_last", [])
bad, out = [], []
for run in runs:
    ll = json.load(open(os.path.join(run, "looplog.json")))
    for r in rules:
        fns = set()
        for ctx, h, b, v in ll:
            if h == r["header"]:
                m = re.match(r"(?:call|site) (\S+?)(?: ->| @)", str(ctx))
                fns.add(m.group(1) if m else str(ctx))
        # O-10 review: allowed functions are `file:fn` (the line dropped), never a bare name
        fk = lambda f: ":".join(f.split(":")[::2]) if f.count(":") == 2 else f
        stray = [f for f in fns if fk(f) not in r["fns"]]
        out.append({"run": os.path.basename(run), "header": r["header"], "bound": r["bound"], "matched_fns": sorted(fns), "stray": stray})
        if stray: bad.append(out[-1])
rules = json.load(open(os.path.join(chain, "SQ2_RULES.json")))
mult0 = rules.get("o10_mult0_sites", [])
mult0_rows = rules.get("o10_mult0_rows", {})
for run in runs:
    rows = json.load(open(os.path.join(run, "work", "text_budget.caps.out.json")))["rows"]
    for k in mult0:
        hit = [r for r in rows if r["file"].endswith("/" + k.split(":")[0]) and r["line"] == int(k.split(":")[1])]
        pos = [r for r in hit if r.get("mult")]
        out.append({"run": os.path.basename(run), "mult0_site": k, "rows": len(hit), "positive": len(pos)})
        if pos or not hit: bad.append(out[-1])
        # MODAL-CS-M0 (§8.63): where the rules bind a site's exact TEXT row count, any other count fails.
        if k in mult0_rows and len(hit) != mult0_rows[k]:
            out.append({"run": os.path.basename(run), "mult0_site": k, "rows": len(hit), "expected_rows": mult0_rows[k]})
            bad.append(out[-1])


def reached(edges, roots):
    """The functions reached from the roots on the point's graph: text_budget.py's own traversal of edges.json."""
    seen, todo = set(roots), list(roots)
    while todo:
        for w in edges.get(todo.pop(), []):
            if w not in seen:
                seen.add(w); todo.append(w)
    return seen


# MODAL-CS-UR (§8.63): each named function must be found once in every point's graph (edges.json, by its file and name)
# and not be reached from the point's roots (text_budget's roots). A point whose graph or roots are missing, a crate
# with no function in the graph, a missing or ambiguous function, and a reached one each fail the run. A reviewed
# absence (state "reviewed-absent", emitted only where the rules' reviewed condition holds) requires the function to be
# absent from every graph. The graph resolves a method call by name, narrowed by the receiver's type where it is known:
# a real call `x.lift_into(..)` from any scanned function is an edge from that function's node to the named function's
# node (as skew_pencil.rs's modal_lift_within -> restraint_basis.rs's lift_into is today), and a call-shaped token naming
# it that produced no edge is a row of the graph's audit.json. So a false same-name edge reaching it stops the run (to
# be resolved by a reviewed edge rule, never passed), and an unresolved token naming it stops too: the check cannot be
# satisfied by the graph failing to resolve a real call. This only reads edges.json, audit.json's rows and the roots of
# text_budget.caps.out.json; it changes nothing.
unreached = rules.get("o10_unreached_fns", [])
if unreached and not runs:
    bad.append({"unreached_fn": "*", "error": "no point to check"})
for run in runs:
    for spec in unreached:
        res = {"run": os.path.basename(run), "unreached_fn": spec["file"] + ":" + spec["name"], "state": spec["state"]}
        out.append(res)
        try:
            edges = json.load(open(os.path.join(run, "edges.json")))["edges"]
            roots = json.load(open(os.path.join(run, "work", "text_budget.caps.out.json")))["roots"]
            unresolved = [r["line"] for r in json.load(open(os.path.join(run, "audit.json")))["rows"] if r["name"] == spec["name"]]
        except (OSError, KeyError, ValueError) as e:
            res["error"] = f"the point's graph or roots are missing ({type(e).__name__})"; bad.append(res); continue
        nodes = set(edges) | {w for ws in edges.values() for w in ws}
        crate = spec["file"].split("/src/")[0] + "/src/"
        hits = sorted(n for n in nodes if n.rsplit(":", 2)[0] == spec["file"] and n.rsplit(":", 1)[-1] == spec["name"])
        res["found"] = hits
        if not roots or not any(n.startswith(crate) for n in nodes):
            res["error"] = "no roots, or the function's crate is not in the graph"; bad.append(res)
        elif unresolved:
            res["error"] = f"a call naming it produced no edge (audit.json {unresolved[:3]})"; bad.append(res)
        elif spec["state"] == "reviewed-absent":
            if hits:
                res["error"] = "reviewed absent, but the graph has it"; bad.append(res)
        elif len(hits) != 1:
            res["error"] = "missing from the graph" if not hits else "ambiguous in the graph"; bad.append(res)
        elif hits[0] in reached(edges, roots):
            res["error"] = "reached from a retained root"; bad.append(res)
print(json.dumps(out, indent=1))
if bad:
    print("O10 CHECK FAILED: " + json.dumps(bad)); sys.exit(3)
