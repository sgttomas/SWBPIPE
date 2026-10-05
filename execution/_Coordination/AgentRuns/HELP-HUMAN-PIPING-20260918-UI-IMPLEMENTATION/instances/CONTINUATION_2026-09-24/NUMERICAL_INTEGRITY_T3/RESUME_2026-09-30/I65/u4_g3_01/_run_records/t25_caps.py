"""I65 U4 G3 T25 (RV83 B-1): selected SOURCE-BLOCKS-1 finalization at the D1 caps.

Entered only when the case's legacy source recovery SELECTED (T07 succeeded) and the
case continues to finalization: per case FinalizedSourceBlockCase::exact
(PP/lib.rs:4972 -> source_receipt.rs:668-700) and once per invocation
FinalizedSourceBlockReceipt::finalize (PP/lib.rs:2796-2798 -> source_receipt.rs:869-1060).
`composite` is false in D1 (lib.rs:2743 requires is_exact), so composite.rs is never
entered. Owners are summed over the steps (a conservative sum of the step maxima), while
the ordinary owners and the T07 selected output stay live (they are other T05 rows).

The hash route (source_receipt.rs:30-37) is: json! wrapper deep copy of the payload, a
serde_json::to_string text (J3: capacity <= max(128, 2e)), checked_parse into a fresh
Value tree plus its per-frame seen-key clones and scratch (J5, J6, J10), and the
canonical rendering (J4: capacity <= max(8, 2J)), all coexisting at the render (I54
COEFFICIENTS J1-J10). Escaped length e <= EPS * string_bytes + key_bytes + 8 * values
+ 24 * numbers, where EPS is the worst escape factor of the input text: 6 (any byte may
be a control character, written as \\u00XX) or 2 under the proposed D1.11 clause (no
input string carries a control character) -- quotes and backslashes still double.
Usage: python3 t25_caps.py <caps|milestone> [eps]
"""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from g3lib import X, B, s, V, A, Xc, Mat, HashReq, pushcap, appendcap, total, ASSUMED, j3cap

which = sys.argv[1] if len(sys.argv) > 1 else "caps"
EPS = int(sys.argv[2]) if len(sys.argv) > 2 else 6
H = os.path.dirname(os.path.abspath(__file__))
TXT = json.load(open(os.path.join(H, f"text_closure.{which}.json")))["atoms"]
T07 = json.load(open(os.path.join(H, f"t07_repair.{which}.out.json")))
if which == "caps":
    n, m, g, sp, r, l = 32, 32, 32, 32, 192, 192
else:
    n, m, g, sp, r, l = 2, 1, 4, 3, 6, 3
N = 6 * n; F = N; k = min(N, r); C = 144 * m + sp
Fn = 42 * m + N + sp + 6 * g
P = 7 * n + 51 * m + 8 * g + 3
R0 = 7 * n + 51 * m + g + 3
D = TXT["D_env"]; ROW = TXT["Text(row)"]; DIAG = TXT["Text(diag_env)"]   # the envelope's own diagnostics
ID, RID = 128, 1024
units = T07["descriptor_units_#1"]

def load_x(sym):
    x = X()
    for term in sym.split(" + "):
        if "*" in term:
            c, a = term.split("*", 1); x = x + X({a: int(c)})
        elif term:
            x = x + B(int(term))
    return x
t07rows = {k_: load_x(v["symbolic"]) for k_, v in T07["rows"].items()}
def t07(*prefixes):
    return total(v for k_, v in t07rows.items() if k_.split(" ")[0] in prefixes)

