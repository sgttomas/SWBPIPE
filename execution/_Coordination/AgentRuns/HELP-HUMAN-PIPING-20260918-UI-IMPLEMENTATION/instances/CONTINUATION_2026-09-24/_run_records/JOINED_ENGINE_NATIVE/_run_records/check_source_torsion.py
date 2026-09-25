from pathlib import Path
from decimal import Decimal as D,localcontext
import json,math,hashlib,sys
out=Path(__file__).parent;label,variant,mode=sys.argv[1:];assert variant in ('n05','n06') and mode in ('sparse_interactive','dense_scrutiny')
p=out/'private-rows'/(label+'.json');stored=json.loads(p.read_text())['row'];model=json.loads(stored['model_json']);raw=json.loads(stored['mechanics_result_json']);body=raw['source_block_recovery']['body'];case=body['cases'][0]
assert model['schema_version']=='0.3.0' and len(model['load_cases'])==1 and model['load_cases'][0]['pressure_regions']==[]
assert raw['producer']['semantic_contract_id']=='openpipestress.result_semantics/0.3.0/physics-source-1'
assert body['status']=='qualified' and case['outcome']=='qualified' and case['selected_method']=='retained_source_blocks_exact_v1' and case['requested_mode']==mode
assert case['ordinary_attempt']['outcome'] in ('sensitive','rejected')
M=D('1e-8') if variant=='n05' else D('1e-16');K=D('1e-4') if variant=='n05' else D('1e-12')
spring=next(x for x in model['supports'] if x['id']=='independent-spring');assert spring['stiffness']['dof']=='RX' and spring['stiffness']['value']=={'unit':'N*m/rad','value':float(K)}
loads=model['load_cases'][0]['primitive_loads'];assert len(loads)==1 and loads[0]['direction']=='rotation_x' and loads[0]['magnitude']=={'unit':'N*m','value':float(M)}
mat=model['materials'][0];assert mat['elastic_modulus']=={'unit':'Pa','value':200e9} and mat['poisson_ratio']=={'unit':'1','value':0.25} and mat['constitutive_basis']=='homogeneous_isotropic_E_nu_v1'
pipe=model['pipe_segments'][0];assert pipe['section']['outside_diameter']=={'unit':'m','value':0.2} and pipe['section']['wall_thickness']=={'unit':'m','value':0.01}
assert {n['id']:n['position'] for n in model['nodes']}=={'independent-root':{'x':0,'y':0,'z':0},'independent-tip':{'x':2,'y':0,'z':0}}
with localcontext() as c:
 c.prec=100;a=D(1);b=D(1)/D(2).sqrt();t=D(1)/4;k=D(1)
 for _ in range(9):
  avg=(a+b)/2;b=(a*b).sqrt();t-=k*(a-avg)**2;a=avg;k*=2
 pi=(a+b)**2/(4*t);ro=D('0.1');ri=D('0.09');G=D('80e9');L=D(2);J=pi*(ro**4-ri**4)/2
 references={'root_rx_rad':float(M/K),'tip_rx_rad':float(M/K+M*L/(G*J)),'spring_Mx_Nm':float(-M),'section_torsional_shear_MPa':float(M*ro/J/D('1e6'))}
rows=raw['results'];assert len({x['id'] for x in rows})==len(rows)
checks=[]
def one(kind,entity,component=None,location=None):
 matches=[r for r in rows if r['kind']==kind and r['entity_ref']==entity and r.get('basis_ref')=={'ref_type':'load_case','ref_id':'case'} and (component is None or r['metadata']['component']==component) and (location is None or r['metadata']['location']==location)];assert len(matches)==1,(kind,entity,component,location,len(matches));return matches[0]
def check(r,expected,unit):
 value=r['value'];assert type(value) in (int,float) and math.isfinite(value) and r['unit']==unit
 error=abs(value-expected);passed=value==0 if expected==0 else error<=1e-9*abs(expected)
 entry={'result_id':r['id'],'observed':value,'expected':expected,'unit':unit,'criterion':'exact analytical zero' if expected==0 else 'unchanged nonzero relative1e-9','relative_error':None if expected==0 else error/abs(expected),'passed':passed};checks.append(entry);assert passed,entry
check(one('global_nodal_rotation_x','independent-root'),references['root_rx_rad'],'rad');check(one('global_nodal_rotation_x','independent-tip'),references['tip_rx_rad'],'rad')
for support in ['anchor','independent-spring']:
 for component in ['Fx','Fy','Fz','Mx','My','Mz']:
  r=one('support_reaction_component_v2',support,component);assert r['metadata']['coordinate_system']=='global'
  check(r,references['spring_Mx_Nm'] if support=='independent-spring' and component=='Mx' else 0,'N' if component.startswith('F') else 'N*m')
for location in ['end_i','quarter_1','midspan','quarter_3','end_j']:check(one('element_local_torsional_shear_stress','independent-member','torsional_shear_stress',location),references['section_torsional_shear_MPa'],'MPa')
maximum=one('pipe_elastic_normal_stress_maximum_v2','independent-member');check(maximum,0,'Pa')
ex=raw['contract_evidence']['exact_cases'][0]['pipe_stress_extrema'][0];assert ex['locations']=={'kind':'whole_span_constant'} and ex['basis']=='retained_source_endpoint_normal_max_v1' and ex['value_lower_pa']==ex['value_upper_pa']==0
summary=raw['summary']['max_open_formula_stress'];assert summary['result_ref']==maximum['id'] and summary['value']==0 and summary['unit']=='Pa'
record={'candidate':'8b982aa7ce64afe37e6067d1038d92608f4aaf3f','variant':variant,'mode':mode,'private_capture_sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'checker_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'model_sha256':hashlib.sha256(stored['model_json'].encode()).hexdigest(),'method':'Independent Decimal100 AGM pi and circular Saint-Venant torsion: root rotation M/K, tip rotation M/K+ML/GJ; support balance; signed j-side section shear Mr/J; no product/author imports','original_ordinary_outcome':case['ordinary_attempt']['outcome'],'selected_method':case['selected_method'],'recorded_receipt_outcome':body['status'],'scope':'Numeric/reference and saved-data identity checks; actual native CUA Run is separate origin evidence; this script does not authenticate imported receipts','references':references,'checks':checks,'count':len(checks),'all_passed':True,'whole_span_constant_verified':True,'observed_result_count':len(rows)}
(out/'row-audits'/(label+'-analytical.json')).write_text(json.dumps(record,indent=2)+'\n');print(json.dumps({'variant':variant,'mode':mode,'checks':len(checks),'all_passed':True,'max_nonzero_relative_error':max(x['relative_error'] or 0 for x in checks),'ordinary':case['ordinary_attempt']['outcome']}))
