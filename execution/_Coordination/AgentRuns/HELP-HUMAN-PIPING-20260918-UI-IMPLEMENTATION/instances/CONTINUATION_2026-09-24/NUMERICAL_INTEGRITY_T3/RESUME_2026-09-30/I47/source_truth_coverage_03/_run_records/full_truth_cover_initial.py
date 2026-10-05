#!/usr/bin/env python3
"""Finite exact required-material-cover audit of frozen I47 captures. No product imports."""
import sys
sys.dont_write_bytecode=True
from pathlib import Path
from fractions import Fraction as Q
import importlib.util,json,hashlib
HERE=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('frozen',HERE/'FROZEN_ORACLE.py')
o=importlib.util.module_from_spec(spec);spec.loader.exec_module(o)
def data(name):return json.loads((HERE/name).read_text())
def dump(name,x):(HERE/name).write_text(json.dumps(x,indent=2)+'\n')
def dyadic_profile(q):
    q=Q(q);d=q.denominator
    assert d>0 and d&(d-1)==0
    n=abs(q.numerator)
    sig=n.bit_length()-(n&-n).bit_length()+1 if n else 0
    return {'fraction':str(q),'significant_bits':sig,'denominator_exponent':d.bit_length()-1,'fits_1024':sig<=1024}
def fixed_specimen(r):
    req=r['request'];m=req['model'];loaded=r['label'].endswith('loaded')
    assert len(m['nodes'])==2 and len(m['pipe_segments'])==1 and len(m['supports'])==1 and len(m['load_cases'])==1
    assert m['nodes'][0]['position']=={'x':0.0,'y':0.0,'z':0.0}
    assert m['nodes'][1]['position']=={'x':1.0,'y':0.0,'z':0.0}
    assert r['input']['nodes']==[[0.0,0.0,0.0],[1.0,0.0,0.0]]
    pipe=m['pipe_segments'][0]
    assert pipe['from']==m['nodes'][0]['id'] and pipe['to']==m['nodes'][1]['id']
    assert pipe['y_reference']=={'x':0.0,'y':1.0,'z':0.0}
    assert m.get('combinations',[])==[] and m.get('components',[])==[]
    support=m['supports'][0]
    assert support['family']=='anchor' and support['node']==m['nodes'][0]['id']
    assert set(support['restraints'])=={'UX','UY','UZ','RX','RY','RZ'}
    case=m['load_cases'][0];loads=case['primitive_loads']
    assert len(loads)==3
    for item,(direction,unit) in zip(loads,[('UX','N'),('UY','N'),('RX','N*m')]):
        assert item['direction']==direction and item['magnitude']=={'unit':unit,'value':float(loaded)}
        assert item['target']=={'type':'node','node':m['nodes'][1]['id']}
    D,t=Q(r['input']['D']),Q(r['input']['t']);assert 0<t<D/2
    return loaded

