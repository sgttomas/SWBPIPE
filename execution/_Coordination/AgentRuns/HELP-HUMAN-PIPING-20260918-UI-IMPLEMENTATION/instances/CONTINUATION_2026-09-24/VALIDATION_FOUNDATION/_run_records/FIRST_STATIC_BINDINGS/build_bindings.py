#!/usr/bin/env python3
"""Author original inputs and bind observed producer metadata, never result values.
Does not run product or gate. All expected values come from independent_reference.
"""
from pathlib import Path
from decimal import Decimal as D
import json,copy,hashlib
HERE=Path(__file__).parent
PHYS=Path('/private/tmp/piping-engine-integration-20260925/projects/chirality-piping')
C=Path('execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24')
CAP=PHYS/C/'ENGINE_INTEGRATION/PHYSICS_READER_JOIN/RESUME_01/CLI_CAPTURE'
def read(path):return json.loads(path.read_text())
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def save(name,value):
    path=HERE/name;path.write_text(json.dumps(value,indent=2)+'\n');return {'path':name,'sha256':sha(path)}
wire=read(CAP/'fields.stdin.json');manifest=read(CAP/'MANIFEST.json')
assert sha(CAP/'fields.stdin.json')==manifest['stdin_sha256']
ordinary_path=PHYS/'fixtures/results/physics_connected_mechanics_dense.json'
ordinary=read(ordinary_path)
assert ordinary['producer']['semantic_contract_id']=='openpipestress.result_semantics/0.3.0/physics-1'
rows=[r for r in ordinary['results'] if r.get('basis_ref',{}).get('ref_id')=='case:six-component-load']
sem_path=PHYS/'fixtures/results/semantic_contract_v0_3_physics_1.json';sem=read(sem_path)
assert sha(sem_path)=='9a2cf6268b57bd5265a1a115497c07450819dd4d03cd5ab618097bd9d19da8cc'
provenance='project_original_static_reference_candidate_no_external_data_no_library_or_code_rules'
base_meta=copy.deepcopy(wire['solve']['preview_model']['model'])

def template(kind,component,location,role):
    cand=[]
    for row in rows:
        if row['kind']!=kind:continue
        m=row.get('metadata',{})
        if component is not None and m.get('component')!=component:continue
        if location is not None and m.get('location')!=location:continue
        if role=='root' and 'root' not in row['entity_ref']:continue
        if role=='tip' and 'tip' not in row['entity_ref']:continue
        cand.append(row)
    assert len(cand)==1,(kind,component,location,role,len(cand))
    return cand[0]

def make_selector(assertion):
    key=assertion['assertion_id'];parts=key.split('.');caseid='case';entity='pipe';component=location=None;role='pipe'
    if parts[0]=='node':
        role=parts[1];entity=role;slot=parts[2]
        if slot=='magnitude':kind='displacement_magnitude';rid='result:disp:'+entity
        else:
            axis=slot[-1];rot=slot[0]=='r';kind='global_nodal_'+('rotation_' if rot else 'displacement_')+axis
            component='nodal_'+('rotation_' if rot else 'displacement_')+axis;location='node';rid='result:disp:'+entity+':'+slot
    elif parts[0]=='support':
        entity='anchor';role='support';component=parts[1];location='node'
        if component in ['force_magnitude','moment_magnitude']:kind='support_reaction_'+component+'_v2'
        else:kind='support_reaction_component_v2'
        rid='result:support-action:4:case:6:anchor:'+component
    elif parts[0]=='section':
        location=parts[1];slot=parts[2]
        kind,component,suffix,prefix={'N':('element_local_axial_force','axial_force','axial','force'),'Vy':('element_local_shear_force_y','shear_force_y','shear-y','force'),'Vz':('element_local_shear_force_z','shear_force_z','shear-z','force'),'T':('element_local_torsional_moment','torsional_moment','torsion','moment'),'My':('element_local_bending_moment_y','bending_moment_y','bending-y','moment'),'Mz':('element_local_bending_moment_z','bending_moment_z','bending-z','moment')}[slot]
        if location=='end_i':rid=f'result:{prefix}:pipe:{suffix}'
        elif location=='end_j':rid=f'result:{prefix}:pipe:{suffix}:end-j'
        else:rid=f'result:{prefix}:pipe:{location.replace("_","-")}:{suffix}'
    elif key=='stress.maximum_absolute_normal':
        kind='pipe_elastic_normal_stress_maximum_v2';component='maximum_absolute_normal_stress';location='governing_station';rid='result:elastic-maximum:4:case:4:pipe'
    else:
        location=parts[1];slot=parts[2]
        component,suffix={'axial':('axial_normal_stress','axial-normal'),'bending_y':('bending_normal_stress_y','bending-normal-y'),'bending_z':('bending_normal_stress_z','bending-normal-z'),'torsional_shear':('torsional_shear_stress','torsional-shear')}[slot]
        kind='element_local_'+component;rid=f'result:stress:pipe:{location.replace("_","-")}:{suffix}'
    source=template(kind,component,location,role)
    signature=[s for s in sem['rows'] if s['kind']==kind and s['unit']==source['unit'] and s['component']==source.get('metadata',{}).get('component')]
    assert len(signature)==1,(key,kind)
    signature=signature[0]
    assert assertion['unit']==source['unit'] and assertion['dimension']==signature['source_physical_semantic_dimension']
    return {'id':rid,'kind':kind,'unit':source['unit'],'entity_ref':entity,'basis_ref':{'ref_type':'load_case','ref_id':'case'},'metadata':copy.deepcopy(source.get('metadata')),'dimension':signature['source_physical_semantic_dimension']}, {'source_fixture_row_id':source['id'],'semantic_signature_id':signature['signature_id'],'family':signature['family'],'metadata_presence':'present' if 'metadata' in source else 'absent'}

