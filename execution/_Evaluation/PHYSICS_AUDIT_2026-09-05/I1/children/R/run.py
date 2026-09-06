from pathlib import Path
import json,subprocess,hashlib,time
p=Path(__file__).resolve().parent;b=Path('/tmp/piping-audit-I1-R-driver/target/debug/piping-audit-i1-r');records=[]
for f in sorted((p/'fixtures').glob('*.json')):
 for mode in ['sparse','dense']:
  t=time.time();r=subprocess.run([str(b),str(f),mode],capture_output=True,text=True)
  out=p/'results'/f'{f.stem}_{mode}.json'
  if r.returncode==0:out.write_text(r.stdout)
  records.append({'fixture':f.name,'mode':mode,'exit':r.returncode,'stderr':r.stderr,'seconds':time.time()-t,'stdout_sha256':hashlib.sha256(r.stdout.encode()).hexdigest()})
(p/'_run_records/EXECUTIONS.json').write_text(json.dumps(records,indent=2)+'\n')
print(json.dumps(records,indent=2))
