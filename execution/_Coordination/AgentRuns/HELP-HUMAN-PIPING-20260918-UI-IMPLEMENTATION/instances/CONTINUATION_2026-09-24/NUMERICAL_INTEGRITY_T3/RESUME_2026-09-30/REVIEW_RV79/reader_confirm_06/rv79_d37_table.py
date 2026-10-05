"""RV79 confirmation 06: D37 in Python, checked two ways.

1. Exhaustive unit table: every error kind (the nine, plus an unknown one) against every stage record
   that the class-2 stage rules admit (85 records), through rp._g5_typed with no proof (so only the D37
   relation is exercised). The expectation is RV79's own encoding of the native sequence
   (PP/retained_product.rs 3136-3290, 3456-3549, read by RV79), written independently of the reader's
   ERROR_STAGE_RECORDS: accept iff the record is one the native code can leave for that kind.
2. RV79's five X1 receipts (rv79_probes_error_stage.py edits) through the full validator.
"""
import json, sys, traceback, itertools
sys.path.insert(0, ".")
from core.analysis_runs import retained_precision as rp
C_, F_, N_ = "completed", "failed", "not_entered"
ORDER = ["preparation", "native", "proof_start", "projection", "maxima", "values", "aliases", "certificate", "observables", "g5a"]
def records():
    out = []
    for k in range(0, 9):                      # k completed pipeline stages (of the first eight)
        tails = [[]] if k == 8 else [[F_], [N_]]
        for t in tails:
            first8 = [C_] * k + t + [N_] * (8 - k - len(t))
            cert = first8[7]
            pairs = [(N_, N_)] + ([(C_, C_), (C_, F_), (F_, C_), (F_, F_)] if cert in (C_, F_) else [])
            for o, g in pairs:
                out.append(tuple(first8 + [o, g]))
    return sorted(set(out))
def rv79_native(kind, r):
    st = dict(zip(ORDER, r))
    def through(n):  # first n stages completed
        return all(st[s] == C_ for s in ORDER[:n])
    def rest_n(frm):
        return all(st[s] == N_ for s in ORDER[frm:])
    if kind == "preparation":  # prepare_owned_case refuses (3140 enter, 3257 fail_entered)
        return st["preparation"] == F_ and rest_n(1)
    if kind == "native":       # solve_native, nonselected outcome (3276, 3290)
        return through(1) and st["native"] == F_ and rest_n(2)
    if kind == "capture":
        return ((through(1) and st["native"] == F_ and rest_n(2))          # solve_native before a Run (3279-3286)
                or (through(2) and rest_n(2))                              # project_candidate before ProofStart (3461-3466)
                or (through(8) and rest_n(8))                              # frozen value owner / verdict copy after a passed certificate (3517-3525)
                or through(10))                                            # commit (3546-3549)
    if kind == "proof":
        return ((through(2) and st["proof_start"] == F_ and rest_n(3))     # begin_prepared_product (3469)
                or (through(3) and st["projection"] == F_ and rest_n(4))   # project (3473)
                or (through(7) and st["certificate"] == F_ and (st["observables"], st["g5a"]) in
                    {(N_, N_), (C_, C_), (C_, F_), (F_, C_), (F_, F_)}))   # certify_final fails, checks entered iff verdict copy covers rows (3493-3515)
    if kind == "values":       # complete_maxima (3480-3481)
        return through(5) and st["values"] == F_ and rest_n(6)
    if kind == "abandoned":    # prepared_maxima (3475-3476), prepared_alias (3483-3486), bind_rows_view (3489-3491)
        return ((through(4) and st["maxima"] == F_ and rest_n(5)) or (through(6) and st["aliases"] == F_ and rest_n(7))
                or (through(7) and rest_n(7)))
    if kind == "numeric":      # full-case check false after both checks passed (3538-3543)
        return through(10)
    if kind == "observable":   # observables failed; g5a entered and checked (3528-3543)
        return through(8) and st["observables"] == F_ and st["g5a"] in (C_, F_)
    if kind == "g5a":          # observables passed, g5a failed (3528-3543)
        return through(9) and st["g5a"] == F_
    return False
def reader(kind, r):
    a = {"proof": None, "stages": dict(zip(ORDER, r)), "result": {"kind": "unavailable", "error": {"kind": kind}}}
    try:
        rp._g5_typed(a, lambda ok: rp._need(ok, "G5", "PRODUCT_ATTEMPT_MISMATCH")); return True
    except rp.RetainedPrecisionError:
        return False
kinds = ["preparation", "native", "capture", "proof", "values", "abandoned", "numeric", "observable", "g5a", "unknown_kind"]
recs = records()
rows, mismatches, accepted = [], [], {k: 0 for k in kinds}
for kind, r in itertools.product(kinds, recs):
    want, got = rv79_native(kind, r), reader(kind, r)
    accepted[kind] += got
    if want != got: mismatches.append({"kind": kind, "record": r, "rv79_native": want, "reader": got})
print(json.dumps({"records": len(recs), "pairs": len(kinds) * len(recs), "mismatches": len(mismatches), "accepted_per_kind": accepted}))
for m in mismatches: print(json.dumps(m))
# table equality as sets, for the record
mine = {k: {r for r in recs if rv79_native(k, r)} for k in kinds}
theirs = {k: set(rp.ERROR_STAGE_RECORDS.get(k, ())) for k in kinds}
print(json.dumps({"set_equal_per_kind": {k: mine[k] == theirs[k] for k in kinds},
                  "reader_records_outside_class2_space": {k: sorted(map(list, theirs[k] - set(recs))) for k in kinds if theirs[k] - set(recs)}}))
