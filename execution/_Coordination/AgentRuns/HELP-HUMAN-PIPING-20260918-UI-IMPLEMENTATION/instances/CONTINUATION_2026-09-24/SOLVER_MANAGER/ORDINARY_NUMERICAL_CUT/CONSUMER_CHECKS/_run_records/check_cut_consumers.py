from pathlib import Path
import hashlib,json,os,subprocess,time
R=Path('/private/tmp/piping-numerical-corrections-20260924');P=R/'projects/chirality-piping';D=P/'execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/SOLVER_MANAGER/ORDINARY_NUMERICAL_CUT/CONSUMER_CHECKS/_run_records';D.mkdir(parents=True,exist_ok=True)
def hashes():
 files=set()
 for base in [P/'core',P/'apps/desktop/src',P/'apps/desktop/scripts',P/'schemas',P/'fixtures']:
  for p in base.rglob('*'):
   if not p.is_file() or any(x in p.parts for x in ['target','node_modules','__pycache__']):continue
   if p.suffix in ['.rs','.ts','.tsx','.mjs','.json','.yaml','.yml'] or p.name in ['Cargo.toml','Cargo.lock']:files.add(p)
 for p in [P/'package.json',P/'package-lock.json',P/'apps/desktop/package.json',P/'apps/desktop/vite.config.ts',P/'apps/desktop/tsconfig.json']:
  if p.exists():files.add(p)
 return {str(p.relative_to(R)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(files)}
before=hashes();(D/'INPUTS.json').write_text(json.dumps({'hashes':before,'started_unix':time.time(),'target_policy':'CARGO_TARGET_DIR unset; maintained build resolves each independent manifest default target'},indent=2)+'\n')
env=os.environ.copy();env.pop('CARGO_TARGET_DIR',None);env['CARGO_BUILD_JOBS']='2';env['CARGO_NET_OFFLINE']='true'
commands=[('wasm',['npm','run','build:wasm','--workspace','apps/desktop']),('vitest',['npm','test','--workspace','apps/desktop','--','src/services/previewService.test.ts','src/App.test.tsx','--maxWorkers=2']),('tsc',['./node_modules/.bin/tsc','-b','apps/desktop/tsconfig.json'])]
rows=[]
for stage,cmd in commands:
 with (D/(stage+'.log')).open('w') as log:proc=subprocess.run(cmd,cwd=P,env=env,stdout=log,stderr=subprocess.STDOUT)
 current=hashes();row={'stage':stage,'command':cmd,'cwd':str(P),'exit_code':proc.returncode,'source_and_locks_unchanged':current==before,'log_sha256':hashlib.sha256((D/(stage+'.log')).read_bytes()).hexdigest(),'ended_unix':time.time()};rows.append(row)
 if current!=before:row['changed_paths']=[p for p in set(before)|set(current) if before.get(p)!=current.get(p)]
 (D/'STAGES.json').write_text(json.dumps(rows,indent=2)+'\n');print(json.dumps(row),flush=True)
 print((D/(stage+'.log')).read_text()[-2500:],flush=True)
 if proc.returncode or current!=before:raise SystemExit(proc.returncode or 2)
assets={str(p.relative_to(R)):hashlib.sha256(p.read_bytes()).hexdigest() for d in ['wasm-engine','self-weight-engine'] for p in (P/'apps/desktop/public'/d).rglob('*') if p.is_file()}
(D/'ASSETS.json').write_text(json.dumps(assets,indent=2)+'\n')
