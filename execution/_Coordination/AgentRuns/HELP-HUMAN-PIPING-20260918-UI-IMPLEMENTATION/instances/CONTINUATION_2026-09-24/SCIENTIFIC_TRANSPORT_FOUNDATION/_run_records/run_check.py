#!/usr/bin/env python3
"""Bounded local check runner; preserves raw output/exit/candidate source hashes."""
from pathlib import Path
import datetime,hashlib,json,os,subprocess,sys,time
root=Path(subprocess.check_output(['git','rev-parse','--show-toplevel'],text=True).strip())
project=root/'projects/chirality-piping'
output=Path(__file__).resolve().parent
label=sys.argv[1]; command=sys.argv[2:]
assert label.replace('_','').replace('-','').isalnum()
paths=list((project/'core/serialization/canonical_json').rglob('*.rs'))
paths=[p for p in paths if 'target' not in p.parts]
paths += [project/p for p in ['core/serialization/canonical_json/Cargo.toml','core/serialization/canonical_json/Cargo.lock','core/serialization/canonical_json/adapter.py','core/serialization/canonical_json/README.md','tools/serialization/build_checked_json.py','tests/test_binary64_canonical_json_adapter.py']]
env=os.environ.copy();env.update({'PYTHONDONTWRITEBYTECODE':'1','CARGO_BUILD_JOBS':'2','CARGO_NET_OFFLINE':'true','PYTHONPATH':str(project)})
env.pop('CARGO_TARGET_DIR',None)
start=time.time()
with (output/(label+'.log')).open('w') as log:
    result=subprocess.run(command,cwd=project,env=env,stdout=log,stderr=subprocess.STDOUT)
record={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'command':command,'cwd':'REPO_ROOT/projects/chirality-piping','environment':{k:env.get(k) for k in ['PYTHONDONTWRITEBYTECODE','CARGO_BUILD_JOBS','CARGO_NET_OFFLINE','OPENPIPESTRESS_CHECKED_JSON_BIN','OPENPIPESTRESS_BINARY64_JSON_BIN']},'CARGO_TARGET_DIR':'unset','exit_code':result.returncode,'seconds':time.time()-start,'files':[{'path':str(p.relative_to(root)),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()} for p in sorted(paths)],'log_sha256':hashlib.sha256((output/(label+'.log')).read_bytes()).hexdigest()}
(output/(label+'.json')).write_text(json.dumps(record,indent=2)+'\n')
print(json.dumps({'label':label,'exit_code':result.returncode,'seconds':record['seconds']}))
print('\n'.join((output/(label+'.log')).read_text().splitlines()[-28:]))
sys.exit(result.returncode)
