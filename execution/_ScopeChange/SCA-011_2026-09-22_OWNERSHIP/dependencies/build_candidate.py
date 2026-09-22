#!/usr/bin/env python3
"""Rebuild the non-authoritative SCA-011 graph candidate; never writes live files."""
import csv
import hashlib
import io
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[6]
PROJECT = ROOT / 'projects/chirality-piping'
OUT = Path(__file__).resolve().parent
BASE = PROJECT / 'execution/_DAG/DAG-010'
PROPOSAL = PROJECT / 'execution/_Coordination/_PROPOSALS/R5_R6_FOLLOWUP_2026-09-22/PIPING_OWNERSHIP_PROPOSAL.md'
DATE = '2026-09-22'

def read(path):
    with path.open(newline='') as f:
        r = csv.DictReader(f)
        return r.fieldnames, list(r)

def csv_bytes(fields, rows, header=True):
    s = io.StringIO(newline='')
    w = csv.DictWriter(s, fieldnames=fields, lineterminator='\n')
    if header: w.writeheader()
    w.writerows(rows)
    return s.getvalue().encode()

def sha(data): return hashlib.sha256(data).hexdigest()

fields, old = read(BASE / 'DependencyEdges.csv')
nfields, oldnodes = read(BASE / 'DeliverableNodes.csv')
nodes = {n['DeliverableID']: n for n in oldnodes}
new_specs = [
    ('DEL-04-07','Product solve integration and nonlinear orchestration','BACKEND_FEATURE_SLICE','L',['SOW-005','SOW-011','SOW-012','SOW-052','SOW-053'],['OBJ-003','OBJ-012']),
    ('DEL-07-11','Desktop workspace shell and host integration','UX_UI_SLICE','L',['SOW-078'],['OBJ-006','OBJ-010','OBJ-015']),
    ('DEL-07-12','Interoperability and evidence-review workspace','UX_UI_SLICE','L',['SOW-079'],['OBJ-006','OBJ-007','OBJ-018']),
    ('DEL-16-06','Controlled model operation application','BACKEND_FEATURE_SLICE','M',['SOW-069','SOW-070'],['OBJ-015']),
]
newnodes=[]
for ident,name,kind,envelope,scope,objectives in new_specs:
    package = 'PKG-'+ident.split('-')[1]
    sibling=next(n for n in oldnodes if n['PackageID']==package)
    location=str(Path(sibling['ExecutionPath']).parent / (ident+'_'+name))
    n={k:'' for k in nfields}
    n.update(NodeID=ident,PackageID=package,DeliverableID=ident,DeliverableName=name,DeliverableType=kind,
        ScopeItems=','.join(scope),Objectives=','.join(objectives),ContextEnvelope=envelope,
        ExecutionPath=location,ContextPath=location+'/_CONTEXT.md',DependenciesPath=location+'/_DEPENDENCIES.md',
        HasFourDocumentKit='FALSE',HasSemanticMatrix='FALSE',HasSemanticLensing='FALSE',HasReview='FALSE',
        SourceRegister='execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP/candidate/projects/chirality-piping/docs/_Registers/Deliverables.csv',
        Notes='PROPOSAL: SCA-011 group-2 candidate identity; not an adopted node or lifecycle promotion. Candidate controls are prepared separately; live path is a proposed landing.')
    nodes[ident]=n
    newnodes.append(n)

added=[]
existing=[]
base_pairs={}
for r in old:
    if r['Status']=='ACTIVE' and r['DependencyClass']=='EXECUTION' and r['TargetType']=='DELIVERABLE':
        pair=(r['FromDeliverableID'],r['TargetDeliverableID']) if r['Direction']=='UPSTREAM' else (r['TargetDeliverableID'],r['FromDeliverableID'])
        base_pairs[pair]=r

