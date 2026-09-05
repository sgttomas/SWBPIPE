from pathlib import Path
import os,sys,json,subprocess,datetime,hashlib
r=Path(__file__).resolve().parent
root=Path.cwd()
p=root/'projects/chirality-piping'
i=json.loads((r/'isolation.json').read_text())
env=os.environ.copy();env.update({k:i[k] for k in ['CARGO_HOME','CARGO_TARGET_DIR','RUSTUP_TOOLCHAIN']});env['CARGO_HTTP_TIMEOUT']='30';env['CARGO_NET_RETRY']='0'
op='core/model_operations/operation_applier/Cargo.toml';sw='core/loads/self_weight_wasm/Cargo.toml'
phase=sys.argv[1]
cmds={'fetch-operation':['cargo','fetch','--locked','--manifest-path',op], 'fetch-selfweight':['cargo','fetch','--locked','--manifest-path',sw], 'probe-selfweight':['cargo','build','--locked','--offline','--manifest-path',sw,'--target','wasm32-unknown-unknown','--features','wasm','--release'], 'build-operation':['cargo','build','--locked','--offline','--manifest-path',op,'--target','wasm32-unknown-unknown','--features','wasm','--release'], 'build-selfweight':['cargo','build','--locked','--offline','--manifest-path',sw,'--target','wasm32-unknown-unknown','--features','wasm','--release'], 'actual-builder':['npm','run','build:wasm:desktop']}
cmd=cmds[phase]
if not phase.startswith('fetch-'): env['CARGO_NET_OFFLINE']='true'
log=r/(phase+'-'+sys.argv[2]+'.log')
start=datetime.datetime.now(datetime.timezone.utc).isoformat()
with log.open('wb') as f:
 f.write((json.dumps({'command':cmd,'cwd':'{WORKING_ROOT}','isolation':i,'offline':env.get('CARGO_NET_OFFLINE'),'start':start})+'\n').encode());f.flush()
 result=subprocess.run(cmd,cwd=p,env=env,stdout=f,stderr=subprocess.STDOUT)
entry={'phase':phase,'attempt':sys.argv[2],'command':cmd,'exit_code':result.returncode,'log':log.name,'sha256':hashlib.sha256(log.read_bytes()).hexdigest(),'start':start,'end':datetime.datetime.now(datetime.timezone.utc).isoformat()}
with (r/'commands.jsonl').open('a') as f: f.write(json.dumps(entry)+'\n')
print(json.dumps(entry));print(log.read_text()[-5000:]);sys.exit(result.returncode)
