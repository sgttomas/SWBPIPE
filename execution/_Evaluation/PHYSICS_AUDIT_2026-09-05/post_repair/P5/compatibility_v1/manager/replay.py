from pathlib import Path
import json,hashlib,subprocess
p=Path(__file__).resolve().parent
w=next(x for x in p.parents if x.name=='chirality-piping')
b=json.loads((p/'NUMERIC_BEFORE_BINDING.json').read_text())
(p/'results').mkdir(exist_ok=True)
def numeric(x,path=''):
 if isinstance(x,dict): return {k:v for a,b in x.items() for k,v in numeric(b,path+'/'+a).items()}
 if isinstance(x,list): return {k:v for a,b in enumerate(x) for k,v in numeric(b,path+'/'+str(a)).items()}
 return {path:x} if type(x) in (int,float) else {}
r=[]
for i,e in enumerate(b['executions']):
 inp=w/e['input']; prior=w/e['prior_output']
 assert hashlib.sha256(inp.read_bytes()).hexdigest()==e['input_sha256']
 assert hashlib.sha256(prior.read_bytes()).hexdigest()==e['prior_output_sha256']
 run=subprocess.run([str(p/'target/debug/p5-compat-numeric-replay'),str(inp),e['mode']],capture_output=True)
 assert run.returncode==0,run.stderr
 out=p/'results'/prior.name; out.write_bytes(run.stdout)
 old=json.loads(prior.read_text());new=json.loads(run.stdout)
 assert numeric(old)==numeric(new),(e['input'],e['mode'])
 def strip_meta(x):
  if isinstance(x,dict):return {k:strip_meta(v) for k,v in x.items() if k!='metadata'}
  if isinstance(x,list):return [strip_meta(v) for v in x]
  return x
 assert strip_meta(old)==strip_meta(new),(e['input'],'nonmetadata')
 r.append({'input':e['input'],'mode':e['mode'],'numeric_leaf_count':len(numeric(new)),'all_nonmetadata_exactly_equal':True,'output_sha256':hashlib.sha256(run.stdout).hexdigest()})
(p/'NUMERICAL_COMPARISON.json').write_text(json.dumps({'source_sha256':hashlib.sha256((w/'core/product_physics/src/lib.rs').read_bytes()).hexdigest(),'verdict':'PASS','executions':r},indent=2)+'\n')
print(len(r),'executions PASS',sum(x['numeric_leaf_count'] for x in r),'numeric leaves')
