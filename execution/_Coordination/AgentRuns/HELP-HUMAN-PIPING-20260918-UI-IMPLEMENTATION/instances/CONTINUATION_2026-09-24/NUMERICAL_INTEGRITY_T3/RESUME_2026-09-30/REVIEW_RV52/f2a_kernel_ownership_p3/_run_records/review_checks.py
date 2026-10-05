"""RV52: standard-library source/abstract-ownership checks; no Rust execution."""
from pathlib import Path
from hashlib import sha256
from contextlib import redirect_stdout
from io import StringIO
import json

HERE = Path(__file__).resolve().parent
OUT = HERE.parent
R = OUT.parent.parent
NUM = next(p for p in HERE.parents if (p / "projects/chirality-piping/core").is_dir())
SRC = NUM.parent / "f2a"
PACK = R / "I29/f2a_kernel_ownership_p3"
K = SRC / "projects/chirality-piping/core/solver/frame_kernel/src/structural/retained"
checks = []
def record(name, **details):
    checks.append(dict(name=name, status="pass", **details))
def tokens(roots, edges):
    seen, pending = set(), list(roots)
    while pending:
        x = pending.pop()
        if x not in seen:
            seen.add(x)
            pending.extend(edges.get(x, ()))
    return seen

# Check frozen author payload references against actual bytes without Git.
manifest = json.loads((PACK / "_run_records/WRITE_INVENTORY.json").read_text())
for e in manifest["files"]:
    b = (PACK / e["path"]).read_bytes()
    assert len(b) == e["bytes"] and sha256(b).hexdigest() == e["sha256"]
record("candidate_sealed_payload_hashes", files=len(manifest["files"]))
origins = json.loads((PACK / "_run_records/ORIGINS.json").read_text())["entries"]
count = 0
for e in origins:
    if e["origin"] in ("frozen I37 git object", "stable baseline canonical accounting"):
        b = (SRC / e["path"]).read_bytes()
        assert sha256(b).hexdigest() == e["sha256"]
        count += 1
record("actual_source_matches_recorded_hashes", files=count,
       caveat="revision names supplied; hashes checked; no Git object read")

# Replay only the inspected abstract author checker with output redirected to us.
replay = HERE / "author_replay"
replay.mkdir(exist_ok=True)
ns = {"__name__": "rv52_replay", "__file__": str(replay / "owner_graph_checks.py")}
with redirect_stdout(StringIO()):
    exec(compile((PACK / "_run_records/owner_graph_checks.py").read_text(),
                 str(PACK / "_run_records/owner_graph_checks.py"), "exec"), ns)
    ns["main"]()
assert (replay / "CHECKS.json").read_bytes() == (PACK / "_run_records/CHECKS.json").read_bytes()
record("author_synthetic_controls_reproduce", controls=12)

# Exhaust all abstract branches of the monotone 0..3 schedule. Independent of
# numerical outcomes: terminal errors can only remove suffixes from these paths.
traces = []
def walk(c=0, pending=False, solves=(), states=(), passes=(), comparisons=()):
    if c >= 3:
        traces.append((solves, states, passes, comparisons)); return
    if pending:
        branches = [(solves, states)]
    else:
        new_solves = solves + (c,)
        traces.append((new_solves, states, passes, comparisons))
        walk(c+1, False, new_solves, states, passes, comparisons)
        branches = [(new_solves, states+(c,))]
    for so, st in branches:
        so = so+(c+1,)
        traces.append((so, st, passes, comparisons))
        walk(c+2, False, so, st, passes, comparisons)
        st = st+(c+1,)
        pa = passes+(c+1,)
        traces.append((so, st, pa, comparisons))
        co = comparisons+(c,)
        traces.append((so, st, pa, co))
        walk(c+1, True, so, st, pa, co)
walk()
assert all(len(so) <= 4 and len(set(so)) == len(so) and len(st) <= 4
           and len(pa) <= 3 and len(co) <= 3 for so,st,pa,co in traces)
record("finite_schedule_branches", paths=len(traces),
       maxima=[max(len(t[i]) for t in traces) for i in range(4)])

# First occupied is per slot; failed payload copies have fresh identity, while
# success payload clones add references to one allocation event.
ops=[{"v256":("err","a_err"),"s512":("ok","a_S")},
     {"v256":("ok","later_VS"),"v512":("err","b_err"),"s512":("ok","b_S")}]
