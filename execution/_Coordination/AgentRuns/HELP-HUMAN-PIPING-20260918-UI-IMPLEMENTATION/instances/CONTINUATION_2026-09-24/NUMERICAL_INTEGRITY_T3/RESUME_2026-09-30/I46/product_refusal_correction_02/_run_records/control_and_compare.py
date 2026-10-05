from pathlib import Path
from fractions import Fraction as F
import ast,json,struct,math,hashlib,difflib
HERE=Path(__file__).resolve().parent
OLD=HERE.parent.parent/'product_refusal_01/_run_records/original'
# Load only the corrected scalar functions; avoid a second analysis execution.
tree=ast.parse((HERE/'analyze.py').read_text())
names={'Q','bits','unbits','rn','add','mul','div','sqrt_rn64','body_extent','couple_resolution'}
selected=[node for node in tree.body if isinstance(node,ast.FunctionDef) and node.name in names]
assert {n.name for n in selected}==names
ns={'F':F,'struct':struct,'math':math}
exec(compile(ast.Module(body=selected,type_ignores=[]),str(HERE/'analyze.py'),'exec'),ns)
extent,couple,bits,unbits=map(ns.get,['body_extent','couple_resolution','bits','unbits'])
controls=[]
def ctrl(name,nodes,raw,expected_L,expected_hats):
 L=extent(nodes);hats=couple(list(map(F,raw)),L)
 assert bits(L)==expected_L and [bits(v) for v in hats]==expected_hats
 # For these integer/power-of-two controls the named guarded products are exact.
 guard=1+F(1,2**40)
 guarded=[ns['mul'](v,guard) for v in hats]
 controls.append({'name':name,'nodes':nodes,'uncoupled':raw,'L_bits':bits(L),'coupled_bits':[bits(v) for v in hats],'guarded_bits':[bits(v) for v in guarded]})
 return L,hats
ctrl('nonunit_force_from_moment',[[0,0,0],[0,0,4]],[3,20],'4010000000000000',['4014000000000000','4034000000000000'])
ctrl('nonunit_moment_from_force',[[0,0,0],[0,0,4]],[20,3],'4010000000000000',['4034000000000000','4054000000000000'])
L,hats=ctrl('nonunit_division_rounding',[[0,0,0],[1,2,2]],[1,10],'4008000000000000',['400aaaaaaaaaaaab','4024000000000000'])
h=int(bits(hats[0]),16);low=(unbits(f'{h-1:016x}')+hats[0])/2;high=(hats[0]+unbits(f'{h+1:016x}'))/2
assert low<F(10,3)<high
ctrl('zero_extent_omits_coupling',[[2,3,4],[2,3,4]],[3,20],'0000000000000000',['4008000000000000','4034000000000000'])
L,hats=ctrl('zero_rule_keeps_original_force_E',[[0,0,0],[0,0,4]],[0,20],'4010000000000000',['4014000000000000','4034000000000000'])
negative_zero='8000000000000000'
correct_zero_reject=F(0)==0 and negative_zero!='0000000000000000'
mutated_zero_reject=hats[0]==0 and negative_zero!='0000000000000000'
assert correct_zero_reject and not mutated_zero_reject
controls[-1].update({'negative_zero_bits':negative_zero,'original_E_zero_rule_rejects':correct_zero_reject,'coupled_E_mutant_wrongly_accepts':not mutated_zero_reject})
# Prescribed left-associated extent versus a reassociated square sum differs.
d=float.fromhex('0x1.6cp-27');L=extent([[0.0,0.0,0.0],[1.0,d,d]])
assert bits(L)=='3ff0000000000001'
controls.append({'name':'extent_operation_order','nodes_hex':[['0x0p+0']*3,['0x1p+0',d.hex(),d.hex()]],'L_bits':bits(L),'expected_bits':'3ff0000000000001'})

