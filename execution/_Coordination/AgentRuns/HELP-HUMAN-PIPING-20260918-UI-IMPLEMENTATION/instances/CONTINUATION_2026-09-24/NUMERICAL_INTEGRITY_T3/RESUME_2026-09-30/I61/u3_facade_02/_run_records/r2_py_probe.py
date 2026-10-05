"""I61 U3 grant 1b, R-2 condition (evidence only): the unchanged Python base
preview-physics-1 reader and standing over the same bytes as the Rust/TS/runner checks."""
import json, sys
from pathlib import Path
P, OUT = Path(sys.argv[1]), Path(sys.argv[2])
sys.path.insert(0, str(P))
from core.analysis_runs import compatibility as c
from core.analysis_runs.preview_physics_evidence import validate_preview_physics_evidence
lines = []
for label in ["milestone", "exportable"]:
    req = json.loads((OUT / f"{label}_request.json").read_text())
    bases = [{"ref_type": "load_case", "ref_id": x["id"]} for x in req["model"]["load_cases"]]
    for mode in ["sparse_interactive", "dense_scrutiny"]:
        base = json.loads((OUT / f"{label}_base_{mode}.json").read_text())
        validate_preview_physics_evidence(base)
        base_standing = c.numerical_use_standing(base, bases)
        for kind in ["plain", "receipt"]:
            noticed = json.loads((OUT / f"{label}_noticed_{kind}_{mode}.json").read_text())
            assert noticed["diagnostics"][-1]["code"] == "RETAINED_PRECISION_UNAVAILABLE"
            validate_preview_physics_evidence(noticed)
            standing = c.numerical_use_standing(noticed, bases)
            assert standing == base_standing, (standing, base_standing)
            lines.append(f"I61_R2_PY {label} {mode} {kind} reader=ok standing={standing} (base {base_standing})")
        dangling = json.loads((OUT / f"{label}_noticed_plain_{mode}.json").read_text())
        dangling["diagnostics"][-1]["affected_refs"] = ["result:not-a-row"]
        try:
            validate_preview_physics_evidence(dangling)
            lines.append(f"I61_R2_PY {label} {mode} negative_control=ACCEPTED")
        except ValueError as e:
            lines.append(f"I61_R2_PY {label} {mode} negative_control=refused ({str(e)[:60]})")
print("\n".join(lines))
