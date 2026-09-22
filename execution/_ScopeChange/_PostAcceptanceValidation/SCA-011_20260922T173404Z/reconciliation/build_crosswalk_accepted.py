#!/usr/bin/env python3
"""SCA-011 bounded ownership derivative. No discovery or product-state mutation."""
import argparse, csv, hashlib, io, json, re
from pathlib import Path
HERE=Path(__file__).resolve().parent
ROOT=next(p for p in HERE.parents if (p/'projects/chirality-piping/AGENTS.md').exists())
PROJECT=ROOT/'projects/chirality-piping'
SCA=PROJECT/'execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP'
ORIGINAL=SCA/'application/reconciliation'
POST=HERE.parent
RECON=PROJECT/'execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS'
R6=RECON/'BACKCHECK/R6_2026-09-22'
READS={}
def read(path):
    p=Path(path); data=p.read_bytes(); READS[str(p.relative_to(ROOT))]=hashlib.sha256(data).hexdigest(); return data.decode(errors="replace")
def rows(p): return list(csv.DictReader(io.StringIO(read(p))))
def csvtext(rs):
    out=io.StringIO(); w=csv.DictWriter(out,fieldnames=list(rs[0]),lineterminator='\n');w.writeheader();w.writerows(rs);return out.getvalue()
def hook(d,c): return f'{d}:SOW#{c}'
def accepted_carriers():
    read(POST/'OWNER_DECISION.md'); read(POST/'PLAN.md')
    decomp=json.loads(read(SCA/'evidence/SOURCES_DECOMPOSITION.json'))['group3_promotion_metadata']['files']
    interface=json.loads(read(SCA/'interfaces/INTERFACE_PROMOTION.json'))['group3_files']
    accepted={r['target']:r['accepted_sha256'] for r in decomp+interface}
    actual=json.loads(read(POST/'carriers/FINAL_TARGET_MANIFEST.json'))
    assert actual['status']=='PASS'
    assert {r['target']:r['actual_sha256'] for r in actual['targets']}==accepted, 'Final carrier manifest differs from reviewed accepted transformations'
    assert len(accepted)==51, 'Final carrier denominator changed'
    for path,expected in accepted.items():
        read(ROOT/path)
        assert READS[path]==expected, f'Accepted carrier mismatch: {path}'
    return accepted