def row(consumer, ref, target, statement, typ='PREREQUISITE', source='Dependency delta and amendment propagation', quote='', notes=''):
    n=nodes[consumer]
    r={k:'' for k in fields}
    r.update(RegisterSchemaVersion='v3.1',DependencyID=f'SCA011-E{len(added)+1:03d}',FromPackageID=n['PackageID'],
        FromDeliverableID=consumer,FromDeliverableName=n['DeliverableName'],DependencyClass='EXECUTION',AnchorType='NOT_APPLICABLE',
        Direction='UPSTREAM',DependencyType=typ,TargetType='DELIVERABLE' if target else 'DOCUMENT',TargetRefID=ref,
        TargetDeliverableID=ref if target else '',TargetPackageID=nodes[ref]['PackageID'] if target else '',
        TargetName=nodes[ref]['DeliverableName'] if target else ref,TargetLocation=nodes[ref]['ExecutionPath'] if target else 'location TBD; staged contract artifact not yet adopted',
        Statement=statement,EvidenceFile=str(PROPOSAL.relative_to(PROJECT)),SourceRef=source,EvidenceQuote='',
        Explicitness='EXPLICIT',RequiredMaturity='SEMANTIC_READY',ProposedMaturity='TBD',SatisfactionStatus='PENDING',Confidence='HIGH',
        Origin='EXTRACTED',FirstSeen=DATE,LastSeen=DATE,Status='ACTIVE',
        Notes='PROPOSAL: SourceRef supports the extracted relation; Statement is an agent synthesis, not a quotation. SCA-011 group-2 graph candidate only; ACTIVE describes the proposed relation, not graph adoption. Required maturity is a proposed contract-review gate; actual target maturity and conformance are not inferred from existing code. '+notes)
    added.append(r)
    return r

def deps(consumer, targets, subject, source):
    for target in targets.split():
        quote=subject
        if (consumer,target) in base_pairs:
            prior=base_pairs[consumer,target]
            existing.append({'Consumer':consumer,'Upstream':target,'PreservedDependencyID':prior['DependencyID'],'Treatment':'EXISTING_ROW_UNCHANGED','Reason':subject})
            continue
        architecture=target.startswith('DEL-00-')
        row(consumer,target,True,f'{consumer} consumes {nodes[target]["DeliverableName"]} from {target}: {subject}',
            'CONSTRAINT' if architecture else 'PREREQUISITE',source,quote,
            'Architecture consistency input; no PKG-00 lifecycle promotion.' if architecture else '')

# Pass 1: candidate identities and trace anchors. No existing anchor is rewritten.
for ident,name,kind,envelope,scopes,objectives in new_specs:
    for ref,anchor,target in [(ident,'IMPLEMENTS_NODE','WBS_NODE')]+[(x,'TRACES_TO_REQUIREMENT','REQUIREMENT') for x in scopes+objectives]:
        r=row(ident,ref,False,f'{ident} implements its candidate decomposition identity.' if anchor=='IMPLEMENTS_NODE' else f'{ident} traces to {ref} in the candidate decomposition.',
            'OTHER','Proposed identity table',name,'Identity/coverage trace only; not evidence of satisfaction of the scope item.')
        r.update(DependencyClass='ANCHOR',AnchorType=anchor,TargetType=target,TargetName=name if ref==ident else ref,
            TargetLocation='execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP/candidate/projects/chirality-piping/docs/_Registers/Deliverables.csv',
            RequiredMaturity='TBD',ProposedMaturity='TBD',SatisfactionStatus='NOT_APPLICABLE')

# Pass 2: explicit execution inputs. Existing directed edges are reused, never duplicated.
deps('DEL-04-07','DEL-02-01 DEL-02-02 DEL-02-03 DEL-03-08 DEL-04-01 DEL-04-03 DEL-04-04 DEL-04-06 DEL-05-01 DEL-05-02 DEL-05-03 DEL-05-04 DEL-00-02 DEL-00-03 DEL-00-06 DEL-00-08',
     'model/units/boundary, section/mass, assembly/classifier/diagnostics, loads/recovery/status; primitive mathematics remains with the upstream owner.','P-OWN-01; dependency table DEL-04-07')
deps('DEL-07-11','DEL-00-03 DEL-00-05 DEL-00-07 DEL-02-05 DEL-07-06 DEL-14-02 DEL-16-06',
     'shell consumes service/GUI/host constraints, persistence, accessibility, actual run records and controlled application; mounting does not transfer feature semantics.','P-OWN-02; dependency table DEL-07-11')
deps('DEL-16-06','DEL-16-01 DEL-16-02 DEL-16-03 DEL-02-01 DEL-02-02 DEL-02-05 DEL-00-03 DEL-00-07',
     'application requires schema-valid preview, current-model basis and the pre-application acceptance decision, with persistence and service constraints; post-application receipt is a separate stage.','P-OWN-03; dependency table DEL-16-06')
