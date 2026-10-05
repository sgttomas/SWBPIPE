# RV48: exact integer/source review evidence only. No model, compiler or solver.
import contextlib
import hashlib
import io
import itertools
import json
import math
import os
from pathlib import Path
import subprocess

# Resolve from the explicit invocation cwd; do not discover/change another checkout.
ROOT = Path.cwd()
OUT = Path(__file__).resolve().parent
R = "projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/"
CAND = "8eaca35bdc1eedc46e8c3b8fb612f1b304399d4a"
SOURCE = "49034a940f3f8cd3f3da4d4cbc839943b808063d"
ENV = dict(os.environ, GIT_OPTIONAL_LOCKS="0")
def blob(rev, path):
    return subprocess.check_output(["git", "show", rev + ":" + path], cwd=ROOT, env=ENV)
def sha(data):
    return hashlib.sha256(data).hexdigest()
rows = []
def check(name, predicate, evidence):
    assert predicate, (name, evidence)
    rows.append({"check": name, "passed": True, "evidence": evidence})

packet = R + "I29/f2a_preparation_counts_p1/"
inventory = json.loads(blob(CAND, packet + "WRITE_INVENTORY.json"))
for item in inventory["files"]:
    data = blob(CAND, packet + item["path"])
    check("frozen payload digest: " + item["path"], len(data) == item["bytes"] and sha(data) == item["sha256"], {"bytes": len(data), "sha256": sha(data)})
# Execute the inspected author script with a synthetic __file__ inside this review.
# No author-directory write and no tracked-source copy is retained.
rerun = OUT / "author_reproduction"
rerun.mkdir(exist_ok=True)
author_code = blob(CAND, packet + "_run_records/count_checks.py")
author_stdout = io.StringIO()
author_ns = {"__file__": str(rerun / "count_checks.py"), "__name__": "__main__"}
with contextlib.redirect_stdout(author_stdout):
    exec(compile(author_code, "frozen-author-count-checks", "exec"), author_ns)
actual = (rerun / "COUNT_CHECKS.json").read_bytes()
expected = blob(CAND, packet + "_run_records/COUNT_CHECKS.json")
check("author's 16 checks reproduced byte-for-byte in reviewer scope", actual == expected, {"stdout": author_stdout.getvalue().strip(), "sha256": sha(actual), "author_script_sha256": sha(author_code)})

bad_input_returns = {
    "validated_bool_n": author_ns["validated"](True,0,0,0,0,0,0,0),
    "validated_negative_k": author_ns["validated"](1,0,0,-1,0,0,0,0),
    "combo_compensated_negative_load_count": author_ns["combo"]([38,38],[-1,1],0),
}
check("malformed scalar helper inputs are accepted (confirmed tooling gap)", bad_input_returns["validated_bool_n"]["N"]==6 and bad_input_returns["validated_negative_k"]["F"]==7 and bad_input_returns["combo_compensated_negative_load_count"]["load_visits"]==0, bad_input_returns)

widths = {
    "source_header_and_counts": (6 + 8*4, 38),
    "stiffness_header_and_counts": (6 + 5*4, 26),
    "node": (3*8, 24),
    "member": (3*4 + 9*8, 84),
    "axis_spring": (4 + 4 + 1 + 8, 17),
    "constraint_with_value": (4 + 1 + 8, 13),
    "constraint_stiffness_only": (4 + 1, 5),
    "nodal_term_before_source_id_bytes": (4 + 1 + 4 + 8, 17),
    "station": (2*4 + 8, 16),
    "support_before_child_ids": (2*4 + 6 + 2*4, 22),
    "ledger_entry_before_limbs": (4 + 1 + 1 + 8 + 4, 18),
    "ledger_or_combination_header": (6 + 4, 10),
    "combination_operand_prefix": (8 + 4, 12),
}
check("field-by-field native encoding widths", all(a == b for a,b in widths.values()), widths)
scalar_cases = 0
for n,m,s,k,l,g in itertools.product(range(1,5),range(5),range(4),range(7),range(4),range(3)):
    if k > 6*n:
        continue
    t = 3*m
    check_q = 6*n+n+2*6*m+6*t+s+k+2*g
    assert check_q == 7*n+30*m+s+k+2*g
    I, Cs = 2*l, s*g
    field_total = 6+5*4+24*n+84*m+17*s+13*k + 4+17*l+I + 4+16*t + 4+22*g+4*Cs
    assert field_total == 38+24*n+84*m+17*s+13*k+17*l+I+16*t+22*g+4*Cs
    scalar_cases += 1
