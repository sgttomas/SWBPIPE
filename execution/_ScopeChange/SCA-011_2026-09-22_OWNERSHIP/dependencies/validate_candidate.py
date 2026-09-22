#!/usr/bin/env python3
"""Validate full candidate, preservation, staged cycles, maturity and negative probes."""
import copy
import csv
import hashlib
import json
import sys
from collections import Counter, defaultdict
from pathlib import Path
OUT=Path(__file__).resolve().parent
ROOT=OUT.parents[5]
sys.path.insert(0,str(ROOT/'tools/coordination'))
import audit_dag
BASE=ROOT/'projects/chirality-piping/execution/_DAG/DAG-010'

def read(p):
    with p.open(newline='') as f:return list(csv.DictReader(f))
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()

old=read(BASE/'DependencyEdges.csv'); rows=read(OUT/'DependencyEdges.csv'); nodes=read(OUT/'DeliverableNodes.csv')
added=read(OUT/'DependencyEdges.additions.csv')
staged=json.loads((OUT/'StagedInterfaces.json').read_text())['bindings']
manifest=json.loads((OUT/'MANIFEST.json').read_text())

# Explicit stage expansion; the stock audit includes all deliverable INTERFACE edges
# but does not model DOCUMENT targets. No cycle is dismissed by edge type alone.
stage_alias={
 'DEL-07-01':'DEL-07-01::palette-implementation',
 'DEL-07-09':'DEL-07-09::coverage-assessment',
 'DEL-08-04':'DEL-08-04::base-result-envelope',
 'DEL-14-05':'DEL-14-05::comparison-contract',
 'DEL-16-03':'DEL-16-03::acceptance-decision',
 'DEL-16-06':'DEL-16-06::application',
}
def stage_edges(rs):
    result=[]
    for r in audit_dag.graph_rows(rs,{'ACTIVE'}):
        a,b=audit_dag.normalized_edge(r)
        result.append({'consumer':stage_alias.get(a,a),'upstream':stage_alias.get(b,b),'basis':r['DependencyID']})
    for s in staged:
        result.append({'consumer':s['consumer_stage'],'upstream':s['producer_stage'],'basis':s['edge_id']})
    result.extend([
      {'consumer':'DEL-07-09::organization-contract','upstream':'DEL-16-01','basis':'Organization contract binds operation taxonomy; DAG-010-DEL-07-09-E001'},
      {'consumer':'DEL-07-09::coverage-assessment','upstream':'DEL-07-09::organization-contract','basis':'Coverage tests the already-defined organization contract'},
      {'consumer':'DEL-08-04::comparison-export-mapping','upstream':'DEL-08-04::base-result-envelope','basis':'Comparison mapping extends base export; does not redefine the base'},
      {'consumer':'DEL-16-03::final-audit','upstream':'DEL-16-03::acceptance-decision','basis':'Final audit links the actual pre-application acceptance decision'},
    ])
    return result

def check(rs, ns, stage_extra=()):
    errors=[]
    if rs[:len(old)]!=old:errors.append('BASELINE_ROWS_CHANGED')
    ids=[r['DependencyID'] for r in rs]
    if len(ids)!=len(set(ids)):errors.append('DUPLICATE_ID')
    errors.extend('CANONICAL:'+x['Issue'] for x in audit_dag.validate_canonical_rows(rs))
    known={n['DeliverableID'] for n in ns}
    for r in rs:
        if r['FromDeliverableID'] not in known:errors.append('DANGLING_FROM')
        if r['TargetType']=='DELIVERABLE' and r['TargetDeliverableID'] not in known:errors.append('DANGLING_TARGET')
    g,gn,records=audit_dag.build_graph(audit_dag.graph_rows(rs,{'ACTIVE'}))
    if audit_dag.find_sccs(g,gn) or any(a==b for a,children in g.items() for b in children):errors.append('DELIVERABLE_CYCLE')
    if audit_dag.duplicate_edges(records):errors.append('DUPLICATE_DIRECTED_EDGE')
    new=rs[len(old):]
    for r in new:
        if not r['EvidenceFile'] or not r['SourceRef']:errors.append('MISSING_EVIDENCE')
        if r['DependencyClass']=='EXECUTION' and (r['RequiredMaturity'],r['ProposedMaturity'],r['SatisfactionStatus'])!=('SEMANTIC_READY','TBD','PENDING'):
            errors.append('UNSUPPORTED_MATURITY_OR_SATISFACTION')
    for n in ns[102:]:
        anchors=[r for r in new if r['FromDeliverableID']==n['DeliverableID'] and r['AnchorType']=='IMPLEMENTS_NODE']
        if len(anchors)!=1:errors.append('PARENT_ANCHOR_COUNT')
    se=stage_edges(rs)+list(stage_extra)
    sg=defaultdict(set); sn=set()
    for r in se:sg[r['consumer']].add(r['upstream']);sn.update((r['consumer'],r['upstream']))
    if audit_dag.find_sccs(sg,sn) or any(a==b for a,children in sg.items() for b in children):errors.append('STAGE_CYCLE')
    return sorted(set(errors))

