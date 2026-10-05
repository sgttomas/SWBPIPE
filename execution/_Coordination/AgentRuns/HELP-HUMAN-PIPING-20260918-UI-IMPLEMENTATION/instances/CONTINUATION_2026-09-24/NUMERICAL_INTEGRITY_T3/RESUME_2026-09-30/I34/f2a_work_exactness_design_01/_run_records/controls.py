"""Finite abstract work-state checks only. No product imports or solver execution.
Run from explicit repository cwd; writes only adjacent CONTROLS.json.
None means an unavailable exact amount, not a selected Rust representation.
"""
from __future__ import annotations
from dataclasses import dataclass, asdict
from itertools import product
from pathlib import Path
import json

M = (1 << 64) - 1
SAFE_JSON = (1 << 53) - 1
E, O, I = 0, 1, 2

@dataclass(frozen=True)
class C:
    value: int | None
    state: int = E
    def __post_init__(self):
        assert 0 <= self.state <= 3
        assert (self.value is not None) == (self.state == E)
        assert self.value is None or self.value >= 0

def combine_state(c, state):
    s = c.state | state
    return C(None if s else c.value, s)

def add(a, b, maximum=M):
    state = a.state | b.state
    if state:
        return C(None, state)
    n = a.value + b.value
    return C(n) if n <= maximum else C(None, O)

def times(a, coefficient, maximum=M):
    assert coefficient >= 0
    if a.state:
        return a  # zero is deliberately no status eraser
    n = a.value * coefficient
    return C(n) if n <= maximum else C(None, O)

def delta(after, before, same_stream=True):
    s = after.state | before.state | (0 if same_stream else I)
    if s:
        return C(None, s)
    if after.value < before.value:
        return C(None, I)
    return C(after.value - before.value)

def room(limit, used):
    if used.state:
        return C(None, used.state)
    return C(max(limit - used.value, 0))

def total(values, maximum=M):
    out = C(0)
    for v in values:
        out = add(out, v, maximum)
    return out

def price(l):
    return [2*l, 2*l, l*l, (l+1)*(64*l+2), (l+2)*(64*l+2), 2*l, 12*l, l*l+4*l]

def snapshot(c):
    return C(c.value, c.state)

def emit(c, provenance=True):
    return provenance and c.state == E and c.value <= SAFE_JSON

rows = []
def check(name, assertions, evidence):
    assert all(assertions), name
    rows.append({"id": name, "pass": True, "evidence": evidence})

laws = 0
for a, b, c in product(range(4), repeat=3):
    assert (a | b) | c == a | (b | c)
    assert a | b == b | a
    assert a | a == a
    assert a | b >= a and a | b >= b
    laws += 1
check("S01_state_join", [laws == 64], {"triples": laws, "states": ["E", "O", "I", "OI"]})

exhaustive = 0
for a, b in product(range(32), repeat=2):
    r = add(C(a), C(b), 31)
    assert (r == C(a+b)) if a+b <= 31 else (r == C(None, O))
    d = delta(C(a), C(b))
    assert (d == C(a-b)) if a >= b else (d == C(None, I))
    exhaustive += 1
check("S02_add_and_underflow", [True], {"pairs_at_max31": exhaustive, "u64_boundary": asdict(add(C(M), C(1)))})

lost = add(C(M), C(1))
cloned = snapshot(lost)
check("S03_MAX_minus_MAX_not_zero", [delta(cloned, lost) == C(None, O), delta(C(M), C(M)) == C(0)],
      {"legacy_saturated_subtraction": 0, "lost_delta": asdict(delta(cloned, lost)), "exact_delta": asdict(delta(C(M), C(M)))})
check("S04_chronology", [delta(C(2), C(3)) == C(None, I), delta(C(7), C(7), False) == C(None, I), delta(lost, C(7), False) == C(None, O|I)],
      {"underflow": asdict(delta(C(2), C(3))), "foreign_equal_snapshot": asdict(delta(C(7), C(7), False))})
check("S05_clear_reset_clone", [snapshot(lost) == lost, combine_state(C(0), lost.state) == lost],
      {"numeric_value_after_clear": 0, "work_state_after_clear_reset_clone": "O", "new_independent_owner": asdict(C(0))})

price_rows = []
clean_count = 0
for l in (4, 8, 16):
    p = price(l)
    for counts in product(range(3), repeat=8):
        exact = sum(n*q for n, q in zip(counts, p))
        checked = total(times(C(n), q) for n, q in zip(counts, p))
        legacy = min(M, exact)
        assert checked == C(legacy)
        clean_count += 1
    price_rows.append({"limbs": l, "prices_in_add_sub_mul_div_sqrt_round_two_sum_two_product_order": p})
