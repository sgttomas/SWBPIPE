#!/usr/bin/env python3
"""Additive geometry expectation/criterion preparation; never runs product/solver.
The checked original Decimal100 annulus and pi are the only mathematical source.
"""
from pathlib import Path
from decimal import Decimal as D, localcontext
import json, hashlib, subprocess, sys
HERE=Path(__file__).parent
BASE=HERE.parent
REPO=Path('/Users/ryan/.codex/worktrees/6614/chirality')
PREFIX='projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/VALIDATION_FOUNDATION/FIRST_STATIC_BINDINGS/'
CHECKPOINT='74bd1bac253a9d58bac095efd8b8b888a3d351cd'
def digest(b):return hashlib.sha256(b).hexdigest()
def read(p):return json.loads(p.read_text())
def write(name,obj):(HERE/name).write_text(json.dumps(obj,indent=2)+'\n')

original=read(BASE/'PACKET_MANIFEST.json')
assert len(original['files'])==25
before=[]
for entry in original['files']:
    b=(BASE/entry['path']).read_bytes()
    assert digest(b)==entry['sha256'],entry['path']
    committed=subprocess.check_output(['git','show',CHECKPOINT+':'+PREFIX+entry['path']],cwd=REPO)
    assert committed==b,entry['path']
    before.append({'path':entry['path'],'sha256':digest(b),'matches_original_manifest_and_checkpoint':True})
source=read(BASE/'section.reference.json');v=source['values']
with localcontext() as ctx:
    ctx.prec=100
    pi=D(v['pi']);ro=D(v['ro_m']);ri=D(v['ri_m']);od=2*ro;wall=ro-ri
    expected={'outside_diameter_m':od,'effective_wall_thickness_m':wall,'ro_m':ro,'ri_m':ri,
              'As_m2':D(v['A_m2']),'Ai_m2':pi*ri**2,'I_m4':D(v['I_m4']),
              'J_m4':D(v['J_m4']),'Z_m3':D(v['Z_m3'])}
    # Mathematical cross-check against the same already reviewed source relations;
    # this is not an independent second oracle or a mechanics tolerance.
    recomputed={'As_m2':pi*(ro**2-ri**2),'I_m4':pi*(ro**4-ri**4)/4,
                'J_m4':pi*(ro**4-ri**4)/2,'Z_m3':pi*(ro**4-ri**4)/(4*ro)}
    for key, value in recomputed.items():assert abs((value-expected[key])/expected[key])<D('1e-96')
    assert expected['Ai_m2']>0 and wall==D('.01') and od==D('.20')

criteria={
 'section.source_length_identity':{'kind':'exact_same_unit_input_identity','unit':'m','dimension':'length','relative_tolerance':0,'absolute_tolerance':0,'predicate':'finite observed numeric value equals the finite numeric value decoded from the specified bound authored input; reject bool/string/nonfinite/missing','scope':'OD and effective wall only, for these inputs already authored in m with no mill-tolerance reduction'},
 'section.derived_length_relative':{'kind':'same_unit_relative','unit':'m','dimension':'length','relative_tolerance':1e-9,'absolute_tolerance':0,'predicate':'abs(observed-reference)<=1e-9*max(abs(observed),abs(reference)); require both finite positive numeric values'},
 'section.area_relative':{'kind':'same_unit_relative','unit':'m^2','dimension':'area','relative_tolerance':1e-9,'absolute_tolerance':0,'predicate':'abs(observed-reference)<=1e-9*max(abs(observed),abs(reference)); require both finite positive numeric values'},
 'section.second_moment_relative':{'kind':'same_unit_relative','unit':'m^4','dimension':'second_moment_area','relative_tolerance':1e-9,'absolute_tolerance':0,'predicate':'abs(observed-reference)<=1e-9*max(abs(observed),abs(reference)); require both finite positive numeric values'},
 'section.modulus_relative':{'kind':'same_unit_relative','unit':'m^3','dimension':'section_modulus','relative_tolerance':1e-9,'absolute_tolerance':0,'predicate':'abs(observed-reference)<=1e-9*max(abs(observed),abs(reference)); require both finite positive numeric values'}
}
entries=[
 ('outside_diameter_m','outside diameter','length','m','OD=0.20 m','section.source_length_identity','/solve/preview_model/model/pipe_segments/0/section/outside_diameter/value'),
 ('effective_wall_thickness_m','effective wall','length','m','t_eff=t=0.01 m; no deduction input is present','section.source_length_identity','/solve/preview_model/model/pipe_segments/0/section/wall_thickness/value'),
 ('ro_m','outer radius','length','m','ro=OD/2','section.derived_length_relative',None),
 ('ri_m','inner radius','length','m','ri=OD/2-t_eff','section.derived_length_relative',None),
 ('As_m2','wall cross-section area','area','m^2','As=pi*(ro^2-ri^2)=0.0019*pi','section.area_relative',None),
 ('Ai_m2','bore area','area','m^2','Ai=pi*ri^2=0.0081*pi','section.area_relative',None),
 ('I_m4','each equal principal second moment','second_moment_area','m^4','Iy=Iz=I=pi*(ro^4-ri^4)/4=0.0000085975*pi','section.second_moment_relative',None),
 ('J_m4','polar second moment/torsion constant','second_moment_area','m^4','J=2I','section.second_moment_relative',None),
 ('Z_m3','circular bending section modulus','section_modulus','m^3','Z=I/ro','section.modulus_relative',None),
]
quantities=[]
for field,name,dimension,unit,formula,rule,pointer in entries:
    quantities.append({'quantity_id':'section.'+field,'observed_field':field,'quantity':name,'dimension':dimension,'unit':unit,
       'source_formula':formula,'expected_decimal':str(expected[field]),'reference_value_binary64':float(expected[field]),
       'criterion_ref':rule,'bound_input_pointer_for_exact_identity':pointer})
