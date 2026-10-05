#!/usr/bin/env python3
"""Records-only exact backcheck. Run only this new owned script, never original packets."""
from pathlib import Path
from fractions import Fraction as F
import runpy, json, hashlib, datetime, re, contextlib, io
W=Path.cwd();R=Path('projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30')
O=W/R/'REVIEW_RV64/full_truth_addendum_02';D=O/'_run_records'
sha=lambda b:hashlib.sha256(b).hexdigest()
# Inventory snapshots before running our own copy of the independent prior checker.
original_roots=[W/R/x for x in ['I49/conservative_predicate_01','I49/full_truth_addendum_02','REVIEW_RV64/conservative_predicate_01']]
def snapshot():return [{'path':str(f.relative_to(W)),'sha256':sha(f.read_bytes()),'bytes':f.stat().st_size} for p in original_roots for f in sorted(p.rglob('*')) if f.is_file()]
before=snapshot();(D/'ORIGINALS_BEFORE.json').write_text(json.dumps(before,indent=2)+'\n')
with contextlib.redirect_stdout(io.StringIO()):
 a=runpy.run_path(str(D/'independent_base.py'))
write,git,record=a['write'],a['git'],a['record'];pow2,pack,ff,rb,rn,frombits,exponent,significant=(a[k] for k in ['pow2','pack','ff','rb','rn','frombits','exponent','significant'])
extra_origins=[];extra_git=[]
def pin(path,role,revision='4987f8290fbdd8b2ca228da24c138a1b90453eed',copy=None):
 p=W/path;b=p.read_bytes();oldn=len(a['commands']);assert b==git(W,['show',revision+':'+str(path)]);extra_git.extend(a['commands'][oldn:]);extra_origins.append({'path':str(p),'sha256':sha(b),'role':role,'revision':revision})
 return b
pin(R/'BRIEFS/RV64_FULL_TRUTH_BACKCHECK.md','active continuation brief',copy='BRIEF.md')
pin(R/'verification/material_truth_coverage_challenge_01/CHALLENGE.md','ROOT challenge',copy='ROOT_CHALLENGE.md')
pin(R/'I36/f2a_preview_truth_01/RETURN.md','required preview truth warrant, especially section 2',copy='I36_PREVIEW_RETURN.md')
pin(R/'I36/f2a_ordinary_coefficients_02/RETURN.md','required coefficient cover, section 1 step 6 and section 2',copy='I36_COEFFICIENTS_RETURN.md')
subject=W/R/'I49/full_truth_addendum_02';subj=[]
for name,expected in [('RETURN.md','edfd6c25fa515158b932311d89898c828db9aad1700e0bcf6324d29406106fa0'),('_run_records/INVENTORY.json','1c7bf58603167c45ec20f35fb90879dafdc921f12141fa6ae59890ce35c605c7')]:
 b=(subject/name).read_bytes();assert sha(b)==expected;extra_origins.append({'path':str(subject/name),'sha256':sha(b),'role':'new subject identity'})
for e in json.loads((subject/'_run_records/INVENTORY.json').read_text()):
 b=(subject/e['path']).read_bytes();assert sha(b)==e['sha256'] and len(b)==e['bytes'];subj.append(e)
assert len(subj)==26
write('SUBJECT_ADDENDUM_PAYLOAD_CHECKS.json',subj)
# Check prior independent reviewer inventory too, without writing its files.
p=W/R/'REVIEW_RV64/conservative_predicate_01'
assert sha((p/'RETURN.md').read_bytes())=='52e15bcf18f5ca7caba217368350082df855533fbfaa6fcd770b5ff1e7608f31'
for e in json.loads((p/'_run_records/INVENTORY.json').read_text()):assert sha((p/e['path']).read_bytes())==e['sha256']
# Re-derive additional UX readout directly from fresh captured operands and independent pi.
case=a['case'];Eh,E,Gh,G,diam,wall,n,scale=(a[k] for k in ['Eh','E','Gh','G','diam','wall','n','scale'])
atan=a['atan_bracket'];xlo,xhi=atan(2,n=700);ylo,yhi=atan(3,n=700);pi_lo,pi_hi=4*(xlo+ylo),4*(xhi+yhi)
resolved=(1/(Eh*pi_hi*wall*(diam-wall)),1/(Eh*pi_lo*wall*(diam-wall)))
exact_source=(1/(E*pi_hi*wall*(diam-wall)),1/(E*pi_lo*wall*(diam-wall)))
assert resolved==tuple(x*E/Eh for x in exact_source)
assert n<resolved[0]
errlo,errhi=resolved[0]-n,resolved[1]-n
assert errlo>a['sharp_exact'] and errlo>a['sharp64']
# Independent full-free symbolic B^T D B expansion, derived from frozen b()/D_PATTERN.
# Each polynomial is a 4-vector of coefficients of positive (EA,GJ,EI_z,EI_y), L=1.
source=(W/R/'I49/conservative_predicate_01/_run_records/SOURCE_source_residual.rs').read_text()
block=source.split('const D_PATTERN:')[1].split('];',1)[0]
pattern=[tuple(map(int,m)) for m in re.findall(r'\((\d+),\s*(\d+),\s*(\d+),\s*(\d+)\)',block)]
assert len(pattern)==10
ex,ey,ez=[1,0,0],[0,1,0],[0,0,1]
def Bentry(r,j):
 k=j%3;atj=j>=6;rot=j%6>=3
 if r==0 and not rot:return ex[k]*(1 if atj else -1)
 if r==1 and rot:return ex[k]*(1 if atj else -1)
 if r in [2,3] and not rot:return ey[k]*(-1 if atj else 1)
 if r in [4,5] and not rot:return ez[k]*(1 if atj else -1)
 if r==2 and rot and not atj:return ez[k]
 if r==3 and rot and atj:return ez[k]
 if r==4 and rot and not atj:return ey[k]
 if r==5 and rot and atj:return ey[k]
 return 0
