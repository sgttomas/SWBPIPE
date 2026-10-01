#!/usr/bin/env python3
"""Exactly three input-pinned source controls; independent rational truth.

<VENV>/bin/python -B source_controls.py freeze DESTINATION
<VENV>/bin/python -B source_controls.py compare OUTPUT_TSV REPORT_JSON
Freeze reads this script's sibling inputs/A1_EXTRA_INPUTS.json. Comparison reads
its sibling TRUTH.json, never solver output to establish mathematical truth.
Reported-claim checks do not certify a repaired scale/bound construction.
"""
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import struct
import sys

sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
IDS = {'EXTRA-FM-01', 'EXTRA-MF-01', 'EXTRA-ZR-01'}
INPUT_SHA = '546b3643b496db9bb259694baf638153b535a55ff87d9d775b27cfcb61a38efe'
TRUTH_SHA = 'aac399bfb68bbb8b1cbcce08cc2901229535e88860dc5cc812cf08c9fe3ba0ee'
OLD_ORACLE_SHA = 'fa8ea6f303148d9babb5d9fe6c53f64377b13cb130d03d076d4cec7d3f4c15e0'
old_path = HERE.parent/'exact_oracle.py'
assert hashlib.sha256(old_path.read_bytes()).hexdigest() == OLD_ORACLE_SHA
spec = importlib.util.spec_from_file_location('independent_exact_primitives',old_path)
q = importlib.util.module_from_spec(spec)
spec.loader.exec_module(q)
q.self_check()


def need(ok, message):
    if not ok:
        raise ValueError(message)


def finite_bits(s, nonnegative=False):
    need(isinstance(s,str) and re.fullmatch(r'[0-9a-f]{16}',s), 'invalid bit string')
    b = int(s,16)
    need(((b>>52)&2047) != 2047, 'nonfinite binary64')
    need(b != 1<<63, 'noncanonical negative zero')
    need(not nonnegative or b < 1<<63, 'negative scale/bound')
    return q.value(b)


def validate_source(p):
    """Check constructor requirements and this fixed assignment's topology.

    Scope restrictions are assertions, not substitutions or source repairs.
    """
    need(set(p) == {'nodes_bits','members','constraints','loads','springs',
                    'directional_springs','stations','supports'}, 'unexpected primitive field')
    need(len(p['nodes_bits']) == 2, 'requires the two declared nodes')
    nodes = [[finite_bits(x) for x in v] for v in p['nodes_bits']]
    need(all(len(v)==3 for v in nodes), 'coordinate arity')
    need(nodes[0]==[0,0,0] and nodes[1][1:]==[0,0] and nodes[1][0]>0, 'fixed axial geometry')
    need(len(p['members'])==1, 'fixed single member')
    m=p['members'][0]
    need((m['id'],m['node_i'],m['node_j'])==(1,0,1), 'member identity/nodes')
    for key in ('elastic_modulus_bits','shear_modulus_bits','area_bits',
                'second_moment_y_bits','second_moment_z_bits','torsion_constant_bits'):
        v=finite_bits(m[key]);need(v>0,'nonpositive property '+key)
        if key not in ('elastic_modulus_bits','shear_modulus_bits'):
            need(v>=q.p2(-1022),'subnormal derived property '+key)
    need([finite_bits(x) for x in m['y_reference_bits']]==[0,1,0], 'fixed nonparallel y reference')
    need(all(finite_bits(m[k])==1 for k in ('elastic_modulus_bits','shear_modulus_bits',
                                          'second_moment_y_bits','second_moment_z_bits')), 'fixed unit properties')
    springs=p['springs']
    need(len({s['id'] for s in springs})==len(springs), 'duplicate spring ID')
    for s in springs:
        need(0<=s['id']<2**32 and 0<=s['global_dof']<12,'spring ID/DOF range')
        need(finite_bits(s['stiffness_bits'])>0,'nonpositive spring')
    constraints=p['constraints']
    need(len({c['global_dof'] for c in constraints})==len(constraints),'duplicate constraint')
    for c in constraints:
        need(0<=c['global_dof']<12 and finite_bits(c['value_bits'])==0,'constraint DOF/value')
    for l in p['loads']:
        need(0<=l['global_dof']<12 and bool(l['source_id']),'load DOF/source ID')
        finite_bits(l['value_bits'])
    need(not any(p[k] for k in ('directional_springs','stations','supports')), 'nonempty excluded lists')
    free=set(range(12))-{c['global_dof'] for c in constraints}
    need(free and free <= {6,9}, 'fixed free axial/torsional coordinates')
    need(all(s['global_dof'] in free for s in springs),'spring outside free set')
    need(all(l['global_dof'] in free for l in p['loads']),'load outside free set')
    return nodes[1][0],free


