"""T4-I12 repair round 02: T4-U3's references, with T4-RV5's findings applied.

Reads the frozen round-01 file (R4/T4-I12/u3_reference_cases.json, sha256 d7eb1eba...; SHA256SUMS
ee4b411f...) and writes the full round-02 file. Every round-00/01 leaf is kept byte-identical;
round 02 only ADDS keys (check_round02.py proves it leaf by leaf). What it adds:

  B-1  the SH-140 hanger's installed/cold/hot loads, travel and load-side reference, verbatim from
       the demo fixture, in every system document (9 documents);
  S-1  one zero-floor rule for every FK unit-level array, and its floors (round_02.zero_floors),
       each from the case's own inputs in exact rationals;
  S-2  the Euler-Bernoulli dependence of the system and NI cases (D-6 not decided);
  S-3  concrete, machine-applicable patches (RFC 6902 operations) and exact expected blocking-code
       sets for every refusal variant;
  N-1..N-4, the S20 applied-set reconciliation, and two small unit cases (N-3).

Standard library only.  Usage: python -I u3_round02.py <frozen round-01 json> <output json>
"""
import copy
import hashlib
import json
import sys
from decimal import Decimal, getcontext
from fractions import Fraction as Fr

getcontext().prec = 80
FROZEN_SHA256 = "d7eb1eba0d893c52c740222f52ea05cbeaa84937fc6d1edf1c9b14d3b0f5bce7"
CHECKS = []


def check(name, ok):
    CHECKS.append((name, bool(ok)))
    if not ok:
        print("FAIL:", name)


# ------------------------------------------------------------------ exact arithmetic
def F(x):
    if isinstance(x, dict):
        return Fr(x["exact"])
    if isinstance(x, float):
        return Fr(repr(x))
    return Fr(x)


def vec(v):
    if isinstance(v, dict) and "exact" in v and isinstance(v["exact"], list):
        v = v["exact"]
    return [F(x) for x in v]


def mat(a):
    return [[F(x) for x in r] for r in a]


def zeros(n, m):
    return [[Fr(0)] * m for _ in range(n)]


def eye(n):
    return [[Fr(int(i == j)) for j in range(n)] for i in range(n)]


def tr(a):
    return [list(r) for r in zip(*a)]


def mm(a, b):
    bt = tr(b)
    return [[sum((x * y for x, y in zip(r, c)), Fr(0)) for c in bt] for r in a]


def mv(a, v):
    return [sum((x * y for x, y in zip(r, v)), Fr(0)) for r in a]


def vadd(a, b):
    return [x + y for x, y in zip(a, b)]


def vsub(a, b):
    return [x - y for x, y in zip(a, b)]


def vsc(s, a):
    return [s * x for x in a]


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def dot(a, b):
    return sum((x * y for x, y in zip(a, b)), Fr(0))


def absm(a):
    return [[abs(x) for x in r] for r in a]


def absv(v):
    return [abs(x) for x in v]


def skew(v):
    return [[Fr(0), -v[2], v[1]], [v[2], Fr(0), -v[0]], [-v[1], v[0], Fr(0)]]


def hcat(*blocks):
    return [sum((b[i] for b in blocks), []) for i in range(len(blocks[0]))]


def rank(a):
    m = [list(r) for r in a]
    rk, rows, cols = 0, len(m), len(m[0])
    for c in range(cols):
        p = next((i for i in range(rk, rows) if m[i][c] != 0), None)
        if p is None:
            continue
        m[rk], m[p] = m[p], m[rk]
        for i in range(rows):
            if i != rk and m[i][c] != 0:
                f = m[i][c] / m[rk][c]
                m[i] = [x - f * y for x, y in zip(m[i], m[rk])]
        rk += 1
    return rk


def ex(x):
    x = Fr(x)
    return str(x.numerator) if x.denominator == 1 else f"{x.numerator}/{x.denominator}"


def dec(x, sig=20):
    x = Fr(x)
    if x == 0:
        return "0"
    return format(Decimal(x.numerator) / Decimal(x.denominator), "." + str(sig) + "g")


def both(x):
    return {"exact": ex(x), "decimal": dec(x, 34)}


def bothv(v):
    return {"exact": [ex(x) for x in v], "decimal": [dec(x, 34) for x in v]}


# ------------------------------------------------------------------ connector kinematics (JR section 2)
def q_kin(d, ai, aj, r, Q):
    """q from the kinematic definition (not from a stored B)."""
    ui, ti, uj, tj = d[0:3], d[3:6], d[6:9], d[9:12]
    vi = vadd(ui, cross(ti, ai))
    vj = vadd(uj, cross(tj, aj))
    tc = vsc(Fr(1, 2), vadd(ti, tj))
    QT = tr(Q)
    return mv(QT, vsub(vsub(vj, vi), cross(tc, r))) + mv(QT, vsub(tj, ti))


def B_kin(ai, aj, r, Q):
    cols = []
    for k in range(12):
        e = [Fr(0)] * 12
        e[k] = Fr(1)
        cols.append(q_kin(e, ai, aj, r, Q))
    return tr(cols)


def r_operand(xi, xj, ai, aj):
    """RV130 S-4: r's operand is |x_j - x_i| + |a_j - a_i| per component (never |r|)."""
    return [abs(a) + abs(b) for a, b in zip(vsub(xj, xi), vsub(aj, ai))]


def B_operand(ai, aj, rop, Q):
    """B's closed form Q^T[-I, S(a_i)+S(r/2), I, -S(a_j)+S(r/2)] (translation rows) and
    Q^T[0, -I, 0, I] (rotation rows), with every operand replaced by its absolute value
    (rop = r's operand, r_operand())."""
    QTa = absm(tr(Q))
    I3, Z3 = eye(3), zeros(3, 3)
    hr = vsc(Fr(1, 2), rop)
    s = lambda v: absm(skew(v))
    add = lambda a, b: [[x + y for x, y in zip(ra, rb)] for ra, rb in zip(a, b)]
    Bt = mm(QTa, hcat(I3, add(s(ai), s(hr)), I3, add(s(aj), s(hr))))
    Br = mm(QTa, hcat(Z3, I3, Z3, I3))
    return Bt + Br


def ldl_pivots(k):
    """Symmetric elimination in exact rationals without pivoting; a zero pivot whose remaining
    column is nonzero makes the matrix indefinite (returned as the flag), never skipped."""
    a = [list(r) for r in k]
    n = len(a)
    piv, zero_pivot_nonzero_column = [], False
    for c in range(n):
        p = a[c][c]
        piv.append(p)
        if p == 0:
            if any(a[i][c] != 0 for i in range(c + 1, n)):
                zero_pivot_nonzero_column = True
            continue
        for i in range(c + 1, n):
            f = a[i][c] / p
            for j in range(c, n):
                a[i][j] -= f * a[c][j]
    return piv, zero_pivot_nonzero_column


def from_upper21(u):
    M = zeros(6, 6)
    k = 0
    for i in range(6):
        for j in range(i, 6):
            M[i][j] = M[j][i] = F(u[k])
            k += 1
    return M


def upper21(M):
    return [M[i][j] for i in range(6) for j in range(i, 6)]


def K_from_H(H, Ls):
    dsc = [Ls, Ls, Ls, Fr(1), Fr(1), Fr(1)]
    return [[H[i][j] / (dsc[i] * dsc[j]) for j in range(6)] for i in range(6)]


def H_from_K(K, Ls):
    dsc = [Ls, Ls, Ls, Fr(1), Fr(1), Fr(1)]
    return [[K[i][j] * dsc[i] * dsc[j] for j in range(6)] for i in range(6)]


def rigid_modes(xi, xj, o):
    modes = []
    for k in range(3):
        t = [Fr(0)] * 3
        t[k] = Fr(1)
        modes.append(t + [Fr(0)] * 3 + t + [Fr(0)] * 3)
    for k in range(3):
        w = [Fr(0)] * 3
        w[k] = Fr(1)
        modes.append(cross(w, vsub(xi, o)) + w + cross(w, vsub(xj, o)) + w)
    return modes


def get(doc, pointer):
    o = doc
    for part in pointer.strip("/").split("/"):
        o = o[int(part)] if isinstance(o, list) else o[part]
    return o


# ------------------------------------------------------------------ read and verify the frozen file
src, out_path = sys.argv[1], sys.argv[2]
raw = open(src, "rb").read()
check("input is the frozen round-01 file (sha256 d7eb1eba...)", hashlib.sha256(raw).hexdigest() == FROZEN_SHA256)
doc = json.loads(raw)
r01 = copy.deepcopy(doc)
cases = doc["cases"]
R02 = "T4-I12 repair round 02"
PROV = "invented T4-I12 re-authoring of the invented demo; not library, catalog, manufacturer or code-rule data"
R02_PROV = R02 + "; " + PROV

