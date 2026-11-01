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
# R-A D3 (§8.13): on R-A's source, register the illustrative s(Option<u32>) its T07 rows use (in-build at G5).
import ra_owners_model as _ra_model
_ra_model.active(os.environ.get("RP_SNAPSHOT"))

which = sys.argv[1] if len(sys.argv) > 1 else "caps"
HERE = os.path.dirname(os.path.abspath(__file__))
# KD5O1: history pricing preserves the registered earlier tree. On an
# implementing snapshot, account for the actual owned correction/closure/group
# structures (Design retained-pricing-rules.md §F). No product format fallback.
_snapshot = os.environ.get("RP_SNAPSHOT")
_fc_path = os.path.join(_snapshot, "core/solver/frame_kernel/src/structural/formation_check.rs") if _snapshot else None
KD5 = bool(_fc_path and os.path.exists(_fc_path) and "pub struct MemberBoundInputs" in open(_fc_path, encoding="utf-8").read())
# KD5O13: inspect the exact private helper bodies before charging their
# new three-frame staging. Historical snapshots keep their original model.
_directed_helper_workspace = X()
if _snapshot:
    from kd5_text_model import inspect_directed_helpers as _inspect_directed_helpers
    if _inspect_directed_helpers(_snapshot):
        _directed_helper_workspace = s("DirectedHelperWorkspace")

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
_wide = False
if _snapshot:
    import b7a_domain_model as _wide_model
    _wide_model.inspect(_snapshot)
    _wide = True
import w1_reductions_model as _w1_reductions_model
_w1_reduced = _w1_reductions_model.inspect(_snapshot) if _snapshot else False
force_q = 12 * l if _wide else l
# T4-U6/U8 D3 (retained-pricing-rules.md, "T4-U6/U8: actual-source D3"): the D-5/D-6 resident owners, each priced
# positively where its source exists (u6_owners_model checks the source text): U6-ES (BuiltModel.exact_sections),
# U6-WS (the case solve's reduced stresses, stress sites and reduced spans), U8-FM (FormationSource.frame_members).
import u6_owners_model as _u6_model
_u6 = _u6_model.inspect(_snapshot) if _snapshot else {"exact_sections": False, "reduced_workspace": False,
                                                      "frame_members": False, "all_rows": False}
_FM = (Xc("String", m) + B(m * textcap(_u6_model.MEMBER_PREFIX))) if _u6["frame_members"] else X()
# T4-S5 D3 (shared epoch 4; retained-pricing-rules.md §8.8): the T4-U4b/U5b owners outside the TEXT inventory, each
# priced positively where its reviewed source exists (u4b_u5b_owners_model binds the function bodies): U5B-PD (the
# published corrected nodal displacements), U4B-ST (one member's station quantities), U5B-AB (the absorbed-diagonal
# refusal's free-DOF flags in each contribution-sum call).
import u4b_u5b_owners_model as _u45_model
_u45 = _u45_model.inspect(_snapshot) if _snapshot else {"published_displacements": False, "station_quantities": False,
                                                        "absorption": False}
_AB = _u45_model.absorption_flags(N) if _u45["absorption"] else X()
# U5B-CG (T4-S6, §8.8): each AssemblyEvidence's contribution-group vector beside its contributions.
_CG = _u45_model.contribution_groups(m, sp) if _u45.get("contribution_groups") else X()
_mc = __import__("g3lib").g4_caps()
CC, LL = (_mc["c"], _mc["L"]) if which == "caps" else (1, l)   # I82: requested cases, total loads
if _wide:
    import b7a_phase_model as _phase_model
    _phase_context = _phase_model.consumer_context(_snapshot, HERE)
    # Authored cases and typed C_eq slots remain distinct.
    _actual_cases = _mc.get("lc", _mc["c"]) if which == "caps" else 1
    _phase_ordinary = _phase_model.ordinary_counts(_phase_context, _actual_cases, CC, l)
    _phase_success = _phase_model.successful_counts(_phase_context, _actual_cases, CC, l)
    # E is admitted before ordinary: nodal q<=l, no formed producer, base only.
    force_q = _phase_ordinary["force_per_case"]
    _formed_q = _phase_ordinary["formed_per_case"]
    _named_cases = _phase_ordinary["named"]
    def _phase_owner(owner):
        return _phase_model.formed_owner(_phase_context, _actual_cases, CC, l, ID,
            owner=owner, branch=_phase_context["branch"])
