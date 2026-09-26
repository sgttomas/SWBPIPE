from pathlib import Path
import json,subprocess,os
p=Path('/private/tmp/piping-joined-qualification-20260925');d=json.loads((p/'numerical-e650/numerical.json').read_text());out=p/'lock-check';out.mkdir(exist_ok=True);env=os.environ.copy();env['CARGO_NET_OFFLINE']='true';env['CARGO_BUILD_JOBS']='2'
results=[]
for i,c in enumerate(x for x in d['planned_commands'] if x[:2]==['cargo','fetch']):
 cmd=c+['--offline'];r=subprocess.run(cmd,cwd=Path.cwd()/'projects/chirality-piping',env=env,capture_output=True,text=True);(out/f'{i:02}.log').write_text(r.stdout+r.stderr);results.append({'command':cmd,'exit_code':r.returncode,'log':f'{i:02}.log'});
 if r.returncode: print('FAILED',c[c.index('--manifest-path')+1],(r.stderr+r.stdout).splitlines()[:2],flush=True)
(out/'RESULTS.json').write_text(json.dumps(results,indent=2)+'\n');print('locked checks',len(results),'failed',sum(x['exit_code']!=0 for x in results),flush=True)
