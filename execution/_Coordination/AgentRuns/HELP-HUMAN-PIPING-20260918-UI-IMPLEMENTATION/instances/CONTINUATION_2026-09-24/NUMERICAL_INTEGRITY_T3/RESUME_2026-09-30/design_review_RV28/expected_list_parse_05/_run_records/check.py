"""Fixed-byte/source/integer backcheck only; no JSON input parser execution."""
from pathlib import Path
import hashlib,json,os,re,subprocess,sys
K,OUT=map(Path,sys.argv[1:]);raw=OUT/'_run_records'
T='projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3'
R=T+'/RESUME_2026-09-30';P=R+'/I23/expected_list_parse_20';REV='40129a225d73860ac2a53da9a2fa73869df668f3'
records=[];commands=[];checks=[]
def sha(b):return hashlib.sha256(b).hexdigest()
def ck(n,x):
 assert x,n
 checks.append(n)
seal=(K/P/'SHA256SUMS').read_bytes();ck('seal identity',sha(seal)=='5d256c1dbbc107b2024eb9bd6e7a86a8649066905a2d42e59fc20bb1a0dfe6bf')
for line in seal.decode().splitlines():
 h,f=line.split(maxsplit=1);ck('sealed '+f,sha((K/P/f).read_bytes())==h)
def git(path):
 p=subprocess.run(['git','show',REV+':'+path],cwd=K,env=dict(os.environ,GIT_OPTIONAL_LOCKS='0'),capture_output=True)
 commands.append({'cwd':str(K),'argv':['git','show',REV+':'+path],'GIT_OPTIONAL_LOCKS':'0','exit':p.returncode,'stdout_sha256':sha(p.stdout),'stderr':p.stderr.decode()});ck('git '+path,p.returncode==0)
 records.append({'origin':'Git object','revision':REV,'path':path,'sha256':sha(p.stdout),'bytes':len(p.stdout)})
 return p.stdout
ip='projects/chirality-piping/validation/benchmarks/numerical_robustness/cases/expected_unresolved.json';data=git(ip)
ck('input identity',len(data)==1357 and sha(data)=='2e5d0975a90c69e5d198d61760cc7734d42615461710da1fe226f60b4a46f2c9')
ck('raw evidence equality',data==(K/P/'_run_records/expected_unresolved.input.txt').read_bytes())
offsets=[i for i,b in enumerate(data) if b==92];ck('exact backslashes',offsets==[1300,1351])
ck('quote escape bytes',all(data[i:i+2]==b'\\"' for i in offsets))
line=next(x for x in data.splitlines() if b'"source": ' in x);payload=line.split(b': "',1)[1][:-1]
segments=payload.split(b'\\"');ck('segment counts',list(map(len,segments))==[20,49,0]);ck('payload/decode lengths',len(payload)==73 and len(b'"'.join(segments))==71)
stripped=re.sub(rb'"(?:[^"\\]|\\.)*"',b'',data);ck('no numeric or value token scratch route',not re.search(rb'[0-9A-Za-z]',stripped))
names=re.findall(rb'"case": "([^"\\]*)"',data);ck('typed case names',[len(n) for n in names]==[15,15]);ck('carried typed byte arithmetic',2*24+sum(map(len,names))==78)
length=capacity=0;trace=[]
for op,n in [('extend',20),('push',1),('extend',49),('push',1),('extend',0)]:
 old=capacity;need=length+n;grow=need>capacity
 if grow:capacity=max(8,2*capacity,need)
 trace.append({'operation':op,'additional':n,'length_before':length,'old_capacity':old,'new_capacity':capacity,'active_old':old if grow else 0,'scratch_move':capacity+(old if grow else 0)})
 length=need
ck('growth capacities',[x['new_capacity'] for x in trace]==[20,40,80,80,80]);ck('retained/old/move',(capacity,max(x['active_old'] for x in trace),max(x['scratch_move'] for x in trace))==(80,40,120))
binding=json.loads((K/P/'_run_records/SOURCE_BINDINGS.json').read_text())
for row in binding['bindings']:
 b=Path(row['actual_path']).read_bytes();ck('source hash '+row['origin'],sha(b)==row['sha256']);records.append(row)
 if 'FROZEN_ARCHIVE' in row['origin']:
  rel=row['origin'].split('<FROZEN_ARCHIVE>/',1)[1];ck('archive equals pinned '+rel,b==git(rel))
for rel in [R+'/I21/source_02/LIBRARY_CONTRACTS.md',R+'/source_review_RV30/vr_caller_07/RETURN.md',R+'/I21/source_11/FORMULAS_AND_GAPS.md',P+'/RETURN.md',P+'/_run_records/SCRATCH_WARRANT.json']:
 b=(K/rel).read_bytes();records.append({'origin':'<K6C>','path':rel,'sha256':sha(b),'bytes':len(b)})
(raw/'BASIS.json').write_text(json.dumps({'agent':'/root/rv28_a1_design','parent':'/root','role':'TASK','mechanism':'collaboration.followup_task','instruction_basis':'same active session and sealed prior instruction/skill binding','seal_sha256':sha(seal),'source_records':records},indent=2)+'\n')
(raw/'COMMANDS.json').write_text(json.dumps(commands,indent=2)+'\n')
(raw/'CHECKS.json').write_text(json.dumps({'status':'PASS','count':len(checks),'checks':checks,'offsets':offsets,'segment_lengths':list(map(len,segments)),'trace':trace,'parser_runs':0,'runtime_runs':0},indent=2)+'\n')
print(json.dumps({'status':'PASS','checks':len(checks),'retained_scratch':80,'scratch_move_upper':120,'active_old':40,'typed_list_retained_carried':78,'parser_runs':0}))