def build():
    accepted=accepted_carriers()
    for p in ['AGENTS.md','agents/AGENT_TASK.md','projects/chirality-piping/AGENTS.md','workflows/index.json','workflows/bounded-reconciliation/WORKFLOW.md'] :read(ROOT/p)
    for p in ['Propagation_Plan.md','DECISION_PACKAGE.md','Amendment_Actions.csv','interfaces/SOURCE_OWNER_ASSESSMENT.md']:read(SCA/p)
    read(PROJECT/'execution/_ScopeChange/checkpoint_snapshots/SCA-011_GROUP-2_2026-09-22/DECISION.md')
    read(PROJECT/'execution/_ScopeChange/checkpoint_snapshots/SCA-011_GROUP-2_2026-09-22/ACCEPTED_MANIFEST.csv')
    read(PROJECT/'execution/_Coordination/_PROPOSALS/R5_R6_FOLLOWUP_2026-09-22/PIPING_OWNERSHIP_PROPOSAL.md')
    coverage={r['CapabilityID']:r for r in rows(RECON/'R3/CAPABILITY_COVERAGE.csv')}
    accounting=rows(R6/'CAPABILITY_ACCOUNTING.csv'); claims={r['ClaimKey']:r for r in rows(R6/'CLAIM_DISPOSITIONS.csv')}
    inventory={r['CapabilityID']:r for p in sorted((RECON/'R1_INVENTORY').glob('INV_*.csv')) for r in rows(p) if r['CapabilityID'].startswith('CAP-')}
    hrows=[r for r in rows(RECON/'R3/SCOPE_CHANGE_HANDOFF/SCOPE_CHANGE_ITEMS.csv') if r['ItemID'].startswith('H1-')]
    hbycap={}
    for h in hrows:
        for cap in h['Capabilities'].split(';'):
            if cap:hbycap.setdefault(cap,[]).append(h)
    manifests=rows(SCA/'APPLY_MANIFEST.csv'); sows={}; contexts={}
    for m in manifests:
        p=ROOT/m['CanonicalTarget']
        if p.name=='ScopeOfWork.md':
            txt=read(p); assert hashlib.sha256(txt.encode()).hexdigest()==accepted[m['CanonicalTarget']],f'Postimage differs: {p}'
            d=re.search(r'deliverable_id: (DEL-\d\d-\d\d)',txt)[1];sows[d]=(p,txt)
            for name in ['_STATUS.md','MEMORY.md']:
                q=p.parent/name
                if q.exists():read(q)
    for d in ['DEL-07-09','DEL-16-01']:
        q=list((PROJECT/'execution').glob(f'PKG-*/1_Working/{d}_*/_CONTEXT.md'))[0];contexts[d]=(q,read(q))
        for name in ['_STATUS.md','MEMORY.md']:
            p=q.parent/name
            if p.exists():read(p)
    # Additional source/consumer check requested before freeze; all are existing responsibility contracts.
    read(PROJECT/'execution/_Decomposition/SOFTWARE_DECOMP.md')
    for d in ['DEL-03-06','DEL-13-01']:
        folder=next((PROJECT/'execution').glob(f'PKG-*/1_Working/{d}_*'))
        for name in ['ScopeOfWork.md','_STATUS.md','MEMORY.md']:read(folder/name)
    for name in ['workspace/workspaceSession.ts','offline-proposal-intake/workflowSupport.ts','offline-proposal-intake/OfflineProposalIntakePanel.tsx','hanger-selection/hangerSelection.ts','hanger-selection/HangerSelectionPanel.tsx','self-weight-authoring/SelfWeightPlanPanel.tsx']:
        read(PROJECT/'apps/desktop/src/features'/name)
    mappings={}
    def add(cap,group,d,c,facet,relation='CLAIMED_BY',constraints=''):
        assert cap in coverage,cap
        assert cap not in mappings,cap
        mappings[cap]={'Proposal':group,'ForwardOwner':d,'ForwardClaimHook':hook(d,c),'Relation':relation,'AssignedFacet':facet,'BoundaryConstraints':constraints}
    # Exact H1 subjects cited by P-OWN-01. Primitive CAP-PHYS-022 is deliberately not transferred.
    for h in hrows:
        n=int(h['ItemID'][3:]); cap=h['Capabilities']
        if n in list(range(1,17))+list(range(18,29)):
            if n==18: add(cap,'P-OWN-01','DEL-04-01','CLM-030','Curved-member primitive retained; integration consumes it.');continue
            c='CLM-002' if cap in ['CAP-PHYS-009','CAP-PHYS-010'] or cap.startswith('CAP-SOLVER-') else 'CLM-001'
            if cap=='CAP-PHYS-007':c='CLM-003'
            if cap in ['CAP-PHYS-003','CAP-PHYS-017','CAP-PHYS-018','CAP-PHYS-026','CAP-PHYS-027']:c='CLM-004'
            add(cap,'P-OWN-01','DEL-04-07',c,'Product composition/adapter or assembled-loop facet only; primitive mathematics retained.','CLAIMED_BY','DEL-03-08 section/mass; DEL-04-01 assembly; DEL-04-04 classification; DEL-04-06 diagnostics; PKG-05 load/recovery/status. Corrosion/friction/convergence and engineering evidence unchanged.')
        elif n in list(range(29,59))+[61,62]:
            c='CLM-002' if cap in ['CAP-WSUI-011','CAP-WSUI-012','CAP-WSUI-001','CAP-WSUI-003','CAP-WSUI-005'] else 'CLM-001'
            if cap=='CAP-SHELL-051':c='CLM-003'
            if cap=='CAP-WSUI-044':c='CLM-002'
            if cap in ['CAP-SHELL-018','CAP-SHELL-053','CAP-WSUI-008','CAP-WSUI-020']:c='CLM-004'
            add(cap,'P-OWN-02','DEL-07-11',c,'Common shell/host/session composition only; feature panel semantics stay with their owners.','CLAIMED_BY','Persistence DEL-02-05; run records DEL-14-02; feature review DEL-07-08; solve-progress DEL-07-07; architecture DEL-00-03/05/07; security policy unchanged.')
        elif 63<=n<=76:
            add(cap,'P-OWN-03','DEL-16-06','CLM-001' if cap=='CAP-COREB-002' else 'CLM-003','Controlled application facet; validation and operation semantics are not transferred.','CLAIMED_BY','Schema DEL-16-01; validation DEL-16-02; explicit acceptance/audit DEL-16-03; persistence DEL-02-05. No schema waiver or actor/retention resolution.')
    for cap,c in [('CAP-COREB-005','CLM-002'),('CAP-COREB-019','CLM-003'),('CAP-SHELL-019','CLM-003')]:
        add(cap,'P-OWN-03','DEL-16-06',c,'Application gate/native-wasm application facet only; validation-only and utility facets retain original ownership.')
    for cap in ['CAP-WSUI-027','CAP-WSUI-028','CAP-WSUI-029','CAP-FEATC-003','CAP-FEATC-022']:
        add(cap,'P-OWN-02/P-OWN-03','DEL-07-08','CLM-030','Review/apply interaction or proposal review retained; shared session is DEL-07-11 and execution DEL-16-06.')
    for cap in ['CAP-FEATB-022','CAP-FEATB-023','CAP-FEATB-025','CAP-FEATC-016','CAP-FEATC-020','CAP-FEATC-031','CAP-FEATC-036']:
        add(cap,'P-OWN-04','DEL-07-03','CLM-042','Load/library/material editor or self-weight interaction; all changes use PKG-16.','CLAIMED_BY','DEL-07-09 covers vocabulary; DEL-05-01 generation; DEL-03-08 mass; DEL-03-02/07 hanger schema/provenance; DEL-04-07 integration.')
    for cap in ['CAP-FEATC-004','CAP-FEATC-005','CAP-FEATC-034','CAP-WSUI-022','CAP-WSUI-024']:
        add(cap,'P-OWN-04','DEL-07-02','CLM-038','Generic table/inspector or support editing/intent preparation; no dedicated support slice.','CLAIMED_BY','DEL-07-09 vocabulary coverage; DEL-07-03 load-manager editing; PKG-16 operation contract.')
    for cap in ['CAP-VIEW-025','CAP-VIEW-026']:add(cap,'P-OWN-04','DEL-07-01','CLM-033','Palette rendering/routing implementation.','CLAIMED_BY','DEL-07-09 single vocabulary/organization contract; held and unsupported routes remain visible.')
    for cap in ['CAP-COREB-046','CAP-DATA-051']:add(cap,'P-OWN-04','DEL-03-07','CLM-027','Hanger provenance/rights fixture boundary retained; no catalog or import-rights grant.')
    add('CAP-DATA-006','P-OWN-04','DEL-03-02','CLM-030','Hanger schema responsibility retained; UI and coverage do not own schema.')
    for cap in ['CAP-FEATB-008','CAP-FEATB-009','CAP-WSUI-035','CAP-COREB-030']:add(cap,'P-OWN-06','DEL-07-04','CLM-033','Rule-check/status/unit-aware warning presentation only.','CLAIMED_BY','PKG-06 rule semantics; DEL-05-04 status semantics; DEL-02-02 units; protected numerical policy unchanged.')
    add('CAP-SHELL-008','P-OWN-02','DEL-07-11','CLM-002','Workspace-session bootstrap and native fixture dispatch only; no transfer of model/design-knowledge schema or fixture policy.','CLAIMED_BY','CLM-001 host/service route also applies. DEL-13-01 owns knowledge schema/provenance; silent fallback/current-source evidence requires checking, not acceptance by ownership.')
    add('CAP-FEATC-023','P-OWN-02/P-OWN-04','DEL-07-08','CLM-030','Offline proposal review preparation/currentness facet; shared hanger/self-weight consumers are separately DEL-07-03 CLM-042. No exclusive whole-file ownership.','COVERS','DEL-16-01/02 constrain operation/schema/preview; helper defaults do not establish validation, acceptance or authentication. See RESIDUAL_FACET_BINDINGS.csv.')
    add('CAP-PHYS-021','P-OWN-01','DEL-04-07','CLM-001','Product adapter/composition of user-supplied expansion-joint contracts and load/kernel handoffs only.','CLAIMED_BY','DEC-045 already selects dedicated user-stiffness macro-element and load-side pressure thrust; DEL-03-06 owns component contract, PKG-04/05 mathematical primitives remain unchanged. ER-35 and conformance are not closed.')
    panels=rows(SCA/'interfaces/GUI_CONTRACT_BINDINGS.csv')
    panelcaps={'native-package':'CAP-FEATC-021','caepipe-mbf':'CAP-FEATC-008','caepipe-external':'CAP-FEATC-007','stress-neutral':'CAP-FEATC-032','pcf-export':'CAP-FEATC-024','review-geometry':'CAP-FEATC-028','handoff':'CAP-FEATC-015','local-fea-handoff':'CAP-FEATC-019','external-prover':'CAP-FEATC-014','adapter-framework':'CAP-FEATC-002','headless-runner':'CAP-FEATC-017','export-adapter-sdk':'CAP-FEATC-011','validation-evidence':'CAP-FEATB-031'}
    panelout=[]
    for p in panels:
        cap=panelcaps[p['panel']]
        add(cap,'P-OWN-05','DEL-07-12','CLM-001','GUI consumption/rendering/explicit dispatch only; compound builder/contract facets stay at source owners.','CLAIMED_BY',p['record_owner']+'; '+p['upstream_constraints']+'; '+p['held_activation'])
        q=PROJECT/f"apps/desktop/src/features/{p['panel']}/{p['source_file']}";read(q)
        panelout.append({'CapabilityID':cap,'Panel':p['panel'],'GUIClaimHook':hook('DEL-07-12','CLM-001')+';'+hook('DEL-07-12','CLM-002')+';'+hook('DEL-07-12','CLM-003')+';'+hook('DEL-07-12','CLM-004'),'GUIRelation':'CLAIMED_BY','RecordOwner':p['record_owner'],'RecordRelation':'COVERS','ConstraintOwners':p['upstream_constraints'],'ConstraintRelation':'CONSTRAINS','ContractArtifacts':p['contract_artifacts_project_relative_under_schemas_unless_named'],'CodePath':str(q.relative_to(ROOT)),'CodeSHA256':READS[str(q.relative_to(ROOT))],'ObservedEvidence':'Source file exists; runtime/schema conformance not reassessed.','PreservedObligation':p['preservation_and_evidence'],'HeldActivation':p['held_activation']})
    # Unambiguous internal production-contract subjects only; no GUI/runtime activation.
    for cap in ['CAP-PHYS-035','CAP-PHYS-036']:
        add(cap,'P-OWN-06','DEL-13-04','DEL-13-04-SCA011-REQ-001','TransformResult production contract boundary; existing internal adapter does not gain public API/GUI/runtime/external activation.','COVERS','Exact schema acceptance, wider scalar/runtime traces and DEL-13-04-REQ-012 exclusions unchanged.')
    capout=[]
    for a in accounting:
        cap=a['CapabilityID']; old=coverage[cap]; inv=inventory[cap]; m=mappings.get(cap,{})
        locs=[]
        for token in (inv['EntryPoints'].split(';') if m else []):
            raw=token.split('::')[0].strip(); q=ROOT/raw
            if q.is_file():read(q);locs.append(raw)
            elif q.is_dir():
                for child in sorted(q.rglob('*')):
                    if child.is_file():read(child);locs.append(str(child.relative_to(ROOT)))
        capout.append({'CapabilityID':cap,'Selection':'TARGETED_FACET_BOUND' if m else 'OUTSIDE_TARGETED_DERIVATIVE','OriginalClassification':a['OriginalClassification'],'OriginalProposedOwner':a['ProposedOwner'],'OriginalR5Disposition':a['R5Disposition'],'OriginalNextStep':a['NextStep'],'OriginalCoverageStatus':old['Status'],'OriginalOwners':old['Owners'],'OriginalRelations':old['Relations'],'OriginalOwnerKeys':old['OwnerKeys'],'H1Keys':';'.join(h['ItemID'] for h in hbycap.get(cap,[])),'PreviousPackets':';'.join(sorted({h['BlockedOnPacket'] for h in hbycap.get(cap,[]) if h['BlockedOnPacket']})),'Proposal':m.get('Proposal',''),'ForwardOwner':m.get('ForwardOwner',''),'ForwardClaimHook':m.get('ForwardClaimHook',''),'Relation':m.get('Relation',''),'AssignedFacet':m.get('AssignedFacet','No new ownership conclusion in this bounded derivative.'),'BoundaryConstraints':m.get('BoundaryConstraints','Existing source dispositions remain unchanged.'),'OriginalCapability':old['Capability'],'OriginalCodeEvidence':inv['EntryPoints'],'OriginalTestLocators':inv['Tests'],'OriginalEvidenceNotes':inv['Notes'],'ObservedCodeEvidence':'SOURCE_LOCATOR_PRESENT_UNEXECUTED' if locs and m else ('LOCATOR_UNRESOLVED_UNEXECUTED' if m else 'NOT_REASSESSED'),'CurrentCodePaths':';'.join(locs) if m else '', 'CurrentEvidenceState':'UNKNOWN_CURRENT_CONFORMANCE','RemainingEffect':'UNCHANGED_IMPLEMENTATION_VERIFICATION_AND_ACCEPTANCE_REMAINDERS'})
    residual_facets=[
        {'CapabilityID':'CAP-SHELL-008','Relation':'CLAIMED_BY','ForwardHook':hook('DEL-07-11','CLM-002'),'Facet':'Session bootstrap/native host dispatch','Evidence':'workspace/workspaceSession.ts initialization effect; previewService.ts loadPreviewModel/loadDesignKnowledge; native load_preview_model/load_design_knowledge','RetainedResidual':'DEL-13-01 schema/provenance and fixture policy unchanged; current native fallback/source-identity behavior unverified.'},
        {'CapabilityID':'CAP-FEATC-023','Relation':'CLAIMED_BY','ForwardHook':hook('DEL-07-08','CLM-030'),'Facet':'Offline-review asynchronous preparation/currentness','Evidence':'OfflineProposalIntakePanel.tsx imports usePreparation; reads file then queues whole batch for review','RetainedResidual':'No validation/acceptance/identity result inferred; invalidation and actual product operation route need witnesses.'},
        {'CapabilityID':'CAP-FEATC-023','Relation':'CLAIMED_BY','ForwardHook':hook('DEL-07-03','CLM-042'),'Facet':'Hanger-library/self-weight intent preparation/currentness','Evidence':'HangerSelectionPanel.tsx, hangerSelection.ts and SelfWeightPlanPanel.tsx import shared workflowSupport helpers','RetainedResidual':'Shared helper is consumer implementation detail; mathematical generation, schema and actual application retain existing owners.'},
        {'CapabilityID':'CAP-FEATC-023','Relation':'CONSTRAINS','ForwardHook':hook('DEL-16-02','CLM-027'),'Facet':'Proposed/unvalidated intents cannot bypass validation and explicit acceptance','Evidence':'workflowSupport.ts draftIntent marks not_run/not_applied and requires_user_acceptance; usePreparation suppresses obsolete callback consumption','RetainedResidual':'Source declarations are not measured conformance; actor type is not authentication.'},
        {'CapabilityID':'CAP-PHYS-021','Relation':'CLAIMED_BY','ForwardHook':hook('DEL-04-07','CLM-001'),'Facet':'Product input adapter/composition into stiffness and pressure-load handoffs','Evidence':'product_physics/src/lib.rs build_expansion_joint_user_stiffness_elements and expansion_joint_pressure_thrust_inputs_by_pipe consume source model fields','RetainedResidual':'DEL-03-06 schema/DOF contract and DEC-045 remain governing; primitive mathematics and ER-35 engineering assessment are not transferred or accepted.'},
        {'CapabilityID':'CAP-PHYS-021','Relation':'COVERS','ForwardHook':hook('DEL-04-07','CLM-004'),'Facet':'Product diagnostic/provenance and review-result handoff','Evidence':'product_physics/src/lib.rs append_expansion_joint_user_stiffness_results and append_expansion_joint_pressure_thrust_results','RetainedResidual':'No correctness or pressure/stiffness mapping acceptance follows from a review row.'}
    ]
    unresolved=[]
    reasons={'CAP-PHYS-021':'Integration owner assigned. Remaining: DEL-04-07 must demonstrate source/DOF/load-side contract conformance with DEL-03-06 and PKG-04/05 owners; ER-35 engineering evidence remains. No new generic owner-election gate.', 'CAP-SHELL-008':'Bootstrap/host owner assigned. Remaining: DEL-07-11 checks native lookup/fallback/source-identity and recovery; DEL-13-01 schema/provenance and fixture policy are unchanged.', 'CAP-FEATC-023':'Consumer facets assigned to DEL-07-08 and DEL-07-03. Remaining: those owners check obsolete-result suppression, proposed/validated distinction and operation handoff; no new file-owner deliverable or scope change.'}
    for cap,reason in reasons.items():
        x=next(r for r in capout if r['CapabilityID']==cap)
        assert x['Selection']=='TARGETED_FACET_BOUND'
        unresolved.append({'CapabilityID':cap,'H1Keys':x['H1Keys'],'PreviousPackets':x['PreviousPackets'],'OriginalClassification':x['OriginalClassification'],'OriginalR5Disposition':x['OriginalR5Disposition'],'OwnershipState':'ACCEPTED_RESPONSIBILITY_APPLIED_TO_BOUNDED_FACETS','RemainingWork':reason})
    # Derive affected source claims from exact original ownership links, not invented keys.
    refs={}; invalid=[]
    for cap,m in mappings.items():
        for k in re.findall(r'DEL-\d\d-\d\d:(?:SOW|CONTEXT|STATUS|MEMORY|AB)[^;| ]*',coverage[cap]['OwnerKeys']):
            if k not in claims:invalid.append({'CapabilityID':cap,'OriginalOwnerKey':k,'Disposition':'ORIGINAL_REFERENCE_NOT_IN_R6_DENOMINATOR; no invented claim key'});continue
            refs.setdefault(k,{'caps':set(),'hooks':set(),'reason':set()}); refs[k]['caps'].add(cap);refs[k]['hooks'].add(m['ForwardClaimHook']);refs[k]['reason'].add('Original capability owner key constrains/qualifies assigned facet; no whole-claim transfer.')
    # Explicitly changed ownership/held-envelope subjects; source rows stay immutable.
    for k,r in claims.items():
        d=r['DeliverableID']; subject=r['ClaimSubject']; target=None
        if d=='DEL-07-03' and any(t in subject for t in ['R-005','R-006','PDU-041']):target=hook('DEL-07-02','CLM-038') if ('R-006' in subject and 'R-005' not in subject) else hook('DEL-07-03','CLM-042')+';'+hook('DEL-07-02','CLM-038')
        elif d=='DEL-04-04' and ('loop owned' in subject or 'loop artifact' in subject):target=hook('DEL-04-07','CLM-002')+';'+hook('DEL-04-04','CLM-028')
        elif d=='DEL-16-01' and k in ['DEL-16-01:SOW#CLM-009.r01','DEL-16-01:SOW#CLM-009.r03']:target=hook('DEL-16-06','CLM-001')+';'+hook('DEL-16-02','CLM-027')
        elif d=='DEL-16-03' and 'application outside' in subject:target=hook('DEL-16-06','CLM-004')+';'+hook('DEL-16-03','CLM-029')
        elif d in ['DEL-13-03','DEL-13-04','DEL-14-03','DEL-14-04','DEL-14-05','DEL-08-04'] and re.search(r'PDU-023|PDU-011|ValidationResult|TransformResult|output.schema|comparison.export',subject,re.I):target=hook(d,d+'-SCA011-REQ-001')
        if target:
            refs.setdefault(k,{'caps':set(),'hooks':set(),'reason':set()});refs[k]['hooks'].update(target.split(';'));refs[k]['reason'].add('Forward home/boundary supplied; original runtime/schema/operator/engineering residual not discharged.')
    # Shared helper has two concrete consumer homes; preserve both in source-claim links.
    for x in refs.values():
        if 'CAP-FEATC-023' in x['caps']:x['hooks'].add(hook('DEL-07-03','CLM-042'))
    claimout=[]
    for k,x in sorted(refs.items()):
        r=claims[k]
        claimout.append({'ClaimKey':k,'SourceCapabilities':';'.join(sorted(x['caps'])),'OriginalDisposition':r['OriginalDisposition'],'OriginalRoute':r['OriginalRoute'],'OriginalPacket':r['Packet'],'OriginalR5Outcome':r['R5Outcome'],'OriginalClaimSubject':r['ClaimSubject'],'OriginalRepairProposal':r['OriginalRepairProposal'],'OriginalReason':r['Reason'],'OriginalNextHolder':r['NextHolder'],'ForwardClaimHooks':';'.join(sorted(x['hooks'])),'Relation':'CONSTRAINS','MappingMeaning':' '.join(sorted(x['reason'])),'CurrentEvidenceState':'NOT_REASSESSED; ORIGINAL_STATE_PRESERVED','RemainingEffect':'UNCHANGED_IMPLEMENTATION_VERIFICATION_AND_ACCEPTANCE_REMAINDERS'})
    for m in mappings.values():
        d=m['ForwardOwner'];frag=m['ForwardClaimHook'].split('#',1)[1];assert d in sows and frag in sows[d][1],m
    for x in claimout:
        for h in x['ForwardClaimHooks'].split(';'):
            d=h.split(':')[0];frag=h.split('#')[1];assert d in sows and frag in sows[d][1],h
    boundaryout=[]
    def boundary(subject,d,relation,h,meaning):
        p,txt=contexts[d] if ':CONTEXT#' in h else sows[d]
        token='## SCA-011 responsibility' if ':CONTEXT#' in h else h.split('#')[1]
        assert token in txt,h
        boundaryout.append({'Subject':subject,'Owner':d,'Relation':relation,'CurrentHook':h,'SourcePath':str(p.relative_to(ROOT)),'SourceSHA256':READS[str(p.relative_to(ROOT))],'Meaning':meaning,'VerificationState':'REQUIRED_NOT_ESTABLISHED_BY_OWNERSHIP'})
    boundary('Vocabulary and editor coverage','DEL-07-09','COVERS','DEL-07-09:CONTEXT#sca-011-responsibility','Coverage of palette/load/support routes; implementation and OPEN lifecycle remain distinct.')
    boundary('Palette organization','DEL-07-09','CONSTRAINS','DEL-07-09:CONTEXT#sca-011-responsibility','One organization contract constrains DEL-07-01 implementation; not a reverse execution prerequisite.')
    for d,c,subject in [('DEL-07-01','CLM-033','Palette implementation'),('DEL-07-02','CLM-038','Generic/support editor implementation'),('DEL-07-03','CLM-042','Load/library/self-weight editor implementation'),('DEL-16-02','CLM-027','Validation/preview'),('DEL-16-03','CLM-029','Acceptance/audit policy')]:
        boundary(subject,d,'CLAIMED_BY',hook(d,c),'Existing responsibility retained; schema, user acceptance and actual outcome are separate checks.')
    for d in ['DEL-13-03','DEL-13-04','DEL-14-05','DEL-14-03','DEL-14-04','DEL-08-04']:
        relation='CLAIMED_BY' if d in ['DEL-13-03','DEL-13-04','DEL-14-05'] else 'COVERS'
        boundary('Production interface ownership' if relation=='CLAIMED_BY' else 'Production interface producer/consumer binding',d,relation,hook(d,d+'-SCA011-REQ-001'),'Exact wire/schema acceptance, conformance and applicable runtime/PDU/operator/engineering residuals remain open; draft wrappers are not authoritative.')
    for row in residual_facets:
        h=row['ForwardHook'];d=h.split(':')[0];frag=h.split('#')[1];assert frag in sows[d][1],h
    assert len(accounting)==598 and len(claims)==9889
    assert not invalid,invalid
    counts={'original_capability_denominator':598,'targeted_capability_facets':len(mappings),'outside_targeted_derivative':598-len(mappings),'original_claim_denominator':9889,'targeted_original_claims':len(claimout),'claims_outside_targeted_derivative':9889-len(claimout),'gui_panel_bindings':len(panelout),'new_delivery_homes':4,'new_product_tests':0,'implementation_remainders_closed':0,'current_conformance_proved':0,'whole_corpus_owned_claim':False}
    return capout,claimout,panelout,boundaryout,unresolved,residual_facets,counts

