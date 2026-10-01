"""Bounded RV28 prefix review evidence. No product/parser/model execution.
Read existing metadata, hash raw input bytes, and inspect string/number lexemes.
Usage: python -B check_evidence.py K6C NUM APP WT RUST OUT
"""
from pathlib import Path
import hashlib,json,os,re,subprocess,sys
K6C,NUM,APP,WT,RUST,OUT=map(Path,sys.argv[1:]); raw=OUT/'_run_records'
T='projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3'
R=T+'/RESUME_2026-09-30'; P=R+'/metric_design_08_vr_prefix'
REV='40129a225d73860ac2a53da9a2fa73869df668f3'
env=dict(os.environ,GIT_OPTIONAL_LOCKS='0'); checks=[];basis=[];commands=[]
def sha(b):return hashlib.sha256(b).hexdigest()
def ck(n,b):
 assert b,n
 checks.append(n)
def local(path):
 b=(K6C/path).read_bytes();basis.append({'origin':'<K6C>','path':path,'sha256':sha(b),'bytes':len(b)});return b
def gshow(root,rev,path):
 p=subprocess.run(['git','show',rev+':'+path],cwd=root,env=env,capture_output=True)
 commands.append({'cwd':str(root),'argv':['git','show',rev+':'+path],'GIT_OPTIONAL_LOCKS':'0','exit':p.returncode,'stdout_sha256':sha(p.stdout),'stderr':p.stderr.decode()});ck('git '+path,p.returncode==0)
 basis.append({'origin':'Git object','path':path,'revision':rev,'sha256':sha(p.stdout),'bytes':len(p.stdout)})
 return p.stdout
brief=gshow(NUM,'a341d1cf9527fb2b9dfa2bb486f88057847aa94c',R+'/BRIEFS/K6C_PENDING_PROPOSAL_REVIEWS.md');(raw/'BRIEF.md').write_bytes(brief)
for path in ['AGENTS.md','agents/AGENT_TASK.md','projects/chirality-piping/AGENTS.md','.agents/skills/software-code-review/SKILL.md']:
 b=(APP/path).read_bytes();basis.append({'origin':'<APP_WORKTREE> active instructions/skill','path':path,'sha256':sha(b),'bytes':len(b)})
seals=[]
for packet,expected in [(P,'ab75fce30ff4be0716f2fbc35af17079294bde6f6c81f5c3e7245e918a7492a7'),(R+'/source_review_RV30/k0_assembly_16',None),(R+'/source_review_RV30/h_leaves_09',None),(R+'/source_review_RV30/h_request_bindings_10',None)]:
 b=local(packet+'/SHA256SUMS')
 if expected:ck('prefix seal identity',sha(b)==expected)
 count=0
 for line in b.decode().splitlines():
  h,f=line.split(maxsplit=1);path=K6C/packet/f.lstrip('*')
  if not path.exists():path=K6C/f.lstrip('*')
  ck('payload '+str(path.relative_to(K6C)),sha(path.read_bytes())==h);count+=1
 seals.append({'packet':packet,'seal_sha256':sha(b),'entries':count})
for path in [P+'/PREFIX_UNION.md',P+'/NODE_INTERFACE.md',P+'/RETURN.md',P+'/RUN.json',R+'/source_review_RV30/k0_assembly_16/RETURN.md',R+'/source_review_RV30/k0_assembly_16/FINDINGS.json',R+'/I21/source_02/LIBRARY_CONTRACTS.md',R+'/I21/source_09/STRING_CONTRACTS.json',R+'/I21/source_12/H_LEAVES.md',R+'/source_review_RV30/h_leaves_09/RETURN.md',R+'/source_review_RV30/h_request_bindings_10/RETURN.md']:
 local(path)
binding=json.loads(local(P+'/_run_records/SOURCE_BINDING.json'))
products=[];std=[];packages=[]
for row in binding:
 origin=row['origin'];snapshot=row.get('raw_snapshot')
 if not snapshot:continue
 b=local(P+'/'+snapshot)
 if ' git '+REV+':' in origin:
  path=origin.split(REV+':',1)[1];gb=gshow(K6C,REV,path);ck('product full blob '+path,gb==b and sha(b)==row['sha256']);products.append(path)
 elif origin.startswith('<RUST>'):
  original=RUST/origin.split('<RUST>/',1)[1]
  ck('original std html '+str(original),sha(original.read_bytes())==row['html_sha256'])
  ck('retained decoded std '+snapshot,sha(b)==row['decoded_sha256']);std.append(snapshot)
 elif origin.startswith('<CARGO_HOME>'):
  ck('package snapshot '+snapshot,sha(b)==row['sha256'] and row['matches_sealed_authenticated_member']);packages.append(snapshot)
