from pathlib import Path
import json,math,hashlib,sys
out=Path(__file__).parent;label,mode=sys.argv[1:];assert mode in ('sparse_interactive','dense_scrutiny')
rp=out/'connected-native-references.json';assert hashlib.sha256(rp.read_bytes()).hexdigest()=='f2e5ca626d5055157fce2bc3ff149f7c2823a429f3b7b37e1e1e82373b31595c';refs=json.loads(rp.read_text())['mixed']
p=out/'private-rows'/(label+'.json');saved=json.loads(p.read_text())['row'];model=json.loads(saved['model_json']);raw=json.loads(saved['mechanics_result_json']);rows=raw['results'];body=raw['source_block_recovery']['body'];records={x['basis_ref']['ref_id']:x for x in body['cases']}
sc='case';pc='case:ordinary-pressure';assert set(records)=={sc,pc} and body['status']=='qualified'
assert all(c['requested_mode']==mode and c['outcome']=='qualified' for c in records.values())
assert records[sc]['selected_method']=='retained_source_blocks_exact_v1' and records[sc]['ordinary_attempt']['outcome'] in ('sensitive','rejected')
assert records[pc]['selected_method']==('ordinary_sparse_structural_v1' if mode=='sparse_interactive' else 'ordinary_dense_structural_v1') and records[pc]['ordinary_attempt']['outcome']=='checks_passed'
assert raw['producer']['semantic_contract_id']=='openpipestress.result_semantics/0.3.0/physics-source-1'
cases={c['id']:c for c in model['load_cases']};assert set(cases)=={sc,pc} and cases[sc]['pressure_regions']==[] and cases[pc]['modulus_basis_ref']=='point:compliant'
mat=model['materials'][0];assert mat['elastic_modulus']=={'unit':'Pa','value':200e9} and mat['poisson_ratio']=={'unit':'1','value':.25}
point=next(x for x in mat['temperature_points'] if x['id']=='point:compliant');assert point['elastic_modulus']=={'unit':'Pa','value':2000} and point['poisson_ratio']=={'unit':'1','value':.25} and point['temperature']=={'unit':'degC','value':20};assert 'thermal_expansion_coefficient' not in point and 'shear_modulus' not in point
region=cases[pc]['pressure_regions'][0];assert region['pressure']=={'unit':'Pa','value':1e-5} and all(t['closure_transfer']=='transfers_to_wall' for t in region['terminals'])
assert len(cases[sc]['primitive_loads'])==1 and cases[sc]['primitive_loads'][0]['magnitude']=={'unit':'N*m','value':1e-8} and cases[sc]['primitive_loads'][0]['direction']=='rotation_x'
assert {n['id']:n['position'] for n in model['nodes']}=={'independent-root':{'x':0,'y':0,'z':0},'independent-tip':{'x':2,'y':0,'z':0}}
pipe=model['pipe_segments'][0];assert pipe['section']['outside_diameter']=={'unit':'m','value':.2} and pipe['section']['wall_thickness']=={'unit':'m','value':.01}
assert next(x for x in model['supports'] if x['id']=='independent-spring')['stiffness']['value']=={'unit':'N*m/rad','value':.0001}
checks=[]
def one(case,kind,entity,component=None,location=None):
 a=[r for r in rows if r['kind']==kind and r['entity_ref']==entity and r.get('basis_ref')=={'ref_type':'load_case','ref_id':case} and (component is None or r['metadata']['component']==component) and (location is None or r['metadata']['location']==location)];assert len(a)==1,(case,kind,entity,component,location,len(a));return a[0]
def check(r,expected,unit,zero_scale=0):
 value=r['value'];assert type(value) in (int,float) and math.isfinite(value) and r['unit']==unit
 scale=abs(expected) if expected!=0 else zero_scale;error=abs(value-expected);passed=error<=1e-9*scale
 item={'result_id':r['id'],'case':r['basis_ref']['ref_id'],'observed':value,'expected':expected,'unit':unit,'scale':scale,'scaled_error':error/scale if scale else 0,'passed':passed};checks.append(item);assert passed,item
check(one(sc,'global_nodal_rotation_x','independent-root'),refs['source_root_rx_rad']['binary64'],'rad');check(one(sc,'global_nodal_rotation_x','independent-tip'),refs['source_tip_rx_rad']['binary64'],'rad')
check(one(sc,'support_reaction_component_v2','independent-spring','Mx'),-1e-8,'N*m');check(one(sc,'pipe_elastic_normal_stress_maximum_v2','independent-member'),0,'Pa')
for location in ['end_i','quarter_1','midspan','quarter_3','end_j']:
 check(one(sc,'element_local_torsional_shear_stress','independent-member','torsional_shear_stress',location),refs['source_torsional_shear_MPa']['binary64'],'MPa')
 check(one(pc,'pipe_wall_axial_force_v2','independent-member',None,location),refs['pressure_wall_N']['binary64'],'N')
 check(one(pc,'pipe_effective_axial_force_v2','independent-member',None,location),0,'N',refs['pressure_wall_N']['binary64'])
 check(one(pc,'pipe_axial_membrane_stress_v2','independent-member',None,location),refs['pressure_axial_membrane_Pa']['binary64'],'Pa')
 for kind,component,key in [('pipe_lame_radial_stress_v2','lame_inner_radial_stress','pressure_inner_radial_Pa'),('pipe_lame_radial_stress_v2','lame_outer_radial_stress','pressure_outer_radial_Pa'),('pipe_lame_hoop_stress_v2','lame_inner_hoop_stress','pressure_inner_hoop_Pa'),('pipe_lame_hoop_stress_v2','lame_outer_hoop_stress','pressure_outer_hoop_Pa')]:check(one(pc,kind,'independent-member',component,location),refs[key]['binary64'],'Pa',1e-5)
for support in ['anchor','independent-spring']:
 for component in ['Fx','Fy','Fz','Mx','My','Mz']:
  force=component.startswith('F');check(one(pc,'support_reaction_component_v2',support,component),0,'N' if force else 'N*m',refs['pressure_wall_N']['binary64']*(1 if force else 2))
check(one(pc,'global_nodal_displacement_x','independent-tip'),refs['pressure_tip_ux_mm']['binary64'],'mm');maximum=one(pc,'pipe_elastic_normal_stress_maximum_v2','independent-member');check(maximum,refs['pressure_axial_membrane_Pa']['binary64'],'Pa')
summary=raw['summary']['max_open_formula_stress'];assert summary['result_ref']==maximum['id'] and summary['value']==maximum['value'] and summary['unit']=='Pa'
result={'candidate':'8b982aa7ce64afe37e6067d1038d92608f4aaf3f','mode':mode,'reference_sha256':hashlib.sha256(rp.read_bytes()).hexdigest(),'checker_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'private_capture_sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'model_sha256':hashlib.sha256(saved['model_json'].encode()).hexdigest(),'scope':'Independent named physical references and case/material/method separation; actual native Run is separate origin evidence','criteria':'Nonzero relative1e-9; exact source zeros; existing pressure_runtime zero scale from cap force, cap force*L or pressure','case_methods':{k:v['selected_method'] for k,v in records.items()},'checks':checks,'count':len(checks),'all_passed':True,'global_maximum_case':pc,'thermal_alpha_not_invented':True}
(out/'row-audits'/(label+'-analytical.json')).write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({'mode':mode,'checks':len(checks),'all_passed':True,'maximum_scaled_error':max(x['scaled_error'] for x in checks),'global_maximum_Pa':maximum['value']}))
