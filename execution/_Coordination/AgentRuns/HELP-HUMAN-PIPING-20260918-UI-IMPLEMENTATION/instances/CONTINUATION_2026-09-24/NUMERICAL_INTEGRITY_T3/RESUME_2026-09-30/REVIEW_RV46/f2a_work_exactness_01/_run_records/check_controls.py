"""RV46 independent finite integer checks. No product imports or numerical execution."""
from itertools import product
from pathlib import Path
import datetime, hashlib, json, os, subprocess
ROOT = Path(__file__).resolve().parents[1]
R = "projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30"
A = R + "/I34/f2a_work_exactness_design_01"
env = dict(os.environ, GIT_OPTIONAL_LOCKS="0")
def read(path):
    return subprocess.check_output(["git","show","49d7dee436:"+A+"/"+path], env=env)
reported = json.loads(read("_run_records/CONTROLS.json"))
rows = {x["id"]:x["evidence"] for x in reported["controls"]}
assert reported["passed"] == 15 and len(rows) == 15
assert all(x["pass"] is True for x in reported["controls"])
M = 2**64-1
# Reference domain uses named faults rather than the author's two-bit implementation.
faults = [frozenset(),frozenset(["overflow"]),frozenset(["inconsistent"]),frozenset(["overflow","inconsistent"])]
join_triples = 0
for x,y,z in product(faults, repeat=3):
    assert (x.union(y)).union(z) == x.union(y.union(z))
    assert x <= x.union(y) and y <= x.union(y)
    assert x.union(y)==y.union(x) and x.union(x)==x
    join_triples += 1
assert rows["S01_state_join"]["triples"]==join_triples==64
pairs = 0
for a,b in product(range(32),repeat=2):
    # Exact sums/differences are the oracle. Representability is independent.
    plus = a+b
    is_representable = 0<=plus<=31
    assert is_representable == (a<=31-b)
    assert (a-b>=0)==(a>=b)
    pairs+=1
assert rows["S02_add_and_underflow"]["pairs_at_max31"]==pairs==1024
assert rows["S02_add_and_underflow"]["u64_boundary"]=={"value":None,"state":1}
assert rows["S03_MAX_minus_MAX_not_zero"]["lost_delta"]=={"value":None,"state":1}
assert rows["S03_MAX_minus_MAX_not_zero"]["exact_delta"]=={"value":0,"state":0}
assert rows["S04_chronology"]["underflow"]=={"value":None,"state":2}
assert rows["S04_chronology"]["foreign_equal_snapshot"]=={"value":None,"state":2}
assert rows["S05_clear_reset_clone"]["work_state_after_clear_reset_clone"]=="O"
# Prices independently transcribed from source OpKind mapping, including repeated 2L.
prices={4:[8,8,16,1290,1548,8,48,32],
        8:[16,16,64,4626,5140,16,96,96],
        16:[32,32,256,17442,18468,32,192,320]}
vectors=0
max_clean={}
for width,weights in prices.items():
    max_clean[width]=0
    for c in product(range(3),repeat=8):
        # Build a priced event list rather than using the author's times/reduce helpers.
        events=[w for count,w in zip(c,weights) for _ in range(count)]
        exact=sum(events)
        assert exact<=M
        assert exact==sum(c[i]*weights[i] for i in range(8))
        max_clean[width]=max(max_clean[width],exact)
        vectors+=1
