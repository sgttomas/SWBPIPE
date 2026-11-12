"""predicted_formulation_difference.json: per case id, compared quantity and rule, the consistent and the
lumped value at the same member count and their relative difference (lumped / consistent - 1).

Computed only from the [decimal, binary64] pairs pack_summary.json publishes (asserted): the relative
difference is formed from the published 40-digit decimals and adds no other number. The statement and
date are constants, not read from the clock.
"""

from decimal import Decimal

from pinned import load

N_, R_, C_, FE_, B_ = load()

STATEMENT = ("Predicted before any cross-engine comparison. Recorded 2026-11-12. It states, per member count, how far "
             "a plain translational lumped-mass formulation is expected to differ from SWBPIPE's stated formulation "
             "on these cases, so that when sealed captures are opened the expected convention difference is on record "
             "from before and is not fitted afterwards. It is a prediction about a formulation, not about any program: "
             "whether another program's mass formulation is this one is not asserted.")
RECORDED = "2026-11-12"
KINDS = ["frequency", "displacement", "force_or_moment", "support_reaction"]


def kind_of(row):
    q = row["quantity"]
    if q.startswith("frequency"):
        return "frequency"
    if row["unit"] == "m":
        return "displacement"
    if q.startswith("base_") or q.startswith("tip_reaction") or q.startswith("spring_"):
        return "support_reaction"
    return "force_or_moment"


def build(cross, store):
    rows = store["summary_rows"]
    import json
    summary_doc = json.loads(store["summary_bytes"].decode("utf-8"))
    published = summary_doc["rows"]
    same = published == rows
    cases, order = {}, []
    for r in rows:
        if r["case"] not in cases:
            cases[r["case"]] = []
            order.append(r["case"])
        if r["consistent"] is None or r["lumped"] is None:
            continue
        c, l = Decimal(r["consistent"][0]), Decimal(r["lumped"][0])
        d = l / c - 1
        cases[r["case"]].append({"quantity": r["quantity"], **({"rule": r["rule"]} if "rule" in r else {}),
                                 "kind": kind_of(r), "unit": r["unit"], "consistent": r["consistent"],
                                 "lumped": r["lumped"],
                                 "relative_difference": [N_.sci(d, 6), repr(float(d))], "_d": d})
    out = []
    for cid in order:
        rs = cases[cid]
        largest = {}
        for k in KINDS:
            sel = [x for x in rs if x["kind"] == k]
            if not sel:
                largest[k] = None
                continue
            m = max(sel, key=lambda x: abs(x["_d"]))
            largest[k] = {"value": [N_.sci(abs(m["_d"]), 6), repr(float(abs(m["_d"])))],
                          "quantity": m["quantity"] + (f" {m['rule']}" if "rule" in m else "")}
        for x in rs:
            del x["_d"]
        entry = {"case": cid, "rows": rs, "largest_abs_relative_difference_by_kind": largest}
        if not rs:
            entry["note"] = ("no consistent and lumped values in this package for this case: the N = 8 values of case D "
                             "are the sibling discrete package's")
        out.append(entry)
    cross.append({"id": "PRED-from-summary", "what": "the rows are taken from the pairs pack_summary.json publishes "
                  "(the generated file read back equals the rows used)", "agrees": same})
    return {
        "_head": {"statement": STATEMENT, "recorded": RECORDED},
        "definitions": {
            "relative_difference": "lumped / consistent - 1, from the published 40-digit decimals of pack_summary.json; "
                                   "[decimal with 6 significant digits, nearest binary64 of the quotient]",
            "consistent": "the generator's planar FE with the consistent mass SWBPIPE states (DEL-04-09 REQ-001)",
            "lumped": "the half-and-half translational lumped mass: half of each member's mass at each end "
                      "translation, no rotary inertia",
            "kinds": {"frequency": "frequency rows", "displacement": "tip or mid-span displacement",
                      "force_or_moment": "element forces and moments (none among case C's compared quantities)",
                      "support_reaction": "base and tip reaction forces and moments"},
            "omitted": "rows where either value is null (mid-span at N = 1; the lumped N = 1 Z plane; case D)"},
        "cases": out}
