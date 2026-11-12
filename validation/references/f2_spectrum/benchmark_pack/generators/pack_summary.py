"""pack_summary.json: a small flat list of the compared quantities per benchmark case id, with the
Rayleigh continuum value (the product's continuum limit), the consistent-mass value and the lumped-mass
value at that member count, each as [decimal with 40 significant digits, nearest binary64], SI units."""

from pinned import load
import pack_c as PC

N_, R_, C_, FE_, B_ = load()

UNITS = {"m": "m", "N": "N", "Nm": "N*m"}


def pair(x):
    if x is None:
        return None
    v = N_.val(x)
    return [v["value"], v["binary64"]]


def unit_of(q):
    return {"m": "m", "N": "N", "Nm": "N*m"}[q.rsplit("_", 1)[1]]


def build(cross, store):
    c, merged, d = store["c"], store["c_merged"], store["d"]
    rows = []
    cases = [(f"cpb-dyn-c-n{N:02d}-flat", N, "FLAT") for N in PC.NS] + [("cpb-dyn-c-n08-two", 8, "TWO")]
    for cid, N, tid in cases:
        ref = merged["rayleigh"]
        for fam, idx in (("Y", 1), ("Z", 1), ("Y", 2), ("Z", 2), ("Y", 3), ("Z", 3)):
            def f_of(ml):
                return next((e[0] for e in ml if e[1] == fam and e[2] == idx), None)
            rows.append({"case": cid, "quantity": f"frequency_{fam}{idx}", "unit": "Hz",
                         "continuum": pair(f_of(ref)), "consistent": pair(f_of(merged[("consistent", N)])),
                         "lumped": pair(f_of(merged[("lumped", N)]))})
        for q in PC.Y_Q + PC.Z_Q:
            for r in PC.RULES3:
                def g(key):
                    cm = c.get(key)
                    return None if cm is None or cm[q] is None else cm[q][r]
                rows.append({"case": cid, "quantity": q, "rule": r, "unit": unit_of(q),
                             "continuum": pair(g(("cont", "rayleigh", tid))),
                             "consistent": pair(g(("consistent", N, tid))), "lumped": pair(g(("lumped", N, tid)))})
    for q in PC.Y_Q + PC.Z_Q:
        for r in ("SRSS", "ABSSUM"):
            rows.append({"case": "cpb-dyn-c-n08-trunc", "quantity": q, "rule": f"periodic_{'SRSS'}_with_rigid_{r}",
                         "unit": unit_of(q), "continuum": pair(c[("trunc", "rayleigh")][q][r]),
                         "consistent": pair(c[("trunc", "consistent", 8)][q][r]),
                         "lumped": pair(c[("trunc", "lumped", 8)][q][r])})
    c2, c6 = d["rayleigh"]
    for label, cm in (("modes_1_2", c2), ("modes_1_6_information", c6)):
        for q in cm:
            for r in ("SRSS", "ABSSUM", "TEN_PERCENT", "CQC"):
                rows.append({"case": "cpb-dyn-d-n08-flat", "quantity": q, "rule": r, "modes": label,
                             "unit": unit_of(q), "continuum": pair(cm[q][r]), "consistent": None, "lumped": None})
    cross.append({"id": "SUMMARY-rows", "what": "every case id has rows", "agrees": len({r["case"] for r in rows}) == 8})
    store["summary_rows"] = rows
    return {
        "definitions": {
            "continuum": "Rayleigh continuum (rotary inertia, contents none), the product's continuum limit; case C with "
                         "its first six merged modes periodic (truncation case: Y1 and Z1 periodic, f_r = 1.25 Hz)",
            "consistent": "the generator's planar FE at that member count with the consistent mass the product states",
            "lumped": "NOT the product's formulation: half of each member's mass at each end translation",
            "null": "not available at that member count (mid-span is not a node at N = 1; the lumped N = 1 model "
                    "has no finite Z-plane mode, its only Z-plane DOF being a massless rotation), or not this "
                    "package's (case D at N = 8: the sibling discrete package's values)",
            "rules": "SRSS; ABSSUM (the absolute sum); TEN_PERCENT (the ten percent method); CQC at damping 0.05 "
                     "(case D): modal rules of DEL-04-10 CLM-001",
            "pair": "[decimal with 40 significant digits, nearest binary64]"},
        "rows": rows}