# =============================================================================================
# B-1: restore SH-140's hanger metadata verbatim from the demo fixture
# =============================================================================================
# Verbatim copy of support:SH-140's "hanger" object in P/fixtures/product_preview/invented_preview_model.json
# (blob f736148e2ee69b867164bec510ea34a5b54865f2 at origin/main 10b70036ef; file sha256 986c0559...871c,
# the same bytes as the round-0 basis ed012c7ccf).
DEMO_HANGER = json.loads("""{
 "hanger_type": "variable_spring_hanger",
 "stiffness": {"dof": "UZ", "value": {"value": 42000, "unit": "N/m"}},
 "installed_load": {"value": 460, "unit": "N"},
 "cold_load": {"value": 430, "unit": "N"},
 "hot_load": {"value": 390, "unit": "N"},
 "travel_range": {"value": 0.045, "unit": "m"},
 "manufacturer_reference": "invented_user_entered_variable_hanger_reference_no_catalog",
 "source_reference": "invented_user_entered_spring_hanger_values_no_catalog",
 "load_side_review_reference": "invented_review_only_hot_cold_load_metadata",
 "mechanics_consumption": "linear_spring_primitive_user_stiffness"
}""")
RESTORED = ["installed_load", "cold_load", "hot_load", "travel_range", "load_side_review_reference"]

doc_paths = []


def find_docs(o, p):
    if isinstance(o, dict):
        if "model" in o and isinstance(o["model"], dict) and "schema_version" in o["model"]:
            doc_paths.append(p)
        for k, v in o.items():
            find_docs(v, p + [k])
    elif isinstance(o, list):
        for i, v in enumerate(o):
            find_docs(v, p + [str(i)])


find_docs(doc, [])
check("B-1: nine system documents (0.3.0 x2, 0.4.0 x7) found", len(doc_paths) == 9)
hanger_fix = []
for p in doc_paths:
    m = get(doc, "/" + "/".join(p))["model"]
    sh = [s for s in m["supports"] if s["id"] == "support:SH-140"]
    check(f"B-1 {'/'.join(p)}: exactly one SH-140", len(sh) == 1)
    h = sh[0]["hanger"]
    check(f"B-1 {'/'.join(p)}: the frozen hanger is the demo hanger minus the restored fields (no value differs)",
          {k: v for k, v in DEMO_HANGER.items() if k not in RESTORED} == h)
    sh[0]["hanger"] = copy.deepcopy(DEMO_HANGER)
    hanger_fix.append("/" + "/".join(p) + "/model/supports/" + str(m["supports"].index(sh[0])) + "/hanger")

# =============================================================================================
# S-1: one zero-floor rule for every FK unit-level array
# =============================================================================================
EVALS = []  # (expected pointer, inputs pointer or None, q_ref pointer override, kind)
for cid in ["U3-J1-LATERAL", "U3-J1-COMMON-ROTATION", "U3-J2-ROTATION", "U3-J2-ROTATION-HELD", "U3-REF-ENDMOMENT",
            "U3-GENERIC-SKEW-OFFSET-PRESTRESS", "U3-FRAME-COVARIANCE", "U3-OFFSETS", "U3-REVERSAL"]:
    EVALS.append((f"/cases/{cid}/expected", f"/cases/{cid}/inputs", None, "evaluate"))
EVALS.append(("/cases/U3-REF-ENDMOMENT/companion_pure_qrz", "/cases/U3-REF-ENDMOMENT/inputs", None, "evaluate"))
for comp in ["tx", "ty", "tz", "rx", "ry", "rz"]:
    EVALS.append((f"/cases/U3-SIX-COMPONENTS/expected/{comp}", "/cases/U3-SIX-COMPONENTS/inputs", None, "evaluate"))
EVALS.append(("/cases/U3-COUPLED-H-SCALE-PRELOAD/expected/coupled", "/cases/U3-COUPLED-H-SCALE-PRELOAD/inputs/coupled", None, "evaluate"))
EVALS.append(("/cases/U3-COUPLED-H-SCALE-PRELOAD/expected/preload_installed_d0", "/cases/U3-COUPLED-H-SCALE-PRELOAD/inputs/coupled",
              "/cases/U3-COUPLED-H-SCALE-PRELOAD/inputs/preload_q_ref", "evaluate"))
EVALS.append(("/cases/U3-RAW-DIFFERENCE-NEGATIVE/expected/historical_correct", None, None, "evaluate"))
EVALS.append(("/cases/U3-KD5-UTM-SKEW-OFFSET-COUPLED/expected/identical_at_every_location",
              "/cases/U3-KD5-UTM-SKEW-OFFSET-COUPLED/inputs/shared", None, "kd5"))
for st in ["node_i_anchored_node_j_free", "node_i_anchored_node_j_ux_held", "both_nodes_held"]:
    EVALS.append((f"/cases/U3-PRELOAD-RELIEF/expected/{st}", "/cases/U3-PRELOAD-RELIEF/inputs", None, "solved"))
EVALS.append(("/cases/U3-B-ORACLE/expected", "/cases/U3-B-ORACLE/inputs", None, "oracle"))

# round-01 stated floors stay in force where they were given (a floor never falls below a stated one)
STATED = {
    "/cases/U3-J1-COMMON-ROTATION/expected": {"translation": Fr(3, 1000), "force": Fr(240), "energy": Fr(9, 25)},
    "/cases/U3-RAW-DIFFERENCE-NEGATIVE/expected/historical_correct": {"translation": Fr(1, 50), "force": Fr(2, 5), "energy": Fr(1, 250)},
}
FAM12 = ["translation"] * 3 + ["rotation"] * 3 + ["translation"] * 3 + ["rotation"] * 3
FAM12F = ["force"] * 3 + ["moment"] * 3 + ["force"] * 3 + ["moment"] * 3
FAM6Q = ["translation"] * 3 + ["rotation"] * 3
FAM6G = ["force"] * 3 + ["moment"] * 3


def family_entries(e, kind):
    """(family, exact value) for every compared entry of an evaluation object."""
    out = []
    for key, val in e.items():
        if key in ("B",):
            out += [("B", F(x)) for r in val for x in r]
        elif key == "Ke":
            out += [("Ke", F(x)) for r in val for x in r]
        elif key in ("r", "p_i", "p_j"):
            out += [("geometry", x) for x in vec(val)]
        elif key in ("q", "q_for_d_given"):
            out += list(zip(FAM6Q, vec(val)))
        elif key in ("g", "g_recovered", "K_qref"):
            out += list(zip(FAM6G, vec(val)))
        elif key == "F_global":
            out += [("force", x) for x in vec(val)]
        elif key == "M_global":
            out += [("moment", x) for x in vec(val)]
        elif key == "end_actions_node_on_element":
            for blk, fam in (("Fi", "force"), ("Mi", "moment"), ("Fj", "force"), ("Mj", "moment")):
                out += [(fam, x) for x in vec(val[blk])]
        elif key in ("installed_rhs_BT_K_qref", "support_on_element_reactions"):
            out += list(zip(FAM12F, vec(val)))
        elif key in ("energy", "virtual_work"):
            out.append(("energy", F(val)))
        elif key == "d" and kind == "solved":
            out += list(zip(FAM12, vec(val)))
    return out


def recover_K(B, Ke):
    """K = (B B^T)^-1 B Ke B^T (B B^T)^-1 (B has full row rank 6)."""
    BBt = mm(B, tr(B))
    n = 6
    aug = [list(BBt[i]) + [Fr(int(i == j)) for j in range(n)] for i in range(n)]
    for c in range(n):
        p = next(i for i in range(c, n) if aug[i][c] != 0)
        aug[c], aug[p] = aug[p], aug[c]
        piv = aug[c][c]
        aug[c] = [x / piv for x in aug[c]]
        for i in range(n):
            if i != c and aug[i][c] != 0:
                f = aug[i][c]
                aug[i] = [x - f * y for x, y in zip(aug[i], aug[c])]
    inv = [r[n:] for r in aug]
    return mm(mm(inv, mm(mm(B, Ke), tr(B))), inv)


