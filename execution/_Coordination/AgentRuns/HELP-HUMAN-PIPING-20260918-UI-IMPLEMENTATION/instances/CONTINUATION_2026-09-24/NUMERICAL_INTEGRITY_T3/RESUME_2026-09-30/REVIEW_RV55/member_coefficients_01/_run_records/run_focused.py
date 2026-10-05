#!/usr/bin/env python3
"""RV55 focused existing tests; execute only after recorded ROOT lane grant."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,os,signal,subprocess,sys,time
raw=Path(__file__).resolve().parent
source=Path(sys.argv[1]); mode=sys.argv[2]
assert mode in ['debug','optimized','compatibility']
assert (raw/'LANE_GRANTED.json').exists(), 'ROOT handoff must be recorded first'
manifest=source/'projects/chirality-piping/core/solver/frame_kernel/Cargo.toml'
argv=['cargo','test','--manifest-path',str(manifest),'--locked','--offline','--lib']
if mode=='optimized':argv+=['--release']
argv+=['structural::retained::directed::' if mode=='compatibility' else 'structural::retained::product_certificate::','--','--test-threads=2']
env=os.environ.copy();env.update(CARGO_BUILD_JOBS='4',RUST_TEST_THREADS='2',CARGO_TARGET_DIR=str(source.parent/'rv55-target/frame_kernel'),GIT_OPTIONAL_LOCKS='0')
os.kill(5387,0)
paths=[manifest.parent/'src/structural/retained'/p for p in ['product_certificate.rs','directed.rs','mod.rs','directed/certificate.rs']]+[manifest.parent/'tests/retained_k4'/p for p in ['product_certificate_tests.rs','product_certificate_vectors.rs','product_certificate_vectors.py','directed_tests.rs','certificate_arithmetic_tests.rs','certificate_arithmetic_vectors.rs']]
rec={'argv':argv,'cwd':str(source),'env':{k:env[k] for k in ['CARGO_BUILD_JOBS','RUST_TEST_THREADS','CARGO_TARGET_DIR','GIT_OPTIONAL_LOCKS']},'start_utc':datetime.now(timezone.utc).isoformat(),'wall_limit_seconds':1200,'guard_pid':5387,'source_hashes':{str(p.relative_to(source)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}}
t=time.monotonic()
with (raw/(mode+'.stdout')).open('w') as out,(raw/(mode+'.stderr')).open('w') as err:
 proc=subprocess.Popen(argv,cwd=source,env=env,stdout=out,stderr=err,start_new_session=True)
 rec['pid']=proc.pid;(raw/(mode+'.json')).write_text(json.dumps(rec,indent=2)+'\n')
 try:code=proc.wait(timeout=1200)
 except subprocess.TimeoutExpired:
  os.killpg(proc.pid,signal.SIGTERM)
  try:proc.wait(timeout=10)
  except subprocess.TimeoutExpired:os.killpg(proc.pid,signal.SIGKILL);proc.wait()
  code=proc.returncode;rec['timeout']=True
rec.update(exit_code=code,elapsed_seconds=time.monotonic()-t,end_utc=datetime.now(timezone.utc).isoformat(),process_exited=proc.poll() is not None,source_unchanged=all(hashlib.sha256((source/p).read_bytes()).hexdigest()==h for p,h in rec['source_hashes'].items()))
(raw/(mode+'.json')).write_text(json.dumps(rec,indent=2)+'\n')
print(json.dumps({k:v for k,v in rec.items() if k!='source_hashes'},indent=2));print((raw/(mode+'.stdout')).read_text());print((raw/(mode+'.stderr')).read_text());sys.exit(code)
