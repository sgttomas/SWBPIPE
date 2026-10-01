from pathlib import Path
from fractions import Fraction as F
import subprocess,os,json,hashlib,csv,re
ROOT=Path(__import__('sys').argv[1]).resolve(); WT=ROOT.parent; OUT=WT/'scratch/rv27-resume'
T3=Path('projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3');R=T3/'RESUME_2026-09-30';PK=ROOT/R/'_run_records/aud_t3_04';ENV=dict(os.environ,GIT_OPTIONAL_LOCKS='0',PYTHONDONTWRITEBYTECODE='1')
def sha(b):return hashlib.sha256(b).hexdigest()
def git(*args):return subprocess.check_output(['git',*args],cwd=ROOT,env=ENV)
result={'input_manifest_failures':[],'sweeps':[],'suite_counts':[],'bindings':[]}
for s in (PK/'INPUT_SHA256SUMS').read_text().splitlines():
 h,p=s.split(None,1);p=p.strip();b=Path(p.replace('<wt>',str(WT))).read_bytes() if p.startswith('<wt>') else git('show','3446bdf51d90b8f1812d161cc638d85c97bbef16:'+p)
 if h!=sha(b):result['input_manifest_failures'].append(p)
for row in csv.DictReader((PK/'checks/SWEEP_IDENTITIES.tsv').open(),delimiter='\t'):
 name=row['sweep'];folder=PK/'sweeps'/name;source=WT/'scratch'/name; sp=next(folder.glob('SWEEP*'));raw=source/sp.name
 slice_name={'sweep_vk':'VK','sweep_k6b':'K6B'}.get(name,name.removeprefix('sweep_').upper()); merge=ROOT/T3/'IMPLEMENTATION'/f'{slice_name}_MERGE'; prior=(merge/'dec025/sweep_json_original_sha256.txt').read_text();j=json.loads(sp.read_text()); record=(merge/'RECORD.md').read_text()
 logs=sorted((folder/'suites').glob('*.log'));manifests=(folder/'suites/manifests.txt').read_text().splitlines(); summary=(folder/'suites.log').read_text();fail=[]
 for index,manifest in enumerate(manifests):
  p=folder/'suites'/f"{index:03}_{manifest.replace('/','_')}.log";s=p.read_text();counts=[sum(int(m[i]) for m in re.findall(r'^test result:.*?(\d+) passed; (\d+) failed; (\d+) ignored;',s,re.M)) for i in range(3)]
  expected=f'passed={counts[0]} failed={counts[1]} ignored={counts[2]}'
  if not re.search(r'^\d+ '+re.escape(expected+' '+manifest)+'$',summary,re.M):fail.append(manifest)
 result['sweeps'].append({'sweep':name,'source_hash_matches_prior':sha(raw.read_bytes()) in prior,'clean_final_head':j['git']['commit_hash']==row['candidate'] and not j['git']['working_tree_dirty'],'head_in_merge_record':row['candidate'] in record,'manifest_count':len(manifests),'log_count':len(logs),'summary_matches_prior':(source/'suites.log').read_bytes()==(merge/'dec025/suites.log').read_bytes(),'derived_counts_failures':fail,'failure_lines':re.findall(r'^test .* \.\.\. FAILED$', '\n'.join(p.read_text() for p in logs),re.M)})
# Supplied basis hashes record actual origins, not role activation.
for entry in json.loads((ROOT/R/'BASIS.json').read_text())['context']:
 result['bindings'].append({'origin':entry['origin'],'matches':sha((ROOT/entry['origin']).read_bytes())==entry['sha256']})
for entry in json.loads((ROOT/R/'DISPATCH.json').read_text())['agents']:
 result['bindings'].append({'origin':entry['brief'],'matches':sha(git('show','3446bdf51d90b8f1812d161cc638d85c97bbef16:'+entry['brief']))==entry['brief_sha256']})
result['oracle_provenance_hash']=sha((ROOT/R/'oracle_provenance/CHECKPOINT_0.md').read_bytes())
result['carried_records']={str(p): (ROOT/p).read_bytes().startswith(git('show','cd7e5f629f8e3f61aaea79b0216a451888e6bc9e:'+str(p))) for p in [T3/'ROOT_RULINGS_V1.md',T3/'REVIEW/RECORDS_PR1063_REVIEW.md']}
result['numerics_parents']=git('show','-s','--format=%P','4b620df6aba447b58ad099ba6db9c90059231a3f').decode().strip().split()
result['numerical_diff_from_audit']=git('diff','--name-only','3bddc2b05f6106e969c7cf43373b230845c7cc66','90b6bcbbf64b13975211038bf3f33bb87273e646','--','projects/chirality-piping/core/solver','projects/chirality-piping/validation/benchmarks').decode().splitlines()
unit=F(1,2**1074); rotation=F(5,4)*unit; L=F(2**100); verify=L*rotation; pub=F(float(L)*float(rotation)); bound=pub/F(2**64); truth=bound*F(9,8)
result['independent_arithmetic']={'ratio':str(verify/pub),'loss':str((verify-pub)/verify),'premise_fails':verify-pub>(F(1,2**64)+F(1,2**52))*verify+unit/2,'stop_accepts':truth<=verify/F(2**64),'published_interval_misses':truth>bound*(1+F(1,2**22)),'scale_hex':float(pub).hex(),'tiny_published_zero':float(1/ L)*float(rotation)==0.0,'scope':'abstract exact-fraction arithmetic, no solver witness'}
pr=subprocess.run(['gh','pr','view','1063','--repo','sgttomas/chirality','--json','state,headRefOid,mergeCommit,mergedAt,statusCheckRollup'],cwd=ROOT,env=ENV,capture_output=True,text=True);(OUT/'pr1063.json').write_text(pr.stdout);result['pr1063_query_exit']=pr.returncode
(OUT/'BINDINGS.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))