source_path='projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/source.rs'
src=gshow(K6C,REV,source_path);(raw/'source_free_dofs_excerpt.txt').write_text('\n'.join(f'{i+1}: {line}' for i,line in enumerate(src.decode().splitlines()) if 620<=i+1<=637)+'\n')
scale=local(P+'/_run_records/product_02.txt').decode()
ck('free_dofs allocates fresh filtered Vec',b'pub fn free_dofs(&self) -> Vec<usize>' in src and b'.filter(|&g| self.constrained[g].is_none())' in src)
ck('field evaluated before pattern layout encoding',scale.index('free_dofs: source.free_dofs().len()')<scale.index('pattern_entries: pattern_entries(model)')<scale.index('rows: layout(source).len()')<scale.index('source_encoding_len: source.encoding().len()'))
ref=RUST/'share/doc/rust/html/reference/destructors.html'; rb=ref.read_bytes()
basis.append({'origin':'<RUST>/share/doc/rust/html/reference/destructors.html','sha256':sha(rb),'bytes':len(rb),'consultation':'Temporary scope enumeration and tail-expression rule only; no runtime/code execution'})
(raw/'temporary_scope_excerpt.html').write_text('\n'.join(rb.decode().splitlines()[510:551])+'\n')
grammar=json.loads(local(P+'/_run_records/INPUT_GRAMMAR.json'));inputs=json.loads(local(R+'/I23/external_inputs_06/INPUTS.json'))
fpath=grammar['family_file']['path'];fb=gshow(K6C,REV,fpath)
ck('family identity',sha(fb)==grammar['family_file']['sha256'] and len(fb)==986182)
lines=fb.splitlines();ck('24 family lines',len(lines)==24)
bound=[]
def lexical(name,b,desc,spring_count):
 b.decode('utf-8');ck(name+' hash',sha(b)==desc['sha256'] and len(b)==desc['bytes'])
 ck(name+' no string escapes',b'\\' not in b)
 # Only lexical metadata scan; no JSON/typed/model parser is executed on inputs.
 tokens=list(re.finditer(rb'"[^"\\]*"',b));stripped=re.sub(rb'"[^"\\]*"',b'""',b)
 numbers=re.findall(rb'(?<![A-Za-z_])[-+]?[0-9]+(?:\.[0-9]+)?(?:[eE][-+]?[0-9]+)?',stripped)
 ck(name+' unsigned u64 lexemes',all(re.fullmatch(rb'0|[1-9][0-9]*',n) and int(n)<=2**64-1 for n in numbers))
 ck(name+' integer metadata count',len(numbers)==desc['integer_token_count'])
 ck(name+' no nonstandard numeric tokens',not any(x in stripped for x in [b'NaN',b'Infinity']))
 empty_springs=len(re.findall(rb'"springs"\s*:\s*\[\s*\]',b));all_springs=len(re.findall(rb'"springs"\s*:',b))
 ck(name+' raw spring-array empty',empty_springs==all_springs==spring_count)
 bound.append({'name':name,'sha256':sha(b),'bytes':len(b),'unsigned_integer_tokens':len(numbers),'string_lexemes':len(tokens),'empty_spring_arrays':empty_springs,'duplicate_keys_and_histogram_basis':'preserved INPUT_GRAMMAR metadata, not re-parsed here'})
for i,(b,d) in enumerate(zip(lines,grammar['family_file']['cases'])):
 lexical('family line '+str(i+1),b,d,1 if d['embedded_model'] else 0)
 ck('embedded spring metadata '+str(i),d['model_springs']==0 if d['embedded_model'] else d['model_springs'] is None)
byid={e['id']:e for e in inputs['entries']}
for d in grammar['external']:
 e=byid[d['id']];path=WT/e['raw_path'].split('<wt>/',1)[1];before=path.stat();b=path.read_bytes();after=path.stat()
 ck(d['id']+' stable read',(before.st_size,before.st_mtime_ns,before.st_ino)==(after.st_size,after.st_mtime_ns,after.st_ino))
 ck(d['id']+' external supplied identity',sha(b)==e['expected']['model_sha256'])
 lexical(d['id'],b,d,1)
ck('24 models no springs',len(grammar['external'])==12 and sum(d['embedded_model'] for d in grammar['family_file']['cases'])==12 and all(d['model_springs']==0 for d in grammar['external']))
raw.mkdir(parents=True,exist_ok=True)
(raw/'BASIS.json').write_text(json.dumps({'agent':'/root/rv28_a1_design','parent':'/root','mechanism':'collaboration.followup_task','source_records':basis,'preserved_seals':seals,'consultation':'canonical K0 read first; source12 runtime clarification explicitly requested by ROOT'},indent=2)+'\n')
(raw/'COMMANDS.json').write_text(json.dumps(commands,indent=2)+'\n')
(raw/'INPUT_BYTE_RECHECK.json').write_text(json.dumps(bound,indent=2)+'\n')
(raw/'CHECKS.json').write_text(json.dumps({'status':'PASS','count':len(checks),'checks':checks,'source_products':len(products),'std_pages_bound':len(std),'package_snapshots_bound':len(packages),'scope':'hash/source/lexical metadata verification only; no parser/typed-model/runtime execution'},indent=2)+'\n')
print(json.dumps({'status':'PASS','checks':len(checks),'documents':len(bound),'products':len(products),'std_pages':len(std),'package_pages':len(packages),'complete_prefix_verified':False,'finding':'RV28-PREFIX-1 missing free_dofs temporary'}))
