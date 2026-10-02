import ast,copy,importlib.util,json,sys,types
from pathlib import Path
out=Path(__file__).resolve().parent
old=out.parents[1]/'vr_implementation_02/_run_records/clean'
new=out/'clean'
hrel=Path('projects/chirality-piping/core/solver/performance_harness/runner/k6_runner.py')
vrel=Path('projects/chirality-piping/validation/benchmarks/numerical_robustness/runner/vk_scale_runner.py')
def load(path,name):
 spec=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m
def fn(source,name):return next(x for x in ast.parse(source).body if isinstance(x,ast.FunctionDef) and x.name==name)
h=load(new/hrel,'repaired_h');v=load(new/vrel,'repaired_vr')
oldh=(old/hrel).read_text();oldv=(old/vrel).read_text();newh=(new/hrel).read_text();newv=(new/vrel).read_text()
# Only the old pure admission definitions are compiled, with the unchanged global
# helper/constants environment; old VR is explicitly connected to old H admission.
hglobals=dict(vars(h));exec(compile(ast.Module(body=[fn(oldh,'admission')],type_ignores=[]),'<908 pure H admission>','exec'),hglobals)
old_h=types.SimpleNamespace(**hglobals)
vglobals=dict(vars(v),k6=old_h);exec(compile(ast.Module(body=[fn(oldv,'admission')],type_ignores=[]),'<908 pure VR admission>','exec'),vglobals)
old_v=vglobals['admission']
classifications=[None,'not_run','ok','error','timed_out','heap_cap_abort']
nh=nv=0
for run in h.schedule():
 previous=h.previous_size(run['model'])
 for classification in classifications:
  measured=[] if previous is None else [dict(run,model=previous,run_id='previous',members=max(1,run['members']//10),classification=classification)]
  for estimate in ['absent','empty',None,0,1,run['heap_cap_bytes']//2,run['heap_cap_bytes']//2+1,10**12]:
   counts={} if estimate=='absent' else {run['model']:({} if estimate=='empty' else {h.estimate_key(run['mode']):estimate})}
   for ascent in [False,True]:
    a=old_h.admission(run,copy.deepcopy(counts),measured,None,require_ascent=ascent)
    b=h.admission(run,copy.deepcopy(counts),measured,None,require_ascent=ascent)
    assert a==b,(run['run_id'],classification,estimate,ascent,a,b);nh+=1
for run in v.schedule():
 previous=h.previous_size(run['model'])
 for classification in classifications:
  measured=[] if previous is None else [dict(run,model=previous,run_id='previous',members=max(1,run['members']//10),classification=classification)]
  for estimate in ['absent','empty',None,0,1,run['heap_cap_bytes']//2,run['heap_cap_bytes']//2+1,10**12]:
   counts={} if estimate=='absent' else {run['model']:({} if estimate=='empty' else {'estimate_max_bytes':estimate,h.estimate_key(v.MODE):estimate})}
   for approved in [False,True]:
    a=old_v(run,copy.deepcopy(counts),measured,None,None,approved);b=v.admission(run,copy.deepcopy(counts),measured,None,None,approved)
    assert a==b,(run['run_id'],classification,estimate,approved,a,b);nv+=1
ast_checks={}
for label,a,b,names in [('H',oldh,newh,['plan','binary_argv','bind_w1_launch_counts','run_counts']),('VR',oldv,newv,['plan','binary_argv','bind_scale_launch_counts','run_counts'])]:
 for name in names:
  try:oldnode=fn(a,name);newnode=fn(b,name)
  except StopIteration:
   assert label=='H' and name=='run_counts';continue
  ast_checks[label+'.'+name]=ast.dump(oldnode)==ast.dump(newnode)
assert all(ast_checks.values())
print(json.dumps({'interpreter':sys.executable,'version':sys.version,'comparison':'old pure admission definitions from frozen908 vs repaired10315 with unchanged helpers/constants; old VR connected to old H admission','H_equal_dictionaries':nh,'VR_equal_dictionaries':nv,'unchanged_function_AST':ast_checks,'VR_counts_cap':v.COUNTS_CAP_BYTES,'all_match':True},indent=2))
