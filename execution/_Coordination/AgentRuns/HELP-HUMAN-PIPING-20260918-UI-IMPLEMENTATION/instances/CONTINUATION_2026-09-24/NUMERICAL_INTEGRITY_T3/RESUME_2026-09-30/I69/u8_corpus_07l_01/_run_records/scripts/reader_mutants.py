"""Records-only discrimination check: four in-memory mutants of the Python reader's `_g5a_coverage`
(no file is edited), each run over every 07l corpus entry. An entry kills a mutant when its outcome
differs from its expectation. Usage: reader_mutants.py P_ROOT MUTANT_ID|none"""
import importlib.util, inspect, json, sys, textwrap
from copy import deepcopy
from pathlib import Path
P = Path(sys.argv[1]); WHICH = sys.argv[2]; sys.path.insert(0, str(P))
from core.analysis_runs import retained_precision as rp
MUTANTS = {
    "RM1_a_exclusion_removed": ("if any(a[k] and not non_input[k] for k in range(4)): continue", "pass"),
    "RM2_l0_feasibility_coupled": ("positive = list(a) if length == 0 else [", "positive = ["),
    "RM3_l0_hats_coupled": ("if length != 0: hats = [hats[0] or hats[1]] * 2", "hats = [hats[0] or hats[1]] * 2"),
    "RM4_no_free_dof_rule_removed": ('if not free: need(not has_data[b["body"]])', "pass"),
}
if WHICH != "none":
    old, new = MUTANTS[WHICH]
    src = textwrap.dedent(inspect.getsource(rp._g5a_coverage))
    assert src.count(old) == 1, WHICH
    exec(compile(src.replace(old, new), rp.__file__ + f"<{WHICH}>", "exec"), rp.__dict__)
spec = importlib.util.spec_from_file_location("harness", P / "tests/test_retained_precision_contract.py")
h = importlib.util.module_from_spec(spec); spec.loader.exec_module(h)
c = h.corpus()
cases = {f["id"]: f for f in c["cases"]}
killed = []
def outcome(source, invocation):
    try:
        return ("pass", rp._validate_draft(source, invocation)["classifications"])
    except rp.RetainedPrecisionError as e:
        return ("refuse", {"gate": e.gate, "code": e.code})
for f in c["cases"]:
    o = outcome(deepcopy(f["source"]), deepcopy(f["invocation"]))
    if o != ("pass", f["expected_classifications"]): killed.append(("case", f["id"], o[0]))
for kind in ("mutations", "must_pass"):
    for e in c[kind]:
        s, i = h.apply_entry(cases[e["base"]], e)
        o = outcome(s, i)
        want = ("pass", cases[e["base"]]["expected_classifications"]) if e["expected"] == "pass" else ("refuse", e["expected"])
        if o != want: killed.append((kind, e["id"], o[0] if o[0] == "pass" else o[1]["gate"]))
print(json.dumps({"mutant": WHICH, "killed_by": killed}))
