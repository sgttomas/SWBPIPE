"""RV46 API-02: read-only Git identity and bounded exact abstract backchecks."""
from pathlib import Path
from itertools import product
import datetime,hashlib,json,os,subprocess
out=Path(__file__).resolve().parents[1]
env=dict(os.environ,GIT_OPTIONAL_LOCKS="0")
def git(*args):return subprocess.check_output(["git",*args],env=env)
A="projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I34/f2a_work_exactness_api_02"
rev=git("rev-parse","45d9a2d75a").decode().strip()
base=git("rev-parse","49034a940f").decode().strip()
def author(path):return git("show",rev+":"+A+"/"+path)
inv=json.loads(author("WRITE_INVENTORY.json"))
paths=git("ls-tree","-r","--name-only",rev,A).decode().splitlines()
assert len(paths)==8
assert sorted(paths)==sorted(git("diff","--name-only",rev+"^",rev).decode().splitlines())
for e in inv["payloads"]:
 b=author(e["path"]);assert len(b)==e["bytes"] and hashlib.sha256(b).hexdigest()==e["sha256"]
p="projects/chirality-piping"
assert not git("diff","--name-only",base,rev,"--",p+"/core",p+"/apps",p+"/schemas",p+"/fixtures",p+"/tests",p+"/validation")
prior="projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I34/f2a_work_exactness_design_01"
assert not git("diff","--name-only","49d7dee436",rev,"--",prior)
reported=json.loads(author("_run_records/CONTROLS.json"))
assert reported["passed"]==12 and len(reported["controls"])==12
assert all(c["pass"] for c in reported["controls"])
C={c["id"]:c["result"] for c in reported["controls"]}
M=2**64-1
# Owner identity and arithmetic admissibility are independent conditions.
same_count=foreign_count=0
for a,b in product(range(32),repeat=2):
 for same in (True,False):
  valid=same and a>=b
  if valid:
   assert 0<=a-b<=31;same_count+=1
  elif not same:foreign_count+=1
assert C["A01_stream_identity"]=={"foreign_equal":"I","exact_same_equal":0,"lost_equal":"O"}
base_count=M-500
assert base_count+128<=M<base_count+128+384
assert base_count+384<=M
assert C["A02_persistent_clone"]["before"]==base_count
assert C["A02_persistent_clone"]["persistent_clone_round"]=="overflow"
# Shift is charged to the aggregate rather than incorrectly to term work;
# both totals reserve pending b. Source proof distinguishes these components.
term=25;shift=0;pending=4;cap=31
assert term+pending<=cap and term+shift+pending<=cap
shift+=1
assert term+shift+1+pending<=cap
term+=1
assert term+shift==27 and term+shift+1+pending>cap
assert C["A03_pending_base"]["committed_prefix"]==27
assert C["A03_pending_base"]["next_carry_performed"] is False
assert C["A04_poisoned_clear"]["value_reset"]=="full"
assert C["A04_poisoned_clear"]["work_flag"]=="O"
assert C["A05_compound_poison"]["destination_poisoned"] is True
assert C["A06_prior_and_no_escalation"]["condition_with_O"]["prior"]=="Condition"
assert C["A06_prior_and_no_escalation"]["span_with_I"]["prior"]=="Span"
assert C["A07_zero_price_status"]["actual_new_increment"]==0
assert C["A07_zero_price_status"]["invocation_receipt_state"]=="O"
assert C["A08_c2_pre_source"]["call"]["run"] is None
assert C["A08_c2_pre_source"]["call"]["before"]==C["A08_c2_pre_source"]["call"]["after"]
expected_helpers={"chord_d":1152,"chord_c":2688,"determinant_sum":12416,
 "determinant_c128":192,"determinant_c192":384,"intensified_sum":1664,
 "row_bound_num_clone":2048,"row_bound_den_clone":896,"row_bound_context":17506,
 "fresh_correction_scratch":2688,"binary64_up":1152}
