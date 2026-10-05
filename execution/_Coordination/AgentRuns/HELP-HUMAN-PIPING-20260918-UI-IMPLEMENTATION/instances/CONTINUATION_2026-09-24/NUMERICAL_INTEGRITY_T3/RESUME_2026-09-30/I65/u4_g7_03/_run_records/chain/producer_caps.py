"""I65 U4 G3: T11-T15 (W1 producer) at the D1 caps, from the accepted rosters.

Sources of each row (all accepted at their scope):
  T11 observation: ProductCapture fields (retained_product.rs at U1(a) 59a5de2032) and
      I51 public_producer_admission_05 COMPOSITION section 2 (W_observation, late capture);
      U1(a) F4 OrdinarySeed (typed G-b/G-l capture), counted from source.
  T12 preparation: I29 f2a_preparation_counts_p1 COUNTS_AND_OWNERSHIP section 5 roster
      (RV48), with I51 section 3 rows 1-2.
  T13 native run: I29 f2a_kernel_ownership_p3 ENVELOPE sections 3-4, the conservative
      alternative K_req <= P1 + headers + C2 delta + RetainSuperset + max ScratchSuperset.
  T14 lanes, projection/maximum, final certificate: I51 COMPOSITION section 3 rows 3-6;
      the maximum helper from I54 BOUND:76; final_case.rs prepared reservations.
  T15 C3 typed trace: I51 COMPOSITION section 4 counts.
Container laws as g3lib. V_T(x) of P1/P3 (an unbound capacity family) is evaluated with the
accepted push law s(T)*PushCap(x) (I54 COEFFICIENTS V3), C_T(x;q) <= s(T)*(4q + 2x) (sum of
max(4, 2x_i)), exact clones with X. FK-private strides are layout atoms; Wide<L> is
{negative: bool, exponent: i64, significand: [u64; L]} (FK/wide.rs:204-208), whose upper by
per-field rounding to alignment 8 is 16 + 8L (G5 witnesses it). ASSUMED values are illustrative.
Usage: python3 producer_caps.py <caps|milestone>
"""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from g3lib import X, B, s, atom, V, A, Xc, Mat, HashReq, pushcap, total, ASSUMED, textcap

which = sys.argv[1] if len(sys.argv) > 1 else "caps"
H_ = os.path.dirname(os.path.abspath(__file__))
TXT = json.load(open(os.path.join(H_, f"text_closure.{which}.json")))["atoms"]
if which == "caps":
    from g3lib import g4_caps as _gc
    _c = _gc(); n, m, sp, r, l, g = _c["n"], _c["m"], _c["s"], _c["r"], _c["l"], _c["g"]
else:
    n, m, sp, r, l, g = 2, 1, 3, 6, 3, 4
d = 0                                    # product directional springs (C2)
N = 6 * n; F = N; k = min(N, r); Bk = F; b = n
U = 78 * m + sp; P = 144 * m + sp; Zu = min(N * N, P); Hp = F * (F + 1) // 2
t = 3 * m; v = min(N, l)
Q = 7 * n + 30 * m + sp + k + 2 * g
P_final = 7 * n + 51 * m + 8 * g + 3
Cs = sp; Ca = r
ID, RID = 128, 1024
I_src = l * ID
E_src = 38 + 24 * n + 84 * m + 17 * sp + 13 * k + 17 * l + I_src + 16 * t + 22 * g + 4 * Cs
E_stf = 26 + 24 * n + 84 * m + 17 * sp + 5 * k
LAMBDA = 68
E_led = 10 + 18 * v + 8 * LAMBDA * v
Tr = 512                                  # tracker_rows default (P3 section 4)
O = 3 * Q                                 # offers <= Q + 2 Q_force, Q_force <= Q
Kt = 8 * b                                # tracker keys <= 8b

def Cv(t_, x, q):                         # q child vectors with x elements in total
    return s(t_) * (4 * q + 2 * x) if x > 0 else X()
def W(L):                                 # one Wide<L> by name
    return f"Wide<{L}>"
def Sort(t_, h):
    return X() if h <= 20 else s(t_) * max(h, 48)

rows = {}
# ------------------------------------------------------------------ T11 observation (G-A..G-B)
seed = (s("OrdinarySeed") * 4 + B(ID) + B(4 * 8 * N) + B(4 * RID))
rows["T11.1 ProductCapture early observation (nodes, materials, selections, basis record)"] = (
    V("(String,[f64;3])", n) + B(n * ID) + V("(String,f64,f64)", 8) + B(8 * ID)
    + V("MaterialSelection", 8) + B(8 * 2 * ID) + s("BasisRecord") + B(2 * RID))