zero_floors = {}
floor_details = []
exact_zero_families = []
for ptr, inp_ptr, qref_ptr, kind in EVALS:
    e = get(doc, ptr)
    if kind == "kd5":
        inp = get(doc, inp_ptr)
        Q, ai, aj, r = mat([[x["exact"] for x in row] for row in inp["Q_row_major_columns_are_axes"]]), \
            [F(x) for x in inp["a_i_global"]], [F(x) for x in inp["a_j_global"]], [F(x) for x in inp["r_exact"]]
        K = mat(inp["K_physical"])
        d = [F(x) for x in inp["d"]]
        qref = [F(x) for x in inp["q_ref"]]
        locs = get(doc, "/cases/U3-KD5-UTM-SKEW-OFFSET-COUPLED/inputs/locations")
        diffs = {k: vsub([F(x) for x in L_["x_j"]], [F(x) for x in L_["x_i"]]) for k, L_ in locs.items()}
        check(f"{ptr}: x_j - x_i is the same at every location", len({tuple(v_) for v_ in diffs.values()}) == 1)
        check(f"{ptr}: r_exact = (x_j - x_i) + (a_j - a_i)", r == vadd(list(diffs.values())[0], vsub(aj, ai)))
        xi, xj = [F(x) for x in locs["ORDINARY"]["x_i"]], [F(x) for x in locs["ORDINARY"]["x_j"]]
    elif kind == "oracle":
        inp = get(doc, inp_ptr)
        Q, ai, aj, r = mat(inp["Q_row_major_columns_are_axes"]), vec(inp["a_i_global"]), vec(inp["a_j_global"]), vec(inp["r"])
        xi, xj = vec(inp["x_i"]), vec(inp["x_j"])
        check(f"{ptr}: r = (x_j + a_j) - (x_i + a_i)", r == vsub(vadd(xj, aj), vadd(xi, ai)))
        gg, d = vec(inp["g_given"]), vec(inp["d_given"])
        K, qref = None, None
    elif kind == "solved":
        inp = get(doc, inp_ptr)
        Q, ai, aj = mat(inp["Q_row_major_columns_are_axes"]), vec(inp["a_i_global"]), vec(inp["a_j_global"])
        xi, xj = vec(inp["x_i"]), vec(inp["x_j"])
        r = vsub(vadd(xj, aj), vadd(xi, ai))
        K, qref = mat(inp["K_physical"]), vec(inp["q_ref"])
        d = vec(e["d"]) if "d" in e else [Fr(0)] * 12
    elif inp_ptr is None:  # historical raw-difference control: geometry from the published r, p
        Bpub = mat(e["B"])
        xi, xj = vec(e["p_i"]), vec(e["p_j"])
        ai = aj = [Fr(0)] * 3
        r = vec(e["r"])
        Q = [[Fr(int(i == j)) for j in range(3)] for i in range(3)]
        K = recover_K(Bpub, mat(e["Ke"]))
        qref = [Fr(0)] * 6
        check(f"{ptr}: stress-free (K q_ref published as 0)", all(x == 0 for x in vec(e["K_qref"])))
        d = vec(e["d"])
    else:
        inp = get(doc, inp_ptr)
        Q, ai, aj = mat(inp["Q_row_major_columns_are_axes"]), vec(inp["a_i_global"]), vec(inp["a_j_global"])
        xi, xj = vec(inp["x_i"]), vec(inp["x_j"])
        r = vsub(vadd(xj, aj), vadd(xi, ai))
        K = mat(inp["K_physical"])
        qref = vec(get(doc, qref_ptr)) if qref_ptr else vec(inp["q_ref"])
        d = vec(e["d"])
        check(f"{ptr}: K_physical = D^-1 H D^-1 from the authored H and Ls",
              K == K_from_H(from_upper21(inp["H_upper_triangle_21_N_m"]), F(inp["translation_scale_Ls_m"])))
    B = B_kin(ai, aj, r, Q)
    # independent re-verification of the published (round-00/01) values from the case's own inputs
    if "B" in e:
        check(f"{ptr}: published B equals B formed from the kinematic definition (72 entries)", mat(e["B"]) == B)
    check(f"{ptr}: rank B = 6", rank(B) == 6)
    if kind == "oracle":
        f = mv(tr(B), gg)
        ea = e["end_actions_node_on_element"]
        check(f"{ptr}: published end actions equal B^T g_given (N-4: the product forms B^T g from b())",
              f == vec(ea["Fi"]) + vec(ea["Mi"]) + vec(ea["Fj"]) + vec(ea["Mj"]))
        check(f"{ptr}: q_for_d_given = B d_given", mv(B, d) == vec(e["q_for_d_given"]))
        check(f"{ptr}: virtual work g.(B d) = f.d", dot(gg, mv(B, d)) == F(e["virtual_work"]) == dot(f, d))
        Bop = B_operand(ai, aj, r_operand(xi, xj, ai, aj), Q)
        v = mv(Bop, absv(d))
        P = mv(tr(Bop), absv(gg))
        S = {"B": max(max(rw) for rw in Bop),
             "translation": max(v[0:3]), "rotation": max(v[3:6]),
             "force": max(P[0:3] + P[6:9]), "moment": max(P[3:6] + P[9:12]),
             "energy": dot(absv(gg), v)}
    else:
        Ke = mm(mm(tr(B), K), B)
        if "Ke" in e:
            check(f"{ptr}: published Ke equals B^T K B", mat(e["Ke"]) == Ke)
        if kind != "solved":
            q = mv(B, d)
            check(f"{ptr}: published q equals B d (and the kinematic q)", vec(e["q"]) == q == q_kin(d, ai, aj, r, Q))
            g = mv(K, vsub(q, qref))
            check(f"{ptr}: published g equals K (q - q_ref)", vec(e["g"]) == g)
            check(f"{ptr}: published energy equals (q - q_ref).g / 2", F(e["energy"]) == dot(vsub(q, qref), g) / 2)
            fb = mv(tr(B), g)
            ea = e["end_actions_node_on_element"]
            check(f"{ptr}: published end actions equal B^T g", fb == vec(ea["Fi"]) + vec(ea["Mi"]) + vec(ea["Fj"]) + vec(ea["Mj"]))
            check(f"{ptr}: published B^T K q_ref", vec(e["installed_rhs_BT_K_qref"]) == mv(tr(B), mv(K, qref)))
        else:
            q = mv(B, d)
            if "q" in e:
                check(f"{ptr}: published q equals B d", vec(e["q"]) == q)
            g = mv(K, vsub(q, qref))
            check(f"{ptr}: published g_recovered equals K (B d - q_ref)", vec(e["g_recovered"]) == g)
            rhs = mv(tr(B), mv(K, qref))
            check(f"{ptr}: published support-on-element reactions equal Ke d - B^T K q_ref",
                  vec(e["support_on_element_reactions"]) == vsub(mv(Ke, d), rhs))
            if "energy" in e:
                check(f"{ptr}: published energy", F(e["energy"]) == dot(vsub(q, qref), g) / 2)
        Bop = B_operand(ai, aj, r_operand(xi, xj, ai, aj), Q)
        v = mv(Bop, absv(d))
        w = vadd(v, absv(qref))
        G = mv(absm(K), w)
        P = mv(tr(Bop), G)
        S = {"B": max(max(rw) for rw in Bop),
             "Ke": max(max(rw) for rw in mm(mm(tr(Bop), absm(K)), Bop)),
             "translation": max(w[0:3]), "rotation": max(w[3:6]),
             "force": max(G[0:3] + P[0:3] + P[6:9]), "moment": max(G[3:6] + P[3:6] + P[9:12]),
             "energy": dot(w, G) / 2}
        if xi is not None:
            S["geometry"] = max(max(abs(a) + abs(b) for a, b in zip(vsub(xj, xi), vsub(aj, ai))),
                                max(abs(a) + abs(b) for a, b in zip(xi, ai)), max(abs(a) + abs(b) for a, b in zip(xj, aj)))
    ents = family_entries(e, kind)
    fams = sorted({f for f, _ in ents})
    floors = {}
    for fam in fams:
        mx = max(abs(x) for f, x in ents if f == fam)
        if fam in S:
            check(f"{ptr}: operand scale bounds the family {fam} (|exp| <= S)", mx <= S[fam])
        fl = max([mx, S.get(fam, Fr(0)), STATED.get(ptr, {}).get(fam, Fr(0))])
        if fl == 0:
            # exact-zero family: every term of every operand path has an exact-zero factor
            check(f"{ptr}: family {fam} has floor 0 only because its operand scale is exactly 0", fam in S and S[fam] == 0 and mx == 0)
            exact_zero_families.append((ptr, fam))
        else:
            check(f"{ptr}: family {fam} floor is positive", fl > 0)
        floors[fam] = fl
        floor_details.append((ptr, fam, mx, S.get(fam), fl))
    zero_floors[ptr] = {fam: {"exact": ex(fl), "decimal": dec(fl)} for fam, fl in floors.items()}
    # U3-COUPLED: q-hat (scaled coordinates, D^-1 q) belongs to the coupled evaluation
    if ptr.endswith("/expected/coupled"):
        Ls = F(get(doc, inp_ptr)["translation_scale_Ls_m"])
        qhat = vec(get(doc, "/cases/U3-COUPLED-H-SCALE-PRELOAD/expected/qhat"))
        check("U3-COUPLED: q-hat = D^-1 q", qhat == [x / Ls for x in q[0:3]] + q[3:6])
        Sq = max([x / Ls for x in w[0:3]] + w[3:6])
        flq = max(max(abs(x) for x in qhat), Sq)
        zero_floors["/cases/U3-COUPLED-H-SCALE-PRELOAD/expected/qhat"] = {"scaled_coordinate": {"exact": ex(flq), "decimal": dec(flq)}}

# discriminators keep their power under the rule: margin = |wrong - exp| / (1e-12 * max(|exp|, floor))
margins = {}


