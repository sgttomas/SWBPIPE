"""RV91: Python per-row binding refusal and classification summary on both
milestones (U6b at c89a7a986c), for comparison with TS (registered and not)."""
import json, sys
from pathlib import Path
ROOT = Path(sys.argv[1]); OUT = Path(sys.argv[2])
sys.path.insert(0, str(ROOT))
from core.analysis_runs import compatibility as c
out = {}
for mode in ["sparse_interactive", "dense_scrutiny"]:
    doc = json.loads((ROOT / f"fixtures/results/retained_precision_milestone_successor_{mode}.json").read_text())
    s, inv = doc["source"], doc["invocation"]
    out[mode] = {"binding": {r["id"]: c.rule_binding_refusal(s, r) for r in s["results"]},
                 "summary_with_invocation": c.classification_summary(s, inv),
                 "summary_without_invocation": c.classification_summary(s, None)}
OUT.write_text(json.dumps(out, indent=1))
print({m: (sum(1 for v in o["binding"].values() if v), o["summary_with_invocation"]) for m, o in out.items()})