rows["T11.2 solver observations (case, mode basis, parity basis copies)"] = s("SolverObservations") + B(ID + 2 * RID)
rows["T11.3 support/member/term identity maps (capture_supports, members, terms)"] = (
    V("MemberIdentity", m) + B(m * 4 * ID) + V("TermIdentity", l) + B(l * 2 * ID) + V("(String,usize)", g) + B(g * ID)
    + V("SpringIdentity", sp) + B(sp * 2 * ID) + V("[bool;6]", g) + V("ProductMemberFacts", m))
rows["T11.4 U1(a) F4 OrdinarySeed (one case: case id, initial, W2 trigger/failure, refs)"] = seed
rows["T11.5 late old-source capture P1_old_source (Option<PrimitiveSource> = SourceParts + constructor survivors)"] = (
    V("SrcCoord", n) + V("SrcMember", m) + V("SrcSpring", sp) + V("SrcConstraint", r) + V("SrcNodal", l) + B(l * ID)
    + V("SrcStation", t) + V("SrcSupport", g) + Cv("u32", Cs, g) + Xc("Option<f64>", N) + B(4 * pushcap(n)))
rows["T11.6 operational records, case id, error, census arrays"] = V("OperationalSpent", 2 * m) + B(ID) + s("CaptureError")
# U1 grant 2 delta (merged into NUM at 4a13e369b9, after this grant's source reads): ProductCapture
# gains `invocation_digest: Option<String>`, a clone of the 64-byte hex invocation digest
# (retained_product.rs, RV82-S2); ProductProofWork's new `anchor` is an Arc::clone (no new heap).
rows["T11.7 U1 grant 2 delta: invocation_digest clone"] = B(64)
# ------------------------------------------------------------------ T12 preparation (P1 section 5)
maps = (V("NodeMapE", n) + V("MemberMapE", m) + V("SpringMapE", sp) + V("SupportMapE", g) + V("ConstraintMapE", r)
        + Cv("usize", Ca, r) + V("NodalMapE", l) + V("StationMapE", t) + V("SectionMapE", m)
        + B((n + m + sp + g + r + l + t + m) * ID))
source_parts = (V("SrcCoord", n) + V("SrcMember", m) + V("SrcSpring", sp) + V("SrcConstraint", r) + V("SrcNodal", l)
                + B(I_src) + V("SrcStation", t) + V("SrcSupport", g) + Cv("u32", Cs, g))
constructor = (Sort("SrcMember", m) + Sort("SrcSpring", sp) + Sort("SrcConstraint", r) + Sort("SrcNodal", l)
               + Sort("SrcStation", t) + Sort("SrcSupport", g) + s("ExactWideSum") * 4 + Xc("Option<f64>", N)
               + B(8 * n + 2 * 4 * n))
rows["T12.1 InputMap/material/source registries (owned id copies)"] = maps + V("MaterialDescriptor", 8)
rows["T12.2 SourceParts + PrimitiveSource constructor scratch"] = source_parts + constructor
rows["T12.3 source clone (original + one complete clone)"] = source_parts + Xc("Option<f64>", N) + B(4 * n * 2)
rows["T12.4 exact ledger (accumulators, nets, limbs)"] = (V("(usize,ExactAccumulator,bool)", v) + V("(usize,LedgerNet)", v)
                                                          + Cv("u64", LAMBDA * v, v) + B(v))
rows["T12.5 prescriptions, identity encodings (E_src, E_stf, E_led)"] = (V("(usize,VecPair)", k) + Cv("Pair", k, k) + B(8)
                                                                          + B(E_src) + B(E_stf) + B(E_led))
rows["T12.6 layout/extents, body membership"] = (V("QuantityMeta", Q) + B(8 * b) + B(4 * n) + V("SrcCoord", n)
                                                  + V("BodyMapE", b) + Cv("u32", n, b) + Cv("u32", m, b))
rows["T12.7 geometry (assessment + exact witness)"] = (V("BodyGeometry", b) + V("SrcCoord", n) + s("Row6") * (2 * pushcap(k + sp))
                                                        + V("Row6", 2 * n + 7) + V("Expansion6", n) + B(n * 33 * 8 * 2)
                                                        + V("Row6", n) + B(4096))
rows["T12.8 pattern/tagging"] = (V("(usize,usize)", U) + s("Vec<usize>") * N + Cv("usize", P, N) + B(8 * pushcap(N + 1))
                                  + B(2 * 8 * pushcap(Zu)) + V("Tag", U) + B(2 * 8 * pushcap(Zu + 1)) + V("ContributionE", U))
