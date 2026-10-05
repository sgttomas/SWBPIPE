"""Independent fixed-case exact rational/directed oracle over actual I50 captures.
No product imports. Frame, static equilibrium, relative torsion and predicates are
derived from the exact captured input. All candidate PASS predicates are checked.
"""
from fractions import Fraction as F
from pathlib import Path
import json, math, struct, sys
def bits(x): return struct.pack('>d',float(x)).hex()
def dec(x): return struct.unpack('>d',bytes.fromhex(x))[0]
def point(x): return F(x),F(x)
def add(a,b): return a[0]+b[0],a[1]+b[1]
def mul(a,b):
    x=[u*v for u in a for v in b];return min(x),max(x)
def div(a,b):
    assert b[0]>0
    x=[u/v for u in a for v in b];return min(x),max(x)
def hull(a,b):return min(a[0],b[0]),max(a[1],b[1])
def neg(a):return -a[1],-a[0]
def sqrtq(x):
    assert x>=0
    n=640;k=math.isqrt((x.numerator<<(2*n))//x.denominator);lo=F(k,1<<n)
    return (lo,lo) if lo*lo==x else (lo,F(k+1,1<<n))
def root(a):return sqrtq(a[0])[0],sqrtq(a[1])[1]
def atan_inverse(q):
    n=450;s=F(0);power=F(1,q)
    for k in range(n):s+=(-1 if k%2 else 1)*power/(2*k+1);power/=q*q
    # Even alternating prefix ends negative; next positive term bounds remainder.
    return s,s+power/(2*n+1)
PI=mul(point(4),add(atan_inverse(2),atan_inverse(3)))
def norm(xs):
    total=point(0)
    for a in xs:
        lo=F(0) if a[0]<=0<=a[1] else min(abs(a[0]),abs(a[1]))
        hi=max(abs(a[0]),abs(a[1]));total=add(total,(lo*lo,hi*hi))
    return root(total)
def distance(n,a):
    n=F(n);return (a[0]-n if n<a[0] else n-a[1] if n>a[1] else F(0),max(n-a[0],a[1]-n))
def norm_value(r):
    y=r['value'];return y/1000.0 if r['unit']=='mm' else y*1e6 if r['unit']=='MPa' else y
def raw_truth(a,r):return mul(a,point(1000)) if r['unit']=='mm' else div(a,point(1000000)) if r['unit']=='MPa' else a
def kind(r):
    k=r['kind']
    if k=='displacement_magnitude' or k.startswith('global_nodal_displacement'):return 0
    if k.startswith('global_nodal_rotation'):return 1
    if k.startswith('element_local_') and 'force' in k:return 2
    if k.startswith('element_local_') and 'moment' in k:return 3
    if k=='support_reaction_component_v2':return 2 if r['metadata']['component'][0]=='F' else 3
    if k=='support_reaction_force_magnitude_v2':return 2
    if k=='support_reaction_moment_magnitude_v2':return 3
def ru(q):
    x=float(q);return math.nextafter(x,math.inf) if F(x)<q else x
def sharper64(n,s):
    a0=2.0**-64*max(abs(n),s);a1=a0*(1+2.0**-21)
    u0=2.0**-53*abs(n);u1=u0+2.0**-1074
    return a1+u1
def check(record):
    req=record['request'];m=req['model'];source=record['source'];facts=record['facts'][0]
    assert len(m['nodes'])==2 and len(m['pipe_segments'])==1 and len(m['supports'])==4
    assert [m['nodes'][i]['position'] for i in range(2)]==[dict(x=0,y=0,z=0),dict(x=1,y=2,z=2)]
    assert m['pipe_segments'][0]['y_reference']==dict(x=1,y=0,z=0)
    assert len(source['members'])==1 and len(source['nodes'])==2
    for node,raw_node in zip(source['nodes'],m['nodes']):
        assert [bits(v) for v in node]==[bits(raw_node['position'][axis]) for axis in 'xyz']
    member=source['members'][0]
    assert member['id']==0 and member['nodes']==[0,1] and member['y_reference']==[1.0,0.0,0.0]
    assert all(bits(member[k])==bits(facts[k]) for k in ['A','I','J'])
    # ex=d/3. Projecting y_ref gives (8,-2,-2)/9, with squared norm 8/9;
    # ey=(4,-1,-1)/(3 sqrt2), ez=(0,1,-1)/sqrt2. Exact dot/cross checks
    # are performed on the numerator vectors, keeping sqrt2 symbolic.
    d=[F(1),F(2),F(2)];ex=[x/3 for x in d];yn=[F(4),F(-1),F(-1)];zn=[F(0),F(1),F(-1)]
    dot=lambda a,b:sum(x*y for x,y in zip(a,b))
    cross=lambda a,b:[a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]]
    assert dot(ex,ex)==1 and dot(yn,yn)==18 and dot(zn,zn)==2
    assert dot(ex,yn)==dot(ex,zn)==dot(yn,zn)==0 and cross(ex,yn)==[3*x for x in zn]
    loads=m['load_cases'][0]['primitive_loads'];alpha=F(loads[0]['magnitude']['value']);M=[F(x['magnitude']['value']) for x in loads]
    assert M==[alpha*x for x in d] and len(loads)==3
    assert [x['direction'] for x in loads]==['RX','RY','RZ']
    assert m['supports'][0]['restraints']==['UX','UY','UZ']
    ks=[F(s['stiffness']['value']['value']) for s in m['supports'][1:]]
    assert ks==[144,1000000,1000000]
    assert [(s['id'],s['node'],s['stiffness']['dof']) for s in m['supports'][1:]]==[(f'spring:N0:{i}','N0',axis) for i,axis in enumerate(['RX','RY','RZ'])]
    theta=[M[i]/ks[i] for i in range(3)];translation=cross(theta,d)
    assert translation[0]==0 and translation[1]==-translation[2]
    T=dot(M,ex);assert T==3*alpha
    mat=m['materials'][0];E=F(mat['elastic_modulus']['value']);G=F(mat['shear_modulus']['value'])
    assert (E,G)==(200000000000,80000000000) and source['members'][0]['E']==float(E) and source['members'][0]['G']==float(G)
    D=F(m['pipe_segments'][0]['section']['outside_diameter']['value']);t=F(m['pipe_segments'][0]['section']['wall_thickness']['value'])
    assert facts['D']==float(D) and facts['t']==float(t) and D>2*t>0
    c=D/2;ri=c-t;A=mul(PI,point(t*(D-t)));I=mul(A,point((c*c+ri*ri)/4));J=mul(point(2),I)
    assert len(source['springs'])==3 and source['supports'][0]['rigid']==[True]*3+[False]*3
    for i,s in enumerate(source['springs']):assert s==dict(id=i,node=0,axis=i+3,k=float(ks[i]))
    for i,g in enumerate(source['supports']):
        assert g['id']==i and g['node']==0 and g['springs']==([] if i==0 else [i-1])
        if i:assert g['rigid']==[False]*6
    # All arbitrary positive bending/axial coefficients preserve this static pure
    # torsion solution; only positive GJ enters relative rotation. Each expression
    # below is monotone in 1/J, so endpoint intervals cover the complete J bracket.
    def truth(r,geometric):
        jj=J if geometric else point(facts['J']);cc=point(c if geometric else facts['c'])
        rotations=[add(point(theta[i]),div(point(T*3*ex[i]),mul(point(G),jj))) for i in range(3)]
        k=r['kind'];entity=r['entity_ref'];meta=r.get('metadata') or {}
        if k.startswith('global_nodal_displacement_'):return point(0 if entity=='N0' else translation['xyz'.index(k[-1])])
        if k.startswith('global_nodal_rotation_'):
            axis='xyz'.index(k[-1]);return point(theta[axis]) if entity=='N0' else rotations[axis]
        if k=='displacement_magnitude':return point(0) if entity=='N0' else norm([point(x) for x in translation])
        if k.startswith('support_reaction_'):
            v=[F(0)]*6
            if entity!='rigid:N0':
                i=['spring:N0:0','spring:N0:1','spring:N0:2'].index(entity);v[3+i]=-M[i]
            if k=='support_reaction_component_v2':return point(v[['Fx','Fy','Fz','Mx','My','Mz'].index(meta['component'])])
            return norm([point(x) for x in (v[:3] if k=='support_reaction_force_magnitude_v2' else v[3:])])
        if k=='element_local_torsional_moment':return point(-T if meta['location']=='end_i' else T)
        if k=='element_local_torsional_shear_stress':return div(mul(point(T),cc),jj)
        if k.startswith('element_local_') or k=='pipe_elastic_normal_stress_maximum_v2':return point(0)
        raise AssertionError(k)
    rows=record['envelope']['results'];vs=record['verdicts'];complete=len(vs)==len(rows)
    if 'observations' in record:
        captured=record['observations'];assert record['invocation_calls']==record['observation_calls']==1
        assert captured['case']=='case' and captured['mode']==record['mode']
        mode_row=next(r for r in rows if r['kind']=='linear_solver_mode_basis')
        expected=1.0 if record['mode']=='sparse_interactive' else 2.0
        assert bits(mode_row['value'])==captured['mode_bits']==bits(expected)
        assert mode_row['metadata']['basis']==captured['mode_basis']
        parity=[r for r in rows if r['kind']=='sparse_live_path_dense_parity_relative_delta']
        assert len(parity)==int(captured['parity_produced']) and captured['parity_produced']==(captured['parity'] is not None)
        assert record['mode']=='dense_scrutiny' or not parity
        if parity:
            p=parity[0];assert math.isfinite(p['value']) and p['value']>=0
            assert bits(p['value'])==captured['parity']['bits'] and p['metadata']['basis']==captured['parity']['basis']
    primary=[0.0]*4
    for r in rows:
        k=kind(r)
        if k is not None and not(r['entity_ref']=='N0' and r['kind'].startswith('global_nodal_displacement_')):primary[k]=max(primary[k],abs(norm_value(r)))
    tr,ro,fo,mo=primary;scales=[max(tr,3.0*ro),max(ro,tr/3.0),max(fo,mo/3.0),max(mo,3.0*fo)]
    assert record['native']['precision']!=512, 'p512 floors require explicit separate captured floor binding'
    ancillary={'linear_solver_mode_basis','sparse_live_path_dense_parity_relative_delta'}
    results=[];candidate_passes=0;false_passes=[];unproved=[];conservative=[]
    for i,r in enumerate(rows):
        if r['kind'] in ancillary:
            if complete:
                v=vs[i];assert v['row']==i and v['class']=='None' and v['scale_bits']=='0000000000000000' and v['predicates']==[None]*4 and v['passed']
                assert v['normalized_bits']==bits(r['value'])
            continue
        n=norm_value(r);k=kind(r)
        scale=scales[k] if k is not None else scales[2]/facts['A']+(dec('4006a09e667f3bcd') if r['kind']=='pipe_elastic_normal_stress_maximum_v2' else 1.0)*(scales[3]/facts['Z'])
        input_derived=r['entity_ref']=='N0' and r['kind'].startswith('global_nodal_displacement_')
        cls='input' if input_derived else 'absolute' if scale<2.0**-988 or abs(n)<2.0**-34*scale else 'relative'
        gs,kk=truth(r,True),truth(r,False);truths=[gs,kk]
        low=max(distance(n,x)[0] for x in truths);high=max(distance(n,x)[1] for x in truths)
        b=None
        if cls=='input':tests=[('InputDerived',low,high,F(0))]
        elif cls=='absolute':
            b=ru(F(scale)*F(2)**-64) if scale else 0.0
            if 0<scale<2.0**-988:b=ru(F(b)+F(ru(F(2)**-53*abs(F(n))))+F(2)**-1074)
            tests=[('Absolute',low,high,F(b))]
        else:
            exact=F(2)**-64*max(abs(F(n)),F(scale))*(1+F(2)**-21)+F(2)**-53*abs(F(n))+F(2)**-1074
            raw=[raw_truth(x,r) for x in truths]
            tests=[('SharperExact',low,high,exact),('SharperBinary64',low,high,F(sharper64(n,scale))),('DecimalSi',low,high,abs(F(n))/10**9),('DecimalRaw',max(distance(r['value'],x)[0] for x in raw),max(distance(r['value'],x)[1] for x in raw),abs(F(r['value']))/10**9)]
        v=vs[i] if complete else None
        if v:
            assert v['row']==i and bits(n)==v['normalized_bits'] and bits(scale)==v['scale_bits'],(record['mode'],i,'bits')
            expected='Some(InputDerived)' if cls=='input' else 'Some(RelativeVerified)' if cls=='relative' else 'Some(AbsoluteVerified { bound_bits: '+str(int(bits(b),16))+' })'
            assert v['class']==expected,(i,expected,v['class'])
            assert v['passed']==all(p is not False for p in v['predicates'])
        evaluated=[]
        for j,(name,lo,hi,a) in enumerate(tests):
            verdict='pass' if hi<=a else 'fail' if lo>a else 'unproved'
            if verdict=='unproved':unproved.append([i,name])
            if v and v['predicates'][j]:
                candidate_passes+=1
                if verdict!='pass':false_passes.append([i,name,verdict])
            if v and v['predicates'][j] is False and verdict=='pass':conservative.append([i,name])
            separation=max(gs[0]-kk[1],kk[0]-gs[1],F(0))
            # With the actual fixed S, any center satisfying the represented
            # sharper bound obeys |n| <= (|qK|+aS+d)/(1-a-b), since max<=S+|n|.
            # Bound the allowance over that entire necessary center range.
            aa=F(2)**-64*(1+F(2)**-21);bb=F(2)**-53;dd=F(2)**-1074
            center_bound=(max(abs(kk[0]),abs(kk[1]))+aa*F(scale)+dd)/(1-aa-bb)
            allowance_upper=aa*max(center_bound,F(scale))+bb*center_bound+dd
            evaluated.append({'name':name,'verdict':verdict,'error_lower':str(lo),'error_upper':str(hi),'allowance':str(a),'candidate':None if v is None else v['predicates'][j],'separation_exceeds_two_actual_allowances':name!='DecimalRaw' and separation>2*a,
                'fixed_scale_no_common_sharper_center':name=='SharperExact' and separation>2*allowance_upper,
                'readout_separation_lower':str(separation),'fixed_scale_all_centers_allowance_upper':str(allowance_upper)})
        results.append({'row':i,'id':r['id'],'kind':r['kind'],'class':cls,'source_si':[str(x) for x in gs],'represented_si':[str(x) for x in kk],'source_raw':[str(x) for x in raw_truth(gs,r)],'represented_raw':[str(x) for x in raw_truth(kk,r)],'predicates':evaluated,'candidate_passed':None if v is None else v['passed']})
    assert len(results)==97
    assert not unproved and not false_passes,(unproved,false_passes)
    # Independent unchanged zero/sanity G5a checks over actual final normalized rows.
    resolution=[dec(x) for x in record['native']['resolution'][0][1:]]
    first_zero=next((i for i,r in enumerate(rows) if kind(r) in (2,3) and resolution[kind(r)-2]==0 and bits(norm_value(r))!='0000000000000000'),None)
    hats=[max(resolution[0],resolution[1]/3.0),max(resolution[1],resolution[0]*3.0)]
    upper=[x*dec('3ff0000000001000') for x in hats]
    if complete:
        if first_zero is not None:assert record['g5a']=='Some(Zero { row: '+str(first_zero)+' })'
        else:
            assert all(upper[j]>=scales[j+2] for j in range(2))
            rn=lambda x:float(F(x))
            ka=rn(F(rn(E*F(facts['A'])))/3);kt=rn(F(rn(G*F(facts['J'])))/3)
            norms=[]
            for rotation in [False,True]:
                prefix='global_nodal_rotation_' if rotation else 'global_nodal_displacement_'
                ends=[]
                for node in ['N0','N1']:
                    v=[abs(norm_value(next(r for r in rows if r['entity_ref']==node and r['kind']==prefix+axis))) for axis in 'xyz']
                    ends.append(rn(F(rn(F(v[0])+F(v[1])))+F(v[2])))
                norms.append(rn(F(ends[0])+F(ends[1])))
            lower=[]
            for j,coefficient in enumerate([ka,kt]):
                threshold=rn(F(2)**-59*F(scales[j]))
                lower.append(0.0 if norms[j]<=threshold else rn(F(coefficient)*F(rn(F(norms[j])-F(rn(F(2)**-60*F(scales[j])))))))
            assert all(upper[j]>=lower[j] for j in range(2))
            assert record['g5a']=='None'
    # Independent observable checks: complete four-support component/magnitude
    # roster, exact mathematical norm bracket, maximum midpoint and headlines.
    byid={r['id']:r for r in rows};assert len(byid)==len(rows)
    evidence=record['envelope']['contract_evidence'];assert evidence['combination_gates']==[]
    case=evidence['preview_cases'][0];assert case['load_case_id']=='case'
    assert sorted(case['support_attribution']['attributed_support_ids'])==sorted(s['id'] for s in m['supports']) and case['support_attribution']['withheld']==[]
    for support in m['supports']:
        sr=[r for r in rows if r['entity_ref']==support['id'] and r['kind'].startswith('support_reaction_')];assert len(sr)==8
        for names,kind_name in [(['Fx','Fy','Fz'],'support_reaction_force_magnitude_v2'),(['Mx','My','Mz'],'support_reaction_moment_magnitude_v2')]:
            vv=[next(r['value'] for r in sr if r['kind']=='support_reaction_component_v2' and r['metadata']['component']==name) for name in names]
            actual=next(r['value'] for r in sr if r['kind']==kind_name)
            bracket=norm([point(v) for v in vv]);tolerance=64.0*2.0**-52*max(abs(actual),2.0**-1022)
            assert distance(actual,bracket)[1]<=F(tolerance)
    for ex in case['pipe_stress_extrema']:
        r=byid[ex['result_id']];assert r['entity_ref']==ex['pipe_id'] and r['value']==ex['value_lower_pa']+.5*(ex['value_upper_pa']-ex['value_lower_pa'])
    for field,k in [('max_displacement','displacement_magnitude'),('max_open_formula_stress','pipe_elastic_normal_stress_maximum_v2')]:
        best=sorted([r for r in rows if r['kind']==k],key=lambda r:(-r['value'],r['entity_ref']))[0]
        h=record['envelope']['summary'][field]
        assert (h['result_ref'],h['location_ref'],h['unit'],bits(h['value']))==(best['id'],best['entity_ref'],best['unit'],bits(best['value']))
    return {'mode':record['mode'],'complete_candidate':complete,'mechanical_rows':97,'candidate_pass_predicates':candidate_passes,'false_pass_predicates':false_passes,'unproved':unproved,'truth_miss_rows':sum(any(p['verdict']=='fail' for p in x['predicates']) for x in results),'conservative_predicates':conservative,'source_J':[str(x) for x in J],'represented_J':str(F(facts['J'])),
        'represented_Z_cover':[str(x) for x in hull(point(facts['Z']),div(point(facts['I']),point(facts['c'])))],
        'g5a_zero_row':first_zero,'g5a_independently_checked':complete,'observable_checks':True,'primary_scales':primary,'coupled_scales':scales,'all':results}
records=[json.loads(line.split('I50_RECORD ',1)[1]) for line in Path(sys.argv[1]).read_text().splitlines() if 'I50_RECORD ' in line]
assert len(records)==2
answer=[check(r) for r in records]
Path(sys.argv[2]).write_text(json.dumps(answer,indent=2)+'\n')
print(json.dumps([{k:v for k,v in a.items() if k not in ['all','source_J','represented_J','conservative_predicates']}|{'conservative_predicates':len(a['conservative_predicates'])} for a in answer]))
