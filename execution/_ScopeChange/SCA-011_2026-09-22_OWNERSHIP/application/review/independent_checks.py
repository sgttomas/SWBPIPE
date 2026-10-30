"""Independent records-only review; writes solely within this review directory."""
from pathlib import Path
import csv, json, hashlib, io, subprocess, re, collections
ROOT=Path.cwd();P=ROOT/'projects/chirality-piping';S=P/'execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP';O=S/'application/review';BASE='3e18334eca72509475684cc86786b3eeade83572'
sources={};checks=[]
def read(p):
 p=Path(p);b=p.read_bytes();sources[str(p.relative_to(ROOT))]=hashlib.sha256(b).hexdigest();return b
def text(p):return read(p).decode()
def rows(p):return list(csv.DictReader(io.StringIO(text(p))))
def check(name,ok,detail):
 checks.append({'check':name,'pass':bool(ok),'detail':detail});assert ok,(name,detail)
def old(p):return subprocess.check_output(['git','show',BASE+':'+str(p.relative_to(ROOT))],cwd=ROOT)
def digest(b):return hashlib.sha256(b).hexdigest()
manifest=rows(S/'application/APPLIED_REVIEW_MANIFEST.csv');actual=[]
for r in manifest:
 b=read(ROOT/r['Path']);actual.append((r['Path'],digest(b)==r['SHA256'] and len(b)==int(r['Size'])))
 if r['Path'].endswith('.json'):json.loads(b)
 if r['Path'].endswith('.csv'):list(csv.DictReader(io.StringIO(b.decode())))
check('all_frozen_files_read_hash_size_parse',all(v for _,v in actual),{'count':len(actual),'manifest_sha256':digest(read(S/'application/APPLIED_REVIEW_MANIFEST.csv'))})
# Accepted manifests refer to historical source bytes, not mutable handoffs.
a=[]
for n in ['SCA-011_GROUP-1_2026-09-22','SCA-011_GROUP-2_2026-09-22']:
 folder=P/'execution/_ScopeChange/checkpoint_snapshots'/n
 read(folder/'DECISION.md');read(folder/'Handoff_State.md')
 for r in rows(folder/'ACCEPTED_MANIFEST.csv'):
  source=r.get('SourceCommit') or BASE
  b=subprocess.check_output(['git','show',source+':'+r['Path']],cwd=ROOT)
  a.append({'snapshot':n,'path':r['Path'],'pass':digest(b)==r['SHA256']})
check('accepted_snapshot_historical_bytes',all(x['pass'] for x in a),a)
# Record all lifecycle/memory pairs, and independently preserve prior states.
pairs=[]
for f in sorted((P/'execution').glob('PKG-*/1_Working/DEL-*/_STATUS.md')):
 st=text(f);memory=[str(q.relative_to(ROOT)) for q in [f.parent/'_MEMORY.md',f.parent/'MEMORY.md'] if q.exists()]
 for m in memory:read(ROOT/m)
 prior=subprocess.run(['git','show',BASE+':'+str(f.relative_to(ROOT))],capture_output=True).stdout.decode()
 state=re.search(r'\*\*Current State:\*\*\s*(\w+)',st)[1]
 before=re.search(r'\*\*Current State:\*\*\s*(\w+)',prior)
 assert state==(before[1] if before else 'OPEN')
 pairs.append({'status':str(f.relative_to(ROOT)),'state':state,'memory':memory,'memory_authority':'continuity_only'})
check('status_memory_pairs',len(pairs)==106,{'pairs':len(pairs),'distribution':dict(collections.Counter(x['state'] for x in pairs))})
# Full dictionary preservation + semantic delta independent of author's checker.
oldedges=rows(P/'execution/_DAG/DAG-010/DependencyEdges.csv');edges=rows(P/'execution/_DAG/DAG-011/DependencyEdges.csv')
oldnodes=rows(P/'execution/_DAG/DAG-010/DeliverableNodes.csv');nodes=rows(P/'execution/_DAG/DAG-011/DeliverableNodes.csv')
check('all_prior_rows_nodes_preserved',edges[:len(oldedges)]==oldedges and nodes[:len(oldnodes)]==oldnodes,{'rows':len(oldedges),'nodes':len(oldnodes)})
added=edges[len(oldedges):];original=rows(S/'dependencies/DependencyEdges.additions.csv');allowed={'DependencyID','EvidenceFile','SourceRef','EvidenceQuote','Notes'}
check('only_accepted_metadata_fields_changed',all(all(b[k]==c[k] for k in b if k not in allowed) for b,c in zip(original,added)) and len(original)==len(added)==84,sorted(allowed))
check('new_prerequisites_unsatisfied',all((r['RequiredMaturity'],r['ProposedMaturity'],r['SatisfactionStatus'])==('SEMANTIC_READY','TBD','PENDING') for r in added if r['DependencyClass']=='EXECUTION'),62)
# Every literal exists in its cited claim/frontmatter/verification locus.
loc=[]
for r in added:
 s=text(P/r['EvidenceFile']);locus=r['SourceRef'].split(' # ',1)[1];q=r['EvidenceQuote'];support=s
 if locus.startswith('frontmatter/'):
  key=locus.split('/',1)[1];support=next(x for x in s.splitlines() if x.startswith(key+':'))
 elif locus.startswith('CLM-'):
  start=s.index('### '+locus);end=s.find('\n### ',start+4);end2=s.find('\n## ',start+4);end=min([x for x in [end,end2,len(s)] if x>=0]);support=s[start:end]
 elif locus.startswith('VER-'):
  support=next(x for x in s.splitlines() if '**'+locus+'**' in x)
 assert q and q in support and len(q.split())<=30
 assert re.fullmatch('DEP-'+r['FromDeliverableID'][4:]+r'-\d{3}',r['DependencyID'])
 loc.append({'id':r['DependencyID'],'locus':locus,'owner':r['FromDeliverableID'],'target':r['TargetDeliverableID']or r['TargetRefID'],'quote':q})