check("independent scalar encoding/layout equalities", scalar_cases == 6720, {"scalar_tuples": scalar_cases, "not_models": True})
upper = sum(1 for a in range(12) for b in range(12) if a <= b)
closure = 2*upper-12
check("12-DOF per-member upper and symmetric counts", (upper,closure)==(78,144), {"upper":upper,"closure":closure})
check("raw duplicate upper is not validated constraint count", 493-402 == 13*(13-6), {"raw_encoding_upper":493,"validated_encoding":402,"raw_r":13,"validated_k":6})
check("ordered repeated combination arithmetic", (10+sum(12+e for e in (123,123,77)),sum((3,3,0)),3*6)==(369,6,18), {"encoding":369,"load_visits":6,"prescription_pairs":18})
U32, U64 = (1<<32)-1, (1<<64)-1
check("station count boundary", 3*(U32//3)<=U32<3*(U32//3+1), {"last_member_count":U32//3,"last_station_id":3*(U32//3)-1})
check("case identity u32 prefix is separate from usize64 encoding", U32+1<=U64 and U32+1>U32, {"operand_bytes":U32+1})
partitions = 0
for alpha,beta,q in itertools.product(range(5),range(1,5),range(5)):
    for parts in itertools.product(range(5),repeat=q):
        assert sum(alpha*(x>0)+beta*x for x in parts)<=alpha*q+beta*sum(parts)
        partitions += 1
check("nonnegative abstract child-capacity majorant", partitions==15620, {"partitions":partitions,"coefficients_are_abstract":True})
check("geometry expansion term count", 2+4*2==10 and 3*10+3==33 and 10+1==11, {"translation":10,"rotation":1,"per_node":33,"exact_scalar_next":11})
product_max = (math.isqrt(1+4*U64)-1)//2
triangle_max = (math.isqrt(1+8*U64)-1)//2
check("unreduced usize64 profile product implies safe block namespace", product_max==U32, {"max_F_if_F_times_Fplus1_fits_usize64":product_max,"max_F_if_only_triangle_fits":triangle_max})
q = U32//6
n,m,s,k,g = q+5,4,6*q,6,q+1
F,f = 6*n-k,6*q+4
witness={"isolated_nodes":q,"four_leaf_fixed_center_star_members":m,"n":n,"s":s,"k":k,"g":g,"F":F,"f":f,"u32_max":U32,"unreduced_profile_product":F*(F+1),"triangle":F*(F+1)//2,"scope":"Scalar/topology description only; no model constructed. This is excluded by an unreduced-product guard, but not by a triangle-only guard."}
check("block sentinel guard distinction", f==U32+1 and max(n,m,s,k,g,3*m)<=U32 and F*(F+1)//2<=U64<F*(F+1), witness)
check("target32 u8 layout boundary", (1<<31)-1 < (1<<31), {"last_isize_byte":(1<<31)-1,"first_forbidden_byte":1<<31,"no_target_profile_chosen":True})
result={"scope":"Exact integer/source evidence only; no numeric byte bound, heap/layout/capacity measurement, model, solver, compiler or runtime probe.","candidate":CAND,"source":SOURCE,"checks":rows,"all_passed":True}
(OUT/"REVIEW_CHECKS.json").write_text(json.dumps(result,indent=2)+"\n")
print(json.dumps({"checks":len(rows),"all_passed":True,"scalar_tuples":scalar_cases,"capacity_partitions":partitions}))