assert C["A09_closed_fresh_helpers"]==expected_helpers
assert max(expected_helpers.values())<2**18
expected_prices={4:[8,8,16,1290,1548,8,48,32],8:[16,16,64,4626,5140,16,96,96],16:[32,32,256,17442,18468,32,192,320]}
assert {r["L"]:r["prices"] for r in C["A10_pinned_prices"]}==expected_prices
assert C["A11_view_gate"]["H_work_lines_on_non_E"] is False
assert C["A11_view_gate"]["VR_record_on_non_E"] is None
assert M>2**53-1 and C["A12_exact_range"]["automatic_lower_bound"] is False
all_paths=set(git("ls-tree","-r","--name-only",base,p+"/core/solver/frame_kernel",p+"/core/solver/performance_harness",p+"/validation/benchmarks/numerical_robustness").decode().splitlines())
wrong=p+"/core/solver/frame_kernel/tests/structural/formation_check_tests.rs"
right=p+"/core/solver/frame_kernel/src/structural/formation_check_tests.rs"
assert wrong not in all_paths and right in all_paths
construction_scan=git("grep","-n","-E","(StageWork|AttemptRecord|SumWork|WidthWork|InvocationMeter|W1Solve|CaseRun) *[{]",base,"--",p+"/core",p+"/validation/benchmarks/numerical_robustness").decode()
(out/"_run_records"/"PUBLIC_CONSTRUCTION_SCAN.txt").write_text(construction_scan)
# No source trees copied; preserve blob identities for the actual consulted sources.
files=[
"core/solver/frame_kernel/src/structural/retained/"+f for f in
["wide_sum.rs","wide/multi.rs","adaptive.rs","verify.rs","bound.rs","factor.rs","directed.rs","source.rs","assemble.rs","recover.rs","ledger.rs","wide.rs"]]
files += ["core/solver/frame_kernel/src/structural.rs",
"core/solver/frame_kernel/src/structural/formation_check.rs",
"core/solver/frame_kernel/src/structural/formation_check_tests.rs",
"core/solver/performance_harness/src/k6/w1/staged.rs",
"core/solver/performance_harness/src/bin/k6_observe/w1.rs",
"core/solver/performance_harness/src/bin/k6_observe/main.rs",
"validation/benchmarks/numerical_robustness/src/records.rs",
"validation/benchmarks/numerical_robustness/src/lane.rs",
"validation/benchmarks/numerical_robustness/examples/vk_records.rs",
"validation/benchmarks/numerical_robustness/examples/vk_scale.rs"]
refs=[(base,p+"/"+f) for f in files]+[(rev,path) for path in paths]
refs += [(rev,"projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I32/f2a_wire_c2/CONTRACT_DELTA.md"),
("221adf0202","projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/REVIEW_RV46/f2a_work_exactness_01/REVIEW.md")]
origins=[]
for r,path in refs:
 b=git("show",r+":"+path)
 origins.append({"revision":git("rev-parse",r).decode().strip(),"path":path,"blob":git("rev-parse",r+":"+path).decode().strip(),"bytes":len(b),"sha256":hashlib.sha256(b).hexdigest()})
(out/"_run_records"/"ORIGINS.json").write_text(json.dumps(origins,indent=2)+"\n")
result={"candidate":rev,"source":base,"verified_files":8,"verified_payloads":len(inv["payloads"]),
"candidate_diff_only_packet":True,"maintained_diff_empty":True,"prior_design_unchanged":True,
"checked_abstract_rows":12,"same_owner_valid_pairs":same_count,"foreign_pairs_rejected":foreign_count,
"helper_bounds":expected_helpers,"price_rows":expected_prices,
"manifest_wrong_path":wrong,"manifest_actual_path":right,
"author_script_executed":False,"no_compiled_or_source_execution":True,
"completed_utc":datetime.datetime.now(datetime.timezone.utc).isoformat()}
(out/"_run_records"/"CHECKS.json").write_text(json.dumps(result,indent=2)+"\n")
print(json.dumps(result))