rows["T12.9 ordering/RCM + free blocks"] = (B(4 * 8 * pushcap(F) + 8 * pushcap(N)) + s("Vec<usize>") * F + Cv("usize", Zu, F)
                                            + s("Vec<usize>") * F + Cv("usize", 2 * Zu, F) + B(8 * pushcap(F) + F)
                                            + Sort("usize", max(F - 1, 0)) + B(5 * 8 * pushcap(F) + 2 * F + 8 * pushcap(F))
                                            + B(4 * pushcap(F)) + s("Vec<usize>") * Bk + Cv("usize", F, Bk) + B(4 * pushcap(Bk))
                                            + B(2 * 8 * F))
rows["T12.10 call/group/source registries, Arc preps"] = (s("ArcCasePrep") + s("ArcGroupPrep") + V("RegistryE", 8)
                                                           + B(4 * RID))
# ------------------------------------------------------------------ T13 native (P3 sections 3-4)
precisions = [(128, 4, 4, None), (256, 4, 8, 8), (512, 8, 16, 16), (1024, 16, 16, 16)]
def Factor(L):
    return (B(2 * 8 * pushcap(F)) + s("Vec<Wide>") * F + s(W(L)) * Hp + B(8 * pushcap(F)) + V(f"PivotScreen<{L}>", F))
retain = X()
for p, L, Rw, Ww in precisions:
    S_p = (s(f"Shared<{L}>") + V(f"MemberOperators<{L}>", m) + V(W(L), Zu) + V(W(Rw), Zu)
           + V(f"BoundedCoefficients<{Rw}>", m) + Factor(L) + (V(W(L), Bk) if p != 128 else X()))
    Z_p = s(f"Solved<{L}>") + V(W(L), N) + s(W(L)) * (6 * pushcap(m)) + V(W(L), Q)
    retain = retain + S_p + Z_p
    if Ww:
        VS_p = (s(f"VerifyShared<{L}>") + V(W(L), Zu) + s(W(Ww)) * (144 * pushcap(m)) + V(f"BlockBound<{L}>", Bk))
        Report = (s(f"VerificationReport<{L}>") + B(16 * pushcap(b)) + s(f"Option<{W(L)}>") * (5 * pushcap(Q))
                  + V(W(L), F) * 2 + V(f"BlockCertificate<{L}>", Bk) + V(f"BlockNorms<{L}>", Bk)
                  + s(f"Option<{W(L)}>") * pushcap(Bk) + V(f"BodyReport<{L}>", b))
        retain = retain + VS_p + Report + V("BlockRefusal", Bk)       # one cached failed-verification child
summary = B(16 * pushcap(b) + 8 * pushcap(b) + 16 * pushcap(b))
attempts = V("AttemptRecord", 4) + summary * 4 + V("BlockRefusal", 2 * Bk) * 4
geo = V("BodyGeometry", b) + s("(u32,SpringKind)") * (2 * n)
publication = V("PublishedRow", Q) + V("(u32,Kind,u64)", 4 * b) + B(8 * Q)
evidence = (B(4 * b * 16 * 2) + B(8 * k) + V("(QuantityId,u64)", Q) + V("(QuantityId,Binary64Outcome)", Q)
            + B(E_src) + B(E_led) + B(22 + (N + 6 * m) * (9 + 8 * 16)) + B(b * 48 + 2 * b * 48 + b * 32 + b * 32 + b * 48))
