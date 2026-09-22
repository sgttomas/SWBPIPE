from pathlib import Path
import json,hashlib,csv,datetime,platform,sys,difflib
ROOT=Path.cwd();P=Path('projects/chirality-piping');R=P/'execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP';H=P/'execution/_ScopeChange/_PostAcceptanceValidation/SCA-011_20260922T173404Z';O=H/'active_audit';sha=lambda b:hashlib.sha256(b).hexdigest()
sources=json.loads((O/'SOURCES.json').read_text());prior_sources=json.loads((O/'SOURCES.json').read_text());changes=[]
for p,v in sources['sources'].items():
 if ':'in p.split('/')[0]:continue
 f=ROOT/p
 if f.is_file()and sha(f.read_bytes())!=v['sha256']:
  changes.append({'path':p,'before':v['sha256'],'after':sha(f.read_bytes())});assert p==str(H/'dependencies/RETURN.md'),f'Unexpected input drift: {p}';sources['sources'][p]={'sha256':sha(f.read_bytes()),'bytes':f.stat().st_size,'basis':'packaging_only_return_refresh'}
(O/'PRE_PACKAGING_SOURCES.json').write_text(json.dumps(prior_sources,indent=2)+'\n')
old=H/'dependencies/preacceptance/projects/chirality-piping/execution/_DAG/_LATEST.md';new=old.with_name('latest-pointer-before.txt');assert not old.exists()and sha(new.read_bytes())=='9da625a75f75819fe39a05532d67697fa6de0d7a84982c4e2e1b89c6d3ef80b3'
for p in [new,H/'dependencies/POINTER_EVIDENCE_PACKAGING_REPAIR.json',H/'dependencies/OUTPUT_MANIFEST.json',H/'dependencies/ADOPTION_VERIFICATION.json',H/'CURRENT_VALIDATION.json']:
 if p.is_file():sources['sources'][str(p)]={'sha256':sha(p.read_bytes()),'bytes':p.stat().st_size,'basis':'final_append_only_packaging_backcheck'}
