"""RV85 (evidence only): the unchanged Python base preview-physics-1 reader over the
notice-bearing envelopes the 1b product actually published behind RV85's stub (real
W1-ran fallbacks), against the plain envelopes; plus three negative controls."""
import json, sys, copy
from pathlib import Path
P, D = Path(sys.argv[1]), Path(sys.argv[2])
sys.path.insert(0, str(P))
from core.analysis_runs import compatibility as c
from core.analysis_runs.preview_physics_evidence import validate_preview_physics_evidence
def verdict(env):
    try:
        validate_preview_physics_evidence(env); return "ok"
    except Exception as e:
        return "refused(" + str(e)[:70] + ")"
lines = []
for noticed_path in sorted(D.glob("noticed_*.json")):
    stem = noticed_path.name[len("noticed_"):-len(".json")]
    name, mode = stem.rsplit("_", 2)[0], "_".join(stem.rsplit("_", 2)[1:])
    noticed = json.loads(noticed_path.read_text()); plain = json.loads((D / f"plain_{stem}.json").read_text())
    req = json.loads((D / f"request_{name}.json").read_text())
    bases = [{"ref_type": "load_case", "ref_id": x["id"]} for x in req["model"]["load_cases"]]
    last = noticed["diagnostics"][-1]
    assert last["code"] == "RETAINED_PRECISION_UNAVAILABLE" and len(noticed["diagnostics"]) == len(plain["diagnostics"]) + 1
    s_plain, s_noticed = c.numerical_use_standing(plain, bases), c.numerical_use_standing(noticed, bases)
    neg = {}
    d = copy.deepcopy(noticed); d["diagnostics"][-1]["affected_refs"] = ["result:not-a-row"]; neg["dangling_result_ref"] = verdict(d)
    d = copy.deepcopy(noticed); d["diagnostics"][-1]["affected_refs"] = ["no-such-case"]; neg["unknown_case_ref"] = verdict(d)
    d = copy.deepcopy(noticed); d["diagnostics"].append(copy.deepcopy(last)); neg["duplicate_notice_id"] = verdict(d)
    lines.append(f"RV85_PY {name} {mode} plain={verdict(plain)} noticed={verdict(noticed)} standing={s_noticed} (plain {s_plain}) equal={s_noticed == s_plain} negatives={neg}")
print("\n".join(lines))
