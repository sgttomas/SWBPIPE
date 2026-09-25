"""Independent candidate review: exact free bodies/energy integrals and AGM pi.
Never imports or executes author generators or production code; never solves.
"""
from pathlib import Path
from fractions import Fraction as F
from decimal import Decimal as D, localcontext
import json, hashlib, math, re

OUT = Path(__file__).parent
PACK = OUT.parent
REPO = next(p for p in PACK.parents if (p / 'AGENTS.md').is_file())
PHYS = Path('/private/tmp/piping-engine-integration-20260925/projects/chirality-piping')

def read(path): return json.loads(path.read_text())
def sha(path): return hashlib.sha256(path.read_bytes()).hexdigest()
def dec(q):
    q = F(q)
    return D(q.numerator) / D(q.denominator)
def close(a,b):
    return a == b == 0 or abs(a-b) <= abs(b)*D('1e-94')
def integrate_product(p,q,L=F(2)):
    return sum((a*b*L**(i+j+1)/F(i+j+1) for i,a in enumerate(p) for j,b in enumerate(q)), F())
def polynomial(p,x): return sum((a*x**i for i,a in enumerate(p)), F())
def neg(v): return [-x for x in v]
def norm(v): return sum(x*x for x in v).sqrt()
def agm_pi():
    a,b,t,p = D(1),D(1)/D(2).sqrt(),D(1)/4,D(1)
    for _ in range(10):
        old=a; a=(a+b)/2; b=(old*b).sqrt(); t-=p*(old-a)**2; p*=2
    return (a+b)**2/(4*t)

manifest=read(PACK/'PACKET_MANIFEST.json')
assert sha(PACK/'PACKET_MANIFEST.json') == '0603a990838166c1da43b26f8b657dae3051289097700a76859d1afaa45db02b'
for r in manifest['files']:
    assert sha(PACK/r['path']) == r['sha256'] and (PACK/r['path']).stat().st_size == r['bytes']
assert sha(PACK/'RETURN.md') == 'b7af8eee3e5972be9c286b84ed410c3dff98ae78b768dd6ffbfe150db172c81c'
bind=read(PACK/'INPUT_REFERENCE_BINDINGS.json')
for case in bind['cases']:
    for k in ('input','product_request','reference','criterion','selectors'):
        assert sha(PACK/case[k]['path']) == case[k]['sha256']
