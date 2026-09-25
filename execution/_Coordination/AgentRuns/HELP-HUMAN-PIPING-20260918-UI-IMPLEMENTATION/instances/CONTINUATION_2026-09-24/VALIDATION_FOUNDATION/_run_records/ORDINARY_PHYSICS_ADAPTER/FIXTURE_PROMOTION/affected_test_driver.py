
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
