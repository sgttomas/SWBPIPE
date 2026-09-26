"""Bounded ordinary-adapter checks; no solver/build/native/pytest invocation."""
from pathlib import Path
import hashlib,json,os,subprocess,sys,time
E=Path(__file__).resolve().parent
P=next(p for p in E.parents if p.name=='chirality-piping')
paths=[*sorted((P/'tools/validation').glob('qualification*.py')),P/'tools/validation/build_first_static_selection.py',*sorted((P/'tests').glob('test_qualification*.py')),P/'tests/test_first_static_selection.py',P/'validation/qualification/GATE_USAGE.md',P/'validation/qualification/fixtures/ordinary_physics_1_semantics.json']
def sha(data):return hashlib.sha256(data).hexdigest()
def snap():return [{'path':str(p.relative_to(P)),'sha256':sha(p.read_bytes())} for p in paths]
before=snap(); records=[]
for label,pattern in [('qualification','test_qualification*.py'),('derivation','test_first_static_selection.py')]:
 cmd=[sys.executable,'-m','unittest','discover','-s','tests','-p',pattern,'-v']
 log=E/'_run_records'/f'final_{label}_02.log'
 start=time.monotonic()
 with log.open('wb') as out:
  process=subprocess.run(cmd,cwd=P,env=os.environ|{'PYTHONDONTWRITEBYTECODE':'1'},stdout=out,stderr=subprocess.STDOUT)
 records.append({'command':cmd,'exit_code':process.returncode,'elapsed_seconds':time.monotonic()-start,'log':str(log.relative_to(E)),'log_sha256':sha(log.read_bytes())})
after=snap()
(E/'FOCUSED_CHECKS_02.json').write_text(json.dumps({'before':before,'after':after,'source_unchanged':before==after,'checks':records,'claim':'Synthetic harness/derivation evidence only. No actual solver, new reference execution, build, native or release qualification.'},indent=2)+'\n')
print(json.dumps({'checks':records,'source_unchanged':before==after},indent=2),flush=True)
assert before==after and all(x['exit_code']==0 for x in records)
