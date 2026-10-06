"""Records-only: corpus mutants against I69's new and changed Python tests (arch copy only; restored
and sha-checked after each). Usage: test_mutants.py P_ROOT RUNNER"""
import hashlib, json, subprocess, sys
from pathlib import Path
P = Path(sys.argv[1]); RUN = sys.argv[2:]
path = P / "fixtures/results/retained_precision_cases.json"
pristine = path.read_bytes(); sha = hashlib.sha256(pristine).hexdigest()
def by_id(items, i): return next(x for x in items if x["id"] == i)
def tm1(c):  # one body-0 row value moved by one ulp in the sparse base
    import math
    r = by_id(c["cases"], "u8_l0_isolated_node_sparse_interactive")["source"]["results"][1]
    r["value"] = math.nextafter(r["value"], math.inf)
def tm2(c):  # an integral float in place of an integer: same value under D25/D32, different JSON
    by_id(c["cases"], "u8_l0_isolated_node_dense_scrutiny")["source"]["retained_precision"]["body"]["receipt_version"] = 1.0
def tm3(c):
    by_id(c["cases"], "u8_l0_isolated_node_sparse_interactive")["provenance"]["u8_head"] = "b1e2d7741e"
def tm4(c):
    c["provenance"]["claim"] = "Arithmetic and reader controls only; no producer execution or native Current evidence."
def tm5(c):
    c["must_pass"].pop()
def tm6(c):
    m = c["mutations"]; m[278], m[279] = m[279], m[278]
MUTANTS = {"TM1_base_value_one_ulp": tm1, "TM2_integral_float_receipt_version": tm2, "TM3_provenance_head": tm3,
           "TM4_claim_reverted": tm4, "TM5_must_pass_dropped": tm5, "TM6_appended_order_swapped": tm6}
try:
    for name, fn in MUTANTS.items():
        c = json.loads(pristine); fn(c); path.write_text(json.dumps(c, indent=2) + "\n")
        out = subprocess.run(RUN + ["-k", "producer_solved_bases or snapshot_07_counts"], capture_output=True, text=True)
        failed = [l.split(" ")[1] for l in out.stdout.splitlines() if l.startswith("FAILED ")]
        print(json.dumps({"mutant": name, "rc": out.returncode, "failed": [f.split("::")[-1] for f in failed]}))
finally:
    path.write_bytes(pristine)
    assert hashlib.sha256(path.read_bytes()).hexdigest() == sha
    print("restored", sha)
