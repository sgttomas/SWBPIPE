"""RV61 independent metadata-driven exact arithmetic; no I46 code imported."""
from fractions import Fraction as Q
from pathlib import Path
import json, math, struct
HERE=Path(__file__).resolve().parent
ORIGINAL=HERE.parents[2]/'I46/product_refusal_01/_run_records/original'
def q(x): return Q(*x.as_integer_ratio()) if isinstance(x,float) else Q(x)
def decode(h): return q(struct.unpack('>d', bytes.fromhex(h))[0])
def bits(x):return struct.pack('>d',float(x)).hex()
def p2(e):return Q(2**e) if e>=0 else Q(1,2**-e)
def rn(x):
 x=q(x)
 if x==0:return x
 s=-1 if x<0 else 1;x=abs(x);n,d=x.numerator,x.denominator
 e=n.bit_length()-d.bit_length()
 if x<p2(e):e-=1
 quantum=p2(max(-1074,e-52));z=x/quantum;k,rem=divmod(z.numerator,z.denominator)
 k+=int(2*rem>z.denominator or (2*rem==z.denominator and k%2==1))
 return s*k*quantum
class I:
 def __init__(self,lo,hi=None):self.lo=q(lo);self.hi=q(lo if hi is None else hi);assert self.lo<=self.hi
 def __add__(self,b):
  b=iv(b);return I(self.lo+b.lo,self.hi+b.hi)
 __radd__=__add__
 def __mul__(self,b):
  b=iv(b);v=[a*c for a in [self.lo,self.hi] for c in [b.lo,b.hi]];return I(min(v),max(v))
 __rmul__=__mul__
 def __truediv__(self,b):
  b=iv(b);assert b.lo>0;return self*I(1/b.hi,1/b.lo)
 def sqrt(self):
  assert self.lo>=0
  def rootfloor(x):return Q(math.isqrt((x.numerator<<800)//x.denominator),1<<400)
  a,b=rootfloor(self.lo),rootfloor(self.hi)
  if b*b<self.hi:b+=p2(-400)
  assert a*a<=self.lo<=self.hi<=b*b
  return I(a,b)
def iv(x):return x if isinstance(x,I) else I(x)
def atan_bracket(n,N=384):
 total=Q();term=Q(1,n)
 for k in range(N):total+=term/(2*k+1);term/=-n*n
 nxt=term/(2*N+1);return I(min(total,total+nxt),max(total,total+nxt))
# atan(1/2)+atan(1/3)=pi/4: tangent addition gives 1 and sum in (0,pi/2).
PI=4*(atan_bracket(2)+atan_bracket(3));assert PI.hi-PI.lo<p2(-760)
EPS=p2(-64)+p2(-85);U=p2(-53);H=p2(-1074)
def allowance(x,s):return EPS*max(abs(x),s)+U*abs(x)+H
def a64(x,s):return rn(rn(rn(p2(-64)*max(abs(x),s))*(1+p2(-21)))+rn(rn(U*abs(x))+H))
def normalize(x,unit):return rn(x/1000) if unit=='mm' else rn(x*1000000) if unit=='MPa' else x
def raw_iv(x,unit):return x*1000 if unit=='mm' else x/1000000 if unit=='MPa' else x
def error(x,t):
 upper=max(abs(x-t.lo),abs(x-t.hi));lower=t.lo-x if x<t.lo else x-t.hi if x>t.hi else Q()
 return lower,upper
def outcome(x,t,bound):
 lo,hi=error(x,t)
 assert hi<=bound or lo>bound, 'unresolved enclosure'
 return hi<=bound
records=json.loads((ORIGINAL/'captured_records.json').read_text());cases={};current=None;request=None
for obj in records:
 if obj['tag']=='I45_REQUEST':request=obj['data']
 elif obj['tag']=='I45_CASE':current=obj['data'];cases[current]={'REQUEST':request}
 else:cases[current][obj['tag'][4:]]=obj['data']
# Decode all captured canonical source bytes using frozen source.rs:775-888.
# This binds the scalar properties, loaded DOFs, station ordinals and native node identities.
source_decodes={}
for cname,case in cases.items():
 data=bytes(case['NATIVE_IDENTITY']['source']);assert data[:6]==b'K4SRC\x01';offset=6
 def take(fmt):
  global offset
  value=struct.unpack_from('<'+fmt,data,offset)[0];offset+=struct.calcsize('<'+fmt);return value
 def dof():return [take('I'),take('B')]
 nodes=[[take('d') for _ in range(3)] for _ in range(take('I'))]
 assert nodes==case['INPUT']['nodes'];assert take('I')==1
 member=[take('I') for _ in range(3)];props=[take('d') for _ in range(6)];axis=[take('d') for _ in range(3)]
 assert member==[0,0,1] and axis==[0.,1.,0.]
 assert [bits(x) for x in props]==[bits(case['INPUT'][k]) for k in ['E','G','A','I','I','J']]
 assert take('I')==0 and take('I')==0
 constraints=[(dof(),take('d')) for _ in range(take('I'))]
 assert constraints==[([0,i],0.) for i in range(6)]
 loads=[]
 for _ in range(take('I')):
  target=dof();length=take('I');name=data[offset:offset+length].decode();offset+=length;loads.append((target,name,take('d')))
 assert loads==[([1,comp],name,float(cname=='loaded')) for comp,name in [(0,'i45:UX'),(1,'i45:UY'),(3,'i45:RX')]]
 stations=[(take('I'),take('I'),take('d')) for _ in range(take('I'))]
 assert stations==[(0,0,.25),(1,0,.5),(2,0,.75)]
 assert take('I')==1 and take('I')==0 and take('I')==0
 assert [take('B') for _ in range(6)]==[1]*6
 assert take('I')==0 and take('I')==0 and offset==len(data)
 source_decodes[cname]={'member':member,'E_G_A_Iy_Iz_J_bits':[bits(x) for x in props],'constraints':constraints,'loads':loads,'stations':stations,'bytes_consumed':offset}
assert rn(1+p2(-53))==1 and rn(1+3*p2(-53))==1+p2(-51)
assert rn(p2(-1075))==0 and rn(3*p2(-1075))==2*p2(-1074)
assert rn(-1-3*p2(-53))==-1-p2(-51)

# Determine class/association from preserved row metadata rather than I46 row-number tables.
def kclass(row):
 kind=row['kind'];unit=row['unit']
 if kind=='linear_solver_mode_basis':return 'nonquantity'
 if kind=='displacement_magnitude':return 'tr'
 if kind.startswith('global_nodal_'):
  if row['entity_ref'].endswith('ANCHOR'):return 'input'
  return 'tr' if 'displacement' in kind else 'ro'
 if unit=='N':return 'fo'
 if unit=='N*m':return 'mo'
 return 'maximum' if kind=='pipe_elastic_normal_stress_maximum_v2' else 'stress'
components={'axial_force':'Ux','shear_force_y':'Uy','shear_force_z':'Uz','torsional_moment':'Rx','bending_moment_y':'Ry','bending_moment_z':'Rz'}
supportcomp={'Fx':'Ux','Fy':'Uy','Fz':'Uz','Mx':'Rx','My':'Ry','Mz':'Rz'}
stationno={'quarter_1':0,'midspan':1,'quarter_3':2}
xpos={'end_i':Q(0),'quarter_1':Q(1,4),'midspan':Q(1,2),'quarter_3':Q(3,4),'end_j':Q(1)}
def native_quantity(row):
 kind=row['kind'];m=row.get('metadata',{});comp=m.get('component');loc=m.get('location');node=0 if row['entity_ref'].endswith('ANCHOR') else 1
 if kind.startswith('global_nodal_'):return f'Displacement(Dof {{ node: {node}, component: '+row['id'].rsplit(':',1)[1].capitalize()+' })'
 if kind=='displacement_magnitude':return f'DisplacementMagnitude({node})'
 if kind=='support_reaction_component_v2':return 'Reaction(Dof { node: 0, component: '+supportcomp[comp]+' })'
 if kind=='support_reaction_force_magnitude_v2':return 'SupportForceMagnitude(0)'
 if kind=='support_reaction_moment_magnitude_v2':return 'SupportMomentMagnitude(0)'
 if comp in components:
  if loc in stationno:return f'StationAction {{ station: {stationno[loc]}, component: {components[comp]} }}'
  return f'EndAction {{ member: 0, end: {"I" if loc=="end_i" else "J"}, component: {components[comp]} }}'
 return None
def truth(row,p,source,loaded):
 if not loaded:return I(0)
 D,t,E,G=[q(p[k]) for k in ['D','t','E','G']];c=D/2;ri=c-t
 if source:
  A=PI*t*(D-t);J=A*(c*c+ri*ri)/2;Inertia=J/2;Z=Inertia/c
 else:
  A,Inertia,J=[I(p[k]) for k in ['A','I','J']];z=q(p['I'])/c;Z=I(min(z,q(p['Z'])),max(z,q(p['Z'])));c=q(p['c'])
 kind=row['kind'];m=row.get('metadata',{});comp=m.get('component');loc=m.get('location');clas=kclass(row)
 ux=I(1)/(E*A);uy=I(1)/(3*E*Inertia);rx=I(1)/(G*J);rz=I(1)/(2*E*Inertia)
 if clas=='input':return I(0)
 if kind=='displacement_magnitude':return I(0) if row['entity_ref'].endswith('ANCHOR') else (ux*ux+uy*uy).sqrt()
 if kind.startswith('global_nodal_'):return {'ux':ux,'uy':uy,'rx':rx,'rz':rz}.get(row['id'].rsplit(':',1)[1],I(0))
 if kind=='support_reaction_component_v2':return I(-1 if comp in ['Fx','Fy','Mx','Mz'] else 0)
 if 'support_reaction_' in kind:return I(2).sqrt()
 if clas in ['fo','mo']:
  if comp in ['shear_force_z','bending_moment_y']:return I(0)
  if loc=='end_i':return I(-1)
  if comp=='bending_moment_z':return I(1-xpos[loc])
  return I(1)
 if clas=='maximum':return I(1)/A+I(1)/Z
 if comp=='axial_normal_stress':return I(1)/A
 if comp=='bending_normal_stress_y':return I(0)
 if comp=='bending_normal_stress_z':return I(1-xpos[loc])/Z
 if comp=='torsional_shear_stress':return I(c)/J
 raise AssertionError(kind)
def scales(rows,values,p):
 maxima={k:Q() for k in ['tr','ro','fo','mo']}
 for i,x in values.items():
  key=kclass(rows[i])
  if key in maxima:maxima[key]=max(maxima[key],abs(normalize(x,rows[i]['unit'])))
 tr,ro,fo,mo=[maxima[k] for k in ['tr','ro','fo','mo']]
 # Actual L is exactly 1; use original four operands for both coupling directions.
 s={'tr':max(tr,rn(ro)),'ro':max(ro,rn(tr)),'fo':max(fo,rn(mo)),'mo':max(mo,rn(fo)),'input':Q()}
 for key,k in [('stress',Q(1)),('maximum',decode('4006a09e667f3bcd'))]:s[key]=rn(rn(s['fo']/q(p['A']))+rn(k*rn(s['mo']/q(p['Z']))))
 return s
def check(row,y,s,t):
 n=normalize(y,row['unit']);key=kclass(row)
 if key=='input':return {'class':'input','tests':[outcome(n,t,Q())]}
 scale=s[key]
 if scale==0 or abs(n)<rn(p2(-34)*scale):
  exact=scale*p2(-64);b=rn(exact)
  if b<exact:b=q(math.nextafter(float(b),math.inf))
  tests=[outcome(n,t,b)];label='absolute'
 else:
  tests=[outcome(n,t,allowance(n,scale)),outcome(n,t,a64(n,scale)),outcome(n,t,abs(n)/10**9),outcome(y,raw_iv(t,row['unit']),abs(y)/10**9)];label='relative'
 return {'class':label,'tests':tests}
results={};association=[]
for name,case in cases.items():
 rows=case['ROWS'];p=case['INPUT'];loaded=name=='loaded';assert case['G5A_DATA']['precision']==128
 assert p['nodes']==[[0.,0.,0.],[1.,0.,0.]]
 req=case['REQUEST'];model=req['model'];assert len(model['nodes'])==2 and len(model['pipe_segments'])==1 and len(model['supports'])==1
 assert set(model['supports'][0]['restraints'])=={'UX','UY','UZ','RX','RY','RZ'}
 loads=model['load_cases'][0]['primitive_loads'];assert [(a['direction'],a['magnitude']['value'],a['magnitude']['unit']) for a in loads]==[('UX',float(loaded),'N'),('UY',float(loaded),'N'),('RX',float(loaded),'N*m')]
 assert req['materials'][0]['elastic_modulus']['value']==p['E'] and req['materials'][0]['shear_modulus']['value']==p['G']
 assert model['pipe_segments'][0]['section']['outside_diameter']['value']==p['D'] and model['pipe_segments'][0]['section']['wall_thickness']['value']==p['t']
 vals={i:q(r['value']) for i,r in enumerate(rows) if i};ss=scales(rows,vals,p)
 natives={r['quantity']:r for r in case['NATIVE_ROWS']};mapped={i:natives[native_quantity(row)] for i,row in enumerate(rows) if native_quantity(row)}
 assert len(mapped)==52 and len({x['ordinal'] for x in mapped.values()})==52
 projected={i:rn(decode(v['value_bits'])*1000) if rows[i]['unit']=='mm' else decode(v['value_bits']) for i,v in mapped.items()};ps=scales(rows,projected,p)
 nrows=[dict(row,unit='m') if row['unit']=='mm' else row for row in rows]
 ns=scales(nrows,{i:decode(v['value_bits']) for i,v in mapped.items()},p)
 assert ns==ps
 rr={};fails={}
 for mode,vs,sc in [('ordinary',vals,ss),('native', {i:decode(v['value_bits']) for i,v in mapped.items()},ps),('projected',projected,ps)]:
  rr[mode]={};fails[mode]={}
  for branch in ['source','represented']:
   checks={}
   for i,y in vs.items():
    row=dict(rows[i])
    if mode=='native' and row['unit']=='mm':row['unit']='m'
    checks[i]=check(row,y,sc,truth(row,p,branch=='source',loaded))
   rr[mode][branch]=checks;fails[mode][branch]=[i for i,c in checks.items() if not all(c['tests'])]
 for i in vals:
  # Input-derived scale is immaterial to point identity; recorded product scale still follows kind.
  key=kclass(rows[i]); scale=ss['ro' if 'rotation' in rows[i]['kind'] else 'tr'] if key=='input' else ss[key]
  assert bits(scale)==case['VERDICTS'][i]['scale_bits']
  assert case['VERDICTS'][i]['passed']==all(all(rr['ordinary'][b][i]['tests']) for b in ['source','represented'])
 for i,v in mapped.items():
  if 'AbsoluteVerified' in v['class']:
   b=int(v['class'].split(': ')[1].split(' ')[0]);assert bits(ps[kclass(rows[i])]*p2(-64))==f'{b:016x}'
 if loaded:association=[{'row':i,'id':rows[i]['id'],'unit':rows[i]['unit'],'native_quantity':v['quantity'],'native_ordinal':v['ordinal'],'native_bits':v['value_bits']} for i,v in mapped.items()]
 zero_bad=[i for i,r in enumerate(rows) if kclass(r) in ['fo','mo'] and case['G5A_DATA']['resolution'][0][1 if kclass(r)=='fo' else 2]=='0000000000000000' and bits(r['value'])!='0000000000000000']
 # G5a reader arithmetic, with actual resolutions treated as captured attestation.
 g=case['G5A_DATA']; Efo,Emo=[decode(v) for v in g['resolution'][0][1:]]
 ef,em=max(Efo,rn(Emo)),max(Emo,rn(Efo));guard=decode('3ff0000000001000')
 ceilings=[rn(ef*guard),rn(em*guard)]
 totals=[]
 for prefix in ['global_nodal_displacement_','global_nodal_rotation_']:
  pernode=[]
  for entity in ['node:N-DEC092-ANCHOR','node:N-DEC092-TIP']:
   xyz=[abs(normalize(q(next(r for r in rows if r['entity_ref']==entity and r['kind']==prefix+axis)['value']),next(r for r in rows if r['entity_ref']==entity and r['kind']==prefix+axis)['unit'])) for axis in ['x','y','z']]
   pernode.append(rn(rn(xyz[0]+xyz[1])+xyz[2]))
  totals.append(rn(pernode[0]+pernode[1]))
 stiffness=[rn(q(p['E'])*q(p['A'])),rn(q(p['G'])*q(p['J']))]
 assert [bits(x) for x in stiffness]==[bits(g['operational'][0][k]) for k in ['ka','kt']]
 lower=[Q() if total<=rn(p2(-59)*scale) else rn(stiff*rn(total-rn(p2(-60)*scale))) for total,scale,stiff in zip(totals,[ss['tr'],ss['ro']],stiffness)]
 assert all(a>=b for a,b in zip(ceilings,[ss['fo'],ss['mo']]))
 assert all(a>=b for a,b in zip(ceilings,lower))
 assert all(decode(v[-1])<=Q(1,4) for v in g['estimate']) and all(decode(v[-1])<=1 for v in g['charge'])
 assert all(q(v[-1])<=Q(1,2) for v in g['theta']) and all(decode(v[-1])>0 for v in g['B'])
 g5={'lower_bound_bits':[bits(x) for x in lower],'resolution_guard_bits':[bits(x) for x in ceilings],'sanity':True,'lower_bound':True,'summary_scalar_bounds':True,'scope':'Captured attestation only; unpublished origins not independently replayed.'}
 results[name]={'g5a_arithmetic':g5,'failures':fails,'ordinary_scales':{k:bits(v) for k,v in ss.items()},'projected_scales':{k:bits(v) for k,v in ps.items()},'zero_sign_failures':zero_bad,'checks':rr}
# Scalar conclusion is independent of an enclosure merely failing to contain the point.
l=cases['loaded'];p=l['INPUT'];n=decode(next(v for v in l['NATIVE_ROWS'] if v['quantity']=='Displacement(Dof { node: 1, component: Rx })')['value_bits'])
D,t,G,JK=[q(p[k]) for k in ['D','t','G','J']];c=D/2;ri=c-t
Js=PI*t*(D-t)*(c*c+ri*ri)/2
assert t*(D-t)*(c*c+ri*ri)/2==(D**4-(D-2*t)**4)/32
qK=1/(G*JK);qS=I(1)/(G*Js);A=allowance(n,n);Af=a64(n,n);err=error(n,qS);b=int(bits(n),16)
midlo=(decode(f'{b-1:016x}')+n)/2;midhi=(decode(f'{b+1:016x}')+n)/2
assert midlo<qK<midhi and rn(qK)==n and bits(l['ROWS'][12]['value'])==bits(n)
assert err[0]>max(A,Af) and abs(n-qK)<min(A,Af)
# Public-relative K test forces y>0 in the narrow interval [ymin,ymax].
ymin=qK/Q(1000000001,1000000000);ymax=qK/Q(999999999,1000000000)
other=max(abs(decode(v['native_bits'])) for v in association if v['row']!=12 and kclass(l['ROWS'][v['row']]) in ['tr','ro'])
assert ymin>other and ymin>p2(-34)*n
gap=qS.lo-qK;twobound=2*allowance(ymax,n);assert ymax>n and gap>twobound
scalar={'D_bits':bits(D),'t_bits':bits(t),'G_bits':bits(G),'JK_bits':bits(JK),'n_bits':bits(n),'pi_lower':str(PI.lo),'pi_upper':str(PI.hi),'source_J_lower':str(Js.lo),'source_J_upper':str(Js.hi),'qK':str(qK),'qS_lower':str(qS.lo),'qS_upper':str(qS.hi),'n':str(n),'rn_midpoint_lower':str(midlo),'rn_midpoint_upper':str(midhi),'A_exact':str(A),'A_f64':str(Af),'source_error_lower':str(err[0]),'source_error_upper':str(err[1]),'represented_error':str(abs(n-qK)),'source_error_ratio':float(err[0]/A),'gap_lower':str(gap),'two_allowance_upper':str(twobound),'dual_gap_ratio':float(gap/twobound),'ymin':str(ymin),'ymax':str(ymax),'other_tr_ro_max':str(other),'source_J_relative_difference':float((JK-Js.hi)/Js.hi)}
(HERE/'independent_results.json').write_text(json.dumps({'method':'atan(1/2)+atan(1/3), exact interval + metadata-derived cantilever truth and native associations, integer RN64, scalar lower-bound proof','cases':results,'source_decodes':source_decodes,'associations':association,'scalar':scalar},indent=2)+'\n')
# Comparison is post-computation only; no I46 numerical results are derivation inputs.
old=json.loads((ORIGINAL/'analysis_results.json').read_text())
for name in results:
 for new,oldmode in [('ordinary','ordinary'),('native','native_actual_point'),('projected','algebraic_direct_unit_projection')]:assert results[name]['failures'][new]==old['cases'][name]['failures'][oldmode]
oldscalar=json.loads((ORIGINAL/'rx_counterexample.json').read_text());assert qK==Q(oldscalar['qK_exact']) and A==Q(oldscalar['A_exact']) and Af==Q(oldscalar['A_f64_exact'])
assert qS.lo<=Q(oldscalar['qS_lower'])<=Q(oldscalar['qS_upper'])<=qS.hi
print(json.dumps({'cases':{k:{'failures':v['failures'],'ordinary_scales':v['ordinary_scales'],'projected_scales':v['projected_scales'],'zero_sign_failures':v['zero_sign_failures']} for k,v in results.items()},'source_error_ratio':scalar['source_error_ratio'],'dual_gap_ratio':scalar['dual_gap_ratio'],'source_error_lower':float(err[0]),'A_exact':float(A),'represented_error':float(abs(n-qK)),'gap_lower':float(gap),'two_allowance_upper':float(twobound),'I46_comparison':'all counts and scalar exact operands match; narrow I46 pi enclosure lies inside independent enclosure'},indent=2))