records=data('CAPTURES.json');old=data('OLD_RESULTS.json')
assert [r['label'] for r in records]==['point-zero','point-loaded','interpolated-zero','interpolated-loaded']
assert len(old['checks'])==4
results=[];changes=[];false_passes=[];ux=None;candidate=[];witness=None
for rec,previous in zip(records,old['checks']):
    loaded=fixed_specimen(rec)
    selection=o.select_source(rec) # exact captured selected operands, same frozen method
    prior=o.check(rec['label'],rec['input'],rec['rows'],rec['verdicts'])
    assert prior==previous,('old numerical result not exactly reproduced',rec['label'])
    assert o.g5a_check(rec['label'],rec['input'],rec['rows'],rec['verdicts'],rec['g5a'])==old['g5a'][len(results)]
    assert o.observable_check(rec)==old['observables'][len(results)]
    Es,Gs=rec['input']['source_E'],rec['input']['source_G'];Eh,Gh=Q(rec['input']['E']),Q(rec['input']['G'])
    E=(min(Es,Eh),max(Es,Eh));G=(min(Gs,Gh),max(Gs,Gh));assert E[0]>0 and G[0]>0
    # This fixed midpoint case has exact dyadic h, every numerator product/sum,
    # and quotient with <<1024 significant bits. I36's directed X_- and X_+
    # therefore coincide at X_s; its H_X is exactly the endpoint hull used here.
    profiles={key:dyadic_profile(v) for key,v in [('Es',Es),('Gs',Gs),('Eh',Eh),('Gh',Gh)]}
    if selection['kind']=='interpolated':
        for prop in ['e','g']:
            trace=selection['interpolation'][prop]
            for i,q in enumerate(trace['products']):profiles[prop+'_product_'+str(i)]=dyadic_profile(Q(q))
            profiles[prop+'_numerator']=dyadic_profile(Q(trace['numerator']))
            profiles[prop+'_denominator']=dyadic_profile(Q(trace['denominator']))
        assert all(p['fits_1024'] for p in profiles.values())
    variants=[('geometry_exact_E_exact_G',Es,Gs),('geometry_resolved_E_resolved_G',Eh,Gh),('geometry_exact_E_resolved_G',Es,Gh),('geometry_resolved_E_exact_G',Eh,Gs)]
    previous_by_row={x['row']:x for x in previous['all']}
    node_ids=[r['entity_ref'] for r in rec['rows'] if r['kind']=='displacement_magnitude']
    compared=[]
    for index,row in enumerate(rec['rows']):
        if row['kind'] in ['linear_solver_mode_basis','modulus_basis_record']:continue
        original=previous_by_row[index];v=rec['verdicts'][index];n=o.normalized(row)
        assert original['normalized_bits']==o.bits(n)==v['normalized_bits']
        assert original['class']==v['class'] and original['scale_bits']==v['scale_bits']
        loc=row.get('metadata',{}).get('location')
        if row['kind']=='support_reaction_component_v2':loc=row['metadata']['component']
        node=node_ids.index(row['entity_ref']) if row['entity_ref'] in node_ids else None
        laws=[]
        for name,e,g in variants:
            operands=dict(rec['input'],source_E=e,source_G=g)
            a=o.source_pair(operands,row['kind'],loc,node,loaded,True)
            laws.append((name,a,o.raw(a,row)))
        k=o.source_pair(rec['input'],row['kind'],loc,node,loaded,False)
        laws.append(('represented_K_with_required_Z_cover',k,o.raw(k,row)))
        # Positive monotonic E/G dependence or exact zero/static independence
        # (proved in METHOD.md) makes the four source corners cover the whole rectangle.
        material_hull=(min(x[1][0] for x in laws[:4]),max(x[1][1] for x in laws[:4]))
        total_hull=o.hull(material_hull,k)
        checks=[]
        for pi,predicate in enumerate(original['predicates']):
            name=predicate['name'];is_raw=name=='DecimalRaw';y=row['value'] if is_raw else n
            allowance=Q(predicate['allowance'])
            distances=[(label,*o.distance_bounds(y,raw if is_raw else si)) for label,si,raw in laws]
            lower=max(z[1] for z in distances);upper=max(z[2] for z in distances)
            assert lower>allowance or upper<=allowance,('unproved truth status',rec['label'],index,name)
            truth_pass=upper<=allowance
            candidate_pass=v['predicates'][pi]
            assert type(candidate_pass) is bool
            if candidate_pass and not truth_pass:
                alert={'case':rec['label'],'row':index,'id':row['id'],'predicate':name,'allowance':str(allowance),'error_lower':str(lower),'error_upper':str(upper)}
                false_passes.append(alert);dump('FALSE_PASS.json',false_passes);print('FALSE_PASS',json.dumps(alert),flush=True)
                raise AssertionError('candidate PASS violates complete material cover')
            failing_laws=[label for label,lo,hi in distances if lo>allowance]
            checks.append({'name':name,'allowance':str(allowance),'candidate_passed':candidate_pass,'old_two_readout_truth_passed':predicate['passed'],'full_cover_truth_passed':truth_pass,'error_lower':str(lower),'error_upper':str(upper),'failing_laws':failing_laws,'law_distances':[{'law':label,'lower':str(lo),'upper':str(hi)} for label,lo,hi in distances]})
        full_pass=all(x['full_cover_truth_passed'] for x in checks)
        category='candidate_pass_full_cover_pass' if v['passed'] else 'truth_pass_candidate_refusal' if full_pass else 'required_truth_miss_candidate_refusal'
        assert not v['passed'] or full_pass
        item={'row':index,'id':row['id'],'kind':row['kind'],'candidate_passed':v['passed'],'candidate_failed':v['failed'],'class':v['class'],'scale_bits':v['scale_bits'],'bound_bits':v['bound_bits'],'normalized_bits':v['normalized_bits'],'old_truth_passed':original['exact_passed'],'full_cover_truth_passed':full_pass,'category':category,'predicates':checks,'material_rectangle_si':[str(x) for x in material_hull],'full_cover_si':[str(x) for x in total_hull],'laws':[{'name':label,'si':[str(x) for x in si],'raw':[str(x) for x in raw]} for label,si,raw in laws]}
        compared.append(item)
        if original['exact_passed']!=full_pass or any(x['old_two_readout_truth_passed']!=x['full_cover_truth_passed'] for x in checks):
            changes.append({'case':rec['label'],'row':index,'id':row['id'],'old_truth_passed':original['exact_passed'],'full_cover_truth_passed':full_pass,'changed_predicates':[x for x in checks if x['old_two_readout_truth_passed']!=x['full_cover_truth_passed']]})
        if rec['label']=='interpolated-loaded' and row['id']=='result:disp:node-N-DEC092-TIP:ux':
            law=next(si for label,si,_ in laws if label=='geometry_resolved_E_resolved_G')
            es_law=next(si for label,si,_ in laws if label=='geometry_exact_E_exact_G')
            assert law==tuple(x*Es/Eh for x in es_law)
            lower,upper=o.distance_bounds(n,law);a=Q(checks[0]['allowance']);assert lower>a
            ux={'row':index,'id':row['id'],'normalized_bits':o.bits(n),'source_exact_E':str(Es),'source_resolved_E':str(Eh),'geometry_with_resolved_E_si':[str(x) for x in law],'exact_rescaling_agrees':True,'error_lower':str(lower),'error_upper':str(upper),'allowance_exact':str(a),'error_lower_display':float(lower),'allowance_display':float(a),'ratio_lower':str(lower/a),'ratio_lower_display':float(lower/a),'candidate_passed':v['passed'],'old_two_readout_truth_passed':original['exact_passed'],'full_cover_truth_passed':full_pass}
        if witness is None and rec['label']=='interpolated-loaded' and category=='truth_pass_candidate_refusal':
            witness={'case':rec['label'],**item}
    assert len(compared)==73
    summary={'case':rec['label'],'mechanical_rows':len(compared),'candidate_numeric_passes':sum(x['candidate_passed'] for x in compared),'old_truth_miss_rows':previous['actual_misses'],'new_truth_miss_rows':sum(not x['full_cover_truth_passed'] for x in compared),'old_conservative_rows':len(previous['conservative_only_refusals']),'new_conservative_rows':sum(x['category']=='truth_pass_candidate_refusal' for x in compared),'candidate_full_case':rec['g5a']['full_case'],'g5a_unchanged':old['g5a'][len(results)],'false_passes':0}
    results.append({'summary':summary,'source_E_interval':[str(x) for x in E],'source_G_interval':[str(x) for x in G],'directed_material_exactness':profiles,'selection':selection,'rows':compared})
