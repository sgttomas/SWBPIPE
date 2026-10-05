#!/usr/bin/env python3
"""RV66 independent finite exact arithmetic; no product/model execution or oracle import.
Run in NUM: python3 <this file> <scratch-output-dir>.
Fraction arithmetic defines every decision. Decimal displays are descriptive.
"""
from fractions import Fraction as Q
from decimal import Decimal, localcontext
from pathlib import Path
import json, sys, struct, hashlib
ROOT=Path.cwd()
R=ROOT/'projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30'
OUT=Path(sys.argv[1]); OUT.mkdir(parents=True, exist_ok=True)
def p2(e): return Q(2**e) if e>=0 else Q(1,2**(-e))
def f(x): return Q.from_float(x) if isinstance(x,float) else Q(x)
def bitq(b): return f(struct.unpack('>d', bytes.fromhex(b))[0])
def bits(x): return struct.pack('>d',float(x)).hex()
def rn(x):
 x=Q(x)
 if x==0: return x
 if x<0:return -rn(-x)
 e=x.numerator.bit_length()-x.denominator.bit_length()
 if x<p2(e):e-=1
 quantum=p2(max(e-52,-1074)); a=x/quantum
 q,r=divmod(a.numerator,a.denominator)
 if 2*r>a.denominator or (2*r==a.denominator and q%2):q+=1
 out=q*quantum
 assert out<=p2(1024)-p2(971),'overflow'
 return out

def ru(x):
 x=Q(x);r=rn(x)
 if r>=x:return r
 e=r.numerator.bit_length()-r.denominator.bit_length() if r else -1074
 if r and r<p2(e):e-=1
 return r+p2(max(e-52,-1074))

def absolute_exclusion(K,S):
 threshold=rn(p2(-34)*S);bound=ru(eps*S)
 assert S>=p2(-988) and K-threshold>bound
 return {'threshold':dec(threshold),'bound':dec(bound),'gap_lower':dec(K-threshold-bound),'absolute_class_excluded':True}

def dec(x):
 with localcontext() as c:
  c.prec=48;return str(Decimal(x.numerator)/Decimal(x.denominator))
def item(x): return {'fraction':str(x),'decimal':dec(x)}
def abs_interval(n,lo,hi):
 return (max(Q(0),lo-n,n-hi),max(abs(n-lo),abs(n-hi)))
eps,u,h,q=p2(-64),p2(-53),p2(-1074),1+p2(-21)
alpha=eps*q
# General nonnegative RN64 upper bound RN(z) <= (1+u)z+h.
lam=1+u
op_alpha=lam**3*alpha;op_beta=lam**3*u
op_h=h*(lam**2*q+2*lam**2+2*lam+1)
assert 0<alpha+u<1 and 0<op_alpha+op_beta<1
def ae(n,S):return alpha*max(abs(n),S)+u*abs(n)+h
def af(n,S):return rn(rn(rn(eps*max(abs(n),S))*q)+rn(rn(u*abs(n))+h))
def no_center(K,source_lo,S,a=alpha,b=u,c=h):
 # max(|n|,S)<=|n|+S. A represented pass first bounds all possible |n|.
 N=(abs(K)+a*S+c)/(1-a-b)
 B=a*max(N,S)+b*N+c
 return {'abs_n_upper':item(N),'allowance_upper':item(B),
         'gap_minus_twice_allowance_lower':item(source_lo-K-2*B),
         'no_common_center':source_lo-K>2*B}
# Exact RN discriminators, including subnormals and ties.
assert rn(1+p2(-53))==1 and rn(1+3*p2(-53))==1+p2(-51)
assert rn(p2(-1075))==0 and rn(3*p2(-1075))==2*h
assert rn(-Q(1,3))==-rn(Q(1,3))

manifest=json.loads((R/'verification/i50_first_observed_02/FIRST_RUN.json').read_text())
verified=[]
for v in manifest['candidate_sources']+manifest['evidence']:
 raw=Path(v['location']).read_bytes(); digest=hashlib.sha256(raw).hexdigest()
 assert len(raw)==v['bytes'] and digest==v['sha256']
 verified.append({'path':v['location'],'sha256':digest,'bytes':len(raw),
                  'use':'hash only' if 'oracle' in v['location'] else 'frozen manifest member'})
log=next(v['location'] for v in manifest['evidence'] if v['location'].endswith('pp_named_second.log'))
records=[json.loads(l.split('I50_RECORD',1)[1]) for l in Path(log).read_text().splitlines() if 'I50_RECORD' in l]
assert len(records)==2
fixture=json.loads(Path(manifest['candidate_sources'][4]['location']).read_text())
for rec in records: assert rec['request']==fixture
# Decode actual source and canonical net ledger, independently of the author's oracle.
class Reader:
 def __init__(self,a): self.a=bytes(a);self.i=0
 def take(self,n):b=self.a[self.i:self.i+n];self.i+=n;assert len(b)==n;return b
 def number(self,fmt):return struct.unpack('<'+fmt,self.take(struct.calcsize(fmt)))[0]
 def u32(self):return self.number('I')
 def byte(self):return self.number('B')
 def fq(self):return f(self.number('d'))
 def dof(self):return (self.u32(),self.byte())
 def done(self):assert self.i==len(self.a)
