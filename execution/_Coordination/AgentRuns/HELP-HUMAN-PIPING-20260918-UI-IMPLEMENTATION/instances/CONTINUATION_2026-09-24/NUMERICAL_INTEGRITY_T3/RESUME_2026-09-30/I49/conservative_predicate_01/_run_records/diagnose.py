"""I49 exact, source-driven row10 diagnosis; no product imports or runtime.
Run from NUM with python3 <this path>. Reads only captured copies and writes owned JSON.
"""
from fractions import Fraction as F
from pathlib import Path
from decimal import Decimal, localcontext
import json, struct, hashlib, datetime
P=Path(__file__).resolve().parent
D=json.loads((P/'ACTUAL_CASE.json').read_text())
def b64(h):
 b=int(h,16); sign=-1 if b>>63 else 1;e=(b>>52)&2047;m=b&((1<<52)-1)
 assert e<2047
 return sign*F(m if e==0 else m+(1<<52))*pow2(-1074 if e==0 else e-1075)
def pow2(e):return F(2**e) if e>=0 else F(1,2**(-e))
def bits(f):return struct.pack('>d',f).hex()
def atom(f):return b64(bits(f))
def exp2(q):
 q=abs(q);e=q.numerator.bit_length()-q.denominator.bit_length()
 if q<pow2(e):e-=1
 assert pow2(e)<=q<pow2(e+1)
 return e
def rn_bits(q):
 if q==0:return '0000000000000000'
 sign=(1<<63) if q<0 else 0;q=abs(q);e=exp2(q);lsb=max(e-52,-1074)
 z=q/pow2(lsb);m,r=divmod(z.numerator,z.denominator)
 if 2*r>z.denominator or (2*r==z.denominator and m%2):m+=1
 if m==1<<53:m>>=1;e+=1
 if e<-1022:v=m
 else:v=((e+1023)<<52)+(m-(1<<52))
 return f'{sign|v:016x}'
def rn(q):return b64(rn_bits(q))
def dec(q):
 with localcontext() as c:
  c.prec=28;return str(Decimal(q.numerator)/Decimal(q.denominator))
def show(q):return {'exact':str(q),'decimal':dec(q)}
def allowance(x,s):
 a0=rn(pow2(-64)*max(abs(x),s));a1=rn(a0*(1+pow2(-21)))
 u0=rn(pow2(-53)*abs(x));u1=rn(u0+pow2(-1074));a=rn(a1+u1)
 return max(abs(x),s)*(pow2(-64)+pow2(-85))+abs(x)*pow2(-53)+pow2(-1074),a
sel=D['I47_SELECTION'];v=sel['values'];inp=D['I47_INPUT'];req=D['I47_REQUEST'];rows=D['I47_ROWS'];nr=D['I47_NATIVE_ROWS'];ver=D['I47_VERDICTS'][10]
assert D['I47_CASE']=='interpolated-loaded'
assert rows[10]['id']=='result:disp:node-N-DEC092-TIP:ux' and ver['row']==10
assert rows[10]['basis_ref']['ref_id']==sel['case']=='case:i45-loaded'
assert nr[6]['quantity']=='Displacement(Dof { node: 1, component: Ux })'
assert nr[9]['quantity']=='Displacement(Dof { node: 1, component: Rx })'
assert v['kind']=='interpolated' and (v['lower'],v['upper'])==(1,2)
tl,t,th=[b64(v[k]) for k in ['t_lo','t','t_hi']]
assert (tl,t,th)==(293,303,313)
material=req['materials'][sel['material_ordinal']]
assert material['id']==sel['material']
for ordinal,suffix in [(v['lower'],'_lo'),(v['upper'],'_hi')]:
 point=material['temperature_points'][ordinal]
 assert point['id']==sel['point_ids'][0 if suffix=='_lo' else 1]
 assert point['temperature']['unit']=='K' and atom(point['temperature']['value'])==b64(v['t'+suffix])
 for key,c in [('elastic_modulus','e'),('shear_modulus','g')]:
  assert point[key]['unit']=='Pa' and bits(point[key]['value'])==v[c+suffix]
