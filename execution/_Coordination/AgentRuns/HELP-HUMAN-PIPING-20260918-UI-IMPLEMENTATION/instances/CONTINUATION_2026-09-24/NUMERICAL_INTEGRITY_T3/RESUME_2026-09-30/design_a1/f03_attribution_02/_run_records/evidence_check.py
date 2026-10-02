"""Hash/inventory checks of existing packets only; no collector or runtime."""
import hashlib,json,os,pathlib,subprocess,sys
ROOT=pathlib.Path(sys.argv[1]);NUM=pathlib.Path(sys.argv[2]);R=sys.argv[3];OUT=pathlib.Path.cwd()
env=dict(os.environ,GIT_OPTIONAL_LOCKS='0');REV='40129a225d73860ac2a53da9a2fa73869df668f3'
def sha(b):return hashlib.sha256(b).hexdigest()
sources=[];sealchecks=[]
def seal(rel,expected=None):
 p=ROOT/rel;m=p/'SHA256SUMS';digest=sha(m.read_bytes())
 if expected:assert digest==expected,(rel,digest)
 count=0
 for line in m.read_text().splitlines():
  h,n=line.split('  ',1);assert sha((p/n).read_bytes())==h,(rel,n);count+=1
 sealchecks.append({'path':rel,'manifest_sha256':digest,'payloads_verified':count})
seal(R+'/I22/vk_f03_diagnostic_01','cf3e83b1c6d70dd7210b8baeecc4139809370523a16bc9c7605f6e78ab293459')
seal(R+'/manager/vk_f03_diagnostic_01','a2095430a71f8cdb4f9f819fcea56d15b89a2e759c2fe530add0c01572d49365')
seal(R+'/source_review_RV29/vk_f03_source_29')
seal(R+'/verification/vk_f03_plan_01')
P='projects/chirality-piping';T=R.rsplit('/',1)[0];K=P+'/core/solver/frame_kernel/src/structural/retained';VR=P+'/validation/benchmarks/numerical_robustness'
for p in [K+'/assemble.rs',K+'/factor.rs',K+'/seeded.rs',VR+'/tests/lane.rs',T+'/IMPLEMENTATION/VK/PLAN_CHECKPOINT0.md']:
 b=subprocess.run(['git','show',REV+':'+p],cwd=ROOT,env=env,check=True,stdout=subprocess.PIPE).stdout
 sources.append({'path':p,'revision':REV,'sha256':sha(b),'read':'selected cited sections'})
p=R+'/BRIEFS/F03_ATTRIBUTION_RESUME.md'
b=subprocess.run(['git','show','a341d1cf9527fb2b9dfa2bb486f88057847aa94c:'+p],cwd=NUM,env=env,check=True,stdout=subprocess.PIPE).stdout
sources.append({'path':p,'revision':'a341d1cf9527fb2b9dfa2bb486f88057847aa94c','sha256':sha(b),'read':'full brief'})
for p in [R+'/I23/vk_prep_05/VK_MAPS.json',R+'/verification/vk_f03_plan_01/PLAN.md',R+'/source_review_RV29/vk_f03_source_29/REVIEW.md',R+'/I22/vk_f03_diagnostic_01/RETURN.md',R+'/manager/vk_f03_diagnostic_01/RETURN.md',R+'/I22/vk_f03_diagnostic_01/SOURCE_LINKS.json',R+'/I22/vk_f03_diagnostic_01/CASE_LINKS.json',R+'/I22/vk_f03_diagnostic_01/READINESS.json',R+'/source_review_RV29/vk_f03_source_29/_run_records/SOURCE_BRANCH.json']:
 b=(ROOT/p).read_bytes();sources.append({'path':p,'sha256':sha(b),'read':'full text or bounded structured selection'})