def margin_vec(ptr, wrong, exp, fams):
    fl = zero_floors[ptr]
    best = Fr(0)
    for w_, e_, f_ in zip(wrong, exp, fams):
        tol = Fr(1, 10 ** 12) * max(abs(e_), F(fl[f_]["exact"]))
        best = max(best, abs(w_ - e_) / tol)
    return best


gen = "/cases/U3-GENERIC-SKEW-OFFSET-PRESTRESS/expected"
for k, dsc in enumerate(cases["U3-GENERIC-SKEW-OFFSET-PRESTRESS"]["wrong_result_discriminators"]):
    if "q" in dsc:
        margins[f"U3-GENERIC discriminator {k} ({dsc['what'][:40]})"] = margin_vec(gen, vec(dsc["q"]), vec(get(doc, gen)["q"]), FAM6Q)
off = "/cases/U3-OFFSETS/expected"
for k, dsc in enumerate(cases["U3-OFFSETS"]["wrong_result_discriminators"]):
    if "q" in dsc:
        margins[f"U3-OFFSETS discriminator {k}"] = margin_vec(off, vec(dsc["q"]), vec(get(doc, off)["q"]), FAM6Q)
pr = "/cases/U3-PRELOAD-RELIEF/expected/node_i_anchored_node_j_free"
margins["U3-PRELOAD-RELIEF residual-omitted g"] = margin_vec(pr, vec(cases["U3-PRELOAD-RELIEF"]["wrong_result_discriminators"][0]["g"]),
                                                            vec(get(doc, pr)["g_recovered"]), FAM6G)
cr = "/cases/U3-J1-COMMON-ROTATION/expected"
craw = cases["U3-J1-COMMON-ROTATION"]["wrong_result_discriminators"][0]
margins["U3-J1-COMMON-ROTATION raw q_y"] = F(craw["q_y_m"]) / (Fr(1, 10 ** 12) * F(zero_floors[cr]["translation"]["exact"]))
margins["U3-J1-COMMON-ROTATION raw g_y"] = F(craw["g_y_N"]) / (Fr(1, 10 ** 12) * F(zero_floors[cr]["force"]["exact"]))
margins["U3-J1-COMMON-ROTATION raw energy"] = F(craw["energy_J"]) / (Fr(1, 10 ** 12) * F(zero_floors[cr]["energy"]["exact"]))
hr_ = "/cases/U3-RAW-DIFFERENCE-NEGATIVE/expected/historical_correct"
hraw = cases["U3-RAW-DIFFERENCE-NEGATIVE"]["expected"]["historical_raw"]
margins["U3-RAW historical raw q_y"] = F(hraw["q_y_m"]) / (Fr(1, 10 ** 12) * F(zero_floors[hr_]["translation"]["exact"]))
margins["U3-RAW historical raw g_y"] = F(hraw["g_y_N"]) / (Fr(1, 10 ** 12) * F(zero_floors[hr_]["force"]["exact"]))
margins["U3-RAW historical raw energy"] = F(hraw["energy_J"]) / (Fr(1, 10 ** 12) * F(zero_floors[hr_]["energy"]["exact"]))
for k, m_ in margins.items():
    check(f"S-1 discriminator keeps its power: {k}: margin {dec(m_, 3)} > 1e6", m_ > 10 ** 6)

# RV5's probe: a plain binary64 formation gives -1.665e-16 at U3-FRAME-COVARIANCE B[0][5] (exact 0)
fc = zero_floors["/cases/U3-FRAME-COVARIANCE/expected"]["B"]
check("S-1: RV5's binary64 B[0][5] defect (1.665e-16) is inside 1e-12 x the B floor of U3-FRAME-COVARIANCE",
      Fr("1.665e-16") <= Fr(1, 10 ** 12) * F(fc["exact"]))
gb = zero_floors["/cases/U3-GENERIC-SKEW-OFFSET-PRESTRESS/expected"]["B"]
check("S-1: RV5's other association (1.7e-16 at U3-GENERIC B[0][5]) is inside 1e-12 x the B floor", Fr("1.7e-16") <= Fr(1, 10 ** 12) * F(gb["exact"]))

ZERO_RULE = {
    "id": "R02-Z",
    "applies_to": "every FK unit-level array of every unit-level case (all cases except U3-SYS-*, U3-NI-FRICTION-FRAME, U3-W4-LINK-RULE's exact decision and U3-FINITE-ROTATION-NEGATIVE's own rule)",
    "rule": "each published entry passes when |obs - exp| <= 1e-12 * max(|exp|, F), where F is the floor of the entry's family in that evaluation (a case's expected object, a named sub-case or state). F = max(the family's largest |exp| in the evaluation, the family's operand scale, any floor round 01 stated for that family). The operand scale is the same exact expression evaluated with every operand replaced by its absolute value. B_op (B's operand form) = |Q^T|[I, |S(a_i)|+|S(r_op/2)|, I, |S(a_j)|+|S(r_op/2)|] for the translation rows, with r_op = |x_j - x_i| + |a_j - a_i| per component (RV130 S-4's r operand, never |r|), and |Q^T|[0, I, 0, I] for the rotation rows, so B_op >= |B| entrywise and is nonzero wherever an exact zero of B arises by cancellation. B -> max B_op; Ke -> B_op^T|K|B_op; with v = B_op|d|, w = v + |q_ref|, G = |K|w, P = B_op^T G: translation -> w[0:3], rotation -> w[3:6], force -> G[0:3], P[0:3], P[6:9], moment -> G[3:6], P[3:6], P[9:12], energy -> w.G/2; geometry (r, p_i, p_j) -> |x_j - x_i| + |a_j - a_i|, |x_i| + |a_i|, |x_j| + |a_j| per component. U3-B-ORACLE (no K): v = B_op|d_given|, P = B_op^T|g_given|, energy (virtual work) -> |g_given|.v. Families: B; Ke; geometry; translation (q_t, solved d_t); rotation (q_r, solved d_r); force (g_t, K q_ref translation part, F_global, Fi, Fj, force entries of B^T K q_ref and of reactions); moment (g_r, its K q_ref part, M_global, Mi, Mj, moment entries); energy (U, virtual work); scaled_coordinate (U3-COUPLED q-hat). Rank, PD flags and pivots are exact (no tolerance).",
    "exact_zero_families": "F = 0 only where the family's operand scale is exactly 0: every term of every operand path has an exact-zero factor (a zero input, or a structurally zero entry of B, K or q_ref). Every binary64 evaluation of JR's expressions then returns an exact zero (+-0) in any association, through B^T g or through Ke d - B^T K q_ref, so the comparison is exact and a correct product meets it; a nonzero value there is a real defect (a coupling that does not exist). These families are listed in exact_zero_family_list",
    "floors": "round_02.zero_floors, keyed by the JSON pointer of the evaluation; every floor is positive, so no exact zero is ever compared without a scale",
    "supersedes": "the tolerance wording of the round-00/01 FK unit criteria ('relative 1e-12', 'bitwise when B/Ke formation is exact', U3-KD5's 'against max|Ke|'); a bitwise match remains possible but is not required. U3-FINITE-ROTATION-NEGATIVE keeps |obs - exp| <= 1e-12 * L (L is its translation floor); U3-W4-LINK-RULE stays exact; system and NI criteria are unchanged",
    "why": "T4-RV5 S-1: a plain binary64 formation gives -1.665e-16 at U3-FRAME-COVARIANCE B[0][5] and 1.7e-16 at U3-GENERIC's, against exact zeros; the round-01 wording would fail a correct product there. No expected value changes",
    "moment_floor_common_rotation": "U3-J1-COMMON-ROTATION's moment floor (RV5) is its operand scale, in round_02.zero_floors['/cases/U3-J1-COMMON-ROTATION/expected'].moment",
    "discriminator_margins_min": dec(min(margins.values()), 3),
    "exact_zero_family_list": [f"{p_} : {f_}" for p_, f_ in exact_zero_families],
}

# =============================================================================================
# S-3: concrete patches and exact blocking-code sets for every refusal variant
# =============================================================================================
BASE001 = "/cases/U3-SYS-DEMO-CONNECTOR-001/inputs/document_v3_0.3.0"
base_doc = get(doc, BASE001)
mdl = base_doc["model"]
L100 = [c["id"] for c in mdl["load_cases"]].index("load:L-100")
C150 = [c["id"] for c in mdl["components"]].index("component:C-150")
P130 = [p["id"] for p in mdl["pipe_segments"]].index("pipe:P-130")
rv = cases["U3-SYS-DEMO-CONNECTOR-001"]["refusal_variants"]
ONLY = "the set of blocking diagnostic codes in the envelope is exactly {%s} (duplicates per case allowed); no other blocking code (in particular no SPRING_HANGER_*, UNIT_INPUT_INVALID, PROVENANCE_INPUT_MISSING, PRESSURE_REGION_*/PRESSURE_TERMINAL_*, LEGACY_FINITE_CONNECTOR_REAUTHOR_REQUIRED or OBJECTIVE_CONNECTOR_INPUT_INCOMPLETE unless it is the named code)"


