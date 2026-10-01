from pathlib import Path
import subprocess,os,hashlib,json,gzip,csv,re,sys
ROOT=Path(__import__('sys').argv[1]).resolve()
WT=ROOT.parent
T3=Path('projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3')
R=T3/'RESUME_2026-09-30'; PK=ROOT/R/'_run_records/aud_t3_04'
OUT=WT/'scratch/rv27-resume'; ENV=dict(os.environ,GIT_OPTIONAL_LOCKS='0',PYTHONDONTWRITEBYTECODE='1')
BASE='d01ad98a754698631f927709d08284c272de85e8'; HEAD='90b6bcbbf64b13975211038bf3f33bb87273e646'
def sha(b): return hashlib.sha256(b).hexdigest()
def shafile(p):
 h=hashlib.sha256()
 with open(p,'rb') as f:
  for b in iter(lambda:f.read(1024*1024),b''):h.update(b)
 return h.hexdigest()
def git(*args): return subprocess.check_output(['git',*args],cwd=ROOT,env=ENV)
def checkmanifest(p,wd):
 rows=[]
 for s in p.read_text().splitlines():
  if not s.strip():continue
  expected,rel=s.split(None,1); rel=rel.removeprefix('*').strip(); f=wd/rel
  rows.append({'path':rel,'expected':expected,'actual':shafile(f) if f.is_file() else None})
 return {'manifest':str(p.relative_to(ROOT)),'entries':len(rows),'failures':[x for x in rows if x['expected']!=x['actual']]}
result={'candidate':HEAD,'base':BASE,'head_actual':git('rev-parse','HEAD').decode().strip(),'status_before':git('status','--porcelain=v1').decode(),'python':sys.version}
paths=git('diff','--name-only',BASE,HEAD).decode().splitlines(); result['changed_files']=len(paths)
(OUT/'changed_paths.txt').write_text('\n'.join(paths)+'\n')
result['scope_unexpected']=[p for p in paths if not (p.startswith(str(R)+'/') or p in [str(T3/'ROOT_RULINGS_V1.md'),str(T3/'REVIEW/RECORDS_PR1063_REVIEW.md'),'projects/chirality-piping/execution/_Coordination/WorkGraphs/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/WORK_GRAPH.md'])]
result['append_only']={p:(ROOT/p).read_bytes().startswith(git('show',BASE+':'+p)) for p in [str(T3/'ROOT_RULINGS_V1.md'),str(T3/'REVIEW/RECORDS_PR1063_REVIEW.md')]}
result['packet_seal']=shafile(PK/'SHA256SUMS');result['packet_manifest']=checkmanifest(PK/'SHA256SUMS',PK)
listed={s.split(None,1)[1].strip() for s in (PK/'SHA256SUMS').read_text().splitlines()};actual={str(p.relative_to(PK)) for p in PK.rglob('*') if p.is_file() and p.name!='SHA256SUMS'}
result['packet_coverage']={'unlisted':sorted(actual-listed),'missing':sorted(listed-actual)}
result['relocation_identity_failures']=[]
old_packet=WT/'preserved-evidence/t3-audit-20260930/packet'
for p in sorted(PK.rglob('*')):
 if p.is_file():
  q=old_packet/p.relative_to(PK)
  if not q.is_file() or shafile(q)!=shafile(p): result['relocation_identity_failures'].append(str(p.relative_to(PK)))
result['archival_manifests']=[]
for row in json.loads((ROOT/R/'verification/hashes.json').read_text())['results']:
 result['archival_manifests'].append(checkmanifest(ROOT/T3/row['manifest'],ROOT/T3/row['working_directory']))
result['audit_manifest']=checkmanifest(ROOT/T3/'AUDIT/SHA256SUMS',ROOT/T3/'AUDIT')
result['audit_unchanged']=not bool(git('diff','--name-only',BASE,HEAD,'--',str(T3/'AUDIT')))
result['replays']={}
v=json.loads((ROOT/R/'verification/verification.json').read_text())
for name,item in v['replays'].items():
 b=(ROOT/R/'verification'/f'{name}.json').read_bytes(); old=(ROOT/T3/'AUDIT/_run_records'/f'{name}.json').read_bytes()
 result['replays'][name]={'actual':sha(b),'recorded':item['sha256'],'historical_bytes_equal':b==old}
