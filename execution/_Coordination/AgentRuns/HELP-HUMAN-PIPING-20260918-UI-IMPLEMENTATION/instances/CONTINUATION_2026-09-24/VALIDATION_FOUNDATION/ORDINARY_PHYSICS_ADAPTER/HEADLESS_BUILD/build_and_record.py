"""One authorized fresh offline headless build; no solver execution."""
from pathlib import Path
import hashlib,json,os,platform,shutil,subprocess,time
R=Path('/private/tmp/piping-engine-integration-20260925')
P=R/'projects/chirality-piping'
E=Path(__file__).resolve().parent
T=Path('/private/tmp/piping-validation-headless-20260925')
COMMIT='8b982aa7ce64afe37e6067d1038d92608f4aaf3f'
def sha(data): return hashlib.sha256(data).hexdigest()
def write(name,value): (E/name).write_text(json.dumps(value,indent=2,sort_keys=True)+'\n')
def command(args): return subprocess.check_output(args,cwd=R,text=True).strip()
assert command(['git','rev-parse','HEAD'])==COMMIT
assert not command(['git','diff','--name-only'])
assert not command(['git','diff','--cached','--name-only'])
env=os.environ.copy()
removed=['RUSTFLAGS','CARGO_ENCODED_RUSTFLAGS','RUSTC_WRAPPER','RUSTC_WORKSPACE_WRAPPER','RUSTDOCFLAGS','CARGO_TARGET_DIR']
for key in removed: env.pop(key,None)
env.update(CARGO_TARGET_DIR=str(T),CARGO_BUILD_JOBS='2',CARGO_NET_OFFLINE='true')
cargo=str(Path(shutil.which('cargo')).absolute())
manifest=P/'core/runner/headless/Cargo.toml'
meta_command=[cargo,'metadata','--manifest-path',str(manifest),'--format-version','1','--offline','--locked']
with (E/'metadata.stderr.log').open('wb') as err:
 result=subprocess.run(meta_command,cwd=R,env=env,stdout=subprocess.PIPE,stderr=err,check=True)
(E/'metadata.json').write_bytes(result.stdout)
meta=json.loads(result.stdout)
tracked=command(['git','ls-files']).splitlines()
inputs=set()
for pkg in meta['packages']:
 root=Path(pkg['manifest_path']).parent.resolve()
 if root.is_relative_to(R):
  prefix=str(root.relative_to(R))+'/'
  inputs.update(R/f for f in tracked if f.startswith(prefix))
 else:
  inputs.update(p for p in root.rglob('*') if p.is_file() and '.git' not in p.parts and 'target' not in p.parts)
# Record static include sources outside package directories (all local included files
# are either beneath a package root or named by a literal Rust include path).
import re
for path in tuple(inputs):
 if path.suffix=='.rs':
  for rel in re.findall(r'include_(?:str|bytes)!\s*\(\s*"([^"]+)"',path.read_text()):
   included=(path.parent/rel).resolve()
   if included.is_file(): inputs.add(included)
configs=[]
for directory in [R,P,manifest.parent,Path.home()]:
 for suffix in ('.cargo/config','.cargo/config.toml','rust-toolchain','rust-toolchain.toml'):
  path=directory/suffix
  configs.append({'path':str(path),'exists':path.is_file()})
  if path.is_file(): inputs.add(path)
inputs.add(manifest.with_name('Cargo.lock'))
def snapshot():
 return [{'path':str(p),'sha256':sha(p.read_bytes()),'byte_length':p.stat().st_size} for p in sorted(inputs)]
before=snapshot();write('SOURCE_BEFORE.json',before)
build=[cargo,'build','--manifest-path',str(manifest),'--bin','openpipestress-runner','--offline','--locked','--jobs','2','--target-dir',str(T),'--message-format','json-render-diagnostics']
record={'format':'openpipestress.validation_headless_build/1','candidate_commit':COMMIT,'source_root':str(R),'target_dir':str(T),'metadata_command':meta_command,'build_command':build,'environment_overrides':{'CARGO_TARGET_DIR':str(T),'CARGO_BUILD_JOBS':'2','CARGO_NET_OFFLINE':'true'},'removed_optional_build_environment_keys':removed,'cargo':command([cargo,'--version']),'rustc':command(['rustc','-vV']),'python':platform.python_version(),'platform':platform.platform(),'configuration_probes':configs,'metadata_sha256':sha(result.stdout),'source_before_sha256':sha((E/'SOURCE_BEFORE.json').read_bytes()),'dependency_package_count':len(meta['packages']),'bound_input_count':len(before),'solver_execution_performed':False,'source_status_before':command(['git','status','--porcelain=v1'])}
write('BUILD_STARTED.json',record)
start=time.monotonic()
with (E/'cargo.stdout.jsonl').open('wb') as out,(E/'cargo.stderr.log').open('wb') as err:
 process=subprocess.run(build,cwd=R,env=env,stdout=out,stderr=err)
after=snapshot();write('SOURCE_AFTER.json',after)
record.update(exit_code=process.returncode,elapsed_seconds=time.monotonic()-start,source_unchanged=before==after,source_after_sha256=sha((E/'SOURCE_AFTER.json').read_bytes()),source_status_after=command(['git','status','--porcelain=v1']),stdout_sha256=sha((E/'cargo.stdout.jsonl').read_bytes()),stderr_sha256=sha((E/'cargo.stderr.log').read_bytes()))
exe=T/'debug/openpipestress-runner'
if process.returncode==0:
 record['executable']={'path':str(exe),'sha256':sha(exe.read_bytes()),'byte_length':exe.stat().st_size}
 record['dep_info']={'path':str(exe.with_suffix('.d')),'sha256':sha(exe.with_suffix('.d').read_bytes())}
 (E/'runner.dep-info.txt').write_bytes(exe.with_suffix('.d').read_bytes())
write('BUILD_RESULT.json',record)
print(json.dumps({k:record[k] for k in ('exit_code','elapsed_seconds','source_unchanged','dependency_package_count','bound_input_count','solver_execution_performed')},indent=2),flush=True)
if 'executable' in record: print(json.dumps(record['executable']),flush=True)
assert process.returncode==0 and before==after