assert bind['actual_new_case_outputs'] is None and bind['pending_independent_review']
wire=read(PACK/'WIRE_BASIS.json')
fixture_path=PHYS/wire['ordinary_metadata_basis']['path']
table_path=PHYS/wire['semantic_table']['path']
assert sha(fixture_path)==wire['ordinary_metadata_basis']['sha256']
assert sha(table_path)==wire['semantic_table']['sha256']
fixture=read(fixture_path); rows={r['id']:r for r in fixture['results']}
table={r['signature_id']:r for r in read(table_path)['rows']}
assert fixture['producer']['semantic_contract_id']=='openpipestress.result_semantics/0.3.0/physics-1'
summary={}
all_comparisons=[]
with localcontext() as ctx:
    ctx.prec=125
    pi=agm_pi(); ro,ri,L=F(1,10),F(9,100),F(2)
    E,nu=F(200000000000),F(1,4); G=E/(2*(1+nu))
    # Integrals over polar annulus: area dA = r dr dtheta, Iy = r^3 sin²theta dr dtheta.
    Acoef=2*(ro**2-ri**2)/2; Icoef=(ro**4-ri**4)/4
    A,I=dec(Acoef)*pi,dec(Icoef)*pi; J=2*I; Z=I/dec(ro)
    props={'pi':pi,'L_m':dec(L),'ro_m':dec(ro),'ri_m':dec(ri),'E_Pa':dec(E),'nu':dec(nu),'G_Pa':dec(G),'A_m2':A,'I_m4':I,'J_m4':J,'Z_m3':Z}
    for name,value in read(PACK/'section.reference.json')['values'].items(): assert close(D(value),props[name]),name
    # Bound effect of the explicitly represented diameter/thickness operands only.
    rbf=F.from_float(.2)/2; ibf=rbf-F.from_float(.01)
    Abf=dec(rbf*rbf-ibf*ibf)*pi; Ibf=dec((rbf**4-ibf**4)/4)*pi; Zbf=Ibf/dec(rbf)
    geometry_changes={k:str(abs((a-b)/b)) for k,a,b in [('A',Abf,A),('I',Ibf,I),('Z',Zbf,Z)]}
    assert max(D(x) for x in geometry_changes.values()) < D('1e-15')
    for cid,(fx,fy,fz,tx) in {'axial':(1000,0,0,0),'bending_torsion':(0,30,-40,20)}.items():
        force=list(map(F,(fx,fy,fz))); applied_m=[F(tx),F(0),F(0)]
        # Complete tip free-body wrench referred to a cut at x.
        action_polys=[[F(fx)],[F(fy)],[F(fz)],[F(tx)],[-L*fz,F(fz)],[L*fy,F(-fy)]]
        cut0=[polynomial(q,F(0)) for q in action_polys]; reaction=neg(cut0)
        # Castigliano/unit-load: integrate actual resultant times virtual resultant.
        tip=[dec(integrate_product(action_polys[0],[F(1)]))/(dec(E)*A),
             dec(integrate_product(action_polys[5],[L,F(-1)]))/(dec(E)*I),
             dec(integrate_product(action_polys[4],[-L,F(1)]))/(dec(E)*I),
             dec(integrate_product(action_polys[3],[F(1)]))/(dec(G)*J),
             dec(integrate_product(action_polys[4],[F(1)]))/(dec(E)*I),
             dec(integrate_product(action_polys[5],[F(1)]))/(dec(E)*I)]
        expected={}; units={}
        def put(name,value,unit):
            expected[name]=value;units[name]=unit
        for node,values in [('root',[D(0)]*6),('tip',tip)]:
            for i,dof in enumerate(['ux','uy','uz','rx','ry','rz']): put(f'node.{node}.{dof}',values[i]*(1000 if i<3 else 1),'mm' if i<3 else 'rad')
            put(f'node.{node}.magnitude',norm(values[:3])*1000,'mm')
        for i,component in enumerate(['Fx','Fy','Fz','Mx','My','Mz']): put('support.'+component,dec(reaction[i]),'N' if i<3 else 'N*m')
        put('support.force_magnitude',norm([dec(x) for x in reaction[:3]]),'N')
        put('support.moment_magnitude',norm([dec(x) for x in reaction[3:]]),'N*m')
        ef=read(PACK/(cid+'.expectations.json')); stations={s['name']:s for s in ef['stations']}
        maxima=[]
        for i,name in enumerate(['end_i','quarter_1','midspan','quarter_3','end_j']):
            x=L*F(i,4);q=[polynomial(poly,x) for poly in action_polys];published=neg(q) if i==0 else q
            for j,component in enumerate(['N','Vy','Vz','T','My','Mz']): put(f'section.{name}.{component}',dec(published[j]),'N' if j<3 else 'N*m')
            stresses=[dec(q[0])/A,dec(q[4])/Z,dec(q[5])/Z,dec(q[3])*dec(ro)/J]
            for j,component in enumerate(['axial','bending_y','bending_z','torsional_shear']): put(f'stress.{name}.{component}',stresses[j]/D(1000000),'MPa')
            # Max of a linear form on a circle from Cauchy-Schwarz, independent of scalar component sum.
            normal_amplitude=dec(ro)*norm([dec(q[4]),dec(q[5])])/I
            plus,minus=stresses[0]+normal_amplitude,stresses[0]-normal_amplitude
            maximum=max(abs(plus),abs(minus));maxima.append(maximum)
            st=stations[name];assert F(st['fraction'])==F(i,4) and F(st['x_m'])==x
            assert list(map(F,st['cut_actions_decimal']))==q
            for k,value in [('sigma_plus_Pa',plus),('sigma_minus_Pa',minus),('normal_abs_Pa',maximum),('torsional_shear_Pa',stresses[3])]: assert close(D(st[k]),value),(cid,name,k)
        put('stress.maximum_absolute_normal',max(maxima),'Pa')
        if cid=='axial':
            assert len(set(maxima))==1 and ef['governing_normal']['kind']=='all_stations_tied'
        else:
            assert all(maxima[i]>maxima[i+1] for i in range(4)) and ef['governing_normal']['station_fraction']=='0'
            for k,sign in [('fiber_sigma_plus_yz_m',1),('fiber_sigma_minus_yz_m',-1)]:
                y,z=map(F,ef['governing_normal'][k]);assert y*y+z*z==ro*ro
                stress=(dec(cut0[4]*z-cut0[5]*y))/I;assert close(stress,sign*max(maxima))
        stored={r['assertion_id']:r for r in ef['values']};lean={r['assertion_id']:r for r in read(PACK/(cid+'.reference.candidate.json'))['values']}
        assert len(expected)==len(stored)==len(lean)==73 and expected.keys()==stored.keys()==lean.keys()
        for key,value in expected.items():
            row=stored[key];assert close(D(row['expected_decimal']),value),(cid,key)
            assert row['value']==float(value)==lean[key]['value'] and row['unit']==units[key]==lean[key]['unit'],(cid,key,'wirevalue')
            assert row['mathematically_zero']==(value==0)
            all_comparisons.append({'case':cid,'assertion_id':key,'independent_decimal':str(value),'stored_binary64_hex':float(row['value']).hex(),'unit':units[key]})
        scales={'force_N':norm(list(map(dec,force))),'moment_Nm':norm(list(map(dec,force)))*dec(L),'length_mm':norm(tip[:3])*1000,'angle_rad':abs(tip[0])/dec(L) if cid=='axial' else norm(tip[4:]),'stress_Pa':max(maxima),'stress_MPa':max(maxima)/D(1000000)}
        for k,value in scales.items(): assert close(D(ef['zero_scales_decimal'][k]),value),(cid,k)
        selectors=read(PACK/(cid+'.selectors.candidate.json'));rules={r['rule_id']:r for r in read(PACK/(cid+'.criteria.candidate.json'))['tolerance_profile']['rules']}
        assert selectors['producer_contract']==fixture['producer']['semantic_contract_id'] and not selectors['scoring_readiness']
        assert len(selectors['assertions'])==73 and len({r['id'] for r in selectors['assertions']})==73 and len({r['selector']['id'] for r in selectors['assertions']})==73
        zeros=nonzeros=prescribed=0
        for a in selectors['assertions']:
            key=a['id'];sel=a['selector'];origin=a['selector_origin'];old=rows[origin['source_fixture_row_id']];sig=table[origin['semantic_signature_id']];rule=rules[a['criterion_rule_id']]
            # Independently reconstruct the single-case public ID vocabulary read in the facade.
            parts=key.split('.')
            if parts[0]=='node':
                entity=parts[1];rid='result:disp:'+entity+('' if parts[2]=='magnitude' else ':'+parts[2])
            elif parts[0]=='support':
                entity='anchor';rid='result:support-action:4:case:6:anchor:'+parts[1]
            elif parts[0]=='section':
                entity='pipe';loc,component=parts[1:]
                family='force' if component in ('N','Vy','Vz') else 'moment'
                tail={'N':'axial','Vy':'shear-y','Vz':'shear-z','T':'torsion','My':'bending-y','Mz':'bending-z'}[component]
                stem=f'result:{family}:pipe:'
                rid=stem+tail if loc=='end_i' else stem+tail+':end-j' if loc=='end_j' else stem+loc.replace('_','-')+':'+tail
            elif key=='stress.maximum_absolute_normal': entity='pipe';rid='result:elastic-maximum:4:case:4:pipe'
            else:
                entity='pipe';tail={'axial':'axial-normal','bending_y':'bending-normal-y','bending_z':'bending-normal-z','torsional_shear':'torsional-shear'}[parts[2]]
                rid='result:stress:pipe:'+parts[1].replace('_','-')+':'+tail
            assert sel['id']==rid and sel['entity_ref']==entity,(cid,key,'identity')
            assert sel['kind']==old['kind']==sig['kind'] and sel['unit']==old['unit']==units[key]==sig['unit']
            assert sel['metadata']==old.get('metadata') and sel['dimension']==sig['source_physical_semantic_dimension']
            assert sel['basis_ref']=={'ref_type':'load_case','ref_id':'case'}
            assert origin['metadata_presence']==('present' if 'metadata' in old else 'absent')
            assert rule['dimension_id']==sel['dimension'] and rule['result_family']==sig['family'] and rule['unit_ref']['ref']==units[key]
            r,a0=rule['relative_tolerance_value'],rule['absolute_tolerance_value'];value=expected[key]
            if key.startswith('node.root.'):
                assert value==0 and r==a0==0;prescribed+=1
            elif value:
                assert r==1e-9 and a0==0;nonzeros+=1
                observation=float(value)*(1+2e-9);assert abs(observation-float(value))>r*max(abs(observation),abs(float(value)))
            else:
                zeros+=1;scale=scales[{'mm':'length_mm','rad':'angle_rad','N':'force_N','N*m':'moment_Nm','Pa':'stress_Pa','MPa':'stress_MPa'}[units[key]]]
                assert r==0 and math.isclose(a0,float(scale*D('1e-9')),rel_tol=3e-16,abs_tol=0)
                assert 2*a0>a0>0
        request=read(PACK/(cid+'.runner_input.candidate.json'));preview=read(PACK/(cid+'.preview_request.json'));assert request['solve']['preview_model']==preview
        required_runner={'request_id','operation','operation_ref','project_ref','model_ref','unit_system_ref','load_basis_refs','input_manifest_ref','requested_outputs','privacy','provenance','professional_boundary','tbd_decisions'}
        assert set(request)=={'request','solve'} and set(request['request'])==required_runner
        runner=request['request'];assert runner['operation']=='solve' and runner['load_basis_refs']==[{'ref_type':'load_case','ref_id':'case'}]
        assert runner['provenance']['review_status']=='prepared_for_independent_reference_review'
        model=preview['model'];assert preview['materials']==[] and len(model['load_cases'])==1 and model['components']==model['combinations']==[]
        assert runner['model_ref']=={'ref_type':'model','ref_id':model['project']['id']} and runner['project_ref']=={'ref_type':'project','ref_id':model['project']['id']}
        assert model['load_cases'][0]['pressure_regions']==[] and model['pressure_contract']=={'version':'2.0.0','mode':'exact_straight_pressure_v2'}
        assert [(r['id'],r['position']) for r in model['nodes']]==[('root',{'x':0,'y':0,'z':0}),('tip',{'x':2,'y':0,'z':0})]
        mat=model['materials'][0];assert len(model['materials'])==1 and mat['elastic_modulus']=={'value':200000000000,'unit':'Pa'} and mat['poisson_ratio']=={'value':.25,'unit':'1'} and 'shear_modulus' not in mat
        loads=model['load_cases'][0]['primitive_loads'];wanted=[('global_x',1000,'concentrated_force','N')] if cid=='axial' else [('global_y',30,'concentrated_force','N'),('global_z',-40,'concentrated_force','N'),('rotation_x',20,'concentrated_moment','N*m')]
        assert [(x['direction'],x['magnitude']['value'],x['category'],x['magnitude']['unit']) for x in loads]==wanted and all(x['target']=={'type':'node','node':'tip'} for x in loads)
        assert model['supports'][0]['restraints']==['UX','UY','UZ','RX','RY','RZ'] and len(model['supports'])==1 and model['supports'][0]['node']=='root'
        pipe=model['pipe_segments'][0];assert len(model['pipe_segments'])==1 and pipe['from']=='root' and pipe['to']=='tip' and pipe['y_reference']=={'x':0,'y':1,'z':0}
        assert pipe['section']=={'outside_diameter':{'value':.2,'unit':'m'},'wall_thickness':{'value':.01,'unit':'m'}}
        summary[cid]={'scalar_expectations':73,'prescribed_zero_count':prescribed,'derived_zero_count':zeros,'nonzero_count':nonzeros,'all_scalar_math_and_wire_values_match':True,'all_selector_metadata_signatures_match':True,'all_rules_dimensionally_match':True,'peak_normal_Pa':str(max(maxima)),'tip_displacement_mm':str(norm(tip[:3])*1000),'root_support_wrench':[str(x) for x in reaction],'draft_input_matches_physical_problem':True}