def check_keys(rs,key,expected):
    keys=[r[key] for r in rs]; assert len(keys)==len(set(keys)),f'Duplicate {key}'
    assert set(keys)==set(expected),f'Missing/invalid {key}'

def main():
    parser=argparse.ArgumentParser();parser.add_argument('--check',action='store_true');args=parser.parse_args()
    caps,claims,panels,boundaries,unresolved,residual_facets,summary=build()
    artifacts={'CAPABILITY_FORWARD_CROSSWALK.csv':csvtext(caps),'CLAIM_FORWARD_CROSSWALK.csv':csvtext(claims),'GUI_PANEL_FORWARD_BINDINGS.csv':csvtext(panels),'BOUNDARY_RELATIONS.csv':csvtext(boundaries),'UNRESOLVED_BOUNDARIES.csv':csvtext(unresolved),'RESIDUAL_FACET_BINDINGS.csv':csvtext(residual_facets),'SUMMARY.json':json.dumps(summary,indent=2)+'\n'}
    comparison=[]
    for name,data in artifacts.items():
        original=(ORIGINAL/name).read_text()
        if name=='BOUNDARY_RELATIONS.csv':
            previous=list(csv.DictReader(io.StringIO(original)))
            current=list(csv.DictReader(io.StringIO(data)))
            assert len(previous)==len(current)
            for old,new in zip(previous,current):
                assert {k:v for k,v in old.items() if k!='SourceSHA256'}=={k:v for k,v in new.items() if k!='SourceSHA256'}, 'Semantic boundary mapping changed'
        else: assert original==data, f'Semantic derivative changed: {name}'
        comparison.append({'path':name,'original_sha256':hashlib.sha256(original.encode()).hexdigest(),'accepted_sha256':hashlib.sha256(data.encode()).hexdigest(),'change':'source hashes only' if name=='BOUNDARY_RELATIONS.csv' else 'byte-identical'})
    comparison_path=HERE/'BEFORE_AFTER_COMPARISON.json'
    if args.check:assert json.loads(comparison_path.read_text())==comparison
    else:comparison_path.write_text(json.dumps(comparison,indent=2)+'\n')
    for name,data in artifacts.items():
        p=HERE/name
        if args.check:assert p.read_text()==data,f'Derivative drift: {name}'
        else:p.write_text(data)
    all_hooks=set()
    for collection,field in [(caps,'ForwardClaimHook'),(claims,'ForwardClaimHooks'),(boundaries,'CurrentHook'),(residual_facets,'ForwardHook')]:
        for row in collection:all_hooks.update(h for h in row[field].split(';') if h)
    quotes=[]
    for h in sorted(all_hooks):
        d=h.split(':')[0];frag=h.split('#')[1]
        filename='_CONTEXT.md' if ':CONTEXT#' in h else 'ScopeOfWork.md'
        paths=list((PROJECT/'execution').glob(f'PKG-*/1_Working/{d}_*/{filename}'))
        assert len(paths)==1,h
        path=paths[0];lines=read(path).splitlines()
        starts=[i for i,line in enumerate(lines) if (line.startswith('### '+frag+' ') or line.startswith('- **'+frag+'**') or (':CONTEXT#' in h and line=='## SCA-011 responsibility'))]
        assert len(starts)==1, f'Nonunique current hook: {h}'
        start=starts[0];quote='\n'.join(lines[start:start+7])
        quotes.append({'Hook':h,'SourcePath':str(path.relative_to(ROOT)),'SourceSHA256':READS[str(path.relative_to(ROOT))],'FirstLine':str(start+1),'ExactCurrentExcerpt':quote,'ExcerptSHA256':hashlib.sha256(quote.encode()).hexdigest()})
    quote_path=HERE/'CURRENT_HOOK_BINDINGS.csv'
    if args.check:assert quote_path.read_text()==csvtext(quotes)
    else:quote_path.write_text(csvtext(quotes))
    bindings={'schema':'sca011-reconciliation-accepted-source-bindings/v2','selected_workflow':'bundled:chirality-root/bounded-reconciliation','role':'TASK','parent':'/root/piping_scope_manager','agent':'/root/piping_scope_manager/piping_applied_reconciliation','delegation':'Codex native collaboration TASK; no children; filesystem brief boundary, not OS subtree isolation','reviewed_applied_commit':'d6cc1482eee78ce860ff18658f11157f7efbd401','actual_group3_decision':str((POST/'OWNER_DECISION.md').relative_to(ROOT)),'inputs':[{'path':p,'sha256':v} for p,v in sorted(READS.items())]}
    original_bindings=json.loads((SCA/'application/SOURCES_RECONCILIATION_APPLICATION.json').read_text())
    original_inputs={r['path']:r['sha256'] for r in original_bindings['inputs']}
    changes=[]
    accepted=accepted_carriers()
    for path,old_hash in original_inputs.items():
        assert path in READS, f'Original supplied source omitted: {path}'
        new_hash=READS[path]
        if old_hash!=new_hash:
            assert path in accepted, f'Unreviewed source change: {path}'
            assert new_hash==accepted[path]
            changes.append({'path':path,'reviewed_applied_sha256':old_hash,'accepted_sha256':new_hash})
    changes_path=HERE/'SOURCE_BINDING_COMPARISON.json'
    if args.check:assert json.loads(changes_path.read_text())==changes
    else:changes_path.write_text(json.dumps(changes,indent=2)+'\n')
    for record in json.loads((HERE/'FROZEN_ORIGINALS.json').read_text()):
        assert hashlib.sha256((ROOT/record['path']).read_bytes()).hexdigest()==record['sha256'], 'Historical reconciliation evidence changed'
    p=HERE/'SOURCES_RECONCILIATION_ACCEPTED.json' 
    if args.check:assert json.loads(p.read_text())==bindings,'Input source hash drift'
    else:p.write_text(json.dumps(bindings,indent=2)+'\n')
    probes=[]
    for label,rs,key in [('capability',caps,'CapabilityID'),('claim',claims,'ClaimKey')]:
        expected=[r[key] for r in rs];check_keys(rs,key,expected)
        variants={'missing':rs[:-1],'duplicate':rs+[rs[0]],'invalid':[{**rs[0],key:'INVALID-SOURCE-KEY'}]+rs[1:]}
        for defect,bad in variants.items():
            try:check_keys(bad,key,expected)
            except AssertionError:probes.append(label+':'+defect+':REJECTED')
            else:raise AssertionError('Negative probe accepted '+label+defect)
    result={'result':'PASS','mode':'check' if args.check else 'generate','counts':summary,'negative_probes':probes,'limitations':'Structural/source-bound ownership derivative only. No runtime, schema-conformance, native, engineering, operator acceptance or lifecycle result.'}
    print(json.dumps(result,indent=2))
if __name__=='__main__':main()
