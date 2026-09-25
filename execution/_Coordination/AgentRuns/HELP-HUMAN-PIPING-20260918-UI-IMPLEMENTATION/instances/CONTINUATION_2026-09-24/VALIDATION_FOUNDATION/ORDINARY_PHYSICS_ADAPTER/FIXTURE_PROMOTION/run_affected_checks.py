"""Affected maintained tests with historical AgentRuns file access refused."""
from pathlib import Path
import hashlib,json,os,subprocess,sys,time
E=Path(__file__).resolve().parent
P=next(p for p in E.parents if p.name=='chirality-piping')
paths=[P/'tests/test_qualification_physics_structure.py',P/'tests/test_qualification_physics_integration.py',P/'tests/test_first_static_selection.py',P/'tests/qualification_fixture_support.py',*sorted((P/'validation/qualification/fixtures/first_static').rglob('*'))]
paths=[p for p in paths if p.is_file()]
def snap():return [{'path':str(p.relative_to(P)),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()} for p in paths]
code=r'''
import os,sys,unittest
from pathlib import Path
attempts=[]
def guard(event,args):
 if event=='open' and args and isinstance(args[0],(str,bytes,os.PathLike)):
  path=os.fsdecode(args[0]).replace('\\','/')
  if '/execution/_Coordination/AgentRuns/' in str(Path(path).absolute()) or path.startswith('execution/_Coordination/AgentRuns/'):
   attempts.append(path)
   raise RuntimeError('Maintained test attempted historical AgentRuns file access: '+path)
sys.addaudithook(guard)
try: open('execution/_Coordination/AgentRuns/synthetic-guard-control')
except RuntimeError: pass
else: raise AssertionError('historical access guard inactive')
assert len(attempts)==1;attempts.clear()
suite=unittest.TestSuite()
for name in ('test_qualification_physics_structure.py','test_qualification_physics_integration.py','test_first_static_selection.py'):
 suite.addTests(unittest.defaultTestLoader.discover('tests',pattern=name))
result=unittest.TextTestRunner(verbosity=2).run(suite)
print('historical_agent_run_access_attempts='+str(len(attempts)))
assert not attempts
raise SystemExit(0 if result.wasSuccessful() else 1)
'''
(E/'affected_test_driver.py').write_text(code)
before=snap();start=time.monotonic()
command=[sys.executable,'-c',code]
with (E/'affected_checks.log').open('wb') as log:
 process=subprocess.run(command,cwd=P,env=os.environ|{'PYTHONDONTWRITEBYTECODE':'1'},stdout=log,stderr=subprocess.STDOUT)
after=snap()
record={'before':before,'after':after,'source_unchanged':before==after,'exit_code':process.returncode,'elapsed_seconds':time.monotonic()-start,'python':sys.executable,'driver_sha256':hashlib.sha256(code.encode()).hexdigest(),'log_sha256':hashlib.sha256((E/'affected_checks.log').read_bytes()).hexdigest(),'scope':'Only affected fixture-dependent tests, with file audit guard refusing historical AgentRuns access. No actual solver or protected-reference rerun.'}
(E/'CHECKS.json').write_text(json.dumps(record,indent=2)+'\n')
print(json.dumps({k:record[k] for k in ('exit_code','elapsed_seconds','source_unchanged','log_sha256')},indent=2))
assert process.returncode==0 and before==after