def decode_source(a):
 r=Reader(a);assert r.take(6)==b'K4SRC\x01'
 nodes=[[r.fq() for _ in range(3)] for _ in range(r.u32())]
 members=[]
 for _ in range(r.u32()):members.append((r.u32(),r.u32(),r.u32(),[r.fq() for _ in range(6)],[r.fq() for _ in range(3)]))
 springs=[(r.u32(),r.dof(),r.fq()) for _ in range(r.u32())]
 assert r.u32()==0 # no directional springs
 constraints=[(r.dof(),r.fq()) for _ in range(r.u32())]
 loads=[]
 for _ in range(r.u32()):loads.append((r.dof(),r.take(r.u32()).decode(),r.fq()))
 stations=[(r.u32(),r.u32(),r.fq()) for _ in range(r.u32())]
 supports=[]
 for _ in range(r.u32()):
  sid,node=r.u32(),r.u32();rigid=[r.byte() for _ in range(6)]
  sp=[r.u32() for _ in range(r.u32())];di=[r.u32() for _ in range(r.u32())]
  supports.append((sid,node,rigid,sp,di))
 r.done();return nodes,members,springs,constraints,loads,stations,supports
def decode_ledger(a):
 r=Reader(a);assert r.take(6)==b'K4LED\x01';d={}
 for _ in range(r.u32()):
  dof=r.dof();sign=r.byte();e=r.number('q');limbs=[r.number('Q') for _ in range(r.u32())]
  d[dof]=(-1 if sign else 1)*sum(v*2**(64*i) for i,v in enumerate(limbs))*p2(e)
 r.done();return d
src=decode_source(records[0]['native']['source_encoding']);nodes,members,springs,constraints,loads,stations,supports=src
assert nodes==[[0,0,0],[1,2,2]] and len(members)==1
assert constraints==[((0,0),0),((0,1),0),((0,2),0)]
assert springs==[(0,(0,3),144),(1,(0,4),1000000),(2,(0,5),1000000)]
assert stations==[(0,0,Q(1,4)),(1,0,Q(1,2)),(2,0,Q(3,4))]
assert supports==[(0,0,[1,1,1,0,0,0],[],[]),(1,0,[0]*6,[0],[]),(2,0,[0]*6,[1],[]),(3,0,[0]*6,[2],[])]
ledger=decode_ledger(records[0]['native']['ledger_encoding'])
assert len(loads)==3 and len(ledger)==3
assert ledger=={dof:v for dof,_,v in loads}
request_loads=fixture['model']['load_cases'][0]['primitive_loads']
assert [(a['direction'],f(a['magnitude']['value'])) for a in request_loads]==[(a,ledger[(1,i)]) for a,i in [('RX',3),('RY',4),('RZ',5)]]
x=ledger[(1,3)];assert ledger[(1,4)]==2*x and ledger[(1,5)]==2*x
T=3*x
facts=records[0]['facts'][0];D,t,c,J,A,Z=[f(facts[k]) for k in ['D','t','c','J','A','Z']]
assert c==D/2 and D==f(fixture['model']['pipe_segments'][0]['section']['outside_diameter']['value'])
assert t==f(fixture['model']['pipe_segments'][0]['section']['wall_thickness']['value'])
assert 'mill_tolerance' not in fixture['model']['pipe_segments'][0]['section']
assert members[0]==(0,0,1,[Q(200000000000),Q(80000000000),A,f(facts['I']),f(facts['I']),J],[1,0,0])
assert J==2*f(facts['I']) and Z==rn(f(facts['I'])/c)
for rec in records:
 assert decode_source(rec['native']['source_encoding'])==src
 assert decode_ledger(rec['native']['ledger_encoding'])==ledger
 assert rec['facts']==records[0]['facts'] and rec['source']==records[0]['source']
 assert rec['native']['precision']==128 and len(rec['native']['rows'])==58
# Independent pi identity: atan(1/2)+atan(1/3)=pi/4. Even partial sums are lower.
def atan_bounds(k,N=180):
 s=sum((Q((-1)**j,(2*j+1)*k**(2*j+1)) for j in range(N)),Q(0))
 return s,s+Q(1,(2*N+1)*k**(2*N+1))