errors=check(rows,nodes)
# Compare new identities to the concurrent concrete decomposition candidate.
reg=OUT.parent/'candidate/projects/chirality-piping/docs/_Registers/Deliverables.csv'
regrows={r['DeliverableID']:r for r in read(reg)}
for n in nodes[102:]:
    r=regrows.get(n['DeliverableID'],{})
    for left,right in [('PackageID','PackageID'),('DeliverableName','Name'),('DeliverableType','Type'),('ScopeItems','CoversScopeItems'),('Objectives','SupportsObjectives'),('ContextEnvelope','ContextEnvelope')]:
        if n[left]!=r.get(right):errors.append('DECOMPOSITION_NODE_MISMATCH:'+n['DeliverableID']+':'+left)
for r in added:
    evidence=ROOT/'projects/chirality-piping'/r['EvidenceFile']
    if not evidence.is_file():errors.append('EVIDENCE_FILE_MISSING:'+r['DependencyID'])
    if r['EvidenceQuote'] and r['EvidenceQuote'] not in evidence.read_text():errors.append('NONVERBATIM_QUOTE:'+r['DependencyID'])
    if r['TargetType']=='DOCUMENT' and not (ROOT/'projects/chirality-piping'/r['TargetLocation']).is_file():errors.append('STAGED_CONTRACT_MISSING:'+r['DependencyID'])
for name in ['DependencyEdges.csv','DeliverableNodes.csv']:
    if not (OUT/name).read_bytes().startswith((BASE/name).read_bytes()):errors.append('BASELINE_BYTE_PREFIX:'+name)
    if digest(BASE/name)!=manifest['outputs'][name]['baseline_sha256']:errors.append('BASELINE_HASH:'+name)
    if digest(OUT/name)!=manifest['outputs'][name]['sha256']:errors.append('CANDIDATE_HASH:'+name)
probes={}
mut=copy.deepcopy(rows);mut[0]['Notes']+=' damaged';probes['baseline_damage']=check(mut,nodes)
mut=copy.deepcopy(rows);mut.append(copy.deepcopy(added[-4]));probes['duplicate_row']=check(mut,nodes)
mut=copy.deepcopy(rows);next(r for r in reversed(mut) if r['TargetType']=='DELIVERABLE')['TargetDeliverableID']='DEL-99-99';probes['dangling_target']=check(mut,nodes)
mut=copy.deepcopy(rows);next(r for r in reversed(mut) if r['DependencyClass']=='EXECUTION')['SatisfactionStatus']='SATISFIED';probes['unsupported_satisfaction']=check(mut,nodes)
for s in staged:
    probes['reverse_stage_'+s['id']]=check(rows,nodes,[{'consumer':s['producer_stage'],'upstream':s['consumer_stage'],'basis':'negative probe'}])
expected={'baseline_damage':'BASELINE_ROWS_CHANGED','duplicate_row':'DUPLICATE_ID','dangling_target':'DANGLING_TARGET','unsupported_satisfaction':'UNSUPPORTED_MATURITY_OR_SATISFACTION'}
expected.update({'reverse_stage_'+s['id']:'STAGE_CYCLE' for s in staged})
if any(expected[k] not in probes[k] for k in expected):errors.append('NEGATIVE_PROBE_FAILED_TO_FAIL')
report={'status':'PASS' if not errors else 'FAIL','errors':errors,'baseline_rows':len(old),'candidate_rows':len(rows),'candidate_nodes':len(nodes),
 'new_row_classes':dict(Counter(r['DependencyClass'] for r in added)),
 'new_execution_satisfaction':dict(Counter(r['SatisfactionStatus'] for r in added if r['DependencyClass']=='EXECUTION')),
 'stage_edges':len(stage_edges(rows)),'stage_scc_count':0 if 'STAGE_CYCLE' not in errors else 'NONZERO','negative_probes':probes,
 'limits':['Stage expansion is a proposed sequencing interpretation of distinct artifacts; it is not adoption or proof the contract artifacts exist.',
           'Baseline maturity/satisfaction fields are preserved, not freshly revalidated; all new execution rows remain PENDING.',
           'Stock graph audit covers full accepted-plus-additive deliverable graph. Supplement adds all three new DOCUMENT bindings and four explicit stage-internal requirements. Other historical DOCUMENT relations retain their original evidence without a new artifact-stage interpretation.']}
(OUT/'StageGraph.json').write_text(json.dumps({'status':'PROPOSED_NOT_ADOPTED','aliases':stage_alias,'edges':stage_edges(rows)},indent=2)+'\n')
(OUT/'VALIDATION.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({k:report[k] for k in ['status','errors','baseline_rows','candidate_rows','candidate_nodes','new_row_classes','stage_edges']}))
raise SystemExit(bool(errors))