selected = s("RetainedSolve") + V("PrecisionState", 4) + attempts + geo + publication + evidence + summary + s("AttemptRecord")
rows["T13.1 RetainSuperset (4 S_p, 4 Z_p, 3 VS_p + reports + cached refusal, selected finish)"] = retain + selected
L = 16
scratch_rows = {
    "shared formation/assembly": V("MemberOperators<16>", m) + B(Zu) + s(W(16)) * pushcap(U) + s(W(16)) * (144 * pushcap(m)),
    "factor/condition/pivot": B(5 * 8 * F) + V(W(L), F) * 5 + s("Vec<usize>") * Bk + B(8 * F) + V(W(L), Bk) + V("TrackerE", F),
    "ordinary solve/refinement": V(W(L), N) + V(W(L), F) * 6 + V("(bool,f64,Wide)", F) + V("TrackerE", F) + V(W(L), F) * 4,
    "bounded fallback": V(W(16), Zu) + s(W(16)) * (144 * pushcap(m)) + V(W(L), N) + V("(EWS,EWS,f64)", F) + V("TrackerE", F) * 5,
    "recovery": s(W(L)) * (6 * pushcap(m) + 12 * pushcap(m) + pushcap(sp) + pushcap(Q)) + s(f"Option<{W(L)}>") * pushcap(N),
    "verification shared": (V(f"BoundedCoefficients<{L}>", m) + V("Option<BoundRefusal>", Bk) + s(W(16)) * (144 * pushcap(m))
                            + B(4 * F) + V(W(L), F) * 4 + V(W(L), Bk) * 2 + V("BlockBound<16>", Bk) + V("BlockRefusal", Bk)),
    "verification pass": (V(W(L), N) + s("Vec<Wide>") * (2 * N) + B(2 * N) + V(W(L), F) * 8 + V(W(L), N) * 2
                          + s(f"Option<{W(L)}>") * (5 * pushcap(Q)) + B(16 * b) + B(Bk) + V("BlockRefusal", Bk) * 2
                          + V("(Wide,Wide,Wide)", Bk) * 2 + V(W(L), Bk) * 2),
    "shifted profile/factor": (B(8 * F) + s("Vec<Wide>") * F + s(W(L)) * Hp * 2 + B(8 * F) + B(Bk) + V(f"Option<{W(L)}>", F) + V(W(L), F)),
    "stop rule": (B(Q) + s(W(L)) * (8 * pushcap(b)) + B(16 * pushcap(b) * 2) + B(4 * b * 16 + 2 * b * 16 * 2)
                  + (V("LazyE", min(O, Tr)) + V("TableE", O) * 2 + Sort("TableE", O)) * 1
                  + X({"Node(TrackerKey,Tracker)": Kt // 5 + 1, "Node(TrackerKey,())": Kt // 5 + 1})),
    "native certificate/publication": V("QuantityMeta", Q) + V("Binary64Outcome", Q) + B(4 * 8 * b * 2) + V("PublishedRow", Q) + B(4 * b * 8 + 8 * Q),
}
rows["T13.2 max ScratchSuperset (sum of the scratch rows, conservative)"] = total(scratch_rows.values())
# ------------------------------------------------------------------ T14 lanes, projection/maximum, certificate
mask = B(N + 2 * Bk)
lane = (mask + V("LawE", Q) + V(W(16), N) * 4 + V(W(16), F) * 3 + V("EnclosureE", Q) + V(W(16), F) * 2)
rows["T14.1 K lane + source lane (two views, masks, correction, readouts)"] = lane * 2
rows["T14.2 projection + maximum (descriptor rows, raw values, conversions, hull, one maximum helper)"] = (
    V("ProductRowSpec", P_final) + V("f64", P_final) + V("ProjectionOutcome", P_final) + V("ConversionE", P_final)
    + B(16 + 4 * 64) + s("Node_SR") * 262_144 + B(TXT["Text(err)"]) + V("MaximumE", m) + V("AliasE", P_final) + B(P_final * RID))
rows["T14.3 final certificate (verdicts, coverage bitmaps/scales, comparisons, intervals)"] = (
    V("ProductRowVerdict", P_final) + V("CoverageFact", n) + V("RunRow", Q) + V("SupportCoverage", g) + V("IntervalE", Q) * 2
    + V("BodyCoverage", b) + V("ProductValue", P_final) * 2 + V("ProductRow", P_final) + s("ArcPrepared"))
# ------------------------------------------------------------------ T15 C3 typed trace
rows["T15 C3 event storage and evidence completion"] = (
    V("PreparedMemberEvent", m) + V("ConversionEvent", 9 * m) + s("LaneTerminal") * 2 + V("FinalRowConversion", P_final)
    + V("OperationalSpent", 2 * m) + s("AdapterSnapshot") + s("RecordedInvocation") + s("RecordedCase") + B(4 * RID))

ASM = dict(ASSUMED)
WIDE = {L_: 16 + 8 * L_ for L_ in (4, 8, 16)}
for L_ in (4, 8, 16):
    ASM[f"s(Wide<{L_}>)"] = WIDE[L_]
    ASM[f"s(Option<Wide<{L_}>>)"] = WIDE[L_] + 8
    for nm, f in [("Shared", 400), ("Solved", 120), ("VerifyShared", 240), ("VerificationReport", 400)]:
        ASM[f"s({nm}<{L_}>)"] = f
    for nm in ("MemberOperators", "BoundedCoefficients", "PivotScreen", "BlockBound", "BlockCertificate", "BlockNorms", "BodyReport"):
        ASM[f"s({nm}<{L_}>)"] = {"MemberOperators": 144 * WIDE[L_], "BoundedCoefficients": 144 * WIDE[L_],
                                  "PivotScreen": 3 * WIDE[L_] + 16, "BlockBound": 4 * WIDE[L_],
                                  "BlockCertificate": 4 * WIDE[L_], "BlockNorms": 2 * WIDE[L_], "BodyReport": 8 * WIDE[L_]}[nm]
for a, v_ in {"OrdinarySeed": 400, "(String,[f64;3])": 48, "(String,f64,f64)": 40, "MaterialSelection": 64,
              "BasisRecord": 96, "SolverObservations": 96, "MemberIdentity": 120, "TermIdentity": 64,
              "SpringIdentity": 64, "[bool;6]": 6, "ProductMemberFacts": 512, "SrcCoord": 24, "SrcMember": 96,
              "SrcSpring": 24, "SrcConstraint": 16, "SrcNodal": 48, "SrcStation": 24, "SrcSupport": 72, "u32": 4,
              "OperationalSpent": 96, "CaptureError": 48, "NodeMapE": 40, "MemberMapE": 64, "SpringMapE": 48,
              "SupportMapE": 64, "ConstraintMapE": 40, "NodalMapE": 48, "StationMapE": 32, "SectionMapE": 64,
              "MaterialDescriptor": 96, "ExactWideSum": 1104, "(usize,ExactAccumulator,bool)": 1120,
              "(usize,LedgerNet)": 40, "u64": 8, "(usize,VecPair)": 32, "Pair": 16, "QuantityMeta": 32,
              "BodyMapE": 56, "BodyGeometry": 160, "Row6": 48, "Expansion6": 144, "Tag": 8, "ContributionE": 40,
              "ArcCasePrep": 1024, "ArcGroupPrep": 1024, "RegistryE": 128, "Vec<Wide>": 24, "PrecisionState": 64,
              "AttemptRecord": 256, "BlockRefusal": 48, "(u32,SpringKind)": 8, "PublishedRow": 64,
              "(u32,Kind,u64)": 16, "(QuantityId,u64)": 16, "(QuantityId,Binary64Outcome)": 32, "RetainedSolve": 2048,
              "TrackerE": 64, "(bool,f64,Wide)": 160, "(EWS,EWS,f64)": 2216, "Option<BoundRefusal>": 56,
              "(Wide,Wide,Wide)": 432, "Option<Wide<16>>": 152, "LazyE": 2232, "TableE": 48,
              "Node(TrackerKey,Tracker)": 4096, "Node(TrackerKey,())": 512, "Binary64Outcome": 24, "LawE": 64,
              "EnclosureE": 304, "ProductRowSpec": 96, "f64": 8, "ProjectionOutcome": 64, "ConversionE": 48,
              "MaximumE": 128, "AliasE": 32, "ProductRowVerdict": 48, "CoverageFact": 32, "RunRow": 32,
              "SupportCoverage": 64, "IntervalE": 32, "BodyCoverage": 32, "ProductValue": 32, "ProductRow": 96,
              "ArcPrepared": 512, "PreparedMemberEvent": 256, "ConversionEvent": 64, "LaneTerminal": 256,
              "FinalRowConversion": 64, "AdapterSnapshot": 512, "RecordedInvocation": 1024, "RecordedCase": 1024}.items():
    ASM[f"s({a})" if not a.startswith("Node(") else a] = v_
out = {"which": which, "counts": dict(n=n, m=m, s=sp, r=r, l=l, g=g, N=N, F=F, k=k, Q=Q, U=U, P=P, Zu=Zu, H=Hp,
                                      P_final=P_final, E_src=E_src, E_stf=E_stf, E_led=E_led, O=O, Kt=Kt)}
missing = sorted({a for x in rows.values() for a in x if a != "1" and a not in ASM})
out["missing_assumed"] = missing
out["assumed"] = ASM   # G5 part 2: the illustrative table, for the profile cross-check
out["rows"] = {k_: {"symbolic": v_.show(), "assumed_bytes": (v_.ev(ASM) if not missing else None)} for k_, v_ in rows.items()}
groups = {"T11": [k_ for k_ in rows if k_.startswith("T11")], "T12": [k_ for k_ in rows if k_.startswith("T12")],
          "T13": [k_ for k_ in rows if k_.startswith("T13")], "T14": [k_ for k_ in rows if k_.startswith("T14")],
          "T15": [k_ for k_ in rows if k_.startswith("T15")]}
if not missing:
    out["terms"] = {gname: sum(rows[k_].ev(ASM) for k_ in ks) for gname, ks in groups.items()}
    out["symbolic_terms"] = {gname: total(rows[k_] for k_ in ks).show() for gname, ks in groups.items()}
print(json.dumps(out, indent=1))
