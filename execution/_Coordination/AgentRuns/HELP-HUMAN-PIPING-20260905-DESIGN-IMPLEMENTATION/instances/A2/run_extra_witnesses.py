from pathlib import Path
import json,subprocess,hashlib,sys
p=Path(__file__).resolve().parent
binary=Path(sys.argv[1])
for name,fixture in [("nonlinear","invented_nonlinear.json"),("nonlinear_removed","invented_nonlinear_removed.json"),("native_revised","invented_linear_revised.json")]:
 r=subprocess.run([str(binary),str(p/"fixtures"/fixture)],text=True,capture_output=True,check=True)
 (p/"results"/f"{name}.json").write_text(r.stdout)
r=subprocess.run([str(binary),str(p/"fixtures/canonical_analytical.json")],text=True,capture_output=True)
(p/"results/canonical_rejected.json").write_text(json.dumps({"exit_code":r.returncode,"stderr":r.stderr},indent=2)+"\n")
a=json.loads((p/"results/nonlinear.json").read_text());b=json.loads((p/"results/nonlinear_removed.json").read_text())
common={r["id"]:r for r in b["results"]};equal=[];diffs=[];kinds=set()
for row in a["results"]:
 selected=row["kind"].startswith(("global_nodal_","element_local_")) or row["kind"] in ["displacement_magnitude","reaction_resultant","open_formula_stress_summary","pipe_section_pressure_hoop_stress"]
 if selected and row["id"] in common:
  kinds.add(row["kind"])
  (equal if row==common[row["id"]] else diffs).append(row["id"])
a1=json.loads((p/"results/native_m.json").read_text());a2=json.loads((p/"results/native_revised.json").read_text())
summary={"generic_rows_compared":len(equal)+len(diffs),"generic_rows_equal_after_removing_nonlinear_supports":len(equal),"generic_rows_changed":diffs,"compared_kinds":sorted(kinds),"first_displacement":a1["summary"]["max_displacement"],"revised_displacement":a2["summary"]["max_displacement"],"same_run_id_after_changed_geometry":a1["run_id"]==a2["run_id"],"same_model_ref_after_changed_geometry":a1["model_ref"]==a2["model_ref"],"baseline_binary_sha256":hashlib.sha256(binary.read_bytes()).hexdigest()}
(p/"results/identity_and_nonlinear_summary.json").write_text(json.dumps(summary,indent=2)+"\n");print(json.dumps(summary))
