"""Bounded independent producer binding/norm checks; no product or build execution."""
from pathlib import Path
from fractions import Fraction as F
import hashlib,json,math,random,re,struct,subprocess,sys

ROOT=Path('/private/tmp/piping-engine-integration-20260925')
HERE=Path(__file__).resolve().parent
JOIN=HERE.parent.parent
P=ROOT/'projects/chirality-piping'
assertions=0
def sha(path):return hashlib.sha256(Path(path).read_bytes()).hexdigest()
def check(value,label):
    global assertions
    assertions+=1
    assert value,label
class Refused(Exception):pass
def outward(x,upper):
    if not math.isfinite(x) or x<0:raise Refused('arithmetic')
    y=math.nextafter(x,math.inf if upper else -math.inf) if upper or x else 0.
    if not math.isfinite(y):raise Refused('outward')
    return y
def corner(v,upper):
    m=max(v)
    if m==0:return 0.
    squares=[]
    for x in v:
        if x==0:squares.append(0.)
        elif x==m:squares.append(1.)
        else:
            ratio=outward(x/m,upper)
            squares.append(outward(ratio*ratio,upper))
    def add(x,y):return y if x==0 else x if y==0 else outward(x+y,upper)
    total=add(add(squares[0],squares[1]),squares[2])
    root=1. if total==1 else outward(math.sqrt(total),upper)
    return m if root==1 else outward(m*root,upper)
def abs_bounds(interval):
    lo,hi=interval
    return [lo,hi] if lo>=0 else [-hi,-lo] if hi<=0 else [0.,max(-lo,hi)]
def norm(v,intervals):
    if not all(map(math.isfinite,v)) or any(not all(map(math.isfinite,p)) or not p[0]<=x<=p[1] for p,x in zip(intervals,v)):raise Refused('interval')
    ab=list(map(abs_bounds,intervals))
    lo=corner([p[0] for p in ab],False);hi=corner([p[1] for p in ab],True)
    m=max(map(abs,v))
    if m==0:q=0.
    else:
        a,b,c=[x/m for x in v];q=m*math.sqrt((a*a+b*b)+c*c)
    if not math.isfinite(q) or not lo<=q<=hi:raise Refused('representative')
    error=0. if lo==q==hi else outward(max(q-lo,hi-q),True)
    if hi==0:relative=0.
    else:
        if any(x<sys.float_info.min for x in [lo,hi,q]):raise Refused('publication')
        relative=0. if error==0 else outward(error/lo,True)
    if relative>1e-9:raise Refused('criterion')
    return q,[lo,hi],error,relative
def verify_norm(v,intervals,result,name):
    q,(lo,hi),error,relative=result
    ab=[list(map(F,abs_bounds(p))) for p in intervals]
    check(F(lo)**2<=sum(p[0]**2 for p in ab),name+' continuous lower norm')
    check(F(hi)**2>=sum(p[1]**2 for p in ab),name+' continuous upper norm')
    check(F(error)>=max(F(q)-F(lo),F(hi)-F(q)),name+' absolute error')
    check(lo<=q<=hi,name+' representative bounds')
    if hi:
        check(F(relative)>=F(error)/F(lo) and F(relative)<=F(1e-9),name+' composed relative')
    else:check(q==lo==error==relative==0,name+' exact zero')

# New 3D checked norm recipe, separately transcribed from reviewed Rust; exact
# rational squared comparisons are the oracle. This is not a solver/physical
# reference exercise and does not requalify the earlier endpoint helper.
rng=random.Random(20260925)
trials=[([0.,0.,0.],[[0.,0.]]*3),([3.,-4.,0.],[[3.,3.],[-4.,-4.],[0.,0.]]),
        ([1.,math.ulp(0.),0.],[[1.,1.],[math.ulp(0.)]*2,[0.,0.]]),
        ([3.,4.,0.],[[2.9,3.1],[4.,4.],[0.,0.]]),
        ([sys.float_info.min,0.,0.],[[sys.float_info.min]*2,[0.,0.],[0.,0.]])]
for i in range(240):
    scale=math.ldexp(1.,rng.randint(-1074,1021))
    values=[rng.uniform(-3,3)*scale for _ in range(3)]
    radius=[0.,1e-13,3e-10,2e-9][i%4]
    intervals=[[x-abs(x)*radius,x+abs(x)*radius] for x in values]
    trials.append((values,intervals))