structure=read(PACK/'STRUCTURAL_EXPECTATIONS.json');assert len(structure['required_checks'])==structure['required_checks_per_case']==10 and not structure['scoring_readiness']
assert {x['id'] for x in structure['required_checks']}=={'transport_identity_and_mode','actual_input_binding','ordinary_physics_contract','complete_case_material_section','finite_numerical_standing','complete_unique_scalar_rows','normal_maximum_evidence','governing_location','stress_summary_binding','displacement_summary_binding'}
checks={'status':'PASS bounded independent arithmetic and static binding checks; no target admission','method':'Exact Fraction polynomial equilibrium and virtual-work integrals, Decimal125 AGM pi and Cauchy-Schwarz circular extrema; no author/product imports','packet_files_hashed':len(manifest['files']),'cases':summary,'geometry_binary64_operand_relative_changes':geometry_changes,'structural_obligations':20,'actual_execution':'none; no candidate output or solver/scoring pass','source_packets_unchanged':all(sha(PACK/r['path'])==r['sha256'] for r in manifest['files'])}
(OUT/'CHECKS.json').write_text(json.dumps(checks,indent=2)+'\n')
(OUT/'SCALAR_BACKCHECK.json').write_text(json.dumps(all_comparisons,indent=2)+'\n')
print(json.dumps(checks,indent=2))