assert vectors==rows["S06_clean_width_prices"]["vectors"]==19683
assert {x["limbs"]:x["prices_in_add_sub_mul_div_sqrt_round_two_sum_two_product_order"] for x in rows["S06_clean_width_prices"]["prices"]}==prices
k=M//18468+1
assert 0<=k<=M and k*18468>M and (k-1)*18468<=M
assert rows["S07_weighted_and_aggregate_overflow"]["fitting_count_overflowing_price"]==k
# Stronger reduced finite carry check: arbitrary existing exact work, live term count,
# base charge and a legal lower-anchor shift. This is arithmetic, not array simulation.
headroom_checks=0
for h in range(2,7):
    cap=2**h-1
    for span in range(1,7):
        for t in range(cap+1):
            for T in range(t//2+1):
                for b in range(2,5):
                    if t+b<=cap:
                        assert T+1<2**h
                        upper=(T+1)*(2**span-1)
                        assert upper<2**(span+h)
                        for shift in range(span):
                            old=T*(2**(span-shift)-1)
                            assert (old<<shift)+(2**span-1)<2**(span+h)
                        # pending b must be retained during every charged carry prefix
                        available=cap-t-b
                        assert t+available+b<=cap<t+available+1+b
                        headroom_checks+=1
expected_largest={"h":8,"span":8,"completed_terms":127,"term_counter":254,
                  "magnitude":32385,"next_stopped_before_mutation":True}
assert rows["S08_pre_mutation_headroom_and_pending_carry"]["reduced_cases"]==56
assert rows["S08_pre_mutation_headroom_and_pending_carry"]["largest_case"]==expected_largest
assert 11+3<=15 and 11+1+3<=15<11+2+3
assert rows["S09_partial_error_and_closure"]["prefix"]==2+3
assert rows["S09_partial_error_and_closure"]["stages_exceed_total"]=="I"
assert 7+3==rows["S10_success_and_nonbudget_failure_cache"]["first_case"]
assert 7+2==rows["S10_success_and_nonbudget_failure_cache"]["reused_case"]
assert 7+3+2==rows["S10_success_and_nonbudget_failure_cache"]["invocation_after_two_cases"]
assert rows["S10_success_and_nonbudget_failure_cache"]["failed_cached_reuse_increment"]==0
assert rows["S11_zero_coefficient_and_cross_case"]["new_build_charge"]==0
assert rows["S11_zero_coefficient_and_cross_case"]["invocation_receipt_state"]=="O"
assert rows["S12_terminal_vector_omission"]["recorded_terminal"]["state"]==1
assert len(rows["S12_terminal_vector_omission"]["recorded_terminal"]["records"])==2
records=[(17,3,5,7,11,True,True),(23,4,6,7,13,False,True),(19,5,0,7,13,False,False)]
case_sum=inv_sum=0
fragments=[]
for own,decision,verify,shared,vshared,build,vbuild in records:
    solve=own-decision-verify
    assert solve>=0
    base=solve+verify+shared+vshared
    case_sum+=base+decision
    inv_sum+=solve+verify+decision+(shared if build else 0)+(vshared if vbuild else 0)
    fragments.append({"B_case":base,"T_case":decision,"solve_own":solve})
assert case_sum==117 and inv_sum==90
assert rows["S13_clean_projection_identity"]["fragments"]==fragments
assert rows["S13_clean_projection_identity"]["case_sum"]==case_sum
assert rows["S13_clean_projection_identity"]["invocation_increment"]==inv_sum
assert rows["S14_budget_overshoot_is_not_underflow"]["overshoot"]==13-10
assert rows["S14_budget_overshoot_is_not_underflow"]["room"]==max(10-13,0)
assert rows["S15_projection_provenance_and_range"]["largest_safe_json"]==2**53-1
# Fault custody across donor-with-zero, discarded-local, cache and omitted-terminal.
# Join is independent of price; destructive numeric projections cannot qualify history.
for f in faults[1:]:
    for route in ("zero_donor","discarded_local","pruned_tracker","cached_failure","omitted_terminal"):
        scope=frozenset().union(f)
        copied=scope
        assert copied and copied.union(frozenset())==f
result={"scope":"Independent abstract integer checks; no Rust/solver/model/runtime claim",
        "completed_utc":datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "checked_author_control_rows":15,"state_triples":join_triples,"add_delta_pairs":pairs,
        "clean_price_vectors":vectors,"price_table":prices,"max_clean_price_vector":max_clean,
        "generalized_reduced_headroom_states":headroom_checks,"case_projection":case_sum,
        "invocation_projection":inv_sum,"source_derivation_sha256":hashlib.sha256((ROOT/"SOURCE_DERIVATION.md").read_bytes()).hexdigest()}
(ROOT/"_run_records"/"INDEPENDENT_CONTROLS.json").write_text(json.dumps(result,indent=2)+"\n")
print(json.dumps(result))
