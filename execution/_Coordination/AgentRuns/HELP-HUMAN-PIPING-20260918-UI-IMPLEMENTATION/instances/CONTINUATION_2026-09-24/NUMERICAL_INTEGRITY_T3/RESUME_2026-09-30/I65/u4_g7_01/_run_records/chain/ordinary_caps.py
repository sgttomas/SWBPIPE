"""I65 U4 G3 T05: the accepted I54 ordinary formulas evaluated at the D1 caps, per mode.

Each family below transcribes one accepted row (I54 direct_ordinary_bound_02 BOUND.md
rows :52-81, W2_RETURN.md, and the correction_03 ADDENDUM.md, which replaces the audit,
residual, rigid, stress and formation-guard rows). The row id in each comment is the
I54 source of the formula. G3 adds: O-N (normalization, S-4; G4 adds RV83 R-3's provenance parse), T_resident (T03 at the
typed capacity caps), T07 (RESIDUALS_G3.md) and T25 (RESIDUALS_G3.md). Formulas take
counts as independent arguments; each is nondecreasing in every argument (monotonicity
lemma per family in ORDINARY.md), so the value at the cap vector bounds every D1 input.

Text atoms (Text(...), D) are closed by TEXT.md; their values are read from
text_closure.<which>.json. Layout atoms are evaluated with g3lib.ASSUMED (illustrative).
Usage: python3 ordinary_caps.py <caps|milestone>
"""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from g3lib import X, B, s, atom, V, A, Xc, Mat, HashReq, buckets, pushcap, appendcap, total, ASSUMED, textcap

which = sys.argv[1] if len(sys.argv) > 1 else "caps"
HERE = os.path.dirname(os.path.abspath(__file__))
if which == "caps":
    from g3lib import g4_caps as _gc
    _c = _gc(); n, m, g, sp, r, l = _c["n"], _c["m"], _c["g"], _c["s"], _c["r"], _c["l"]
    N = 6 * n; F = N; k = min(N, r)
    C = 144 * m + sp; Z = min(N * N, C); Zf = min(Z, F * F)
    E = 78 * m + sp; H = F * (F + 1) // 2; d = m + sp
    Ps = 4 * Z + 2 * C; Pd = 6 * Z + 2 * C          # sum PushCap(8,c) <= sum max(4,2c)
    mats = 8; pts = 16                                # model + request materials; points each
    P = 7 * n + 51 * m + 8 * g + 3; R0 = 7 * n + 51 * m + g + 3
    li_max = l
else:  # the I54 named-fixture facts (BOUND:87-93; ADDENDUM histogram) for cross-checks
    n, m, g, sp, r, l = 2, 1, 4, 3, 6, 3
    N = 12; F = 9; k = 3; C = 147; Z = 144; Zf = 81; E = 48; H = 45; d = 2
    Ps = 144 * 4; Pd = 141 * 4 + 3 * 4
    mats = 1; pts = 0
    P = 7 * n + 51 * m + 8 * g + 3; R0 = 7 * n + 51 * m + g + 3
    li_max = 1
ID = __import__("g3lib").g4_caps()["ident"] if which == "caps" else 128
EXP = 2048

def p8(h):          # ADDENDUM: f64 push capacity p(h) = max(4, next_pow2(h))
    if h <= 0:
        return 0
    c = 4
    while c < h:
        c *= 2
    return c
def Bf(h):          # ADDENDUM B(h) = 8 p(h)
    return 8 * p8(h)
def G(h):           # ADDENDUM G(h): old next backing during its geometric reallocation
    return 4 * p8(h) if p8(h) > 4 else 0
def Exp_req(h):
    return B(Bf(h) + Bf(max(h - 1, 0)))
def Exp_mov_extra(h):
    return B(G(h))
def Sort(t, h):     # ADDENDUM S1 stable-sort scratch upper: zero for h <= 20, else s(T)*max(h,48)
    return X() if h <= 20 else s(t) * max(h, 48)
def Adj(t):         # BOUND row 'Ordering / RCM'
    return s("Vec<usize>") * F + B(8 * (4 * F + 2 * t))

