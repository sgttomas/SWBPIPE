"""Independent source-traced arithmetic decomposition, not a product matrix capture."""
from pathlib import Path
from decimal import Decimal as D, localcontext
from fractions import Fraction as F
import json, re
HERE=Path(__file__).resolve().parent
P=Path.cwd()/'projects/chirality-piping'
rows=json.loads((HERE.parent/'_run_records/SELECTED.json').read_text())
refs=json.loads((P/'validation/benchmarks/numerical_integrity/fixtures.json').read_text())
def dec(x):return D(x.numerator)/D(x.denominator)
def ff(x):return F.from_float(float(x))
output=[]
with localcontext() as c:
 c.prec=100
 for r in rows:
  if r['case']!='N05':continue
  o=r['observed'];d=next(x['message'] for x in o['diagnostics'] if x['code']=='NUMERICAL_INTEGRITY_SENSITIVE')
  audit=re.search(r'ContributionRounding \{ row: 3, col: 3,.*?accumulated_expansion: \[(.*?)\], difference_expansion: \[(.*?)\]',d)
  terms=[ff(x.strip()) for x in audit.group(1).split(',')];difference=sum((ff(x.strip()) for x in audit.group(2).split(',')),F())
  k=ff(r['input']['supports'][1]['stiffness']['value']['value']);t=ff(r['input']['load_cases'][0]['primitive_loads'][0]['magnitude']['value'])
  intended_diagonal=sum(terms,F());stored_diagonal=intended_diagonal-difference;a=intended_diagonal-k
  assert F.from_float(float(a))==a
  stored_increment=stored_diagonal-a
  assert stored_increment>0
  stored_root=t/stored_increment;contribution_root=t/k
  actual=ff(next(x['value'] for x in o['results'] if x['id']=='result:disp:root:rx'))
  actual_tip=ff(next(x['value'] for x in o['results'] if x['id']=='result:disp:tip:rx'))
  intended_root=D(refs['N']['N05']['theta_root_rad'])
  torques=[{'id':x['id'],'value':x['value'],'reference':('-1e-8' if x['id']=='result:moment:pipe:torsion' else '1e-8'),'relative_error':str((D.from_float(x['value'])-D('-1e-8' if x['id']=='result:moment:pipe:torsion' else '1e-8'))/D('-1e-8' if x['id']=='result:moment:pipe:torsion' else '1e-8'))} for x in o['results'] if x['kind']=='element_local_torsional_moment']
  output.append({'mode':r['mode'],'inference_basis':'Input has one x-aligned straight frame and one root-RX spring. frame_kernel local torsion block uses [[a,-a],[-a,a]]; product assembled root diagonal audit is two exact expansion terms, with difference from stored diagonal. Inferred a = audited intended diagonal minus exact represented input spring. This source-traced reconstruction is not a retained full product stiffness matrix.','inferred_product_a_binary64_hex':float(a).hex(),'inferred_product_a_exact':str(dec(a)),'np_a_a_binary64_hex':refs['NP']['A'][0]['a_binary64_hex'],'product_a_relative_difference_from_exact_analytical':str((dec(a)-D(refs['N']['N05']['a_Nm_per_rad']))/D(refs['N']['N05']['a_Nm_per_rad'])),'stored_diagonal_exact':str(dec(stored_diagonal)),'contribution_difference_exact':str(dec(difference)),'stored_increment_exact':str(dec(stored_increment)),'stored_root_exact':str(dec(stored_root)),'contribution_root_exact':str(dec(contribution_root)),'source_intended_root':str(intended_root),'assembly_root_bias_relative_to_contribution_solution':str(dec((stored_root-contribution_root)/contribution_root)),'published_root_solver_error_relative_to_stored_solution':str(dec((actual-stored_root)/stored_root)),'published_root_total_error_relative_to_analytical':str((dec(actual)-intended_root)/intended_root),'published_root_spring_action_relative_error':str((dec(-k*actual)-D('-1e-8'))/D('-1e-8')),'published_member_torques':torques})
(HERE/'ARITHMETIC_DECOMPOSITION.json').write_text(json.dumps(output,indent=2)+'\n')
print(json.dumps([{k:r[k] for k in ('mode','inferred_product_a_binary64_hex','stored_increment_exact','assembly_root_bias_relative_to_contribution_solution','published_root_solver_error_relative_to_stored_solution','published_root_total_error_relative_to_analytical','published_member_torques')} for r in output],indent=2))