bindings=read(BASE/'INPUT_REFERENCE_BINDINGS.json')
cases=[]
for case in bindings['cases']:
    input_path=BASE/case['input']['path'];raw=input_path.read_bytes();assert digest(raw)==case['input']['sha256']
    doc=json.loads(raw);model=doc['solve']['preview_model']['model'];pipe=model['pipe_segments'][0]
    assert len(model['pipe_segments'])==1 and pipe['id']=='pipe'
    assert pipe['section']=={'outside_diameter':{'value':.2,'unit':'m'},'wall_thickness':{'value':.01,'unit':'m'}}
    assert 'section_ref' not in pipe and model['load_cases'][0]['id']=='case'
    assert model['pressure_contract']=={'version':'2.0.0','mode':'exact_straight_pressure_v2'}
    cases.append({'case_id':case['case_id'],'input':{'path':'../'+case['input']['path'],'sha256':case['input']['sha256']},
                  'request_id':doc['request']['request_id'],'model_ref':model['project']['id'],'load_case_ref':'case','pipe_ref':'pipe',
                  'modes':['sparse_interactive','dense_scrutiny'],'quantity_ids':[q['quantity_id'] for q in quantities]})
write('STRUCTURAL_NUMERIC_BINDING.json',{
 'format':'openpipestress.first_static_structural_numeric_addendum/1',
 'status':'prepared_for_independent_review; no runtime/admission claim',
 'parent_checkpoint':CHECKPOINT,'parent_obligation':'complete_case_material_section',
 'reference_basis':{'path':'../section.reference.json','sha256':digest((BASE/'section.reference.json').read_bytes()),'method':'Reuse checked Decimal100 annulus/pi; add Ai by independent annular/bore geometry, no product output target'},
 'observation_namespace':'payload.mechanics_envelope.contract_evidence.exact_cases',
 'selection':{'case_key':'load_case_id','case_value':'case','case_match_count':1,'nested_array':'pipe_sections','pipe_key':'pipe_id','pipe_value':'pipe','pipe_match_count':1,'geometry_basis':'authored_normalized_od_wall_v1'},
 'quantities':quantities,'criteria':criteria,'applies_to':cases,
 'complete_case_rule':'All nine required subquantities must exist and pass. No skip/omission or coercion; absent/invalid/mismatched data fails the existing structural obligation. This does not add result-row selectors.',
 'counting':'Nine numeric subchecks per case/mode (36 for two cases and two modes), inside the existing structural check; original73 scalar rows and10 structural obligations per execution are unchanged.',
 'criterion_authority':'New bounded technical completion for the two named moderate-input development cases. Reuses the selected relative predicate, does not redefine inherited protected assertions or any global/release tolerance.',
 'near_zero_policy':'None of these nine quantities is zero or near-zero in this fixed fixture. Derived references and observations must be positive; no absolute floor, guessed near-zero fallback or extrapolation to other geometry.',
 'enclosure_policy':{'additional_numeric_tolerance_selected':False,'exact_relations':['finite ordered bounds','published maximum contained in its declared coefficient-only interval','summary value/unit/reference equals its bound raw row','declared station fraction is in[0,1]; combined-case witness is exactly0'], 'method_specific_consistency':'Validate certified-gap and bound-field relationships through the pinned ordinary physics-1 validator/method contract. This addendum supplies no substitute epsilon or expected gap value. If unavailable, report the original normal_maximum_evidence check unimplemented/unavailable.', 'independent_reference_comparison':'Use the already selected scalar normal-maximum criterion separately. Do not require the ideal-decimal mechanical reference to lie inside a coefficient-only interval.'}
})
# Preserve bytes check after writing only the new subtree.
for e in before:assert digest((BASE/e['path']).read_bytes())==e['sha256']
write('CHECKS.json',{'status':'PASS additive preparation only','original_25_files_unchanged':before,'original_checkpoint':CHECKPOINT,
 'quantities':len(quantities),'case_mode_numeric_subchecks_if_executed':36,'new_product_or_solver_runs':0,'python':sys.version,
 'arithmetic':'Original checked Decimal100/pi reused; positive Ai=0.0081*pi added; A/I/J/Z source-relation check only, not another independent proof',
 'generator_sha256':digest(Path(__file__).read_bytes()),'independent_review_pending':True})
print(json.dumps({'status':'PASS preparation','Ai_m2':str(expected['Ai_m2']),'quantities':len(quantities),'original_files_unchanged':len(before),'solver_runs':0}))
if __name__=='__main__':pass
