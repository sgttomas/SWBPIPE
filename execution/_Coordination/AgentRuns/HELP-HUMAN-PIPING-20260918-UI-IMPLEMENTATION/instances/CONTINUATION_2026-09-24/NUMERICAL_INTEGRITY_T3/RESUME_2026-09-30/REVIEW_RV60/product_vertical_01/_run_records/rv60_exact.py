"""Independent RV60 Fraction interval/closed cantilever verifier. No product imports."""
from fractions import Fraction as Q
from math import isqrt, sqrt, nextafter, inf
from pathlib import Path
import struct,json,sys,collections

def bits(x):return struct.pack('>d',x).hex()
def fhex(s):return struct.unpack('>d',bytes.fromhex(s))[0]
def rational(x):return x if isinstance(x,Q) else Q(x)
class I:
 def __init__(self,a,b=None):self.a=rational(a);self.b=rational(a if b is None else b);assert self.a<=self.b
 def __add__(self,o):o=iv(o);return I(self.a+o.a,self.b+o.b)
 __radd__=__add__
 def __neg__(self):return I(-self.b,-self.a)
 def __sub__(self,o):return self+-iv(o)
 def __rsub__(self,o):return iv(o)+-self
 def __mul__(self,o):
  o=iv(o);p=[a*b for a in (self.a,self.b) for b in (o.a,o.b)];return I(min(p),max(p))
 __rmul__=__mul__
 def __truediv__(self,o):
  o=iv(o);assert o.a*o.b>0;return self*I(1/o.b,1/o.a)
 def __rtruediv__(self,o):return iv(o)/self
 def root(self):
  assert self.a>=0
  den=1<<640
  def lower(q):return Q(isqrt((q.numerator*den*den)//q.denominator),den)
  a=lower(self.a);b=lower(self.b)
  return I(a,b if b*b==self.b else b+Q(1,den))
def iv(x):return x if isinstance(x,I) else I(x)
def atan(q,n):
 q=Q(1,q);s=sum(((-1)**i*q**(2*i+1)/Q(2*i+1) for i in range(n)),Q(0));t=(-1)**n*q**(2*n+1)/Q(2*n+1);return I(min(s,s+t),max(s,s+t))
PI=16*atan(5,200)-4*atan(239,64)
assert PI.b-PI.a<Q(1,2**900)
def errors(y,truths):
 y=Q(y)
 lo=max(max(t.a-y,y-t.b,0) for t in truths)
 hi=max(max(abs(y-t.a),abs(y-t.b)) for t in truths)
 return lo,hi
def decide(err,limit):
 if err[1]<=limit:return True
 if err[0]>limit:return False
 raise AssertionError('independent precision insufficient at predicate boundary')
def norm(r):return r['value']/1000.0 if r['unit']=='mm' else r['value']*1e6 if r['unit']=='MPa' else r['value']
ACTIONS={'element_local_axial_force':0,'element_local_shear_force_y':1,'element_local_shear_force_z':2,'element_local_torsional_moment':3,'element_local_bending_moment_y':4,'element_local_bending_moment_z':5}
DISP={f'global_nodal_{kind}_{axis}':i for i,(kind,axis) in enumerate([(k,a) for k in ['displacement','rotation'] for a in 'xyz'])}
STRESS={'element_local_axial_normal_stress':0,'element_local_bending_normal_stress_y':4,'element_local_bending_normal_stress_z':5,'element_local_torsional_shear_stress':3}
def group(r):
 k=r['kind']
 if k in DISP:return DISP[k]//3
 if k=='displacement_magnitude':return 0
 if k in ACTIONS:return 2+ACTIONS[k]//3
 if k=='support_reaction_component_v2':return 2+(['Fx','Fy','Fz','Mx','My','Mz'].index(r['metadata']['component'])//3)
 if k=='support_reaction_force_magnitude_v2':return 2
 if k=='support_reaction_moment_magnitude_v2':return 3
 return None

def source_checks(case):
 inp,rows,verdicts=case['INPUT'],case['ROWS'],case['VERDICTS'];request=case['REQUEST'];loaded=case['case']=='loaded';load=int(loaded)
 # Scope is independently read from the actual retained input and original request, not verdicts.
 assert inp['nodes']==[[0.,0.,0.],[1.,0.,0.]]
 assert request['model']['nodes'][0]['position']=={'x':0.0,'y':0.0,'z':0.0}
 assert Q(inp['D'])==Q(.1) and Q(inp['t'])==Q(.005)
 assert inp['E']==210e9 and inp['G']==80e9
 loads=request['model']['load_cases'][0]['primitive_loads'];assert len(loads)==3
 assert {(l['direction'],l['magnitude']['value']) for l in loads}=={('UX',load),('UY',load),('RX',load)}
 assert len(rows)==74 and len(verdicts)==74 and len({r['id'] for r in rows})==74
 counts=collections.Counter(r['kind'] for r in rows)
 assert sum(counts[k] for k in DISP)==12 and counts['displacement_magnitude']==2
 assert sum(counts[k] for k in ACTIONS)==30 and sum(counts[k] for k in STRESS)==20
 assert counts['support_reaction_component_v2']==6 and counts['support_reaction_force_magnitude_v2']==counts['support_reaction_moment_magnitude_v2']==counts['pipe_elastic_normal_stress_maximum_v2']==counts['linear_solver_mode_basis']==1
 d,t,E,G=map(Q,[inp['D'],inp['t'],inp['E'],inp['G']]);inner=d-2*t
 area=PI*(d*d-inner*inner)/4;moment=PI*(d**4-inner**4)/64;polar=2*moment;c=d/2
 source={'A':area,'I':moment,'J':polar,'Z':moment/c,'c':I(c),'E':I(E),'G':I(G)}
 represented={k:I(inp[k]) for k in ['A','I','J','Z','c','E','G']}
 represented_geo={**represented,'Z':represented['I']/represented['c']}
 base=request['model']['nodes'][0]['id'];tip=request['model']['nodes'][1]['id']
 def action(j,loc):
  if loc in ['end_i','end_j']:
   end=loc=='end_i';return I(([-1,-1,0,-1,0,-1] if end else [1,1,0,1,0,0])[j]*load)
  frac={'quarter_1':Q(1,4),'midspan':Q(1,2),'quarter_3':Q(3,4)}[loc]
  return I([1,1,0,1,0,1-frac][j]*load)
 def truth(r,p):
  kind=r['kind'];loc=(r.get('metadata') or {}).get('location')
  u=[load/(p['E']*p['A']),load/(3*p['E']*p['I']),I(0),load/(p['G']*p['J']),I(0),load/(2*p['E']*p['I'])]
  if kind in DISP:return I(0) if r['entity_ref']==base else u[DISP[kind]]
  if kind=='displacement_magnitude':return I(0) if r['entity_ref']==base else (u[0]*u[0]+u[1]*u[1]).root()
  if kind in ACTIONS:return action(ACTIONS[kind],loc)
  if kind=='support_reaction_component_v2':return I([-1,-1,0,-1,0,-1][['Fx','Fy','Fz','Mx','My','Mz'].index(r['metadata']['component'])]*load)
  if kind.startswith('support_reaction_'):return I(2*load).root()
  if kind in STRESS:
   j=STRESS[kind];a=action(j,loc);a=-a if loc=='end_i' else a
   return a/p['A'] if j==0 else a*p['c']/p['J'] if j==3 else a/p['Z']
  if kind=='pipe_elastic_normal_stress_maximum_v2':return load/p['A']+load/p['Z']
  raise AssertionError(kind)
 # Final scale exclusion comes from authored constrained components. Native magnitudes are not input-derived.
 scales=[0.]*4
 for row in rows:
  g=group(row);input_derived=row['kind'] in DISP and row['entity_ref']==base
  if g is not None and not input_derived:scales[g]=max(scales[g],abs(norm(row)))
 tr,ro,fo,mo=scales;scales=[max(tr,ro),max(ro,tr),max(fo,mo),max(mo,fo)] # extent=1, original operands
 report=[]
 for j,(row,v) in enumerate(zip(rows,verdicts)):
  if row['kind']=='linear_solver_mode_basis':assert v['class']=='nonquantity' and v['passed'];continue
  g=group(row);input_derived=row['kind'] in DISP and row['entity_ref']==base
  scale=scales[g] if g is not None else scales[2]/inp['A']+(fhex('4006a09e667f3bcd') if row['kind']=='pipe_elastic_normal_stress_maximum_v2' else 1.0)*(scales[3]/inp['Z'])
  n=norm(row);assert bits(n)==v['normalized_bits'];assert bits(scale)==v['scale_bits'],(j,scale,v)
  cl='input' if input_derived else 'absolute' if scale<2.0**-988 or abs(n)<2.0**-34*scale else 'relative'
  assert cl==v['class'],(j,cl,v)
  ts=[truth(row,p) for p in [source,represented,represented_geo]]
  err=errors(n,ts)
  if cl=='input':pred=[decide(err,Q(0)),None,None,None];limits=[Q(0)]
  elif cl=='absolute':
   b=scale/2.**64
   if Q(b)<Q(scale)/2**64:b=nextafter(b,inf)
   assert bits(b)==v['bound_bits'];pred=[decide(err,Q(b)),None,None,None];limits=[Q(b)]
  else:
   ax=abs(Q(n));m=max(ax,Q(scale));allow=m*Q(1,2**64)*(1+Q(1,2**21))+ax*Q(1,2**53)+Q(1,2**1074)
   a0=2.**-64*max(abs(n),scale);a1=a0*(1+2.**-21);u0=2.**-53*abs(n);u1=u0+2.**-1074;b64=a1+u1
   conversion=Q(1000) if row['unit']=='mm' else Q(1,1000000) if row['unit']=='MPa' else Q(1)
   rawerr=errors(row['value'],[t*conversion for t in ts])
   pred=[decide(err,allow),decide(err,Q(b64)),decide(err,ax/Q(10**9)),decide(rawerr,abs(Q(row['value']))/Q(10**9))];limits=[allow,Q(b64),ax/Q(10**9),abs(Q(row['value']))/Q(10**9)]
  assert pred==v['predicates'],(j,row['id'],pred,v)
  passed=all(x is not False for x in pred);assert passed==v['passed']
  source_pred=[decide(errors(n,[ts[0]]),limit) for limit in limits[:3]]
  if cl=='relative':source_pred.append(decide(errors(row['value'],[ts[0]*conversion]),limits[3]))
  if not passed:assert not all(source_pred),(j,'dual-only miss')
  report.append({'source_predicates':source_pred,'source_allowance_ratio_lower':float(errors(n,[ts[0]])[0]/limits[0]) if limits[0] else None,'row':j,'id':row['id'],'predicates':pred,'pass':passed,'source_error_lower':float(errors(n,[ts[0]])[0]),'dual_error_lower':float(err[0]),'dual_error_upper':float(err[1])})
 # Complete G5a shape, comparisons, scalar order and exact prefix.
 data=case['G5A_DATA'];assert data['precision']==128
 expected_presence=[loaded]*4;expected_data=loaded
 cov=data['coverage'];assert cov==[{'body':0,'charge':[loaded]*2,'data':loaded,'estimate':[loaded]*2,'stop':expected_presence}]
 for label,limit,kinds in [('stop',2.**-64,list(range(4))),('estimate',.25,[2,3]),('charge',1.,[2,3])]:
  entries=data[label];assert len(entries)==(len(kinds) if loaded else 0)
  assert {(e[0],e[1]) for e in entries}==({(0,k) for k in kinds} if loaded else set())
  assert all(0<=fhex(e[2])<=limit and (fhex(e[2])!=0 or e[2]=='0000000000000000') for e in entries)
 assert len(data['theta'])==1 and data['theta'][0][0]==0 and 0<=data['theta'][0][1]<=.5
 assert len(data['B'])==int(loaded) and all(e[0]==0 and fhex(e[1])>0 for e in data['B'])
 assert len(data['resolution'])==1 and data['resolution'][0][0]==0
 res=list(map(fhex,data['resolution'][0][1:]));assert all(x>=0 for x in res)
 op=data['operational'][0];assert op['operations']==23 and op['checks']==39 and not op['lost']
 # independent literal successful binary64 schedule from captured K fields
 delta=[inp['nodes'][1][k]-inp['nodes'][0][k] for k in range(3)]
 def nrm(d):return sqrt((d[0]*d[0]+d[1]*d[1])+d[2]*d[2])
 mag=nrm(delta);inverse=1./mag;normalized=[x*inverse for x in delta];length=nrm(delta)
 ka=(inp['E']*inp['A'])/length;kt=(inp['G']*inp['J'])/length
 assert (bits(length),bits(ka),bits(kt))==(bits(op['L']),bits(op['ka']),bits(op['kt']))
 zero_fail=next((j for j,row in enumerate(rows) if group(row) in [2,3] and res[group(row)-2]==0. and bits(norm(row))!='0000000000000000'),None)
 hats=[max(res[0],res[1]),max(res[1],res[0])];upper=[x*fhex('3ff0000000001000') for x in hats]
 lowers=[]
 if zero_fail is None:
  assert upper[0]>=scales[2] and upper[1]>=scales[3]
  for k in range(2):
   endpoint=[]
   for entity in [base,tip]:
    vals=[abs(norm(next(r for r in rows if r['entity_ref']==entity and r['kind']==kind))) for kind,idx in DISP.items() if idx//3==k]
    endpoint.append((vals[0]+vals[1])+vals[2])
   N=endpoint[0]+endpoint[1];lb=0. if N<=2.**-59*scales[k] else [ka,kt][k]*(N-2.**-60*scales[k]);lowers.append(lb);assert upper[k]>=lb
 expected_failure=None if zero_fail is None else f'Zero {{ row: {zero_fail} }}'
 assert data['failure']==expected_failure
 assert case['G5A_WORK']==f'ScalarWork {{ entered: {35 if loaded else 9}, checks: {35 if loaded else 9}, lost: false }}'
 return {'case':case['case'],'rows':report,'pass_count':sum(x['pass'] for x in report),'fail_count':sum(not x['pass'] for x in report),'g5a_failure':expected_failure,'g5a_lower':lowers,'g5a_upper':upper,'g5a_operations':35 if loaded else 9}

if __name__=='__main__':
 source=Path(sys.argv[1]);out=Path(sys.argv[2]);cases=[];request=None;c=None
 for line in source.read_text().splitlines():
  if line.startswith('I45_REQUEST '):request=json.loads(line.split(' ',1)[1])
  elif line.startswith('I45_CASE '):c={'case':line.split()[1],'REQUEST':request};cases.append(c)
  elif line.startswith('I45_') and c is not None:
   key,value=line[4:].split(' ',1)
   if key in ['INPUT','ROWS','VERDICTS','G5A_DATA']:c[key]=json.loads(value)
   elif key=='G5A_WORK':c[key]=value
 assert len(cases)==2
 result=[source_checks(c) for c in cases]
 out.write_text(json.dumps({'method':'fresh Fraction closed cantilever; Machin alternating pi bounds; integer-square-root interval; exact source and two represented bending meanings; all predicates + G5a including coupled resolution','source_log':str(source),'results':result},indent=2)+'\n')
 print(json.dumps([{k:v for k,v in c.items() if k!='rows'} for c in result],indent=2))