def value_tree(arr_elems, objects, entries, string_bytes, key_bytes):
    """serde_json Value tree upper (I54 BOUND:36-44, T2): array slots, BTree nodes, bytes."""
    return (s("Value") * arr_elems + X({"Node(String,Value)": objects + entries // 5})
            + B(string_bytes + key_bytes))
from g3lib import g4_caps as _gc2
_cc = _gc2()
RAW = value_tree(2 * _cc["raw_values"], _cc["raw_values"], _cc["raw_values"], 2 * _cc["raw_string_bytes"], 2 * _cc["raw_key_bytes"]) + B(128)   # T02 at the caps
TEXTF = os.path.join(HERE, f"text_closure.{which}.json")
TXT = json.load(open(TEXTF))["atoms"] if os.path.exists(TEXTF) else {}
DIAG = TXT.get("D", 0)
# preview evidence tree per case (preview_physics.rs:697-703, 298-304): extrema objects (<= m, 15 entries,
# 3 static strings <= 120 B, pipe id and result id), coverage lists (<= 2m ids), attribution (<= g ids,
# <= g withheld objects of 2 entries), intensified measures (D1.4: none); keys <= 40 bytes each
PREVIEW = value_tree(3 * m + 2 * g, 3 + m + g, 9 + 15 * m + 2 * g,
                     m * (128 + 1024 + 3 * 120) + (2 * m + g) * 128 + g * (128 + 64),
                     (9 + 15 * m + 2 * g) * 40)
T = {}   # text atoms (values from TEXT.md)
def tx(name):
    return atom("Text(" + name + ")")

fam = {}      # family -> X (requested)
mov = {}      # family -> X (largest single old backing inside the family)
phase = {}    # family -> ordinary phase name (BOUND:18-25)

def add(name, ph, req, mv=None):
    fam[name] = req; phase[name] = ph; mov[name] = mv or X()

# ---------------- T_resident: T03 typed request at the typed capacity caps --------------
strings = (mats * (7 + pts * 7) + 6 + 2 * n + 12 * m + 6 * g + r + 2 + 7 * l) if which == "caps" else 0
add("T_resident (T03 typed request, typed capacity caps)", "T_resident",
    (Xc("MaterialInput", mats) + Xc("TemperaturePoint", mats * pts) + Xc("PreviewNode", n)
     + Xc("PreviewPipe", m) + Xc("PreviewSupport", g) + Xc("String", r) + Xc("PreviewLoadCase", 1)
     + Xc("PrimitiveLoadInput", l) + Xc("Authored<Vec<ExpansionLawInput>>", 4)
     + B(strings * ID) + RAW) if which == "caps" else X())

# ---------------- R_raw: the captured raw request (T02; CapturedInvocation.raw + digest) -------
add("R_raw captured request (T02 at the caps)", "R_raw", RAW if which == "caps" else
    value_tree(18, 37, 131, 1039, 855) + B(128))
# ---------------- ErrorPrefix: blocked_envelope's own owners (lib.rs:13110) ----------------
add("ErrorPrefix: blocked envelope quality/formulation/evidence (text in TEXT.md)", "ErrorPrefix",
    value_tree(64, 24, 160, 24 * 1024, 160 * 40) + B(64 * 1024))
# ---------------- B_cache: material copies (BOUND row 'Material copies') ---------------
mat_tree = (Xc("MaterialInput", 4) + Xc("TemperaturePoint", 4 * pts) + B(4 * (7 + pts * 7) * ID))
add("Material copies (working clone + default-basis clone)", "B_cache", mat_tree * 2)

# ---------------- O-N: normalization (S-4; PP lib.rs:2326-2331, 7514-7600, 8347-8446) ---
# per quantity, sequentially: one format! id, one refs Vec of two Strings, one canonical
# unit String (static symbol, replaces the old unit in place: within the 128-byte cap).
add("O-N normalize_model_units / resolve_shared_sections (no sections)", "Normalization",
    B(textcap(220 + 2 * ID)) + s("String") * 4 + B(2 * ID + 32) + B(2 * ID) + s("String") * 3)

# G4 (RV83 R-3): the per-load provenance parse (self_weight.rs:796-799, inspect_applied_self_weight):
# serde_json::from_str::<Value>(provenance).ok(), one load at a time, the Value living to the end
# of that load's iteration. Under D1.10 a provenance that parses is a non-object Value (array,
# string, number, literal) of <= 128 bytes (D1 text cap): <= 64 values and <= 64 arrays (each
# value or `[]` takes >= 2 bytes with its separator), <= 42 objects nested inside arrays (`{}`
# plus a separator) with <= 25 entries in all (`"":0,` >= 5 bytes), <= 128 string and key bytes.
# Arrays are push-built (capacity <= max(4, 2h)); a failed parse leaves one boxed Error.
add("O-N per-load provenance parse transient (RV83 R-3; one at a time)", "Normalization",
    s("Value") * (4 * 64 + 2 * 64) + X({"Node(String,Value)": 42 + 25 // 5 + 1}) + B(128 + 128 + 128))

# ---------------- BuildScratch / Built resident (BOUND rows :53-54) ----------------------
add("BuiltModel straight resident", "Built",
    V("FrameNode", n) + V("StraightPipeElement", m) + V("FrameElement", m) + A("LinearSupport", g)
    + HashReq("(String,DerivedSection)", m) + B(2 * m * ID + g * ID)
    + B(8 * (4 * g + 2 * r)) + B(8 * sp) + B(32))
add("Built helper maps (material, node, pipe)", "BuildScratch",
    HashReq("(&str,&T)", mats) + HashReq("(&str,usize)", n) + HashReq("(&str,&T)", m))

# ---------------- Boundary (BOUND :55) ----------------------------------------------------
add("Boundary", "Boundary",
    V("usize", r) + V("SpringEntry", sp) + B(sp * ID) + V("SupportFinding", r))

# ---------------- Basis assembly (BOUND :56-57) ------------------------------------------
def K_resident():
    return B(8 * (N + 1) + 8 * appendcap(Z) + 16 * Z)
def pattern_build():
    return (s("Vec<usize>") * n + B(8 * (4 * n + 8 * m)) + B(N) + s("Vec<usize>") * N
            + B(8 * (4 * N + 2 * Z)) + B(8 * (N + 1) + 8 * appendcap(Z) + 8 * Z)
            + Xc("(usize,usize,Matrix12)", m) + Xc("(usize,usize)", m) + B(8 * sp))
add("Sparse K resident (basis stiffness)", "Basis", K_resident())
add("Sparse pattern construction + assembly retention", "PatternAssemblyScratch", pattern_build(),
    B(8 * appendcap(Z)))

# ---------------- CasePrefix (BOUND :58-60; ADDENDUM Bodies) ------------------------------
add("Primitive loads and application", "CasePrefix",
    A("PrimitiveLoad", l) + V("NodalLoadContribution", l) + B(2 * l * ID) + Sort("NodalLoadContribution", l)
    + HashReq("(&str,usize)", n) + HashReq("(&str,&T)", m))
def assembled_force():
    return (B(8 * N) + V("ForceTerm", l) + V("Option<FormationRecord>", l) + s("Vec<usize>") * N
            + B(8 * (4 * N + 2 * l)) + B(8 * pushcap(min(l, N))) + B(l * ID))
add("AssembledForce, nodal family", "CasePrefix", assembled_force(), s("ForceTerm") * (pushcap(l) // 2))
add("Sparse reduction (+ caller prescribed, observation force)", "CasePrefix",
    A("usize", F) + Xc("(usize,f64)", k) + B(8 * F) + Xc("Option<usize>", N) + B(8 * k)
    + Xc("Option<f64>", N) + Xc("(usize,f64)", k) + B(8 * k) + B(8 * N))
b = n
add("Formation-guard Bodies (build + resident)", "CasePrefix",
    B(40 * n + 56 * b) + A("(usize,usize)", m) + HashReq("(usize,usize)", b) + B(8 * n + 8 * b))

# ---------------- Typed core (both modes) --------------------------------------------------
def typed_core(mode):
    out = {}
    if mode == "sparse":
        # AssemblyEvidence pattern copy (BOUND :61) incl. constructor's two further Z arrays and
        # formation_source's second frame/spring clone
        out["AssemblyEvidence (sparse pattern copy)"] = (
            B(8 * (N + 1) + 16 * Z + 16 * Z + 16 * Z) + V("StiffnessContribution", C)
            + Xc("Option<[f64;3]>", n) + V("(usize,usize,bool)", m) + B(8 * pushcap(sp))
            + Xc("FrameElement", m) * 2 + Xc("(usize,f64)", sp) * 2)
        # Sparse typed preparation (BOUND :63)
        out["Sparse typed preparation"] = (
            B(8 * (F + 1) + 3 * 8 * appendcap(Zf) + 8 * Zf + 8 * F + 8 * F + 4 * F)
            + B(8 * N) + A("(usize,usize)", F) + A("(usize,f64,f64)", k) + B(8 * N) + B(8 * N))
        # Sparse typed skyline factor (BOUND :66) + RCM ordering for it (BOUND :65)
        # RCM <= Adj(F,2E)+Adj(F,4E)+16F+F+max(Sort(usize,F-1), 6V(usize,F)+F): the max is
        # replaced by the (larger) sum of both alternatives
        rcm = (Adj(2 * E) + Adj(4 * E) + B(16 * F + F) + B(6 * 8 * pushcap(F) + F)
               + Sort("usize", max(F - 1, 0)))
        out["Sparse skyline factor + ordering"] = (s("Vec<f64>") * F + B(8 * H + 16 * F + 8 * F + F + 8 * F + 8 * F)
                                                    + V("PivotEvidence", F) + B(16 * F + 8 * F) + rcm)
        sums_headers = s("Expansion") * (2 * Z)
        out["Contribution audit (sparse)"] = (sums_headers + B(8 * (Ps + Pd)) + V("ContributionRounding", Z)
                                              + B(8 * (2 * C + Z)) + B(Bf(d + 1))
                                              + Exp_req(2 * min(C + Z, k * (d + 1))))
        fresh_sums = s("Expansion") * Z + B(8 * Ps + Bf(d))
    else:
        # dense AssemblyEvidence + dense typed preparation (BOUND :62)
        out["AssemblyEvidence (dense) + three dense global views"] = (
            Mat(8, N, N) * 2 + Mat(8, N, N) * 2 + V("StiffnessContribution", C) + Xc("Option<[f64;3]>", n)
            + V("(usize,usize,bool)", m) + B(8 * pushcap(sp)) + Xc("FrameElement", m) * 2
            + Xc("(usize,f64)", sp) * 2 + Mat(8, N, N) * 3)
        out["Dense typed preparation + Cholesky"] = (Mat(8, F, F) + B(8 * F + 4 * F) + Mat(8, F, F)
                                                     + V("PivotEvidence", F))
        dense_headers = (s("Vec<Expansion>") * N + s("Expansion") * (N * N)) * 2
        out["Contribution audit (dense)"] = (dense_headers + B(8 * (Ps + Pd)) + V("ContributionRounding", Z)
                                             + B(8 * (2 * C + Z)) + B(Bf(d + 1))
                                             + Exp_req(2 * min(C + Z, k * (d + 1))))
        fresh_sums = s("Vec<Expansion>") * N + s("Expansion") * (N * N) + B(8 * Ps + Bf(d))
    # common to both modes
    out["Condition/refinement basic vectors"] = B(40 * F + 8 * (6 * F + N))
    report = (B(4 * F) + Xc("PivotEvidence", F) + V("ResidualRow", F) * 2 + Xc("ContributionRounding", Z)
              + B(8 * (2 * C + Z)) + tx("sym"))
    fidelity = V("LoadFidelityRow", N) + s("String") * l + B(l * ID) + tx("audit_error")
    out["Completed report (+ original factor/prepared still live)"] = report
    out["Load-fidelity audit and report (returned report cloned once)"] = (
        s("Vec<&ForceTerm>") * N + B(8 * (4 * N + 2 * l)) + B(N) + fresh_sums + A("(f64,f64)", C)
        + Exp_req(1 + 2 * C + 2 * l) + Sort("String", li_max) + fidelity * 2)
    out["Intended-action audit"] = fresh_sums + Exp_req(1 + 2 * C)
    out["Ordinary formation check (outer vectors)"] = (
        Xc("Option<usize>", N) + Xc("ExactAccumulator", F) + Xc("Vec<f64>", F) + B(8 * (4 * F + 288 * m))
        + Xc("Option<i32>", F) + B(8 * F + 16 * F) + B(72 * n + 24) + Xc("(f64,f64)", 2 * n)
        + Xc("Option<[f64;3]>", n) + V("(usize,[f64;3],usize,[f64;3])", m) + tx("formation_detail"))
    out["H_formation128 (T06: heap 0)"] = X()
    kpk = k + sp
    out["Rigid-body geometry (BodyEvidence + witnesses)"] = (
        B(n + 8 * pushcap(n) + 24 * pushcap(n) + 8 * pushcap(kpk))
        + B(24 * pushcap(n) + 48 * pushcap(kpk) + 48 * kpk + 48 * pushcap(7 + 2 * n))
        + s("Expansion") * (6 * pushcap(n)) + B(48 * pushcap(n) + 480 * n + 304 + 8 * N))
    out["Backend solve vectors"] = B(16 * F + 8 * F)
    return out, report, fidelity, fresh_sums

core_mov = {
    "sparse": B(G(d + 1)) + s("ContributionRounding") * (pushcap(Z) // 2) + B(G(1 + 2 * C)),
    "dense": B(G(d + 1)) + s("ContributionRounding") * (pushcap(Z) // 2) + B(G(1 + 2 * C)),
}

# ---------------- Legacy profile observation (BOUND :65, :68) ----------------------------
def legacy_profile():
    profile = B(8 * (3 * F + 1) + 8 * appendcap(H))
    clone = B(8 * (3 * F + 1) + 8 * H)
    rcm = Adj(2 * E) + Adj(4 * E) + B(16 * F + F + 6 * 8 * pushcap(F) + F) + Sort("usize", max(F - 1, 0))
    return (rcm + profile * 2 + clone + B(24 * F) + B(8 * F + F) + V("SymmetricMatrixEntry", E)
            + B(8 * pushcap(F)) + B(N) + Xc("Option<usize>", N))
add("Legacy profile observation", "LegacyProfile", legacy_profile(), B(8 * appendcap(H)))

# ---------------- Dense parity tail (BOUND :69) -- dense mode only -------------------------
dense_parity = (Mat(8, N, N) + Mat(8, F, F) * 2 + B(8 * F) + A("usize", F) + B(8 * k) + B(N)
                + B(8 * F + 8 * max(F - 1, 0) + 8 * F) + legacy_profile())

# ---------------- W2 (W2_RETURN.md) ---------------------------------------------------------
def w2(mode, core_total, report, fidelity):
    scaled_force = (B(8 * N) + A("ForceTerm", l) + B(l * ID) + Xc("Vec<usize>", N) + B(8 * l)
                    + Xc("Option<FormationRecord>", l) + B(8 * pushcap(min(l, N))))
    scaled_prims = A("FrameElement", m) + A("(usize,f64)", sp)
    kbuild = K_resident() + pattern_build()
    evaluate = (kbuild + core_total + scaled_force + scaled_prims * 2 + B(N) + A("usize", F)) * 2
    unscale = V("RecordOutcome", 6 * F) + B(4 * F + 8 * (d + 1))
    publication = (Xc("(usize,PublishedValue)", k) + A("PublishedValue", sp) + A("[PublishedValue;12]", m)
                   + Xc("String", m) + B(m * ID + 8 * sp) + Xc("RecordOutcome", 6 * F))
    qf = textcap(max(ID + 3, 31)) + textcap(max(14 + max(ID + 3, 31), 12 + ID, 8))
    tail = (K_resident() + B(8 * N) + report * 2 + fidelity * 2 + tx("formation_detail") * 2
            + V("RecordOutcome", 6 * F) + publication + scaled_force + B(8 * N + 8 * k)
            + A("PublishedValue", k) + A("usize", F) + B(8 * F) + HashReq("&str", l) + B(qf))
    tail_mov = (A("PublishedValue", k) + A("PublishedValue", sp) + A("[PublishedValue;12]", m) + A("usize", F)
                + A("ForceTerm", l) + B(8 * pushcap(min(l, N))) + B(qf))
    return evaluate + unscale + tail + B(16 * N) + Xc("(usize,f64)", sp), tail_mov

# ---------------- Ordinary recovery (BOUND :75; ADDENDUM stress, guard maps, records) ----
J = 17 + max(8, 2 * 57)
stress_children = s("AnalysisStatus") * 8 + s("StressFinding") * 4 + B(4 * J)
add("Straight member recovery (+ stress supplement)", "OrdinaryRecovery",
    B(96 + 160) + s("StationResultants") * 4 + B(8 * N)
    + HashReq("(String,[f64;6])", g) + B(g * ID) + Xc("SupportVector", g) + B(g * ID)
    + V("MemberRecord", m) + B(m * ID) + V("RecoveryRecord", m) + B(m * ID)
    + tx("err") * (2 * m + 1) + tx("err") * m
    + s("(&str,StressRecoveryResult)") * 3 + stress_children * 6 + s("AnalysisStatus") + B(64 + 16 + 160),
    B(32) + s("AnalysisStatus") * 8 + s("StressFinding") * 2 + B(max(8, 2 * 57)))
q = m + n + g
add("Formation-guard late maps and RecoveryFinding", "OrdinaryRecovery",
    HashReq("(String,usize)", q) + B(q * ID) + HashReq("(usize,(f64,f64))", b) + HashReq("(usize,f64)", b)
    + tx("recovery_finding"))
add("Ordinary scalar maximum (one helper at a time)", "Maximum",
    B(16) + s("QuadraticStressSpan") * 4 + s("Node_SR") * 262_144 + tx("err"),
    s("Node_SR") * (393_216 - 262_144))

# ---------------- Old case staging (BOUND :77-78) -------------------------------------------
add("Source row qualification", "OldCaseStaging",
    HashReq("(String,String)", R0) + B(2 * R0 * ID) + Xc("ResultItem", R0) + tx("row") * R0 + tx("row") * 2)
add("Existing ordinary/failed source case (FinalizedSourceBlockCase::ordinary)", "OldCaseStaging",
    Xc("RowTreatment", R0) + B(R0 * ID) + B(3 * ID + 64) + value_tree(16, 4, 40, 2048, 40 * 24)
    + s("Value") * (2 * R0) + X({"Node(String,Value)": 3 * R0}) + B(128 * R0) + tx("row") * R0
    + X({"Node(&String,())": R0}) + s("ResultBasisRef"))

# ---------------- Preview rewrite, aggregation, final copy (BOUND :79-81) ----------------
add("Preview rewrite", "PreviewRewrite",
    Xc("ResultItem", R0) * 2 + (Xc("ResultItem", 8 * g + m) + tx("row") * (8 * g + m))
    + HashReq("(String,ResultItem)", 8 * g) + X({"Node(String,ResultItem)": max(1, (8 * g + m))})
    + HashReq("String", R0) + B(R0 * ID) + PREVIEW)
add("Final aggregation (results + rows_by_base_id clone + id_map)", "AggregationSuffix",
    (Xc("ResultItem", P) + tx("row") * P) * 2 + X({"Node(String,BTreeMap)": P}) + X({"Node(String,ResultItem)": P})
    + HashReq("(String,String)", P) + B(2 * P * ID))
add("Preview tree final copy (envelope evidence)", "AggregationSuffix", PREVIEW)
# The Vec<Diagnostic> backing only: every diagnostic String (id, code, severity, source, message,
# refs) was allocated by a text site and is already inside T08's TAV, so adding Text(diag_total)
# here would count the same bytes twice.
add("Retained diagnostics (Vec<Diagnostic> backing; strings are in T08 TAV)", "Diagnostics",
    V("Diagnostic", DIAG))

# ---------------- LegacyRecoveryPrefix_or_Exact (T07) and selected finalization (T25) -----
def load_symbolic(path, key):
    d = json.load(open(os.path.join(HERE, path)))
    x = X()
    for term in d[key]["symbolic"].split(" + "):
        if "*" in term:
            c, a = term.split("*", 1); x = x + X({a: int(c)})
        else:
            x = x + B(int(term))
    return x, d
t07, t07d = load_symbolic(f"t07_repair.{which}.out.json", "T07_requested")
t07m, _ = load_symbolic(f"t07_repair.{which}.out.json", "moving_extra")
add("T07 generic deep legacy-exact (RESIDUALS_G3 T07)", "LegacyRecoveryPrefix_or_Exact", t07, t07m)
t25p = os.path.join(HERE, f"t25_caps.{which}.out.json")
if os.path.exists(t25p):
    t25, _ = load_symbolic(f"t25_caps.{which}.out.json", "T25_requested")
    t25m, _ = load_symbolic(f"t25_caps.{which}.out.json", "moving_extra")
    add("T25 selected source-blocks finalization (RESIDUALS_G3 T25)", "OldCaseStaging", t25, t25m)
else:
    add("T25 selected source-blocks finalization (RESIDUALS_G3 T25)", "OldCaseStaging", atom("T25_PENDING"))

out = {"which": which, "counts": dict(n=n, m=m, g=g, s=sp, r=r, l=l, N=N, F=F, k=k, C=C, Z=Z, Zf=Zf, E=E,
                                       H=H, d=d, Ps=Ps, Pd=Pd, P=P, R0=R0)}
modes = {}
for mode in ("sparse", "dense"):
    core, report, fidelity, _ = typed_core(mode)
    core_total = total(core.values())
    w2_req, w2_mov = w2(mode, core_total, report, fidelity)
    rows = dict(fam)
    rows.update({f"Typed core [{mode}]: {k_}": v for k_, v in core.items()})
    rows["W2 (two evaluations + unscale + return tail + direction error)"] = w2_req
    if mode == "dense":
        rows["Dense parity tail (DenseScrutiny, formed, no W2)"] = dense_parity
    movs = dict(mov)
    movs["typed core"] = core_mov[mode]
    movs["W2 tail"] = w2_mov
    modes[mode] = {"rows": rows, "mov": movs}

def dump(x, vals):
    return {"symbolic": x.show(), "assumed_bytes": x.ev(vals)}

text_path = os.path.join(HERE, f"text_closure.{which}.json")
TEXTV = json.load(open(text_path))["atoms"] if os.path.exists(text_path) else {}
vals = dict(ASSUMED); vals.update(TEXTV)
out["text_atoms_source"] = os.path.basename(text_path) if TEXTV else "MISSING (text atoms unevaluated)"
for mode, dct in modes.items():
    req = total(dct["rows"].values())
    try:
        movx = max(dct["mov"].values(), key=lambda x: x.ev(vals))
        mv = movx.ev(vals)
    except KeyError:
        movx, mv = X(), None
    try:
        rq = req.ev(vals)
    except KeyError as e:
        rq = "unevaluated: " + str(e)
    out[mode] = {
        "rows": {k_: (dump(v, vals) if all(a in vals or a == "1" for a in v) else {"symbolic": v.show()})
                 for k_, v in dct["rows"].items()},
        "O_req": {"symbolic": req.show(), "assumed_bytes": rq},
        "O_mov_extra_largest_old_backing": {"symbolic": movx.show(), "assumed_bytes": mv},
    }
if which == "milestone":
    # Cross-check: the transcribed sub-expressions reproduce I54's published milestone constants.
    kk = k + sp
    checks = {
        "K(N,Z) CSR resident = 4712 (I54 calculate.py)": (K_resident().ev({}), 4712),
        "sparse prepared primitive arrays = 4796": (8 * (F + 1) + 3 * 8 * appendcap(Zf) + 8 * Zf + 16 * F + 4 * F, 4796),
        "evidence pattern clone + two retained Z arrays = 4712": (8 * (N + 1) + 32 * Z, 4712),
        "evidence pattern clone + four construction Z arrays = 7016": (8 * (N + 1) + 48 * Z, 7016),
        "rigid witness primitive children = 2992 (ADDENDUM)": (24 * pushcap(n) + 48 * pushcap(kk) + 48 * kk
                                                           + 48 * pushcap(7 + 2 * n) + 48 * pushcap(n) + 480 * n + 304, 2992),
        "audit requested children = 12752 (ADDENDUM)": (8 * (Ps + Pd + 2 * C + Z) + Bf(d + 1), 12752),
    }
    out["milestone_cross_checks"] = {k_: {"value": v_[0], "I54": v_[1], "equal": v_[0] == v_[1]} for k_, v_ in checks.items()}
print(json.dumps(out, indent=1))