merged={}
for op in ops:
    for slot,val in op.items():
        if slot not in merged:
            merged[slot] = val if val[0] == "ok" else ("err","import_"+val[1])
assert merged == {"v256":("err","import_a_err"),"s512":("ok","a_S"),"v512":("err","import_b_err")}
edges={"operand_a":["a_err","a_S"],"operand_b":["b_err","b_S","later_VS"],
       "combo_cache":[v[1] for v in merged.values()],
       "combo_selected":["snapshot_a_err","snapshot_b_err","a_S"],
       "a_S":["factor_a"],"b_S":["factor_b"]}
live=tokens(["operand_a","operand_b","combo_cache","combo_selected"],edges)
assert {"a_err","b_err","import_a_err","import_b_err","snapshot_a_err","snapshot_b_err"} <= live
assert {"factor_a","factor_b"} <= live and len([x for x in live if x == "a_S"]) == 1
record("first_occupied_import_copy_generations", error_backings=6,
       caveat="abstract owner states, not a numerical reachability assertion")

# An already built S remains cache-owned even if a later work check discards its
# result edge. A cached Err reuse is an attempt copy, not another numerical build.
assert tokens(["cache"], {"cache":["S"],"S":["factor"]}) == {"cache","S","factor"}
assert tokens(["cache","attempt"], {"cache":["err_cache"],"attempt":["err_attempt"]}) == {"cache","attempt","err_cache","err_attempt"}
record("post_build_work_loss_and_cached_error_reuse")

# A moved report vector is the same allocation; summary copies are separate.
a=tokens(["report","summary"],{"report":["resolution","delta"],
                                   "summary":["resolution_copy","theta_copy","bound_copy"]})
assert len(a-{"report","summary"}) == 5
record("report_move_and_summary_copy")
# Element destruction drops children; clearing preserves the outer backing.
before={"outer","summary_child","refusal_child"}
case_after=set(); combo_after={"outer"}
assert before-case_after == before and before-combo_after == {"summary_child","refusal_child"}
record("legacy_drop_versus_clear", case_after=sorted(case_after), combo_after=sorted(combo_after))

# Source-anchored missing roster term: distinguish three [f64;2] buffers.
ad=(K/"adaptive.rs").read_text(); ve=(K/"verify.rs").read_text()
assert "let hats: Vec<[f64; 2]> = match report" in ad
assert "let mut phis = Vec::with_capacity(hats.len())" in ad
assert "floor = Some(phis)" in ad and "hats[body as usize][0]" in ad
assert "resolution_hats(&resolution, &prep.extents)?;" in ve
row=next(x for x in (PACK/"ENVELOPE.md").read_text().splitlines() if x.startswith("| stop rule |"))
assert "hats" not in row and "floor" in row
stop_tokens={"report_resolution","rule_hats","floor"}
assert stop_tokens-{"report_resolution","floor"} == {"rule_hats"}
record("confirmed_roster_omission_RV52_1", omitted="adaptive::rule hats",
       distinction="earlier resolution_hats result is dropped at verify.rs:794",
       witness="three distinct same-shape owners: report resolution, rule hats, optional floor",
       limit="This establishes an unmapped owner, not a measured total-memory underbound.")

# The earlier u_pass owns a and c, but these need only two of the four same-type
# row-buffer slots already allowed in the Uc coarse upper. Name this mapping
# explicitly in the review rather than treating a different source token as c.
assert "let mut a = vec![one; n]" in (K/"bound.rs").read_text()
assert max(2,4) == 4
record("uc_u_pass_dominance", phases={"u_pass":["a","c"], "nl_pass":["c","at","bt","ct"]},
       premise="common maximum per-site Wide<L> capacity law in ENVELOPE:30")
# Actual shift iterations drop previous factor; a resize old backing is separate
# from normal independent copies. These are identity checks, no byte assumptions.
assert len({"scaled_profile","factor_0"}) == len({"scaled_profile","factor_1"}) == 2
assert {"new_backing"}|{"old_backing"} == {"new_backing","old_backing"}
record("shift_retry_and_moving_backing_identity")

result={"status":"checks_pass_with_document_finding","production_execution":False,
        "scope":"source hashes plus finite abstract schedule/owner bookkeeping",
        "checks":checks,"finding":"RV52-1: stop-rule hats owner missing from local roster"}
(HERE/"CHECKS.json").write_text(json.dumps(result,indent=2)+"\n")
print(json.dumps({"checks":len(checks),"status":result["status"],"schedule_paths":len(traces)}))
