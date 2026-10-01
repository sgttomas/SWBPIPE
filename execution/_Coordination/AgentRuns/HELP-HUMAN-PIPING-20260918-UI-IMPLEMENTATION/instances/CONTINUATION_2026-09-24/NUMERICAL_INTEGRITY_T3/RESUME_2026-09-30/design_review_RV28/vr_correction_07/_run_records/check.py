"""Same-reviewer additive overlay metadata/integer backcheck; no source execution."""
from pathlib import Path
import hashlib,json,copy,sys
K,OUT=map(Path,sys.argv[1:]);raw=OUT/'_run_records'
T='projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3';R=T+'/RESUME_2026-09-30'
C=R+'/metric_design_11_vr_correction';BASE=R+'/metric_design_10_vr_numbers';PRIOR=R+'/design_review_RV28/vr_numbers_06'
checks=[];records=[]
def sha(b):return hashlib.sha256(b).hexdigest()
def ck(n,x):
 assert x,n
 checks.append(n)
def rd(p):
 b=(K/p).read_bytes();records.append({'origin':'<K6C>','path':p,'sha256':sha(b),'bytes':len(b)});return b
seals=[]
for packet,h in [(C,'21bc713d669f4b4dcfe618a3454a874fef97edba230920b56647f3d63d8b8b14'),(BASE,'a5390504ba0198ebe280877b75d8b374a38c266e780a7e3dc0d7a280f1fb9a9c'),(PRIOR,'cd7e0f6108c88245d8bb729c381528644bcfba0cbb292aca225176a74fcda626')]:
 s=rd(packet+'/SHA256SUMS');ck('seal '+packet,sha(s)==h);n=0
 for line in s.decode().splitlines():
  digest,f=line.split(maxsplit=1);p=K/packet/f
  if not p.exists():p=K/f
  ck('payload '+packet+'/'+f,sha(p.read_bytes())==digest);n+=1
 seals.append({'path':packet,'sha256':h,'entries':n})
rd(C+'/RETURN.md');rd(C+'/CORRECTION.md');rd(PRIOR+'/REVIEW.md')
overlay=json.loads(rd(C+'/OVERLAY.json'));basebytes=rd(BASE+'/VR_CALLER_TABLE.json');base=json.loads(basebytes);desc=json.loads(rd(C+'/_run_records/DESCRIPTORS.json'))
ck('table binding',sha(basebytes)==overlay['candidate_table_sha256']);ck('base seal binding',overlay['candidate_seal']==seals[1]['sha256']);ck('review binding',overlay['review_seal']==seals[2]['sha256'])
old={x['id']:x for x in base['rows']};changes={x['id']:x for x in overlay['rows']};descs={x['id']:x for x in desc}
ck('unique complete24',len(old)==len(changes)==len(descs)==24 and len(overlay['rows'])==24 and set(old)==set(changes)==set(descs))
prior=json.loads(rd(PRIOR+'/_run_records/DOMINANCE_AND_SUMMARY.json'));truth={(r['id'],r['metric']):r for r in prior['phase_findings']}
fam=rd('projects/chirality-piping/validation/benchmarks/numerical_robustness/cases/rf_large.jsonl');ck('fixture identity',sha(fam)=='16357afa5efeaaac3ab5798bb6f632104bec1e2dfada288f5b0936bfe6188759')
cases={c['id']:c for c in map(json.loads,fam.splitlines())}
E='expected_list_initialization_while_outcome_alive';N='nonselected_diagnostic_while_outcome_alive';out=[]
joined=copy.deepcopy(base);newbyid={x['id']:x for x in joined['rows']}
for id,c in changes.items():
 f=cases[id];I=len(id.encode());key=max(len(x[0].encode()) for x in f['rows']);exp=max(len(x[1].encode()) for x in f['rows']);B=old[id]['shape']['B'];dout=102+23*B;why=max(9,40,75,key+18)
 terms=[I+key+exp+6+why,I+key+94,I+93,I+56,I+36,I+39+dout,I+75,I+20,I+16+dout];df=max(terms);failure=96+4*max(8,2*df)
 ck(id+' diagnostic descriptors',terms==descs[id]['Dfail_candidates'] and df==descs[id]['Dfail']==c['Dfail']);ck(id+' PriorFailures',failure==descs[id]['PriorFailures']==c['PriorFailures'])
 ck(id+' exact metrics',set(c['metrics'])=={'requested','moving'})
 for metric,m in c['metrics'].items():
  slots=old[id][metric]['kernel_phase_addends'];reps=m['replacements'];ck(id+metric+' exact fields',set(reps)=={E,N})
  changed=dict(slots)
  for name,delta in [(E,failure),(N,78)]:
   expected=slots[name]+delta;v=reps[name];ck(id+metric+name+' old/delta/new',(v['old'],v['delta'],v['new'])==(slots[name],delta,expected));changed[name]=expected;newbyid[id][metric]['kernel_phase_addends'][name]=expected
   field='F1_corrected_candidate' if name==E else 'F2_corrected_candidate';ck(id+metric+name+' independent prior result',expected==truth[id,metric][field])
  before=max(v for k,v in slots.items() if k!='solve_max');after=max(v for k,v in changed.items() if k!='solve_max')
  ck(id+metric+' common max',before==after==m['common_outcome_max_old']==m['common_outcome_max_new'])
  ck(id+metric+' other addends untouched',all(changed[k]==slots[k] for k in slots if k not in [E,N]))
  out.append({'id':id,'metric':metric,'PriorFailures':failure,'E_old':slots[E],'E_new':changed[E],'N_old':slots[N],'N_new':changed[N],'common_max':after,'E_margin':before-changed[E],'N_margin':before-changed[N],'comparison_margin':slots['selected_or_other_outcome_comparison']-changed[E]})