def expected_only(code, refs, extra=None):
    e = {"status_mechanics": "MODEL_INCOMPLETE", "results": "none", "blocking_codes_exactly": [code],
         "rule": ONLY % code, "affected_refs_include": refs}
    if extra:
        e.update(extra)
    return e


for key, load_key in (("replaced_span_weight", "added_load"), ("replaced_span_thermal", "added_load")):
    v = rv[key]
    v["document_patch_round_02"] = {"base": BASE001 + " (round 02, hanger restored)",
                                    "operations": [{"op": "add", "path": f"/model/load_cases/{L100}/primitive_loads/-", "value": copy.deepcopy(v[load_key])}]}
    v["expected_round_02"] = expected_only("JOINT_REPLACED_SPAN_LOAD_UNOWNED", v["expected"]["affected_refs_include"],
                                           {"fallback": v["expected"]["fallback"],
                                            "s20": "0.3.0 has no load_sources: every primitive a case stores, that case applies; the load is in L-100's applied set, so L-100 refuses (Design S20 applied-set rule)"})

REGION = {"id": "region:L-100-R-100", "member_pipe_ids": ["pipe:P-100"], "pressure_basis": "internal_differential_zero_external_v1",
          "pressure": {"value": 1000000.0, "unit": "Pa"},
          "terminals": [{"node_ref": "node:N-100", "closure_transfer": "transfers_to_wall", "provenance": R02_PROV},
                        {"node_ref": "node:N-110", "closure_transfer": "transfers_to_wall", "provenance": R02_PROV}],
          "provenance": R02_PROV}
v = rv["joint_case_with_pressure_region"]
v["document_patch_round_02"] = {"base": BASE001 + " (round 02, hanger restored)",
                                "operations": [{"op": "replace", "path": f"/model/load_cases/{L100}/pressure_regions", "value": [REGION]}],
                                "why_this_region": "one well-formed region (v2 region schema at 10b70036ef, deny_unknown_fields: id, member_pipe_ids, pressure_basis, pressure, terminals, provenance) on pipe:P-100 only: a one-pipe acyclic chain whose two terminals are its end nodes with an explicit closure transfer, a finite 1 MPa in Pa, the material's E/nu basis present. It touches neither the connector's nodes nor the replaced span, so S20 is not engaged. If T4-U2a's v3 region schema differs, T4-U3 re-expresses this one region in it, on P-100 only"}
v["expected_round_02"] = expected_only("JOINT_PRESSURE_INTERFACE_UNRESOLVED", ["component:C-150"],
                                       {"until": "T4-U5 (J3/J4)", "not": "a pressure-region diagnostic: the region is well formed"})

ANNOT = {"id": "component:C-150", "label": "Invented expansion joint (annotation only)", "kind": "expansion_joint", "node": "node:N-140",
         "mechanics_interface": {"solver_consumption": "not_solver_consumed", "rule_check_consumption": "not_rule_checked"},
         "provenance": R02_PROV}
ANNOT_REF = copy.deepcopy(ANNOT)
ANNOT_REF["geometry"] = {"expansion_joint_pipe_ref": "pipe:P-130"}
v = rv["annotation_only_joint_on_exact_route"]
SEAM = "EXACT_PRESSURE_FAMILY_NOT_ADMITTED"
v["document_patches_round_02"] = {
    "a_annotation_minimal": {"base": BASE001 + " (round 02, hanger restored)",
                             "operations": [{"op": "replace", "path": f"/model/components/{C150}", "value": ANNOT}],
                             "meaning": "an explicit annotation-only joint (no objective_connector, no pipe reference, no rates); pipe:P-130 stays an ordinary pipe in the document"},
    "b_annotation_with_pipe_ref": {"base": BASE001 + " (round 02, hanger restored)",
                                   "operations": [{"op": "replace", "path": f"/model/components/{C150}", "value": ANNOT_REF}],
                                   "meaning": "round 0's form: the annotation also names expansion_joint_pipe_ref pipe:P-130"}}
v["expected_round_02"] = expected_only(SEAM, ["component:C-150"], {
    "applies_to": ["a_annotation_minimal", "b_annotation_with_pipe_ref"],
    "code_status": "EXACT_PRESSURE_FAMILY_NOT_ADMITTED is the seam's named refusal proposed in T4-I10 REVISION_01 (S-2; SLOT_TABLE section 4.2 row 'no connector, not_solver_consumed, v3'); revision 01 awaits RV130's confirmation. If confirmation renames it, both sub-variants take the confirmed seam code; nothing else changes",
    "message": "names the family (expansion_joint, annotation only) and says annotation mode applies on the pressure-free route only",
    "never": "analysed as pipe P-130 on the exact route; no result is published",
    "reading_for_b": "SLOT_TABLE 4.2's legacy row needs a pipe reference or rates together with a consumption other than not_solver_consumed (or none); an explicit not_solver_consumed annotation is the D-4 annotation population, not a legacy one. So b also takes the seam code and never LEGACY_FINITE_CONNECTOR_REAUTHOR_REQUIRED. Sub-variant a holds under any ordering of the classifier rows"})

TOPO_PAR = {"type": "parallel_with_elements", "element_refs": ["pipe:P-130"]}
TOPO_SER = {"type": "series_between_end_planes", "end_i_role": "through", "end_j_role": "terminal",
            "adjacent_i_pipe_refs": ["pipe:P-120"], "adjacent_j_pipe_refs": []}
v = rv["series_or_parallel_topology"]
v["document_patches_round_02"] = {
    "parallel_with_elements": {"base": BASE001 + " (round 02, hanger restored)",
                               "operations": [{"op": "replace", "path": f"/model/components/{C150}/objective_connector/topology", "value": TOPO_PAR}],
                               "meaning": "well formed under CONNECTOR_CONTRACT_V1's ConnectorTopologyV1: pipe:P-130 is the only element sharing the attachment pair (N-130, N-140), so the list is exhaustive"},
    "series_between_end_planes": {"base": BASE001 + " (round 02, hanger restored)",
                                  "operations": [{"op": "replace", "path": f"/model/components/{C150}/objective_connector/topology", "value": TOPO_SER},
                                                 {"op": "remove", "path": f"/model/pipe_segments/{P130}"}],
                                  "meaning": "well formed under CONNECTOR_CONTRACT_V1: end_i (N-130) is a through end with adjacent pipe:P-120 terminating there; end_j (N-140) is terminal with no adjacent pipe; pipe:P-130 is removed because a direct span may not bridge a series pair. No load, support or region names P-130 in this document"}}
v["expected_round_02"] = expected_only("OBJECTIVE_CONNECTOR_TOPOLOGY_UNRESOLVED", ["component:C-150"], {
    "applies_to": ["parallel_with_elements", "series_between_end_planes"],
    "rule_basis": "ruling: the connector topology is replaces_span only (SLOT_TABLE 4.2: 'any other topology')"})

# 0.4.0 replaced-span refusals (round 01): the same exact-set statement
er = cases["U3-SYS-LR1-REPLACED-SPAN-EIGENSTRAIN-REFUSAL"]
for key in ("a_thermal_on_replaced_span", "b_fit_strain_on_replaced_span", "c_weight_on_replaced_span"):
    x = er["expected"][key]
    x["expected_round_02"] = expected_only("JOINT_REPLACED_SPAN_LOAD_UNOWNED", x["expected"]["affected_refs_include"],
                                           {"refused_cases": x["expected"]["refused_cases"],
                                            "document": "this variant's document_v3_0.4.0, with the round-02 hanger"})

# =============================================================================================
# S-2, N-1, N-2, N-4: statements on the cases they govern
# =============================================================================================
EB = ("The values assume Euler-Bernoulli frame elements (FK/src/lib.rs EB local stiffness, the only frame formulation at the code basis). "
      "D-6 (shear default, owner-held before T4-U8) is not decided here. If D-6 keeps Euler-Bernoulli through an explicit selection in v3, "
      "this case's documents must carry that explicit Euler-Bernoulli selection and every value stands. If D-6 makes Timoshenko the default "
      "for new v3 documents, this case is re-frozen under that formulation (T4-RV5: phi = 12EI/(kGAL^2) is about 0.010-0.035 for these spans, far above 1e-9)")
for cid in ["U3-SYS-DEMO-CONNECTOR-001", "U3-SYS-DEMO-CONNECTOR-002-LR1", "U3-SYS-LR1-REPLACED-SPAN-EIGENSTRAIN-REFUSAL",
            "U3-SYS-REPLACED-SPAN-MATERIAL-CONTROL", "U3-SYS-LR1-REPLACED-SPAN-STORED-UNAPPLIED-CONTROL"]:
    cases[cid]["frame_formulation_round_02"] = EB
cases["U3-NI-FRICTION-FRAME"]["frame_formulation_round_02"] = (
    "The replacement coupling is FK's FrameElement with Euler-Bernoulli stiffness (12EI/L^3 = 48/5). A D-6 choice that only changes the "
    "default for new v3 documents leaves these values; a choice that changes FK FrameElement's own default re-freezes them")