assert ux is not None and witness is not None and not false_passes
root=data('ROOT_CHECK_COPY.json');assert Q(root['allowance_exact_m'])==Q(ux['allowance_exact'])
root_q=tuple(Q(x) for x in root['q_interval']);our_q=tuple(Q(x) for x in ux['geometry_with_resolved_E_si'])
assert max(root_q[0],our_q[0])<=min(root_q[1],our_q[1])
ux['root_independent_brackets_overlap']=True
output={'source_pin':'d0daa18717f8243a7232e898c9ef9b4f4d18d9e4','scope':'fixed four one-member aligned cantilever captures only','complete_mechanical_rows':292,'required_source_cover':'full independent positive E/G rectangle including exact/resolved endpoints with source annulus, separately hulled with admitted K readout and represented Z cover','false_passes':false_passes,'cases':results,'changed_rows':changes,'ux_correction':ux,'one_conservative_witness':witness,'unchanged':'all captured requests, values, classes, scales, bounds, predicates, G5a and full-case outcomes; no runtime'}
dump('FULL_COVER_RESULTS.json',output)
dump('COMPARISON.json',{'case_summaries':[x['summary'] for x in results],'changed_rows':changes,'old_result_objects_reproduced_exactly':True,'captured_fields_unchanged':True,'ux':ux,'conservative_witness':{'case':witness['case'],'row':witness['row'],'id':witness['id'],'candidate_passed':witness['candidate_passed'],'full_cover_truth_passed':witness['full_cover_truth_passed']}})
print(json.dumps({'cases':[x['summary'] for x in results],'changed_rows':[(x['case'],x['row'],x['id']) for x in changes],'ux':{k:v for k,v in ux.items() if k.endswith('display')},'one_conservative_witness':{'case':witness['case'],'row':witness['row'],'id':witness['id']},'false_passes':0},indent=2))
