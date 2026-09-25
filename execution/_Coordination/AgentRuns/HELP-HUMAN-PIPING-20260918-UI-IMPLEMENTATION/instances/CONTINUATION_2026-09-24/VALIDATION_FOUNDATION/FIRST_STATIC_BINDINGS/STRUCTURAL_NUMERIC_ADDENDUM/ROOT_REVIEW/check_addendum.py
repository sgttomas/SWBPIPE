from decimal import Decimal as D, localcontext
from pathlib import Path
import hashlib, json, math, subprocess
review=Path(__file__).resolve().parent
add=review.parent
parent=add.parent
repo=next(p for p in review.parents if (p/'AGENTS.md').exists() and (p/'projects/chirality-piping').exists())
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
binding=add/'STRUCTURAL_NUMERIC_BINDING.json'
assert sha(binding)=='1ed8aac0910318902cd5f690aeee8cb74bfee036617e33eaa4e2c23ec98ff3c2'
m=json.loads(binding.read_text())
for row in json.loads((add/'MANIFEST.json').read_text())['files']:
 p=add/row['path'];assert sha(p)==row['sha256'] and p.stat().st_size==row['bytes']
originals=json.loads((parent/'PACKET_MANIFEST.json').read_text())['files']
assert len(originals)==25
for row in originals:
 p=parent/row['path'];raw=p.read_bytes();assert sha(p)==row['sha256']
 committed=subprocess.check_output(['git','show','74bd1bac253a9d58bac095efd8b8b888a3d351cd:'+str(p.relative_to(repo))],cwd=repo)
 assert raw==committed
# Independent AGM pi and circular-sector integrations; no author or product import.
with localcontext() as ctx:
 ctx.prec=135
 a=D(1);b=D(1)/D(2).sqrt();t=D(1)/4;mult=D(1)
 for _ in range(10):
  an=(a+b)/2;b=(a*b).sqrt();t-=mult*(a-an)**2;a=an;mult*=2
 pi=(a+b)**2/(4*t)
 ro=D(1)/10;ri=D(9)/100
 # Areas integrate r dr dtheta; principal inertia integrates r^3 dr sin^2(theta) dtheta.
 wall=2*pi*(ro**2-ri**2)/2;bore=pi*ri**2
 inertia=pi*(ro**4-ri**4)/4
 expected={'outside_diameter_m':D('0.2'),'effective_wall_thickness_m':D('0.01'),'ro_m':ro,'ri_m':ri,'As_m2':wall,'Ai_m2':bore,'I_m4':inertia,'J_m4':2*inertia,'Z_m3':inertia/ro}
 units={'outside_diameter_m':('length','m'),'effective_wall_thickness_m':('length','m'),'ro_m':('length','m'),'ri_m':('length','m'),'As_m2':('area','m^2'),'Ai_m2':('area','m^2'),'I_m4':('second_moment_area','m^4'),'J_m4':('second_moment_area','m^4'),'Z_m3':('section_modulus','m^3')}
 assert len(m['quantities'])==9 and {q['observed_field'] for q in m['quantities']}==set(expected)
 checks=[]
 for q in m['quantities']:
  key=q['observed_field'];value=expected[key];target=D(q['expected_decimal']);c=m['criteria'][q['criterion_ref']]
  assert abs(target-value)/abs(value)<D('1e-97')
  assert q['reference_value_binary64']==float(value)
  assert (q['dimension'],q['unit'])==units[key] and (c['dimension'],c['unit'])==units[key]
  exact=q['bound_input_pointer_for_exact_identity'] is not None
  assert c['absolute_tolerance']==0 and c['relative_tolerance']==(0 if exact else 1e-9)
  assert c['kind']==('exact_same_unit_input_identity' if exact else 'same_unit_relative')
  def passes(x):
   if isinstance(x,bool) or not isinstance(x,(int,float)) or not math.isfinite(x) or x<=0:return False
   ref=q['reference_value_binary64']
   return x==ref if exact else abs(x-ref)<=1e-9*max(abs(x),abs(ref))
  assert passes(float(value))
  assert all(not passes(x) for x in [None,True,str(float(value)),0,-float(value),float('nan'),float('inf'),float(value)*1.000001])
  assert not passes(math.nextafter(float(value),math.inf)) if exact else passes(math.nextafter(float(value),math.inf))
  checks.append({'field':key,'independent_decimal':str(value),'expected_binary64':float(value),'criterion':q['criterion_ref'],'independent_positive_and_negative_controls':'pass'})
assert len(m['applies_to'])==2
for case in m['applies_to']:
 p=add/case['input']['path'];assert sha(p)==case['input']['sha256'];r=json.loads(p.read_text());model=r['solve']['preview_model']['model']
 assert model['project']['id']==case['model_ref'] and case['modes']==['sparse_interactive','dense_scrutiny']
 assert len(model['pipe_segments'])==1 and model['pipe_segments'][0]['id']=='pipe'
 section=model['pipe_segments'][0]['section'];assert section=={'outside_diameter':{'value':0.2,'unit':'m'},'wall_thickness':{'value':0.01,'unit':'m'}}
 assert [x['id'] for x in model['load_cases']]==['case']
 assert set(case['quantity_ids'])=={q['quantity_id'] for q in m['quantities']}
 for q in m['quantities']:
  pointer=q['bound_input_pointer_for_exact_identity']
  if pointer:
   item=r
   for part in pointer.strip('/').split('/'):item=item[int(part)] if isinstance(item,list) else item[part]
   assert item==q['reference_value_binary64']
assert m['parent_obligation']=='complete_case_material_section'
assert m['selection']=={'case_key':'load_case_id','case_value':'case','case_match_count':1,'nested_array':'pipe_sections','pipe_key':'pipe_id','pipe_value':'pipe','pipe_match_count':1,'geometry_basis':'authored_normalized_od_wall_v1'}
result={'status':'clear_for_bounded_development_binding','reviewer':'/root','role':'HELP_HUMAN','nonimplementation':'Root did not author the addendum or generator; independent derivation and checker do not import them or product code','binding_sha256':sha(binding),'manifest_sha256':sha(add/'MANIFEST.json'),'checker_sha256':sha(Path(__file__)),'original25_unchanged_at':'74bd1bac253a9d58bac095efd8b8b888a3d351cd','arithmetic':'Decimal135 AGM pi; sector area and second-moment integrations','checks':checks,'coverage':{'cases':2,'modes_each':2,'section_subchecks_per_run':9,'section_subchecks_total':36,'existing_scalars_total_unchanged':292,'existing_structural_obligations_total_unchanged':40},'actual_solver_runs':0,'limits':['Adapter implementation and its review remain required','No product imports, reference generator execution, solver/native/build/network or source edits','No global or release tolerance and no coefficient-enclosure epsilon added']}
(review/'CHECKS.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({'status':result['status'],'fields':len(checks),'originals':len(originals),'cases':len(m['applies_to']),'actual_solver_runs':0}))
