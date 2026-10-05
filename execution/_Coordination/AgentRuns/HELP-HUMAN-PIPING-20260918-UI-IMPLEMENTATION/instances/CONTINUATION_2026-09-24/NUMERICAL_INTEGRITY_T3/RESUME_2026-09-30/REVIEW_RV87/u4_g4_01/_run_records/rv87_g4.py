"""RV87: independent re-derivation of U4 G4's new terms (T16-T19) and the composed admission
maximum (phases X1-X2, W1-W5, both modes) at the D1 caps with l <= 128, eps = 2.

Stdlib only. Written by RV87 from source at NUM 6796050f47 (= a634ac8b53 for every core file).
The packet's scripts are not imported or executed; only their *output numbers* for the terms
other reviewers own (O, T25, TAV_X/TAV_W, T11-T15, the text atoms) are read as inputs, and the
symbolic forms in those outputs are evaluated by this script's own evaluator, so the stride
sensitivity can be applied to them.

Usage: python3 rv87_g4.py <packet _run_records dir> <snapshot projects/chirality-piping dir> <l>
Output: JSON on stdout.
"""
import json, os, re, sys

RR, SNAP, LCAP = sys.argv[1], sys.argv[2], int(sys.argv[3])
IDSCAN = json.load(open(sys.argv[4])) if len(sys.argv) > 4 else None   # rv87_idclass_scan output (optional)
SUF = ".l128" if LCAP == 128 else ""
def J(name):
    return json.load(open(os.path.join(RR, name)))

# ----------------------------------------------------------------------------- strides
# ILLUSTRATIVE 64-bit strides are an input table (the G3/G4 ASSUMED values; G5 measures them).
# Read from the packet's g3lib.py/g4_caps.py source text as data (not executed).
def assumed_table():
    """Only the ASSUMED dict literals (g3lib.py: ASSUMED = {...} and ASSUMED.update({...});
    g4_caps.py: its ASSUMED.update({...})), read as data."""
    t = {}
    for fn in ("g3lib.py", "g4_caps.py"):
        txt = open(os.path.join(RR, fn)).read()
        for blk in re.findall(r"ASSUMED(?: = \{|\.update\(\{)(.*?)\}\)?\n\n", txt, flags=re.S):
            for k, v in re.findall(r'"((?:s|Node)\([^"]*\))":\s*(\d+)', blk):
                t[k] = int(v)
    t.setdefault("Node(&str,())", 288)
    t["Node((String,String),())"] = 600
    return t
ASSUMED = assumed_table()

def parse_sym(sym):
    """'c*atom + ... + const' -> {atom: c, '1': const}."""
    out = {}
    if sym.strip() == "0":
        return out
    for part in sym.split(" + "):
        part = part.strip()
        m = re.fullmatch(r"(\d+)\*(.+)", part)
        if m:
            out[m.group(2)] = out.get(m.group(2), 0) + int(m.group(1))
        else:
            out["1"] = out.get("1", 0) + int(part)
    return out

class L(dict):
    """A linear form over stride atoms with an integer byte constant '1'."""
    def __add__(self, o):
        r = L(self)
        for k, v in o.items():
            r[k] = r.get(k, 0) + v
        return r
    def __mul__(self, c):
        return L({k: v * c for k, v in self.items()})
    def ev(self, scale=1.0, text=None):
        tot = 0
        for k, v in self.items():
            if k == "1":
                tot += v
            elif k.startswith("Text("):
                tot += v * text[k]
            else:
                tot += v * ASSUMED[k] * scale
        return int(round(tot))
    def stride_part(self):
        return L({k: v for k, v in self.items() if k != "1" and not k.startswith("Text(")})
def b(n): return L({"1": n})
def s(t): return L({f"s({t})": 1})
def node(kv): return L({f"Node({kv})": 1})

def pushcap(h, minimum=4):
    if h <= 0: return 0
    c = 0
    for length in range(1, h + 1):
        if length > c:
            c = max(2 * c, length, minimum)
    return c
def buckets(h):
    if h == 0: return 0
    if h < 4: return 4
    if h < 8: return 8
    adj = (h * 8 + 6) // 7; bb = 1
    while bb < adj: bb *= 2
    return bb
def hashreq(k, h):
    bb = buckets(h)
    return s(k) * bb + b(bb + 23) if bb else L()
def tnodes(n): return 0 if n <= 0 else 1 + (n - 1) // 5

