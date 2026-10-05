"""Independent complete named-row check. Reads fresh captured outputs, no author oracle imports.
Exact Machin pi bounds, dyadic integer sqrt bounds, exact source/ledger decode,
case statics and all final raw/SI predicates. No solver is invoked.
"""
from pathlib import Path
from fractions import Fraction as Q
import json,math,struct,sys,hashlib
OUT=Path(sys.argv[2]); OUT.mkdir(parents=True,exist_ok=True)
def F(x):return Q.from_float(float(x))
def bits(x):return struct.pack('>d',float(x)).hex()
def value(h):return struct.unpack('>d',bytes.fromhex(h))[0]
def point(x):return (Q(x),Q(x))
def plus(a,b):return (a[0]+b[0],a[1]+b[1])
def times(a,b):
 v=[x*y for x in a for y in b];return min(v),max(v)
def over(a,b):
 assert 0<b[0]<=b[1];return times(a,(1/b[1],1/b[0]))
def minus(a):return -a[1],-a[0]
def atan_inv(k,terms=200):
 s=Q(0)
 for i in range(terms):s+=Q((-1)**i,(2*i+1)*k**(2*i+1))
 return s,s+Q(1,(2*terms+1)*k**(2*terms+1))
# tan(4 atan(1/5)-atan(1/239))=1; angle in (0,pi/2).
a=Q(1,5);t2=2*a/(1-a*a);t4=2*t2/(1-t2*t2)
assert (t4-Q(1,239))/(1+t4*Q(1,239))==1
p5,p239=atan_inv(5),atan_inv(239)
plo=16*p5[0]-4*p239[1];phi=16*p5[1]-4*p239[0]
D=1<<256
PI=(Q((plo*D).__floor__(),D),Q((phi*D).__ceil__(),D))
assert PI[0]<=plo<phi<=PI[1] and PI[1]-PI[0]<=Q(2,D)
def sqrt_bounds(x):
 assert x>=0
 a=math.isqrt(x.numerator*D*D//x.denominator);lo=Q(a,D)
 hi=lo if lo*lo==x else Q(a+1,D)
 assert lo*lo<=x<=hi*hi
 return lo,hi
def norm(xs):
 assert all(a==b for a,b in xs)
 return sqrt_bounds(sum(a*a for a,b in xs))
def err_bounds(x,ts):
 upper=max(max(abs(x-lo),abs(x-hi)) for lo,hi in ts)
 lower=max(max(lo-x,x-hi,Q(0)) for lo,hi in ts)
 return lower,upper
def decide(error,limit):
 if error[1]<=limit:return True
 if error[0]>limit:return False
 return None
def up_float(x):
 f=float(x)
 if F(f)<x:f=math.nextafter(f,math.inf)
 assert F(f)>=x
 return f
class Reader:
 def __init__(self,b):self.b=bytes(b);self.i=0
 def get(self,n):
  v=self.b[self.i:self.i+n];assert len(v)==n;self.i+=n;return v
 def u8(self):return self.get(1)[0]
 def u32(self):return struct.unpack('<I',self.get(4))[0]
 def f64(self):return struct.unpack('<d',self.get(8))[0]
 def dof(self):return self.u32(),self.u8()
 def done(self):assert self.i==len(self.b)
def decode_source(raw):
 r=Reader(raw);assert r.get(6)==b'K4SRC\x01'
 nodes=[[r.f64() for _ in range(3)] for _ in range(r.u32())]
 members=[]
 for _ in range(r.u32()):members.append(dict(id=r.u32(),ni=r.u32(),nj=r.u32(),props=[r.f64() for _ in range(6)],y=[r.f64() for _ in range(3)]))
 springs=[dict(id=r.u32(),dof=r.dof(),k=r.f64()) for _ in range(r.u32())]
 assert r.u32()==0
 constraints=[(r.dof(),r.f64()) for _ in range(r.u32())]
 loads=[]
 for _ in range(r.u32()):
  d=r.dof();s=r.get(r.u32()).decode();v=r.f64();loads.append((d,s,v))
 stations=[(r.u32(),r.u32(),r.f64()) for _ in range(r.u32())]
 groups=[]
 for _ in range(r.u32()):
  g=dict(id=r.u32(),node=r.u32(),fixed=[r.u8() for _ in range(6)],springs=[r.u32() for _ in range(r.u32())])
  assert r.u32()==0;groups.append(g)
 r.done();return nodes,members,springs,constraints,loads,stations,groups
def decode_ledger(raw):
 r=Reader(raw);assert r.get(6)==b'K4LED\x01';out=[]
 for _ in range(r.u32()):
  dof=r.dof();sign=r.u8();exponent=struct.unpack('<q',r.get(8))[0]
  limbs=[struct.unpack('<Q',r.get(8))[0] for _ in range(r.u32())]
  mag=sum(n<<(64*i) for i,n in enumerate(limbs));assert not mag or mag%2==1
  v=Q((-1 if sign else 1)*mag)*(Q(2)**exponent);out.append((dof,v))
 r.done();return out

def run(record):
 raw=record['request'];model=raw['model'];native=record['native'];fact=record['facts'][0]
 assert not raw['materials'] and not model['combinations'] and not model.get('components',[])
 assert len(model['load_cases'])==len(model['pipe_segments'])==len(model['materials'])==1
 nodes,members,springs,constraints,loads,stations,groups=decode_source(native['source_encoding'])
 assert nodes==[[0.,0.,0.],[1.,2.,2.]] and len(members)==1
 assert members[0]['id']==0 and [members[0]['ni'],members[0]['nj']]==[0,1]
 assert members[0]['y']==[1.,0.,0.] and constraints==[((0,0),0.),((0,1),0.),((0,2),0.)]
 assert stations==[(0,0,.25),(1,0,.5),(2,0,.75)]
 M=[F(v['magnitude']['value']) for v in model['load_cases'][0]['primitive_loads']]
 assert M==[M[0],2*M[0],2*M[0]]
 assert loads==[((1,i+3),'load:'+str(i),float(v)) for i,v in enumerate(M)]
 assert decode_ledger(native['ledger_encoding'])==[((1,i+3),v) for i,v in enumerate(M)]
 assert springs==[dict(id=i,dof=(0,i+3),k=k) for i,k in enumerate([144.,1e6,1e6])]
 assert groups==[dict(id=0,node=0,fixed=[1,1,1,0,0,0],springs=[])]+[dict(id=i+1,node=0,fixed=[0]*6,springs=[i]) for i in range(3)]
 source=record['source'];assert source['nodes']==nodes and len(source['members'])==1
 for i,s in enumerate(source['springs']):assert (s['id'],s['node'],s['axis'],s['k'])==(i,0,i+3,springs[i]['k'])
 for a,b in zip(source['supports'],groups):assert a['id']==b['id'] and a['node']==b['node'] and a['rigid']==[bool(x) for x in b['fixed']] and a['springs']==b['springs']
 E,G,A,Iy,Iz,J=map(F,members[0]['props']);assert E==F(200e9) and G==F(80e9) and Iy==Iz
 assert min(E,G,A,Iy,Iz,J)>0
 assert [F(fact[k]) for k in ['A','I','I','J']]==[A,Iy,Iz,J]
 D0,t,c=F(fact['D']),F(fact['t']),F(fact['c']);assert c==D0/2 and 0<t<c
 section=model['pipe_segments'][0]['section'];assert D0==F(section['outside_diameter']['value']) and t==F(section['wall_thickness']['value'])
 mat=model['materials'][0];assert F(mat['elastic_modulus']['value'])==E and F(mat['shear_modulus']['value'])==G
 assert model['pipe_segments'][0]['y_reference']==dict(x=1,y=0,z=0)
 # Exact orthonormal axes: x=(1,2,2)/3, y=(4,-1,-1)/(3sqrt2), z=(0,1,-1)/sqrt2.
 ax=[1,2,2];ay=[4,-1,-1];az=[0,1,-1]
 assert sum(x*x for x in ax)==9 and sum(y*y for y in ay)==18 and sum(z*z for z in az)==2
 assert sum(x*y for x,y in zip(ax,ay))==sum(x*z for x,z in zip(ax,az))==sum(y*z for y,z in zip(ay,az))==0
 assert [ax[1]*ay[2]-ax[2]*ay[1],ax[2]*ay[0]-ax[0]*ay[2],ax[0]*ay[1]-ax[1]*ay[0]]==[9*z for z in az]
 T=3*M[0];theta=[m/F(s['k']) for m,s in zip(M,springs)]
 u=[2*theta[1]-2*theta[2],theta[2]-2*theta[0],2*theta[0]-theta[1]]
 assert u[0]==0 and u[1]==-u[2]
 ri=c-t;Jgeo=times(PI,point((c**4-ri**4)/2));Igeo=times(Jgeo,point(Q(1,2)))
 Ageo=times(PI,point(c*c-ri*ri));Zgeo=over(Igeo,point(c))
 # Four readout descriptions; both represented-Z choices remain explicit, even where numerator=0.
 cases=[('K/Zhat',point(J),point(F(fact['Z']))),('K/I-over-c',point(J),point(Iy/c)),('source',Jgeo,Zgeo)]
 def truth(row):
  kind=row['kind'];meta=row.get('metadata') or {};entity=row['entity_ref'];comp=meta.get('component','');loc=meta.get('location','')
  ans=[]
  for label,j,z in cases:
   v=point(0)
   if kind.startswith('global_nodal_displacement_'):
    assert entity in ['N0','N1'];v=point(u['xyz'.index(kind[-1])] if entity=='N1' else 0)
   elif kind.startswith('global_nodal_rotation_'):
    a='xyz'.index(kind[-1]);v=point(theta[a])
    if entity=='N1':v=plus(v,over(point(3*M[a]),times(point(G),j)))
   elif kind=='displacement_magnitude':v=norm([point(x) for x in u]) if entity=='N1' else point(0)
   elif kind.startswith('support_reaction_'):
    sid=next(i for i,s in enumerate(model['supports']) if s['id']==entity)
    if sid:
     if kind=='support_reaction_component_v2' and comp==['Mx','My','Mz'][sid-1]:v=point(-M[sid-1])
     if kind=='support_reaction_moment_magnitude_v2':v=point(abs(M[sid-1]))
   elif kind=='element_local_torsional_moment':v=point(-T if loc=='end_i' else T)
   elif kind=='element_local_torsional_shear_stress':v=over(point(T*c),j)
   elif kind in ['element_local_axial_force','element_local_shear_force_y','element_local_shear_force_z',
       'element_local_bending_moment_y','element_local_bending_moment_z','element_local_axial_normal_stress',
       'element_local_bending_normal_stress_y','element_local_bending_normal_stress_z','pipe_elastic_normal_stress_maximum_v2']:v=point(0)
   else:raise AssertionError(kind)
   ans.append(v)
  return ans
 def family(row):
  k=row['kind'];unit=row['unit']
  if k in ['linear_solver_mode_basis','sparse_live_path_dense_parity_relative_delta']:return None
  if k.startswith('global_nodal_displacement_') or k=='displacement_magnitude':return 0
  if k.startswith('global_nodal_rotation_'):return 1
  if unit=='N':return 2
  if unit=='N*m':return 3
  if unit in ['MPa','Pa']:return 4
  raise AssertionError((k,unit))
 def normalized(row):return row['value']/1000. if row['unit']=='mm' else row['value']*1e6 if row['unit']=='MPa' else row['value']
 rows=record['envelope']['results'];verdicts=record['verdicts'];assert len(rows)==len(verdicts)
 expected=[]
 for node in ['N0','N1']:
  expected.append(('displacement_magnitude',node,None,None))
  for c0 in 'xyz':
   expected.append(('global_nodal_displacement_'+c0,node,'nodal_displacement_'+c0,'node'))
   expected.append(('global_nodal_rotation_'+c0,node,'nodal_rotation_'+c0,'node'))
 for support in model['supports']:
  for c0 in ['Fx','Fy','Fz','Mx','My','Mz']:expected.append(('support_reaction_component_v2',support['id'],c0,'node'))
  for v0 in ['force','moment']:expected.append(('support_reaction_'+v0+'_magnitude_v2',support['id'],v0+'_magnitude','node'))
 for loc in ['end_i','end_j','quarter_1','midspan','quarter_3']:
  for kind0 in ['axial_force','shear_force_y','shear_force_z','torsional_moment','bending_moment_y','bending_moment_z']:
   expected.append(('element_local_'+kind0,'M1',kind0,loc))
  for kind0 in ['axial_normal_stress','bending_normal_stress_y','bending_normal_stress_z','torsional_shear_stress']:
   expected.append(('element_local_'+kind0,'M1',kind0,loc))
 expected.append(('pipe_elastic_normal_stress_maximum_v2','M1','maximum_absolute_normal_stress','governing_station'))
 actual=[(r['kind'],r['entity_ref'],(r.get('metadata') or {}).get('component'),(r.get('metadata') or {}).get('location')) for r in rows if family(r) is not None]
 assert len(actual)==len(expected)==97 and set(actual)==set(expected) and len(set(actual))==97
 ids=[r['id'] for r in rows];assert len(set(ids))==len(ids)
 scales=[0.]*4
 for r in rows:
  f=family(r);input_derived=r['entity_ref']=='N0' and r['kind'].startswith('global_nodal_displacement_')
  if f is not None and f<4 and not input_derived:scales[f]=max(scales[f],abs(normalized(r)))
 tr,ro,fo,mo=scales;scales=[max(tr,3.*ro),max(ro,tr/3.),max(fo,mo/3.),max(mo,3.*fo)]
 assert native['precision']==128
 # Check ancillary custody against the actual producing capture; no parity criterion is invented.
 obs=record['observations'];assert obs['case']=='case' and obs['mode']==record['mode']
 mode_rows=[r for r in rows if r['kind']=='linear_solver_mode_basis'];assert len(mode_rows)==1
 expected_mode=1. if record['mode']=='sparse_interactive' else 2.
 assert bits(mode_rows[0]['value'])==obs['mode_bits']==bits(expected_mode)
 assert mode_rows[0]['metadata']['basis']==obs['mode_basis']
 parity=[r for r in rows if r['kind']=='sparse_live_path_dense_parity_relative_delta']
 assert len(parity)==int(obs['parity_produced']) and obs['parity_produced']==(obs['parity'] is not None)
 assert record['mode']=='dense_scrutiny' or not parity
 if parity:
  assert math.isfinite(parity[0]['value']) and parity[0]['value']>=0
  assert bits(parity[0]['value'])==obs['parity']['bits'] and parity[0]['metadata']['basis']==obs['parity']['basis']
 # Independently recompute unchanged G5a force/moment zero, sanity and motion lower guards.
 assert len(native['resolution'])==1 and native['resolution'][0][0]==0
 resolution=[value(x) for x in native['resolution'][0][1:]]
 for r in rows:
  k=family(r)
  if k in [2,3] and resolution[k-2]==0.:assert bits(normalized(r))=='0000000000000000'
 hats=[max(resolution[0],resolution[1]/3.),max(resolution[1],3.*resolution[0])]
 upper=[x*value('3ff0000000001000') for x in hats]
 assert all(upper[i]>=scales[i+2] for i in range(2))
 op_coefficients=[(float(E)*fact['A'])/3.,(float(G)*fact['J'])/3.]
 lower=[]
 for k,prefix in enumerate(['global_nodal_displacement_','global_nodal_rotation_']):
  ends=[]
  for node in ['N0','N1']:
   xyz=[abs(normalized(next(r for r in rows if r['entity_ref']==node and r['kind']==prefix+c0))) for c0 in 'xyz']
   ends.append((xyz[0]+xyz[1])+xyz[2])
  total=ends[0]+ends[1]
  low=0. if total<=2.**-59*scales[k] else op_coefficients[k]*(total-2.**-60*scales[k])
  lower.append(low);assert upper[k]>=low
 # Independently check support observable norms, final maximum midpoint and headline aliases.
 evidence=record['envelope']['contract_evidence'];assert evidence['combination_gates']==[]
 assert len(evidence['preview_cases'])==1
 ce=evidence['preview_cases'][0];assert ce['load_case_id']=='case'
 assert sorted(ce['support_attribution']['attributed_support_ids'])==sorted(s['id'] for s in model['supports'])
 assert ce['support_attribution']['withheld']==[]
 for support in model['supports']:
  sr=[r for r in rows if r['entity_ref']==support['id'] and r['kind'].startswith('support_reaction_')];assert len(sr)==8
  for names,kind in [('Fx Fy Fz','support_reaction_force_magnitude_v2'),('Mx My Mz','support_reaction_moment_magnitude_v2')]:
   cs=[next(r['value'] for r in sr if r['kind']=='support_reaction_component_v2' and r['metadata']['component']==c0) for c0 in names.split()]
   actual=next(r['value'] for r in sr if r['kind']==kind);interval=norm([point(F(x)) for x in cs])
   assert err_bounds(F(actual),[interval])[1]<=F(64.*2.**-52*max(abs(actual),2.**-1022))
 by_id={r['id']:r for r in rows}
 for ex in ce['pipe_stress_extrema']:
  r=by_id[ex['result_id']];lo=ex['value_lower_pa'];hi=ex['value_upper_pa']
  assert r['entity_ref']==ex['pipe_id'] and 0<=lo<=r['value']<=hi and bits(r['value'])==bits(lo+.5*(hi-lo))
 for field,kind in [('max_displacement','displacement_magnitude'),('max_open_formula_stress','pipe_elastic_normal_stress_maximum_v2')]:
  best=sorted([r for r in rows if r['kind']==kind],key=lambda r:(-r['value'],r['entity_ref']))[0]
  h=record['envelope']['summary'][field];assert (h['result_ref'],h['location_ref'],h['unit'],bits(h['value']))==(best['id'],best['entity_ref'],best['unit'],bits(best['value']))
 output=[];passed_predicates=0;false_pass=[];undecided=[];truth_misses=0;conservative=0;candidate_pass_rows=0
 for i,(r,v) in enumerate(zip(rows,verdicts)):
  assert r['basis_ref']=={'ref_id':'case','ref_type':'load_case'} and v['row']==i
  f=family(r);y=F(r['value']);n=normalized(r);assert bits(n)==v['normalized_bits']
  if f is None:
   assert v['class']=='None' and v['predicates']==[None]*4 and v['passed'];continue
  ts=truth(r); input_derived=r['entity_ref']=='N0' and r['kind'].startswith('global_nodal_displacement_')
  if f<4:scale=scales[f]
  else:
   k=value('4006a09e667f3bcd') if r['kind']=='pipe_elastic_normal_stress_maximum_v2' else 1.
   scale=scales[2]/fact['A']+k*(scales[3]/fact['Z'])
  assert bits(scale)==v['scale_bits'],(i,scale,value(v['scale_bits']))
  hn=err_bounds(F(n),ts)
  factor=Q(1,1000) if r['unit']=='mm' else Q(1000000) if r['unit']=='MPa' else Q(1)
  hu=err_bounds(y,[over(t0,point(factor)) for t0 in ts])
  relative=not input_derived and scale>=value('0230000000000000') and not abs(n)<value('3dd0000000000000')*scale
  if input_derived:
   assert v['class']=='Some(InputDerived)';limits=[Q(0)];tests=[decide(hn,Q(0))]
  elif relative:
   assert v['class']=='Some(RelativeVerified)'
   ex=Q(1,1<<64)*max(abs(F(n)),F(scale))*(1+Q(1,1<<21))+Q(1,1<<53)*abs(F(n))+Q(1,1<<1074)
   af0=2.**-64*max(abs(n),scale);af1=af0*value('3ff0000080000000');uf0=2.**-53*abs(n);uf1=uf0+value('0000000000000001');af=af1+uf1
   limits=[ex,F(af),abs(F(n))/10**9,abs(y)/10**9]
   tests=[decide(hn,ex),decide(hn,F(af)),decide(hn,limits[2]),decide(hu,limits[3])]
  else:
   b=up_float(F(scale)/2**64);assert v['class']=='Some(AbsoluteVerified { bound_bits: '+str(int(bits(b),16))+' })'
   limits=[F(b)];tests=[decide(hn,F(b))]
  cp=v['predicates'][:len(tests)];assert all(t is None for t in v['predicates'][len(tests):])
  for j,(actual,proved) in enumerate(zip(cp,tests)):
   if actual:passed_predicates+=1
   if actual and proved is not True:false_pass.append([i,j,proved])
   if proved is None:undecided.append([i,j])
  truth_pass=all(x is True for x in tests)
  if not truth_pass:truth_misses+=1
  if truth_pass and not v['passed']:conservative+=1
  if v['passed']:candidate_pass_rows+=1
  output.append(dict(row=i,id=r['id'],kind=r['kind'],entity=r['entity_ref'],unit=r['unit'],class_name=v['class'],scale_bits=bits(scale),
    truth_intervals=[[str(a),str(b)] for a,b in ts],si_error=[str(x) for x in hn],raw_error=[str(x) for x in hu],limits=[str(x) for x in limits],
    independently_decided=tests,candidate_predicates=cp,candidate_pass=v['passed'],truth_pass=truth_pass))
 assert len(output)==97 and len(native['rows'])==58
 assert not false_pass and not undecided
 assert record['error']==record['numeric_failure']==record['g5a']==record['observable']=='None'
 assert record['invocation_calls']==record['observation_calls']==native['calls']==1
 assert record['hooks']==[1,1,1]
 assert not record['numeric_pass'] and not record['full_case']
 summary=dict(mode=record['mode'],mechanical_rows=len(output),candidate_PASS_predicates=passed_predicates,candidate_pass_rows=candidate_pass_rows,
  actual_truth_miss_rows=truth_misses,truth_pass_certificate_refusal_rows=conservative,false_PASS=false_pass,unresolved=undecided,
  full_source_ledger_decoded=True,ancillary_custody_checked=True,g5a_checked=True,g5a_upper=upper,g5a_lower=lower,observables_checked=True,
  scales=[bits(s) for s in scales],pi=[str(s) for s in PI],source_J=[str(s) for s in Jgeo],
  represented_Z_readouts=[str(F(fact['Z'])),str(Iy/c)],scope='Both complete fixed named captures; not arbitrary future scales or projection')
 return summary,output

log=Path(sys.argv[1]);records=[json.loads(s[11:]) for s in log.read_text().splitlines() if s.startswith('I50_RECORD ')]
assert len(records)==2
summaries=[]
for record in records:
 s,rows=run(record);summaries.append(s)
 (OUT/(record['mode']+'_rows.json')).write_text(json.dumps(rows,indent=2)+'\n')
answer=dict(input=dict(location=str(log),bytes=log.stat().st_size,sha256=hashlib.sha256(log.read_bytes()).hexdigest()),
 method='Independent Machin alternating-series pi, rational statics/source/ledger decode, directed integer sqrt, actual f64 scales, exact raw/SI predicate intervals',summaries=summaries)
(OUT/'SUMMARY.json').write_text(json.dumps(answer,indent=2)+'\n')
print(json.dumps([{k:v for k,v in s.items() if k not in ['pi','source_J','represented_Z_readouts']} for s in summaries],indent=2))
