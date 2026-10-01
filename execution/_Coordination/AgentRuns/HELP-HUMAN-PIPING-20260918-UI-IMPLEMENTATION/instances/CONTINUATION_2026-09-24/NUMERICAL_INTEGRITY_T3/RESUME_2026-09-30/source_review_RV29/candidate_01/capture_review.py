from pathlib import Path
import subprocess,os,json,hashlib,sys,datetime
coord=Path(sys.argv[1]);out=Path(sys.argv[2]);scratch=Path.cwd();env=dict(os.environ,GIT_OPTIONAL_LOCKS='0');pin='3cf296e36645d97e4c657c8ad1a6322bc4163f16';base='d01ad98a754698631f927709d08284c272de85e8';commands=[];records=[]
def git(args):
 p=subprocess.run(['git',*args],cwd=coord,env=env,capture_output=True)
 commands.append({'argv':['git',*args],'cwd':'<COORD>','env':{'GIT_OPTIONAL_LOCKS':'0'},'exit':p.returncode,'stdout_sha256':hashlib.sha256(p.stdout).hexdigest(),'stderr':p.stderr.decode()})
 if p.returncode not in [0,1]:raise RuntimeError(p.stderr.decode())
 return p.stdout
paths=['projects/chirality-piping/core/solver/frame_kernel/src/structural.rs','projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/adaptive.rs','projects/chirality-piping/core/solver/frame_kernel/tests/retained_k4/publication_tests.rs','projects/chirality-piping/core/solver/performance_harness/tests/k6b_export.rs']
diff=git(['diff',base,pin,'--',*paths]);(out/'CORE.diff').write_bytes(diff)
changed=git(['diff','--name-only',base,pin,'--','projects/chirality-piping/core']).decode().splitlines();assert changed==paths
for path in paths+['projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/'+n for n in ['wide.rs','wide_sum.rs','wide/multi.rs','verify.rs','recover.rs','combine.rs']]+['projects/chirality-piping/validation/benchmarks/numerical_robustness/src/records.rs']:
 b=git(['show',pin+':'+path]);records.append({'origin':pin+':'+path,'sha256':hashlib.sha256(b).hexdigest(),'bytes':len(b)})
 if path in paths:(scratch/('frozen_'+Path(path).name)).write_bytes(b)
 if path.endswith(('wide.rs','wide_sum.rs','wide/multi.rs')):assert b==git(['show',base+':'+path])
reach=git(['grep','-n','-E','retained_api|retained::|M03-INTEGRITY-MP-v[12]',pin,'--','projects/chirality-piping/core','projects/chirality-piping/validation/benchmarks/numerical_robustness/src']);(out/'REACH.txt').write_bytes(reach)
(out/'REVIEW_BASIS.json').write_text(json.dumps({'candidate':pin,'full_maintained_change_set':changed,'source_records':records,'prior_instruction_skill_basis':'../preparation_01/BASIS.json','checkpoint_basis':'BASIS.json','time_utc':datetime.datetime.now(datetime.timezone.utc).isoformat()},indent=2)+'\n')
(out/'REVIEW_COMMANDS.json').write_text(json.dumps(commands,indent=2)+'\n')
# Do not edit the ledger freeze or accept a changed input after observation.
for line in (out/'LEDGER_SHA256SUMS').read_text().splitlines():
 d,n=line.split(maxsplit=1);assert hashlib.sha256((out/n).read_bytes()).hexdigest()==d
r=subprocess.run([sys.executable,'-B',str(out/'freeze_ledger.py'),str(scratch/'ledger_rerun'),str(out.parent/'preparation_01/LEDGER.json')],capture_output=True,env=env)
(scratch/'ledger_rerun.stdout.json').write_bytes(r.stdout);(scratch/'ledger_rerun.stderr.txt').write_bytes(r.stderr)
assert r.returncode==0 and r.stderr==b''
assert (scratch/'ledger_rerun/LEDGER.json').read_bytes()==(out/'LEDGER.json').read_bytes()
print(json.dumps({'maintained_files':len(changed),'ledger_seal_intact':True,'ledger_rerun_identical':True,'unchanged_pricing_primitive_files':3,'rust_execution':False},indent=2))
