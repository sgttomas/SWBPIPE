"""I65 U4 G4: T16 publication, T17 precommit reader, T18 transfer/fallback/reserve, T19 Direct
completion, and the T24 composition under the restated admission law (RR "RV84 on U4 G3", S-5)
with the 0.9 M margin rule (RR "U4 G3 verified"). Stdlib only; symbolic per layout atom, evaluated
with g3lib.ASSUMED plus the G4 strides below (ILLUSTRATIVE; G5 evaluates in-build).

Basis: NUM b1f80234dc (U1 merged at 4a13e369b9, U3 grant 1 merged at bee3dc07ca), read from a
git-archive snapshot. Source citations are at that revision.

Inputs (this directory): text_closure.<which>.json and text_budget{,_X,_W,_env}.<which>.out.json
(T08 on the repaired graph), ordinary_caps.<which>.out.json (O), t25_g4.<which>.eps<e>.out.json
(T25), producer_caps.<which>.out.json (T11-T15), composite_text.<which>.json (L_max), and the 13
result_export static JSON files (read from the snapshot given as argv[3]).
Usage: python3 g4_caps.py <caps|milestone> <eps> <snapshot projects/chirality-piping dir>
Environment: G4_CAPS (JSON) overrides caps for the sensitivity runs (same keys as t25_g4.py).
"""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from g3lib import X, B, s, V, A, Xc, HashReq, pushcap, total, ASSUMED, j3cap, tree_nodes

which = sys.argv[1]
EPS = int(sys.argv[2])
SNAP = sys.argv[3]
H = os.path.dirname(os.path.abspath(__file__))
CAPS = json.loads(os.environ.get("G4_CAPS", "{}"))
def J(name):
    return json.load(open(os.path.join(H, name)))
TXT = J(f"text_closure.{which}.json")["atoms"]
TB = {b: J(f"text_budget{b}.{which}.out.json") for b in ("", "_X", "_W", "_env")}
ORD = J(f"ordinary_caps.{which}.out.json")
T25 = J(f"t25_g4.{which}.eps{EPS}.out.json")
PROD = J(f"producer_caps.{which}.out.json")["terms"]
COMP = J(f"composite_text.{which}.json")["classes"]

if which == "caps":
    n, m, g, sp, r, l = (CAPS.get(k_, v_) for k_, v_ in (("n", 32), ("m", 32), ("g", 32), ("s", 32), ("r", 192), ("l", 192)))
else:
    n, m, g, sp, r, l = 2, 1, 4, 3, 6, 3
N = 6 * n; k = min(N, r)
P = 7 * n + 51 * m + 8 * g + 3                     # P_final
Q = 7 * n + 30 * m + sp + k + 2 * g
ID, RID, KB, BITS, TOK, SHA = CAPS.get("ident", 128), 1024, 32, 16, 64, 64
RAWV, RAWS, RAWK = CAPS.get("raw_values", 16_384), CAPS.get("raw_string_bytes", 65_536), CAPS.get("raw_key_bytes", 65_536)
ROW, DENV, DIAGENV = TXT["Text(row)"], TXT["D_env"], TXT["Text(diag_env)"]
L_PUB = max(COMP["message_int"], TB[""]["largest_single_site_bytes"])   # G5 part 2 (RV84 C-N1(a)): the integrity message can reach the largest reached site (2,599,962 B)
L_ROW = 2173                                        # the longest id/literal class (composite_text static)
L_DIAGID = 2330                                     # longest reached `diagnostic:` id template (text_budget rows)
M = 4_026_531_840
R = 64 * 2**20

# G4 strides (ILLUSTRATIVE, as g3lib.ASSUMED; G5 evaluates every atom in-build)
ASSUMED.update({"s(RowBinding)": 64, "s(ProductRecipe)": 16, "s(PreparedAttemptView)": 512,
                "s(RowClassification)": 128, "s(MechanicsEnvelope)": 640, "s(ThreadPacketOutput)": 2048,
                "Node(&str,())": ASSUMED.get("Node(&str,())", 288), "s((&str,&Value))": 32, "s(&str)": 16,
                "s(&Value)": 8, "s(Validation)": 96, "s(W1Fallback)": 48, "s((String,String))": 48})