def source_encoding(p,stiffness=False):
    """Source.rs canonical bytes: little endian, sorted declared lists."""
    out=bytearray(b'K4STF\x01' if stiffness else b'K4SRC\x01')
    def u32(x):out.extend(struct.pack('<I',x))
    def f64(s):out.extend(struct.pack('<Q',int(s,16)))
    def dof(g):u32(g//6);out.append(g%6)
    u32(len(p['nodes_bits']))
    for xyz in p['nodes_bits']:
        for x in xyz:f64(x)
    members=sorted(p['members'],key=lambda x:x['id']);u32(len(members))
    for m in members:
        for key in ('id','node_i','node_j'):u32(m[key])
        for key in ('elastic_modulus_bits','shear_modulus_bits','area_bits',
                    'second_moment_y_bits','second_moment_z_bits','torsion_constant_bits'):f64(m[key])
        for x in m['y_reference_bits']:f64(x)
    springs=sorted(p['springs'],key=lambda x:x['id']);u32(len(springs))
    for s in springs:u32(s['id']);dof(s['global_dof']);f64(s['stiffness_bits'])
    need(not p['directional_springs'],'fixed empty directional springs');u32(0)
    constraints=sorted(p['constraints'],key=lambda x:x['global_dof']);u32(len(constraints))
    for c in constraints:
        dof(c['global_dof'])
        if not stiffness:f64(c['value_bits'])
    if not stiffness:
        loads=sorted(p['loads'],key=lambda x:(x['global_dof'],x['source_id'].encode(),int(x['value_bits'],16)))
        u32(len(loads))
        for l in loads:
            dof(l['global_dof']);label=l['source_id'].encode();u32(len(label));out.extend(label);f64(l['value_bits'])
        need(not p['stations'] and not p['supports'],'fixed empty stations/supports');u32(0);u32(0)
    return bytes(out)


def derive(case):
    p=case['primitive'];L,free=validate_source(p);m=p['members'][0]
    a=finite_bits(m['area_bits'])/L;t=finite_bits(m['torsion_constant_bits'])/L
    forces=[q.F(0)]*12;spring_k=[q.F(0)]*12;u=[q.F(0)]*12
    for l in p['loads']:forces[l['global_dof']]+=finite_bits(l['value_bits'])
    for s in p['springs']:spring_k[s['global_dof']]+=finite_bits(s['stiffness_bits'])
    for g in free:u[g]=forces[g]/((a if g==6 else t)+spring_k[g])
    N=a*(u[6]-u[0]);T=t*(u[9]-u[3])
    internal=[q.F(0)]*12;internal[0]=-N;internal[6]=N;internal[3]=-T;internal[9]=T
    for g in range(12):internal[g]+=spring_k[g]*u[g]
    need(all(internal[g]==forces[g] for g in free),'exact free equilibrium failed')
    rows=[]
    def add(key,kind,x,input_derived=False):
        oc,b=q.outcome(x)
        rows.append({'key':key,'kind':kind,'body':0,'input_derived':input_derived,'x':x,
                     'value':None if b is None else q.value(b),'direct_outcome':oc,'direct_bits':None if b is None else f'{b:016x}'})
    for g in range(12):add('D:'+str(g),'Translation' if g%6<3 else 'Rotation',u[g],g not in free)
    for node in range(2):add('M:'+str(node),'Translation',abs(u[6*node]))
    for end,sign in [('I',-1),('J',1)]:
        for c in range(6):add(f'E:1:{end}:{c}','Force' if c<3 else 'Moment',sign*N if c==0 else sign*T if c==3 else q.F(0))
    for s in sorted(p['springs'],key=lambda x:x['id']):
        g=s['global_dof'];add(f'S:{s["id"]}:{g%6}','Force' if g%6<3 else 'Moment',-finite_bits(s['stiffness_bits'])*u[g])
    for g in range(12):
        if g not in free:add('R:'+str(g),'Force' if g%6<3 else 'Moment',internal[g]-forces[g])
    raw,scales=q.scales(rows,L)
    true_raw={kind:max([abs(r['x']) for r in rows if not r['input_derived'] and r['direct_outcome']=='Value' and r['kind']==kind],default=q.F(0)) for kind in q.KINDS}
    tr,ro,fo,mo=(true_raw[k] for k in q.KINDS)
    true_coupled=dict(zip(q.KINDS,[max(tr,L*ro),max(ro,tr/L),max(fo,mo/L),max(mo,L*fo)]))
    for r in rows:
        r['truth']=q.encoded(r['x']);r['rounding_cell']=q.cell(r['x'])
        cls,bound=q.classification(r['value'],scales[r['kind']],r['input_derived'])
        r['direct_class_pinned_baseline']=cls;r['direct_bound_bits_pinned_baseline']=None if bound is None else f'{bound:016x}'
        r['o9_included_in_direct_scale']=not r['input_derived'] and r['direct_outcome']=='Value'
        del r['x'];del r['value']
    canonical=json.dumps(p,sort_keys=True,separators=(',',':'),ensure_ascii=False).encode()
    enc=source_encoding(p);stf=source_encoding(p,True)
    return {'id':case['id'],'primitive':p,'free_dofs':sorted(free),
            'constructor_static_check':'valid at pinned source; no constructor/solver run',
            'input_identity':{'canonical_json_definition':'UTF-8 json.dumps(sort_keys=True,separators=(comma,colon),ensure_ascii=False), no newline',
                              'canonical_primitive_json_sha256':hashlib.sha256(canonical).hexdigest(),
                              'source_encoding_hex':enc.hex(),'source_encoding_sha256':hashlib.sha256(enc).hexdigest(),
                              'stiffness_encoding_hex':stf.hex(),'stiffness_encoding_sha256':hashlib.sha256(stf).hexdigest()},
            'equation_terms':{k:q.encoded(v) for k,v in {'length':L,'axial_member_stiffness':a,'torsional_member_stiffness':t,
                                'axial_spring_total':spring_k[6],'torsional_spring_total':spring_k[9],'load_6':forces[6],'load_9':forces[9],
                                'u_6':u[6],'u_9':u[9],'N':N,'T':T}.items()},
            'exact_free_equilibrium':'zero residual at every free DOF',
            'direct_raw_scales':{k:q.encoded(v) for k,v in raw.items()},
            'direct_coupled_scale_bits_pinned_baseline':{k:f'{q.round_bits(v):016x}' for k,v in scales.items()},
            'exact_truth_coupled_scales':{k:q.encoded(v) for k,v in true_coupled.items()},'rows':rows}


def freeze(destination):
    b=(HERE/'inputs/A1_EXTRA_INPUTS.json').read_bytes();need(hashlib.sha256(b).hexdigest()==INPUT_SHA,'input changed')
    data=json.loads(b);need(len(data['cases'])==3 and {c['id'] for c in data['cases']}==IDS,'exactly three fixed IDs required')
    result={'schema':'a1-extra-source-truth-v1','source_basis':data['source_basis'],'input_sha256':INPUT_SHA,
            'claims_basis':'Existing CHECKPOINT_0 accepted D1/R7/A1 claims; pinned direct class/scale baseline is not a repair prediction',
            'cases':[derive(c) for c in data['cases']]}
    destination.mkdir(parents=True,exist_ok=True);target=destination/'TRUTH.json';need(not target.exists(),'truth already exists')
    target.write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps({'cases':len(result['cases']),'rows':sum(len(c['rows']) for c in result['cases']),
                      'truth_sha256':hashlib.sha256(target.read_bytes()).hexdigest()}))


