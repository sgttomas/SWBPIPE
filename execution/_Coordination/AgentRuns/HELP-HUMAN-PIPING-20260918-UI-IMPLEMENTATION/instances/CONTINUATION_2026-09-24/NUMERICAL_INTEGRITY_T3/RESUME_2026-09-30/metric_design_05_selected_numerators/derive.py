"""Fixed-source power bounds only. No reference values, solver or model generation."""
from fractions import Fraction as F
import hashlib,json,os,pathlib,subprocess,sys

ROOT=pathlib.Path(sys.argv[1]);R=sys.argv[2];OUT=pathlib.Path.cwd()
REV='40129a225d73860ac2a53da9a2fa73869df668f3'
env=dict(os.environ,GIT_OPTIONAL_LOCKS='0')
def p2(e):return F(2**e) if e>=0 else F(1,2**(-e))
def decode(s):
 b=int(s,16);sgn=-1 if b>>63 else 1;ex=(b>>52)&2047;fr=b&((1<<52)-1)
 assert ex!=2047
 return sgn*(F(fr)*p2(-1074) if ex==0 else F((1<<52)+fr)*p2(ex-1075))
def floorlog(x):
 assert x>0
 e=x.numerator.bit_length()-x.denominator.bit_length()
 return e if x>=p2(e) else e-1
def ceil_log(x):
 e=floorlog(x);return e if x==p2(e) else e+1