assert atom(req['model']['load_cases'][0]['modulus_basis_temperature']['value'])==t
inter={}
for c in ['e','g']:
 lo,hi,hat=[b64(v[c+k]) for k in ['_lo','_hi','_hat']]
 products=[th*lo,-t*lo,t*hi,-tl*hi];source=sum(products)/(th-tl)
 inter[c]={'products':[str(a) for a in products],'denominator':str(th-tl),'source':show(source),'represented':show(hat),'difference':show(source-hat)}
Es=F(inter['e']['source']['exact']);Gs=F(inter['g']['source']['exact']);Eh=b64(v['e_hat']);Gh=b64(v['g_hat'])
assert Eh==atom(inp['E']) and Gh==atom(inp['G'])
model=req['model']; assert len(model['pipe_segments'])==1
assert len(model['nodes'])==2 and model['nodes'][0]['position']==dict(x=0.0,y=0.0,z=0.0) and model['nodes'][1]['position']==dict(x=1.0,y=0.0,z=0.0)
loads=model['load_cases'][0]['primitive_loads'];assert [(l['direction'],atom(l['magnitude']['value'])) for l in loads]==[('UX',1),('UY',1),('RX',1)]
assert all(l['target']['node']=='node:N-DEC092-TIP' for l in loads)
assert model['supports'][0]['restraints']==['UX','UY','UZ','RX','RY','RZ']
x=b64(nr[6]['value_bits']);y=atom(rows[10]['value']);n=rn(y/1000);s=b64(ver['scale_bits'])
assert rn_bits(y/1000)==ver['normalized_bits']==nr[6]['value_bits']
tr=max(abs(rn(atom(r['value'])/1000)) for r in rows if r['kind']=='displacement_magnitude' or r['kind'].startswith('global_nodal_displacement_'))
ro=max(abs(atom(r['value'])) for r in rows if r['kind'].startswith('global_nodal_rotation_'))
assert max(tr,ro)==s and s==b64(nr[9]['value_bits'])
Ae,Ab=allowance(n,s);assert ver['class']=='relative' and abs(n)>=rn(pow2(-34)*s)
# Independent pi: alternating atan bounds, not the packet author's oracle or runtime pi.
def atan_bounds(den,N=128):
 z=F(1,den);s=sum(((-1)**j)*z**(2*j+1)/F(2*j+1) for j in range(N));nextterm=z**(2*N+1)/F(2*N+1)
 return (s,s+nextterm) if N%2==0 else (s-nextterm,s)
