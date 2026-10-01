#!/usr/bin/env python3
"""Additive bare-b reported-claim comparator for 20 fixed existing sources.

<VENV>/bin/python -B bare_b_compare.py OUTPUT_TSV NEW_REPORT_JSON
Exit 0: selected and all applicable truth/claim checks pass.
Exit 1: selected result has numerical/range failures.
Exit 2: invalid interface, source/contract identity, or frozen basis.
Exit 3: valid non-selected result; no accuracy pass.
No certificate H, radius, bound-construction, runtime, or admission proof.
"""
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import sys

sys.dont_write_bytecode=True
HERE=Path(__file__).resolve().parent
ORACLE=HERE.parent
PINS={
    'exact_oracle.py':'fa8ea6f303148d9babb5d9fe6c53f64377b13cb130d03d076d4cec7d3f4c15e0',
    'TRUTH.json':'1772d703e032a71587b922a2f6d825718f777c9dae78d1041fbd874210ee87ea',
    'addendum_03_source_controls/TRUTH.json':'aac399bfb68bbb8b1cbcce08cc2901229535e88860dc5cc812cf08c9fe3ba0ee',
    'addendum_04_bare_b/SOURCE_IDENTITIES.json':'4f0865d132225612cd11f43ca715038d8b485646e6eb6f7bf8f51fc922e8d7f2',
}
METHOD='contribution_preserving_multiprecision_v1'
POLICY='M03-INTEGRITY-MP-v2'
FLOOR_RATIO='3dd0000000000000'
IDS={f'B{i:02d}' for i in range(1,17)} | {'C17','EXTRA-FM-01','EXTRA-MF-01','EXTRA-ZR-01'}
KINDS={'Translation','Rotation','Force','Moment'}


def need(ok,message):
    if not ok:raise ValueError(message)


def pinned(name):
    b=(ORACLE/name).read_bytes()
    need(hashlib.sha256(b).hexdigest()==PINS[name], 'frozen hash mismatch: '+name)
    return b


spec=importlib.util.spec_from_file_location('frozen_exact_rationals',ORACLE/'exact_oracle.py')
pinned('exact_oracle.py')
q=importlib.util.module_from_spec(spec);spec.loader.exec_module(q)


def finite(s,nonnegative=False):
    need(isinstance(s,str) and re.fullmatch(r'[0-9a-f]{16}',s),'bits require 16 lowercase hex digits')
    b=int(s,16)
    need((b>>52)&2047 != 2047,'nonfinite binary64')
    need(b != 1<<63,'negative zero is noncanonical')
    need(not nonnegative or b < 1<<63,'negative scale, floor, or bound')
    return q.value(b)


def rn_nonnegative(x):
    need(x>=0,'negative intermediate')
    b=q.round_bits(x)
    need(b<q.INF,'nonfinite relative intermediate')
    return q.value(b)


def relative_allowances(x,scale):
    """Accepted correction §2: preserve all five binary64 steps, no FMA."""
    need(scale>=0,'negative scale')
    eps,u,h=q.p2(-64),q.p2(-53),q.H
    maximum=max(abs(x),scale)
    exact=eps*maximum*(1+q.p2(-21))+u*abs(x)+h
    a0=rn_nonnegative(eps*maximum)
    a1=rn_nonnegative(a0*(1+q.p2(-21)))
    u0=rn_nonnegative(u*abs(x))
    u1=rn_nonnegative(u0+h)
    a2=rn_nonnegative(a1+u1)
    return exact,a2,dict(a0=a0,a1=a1,u0=u0,u1=u1,a2=a2)


