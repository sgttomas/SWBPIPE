"""RV28 bounded profile proposal source/identity check; no build or runtime."""
from pathlib import Path
import hashlib,json,sys
K,OUT=map(Path,sys.argv[1:]);raw=OUT/'_run_records'
T='projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3';R=T+'/RESUME_2026-09-30';P=R+'/metric_design_12_profile_binding'
checks=[];basis=[]
def sha(b):return hashlib.sha256(b).hexdigest()
def ck(n,x):
 assert x,n
 checks.append(n)
def read(rel):
 b=(K/rel).read_bytes();basis.append({'path':rel,'origin':'<K6C> current file matched to supplied hash where provided','sha256':sha(b),'bytes':len(b)});return b
seal=read(P+'/SHA256SUMS');ck('seal',sha(seal)=='e43a9eb361b3f99b261bef0899db5b44c0d021de8f1af3dc24faabc91860d123');n=0
for line in seal.decode().splitlines():
 h,f=line.split(maxsplit=1);ck('payload '+f,sha((K/P/f).read_bytes())==h);n+=1
ck('nine payloads',n==9)
proposal=read(P+'/PROPOSAL.md').decode();read(P+'/RETURN.md');read(P+'/_run_records/ROOT_SCOPE.md');read(P+'/_run_records/PARENT_CLARIFICATION.txt')
sources=json.loads(read(P+'/_run_records/SOURCES.json'))
for s in sources:
 b=read(s['path']);ck('source bound '+s['path'],sha(b)==s['sha256'])
plan=read(R+'/I21/k0_assembly_16/DESIGN_AND_PLAN.md').decode()
ck('canonical plan snapshot',(K/P/'_run_records/DESIGN_AND_PLAN.md').read_bytes()==plan.encode())
ck('unaccepted carrier explicitly pending','A future implementation must decide the exact build-metadata carrier' in plan)
ck('runtime verdict not previously selected','this is not a newly selected runtime verdict or waiver' in plan)
ck('seam retained','ROOT selected VR -> H' in plan)
for f in [R+'/I21/k0_assembly_16/K0_CANDIDATE.md',R+'/source_review_RV30/k0_assembly_16/RETURN.md',R+'/I21/k0_assembly_16/BLOCKING_CELLS.md']:
 read(f)
H='projects/chirality-piping/core/solver/performance_harness';VR='projects/chirality-piping/validation/benchmarks/numerical_robustness'
bin_test=(K/H/'tests/k6b_bin.rs').read_text();ck('ordinary subprocess declared','CARGO_BIN_EXE_k6_observe' in bin_test and 'No time or memory bound is' in bin_test)
ck('subprocess numerical success checks','output.status.success()' in bin_test and 'estimate_exceeds_half_cap' in bin_test)
manifest=(K/H/'Cargo.toml').read_text();ck('no libtest harness','test = false' in manifest)
h_test=(K/H/'tests/k6b_w1.rs').read_text();ck('all33 exact count estimates','assert_eq!(lines, 33)' in h_test and 'estimate_adm_bytes_w1a' in h_test)
v_test=(K/VR/'tests/scale.rs').read_text();ck('all193 storage/estimate relations','assert_eq!(n, 193)' in v_test and 'e.model < e.fixed && e.fixed < e.sel128 && e.sel128 <= e.max' in v_test)
guard=(K/VR/'tests/feature_guard.rs').read_text();ck('guard remains source and feature based','no_vr_source_names_the_execution_tree' in guard and 'ci_passes_no_cargo_features' in guard)
md=(K/H/'runner/k6_runner.py').read_text();ck('metadata labels supplied build information',"'build_profile': 'release'" in md and "'source_commit': source_commit" in md and "md['binary_sha256']" in md)
ci=(K/'.github/workflows/piping-desktop-e2e.yml').read_text();ck('Linux CI excludes execution',all(x in ci for x in ['runs-on: ubuntu-latest','!/projects/*/execution/','dtolnay/rust-toolchain@1.97.1']))
ck('reference not current build attestation','not a build attestation' in proposal and 'they do not certify the' in proposal)
ck('no runtime evidence dependency','do not make Rust source/tests read the execution tree' in proposal)
ck('missing proof not zero','Invalid or missing' in proposal and 'no zero' in proposal)
(raw/'BASIS.json').write_text(json.dumps({'agent':'/root/rv28_a1_design','parent':'/root','role':'TASK','mechanism':'collaboration.followup_task','source_records':basis,'instruction_basis':'same active session, prior sealed Root/TASK/Piping/skill records','scope':'all provided identities checked; semantic design assessment is in REVIEW.md'},indent=2)+'\n')
(raw/'CHECKS.json').write_text(json.dumps({'status':'PASS','count':len(checks),'checks':checks,'runtime_build_compiler_git_index_runs':0},indent=2)+'\n')
print(json.dumps({'status':'PASS','checks':len(checks),'sealed_payloads':n,'source_records':len(basis),'design_only':True}))