raw_l100 = cases["U3-SYS-DEMO-CONNECTOR-001"]["wrong_result_discriminators"]
cases["U3-SYS-DEMO-CONNECTOR-001"]["plan_section5_substitute_round_02"] = {
    "plan_row": "plan section 5, T4-U3: '658.44 N*m now balancing'",
    "substitute": "the v3 analogue (T4-RV5 N-1): with the deleted element's raw-difference measure the global moment about the origin is unbalanced by "
                  + ", ".join(f"{cid_}: [{', '.join(raw_l100[cid_]['raw_difference_element_solve']['global_moment_imbalance_about_origin_N_m'])}] N*m"
                              for cid_ in ("load:L-100", "load:L-200", "load:L-300"))
                  + " (770 and 275 N*m about global X, 440 N*m about global Y); with the connector, applied loads plus reactions balance below the case's criterion (|sum M about the origin| <= 1e-9 * force floor * 8.27 m)",
    "why_not_658.44": "the legacy figure belongs to the pressure-stripped 0.1.0 demo with NL-140, NL-130-FRIC, CE-120 and the parallel element; the exact route refuses those supports and the connector is v3-only (D-4), so it is not reproducible",
    "record": "T4 WORKING_ITEMS records this substitution in the plan's T4-U3 row; no stop rule is involved"}
for cid_ in ("load:L-100", "load:L-200", "load:L-300"):
    check(f"N-1: {cid_} raw-difference imbalance is the one quoted",
          raw_l100[cid_]["raw_difference_element_solve"]["global_moment_imbalance_about_origin_N_m"] ==
          {"load:L-100": ["-770", "0", "0"], "load:L-200": ["-275", "0", "0"], "load:L-300": ["0", "440", "0"]}[cid_])

w4 = cases["U3-W4-LINK-RULE"]
gen_in = cases["U3-GENERIC-SKEW-OFFSET-PRESTRESS"]["inputs"]
Ls_g = F(gen_in["translation_scale_Ls_m"])
H_pd = from_upper21(gen_in["H_upper_triangle_21_N_m"])
H_psd = from_upper21(w4["inputs"]["psd_H_upper_triangle_21_Ls_2m"])
H_ind = from_upper21(w4["inputs"]["indefinite_H_upper_triangle_21_Ls_2m"])
sign = lambda xs: [(x > 0) - (x < 0) for x in xs]
pk_psd, z_psd = ldl_pivots(K_from_H(H_psd, Fr(2)))
ph_psd, zh_psd = ldl_pivots(H_psd)
pk_ind, z_ind = ldl_pivots(K_from_H(H_ind, Fr(2)))
ph_ind, zh_ind = ldl_pivots(H_ind)
ph_pd, zh_pd = ldl_pivots(H_pd)
check("N-2: PSD case, K pivots as published", [ex(x) for x in pk_psd] == w4["expected"]["psd_case"]["ldl_pivots_K"])
check("N-2: PSD case, H and K pivots have the same signs (inertia of H = D K D)", sign(pk_psd) == sign(ph_psd))
check("N-2: PSD case, every zero pivot has an all-zero remaining column (semidefinite, skipped legitimately)", not z_psd and not zh_psd)
check("N-2: indefinite case, H pivots as published", [ex(x) for x in ph_ind] == w4["expected"]["indefinite_case"]["ldl_pivots_H"])
check("N-2: indefinite case, H and K pivots have the same signs", sign(pk_ind) == sign(ph_ind) and min(ph_ind) < 0)
check("N-2: PD case (U3-GENERIC's authored H), every pivot of H > 0", all(x > 0 for x in ph_pd) and not zh_pd)
H_zero = from_upper21(["0", "1/2", "0", "0", "0", "0", "1", "0", "0", "0", "0", "1", "0", "0", "0", "1", "0", "0", "1", "0", "1"])
pz, zz = ldl_pivots(H_zero)
check("N-2: zero-pivot example: pivot 0 with a nonzero remaining column -> indefinite (2x2 minor -1/4 < 0)",
      zz and H_zero[0][0] * H_zero[1][1] - H_zero[0][1] ** 2 == Fr(-1, 4))
w4["decision_operand_round_02"] = {
    "operand": "the exact PD/PSD/indefinite decision is taken on the authored binary64 H (each upper-triangle entry the binary64 value read from the record), in exact rational arithmetic. Because H = D K D with D = diag(Ls, Ls, Ls, 1, 1, 1) positive, H and K have the same inertia (Sylvester), so the published K pivots (PD, PSD) and H pivots (indefinite) are equivalent evidence; checked here for all three cases",
    "binary64_K": "a K formed in binary64 as H/Ls^2 is rounded when Ls is not a power of two; every Ls in these cases (1/4, 1/2, 1, 2) is dyadic, so K is exact here either way; the decision still reads H",
    "zero_pivot": "in the symmetric elimination, a zero pivot whose remaining column (the entries below it in the current Schur complement) is not all zero means the matrix is indefinite (a 2x2 principal minor [[0, b], [b, c]] with b != 0 is negative): reject, never skip. A zero pivot with an all-zero remaining column is a semidefinite direction (U3-W4's PSD case)",
    "zero_pivot_example": {"H_upper_triangle_21_N_m": ["0", "1/2", "0", "0", "0", "0", "1", "0", "0", "0", "0", "1", "0", "0", "0", "1", "0", "0", "1", "0", "1"],
                           "translation_scale_Ls_m": "1", "pivots": [ex(x) for x in pz], "zero_pivot_nonzero_remaining_column": True,
                           "decision": "reject (OBJECTIVE_CONNECTOR_INPUT_INCOMPLETE: H not PSD); an elimination that skips the zero pivot would wrongly accept it as PSD"}}

cases["U3-B-ORACLE"]["product_test_round_02"] = (
    "U3-B-ORACLE supplies g (g_given) and d (d_given), not K: a product test forms B from the product's b() for these inputs, then f = B^T g_given "
    "and compares it with end_actions_node_on_element, q = B d_given against q_for_d_given, and g_given.q = f.d_given against virtual_work (all checked here). "
    "R02-Z floors: round_02.zero_floors['/cases/U3-B-ORACLE/expected']")

# =============================================================================================
# N-3: zero-length explicit Q works; Q.x not aligned with r is refused
# =============================================================================================
Qg = mat(gen_in["Q_row_major_columns_are_axes"])
Kg = mat(gen_in["K_physical"])
xi0, xj0 = [Fr(0)] * 3, [Fr(2, 5), Fr(0), Fr(0)]
ai0, aj0 = [Fr(1, 5), Fr(0), Fr(0)], [Fr(-1, 5), Fr(0), Fr(0)]
r0 = vsub(vadd(xj0, aj0), vadd(xi0, ai0))
check("N-3a: the attachments coincide (r = 0) on a 0.4 m span", r0 == [0, 0, 0])
qref0 = [Fr(1, 1000), Fr(-1, 2000), Fr(0), Fr(1, 500), Fr(0), Fr(-1, 1000)]
d0 = [Fr(k - 5, 1000) for k in range(12)]
B0 = B_kin(ai0, aj0, r0, Qg)
check("N-3a: rank B = 6", rank(B0) == 6)
Ke0 = mm(mm(tr(B0), Kg), B0)
for o in ([Fr(0)] * 3, [Fr(7), Fr(-3), Fr(5)]):
    for m_ in rigid_modes(xi0, xj0, o):
        check("N-3a: rigid motion -> B d = 0 and Ke d = 0", mv(B0, m_) == [0] * 6 and mv(Ke0, m_) == [0] * 12)
check("N-3a: rank Ke = 6 (PD K)", rank(Ke0) == 6)
q0 = mv(B0, d0)
g0 = mv(Kg, vsub(q0, qref0))
U0 = dot(vsub(q0, qref0), g0) / 2
f0 = mv(tr(B0), g0)
rhs0 = mv(tr(B0), mv(Kg, qref0))
Fv0, Mv0 = mv(Qg, g0[0:3]), mv(Qg, g0[3:6])
check("N-3a: end blocks with r = 0: Fi = -F, Mi = -a_i x F - M, Fj = F, Mj = a_j x F + M",
      f0 == vsc(-1, Fv0) + vsub(vsc(-1, cross(ai0, Fv0)), Mv0) + Fv0 + vadd(cross(aj0, Fv0), Mv0))
for oo in ([Fr(0)] * 3, [Fr(7), Fr(-3), Fr(5)]):
    fs = vadd(f0[0:3], f0[6:9])
    ms = vadd(vadd(vadd(cross(vsub(xi0, oo), f0[0:3]), f0[3:6]), cross(vsub(xj0, oo), f0[6:9])), f0[9:12])
    check("N-3a: end actions balance", fs == [0] * 3 and ms == [0] * 3)