check("S06_clean_width_prices", [clean_count == 19683], {"vectors": clean_count, "prices": price_rows})
check("S07_weighted_and_aggregate_overflow", [times(C(M//18468+1),18468).state == O, add(C(M-2),C(3)).state == O],
      {"fitting_count_overflowing_price": M//18468+1, "price": 18468, "component_values_fit": [M-2,3]})

# Reduced-width analog of the headroom proof. Counter capacity has h bits,
# individual terms have at most span bits. base=2 is add_raw's smallest charge.
# No float, numerical model or production accumulator is executed.
headroom_rows = []
for h in range(2, 9):
    maximum = (1 << h) - 1
    for span in range(1, 9):
        term = (1 << span) - 1
        t, n, magnitude = 0, 0, 0
        while t + 2 <= maximum:
            assert n <= t
            assert n + 1 <= t + 1 <= maximum
            proposed = magnitude + term
            assert proposed < (1 << (span+h))
            magnitude = proposed
            t += 2
            n += 1
        before = magnitude
        stopped = t+2 > maximum
        assert stopped and magnitude == before
        # A safe downward-anchor move preserving the span ceiling.
        if span >= 2:
            old = n * ((1 << (span-1))-1)
            assert (old << 1) < (1 << (span+h))
        headroom_rows.append({"h": h, "span": span, "completed_terms": n, "term_counter": t, "magnitude": magnitude, "next_stopped_before_mutation": True})
# Pending base must remain reserved while a carry event is checked.
maximum, t, base, carry_events = 15, 11, 3, 0
assert t + base <= maximum
assert t + 1 + base <= maximum
carry_events += 1
assert t + carry_events + 1 + base > maximum
check("S08_pre_mutation_headroom_and_pending_carry", [len(headroom_rows)==56],
      {"reduced_cases": len(headroom_rows), "largest_case": headroom_rows[-1], "pending_carry_boundary": {"maximum": maximum, "incoming": t, "base_reserved": base, "successful_carry_charges": carry_events, "next_carry_refused": True}, "u64_symbolic": "T<=t; t+b<=2^64-1 and b>=2 imply T+1<2^64; each magnitude<2^(8128+64)"})

# Exact ordinary failures retain the incurred prefix. An accounting failure has
# no invented exact total even if the payload retains a diagnostic prefix.
exact_error_prefix = total([C(2),C(3)])
bad_error = combine_state(exact_error_prefix, O)
check("S09_partial_error_and_closure", [exact_error_prefix == C(5), not emit(bad_error), delta(C(4),C(5)).state==I],
      {"unchanged_numeric_reason": "existing arithmetic refusal", "prefix": 5, "accounting_state": "O", "stages_exceed_total": "I", "saturated_lower_bound_claim": False})

shared, first_own, next_own = C(7), C(3), C(2)
first_case = add(shared, first_own)
next_case = add(shared, next_own)
invocation = add(first_case, next_own)
failed_cache = C(5)
check("S10_success_and_nonbudget_failure_cache", [first_case==C(10), next_case==C(9), invocation==C(12), times(failed_cache,0)==C(0)],
      {"first_case":10,"reused_case":9,"invocation_after_two_cases":12,"failed_build_first_charge":5,"failed_cached_reuse_increment":0,"failed_cached_case_charge":5,"budget_failure_cached":False})

# Actual reused-build increment is 0; receipt validity still joins the dependency.
physical_increment = C(0)
inv_bad = combine_state(add(C(12), physical_increment), lost.state)
check("S11_zero_coefficient_and_cross_case", [times(lost,0)==lost, inv_bad.state==O, not emit(inv_bad), snapshot(inv_bad)==inv_bad],
      {"new_build_charge":0,"case_or_cache_state":"O","invocation_receipt_state":"O","next_case_may_reset_invocation":False,"previous_selected_numeric_standing_changed":False})

terminal = {"reason":"Structure", "records":[{"work":asdict(C(5))},{"work":asdict(lost)}], "state":O}
legacy_vector = []
check("S12_terminal_vector_omission", [terminal["state"]==O, len(terminal["records"])==2, inv_bad.state==O],
      {"recorded_terminal": terminal, "legacy_omitted_vector":legacy_vector, "legacy_view_qualifies_exact_receipt":False,"pre_source_refusal_invents_run":False})

# C1 physical records: O=own, D=stop/certification, Q=verification own,
# S=shared, V=verification shared; coefficients indicate actual builds.
records = [(17,3,5,7,11,1,1),(23,4,6,7,13,0,1),(19,5,0,7,13,0,0)]
case_sum=0
inv_sum=0
fragments=[]
for own,d,q,s,v,bs,bv in records:
    assert d+q <= own
    solve_own=delta(delta(C(own),C(d)),C(q))
    B=own-d+s+v
    T=d
    assert B+T==own+s+v
    case_sum+=B+T
    inv_sum+=own+bs*s+bv*v
    fragments.append({"B_case":B,"T_case":T,"solve_own":solve_own.value})
check("S13_clean_projection_identity", [case_sum==117, inv_sum==90],
      {"records":records,"fragments":fragments,"case_sum":case_sum,"invocation_increment":inv_sum,"prices_or_numeric_values_changed":False})
check("S14_budget_overshoot_is_not_underflow", [room(10,C(13))==C(0), room(10,lost).state==O],
      {"limit":10,"actual_spent":13,"overshoot":3,"room":0,"status":"E","priority_when_both_over":"Case"})
check("S15_projection_provenance_and_range", [emit(C(SAFE_JSON)), not emit(C(SAFE_JSON+1)), not emit(C(M)), not emit(C(0),False), not emit(C(None,I))],
      {"largest_safe_json":SAFE_JSON,"exact_native_MAX_is_not_saturation":True,"unknown_is_not_zero":True})

out = {"scope":"abstract state/exact integer controls; no Rust, compiler, solver, model, runtime or host witness", "passed":len(rows), "controls":rows}
Path(__file__).with_name("CONTROLS.json").write_text(json.dumps(out,indent=2)+"\n")
print(json.dumps({"passed":len(rows),"clean_price_vectors":clean_count,"headroom_cases":len(headroom_rows)}))