PA = CC * P                                                   # I82: rows of the c-case envelope
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
# O4-EV (retained-pricing-rules.md §8.9): a Timoshenko member's evidence beam_theory carries two number entries beyond
# the census entry (o4_owners_model.beam_theory_evidence), charged beside the evidence owner.
import o4_owners_model as _o4_model
_o4 = bool(_snapshot) and _o4_model.inspect(_snapshot)
if _o4:
    _o4_ev = _o4_model.beam_theory_evidence(m)
    PREVIEW = PREVIEW + value_tree(0, 0, _o4_ev["ent"], 0, _o4_ev["keyb"])
T = {}   # text atoms (values from TEXT.md)
def tx(name):
    return atom("Text(" + name + ")")

fam = {}      # family -> X (requested)
mov = {}      # family -> X (largest single old backing inside the family)
phase = {}    # family -> ordinary phase name (BOUND:18-25)

def add(name, ph, req, mv=None):
    fam[name] = req; phase[name] = ph; mov[name] = mv or X()

# ---------------- T_resident: T03 typed request at the typed capacity caps --------------
# S1-TS (retained-pricing-rules.md §8.36): the census of the admitted typed strings read at the snapshot (a pipe's 13, not
# the basis' 12; route E's three contract strings and the node-proximity tolerance's unit beyond the basis' six model
# strings); a snapshot outside that reading keeps the basis count.
import typed_strings_model as _ts_model
_ts = _ts_model.inspect(_snapshot) or {"pipe": 12, "model": 0}   # before model document 0.7.0: the basis count
strings = (mats * (7 + pts * 7) + 6 + _ts["model"] + 2 * n + _ts["pipe"] * m + 6 * g + r + 2 * CC + 7 * LL) if which == "caps" else 0
add("T_resident (T03 typed request, typed capacity caps)", "T_resident",
    (Xc("MaterialInput", mats) + Xc("TemperaturePoint", mats * pts) + Xc("PreviewNode", n)
     + Xc("PreviewPipe", m) + Xc("PreviewSupport", g) + Xc("String", r) + Xc("PreviewLoadCase", CC)
     + Xc("PrimitiveLoadInput", LL) + Xc("Authored<Vec<ExpansionLawInput>>", 4)
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
if _wide:
    # The already-counted working/base clones remain. At most CC additional
    # selectors retain independent material vectors until ordinary rendering.
    add("Named basis material clones (B7a stage 1)", "B_cache", mat_tree * _named_cases)
    add("Typed named selector strings (B7a stage 1)", "T_resident", B(CC * ID))

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
_built_resident = (V("FrameNode", n) + V("StraightPipeElement", m) + V("FrameElement", m) + A("LinearSupport", g)
                   + HashReq("(String,DerivedSection)", m) + B(2 * m * ID + g * ID)
                   + B(8 * (4 * g + 2 * r)) + B(8 * sp) + B(32))
if _u6["exact_sections"]:
    # U6-ES: one (pipe id, ExactSectionBases) entry per built pipe, beside the DerivedSection map. The illustrative
    # stride is the chain's own check only; the in-build size_of qualifies (g5_profile).
    ASSUMED.setdefault("s((String,ExactSectionBases))", 400)
    _built_resident = _built_resident + HashReq("(String,ExactSectionBases)", m) + B(m * ID)
add("BuiltModel straight resident", "Built", _built_resident)
if _wide:
    add("Named basis BuiltModel residents (B7a stage 1)", "Built", _built_resident * _named_cases)
    _resolver_req, _resolver_mov = _wide_model.resolver_scratch(m, ID)
    add("Named basis resolver construction (B7a stage 1)", "BuildScratch", _resolver_req * _named_cases, _resolver_mov if _named_cases else X())
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
if _wide:
    add("Named basis stiffness residents (B7a stage 1)", "Basis", K_resident() * _named_cases)
    _basis_req, _basis_mov = _wide_model.cached_basis_headers(_named_cases, ID)
    add("All held basis state tuple/key/record owners (B7a stage 1)", "Basis", _basis_req, _basis_mov)
add("Sparse pattern construction + assembly retention", "PatternAssemblyScratch", pattern_build(),
    B(8 * appendcap(Z)))

# ---------------- CasePrefix (BOUND :58-60; ADDENDUM Bodies) ------------------------------
add("Primitive loads and application", "CasePrefix",
    A("PrimitiveLoad", l) + V("NodalLoadContribution", l) + B(2 * l * ID) + Sort("NodalLoadContribution", l)
    + HashReq("(&str,usize)", n) + HashReq("(&str,&T)", m))
def assembled_force():
    closure = V("LoadClosureRecord", force_q) + s("Vec") if KD5 else X()
    formed = _phase_owner("assembled_children") if _wide else X()
    return (closure + formed + B(8 * N) + V("ForceTerm", force_q) + V("Option<FormationRecord>", force_q) + s("Vec<usize>") * N
            + B(8 * (4 * N + 2 * force_q)) + B(8 * pushcap(min(force_q, N))) + B(force_q * ID))
_formed_mov = (s("LoadPrimitive") * (pushcap(force_q) // 2)
               + s("LoadClosureRecord") * (pushcap(force_q) // 2)
               + B(8 * force_q * 16)) if _wide else X()
add("AssembledForce, nodal family", "CasePrefix", assembled_force(),
    s("ForceTerm") * (pushcap(force_q) // 2) + _formed_mov)
if _wide:
    add("Element uniform LoadApplication owners (B7a stage 1)", "CasePrefix",
        _phase_owner("element_application"),
        s("ElementUniformLoadContribution") * (pushcap(l if _formed_q else 0) // 2))
    _formation_req, _formation_mov = _wide_model.formation_constructor()
    _element_inputs = l if _formed_q else 0
    add("Formed load constructor staging (B7a stage 1)", "CasePrefix",
        _phase_owner("uniform_constructor"),
        _formation_mov if _element_inputs else X())
# B5O1: positive new carriers, including both return/move/argument slots.
# D1.4 refuses every component; D1.5 requires absent/empty region lists with
# capacity zero. Only their joint/region backing children are empty. The
# owned carriers and prepared helper's argument/return staging are retained.
if _snapshot:
    from batch5_model import active as _batch5_active
    if _batch5_active(_snapshot):
        add("Prepared pressure case carriers and staging (B5O1)", "CasePrefix",
            s("ExactPressureCaseOption") * 5 + s("PressureRegionInput") * 2
            + s("LoadFormula") * 2 + s("PreparedCaseWorkspace")
            + s("PreparedCaseResult") * 2 + s("BTreeMapHeader")
            + HashReq("(&str,&T)", mats) + B(24 * N) + s("Vec") * (3 * N + 8))
add("Sparse reduction (+ caller prescribed, observation force)", "CasePrefix",
    A("usize", F) + Xc("(usize,f64)", k) + B(8 * F) + Xc("Option<usize>", N) + B(8 * k)
    + Xc("Option<f64>", N) + Xc("(usize,f64)", k) + B(8 * k) + B(8 * N))
b = n
add("Formation-guard Bodies (build + resident)", "CasePrefix",
    B(40 * n + 56 * b) + A("(usize,usize)", m) + HashReq("(usize,usize)", b) + B(8 * n + 8 * b))
# W-8 topology warnings D3 (retained-pricing-rules.md §8.16): the topology checks' transient working set, once per
# ordinary run after the build (W8-TW), and validation's node-degree map (W8-ND), each priced positively where its
# reviewed source exists (w8_topology_model binds topology.rs and the validation bodies). Not per case.
import w8_topology_model as _w8_model
if _snapshot and _w8_model.inspect(_snapshot):
    add("W-8 topology warnings workspace (W8-TW)", "BuildScratch", _w8_model.topology_workspace(n, m, g))
    add("W-8 validation node-degree map (W8-ND)", "BuildScratch", _w8_model.node_degrees(m))
# W-15 D3 (retained-pricing-rules.md §8.21, W15-PM): the eligibility predicate's and mapping-conflict check's member
# maps (rigid_component pipe_map: two exact-size tables of m entries live at once in applied_scales; conflicts' in
# validation), per ordinary run; every other owner of both is empty on D1 (P2). w15_owners_model binds the statements.
import w15_owners_model as _w15_model
if _snapshot and _w15_model.inspect(_snapshot):
    add("W-15 eligibility and mapping-conflict member maps (W15-PM)", "BuildScratch", _w15_model.producer_maps(m))

# ---------------- Typed core (both modes) --------------------------------------------------
def typed_core(mode):
    out = {}
    if mode == "sparse":
        # AssemblyEvidence pattern copy (BOUND :61) incl. constructor's two further Z arrays and
        # formation_source's second frame/spring clone
        out["AssemblyEvidence (sparse pattern copy)"] = (
            B(8 * (N + 1) + 16 * Z + 16 * Z + 16 * Z) + V("StiffnessContribution", C)
            + Xc("Option<[f64;3]>", n) + V("(usize,usize,bool)", m) + B(8 * pushcap(sp))
            + Xc("FrameElement", m) * 2 + Xc("(usize,f64)", sp) * 2
            + _FM * 3    # U8-FM: PP's identities, the evidence's to_vec and formation_source's clone
            + _CG)       # U5B-CG
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
                                              + Exp_req(2 * min(C + Z, k * (d + 1)))
                                              + _AB)   # U5B-AB
        fresh_sums = s("Expansion") * Z + B(8 * Ps + Bf(d))
    else:
        # dense AssemblyEvidence + dense typed preparation (BOUND :62)
        out["AssemblyEvidence (dense) + three dense global views"] = (
            Mat(8, N, N) * 2 + Mat(8, N, N) * 2 + V("StiffnessContribution", C) + Xc("Option<[f64;3]>", n)
            + V("(usize,usize,bool)", m) + B(8 * pushcap(sp)) + Xc("FrameElement", m) * 2
            + Xc("(usize,f64)", sp) * 2 + Mat(8, N, N) * 3
            + _FM * 3    # U8-FM: PP's identities, the evidence's to_vec and formation_source's clone
            + _CG)       # U5B-CG
        out["Dense typed preparation + Cholesky"] = (Mat(8, F, F) + B(8 * F + 4 * F) + Mat(8, F, F)
                                                     + V("PivotEvidence", F))
        dense_headers = (s("Vec<Expansion>") * N + s("Expansion") * (N * N)) * 2
        out["Contribution audit (dense)"] = (dense_headers + B(8 * (Ps + Pd)) + V("ContributionRounding", Z)
                                             + B(8 * (2 * C + Z)) + B(Bf(d + 1))
                                             + Exp_req(2 * min(C + Z, k * (d + 1)))
                                             + _AB)   # U5B-AB
        fresh_sums = s("Vec<Expansion>") * N + s("Expansion") * (N * N) + B(8 * Ps + Bf(d))
    # common to both modes
    out["Condition/refinement basic vectors"] = B(40 * F + 8 * (6 * F + N))
    if _directed_helper_workspace:
        out["Directed product/quotient helper staging (KD5O13)"] = _directed_helper_workspace
    report = (B(4 * F) + Xc("PivotEvidence", F) + V("ResidualRow", F) * 2 + Xc("ContributionRounding", Z)
              + B(8 * (2 * C + Z)) + tx("sym"))
    fidelity = V("LoadFidelityRow", N) + s("String") * force_q + B(force_q * ID) + tx("audit_error")
    out["Completed report (+ original factor/prepared still live)"] = report
    out["Load-fidelity audit and report (returned report cloned once)"] = (
        s("Vec<&ForceTerm>") * N + B(8 * (4 * N + 2 * force_q)) + B(N) + fresh_sums + A("(f64,f64)", C)
        + Exp_req(1 + 2 * C + 2 * force_q) + Sort("String", force_q if _wide else li_max) + fidelity * 2
        + _AB)   # U5B-AB
    out["Intended-action audit"] = fresh_sums + Exp_req(1 + 2 * C) + _AB   # U5B-AB
    out["Ordinary formation check (outer vectors)"] = (
        Xc("Option<usize>", N) + Xc("ExactAccumulator", F) + Xc("Vec<f64>", F) + B(8 * (4 * F + 288 * m))
        + Xc("Option<i32>", F) + B(8 * F + 16 * F) + B(72 * n + 24) + Xc("(f64,f64)", 2 * n)
        + Xc("Option<[f64;3]>", n) + V("(usize,[f64;3],usize,[f64;3])", m) + tx("formation_detail"))
    if KD5:
        # KD5O2: rho is reused as s/s2; epsilon_e later owns one F row.
        # Three rows are conservative over both peaks. Five F f64 arrays
        # cover psi, correction RHS/solve scratch and epsilon row/column sums.
        # The old per-row list allowance is retained conservatively.
        out["Ordinary formation check (outer vectors)"] += Xc("ExactAccumulator", F) * 2 + B(8 * 5 * F)
        # B7a stage 1 admits full-span uniform straight member inputs.
        # Typed formation owns one cloned primitive source and constructs
        # comparison entries independently; q=12*l includes zero terms.
        out["D1 formed primitive and independent manifest children (KD5O3)"] = (
            _phase_owner("typed_source_children")
            if _wide else s("Vec") + V("LoadPrimitive", 0) + V("LoadClosureEntry", 0))
        # Both correction vectors and global exponents remain in the solution
        # through recovery. Inline owner includes all three Vec headers.
        out["StructuralSolution member-bound payload (KD5O4)"] = B(20 * N) + s("MemberBoundInputs")
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
    scaled_children = _phase_owner("scaled_force_children") if _wide else X()
    scaled_force = ((Xc("LoadClosureRecord", force_q) + s("Vec") if KD5 else X()) + scaled_children
                    + B(8 * N) + A("ForceTerm", force_q) + B(force_q * ID) + Xc("Vec<usize>", N) + B(8 * force_q)
                    + Xc("Option<FormationRecord>", force_q) + B(8 * pushcap(min(force_q, N))))
    scaled_prims = A("FrameElement", m) + A("(usize,f64)", sp) + _FM   # U8-FM: each scaled copy's identities
    kbuild = K_resident() + pattern_build()
    evaluate = (kbuild + core_total + scaled_force + scaled_prims * 2 + B(N) + A("usize", F)) * 2
    # W1-L1c: b=0 borrows f; the sole scaled force drops inside the second
    # evaluation, before the caller tail. Keep the existing header allowance.
    if _w1_reduced and _wide:
        evaluate = evaluate + X({key: -value for key, value in scaled_children.items()})
    tail_force = (scaled_force + X({key: -value for key, value in scaled_children.items()})
                  if _w1_reduced and _wide else scaled_force)
    unscale = V("RecordOutcome", 6 * F) + B(4 * F + 8 * (d + 1))
    publication = (Xc("(usize,PublishedValue)", k) + A("PublishedValue", sp) + A("[PublishedValue;12]", m)
                   + Xc("String", m) + B(m * ID + 8 * sp) + Xc("RecordOutcome", 6 * F))
    qf = textcap(max(ID + 3, 31)) + textcap(max(14 + max(ID + 3, 31), 12 + ID, 8))
    tail = (K_resident() + B(8 * N) + report * 2 + fidelity * 2 + tx("formation_detail") * 2
            + V("RecordOutcome", 6 * F) + publication + tail_force + B(8 * N + 8 * k)
            + A("PublishedValue", k) + A("usize", F) + B(8 * F) + HashReq("&str", l) + B(qf))
    if KD5:
        # KD5O5: outcome.solution and its PreviewLinearSolve clone coexist.
        tail += (B(20 * N) + s("MemberBoundInputs")) * 2
    tail_mov = (A("PublishedValue", k) + A("PublishedValue", sp) + A("[PublishedValue;12]", m) + A("usize", F)
                + A("ForceTerm", force_q) + B(8 * pushcap(min(force_q, N))) + B(qf) + _formed_mov)
    return evaluate + unscale + tail + B(16 * N) + Xc("(usize,f64)", sp), tail_mov

# ---------------- Ordinary recovery (BOUND :75; ADDENDUM stress, guard maps, records) ----
# KD5O6: seven locations/four member groups. The repaired support
# constructor retains force/moment vectors and separately published
# magnitudes (four evaluands, two physical families). Historical KD5
# snapshots before that constructor retain two groups. Each record
# push-builds its own groups; the per-record PushCap retains all backing.
# Labels are static <=128 B each;
# bound Err children use the existing 8192-B error-content class.
_recovery_count = m + g if KD5 else m
_cp_path = os.path.join(_snapshot, "core/product_physics/src/certified_publication.rs") if KD5 else None
_support_groups = 4 if _cp_path and '"support_force_magnitude"' in open(_cp_path, encoding="utf-8").read() else 2
_recovery_evaluands = 28 * m + _support_groups * g
_recovery_groups = (s("RecoveryGroup") * (pushcap(28) * m + pushcap(_support_groups) * g)
                    + B(_recovery_evaluands * (2 * ID + textcap(8192)))
                    + V("String", _recovery_evaluands)
                    + V("(String,[f64;2])", g) + B(g * ID) + s("Vec")) if KD5 else X()
if KD5:
    # KD5O8: price the recovery's separately constructed FormationSource
    # explicitly, preserving the prior assembly/check copy allowances.
    _recovery_groups += Xc("FrameElement", m) + Xc("(usize,f64)", sp) + s("Vec") * 6
    _recovery_groups += _FM   # U8-FM: certified_publication's source() fills frame_members from PP's identities
    if _wide:
        _recovery_groups += _phase_owner("recovery_source_children")
# KD5O11: PP caller owners and return/move slots, separate from the
# kernel-only coefficient workspace. Retain the sum of disjoint slot
# alternatives; no compiler reuse or move elision is presumed.
_pp_certified_callers = (
    s("CertifiedMemberOption") * 2
    + s("CertifiedMemberRecoveryResult") * 2
    + s("CertifiedMemberRecovery") * 4
    + s("CertifiedMemberRecoveryPairResult") * 2
    + s("CertifiedSupportRows") * 2
    + s("CertifiedSupportResult") + s("CertifiedSupportOption")
    + s("CertifiedRow") * 16 + s("CertifiedGroup") * 2
) if KD5 else X()
J = 17 + max(8, 2 * 57)
stress_children = s("AnalysisStatus") * 8 + s("StressFinding") * 4 + B(4 * J)
# U6-WS: per member (one member's at a time), the reduced section's end and station stresses beside the nominal
# ones (2 ends + <= 3 stations, KD5L7; the bare StressRecoveryResult priced at the (&str, ..) tuple), their
# children, the stations Vec, the stress_sites Vec (vec! of 2 plus <= 3 pushes) and the stress-finding bases (<= 2).
_reduced_stresses = (s("(&str,StressRecoveryResult)") * 5 + stress_children * 5 + s("Vec")
                     + B(pushcap(5) * _u6_model.STRESS_SITE + 2 * _u6_model.FINDING_BASIS)) if _u6["reduced_workspace"] else X()
add("Straight member recovery (+ stress supplement)", "OrdinaryRecovery",
    B(96 + 160) + s("StationResultants") * 4 + B(8 * N)
    + HashReq("(String,[f64;6])", g) + B(g * ID) + Xc("SupportVector", g) + B(g * ID)
    + V("MemberRecord", m) + B(m * ID) + V("RecoveryRecord", _recovery_count) + B(_recovery_count * ID) + _recovery_groups + _pp_certified_callers
    + tx("err") * (2 * m + 1) + tx("err") * m
    + s("(&str,StressRecoveryResult)") * 3 + stress_children * 6 + s("AnalysisStatus") + B(64 + 16 + 160)
    + _reduced_stresses,
    B(32) + s("AnalysisStatus") * 8 + s("StressFinding") * 2 + B(max(8, 2 * 57)))
if _u45["published_displacements"]:
    # U5B-PD: the case's published nodal displacements, live from the solve's publication through its rows.
    add("Published corrected nodal displacements (U5B-PD)", "OrdinaryRecovery", _u45_model.ordinary_published_displacements(N))
if _u45["station_quantities"]:
    # U4B-ST: one member's station-quantity transients (append_station_quantities and its caller), one member live.
    add("Station quantities of one member (U4B-ST)", "OrdinaryRecovery", _u45_model.ordinary_station_quantities())
if _u45.get("published_index"):
    # RT-A3 increment 1b (retained-pricing-rules.md §8.37): the case solve's published-row index. RTA3B-PS: the case's
    # PublishedScope entity set (<= the case's rows, P; live to the end of the case solve, as U5B-PD). RTA3B-MR: one
    # member's MemberRows slots (<= its rows per member, the route's P_final coefficient of m; in U4B-ST's block).
    _rpm, _rem = divmod(P - (7 * n + 8 * g + 3), m)
    if _rem:
        raise SystemExit(f"RTA3B-MR: P_final is not 7n + r*m + 8g + 3 ({P}); re-review the rows per member")
    add("Published-row scope of one case (RTA3B-PS)", "OrdinaryRecovery", _u45_model.published_scope_entities(P))
    add("Member row index of one member (RTA3B-MR)", "OrdinaryRecovery", _u45_model.member_row_slots(_rpm))
if _o4:
    # O4-SR: a selected case whose member's stress sections differ holds the reduced basis's composite maximum (its
    # value and evidence Value: one pipe_stress_extrema entry of the census) beside the nominal one, one member at a time.
    _mx = _o4_model.maximum_evidence_facts()
    add("Selected reduced maximum beside the nominal one (O4-SR)", "OrdinaryRecovery",
        s("CompositeMaximum") + value_tree(_mx["arr"], _mx["obj"], _mx["ent"], _mx["strb"], _mx["keyb"]))
q = m + n + g
add("Formation-guard late maps and RecoveryFinding", "OrdinaryRecovery",
    HashReq("(String,usize)", q) + B(q * ID) + HashReq("(usize,(f64,f64))", b) + HashReq("(usize,(f64,f64))" if KD5 else "(usize,f64)", b)
    + tx("recovery_finding") + _directed_helper_workspace)
_coefficient_spans = X()
if KD5 and "certify_straight_stress_spans(" in open(_cp_path, encoding="utf-8").read():
    # KD5O10/B7a: every admitted uniform input emits one row-0 physical
    # source occurrence and twelve terms, even for repeated targets/IDs.
    # Thus source_count<=l is warranted without a uniqueness discount.
    # Full-span inputs deduplicate to boundaries 0/1. Retain the larger
    # conservative 2l+1 span allowance. FK
    # with_capacity is exact; PP Result collect is conservatively push-grown.
    # The FK input and PP output coexist while mapping; no in-place reuse.
    # Summing the earlier boundary backing is conservative over both peaks.
    _span_count = 2 * l + 1
    _coefficient_spans = (Xc("CertifiedStraightStressSpan", _span_count)
                          + V("EnclosedQuadraticStressSpan", _span_count)
                          + B(8 * (2 * l + 2)) + s("Vec") * 3
                          + s("CertifiedStressControlWorkspace"))
    if _u6["reduced_workspace"]:
        # U6-WS: the exact maximum's reduced basis forms its own spans (straight_stress_spans on the reduced
        # section) while the member's nominal spans stay held: one more PP output beside this workspace.
        _coefficient_spans = _coefficient_spans + V("EnclosedQuadraticStressSpan", _span_count) + s("Vec")
_fallback_workspace = X()
if KD5 and "fn fallback_circular_maximum(" in open(_cp_path, encoding="utf-8").read():
    import re as _pricing_re
    _fallback_source = open(_cp_path, encoding="utf-8").read()
    if not _pricing_re.search(r"struct FallbackError\s*\{\s*magnitude: f64,\s*error: f64,\s*\}", _fallback_source):
        raise SystemExit("KD5O12: fallback arithmetic value layout changed; re-review replay workspace")
    _exact_sum_source = open(os.path.join(_snapshot, "core/solver/frame_kernel/src/exact_sum.rs"), encoding="utf-8").read()
    if not _pricing_re.search(r"const LIMBS: usize\s*=\s*68;", _exact_sum_source):
        raise SystemExit("KD5O12: exact-product magnitude grid changed; re-review replay scratch buffers")
    _fallback_workspace = s("FallbackReplayWorkspace")
add("Ordinary scalar maximum (one helper at a time)", "Maximum",
    B(16) + s("QuadraticStressSpan") * 4 + _coefficient_spans + _fallback_workspace + s("Node_SR") * 262_144 + tx("err"),
    s("Node_SR") * (393_216 - 262_144))

# ---------------- Old case staging (BOUND :77-78) -------------------------------------------
add("Source row qualification", "OldCaseStaging",
    HashReq("(String,String)", R0) + B(2 * R0 * ID) + Xc("ResultItem", R0) + tx("row") * R0 + tx("row") * 2)
# W-7 D3 (retained-pricing-rules.md §8.20, W7-QF): qualify_source_case_rows' references to the case's rows other than
# its global end rows, push-grown, live through the qualification.
import w7_owners_model as _w7_model
if _snapshot and _w7_model.inspect(_snapshot):
    add("Source row qualification: global end filter (W7-QF)", "OldCaseStaging", _w7_model.source_filter(R0))
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
    (Xc("ResultItem", PA) + tx("row") * PA) * 2 + X({"Node(String,BTreeMap)": PA}) + X({"Node(String,ResultItem)": PA})
    + HashReq("(String,String)", PA) + B(2 * PA * ID))
if _wide:
    _map_req, _map_mov = _wide_model.record_map(_named_cases, ID)
    add("Independent combination modulus record map (B7a stage 1)", "AggregationSuffix", _map_req, _map_mov)
    add("Additional held failed-manifest envelope owners (B7a stage 1)", "OldCaseStaging", _wide_model.failed_envelope_copy(CC, ID, _snapshot))
add("Preview tree final copy (envelope evidence)", "AggregationSuffix", PREVIEW)
# Silent-drop guard D3 (retained-pricing-rules.md §8.19): the combination callee's withheld names and support-group rows
# (SD-WH, route L with combinations, beside the W1-L5 index), validation's per-combination term set (SD-HS) and the
# closed-record scan's probe error (SD-PB), each priced positively where its reviewed source exists (sd_owners_model).
import sd_owners_model as _sd_model
if _snapshot and _sd_model.inspect(_snapshot):
    if _mc.get("z", 0):
        add("Combination withheld names and group rows (SD-WH)", "AggregationSuffix", _sd_model.withheld_names(PA, g))
    add("Validation duplicate-term set (SD-HS)", "BuildScratch", _sd_model.seen_terms())
    add("Closed-record scan probe error (SD-PB)", "BuildScratch", _sd_model.probe_error())
# Model document 0.9.0 D3 (retained-pricing-rules.md §8.36): the lowering's views, findings and operand resolution
# (S1-VW) and the withholding pass (S1-WH), once per ordinary run, priced positively where the reviewed modules exist
# (s1_owners_model binds their bytes and the two calls).
import s1_owners_model as _s1_model
if _snapshot and _s1_model.inspect(_snapshot):
    _s1_z = _mc.get("z", 0) if which == "caps" else 0
    add("Model document 0.9.0 lowering views and findings (S1-VW)", "Normalization", _s1_model.views_and_findings(m, CC, g, r, _s1_z))
    add("Model document 0.9.0 interim withholding (S1-WH)", "Normalization", _s1_model.withholding(CC, _s1_z))
# W-16 slice a D3 (retained-pricing-rules.md §8.23): case_prescribed's node index, once per case solved without a load
# state, one case at a time (W16-NI), and validate's findings vector, <= one per primitive load (W16-VF), each priced
# positively where its reviewed source exists (w16_owners_model binds imposed_motion.rs and the statements).
import w16_owners_model as _w16_model
if _snapshot and _w16_model.inspect(_snapshot):
    add("Imposed-motion case node index (W16-NI)", "CasePrefix", _w16_model.node_index(n))
    add("Imposed-motion validation findings backing (W16-VF)", "BuildScratch", _w16_model.validation_findings(LL))
    # W-16 stage 2 D3 (§8.29): case_spring_ground's base vectors over the case's s springs (W16S-SG).
    add("Imposed-motion spring base vectors (W16S-SG)", "CasePrefix", _w16_model.spring_ground(sp))
# Closed formed guard construction pays the native row headers, both source
# lists, maps, fired backing/moving storage and six independently cloned items.
if _wide:
    import b7a_formation_model as _formed_model
    _formed_model.inspect(_snapshot)
    add("Load-row formation guard construction and independent source lists (B7a)",
        "OrdinaryRecovery", _formed_model.finding_construction(_phase_ordinary["finding_cases"], n, l, ID))
# The Vec<Diagnostic> backing only: every diagnostic String (id, code, severity, source, message,
# refs) was allocated by a text site and is already inside T08's TAV, so adding Text(diag_total)
# here would count the same bytes twice.
add("Retained diagnostics (Vec<Diagnostic> backing; strings are in T08 TAV)", "Diagnostics",
    V("Diagnostic", DIAG))
# KD5O14: each case's LoadCaseSolve.withheld_summaries is its own push-grown
# Vec<Diagnostic> (at most one per member), live beside the envelope's vector
# until the solves drop (an append leaves its capacity). Strings and the
# count inside D are already priced; only the separate backings are added.
_lib_path = os.path.join(_snapshot, "core/product_physics/src/lib.rs") if KD5 else None
if KD5 and "    withheld_summaries: Vec<Diagnostic>,\n" in open(_lib_path, encoding="utf-8").read():
    add("Withheld certified-summary diagnostics backing per case (KD5O14)", "Diagnostics",
        V("Diagnostic", m) * CC)

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

PER_CASE_FAMILIES = ['Primitive loads and application', 'AssembledForce, nodal family', 'Sparse reduction (+ caller prescribed, observation force)', 'Formation-guard Bodies (build + resident)', 'Legacy profile observation', 'Straight member recovery (+ stress supplement)', 'Formation-guard late maps and RecoveryFinding', 'Ordinary scalar maximum (one helper at a time)', 'Source row qualification', 'Existing ordinary/failed source case (FinalizedSourceBlockCase::ordinary)', 'Preview rewrite', 'Preview tree final copy (envelope evidence)', 'T07 generic deep legacy-exact (RESIDUALS_G3 T07)']
if "Prepared pressure case carriers and staging (B5O1)" in fam:
    PER_CASE_FAMILIES.append("Prepared pressure case carriers and staging (B5O1)")
for _f in ("Published corrected nodal displacements (U5B-PD)", "Station quantities of one member (U4B-ST)",
           "Selected reduced maximum beside the nominal one (O4-SR)", "Published-row scope of one case (RTA3B-PS)",
           "Member row index of one member (RTA3B-MR)"):
    if _f in fam:
        PER_CASE_FAMILIES.append(_f)
if _wide:
    PER_CASE_FAMILIES.append("Formed load constructor staging (B7a stage 1)")
    PER_CASE_FAMILIES.append("Element uniform LoadApplication owners (B7a stage 1)")
for _f in PER_CASE_FAMILIES:                       # I82: one owner set per requested case, all counted live
    assert _f in fam, _f
    fam[_f] = fam[_f] * CC
out = {"which": which, "counts": dict(n=n, m=m, g=g, s=sp, r=r, l=l, N=N, F=F, k=k, C=C, Z=Z, Zf=Zf, E=E,
                                       H=H, d=d, Ps=Ps, Pd=Pd, P=P, R0=R0)}
modes = {}
for mode in ("sparse", "dense"):
    core, report, fidelity, _ = typed_core(mode)
    core_total = total(core.values())
    w2_req, w2_mov = w2(mode, core_total, report, fidelity)
    rows = dict(fam)
    rows.update({f"Typed core [{mode}]: {k_}": v * CC for k_, v in core.items()})
    rows["W2 (two evaluations + unscale + return tail + direction error)"] = w2_req * CC
    if mode == "dense":
        rows["Dense parity tail (DenseScrutiny, formed, no W2)"] = dense_parity * CC
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