# Whole deliverable owns a bounded panel inventory; each row identifies its consuming family.
panels={
 'DEL-15-01':'handoff panel: canonical package and manifest',
 'DEL-15-02':'handoff/local-FEA/external-prover panels: target mapping and unsupported behavior',
 'DEL-15-03':'handoff panel: explicit downstream export workflow',
 'DEL-15-04':'external-prover boundary panel: boundary metadata',
 'DEL-17-01':'CAEPIPE/export panels: source-basis constraints only, never extracted equations as engineering authority',
 'DEL-17-02':'export panels: common package/profile/stable-ID/unit contract',
 'DEL-17-03':'native-package panel: native open JSON package',
 'DEL-17-04':'caepipe-mbf panel: deterministic MBF writer records',
 'DEL-17-05':'caepipe-external panel: licensed run evidence and CSV records, no activation authority',
 'DEL-17-06':'stress-neutral panel: CSV/JSON package',
 'DEL-17-07':'pcf-export panel: conservative PCF records',
 'DEL-17-08':'review-geometry panel: GLB/glTF review geometry records',
 'DEL-17-09':'export-adapter-sdk panel: SDK and already-scoped target records',
 'DEL-10-02':'adapter-framework panel: framework validation and privacy contract',
 'DEL-10-03':'local-fea-handoff panel: local FEA data contract; held formats/invocation remain held',
 'DEL-10-05':'headless-runner panel: structured I/O runner records without GUI ownership of the runner',
 'DEL-09-04':'validation-evidence panel: validation manual evidence; display does not confer engineering acceptance',
 'DEL-12-02':'all explicit export paths: private data redaction and export controls',
 'DEL-00-05':'panel GUI-state separation', 'DEL-00-06':'diagnostic and result-envelope transport constraints',
 'DEL-00-07':'adapter/result no-bypass boundary constraints',
}
for target,subject in panels.items():
    deps('DEL-07-12',target,subject+'; a prerequisite for this family only, not every panel.','P-OWN-05; dependency table DEL-07-12')
deps('DEL-07-01','DEL-16-06','palette dispatch uses the sole controlled operation application seam.','P-OWN-04; dependency table palette implementation')
deps('DEL-07-03','DEL-05-01 DEL-05-02 DEL-16-06 DEL-03-01 DEL-03-02 DEL-03-07','load/self-weight interaction dispatches to existing generators and controlled application; existing library contracts are retained.','P-OWN-04; dependency table load/self-weight UI')
deps('DEL-07-04','DEL-06-02 DEL-06-03 DEL-05-04 DEL-02-02','rule-check interaction preserves runner/required-input/status/unit-aware diagnostic contracts.','P-OWN-06; dependency table rule-check UI')
deps('DEL-07-08','DEL-16-06 DEL-14-05','operation outcomes and common comparison envelope; existing scope remains.','P-OWN-03/06; dependency table operations/comparison')
deps('DEL-14-03','DEL-14-05','comparison producer uses the shared mapping/tolerance/export contract.','P-OWN-06; dependency table result interfaces')
deps('DEL-14-04','DEL-14-05','comparison producer uses the shared mapping/tolerance/export contract.','P-OWN-06; dependency table result interfaces')

