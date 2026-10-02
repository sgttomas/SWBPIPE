"""Exact primitive inequalities and frozen-record inventory only; no solver emulation."""
import hashlib,json,math,os,pathlib,subprocess,sys
from fractions import Fraction as F
ROOT=pathlib.Path(sys.argv[1]);R=sys.argv[2];OUT=pathlib.Path.cwd()
REV='40129a225d73860ac2a53da9a2fa73869df668f3';env=dict(os.environ,GIT_OPTIONAL_LOCKS='0')
def sha(x):return hashlib.sha256(x).hexdigest()
def p2(n):return F(2**n) if n>=0 else F(1,2**-n)
def dec(s):
 b=int(s,16);e=(b>>52)&2047;m=b&((1<<52)-1);assert e<2047
 return (-1 if b>>63 else 1)*(m*p2(-1074) if e==0 else ((1<<52)+m)*p2(e-1075))
def exact_sqrt(q):
 n=math.isqrt(q.numerator);d=math.isqrt(q.denominator)
 assert n*n==q.numerator and d*d==q.denominator
 return F(n,d)
def obj(path):
 return subprocess.run(['git','show',REV+':'+path],cwd=ROOT,env=env,check=True,stdout=subprocess.PIPE).stdout
links=json.loads((ROOT/R/'I22/vk_f03_diagnostic_01/CASE_LINKS.json').read_text());byid={x['id']:x for x in links};out=[];sources=[]
for file in ['rf_chain.jsonl','rf_skew.jsonl']:
 path='projects/chirality-piping/validation/benchmarks/numerical_robustness/cases/'+file
 raw=obj(path);sources.append({'path':path,'revision':REV,'sha256':sha(raw)})
 for line in raw.decode().splitlines():
  c=json.loads(line);m=c['model'];id=c['id'];link=byid[id]
  nodes=[[dec(z) for z in n[1:]] for n in m['nodes']]
  root_members=[e for e in m['members'] if 0 in e[2:4]]
  assert len(root_members)==1
  e=root_members[0];diff=[nodes[e[3]][j]-nodes[e[2]][j] for j in range(3)]
  L=exact_sqrt(sum(z*z for z in diff));assert diff[0]>0
  spring=next(s for s in m['springs'] if s[0]=='S.N0.0');k=dec(spring[6])
  rotational=spring[3]=='r';component=3 if rotational else 0
  assert (0,component) not in [tuple(x) for x in m['constraints']]
  E,G,A,Iy,Iz,J=[dec(z) for z in e[4:10]]
  axial_lower_component=(G*J/L if rotational else E*A/L)*(diff[0]/L)**2
  member_lower=axial_lower_component/32
  diagonal=[];soft=None
  for s in m['springs']:
   if s[2]!=0 or (s[3]=='r')!=rotational:continue
   if s[4] is not None:
    if s[4]!=component:continue
    exact=dec(s[6]);lower=upper=exact
   else:
    n=[dec(z) for z in s[5]];exact=dec(s[6])*n[0]**2/sum(z*z for z in n)
    lower=exact/2;upper=exact*2
   row={'id':s[1],'key':s[0],'kind':'directional' if s[4] is None else 'global','exact_intended_diagonal':str(exact),'formed_lower':str(lower),'formed_upper':str(upper)}
   diagonal.append(row)
   if s[0]=='S.N0.0':soft=(lower,upper)
  assert soft and soft[0]>0 and member_lower>soft[1]
  for x in diagonal:
   if x['key']!='S.N0.0':assert F(x['formed_lower'])>soft[1]
  record={'id':id,'family':c['family'],'source_sha256':link['source_sha256'],'root_member':e[:4],
   'soft_spring':spring,'root_global_dof':component,'intended_chord_length':str(L),
   'member_diagonal_certified_lower':str(member_lower),'spring_diagonal_bounds':diagonal,
   'strict_minimum':'S.N0.0 positive, unique at root X diagonal','member_lower_over_soft_upper':str(member_lower/soft[1]),
   'NONE_sequence':link['NONE_sequence'],'F03_sequence':link['F03_sequence'],'reported_global_dof':link['observed_global_dof'],
   'reported_component':link['component'],'outcome_source_attribution':'root soft diagonal deletion plus controlled F03 outcome; no claim root deletion alone produces reported pivot',
   'historical_NC_LOST_SOFT_controls':link['historical_NC_LOST_SOFT_controls']}
  if file=='rf_chain.jsonl':
   coeff=[]
   for a in m['members']:
    dx=nodes[a[3]][0]-nodes[a[2]][0]
    assert all(nodes[a[3]][j]==nodes[a[2]][j] for j in [1,2]) and dx>0
    ka=dec(a[5])*dec(a[9])/dx if rotational else dec(a[4])*dec(a[6])/dx
    coeff.append(ka)
   record['interior_positive_minima_count']=len(coeff)-1
   record['exact_equal_input_adjacent_pairs']=[i+1 for i in range(len(coeff)-1) if coeff[i]==coeff[i+1]]
   record['ideal_rigid_vector_deleted_energy']=str(-sum(min(a,b) for a,b in zip(coeff,coeff[1:])))
   record['formed_energy_fact']='negative sum of actual positive smaller adjacent formed coefficients; independent of ties'
  else:
   zdirs=[s for s in m['springs'] if s[2]==0 and (s[3]=='r')==rotational and s[4] is None and dec(s[5][2])==0]
   record['root_Z_exact_zero_directional_terms']=[s[0] for s in zdirs]
   record['root_Z_zero_minimum_tie_count']=len(zdirs)
  out.append(record)
assert len(out)==66 and {x['id'] for x in out}==set(byid)
result={'status':'SOURCE_CONSEQUENCE_FOR_RV29','basis':REV,'case_count':66,'source_inputs':sources,
 'minimum_member_lower_over_soft_upper':str(min(F(x['member_lower_over_soft_upper']) for x in out)),
 'all_root_X_soft_terms_positive_unique_minima':True,'cases':out}
(OUT/'ROOT_DIAGONAL_INEQUALITIES.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({k:result[k] for k in ['case_count','minimum_member_lower_over_soft_upper','all_root_X_soft_terms_positive_unique_minima']}))

