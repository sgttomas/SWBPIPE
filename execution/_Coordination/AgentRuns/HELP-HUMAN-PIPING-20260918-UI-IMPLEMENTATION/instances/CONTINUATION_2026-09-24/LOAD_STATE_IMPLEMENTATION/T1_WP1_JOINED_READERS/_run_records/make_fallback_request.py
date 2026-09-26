"""Author the SF-1 fallback request: the committed eigen_motion witness plus one
declared 1e-6 N tip UZ force (extra:0), exactly as `with_extra_loads(witness(), 1)`
in core/product_physics/src/source_receipt/load_state_fallback_tests.rs.
All inputs are invented. Run from WORKING_ROOT."""
import json
from pathlib import Path

request = json.loads(Path("fixtures/product_preview/load_reference_source/eigen_motion.request.json").read_text())
case = request["model"]["load_cases"][0]
load = json.loads(json.dumps(case["primitive_loads"][0]))
load["id"] = "extra:0"
load["direction"] = "UZ"
load["category"] = "concentrated_force"
load["dimension"] = "force"
load["magnitude"] = {"value": 1.0e-6, "unit": "N"}
case["primitive_loads"].append(load)
case["analysis_state"]["load_sources"].append({"source_ref": "extra:0", "factor": 1.0})
out = Path("core/reporting/result_export/tests/fixtures/load_reference_fallback_uz.request.json")
out.write_text(json.dumps(request, indent=2, ensure_ascii=False) + "\n")
print(out)