check("N-3a: Ke d - B^T K q_ref = B^T g", vsub(mv(Ke0, d0), rhs0) == f0)
Qi = eye(3)
qI = mv(B_kin(ai0, aj0, r0, Qi), d0)
check("N-3a: identity substituted for the authored Q gives a different q (discriminator)", qI != q0)
Bop0 = B_operand(ai0, aj0, r_operand(xi0, xj0, ai0, aj0), Qg)
v0 = mv(Bop0, absv(d0))
w0 = vadd(v0, absv(qref0))
G0 = mv(absm(Kg), w0)
P0 = mv(tr(Bop0), G0)
fl0 = {"B": max(max(rw) for rw in Bop0), "Ke": max(max(rw) for rw in mm(mm(tr(Bop0), absm(Kg)), Bop0)),
       "geometry": max(max(abs(a) + abs(b) for a, b in zip(vsub(xj0, xi0), vsub(aj0, ai0))), max(abs(a) + abs(b) for a, b in zip(xi0, ai0)),
                       max(abs(a) + abs(b) for a, b in zip(xj0, aj0))),
       "translation": max(w0[0:3]), "rotation": max(w0[3:6]), "force": max(G0[0:3] + P0[0:3] + P0[6:9]),
       "moment": max(G0[3:6] + P0[3:6] + P0[9:12]), "energy": dot(w0, G0) / 2}
H0 = H_from_K(Kg, Ls_g)
zl_inputs = {"x_i": [ex(x) for x in xi0], "x_j": [ex(x) for x in xj0], "a_i_global": [ex(x) for x in ai0], "a_j_global": [ex(x) for x in aj0],
             "attachment_local": {"initial_node_axes_global_i": [[ex(x) for x in rw] for rw in eye(3)], "offset_local_i": [ex(x) for x in ai0],
                                  "initial_node_axes_global_j": [[ex(x) for x in rw] for rw in eye(3)], "offset_local_j": [ex(x) for x in aj0]},
             "Q_row_major_columns_are_axes": [[ex(x) for x in rw] for rw in Qg], "translation_scale_Ls_m": ex(Ls_g),
             "H_upper_triangle_21_N_m": [ex(x) for x in upper21(H0)], "K_physical": [[ex(x) for x in rw] for rw in Kg], "q_ref": [ex(x) for x in qref0],
             "d": [ex(x) for x in d0], "topology": "replaces_span of the 0.4 m span x_i -> x_j (the offsets bring the attachments together)"}
zl_expected = {"r": [ex(x) for x in r0], "p_i": [ex(x) for x in vadd(xi0, ai0)], "p_j": [ex(x) for x in vadd(xj0, aj0)],
               "B": [[ex(x) for x in rw] for rw in B0], "Ke": [[ex(x) for x in rw] for rw in Ke0], "rank_B": 6, "rank_Ke": rank(Ke0),
               "installed_rhs_BT_K_qref": [ex(x) for x in rhs0], "q": bothv(q0), "g": bothv(g0), "energy": both(U0),
               "F_global": bothv(Fv0), "M_global": bothv(Mv0),
               "end_actions_node_on_element": {"Fi": bothv(f0[0:3]), "Mi": bothv(f0[3:6]), "Fj": bothv(f0[6:9]), "Mj": bothv(f0[9:12])},
               "admission": "admitted: r = 0, so the Q.x alignment rule does not apply; the authored Q is used exactly (no length-derived axis, no identity substituted)"}
cases["U3-ZERO-LENGTH-EXPLICIT-Q"] = {
    "case_id": "U3-ZERO-LENGTH-EXPLICIT-Q", "brief_item": "N-3 (round 02)",
    "purpose": "JR section 6 control 'zero-length explicit Q works' (T4-RV5 N-3): under replaces_span, r = 0 arises only through offsets. Coincident attachments with U3-GENERIC's skew Q and coupled PD K, nonzero q_ref, a generic d",
    "inputs": zl_inputs, "expected": zl_expected,
    "criterion": "R02-Z (round_02.zero_floors['/cases/U3-ZERO-LENGTH-EXPLICIT-Q/expected']); rank exact",
    "wrong_result_discriminators": [{"what": "identity substituted for the authored Q (or an axis derived from the zero-length r)", "q": [ex(x) for x in qI]}],
    "companion_refusal": {"what": "the same record with connector_axes_global omitted", "expected": {"status_mechanics": "MODEL_INCOMPLETE", "blocking_codes_exactly": ["OBJECTIVE_CONNECTOR_INPUT_INCOMPLETE"],
                                                                                              "rule": "explicit Q is required for coincident attachments; no identity frame is inserted"}},
}
zero_floors["/cases/U3-ZERO-LENGTH-EXPLICIT-Q/expected"] = {k: {"exact": ex(x), "decimal": dec(x)} for k, x in fl0.items()}
check("N-3a: every floor positive", all(x > 0 for x in fl0.values()))

# N-3b: Q.x not aligned with r
xi1, xj1 = [Fr(0)] * 3, [Fr(3, 10), Fr(0), Fr(0)]
Q_perp = [[Fr(0), Fr(-1), Fr(0)], [Fr(1), Fr(0), Fr(0)], [Fr(0), Fr(0), Fr(1)]]   # columns e_y, -e_x, e_z
Q_anti = [[Fr(-1), Fr(0), Fr(0)], [Fr(0), Fr(-1), Fr(0)], [Fr(0), Fr(0), Fr(1)]]  # columns -e_x, -e_y, e_z
r1 = vsub(xj1, xi1)
for nm, Qx in (("perpendicular", Q_perp), ("antiparallel", Q_anti)):
    det = (Qx[0][0] * (Qx[1][1] * Qx[2][2] - Qx[1][2] * Qx[2][1]) - Qx[0][1] * (Qx[1][0] * Qx[2][2] - Qx[1][2] * Qx[2][0])
           + Qx[0][2] * (Qx[1][0] * Qx[2][1] - Qx[1][1] * Qx[2][0]))
    check(f"N-3b {nm}: Q is proper orthonormal (so only the alignment rule can refuse it)", mm(tr(Qx), Qx) == eye(3) and det == 1)
    qx = [Qx[0][0], Qx[1][0], Qx[2][0]]
    check(f"N-3b {nm}: Q.x is not r/|r|", not (cross(qx, r1) == [0, 0, 0] and dot(qx, r1) > 0))
cases["U3-QX-MISALIGNED-REFUSAL"] = {
    "case_id": "U3-QX-MISALIGNED-REFUSAL", "brief_item": "N-3 (round 02)",
    "purpose": "CONNECTOR_CONTRACT_V1 section 2 control (T4-RV5 N-3): for r != 0, Q.x must follow r/|r| (JR section 2: 'for nonzero r its x-axis follows r'); a proper Q whose x column does not is refused, never re-orthogonalized, re-aligned or flipped",
    "inputs": {"x_i": [ex(x) for x in xi1], "x_j": [ex(x) for x in xj1], "a_i_global": ["0", "0", "0"], "a_j_global": ["0", "0", "0"], "r": [ex(x) for x in r1],
               "K_physical": cases["U3-J1-LATERAL"]["inputs"]["K_physical"], "translation_scale_Ls_m": "1",
               "H_upper_triangle_21_N_m": cases["U3-J1-LATERAL"]["inputs"]["H_upper_triangle_21_N_m"], "q_ref": ["0"] * 6,
               "variants": {"perpendicular": {"Q_row_major_columns_are_axes": [[ex(x) for x in rw] for rw in Q_perp], "Q_x": ["0", "1", "0"]},
                            "antiparallel": {"Q_row_major_columns_are_axes": [[ex(x) for x in rw] for rw in Q_anti], "Q_x": ["-1", "0", "0"]}},
               "control": "Q = I with the same geometry is U3-J1-LATERAL (admitted)"},
    "expected": {"fk": "the FK connector constructor refuses both variants (its error variant is T4-U3's choice); no B, Ke or action is formed",
                 "through_pp": {"status_mechanics": "MODEL_INCOMPLETE", "results": "none", "blocking_codes_exactly": ["OBJECTIVE_CONNECTOR_INPUT_INCOMPLETE"],
                                "affected_refs_include": ["the connector's component id"], "rule": "SLOT_TABLE 4.2: a required field invalid, or an FK constructor refusal, gives OBJECTIVE_CONNECTOR_INPUT_INCOMPLETE"},
                 "sign": "the antiparallel variant is refused too: alignment means Q.x = +r/|r| (U3-REVERSAL keeps this by Q' = QJ when r changes sign)",
                 "guard": "both misalignments are O(1); the representation guard (CONNECTOR_CONTRACT_V1: machine-precision scaled) is not exercised here"},
    "criterion": "structural: refusal with the named code; no tolerance",
}

