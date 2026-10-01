"""RV28 narrow source/hash check. Does not execute floating parsing or products."""
from pathlib import Path
import hashlib,html,json,os,re,subprocess,sys
K,OUT=map(Path,sys.argv[1:]);raw=OUT/'_run_records'
T='projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3'
R=T+'/RESUME_2026-09-30';P=R+'/I23/f64_parse_source_19'
checks=[];origins=[];commands=[]
def sha(b):return hashlib.sha256(b).hexdigest()
def ck(n,x):
 assert x,n
 checks.append(n)
def seal(path,expected=None):
 b=(K/path/'SHA256SUMS').read_bytes()
 if expected:ck('seal '+path,sha(b)==expected)
 n=0
 for line in b.decode().splitlines():
  h,f=line.split(maxsplit=1);q=K/path/f
  if not q.exists():q=K/f
  ck('sealed payload '+f,sha(q.read_bytes())==h);n+=1
 return {'path':path,'seal_sha256':sha(b),'entries':n}
seals=[seal(P,'5b5f5384465b3cddae63602816eb82652aab25b2eb29e3d120c1ec2f1d75270c'),seal(R+'/source_review_RV30/finite_callers_12')]
binding=json.loads((K/P/'_run_records/SOURCE_BINDINGS.json').read_text())
for row in binding['stdlib_pages']:
 b=Path(row['actual_path']).read_bytes();ck('html '+row['origin'],sha(b)==row['html_sha256'])
 code=b.decode().split('<pre class="rust"><code>',1)[1].split('</code></pre>',1)[0]
 code=re.sub(r'<a\b[^>]*data-nosnippet[^>]*>.*?</a>','',code,flags=re.S)
 code=html.unescape(re.sub(r'<[^>]*>','',code))
 ck('decoded '+row['origin'],sha(code.encode())==row['decoded_sha256'])
 lines=code.splitlines();excerpt=(K/P/'_run_records'/row['evidence_file']).read_text()
 for line in excerpt.splitlines():
  if not line:continue
  no,text=line.split(': ',1);ck('excerpt '+row['evidence_file']+':'+no,lines[int(no)-1]==text)
 origins.append(row)
for row in binding['application_sites']:
 p=subprocess.run(['git','show',row['revision']+':'+row['path']],cwd=K,env=dict(os.environ,GIT_OPTIONAL_LOCKS='0'),capture_output=True)
 commands.append({'cwd':str(K),'argv':['git','show',row['revision']+':'+row['path']],'GIT_OPTIONAL_LOCKS':'0','exit':p.returncode,'stdout_sha256':sha(p.stdout),'stderr':p.stderr.decode()})
 ck('caller '+row['path'],p.returncode==0 and sha(p.stdout)==row['sha256'])
 lines=p.stdout.decode().splitlines()
 for line in (K/P/'_run_records'/row['evidence_file']).read_text().splitlines():
  if not line:continue
  no,text=line.split(': ',1);ck('caller excerpt '+no,lines[int(no)-1]==text)
 origins.append(row)
for path in [P+'/RETURN.md',P+'/_run_records/WARRANT.json',R+'/source_review_RV30/finite_callers_12/RETURN.md',R+'/source_review_RV30/finite_callers_12/INTEGER_INPUT_RECHECK.json']:
 b=(K/path).read_bytes();origins.append({'origin':'<K6C>','path':path,'sha256':sha(b),'bytes':len(b)})
(raw/'BASIS.json').write_text(json.dumps({'agent':'/root/rv28_a1_design','parent':'/root','role':'TASK','mechanism':'collaboration.followup_task','instruction_basis':'same session; preserved vr_prefix_02 instruction/skill bindings','source_records':origins,'seals':seals,'boundary':'Installed source/excerpt equality and inherited fixed-input finite premise; no actual decimal strings parsed here'},indent=2)+'\n')
(raw/'COMMANDS.json').write_text(json.dumps(commands,indent=2)+'\n')
(raw/'CHECKS.json').write_text(json.dumps({'status':'PASS','count':len(checks),'checks':checks,'parser_or_runtime_executions':0},indent=2)+'\n')
print(json.dumps({'status':'PASS','checks':len(checks),'stdlib_pages':14,'caller_sites':2,'seals':2,'decimal_parser_runs':0}))