basis=json.loads((ROOT/T3/'AUDIT/_run_records/basis.json').read_text())
result['principal_inputs_count']=len(basis['inputs']);result['principal_input_mismatches']=[]
for row in basis['inputs']:
 current=shafile(ROOT/row['path']); before=sha(git('show',BASE+':'+row['path']))
 if current!=row['sha256'] or before!=row['sha256']:result['principal_input_mismatches'].append(dict(row,current=current,base=before))
result['script_binding']=shafile(ROOT/T3/'AUDIT/_run_records/audit_checks.py')==v['script_sha256']==sha(git('show',v['script_origin']))
result['originals']={'count':0,'failures':[]};result['sanitized']={'count':0,'failures':[]}
VENV=str(WT.parent.parent/'projects/chirality-piping/.venv')
def sanitize(b):
 for original,replacement in [(VENV,'<VENV>'),(str(WT),'<wt>'),(str(Path.home()),'<home>')]: b=b.replace(original.encode(),replacement.encode())
 b=re.sub(rb'/(?:private/)?var/folders/[^/]+/[^/]+/T/',b'<tmp>/',b);b=re.sub(rb'/(?:private/)?tmp/',b'<tmp>/',b)
 b=re.sub(rb'\x1b\][^\x07\x1b]*(?:\x07|\x1b\\)',b'',b);b=re.sub(rb'\x1b\[[0-?]*[ -/]*[@-~]',b'',b);return b
for row in csv.DictReader((PK/'ORIGINALS.tsv').open(),delimiter='\t'):
 source=Path(row['source_path'].replace('<wt>',str(WT))); local=Path(row['local_copy'].replace('<LOCAL>',str(WT/'preserved-evidence/t3-audit-20260930')))
 result['originals']['count']+=1
 if shafile(source)!=row['sha256'] or shafile(local)!=row['sha256'] or source.stat().st_size!=int(row['bytes']):result['originals']['failures'].append(row['source_path'])
for row in csv.DictReader((PK/'SANITIZED.tsv').open(),delimiter='\t'):
 source=Path(row['original_source'].replace('<wt>',str(WT))); target=PK/row['packet_path'];b=target.read_bytes()
 result['sanitized']['count']+=1
 if sha(b)!=row['sha256'] or len(b)!=int(row['bytes']) or sanitize(source.read_bytes())!=b:result['sanitized']['failures'].append(row['packet_path'])
result['gate_archives']=[]
prior=(ROOT/T3/'IMPLEMENTATION/KF2/_run_records/b/gate/uncommitted_sha256.txt').read_text()
for row in csv.DictReader((PK/'GATE_ARCHIVES.tsv').open(),delimiter='\t'):
 p=PK/f"gate/part1_{row['side']}.runs.jsonl.gz";h=hashlib.sha256();n=0;count=0;nonobject=0
 with gzip.open(p,'rb') as f:
  for line in f:
   h.update(line);n+=len(line);count+=1; nonobject+=not isinstance(json.loads(line),dict)
 source=WT/f"scratch/i20/b/gate/part1_{row['side']}/runs.jsonl"
 result['gate_archives'].append({'side':row['side'],'restored_sha256':h.hexdigest(),'bytes':n,'records':count,'nonobjects':nonobject,'matches_prior_record':h.hexdigest() in prior,'matches_raw_original':h.hexdigest()==shafile(source),'metadata_matches':h.hexdigest()==row['raw_sha256'] and n==int(row['raw_bytes']) and count==int(row['record_count']) and shafile(p)==row['sanitized_gzip_sha256']})
result['status_after']=git('status','--porcelain=v1').decode()
(OUT/'CHECKS.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({k:v for k,v in result.items() if k not in ['archival_manifests']},indent=2))
print('archival manifests',len(result['archival_manifests']),'entries',sum(x['entries'] for x in result['archival_manifests']),'failures',[r for r in result['archival_manifests'] if r['failures']])
