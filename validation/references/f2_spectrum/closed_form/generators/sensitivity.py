"""Item 6: sensitivity of each item 5 response to one mode's frequency.

Reading (stated in the README): mode n's frequency alone is varied, its shape,
its participation and its force-recovery coefficient q_n held fixed (a
conditional SRSS sensitivity). A modal contribution is
  displacement  Q_n = Gamma_n W_n s S(f_n) / w_n^2   ->  d ln Q_n / d ln f_n = sigma_n - 2
  force, moment, reaction  Q_n = Gamma_n q_n s S(f_n) with q_n fixed  ->  d ln Q_n / d ln f_n = sigma_n
where sigma_n = d ln S / d ln f at f_n (0 on a flat segment; on a log-log
segment, ln(S1/S0)/ln(f1/f0)). For the SRSS combination R,
  d ln R / d ln f_n = (Q_n / R)^2 (d ln Q_n / d ln f_n).
A frequency disagreement of relative size e moves R by about e times this.
The closed form is checked by a central difference in decimal.
"""

from decimal import Decimal

import beams as B
import beams_build as BB
import rules
from numerics import dec, dln, sci, val

H = Decimal("1e-20")


def build(cross):
    c = BB.compute()
    real = c["real"]
    out = []
    worst = Decimal(0)
    largest = {}
    for rid, r in real.items():
        cc = r["cont"]["rayleigh"]
        kinds = {q[0]: q[1] for q in B.quantity_list(r["span"])}
        doc = {"id": rid, "model": "rayleigh continuum", "spectra": {}}
        for t in r["tables"]:
            resp = cc.resp[t["id"]]
            sig = [rules.dlnS_dlnf(t, f) for f in cc.freq]
            sd = {"sigma_n = dlnS/dlnf": [val(s) for s in sig], "per_quantity": {}}
            for q in r["qnames"]:
                Q = resp["contrib"][q]
                R2 = sum((x * x for x in Q), Decimal(0))
                e_disp = -2 if kinds[q] == "displacement" else 0
                rows = []
                for n in range(B.N_MODES):
                    e = sig[n] + e_disp
                    d = (Q[n] * Q[n] / R2) * e
                    # central difference: perturb f_n by (1 +- H) through the table rule
                    lr = []
                    for sgn in (1, -1):
                        fp = cc.freq[n] * (1 + sgn * H)
                        Sp = rules.evaluate(t, fp)
                        fac = Sp / (resp["S"][n] / dec(B.SCALE)) * (1 + sgn * H) ** e_disp
                        Qp = [x if i != n else x * fac for i, x in enumerate(Q)]
                        lr.append(dln(sum((x * x for x in Qp), Decimal(0))) / 2)
                    fd = (lr[0] - lr[1]) / (dln(1 + H) - dln(1 - H))
                    worst = max(worst, abs(fd - d))
                    rows.append({"mode": n + 1, "dlnQn_dlnfn": val(e) if Q[n] != 0 else "0 (no contribution)",
                                 "dlnR_SRSS_dlnfn": val(d), "finite_difference": sci(fd, 12)})
                    key = (kinds[q], t["id"].split("_L")[0])
                    if abs(d) > largest.get(key, (Decimal(-1),))[0]:
                        largest[key] = (abs(d), rid, q, n + 1)
                sd["per_quantity"][q] = {"kind": kinds[q], "modes": rows}
            doc["spectra"][t["id"]] = sd
        out.append(doc)
    cross.append({"id": "SENS-finite-difference", "what": "closed-form log-derivative of the SRSS response against a "
                  "central difference with relative step 1e-20", "max_abs_diff": sci(worst),
                  "agrees": worst < Decimal("1e-30")})
    c["sens_largest"] = largest
    return {
        "item": "6. Sensitivity of the item 5 SRSS responses to each mode's frequency (Rayleigh continuum)",
        "reading": "a conditional SRSS sensitivity: one mode's frequency varied alone, with its shape, its "
                   "participation and its force-recovery coefficient q_n held fixed; SRSS over the first five modes. "
                   "It excludes changes of the CQC coefficient, of the missing-mass rigid response, of the shape and "
                   "recovery, and rotations",
        "largest_magnitude": [{"kind": k[0], "spectrum": k[1], "value": val(v[0]), "realization": v[1],
                               "quantity": v[2], "mode": v[3]} for k, v in sorted(largest.items())],
        "realizations": out,
    }
