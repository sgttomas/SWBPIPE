"""Narrow additive RV28 backcheck: sealed identity/source and integer evidence.
No runtime/model/parser/solver/library execution. K6C NUM OUT arguments.
"""
from pathlib import Path
import hashlib,json,os,subprocess,sys
K6C,NUM,OUT=map(Path,sys.argv[1:]);raw=OUT/'_run_records'
T='projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3'
R=T+'/RESUME_2026-09-30';REV='40129a225d73860ac2a53da9a2fa73869df668f3'
records=[];commands=[];checks=[]
def sha(b):return hashlib.sha256(b).hexdigest()
def ck(n,x):
 assert x,n
 checks.append(n)
def read(path):
 b=(K6C/path).read_bytes();records.append({'path':path,'origin':'<K6C>','sha256':sha(b),'bytes':len(b)});return b
def git(root,rev,path):
 p=subprocess.run(['git','show',rev+':'+path],cwd=root,env=dict(os.environ,GIT_OPTIONAL_LOCKS='0'),capture_output=True)
 commands.append({'cwd':str(root),'argv':['git','show',rev+':'+path],'env':{'GIT_OPTIONAL_LOCKS':'0'},'exit':p.returncode,'stdout_sha256':sha(p.stdout),'stderr':p.stderr.decode()})
 ck('git '+path,p.returncode==0);records.append({'path':path,'revision':rev,'origin':'Git object','sha256':sha(p.stdout),'bytes':len(p.stdout)})
 return p.stdout
brief=git(NUM,'38360301a1a2074d0c206f1d02f9ed016d96a7d6',R+'/BRIEFS/K6C_PREFIX_CORRECTION_09.md');(raw/'BRIEF.md').write_bytes(brief)
seals=[]
for packet,digest in [('metric_design_09_prefix_correction','72582e7a4e58721e32290dfd61d7d226f4d9be681007c34763869095423f7b82'),('metric_design_08_vr_prefix','ab75fce30ff4be0716f2fbc35af17079294bde6f6c81f5c3e7245e918a7492a7'),('design_review_RV28/vr_prefix_02','d4748c38bce84b209523ed9d4def64bc9181a74f37bbd5467f2c29447b10c5fd')]:
 base=R+'/'+packet;b=read(base+'/SHA256SUMS');ck('seal '+packet,sha(b)==digest);count=0
 for line in b.decode().splitlines():
  h,path=line.split(maxsplit=1);f=K6C/base/path
  if not f.exists():f=K6C/path
  ck('payload '+str(f.relative_to(K6C)),sha(f.read_bytes())==h);count+=1
 seals.append({'packet':base,'sha256':digest,'entries':count})
corr=read(R+'/metric_design_09_prefix_correction/CORRECTION.md').decode();read(R+'/metric_design_09_prefix_correction/RETURN.md')
for path in ['I21/source_02/LIBRARY_CONTRACTS.md','I21/source_12/H_LEAVES.md','source_review_RV30/h_request_bindings_10/RETURN.md','design_review_RV28/vr_prefix_02/REVIEW.md','design_review_RV28/vr_prefix_02/_run_records/temporary_scope_excerpt.html']:
 read(R+'/'+path)
paths=[('projects/chirality-piping/validation/benchmarks/numerical_robustness/src/scale.rs',133,154),('projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/source.rs',624,635),('projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/recover.rs',97,121)]
for i,(path,a,b) in enumerate(paths):
 data=git(K6C,REV,path);(raw/f'source_{i}.txt').write_text('\n'.join(f'{j+1}: {line}' for j,line in enumerate(data.decode().splitlines()) if a<=j+1<=b)+'\n')
expected=['HAdj,','RAdj + HProfileCount,','HFree,','RFree + HPatternCount,','RFree + HLayout,','RFree + RLayout + HEncoding(X)']
for text in expected:ck('corrected term '+text,text in corr)
ck('free retained definition','RFree   = G(usize,f)' in corr)
ck('free growth definition','HFree   = RFree + epsilon*O_G(usize,f)' in corr)
ck('layout growth definition','HLayout = RLayout + epsilon*O_G(QuantityMeta,q)' in corr)
ck('component integer',8*(1<<(60000-1).bit_length())==524288)
ck('unchanged cut excludes dead buffers','Do not add RFree, RLayout or the second encoding buffer to C_tV' in corr)
ck('existing entry bound','EntryPeak <= M_stack + L_info + 4' in corr and 'EntryGrowingOld = 0' in corr)
ck('remaining qualified runtime','Failed startup, foreign' in corr and 'panic/unwind' in corr and 'artifact correspondence' in corr)
ck('remaining format ownership','wrapped serde writer/error route' in corr)
(raw/'BASIS.json').write_text(json.dumps({'agent':'/root/rv28_a1_design','parent':'/root','role':'TASK','mechanism':'collaboration.followup_task','instruction_basis':'same session, recorded origins/hashes in sealed vr_prefix_02/_run_records/BASIS.json','source_records':records,'seals':seals},indent=2)+'\n')
(raw/'COMMANDS.json').write_text(json.dumps(commands,indent=2)+'\n')
(raw/'CHECKS.json').write_text(json.dumps({'status':'PASS','count':len(checks),'checks':checks,'limit':'mechanical evidence checks support the independently reasoned BACKCHECK.md; string presence is not a mathematical proof'},indent=2)+'\n')
print(json.dumps({'status':'PASS','checks':len(checks),'seals':len(seals),'source_blobs':3,'finding':'RV28-PREFIX-1 closed at source-design level','full_Emax_accepted':False}))