def sha(b):return hashlib.sha256(b).hexdigest()
source14=ROOT/R/'I21/source_14'
desc=json.loads((source14/'FIXED_INEQUALITIES_ALL.json').read_text())
records=[];input_records=[];loaded={}
for c in desc['cases']:
 gaps=c['observed_division_gaps']
 if not gaps:continue
 path=c['file']
 if path not in loaded:
  raw=subprocess.run(['git','show',REV+':'+path],cwd=ROOT,env=env,check=True,stdout=subprocess.PIPE).stdout
  assert (ROOT/path).read_bytes()==raw
  loaded[path]=raw.decode().splitlines();input_records.append({'path':path,'revision':REV,'sha256':sha(raw)})
 case=json.loads(loaded[path][c['line']-1]);assert case['id']==c['id']
 m=case['model'] # Never inspect expected rows, controls or reference scales.
 nodes=[[decode(v) for v in x[1:]] for x in m['nodes']]
 members=m['members'];constraints={tuple(x) for x in m['constraints']}
 assert all((0,j) in constraints for j in range(3))
 assert len(members)==len(nodes)-1
 seen={0}
 while True:
  nxt=seen|{int(e[j]) for e in members for j,k in [(2,3),(3,2)] if int(e[k]) in seen}
  if nxt==seen:break
  seen=nxt
 assert len(seen)==len(nodes)
 assert not m['omitted_springs']
 # ell0 bounds total physical tree length by a power of two.
 chords={}
 for e in members:
  d=floorlog(max(abs(nodes[e[3]][j]-nodes[e[2]][j]) for j in range(3)))
  chords[e[0]]=d
 length_sum=sum((p2(d+2) for d in chords.values()),F(0))
 D=ceil_log(length_sum);length_scale=p2(D)
 body_d=floorlog(max(max(x[j] for x in nodes)-min(x[j] for x in nodes) for j in range(3)))
 assert D>=body_d+2
 assert -511<=body_d and 2*body_d+4<=1023
 C_coupling_exp=max(2,2+D-body_d)
 assert C_coupling_exp<=20
 # Cbar is a diagonal-compliance bound for root-grounded released tree.
 Cbar=F(0);member_proofs={}
 for e in members:
  name=e[0];d=chords[name]
  E,G,A,Iy,Iz,J=[decode(b) for b in e[4:10]]
  assert min(E,G,A,Iy,Iz,J)>0
  assert all(-1022<=floorlog(z)<=1023 for z in [E,G,A,Iy,Iz,J])
  # True member length <= 2^(d+2); these are exact rational compliance bounds.
  lc=p2(d+2)
  term=lc/(E*A*length_scale**2)+lc/(G*J)+lc/(E*Iy)+lc/(E*Iz)
  Cbar+=term
  member_proofs[name]={'id':e[1],'chord_max_component_exp':d,
     'E_exp':floorlog(E),'G_exp':floorlog(G),'A_exp':floorlog(A),'J_exp':floorlog(J)}
 # Keep only necessary root rotational springs; release other constraints.
 spring_compliance=F(0)
 for axis in range(3,6):
  if (0,axis) in constraints:continue
  ss=[s for s in m['springs'] if s[2]==0 and s[3]=='r' and s[4]==axis]
  assert len(ss)==1
  s=ss[0];direction=[decode(x) for x in s[5]]
  assert direction==[F(int(j==axis-3)) for j in range(3)]
  k=decode(s[6]);assert k>0
  assert -1022<=floorlog(k)<=1023
  spring_compliance+=1/k
 Cbar+=spring_compliance
 Fbar=sum((abs(decode(x[2]))*(length_scale if x[1]<3 else 1) for x in m['loads']),F(0))
 assert Fbar>0
 U=Cbar*Fbar;u=ceil_log(U)
 alpha=p2(C_coupling_exp-64)
 assert alpha<F(1,2) and (1+alpha)/(1-alpha)<4
 row_records=[]
 for g in gaps:
  assert case['rows'][g['row_index']][0]==g['key']
  e=next(e for e in members if e[0]==g['member']);mp=member_proofs[e[0]]
  diffs=[nodes[e[3]][j]-nodes[e[2]][j] for j in range(3)]
  nonzero=[abs(z) for z in diffs if z]
  exact_axis=len(nonzero)==1 and nonzero[0]==p2(mp['chord_max_component_exp'])
  ea,eb=(mp['G_exp'],mp['J_exp']) if g['coefficient']=='kt' else (mp['E_exp'],mp['A_exp'])
  ell=ea+eb-mp['chord_max_component_exp']-(0 if exact_axis else 2)
  assert ell==g['lower_pow2']
  assert -1022<=-ea<=1021
  assert -1022<=ell and ea+eb-mp['chord_max_component_exp']+2<=1023
  h=(mp['G_exp']+mp['J_exp'] if g['coefficient']=='kt' else mp['E_exp']+mp['A_exp'])-mp['chord_max_component_exp']+2
  assert h==g['upper_pow2']
  # candidate state <= 4 U; dot <= 8 component bound, end difference <=16;
  # coefficient <= 2^h; lane divisor >=2^ell. Binary64 rounding preserves powers.
  power=u+(D if g['coefficient']=='ka' else 0)+h-g['lower_pow2']+6
  numerator_power=power+ell
  assert -1022<=numerator_power<=1023
  row_records.append({'row_index':g['row_index'],'key':g['key'],'member':g['member'],
    'j_end_component':'RX' if g['coefficient']=='kt' else 'UX',
    'lane_coefficient_lower_pow2':g['lower_pow2'],'retained_coefficient_upper_pow2':h,
    'lane_coefficient_lower_independently_rederived':True,'exact_axis_power2_chord':exact_axis,
    'retained_and_published_numerator_absolute_upper_pow2':numerator_power,
    'quotient_absolute_upper_pow2':power,'proved_finite':power<=1023})
 records.append({'id':c['id'],'file':path,'line':c['line'],'source_model_sha256':sha(json.dumps(m,sort_keys=True,separators=(',',':')).encode()),
  'source_nodes':len(nodes),'source_members':len(members),'root_grounded_tree_checks':True,
  'length_scale_pow2':D,'body_span_lower_pow2':body_d,'coupling_bound_pow2':C_coupling_exp,
  'exact_compliance_upper':str(Cbar),'exact_normalized_load_l1_upper':str(Fbar),
  'exact_truth_normalized_displacement_upper':str(U),
  'compliance_upper_ceil_pow2':ceil_log(Cbar),'normalized_load_l1_ceil_pow2':ceil_log(Fbar),
  'exact_truth_normalized_displacement_upper_pow2':u,
  'rows':row_records})
result={'status':'PROPOSED_SOURCE_DERIVATION','basis':REV,
 'premises':['accepted R7 verification error theorem and complete T/R rule(a)',
 'lane coefficient lower bounds independently rederived from primitives; source14 itself remains pending RV30',
 'rooted-tree complementary-energy and constraint-release derivation in DERIVATION.md'],
 'input_records':input_records,'source14_descriptor_sha256':sha((source14/'FIXED_INEQUALITIES_ALL.json').read_bytes()),
 'cases':records,'summary':{'cases':len(records),'rows':sum(len(c['rows']) for c in records),
 'finite_rows':sum(r['proved_finite'] for c in records for r in c['rows']),
 'worst_quotient_upper_pow2':max(r['quotient_absolute_upper_pow2'] for c in records for r in c['rows'])}}
(OUT/'DERIVED_BOUNDS.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result['summary']))
for c in records:print(c['id'],c['length_scale_pow2'],c['exact_truth_normalized_displacement_upper_pow2'],max(r['quotient_absolute_upper_pow2'] for r in c['rows']))