a2,b2=atan_bounds(2);a3,b3=atan_bounds(3)
series_lo,series_hi=4*(a2+a3),4*(b2+b3)
unit=p2(-256)
pi_lo=(series_lo//unit)*unit;pi_hi=(-(-series_hi//unit))*unit
assert 3<pi_lo<pi_hi<4 and pi_hi-pi_lo==unit
ri=c-t; g=(c*c-ri*ri)*(c*c+ri*ri)
Jlo,Jhi=pi_lo*g/2,pi_hi*g/2
slo,shi=T*c/Jhi,T*c/Jlo
K=T*c/J
assert 0<Jlo<Jhi<J and K<slo<shi
assert Q('26.65694888657507787210')<slo<shi<Q('26.65694888657507787211')
assert Q('26.65694888657506343057')<K<Q('26.65694888657506343058')
assert slo-K>Q('1.4441530601225e-14')

def coupled(s):
 tr,ro,fo,mo=s;L=Q(3)
 return [max(tr,rn(L*ro)),max(ro,rn(tr/L)),max(fo,rn(mo/L)),max(mo,rn(L*fo))]
def stress_scale(coupled):return rn(rn(coupled[2]/A)+rn(Q(1)*rn(coupled[3]/Z)))
def kind(row):
 k=row['kind'];u=row['unit']
 if k in ('displacement_magnitude','global_nodal_displacement_x','global_nodal_displacement_y','global_nodal_displacement_z'):return 0
 if k.startswith('global_nodal_rotation_'):return 1
 if k in ('element_local_axial_force','element_local_shear_force_y','element_local_shear_force_z','support_reaction_force_magnitude_v2'):return 2
 if k in ('element_local_torsional_moment','element_local_bending_moment_y','element_local_bending_moment_z','support_reaction_moment_magnitude_v2'):return 3
 if k=='support_reaction_component_v2':return 2 if u=='N' else 3
 return None
summary={'primitive_bits':{k:bits(f(v)) for k,v in facts.items()},'load_bits':[bits(v) for _,_,v in loads],'material_bits':{'E':bits(members[0][3][0]),'G':bits(members[0][3][1])},'pi_interval_width':dec(pi_hi-pi_lo),'torque':item(T),'source_stress_lower':item(slo),'source_stress_upper':item(shi),'represented_stress':item(K),'separation_lower':item(slo-K),'separation_upper':item(shi-K),'modes':[]}
for rec in records:
 rows=rec['envelope']['results'];s=[Q(0)]*4;winners=[None]*4
 for row in rows:
  slot=kind(row)
  if slot is None:continue
  # InputDerived root translations are excluded; magnitude remains a scaled kind.
  if row['kind'].startswith('global_nodal_displacement_') and row['entity_ref']=='N0':continue
  value=f(row['value']); n=rn(value/1000) if row['unit']=='mm' else value
  if abs(n)>s[slot]:s[slot]=abs(n);winners[slot]=row['id']
 cs=coupled(s);S=stress_scale(cs)
 i,row=next((i,row) for i,row in enumerate(rows) if row['id']=='result:stress:M1:end-i:torsional-shear')
 y=f(row['value']);n=rn(y*1000000)
 assert row['unit']=='MPa' and row['basis_ref']=={'ref_id':'case','ref_type':'load_case'} and row['entity_ref']=='M1'
 assert n>=rn(p2(-34)*S)
 errlo,errhi=abs_interval(n,slo,shi)
 entry={'mode':rec['mode'],'row_index':i,'raw_bits':bits(y),'normalized_bits':bits(n),'normalized':dec(n),'uncoupled_primary_scales':[dec(a) for a in s],'scale_winners':winners,'coupled_primary_scales':[dec(a) for a in cs],'current_stress_scale':dec(S),'current_stress_scale_bits':bits(S),'sharper_exact_at_current':dec(ae(n,S)),'sharper_binary64_at_current':dec(af(n,S)),'represented_point_error':dec(abs(n-K)),'source_point_error_lower':dec(errlo),'source_point_error_upper':dec(errhi),'current_exact_no_center':no_center(K,slo,S),'current_binary64_no_center':no_center(K,slo,S,op_alpha,op_beta,op_h)}
 assert abs(n-K)>ae(n,S) and errlo>ae(n,S)
 assert abs(n-K)>af(n,S) and errlo>af(n,S)
 # Absolute-class centers cannot approach K either at this fixed scale.
 entry['absolute_exclusion']=absolute_exclusion(K,S)
 if rec['mode']=='sparse_interactive':
  verdict=rec['verdicts'][i]; assert verdict['row']==i and verdict['scale_bits']==bits(S) and verdict['normalized_bits']==bits(n)
  assert verdict['class']=='Some(RelativeVerified)' and verdict['predicates']==[False,False,True,True]
  entry['actual_certificate']='sparse relative refusing both sharper predicates; decimal pair passed'
 else:
  assert rec['verdicts']==[];entry['actual_certificate']='none in FIRST_RUN; calculated scale is algebraic only'
 # Hypothetical final primary projection: captured native values, mm roundtrip,
 # source-prescribed InputDerived exclusions, support slot values already included.
 ns=[Q(0)]*4;nwin=[None]*4
 for nr in rec['native']['rows']:
  if nr['class']=='InputDerived':continue
  slot=['Translation','Rotation','Force','Moment'].index(nr['kind']);v=bitq(nr['value_bits'])
  if slot==0:v=rn(rn(v*1000)/1000)
  if abs(v)>ns[slot]:ns[slot]=abs(v);nwin[slot]=nr['id']
 ncs=coupled(ns);nS=stress_scale(ncs)
 tv=-bitq(next(v['value_bits'] for v in rec['native']['rows'] if v['id']=='EndAction { member: 0, end: I, component: Rx }'))
 projected_y=rn(rn(rn(tv*c)/J)/1000000); projected_n=rn(projected_y*1000000)
 entry['hypothetical_native_primary_projection']={'uncoupled':[dec(a) for a in ns],'winners':nwin,'coupled':[dec(a) for a in ncs],'stress_scale':dec(nS),'stress_scale_bits':bits(nS),'absolute_exclusion':absolute_exclusion(K,nS),'torsion_raw_bits':bits(projected_y),'torsion_normalized_bits':bits(projected_n),'exact_no_center':no_center(K,slo,nS),'binary64_no_center':no_center(K,slo,nS,op_alpha,op_beta,op_h)}
 for key in ['current_exact_no_center','current_binary64_no_center']:
  assert entry[key]['no_common_center']
  assert Q(entry[key]['allowance_upper']['fraction'])<Q('2.962449588731e-15')
  assert Q(entry[key]['gap_minus_twice_allowance_lower']['fraction'])>Q('8.5166314237e-15')
 assert entry['hypothetical_native_primary_projection']['exact_no_center']['no_common_center']
 assert entry['hypothetical_native_primary_projection']['binary64_no_center']['no_common_center']
 for key in ['exact_no_center','binary64_no_center']:
  evidence=entry['hypothetical_native_primary_projection'][key]
  assert Q(evidence['allowance_upper']['fraction'])<Q('2.962449588731e-15')
  assert Q(evidence['gap_minus_twice_allowance_lower']['fraction'])>Q('8.5166314237e-15')
 summary['modes'].append(entry)
# Discriminating scalar control, not an adopted scale or complete producer.
control_S=Q(2**18);control_y=rn(((slo+shi)/2+K)/2/1000000);control_n=rn(control_y*1000000)
control_H=max(abs(control_n-K),abs(control_n-slo),abs(control_n-shi))
control_raw_H=max(abs(control_y-K/1000000),abs(control_y-slo/1000000),abs(control_y-shi/1000000))
assert control_n>rn(p2(-34)*control_S)
control_checks=[control_H<=ae(control_n,control_S),control_H<=af(control_n,control_S),10**9*control_H<=abs(control_n),10**9*control_raw_H<=abs(control_y)]
assert all(control_checks)
summary['scalar_scale_dependence_control']={'S':str(control_S),'y_bits':bits(control_y),'n_bits':bits(control_n),'H_upper':dec(control_H),'four_checks':control_checks,'scope':'artificial scalar feasibility discriminator only; no scale proposal or complete producer'}
fractions={'pi_series_lo':series_lo,'pi_series_hi':series_hi,'pi_lo':pi_lo,'pi_hi':pi_hi,'D':D,'t':t,'c':c,'A_K':A,'I_K':f(facts['I']),'J_K':J,'Z_hat':Z,'G_factor':g,'J_source_lo':Jlo,'J_source_hi':Jhi,'T':T,'K':K,'source_lo':slo,'source_hi':shi,'alpha':alpha,'beta':u,'h':h,'op_alpha':op_alpha,'op_beta':op_beta,'op_h':op_h}
(OUT/'exact_fractions.json').write_text(json.dumps({k:str(v) for k,v in fractions.items()},indent=2)+'\n')
(OUT/'results.json').write_text(json.dumps(summary,indent=2)+'\n')
(OUT/'verified_inputs.json').write_text(json.dumps(verified,indent=2)+'\n')
print(json.dumps({k:v for k,v in summary.items() if k not in ('modes',)},indent=2))
for m in summary['modes']:
 print(m['mode'],'S',m['current_stress_scale'],'n',m['normalized'],'source error',m['source_point_error_lower'],'exact?',m['current_exact_no_center']['no_common_center'],'gap-2B',m['current_exact_no_center']['gap_minus_twice_allowance_lower']['decimal'],'projected S',m['hypothetical_native_primary_projection']['stress_scale'])
print('PASS: frozen hashes, source/ledger binding, arithmetic controls, both readout gaps, current and projected fixed-scale impossibility, scalar feasibility control')
