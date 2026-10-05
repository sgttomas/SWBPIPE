"""Records-only exact arithmetic for I49 full-readout correction and one UZ target.
Uses immutable captured data and an independently reviewed rational pi bracket.
No product import, model/solver execution, private endpoint or center reconstruction.
"""
from fractions import Fraction as F
from pathlib import Path
from decimal import Decimal, localcontext
import json,struct,datetime
P=Path(__file__).resolve().parent
D=json.loads((P/'PRIOR_ACTUAL_CASE.json').read_text());old=json.loads((P/'PRIOR_RESULTS.json').read_text());challenge=json.loads((P/'ROOT_CHECK.json').read_text())
def p2(e):return F(2**e) if e>=0 else F(1,2**(-e))
def decode(bits):
 b=int(bits,16);e=(b>>52)&2047;assert e!=2047
 return (-1 if b>>63 else 1)*F((b&((1<<52)-1))+(0 if e==0 else 1<<52))*p2(-1074 if e==0 else e-1075)
def b(f):return struct.pack('>d',f).hex()
def q(f):return decode(b(f))
def exponent(x):
 x=abs(x);e=x.numerator.bit_length()-x.denominator.bit_length()
 return e-1 if x<p2(e) else e
def rbits(x):
 if x==0:return '0000000000000000'
 sign=(1<<63) if x<0 else 0;x=abs(x);e=exponent(x);y=x/p2(max(e-52,-1074));a,r=divmod(y.numerator,y.denominator)
 if r*2>y.denominator or (r*2==y.denominator and a%2):a+=1
 if a==1<<53:a>>=1;e+=1
 bits=sign|(a if e<-1022 else ((e+1023)<<52)|(a-(1<<52)))
 return f'{bits:016x}'
def rn(x):return decode(rbits(x))
def dec(x):
 with localcontext() as ctx:
  ctx.prec=28;return str(Decimal(x.numerator)/Decimal(x.denominator))
def shown(x):return {'exact':str(x),'decimal':dec(x)}
def allowance(n,s):
 a=rn(rn(p2(-64)*max(abs(n),s))*(1+p2(-21)));u=rn(rn(p2(-53)*abs(n))+p2(-1074))
 return (p2(-64)+p2(-85))*max(abs(n),s)+p2(-53)*abs(n)+p2(-1074),rn(a+u)
rows=D['I47_ROWS'];native=D['I47_NATIVE_ROWS'];v=D['I47_SELECTION']['values'];inp=D['I47_INPUT'];model=D['I47_REQUEST']['model']
assert D['I47_CASE']=='interpolated-loaded'
assert len(model['nodes'])==2 and [r['position'] for r in model['nodes']]==[dict(x=0.0,y=0.0,z=0.0),dict(x=1.0,y=0.0,z=0.0)]
assert len(model['pipe_segments'])==1 and model['pipe_segments'][0]['y_reference']==dict(x=0.0,y=1.0,z=0.0)
assert len(model['supports'])==1 and model['supports'][0]['family']=='anchor' and model['supports'][0]['node']=='node:N-DEC092-ANCHOR' and model['supports'][0]['restraints']==['UX','UY','UZ','RX','RY','RZ']
assert [(l['direction'],q(l['magnitude']['value'])) for l in model['load_cases'][0]['primitive_loads']]==[('UX',1),('UY',1),('RX',1)]
for c in ['ux','uy','uz','rx','ry','rz']:
 ar=next(r for r in rows if r['id']=='result:disp:node-N-DEC092-ANCHOR:'+c);assert q(ar['value'])==0
Tlo,T,Thi=[decode(v[k]) for k in ['t_lo','t','t_hi']];assert (Tlo,T,Thi)==(293,303,313)
props={}
for c in ['e','g']:
 lo,hi,hat=[decode(v[c+s]) for s in ['_lo','_hi','_hat']]
 exact=(Thi*lo-T*lo+T*hi-Tlo*hi)/(Thi-Tlo)
 assert 0<hat<exact
 props[c]={'exact':exact,'hat':hat}