Bfull=[[Bentry(r,j) for j in range(12)] for r in range(6)]
K=[[[sum(Bfull[r][i+6]*(2**k)*Bfull[c][j+6] for r,c,p,k in pattern if p==which) for which in range(4)] for j in range(6)] for i in range(6)]
# Matrices, not observed output zeros, establish invariant homogeneous subsystem.
indices=[2,4];other=[0,1,3,5]
assert all(K[i][j]==[0,0,0,0] for i in indices for j in other)
assert all(K[j][i]==[0,0,0,0] for i in indices for j in other)
M=[[K[i][j][3] for j in indices] for i in indices]
assert M==[[12,6],[6,4]]
assert all(K[i][j][:3]==[0,0,0] for i in indices for j in indices)
determinant=M[0][0]*M[1][1]-M[0][1]*M[1][0];assert M[0][0]>0 and determinant>0
forcing=[0]*6
for load in a['decoded']['loads']:
 assert load['dof'][0]==1
 forcing[load['dof'][1]]+=load['value']
assert forcing==[1,1,0,1,0,0] and all(forcing[i]==0 for i in indices)
assert all(c['value']==0 for c in a['decoded']['constraints'])
# A symbolic discriminating control: nonzero forcing in UZ would give nonzero solution.
assert F(M[1][1],determinant)==F(1,3)
# Actual row is discovered by identity; class and bound are not inferred from displayed zero.
uz_index=next(i for i,x in enumerate(case['I47_ROWS']) if x['id']=='result:disp:node-N-DEC092-TIP:uz')
ur=case['I47_ROWS'][uz_index];uv=next(x for x in case['I47_VERDICTS'] if x['row']==uz_index)
un=next(x for x in case['I47_NATIVE_ROWS'] if x['quantity']=='Displacement(Dof { node: 1, component: Uz })')
assert ur['kind']=='global_nodal_displacement_z' and ur['unit']=='mm' and ur['basis_ref']['ref_id']==case['I47_SELECTION']['case']
assert rb(ff(ur['value']))==uv['normalized_bits']==un['value_bits']=='0000000000000000'
assert un['body']==0 and un['kind']==0 and un['class'].startswith('AbsoluteVerified')
assert uv['class']=='absolute' and uv['predicates']==[False,None,None,None] and uv['failed']=='Absolute'
assert uv['scale_bits']==rb(scale)
threshold=rn(frombits('3dd0000000000000')*scale);assert threshold>0
assert frombits(uv['normalized_bits'])<threshold
allowance=rn(scale/pow2(64));assert allowance*pow2(64)==scale # no next-up needed
assert rb(allowance)==uv['bound_bits']
assert int(uv['bound_bits'],16)==int(re.search(r'bound_bits: (\d+)',un['class']).group(1))
assert F(0)<=allowance
Ihat=ff(a['input']['I']);diag=12*Eh*Ihat;ezdiag=exponent(diag);sz=-(ezdiag//2)
assert significant(diag)<=256
Bbound=a['B'];delta=(G-Gh)/(G+Gh);rx_scale=a['sr'];Lz=2*Bbound*pow2(sz+rx_scale)*delta
assert Lz>allowance and Lz/a['lower']==pow2(sz-a['sx'])
proj=next(x for x in a['projection'] if x['native']==un['ordinal'])
assert proj['raw_bits']==proj['normalized_bits']=='0000000000000000'
assert a['projected_scale']==scale and rb(rn(a['projected_scale']/pow2(64)))==uv['bound_bits']
# Only now compare the new author/ROOT arithmetic; the independent proof above uses neither.
author=json.loads((subject/'_run_records/RESULTS.json').read_text())
root=json.loads(pin(R/'verification/material_truth_coverage_challenge_01/ROOT_CHECK.json','post-derivation ROOT fraction comparison',copy='ROOT_CHECK.json'))
prior_author=json.loads((W/R/'I49/conservative_predicate_01/_run_records/RESULTS.json').read_text())
old_q=tuple(map(F,prior_author['independent_truth']['source_UX_interval']))
reported_q=tuple(x*E/Eh for x in old_q)
assert reported_q==tuple(map(F,root['q_interval']))==tuple(map(F,author['UX_required_resolved_material_source_geometry']['interval']))
# The independent much tighter true bracket lies within the reported rational enclosure.
assert reported_q[0]<=resolved[0]<=resolved[1]<=reported_q[1]
reported_error=(reported_q[0]-n,reported_q[1]-n)
assert reported_error==tuple(F(root[k]) for k in ['error_lower_m','error_upper_m'])
assert reported_error[0]==F(author['UX_required_resolved_material_source_geometry']['error_lower']['exact'])
assert reported_error[0]>a['sharp_exact'] and reported_error[0]>a['sharp64']
assert F(author['UZ_genuinely_conservative_target']['represented_UZ_diagonal']['exact'])==diag
assert F(author['UZ_genuinely_conservative_target']['shared_radius_UZ_lower_bound']['exact'])==Lz
assert F(author['UZ_genuinely_conservative_target']['actual_bound']['exact'])==allowance
# Validate pi containment directly by mapping the published source response bracket back to pi.
old_pi=(1/(E*wall*(diam-wall)*old_q[1]),1/(E*wall*(diam-wall)*old_q[0]))
assert old_pi[0]<=pi_lo<=pi_hi<=old_pi[1]
results={'completed_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'all_checks_passed':True,'pi_method':'pi=4(atan(1/2)+atan(1/3)), 700 rational alternating-series terms each with next-term remainder bounds','UX':{'row':10,'native':6,'resolved_E_source_annulus_bracket':[pack(x) for x in resolved],'error_lower':pack(errlo),'error_upper':pack(errhi),'sharp_exact':pack(a['sharp_exact']),'sharp_binary64':pack(a['sharp64']),'ratio_lower_exact':pack(errlo/a['sharp_exact']),'fails_both':True,'author_ROOT_reported_bracket':[str(x) for x in reported_q],'author_ROOT_reported_error_bounds':[str(x) for x in reported_error],'author_ROOT_exact_fraction_identity_checked':True,'independent_pi_bracket_nested_inside_author_bracket':True,'planning_correction':'Tightening alone cannot admit unchanged UX across the already-required cover.'},'UZ':{'row':uz_index,'native':un['ordinal'],'actual_verdict':uv,'actual_native':un,'truth_full_positive_cover':'0','error':'0','free_B':[r[6:] for r in Bfull],'D_pattern':pattern,'symbolic_free_K_EA_GJ_EIz_EIy':K,'subsystem_UZ_RY':M,'leading_minor_factor':M[0][0],'determinant_factor':determinant,'forcing':forcing,'nonzero_force_discriminator':'For UZ RHS=1 and RY RHS=0, UZ=1/(3*C4), not zero. Actual RHS is zero.','diagonal':pack(diag),'diagonal_significant_bits':significant(diag),'diagonal_exponent':ezdiag,'radix_scale':sz,'RX_radix_scale':rx_scale,'B':pack(Bbound),'minimax_residual':pack(delta),'halfwidth_lower':pack(Lz),'absolute_allowance':pack(allowance),'ratio_lower_to_allowance':pack(Lz/allowance),'halfwidth_ratio_to_UX':str(Lz/a['lower']),'projection':proj,'projected_scale_bits':rb(a['projected_scale']),'projected_absolute_bits':rb(rn(a['projected_scale']/pow2(64))),'full_cover_conservative_refusal':True},'limits':['No full-row survey, runtime execution, new algorithm or acceptance.','No private endpoints, correction center or actual native radius reconstructed.','Original two point checks and geometric lower bound retained at narrower scope.']}
write('RESULTS.json',results);write('ADDENDUM_ORIGINS.json',extra_origins);write('ADDENDUM_GIT_READS.json',extra_git)
after=snapshot();assert before==after;write('ORIGINALS_AFTER.json',{'file_count':len(after),'equals_before':True,'before_manifest_sha256':sha((D/'ORIGINALS_BEFORE.json').read_bytes())})
write('PRESERVATION_CHECK.json',{'checked_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'file_count':len(before),'before_equals_after':True})
print(json.dumps({'all_checks_passed':True,'UX_error_lower':pack(errlo)['decimal'],'UX_ratio':pack(errlo/a['sharp_exact'])['decimal'],'UZ_diagonal':pack(diag)['decimal'],'UZ_exponent':ezdiag,'UZ_scale':sz,'UZ_halfwidth':pack(Lz)['decimal'],'UZ_allowance':pack(allowance)['decimal'],'UZ_ratio':pack(Lz/allowance)['decimal'],'preserved_original_file_count':len(before)},indent=2))
