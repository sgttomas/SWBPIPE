#!/usr/bin/env python3
"""RV64 exact arithmetic only. No production/native/solver execution; no author code."""
from pathlib import Path
from fractions import Fraction as F
from decimal import Decimal, localcontext
import json, struct, hashlib, subprocess, os, datetime
W=Path.cwd(); assert W.name=='numerics'
R=Path('projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30')
O=W/R/'REVIEW_RV64/conservative_predicate_01'; D=O/'_run_records'; I=W/R/'I49/conservative_predicate_01'; C=W.parent/'f2a'; rev='d0daa18717f8243a7232e898c9ef9b4f4d18d9e4'
def sha(b):return hashlib.sha256(b).hexdigest()
def write(n,o): (D/n).write_text(json.dumps(o,indent=2,sort_keys=True)+'\n')
origins=[]; commands=[]
def record(p,role):
 b=p.read_bytes();origins.append({'path':str(p),'sha256':sha(b),'bytes':len(b),'role':role});return b
def git(root,args):
 cmd=['git',*args];commands.append({'cwd':str(root),'env':{'GIT_OPTIONAL_LOCKS':'0'},'argv':cmd})
 return subprocess.check_output(cmd,cwd=root,env={**os.environ,'GIT_OPTIONAL_LOCKS':'0'})
for p in ['AGENTS.md','agents/AGENT_TASK.md','projects/chirality-piping/AGENTS.md',str(R/'BRIEFS/RV64_CONSERVATIVE_PREDICATE_REVIEW.md')]:
 b=record(W/p,'active instruction');assert b==git(W,['show','03025d023bf1bc42c73ad9559b492134d100b338:'+p])
packet_checks=[]
for name,expected in [('RETURN.md','7c5da65a02762ae34e3e2847e61571e5ae4f7fa7516165460d118f7da7bf353c'),('SEAL.json','bfac20c908d6c7c96574f0bfaf3768fa99a36fb125ad6e1353a58d8b212d40b7'),('_run_records/INVENTORY.json','067326ba5ba19e01626fb33fb76a49ded06735ae5bfacf9ba606fde25a800870')]:
 b=record(I/name,'review target');assert sha(b)==expected
for e in json.loads((I/'_run_records/INVENTORY.json').read_text()):
 b=(I/e['path']).read_bytes();assert sha(b)==e['sha256'] and len(b)==e['bytes'];packet_checks.append(e)
write('SUBJECT_PAYLOAD_CHECKS.json',packet_checks)
# Preserve reviewed source; equality to immutable revision, NUM and CODE independently checked.
author_origins=json.loads(record(I/'_run_records/ORIGINS.json','source origin index'))
source_checks=[]
for e in author_origins:
 if 'copy' not in e:continue
 path=Path(e['origin']).relative_to(C); pinned=git(C,['show',rev+':'+str(path)])
 b=record(W/e['copy'],'pinned source copy');assert b==pinned and sha(b)==e['sha256']
 assert (W/path).read_bytes()==pinned and (C/path).read_bytes()==pinned
 if path.name != 'lib.rs':(D/('SOURCE_'+path.name)).write_bytes(pinned)
 source_checks.append({'path':str(path),'sha256':sha(pinned),'matches_revision_NUM_CODE':True})
for p,name in [('directed.rs','directed.rs'),('wide.rs','wide.rs'),('wide/multi.rs','wide_multi.rs'),('source.rs','source.rs')]:
 path=Path('projects/chirality-piping/core/solver/frame_kernel/src/structural/retained')/p
 b=git(C,['show',rev+':'+str(path)]);assert (W/path).read_bytes()==b and (C/path).read_bytes()==b
 record(W/path,'additional reviewed source');(D/('SOURCE_'+name)).write_bytes(b)
 source_checks.append({'path':str(path),'sha256':sha(b),'matches_revision_NUM_CODE':True})
