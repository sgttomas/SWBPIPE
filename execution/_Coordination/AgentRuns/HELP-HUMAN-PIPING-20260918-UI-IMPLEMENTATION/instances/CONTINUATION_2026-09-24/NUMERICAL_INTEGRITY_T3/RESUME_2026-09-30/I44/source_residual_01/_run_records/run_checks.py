#!/usr/bin/env python3
"""Bounded focused checks; captures candidate hashes before/after every command."""
from pathlib import Path
import os,sys,subprocess,time,json,hashlib,datetime
ROOT=Path(__file__).resolve().parents[12] if False else Path.cwd()
OUT=Path(__file__).resolve().parent
MANIFEST='projects/chirality-piping/core/solver/frame_kernel/Cargo.toml'
TARGET='/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/targets/i44-source-residual/frame_kernel'
PREFIX='projects/chirality-piping/core/solver/frame_kernel/'
FILES=[PREFIX+p for p in ['src/structural/retained/adaptive.rs','src/structural/retained/directed.rs','src/structural/retained/product_certificate.rs','src/structural/retained/product_certificate/source_residual.rs','tests/retained_k4/source_residual_tests.rs','tests/retained_k4/source_residual_vectors.py','tests/retained_k4/source_residual_vectors.rs','tests/s11_site_table.rs']]
def hashes():return {p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest() for p in FILES}
base=['cargo','test','--manifest-path',MANIFEST]
commands={
 'final_debug':base+['--lib','source_residual','--','--nocapture'],
 'final_release':base+['--release','--lib','source_residual','--','--nocapture'],
 'compat_bridge':base+['--lib','source_bridge','--','--nocapture'],
 'compat_member':base+['--lib','product_certificate::tests'],
 'compat_sqrt':base+['--lib','directed::certificate'],
 's11':base+['--test','s11_site_table'],
}
for name in sys.argv[1:]:
 cmd=commands[name];before=hashes();start=time.time();utc=datetime.datetime.now(datetime.timezone.utc).isoformat()
 env=os.environ.copy();env.update(CARGO_BUILD_JOBS='4',RUST_TEST_THREADS='2',CARGO_TARGET_DIR=TARGET)
 with (OUT/(name+'.log')).open('w') as log:
  try:code=subprocess.run(cmd,env=env,stdout=log,stderr=subprocess.STDOUT,timeout=1200).returncode
  except subprocess.TimeoutExpired:code=124
 after=hashes()
 record={'argv':cmd,'cwd':str(ROOT),'utc':utc,'elapsed_seconds':time.time()-start,'timeout_seconds':1200,'environment':{k:env[k] for k in ['CARGO_BUILD_JOBS','RUST_TEST_THREADS','CARGO_TARGET_DIR']},'exit_code':code,'source_before':before,'source_after':after,'source_unchanged':before==after}
 (OUT/(name+'.command.json')).write_text(json.dumps(record,indent=2)+'\n')
 print(name,code,'unchanged',before==after,flush=True)
 print('\n'.join(line for line in (OUT/(name+'.log')).read_text().splitlines()[-7:] if len(line)<300),flush=True)
 if code or before!=after:sys.exit(1)