class VF:
    """Facts of one serde_json Value tree."""
    def __init__(self, arr=0, obj=0, ent=0, strb=0, keyb=0, nums=0, depth=8):
        self.arr, self.obj, self.ent, self.strb, self.keyb, self.nums, self.depth = arr, obj, ent, strb, keyb, nums, depth
    def __add__(self, o):
        return VF(self.arr + o.arr, self.obj + o.obj, self.ent + o.ent, self.strb + o.strb,
                  self.keyb + o.keyb, self.nums + o.nums, max(self.depth, o.depth))
    def scale(self, c):
        return VF(self.arr * c, self.obj * c, self.ent * c, self.strb * c, self.keyb * c, self.nums * c, self.depth)
    def values(self):
        return self.arr + self.ent + 1
    def tree(self):          # to_value / json! / clone: exact arrays, exact strings
        return (s("Value") * self.arr + X({"Node(String,Value)": self.obj + self.ent // 5})
                + B(self.strb + self.keyb))
    def parsed(self):        # checked_parse: arrays push-built (<= max(4,2h) each)
        return (s("Value") * (4 * self.obj + 2 * self.arr) + X({"Node(String,Value)": self.obj + self.ent // 5})
                + B(self.strb + 2 * self.keyb))           # + every seen key cloned (J6)
    def text(self):
        return EPS * self.strb + self.keyb + 8 * self.values() + 24 * self.nums

def hash_route(vf):
    e = vf.text()
    return (vf.tree() + B(j3cap(e)) + vf.parsed()
            + HashReq("String", 16) * vf.depth          # active seen sets (<= 16 keys per frame bound below)
            + B(2 * max(24, RID) + 2080 + max(8, 2 * e) + vf.depth * 8 * 16 + 144 + 128))

rows = {}
# ---------------------------------------------------------------- per-case exact finalization
# (1) check_input -> check_input_with_physical (source_receipt.rs:270-366)
RAW = (s("Value") * 32_768 + X({"Node(String,Value)": 16_384 + 16_384 // 5}) + B(131_072 + 131_072 + 128))
strings_typed = 8 * (7 + 16 * 7) + 6 + 2 * n + 12 * m + 6 * g + r + 2 + 7 * l
TYPED = (Xc("MaterialInput", 8) + Xc("TemperaturePoint", 128) + Xc("PreviewNode", n) + Xc("PreviewPipe", m)
         + Xc("PreviewSupport", g) + Xc("String", r) + Xc("PreviewLoadCase", 1) + Xc("PrimitiveLoadInput", l)
         + Xc("Authored<Vec<ExpansionLawInput>>", 4) + B(strings_typed * ID) + RAW)
CONTENT = s("(Content,Content)") * 16_384 + B(131_072 + 131_072)   # flatten / internally tagged buffering
requested = RAW + TYPED + CONTENT
rows["T25.1a requested(): raw clone + typed request + serde buffering"] = requested
rows["T25.1b materials clone, O-N transients, diagnostics"] = (Xc("MaterialInput", 4) + Xc("TemperaturePoint", 64)
                                                             + B(4 * (7 + 16 * 7) * ID) + B(2048))
rows["T25.1c build_model + helper maps + boundary"] = (
    V("FrameNode", n) + V("StraightPipeElement", m) + V("FrameElement", m) + A("LinearSupport", g)
    + HashReq("(String,DerivedSection)", m) + B(2 * m * ID + g * ID + 8 * (4 * g + 2 * r) + 8 * sp + 32)
    + HashReq("(&str,&T)", 8) + HashReq("(&str,usize)", n) + HashReq("(&str,&T)", m)
    + V("usize", r) + V("SpringEntry", sp) + B(sp * ID) + V("SupportFinding", r))
rows["T25.1d dense stiffness, loads, application, force, prescribed, free"] = (
    Mat(8, N, N) + A("PrimitiveLoad", l) + V("NodalLoadContribution", l) + B(2 * l * ID)
    + B(8 * N) + V("ForceTerm", l) + V("Option<FormationRecord>", l) + s("Vec<usize>") * N
    + B(8 * (4 * N + 2 * l) + 8 * pushcap(min(l, N)) + l * ID) + Xc("(usize,f64)", k) + B(8 * pushcap(F)))
rows["T25.1e replay_against: Sources #2 and prepare_sources locals"] = t07("A1", "A2", "A3", "A4", "A5", "B1", "B2", "B3", "B4", "B5", "B6", "B7")
rows["T25.1f replay: Context #2, Response #2, values #2, temporaries"] = (
    t07("C1", "C2", "C3", "C4", "C5", "C6", "C7", "D1", "D2", "F1", "F2", "G2") + B(2 * 8 * Fn))
# (2) check_binding_against: Sources #3 and its locals
rows["T25.2 check_binding_against: Sources #3"] = t07("A1", "A2", "A3", "A4", "A5", "B1", "B2", "B3", "B4", "B5", "B6", "B7") + B(8 * Fn)
# (3) rows::bind (rows.rs:562-700)
R = P                                        # the case's rows <= P_final (one case)
supports_vf = VF(arr=g * (6 + 12), obj=g * (1 + 6 + 12), ent=g * 4 + 6 * g * 4 + 12 * g * 3,
                 strb=g * (2 * ID + 32) + 6 * g * (16 + RID + 256) + 12 * g * (32 + ID),
                 keyb=(g * 4 + 6 * g * 4 + 12 * g * 3) * 24, nums=6 * g + 12 * g, depth=6)
rows["T25.3 rows::bind ledger, maps, primary/derived, supports (+ json! copy)"] = (
    X({"Node(usize,String)": max(1, Fn // 5 + 1)}) + B(Fn * RID)               # by_index
    + X({"Node(String,())": max(1, Fn // 5 + 1)}) + B(Fn * RID)                 # used
    + X({"Node(&str,&ResultItem)": max(1, R // 5 + 1)})                          # actual
    + V("Projection", Fn) + B(Fn * 3 * RID)                                      # projections
    + X({"Node(String,RowTreatment)": max(1, R // 5 + 1)}) + B(R * (2 * RID + RID + 4 * (24 + RID)))  # ledger
    + X({"Node(usize,ResultItem)": max(1, Fn // 5 + 1)}) + B(Fn * ROW)           # primary
    + Xc("Derived", R) + B(R * (ROW + 4 * (24 + RID)))                           # derived rows
    + supports_vf.tree() * 2 + Xc("RowTreatment", R))
# (4) source::commitment (source.rs:252-410)
payload_vf = VF(arr=C * 0 + m * (12 + 6 + 3 * 144) + C + m + sp + k + l + Fn + units + N,
                obj=C + m * 3 + sp + k + l + Fn + units + 1,
                ent=C * 4 + m * (8 + 9 + 2) + sp * 6 + k * 3 + l * 5 + Fn * 3 + units * 2 + 12,
                strb=C * (150 + 16) + m * (3 * ID + 6 * 16 + 9 * 16 + 3 * 144 * 16 + 2 * 16)
                + sp * (3 * ID + 2 * 16) + k * (16 + ID) + l * (ID + ID + 16) + units * 16 * 2 + N * ID,
                keyb=(C * 4 + m * 19 + sp * 6 + k * 3 + l * 5 + Fn * 3 + units * 2 + 12) * 32,
                nums=C * 2 + m * 12 + sp + k + l * 2 + Fn + units, depth=7)
functions_vf = VF(arr=Fn * 4, obj=Fn * 3, ent=Fn * 12, strb=Fn * (3 * RID + 256), keyb=Fn * 12 * 24,
                  nums=Fn * 4, depth=5)
plan_vf = functions_vf + VF(arr=R + P, obj=R * 2, ent=R * 8, strb=R * 3 * RID + P * RID, keyb=R * 8 * 24, nums=R)
rows["T25.4 source commitment: payload + hash, functions, plan + hash, blocks"] = (
    payload_vf.tree() + hash_route(payload_vf) + functions_vf.tree() + plan_vf.tree() + hash_route(plan_vf)
    + s("Vec<usize>") * F + B(16 * F) + X({"Node(usize,())": 2 * (F // 5 + 1)}) + B(8 * pushcap(F))
    + VF(obj=1, ent=12, strb=3 * 64 + 128, keyb=12 * 32, nums=6).tree())
# (5)-(7) serialized(rows) (I54 RowJSON law), work Value, the retained case
rowjson = s("Value") * (2 * R) + X({"Node(String,Value)": 3 * R}) + B(128 * R) + B(R * ROW)
rows["T25.5 actual_rows Value, work Value, retained FinalizedSourceBlockCase"] = (
    rowjson + VF(obj=2, ent=12, strb=256, keyb=12 * 32, nums=8).tree() + s("FinalizedSourceBlockCase")
    + B(64 + ID))
# ---------------------------------------------------------------- per-invocation finalize
env_vf = (VF(arr=P + 4 * P, obj=3 * P, ent=15 * P, strb=P * ROW, keyb=15 * P * 24, nums=P, depth=4)
          + VF(arr=D + 4 * D + 3 * (1 + l), obj=D, ent=6 * D, strb=DIAG, keyb=6 * D * 16, depth=4)
          + VF(arr=3 * m + 2 * g, obj=3 + m + g, ent=9 + 15 * m + 2 * g,
               strb=m * (128 + 1024 + 3 * 120) + (2 * m + g) * 128 + g * (128 + 64),
               keyb=(9 + 15 * m + 2 * g) * 40, nums=10 * m, depth=6)
          + VF(arr=64, obj=24, ent=160, strb=24 * 1024, keyb=160 * 40, nums=60, depth=5))
publication = env_vf.tree()
rows["T25.6 serialized(envelope) = publication Value"] = publication
rows["T25.7 requested() x2 (the first a temporary), assessed quality Values x2"] = (requested * 2 + VF(obj=8, ent=60, strb=4096, keyb=2400, nums=20).tree() * 2)
rows["T25.8 ids set, accounted set, observations"] = (X({"Node(&str,())": (P + D) // 5 + 1}) + X({"Node(String,())": R // 5 + 1})
                                                      + B(R * RID) + Xc("String", P) + B(P * RID))
case_wire_vf = (payload_vf.scale(0) + VF(obj=1, ent=12, strb=3 * 64 + 128, keyb=12 * 32, nums=6)
                + VF(arr=Fn, obj=Fn, ent=Fn * 4, strb=Fn * 3 * RID, keyb=Fn * 4 * 24)
                + VF(arr=R, obj=R, ent=R * 5, strb=R * (2 * RID + RID + 4 * RID), keyb=R * 5 * 24)
                + supports_vf + VF(obj=2, ent=12, strb=256, keyb=12 * 32, nums=8))
rows["T25.9 per case: actual rows clone + serialized(actual) + wire entry"] = (
    Xc("ResultItem", P) + B(P * ROW) + rowjson + case_wire_vf.tree())
body_vf = case_wire_vf + VF(arr=P, obj=4, ent=20, strb=P * RID + 512, keyb=20 * 32, nums=4)
rows["T25.10 body json! (copies wire) + hash(publication) inside it"] = body_vf.tree() + hash_route(env_vf)
rows["T25.11 hash(body), into_wire copy, retained receipt in the envelope"] = hash_route(body_vf) + body_vf.tree() * 2

evv = dict(ASSUMED)
def ev(x):
    return x.ev(evv)
def pick_max(cands):
    """Stage maxima are compared in-build (G5); here under ASSUMED layouts, recording which wins."""
    best = max(cands, key=lambda kv: ev(kv[1]))
    return best
# The steps run in order and each step's locals drop at its return (source_receipt.rs:668-700,
# 869-1060). Carried owners: the case outputs (projections, rows, supports, actual_rows, work)
# from step 3 on; at the invocation finalize, publication and the second typed request live to
# the end, and the body/wire chain grows.
carried_case = (V("Projection", Fn) + B(Fn * 3 * RID) + Xc("RowTreatment", R) + B(R * (3 * RID + 4 * (24 + RID)))
                + supports_vf.tree() + rowjson + VF(obj=2, ent=12, strb=256, keyb=12 * 32, nums=8).tree()
                + VF(obj=1, ent=12, strb=3 * 64 + 128, keyb=12 * 32, nums=6).tree())
stage = {
    "S1 check_input (requested, rebuild, replay)": total(rows[k_] for k_ in rows if k_.split(" ")[0] in ("T25.1a", "T25.1b", "T25.1c", "T25.1d", "T25.1e", "T25.1f")),
    "S2 check_binding_against": rows["T25.2 check_binding_against: Sources #3"],
    "S3 rows::bind": rows["T25.3 rows::bind ledger, maps, primary/derived, supports (+ json! copy)"],
    "S4 source commitment (+ carried ledger)": rows["T25.4 source commitment: payload + hash, functions, plan + hash, blocks"] + carried_case,
    "S5 serialized(rows), work, case": rows["T25.5 actual_rows Value, work Value, retained FinalizedSourceBlockCase"] + carried_case,
}
case_name, case_peak = pick_max(list(stage.items()))
inv_base = (publication + requested + VF(obj=8, ent=60, strb=4096, keyb=2400, nums=20).tree() * 2
            + rows["T25.8 ids set, accounted set, observations"]
            + rows["T25.9 per case: actual rows clone + serialized(actual) + wire entry"])
inv = {
    "I1 body json! with hash(publication) inside": inv_base + body_vf.tree() + hash_route(env_vf),
    "I2 hash(body)": inv_base + body_vf.tree() + hash_route(body_vf),
    "I3 into_wire copy": inv_base + body_vf.tree() * 2,
}
inv_name, inv_peak = pick_max(list(inv.items()))
req = carried_case + (case_peak if ev(case_peak) >= ev(inv_peak) else inv_peak)
out = {"which": which, "EPS": EPS,
       "facts": {"publication": vars(env_vf), "payload": vars(payload_vf), "body": vars(body_vf),
                 "publication_text_upper": env_vf.text(), "payload_text_upper": payload_vf.text()},
       "rows": {k_: {"symbolic": v.show(), "assumed_bytes": ev(v)} for k_, v in rows.items()},
       "stages": {k_: ev(v) for k_, v in list(stage.items()) + list(inv.items())},
       "peak": {"case_stage": case_name, "invocation_stage": inv_name,
                "rule": "T25 = carried case outputs + max(per-case stage peak, per-invocation stage peak); "
                        "the max is taken in-build (G5) over the evaluated stage expressions"},
       "sum_of_rows_for_reference": ev(total(rows.values())),
       "T25_requested": {"symbolic": req.show(), "assumed_bytes": ev(req)}}
mv = B(j3cap(env_vf.text()) // 2)      # the largest single old backing: the publication text's last growth
out["moving_extra"] = {"symbolic": mv.show(), "assumed_bytes": ev(mv)}
print(json.dumps(out, indent=1))
