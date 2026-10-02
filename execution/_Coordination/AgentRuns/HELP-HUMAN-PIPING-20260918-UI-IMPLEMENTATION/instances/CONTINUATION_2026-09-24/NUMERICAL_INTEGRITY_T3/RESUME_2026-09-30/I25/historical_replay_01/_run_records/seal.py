from inspect import *
import datetime,platform
# Do not call helper read() after freezing READS.jsonl.
reads=[json.loads(t)for t in (REC/'READS.jsonl').read_text().splitlines()]
external={}
for x in reads:
 p=pathlib.Path(x['path'])
 if p.is_relative_to(OUT):continue
 external.setdefault(str(p),set()).add(x['sha256'])
inputs=[]
for p,expected in sorted(external.items()):
 actual=hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
 assert len(expected)==1 and actual in expected,(p,expected,actual)
 inputs.append({'origin':p,'sha256':actual})
(REC/'INPUT_INVENTORY.json').write_text(json.dumps(inputs,indent=2)+'\n')
now=datetime.datetime.now(datetime.timezone.utc).isoformat()
timing={'first_tool_start':'2026-10-02T02:15:31+00:00','grant_minutes':45,'new_calculation_cutoff':'2026-10-02T02:55:31+00:00','hard_end':'2026-10-02T03:00:31+00:00','numerical_calculation_completed':'2026-10-02T02:28:19.517088+00:00','actual_completion_and_seal':now,'no_process_running':True,'delegation':'native TASK /root/i25_admission_replay child of /root; no descendants','python_executable':sys.executable,'python_version':platform.python_version(),'cwd':str(ROOT)}
(REC/'TIMING.json').write_text(json.dumps(timing,indent=2)+'\n')
files=[p for p in OUT.rglob('*')if p.is_file()and p.name not in ['SHA256SUMS','WRITE_INVENTORY.json']]
rows=[{'path':str(p.relative_to(OUT)),'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'bytes':p.stat().st_size}for p in sorted(files)]
(OUT/'WRITE_INVENTORY.json').write_text(json.dumps({'scope':str(OUT),'files':rows,'exclusions':'This inventory excludes itself and SHA256SUMS; SHA256SUMS includes this inventory and excludes itself.'},indent=2)+'\n')
files.append(OUT/'WRITE_INVENTORY.json')
manifest=''.join(hashlib.sha256(p.read_bytes()).hexdigest()+'  '+str(p.relative_to(OUT))+'\n'for p in sorted(files))
(OUT/'SHA256SUMS').write_text(manifest)
# Verify seal after actual final writes. No further packet write after this check.
for line in manifest.splitlines():
 sha,name=line.split(None,1);assert hashlib.sha256((OUT/name).read_bytes()).hexdigest()==sha
print(json.dumps({'completion':now,'payload_count':len(files),'input_origins':len(inputs),'seal_sha256':hashlib.sha256(manifest.encode()).hexdigest(),'return_sha256':hashlib.sha256((OUT/'RETURN.md').read_bytes()).hexdigest(),'replay_sha256':hashlib.sha256((REC/'ACTUAL_H_REPLAY.json').read_bytes()).hexdigest()},indent=2))
