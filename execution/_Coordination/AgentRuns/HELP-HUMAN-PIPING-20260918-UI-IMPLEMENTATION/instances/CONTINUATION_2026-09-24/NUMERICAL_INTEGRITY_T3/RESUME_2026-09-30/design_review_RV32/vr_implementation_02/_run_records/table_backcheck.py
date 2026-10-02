import ast,re,json,hashlib
from pathlib import Path
out=Path(__file__).resolve().parent
r=out.parents[2]
root=r.parents[8]
# Explicit source root, avoiding inference from the deep evidence hierarchy.
root=Path('/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/k6c')
vr=out/'clean/projects/chirality-piping/validation/benchmarks/numerical_robustness'
s=(vr/'tests/k6c_envelope.rs').read_text()
def table(name):
 t=s[s.index('const '+name+':'):];start=t.index('= &[')+3;end=t.index('\n];',start)+2
 return ast.literal_eval(t[start:end])
def rows(rel):return json.loads((r/rel).read_text())['rows']
expected=table('EXPECTED');callers=table('CALLERS');cli=table('CLI_EXPECTED')
ref=rows('metric_design_15_result5_join/RESULT5_REFERENCE193.json')
cr=rows('metric_design_14_reference_callers/REFERENCE193_CALLERS.json')
cf={x['id']:x for x in rows('metric_design_15_result5_join/RESULT5_CLI24.json')}
cc={x['id']:x for x in rows('metric_design_14_reference_callers/CLI24_CALLERS.json')}
launch={x['id']:x for x in rows('metric_design_14_reference_callers/CLI24_LAUNCHES.json')}
fields=['model','fixed','decide','max','sel128']
phases=re.search(r'pub const CALLER_PHASES:.*?= \[(.*?)\];',(vr/'src/envelope.rs').read_text(),re.S).group(1)
phases=re.findall(r'"([^"]+)"',phases)
joins=['solve_max','selected_or_other_outcome_comparison','record1_build_while_outcome_alive','expected_list_initialization_while_outcome_alive','nonselected_diagnostic_while_outcome_alive']
results=[]
assert len(expected)==len(callers)==len(ref)==len(cr)==193
for ((id,req,mov),(pr,pm,jr,jm),a,b) in zip(expected,callers,ref,cr):
 assert id==a['id']==b['id']
 checks={}
 for metric,values,pp,jj in [('requested',req,pr,jr),('moving',mov,pm,jm)]:
  checks[metric+'_fields']=values==[a['metrics'][metric]['fields'][f] for f in fields]
  checks[metric+'_phases']=pp==[b[metric]['caller_only_phases'][p] for p in phases]
  checks[metric+'_joins']=jj==[b[metric]['kernel_phase_addends'][p] for p in joins]
 results.append({'id':id,'checks':checks})
cli_results=[]
assert len(cli)==12
for id,ml,el,req,mov,pr,pm,jr,jm in cli:
 checks={'manifest_length':ml==launch[id]['manifest_path_utf8_bytes'],'executable_length':el==launch[id]['argument_utf8_lengths'][0],'embedded':launch[id]['external_model_path_utf8_bytes']==0,'exact_population_policy':cc[id]['shape']['B']==cf[id]['descriptor']['B'] and cc[id]['shape']['b']==cf[id]['descriptor']['b']}
 for metric,values,pp,jj in [('requested',req,pr,jr),('moving',mov,pm,jm)]:
  checks[metric+'_fields']=values==[cf[id]['metrics'][metric]['fields'][f] for f in fields]
  checks[metric+'_phases']=pp==[cc[id][metric]['caller_only_phases'][p] for p in phases]
  checks[metric+'_joins']=jj==[cc[id][metric]['kernel_phase_addends'][p] for p in joins]
 cli_results.append({'id':id,'checks':checks})
assert all(all(x['checks'].values()) for x in results+cli_results)
files=['metric_design_15_result5_join/RESULT5_REFERENCE193.json','metric_design_14_reference_callers/REFERENCE193_CALLERS.json','metric_design_15_result5_join/RESULT5_CLI24.json','metric_design_14_reference_callers/CLI24_CALLERS.json','metric_design_14_reference_callers/CLI24_LAUNCHES.json']
output={'basis':'Independent read of accepted caller14/result5join15 tables, not I24 output. Maintained test constants are matched field-by-field and order-by-order; clean archive suite executes implementation against them.','reference_rows':len(results),'cli_rows':len(cli_results),'reference_numeric_cells':193*(2*5+2*15+2*5),'cli_numeric_cells':12*(2*5+2*15+2*5),'phase_order':phases,'join_order':joins,'all_match':True,'reference':results,'cli':cli_results,'inputs':[{'path':p,'sha256':hashlib.sha256((r/p).read_bytes()).hexdigest()} for p in files]}
(out/'table_backcheck.json').write_text(json.dumps(output,indent=2)+'\n')
print(json.dumps({k:v for k,v in output.items() if k not in ('reference','cli','inputs','phase_order','join_order')},indent=2))