base=ROOT/R/'I22/vk_f03_diagnostic_01'
recs={};reports={};evidence={}
for d in ['D01_NONE_BEFORE','D02_F03','D03_NONE_RETURN']:
 recs[d]=json.loads((base/d/'RECORDS.json').read_text());reports[d]=json.loads((base/d/'REPORTS.json').read_text());evidence[d]=json.loads((base/d/'EVIDENCE.json').read_text())
 for name in ['RECORDS.json','REPORTS.json','EVIDENCE.json','stdout.txt','stderr.txt']:
  b=(base/d/name).read_bytes();sources.append({'path':R+'/I22/vk_f03_diagnostic_01/'+d+'/'+name,'sha256':sha(b),'read':'existing canonical records and raw hash binding'})
 for stream in ['stdout','stderr']:assert sha((base/d/(stream+'.txt')).read_bytes())==evidence[d]['raw_hashes'][stream]
 assert len(recs[d])==66 and evidence[d]['executed']
assert recs['D01_NONE_BEFORE']==recs['D03_NONE_RETURN']
assert reports['D01_NONE_BEFORE']==reports['D03_NONE_RETURN']
assert all(evidence[d]['argv']==evidence['D01_NONE_BEFORE']['argv'] for d in evidence)
plain_env=lambda e:{k:v for k,v in e['environment'].items() if k!='FK_SEEDED_FAULT'}
assert all(plain_env(evidence[d])==plain_env(evidence['D01_NONE_BEFORE']) for d in evidence)
assert [evidence[d]['environment']['FK_SEEDED_FAULT'] for d in ['D01_NONE_BEFORE','D02_F03','D03_NONE_RETURN']]==['NONE','VK-F03','NONE']
assert all(evidence[d]['unset_environment']==evidence['D01_NONE_BEFORE']['unset_environment'] for d in evidence)
rows=json.loads((OUT/'ROOT_DIAGONAL_INEQUALITIES.json').read_text())
assert len(rows['cases'])==66
for d in recs:
 assert [x['id'] for x in recs[d]]==[x['id'] for x in recs['D01_NONE_BEFORE']]
 for a,b in zip(recs['D01_NONE_BEFORE'],recs[d]):assert a['k4src_sha256']==b['k4src_sha256']
from collections import Counter
group=Counter()
for c in rows['cases']:
 seq=' '.join(str(x) for x in c['F03_sequence'])
 group[c['family']+(' Pivot' if 'Pivot' in seq else ' StopRule')]+=1
assert dict(group)=={'RF-CHAIN Pivot':30,'RF-SKEW StopRule':18,'RF-SKEW Pivot':18}
checks={'status':'PASS','seals':sealchecks,'controlled_records_each':66,'total_records':198,'NONE_records_equal':True,'NONE_reports_equal':True,'raw_stream_hashes_match':6,'source_ids_equal_all_processes':True,'group_counts':dict(group),'root_positive_unique_minima':66,'scope':'author source/arithmetic and existing-evidence check; independent RV29 qualification remains'}
(OUT/'CHECKS.json').write_text(json.dumps(checks,indent=2)+'\n')
old=json.loads((ROOT/R/'design_a1/BASIS.json').read_text())
instructions=[s for s in old['source_records'] if s['path'] in ['AGENTS.md','agents/AGENT_HELPS_HUMANS.md',P+'/AGENTS.md',R+'/BRIEFS/COMMON.md']]
basis={'agent':'/root/a1_design','parent':'/root','role':'HELPS_HUMANS','mechanism':'collaboration.followup_task','actual_start_utc':'2026-10-01 19:35:44 UTC','deadline_utc':'2026-10-01 19:50:44 UTC','instruction_hashes_inherited':instructions,'new_sources':sources,'fixture_sources':rows['source_inputs'],'runtime':{'executable':'<VENV>/bin/python','version':sys.version},'write_fence':'<A1>/<R>/design_a1/f03_attribution_02','no_runtime_or_mutation':True}
(OUT/'BASIS.json').write_text(json.dumps(basis,indent=2)+'\n')
print(json.dumps(checks))