Es,Eh=props['e']['exact'],props['e']['hat'];Gs,Gh=props['g']['exact'],props['g']['hat']
# Actual UX identity, unchanged comparator and required source-geometry/resolved-E readout.
ux_i,ux=next((i,r) for i,r in enumerate(rows) if r['id']=='result:disp:node-N-DEC092-TIP:ux');uv=D['I47_VERDICTS'][ux_i]
assert ux_i==uv['row'];n=rn(q(ux['value'])/1000);s=decode(uv['scale_bits']);ae,ab=allowance(n,s)
assert rbits(n)==uv['normalized_bits'];assert ae==F(challenge['allowance_exact_m'])
prior_source=[F(z) for z in old['independent_truth']['source_UX_interval']];scaled=[z*Es/Eh for z in prior_source]
pi_lo,pi_hi=[F(z) for z in old['independent_truth']['pi_interval']];D0,t=q(inp['D']),q(inp['t']);p=t*(D0-t)
direct=[1/(Eh*p*pi_hi),1/(Eh*p*pi_lo)]
assert direct==scaled and scaled==[F(z) for z in challenge['q_interval']]
assert scaled[0]>n
elo,ehi=scaled[0]-n,scaled[1]-n
assert elo==F(challenge['error_lower_m']) and ehi==F(challenge['error_upper_m'])
assert elo>ae and elo>ab
# Recompute the one target's identity, physical scalar subsystem and actual radix exponent.
uz_i,uz=next((i,r) for i,r in enumerate(rows) if r['id']=='result:disp:node-N-DEC092-TIP:uz');zv=D['I47_VERDICTS'][uz_i]
nz=next(r for r in native if r['quantity']=='Displacement(Dof { node: 1, component: Uz })')
assert uz_i==zv['row'] and uz['unit']=='mm' and uz['basis_ref']['ref_id']=='case:i45-loaded'
assert b(uz['value'])==zv['normalized_bits']==nz['value_bits']=='0000000000000000'
assert nz['class']=='AbsoluteVerified { bound_bits: '+str(int(zv['bound_bits'],16))+' }'
assert nz['body']==0 and nz['kind']==0
assert zv['class']=='absolute' and zv['failed']=='Absolute' and zv['predicates']==[False,None,None,None]
# B_yz from source B/D patterns: rows4/5 versus free columns UZ,RY is [[1,0],[1,1]].
Bmat=[[F(1),F(0)],[F(1),F(1)]];Dmat=[[F(4),F(2)],[F(2),F(4)]]
M=[[sum(Bmat[a][i]*Dmat[a][bb]*Bmat[bb][j] for a in range(2) for bb in range(2)) for j in range(2)] for i in range(2)]
assert M==[[12,6],[6,4]];det=M[0][0]*M[1][1]-M[0][1]*M[1][0];assert det==12 and M[0][0]>0
# Therefore EI*M is positive definite for every positive EI in K and the full required source coefficient cover;
# its UZ/RY load is (0,0), all other modal blocks are uncoupled by the exact aligned frame: unique UZ=RY=0.
EI=Eh*q(inp['I']);diagUZ=12*EI;diagRX=Gh*q(inp['J']);ez=exponent(diagUZ);er=exponent(diagRX);sz=-(ez//2);sr=-(er//2)
assert (ez,er,sz,sr)==(21,17,-10,-8)
B=decode(D['I47_G5A_DATA']['B'][0][1]);minimax=(Gs-Gh)/(Gs+Gh);forced=2*B*p2(sz+sr)*minimax
assert forced==16*F(old['forced_geometric_lower_bound']['UX_half_width_lower_bound']['exact'])
zscale=decode(zv['scale_bits']);bound=decode(zv['bound_bits']);assert bound==rn(zscale*p2(-64)) and bound>0
assert F(0)<rn(zscale*p2(-34));assert forced>bound
# Algebraic projection of native primary values only; actual producer is not executed.
maxima=[F(0)]*4
for r in native:
 value=decode(r['value_bits']);raw=rn(value*1000) if r['kind']==0 else value;norm=rn(raw/1000) if r['kind']==0 else raw
 if r['class']!='InputDerived':maxima[r['kind']]=max(maxima[r['kind']],abs(norm))
projscale=max(maxima[0],maxima[1]);projraw=rn(decode(nz['value_bits'])*1000);projn=rn(projraw/1000);projbound=rn(projscale*p2(-64))
assert projscale==zscale and projbound==bound and projraw==projn==0
R={'completed_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'case':'interpolated-loaded','method':'records-only exact arithmetic and source-warrant check; no runtime or private endpoint reconstruction',
'materials':{'E_s':str(Es),'E_hat':str(Eh),'G_s':str(Gs),'G_hat':str(Gh),'both_hulls_positive':True},
'UX_required_resolved_material_source_geometry':{'row':ux_i,'id':ux['id'],'normalized_bits':rbits(n),'scale_bits':uv['scale_bits'],'exact_scale_ratio_Es_over_Ehat':str(Es/Eh),'interval':[str(z) for z in scaled],'error_lower':shown(elo),'error_upper':shown(ehi),'sharper_exact_allowance':shown(ae),'sharper_binary64_allowance':shown(ab),'lower_over_exact_allowance':shown(elo/ae),'fails_both_sharper':True,'matches_ROOT_exact_fractions':True,'source_warrant':'I36 preview truth RETURN section2; ordinary coefficients RETURN section1 step6 and section2 explicitly cover resolved E_hat/G_hat with source geometry','consequence':'Original two point comparisons and geometric lower-bound proof stand, but unchanged UX has a real miss under the full required cover; tighter certification alone cannot make it pass.'},
'UZ_genuinely_conservative_target':{'row':uz_i,'id':uz['id'],'native_ordinal':nz['ordinal'],'native_quantity':nz['quantity'],'raw_bits':b(uz['value']),'normalized_bits':zv['normalized_bits'],'scale_bits':zv['scale_bits'],'class':zv['class'],'actual_candidate_predicates':zv['predicates'],'actual_bound_bits':zv['bound_bits'],'actual_bound':shown(bound),'homogeneous_subsystem_matrix_factor_EI':[[str(z) for z in a] for a in M],'positive_leading_minor':str(M[0][0]),'positive_determinant_factor_EI_squared':str(det),'forcing':['0','0'],'full_positive_required_cover_UZ_truth':'0','truth_error':'0','true_predicate_pass':True,'represented_EI':shown(EI),'represented_UZ_diagonal':shown(diagUZ),'diagonal_exponent':ez,'radix_exponent_UZ':sz,'radix_exponent_RX':sr,'shared_radius_UZ_lower_bound':shown(forced),'lower_bound_over_absolute_allowance':shown(forced/bound),'forced_refusal':True,'native_projection_raw_bits':rbits(projraw),'native_projection_normalized_bits':rbits(projn),'native_projection_scale_bits':rbits(projscale),'native_projection_bound_bits':rbits(projbound),'native_projection_unchanged':True},
'limits':['Only UX and one UZ target considered; no all-row truth survey.','No new material interpretation, narrowing, algorithm, source or output selected.','No actual private correction center, endpoints or native radius reconstructed.','No complete producer, routing, availability, resource, acceptance or release qualification.','Original sealed evidence remains unchanged; RV64 independent backcheck outstanding.']}
(P/'RESULTS.json').write_text(json.dumps(R,indent=2)+'\n')
print(json.dumps({'UX':{'error_lower':dec(elo),'allowance':dec(ae),'ratio':dec(elo/ae),'ROOT_exact_match':True},'UZ':{'row_by_identity':uz_i,'native_ordinal':nz['ordinal'],'truth':'0 over full positive cover','diagonal':dec(diagUZ),'exponent':ez,'radix_exponent':sz,'actual_bound':dec(bound),'forced_radius_lower':dec(forced),'ratio':dec(forced/bound),'native_projection_unchanged':True}},indent=2))
