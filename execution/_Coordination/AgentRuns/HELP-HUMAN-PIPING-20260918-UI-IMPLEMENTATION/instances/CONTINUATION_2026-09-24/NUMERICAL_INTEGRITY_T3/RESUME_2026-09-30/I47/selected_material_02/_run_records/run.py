#!/usr/bin/env python3
from pathlib import Path
import subprocess,sys,os,json,hashlib,datetime,signal,time
C=Path(__file__).resolve().parents[13]
assert (C/'projects/chirality-piping/core/product_physics/Cargo.toml').exists(),str(C)
O=Path(__file__).resolve().parent
name=sys.argv[1]
argv=sys.argv[2:]
crate = 'frame_kernel' if any('/frame_kernel/Cargo.toml' in a for a in argv) else 'product_physics'
env=os.environ.copy();env.update(CARGO_BUILD_JOBS='4',RUST_TEST_THREADS='2',CARGO_TARGET_DIR=str(C.parent/'targets/i47-selected-material'/crate),GIT_OPTIONAL_LOCKS='0')
def hashes():return {str(p.relative_to(C)):hashlib.sha256(p.read_bytes()).hexdigest() for p in (C/'projects/chirality-piping/core').rglob('*') if p.is_file() and 'target' not in p.parts and '.git' not in p.parts}
def now():return datetime.datetime.now(datetime.timezone.utc).isoformat()
inputs={str(Path(a).resolve()):hashlib.sha256(Path(a).read_bytes()).hexdigest() for a in argv if Path(a).is_file() and Path(a).suffix in ['.py','.log']}
record=dict(argv=argv,cwd=str(C),start=now(),before=hashes(),script_inputs=inputs,driver_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),target=env['CARGO_TARGET_DIR'],jobs=4,threads=2,wall_seconds=1200)
with (O/(name+'.log')).open('w') as f:
 p=subprocess.Popen(argv,cwd=C,env=env,stdout=f,stderr=subprocess.STDOUT,start_new_session=True)
 try:record['exit']=p.wait(timeout=1200)
 except subprocess.TimeoutExpired:
  os.killpg(p.pid,signal.SIGTERM);record['exit']='timeout';p.wait(timeout=15)
record.update(after=hashes(),end=now())
(O/(name+'.json')).write_text(json.dumps(record,indent=2)+'\n')
print(name,record['exit'],record['start'],record['end'])
print((O/(name+'.log')).read_text()[-2500:])
sys.exit(0 if record['exit']==0 else 1)