bindings=[]
for cid in ['axial','bending_torsion']:
    model={k:copy.deepcopy(base_meta[k]) for k in ['schema_version','document_kind','data_boundary','analysis_status']}
    model['project']={'id':'project:original-static-'+cid,'name':'Original static '+cid+' reference candidate','description':'Invented analytical fixture, explicit linear elastic EB/circular-annulus scope; no libraries or code rules.','units':{'length':'m','force':'N','pressure':'Pa','stress':'Pa','angle':'rad','temperature':'degC'}}
    model['nodes']=[{'id':id,'label':id,'position':{'x':x,'y':0,'z':0},'provenance':provenance} for id,x in [('root',0),('tip',2)]]
    model['pipe_segments']=[{'id':'pipe','label':'Original annulus','from':'root','to':'tip','section':{'outside_diameter':{'value':0.2,'unit':'m'},'wall_thickness':{'value':0.01,'unit':'m'}},'material':'material','y_reference':{'x':0,'y':1,'z':0},'provenance':provenance}]
    model['materials']=[{'id':'material','label':'Explicit invented E-nu pair','constitutive_basis':'homogeneous_isotropic_E_nu_v1','elastic_modulus':{'value':200000000000,'unit':'Pa'},'poisson_ratio':{'value':0.25,'unit':'1'},'provenance':provenance}]
    model['supports']=[{'id':'anchor','label':'Root six-DOF anchor','node':'root','family':'anchor','restraints':['UX','UY','UZ','RX','RY','RZ'],'provenance':provenance}]
    model['components']=[];model['combinations']=[]
    loads=[('global_x',1000,'concentrated_force','force','N')] if cid=='axial' else [('global_y',30,'concentrated_force','force','N'),('global_z',-40,'concentrated_force','force','N'),('rotation_x',20,'concentrated_moment','moment','N*m')]
    model['load_cases']=[{'id':'case','label':'Original '+cid+' equilibrium case','primitive_loads':[{'id':'load:'+direction,'category':category,'target':{'type':'node','node':'tip'},'direction':direction,'magnitude':{'value':value,'unit':unit},'dimension':dimension,'provenance':provenance} for direction,value,category,dimension,unit in loads],'pressure_regions':[],'provenance':provenance}]
    model['pressure_contract']={'version':'2.0.0','mode':'exact_straight_pressure_v2'}
    payload={'materials':[],'model':model};payload_binding=save(cid+'.preview_request.json',payload)
    request=copy.deepcopy(wire);request['solve']['preview_model']=payload
    rr=request['request'];rr['request_id']='first-static-'+cid+'-v1';rr['model_ref']={'ref_type':'model','ref_id':model['project']['id']};rr['project_ref']={'ref_type':'project','ref_id':model['project']['id']};rr['load_basis_refs']=[{'ref_type':'load_case','ref_id':'case'}]
    rr['input_manifest_ref']={'ref_type':'audit_manifest','ref_id':'manifest:first-static-'+cid+'-v1'}
    rr['provenance']={'source_name':'Project-original first static binding '+cid,'source_location':'C/VALIDATION_FOUNDATION/FIRST_STATIC_BINDINGS','source_license':'project-original-public-content','contributor':'HELP_HUMAN delegated design /root/load_state_design','contributor_certification':'Explicitly invented mathematical test inputs; no external project, material library or code data','redistribution_status':'invented_non_engineering_example','review_status':'prepared_for_independent_reference_review'}
    request_binding=save(cid+'.runner_input.candidate.json',request)
    reference=read(HERE/(cid+'.expectations.json'))
    assertions=[];rules=[];seen_rules=set()
    for a in reference['values']:
        selector,origin=make_selector(a);rid='criterion:'+origin['family']+':'+a['dimension']+':'+a['unit']+':'+a['zero_policy']
        if a['zero_policy']=='exact_prescribed':relative,absolute=0.0,0.0;derivation='exact imposed-zero reconstruction; numeric equality, either zero sign allowed'
        elif not a['mathematically_zero']:relative,absolute=1e-9,0.0;derivation='new field-relative development comparison; abs(delta)<=1e-9*max(abs(reference),abs(observed)); no release criterion'
        else:
            scale_key={'mm':'length_mm','rad':'angle_rad','N':'force_N','N*m':'moment_Nm','MPa':'stress_MPa','Pa':'stress_Pa'}[a['unit']]
            relative,absolute=0.0,float(D(reference['zero_scales_decimal'][scale_key])*D('1e-9'));derivation='new derived-zero absolute budget:1e-9*'+scale_key+'; scale='+reference['zero_scales_decimal'][scale_key]
        assertions.append({'id':a['assertion_id'],'selector':selector,'selector_origin':origin,'criterion_rule_id':rid})
        rule={'rule_id':rid,'dimension_id':a['dimension'],'result_family':origin['family'],'unit_ref':{'ref_type':'unit','ref':a['unit']},'normalization_basis':'same_unit_required','relative_tolerance_value':relative,'absolute_tolerance_value':absolute,'tolerance_value_status':'project_specific_review_required','review':'pending separate independent review; not an inherited blanket1e-9 criterion','provenance':derivation}
        if rid not in seen_rules:rules.append(rule);seen_rules.add(rid)
    assert len({a['selector']['id'] for a in assertions})==73
    criterion=save(cid+'.criteria.candidate.json',{'schema_version':'0.1.0','tolerance_profile':{'profile_id':'first-static-'+cid+'-field-comparison-v1','profile_status':'draft_pending_independent_review','scope':'These two named original finite-input cases only; unchanged inherited assertions and numerical method contracts remain at their owners.','rules':rules}})
    selector_binding=save(cid+'.selectors.candidate.json',{'format':'openpipestress.first_static_selector_candidate/1','case_id':reference['case_id'],'producer_contract':'openpipestress.result_semantics/0.3.0/physics-1','raw_schema_version':'0.2.0','row_namespace':'payload.mechanics_envelope.results','assertions':assertions,'gaps':['Current main thin gate has raw0.1/sparse-only dispatch and requires nonempty metadata; new physics1 dispatch/true absent metadata handling needed.','Scalar rows do not replace required summary/formulation/governing-evidence checks in STRUCTURAL_EXPECTATIONS.json.'],'scoring_readiness':False})
    bindings.append({'case_id':reference['case_id'],'input':request_binding,'product_request':payload_binding,'reference':{'path':cid+'.reference.candidate.json','sha256':sha(HERE/(cid+'.reference.candidate.json'))},'criterion':criterion,'selectors':selector_binding,'required_scalar_rows':73,'modes_proposed':['sparse_interactive','dense_scrutiny'],'scoring_readiness':False})

