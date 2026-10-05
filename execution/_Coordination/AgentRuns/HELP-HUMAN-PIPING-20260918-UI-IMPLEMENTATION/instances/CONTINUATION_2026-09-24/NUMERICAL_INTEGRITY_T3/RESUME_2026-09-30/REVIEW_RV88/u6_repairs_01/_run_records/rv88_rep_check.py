"""RV88 (U6 repair confirmation): oracle for zz_rv88_rep.rs facts.
S-2 messages are rebuilt from RV88's own unit table (D2 4.9.10 admitted units ->
the reader's canonical SI unit) and RV88's own formatter for Rust `{:e}` (shortest
round-trip digits, d[.ddd]e<exp>, no '+'), with the bound bits taken from the
receipt's own absolute_verified list.
Usage: python3 rv88_rep_check.py <facts.tsv> <cand P>"""
import json, struct, sys
from decimal import Decimal
from pathlib import Path

facts = Path(sys.argv[1]).read_text().splitlines()
P = Path(sys.argv[2])
SI = {"m": "m", "mm": "m", "rad": "rad", "N": "N", "kN": "N", "N*m": "N*m", "kN*m": "N*m", "Pa": "Pa", "MPa": "Pa"}
fails, n = [], {"msg": 0, "cd": 0, "case": 0, "dd": 0, "swap": 0}


def rust_e(bits_hex):
    x = struct.unpack(">d", bytes.fromhex(bits_hex))[0]
    if x == 0:
        return "0e0"
    sign, digits, exp = Decimal(repr(x)).as_tuple()
    ds = "".join(map(str, digits)).rstrip("0") or "0"
    e = exp + len(digits) - 1
    return f"{'-' if sign else ''}{ds[0]}{'.' + ds[1:] if len(ds) > 1 else ''}e{e}"


def absolute(kind, si, bits):
    return f"{kind}: retained_precision_absolute_verified; verified only to the receipt's absolute bound b = {rust_e(bits)} {si} (binary64 {bits}), below the relative accuracy floor; source value/unit and annotation retained; withheld from rule binding and reliance"


def not_covered(kind):
    return f"{kind}: retained_precision_not_covered; no verified accuracy for this quantity kind; source value/unit and annotation retained; withheld from rule binding and reliance"


lists = {}
for mode in ("sparse_interactive", "dense_scrutiny"):
    src = json.loads((P / f"fixtures/results/retained_precision_milestone_successor_{mode}.json").read_text())["source"]
    lists[mode] = ({x["result_id"]: x["bound"] for x in src["retained_precision"]["body"]["cases"][0]["selection"]["absolute_verified"]},
                   {r["id"]: r for r in src["results"]})
units_seen = {}
for line in facts:
    f = line.split("\t")
    if f[0] == "msg":
        _, mode, rid, unit, code, msg = f
        bounds, rows = lists[mode]
        n["msg"] += 1
        want = absolute(rows[rid]["kind"], SI[unit], bounds[rid]) if rid in bounds else not_covered(rows[rid]["kind"])
        if code != ("retained_precision_absolute_verified" if rid in bounds else "retained_precision_not_covered") or msg != want:
            fails.append(("msg", mode, rid, msg, want))
        units_seen[(mode, unit, SI.get(unit))] = units_seen.get((mode, unit, SI.get(unit)), 0) + 1
    elif f[0] == "cd":
        _, unit, bits, got = f
        n["cd"] += 1
        want = f'Some(("retained_precision_absolute_verified", {json.dumps(absolute("element_local_axial_force", SI[unit], bits))}))' if unit in SI else f'Some(("retained_precision_not_covered", {json.dumps(not_covered("element_local_axial_force"))}))'
        if got != want:
            fails.append(("cd", unit, bits, got, want))
    elif f[0] == "cdnc":
        _, unit, got = f
        if got != f'Some(("retained_precision_not_covered", {json.dumps(not_covered("element_local_axial_force"))}))':
            fails.append(("cdnc", unit, got))
    elif f[0] == "case":
        _, cid, st, dis, est, edis = f
        n["case"] += 1
        if [st, dis] != [est, edis]:
            fails.append(("case", cid, st, dis, est, edis))
    elif f[0] == "dd":
        _, did, fx, got, exp = f
        n["dd"] += 1
        expv = list(json.loads(exp).values())[0]
        if got != expv:
            fails.append(("dd", did, fx, got, expv))
    elif f[0] == "swap":
        n["swap"] += 1
        if "Err(" not in f[3]:
            fails.append(("swap", f))
print("counts", n)
print("absolute/not_covered messages by (mode, row unit, SI)", units_seen)
print("FAILURES", len(fails))
for x in fails[:10]:
    print("  ", x)