# =============================================================================================
# S20 applied-set reconciliation, gate list summary, round_02 block
# =============================================================================================
S20 = [
    {"case": "U3-SYS-DEMO-CONNECTOR-001", "form": "0.3.0", "replaced_span_content": "none (P-130 carries no load)", "outcome": "admitted", "consistent": True},
    {"case": "U3-SYS-DEMO-CONNECTOR-001 / replaced_span_weight", "form": "0.3.0", "replaced_span_content": "weight primitive on P-130 stored in L-100 (0.3.0: stored = applied by that case)", "outcome": "refused (JOINT_REPLACED_SPAN_LOAD_UNOWNED)", "consistent": True},
    {"case": "U3-SYS-DEMO-CONNECTOR-001 / replaced_span_thermal", "form": "0.3.0", "replaced_span_content": "thermal primitive (12.5 degC) on P-130 in L-100: a load in the applied set, and a nonzero eigenstrain", "outcome": "refused (JOINT_REPLACED_SPAN_LOAD_UNOWNED)", "consistent": True},
    {"case": "U3-SYS-DEMO-CONNECTOR-001 / pressure, annotation, series/parallel", "form": "0.3.0", "replaced_span_content": "none (region on P-100; no connector or no replaces_span in the annotation and topology variants)", "outcome": "refused for their own reasons; S20 not engaged", "consistent": True},
    {"case": "U3-SYS-DEMO-CONNECTOR-002-LR1", "form": "0.4.0", "replaced_span_content": "unchanged_reference, fit none: resolved eigenstrain exactly 0; no load source on P-130", "outcome": "admitted", "consistent": True},
    {"case": "U3-SYS-LR1-REPLACED-SPAN-EIGENSTRAIN-REFUSAL a", "form": "0.4.0", "replaced_span_content": "constant_alpha_interval in L-100: nonzero resolved eigenstrain (3/20000) in L-100 only", "outcome": "L-100 refuses; envelope MODEL_INCOMPLETE", "consistent": True},
    {"case": "U3-SYS-LR1-REPLACED-SPAN-EIGENSTRAIN-REFUSAL b", "form": "0.4.0", "replaced_span_content": "fit_strain 1e-4: nonzero in every case", "outcome": "every case refuses", "consistent": True},
    {"case": "U3-SYS-LR1-REPLACED-SPAN-EIGENSTRAIN-REFUSAL c", "form": "0.4.0", "replaced_span_content": "weight primitive on P-130 listed in L-100's load_sources (in the applied set)", "outcome": "L-100 refuses", "consistent": True},
    {"case": "U3-SYS-LR1-REPLACED-SPAN-EIGENSTRAIN-REFUSAL boundary_zero_strain", "form": "0.4.0", "replaced_span_content": "explicit_interval_strain 0: an explicit zero eigenstrain", "outcome": "admitted; values equal 002-LR1", "consistent": True},
    {"case": "U3-SYS-REPLACED-SPAN-MATERIAL-CONTROL", "form": "0.3.0 and 0.4.0", "replaced_span_content": "a different E/nu selected for P-130 (not a load, not an eigenstrain)", "outcome": "admitted; values equal 001", "consistent": True},
    {"case": "U3-SYS-LR1-REPLACED-SPAN-STORED-UNAPPLIED-CONTROL", "form": "0.4.0", "replaced_span_content": "weight primitive on P-130 stored in L-100 but listed in no case's load_sources (not in any applied set)", "outcome": "admitted; values equal 002-LR1", "consistent": True},
]

GATES_SUMMARY = (
    "Read at origin/main 10b70036ef (run_linear_static_preview_observed, PP/src/lib.rs:2348-2443 and the gates it calls). With the hanger restored, "
    "no generic input gate refuses any of the nine documents; the only refusals today are the v3 seam's and the connector's own gates, which "
    "T4-U2a and T4-U3 replace by design. Gate-by-gate list: ROUND_02.md section 1; mechanical evidence: _run_records/gate_model.py")

doc["round_02"] = {
    "requested_by": "T4 WORKING_ITEMS: T4-I12 repair round 02 for T4-RV5's REVIEW.md (B-1, S-1, S-2, S-3, N-1 to N-4) and the Design's S20 applied-set rule",
    "status": "frozen; awaiting T4-RV5's confirmation addendum; not product evidence",
    "basis": {"frozen_round_01_file_sha256": FROZEN_SHA256, "round_01_SHA256SUMS": "ee4b411fe23767d2cc7096bb6cd7b505ff1c231b8007a5dbc48dd19dfa9138c3",
              "code": "origin/main 10b70036ef (read only)", "review": "R4/T4-RV5/REVIEW.md",
              "design": "P/execution/PKG-03/DEL-03-06/Design/joint-replacement.md (D-4, replaces_span, S20 applied-set rule)"},
    "precedence": "round 02 only adds keys. A key ending in _round_02 supersedes the same-stem round-01 key in the same object (expected_round_02 over expected, document_patch(es)_round_02 over document_patch); R02-Z supersedes the FK unit criteria's tolerance wording as stated in zero_floor_rule.supersedes. Every round-00/01 value stands",
    "b1_hanger_restored": {"fields": RESTORED, "source": "the demo fixture's SH-140 hanger, verbatim (P/fixtures/product_preview/invented_preview_model.json, sha256 986c0559...871c at 10b70036ef and ed012c7ccf)",
                           "documents": hanger_fix,
                           "values_neutral": "the spring is built from hanger.stiffness alone (PP/src/lib.rs:7396-7410 support_stiffness_input); installed/cold/hot loads and travel are review metadata (validation.rs:904-927, review rows lib.rs:12104-12120). No frozen value changes (check_round02.py, leaf by leaf; and the round-01 document solver re-run on the round-02 documents)",
                           "gates": GATES_SUMMARY},
    "zero_floor_rule": ZERO_RULE,
    "zero_floors": zero_floors,
    "s20_reconciliation": {"rule": "Design S20: a nonzero resolved eigenstrain or self-weight on the replaced span, or a load in a case's applied set targeting it, refuses with JOINT_REPLACED_SPAN_LOAD_UNOWNED; an explicit zero eigenstrain and a stored load not applied by any case stay admitted. Presence means presence in the case's applied set (0.4.0: its load_sources; 0.3.0: its primitive_loads, all of which it applies)",
                           "cases": S20,
                           "not_frozen": "a zero-magnitude primitive load on the replaced span in an applied set: by the presence reading it refuses (a load, not an eigenstrain state); no case freezes it"},
    "conditions_for_admission": [
        "C-1 (T4-U2a): pressure_runtime::is_exact must hold for 3.0.0/exact_pressure_v3 (or v3 must otherwise skip the legacy shear-modulus requirement, validation.rs:341-349). The documents carry E/nu without shear_modulus, as v2 documents do; if v3 were treated as non-exact, MATERIAL_INPUT_MISSING would refuse them",
        "C-2 (T4-U2a/T4-U3): today's validate_profile refuses the 3.0.0 contract (PRESSURE_CONTRACT_UNSUPPORTED), pressure_regions outside the exact profile (PREVIEW_CONTRACT_VERSION_MISMATCH) and any objective_connector (OBJECTIVE_CONNECTOR_NOT_IMPLEMENTED); these are the gates T4-U2a's seam and T4-U3's classifier replace, not input defects",
        "C-3 (T4-U3): SLOT_TABLE 4.2's classifier emits only the codes listed in each refusal variant's expected_round_02; admission of every document is still read from code, not run (no product run in this round)"],
    "new_cases": ["U3-ZERO-LENGTH-EXPLICIT-Q", "U3-QX-MISALIGNED-REFUSAL"],
    "addressed": {"B-1": "b1_hanger_restored", "S-1": "zero_floor_rule, zero_floors", "S-2": "frame_formulation_round_02 on the five system cases and U3-NI-FRICTION-FRAME",
                  "S-3": "document_patch(es)_round_02 and expected_round_02 on every refusal variant of U3-SYS-DEMO-CONNECTOR-001 and U3-SYS-LR1-REPLACED-SPAN-EIGENSTRAIN-REFUSAL",
                  "N-1": "U3-SYS-DEMO-CONNECTOR-001.plan_section5_substitute_round_02", "N-2": "U3-W4-LINK-RULE.decision_operand_round_02",
                  "N-3": "new_cases", "N-4": "U3-B-ORACLE.product_test_round_02", "S20": "s20_reconciliation"},
}

# ------------------------------------------------------------------ write
with open(out_path, "w") as fh:
    json.dump(doc, fh, indent=1, sort_keys=False)
    fh.write("\n")

print("floors (pointer | family | max|exp| | operand scale | floor):")
for ptr, fam, mx, s, fl in floor_details:
    print(f"  {ptr} | {fam} | {dec(mx, 6)} | {'-' if s is None else dec(s, 6)} | {dec(fl, 6)}")
print("exact-zero families (operand scale exactly 0):")
for p_, f_ in exact_zero_families:
    print(f"  {p_} | {f_}")
print("discriminator margins (|wrong - exp| / tolerance), minimum", dec(min(margins.values()), 3))
for k, m_ in margins.items():
    print(f"  {k}: {dec(m_, 3)}")
npass = sum(ok for _, ok in CHECKS)
print(f"{npass} of {len(CHECKS)} checks pass")
for name, ok in CHECKS:
    print(("PASS " if ok else "FAIL ") + name)
sys.exit(0 if npass == len(CHECKS) else 1)
