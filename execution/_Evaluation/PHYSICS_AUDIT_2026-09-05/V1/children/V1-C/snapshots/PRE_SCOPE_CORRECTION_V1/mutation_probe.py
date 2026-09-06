"""Isolated observation-corruption mutation; never edits production or expectations."""
from pathlib import Path
import json,re,subprocess,hashlib,os,sys
O=Path(__file__).resolve().parent
P=next(x for x in O.parents if x.name=='chirality-piping')
SOURCE=P/'core/product_physics'
CASE=O/'mutation_copy'
TESTS=['valid_invented_model_exposes_element_force_components','valid_invented_model_exposes_endpoint_stress_components','fixed_fixed_thermal_load_applies_axial_fixed_end_correction']
ORIGINAL='    run_linear_static_preview_with_mode(request, PreviewSolverMode::default())\n'
MUTATED='''    let mut result = run_linear_static_preview_with_mode(request, PreviewSolverMode::default());
    for row in &mut result.results {
        if row.id.starts_with("result:force:") || row.id.starts_with("result:moment:") || row.id.starts_with("result:stress:") { row.value = 0.0; }
    }
    result
'''
def prep():
 (CASE/'src').mkdir(parents=True,exist_ok=True)
 manifest=(SOURCE/'Cargo.toml').read_text()
 manifest=re.sub(r'path = "(\.\.[^"]+)"',lambda m:'path = '+json.dumps(str((SOURCE/m[1]).resolve())),manifest)
 (CASE/'Cargo.toml').write_text(manifest)
 for f in (SOURCE/'src').glob('*.rs'):
  s=f.read_text()
  s=re.sub(r'include_str!\(\s*"([^"]+)"\s*\)',lambda m:'include_str!('+json.dumps(str((f.parent/m[1]).resolve()))+')',s)
  (CASE/'src'/f.name).write_text(s)
 (O/'MUTATION_EXPECTATIONS.json').write_text(json.dumps({'source_sha256':hashlib.sha256((SOURCE/'src/lib.rs').read_bytes()).hexdigest(),'mutation':'Zero all default force/moment/stress published values; IDs, units, metadata, other rows unchanged','predeclared_tests':[{'test':t,'baseline_expected_exit':0,'mutant_expected_exit':0 if i<2 else 101} for i,t in enumerate(TESTS)],'claim_boundary':'Two selected metadata tests are insensitive; numeric thermal control sensitive. Not full-suite mutation score or physical correctness.'},indent=2)+'\n')
def run():
 if not (O/'MUTATION_EXPECTATIONS_V2.json').exists():raise RuntimeError('prepare expectations first')
 env=dict(os.environ,CARGO_NET_OFFLINE='true',CARGO_TARGET_DIR=str(CASE/'target'))
 lib=CASE/'src/lib.rs'; baseline=lib.read_text(); assert baseline.count(ORIGINAL)==1
 reaction='                    reactions[global] * reactions[global]'
 assert baseline.count(reaction)==1
 station=MUTATED.replace('row.id.starts_with("result:force:") || row.id.starts_with("result:moment:") || row.id.starts_with("result:stress:")','row.metadata.as_ref().map(|m| m.basis == "interpolated_from_endpoint_resultants").unwrap_or(false)')
 modes={'baseline':baseline,'W3_force_only_reaction':baseline.replace(reaction,'                    if dof_index(dof) < 3 { reactions[global] * reactions[global] } else { 0.0 }'),'W5_zero_interpolated_stations':baseline.replace(ORIGINAL,station),'positive_zero_force_stress':baseline.replace(ORIGINAL,MUTATED)}
 records=[]
 for mode,source in modes.items():
  lib.write_text(source)
  cmd=['cargo','test','--offline','--manifest-path',str(CASE/'Cargo.toml'),'--lib']
  completed=subprocess.run(cmd,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,env=env)
  record={'mode':mode,'mutated_source_sha256':hashlib.sha256(source.encode()).hexdigest(),'command':cmd,'exit_code':completed.returncode,'output':completed.stdout}
  (O/f'MUTATION_{mode}.json').write_text(json.dumps(record,indent=2)+'\n')
  records.append({'mode':mode,'exit_code':completed.returncode,'summary':re.findall(r'test result:.*',completed.stdout)})
  print(records[-1],flush=True)
 (O/'MUTATION_RESULTS.json').write_text(json.dumps(records,indent=2)+'\n')
if __name__=='__main__':
 if sys.argv[1]=='prepare':prep()
 elif sys.argv[1]=='run':run()