check('84_source_loci_and_owner_ids',len(loc)==84,loc)
# Kahn topological check independent of audit_dag SCC implementation.
stages=json.loads(text(O/'REPAIRED_StageGraph.json'))['edges'];graph=collections.defaultdict(set);indeg=collections.Counter()
for e in stages:
 a,b=e['upstream'],e['consumer'];indeg[a]+=0
 if b not in graph[a]:graph[a].add(b);indeg[b]+=1
queue=collections.deque(n for n in indeg if indeg[n]==0);order=[]
while queue:
 n=queue.popleft();order.append(n)
 for m in graph[n]:
  indeg[m]-=1
  if indeg[m]==0:queue.append(m)
check('independent_stage_topological_order',len(order)==len(indeg),{'edges':len(stages),'vertices':len(indeg)})
# Conditional Group3 transformations: simulate all metadata operations in memory only.
groups=[(S/'evidence/SOURCES_DECOMPOSITION.json',['group3_promotion_metadata','files']),(S/'interfaces/INTERFACE_PROMOTION.json',['group3_files']),(S/'application/dependencies/REPAIRED_GROUP3_PROMOTION.json',['group3_files'])];future={};trans=[]
for p,keys in groups:
 x=json.loads(text(p))
 for k in keys:x=x[k]
 for f in x:
  b=read(ROOT/f['target']);assert digest(b)==f['applied_sha256']
  for op in f['operations']:
   assert b.count(op['old'].encode())==op['expected_occurrences'];b=b.replace(op['old'].encode(),op['new'].encode())
  assert digest(b)==f['accepted_sha256']
  if f['target'] in future:assert b==future[f['target']]
  future[f['target']]=b;trans.append({'target':f['target'],'operations':len(f['operations']),'hash':digest(b)})
check('all_conditional_transforms_in_memory',True,trans)
# Crosswalk source keys and copied values, not just regenerated counts.
r6=P/'execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/BACKCHECK/R6_2026-09-22';acc=rows(r6/'CAPABILITY_ACCOUNTING.csv');caps=rows(S/'application/reconciliation/CAPABILITY_FORWARD_CROSSWALK.csv')
check('capability_key_universe',len(caps)==598 and {r['CapabilityID'] for r in caps}=={r['CapabilityID'] for r in acc},598)
claims={r['ClaimKey']:r for r in rows(r6/'CLAIM_DISPOSITIONS.csv')};linked=rows(S/'application/reconciliation/CLAIM_FORWARD_CROSSWALK.csv')
fields={'OriginalDisposition':'OriginalDisposition','OriginalRoute':'OriginalRoute','OriginalPacket':'Packet','OriginalR5Outcome':'R5Outcome','OriginalClaimSubject':'ClaimSubject','OriginalRepairProposal':'OriginalRepairProposal','OriginalReason':'Reason','OriginalNextHolder':'NextHolder'}
check('121_claim_original_values_preserved',len(linked)==121 and all(r['ClaimKey'] in claims and all(r[k]==claims[r['ClaimKey']][v] for k,v in fields.items()) for r in linked),{'linked':len(linked),'original':len(claims)})
for p in ['AGENTS.md','agents/AGENT_TASK.md','projects/chirality-piping/AGENTS.md','projects/chirality-piping/loop/LOOP_INIT.md','workflows/scope-change/WORKFLOW.md','workflows/scope-change/resources/method.md','workflows/scope-change/resources/contract.md','.agents/skills/software-code-review/SKILL.md','docs/SPEC.md'] :read(ROOT/p)
for glob in ['PKG-*/1_Working/DEL-03-06*/ScopeOfWork.md','PKG-*/1_Working/DEL-13-01*/ScopeOfWork.md']:
 for p in (P/'execution').glob(glob):read(p)
for p in ['apps/desktop/src/features/workspace/workspaceSession.ts','apps/desktop/src/features/offline-proposal-intake/workflowSupport.ts','apps/desktop/src/features/offline-proposal-intake/OfflineProposalIntakePanel.tsx','apps/desktop/src/features/hanger-selection/hangerSelection.ts','apps/desktop/src/features/hanger-selection/HangerSelectionPanel.tsx','apps/desktop/src/features/self-weight-authoring/SelfWeightPlanPanel.tsx','apps/desktop/src/services/previewService.ts','apps/desktop/src-tauri/src/lib.rs','core/product_physics/src/lib.rs']:read(P/p)
report={'status':'PASS','checks':checks,'status_memory_pairs':pairs,'sources':sources,'delegation':{'mechanism':'Codex native collaboration','parent':'/root/piping_scope_manager','task':'/root/piping_scope_manager/piping_final_poststate_review','role':'TASK','children':0,'filesystem_boundary':'brief-scoped; not OS isolation'},'methods':[{'kind':'workflow','identity':'bundled:chirality-root/scope-change','stage':'Group3 independent review'},{'kind':'skill','identity':'repository:.agents/skills/software-code-review/SKILL.md'}]}
(O/'INDEPENDENT_CHECKS.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'status':report['status'],'checks':len(checks),'sources':len(sources),'future_targets':len(future),'pairs':len(pairs)}))
