"""Read-only byte binding and packet self-check; no solver operation."""
import hashlib,json,os,pathlib,subprocess,sys
ROOT=pathlib.Path(sys.argv[1]);NUM=pathlib.Path(sys.argv[2]);R=sys.argv[3]
OUT=pathlib.Path.cwd();REV='40129a225d73860ac2a53da9a2fa73869df668f3'
env=dict(os.environ,GIT_OPTIONAL_LOCKS='0')
def sha(b):return hashlib.sha256(b).hexdigest()
def git(root,revision,path):
 return subprocess.run(['git','show',revision+':'+path],cwd=root,env=env,check=True,stdout=subprocess.PIPE).stdout
sources=[]
def pin(root,revision,path,read):
 raw=git(root,revision,path)
 sources.append({'path':path,'revision':revision,'sha256':sha(raw),'bytes':len(raw),'consultation':read})
 return raw
brief=pin(NUM,'1b5a4b59afa784c5fa0449353b58667c5d932a72',R+'/BRIEFS/K6C_SELECTED_NUMERATOR_DESIGN.md','full bounded brief')
P='projects/chirality-piping';K=P+'/core/solver/frame_kernel/src/structural/retained';VR=P+'/validation/benchmarks/numerical_robustness'
for path in [K+'/adaptive.rs',K+'/assemble.rs',K+'/recover.rs',K+'/verify.rs',K+'/source.rs',K+'/wide/multi.rs',VR+'/src/lane.rs',VR+'/src/floor.rs',VR+'/src/cases.rs']:
 pin(ROOT,REV,path,'selected cited code and local searches; immutable object, not older K6C maintained adaptive')
for path in [R.rsplit('/',1)[0]+'/DESIGN_NUMERICS/REV_5A3_CANDIDATE/D1_REV_5A3_SSTAR_RESOLUTION_R7.md']:
 pin(ROOT,REV,path,'R7 theorem inherited from prior full A1 design reading; byte bound here')
# The inherited instruction bytes and exact design are recorded by original seal.
A1=ROOT.parent/'a1';basis=A1/R/'design_a1/BASIS.json'
original=json.loads(basis.read_text())
instructions=[]
for item in original['source_records']:
 if item['path'] in ['AGENTS.md','agents/AGENT_HELPS_HUMANS.md',P+'/AGENTS.md',R+'/BRIEFS/COMMON.md']:
  instructions.append(item)
for path in [R+'/design_a1/DESIGN_PROPOSAL.md',R+'/design_a1/addendum_01/CORRECTION.md']:
 raw=(A1/path).read_bytes();sources.append({'path':path,'sha256':sha(raw),'bytes':len(raw),'consultation':'own sealed A1 derivation retained in context'})
sealed=[]
root14=ROOT/R/'I21/source_14'
for line in (root14/'SHA256SUMS').read_text().splitlines():
 digest,name=line.split('  ',1);actual=sha((root14/name).read_bytes());assert digest==actual,name
 sealed.append({'path':name,'sha256':digest,'matches':True})
for name in ['FINITE_CALLERS.md','OBSERVATION_GAPS.json','FIXED_INEQUALITIES_ALL.json','INEQUALITY_SUMMARY.json','BINDING.json']:
 raw=(root14/name).read_bytes();sources.append({'path':R+'/I21/source_14/'+name,'sha256':sha(raw),'bytes':len(raw),'consultation':'source14 proposed input, pending RV30; not adopted as acceptance'})
result=json.loads((OUT/'DERIVED_BOUNDS.json').read_text());assert result['summary']['rows']==138 and result['summary']['finite_rows']==138 and result['summary']['cases']==18
assert result['summary']['worst_quotient_upper_pow2']==1020
gaps=json.loads((root14/'OBSERVATION_GAPS.json').read_text())
assert gaps['rows']==138
assert {c['id']:len(c['rows']) for c in result['cases']}=={c['id']:c['rows'] for c in gaps['cases']}
actual={(c['id'],r['row_index'],r['key'],r['member']) for c in result['cases'] for r in c['rows']}
descriptor=json.loads((root14/'FIXED_INEQUALITIES_ALL.json').read_text())
required={(c['id'],r['row_index'],r['key'],r['member']) for c in descriptor['cases'] for r in c['observed_division_gaps']}
assert actual==required and len(actual)==138
proof={'status':'PASS','scope':'author arithmetic/inventory self-check; independent mathematical review still required',
'rows':138,'cases':18,'maximum_quotient_upper_pow2':1020,'all_roster_identities_equal':True,
'all_source14_sealed_payloads_match':len(sealed),'bounds_finite_at_binary64_powers':True}
(OUT/'SELF_CHECK.json').write_text(json.dumps(proof,indent=2)+'\n')
b={'agent':'/root/a1_design','role':'HELPS_HUMANS','parent':'/root','mechanism':'collaboration.followup_task','actual_start_utc':'2026-10-01 17:13:14 UTC','deadline_utc':'2026-10-01 17:38:14 UTC','instruction_hashes_inherited':instructions,'source_records':sources,'fixture_inputs':result['input_records'],'source14_seal':sealed,'runtime':{'executable':'<VENV>/bin/python','version':sys.version,'sha256':sha(pathlib.Path(sys.executable).read_bytes())},'write_fence':'<K6C>/<R>/metric_design_05_selected_numerators','limits':['no Rust/build/runtime/solver/probe/model generation','no child delegation, Git/index mutation, maintained source edit or host tooling','no source14 acceptance, full E_max, admission or product claim']}
(OUT/'BASIS.json').write_text(json.dumps(b,indent=2)+'\n')
print(json.dumps(proof))
