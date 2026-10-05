"""RV86 check 4: negative controls run through U5's comparison script, byte-unchanged.

Layer A (comparison): the real reader validates the unmutated successor; a wrapper then applies one
mutation to the successor rows / receipt and to the reader's returned classifications so that the
script's own consistency assertions hold and the oracle comparison itself must refuse.
Layer B (bytes): the successor bytes are mutated (optionally resealed with the reader's own _hash) and
the unchanged script runs end to end; the refusing layer is recorded.
Usage: c4_controls.py SCRIPT ORACLE I50_LOG READER_ROOT OUTDIR SPARSE DENSE"""
import copy, hashlib, io, json, math, struct, sys, contextlib
from fractions import Fraction as F
from pathlib import Path
script, oracle, log, reader_root, outdir, sparse, dense = sys.argv[1:8]
SCRIPT_SHA = "37343f33f3dc230824564bb23e852374e78c66a34e6ebb35742144726c819dcb"
code = Path(script).read_text(); assert hashlib.sha256(code.encode()).hexdigest() == SCRIPT_SHA
outdir = Path(outdir); outdir.mkdir(parents=True, exist_ok=True)
sys.path.insert(0, reader_root)
from core.analysis_runs import retained_precision as reader
REAL = reader._validate_draft
bits = lambda x: struct.pack('>d', float(x)).hex()
dec = lambda h: struct.unpack('>d', bytes.fromhex(h))[0]
def si(r):
    y = r["value"]; return y / 1000.0 if r["unit"] == "mm" else y * 1e6 if r["unit"] == "MPa" else y
def run(files, mutate=None, tag="x"):
    """Run the unchanged script text; return (exit, stops or exception)."""
    def wrapped(source, invocation=None):
        v = REAL(source, invocation)
        if mutate: mutate(source, v)
        return v
    reader._validate_draft = wrapped
    out = outdir / f"{tag}.json"
    argv = sys.argv; sys.argv = [script, oracle, log, reader_root, str(out)] + files
    buf = io.StringIO(); result = None
    try:
        with contextlib.redirect_stdout(buf): exec(compile(code, script, "exec"), {"__name__": "__main__"})
    except SystemExit as e: result = ("exit", e.code)
    except Exception as e: result = ("exception", f"{type(e).__name__}: {e}"[:300])
    finally: sys.argv = argv; reader._validate_draft = REAL
    stops = json.loads(out.read_text())["stops"] if out.exists() and result[0] == "exit" else None
    return {"result": result, "stops": stops}
def row(src, rid): return next(r for r in src["results"] if r["id"] == rid)
def cls(v, rid): return next(c for c in v["classifications"] if c["result_id"] == rid)
def receipt_abs(src): return src["retained_precision"]["body"]["cases"][0]["selection"]["absolute_verified"]
def set_value(src, v, rid, raw):
    r = row(src, rid); r["value"] = raw; cls(v, rid)["normalized_bits"] = bits(si(r))
results = {}
# Baseline through the harness (must reproduce U5: exit 0, no stops).
results["baseline"] = run([sparse, dense], None, "baseline")
# --- NC1: one ulp beyond the class edge (found on the actual comparison by bisection on the raw bits).
def edge(rid, files):
    doc = json.loads(Path(files[0]).read_text()); r0 = row(doc["source"], rid)["value"]
    i0 = struct.unpack('>q', struct.pack('>d', r0))[0]; sgn = 1 if r0 > 0 else -1
    def verdict(i):
        raw = struct.unpack('>d', struct.pack('>q', i))[0]
        res = run(files[:1], lambda s, v: set_value(s, v, rid, raw), "edge_probe")
        return res["result"] == ("exit", 0)
    lo, hi = i0, i0 + 2**40  # moving away from zero in magnitude
    assert verdict(lo)
    while verdict(hi): hi += 2**40
    while hi - lo > 1:
        mid = (lo + hi) // 2
        if verdict(mid): lo = mid
        else: hi = mid
    return struct.unpack('>d', struct.pack('>q', lo))[0], struct.unpack('>d', struct.pack('>q', hi))[0]
nc1 = {}
for rid in ["result:disp:N1:ry", "result:stress:M1:end-i:torsional-shear", "result:disp:N1:uy"]:
    inside, outside = edge(rid, [sparse])
    a = run([sparse, dense], lambda s, v, rid=rid, x=inside: set_value(s, v, rid, x), f"nc1_inside_{rid.replace(':','_')}")
    b = run([sparse, dense], lambda s, v, rid=rid, x=outside: set_value(s, v, rid, x), f"nc1_outside_{rid.replace(':','_')}")
    nc1[rid] = {"inside_raw": repr(inside), "outside_raw": repr(outside), "ulps_apart": struct.unpack('>q', struct.pack('>d', outside))[0] - struct.unpack('>q', struct.pack('>d', inside))[0],
                "inside": b and a, "outside": b}
    nc1[rid]["inside"] = a
