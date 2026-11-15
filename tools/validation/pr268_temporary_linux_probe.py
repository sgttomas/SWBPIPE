import ast
import importlib.util
import inspect
import json
import math
import os
from pathlib import Path
import platform
import subprocess
import sys
import tempfile
from fractions import Fraction

ROOT = Path.cwd()
OUT = Path(os.environ.get('DIAGNOSTIC_OUTPUT', '/tmp/pr268-linux-evidence'))
OUT.mkdir(parents=True, exist_ok=True)
def save(name, value):
    (OUT / name).write_text(json.dumps(value, indent=2, default=str) + '\n')

save('platform.json', dict(platform=platform.platform(), machine=platform.machine(), python=sys.version,
    libc=platform.libc_ver(), head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),
    posix_spawn=getattr(subprocess,'_USE_POSIX_SPAWN',None)))
(OUT / 'subprocess.py').write_text(inspect.getsource(subprocess))
packages = {
    'f1_modal/continuum': 'generate.py', 'f1_modal/mass_discrete': 'f1_mass_discrete.py',
    'f2_spectrum/closed_form': 'generate.py', 'f2_spectrum/discrete': 'f2_spectrum_discrete.py',
    'f2_spectrum/benchmark_pack': 'generate.py'}
checks = {}
for package, script in packages.items():
    done = subprocess.run([sys.executable, '-I', '-B', 'generators/'+script, '--check'],
        cwd=ROOT/'validation/references'/package, capture_output=True, text=True, timeout=600)
    checks[package] = dict(returncode=done.returncode, stdout=done.stdout, stderr=done.stderr)
    save('checks.json', checks)
    print(package, done.returncode, done.stdout, flush=True)

base = ROOT/'validation/references/f1_modal/continuum'
sys.path.insert(0, str(base/'generators'))
import generate
files, crosses = generate.build()
for name in ['inertia_witnesses.json','SHA256SUMS']:
    (OUT / ('regenerated-'+name)).write_bytes(files[name])
    (OUT / ('committed-'+name)).write_bytes((base/name).read_bytes())
def diffs(a,b,path='$'):
    if isinstance(a,dict) and isinstance(b,dict):
        for k in a.keys() | b.keys():
            yield from diffs(a.get(k),b.get(k),path+'.'+k)
    elif isinstance(a,list) and isinstance(b,list) and len(a)==len(b):
        for i,(x,y) in enumerate(zip(a,b)): yield from diffs(x,y,f'{path}[{i}]')
    elif a != b:
        row = dict(path=path, committed=a, regenerated=b)
        try:
            x,y=float(a),float(b)
            row.update(absolute_difference=y-x, ulps=(y-x)/math.ulp(x), committed_hex=x.hex(), regenerated_hex=y.hex())
        except (ValueError,TypeError,ZeroDivisionError): pass
        yield row
original = json.loads((base/'inertia_witnesses.json').read_bytes())
save('differences.json',list(diffs(original,json.loads(files['inertia_witnesses.json']))))

# Instrument a separately compiled copy: retain every original pow result,
# report floating squares differing from multiplication/exact rounding.
import inertia_witnesses
power_calls=[]
def observed_power(x,y,line):
    value=x**y
    if isinstance(x,float) and y==2:
        multiply=x*x
        exact=float(Fraction.from_float(x)**2)
        power_calls.append(dict(line=line,x=x.hex(),power=value.hex(),multiply=multiply.hex(),exact_rounded=exact.hex()))
    return value
class Powers(ast.NodeTransformer):
    def visit_BinOp(self,node):
        self.generic_visit(node)
        if isinstance(node.op,ast.Pow):
            return ast.copy_location(ast.Call(func=ast.Name(id='observed_power',ctx=ast.Load()),args=[node.left,node.right,ast.Constant(node.lineno)],keywords=[]),node)
        return node
source=(base/'generators/inertia_witnesses.py').read_text()
tree=ast.fix_missing_locations(Powers().visit(ast.parse(source)))
ns=dict(observed_power=observed_power,__name__='instrumented_inertia',__file__=str(base/'generators/inertia_witnesses.py'))
exec(compile(tree,ns['__file__'],'exec'),ns)
body=ns['build']([])
save('pow-operations.json',power_calls)
save('pow-instrumentation-unchanged.json',body==inertia_witnesses.build([]))

# Execute the original complete probe and retain all judged records. Then
# isolate the four starts to identify the event path of each invocation.
sys.path.insert(0,str(ROOT))
test_source=(ROOT/'tests/test_dynamics_references.py').read_text()
tree=ast.parse(test_source)
probe=next(ast.literal_eval(n.value) for n in tree.body if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='OBSERVATION_PROBE' for t in n.targets))
from tools.validation.reference_observation import run_observed
records={}
variants={'original':probe, 'bare_program':'import subprocess\nsubprocess.run(["true"])\n',
 'bare_python':'import subprocess,sys\nsubprocess.run([sys.executable,"-c","pass"])\n',
 'isolated_python':'import subprocess,sys\nsubprocess.run([sys.executable,"-I","-c","from multiprocessing.spawn import spawn_main"])\n',
 'cleared_python':'import subprocess,sys\nsubprocess.run([sys.executable,"-c","from multiprocessing.spawn import spawn_main"],env={})\n'}
for name, source in variants.items():
    with tempfile.TemporaryDirectory() as tmp:
        tmp=Path(tmp)
        script=tmp/'probe.py'; script.write_text(source)
        work=tmp/'work'; work.mkdir()
        result=run_observed([sys.executable,str(script),str(ROOT)],tmp,work,[ROOT/name for name in ('core','apps','api','tests','tools','fixtures','schemas')],60)
        records[name]=result._asdict()
        save('observer-records.json',records)
print('Evidence:',OUT,flush=True)