write('SOURCE_CHECKS.json',source_checks)
logpath=C/R/'I47/selected_material_02/_run_records/pp_debug.log'
log=record(logpath,'raw capture');assert sha(log)=='b443aabe00d24804e98dfe3f662db32d2a3277de976790998653354674f002c5'
oracle=record(C/R/'I47/selected_material_02/_run_records/oracle_final_debug_results.json','oracle identity only, not arithmetic basis');assert sha(oracle)=='43e37d0c2085487c2f86087612400398706dbe4a21d6d8a12ec74420bb8c117f'
lines=log.decode().splitlines();start=lines.index('I47_CASE interpolated-loaded');case={};loc={}
wanted=['I47_INPUT','I47_REQUEST','I47_SELECTION','I47_BOUNDARY','I47_BASIS_CAPTURE','I47_ROWS','I47_VERDICTS','I47_G5A_DATA','I47_NATIVE_ROWS','I47_NATIVE_IDENTITY']
for index,line in enumerate(lines[start+1:],start+2):
 if line.startswith('I47_CASE '):break
 tag,_,v=line.partition(' ')
 if tag in wanted and tag not in case:case[tag]=json.loads(v);loc[tag]=index
assert set(case)==set(wanted)
write('RAW_CASE.json',case);write('RAW_CASE_LINES.json',loc)
def bits(x):return struct.pack('>d',x).hex()
def frombits(x):
 n=int(x,16);sign=-1 if n>>63 else 1;e=(n>>52)&2047;m=n&((1<<52)-1)
 assert e!=2047
 return sign*F(m if e==0 else m+(1<<52))*pow2(-1074 if e==0 else e-1075)
def pow2(e):return F(2**e) if e>=0 else F(1,2**-e)
def ff(x):return frombits(bits(x))
def exponent(x):
 assert x>0;e=x.numerator.bit_length()-x.denominator.bit_length()
 return e if x>=pow2(e) else e-1
def rnd_int(x):
 q,r=divmod(x.numerator,x.denominator);return q+int(2*r>x.denominator or (2*r==x.denominator and q%2))
def rn(x):
 if x==0:return F(0)
 if x<0:return -rn(-x)
 e=max(exponent(x)-52,-1074);return rnd_int(x/pow2(e))*pow2(e)
def rb(x):return bits(float(x))
def significant(x):
 if x==0:return 0
 n=abs(x.numerator)
 while n%2==0:n//=2
 assert x.denominator&(x.denominator-1)==0
 return n.bit_length()
def pack(x):
 with localcontext() as dc:
  dc.prec=40
  return {'fraction':str(x),'decimal':str(Decimal(x.numerator)/Decimal(x.denominator))}
# Decode actual K4SRC, independently from encoding() source rather than author's ACTUAL_CASE.
data=bytes(case['I47_NATIVE_IDENTITY']['source']);pos=0
def take(n):
 global pos
 a=data[pos:pos+n];assert len(a)==n;pos+=n;return a
def u32():return int.from_bytes(take(4),'little')
def dbl():return struct.unpack('<d',take(8))[0]
def dof():return [u32(),take(1)[0]]
assert take(6)==b'K4SRC\x01'
nodes=[[dbl() for _ in range(3)] for _ in range(u32())]
ms=[]
for _ in range(u32()):ms.append({'id':u32(),'ni':u32(),'nj':u32(),'properties':[dbl() for _ in range(6)],'y':[dbl() for _ in range(3)]})
assert u32()==0 # springs
assert u32()==0 # directional springs
constraints=[{'dof':dof(),'value':dbl()} for _ in range(u32())]
loads=[]
for _ in range(u32()):loads.append({'dof':dof(),'id':take(u32()).decode(),'value':dbl()})
stations=[{'id':u32(),'member':u32(),'fraction':dbl()} for _ in range(u32())]
supports=[]
for _ in range(u32()):
 s={'id':u32(),'node':u32(),'restraints':list(take(6))};s['springs']=[u32() for _ in range(u32())];s['directional']=[u32() for _ in range(u32())];supports.append(s)
assert pos==len(data)
decoded={'nodes':nodes,'members':ms,'constraints':constraints,'loads':loads,'stations':stations,'supports':supports};write('DECODED_SOURCE.json',decoded)
assert nodes==[[0.,0.,0.],[1.,0.,0.]] and len(ms)==1 and ms[0]['ni']==0 and ms[0]['nj']==1 and ms[0]['y']==[0.,1.,0.]
assert constraints==[{'dof':[0,k],'value':0.} for k in range(6)]
assert loads==[{'dof':[1,k],'id':'i45:'+n,'value':1.} for k,n in [(0,'UX'),(1,'UY'),(3,'RX')]]
input=case['I47_INPUT'];req=case['I47_REQUEST'];sel=case['I47_SELECTION'];vals=sel['values'];v={k:frombits(x) for k,x in vals.items() if k not in ['kind','lower','upper']}
assert sel['material_ordinal']==0 and sel['case']=='case:i45-loaded' and vals['lower']==1 and vals['upper']==2
mat=req['materials'][0];assert mat['id']==sel['material'];assert [mat['temperature_points'][i]['id'] for i in [1,2]]==sel['point_ids']
for name,field in [('e','elastic_modulus'),('g','shear_modulus')]:
 for suffix,ix in [('lo',1),('hi',2)]:
  q=mat['temperature_points'][ix][field];assert q['unit']=='Pa' and ff(q['value'])==v[name+'_'+suffix]
