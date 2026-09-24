from pathlib import Path
from fractions import Fraction as Q
from decimal import Decimal as D, localcontext
import json, math, re, hashlib, sys
base=Path(__file__).resolve().parent
raw=base/'_run_records'; events=[json.loads(s) for s in (raw/'rows.jsonl').read_text().splitlines()]
linear=[x for x in events if x['event']=='linearized']; iterations=[x for x in events if x['event']=='iteration']; final=linear[-1]; selected=iterations[-1]; model=json.loads((raw/'model.json').read_text())
assert final['u']==selected['u']; assert selected['converged'] and selected['active_count_residual']==0
u=Q(1,2**53)
def dec(q):
 with localcontext() as ctx:
  ctx.prec=120
  return str(D(q.numerator)/D(q.denominator))
def gamma(m):return m*u/(1-m*u)
def fields(dof):return {'node_index':dof//6,'node_id':model['nodes'][dof//6]['id'],'dof':['UX','UY','UZ','RX','RY','RZ'][dof%6],'unit':'N' if dof%6<3 else 'N*m'}
def residuals(k,x,f,free):
 out=[]
 for i in free:
  exact_terms=[Q.from_float(a)*Q.from_float(b) for a,b in zip(k[i],x)]; exact=sum(exact_terms,Q(0))-Q.from_float(f[i]); denominator=sum(map(abs,exact_terms),Q(0))+abs(Q.from_float(f[i]))
  total=0.; intermediates=[]
  for a,b in zip(k[i],x):
   term=a*b; total+=term; intermediates.extend([term,total])
  observed=total-f[i]; intermediates.append(observed); observed_q=Q.from_float(observed)
  nonzero_coefficients=sum(a!=0 for a in k[i]); m=2*nonzero_coefficients+2; gm=gamma(m); target=64*gm
  guarded=Q(0) if denominator==0 and observed==0 else (abs(observed_q)+gm*denominator/(1-gm))/(denominator/(1+gm))*(1+gamma(4))
  evaluation_error=abs(observed_q-exact); legacy_count=2*len(x)+1; legacy_bound=gamma(legacy_count+1)*denominator
  out.append({'global_dof':i,**fields(i),'applied':f[i],'displacement':x[i],'observed_residual':observed,'observed_residual_hex':observed.hex(),'exact_represented_residual':dec(exact),'exact_represented_denominator':dec(denominator),'actual_evaluation_error':dec(evaluation_error),'legacy_literal_operation_count':legacy_count,'legacy_conservative_roundoff_count':legacy_count+1,'legacy_evaluation_error_bound':dec(legacy_bound),'actual_error_within_bound':evaluation_error<=legacy_bound,'all_intermediates_normal_or_zero':all(v==0 or abs(v)>=sys.float_info.min for v in intermediates),'selected_nonzero_coefficient_count':nonzero_coefficients,'selected_policy_operation_count':m,'selected_gamma':dec(gm),'selected_target':dec(target),'observed_componentwise_ratio':dec(abs(observed_q)/denominator) if denominator else '0','exact_represented_componentwise_ratio':dec(abs(exact)/denominator) if denominator else '0','independent_guard_ratio_using_exact_denominator':dec(guarded),'independent_selected_screen_pass':guarded<=target,'observed_work_product':abs(observed*x[i]),'exact_represented_work_product':dec(abs(exact*Q.from_float(x[i]))),'row_derived_target_work_bound':dec(abs(Q.from_float(x[i]))*target*denominator),'nonzero_coefficient_terms':[{'column':j,'dof':['UX','UY','UZ','RX','RY','RZ'][j%6],'coefficient':a,'coefficient_hex':a.hex(),'displacement':x[j],'displacement_hex':x[j].hex(),'rounded_product':a*x[j],'exact_product':dec(exact_terms[j])} for j,a in enumerate(k[i]) if a!=0]})
 return out
rows=residuals(final['K'],final['u'],final['f'],final['free_dofs'])
for r in rows:assert r['observed_residual']==final['reactions'][r['global_dof']]
assert max(abs(r['observed_residual']) for r in rows if r['unit']=='N')==final['force_residual'];assert max(r['observed_work_product'] for r in rows)==final['work_residual']
wrong=residuals(final['K'],linear[0]['u'],final['f'],final['free_dofs'])
bc_errors=[{'global_dof':i,'candidate_initial_value':linear[0]['u'][i],'required_final_value':v,'difference':linear[0]['u'][i]-v} for i,v in zip(final['prescribed_dofs'],final['prescribed_values']) if linear[0]['u'][i]!=v]
assert any(not r['independent_selected_screen_pass'] for r in wrong) and bc_errors
# An independent tiny-load/zero-work control, not a change to the actual fixture.
tiny_k=[[1.0]]; tiny_x=[0.0]; tiny_f=[1e-12]; r=Q.from_float(tiny_f[0]); tiny_eta=Q(1); tiny_target=64*gamma(4)
assert tiny_eta>tiny_target and round(float(r)*1e6)/1e6==0 and tiny_x[0]*float(r)==0
# Independently integrated prismatic EB/axial reference using Gauss-Legendre pi.
with localcontext() as ctx:
 ctx.prec=120;a=D(1);b=D(1)/D(2).sqrt();t=D(1)/4;p=D(1)
 for _ in range(8):
  an=(a+b)/2;b=(a*b).sqrt();t-=p*(a-an)**2;a=an;p*=2
 pi=(a+b)**2/(4*t);od=D('0.168');inside=D('0.154');E=D('200e9');L=D(1);gap=D('0.00005');area=pi*(od**2-inside**2)/4;I=pi*(od**4-inside**4)/64;EI=E*I
 expected={6:D(100)*L/(E*area),7:gap,8:D(97)*L**3/(3*EI),10:-D(97)*L**2/(2*EI),11:3*gap/(2*L)}
 analytic=[]
 for i,v in expected.items():
  actual=D.from_float(final['u'][i]);err=abs((actual-v)/v);assert err<=D('1e-9');analytic.append({'global_dof':i,**fields(i),'quantity':'translation_m' if i%6<3 else 'rotation_rad','unit':'m' if i%6<3 else 'rad','expected_decimal':str(v),'observed':final['u'][i],'relative_error':str(err),'existing_relative_limit':'1e-9','pass':True})
 reference={'theory':'Prismatic small-displacement Euler-Bernoulli and axial elasticity; original synthetic input values, no real-pipe/shear/code acceptance claim','L_over_OD':str(L/od),'max_rotation_rad':max(abs(v) for i,v in expected.items() if i%6>=3).__str__(),'area_m2':str(area),'second_moment_m4':str(I),'references':analytic,'reference_force_balance':'Fx=100 N; net Fz=100-0.3*10=97 N; UY=entered gap5e-5 m; root FY=-3EI*gap/L^3, contact Y=3EI*gap/L^3-100000 N; explicit one-dimensional friction -3 N'}
summary={'actor':'/root/solver_manager','method':'Independent Python Fraction arithmetic on exact captured binary64 inputs, plus 120-digit Gauss-Legendre analytical reference; no production oracle import','entry_test':'tests::mixed_nonlinear_preview_bundle_converges_and_emits_each_support_state','mode':final['mode'],'source_ref':'_run_records/SOURCE_CONTEXT.json','raw_events_ref':'_run_records/rows.jsonl','old_protected_witness':{'actual_force_residual':final['force_residual'],'limit':0.0,'result':'FAILED; unchanged','actual_moment_residual':final['moment_residual'],'actual_work_residual':final['work_residual']},'selected_iteration':{'iteration':selected['iteration'],'converged':selected['converged'],'blocked':selected['blocked'],'active_count_residual':selected['active_count_residual'],'changed_supports':selected['changed_supports'],'states':selected['states'],'friction_dof_force':selected['friction_dof_force'],'convergence_original':selected['convergence_debug']},'rows':rows,'analytical_reference':reference,'negative_controls':{'initial_solution_used_as_final':{'rows':wrong,'final_boundary_mismatches':bc_errors,'rejects':True,'initial_count_residual':iterations[0]['active_count_residual'],'note':'First linearized solution can satisfy its own constraints; it is not the converged final contact/friction state.'},'tiny_omitted_load':{'K':tiny_k,'candidate_u':tiny_x,'true_f':tiny_f,'residual_N':-float(r),'denominator_N':float(r),'eta':'1','target':dec(tiny_target),'old_six_decimal_value':0.0,'work_Nm':0.0,'rejects':True}},'claims_limits':['The old protected zero criterion still fails; no threshold/assertion/source outcome was changed.','Nonzero exact represented residual and floating evaluation error are distinguished. The observed residual is not claimed solely an evaluation artifact.','M03 row checks are separate from positive-factor/model-fidelity/contact/convergence/cap checks; standalone row pass is not operational qualification.','The M03 bound is an operational roundoff-model screen, not a formal IEEE certificate; original represented K is not the intended physical model.','EB reference is applicable to the defined element theory; L/D near5.95 does not establish negligible shear in a real pipe.'],'raw_sha256':{n:hashlib.sha256((raw/n).read_bytes()).hexdigest() for n in ['rows.jsonl','model.json','result.json','run.log','instrumentation.patch','SOURCE_CONTEXT.json']}}
(base/'ANALYSIS.json').write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps({'governing_force_row':max((r for r in rows if r['unit']=='N'),key=lambda r:abs(r['observed_residual'])),'all_row_model_screens_pass':all(r['independent_selected_screen_pass'] for r in rows),'all_analytic_nonzero_refs_pass_1e9':True,'wrong_state_rejected':True,'tiny_load_rejected':True},indent=2))
