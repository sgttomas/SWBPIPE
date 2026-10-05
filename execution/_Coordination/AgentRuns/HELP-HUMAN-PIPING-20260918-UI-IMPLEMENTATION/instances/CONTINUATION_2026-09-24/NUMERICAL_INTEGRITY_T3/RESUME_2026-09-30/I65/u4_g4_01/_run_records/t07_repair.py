"""I65 U4 G3: T07 (generic deep legacy-exact) repaired at the D1 caps (RV83 S-1).

Replaces G2 caps_arithmetic.py section (3). Every owner of the selected-or-refused
legacy-exact path (PP source_recovery::solve, :1434-1600) is listed once, everything live
at once (a conservative sum of the phase maxima). Sources are cited in RESIDUALS_G3.md.

Changes from G2 (RV83 S-1):
  (a) both Snapshot.identity copies (FKS exact_boundary.rs:466 Context, :1429 retained);
  (b) the identity String's serde_json::to_string capacity by J3, max(128, 2e), with the
      length constant +5;
  (c) descriptors #1 by its construction law (members, nodal, springs, supports, with Vec
      slack); #2 and #3 by the 16,384-unit descriptors_charge law, which gates them;
  (d) the Response, evaluation and projection Expansion temporaries;
  (e) exactly three descriptor generations live at once (#1 Sources, #2 FunctionalPlan,
      #3 RetainedFunctionalSet), not seven;
  plus owners G2 omitted: Sources.assembly (dense AssemblyEvidence), Sources.force_terms,
  the prepare_sources locals (maps, ledger, folded stiffness, identity builder), the caller's
  dense recovery stiffness, descriptors_charge's ordering vector and the attempt token.
Moving: requested plus the largest single old backing (one reallocation at a time).
Usage: python3 t07_repair.py <caps|milestone>
"""
import json, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from g3lib import X, B, s, V, A, Xc, Mat, HashReq, HashMov, buckets, pushcap, j3cap, total, ASSUMED

which = sys.argv[1] if len(sys.argv) > 1 else "caps"
if which == "caps":
    from g3lib import g4_caps as _gc
    _c = _gc(); n, m, g, sp, r, l = _c["n"], _c["m"], _c["g"], _c["s"], _c["r"], _c["l"]
else:  # the milestone counts (I54 BOUND:93); used only as a sanity evaluation
    n, m, g, sp, r, l = 2, 1, 4, 3, 6, 3
N = 6 * n
F = N
C = 144 * m + sp            # ordinary full contributions (frames 144 each, springs 1)
Z = min(N * N, C)
ft = l                      # force terms: one per nodal contribution; load_state None in D1
ID = 128                    # D1 identifier/text cap (bytes); clones have capacity = length
T = 256                     # exact Limits.expansion_terms (default)
EXP = 8 * pushcap(T)        # child bytes of one Expansion (PushCap(8,256)=256 slots)
assert EXP == 2048
Fn = 42 * m + N + sp + 6 * g
UNITS = 16_384              # descriptors_charge count limit (functionals.rs:427-476)

rows = {}
mov_candidates = {}

# ---------------- A. Sources (#1), live from prepare_sources' return to solve's end -------
# A1 dense AssemblyEvidence (SA structural_adapter.rs:62-93 + EvidenceParts::new)
rows["A1 Sources.assembly (dense AssemblyEvidence)"] = total([
    V("StiffnessContribution", C), Mat(8, N, N) * 2, Xc("Option<[f64;3]>", n),
    V("(usize,usize,bool)", m), B(8 * pushcap(sp)), Xc("FrameElement", m),
    Xc("(usize,f64)", sp)])