def predicates(x,truth,cls,bound=None,scale=None):
    error=abs(x-truth)
    out={'error':q.encoded(error)}
    if cls=='AbsoluteVerified':
        need(bound is not None and bound>=0,'missing/negative absolute bound')
        out.update(bare_b=q.encoded(bound),bare_b_pass=error<=bound)
    elif cls=='RelativeVerified':
        exact,f64,steps=relative_allowances(x,scale)
        out.update(public_allowance=q.encoded(abs(x)/10**9),public_pass=10**9*error<=abs(x),
                   sharper_exact_allowance=q.encoded(exact),sharper_exact_pass=error<=exact,
                   sharper_binary64_allowance=q.encoded(f64),sharper_binary64_pass=error<=f64,
                   sharper_binary64_steps={k:q.encoded(v) for k,v in steps.items()},
                   truth_denominator_relative_pass_diagnostic=10**9*error<=abs(truth))
    elif cls=='InputDerived':
        out['exact_prescription_pass']=x==truth
    else:raise ValueError('invalid value-bearing class')
    return out


def fixed_basis():
    original=json.loads(pinned('TRUTH.json'))
    extra=json.loads(pinned('addendum_03_source_controls/TRUTH.json'))
    registry=json.loads(pinned('addendum_04_bare_b/SOURCE_IDENTITIES.json'))
    rows={c['id']:c for c in original['cases']+extra['cases'] if c['id'] in IDS}
    identities={c['id']:c for c in registry['cases']}
    need(set(rows)==set(identities)==IDS,'fixed registry coverage')
    return rows,identities