save('INPUT_REFERENCE_BINDINGS.json',{'status':'finite candidate package, not runnable admission or observed results','cases':bindings,'actual_new_case_outputs':None,'pending_independent_review':True})
save('WIRE_BASIS.json',{'candidate_status':manifest['candidate'],'cli_capture_manifest':{'origin':'ENGINE_INTEGRATION/PHYSICS_READER_JOIN/RESUME_01/CLI_CAPTURE/MANIFEST.json','sha256':sha(CAP/'MANIFEST.json')},'actual_existing_cli_stdin_sha256':manifest['stdin_sha256'],'reported_actual_binary':manifest['binary'],'new_inputs_executed':False,'invocation':['openpipestress-runner','solve','--input','-','--solver-mode','<sparse_interactive|dense_scrutiny>','--explicit-local-private-intent'],'stdout_namespace':'payload.mechanics_envelope','canonical_document_not_in_cli_stdout':True,'ordinary_metadata_basis':{'path':str(ordinary_path.relative_to(PHYS)),'sha256':sha(ordinary_path),'case_ref':'case:six-component-load','use':'metadata/row shape only; never numerical oracle'},'semantic_table':{'path':str(sem_path.relative_to(PHYS)),'sha256':sha(sem_path)},'new_cases_expected_contract':'physics-1 ordinary well-conditioned exact-profile; do not force physics-source-1 or accept its different maximum metadata','source_manifest':manifest['source']})
print(json.dumps({'status':'candidate input/selector bindings written; no solve','cases':2,'row_selectors':146,'metadata_absent_rows':4,'ordinary_fixture_sha256':sha(ordinary_path)}))