accepted=refused=0
for i,(v,p) in enumerate(trials):
    try:r=norm(v,p)
    except Refused:refused+=1;continue
    verify_norm(v,p,r,'trial '+str(i));accepted+=1

captures=json.loads((JOIN/'COMPOSITE_RECEIPT/ACTUAL_CAPTURE_02.json').read_text())
capture_checks=[];norm_records=stress_records=max_records=field_station_comparisons=0
for run in captures['runs']:
    inp=ROOT/run['input'];out=ROOT/run['output']
    check(sha(inp)==run['input_sha256'],'captured input bytes')
    check(sha(out)==run['output_sha256'],'captured output bytes')
    wire=json.loads(out.read_text());request=json.loads(inp.read_text())
    body=wire['source_block_recovery']['body'];rows={r['id']:r for r in wire['results']}
    check(body['policy']=='PHYSICS-SOURCE-1' and body['status']=='qualified','composite policy/complete standing')
    check(wire['producer']['semantic_contract_id'].endswith('/physics-source-1'),'composite namespace')
    check(len(rows)==len(wire['results']),'unique actual rows')
    check(len(body['cases'])==len(request['model']['load_cases']),'case coverage')
    work=body['invocation_work']
    check(work['charged']==sum(c['work']['charged']+c['work']['reserved_unobserved_failure'] for c in body['cases'])+work['publication_charged'],'honest complete invocation ledger')
    check(work['limit']==64000000 and work['charged']<=work['limit'],'invocation limit')
    for c,physical,requested in zip(body['cases'],wire['contract_evidence']['exact_cases'],request['model']['load_cases']):
        case=c['basis_ref']['ref_id'];method=c['selected_method']
        check(case==physical['load_case_id']==requested['id'],'case identity')
        check(method==physical['recovery_method'],'selected method crossbinding')
        if method!='retained_source_blocks_exact_v1':
            check(c['ordinary_attempt']['outcome']=='checks_passed' and c['source'] is None,'ordinary physical own warrant')
            continue
        check(c['ordinary_attempt']['outcome'] in ['sensitive','rejected'],'actual ordinary attempt retained')
        check(c['work']['limit']==8000000 and c['work']['charged']<=8000000,'source selected8M')
        check(requested['pressure_regions']==[],'source producer inventory empty')
        projections={p['functional_id']:p for p in c['projections']}
        all_sections={s['pipe_id']:s['stations'] for s in c['source']['section_functionals']}
        geometry={s['pipe_id']:s for s in physical['pipe_sections']}
        for d in c['derived_checks']:
            calculated=norm(d['values'],d['intervals'])
            verify_norm(d['values'],d['intervals'],calculated,'captured norm')
            check(calculated==(d['value'],d['interval'],d['absolute_error_bound'],d['relative_error_bound']),'actual norm evidence reproduces ordered recipe')
            r=rows[d['result_id']]
            check(r['entity_ref']==d['support_id'] and r['value']==d['value'] and r['basis_ref']['ref_id']==case,'norm row owner/value')
            for f,v,interval in zip(d['functional_ids'],d['values'],d['intervals']):
                check(projections[f]['value']==v and projections[f]['interval']==interval,'norm retained action/interval')
            norm_records+=1
        for m in physical['pipe_stress_extrema']:
            check(m['source_identity_sha256']==c['source']['retained_identity_sha256'],'maximum source identity')
            check(m['basis']=='retained_source_endpoint_normal_max_v1','maximum own basis')
            check(rows[m['result_id']]['value']==m['value_pa'],'maximum row value')
            check(m['area_m2']==geometry[m['pipe_id']]['As_m2'] and m['section_modulus_m3']==geometry[m['pipe_id']]['Z_m3'],'maximum actual source section')
            for e,station in zip(m['endpoints'],[all_sections[m['pipe_id']][0],all_sections[m['pipe_id']][4]]):
                check(e['station_fraction']==station['station_fraction'],'endpoint station')
                check(e['actions']==[station['actions'][i] for i in [0,4,5]],'maximum retained section action')
                check(e['functional_indices']==[station['functional_indices'][i] for i in [0,4,5]],'maximum function index')
                check(e['functional_ids']==[station['functional_ids'][i] for i in [0,4,5]],'maximum function ID')
            max_records+=1
        locations={'end_i':0,'quarter_1':1,'midspan':2,'quarter_3':3,'end_j':4}
        components={'axial_normal_stress':0,'bending_normal_stress_y':4,'bending_normal_stress_z':5,'torsional_shear_stress':3}
        for d in c['section_stress_checks']:
            s=all_sections[d['pipe_id']][locations[d['location']]];i=components[d['component']]
            check(d['functional_id']==s['functional_ids'][i] and d['functional_index']==s['functional_indices'][i] and d['action']==s['actions'][i],'station stress retained cut binding')
            row=rows[d['result_id']]
            check(row['entity_ref']==d['pipe_id'] and row['metadata']['location']==d['location'] and row['metadata']['component']==d['component'],'stress row ownership')
            check(row['unit']=='MPa' and (row['value']==0 or abs(row['value'])>=sys.float_info.min),'existing MPa range guard')
            stress_records+=1
        treatment_ids={r['result_id'] for r in c['rows']}
        check(treatment_ids=={r['id'] for r in wire['results'] if r.get('basis_ref',{}).get('ref_id')==case},'complete case row ledger')
        if run['input'].endswith('/fields.request.json'):
            # Bounded connected-station backcheck requested by physics manager,
            # not a new benchmark/reference stream. Independent cut equilibrium
            # from the authored +x span and nodal input: F is constant and
            # M(x)=M_tip + (L-x)e_x cross F_tip.
            source=request['model'];pipe=source['pipe_segments'][0]
            check(pipe['y_reference']=={'x':0,'y':1,'z':0},'field local axes')
            length=F(source['nodes'][1]['position']['x'])-F(source['nodes'][0]['position']['x'])
            loads={l['direction']:F(l['magnitude']['value']) for l in requested['primitive_loads']}
            fx,fy,fz=[loads['global_'+axis] for axis in 'xyz'];tx,my,mz=[loads['rotation_'+axis] for axis in 'xyz']
            g=geometry[pipe['id']]
            def physical_close(actual,expected,label):
                check(abs(F(actual)-expected)<=F(1e-9)*abs(expected) if expected else actual==0.,label)
            for station in all_sections[pipe['id']]:
                f=F(station['station_fraction']);distance=length*(1-f)
                expected=[fx,fy,fz,tx,my-distance*fz,mz+distance*fy]
                for action,target in zip(station['actions'],expected):
                    physical_close(action['value'],target,'captured field exact cut equilibrium')
                    field_station_comparisons+=1
                location={0.:'end_i',0.25:'quarter_1',0.5:'midspan',0.75:'quarter_3',1.:'end_j'}[float(f)]
                stress_values={
                    'axial_normal_stress':expected[0]/F(g['As_m2'])/1000000,
                    'bending_normal_stress_y':expected[4]/F(g['Z_m3'])/1000000,
                    'bending_normal_stress_z':expected[5]/F(g['Z_m3'])/1000000,
                    'torsional_shear_stress':expected[3]*F(g['ro_m'])/F(g['J_m4'])/1000000,
                }
                for component,target in stress_values.items():
                    actual=[r for r in rows.values() if r['entity_ref']==pipe['id'] and r.get('metadata',{}).get('location')==location and r.get('metadata',{}).get('component')==component]
                    check(len(actual)==1,'unique captured station stress')
                    physical_close(actual[0]['value'],target,'captured field station stress recipe')
                    field_station_comparisons+=1
    capture_checks.append({'input':run['input'],'output':run['output'],'charged':work['charged'],'publication_charged':work['publication_charged']})

