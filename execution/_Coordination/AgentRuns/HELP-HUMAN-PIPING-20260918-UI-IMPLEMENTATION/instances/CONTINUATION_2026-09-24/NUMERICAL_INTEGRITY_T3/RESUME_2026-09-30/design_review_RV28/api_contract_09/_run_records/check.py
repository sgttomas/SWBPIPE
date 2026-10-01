"""Source/metadata identity and result-equation backcheck only; no product execution."""
from pathlib import Path
import hashlib,json,sys
K,OUT=map(Path,sys.argv[1:]);raw=OUT/'_run_records'
T='projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3';R=T+'/RESUME_2026-09-30';P=R+'/metric_design_13_api_contract';VR='projects/chirality-piping/validation/benchmarks/numerical_robustness'
checks=[];basis=[]
def sha(b):return hashlib.sha256(b).hexdigest()
def ck(n,x):
 assert x,n
 checks.append(n)
def rd(path):
 b=(K/path).read_bytes();basis.append({'origin':'<K6C>','path':path,'sha256':sha(b),'bytes':len(b)});return b
seal=rd(P+'/SHA256SUMS');ck('seal identity',sha(seal)=='2c290d75ed60f90dda7aa3382548e7c44901f3434376e9b5ce538202c707f0cf');n=0
for line in seal.decode().splitlines():
 h,f=line.split(maxsplit=1);ck('payload '+f,sha((K/P/f).read_bytes())==h);n+=1
ck('23 payloads',n==23)
for path in ['CONTRACT.md','RETURN.md','RESULT_CONTRACT.json','_run_records/API_COMPATIBILITY.md','_run_records/ROOT_SELECTION.txt','_run_records/ROSTER_CHECK.json']:
 rd(P+'/'+path)
origins=json.loads(rd(P+'/_run_records/SOURCE_ORIGINS.json'));byrel={x['path']:x for x in origins}
mapping={'src__scale.rs':VR+'/src/scale.rs','src__cases.rs':VR+'/src/cases.rs','src__lane.rs':VR+'/src/lane.rs','src__records.rs':VR+'/src/records.rs','src__parity.rs':VR+'/src/parity.rs','tests__scale.rs':VR+'/tests/scale.rs','examples__vk_scale.rs':VR+'/examples/vk_scale.rs','retained_source.rs':'projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/source.rs'}
for snapshot,path in mapping.items():
 b=rd(P+'/_run_records/'+snapshot);ck('source snapshot '+snapshot,sha(b)==byrel[path]['sha256']);ck('current source equality '+path,b==(K/path).read_bytes())
rc=json.loads(rd(P+'/_run_records/ROSTER_CONTEXT.json'));storage={};storage_keys=set()
for item in rc['storage_sources']:
 b=rd(item['path']);ck('storage hash '+item['path'],sha(b)==item['sha256'])
 for case in json.loads(b):
  if case['attempts']:
   s=case['attempts'][0]['storage'];storage[case['id']]=s;storage_keys.update(s)
family_files=['rf_chain','rf_skew','rf_weak','rf_large','rf_invariance','rf_range','rf_zero','rf_finite','rf_mech','rf_cancel']
cases=[]
for name in family_files:
 path=VR+'/cases/'+name+'.jsonl';b=rd(path)
 # These fixed input identities are already in the authoritative source09 census;
 # this leg uses only JSON metadata, not model construction or graph counts.
 cases.extend(json.loads(line) for line in b.splitlines() if line)
required={c['id'] for c in cases if c['model'] is not None and c['id'] in storage};large={c['id'] for c in cases if c['family']=='RF-LARGE'}
ck('213 metadata unique',len(cases)==len({c['id'] for c in cases})==213)
ck('193 required calls',len(required)==193);ck('12 overlap181 outside',(len(required&large),len(required-large))==(12,181))
ck('ten required families',len({c['family'] for c in cases if c['id'] in required})==10)
ck('storage has no B or b',not ({'bodies','blocks','B','b'}&storage_keys))
result=json.loads((K/P/'RESULT_CONTRACT.json').read_text());ck('six cells',len(result['remaining_cells'])==6)
ck('20 decimal digits for64bit cap',len(str(2**64-1))==20)
# Finite algebra sanity cases supplement, not replace, the symbolic proof.
for M in [0,1,1024]:
 for extra in [0,4]:
  Ccut=M+extra;B0=3392;E=11;L=78;Tctrl=20;pub=100
  fixed=B0+Ccut+E+L+Tctrl+pub;model=M+pub;A=Ccut+E+L+Tctrl+pub
  O128=B0+100+200;Oall=O128+1024;K128=O128+500;Kall=K128+2000;As=50;C=40000
  selected=max(C,As+K128,A+O128);full=max(C,As+Kall,A+Oall)
  ck('owner subset '+str((M,extra)),fixed-model==B0+extra+E+L+Tctrl and model<fixed<selected<=full)
(raw/'BASIS.json').write_text(json.dumps({'agent':'/root/rv28_a1_design','parent':'/root','role':'TASK','mechanism':'collaboration.followup_task','source_records':basis,'source_origin_records':origins,'instruction_basis':'same active session, prior sealed Root/TASK/Piping/software-code-review records','scope':'metadata/source equality only; no graph counts or parser/model source algorithm run'},indent=2)+'\n')
(raw/'CHECKS.json').write_text(json.dumps({'status':'PASS','count':len(checks),'checks':checks,'roster':{'required':193,'overlap':12,'outside':181},'storage_keys':sorted(storage_keys),'algebra':'symbolic derivation in REVIEW.md; finite checks only sanity cases'},indent=2)+'\n')
print(json.dumps({'status':'PASS','checks':len(checks),'payloads':23,'source_snapshots':8,'required_calls':193,'fixed24_overlap':12,'outside_fixed24':181,'full_numeric193_instantiation':False}))
