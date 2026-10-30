#!/usr/bin/env python3
"""Build candidate-only local mirrors; never writes any live deliverable."""
import csv
import hashlib
import io
import json
from collections import Counter
from pathlib import Path
OUT=Path(__file__).resolve().parent
ROOT=OUT.parents[5]
PROJECT=ROOT/'projects/chirality-piping'

def read(p):
    with p.open(newline='') as f:
        r=csv.DictReader(f);return r.fieldnames,list(r)
def sha(b):return hashlib.sha256(b).hexdigest()
def rendered(fields,rows,header=True):
    s=io.StringIO(newline='');w=csv.DictWriter(s,fieldnames=fields,lineterminator='\n')
    if header:w.writeheader()
    w.writerows(rows);return s.getvalue().encode()
fields,additions=read(OUT/'DependencyEdges.additions.csv')
_,nrows=read(OUT/'DeliverableNodes.csv');nodes={n['DeliverableID']:n for n in nrows}
evidence_root='projects/chirality-piping/execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP/dependencies'
promotion={'status':'PREPARED_CONDITIONAL_GROUP2_GROUP3_TRANSFORMS_NOT_EXECUTED','group2_condition':'Execute only after the actual Group 2 application authorization and candidate/baseline checks. DAG-010 remains accepted while added rows await Group 3.','group3_condition':'Execute only after actual Group 3 audited poststate acceptance and recorded DAG-011 adoption. No transformation itself establishes acceptance.','files':[],'group3_files':[]}
manifest={'status':'PROPOSED_NOT_ADOPTED','application':'Owning TASK must backcheck candidate source documents and local baselines before application; this is an exact preview, not execution of dependency-extract or graph activation.','files':[]}
for ident in sorted({r['FromDeliverableID'] for r in additions}):
    delta=[r for r in additions if r['FromDeliverableID']==ident]
    landing=PROJECT/nodes[ident]['ExecutionPath']
    prior=landing/'Dependencies.csv';prior_index=landing/'_DEPENDENCIES.md'
    destination=OUT/'local-mirrors'/ident;destination.mkdir(parents=True,exist_ok=True)
    old=[];source=None
    if prior.exists():
        localfields,old=read(prior)
        assert set(localfields).issubset(fields),(ident,'unknown local columns')
        assert all(not r[k] for r in delta for k in fields if k not in localfields),(ident,'nonempty omitted extension')
        source=prior.read_bytes()
        assert source.endswith(b'\n')
        data=source+rendered(localfields,[{k:r[k] for k in localfields} for r in delta],False)
    else:data=rendered(fields,delta)
    allrows=old+delta
    (destination/'Dependencies.csv').write_bytes(data)
    classes=Counter(r['DependencyClass'] for r in allrows)
    statuses=Counter(r['Status'] for r in allrows)
    satisfaction=Counter(r['SatisfactionStatus'] for r in allrows)
    index=f'''# Candidate dependency mirror — {ident}

**PROPOSED; NOT ADOPTED.** Prepared under SCA-011 group 2. Live graph authority remains DAG-010. This file describes only the adjacent candidate CSV; no source lifecycle or contract readiness is promoted.

## Declared upstream/downstream lists

No new human-declared list is created. Existing declared lists, if any, are preserved in the quoted historical index below. Candidate relations are EXTRACTED from the ownership proposal, and are not declared owner-approved dependency rows.

## Extracted Dependency Register

{len(allrows)} rows: {dict(statuses)}. Classes: {dict(classes)}. Preserved local rows: {len(old)}; additive rows: {len(delta)}; changed/retired rows: zero.

| Candidate edge | Type | Target | Satisfaction |
|---|---|---|---|
'''
    for r in delta:index+=f'| {r["DependencyID"]} | {r["DependencyClass"]}/{r["DependencyType"]} | {r["TargetRefID"]} | {r["SatisfactionStatus"]} |\n'
    index+=f'''
## Run Notes

Historical preparation method: ad hoc project graph preparation with candidate-only local previews; dependency-extract was consulted for canonical schema/provenance rules, not executed outside its deliverable-local scope. Mode UPDATE-equivalent additive preview; strictness CONSERVATIVE; source documents explicitly limited to P-OWN-01–06 and its dependency table plus the preserved prior mirrors. The preparation used the SCA-011 candidate decomposition and preserved its earlier accepted basis. That preparation step did not change source documents, live local mirrors or the approved DAG. The current application/adoption state is stated at the top of this index; extraction-time candidate wording in CSV Notes is retained provenance, not a later authority claim.

New execution rows use RequiredMaturity=SEMANTIC_READY, ProposedMaturity=TBD, SatisfactionStatus=PENDING. They require actual named contract/source-owner evidence before a production dependency may be called satisfied. Existing satisfaction fields are preserved historical evidence and were not revalidated by this preparation. DOCUMENT stage bindings use repository-root-anchored evidence paths: `{evidence_root}/STAGED_INTERFACE_RATIONALE.md` and `{evidence_root}/StageGraph.json`. These paths remain valid from the live deliverable landing; they are not relative to this preview directory.

## Run History

- 2026-09-22: SCA-011 candidate-only additive preview; {len(delta)} added rows. Earlier local history is quoted below without rewriting its historical claims.

## Lifecycle Summary

Candidate row status: {dict(statuses)}. Candidate satisfaction counts: {dict(satisfaction)}. Exactly one active parent anchor is required and checked. These are dependency row states, not deliverable lifecycle changes.

## Downstream Handoff Notes

Revalidate this preview against the exact candidate source documents and current local baseline under a bounded owning TASK before application. No new source-witness/readiness conclusion follows from this file. Graph authority adoption and pointer changes remain separate. The aggregate preserves DAG-010; it does not copy unrelated local drift back into authority.
'''
    if prior_index.exists():
        historical=prior_index.read_text()
        index+='\n## Quoted historical index (not current candidate counts or graph pointer)\n\nThe following source bytes are retained as historical context; all counts and current-pointer statements inside this fence belong to their earlier record.\n\n```markdown\n'+historical+'\n```\n'
    (destination/'_DEPENDENCIES.md').write_text(index)
    # Conditional metadata-only transitions. Candidate files remain unmodified.
    changes2=[
        (f'# Candidate dependency mirror — {ident}', f'# Dependency mirror — {ident}'),
        ('**PROPOSED; NOT ADOPTED.** Prepared under SCA-011 group 2. Live graph authority remains DAG-010. This file describes only the adjacent candidate CSV; no source lifecycle or contract readiness is promoted.',
         '**APPLIED; GROUP 3 PENDING.** SCA-011 Group 2 applied this local mirror. DAG-010 remains the accepted graph authority; the added rows await Group 3 audited poststate acceptance and DAG-011 adoption. No source lifecycle, contract readiness or dependency satisfaction is promoted.'),
        ('Candidate relations are EXTRACTED from the ownership proposal, and are not declared owner-approved dependency rows.',
         'Added relations are EXTRACTED from the ownership amendment. Their Group 2 application is recorded; their adoption into graph authority awaits Group 3. Origin=EXTRACTED records provenance and is not a human declaration.'),
        ('| Candidate edge | Type | Target | Satisfaction |','| Added edge | Type | Target | Satisfaction |'),
        (f'- 2026-09-22: SCA-011 candidate-only additive preview; {len(delta)} added rows. Earlier local history is quoted below without rewriting its historical claims.',
         f'- 2026-09-22: SCA-011 candidate-only additive preview; {len(delta)} added rows. Earlier local history is quoted below without rewriting its historical claims.\n- SCA-011 Group 2: applied these exact additive rows under the recorded application authorization; Group 3 remains pending.'),
        (f'Candidate row status: {dict(statuses)}. Candidate satisfaction counts: {dict(satisfaction)}.',
         f'Local row status: {dict(statuses)}. Local satisfaction counts: {dict(satisfaction)}.'),
        ('Revalidate this preview against the exact candidate source documents and current local baseline under a bounded owning TASK before application. No new source-witness/readiness conclusion follows from this file. Graph authority adoption and pointer changes remain separate. The aggregate preserves DAG-010; it does not copy unrelated local drift back into authority.',
         'Group 2 applied this local mirror against the recorded source documents and baseline hashes. Group 3 must audit the poststate and record the DAG-011 authority propagation before the new relations become graph authority. No source-witness/readiness conclusion follows from application. All added execution relations remain PENDING with actual maturity TBD; baseline satisfaction evidence was preserved, not revalidated.')
    ]
    applied=index
    ops2=[]
    for before,after in changes2:
        assert applied.count(before)==1,(ident,before)
        ops2.append({'operation':'replace_exact','old':before,'new':after,'expected_occurrences':1})
        applied=applied.replace(before,after,1)
    changes3=[
        (changes2[1][1], '**ACCEPTED DEPENDENCY MIRROR.** SCA-011 Group 3 audited the poststate and recorded DAG-011 adoption. Current graph authority is DAG-011; DAG-010 is the preserved predecessor. All added execution relations remain PENDING with actual maturity TBD; adoption does not promote source lifecycle, contract readiness or dependency satisfaction.'),
        (changes2[2][1], 'Added relations are EXTRACTED from the ownership amendment and adopted into DAG-011 through the recorded Group 3 propagation. Origin=EXTRACTED records provenance and is not a human declaration.'),
        ('- SCA-011 Group 2: applied these exact additive rows under the recorded application authorization; Group 3 remains pending.',
         '- SCA-011 Group 2: applied these exact additive rows under the recorded application authorization.\n- SCA-011 Group 3: audited poststate accepted and DAG-011 adopted through the recorded authority propagation; execution satisfaction remains pending.'),
        (changes2[-1][1], 'Group 3 audited this local mirror against the recorded applied poststate and adopted DAG-011 as current graph authority. The accepted DAG-010 predecessor remains immutable. All added execution relations remain PENDING with actual maturity TBD; baseline satisfaction evidence was preserved, not revalidated. Future satisfaction changes require actual source-owner and consumer evidence; no hold, lifecycle, engineering or release acceptance follows from graph adoption.')
    ]
    accepted=applied;ops3=[]
    for before,after in changes3:
        assert accepted.count(before)==1,(ident,before)
        ops3.append({'operation':'replace_exact','old':before,'new':after,'expected_occurrences':1})
        accepted=accepted.replace(before,after,1)
    target=str((landing/'_DEPENDENCIES.md').relative_to(ROOT))
    promotion['files'].append({'target':target,'candidate_path':str((destination/'_DEPENDENCIES.md').relative_to(ROOT)),
        'candidate_sha256':sha(index.encode()),'operations':ops2,'applied_sha256':sha(applied.encode())})
    promotion['group3_files'].append({'target':target,'applied_sha256':sha(applied.encode()),'operations':ops3,'accepted_sha256':sha(accepted.encode())})
    csv_target=str((landing/'Dependencies.csv').relative_to(ROOT))
    csv_hash=sha(data)
    promotion['files'].append({'target':csv_target,'candidate_path':str((destination/'Dependencies.csv').relative_to(ROOT)),
        'candidate_sha256':csv_hash,'operations':[],'applied_sha256':csv_hash})
    promotion['group3_files'].append({'target':csv_target,'applied_sha256':csv_hash,'operations':[],'accepted_sha256':csv_hash})
    # All live-location evidence references are anchored from repository root.
    for path in [evidence_root+'/STAGED_INTERFACE_RATIONALE.md',evidence_root+'/StageGraph.json']:
        assert (ROOT/path).is_file(),path
    assert '../../STAGED_INTERFACE_RATIONALE.md' not in index.split('## Quoted historical index')[0]

    for name in ['Dependencies.csv','_DEPENDENCIES.md']:
        oldfile=landing/name;newfile=destination/name
        manifest['files'].append({'deliverable':ident,'candidate':str(newfile.relative_to(PROJECT)),
            'proposed_landing':str(oldfile.relative_to(PROJECT)),'sha256':sha(newfile.read_bytes()),
            'baseline_sha256':sha(oldfile.read_bytes()) if oldfile.exists() else None,
            'baseline_csv_rows':len(old),'added_rows':len(delta),'baseline_csv_bytes_preserved':bool(source and data.startswith(source)) if name=='Dependencies.csv' and source else None})
    assert sum(r['Status']=='ACTIVE' and r['AnchorType']=='IMPLEMENTS_NODE' for r in allrows)==1,ident
    assert len({r['DependencyID'] for r in allrows})==len(allrows),ident
manifest['conditional_promotion_manifest']='LOCAL_MIRRORS_PROMOTION.json'
manifest['conditional_group2_group3_hashes']={f['target']:{'candidate_sha256':f['candidate_sha256'],'applied_sha256':f['applied_sha256'],'accepted_sha256':g['accepted_sha256']} for f,g in zip(promotion['files'],promotion['group3_files'])}
(OUT/'LOCAL_MIRRORS_PROMOTION.json').write_text(json.dumps(promotion,indent=2)+'\n')
(OUT/'LOCAL_MIRRORS_MANIFEST.json').write_text(json.dumps(manifest,indent=2)+'\n')
print(f'{len(manifest["files"])} candidate-only local files across {len(manifest["files"])//2} owners')