# ----------------------------------------------------------------------------- caps and atoms
n = m = g = sp = 32; r = 192; l = LCAP
N = 6 * n; k = min(N, r)
P = 7 * n + 51 * m + 8 * g + 3                 # P_final (DOMAIN derived)
Q = 7 * n + 30 * m + sp + k + 2 * g
ID, RID, BITS, TOK, SHA, KB = 128, 1024, 16, 64, 64, 32
RAWV, RAWS, RAWK = 16_384, 65_536, 65_536
EPS = 2
TXT = J(f"text_closure.caps{SUF}.json")["atoms"]
ROW, DENV, DIAGENV = TXT["Text(row)"], TXT["D_env"], TXT["Text(diag_env)"]
COMP = J(f"composite_text.caps{SUF}.json")["classes"]
L_PUB = COMP["message_int"]                    # longest envelope string (the integrity message)
L_DIAGID = 2330                                # longest reached diagnostic-id template bound (text run)
M = 4_026_531_840; R = 64 * 2**20; MARGIN = int(0.9 * M)

# ----------------------------------------------------------------------------- Value facts
class VF:
    """Facts of a serde_json::Value tree (Map = BTreeMap; no preserve_order in the PP lock)."""
    def __init__(self, arr=0, obj=0, ent=0, strb=0, keyb=0, nums=0, depth=8):
        self.arr, self.obj, self.ent, self.strb, self.keyb, self.nums, self.depth = arr, obj, ent, strb, keyb, nums, depth
    def __add__(self, o):
        return VF(self.arr + o.arr, self.obj + o.obj, self.ent + o.ent, self.strb + o.strb, self.keyb + o.keyb,
                  self.nums + o.nums, max(self.depth, o.depth))
    def x(self, c):
        return VF(self.arr * c, self.obj * c, self.ent * c, self.strb * c, self.keyb * c, self.nums * c, self.depth)
    def tree(self):
        # to_value / json! / Value::clone: arrays with_capacity(len) (serde_json value/ser.rs:233-236),
        # strings and keys exact, BTreeMap nodes <= 1 + floor((e-1)/5) per map (T2 law).
        return s("Value") * self.arr + node("String,Value") * (self.obj + self.ent // 5) + b(self.strb + self.keyb)
    def parsed(self):
        # canonical_json::checked_parse (canonical_json lib.rs:44-90): Vec::new()+push per array
        # (pushcap <= max(4,2h) <= 6h for h >= 1), Map::insert, owned strings/keys, and the
        # `seen` HashSet's key clones (counted for every key: conservative).
        return s("Value") * (6 * self.arr) + node("String,Value") * (self.obj + self.ent // 5) + b(self.strb + 2 * self.keyb)
    def text(self):
        # serde_json::to_string length: escaped strings (<= eps*len + 2), keys (+3), 8 B of
        # punctuation per value node, <= 24 B per number.
        return EPS * self.strb + self.keyb + 8 * (self.arr + self.ent + 1) + 24 * self.nums

def j3(e): return max(128, 2 * e)
def hash_route(vf, lmax):
    """domain_hash (retained_wire.rs:194-197; result_export source_blocks.rs:182-187), all owners
    counted as coexisting (conservative: the json! wrapper is a statement temporary): wrapper deep
    copy, to_string text (Vec::with_capacity(128), doubling: <= max(128,2e); ser.rs:2213-2220),
    the checked parse tree with seen-key clones and one HashSet per open level, the canonical
    String (push from empty: <= max(8,2e)), write_canonical's per-string to_string temporary
    (<= max(128, 2*(eps*L+2))), the parser's escape scratch (<= 2L), and small fixed scratch."""
    e = vf.text()
    return (vf.tree() + b(j3(e)) + vf.parsed() + hashreq("String", 16) * vf.depth
            + b(2 * max(24, RID) + 2080 + max(8, 2 * e) + vf.depth * 8 * 16 + 272)
            + b(max(128, 2 * (EPS * lmax + 2))) + b(2 * lmax))
def last_growth(vf):
    return j3(vf.text()) // 2

def rec(cnt, slots=0, obj=1, ent=0, strb=0, nums=0, depth=4):
    return VF(arr=cnt * slots, obj=cnt * obj, ent=cnt * ent, strb=cnt * strb, keyb=cnt * ent * KB, nums=cnt * nums, depth=depth)

# ----------------------------------------------------------------------------- the envelope Value
# serde_json::to_value(&MechanicsEnvelope) (lib.rs:819-838).
# Rows (ResultItem, lib.rs:1983-1997): 8 keys + basis_ref 2 + metadata 5 = 15 entries, 3 objects,
# source_result_refs <= 4 slots (Text(row) counts their content), 1 number. Key bytes exact sum
# is 128 per row; the packet's 15*24 = 360 class is a valid upper bound and is kept.
ROWS = VF(arr=P + 4 * P, obj=3 * P, ent=15 * P, strb=P * ROW, keyb=15 * P * 24, nums=P, depth=4)
# Diagnostics (lib.rs:2027-2036): 6 entries; refs <= 3*D_env + 2,120 (text_budget_env large_refs:
# default 3 refs; 2,091 sites-mult at 4 refs; one site at 32) <= the packet's 4*D_env + 3(1+l).
REFS = 4 * DENV + 3 * (1 + l)
DIAGS = VF(arr=DENV + REFS, obj=DENV, ent=6 * DENV, strb=DIAGENV, keyb=6 * DENV * 16, depth=4)
# contract_evidence (the preview tree) and the fixed members: G3's PREVIEW facts (ordinary_caps)
# and 24 KiB of fixed strings, adopted as basis (not re-derived here).
PREVIEW = VF(arr=3 * m + 2 * g, obj=3 + m + g, ent=9 + 15 * m + 2 * g,
             strb=m * (128 + 1024 + 3 * 120) + (2 * m + g) * 128 + g * (128 + 64), keyb=(9 + 15 * m + 2 * g) * 40,
             nums=10 * m, depth=6)
FIXED = VF(arr=64, obj=24, ent=160, strb=24 * 1024, keyb=160 * 40, nums=60, depth=5)
ENV = ROWS + DIAGS + PREVIEW + FIXED
# successor_envelope (retained_wire.rs:740-773): recovery_method (15 B key, 41 B METHOD) on <= P
# rows; the selected diagnostic (id 30+ID+9, code 27, "info", SELECTED_MESSAGE 272, source 20,
# one ref <= ID); identity 64 / profile 31 replace shorter strings (<= 2*TOK).
SUCC_DELTA = (VF(ent=P, strb=P * 41, keyb=P * 15)
              + VF(arr=2, obj=1, ent=6, strb=(30 + ID + 9) + 27 + 4 + 272 + 20 + ID, keyb=40)
              + VF(strb=2 * TOK))
ENV_S = ENV + SUCC_DELTA
SUCC_DELTA_PKT = (VF(ent=P, strb=P * 41, keyb=P * 15)
                  + VF(arr=2, obj=1, ent=6, strb=(30 + ID + 9) + 26 + 4 + 300 + 20 + ID, keyb=6 * 16) + VF(strb=2 * TOK))
ENV_S_PKT = ENV + SUCC_DELTA_PKT
def ENV_S_of(pkt): return ENV_S_PKT if pkt else ENV_S
# The diagnostics array of a to_value tree has capacity == len (with_capacity), so the one push
# reallocates to 2*len (RawVec::grow_amortized): growth = D_env slots.
ENV_GROWTH_RV87 = s("Value") * DENV
ENV_GROWTH_PKT = s("Value") * (pushcap(DENV + 1) - DENV)

# ----------------------------------------------------------------------------- the receipt body
NB, BUILDS, RECORDS, ATTEMPTS, REFUSALS = n, 7, 4, 4, 3 * N
SOURCE = (rec(1, obj=4, ent=23, strb=TOK + ID + 3 * SHA, nums=3)            # case_source :896-987 (checked)
          + rec(n, slots=4, ent=4, strb=ID + 3 * BITS, nums=2)
          + rec(m, slots=4, ent=15, strb=ID + 9 * BITS, nums=6)
          + rec(sp, slots=1, ent=6, strb=TOK + BITS, nums=4)
          + rec(g, slots=1, ent=4, strb=ID, nums=3)
          + rec(NB, slots=1, ent=3, nums=1) + VF(arr=n + m, nums=n + m)
          + rec(Q, slots=1, obj=3, ent=11, strb=3 * TOK, nums=4)
          + rec(3 * m, slots=1, ent=4, strb=TOK + BITS, nums=2)
          + rec(g, slots=7, ent=5, nums=2) + VF(arr=2 * sp, nums=2 * sp)
          + rec(k, slots=2, obj=2, ent=5, strb=TOK + BITS, nums=3)
          + rec(l, slots=1, obj=2, ent=7, strb=ID + TOK + BITS, nums=4)
          + rec(m, slots=1, obj=2, ent=13, strb=TOK + 10 * BITS, nums=1))
REASON = VF(obj=4, ent=16, strb=8 * TOK, keyb=16 * KB, nums=4)
PHYSICAL = (rec(RECORDS, slots=1, obj=9, ent=81, strb=6 * TOK + 3 * BITS, nums=60) + REASON.x(RECORDS)
            + rec(RECORDS * 3 * NB, slots=1, ent=3, strb=2 * BITS, nums=1)
            + rec(RECORDS * REFUSALS, slots=1, obj=2, ent=7, strb=4 * TOK, nums=2))   # block_refusal :433-450 (checked)
LOGICAL = rec(ATTEMPTS, slots=4, obj=7, ent=22, strb=8 * TOK, nums=10) + REASON.x(ATTEMPTS)
RUN = rec(1, obj=4, ent=20, strb=4 * TOK, nums=12) + rec(14, slots=1, ent=2, strb=TOK, nums=1) + PHYSICAL + LOGICAL
SELECTION = (rec(1, ent=24, strb=3 * SHA + 6 * BITS + 2 * TOK, nums=4)      # selection :1130-1185 (checked)
             + rec(P, slots=1, ent=2, strb=RID + BITS)
             + rec(N, slots=1, ent=2, strb=ID + TOK)
             + rec(m, slots=1, ent=6, strb=ID + 5 * BITS)
             + rec(12 * NB, slots=1, ent=5, strb=4 * BITS + TOK, nums=1))
CASE = rec(1, obj=4, ent=16, strb=4 * TOK + ID + SHA, nums=4) + RUN + SELECTION
PROOF = (rec(1, obj=12, ent=40, strb=8 * TOK, nums=30) + rec(2, slots=20, obj=37, ent=96, strb=12 * TOK, nums=40)
         + rec(P, slots=1, obj=2, ent=4, strb=TOK + 2 * BITS, nums=1) + rec(NB, slots=1, ent=3, nums=2) + VF(arr=16, nums=16))
# product_attempt members (:1264-1290): member, result, work objects, numeric() (:249-252: one
# object + 10 e.count objects of 2 entries, 7 entry slots) and 4 work e.count objects.
PREP_RV87 = (rec(m, slots=1 + 6 + 7 + 5 + 9 + 7, obj=18, ent=47, strb=TOK + 18 * BITS + 12 * TOK, nums=12)
             + rec(9 * m, obj=2, ent=6, strb=3 * TOK + 2 * BITS))
PREP_PKT = rec(m, slots=1 + 6 + 7 + 5 + 9, obj=4, ent=14, strb=TOK + 18 * BITS, nums=8) + rec(9 * m, obj=2, ent=5, strb=3 * TOK + 2 * BITS)
OPERATIONAL = rec(2 * m, slots=1 + 10 + 3, obj=3, ent=12, strb=2 * TOK + 16 * BITS, nums=3)   # operational :659-676
OPERATIONAL_PKT = rec(2 * m, slots=1 + 10 + 3, obj=3, ent=10, strb=2 * TOK + 13 * BITS, nums=3)
def attempt(prep, op): return rec(1, obj=8, ent=40, strb=14 * TOK, nums=20) + prep + op + PROOF
ORDINARY = rec(1, obj=6, ent=20, strb=2 * TOK + ID + 4 * L_DIAGID) + VF(arr=DENV, strb=DENV * L_DIAGID)  # ordinary_value :1412-1434
CALLS = rec(1, slots=4, obj=3, ent=11, strb=2 * TOK, nums=6) + rec(1, slots=2, obj=2, ent=7, strb=SHA + TOK, nums=4)
BUILDS_V = rec(BUILDS, slots=1, obj=4, ent=31, strb=3 * TOK, nums=26) + REASON.x(BUILDS)
MATERIAL = rec(1, slots=2, obj=2, ent=4, strb=TOK) + rec(8, slots=1, obj=3, ent=7, strb=ID + 2 * BITS + 2 * TOK, nums=1)
TOP = rec(1, obj=4, ent=24, strb=11 * TOK + 2 * SHA, nums=8) + rec(1, slots=1, ent=7, strb=TOK, nums=4)   # finish :1436-1455 (checked)
def body(prep, op):
    v = TOP + CASE + SOURCE + MATERIAL + CALLS + BUILDS_V + ORDINARY + attempt(prep, op); v.depth = 9; return v
BODY = body(PREP_RV87, OPERATIONAL)
BODY_PKT = body(PREP_PKT, OPERATIONAL_PKT)
RECEIPT = BODY + VF(obj=1, ent=2, strb=SHA, keyb=2 * KB)
SUCC = ENV_S + RECEIPT + VF(ent=1, keyb=17)
PREP_PAYLOAD = rec(1, obj=2, ent=8, strb=2 * TOK + SHA) + rec(m, slots=1 + 6 + 7 + 5, ent=4, strb=18 * BITS, nums=1)
RAW = VF(arr=2 * RAWV, obj=RAWV, ent=RAWV, strb=2 * RAWS, keyb=2 * RAWK, nums=RAWV, depth=17)   # T02 basis
INVOC_VF = RAW + VF(obj=1, ent=2, strb=TOK, keyb=2 * KB)

# ----------------------------------------------------------------------------- T18.1 staged copy
# FrozenCandidate::staged_envelope (retained_product.rs:3799-3803): MechanicsEnvelope::clone
# (exact capacities) + apply_prepared_overlay (:3757-3783: values in place, Number clones, two
# LocatedQuantity clones replacing the old ones). ResultItem.source_result_refs is a Vec<String>:
# its backing (<= 4 * s(String) per row) is outside Text(row), which counts content only.
PREVIEW_T = (s("Value") * (3 * m + 2 * g) + node("String,Value") * (3 + m + g + (9 + 15 * m + 2 * g) // 5)
             + b(m * (128 + 1024 + 3 * 120) + (2 * m + g) * 128 + g * (128 + 64) + (9 + 15 * m + 2 * g) * 40))
STAGED_PKT = (s("MechanicsEnvelope") + s("ResultItem") * P + b(P * ROW) + s("Diagnostic") * DENV + b(DIAGENV)
              + PREVIEW_T + b(64 * 1024))
STAGED = STAGED_PKT + s("String") * (4 * P)

# ----------------------------------------------------------------------------- T16
LOCALS16_PKT = (s("PreparedAttemptView") + b(8 * (4 * m + P + 64)) + s("RowBinding") * P + hashreq("(&str,&T)", P)
                + b(P * RID) + s("String") * P + b(P * RID) + s("ProductRecipe") * P)
# row_ids: collect::<Option<Vec<String>>> (lower size_hint 0) grows by doubling: pushcap(P) slots.
LOCALS16 = LOCALS16_PKT + s("String") * (pushcap(P) - P)
MEMBERS = BODY.tree()
# serialize_selected_from (:1500-1575): case_v = json!({.. "run":run_v .. "selection":selection_v})
# deep-copies run_v and selection_v (json! `$other:expr` => to_value(&$other), macros.rs:278-279);
# the originals stay owned by the caller frame until it returns, i.e. through finish().
THIRD = RUN.tree() + SELECTION.tree()
def build16(pkt=False):
    """pkt=True: the packet's laws (pushcap growth, no third copies, its BODY grammar, exact
    row_ids), to show this implementation reproduces the packet; pkt=False: RV87's corrected laws."""
    eg = ENV_GROWTH_PKT if pkt else ENV_GROWTH_RV87
    loc = LOCALS16_PKT if pkt else LOCALS16
    bd = BODY_PKT if pkt else BODY
    third = L() if pkt else THIRD
    return {
        "P1": ENV_S_of(pkt).tree() + eg + loc + SOURCE.tree() * 2 + hash_route(SOURCE, ID) + PREP_PAYLOAD.tree(),
        "P2": ENV_S_of(pkt).tree() + eg + loc + bd.tree() + third + bd.tree() + hash_route(ENV_S_of(pkt), L_PUB),
        "P3": ENV_S_of(pkt).tree() + eg + loc + bd.tree() + third + bd.tree() + hash_route(bd, L_DIAGID) + (L() if pkt else b(128)),
        "P4": ENV_S_of(pkt).tree() + eg + loc + bd.tree() + third + bd.tree() * 2 + VF(obj=1, ent=2, strb=SHA).tree(),
    }
# ----------------------------------------------------------------------------- T17
def static_facts(path):
    d = json.load(open(path))
    f = dict(arr=0, obj=0, ent=0, strb=0, keyb=0, nums=0)
    def w(v):
        if isinstance(v, dict):
            f["obj"] += 1; f["ent"] += len(v)
            for kk, xx in v.items():
                f["keyb"] += len(kk.encode()); w(xx)
        elif isinstance(v, list):
            f["arr"] += len(v)
            for xx in v: w(xx)
        elif isinstance(v, str):
            f["strb"] += len(v.encode())
        elif not isinstance(v, bool) and v is not None:
            f["nums"] += 1
    w(d)
    return f
STATIC_FILES = ["schemas/physics_source_recovery.schema.json", "schemas/retained_precision_mp_v2.schema.json",
                "fixtures/results/retained_precision_prepared_ordinary_v1.json",
                "fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json",
                "fixtures/results/semantic_contract_v0_2.json", "fixtures/results/semantic_contract_v0_3_precision_1.json",
                "fixtures/results/semantic_contract_v0_3_physics_1.json", "fixtures/results/semantic_contract_v0_3_load_reference_1.json",
                "fixtures/results/semantic_contract_v0_3_load_reference_source_1.json",
                "fixtures/results/semantic_contract_v0_3_preview_physics_1.json",
                "fixtures/results/semantic_contract_v0_3_physics_source_1.json",
                "fixtures/results/semantic_contract_v0_3_source_blocks_1.json", "schemas/source_block_recovery.schema.json"]
import hashlib
STATICS = L(); statics = {}
for p_ in STATIC_FILES:
    raw_bytes = open(os.path.join(SNAP, p_), "rb").read()
    f_ = static_facts(os.path.join(SNAP, p_))
    # serde_json::from_str: push-built arrays (<= 6 x slots), insert-built maps, exact owned strings/keys
    STATICS = STATICS + s("Value") * (6 * f_["arr"]) + node("String,Value") * (f_["obj"] + f_["ent"] // 5) + b(f_["strb"] + f_["keyb"])
    statics[p_] = {"sha256": hashlib.sha256(raw_bytes).hexdigest(), "bytes": len(raw_bytes), **f_}
ENVP = ENV_S
SETS_PKT = (L({"Node(&str,())": tnodes(P) + 2 * tnodes(DENV) + tnodes(n + m + g)})
            + hashreq("(&str,&Value)", P) * 3 + hashreq("&str", m + g) * 8 + b(8 * 3 * P)
            + s("RowClassification") * pushcap(P) + b(P * (2 * ID + 64))
            + L({"Node((String,String),())": tnodes(k)}) + b(k * 2 * (ID + 8))
            + s("&Value") * (8 * P) + b(8 * 4 * n))
# RowClassification.basis_ref is a cloned Value object (one BTreeMap node, keys ref_type/ref_id).
SETS = SETS_PKT + node("String,Value") * P
G5C = VF(arr=2 * P, obj=P, ent=2 * P, strb=P * (RID + BITS), keyb=P * 20).tree() * 2
G8_VALUES = SOURCE.tree() * 2 + VF(arr=Q, obj=3 * Q, ent=11 * Q, strb=3 * TOK * Q, keyb=11 * Q * KB).tree() * 2
G8_BYTES = b(2 * pushcap(36_742 + 4_986, 8) + 4 * 16 * m)
def receipt_of(bd): return bd + VF(obj=1, ent=2, strb=SHA, keyb=2 * KB)
def succ_of(pkt):
    return ENV_S_of(pkt) + receipt_of(BODY_PKT if pkt else BODY) + VF(ent=1, keyb=17)
def build17(pkt=False):
    bd = BODY_PKT if pkt else BODY
    envp = ENV_S_of(pkt)
    sets = SETS_PKT if pkt else SETS
    return {
        "V1": hash_route(bd, L_DIAGID),
        # retained_precision.rs:591-603: public = source.clone() (whole successor), remove the
        # receipt, then hash(publication): the max of the two moments.
        "V2_clone": succ_of(pkt).tree(),
        "V2_hash": envp.tree() + hash_route(envp, L_PUB),
        "V3": SOURCE.tree() + hash_route(SOURCE, ID) + PREP_PAYLOAD.tree() + hash_route(PREP_PAYLOAD, 64),
        "V4": sets + G5C,
        "V5": succ_of(pkt).tree() + sets,
        "V6": envp.tree() + hash_route(INVOC_VF, ID) + G8_VALUES + G8_BYTES + sets,
    }
INVOC = INVOC_VF.tree()
# ----------------------------------------------------------------------------- T18.2, T19
# ReservedNotice::reserve (lib.rs:3034-3051): id format! (estimated capacity 2*42, then doubling:
# <= 2*(30+ID+12)), code 30, "info" 4, message try_reserve_exact 196, source 20, refs Vec 1 slot
# + case id <= ID; diagnostics try_reserve_exact(1): growth <= 1 slot, moving the old backing.
NOTICE_RV87 = b(2 * (30 + ID + 12) + 30 + 4 + 196 + 20 + ID) + s("String") + s("Diagnostic")
NOTICE_MOV = s("Diagnostic") * DENV
NOTICE_PKT = (s("Diagnostic") + b(2 * (30 + ID + 12)) + b(30) + b(8) + b(2 * (232 + 64)) + b(24) + s("String") + b(ID)
              + s("String") * 4 + s("Diagnostic") * (pushcap(DENV + 1) - DENV))
T19 = b(8192) + s("ThreadPacketOutput")

# ----------------------------------------------------------------------------- inputs from the chain
ORD = J(f"ordinary_caps.caps{SUF}.out.json")
T25J = J(f"t25_g4.caps.eps2{SUF}.out.json")
PROD = J(f"producer_caps.caps{SUF}.out.json")
TBX = J(f"text_budget_X.caps{SUF}.out.json"); TBW = J(f"text_budget_W.caps{SUF}.out.json")
TB = J(f"text_budget.caps{SUF}.out.json")
def resum(tb):
    return sum(rw["req"] for rw in tb["rows"]), max([rw["bytes"] for rw in tb["rows"] if rw["mult"]] + [0])
TAV = {"X": TBX["total_text_requested_bytes"], "W": TBW["total_text_requested_bytes"]}
TAV_CHECK = {"X": resum(TBX), "W": resum(TBW), "whole": resum(TB)}
TXT_MOV = TXT["TAV_text_moving"] - TXT["TAV_text_requested"]
O_ALL = {md: L(parse_sym(ORD[md]["O_req"]["symbolic"])) for md in ("sparse", "dense")}
T25S = L(parse_sym(T25J["T25_requested"]["symbolic"]))
O_BASE = {md: O_ALL[md] + T25S * (-1) for md in O_ALL}
class Given:
    """An input term known by its evaluated bytes (at the ASSUMED strides) and its stride-free
    byte constant (from its symbolic form); the stride part scales in the sensitivity runs."""
    def __init__(self, assumed, sym):
        self.assumed = assumed; self.const = parse_sym(sym).get("1", 0)
    def ev(self, scale=1.0, text=None):
        return int(round(self.const + (self.assumed - self.const) * scale))
    def stride_part(self):
        return Given(self.assumed - self.const, "0")
T11 = Given(PROD["terms"]["T11"], PROD["symbolic_terms"]["T11"])
T12_15 = Given(sum(PROD["terms"][t] for t in ("T12", "T13", "T14", "T15")),
               " + ".join(PROD["symbolic_terms"][t] for t in ("T12", "T13", "T14", "T15")))
T25_MOV = T25J["moving_extra"]["assumed_bytes"]
HELPER_MOV = 131_072 * 64

def compose(scale=1.0, corrected=True, tav_extra=None):
    tx = TXT
    E = lambda x: x.ev(scale, tx)
    pkt = not corrected
    st16v = {kk: E(v) for kk, v in build16(pkt).items()}
    st17 = build17(pkt)
    t16 = max(st16v.values())
    st17v = {kk: E(v) for kk, v in st17.items()}
    v2 = max(st17v["V2_clone"], st17v["V2_hash"])
    t17_stage = max(v2, *(st17v[x] for x in ("V1", "V3", "V4", "V5", "V6")))
    t17 = t17_stage + E(s("Validation") + s("RowClassification") * pushcap(P))
    staged = E(STAGED if corrected else STAGED_PKT)
    succ = E(succ_of(pkt).tree()); invoc = E(INVOC); stat = E(STATICS)
    notice = E(NOTICE_RV87 if corrected else NOTICE_PKT); notice_mov = E(NOTICE_MOV)
    t19 = E(T19); t11 = E(T11); t1215 = E(T12_15)
    mov16 = last_growth(ENV_S_of(pkt))
    out = {}
    TAVx = dict(TAV)
    if tav_extra:
        for br_ in ("X", "W"):
            TAVx[br_] = TAV[br_] + tav_extra[br_]
    for md in ("sparse", "dense"):
        TAV_ = TAVx
        ob = E(O_BASE[md]); t25 = E(T25S)
        ph = {
            "X1": (ob + t25 + TAV_["X"] + t11, max(T25_MOV, TXT_MOV, HELPER_MOV)),
            "X2": (ob + E((BODY_PKT if pkt else BODY).tree()) + TAV_["X"] + t11 + notice + t19, max(TXT_MOV, HELPER_MOV, notice_mov)),
            "W1": (ob + TAV_["W"] + t11, max(TXT_MOV, HELPER_MOV)),
            "W2": (ob + TAV_["W"] + t11 + t1215 + notice, max(TXT_MOV, HELPER_MOV, notice_mov)),
            "W3": (ob + TAV_["W"] + t11 + t1215 + notice + staged + t16, max(TXT_MOV, HELPER_MOV, mov16)),
            "W4": (ob + TAV_["W"] + t11 + t1215 + notice + succ + invoc + stat + t17, max(TXT_MOV, HELPER_MOV, mov16)),
            "W5": (ob + TAV_["W"] + t11 + t1215 + notice + succ + stat + t19, max(TXT_MOV, HELPER_MOV)),
        }
        rows = {kk: {"requested": rq, "moving": mv, "E_mov_plus_R": rq + mv + R, "fraction": round((rq + mv + R) / M, 4)} for kk, (rq, mv) in ph.items()}
        worst = max(rows, key=lambda kk: rows[kk]["E_mov_plus_R"])
        out[md] = {"components": {"O_base": ob, "T25": t25, "TAV_X": TAV["X"], "TAV_W": TAV["W"], "T11": t11, "T12_15": t1215,
                                  "T16": t16, "T17": t17, "STAGED": staged, "SUCC": succ, "INVOC": invoc, "STATICS": stat,
                                  "N1": notice, "T19": t19},
                   "T16_stages": st16v, "T17_stages": st17v, "phases": rows, "max_phase": worst,
                   "max": rows[worst]["E_mov_plus_R"], "max_fraction": rows[worst]["E_mov_plus_R"] / M,
                   "margin_to_0.9M": MARGIN - rows[worst]["E_mov_plus_R"], "margin_to_M": M - rows[worst]["E_mov_plus_R"]}
    return out

base = compose(1.0)
res = {"basis": "NUM 6796050f47 (core files = a634ac8b53; G4 arithmetic pinned b1f80234dc)", "l": l, "P": P, "Q": Q, "k": k,
       "D_env": DENV, "Text(row)": ROW, "Text(diag_env)": DIAGENV, "L_PUB": L_PUB,
       "facts": {"ENV_S": vars(ENV_S), "BODY": vars(BODY), "BODY_pkt_grammar": vars(BODY_PKT), "SUCC_text": SUCC.text(),
                 "ENV_S_text": ENV_S.text(), "BODY_text": BODY.text()},
       "TAV_resum_check": TAV_CHECK, "TAV": TAV, "TXT_MOV": TXT_MOV,
       "statics": statics,
       "corrections_bytes": {
           "ENV push growth (exact-capacity array doubles)": ENV_GROWTH_RV87.ev(1, TXT) - ENV_GROWTH_PKT.ev(1, TXT),
           "run_v + selection_v originals live through finish()": THIRD.ev(1, TXT),
           "staged copy: source_result_refs Vec backings": (STAGED + STAGED_PKT * -1).ev(1, TXT),
           "row_ids collect capacity": (LOCALS16 + LOCALS16_PKT * -1).ev(1, TXT),
           "BODY numeric()/count objects (per BODY tree)": BODY.tree().ev(1, TXT) - BODY_PKT.tree().ev(1, TXT),
           "N1 reserve: exact try_reserve_exact(1) vs push headroom": NOTICE_RV87.ev(1, TXT) - NOTICE_PKT.ev(1, TXT),
           "classification basis_ref nodes (V4-V6 only)": (SETS + SETS_PKT * -1).ev(1, TXT)},
       "base": base,
       "packet_laws_reproduction": compose(1.0, corrected=False),
       "stride_plus10": {md: {"max": v["max"], "fraction": round(v["max_fraction"], 4), "phase": v["max_phase"],
                              "margin_to_0.9M": v["margin_to_0.9M"]} for md, v in compose(1.1).items()},
       "stride_minus10": {md: {"max": v["max"], "fraction": round(v["max_fraction"], 4), "phase": v["max_phase"]} for md, v in compose(0.9).items()},
       }
if IDSCAN:
    extra = {br_: IDSCAN[br_]["under_count_total"] for br_ in ("X", "W")}
    def brief(o): return {md: {"phase": v["max_phase"], "max": v["max"], "fraction": round(v["max_fraction"], 4),
                               "margin_to_0.9M": v["margin_to_0.9M"]} for md, v in o.items()}
    res["with_result_id_class_fix"] = {"tav_extra": extra, "strides_1.0": brief(compose(1.0, True, extra)),
                                       "strides_1.1": brief(compose(1.1, True, extra))}
# stride-borne bytes at the maximum phase (dense, W3): requested split into stride and byte parts
def split(x):
    sp_ = x.stride_part(); return sp_.ev(1, TXT), x.ev(1, TXT) - sp_.ev(1, TXT)
res["input_eval_check"] = {"O_req": {md: [O_ALL[md].ev(1, TXT), ORD[md]["O_req"]["assumed_bytes"]] for md in O_ALL},
                           "T25": [T25S.ev(1, TXT), T25J["T25_requested"]["assumed_bytes"]]}
w3 = {"O_base(dense)": split(O_BASE["dense"]), "T11": split(T11), "T12_15": split(T12_15), "N1": split(NOTICE_RV87),
      "STAGED": split(STAGED), "T16(P2)": split(build16()["P2"]), "TAV_W": (0, TAV["W"])}
res["W3_dense_stride_vs_bytes"] = {kk: {"stride_bytes": a, "byte_bytes": c} for kk, (a, c) in w3.items()}
res["W3_dense_stride_total"] = sum(a for a, _ in w3.values())
print(json.dumps(res, indent=1))
