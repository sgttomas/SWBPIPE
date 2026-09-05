from pathlib import Path
import sys,json,copy,importlib.util,subprocess,hashlib
root=Path(subprocess.check_output(["git","rev-parse","--show-toplevel"],text=True).strip())/"projects/chirality-piping"
sys.dont_write_bytecode=True;sys.path[:0]=[str(root),str(root/"tests")]
from core.model_transform.physical_to_analytical.contract import transform_physical_to_analytical
from core.model_transform.physical_to_analytical._solver_boundary_adapter import adapt_analytical_solver_model
p=Path(__file__).resolve().parent
source=json.loads((root/"fixtures/domain/invented_physical_source_of_truth_model.json").read_text())["model"]
before=copy.deepcopy(source)
t=transform_physical_to_analytical(source).to_dict(); a=adapt_analytical_solver_model(t["analytical_model"]).to_dict()
assert source==before
assert t==transform_physical_to_analytical(source).to_dict()
(p/"results/canonical_transform.json").write_text(json.dumps(t,indent=2)+"\n")
(p/"results/canonical_adapter.json").write_text(json.dumps(a,indent=2)+"\n")
(p/"fixtures/canonical_analytical.json").write_text(json.dumps(t["analytical_model"],indent=2)+"\n")
# Execute every existing focused transform and adapter test directly; they have no pytest fixtures.
checks=[]
for name in ["test_physical_to_analytical_transform","test_analytical_solver_boundary_adapter"]:
 mod=__import__(name)
 for key,fn in vars(mod).items():
  if key.startswith("test_") and callable(fn):
   try: fn();checks.append({"test":name+"."+key,"outcome":"PASS"})
   except Exception as e:checks.append({"test":name+"."+key,"outcome":"FAIL","error":repr(e)})
summary={"invented_fixture_only":True,"source_unchanged":source==before,"transform_blocking":t["has_blocking_findings"],"adapter_blocking":a["has_blocking_findings"],"traceability_count":len(t["traceability_links"]),"source_model_ref":a["source_model_ref"],"analytical_model_ref":a["model_ref"],"node_mappings":a["nodes"],"connectivity":a["straight_pipe_connectivity"],"checks":checks}
(p/"results/canonical_summary.json").write_text(json.dumps(summary,indent=2)+"\n")
print(json.dumps({"traceability_links":len(t["traceability_links"]),"adapter_blocking":a["has_blocking_findings"],"tests":len(checks),"failures":[c for c in checks if c["outcome"]!="PASS"]}))

sys.exit(1 if any(c["outcome"]!="PASS" for c in checks) else 0)
