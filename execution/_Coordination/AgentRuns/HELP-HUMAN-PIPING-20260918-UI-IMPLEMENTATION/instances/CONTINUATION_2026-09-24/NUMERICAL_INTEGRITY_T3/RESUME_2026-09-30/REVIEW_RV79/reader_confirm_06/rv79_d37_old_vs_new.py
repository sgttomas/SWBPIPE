"""RV79 confirmation 06: is any (kind, stage record) pair accepted by the new D37 rule but rejected by the
removed one-direction mapping (old PY:865-871 at 63355a91d2)? That would be a weakening."""
import json, sys, itertools
sys.path.insert(0, ".")
from core.analysis_runs import retained_precision as rp
C_, F_, N_ = "completed", "failed", "not_entered"
ORDER = rp.STAGE_ORDER
def records():
    out = []
    for k in range(0, 9):
        for t in ([[]] if k == 8 else [[F_], [N_]]):
            first8 = [C_] * k + t + [N_] * (8 - k - len(t))
            for o, g in [(N_, N_)] + ([(C_, C_), (C_, F_), (F_, C_), (F_, F_)] if first8[7] in (C_, F_) else []):
                out.append(tuple(first8 + [o, g]))
    return sorted(set(out))
OLD = {"preparation": ("preparation", "capture"), "native": ("native", "capture"), "proof_start": ("proof",), "projection": ("proof",),
       "maxima": ("abandoned",), "values": ("values",), "aliases": ("abandoned",), "certificate": ("proof",)}
def old_rule(kind, r):
    st = dict(zip(ORDER, r)); ff = next((k for k in ORDER[:8] if st[k] == F_), None)
    return True if ff is None else kind in OLD[ff]
def new_rule(kind, r):
    return r in rp.ERROR_STAGE_RECORDS.get(kind, ())
kinds = ["preparation", "native", "capture", "proof", "values", "abandoned", "numeric", "observable", "g5a"]
recs = records()
weakened = [(k, r) for k, r in itertools.product(kinds, recs) if new_rule(k, r) and not old_rule(k, r)]
tightened = [(k, r) for k, r in itertools.product(kinds, recs) if old_rule(k, r) and not new_rule(k, r)]
print(json.dumps({"pairs": len(kinds) * len(recs), "new_accepts_old_rejected": weakened, "old_accepts_new_rejects": len(tightened)}))