for suffix,ix in [('lo',1),('hi',2)]:
 q=mat['temperature_points'][ix]['temperature'];assert q['unit']=='K' and ff(q['value'])==v['t_'+suffix]
assert ff(req['model']['load_cases'][0]['modulus_basis_temperature']['value'])==v['t']
assert req['model']['pipe_segments'][0]['section']['outside_diameter']['unit']=='m'
assert req['model']['pipe_segments'][0]['section']['wall_thickness']['unit']=='m'
for name,key in [('E','e_hat'),('G','g_hat')]:assert ff(input[name])==v[key]
assert ms[0]['properties']==[input[k] for k in ['E','G','A','I','I','J']]
assert bits(input['D'])==bits(req['model']['pipe_segments'][0]['section']['outside_diameter']['value'])
assert bits(input['t'])==bits(req['model']['pipe_segments'][0]['section']['wall_thickness']['value'])
interpolated={};products={}
for k in ['e','g']:
 a,b=v[k+'_lo'],v[k+'_hi'];terms=[v['t_hi']*a,v['t']*a,v['t']*b,v['t_lo']*b]
 products[k]=[pack(x) for x in terms];interpolated[k]=(terms[0]-terms[1]+terms[2]-terms[3])/(v['t_hi']-v['t_lo'])
E,G=interpolated['e'],interpolated['g'];Eh,Gh=v['e_hat'],v['g_hat'];A,J=ff(input['A']),ff(input['J']);diam,wall=ff(input['D']),ff(input['t'])
assert E>Eh and G>Gh and diam>2*wall>0 and J>0
# Fresh independent pi enclosure: pi/4 = atan(1/2)+atan(1/3), alternating-series brackets.
def atan_bracket(d,n=180):
 q=F(1,d);s=sum(((-1)**k*q**(2*k+1)/F(2*k+1) for k in range(n)),F(0));nextterm=(-1)**n*q**(2*n+1)/F(2*n+1)
 return min(s,s+nextterm),max(s,s+nextterm)
a,b=atan_bracket(2);c,d=atan_bracket(3);pi_lo,pi_hi=4*(a+c),4*(b+d)
qk=1/(Eh*A);qs=(1/(E*pi_hi*wall*(diam-wall)),1/(E*pi_lo*wall*(diam-wall)))
row=case['I47_ROWS'][10];verdict=case['I47_VERDICTS'][10];native=case['I47_NATIVE_ROWS'][6]
assert row['id']=='result:disp:node-N-DEC092-TIP:ux' and row['unit']=='mm' and row['basis_ref']['ref_id']==sel['case']
assert native['ordinal']==6 and native['quantity']=='Displacement(Dof { node: 1, component: Ux })' and native['body']==0 and native['kind']==0
raw=ff(row['value']);n=rn(raw/1000);assert rb(n)==verdict['normalized_bits']==native['value_bits'];assert verdict['class']=='relative' and verdict['predicates']==[False,False,True,True]
primary=[F(0)]*4
for x in case['I47_NATIVE_ROWS']:
 if x['class']!='InputDerived':primary[x['kind']]=max(primary[x['kind']],abs(frombits(x['value_bits'])))
scale=max(primary[0],primary[1]);assert rb(scale)==verdict['scale_bits']
# Separate exact-RN64 producer projection and final re-normalization for all primary native rows.
projection=[];projected=[F(0)]*4
for x in case['I47_NATIVE_ROWS']:
 z=frombits(x['value_bits']);kind=x['kind'];rawp=rn(z*1000) if kind==0 else z;norm=rn(rawp/1000) if kind==0 else rawp
 if x['class']!='InputDerived':projected[kind]=max(projected[kind],abs(norm))
 projection.append({'native':x['ordinal'],'quantity':x['quantity'],'kind':kind,'raw_bits':rb(rawp),'normalized_bits':rb(norm)})
