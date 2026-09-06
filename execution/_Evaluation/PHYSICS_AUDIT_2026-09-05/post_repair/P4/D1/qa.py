from pathlib import Path
import json,math,hashlib,csv,subprocess
r=Path(subprocess.check_output(['git','rev-parse','--show-toplevel'],text=True).strip());w=r/'projects/chirality-piping';d=Path(__file__).resolve().parent
new=json.loads((d/'emitter.raw.json').read_text());old=json.loads((w/'validation/benchmarks/sparse_default_promotion_observation.dec053.json').read_text());policy=json.loads((w/'validation/benchmarks/sparse_default_promotion_policy.dec053.json').read_text())
checks=[]
def check(name,value):
 checks.append({'check':name,'pass':bool(value)})
check('nine observations',new['observation_count']==len(new['observations'])==9)
check('exact ordered fixture IDs',[x['fixture_id'] for x in new['observations']]==policy['evidence_requirements']['required_fixture_ids'])
check('nine distinct practical size bands',len({x['practical_size_band'] for x in new['observations']})==9)
check('host RSS observed positive; no threshold',new['hardware_metadata']['rss_kib_at_packet_emit']>0)
check('dependency/control hash stability',json.loads((d/'source_before.json').read_text())==json.loads((d/'source_after.json').read_text()))
checkpoint=w/'execution/_Evaluation/PHYSICS_AUDIT_2026-09-05/post_repair/P4/KERNEL_CHECKPOINT_V1'
for x in json.loads((checkpoint/'MANIFEST.json').read_text())['files']:
 check('accepted checkpoint '+x['path'],hashlib.sha256((checkpoint/x['path']).read_bytes()).hexdigest()==x['sha256'])
rows=[]
for a,b in zip(old['observations'],new['observations']):
 fid=b['fixture_id'];lim=policy['accepted_thresholds_for_observation_set']
 check(fid+' required metrics present',all(k in b for k in policy['evidence_requirements']['required_metrics']))
 check(fid+' all numeric observations finite',all(math.isfinite(v) for v in b.values() if isinstance(v,(int,float))))
 check(fid+' required numbers not null',all(isinstance(b[k],(int,float)) and math.isfinite(b[k]) for k in policy['evidence_requirements']['required_metrics'] if k not in ['allocator_rss_observation_status','hardware_normalization_status']))
 check(fid+' established parity',0<=b['sparse_dense_relative_delta']<=lim['sparse_dense_relative_delta_limit'])
 check(fid+' established residual',0<=b['max_abs_sparse_residual']<=lim['sparse_residual_absolute_limit'])
 check(fid+' established repeat',b['max_abs_sparse_repeat_solution_delta']==lim['sparse_repeat_solution_delta_absolute_limit'])
 check(fid+' established pivot count',b['nonpositive_pivot_count']==lim['nonpositive_pivot_count_limit'])
 check(fid+' condition >=1 and pivot proxy positive',b['true_condition_number_2norm']>=1 and b['sparse_pivot_condition_ratio_estimate']>0)
 check(fid+' observed timing/storage positive',all(b[k]>0 for k in ['dense_first_solve_elapsed_nanos','sparse_first_solve_elapsed_nanos','dense_reduced_matrix_value_storage_bytes','sparse_ordered_profile_value_storage_bytes']))
 stable=['fixture_id','fixture_family','practical_size_band','node_count','element_count','total_dofs','reduced_dofs','dense_reduced_matrix_value_storage_bytes','sparse_ordered_profile_value_storage_bytes','sparse_pivot_condition_ratio_estimate','max_abs_sparse_dense_solution_delta','sparse_dense_relative_delta','max_abs_sparse_residual','max_abs_sparse_repeat_solution_delta','nonpositive_pivot_count']
 check(fid+' historical mechanics and fixture fields unchanged',all(a[k]==b[k] for k in stable))
 rows.append({'fixture_id':fid,'historical_condition':a['true_condition_number_2norm'],'corrected_condition':b['true_condition_number_2norm'],'ratio_corrected_to_historical':b['true_condition_number_2norm']/a['true_condition_number_2norm'],'relative_parity':b['sparse_dense_relative_delta'],'aggregate_residual':b['max_abs_sparse_residual'],'repeat_delta':b['max_abs_sparse_repeat_solution_delta'],'nonpositive_pivots':b['nonpositive_pivot_count'],'interpretation':'corrected spectral diagnostic; mechanics/parity/storage fields unchanged; no condition acceptance threshold'})
with (d/'comparison.csv').open('w',newline='') as f:
 writer=csv.DictWriter(f,fieldnames=list(rows[0]),lineterminator='\n');writer.writeheader();writer.writerows(rows)
sandbox=json.loads((d/'sandbox_attempt/emitter.raw.json').read_text())
check('sandbox and host numeric mechanics identical',all(all(a[k]==b[k] for k in a if k not in ['dense_first_solve_elapsed_nanos','sparse_first_solve_elapsed_nanos']) for a,b in zip(sandbox['observations'],new['observations'])))
qa={'status':'PASS' if all(x['pass'] for x in checks) else 'FAIL','checks':checks,'criteria_source':'validation/benchmarks/sparse_default_promotion_policy.dec053.json','scope':'bounded historical observation criteria and metadata invariants only; no production equilibrium acceptance or unit-normalized admission'}
(d/'QA.json').write_text(json.dumps(qa,indent=2)+'\n');print(qa['status'],len(checks),'checks; RSS',new['hardware_metadata']['rss_kib_at_packet_emit']);assert qa['status']=='PASS',[x for x in checks if not x['pass']]