ck('48 rows96 replacements',len(out)==48)
ck('minimum margins',min(x['E_margin'] for x in out)==272896 and min(x['N_margin'] for x in out)==99241 and min(x['comparison_margin'] for x in out)==228723)
ck('retained range',min(x['PriorFailures'] for x in out)==1592 and max(x['PriorFailures'] for x in out)==1608)
# Check a second authenticated metadata application recognizes exact-new values,
# while an unknown value, stale roster or unauthenticated new-value state rejects.
def apply_metadata(table,authenticated=False):
 data=copy.deepcopy(table);rows={x['id']:x for x in data['rows']}
 if len(rows)!=24 or len(data['rows'])!=24 or set(rows)!=set(changes):raise ValueError('roster')
 for id,c in changes.items():
  for metric,m in c['metrics'].items():
   for field,repl in m['replacements'].items():
    v=rows[id][metric]['kernel_phase_addends'][field]
    if v==repl['old']:rows[id][metric]['kernel_phase_addends'][field]=repl['new']
    elif not(authenticated and v==repl['new']):raise ValueError('value')
 return data
ck('exact first application',apply_metadata(base)==joined);ck('authenticated idempotence',apply_metadata(joined,True)==joined)
for label,bad,auth in [('unauthenticated new',joined,False),('bad roster',dict(base,rows=base['rows'][:-1]),True)]:
 try:apply_metadata(bad,auth);ok=False
 except ValueError:ok=True
 ck(label+' rejected',ok)
bad=copy.deepcopy(base);bad['rows'][0]['requested']['kernel_phase_addends'][E]+=1
try:apply_metadata(bad);ok=False
except ValueError:ok=True
ck('unknown value rejected',ok)
for metric in ['requested','moving']:ck(metric+' caller max unchanged',max(r[metric]['caller_only_max'] for r in joined['rows'])==max(r[metric]['caller_only_max'] for r in base['rows']))
(raw/'BASIS.json').write_text(json.dumps({'agent':'/root/rv28_a1_design','parent':'/root','role':'TASK','mechanism':'collaboration.followup_task','instruction_basis':'same active session; sealed prior RV28 instruction/skill bindings','source_records':records,'seals':seals,'limits':'No source algorithm or broader proof rerun; only overlay metadata and integer arithmetic'},indent=2)+'\n')
(raw/'REPLACEMENTS_CHECKED.json').write_text(json.dumps(out,indent=2)+'\n');(raw/'CHECKS.json').write_text(json.dumps({'status':'PASS','count':len(checks),'checks':checks},indent=2)+'\n')
print(json.dumps({'status':'PASS','checks':len(checks),'cases':24,'metric_rows':48,'replacement_values':96,'common_outcome_maxima_unchanged':48,'overlay_idempotence':'authenticated exact-new only','full_Emax_accepted':False}))
