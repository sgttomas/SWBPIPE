from pathlib import Path
import json,math,hashlib,sys
out=Path(__file__).parent;label,mode=sys.argv[1:];assert mode in ('sparse_interactive','dense_scrutiny')
reference=out/'connected-native-references.json';assert hashlib.sha256(reference.read_bytes()).hexdigest()=='f2e5ca626d5055157fce2bc3ff149f7c2823a429f3b7b37e1e1e82373b31595c'
refs=json.loads(reference.read_text());p=out/'private-rows'/(label+'.json');saved=json.loads(p.read_text())['row'];model=json.loads(saved['model_json']);raw=json.loads(saved['mechanics_result_json']);rows=raw['results'];body=raw['source_block_recovery']['body'];case=body['cases'][0]
assert body['status']=='qualified' and case['requested_mode']==mode and case['selected_method']=='retained_source_blocks_exact_v1' and case['ordinary_attempt']['outcome'] in ('sensitive','rejected')
assert raw['producer']['semantic_contract_id']=='openpipestress.result_semantics/0.3.0/physics-source-1'
assert model['schema_version']=='0.3.0' and len(model['load_cases'])==1 and model['load_cases'][0]['pressure_regions']==[]
loads=model['load_cases'][0]['primitive_loads'];expected_loads={'global_x':(.001,'N'),'global_y':(.002,'N'),'global_z':(-.003,'N'),'rotation_x':(1e-8,'N*m'),'rotation_y':(-.0005,'N*m'),'rotation_z':(.0006,'N*m')};assert len(loads)==6 and {l['direction']:(l['magnitude']['value'],l['magnitude']['unit']) for l in loads}==expected_loads
assert {n['id']:n['position'] for n in model['nodes']}=={'independent-root':{'x':0,'y':0,'z':0},'independent-tip':{'x':2,'y':0,'z':0}}
mat=model['materials'][0];assert mat['elastic_modulus']=={'unit':'Pa','value':200e9} and mat['poisson_ratio']=={'unit':'1','value':.25}
pipe=model['pipe_segments'][0];assert pipe['section']['outside_diameter']=={'unit':'m','value':.2} and pipe['section']['wall_thickness']=={'unit':'m','value':.01}
spring=next(x for x in model['supports'] if x['id']=='independent-spring');assert spring['stiffness']['value']=={'unit':'N*m/rad','value':.0001}
checks=[]
def one(kind,entity,component=None,location=None):
 r=[x for x in rows if x['kind']==kind and x['entity_ref']==entity and x.get('basis_ref')=={'ref_type':'load_case','ref_id':'case'} and (component is None or x['metadata']['component']==component) and (location is None or x['metadata']['location']==location)];assert len(r)==1,(kind,entity,component,location,len(r));return r[0]
def check(row,expected,unit):
 value=row['value'];assert type(value) in (int,float) and math.isfinite(value) and row['unit']==unit
 error=abs(value-expected);passed=value==0 if expected==0 else error<=1e-9*abs(expected)
 item={'result_id':row['id'],'observed':value,'expected':expected,'unit':unit,'relative_error':None if expected==0 else error/abs(expected),'criterion':'exact structural zero' if expected==0 else 'unchanged nonzero relative1e-9','passed':passed};checks.append(item);assert passed,item
for support in ['anchor','independent-spring']:
 for i,component in enumerate(['Fx','Fy','Fz','Mx','My','Mz']):
  expected=refs['fields_anchor_actions'][i] if support=='anchor' else refs['fields_spring_Mx_Nm'] if component=='Mx' else 0
  r=one('support_reaction_component_v2',support,component);assert r['metadata']['coordinate_system']=='global';check(r,expected,'N' if component.startswith('F') else 'N*m')
for axis in 'xyz':
 check(one('global_nodal_displacement_'+axis,'independent-tip'),refs['fields']['tip_u'+axis+'_mm']['binary64'],'mm')
 check(one('global_nodal_rotation_'+axis,'independent-tip'),refs['fields']['tip_r'+axis+'_rad']['binary64'],'rad')
check(one('global_nodal_rotation_x','independent-root'),refs['fields']['root_rx_rad']['binary64'],'rad')
for location in ['end_i','quarter_1','midspan','quarter_3','end_j']:check(one('element_local_torsional_shear_stress','independent-member','torsional_shear_stress',location),refs['fields']['torsional_shear_MPa']['binary64'],'MPa')
r=one('pipe_elastic_normal_stress_maximum_v2','independent-member');check(r,refs['fields']['maximum_normal_Pa']['binary64'],'Pa');ex=raw['contract_evidence']['exact_cases'][0]['pipe_stress_extrema'][0];assert ex['station_fraction']==0 and ex['basis']=='retained_source_endpoint_normal_max_v1'
assert raw['summary']['max_open_formula_stress']['result_ref']==r['id'] and raw['summary']['max_open_formula_stress']['value']==r['value']
result={'candidate':'8b982aa7ce64afe37e6067d1038d92608f4aaf3f','mode':mode,'reference_sha256':hashlib.sha256(reference.read_bytes()).hexdigest(),'checker_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'private_capture_sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'model_sha256':hashlib.sha256(saved['model_json'].encode()).hexdigest(),'original_ordinary_outcome':case['ordinary_attempt']['outcome'],'selected_method':case['selected_method'],'scope':'Named numeric/reference and saved-source consistency; actual CUA invocation supplies separate origin witness','checks':checks,'count':len(checks),'all_passed':True,'governing_station_fraction':0}
(out/'row-audits'/(label+'-analytical.json')).write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({'mode':mode,'checks':len(checks),'all_passed':True,'maximum_relative_error':max(x['relative_error'] or 0 for x in checks),'normal_maximum_Pa':r['value']}))