# Staged artifacts make reciprocal information flows explicit, not cyclic completion gates.
staged=[
 dict(id='SCA011-IF-PALETTE-ORGANIZATION',consumer='DEL-07-01',owner='DEL-07-09',consumer_stage='DEL-07-01::palette-implementation',producer_stage='DEL-07-09::organization-contract',typ='CONSTRAINT',
      statement='Palette implementation conforms to the versioned organization contract owned by DEL-07-09; subsequent DEL-07-09 coverage assessment consumes viewport/tree implementation evidence.',
      basis='DAG-010-DEL-07-09-E002/E003 retained; reverse deliverable prerequisite prohibited.',needs='Name and review the organization-contract artifact separately from its later coverage-assessment record.'),
 dict(id='SCA011-IF-COMPARISON-EXPORT',consumer='DEL-08-04',owner='DEL-14-05',consumer_stage='DEL-08-04::comparison-export-mapping',producer_stage='DEL-14-05::comparison-contract',typ='INTERFACE',
      statement='The general exporter maps the common comparison schema owned by DEL-14-05; that comparison schema first consumes the existing DEL-08-04 base result-envelope contract.',
      basis='DAG-002-E0790 retained; DEL-08-04 base envelope precedes comparison schema, which precedes comparison export mapping.',needs='Name and review a versioned comparison schema and identify the base-envelope version it imports; fixture payloads are not authoritative.'),
 dict(id='SCA011-IF-APPLICATION-OUTCOME',consumer='DEL-16-03',owner='DEL-16-06',consumer_stage='DEL-16-03::final-audit',producer_stage='DEL-16-06::application',typ='HANDOVER',
      statement='Final operation audit consumes the actual outcome of controlled application; application first consumes the pre-application accepted/rejected decision record, never a final receipt that does not yet exist.',
      basis='DEL-16-06 to DEL-16-03 means pre-application acceptance only; final outcome returns after application.',needs='Bind acceptance and outcome record identities/states; validate rejected/stale-basis nonmutation and truthful final outcome linkage.'),
]
for s in staged:
    r=row(s['consumer'],s['id'],False,s['statement'],s['typ'],'Dependency delta and amendment propagation; staged-interface risks',s['statement'],
        f'Contract owner={s["owner"]}; consumer_stage={s["consumer_stage"]}; producer_stage={s["producer_stage"]}. {s["basis"]} OPEN: {s["needs"]}')
    s['edge_id']=r['DependencyID']

# Bind staged refinements to actual candidate contract artifacts, never fabricated quotes.
contract_files=['PALETTE_ORGANIZATION.md','COMPARISON_EXPORT.md','APPLICATION_OUTCOME.md']
for s,filename in zip(staged,contract_files):
    contract=OUT/'staged-contracts'/filename
    assert contract.exists(),contract
    r=next(r for r in added if r['DependencyID']==s['edge_id'])
    r.update(TargetLocation=str(contract.relative_to(PROJECT)),EvidenceFile=str(contract.relative_to(PROJECT)),
             SourceRef='Contract stages and readiness / negative witnesses',EvidenceQuote='',Explicitness='IMPLICIT',Confidence='MEDIUM')
    r['Notes']+=' DERIVED CANDIDATE JUDGMENT: stage interpretation is proposed and must be reviewed; the candidate artifact exists but is not adopted normative schema or satisfied evidence.'
    s['candidate_contract_path']=str(contract.relative_to(PROJECT))

# Accepted bytes are an exact prefix, including line endings and unmodified satisfaction fields.
for name,columns,delta in [('DependencyEdges.csv',fields,added),('DeliverableNodes.csv',nfields,newnodes)]:
    original=(BASE/name).read_bytes()
    assert original.endswith(b'\n')
    (OUT/name).write_bytes(original+csv_bytes(columns,delta,False))
(OUT/'DependencyEdges.additions.csv').write_bytes(csv_bytes(fields,added))
(OUT/'ExistingRelations.csv').write_bytes(csv_bytes(['Consumer','Upstream','PreservedDependencyID','Treatment','Reason'],existing))
(OUT/'StagedInterfaces.json').write_text(json.dumps({'status':'PROPOSED_NOT_ADOPTED','bindings':staged},indent=2)+'\n')
manifest={'status':'PROPOSED_NOT_ADOPTED','accepted_baseline':'DAG-010','modifications':0,'retirements':0,'added_rows':len(added),'added_nodes':len(newnodes),
 'preserved_rows':len(old),'preserved_nodes':len(oldnodes),'existing_relations_reused':len(existing),'outputs':{},
 'maturity_policy':'New execution dependencies require proposed SEMANTIC_READY contracts; ProposedMaturity=TBD and SatisfactionStatus=PENDING. Existing rows unchanged are historical satisfaction evidence, not revalidated today.',
 'canonical_pointer_changed':False,'local_mirrors_changed':False}
for name in ['DependencyEdges.csv','DeliverableNodes.csv']:
    data=(OUT/name).read_bytes(); original=(BASE/name).read_bytes()
    manifest['outputs'][name]={'sha256':sha(data),'baseline_sha256':sha(original),'baseline_bytes':len(original),'baseline_is_exact_byte_prefix':data.startswith(original)}
(OUT/'MANIFEST.json').write_text(json.dumps(manifest,indent=2)+'\n')
print(json.dumps({k:manifest[k] for k in ['added_rows','added_nodes','preserved_rows','preserved_nodes','existing_relations_reused']}))
