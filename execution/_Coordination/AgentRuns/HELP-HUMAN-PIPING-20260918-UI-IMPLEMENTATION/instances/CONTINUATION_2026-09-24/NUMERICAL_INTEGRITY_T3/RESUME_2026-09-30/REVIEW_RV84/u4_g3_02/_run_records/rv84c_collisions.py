"""RV84 confirmation (N-3): for each of the 17 candidate cycles RV84's G3 blunt fan-out produced
(R/REVIEW_RV84/u4_g3_01/_run_records/rv84_augment_scc.out.txt), show the G4 graph's actual edges
out of the cycle's members restricted to same-named targets, and the source text of each
same-named call in the member's body, so the adjudication can be read against source.
Usage: python3 rv84c_collisions.py <G4 _run_records> <P root at b1f80234dc>"""
import json, re, os, sys
G4, SRC = sys.argv[1], sys.argv[2]
cg = json.load(open(os.path.join(G4, "callgraph_edges.json")))
edges = cg["edges"]
cands = ["load_ledger.rs:273:is_empty", "load_ledger.rs:761:is_empty", "functionals.rs:721:is_empty",
         "wide.rs:519:leading_zeros", "load_ledger.rs:269:len", "load_ledger.rs:757:len", "functionals.rs:718:len",
         "bound.rs:79:len", "load_ledger.rs:131:push", "load_ledger.rs:265:get", "wide.rs:573:overflowing_add",
         "source_residual.rs:22:neg", "elastic_extrema.rs:132:cmp", "retained_memory.rs:81:next",
         "retained_receipt.rs:93:len", "certificate.rs:41:status", "structural_adapter.rs:133:geometry",
         "structural_adapter.rs:564:geometry", "adaptive.rs:820:capacity", "lib.rs:462:global_stiffness",
         "lib.rs:445:length", "lib.rs:458:local_stiffness", "wide_sum.rs:158:reserve", "directed.rs:34:step"]
def find(suffix):
    f, l, n = suffix.rsplit(":", 2)
    return [k for k in edges if k.endswith("/" + suffix) or (k.split("/")[-1] == suffix)]
out = {}
for c in cands:
    ks = find(c)
    if not ks:
        # line numbers may have moved: match by file and name
        f, l, n = c.rsplit(":", 2)
        ks = [k for k in edges if k.split("/")[-1].startswith(f + ":") and k.endswith(":" + n)]
    for k in ks:
        name = k.rsplit(":", 1)[1]
        self_edge = k in edges[k]
        same = [t.split("/")[-1] for t in edges[k] if t.rsplit(":", 1)[1] in (name, "add", "mul", "status")]
        rel, line, _ = k.rsplit(":", 2)
        src = open(os.path.join(SRC, rel), encoding="utf-8").read().split("\n")
        body = " ".join(x.strip() for x in src[int(line) - 1:int(line) + 6])[:220]
        out[k.split("/")[-1]] = {"self_edge": self_edge, "same_named_or_work_targets": same, "source": body}
json.dump(out, open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "rv84c_collisions.out.json"), "w"), indent=1)
for k, v in out.items():
    print(k, "| self:", v["self_edge"], "| targets:", v["same_named_or_work_targets"], "\n    ", v["source"][:200])