projected_scale=max(projected[0],projected[1]);assert projected_scale==scale
assert projection[6]['raw_bits']==bits(row['value']) and projection[6]['normalized_bits']==rb(n)
M=max(abs(n),scale);sharp_exact=M*(pow2(-64)+pow2(-85))+abs(n)*pow2(-53)+pow2(-1074)
a0=rn(pow2(-64)*M);a1=rn(a0*(1+pow2(-21)));u0=rn(pow2(-53)*abs(n));u1=rn(u0+pow2(-1074));sharp64=rn(a1+u1)
assert abs(n)>=rn(frombits('3dd0000000000000')*scale)
def errors(truth):return max(abs(n-truth[0]),abs(n-truth[1])),max(abs(raw-1000*truth[0]),abs(raw-1000*truth[1]))
checks={}
for label,truth in [('K',(qk,qk)),('source',qs)]:
 err,rawerr=errors(truth);tests=[err<=sharp_exact,err<=sharp64,err*10**9<=abs(n),rawerr*10**9<=abs(raw)];assert all(tests)
 checks[label]={'si_error_upper':pack(err),'raw_error_upper':pack(rawerr),'predicates':tests}
# Exact diagonal exponents: source Wide exponent is exponent of leading bit.
diagx,diagr=Eh*A,Gh*J;ex,er=exponent(diagx),exponent(diagr);sx,sr=-(ex//2),-(er//2)
assert significant(diagx)<=256 and significant(diagr)<=256
Brec=case['I47_G5A_DATA']['B'];assert len(Brec)==1 and Brec[0][0]==native['body']==0
B=frombits(Brec[0][1]);delta=(G-Gh)/(G+Gh);lower=2*B*pow2(sx+sr)*delta
assert lower>sharp_exact and lower>sharp64
# Secondary claim: exact ceiling endpoints bracket every radius r in [0,sharp64].
# Tight monotone directed rounding preserves the representable x+-ceiling enclosure.
caplo,caphi=n-sharp64,n+sharp64
assert max(significant(caplo),significant(caphi),significant(sharp64))<=1024
# Recorded actual final primary scale recomputed directly from product rows (all translations/rotations).
final_translation=max(abs(rn(ff(x['value'])/1000)) for x in case['I47_ROWS'] if x['kind'] in ['global_nodal_displacement_x','global_nodal_displacement_y','global_nodal_displacement_z','displacement_magnitude'])
final_rotation=max(abs(ff(x['value'])) for x in case['I47_ROWS'] if x['kind'] in ['global_nodal_rotation_x','global_nodal_rotation_y','global_nodal_rotation_z'])
assert max(final_translation,final_rotation)==scale
results={'time_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'source_revision':rev,'selected_precision':case['I47_G5A_DATA']['precision'],'verification_precision':case['I47_G5A_DATA']['precision']*2,'source_sha256':sha(data),'material_four_products':products,'E_source':pack(E),'G_source':pack(G),'E_difference':pack(E-Eh),'G_difference':pack(G-Gh),'K_truth':pack(qk),'source_truth_lower':pack(qs[0]),'source_truth_upper':pack(qs[1]),'pi_lower':pack(pi_lo),'pi_upper':pack(pi_hi),'point_checks':checks,'raw_bits':rb(raw),'normalized_bits':rb(n),'scale_bits':rb(scale),'native_primary_bits':[rb(x) for x in primary],'projected_primary_bits':[rb(x) for x in projected],'sharp_exact':pack(sharp_exact),'sharp_binary64':pack(sharp64),'diagonal_UX':pack(diagx),'diagonal_RX':pack(diagr),'diagonal_significant_bits':[significant(diagx),significant(diagr)],'diagonal_exponents':[ex,er],'scales':[sx,sr],'B':pack(B),'minimax_residual':pack(delta),'geometric_UX_halfwidth_lower':pack(lower),'ratio_exact':pack(lower/sharp_exact),'ratio_binary64':pack(lower/sharp64),'ceiling_endpoint_significant_bits':[significant(caplo),significant(caphi)],'projection_UX':projection[6],'all_assertions_passed':True}
write('PROJECTION.json',projection);write('RESULTS.json',results);write('ORIGINS.json',origins);write('GIT_READS.json',commands)
print(json.dumps({k:results[k] for k in ['time_utc','all_assertions_passed','E_source','G_source','diagonal_exponents','scales','geometric_UX_halfwidth_lower','ratio_binary64','point_checks','ceiling_endpoint_significant_bits']},indent=2))