# absolute: rigid Fx at exactly its bound (pass) and one ulp beyond (fail)
def abs_edge(s, v, rid, k):
    b = dec(cls(v, rid)["bound_bits"]); x = b if k == 0 else math.nextafter(b, math.inf)
    set_value(s, v, rid, x)
for k, name in [(0, "at_bound"), (1, "one_ulp_beyond")]:
    nc1[f"result:support-action:4:case:8:rigid:N0:Fx:{name}"] = run([sparse, dense], lambda s, v, k=k: abs_edge(s, v, "result:support-action:4:case:8:rigid:N0:Fx", k), f"nc1_abs_{name}")
results["NC1_one_ulp_beyond_class"] = nc1
# --- NC2: swap two rows' classes.
def swap(s, v, a, b, receipt=True):
    ca, cb = cls(v, a), cls(v, b)
    for key in ["class", "bound_bits", "scale_bits"]: ca[key], cb[key] = cb[key], ca[key]
    if receipt:
        lst = receipt_abs(s); ids = {x["result_id"]: x for x in lst}
        for rid, c in [(a, ca), (b, cb)]:
            if c["class"] == "absolute_verified":
                if rid in ids: ids[rid]["bound"] = c["bound_bits"]
                else: lst.append({"result_id": rid, "bound": c["bound_bits"]})
            elif rid in ids: lst.remove(ids[rid])
nc2 = {}
for a, b in [("result:stress:M1:end-i:torsional-shear", "result:stress:M1:end-i:bending-normal-y"),
             ("result:disp:N1:ry", "result:support-action:4:case:8:rigid:N0:Fx"),
             ("result:disp:N0:ux", "result:disp:N0:rx")]:
    nc2[f"{a} <-> {b} (reader and receipt)"] = run([sparse, dense], lambda s, v, a=a, b=b: swap(s, v, a, b, True), "nc2_both")
    nc2[f"{a} <-> {b} (reader only)"] = run([sparse, dense], lambda s, v, a=a, b=b: swap(s, v, a, b, False), "nc2_reader")
results["NC2_swap_classes"] = nc2
# --- NC3: corrupt one bound.
def set_bound(s, v, rid, x, reader_side=True, receipt_side=True):
    if reader_side: cls(v, rid)["bound_bits"] = bits(x)
    if receipt_side: next(e for e in receipt_abs(s) if e["result_id"] == rid)["bound"] = bits(x)
nc3 = {}
FX = "result:support-action:4:case:8:rigid:N0:Fx"
fxv = abs(row(json.loads(Path(sparse).read_text())["source"], FX)["value"])
nc3["shrink below the actual error (reader and receipt)"] = run([sparse, dense], lambda s, v: set_bound(s, v, FX, math.nextafter(fxv, 0)), "nc3_shrink")
nc3["receipt bound one ulp off (receipt only)"] = run([sparse, dense], lambda s, v: set_bound(s, v, FX, math.nextafter(dec(cls(v, FX)["bound_bits"]), 0), False, True), "nc3_receipt_only")
nc3["inflate x2 (reader and receipt): limit of the comparison"] = run([sparse, dense], lambda s, v: set_bound(s, v, FX, 2 * dec(cls(v, FX)["bound_bits"])), "nc3_inflate")
results["NC3_corrupt_bound"] = nc3
# --- Layer B: byte-level mutations through the unchanged script and the real reader.
def write_mut(src_file, tag, fn, reseal):
    doc = json.loads(Path(src_file).read_text()); fn(doc)
    if reseal:
        s = doc["source"]; body = s["retained_precision"]["body"]
        body["publication_sha256"] = reader._hash("retained_precision_publication_mp_v2", {k: v for k, v in s.items() if k != "retained_precision"})
        s["retained_precision"]["receipt_sha256"] = reader._hash("retained_precision_receipt_mp_v2", body)
    p = outdir / f"{tag}.successor.json"; p.write_text(json.dumps(doc)); return str(p)
def inflate_bound(doc):
    e = next(x for x in doc["source"]["retained_precision"]["body"]["cases"][0]["selection"]["absolute_verified"] if x["result_id"] == FX); e["bound"] = bits(2 * dec(e["bound"]))
def move_value(doc):
    r = row(doc["source"], "result:disp:N1:ry"); r["value"] = r["value"] * (1 + 4e-9)
def swap_bytes(doc):
    sel = doc["source"]["retained_precision"]["body"]["cases"][0]["selection"]
    e = next(x for x in sel["absolute_verified"] if x["result_id"] == FX); e["result_id"] = "result:disp:N1:ry"
lb = {}
for tag, fn in [("bytes_inflate_bound", inflate_bound), ("bytes_move_value_4e-9", move_value), ("bytes_swap_class_ry_Fx", swap_bytes)]:
    for reseal in [False, True]:
        f = write_mut(sparse, f"{tag}_{'resealed' if reseal else 'raw'}", fn, reseal)
        lb[f"{tag} ({'resealed' if reseal else 'not resealed'})"] = run([f], None, f"{tag}_{reseal}")
results["layer_B_bytes"] = lb
print(json.dumps(results, indent=1))
