"""RV83 independent cap arithmetic for I65 U4 G2 (stdlib only, integer arithmetic).

Written from the RESIDUALS/DOMAIN text and the cited source, not from I65's
caps_arithmetic.py. Strides marked ASSUMED are the same illustrative 64-bit values
I65 used, so that totals can be compared; they are NOT qualified layout facts.

Sections:
  1. derived counts at the D1 caps (with the monotonicity argument per formula);
  2. T02 raw backing at the caps under three node-size choices;
  3. BTree Leaf_up/Internal_up and hashbrown HashReq from the installed-source laws;
  4. T07 deep legacy-exact: (a) the RESIDUALS T07 table as written, (b) the I65
     script's total, (c) the table plus the owners RV83 found missing;
  5. f64 spelling maxima, checked numerically with Python's shortest repr;
  6. T22 values at the caps.
"""
import json, math, random, struct

def up(x, a):
    return -(-x // a) * a

def min_non_zero_cap(s):
    return 8 if s == 1 else (4 if s <= 1024 else 1)

def pushcap(s, h):
    """RawVec amortized growth from empty, h single pushes (installed raw_vec grow_amortized)."""
    cap = 0
    for length in range(1, h + 1):
        if length > cap:
            cap = max(2 * cap, length, min_non_zero_cap(s))
    return cap

out = {}

# ---- 1. derived counts --------------------------------------------------------
n = m = g = 32
r = l = 192
s_ruled = 192
s = min(s_ruled, g)          # one LinearSupport::spring per family=="spring" support (PP/lib.rs:6756-6772)
N = 6 * n
F = N                        # before validation F <= N
k = N                        # unique rigid DOFs <= N
derived = {
    "N=6n": N, "F<=N": F, "k<=N": k, "s_eff<=g": s,
    "Q=7n+30m+s+k+2g": 7*n + 30*m + s + k + 2*g,
    "P_final<=7n+51m+8g+3": 7*n + 51*m + 8*g + 3,
    "C<=144m+s": 144*m + s,
    "Z<=min(N^2,144m+s)": min(N*N, 144*m + s),
    "E<=78m+s": 78*m + s,
    "H<=F(F+1)/2": F*(F+1)//2,
    "source_count=144m+s+l": 144*m + s + l,
    "Fn=42m+N+s+6g": 42*m + N + s + 6*g,
    "N^2": N*N,
    "with s at the ruled 192 instead (if s<=g were false)": {
        "C": 144*m + 192, "Q": 7*n + 30*m + 192 + k + 2*g, "source_count": 144*m + 192 + l,
        "Fn": 42*m + N + 192 + 6*g},
}
assert derived["source_count=144m+s+l"] <= 16384 and N <= 256
out["1_derived"] = derived
out["1_monotonicity"] = (
    "Every derived formula is a nonnegative-coefficient polynomial or min/max of such in "
    "(n,m,g,s,k,l,F); min and max of nondecreasing functions are nondecreasing; F(F+1)/2 "
    "is nondecreasing for F>=0; PushCap(s,h) is nondecreasing in h (each step takes max "
    "with the previous capacity). So substituting caps bounds every in-domain input.")

# ---- 2. T02 --------------------------------------------------------------------
sV = 32  # ASSUMED s(Value)
caps_raw = dict(values=16384, arr_cap=32768, str_cap=131072, key_cap=131072, digest=128)
objects = entries = caps_raw["values"]
nodes_upper = objects + entries // 5
def t02(node):
    return (sV * caps_raw["arr_cap"] + caps_raw["key_cap"] + caps_raw["str_cap"]
            + node * nodes_upper + caps_raw["digest"])
# A census-consistent joint bound: every value except the root is an array element
# or an object entry, so entries <= values-1; objects <= values.
joint_nodes = 16384 + (16383 // 5)
out["2_T02"] = {
    "node_count_upper(objects+floor(entries/5))": nodes_upper,
    "with 632/728 (ASSUMED DWARF)": t02(728),
    "with Leaf_up/Internal_up 640/736": t02(736),
    "joint census bound nodes (entries<=values-1)": joint_nodes,
    "monotone": "linear with nonnegative coefficients in every census fact",
}

# ---- 3. BTree and hashbrown laws ------------------------------------------------
def leaf_up(sk, ak, sv, av):
    A = max(8, ak, av)
    return up(8, A) + up(2, A) + up(2, A) + up(11 * sk, A) + up(11 * sv, A)
def internal(leaf, leaf_align):
    return up(up(leaf, 8) + 12 * 8, max(leaf_align, 8))
def leaf_exact_sorted(sk, ak, sv, av):
    """repr(Rust) sorts by alignment; this is the layout rustc produces in practice
    (NOT a guarantee): fields in decreasing alignment, no gaps."""
    fields = sorted([(8, 8), (2, 2), (2, 2), (11 * sk, ak), (11 * sv, av)], key=lambda f: -f[1])
    off = 0
    for size, align in fields:
        off = up(off, align) + size
    return up(off, max(8, ak, av))
roster = {"(String,Value)": (24, 8, 32, 8), "(String,Quantity)": (24, 8, 32, 8),
          "(&String,SetValZST)": (8, 8, 0, 1), "(String,())": (24, 8, 0, 1),
          "(usize,Vec<&str>)": (8, 8, 24, 8)}
btree = {}
for name, (sk, ak, sv, av) in roster.items():
    lu = leaf_up(sk, ak, sv, av)
    le = leaf_exact_sorted(sk, ak, sv, av)
    btree[name] = {"Leaf_up": lu, "Internal_up": internal(lu, max(8, ak, av)),
                   "Leaf_sorted_practice": le, "Internal_sorted_practice": internal(le, max(8, ak, av))}
out["3_btree_ASSUMED_KV_SIZES"] = btree

GROUP_WIDTH = 8  # hashbrown 0.17.1 control/group/neon.rs: uint8x8_t
def buckets(size, cap):
    if cap == 0:
        return 0
    if cap < 15:
        min_cap = 7 if (GROUP_WIDTH, size) in [(8, 0), (8, 1)] else 3
        if GROUP_WIDTH == 16 and size <= 1:
            min_cap = 14
        c = max(min_cap, cap)
        return 4 if c < 4 else (8 if c < 8 else 16)
    adjusted = cap * 8 // 7
    return 1 << (adjusted - 1).bit_length()          # next_power_of_two
def hashreq(size, align, b):
    ctrl_align = max(align, GROUP_WIDTH)
    return up(size * b, ctrl_align) + b + GROUP_WIDTH
hb = {}
for kk in [1, 3, 4, 7, 8, 14, 15, 28, 29, 56, 57, 112, 113, 192]:
    b = buckets(24, kk)
    hb[kk] = {"buckets(s=24)": b, "HashReq(24,8,b)": hashreq(24, 8, b)}
out["3_hashbrown_s24_a8"] = hb

# ---- 4. T07 ---------------------------------------------------------------------
A = {"s(Vec)": 24, "s(String)": 24, "s(Expansion)": 32, "s(Ratio)": 64,
     "s(SC)": 24, "s(FC)": 40, "s(BW)": 120, "s(FD)": 128, "s(AffineTerm)": 32,
     "s(QFP)": 96, "s(QP)": 96, "s(RP)": 72, "s(RFP)": 72, "s(MR)": 696,
     "s(SpringAction)": 48, "s(SupportActions)": 128, "s(Snapshot)": 232}  # ASSUMED (I65's values)
T = 256
eB = 8 * pushcap(8, T)          # 2048
E = A["s(Expansion)"] + eB      # one Expansion header + children
idb = 128
Fn = derived["Fn=42m+N+s+6g"]; C = derived["C<=144m+s"]; Z = derived["Z<=min(N^2,144m+s)"]
ft = l + N
def mat(stride, rr, cc):
    return A["s(Vec)"] * rr + stride * rr * cc
names = 8 + n + 6*l + 6*g + r + s + 12*m
bits = 8 + 3*n + 4*l + 4*g + r + 4*N + 3*s + 325*m
ident_len = names * (6 * idb + 3) + bits * 21 + 4       # '[' '[' ... ']' ',' '[' ... ']' ']'
# serde_json::to_string -> to_vec: Vec::with_capacity(128), then RawVec amortized growth
# new = max(2*cap, len+additional): final capacity <= max(128, 2*len). Doubling-only value
# (every write smaller than the current spare) is 128*2^k >= len.
ident_cap = max(128, 2 * ident_len)
ident_cap_doubling = 128
while ident_cap_doubling < ident_len:
    ident_cap_doubling *= 2
builder = A["s(String)"] * pushcap(24, names) + names * idb + 8 * pushcap(8, bits)
descriptors = Fn * (A["s(FD)"] + 2 * idb) + 16384 * max(A["s(Vec)"], 8, A["s(AffineTerm)"])
snapshot_text = (A["s(Snapshot)"] + mat(8, N, N) + 3 * 8 * N + A["s(SC)"] * C
                 + ft * (A["s(FC)"] + idb) + mat(8, N, N) + mat(8, N, N) + idb)
# I65 script's snapshot uses 8N+8N+16N for the three vectors (prescribed is 16-byte tuples)
snapshot_script = (A["s(Snapshot)"] + mat(8, N, N) + 8*N + 8*N + 16*N + A["s(SC)"] * C
                   + A["s(FC)"] * ft + ft * idb + mat(8, N, N) + mat(8, N, N) + idb)
k_f = (A["s(Vec)"] * N + A["s(Expansion)"] * N * N + 8 * (4*Z + 2*C)
       + A["s(Expansion)"] * N + 8 * (4*N + 2*ft))
blocks = 24 * pushcap(24, N) + N * 32       # each block vec![i] then push -> cap 4 (8-byte)
witnesses = A["s(BW)"] * pushcap(A["s(BW)"], N) + N * (16 + 3 * eB + eB)
transients = mat(8, N, N) + 2 * N + 8 * N + 2 * eB
response = 2 * N * A["s(Ratio)"] + 4 * N * eB + 6 * E
values = Fn * (A["s(Ratio)"] + 2 * eB)
evaluate_transient = N * E + 12 * eB
projections = (A["s(QFP)"] * pushcap(A["s(QFP)"], Fn) + A["s(QP)"] * pushcap(A["s(QP)"], 2 * N))
def retained(snap):
    return (snap + witnesses + 2 * N * (A["s(Ratio)"] + 2 * eB) + 2 * N * A["s(RP)"]
            + descriptors + Fn * (A["s(Ratio)"] + 2 * eB) + Fn * A["s(RFP)"] + 16)
output = (16 * N + m * (A["s(MR)"] + idb) + s * (A["s(SpringAction)"] + idb)
          + g * (A["s(SupportActions)"] + idb))
table = {
    "sources_descriptors_#1": descriptors, "identity_json_len": ident_len, "identity_builder": builder,
    "snapshot_#1": snapshot_script, "context_k_f": k_f, "blocks": blocks, "witnesses": witnesses,
    "new_charged_transients": transients, "plan_descriptors_#2": descriptors,
    "response": response, "functionalset_values": values, "evaluate_transient": evaluate_transient,
    "projections": projections, "retained_set(snapshot_#2,witnesses_#2,...,descriptors_#3)": retained(snapshot_script),
    "selected_output": output,
}
table_total = sum(table.values())
missing = {
    "Snapshot_#1.identity (Context::new_charged: identity.into(); exact_boundary.rs:477)": ident_len,
    "Snapshot_#2.identity (retain: self.context.source.clone(); exact_boundary.rs:~1429)": ident_len,
    "Sources.identity capacity slack (to_string growth; doubling-only value)": ident_cap_doubling - ident_len,
}
out["4_T07"] = {
    "identity_names": names, "identity_bits": bits, "identity_json_len": ident_len,
    "identity_string_capacity_upper": ident_cap, "identity_string_capacity_doubling_only": ident_cap_doubling, "descriptors_one_copy": descriptors,
    "table_rows_as_written_ASSUMED": table, "table_total_ASSUMED": table_total,
    "I65_script_total_ASSUMED": 36474868,
    "I65_script_descriptor_copies": "3*2 in the sum plus 1 inside RetainedFunctionalSet = 7 copies; the table says 3",
    "missing_owners_ASSUMED": missing, "missing_total": sum(missing.values()),
    "table_plus_missing_ASSUMED": table_total + sum(missing.values()),
}

# ---- 5. f64 spellings -------------------------------------------------------------
def shortest_digits(x):
    """Python repr is shortest round-trip, same digit count as core's shortest strategy."""
    rep = repr(abs(x))
    mant, _, exp = rep.partition("e")
    exp = int(exp) if exp else 0
    if "." in mant:
        ip, fp = mant.split(".")
    else:
        ip, fp = mant, ""
    digits = (ip + fp).lstrip("0")
    point = len(ip) + exp           # position of decimal point relative to digit string start
    if ip.strip("0") == "":         # mantissa like 0.000123
        lead = len(fp) - len(fp.lstrip("0"))
        point = -lead + exp
        digits = fp.lstrip("0")
    digits = digits.rstrip("0") or "0"
    return digits, point            # value = 0.d1d2... * 10^point
def display_len(x):                 # positional, min_precision 0
    d, p = shortest_digits(x)
    sign = 1 if math.copysign(1, x) < 0 else 0
    if d == "0":
        return sign + 1
    if p <= 0:
        return sign + 2 + (-p) + len(d)          # "0." zeros digits
    if p >= len(d):
        return sign + p                          # digits then zeros, no ".0"
    return sign + len(d) + 1
def debug_len(x):
    a = abs(x)
    d, p = shortest_digits(x)
    sign = 1 if math.copysign(1, x) < 0 else 0
    if a != 0 and (a < 1e-4 or a >= 1e16):
        e = p - 1
        return sign + len(d) + (1 if len(d) > 1 else 0) + 1 + len(str(e))
    if d == "0":
        return sign + 3                          # "0.0"
    if p <= 0:
        return sign + 2 + (-p) + len(d)
    if p >= len(d):
        return sign + p + 2                      # ".0" appended (min_precision 1)
    return sign + len(d) + 1
def from_bits(b):
    return struct.unpack("<d", struct.pack("<Q", b))[0]
random.seed(83)
cands = [from_bits(1), -from_bits(1), from_bits(2), from_bits(3), -from_bits(5),
         -2.2250738585072014e-308, -2.225073858507201e-308, -1.7976931348623157e308,
         -1.2345678901234567e-5, -9.999999999999998e15, -1.2345678901234567e15, -0.00012345678901234567]
cands += [-from_bits(random.randrange(1, 1 << 52)) for _ in range(20000)]
cands += [-from_bits(random.randrange(0x0010000000000000, 0x0020000000000000)) for _ in range(20000)]
cands += [-from_bits(random.randrange(1, 0x7FF0000000000000)) for _ in range(40000)]
out["5_f64_spellings"] = {
    "display_max_observed": max(display_len(x) for x in cands),
    "display_of_-5e-324": display_len(-from_bits(1)),
    "debug_max_observed": max(debug_len(x) for x in cands),
    "debug_of_-2.2250738585072014e-308": debug_len(-2.2250738585072014e-308),
    "debug_positional_max_observed": max(debug_len(x) for x in cands if 1e-4 <= abs(x) < 1e16),
    "samples": len(cands),
    "argument": ("Display: L = sign + 1 - e + nd for |x|<1 with nd<=17 and e>=-324; subnormal "
                 "precision makes nd <= e+325, so L <= 327. Debug/LowerExp: sign + 17 digits "
                 "+ '.' + 'e-308' = 24."),
}

# ---- 6. T22 -----------------------------------------------------------------------
out["6_T22"] = {
    "6n": N, "u32(nodes)": n, "3m": 3*m, "u32(supports)": g, "n*n (source.rs:454)": N*N,
    "free*(free+1) (source.rs:356-358; packet says free*n)": F*(F+1),
    "6*supports (final_case.rs:1132)": 6*g,
    "ceil_sqrt r*r (bound.rs:996-1006, u128): r<=ceil(sqrt(F))": math.isqrt(F - 1) + 1,
    "u32::MAX": 2**32 - 1,
}
print(json.dumps(out, indent=1))