(O/'PACKAGING_BACKCHECK.json').write_text(json.dumps({'status':'PASS','timestamp':datetime.datetime.now(datetime.timezone.utc).isoformat(),'old_noncurrent_pointer_copy':str(old),'old_path_absent':True,'new_historical_evidence_path':str(new),'original_and_new_sha256':sha(new.read_bytes()),'affected_prior_input_bindings':changes,'other_consulted_current_inputs_unchanged':True,'canonical_current_pointer_and_graph_unchanged':True,'scope':'Historical evidence-file rename and dependency return/output-manifest update only; retain unaffected active-audit checks.'},indent=2)+'\n')
(O/'SOURCES.json').write_text(json.dumps(sources,indent=2)+'\n');raw=json.loads((O/'raw_checks.json').read_text());assert all(x['pass']for x in raw['checks']);summary=json.loads((O/'coverage_summary.json').read_text());n=len(raw['checks']);recon_inputs=len(next(x['detail']for x in raw['checks']if x['check']=='accepted_derivative_actual_source_currency'));hooks=len(next(x['detail']for x in raw['checks']if x['check']=='current_derivative_hooks_and_loci'))
checks=[('1','Forward packages','PASS','18 declared and materialized packages.'),('2','Forward deliverables','PASS','106 declared and materialized deliverables.'),('3','Reverse folders','PASS','No orphan folders or undeclared IDs.'),('4','ID consistency','PASS','Accepted deliverable/scope identities and parent coupling retained; exact reviewed metadata transforms checked.'),('5','Context fidelity','PASS','All106 identity/name/package/type/envelope rows agree; four new finite controls remain3L+1M.'),('6','Artifact presence','INCOMPLETE','All97 SoWs validate; eight architecture references and one accepted custom OPEN contract present. Full anticipated production-artifact realization remains unassessed.'),('7','Objectives','PASS','All18 objective statements unchanged; displayed support sets agree,including corrected OBJ018.'),('8','Ledger integrity','WARNING','79 scope rows and companion mappings resolve. Stock tool retains2,097 historical findings;zero new findings.'),('9','DOMAIN derivative parity','SKIPPED','SOFTWARE variant; explicit accepted-source derivative currency checked separately.'),('9b','Package shape','WARNING','Pre-existing missing explicit companion inventory/heavy inline duplication retained; current machine truth parity passes.'),('10','Active snapshot/handoff','PASS','Actual owner Group3 decision,418 manifest bindings,accepted SCA011 and adopted DAG011 authority verified. Sealed handoff truthfully points to append-only completion evidence.'),('11','Lifecycle distribution','PASS','100 IN_PROGRESS,one ISSUED,five OPEN;four new homes remain OPEN and all prior lifecycle states preserved.')]
summary['per_check_verdicts']=[{'check':a,'name':b,'verdict':c,'detail':d}for a,b,c,d in checks];summary['active_finalization']={'reviewed_applied_commit':'d6cc1482eee78ce860ff18658f11157f7efbd401','owner_decision':str(H/'OWNER_DECISION.md'),'raw_checks_passed':n,'reviewed_final_targets':71,'unchanged_other_targets':12,'accepted_manifest_bindings':418,'new_dependency_quotes_checked':84,'current_derivative_source_bindings_checked':recon_inputs,'current_derivative_hooks_checked':hooks,'active_scope_snapshot':'SCA-011','active_graph':'DAG-011','product_or_engineering_acceptance':False,'new_human_checkpoint':False,'packaging_backcheck':'PACKAGING_BACKCHECK.json'};(O/'coverage_summary.json').write_text(json.dumps(summary,indent=2)+'\n')
report=f'''# SCA-011 accepted active-state audit

**WARNINGS; zero blocking findings.** All {n} actual-state checks pass. Closure-readiness remains WARN because the pre-existing package-shape warning and stock baseline findings are disclosed; the actual Group3 decision has already accepted this scope-only poststate. This audit creates no new human checkpoint.

The actual owner answer, “Accept the audited result and adopt DAG-011”, is preserved in `{H}/OWNER_DECISION.md`. The reviewed applied-state commit is `d6cc1482eee78ce860ff18658f11157f7efbd401`. Both active pointers now select accepted SCA-011 and adopted DAG-011. The immutable SCA handoff records downstream verification IN_PROGRESS at its seal and directs recovery to this append-only home; it is not overwritten to fabricate an earlier completion.

| Check | Name | Verdict | Evidence / limitation |
|---|---|---|---|
'''+''.join('|'+ '|'.join(x)+'|\n'for x in checks)+f'''
## Exact authority and postimage checks

All71 targets in the three reviewed final-transform manifests are checked against immutable reviewed preimages,exact replacement counts and accepted output hashes. All83 targets pass:those71 final targets plus12 unchanged targets. Some final targets carry no-op operations; “71” describes the reviewed conditional target set rather than asserting71 changed files. Manager FINAL_TARGET_MANIFEST agrees with the independently derived hashes. All418 accepted-manifest entries match actual bytes. Original Group2 application and earlier audit evidence remain untouched.

Current decomposition is revision0.13,current_basis. Coverage is18 packages,106 deliverables,79 scope items and18 objectives. Identity,name,type,context,scope and objective parity pass. All24 affected SoWs retain their claim/output/acceptance/verification identifiers; all97 current SoWs pass the document validator. Existing status/memory pairs were read and hashed; memory remains continuity evidence,not authority. Core-artifact checks do not claim realization of every anticipated implementation artifact.

## Active dependency and derivative evidence

A fresh canonical strict graph audit passes on actual adopted bytes:106 nodes,1,571 rows,1,034 active deliverable edges,zero canonical/endpoint findings,cycles,duplicate or bidirectional edges. The owning current-source stage report is bound to the actual graph hashes and preserves1,041 stage edges and its reported negative probes. The main graph tool does not itself cover DOCUMENT-stage expansion; these are distinct checks,not interchangeable claims.

All84 added dependency quotes are checked against current accepted source hashes and named loci,including the30-word limit. Local/aggregate parity passes; all1,487 predecessor rows and102 predecessor nodes are preserved. Finalization changes no endpoint,direction,type,statement,scope,maturity,satisfaction or quote text. All62 added execution relations remain PENDING with actual maturity TBD. Graph adoption is not prerequisite satisfaction.

The new ownership derivative’s {recon_inputs} input hashes match actual sources and its {hooks} hook excerpts match current hashes and locations. Six mapping/summary/residual artifacts are byte-identical to the reviewed derivative; BOUNDARY_RELATIONS changes only source hashes. The same598 original capability keys,120 targeted responsibility facets,478 outside facets,121 original claim links and13 GUI bindings remain. No original disposition or residual is closed. This is accepted-source rebinding,not a repeat of R1–R6 or a new corpus-wide conformance census.

## Retained baseline and limits

Fresh stock validation exits1 with exactly2,097 previously recorded finding keys andzero new findings. Its discovery still misses companions located in docs/_Registers; this audit independently parses and compares those authoritative companion tables. No tool finding is silently waived and no nonzero result is reported as clean. The inherited missing companion-inventory/heavy-duplication warning remains. Group2’s temporary168 new dependency metadata findings were corrected before acceptance and do not recur.

Protected Piping product core,apps,schemas,tests,DAG010,SCA009 and original R6 paths remain unchanged from the original accepted amendment basis. The reviewed application/evidence/candidate/interface/dependency packages remain unchanged from the reviewed applied-state commit. Draft wire schemas remain excluded. No product tests,engineering acceptance,native qualification,privacy/external activation,lifecycle issuance or release claim is made. Full artifact realization and historical satisfaction are not recertified.

A historical copied `_DAG/_LATEST.md` in dependency evidence was renamed to `latest-pointer-before.txt` because the shared live scanner interpreted its old name as an active pointer. The historical bytes are identical; actual authority pointers,graph/source bytes and earlier checks remain unchanged. PACKAGING_BACKCHECK records the narrow metadata refresh and preserves the preceding source bindings.

## Return

Active-state audit is complete with no unresolved blocker. The separate fresh finalization reviewer and parent-owned Git/CI closeout follow. Parent records completed recovery state in CURRENT_STATE.md after actual returns; that record was deliberately not required or fabricated before this audit ran. Scope-only acceptance does not close the retained implementation,professional,planning or release obligations.
'''
# Readable spacing in prose while leaving capitalized identifiers intact.
import re
report=re.sub(r'(?<=[a-z])(?=\d)',' ',report);report=re.sub(r'(?<=\d)(?=[a-z])',' ',report)
(O/'Decomp_Coverage_Report.md').write_text(report)
(O/'Brief.md').write_text(f'''# Active audit TASK brief

Parent: /root/piping_scope_manager. Executor: /root/piping_scope_manager/piping_postchange_audit. Role TASK, Codex native collaboration, inherited host model/effort, no override or children. Write only `{O}`. Read-only Git source inspection is used; no Git or canonical mutations.

Actual Group3 owner acceptance is in ../OWNER_DECISION.md. Audit exact71 final-transform targets plus12 unchanged postimages, active SCA011/DAG011 authority, lifecycle/claim/coverage parity, preserved dependencies/holds and current accepted-source ownership derivative. Adapt the prior audit with exact recorded changes; do not rerun R1–R6 or product tests. Wait for stable author signals before actual checks. Manager and dependency writer provided those signals. Preserve all earlier application/audit evidence and use this single append-only evidence home. A separate finalization reviewer follows.

Workflow: bundled:chirality-root/audit-decomp; SOFTWARE; ALL18 packages/106 deliverables; actual accepted active state. Explicit assigned output location overrides default _Evaluation snapshot/pointer. No CURRENT_STATE completion record is required before actual results exist.
''')
(O/'ADAPTATION.md').write_text('''# Audit adaptation and execution record

The original reviewed audit script is preserved verbatim as original_audit_poststate.py.txt; ADAPTATION_SOURCE.json binds its original identity. ADAPTATION.patch records every executable change to audit_active_state.py. The selected method remains bundled:chirality-root/audit-decomp.

The 29 prior checks were retained or strengthened for their actual postacceptance subjects: pending-pointer preservation became actual owner-decision and accepted-pointer verification; initial/repaired postimage expectations became the71 reviewed exact final transforms plus12 unchanged targets; original repair hashes are checked against immutable reviewed bytes rather than incorrectly against promoted current bytes. Old preimage custody remains checked. Current quote bindings use the accepted-source record instead of stale Group2 hashes. Lifecycle/claims/coverage/schema/ledger/objective/stock-baseline predicates remain active.

Additional predicates verify418 accepted snapshot bindings,manager-target parity,fresh strict graph output,actual-input-bound stage evidence,unchanged relation fields,current derivative input hashes,unchanged mappings/residuals and current hook loci. No failing criterion was removed. Original historical audit is not represented as a passing live-current replay. No old script or result was overwritten.

The dependency evidence-pointer rename arrived after the successful actual-state run. finish_active_audit.py verifies identical renamed historical bytes,absence of the old pointer-shaped filename,and unchanged consulted current sources except the explicitly updated dependency return. PRE_PACKAGING_SOURCES.json and PACKAGING_BACKCHECK.json preserve that narrow refresh. Unaffected checks were not rerun.

Reproduction: run audit_active_state.py and then finish_active_audit.py from repository root,writing only this active_audit output directory. Save a distinct immutable snapshot before any later rerun against a changed state. The original audited state remains preserved at the reviewed commit. Exact command/output and Python/platform identity are retained in CHECKS.json and raw reports.
''')
(O/'Decision_Log.md').write_text('''# Decisions and limits

- Actual Group3 acceptance changes the expected authority state; it does not promote delivery lifecycle or satisfaction.
- Bind reviewed transforms from immutable d6cc1482eee78ce860ff18658f11157f7efbd401 and retain original amendment3e18334ec comparisons where they prove unchanged identity/lifecycle.
- Final output is in the one designated postacceptance home. Do not modify accepted SCA snapshot or historical application/audit evidence.
- All semantic-heading bindings resolve without ambiguity. No parsing fallback or invented source is used.
- The prior29 checks supply lineage; current checks run on actual accepted bytes. Stock findings compare by identity against the original2,097 baseline keys.
- Reconciliation checking is source/mapping currency,not repetition of the original full census or implementation assessment.
- The immutable accepted handoff’s IN_PROGRESS wording is truthful at seal; the manager’s append-only CURRENT_STATE records completion after returns.
- Evidence pointer packaging repair is independently bound without changing actual graph authority or repeating unaffected checks.
''')
(O/'QA_Report.md').write_text(f'''# QA

{n}/{n} raw checks pass;106-row matrix;all97 current SoWs validate;24 affected SoWs retain local IDs/hooks;71 exact transform targets+12 unchanged targets pass;418 accepted-manifest hashes pass;84 current dependency quote/locus checks pass;{recon_inputs} derivative source bindings and{hooks} current hooks pass. Fresh strict graph validation exits0. Stock register validation exits1 with2,097 unchanged baseline finding keys andzero new findings. Package shape retains one grouped warning.

Per-check limits remain explicit:full production-artifact realization INCOMPLETE;DOMAIN derivative-specific check SKIPPED for SOFTWARE;engineering/product/native and original satisfaction not recertified. Current source hashes,read-only immutable commit bytes,status/memory pairs,commands and raw outputs support the bounded claims. The renamed historical pointer evidence is byte-identical and prior source bindings are preserved. No additional human checkpoint or acceptance of protected obligations is inferred.
''')
(O/'RUN_SUMMARY.md').write_text(f'''# Accepted active-state audit return

RUN_STATUS = WARNINGS

All {n} actual checks pass;zero blockers. Accepted SCA011 and adopted DAG011 are active.71 reviewed final targets+12 unchanged targets and418 snapshot bindings verify. Coverage18 packages/106 deliverables/79 scope items/18 objectives; lifecycle unchanged,all four new homes OPEN.97 SoWs validate;24 affected claim/hook sets preserved.

All84 current dependency source quotes/loci pass;1,487 predecessor rows remain unchanged;fresh canonical strict graph passes. Accepted ownership derivative source currency passes with unchanged mappings/residuals. Historical application/audit evidence remains preserved.

Stock baseline remains2,097 identical findings,zero new findings;one baseline package-shape warning. Currentness claims are bounded to scope/metadata/evidence propagation. No product,engineering,lifecycle or release acceptance follows. Separate finalization review and parent Git/CI closeout remain.
''')
adapt=json.loads((O/'ADAPTATION_SOURCE.json').read_text());adapt['status']='EXECUTED_CURRENT_ACCEPTED_STATE_PASS_WITH_RETAINED_BASELINE_WARNINGS';adapt['adapted_sha256']=sha((O/'audit_active_state.py').read_bytes());adapt['remaining_bindings']='NONE';(O/'ADAPTATION_SOURCE.json').write_text(json.dumps(adapt,indent=2)+'\n')
(O/'ADAPTATION.patch').write_text(''.join(difflib.unified_diff((O/'original_audit_poststate.py.txt').read_text().splitlines(True),(O/'audit_active_state.py').read_text().splitlines(True),fromfile=adapt['source'],tofile=str(O/'audit_active_state.py'))))
meta={'python':sys.version,'platform':platform.platform(),'actual_raw_checks':n,'failed_checks':[],'matrix_rows':106,'registered_stock_exit_code':1,'stock_baseline_findings':2097,'stock_new_findings':0,'artifact_sha256':{p.name:sha(p.read_bytes())for p in O.iterdir()if p.is_file()and p.name!='CHECKS.json'}};(O/'CHECKS.json').write_text(json.dumps(meta,indent=2)+'\n');print(json.dumps({'status':'WARNINGS','raw_checks_passed':n,'blockers':0,'final_source_bindings':len(sources['sources']),'packaging_only_refresh':changes},indent=2))