# Byte-prefix identities: include punctuation, distinct Unicode encodings,
# multibyte symbols and components which would be ambiguous without lengths.
names=['a','a:b','b:c',':','é','é:','e\u0301','管','管:1','🙂','🙂:x','1:a','0:','a-b']
def width(s):return len(s.encode('utf8'))
def pair(a,b):return f'{width(a)}:{a}:{width(b)}:{b}'
pairs=[pair(a,b) for a in names for b in names]
check(len(set(pairs))==len(pairs),'UTF8 composite pair injectivity')
functional=[f'source-functional:{width(a)}:{a}:{i}' for a in names for i in range(5)]
check(len(set(functional))==len(functional),'UTF8 functional injectivity')
def decode_pair(value):
    data=value.encode('utf8');a,rest=data.split(b':',1);n=int(a);left=rest[:n].decode('utf8');rest=rest[n:];assert rest[:1]==b':';b,rest=rest[1:].split(b':',1);n=int(b);right=rest[:n].decode('utf8');assert len(rest)==n;return left,right
for a in names:
    for b in names:check(decode_pair(pair(a,b))==(a,b),'UTF8 roundtrip exact identity')

# Reconstruct each frozen delta in memory; never edit production or replace the
# original freeze. The current candidate is checked against the same base.
freeze_records=[]
for manifest_name,patch_name,verify_current in [('MANIFEST.json','CANDIDATE.patch',False),('CURRENT_MANIFEST.json','CURRENT_CANDIDATE.patch',True)]:
    manifest=json.loads((HERE.parent/manifest_name).read_text());patch=(HERE.parent/patch_name).read_text()
    check(sha(HERE.parent/patch_name)==manifest['patch_sha256'],patch_name+' hash')
    paths=[]
    for part in re.split(r'(?=^diff --git )',patch,flags=re.M)[1:]:
        lines=part.splitlines(keepends=True);path=re.match(r'diff --git a/(.*?) b/(.*?)\n',lines[0])[2];paths.append(path)
        old=[] if 'new file mode 100644\n' in lines else subprocess.run(['git','-C',str(ROOT),'show',manifest['base']+':'+path],capture_output=True,check=True).stdout.decode().splitlines(keepends=True)
        out=[];cursor=0;i=0
        while i<len(lines):
            h=re.match(r'@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@',lines[i])
            if not h:i+=1;continue
            start=max(0,int(h[1])-1);out.extend(old[cursor:start]);cursor=start;i+=1
            while i<len(lines) and not lines[i].startswith('@@ '):
                line=lines[i]
                if line.startswith((' ','-')):
                    check(old[cursor]==line[1:],'patch source context');cursor+=1
                if line.startswith((' ','+')):out.append(line[1:])
                i+=1
        out.extend(old[cursor:]);reconstructed=''.join(out).encode();entry=next(f for f in manifest['files'] if f['path']==path)
        check(hashlib.sha256(reconstructed).hexdigest()==entry['sha256'],'reconstruct frozen bytes '+path)
        if verify_current:check(reconstructed==(ROOT/path).read_bytes(),'final current source unchanged '+path)
    check(paths==[f['path'] for f in manifest['files']],'exact changed-file scope')
    freeze_records.append({'manifest':manifest_name,'sha256':sha(HERE.parent/manifest_name),'patch':patch_name,'patch_sha256':manifest['patch_sha256'],'source_files':manifest['files']})