class VF:
    """Facts of one serde_json Value tree: array slots, objects, entries, string and key bytes,
    numbers, depth. tree() is an exact-capacity build (to_value, json!, clone); parsed() a
    checked parse (push-built arrays: only non-empty ones allocate, max(4, 2h) <= 6h; seen keys
    cloned, J6)."""
    def __init__(self, arr=0, obj=0, ent=0, strb=0, keyb=0, nums=0, depth=8):
        self.arr, self.obj, self.ent, self.strb, self.keyb, self.nums, self.depth = arr, obj, ent, strb, keyb, nums, depth
    def __add__(self, o):
        return VF(self.arr + o.arr, self.obj + o.obj, self.ent + o.ent, self.strb + o.strb, self.keyb + o.keyb,
                  self.nums + o.nums, max(self.depth, o.depth))
    def scale(self, c):
        return VF(self.arr * c, self.obj * c, self.ent * c, self.strb * c, self.keyb * c, self.nums * c, self.depth)
    def values(self):
        return self.arr + self.ent + 1
    def tree(self):
        return s("Value") * self.arr + X({"Node(String,Value)": self.obj + self.ent // 5}) + B(self.strb + self.keyb)
    def parsed(self):
        return s("Value") * (6 * self.arr) + X({"Node(String,Value)": self.obj + self.ent // 5}) + B(self.strb + 2 * self.keyb)
    def text(self):
        return EPS * self.strb + self.keyb + 8 * self.values() + 24 * self.nums


def hash_route(vf, lmax):
    """domain_hash (retained_wire.rs:194-197; result_export source_blocks.rs:182-187): the json!
    wrapper's deep copy, the to_string text (J3: <= max(128, 2e)), the checked parse tree with seen
    sets, the canonical output (<= max(8, 2e)), the per-string to_string temporary (RV84 S-3) and
    the parser's escape scratch, all coexisting at the render (G3 T25 route + S-3)."""
    e = vf.text()
    return (vf.tree() + B(j3cap(e)) + vf.parsed() + HashReq("String", 16) * vf.depth
            + B(2 * max(24, RID) + 2080 + max(8, 2 * e) + vf.depth * 8 * 16 + 144 + 128)
            + B(max(128, 2 * (EPS * lmax + 2))) + B(2 * lmax))


def hash_last_growth(vf):
    return B(j3cap(vf.text()) // 2)


rows = {}
# =========================================================================== envelope facts
# The ordinary MechanicsEnvelope as a Value (G3 T25 env_vf, at the G4 text atoms); on branch W it
# carries contract_evidence (the preview tree, PREVIEW in ordinary_caps).
ENV = (VF(arr=P + 4 * P, obj=3 * P, ent=15 * P, strb=P * ROW, keyb=15 * P * 24, nums=P, depth=4)
       + VF(arr=DENV + 4 * DENV + 3 * (1 + l), obj=DENV, ent=6 * DENV, strb=DIAGENV, keyb=6 * DENV * 16, depth=4)
       + VF(arr=3 * m + 2 * g, obj=3 + m + g, ent=9 + 15 * m + 2 * g,
            strb=m * (128 + 1024 + 3 * 120) + (2 * m + g) * 128 + g * (128 + 64), keyb=(9 + 15 * m + 2 * g) * 40,
            nums=10 * m, depth=6)
       + VF(arr=64, obj=24, ent=160, strb=24 * 1024, keyb=160 * 40, nums=60, depth=5))
# successor deltas (successor_envelope, retained_wire.rs:740-773): recovery_method on every case
# row; the selected diagnostic (id <= 30+ID+9, code, severity, the fixed message, source, one ref);
# identity/profile strings replaced (<= 2 x TOK more).
SUCC_DELTA = (VF(ent=P, strb=P * 41, keyb=P * 15)
              + VF(arr=2, obj=1, ent=6, strb=(30 + ID + 9) + 26 + 4 + 300 + 20 + ID, keyb=6 * 16)
              + VF(strb=2 * TOK))
ENV_S = ENV + SUCC_DELTA

# =========================================================================== receipt body facts
# Per member of finish()'s body (retained_wire.rs:1436-1458), at the caps: count x per-record
# grammar facts read from the builders (case_source :896-987, physical :1002-1040, logical
# :1041-1091, selection :1122-1177, product_attempt/proof_trace :1230-1294, invocation_arrays
# :1375-1402, material_basis :857-894, ordinary_value :1404-1426). Keys <= 32 B per entry (the
# longest receipt key is 30 B); strings: bits 16 B, tokens <= 64 B, sha 64 B, input ids <= ID,
# result ids <= RID, diagnostic ids <= L_DIAGID.
def rec(cnt, slots=0, obj=1, ent=0, strb=0, nums=0, depth=4):
    return VF(arr=cnt * slots, obj=cnt * obj, ent=cnt * ent, strb=cnt * strb, keyb=cnt * ent * KB, nums=cnt * nums, depth=depth)
NB = n                                               # bodies <= n
BUILDS, RECORDS, ATTEMPTS = 7, 4, 4
REFUSALS = 3 * N                                     # bound refusals per record: Uc and S per block (blocks <= F <= N) plus the shared build's
SOURCE = (rec(1, obj=4, ent=23, strb=TOK + ID + 3 * SHA, nums=3)
          + rec(n, slots=1 + 3, ent=4, strb=ID + 3 * BITS, nums=2)                    # id_maps.nodes
          + rec(m, slots=1 + 3, ent=15, strb=ID + 9 * BITS, nums=6)                   # id_maps.members
          + rec(sp, slots=1, ent=6, strb=TOK + BITS, nums=4)                          # id_maps.springs
          + rec(g, slots=1, ent=4, strb=ID, nums=3)                                   # id_maps.support_ids
          + rec(NB, slots=1, ent=3, nums=1) + VF(arr=n + m, nums=n + m)              # body_membership
          + rec(Q, slots=1, obj=3, ent=11, strb=3 * TOK, nums=4)                      # layout
          + rec(3 * m, slots=1, ent=4, strb=TOK + BITS, nums=2)                       # stations
          + rec(g, slots=1 + 6, ent=5, nums=2) + VF(arr=2 * sp, nums=2 * sp)          # supports
          + rec(k, slots=2, obj=2, ent=5, strb=TOK + BITS, nums=3)                    # constraints
          + rec(l, slots=1, obj=2, ent=7, strb=ID + TOK + BITS, nums=4)               # nodal_terms
          + rec(m, slots=1, obj=2, ent=13, strb=TOK + 10 * BITS, nums=1))             # section_terms
REASON = VF(obj=4, ent=16, strb=8 * TOK, keyb=16 * KB, nums=4)                       # an attempt reason subtree
PHYSICAL = (rec(RECORDS, slots=1, obj=9, ent=81, strb=6 * TOK + 3 * BITS, nums=60)
            + REASON.scale(RECORDS)
            + rec(RECORDS * 3 * NB, slots=1, ent=3, strb=2 * BITS, nums=1)          # verification resolution/theta/bound
            + rec(RECORDS * REFUSALS, slots=1, obj=2, ent=7, strb=4 * TOK, nums=2))  # bound_refusals
LOGICAL = rec(ATTEMPTS, slots=1 + 3, obj=7, ent=22, strb=8 * TOK, nums=10) + REASON.scale(ATTEMPTS)
RUN = (rec(1, obj=4, ent=20, strb=4 * TOK, nums=12) + rec(14, slots=1, ent=2, strb=TOK, nums=1)  # origin, terminal, caches
       + PHYSICAL + LOGICAL)
SELECTION = (rec(1, ent=24, strb=3 * SHA + 6 * BITS + 2 * TOK, nums=4)
             + rec(P, slots=1, ent=2, strb=RID + BITS)                                # absolute_verified / not_covered
             + rec(N, slots=1, ent=2, strb=ID + TOK)                                  # input_derived_dofs
             + rec(m, slots=1, ent=6, strb=ID + 5 * BITS)                             # section_terms
             + rec(12 * NB, slots=1, ent=5, strb=4 * BITS + TOK, nums=1))            # body scales, resolution, theta, bound, floor, kind lists
CASE = rec(1, obj=4, ent=16, strb=4 * TOK + ID + SHA, nums=4) + RUN + SELECTION
PROOF = (rec(1, obj=12, ent=40, strb=8 * TOK, nums=30) + rec(2, slots=20, obj=37, ent=96, strb=12 * TOK, nums=40)  # lanes
         + rec(P, slots=1, obj=2, ent=4, strb=TOK + 2 * BITS, nums=1)                # projection_outcomes
         + rec(NB, slots=1, ent=3, nums=2) + VF(arr=16, nums=16))                    # coverage, capacities
# G5 part 2 (RV87 N-1(d), RV87's corrected grammar): each member's work carries numeric() (one object,
# 10 e.count objects of 2 entries, 7 entry slots; retained_wire.rs:249-252) and 4 work e.count objects
# (:1264-1290); operational records carry 12 entries (:659-676).
PREPARATION = (rec(m, slots=1 + 6 + 7 + 5 + 9 + 7, obj=18, ent=47, strb=TOK + 18 * BITS + 12 * TOK, nums=12)
               + rec(9 * m, obj=2, ent=6, strb=3 * TOK + 2 * BITS))                    # conversions
OPERATIONAL = rec(2 * m, slots=1 + 10 + 3, obj=3, ent=12, strb=2 * TOK + 16 * BITS, nums=3)
ATTEMPT = rec(1, obj=8, ent=40, strb=4 * TOK + 10 * TOK, nums=20) + PREPARATION + OPERATIONAL + PROOF
ORDINARY = rec(1, obj=6, ent=20, strb=2 * TOK + ID + 4 * L_DIAGID) + VF(arr=DENV, strb=DENV * L_DIAGID)  # diagnostic_refs
CALLS = rec(1, slots=4, obj=3, ent=11, strb=2 * TOK, nums=6) + rec(1, slots=2, obj=2, ent=7, strb=SHA + TOK, nums=4)
BUILDS_V = rec(BUILDS, slots=1, obj=4, ent=31, strb=3 * TOK + 19 * 0, nums=26) + REASON.scale(BUILDS)
MATERIAL = rec(1, slots=2, obj=2, ent=4, strb=TOK) + rec(8, slots=1, obj=3, ent=7, strb=ID + 2 * BITS + 2 * TOK, nums=1)
TOP = rec(1, obj=4, ent=24, strb=6 * TOK + 5 * TOK + 2 * SHA, nums=8) + rec(1, slots=1, ent=7, strb=TOK, nums=4)
BODY = (TOP + CASE + SOURCE + MATERIAL + CALLS + BUILDS_V + ORDINARY + ATTEMPT)
BODY.depth = 9
RECEIPT = BODY + VF(obj=1, ent=2, strb=SHA, keyb=2 * KB)                             # {"body","receipt_sha256"}
SUCC = ENV_S + RECEIPT + VF(ent=1, keyb=17)                                          # the successor Value
PREP_PAYLOAD = rec(1, obj=2, ent=8, strb=2 * TOK + SHA) + rec(m, slots=1 + 6 + 7 + 5, ent=4, strb=18 * BITS, nums=1)
RAW_VF = VF(arr=2 * RAWV, obj=RAWV, ent=RAWV, strb=2 * RAWS, keyb=2 * RAWK, nums=RAWV, depth=17)
INVOC_VF = RAW_VF + VF(obj=1, ent=2, strb=TOK, keyb=2 * KB)                         # {"request": raw, "solver_mode"}

# =========================================================================== typed staging copy
# U3 FrozenCandidate::staged_envelope (retained_product.rs:3788-3792): MechanicsEnvelope::clone
# (exact capacities) plus the overlay (values and maxima numbers, no growth beyond the clone).
PREVIEW_T = (s("Value") * (3 * m + 2 * g) + X({"Node(String,Value)": 3 + m + g + (9 + 15 * m + 2 * g) // 5})
             + B(m * (128 + 1024 + 3 * 120) + (2 * m + g) * 128 + g * (128 + 64) + (9 + 15 * m + 2 * g) * 40))
STAGED = (s("MechanicsEnvelope") + Xc("ResultItem", P) + B(P * ROW) + Xc("Diagnostic", DENV) + B(DIAGENV)
          + PREVIEW_T + B(64 * 1024)
          + s("String") * (4 * P))          # G5 part 2 (RV87 N-1(b)): ResultItem.source_result_refs Vec backings
rows["T18.1 staged typed envelope copy (U3 D-b; live during T16 only)"] = STAGED

# =========================================================================== T16 publication
# serialize_frozen -> serialize_selected_from (retained_wire.rs:1474-1563 at b1f80234dc); finish
# :1436-1458. Stage maxima: the live set at each hash render.
LOCALS16 = (s("PreparedAttemptView") + B(8 * (4 * m + P + 64))                       # the typed trace view
            + Xc("RowBinding", P) + HashReq("(&str,&T)", P) + B(P * RID)              # bind_rows
            + s("String") * pushcap(P) + B(P * RID) + Xc("ProductRecipe", P))          # row_ids (G5 part 2, RV87 N-1(c): collect::<Option<Vec<_>>> doubles), recipes
# G5 part 2 (RV87 N-1(a)): to_value builds the diagnostics array with_capacity(len), so the selected
# diagnostic's push doubles it: D_env more slots
ENV_GROWTH = s("Value") * DENV
# G5 part 2 (RV87 S-1): case_v = json!({.."run":run_v,.."selection":selection_v}) deep-copies both,
# and the originals live in serialize_selected_from until it returns, after finish()'s P2-P4
THIRD = RUN.tree() + SELECTION.tree()
MEMBERS = BODY.tree()                                                                 # finish()'s member Values (moved in, then copied by json!)
st16 = {
    "P1 env to_value + successor deltas + members + source identity hash":
        ENV_S.tree() + ENV_GROWTH + LOCALS16 + SOURCE.tree() * 2 + hash_route(SOURCE, ID) + PREP_PAYLOAD.tree(),
    "P2 body json! + hash(publication)":
        ENV_S.tree() + ENV_GROWTH + LOCALS16 + MEMBERS + THIRD + BODY.tree() + hash_route(ENV_S, L_PUB),
    "P3 hash(body)":
        ENV_S.tree() + ENV_GROWTH + LOCALS16 + MEMBERS + THIRD + BODY.tree() + hash_route(BODY, L_DIAGID),
    "P4 retained_precision insert (third body copy)":
        ENV_S.tree() + ENV_GROWTH + LOCALS16 + MEMBERS + THIRD + BODY.tree() * 2 + VF(obj=1, ent=2, strb=SHA).tree(),
}
evv = dict(ASSUMED)
def ev(x):
    return x.ev(evv)
T16_name, T16 = max(st16.items(), key=lambda kv: ev(kv[1]))
T16_mov = hash_last_growth(ENV_S)                                                     # the publication text's last growth
rows["T16 publication peak (" + T16_name + ")"] = T16

# =========================================================================== T17 precommit reader
# retained_precision::validate (result_export retained_precision.rs:4263-4313) over the successor
# and the invocation Value built by U3 (lib.rs:2989: json! deep-copies the raw request; dropped at :2995).
def static_tree(path):
    d = json.load(open(path))
    f = dict(arr=0, obj=0, ent=0, strb=0, keyb=0, nums=0)
    def w(v):
        if isinstance(v, dict):
            f["obj"] += 1; f["ent"] += len(v)
            for k_, x_ in v.items():
                f["keyb"] += len(k_.encode()); w(x_)
        elif isinstance(v, list):
            f["arr"] += len(v)
            for x_ in v: w(x_)
        elif isinstance(v, str):
            f["strb"] += len(v.encode())
        elif not isinstance(v, bool) and v is not None:
            f["nums"] += 1
    w(d)
    v = VF(**f)
    # serde_json::from_str: arrays push-built (<= 6 x slots), maps, owned strings and keys
    return s("Value") * (6 * v.arr) + X({"Node(String,Value)": v.obj + v.ent // 5}) + B(v.strb + v.keyb), f
STATIC_FILES = ["schemas/physics_source_recovery.schema.json", "schemas/retained_precision_mp_v2.schema.json",
                "fixtures/results/retained_precision_prepared_ordinary_v1.json",
                "fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json",
                "fixtures/results/semantic_contract_v0_2.json", "fixtures/results/semantic_contract_v0_3_precision_1.json",
                "fixtures/results/semantic_contract_v0_3_physics_1.json", "fixtures/results/semantic_contract_v0_3_load_reference_1.json",
                "fixtures/results/semantic_contract_v0_3_load_reference_source_1.json",
                "fixtures/results/semantic_contract_v0_3_preview_physics_1.json",
                "fixtures/results/semantic_contract_v0_3_physics_source_1.json",
                "fixtures/results/semantic_contract_v0_3_source_blocks_1.json", "schemas/source_block_recovery.schema.json"]
STATICS = X(); static_facts = {}
for p_ in STATIC_FILES:
    t_, f_ = static_tree(os.path.join(SNAP, p_))
    STATICS = STATICS + t_; static_facts[p_] = f_
rows["T17.0 the 13 result_export OnceLock statics (process-lifetime, first use counted here)"] = STATICS
INVOC = INVOC_VF.tree()
ENVP = ENV_S                                                                          # project()/public: the successor without its receipt
SETS = (X({"Node(&str,())": tree_nodes(P) + 2 * tree_nodes(DENV) + tree_nodes(n + m + g)})   # rowids, diagnostic ids x2, model ids
        + HashReq("(&str,&Value)", P) * 3 + HashReq("&str", m + g) * 8 + B(8 * 3 * P)      # preview evidence maps and vectors
        + Xc("RowClassification", pushcap(P)) + B(P * (2 * ID + 64))                       # classifications (+ basis_ref clones)
        + X({"Node((String,String),())": tree_nodes(k)}) + B(k * 2 * (ID + 8))             # numeric_cases prescribed set
        + s("&Value") * (8 * P) + B(8 * 4 * n)                                             # rows_for vectors, extents, raw
        + X({"Node(String,Value)": P}))   # G5 part 2 (RV87 N-2): RowClassification.basis_ref, a cloned Value object per row
# G5 part 2 (RV87 N-2): the walkers allocate per object: objects() (retained_precision.rs:1694) pushes a
# reference per object of a work owner (<= BODY); located() (:1722) pushes (path.clone(), &Map) per object
# of an attempt (accounting_rules, :1850; ATTEMPT), the path <= 9 segments of <= 32 bytes
OBJ_WALK = (s("&Value") * pushcap(BODY.obj) + (s("Vec") + s("&Value")) * pushcap(ATTEMPT.obj)
            + s("String") * (9 * ATTEMPT.obj) + B(ATTEMPT.obj * 9 * 32) + s("String") * 9 + B(9 * 32))
ASSUMED.setdefault("Node((String,String),())", 600); evv.update(ASSUMED)
G5C = VF(arr=2 * P, obj=P, ent=2 * P, strb=P * (RID + BITS), keyb=P * 2 * 10).tree() * 2   # absolutes/uncovered + json! copies
G8_VALUES = SOURCE.tree() * 2 + VF(arr=Q, obj=3 * Q, ent=11 * Q, strb=3 * TOK * Q, keyb=11 * Q * KB).tree() * 2
G8_BYTES = B(2 * pushcap(36_742 + 4_986, 8) + 4 * 16 * m)                             # K4SRC/K4STF identity buffers (E_src, E_stf)
st17 = {
    "V1 G1 receipt hash": hash_route(BODY, L_DIAGID),
    # public = source.clone(), then remove("retained_precision") drops the receipt part before
    # the hash (retained_precision.rs:591-596): the clone peak is SUCC, the hash peak ENVP + route
    "V2 G1 public clone + hash(publication)": max(SUCC.tree(), ENVP.tree() + hash_route(ENVP, L_PUB), key=lambda x: x.ev(dict(ASSUMED))),
    "V3 G1 source and preparation hashes": SOURCE.tree() + hash_route(SOURCE, ID) + PREP_PAYLOAD.tree() + hash_route(PREP_PAYLOAD, 64),
    "V4 G3-G6 working sets": SETS + G5C + OBJ_WALK,
    "V5 G7 project + preview evidence": SUCC.tree() + SETS + OBJ_WALK,
    "V6 G8 projected + invocation hash + model rebuild": ENVP.tree() + hash_route(INVOC_VF, ID) + G8_VALUES + G8_BYTES + SETS + OBJ_WALK,
}
T17_name, T17v = max(st17.items(), key=lambda kv: ev(kv[1]))
T17 = T17v + Xc("Validation", 1) + Xc("RowClassification", pushcap(P))
T17_mov = hash_last_growth(ENVP) if "V2" in T17_name else B(j3cap(INVOC_VF.text()) // 2)
rows["T17 precommit reader peak (" + T17_name + ")"] = T17
rows["T17.1 the invocation Value (U3 lib.rs:2989, live for the precommit to :2995)"] = INVOC

# =========================================================================== T18 transfer, fallback, reserve
# N1 (RR "U3 grant 1 verified", R-2): after W1 work ran, exactly one RETAINED_PRECISION_UNAVAILABLE
# info diagnostic is appended; its space is reserved before W1 starts. Budget: the Diagnostic's
# strings pre-built (id <= 30 + ID + 12, code, severity, the fixed message plus C1:68's reason and
# detail token <= 64, source, one affected ref), and the diagnostics Vec reserved to D_env + 1.
# G5 part 2 (RV87 N-1(e); U3 grant 1b ReservedNotice::reserve, PP lib.rs:3041-3058 at 1e323058f3):
# the id format! (<= 2*(30+ID+12)), code 30, "info" 4, the message try_reserve_exact(196), source 20,
# the refs Vec (one String slot) and the case id; diagnostics try_reserve_exact(1) grows by <= 1 slot
NOTICE = B(2 * (30 + ID + 12) + 30 + 4 + 196 + 20 + ID) + s("String") + s("Diagnostic")
T18_RESERVE = NOTICE
T18_RESERVE_MOV = s("Diagnostic") * DENV                                              # the old backing during the reserve
rows["T18.2 N1 notice reserve (pre-built strings + diagnostics Vec reserve)"] = T18_RESERVE
rows["T18.3 transfer and fallback (moves only; W1Fallback allocates nothing new: a precommit code String is moved)"] = Xc("W1Fallback", 1)

# =========================================================================== T19 Direct completion
# on_reserved_stack (lib.rs:2907-2919): std::thread::scope + Builder::stack_size(R).spawn_scoped:
# Arc<ScopeData>, the Thread handle Arc, the result Packet Arc (holding the moved output), the
# boxed main closure, the child's TLS registrations. The output (RetainedPreviewOutput: typed
# envelope + successor carrier) moves through the packet and join; nothing is copied.
# s(ThreadPacketOutput) (illustrative 2,048): s(MechanicsEnvelope) + the inline
# Option<RetainedAdmissionReport> (a Copy struct of about 70 words; inline whether Some or None, so
# it was already in s(output) at b1f80234dc) + Option<Result<RetainedSuccessor, W1Fallback>> + the
# Packet and Result/Option discriminants: about 1.3 KB, rounded up. G5 measures it.
T19 = B(8192) + s("ThreadPacketOutput")
rows["T19 Direct completion (thread spawn heap; moves)"] = T19

# =========================================================================== T24 composition
out = {"which": which, "EPS": EPS, "basis": "NUM b1f80234dc", "M": M, "R": R, "margin_0.9M": int(0.9 * M),
       "facts": {"ENV_S": vars(ENV_S), "BODY": vars(BODY), "SOURCE": vars(SOURCE), "SUCC_text": SUCC.text(),
                 "ENV_S_text": ENV_S.text(), "BODY_text": BODY.text(), "INVOC_text": INVOC_VF.text(), "statics": static_facts},
       "rows": {k_: {"symbolic": v_.show(), "assumed_bytes": ev(v_)} for k_, v_ in rows.items()},
       "T16_stages": {k_: ev(v_) for k_, v_ in st16.items()}, "T17_stages": {k_: ev(v_) for k_, v_ in st17.items()}}
TAV_X, TAV_W = TB["_X"]["total_text_requested_bytes"], TB["_W"]["total_text_requested_bytes"]
TXT_MOV = TXT["TAV_text_moving"] - TXT["TAV_text_requested"]
SUCC_B = ev(SUCC.tree())   # G5 part 2 (RV84 C-N3(c)): the unused RECEIPT_X is removed
HELPER_MOV = 131_072 * 64                                                       # I54 BOUND:76 maximum helper growth
T07_MOV = 1_797_413 if which == "caps" else 0

# ---- G5 part 2: the profile tree. Every term is kept as a linear form over layout atoms, and every
# maximum (stages, phases, moving candidates) is taken over its candidates IN THE BUILD (G5), never
# chosen here under ASSUMED strides. g5_profile.py turns this tree into retained_memory.rs's
# generated profile; the evaluation below (ASSUMED) is only the cross-check of that transcription.
def parse_sym(text):
    out = X()
    if text.strip() in ("", "0"):
        return out
    for part in text.split(" + "):
        if "*" in part and not part.lstrip("-").isdigit():
            c, a_ = part.split("*", 1)
            out = out + X({a_: int(c)})
        else:
            out = out + B(int(part))
    return out
T25_KEY = "T25 selected source-blocks finalization (RESIDUALS_G3 T25)"
forms = {}
def form(name, x):
    forms[name] = {k_: v_ for k_, v_ in x.items()}
    return name
for mode in ("sparse", "dense"):
    form(f"O_base_{mode}", total(parse_sym(v_["symbolic"]) for k_, v_ in ORD[mode]["rows"].items() if k_ != T25_KEY))
for k_, v_ in T25["stage_forms"].items():
    form("T25_" + k_.split(" ")[0], X(v_))
form("T25_moving", X(T25["moving_form"]))
for t_ in ("T11", "T12", "T13", "T14", "T15"):
    form(t_, parse_sym(J(f"producer_caps.{which}.out.json")["symbolic_terms"][t_]))
# the gate bounds (API_G4.md §2): T11 without its late capture (G-B), and the ordinary seed (G-C)
_prows = J(f"producer_caps.{which}.out.json")["rows"]
form("T11_late_capture", parse_sym(next(v_["symbolic"] for k_, v_ in _prows.items() if k_.startswith("T11.5"))))
form("T11_ordinary_seed", parse_sym(next(v_["symbolic"] for k_, v_ in _prows.items() if k_.startswith("T11.4"))))
for k_, v_ in st16.items():
    form("T16_" + k_.split(" ")[0], v_)
form("T16_moving", T16_mov)
form("T17_V1", st17["V1 G1 receipt hash"])
form("T17_V2_clone", SUCC.tree())
form("T17_V2_hash", ENVP.tree() + hash_route(ENVP, L_PUB))
for k_ in ("V3 G1 source and preparation hashes", "V4 G3-G6 working sets", "V5 G7 project + preview evidence",
           "V6 G8 projected + invocation hash + model rebuild"):
    form("T17_" + k_.split(" ")[0], st17[k_])
form("T17_output", Xc("Validation", 1) + Xc("RowClassification", pushcap(P)))
form("T17_moving_publication", hash_last_growth(ENVP))
form("T17_moving_invocation", B(j3cap(INVOC_VF.text()) // 2))
for name_, x_ in (("STAGED", STAGED), ("SUCC", SUCC.tree()), ("INVOC", INVOC), ("STATICS", STATICS), ("BODY", BODY.tree()),
                  ("NOTICE", NOTICE), ("NOTICE_moving", T18_RESERVE_MOV), ("T19", T19)):
    form(name_, x_)
for name_, c_ in (("TAV_X", TAV_X), ("TAV_W", TAV_W), ("TXT_moving", TXT_MOV), ("HELPER_moving", HELPER_MOV), ("T07_moving", T07_MOV)):
    form(name_, B(c_))
exprs = {
    "T25": {"sum": ["T25_carried_case", {"max": [n_ for n_ in forms if n_.startswith("T25_") and n_[4:5] in ("S", "I")]}]},
    "T12_T15": {"sum": ["T12", "T13", "T14", "T15"]},
    "T16": {"max": [n_ for n_ in forms if n_.startswith("T16_P")]},
    "T17": {"sum": [{"max": ["T17_V1", "T17_V2_clone", "T17_V2_hash", "T17_V3", "T17_V4", "T17_V5", "T17_V6"]}, "T17_output"]},
}
# G5 part 2 (RV84 C-N3(a), (b); ruled): T19's spawn heap is allocated before the observed run and the reader's
# process-lifetime statics are live after the first permitted invocation, so both count in every phase.
EVERY = ["T11", "T19", "STATICS"]
def phases_of(mode):
    ob = f"O_base_{mode}"
    w2 = [ob, "TAV_W", "T12_T15", "NOTICE"] + EVERY
    return {
        "X1 ordinary span with T25 (selected finalization)":
            ([ob, "T25", "TAV_X"] + EVERY, ["T25_moving", "TXT_moving", "HELPER_moving"]),
        "X2 X completion: retained receipt + reserve + Direct completion":
            ([ob, "BODY", "TAV_X", "NOTICE"] + EVERY, ["TXT_moving", "HELPER_moving", "NOTICE_moving"]),
        "W1 ordinary span (W1 not yet run)":
            ([ob, "TAV_W"] + EVERY, ["TXT_moving", "HELPER_moving", "T07_moving"]),
        "W2 G-B, G-C and the W1 phases (T12-T15) with the N1 reserve":
            (w2, ["TXT_moving", "HELPER_moving", "NOTICE_moving"]),
        "W3 publication (T16) with the staged copy":
            (w2 + ["STAGED", "T16"], ["TXT_moving", "HELPER_moving", "T16_moving"]),
        "W4 precommit validation (T17) with the successor and the invocation Value":
            (w2 + ["SUCC", "INVOC", "T17"], ["TXT_moving", "HELPER_moving", "T17_moving_publication", "T17_moving_invocation"]),
        "W5 transfer and Direct completion (T18, T19)":
            (w2 + ["SUCC"], ["TXT_moving", "HELPER_moving"]),
    }
vals = dict(ASSUMED); vals.update(J(f"producer_caps.{which}.out.json")["assumed"]); vals.update(T25["assumed"]); vals.update(evv); vals.update(TXT)
def evn(node):
    if isinstance(node, str):
        if node in exprs:
            return evn(exprs[node])
        return X(forms[node]).ev(vals)
    if "sum" in node:
        return sum(evn(c) for c in node["sum"])
    return max(evn(c) for c in node["max"])
modes = {}
tree_phases = {}
for mode in ("sparse", "dense"):
    ph = {}
    tree_phases[mode] = {}
    for name_, (req_, mov_) in phases_of(mode).items():
        rq = sum(evn(c) for c in req_); mv = max(evn(c) for c in mov_)
        ph[name_] = {"requested": rq, "moving_extra": mv, "E_mov": rq + mv, "E_mov_plus_R": rq + mv + R,
                     "fraction_of_M": round((rq + mv + R) / M, 4)}
        tree_phases[mode][name_] = {"requested": req_, "moving": mov_}
    worst = max(ph.items(), key=lambda kv: kv[1]["E_mov_plus_R"])
    modes[mode] = {"components": {k_: evn(k_) for k_ in ["O_base_" + mode, "T25", "TAV_X", "TAV_W", "T11", "T12_T15", "T16", "T17",
                                                           "STAGED", "SUCC", "INVOC", "STATICS", "NOTICE", "T19", "BODY"]},
                   "phases": ph, "admission_max": {"phase": worst[0], **worst[1]},
                   "margin_rule_0.9M": worst[1]["E_mov_plus_R"] <= int(0.9 * M), "fits_M": worst[1]["E_mov_plus_R"] <= M}
out["modes"] = modes
out["L"] = {"L_PUB": L_PUB, "L_DIAGID": L_DIAGID, "L_ROW": L_ROW}
tree = {"basis": "NUM 1e323058f3 (G5 part 1 code); text at l <= 128 on the R-4 graph", "which": which, "EPS": EPS,
        "M": M, "R": R, "forms": forms, "exprs": exprs, "phases": tree_phases, "L": {"L_PUB": L_PUB, "L_DIAGID": L_DIAGID},
        "text_atoms": {k_: v_ for k_, v_ in TXT.items()}, "assumed": {k_: v_ for k_, v_ in vals.items() if k_ not in TXT},
        "check": {mode: {"max": modes[mode]["admission_max"]["E_mov_plus_R"], "phase": modes[mode]["admission_max"]["phase"]}
                  for mode in modes}}
if os.environ.get("PROFILE_TREE"):
    json.dump(tree, open(os.environ["PROFILE_TREE"], "w"), indent=1, sort_keys=True)
print(json.dumps(out, indent=1))