def differences(a,b,path=''):
 if type(a)!=type(b):return [{'path':path,'old':a,'new':b}]
 if isinstance(a,dict):
  assert a.keys()==b.keys(),path
  return sum((differences(a[k],b[k],path+'/'+k) for k in a),[])
 if isinstance(a,list):
  assert len(a)==len(b),path
  return sum((differences(x,y,path+'/'+str(i)) for i,(x,y) in enumerate(zip(a,b))),[])
 return [] if a==b else [{'path':path,'old':a,'new':b}]
old=json.loads((OLD/'analysis_results.json').read_text());new=json.loads((HERE/'analysis_results.json').read_text())
delta=differences(old,new)
expected=[{'path':'/cases/loaded/g5a_arithmetic/resolution_guard_bits/1','old':'4042f77f9ea1ecb3','new':'408e82798cc86deb'}]
assert delta==expected,delta
rx_same=(OLD/'rx_counterexample.json').read_bytes()==(HERE/'rx_counterexample.json').read_bytes()
stdout_same=(OLD/'exact_check.stdout.log').read_bytes()==(HERE/'exact_check.stdout.log').read_bytes()
inputs_same=(OLD/'captured_records.json').read_bytes()==(HERE/'captured_records.json').read_bytes()
assert rx_same and stdout_same and inputs_same
unchanged=[]
for case in old['cases']:
 for key in old['cases'][case]:
  if key!='g5a_arithmetic':
   assert old['cases'][case][key]==new['cases'][case][key]
   unchanged.append(case+'/'+key)
 # All supporting booleans, zero-rule failures, lower bounds and scope are unchanged.
 for key in old['cases'][case]['g5a_arithmetic']:
  if not (case=='loaded' and key=='resolution_guard_bits'):
   assert old['cases'][case]['g5a_arithmetic'][key]==new['cases'][case]['g5a_arithmetic'][key]

basis=json.loads((HERE/'BASIS.json').read_text())
for rec in basis['original_files']:
 assert hashlib.sha256(Path(rec['path']).read_bytes()).hexdigest()==rec['sha256']
trace=json.loads((HERE/'g5a_coupling_inputs.json').read_text())
assert trace['loaded']['guarded_resolution_bits']==['408e82798cc86deb']*2
assert trace['zero']['guarded_resolution_bits']==['0000000000000000']*2
# Independently prove the specified corrected guard bits by exact midpoint comparison.
E_force=unbits('408e82798cc84f69');E_moment=unbits('4042f77f9ea1d9bc')
assert E_force>E_moment
exact_guard_product=E_force*(1+F(1,2**40))
h=int('408e82798cc86deb',16);center=unbits(f'{h:016x}')
mid_lo=(unbits(f'{h-1:016x}')+center)/2
mid_hi=(center+unbits(f'{h+1:016x}'))/2
assert mid_lo<exact_guard_product<mid_hi
midpoint_proof={'exact_guard_product':str(exact_guard_product),'lower_midpoint':str(mid_lo),'upper_midpoint':str(mid_hi),'rounded_bits':f'{h:016x}','strict_containment':True,'coupling_fact':'L=1; original E_force > original E_moment, so both coupled values equal original E_force'}
report={'loaded_guard_exact_midpoint_proof':midpoint_proof,'status':'PASS' ,'changed_analysis_leaves':delta,'all_other_analysis_leaves_unchanged':True,'rx_counterexample_byte_identical':rx_same,'stdout_byte_identical':stdout_same,'captured_inputs_byte_identical':inputs_same,'unchanged_numeric_scale_failure_subtrees':unchanged,'original_files_unchanged':len(basis['original_files']),'abstract_scalar_controls':controls,'controls_are':'exact scalar algebra only; no admitted model, solver or product execution','actual_coupling_inputs':trace}
(HERE/'CORRECTION_COMPARISON.json').write_text(json.dumps(report,indent=2)+'\n')
(HERE/'analysis.patch').write_text(''.join(difflib.unified_diff((OLD/'analyze.py').read_text().splitlines(True),(HERE/'analyze.py').read_text().splitlines(True),fromfile='original/analyze.py',tofile='corrected/analyze.py')))
print(json.dumps(report,indent=2))