mov_candidates["AE contributions old backing"] = s("StiffnessContribution") * (pushcap(C) // 2)
# A2 force terms (source_recovery.rs:673, :735-740): push-built, each a cloned load id
rows["A2 Sources.force_terms"] = V("ForceContribution", ft) + B(ft * ID)
# A3 descriptors #1 construction law (source_recovery.rs:1059-1240, 1398-1432)
tr = [2, 4, 4, 2, 4, 4] * 2          # nonzeros per local row (FK lib.rs:798-809, 1696-1736)
assert sum(tr) == 40
end_rows = B(12 * 2 * ID) + s("AffineTerm") * 40 + (s("Vec") + B(8)) * 40 + (s("Vec") + B(8)) * 2
tc = [2, 4, 4, 2, 4 + 4, 4 + 4]      # station components; c4/c5 append rows[2]/rows[1]
st = X()
for t in tc:
    st = st + B(2 * ID) + s("AffineTerm") * pushcap(t) + (s("Vec") + B(32)) * t
st = st + s("Vec") * 4 + B(32)       # c0 offset (only when eigen != 0; kept conservatively)
stations = st * 5
per_member = end_rows + stations
nodal = (B(2 * ID) + s("AffineTerm") + s("Vec") + B(8)) * (6 * n)
springs = (B(2 * ID) + s("AffineTerm") + s("Vec") + B(8)) * sp
rows_d = 6 * g
support = (B(2 * ID) * rows_d + s("AffineTerm") * (4 * rows_d + 2 * (C + sp))
           + (s("Vec") + B(8)) * (C + sp) + s("Vec") * (4 * rows_d + 2 * ft) + B(8 * ft))
# outer Vec: simulate the actual growth (extend 12 exact-reserve, then pushes)
cap, length, prev = 0, 0, 0
def grow(req):
    global cap, prev
    if req > cap:
        prev = cap
        cap = max(2 * cap, req, 4)
for _ in range(m):
    grow(length + 12); length += 12
    for _ in range(30):
        grow(length + 1); length += 1
for _ in range(6 * n + sp + 6 * g):
    grow(length + 1); length += 1
assert length == Fn
outer = s("FunctionalDescriptor") * cap
mov_candidates["descriptors #1 outer old backing"] = s("FunctionalDescriptor") * prev
rows["A3 descriptors #1 (construction law)"] = outer + per_member * m + nodal + springs + support
units_member = 42 + 40 * 3 + 4 + 5 * (28 + 28 + 72 + 3)
units_1 = m * units_member + (6 * n + sp) * 4 + rows_d + (C + sp) * 3 + 2 * ft
# A4 identity: names/bits tallied from the identity.* call sites (G2 identity_sites.txt)
names = 8 + n + 6 * l + 6 * g + r + sp + 12 * m
bits = 8 + 3 * n + 4 * l + 4 * g + r + 4 * N + 3 * sp + 325 * m
e_id = 5 + names * (6 * ID + 3) + bits * 21   # escaped JSON [[names],[bits]] upper
rows["A4 Sources.identity String (J3 capacity)"] = B(j3cap(e_id))
mov_candidates["identity JSON old backing"] = B(j3cap(e_id) // 2)
# A5 id vectors (push-built; clones with capacity = length)
rows["A5 member/spring/support id vectors"] = (V("String", m) + B(m * ID) + V("(String,usize)", sp)
                                              + B(sp * ID) + V("(String,usize)", g) + B(g * ID))

# ---------------- B. prepare_sources locals (drop at its return; summed conservatively) ---
rows["B1 AssemblyEvidence construction stores (magnitudes, scatter_counts)"] = Mat(8, N, N) * 2
rows["B2 folded_stiffness"] = Mat(8, N, N)
# B3 folded_force LoadLedger -> finish (I54 BOUND 'AssembledForce, nodal family'); l_i <= l
rows["B3 folded_force ledger"] = (B(8 * N) + V("ForceTerm", l) + V("Option<FormationRecord>", l)
                                  + s("Vec<usize>") * N + B(8 * (4 * N + 2 * l))
                                  + B(8 * pushcap(min(l, N))) + B(l * ID))
rows["B4 hash maps and sets"] = total([
    HashReq("(&str,usize)", n), HashReq("(&str,&T)", l), HashReq("&str", l),
    HashReq("(&str,&T)", g), HashReq("&str", g), HashReq("&str", m), HashReq("(&str,usize)", sp)])
for nm, k, h in [("node_ids", "(&str,usize)", n), ("authored_loads", "(&str,&T)", l)]:
    b = buckets(h)
    mov_candidates[nm + " old table"] = s(k) * (b // 2) + B(b // 2 + 23)
rows["B5 boundary vectors"] = (Xc("Option<usize>", N) + Xc("Option<f64>", N)
                               + V("(&str,usize,u64)", sp) + B(N) + V("(usize,f64)", sp)
                               + B(8 * m) + B(8 * N))      # + member eigen_axial, prescribed scratch
rows["B6 identity builder (names Vec + strings, bits Vec)"] = (V("String", names) + B(names * ID)
                                                              + B(8 * max(4, 2 * bits)))
rows["B7 descriptor construction transients (one member's rows + one key clone)"] = (
    s("FunctionalDescriptor") * 16 + B(12 * 2 * ID) + s("AffineTerm") * (12 * 4)
    + (s("Vec") + B(8)) * 42 + B(2 * ID))

# ---------------- C. Context #1 (exact_boundary.rs:282-486) ------------------------------
snapshot = (B(e_id) + Mat(8, N, N) + B(8 * N) + B(8 * F) + B(16 * N)
            + Xc("StiffnessContribution", C) + Xc("ForceContribution", ft) + B(ft * ID))
rows["C1 Context Snapshot #1 (identity copy 1 = length)"] = snapshot
rows["C2 Context k"] = s("Vec") * N + s("Expansion") * (N * N) + B(8 * (4 * Z + 2 * C))
rows["C3 Context f"] = s("Expansion") * N + B(8 * (4 * N + 2 * ft))
rows["C4 blocks"] = s("Vec<usize>") * pushcap(N) + B(N * 32)
rows["C5 witnesses"] = V("BlockWitness", F) + B(F * (16 + 4 * EXP))
rows["C6 new_charged transients"] = Mat(8, N, N) + B(2 * N + 8 * N) + B(6 * EXP)
rows["C7 source_preflight and attempt token"] = B(16)

# ---------------- D. Response #1 (exact_boundary.rs:525-597) ----------------------------
rows["D1 Response displacement and reactions"] = s("Ratio") * (2 * N) + B(4 * N * EXP)
rows["D2 solve temporaries (11 Expansions, block phase)"] = B(11 * EXP)

# ---------------- E. FunctionalPlan (#2), only when descriptors_charge passes ------------
# exact clone; units = descriptors + offset products + atoms + terms + term products + atoms
desc_clone = s("FunctionalDescriptor") * Fn + B(Fn * 2 * ID) + (s("AffineTerm") + s("Vec") + B(8)) * UNITS
rows["E1 FunctionalPlan descriptors #2 (16,384-unit law)"] = desc_clone
rows["E2 descriptors_charge ordering vectors (plan, retain)"] = B(2 * 8 * Fn)

# ---------------- F. evaluate_functionals (functionals.rs:550-606) ------------------------
rows["F1 FunctionalSet values"] = s("Ratio") * Fn + B(Fn * 2 * EXP)
rows["F2 evaluate transients"] = s("Expansion") * N + B(N * EXP + 12 * EXP)

# ---------------- G. projections (source_recovery.rs:1456-1475) -------------------------
rows["G1 projection vectors"] = V("QualifiedFunctionalProjection", Fn) + V("QualifiedProjection", 2 * N)
mov_candidates["functional projections old backing"] = s("QualifiedFunctionalProjection") * (pushcap(Fn) // 2)
rows["G2 project temporaries"] = B(6 * EXP)

# ---------------- H. RetainedFunctionalSet (functionals.rs:751-824; exact_boundary.rs:1378-1452)
rows["H1 retained Snapshot #2 (identity copy 2 = length)"] = snapshot
rows["H2 retained witnesses, displacement, reactions, projections"] = (
    Xc("BlockWitness", F) + B(F * (16 + 4 * EXP)) + s("Ratio") * (2 * N) + B(2 * N * 2 * EXP)
    + Xc("RetainedProjection", 2 * N) + B(3 * N))
rows["H3 retained descriptors #3 (16,384-unit law)"] = desc_clone
rows["H4 retained values and projections"] = s("Ratio") * Fn + B(Fn * 2 * EXP) + Xc("RetainedFunctionalProjection", Fn)

# ---------------- I. selected output (source_recovery.rs:1480-1600) ----------------------
rows["I1 selected output"] = (B(32 * N) + Xc("MemberRecovery", m) + B(m * ID) + Xc("SpringAction", sp)
                              + B(sp * ID) + Xc("SupportActions", g) + B(g * ID))

# ---------------- J. caller input (PP lib.rs:3679-3687) ---------------------------------
rows["J1 caller dense recovery stiffness (formed.to_dense)"] = Mat(8, N, N)

req = total(rows.values())
mov_max_name = max(mov_candidates, key=lambda k: mov_candidates[k].ev(ASSUMED))
out = {
    "which": which,
    "counts": dict(n=n, m=m, g=g, s=sp, r=r, l=l, N=N, F=F, C=C, Z=Z, Fn=Fn, ft=ft),
    "identity": {"names": names, "bits": bits, "escaped_json_upper": e_id, "capacity_J3": j3cap(e_id)},
    "descriptor_units_#1": units_1, "descriptor_unit_limit": UNITS,
    "descriptor_generations_live_at_once": 3,
    "descriptors_#1_outer_capacity": cap,
    "rows": {k: {"symbolic": v.show(), "assumed_bytes": v.ev(ASSUMED)} for k, v in rows.items()},
    "T07_requested": {"symbolic": req.show(), "assumed_bytes": req.ev(ASSUMED)},
    "moving_extra": {"largest_single_old_backing": mov_max_name,
                     "symbolic": mov_candidates[mov_max_name].show(),
                     "assumed_bytes": mov_candidates[mov_max_name].ev(ASSUMED)},
}
out["T07_moving_assumed_bytes"] = out["T07_requested"]["assumed_bytes"] + out["moving_extra"]["assumed_bytes"]
print(json.dumps(out, indent=1))