a,b=atan_bounds(5);c,d=atan_bounds(239);pi_lo,pi_hi=16*a-4*d,16*b-4*c
D0,T0=atom(inp['D']),atom(inp['t']);p=T0*(D0-T0);Khat=Eh*atom(inp['A']);truthK=1/Khat
truthG=(1/(Es*p*pi_hi),1/(Es*p*pi_lo))
errK=abs(n-truthK);errG=max(abs(n-truthG[0]),abs(n-truthG[1]))
rawK=abs(y-1000*truthK);rawG=max(abs(y-1000*truthG[0]),abs(y-1000*truthG[1]))
assert errK<=Ae and errK<=Ab and errG<=Ae and errG<=Ab
assert errK<=abs(n)/10**9 and errG<=abs(n)/10**9
assert rawK<=abs(y)/10**9 and rawG<=abs(y)/10**9
# Exact native diagonal exponents: axial EA/L and torsional GJ/L; L=1, no other member or spring.
Krx=Gh*atom(inp['J'])
sx=-(exp2(Khat)//2);sr=-(exp2(Krx)//2)
assert (exp2(Khat),exp2(Krx),sx,sr)==(28,17,-14,-8)
B=b64(D['I47_G5A_DATA']['B'][0][1]);assert D['I47_G5A_DATA']['precision']==128
# For any corrected Rx center c and any fixed positive J* admitted by source geometry:
# max(|1-Gh*J*c|,|1-Gs*J*c|) >= (Gs-Gh)/(Gs+Gh).
# Thus omega >= 2^sr times RHS, epsilon >= 2 B omega because 0 <= alpha < 1.
# UX gather radius >= 2^sx epsilon. This lower bound does not use a private center or endpoint.
minimax=(Gs-Gh)/(Gs+Gh)
forced=2*B*pow2(sx+sr)*minimax
assert forced>Ae and forced>Ab
# Algebraic direct unit projection of captured native rows: no producer execution.
project=[];maxima=[F(0) for _ in range(4)];native_maxima=[F(0) for _ in range(4)]
for row in nr:
 q=b64(row['value_bits']);raw=rn(q*1000) if row['kind']==0 else q
 normalized=rn(raw/1000) if row['kind']==0 else raw
 project.append({'ordinal':row['ordinal'],'raw_bits':rn_bits(raw),'normalized_bits':rn_bits(normalized)})
 if row['class']!='InputDerived':
  maxima[row['kind']]=max(maxima[row['kind']],abs(normalized))
  native_maxima[row['kind']]=max(native_maxima[row['kind']],abs(q))
 assert rn_bits(q)==row['value_bits']
project_scale=max(maxima[0],maxima[1]);project_y=b64(project[6]['raw_bits']);project_n=b64(project[6]['normalized_bits']);pAe,pAb=allowance(project_n,project_scale)
assert project_y==y and project_n==n and project_scale==s
assert max(native_maxima[0],native_maxima[1])==s
assert forced>pAe and forced>pAb
result={
 'completed_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),
 'kind':'exact arithmetic/source diagnosis; neither native runtime nor full producer execution',
 'case':D['I47_CASE'],'final_row':10,'native_row':6,'id':rows[10]['id'],
 'interpolation':inter,
 'actual_bits':{'raw':bits(rows[10]['value']),'normalized':rn_bits(n),'native':nr[6]['value_bits'],'scale':rn_bits(s),'binary64_allowance':rn_bits(Ab),'B':D['I47_G5A_DATA']['B'][0][1]},
 'actual':{'raw':show(y),'normalized':show(n),'scale':show(s),'primary_translation_max':show(tr),'primary_rotation_max':show(ro),'class_threshold':show(rn(pow2(-34)*s)),'sharper_exact_allowance':show(Ae),'sharper_binary64_allowance':show(Ab)},
 'independent_truth':{'represented_K':show(Khat),'represented_UX':show(truthK),'source_UX_interval':[str(z) for z in truthG],'source_UX_decimal_bounds':[dec(z) for z in truthG],'pi_method':'Machin alternating rational series; 128 terms per atan','pi_interval':[str(pi_lo),str(pi_hi)],'represented_error':show(errK),'source_error_upper':show(errG),'all_four_row_predicates_pass_both_truths':True},
 'forced_geometric_lower_bound':{'represented_torsion_diagonal':show(Krx),'diagonal_exponents':[28,17],'radix_exponents_UX_RX':[sx,sr],'B':show(B),'material_G_hull_width':show(Gs-Gh),'residual_minimax':show(minimax),'UX_half_width_lower_bound':show(forced),'ratio_to_sharper_exact':show(forced/Ae),'ratio_to_sharper_binary64':show(forced/Ab),'exceeds_both':True,'center_independent':True,'private_endpoints_reconstructed':False,'actual_native_radius_reconstructed':False},
 'direct_native_projection':{'primary_maxima':[show(z) for z in maxima],'native_primary_maxima':[show(z) for z in native_maxima],'native_coupled_scale_bits':rn_bits(max(native_maxima[0],native_maxima[1])),'coupled_translation_rotation_scale':show(project_scale),'raw_bits':rn_bits(project_y),'normalized_bits':rn_bits(project_n),'scale_bits':rn_bits(project_scale),'same_raw_normalized_and_scale':True,'forced_refusal_remains':True},
 'limits':['Single row only; no claim about the other 46 conservative rows.','No private geometric endpoints, correction centers or actual native radius reconstructed.','No output, scale, predicate or source change; projection is algebraic, not an executed producer.','No I47 code clearance, acceptance, routing, availability, C2, resource or release qualification.']}
(P/'RESULTS.json').write_text(json.dumps(result,indent=2)+'\n')
(P/'PROJECTION_PRIMARY_ROWS.json').write_text(json.dumps(project,indent=2)+'\n')
print(json.dumps({'bits':result['actual_bits'],'source_E':str(Es),'source_G':str(Gs),'errorK':dec(errK),'errorG_upper':dec(errG),'allow_exact':dec(Ae),'allow_binary64':dec(Ab),'forced_min':dec(forced),'ratio':dec(forced/Ae),'projection_same':True},indent=2))
