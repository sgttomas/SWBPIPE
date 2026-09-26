from pathlib import Path
import json,subprocess,os,time
out=Path('/private/tmp/piping-joined-qualification-20260925/lock-check'); env=os.environ.copy();env['CARGO_BUILD_JOBS']='2';env['CARGO_NET_OFFLINE']='true';env.pop('CARGO_TARGET_DIR',None);rs=[]
for x in json.loads((out/'REPAIR.json').read_text()):
 m=str(Path(x['path']).relative_to('projects/chirality-piping').with_name('Cargo.toml'));cmd=['cargo','test','--offline','--locked','--manifest-path',m];log=out/(Path(m).parent.name+'-tests.log');t=time.monotonic()
 with log.open('wb') as f:r=subprocess.run(cmd,cwd='projects/chirality-piping',env=env,stdout=f,stderr=subprocess.STDOUT)
 rs.append({'command':cmd,'exit_code':r.returncode,'seconds':time.monotonic()-t,'log':log.name});print(m,r.returncode,flush=True)
(out/'TEST_RESULTS.json').write_text(json.dumps(rs,indent=2)+'\n');raise SystemExit(any(x['exit_code'] for x in rs))
