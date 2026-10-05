#!/usr/bin/env python3
"""Run only after ROOT explicitly grants the I41 Cargo lane. One command at once."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,os,signal,subprocess,sys,time
root=Path.cwd()
raw=Path(__file__).resolve().parent
manifest=root/'projects/chirality-piping/core/solver/frame_kernel/Cargo.toml'
mode=sys.argv[1]
label=sys.argv[2] if len(sys.argv)>2 else mode
assert label.replace('_','').isalnum()
assert mode in ['debug','optimized','compatibility']
argv=['cargo','test','--manifest-path',str(manifest),'--locked','--offline','--lib']
if mode=='optimized':argv+=['--release']
argv += ['structural::retained::directed::' if mode=='compatibility' else 'structural::retained::product_certificate::']
argv += ['--','--test-threads=2']
env=os.environ.copy();env.update(CARGO_BUILD_JOBS='4',RUST_TEST_THREADS='2',CARGO_TARGET_DIR=str(root.parent/'i41-target/frame_kernel'))
# Existing parent-owned guard. This script neither starts nor changes it.
os.kill(5387,0)
paths=[manifest.parent/'src/structural/retained'/p for p in ['product_certificate.rs','directed.rs','mod.rs','directed/certificate.rs']]+[manifest.parent/'tests/retained_k4'/p for p in ['product_certificate_tests.rs','product_certificate_vectors.rs','product_certificate_vectors.py','directed_tests.rs','certificate_arithmetic_tests.rs','certificate_arithmetic_vectors.rs']]
record={'mode':mode,'argv':argv,'cwd':str(root),'environment':{k:env[k] for k in ['CARGO_BUILD_JOBS','RUST_TEST_THREADS','CARGO_TARGET_DIR']},'guard_pid':5387,'start_utc':datetime.now(timezone.utc).isoformat(),'source_hashes':{str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}}
(raw/(label+'.json')).write_text(json.dumps(record,indent=2)+'\n')
start=time.monotonic()
with (raw/(label+'.stdout')).open('w') as out,(raw/(label+'.stderr')).open('w') as err:
    proc=subprocess.Popen(argv,cwd=root,env=env,stdout=out,stderr=err,start_new_session=True)
    record['pid']=proc.pid
    (raw/(label+'.json')).write_text(json.dumps(record,indent=2)+'\n')
    try:code=proc.wait(timeout=1200)
    except subprocess.TimeoutExpired:
        os.killpg(proc.pid,signal.SIGTERM)
        try:proc.wait(timeout=10)
        except subprocess.TimeoutExpired:os.killpg(proc.pid,signal.SIGKILL);proc.wait()
        record['timeout']=True;code=proc.returncode
record.update(exit_code=code,elapsed_seconds=time.monotonic()-start,end_utc=datetime.now(timezone.utc).isoformat(),process_exited=proc.poll() is not None)
record['source_unchanged']=all(hashlib.sha256((root/p).read_bytes()).hexdigest()==h for p,h in record['source_hashes'].items())
(raw/(label+'.json')).write_text(json.dumps(record,indent=2)+'\n')
print(json.dumps({k:v for k,v in record.items() if k!='source_hashes'},indent=2),flush=True)
print((raw/(label+'.stdout')).read_text()[-10000:],flush=True)
print((raw/(label+'.stderr')).read_text()[-6000:],flush=True)
sys.exit(code)