scope_args=[sys.executable,str(ROOT/'tools/software_workflow/validate_change_scope.py'),str(ROOT)]
for path in paths:scope_args+=['--allowed',path,'--path',path]
scope=subprocess.run(scope_args,capture_output=True,text=True,check=True)
(HERE/'SCOPE_CHECK.json').write_text(scope.stdout)
for filename in ['public-05.json','namespace-01.json']:
    record=json.loads((JOIN/'COMPOSITE_RECEIPT'/filename).read_text())
    check(record['exit_code']==0 and record['source_unchanged'],'observed successor pass')
    check(record['before_sha256']==record['after_sha256'],'stable observed check source')
    for path,digest in record['after_sha256'].items():check(sha(ROOT/path)==digest,'observed check current source '+path)
report={'actor':'/root/physics_resume/joined_producer_review','requested_by':'/root','coordination_parent':'/root/physics_resume','execution':'Static/source inspection and independent Fraction 3D norm/binding probes with bounded existing-field station backcheck; no Cargo/rustc/npm/native or new reference stream','assertions':assertions,'norm_trials':len(trials),'norm_accepts':accepted,'norm_refusals':refused,'captured_norm_records':norm_records,'captured_section_stress_records':stress_records,'captured_maximum_records':max_records,'field_station_action_stress_comparisons':field_station_comparisons,'captures':capture_checks,'freezes':freeze_records}
(HERE/'VERIFICATION.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({k:v for k,v in report.items() if k not in ['captures','freezes']}))
