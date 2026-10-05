from pathlib import Path
from fractions import Fraction as Q
import ast,struct,math,json,difflib,hashlib
HERE=Path(__file__).resolve().parent
R=HERE.parents[2]
NEW=R/'I46/product_refusal_correction_02/_run_records'
OLD=R/'I46/product_refusal_01/_run_records/original'
def exact(x):return Q(*x.as_integer_ratio()) if isinstance(x,float) else Q(x)
def pow2(e):return Q(2**e) if e>=0 else Q(1,2**-e)
def bits(x):return struct.pack('>d',float(x)).hex()
def unbits(h):return exact(struct.unpack('>d',bytes.fromhex(h))[0])
def exponent(x):
 e=x.numerator.bit_length()-x.denominator.bit_length()
 return e-1 if x<pow2(e) else e
# Independent direct integer nearest-even; no float division/product as reference.
def rn(x):
 x=exact(x)
 if x==0:return x
 sign=-1 if x<0 else 1;x=abs(x);quantum=pow2(max(exponent(x)-52,-1074));scaled=x/quantum
 k,r=divmod(scaled.numerator,scaled.denominator)
 if 2*r>scaled.denominator or (2*r==scaled.denominator and k%2):k+=1
 return sign*k*quantum
# Independent sqrt via exact integer-isqrt on the binary64 quantum, not candidate bit bisection.
def root_rn(x):
 if x==0:return Q()
 quantum=pow2(max(exponent(x)//2-52,-1074));z=x/(quantum*quantum);k=math.isqrt(z.numerator//z.denominator)
 midpoint=(Q(k)+Q(1,2))*quantum
 if x>midpoint*midpoint or (x==midpoint*midpoint and k%2):k+=1
 return k*quantum
def extent(nodes):
 spans=[rn(max(exact(n[k]) for n in nodes)-min(exact(n[k]) for n in nodes)) for k in range(3)]
 sq=[rn(x*x) for x in spans];summed=rn(rn(sq[0]+sq[1])+sq[2]);return root_rn(summed)
def couple(E,L):
 f,m=map(exact,E)
 return [f,m] if L==0 else [max(f,rn(m/L)),max(m,rn(L*f))]
def guarded(hats):return [rn(x*(1+pow2(-40))) for x in hats]
# Audit exact change extent and zero-rule statement directly, then bind to candidate callable scalars.
oldtext=(OLD/'analyze.py').read_text();newtext=(NEW/'analyze.py').read_text()
patch=''.join(difflib.unified_diff(oldtext.splitlines(True),newtext.splitlines(True),fromfile='original/analyze.py',tofile='corrected/analyze.py'))
assert patch==(NEW/'analysis.patch').read_text()
oldzero=next(s for s in oldtext.splitlines() if s.lstrip().startswith('zero_bad='))
newzero=next(s for s in newtext.splitlines() if s.lstrip().startswith('zero_bad='));assert oldzero==newzero
code=ast.parse(newtext);names={'Q','bits','unbits','rn','add','mul','div','sqrt_rn64','body_extent','couple_resolution'}
ns={'F':Q,'struct':struct};nodes=[n for n in code.body if isinstance(n,ast.FunctionDef) and n.name in names]
exec(compile(ast.Module(body=nodes,type_ignores=[]),'candidate_scalar_functions','exec'),ns)
checks=[]
def control(name,coords,E):
 L=extent(coords);hats=couple(E,L);g=guarded(hats)
 actual_L=ns['body_extent'](coords);actual_hats=ns['couple_resolution'](list(map(exact,E)),actual_L);actual_g=[ns['mul'](h,1+pow2(-40)) for h in actual_hats]
 assert actual_L==L and actual_hats==hats and actual_g==g
 check={'name':name,'L_bits':bits(L),'coupled_bits':[bits(x) for x in hats],'guarded_bits':[bits(x) for x in g]};checks.append(check);return L,hats,g
control('nonunit_force_from_moment',[[0,0,0],[0,0,4]],[3,20])
control('nonunit_moment_from_force',[[0,0,0],[0,0,4]],[20,3])
L,hats,g=control('nonunit_division_rounding',[[0,0,0],[1,2,2]],[1,10]);assert bits(hats[0])=='400aaaaaaaaaaaab'
control('zero_extent_omits_coupling',[[2,3,4],[2,3,4]],[3,20])
L,hats,g=control('zero_rule_keeps_original_force_E',[[0,0,0],[0,0,4]],[0,20]);assert hats[0]==5
negativezero=bits(-0.0);assert negativezero=='8000000000000000'
assert exact(0)==0 and negativezero!='0000000000000000' and hats[0]!=0
checks[-1]['uncoupled_zero_rule_discriminator']=True
small=float.fromhex('0x1.6cp-27');L,hats,g=control('extent_operation_order',[[0.,0.,0.],[1.,small,small]],[0,0]);assert bits(L)=='3ff0000000000001'
square=rn(exact(small)**2);reassociated=root_rn(rn(1+rn(square+square)));assert bits(reassociated)=='3ff0000000000000'
checks[-1]['reassociated_extent_bits']=bits(reassociated)
# Cross-check exact published scalar-control record names and all available numerical fields.
reported=json.loads((NEW/'CORRECTION_COMPARISON.json').read_text())['abstract_scalar_controls'];assert len(reported)==6
for got,want in zip(checks,reported):
 assert got['name']==want['name'] and got['L_bits']==want['L_bits']
 for key in ['coupled_bits','guarded_bits']:
  if key in want:assert got[key]==want[key]
# Actual capture-only coupling with corrected scalar reference; zero rule remains original E.
records=json.loads((NEW/'captured_records.json').read_text());cases={};case=None
for r in records:
 if r['tag']=='I45_CASE':case=r['data'];cases[case]={}
 elif case and r['tag'] in ['I45_INPUT','I45_G5A_DATA','I45_ROWS']:cases[case][r['tag']]=r['data']
actual={};trace=json.loads((NEW/'g5a_coupling_inputs.json').read_text())
for name,cap in cases.items():
 E=list(map(unbits,cap['I45_G5A_DATA']['resolution'][0][1:]));L=extent(cap['I45_INPUT']['nodes']);hats=couple(E,L);g=guarded(hats)
 assert bits(L)==trace[name]['body_extent_bits']=='3ff0000000000000'
 assert [bits(x) for x in E]==trace[name]['uncoupled_resolution_bits']
 assert [bits(x) for x in hats]==trace[name]['coupled_resolution_bits']
 assert [bits(x) for x in g]==trace[name]['guarded_resolution_bits']
 zero_fail=[]
 for i,row in enumerate(cap['I45_ROWS']):
  if row['unit'] in ['N','N*m'] and E[0 if row['unit']=='N' else 1]==0 and bits(row['value'])!='0000000000000000':zero_fail.append(i)
 assert zero_fail==(list(range(35,53)) if name=='zero' else [])
 actual[name]={'L_bits':bits(L),'guarded_bits':[bits(x) for x in g],'original_E_zero_failures':zero_fail}
# Recursive comparison does not rederive/reopen any unaffected main proof.
def flatten(obj,prefix=''):
 if isinstance(obj,dict):return {a:b for k,v in obj.items() for a,b in flatten(v,prefix+'/'+k).items()}
 if isinstance(obj,list):return {a:b for i,v in enumerate(obj) for a,b in flatten(v,prefix+'/'+str(i)).items()}
 return {prefix:obj}
a=flatten(json.loads((OLD/'analysis_results.json').read_text()));b=flatten(json.loads((NEW/'analysis_results.json').read_text()));assert a.keys()==b.keys()
changed=[{'path':k,'old':a[k],'new':b[k]} for k in a if a[k]!=b[k]]
assert changed==[{'path':'/cases/loaded/g5a_arithmetic/resolution_guard_bits/1','old':'4042f77f9ea1ecb3','new':'408e82798cc86deb'}]
oldbytes=(OLD/'analysis_results.json').read_bytes();newbytes=(NEW/'analysis_results.json').read_bytes()
assert oldbytes.count(b'4042f77f9ea1ecb3')==1
assert oldbytes.replace(b'4042f77f9ea1ecb3',b'408e82798cc86deb')==newbytes
for f in ['captured_records.json','rx_counterexample.json','exact_check.stdout.log']:assert (OLD/f).read_bytes()==(NEW/f).read_bytes(),f
prior=json.loads((R/'REVIEW_RV61/product_refusal_01/_run_records/independent_results.json').read_text())
new=json.loads((NEW/'analysis_results.json').read_text())
for name in cases:
 for k in ['resolution_guard_bits','lower_bound_bits','sanity','lower_bound','summary_scalar_bounds']:
  assert prior['cases'][name]['g5a_arithmetic'][k]==new['cases'][name]['g5a_arithmetic'][k]
report={'verdict':'PASS; RV61-C1 closed','candidate':'e3e04c6344d2dbab60e1a4a28eaf86f23bd7ea67','method':'Exact integer RN64 and isqrt-on-quantum independently verify candidate scalar functions; no main-proof rerun in independent checker','controls':checks,'actual_captured_checks':actual,'changed_analysis_leaves':changed,'all_analysis_bytes_unchanged_except_single_hex_token':True,'flattened_leaf_count':len(a),'prior_RV61_correct_G5a_results_match':True,'zero_rule_source_line_unchanged':True,'captured_inputs_rx_and_stdout_byte_identical':True}
(HERE/'independent_backcheck.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report,indent=2))