def compare_text(text,raw_sha256):
    cases,identities=fixed_basis()
    records=[line.split('\t') for line in text.splitlines()]
    def tagged(tag):return [r[1:] for r in records if r[0]==tag]
    def one(tag,n):
        matches=tagged(tag)
        need(len(matches)==1 and len(matches[0])==n,'missing/malformed/duplicate '+tag)
        return matches[0]
    need(one('FORMAT',1)==['a1-public-tsv-v1'],'FORMAT mismatch')
    cid=one('CASE',1)[0];need(cid in IDS,'case outside the 20 fixed sources')
    source_commit=one('SOURCE_COMMIT',1)[0]
    need(re.fullmatch(r'[0-9a-f]{40}',source_commit),'SOURCE_COMMIT must be a full lowercase SHA')
    limits=one('LIMITS',2)
    need(all(re.fullmatch(r'[1-9][0-9]*',v) for v in limits),'LIMITS must be positive canonical integers')
    status=one('STATUS',1)[0];need(status in {'Selected','Refused','Unresolved','SourceRefused'},'unknown STATUS')
    case=cases[cid];identity=identities[cid];expected=case['rows'];keys=[r['key'] for r in expected]
    meta={r['key']:r for r in expected}
    result={'schema':'a1-bare-b-comparison-v1','case':cid,'status':status,'raw_sha256':raw_sha256,
            'source_commit_as_reported':source_commit,'limits_as_reported':limits,
            'required_policy':POLICY,'numeric_accuracy_pass':None,'failures':[],'rows':[],
            'truth_file':identity['truth_file'],'truth_sha256':PINS[identity['truth_file']],
            'claim_boundary':'Exact truth versus reported selected claims; no bound construction, certificate H, private radius, G5a, floor formation, execution or admission certification.'}
    for tag,key in [('SOURCE_ENCODING','source_encoding_hex'),('STIFFNESS_ENCODING','stiffness_encoding_hex')]:
        need(one(tag,1)==[identity[key]],tag+' mismatches exact primitive source')
    result['source_binding_exact']=True
    layout=tagged('LAYOUT')
    if layout or status=='Selected':
        need(len(layout)==len(keys) and all(len(r)==4 for r in layout) and [r[0] for r in layout]==keys,'LAYOUT coverage/order')
        for key,kind,body,inp in layout:
            e=meta[key];need((kind,body,inp)==(e['kind'],'0',str(e['input_derived']).lower()),'LAYOUT identity mismatch: '+key)
    if status!='Selected':
        need(not any(tagged(t) for t in ('SELECTED','IDENTITY','ROW','SCALE','FLOOR','SOURCE_ENCODING_SELECTED')),'unselected result carries publication')
        return result,3
    selected=one('SELECTED',2)
    need(selected in [['128','256'],['256','512'],['512','1024']],'SELECTED candidate/verification precision')
    p=int(selected[0]);result['selected_precision']=p
    need(one('IDENTITY',3)==[METHOD,POLICY,FLOOR_RATIO],'corrected method/policy/floor-ratio identity required')
    need(one('SOURCE_ENCODING_SELECTED',1)==[identity['source_encoding_hex']],'selected source binding mismatch')
    scales_list=tagged('SCALE')
    need(len(scales_list)==4 and all(len(r)==2 for r in scales_list) and {r[0] for r in scales_list}==KINDS,'SCALE coverage/duplicate')
    scales={kind:finite(bits,True) for kind,bits in scales_list}
    floors=tagged('FLOOR')
    need(all(len(r)==2 for r in floors) and len({r[0] for r in floors})==len(floors),'FLOOR duplicate/shape')
    need((p==512 and len(floors)==2 and {r[0] for r in floors}=={'Force','Moment'}) or (p!=512 and not floors),'FLOOR coverage/presence')
    for _,bits in floors:finite(bits,True)
    rows=tagged('ROW')
    need(len(rows)==len(keys) and all(len(r)==6 for r in rows) and [r[0] for r in rows]==keys,'ROW coverage/order/duplicate')
    for key,kind,outcome,bits,cls,bound_bits in rows:
        e=meta[key];truth=q.decoded(e['truth']);need(kind==e['kind'],'ROW kind mismatch')
        item={'key':key,'truth':e['truth'],'class':cls,'outcome':outcome,
              'direct_truth_rounding_matches':outcome==e['direct_outcome'] and bits==(e['direct_bits'] or 'none')}
        if outcome in {'Underflow','Overflow'}:
            need((bits,cls,bound_bits)==('none','Unpublishable','none'),'invalid range outcome fields')
            item['exact_truth_range_pass']=outcome==e['direct_outcome']
            if not item['exact_truth_range_pass']:result['failures'].append('range:'+key)
        else:
            need(outcome=='Value','unknown ROW outcome');x=finite(bits)
            need(cls in {'InputDerived','AbsoluteVerified','RelativeVerified'},'invalid value-bearing class')
            need((cls=='InputDerived')==e['input_derived'],'InputDerived membership mismatch')
            bound=finite(bound_bits,True) if cls=='AbsoluteVerified' else None
            if cls!='AbsoluteVerified':need(bound_bits=='none','unexpected bound')
            checks=predicates(x,truth,cls,bound,scales[kind]);item.update(checks)
            for name in ('bare_b_pass','public_pass','sharper_exact_pass','sharper_binary64_pass','exact_prescription_pass'):
                if name in checks and not checks[name]:result['failures'].append(name+':'+key)
        result['rows'].append(item)
    result['numeric_accuracy_pass']=not result['failures']
    return result,0 if result['numeric_accuracy_pass'] else 1


def run(tsv,report):
    need(not report.exists(),'report already exists; use an additive path')
    raw=tsv.read_bytes();result,code=compare_text(raw.decode(),hashlib.sha256(raw).hexdigest())
    result['comparator_sha256']=hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
    report.write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps({'case':result['case'],'status':result['status'],'numeric_accuracy_pass':result['numeric_accuracy_pass'],
                      'rows':len(result['rows']),'failures':result['failures'],'exit_code':code}))
    return code


if __name__=='__main__':
    try:
        need(len(sys.argv)==3,'usage: bare_b_compare.py OUTPUT_TSV NEW_REPORT_JSON')
        code=run(Path(sys.argv[1]),Path(sys.argv[2]))
    except (ValueError,AssertionError,KeyError,IndexError,StopIteration,OSError) as exc:
        print(json.dumps({'status':'invalid_input','error':str(exc)}),file=sys.stderr);code=2
    raise SystemExit(code)