def compare(tsv,report):
    frozen=(HERE/'TRUTH.json').read_bytes();need(hashlib.sha256(frozen).hexdigest()==TRUTH_SHA,'truth changed')
    truth=json.loads(frozen)
    records=[line.split('\t') for line in tsv.read_text().splitlines()]
    def tagged(tag):return [r[1:] for r in records if r[0]==tag]
    def one(tag,count):
        v=tagged(tag);need(len(v)==1 and len(v[0])==count,'malformed/duplicate '+tag);return v[0]
    need(one('FORMAT',1)==['a1-public-tsv-v1'],'FORMAT')
    cid=one('CASE',1)[0];need(cid in IDS,'fixed case ID required')
    case=next(c for c in truth['cases'] if c['id']==cid)
    status=one('STATUS',1)[0];need(status in {'Selected','Refused','Unresolved','SourceRefused'},'STATUS')
    result={'case':cid,'status':status,'raw_sha256':hashlib.sha256(tsv.read_bytes()).hexdigest(),
            'truth_sha256':hashlib.sha256((HERE/'TRUTH.json').read_bytes()).hexdigest(),'failures':[],'rows':[],
            'claim_boundary':'Truth against reported claims; no certification of repaired scale/bound construction, runtime floor formation, or selection/admission.'}
    if status!='Selected':
        need(not any(tagged(t) for t in ('SELECTED','ROW','SCALE','FLOOR')),'unselected publication')
    else:
        selected=one('SELECTED',2);need(selected in [['128','256'],['256','512'],['512','1024']],'precision');p=int(selected[0])
        for tag,key in [('SOURCE_ENCODING','source_encoding_hex'),('SOURCE_ENCODING_SELECTED','source_encoding_hex'),('STIFFNESS_ENCODING','stiffness_encoding_hex')]:
            need(one(tag,1)==[case['input_identity'][key]],tag+' differs from independently encoded primitive')
        ss=tagged('SCALE');need(len(ss)==4 and all(len(x)==2 for x in ss) and {x[0] for x in ss}==set(q.KINDS),'SCALE')
        scales={k:finite_bits(b,True) for k,b in ss}
        ff=tagged('FLOOR');need(all(len(x)==2 for x in ff) and len({x[0] for x in ff})==len(ff),'duplicate/malformed FLOOR')
        need((p==512 and len(ff)==2 and {x[0] for x in ff}=={'Force','Moment'}) or (p!=512 and not ff),'FLOOR presence')
        for _,b in ff:finite_bits(b,True)
        expected={r['key']:r for r in case['rows']};rr=tagged('ROW');ll=tagged('LAYOUT')
        need(all(len(x)==6 for x in rr) and len(rr)==len(expected) and {x[0] for x in rr}==set(expected),'ROW coverage')
        need(all(len(x)==4 for x in ll) and len(ll)==len(expected) and {x[0] for x in ll}==set(expected),'LAYOUT coverage')
        for key,kind,body,inp in ll:need((kind,body,inp)==(expected[key]['kind'],'0',str(expected[key]['input_derived']).lower()),'LAYOUT mismatch')
        for key,kind,oc,bits,cls,bound in rr:
            e=expected[key];x=q.decoded(e['truth']);need(kind==e['kind'],'ROW kind')
            item={'key':key,'truth':e['truth'],'direct_bits_match':bits==e['direct_bits'],'class':cls}
            if oc in {'Underflow','Overflow'}:
                need((bits,cls,bound)==('none','Unpublishable','none'),'range row shape')
                item['range_consistent']=oc==e['direct_outcome']
                if not item['range_consistent']:result['failures'].append('range:'+key)
            else:
                need(oc=='Value','outcome');v=finite_bits(bits)
                need(cls in {'InputDerived','AbsoluteVerified','RelativeVerified'},'ROW class')
                need((cls=='InputDerived')==e['input_derived'],'InputDerived membership')
                b=finite_bits(bound,True) if cls=='AbsoluteVerified' else None
                if cls!='AbsoluteVerified':need(bound=='none','unexpected bound')
                err=abs(v-x);allow,source=q.allowances(v,scales[kind],cls,b,p)
                item.update(error=q.encoded(err),exact_allowance=q.encoded(allow),source_binary64_allowance=q.encoded(source),
                            exact_claim_pass=err<=allow,source_claim_pass=err<=source)
                if err>allow:result['failures'].append('exact-claim:'+key)
                if err>source:result['failures'].append('source-claim:'+key)
                if cls=='InputDerived' and v!=x:result['failures'].append('prescription:'+key)
                if cls=='RelativeVerified':
                    item['public_relative_published_denominator_pass']=err<=abs(v)/10**9
                    item['public_relative_truth_denominator_pass']=err<=abs(x)/10**9
                    if err>abs(v)/10**9:result['failures'].append('public-relative:'+key)
            result['rows'].append(item)
    report.write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps({'case':cid,'status':status,'rows':len(result['rows']),'failures':result['failures']}))
    return 1 if result['failures'] else 0


if __name__=='__main__':
    try:
        if sys.argv[1]=='freeze':freeze(Path(sys.argv[2]));code=0
        elif sys.argv[1]=='compare':code=compare(Path(sys.argv[2]),Path(sys.argv[3]))
        else:raise ValueError('freeze or compare required')
    except (ValueError,AssertionError,KeyError,IndexError,StopIteration) as exc:
        print(json.dumps({'status':'invalid_input','error':str(exc)}),file=sys.stderr);code=2
    raise SystemExit(code)
