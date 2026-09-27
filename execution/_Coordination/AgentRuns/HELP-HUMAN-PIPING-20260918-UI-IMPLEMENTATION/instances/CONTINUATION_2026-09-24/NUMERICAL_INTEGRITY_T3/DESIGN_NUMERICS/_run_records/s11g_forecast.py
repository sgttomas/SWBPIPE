#!/usr/bin/env python3
"""D1 S11-G forecast (standard library only; no product code built or run).

Usage (from T3/):
  PYTHONDONTWRITEBYTECODE=1 nice -n 19 python3 DESIGN_NUMERICS/_run_records/s11g_forecast.py <P> <out.json>

<P> is the project root `projects/chirality-piping` (run records used `../../../../../../..` from T3/).
Every committed JSON under <P> except execution/, schemas/, node_modules/ and target/ is scanned for
solving models (a dict with `load_cases` and `pipe_segments`, at most four levels deep). Inputs read
from T3/: REFERENCES/references.json (R1 frozen), DETECTION/results.json (P1's observations at main
c61a540ea) and GATE/FORMATION_EXCEPTIONS.json.

Part A, the load-row guard on R1's formed-load cases (the only R1 cases with element loads are
RF-CANCEL-UDL-W1e5, -W1e8, -W1e80). SP's per-load fixed-end terms are formed in binary64 exactly as
`add_spanned_uniform_equivalent_load` writes them; each term's intended value is the exact (Fraction)
value of the same formula on the same held operands (q, L, a, b). The statistic at a row is
|sum of exact formation defects| against 1e-9 * max(|intended net|, S*), with S* the coupled
free-row intended-net scale of the body (restrained rows are also reported, against the same smaller
free-row S*, which is conservative). The rigorous a-priori alternative (c = 16 times the sum of the
monomial magnitudes) is reported for comparison. Every other R1 case has nodal (input) loads only, so
its defects are identically zero.

Part B, the load-row guard on the committed models: a topological screen (free nodes where formed
terms meet each other or an input) and a magnitude screen. The magnitude screen evaluates every loaded
row (free and restrained) with the conservative uniform bound 17 * gamma_16 * sum |formed term| in
place of the exact defect (it bounds every family's defect, SP's worst amplification being 17), against
1e-9 * max(|net|, S*_f). A margin above 1 means the guard cannot fire. Laws not emulated use upper
bounds (free-length and constant-alpha strains 1e-2); weight and distributed loads are full-span.

Part C, recovery-side guard candidates on P1's passing case-modes (captured entry): per member-end
bending row, B = gamma_16 * sum_k |Kloc_rk| * sum_j |T_kj u_j| from P1's observed displacements, the
emulated binary64 K_e.u, and its exact recomputation from the same binary64 operands.
  R-a: B > 1e-9 * max(|q|, S*_moment(body)), S*_moment = max(S(moment), L_b * S(force)).
  R-b: B > 1e-9 * |q| and |q| > 2^10 * B (row-relative bound, resolved above noise).
  R-c: |q - q_exact| > 1e-9 * max(|q|, 2^-34 * S*_moment) (row-relative actual formation error).
The four INPLANE formation rows are evaluated in the S11-F state (the exact net at N2), emulated with
binary64 dense Gaussian elimination, including the exactly rounded recovery from the binary64 u.
"""
import json
import math
import os
import sys
from fractions import Fraction as Fr

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import withheld_rows as W  # noqa: E402  (support-family rule, D1's own)
from sweep_d5_r1 import num  # noqa: E402

U = 2.0 ** -53


def gamma(k):
    return k * U / (1 - k * U)


# ---------------------------------------------------------------- Part A
def sp_uniform_terms(q, L, a, b):
    """SP add_spanned_uniform_equivalent_load, local Y direction: (transverse_i, rotation_i,
    transverse_j, rotation_j) in binary64, as the source writes them."""
    ti = q * L * ((b - b ** 3 + 0.5 * b ** 4) - (a - a ** 3 + 0.5 * a ** 4))
    ri = q * L * L * ((0.5 * b * b - (2.0 / 3.0) * b ** 3 + 0.25 * b ** 4) - (0.5 * a * a - (2.0 / 3.0) * a ** 3 + 0.25 * a ** 4))
    tj = q * L * ((b ** 3 - 0.5 * b ** 4) - (a ** 3 - 0.5 * a ** 4))
    rj = q * L * L * (((-b ** 3 / 3.0) + 0.25 * b ** 4) - ((-a ** 3 / 3.0) + 0.25 * a ** 4))
    return ti, ri, tj, rj


def sp_uniform_intended(q, L, a, b):
    q, L, a, b = Fr(q), Fr(L), Fr(a), Fr(b)
    P = lambda x: Fr(1, 2) * x ** 2 - Fr(2, 3) * x ** 3 + Fr(1, 4) * x ** 4
    Q = lambda x: -x ** 3 / 3 + Fr(1, 4) * x ** 4
    ti = q * L * ((b - b ** 3 + b ** 4 / 2) - (a - a ** 3 + a ** 4 / 2))
    tj = q * L * ((b ** 3 - b ** 4 / 2) - (a ** 3 - a ** 4 / 2))
    return ti, q * L * L * (P(b) - P(a)), tj, q * L * L * (Q(b) - Q(a))


def part_a(ref):
    out = {}
    for cid in ("RF-CANCEL-UDL-W1e5", "RF-CANCEL-UDL-W1e8", "RF-CANCEL-UDL-W1e80"):
        m = ref[cid]["model"]
        wA = float(num(m["member_uniform_loads_N_per_m_global"]["A"][1]))
        wB = float(num(m["member_uniform_loads_N_per_m_global"]["B"][1]))
        L = 2.0
        # members along +x, load along global +y: local y = global y exactly (axis-aligned)
        tA = sp_uniform_terms(wA, L, 0.0, 1.0)
        tB = sp_uniform_terms(wB, L, 0.0, 1.0)
        iA = sp_uniform_intended(wA, L, 0.0, 1.0)
        iB = sp_uniform_intended(wB, L, 0.0, 1.0)
        nodal = Fr(float(num(m["loads"]["S1"]["M"][2])))
        # free DOFs of the body: S1 RX, RY, RZ (S0, S2 fixed; S1 translations pinned)
        rep_terms = [("A.rotation_j", tA[3], iA[3]), ("B.rotation_i", tB[1], iB[1])]
        net_rep = nodal + sum(Fr(t) for _, t, _ in rep_terms)
        defect = sum(Fr(t) - i for _, t, i in rep_terms)
        net_int = net_rep - defect
        mo = float(abs(net_int))  # S*_f(moment): the only loaded free row; no free force rows
        stat = float(abs(defect))
        thr = 1e-9 * max(float(abs(net_int)), mo)
        # the rigorous a-priori bound alternative (sum |monomial| amplification), c = 16
        abs_j = abs(wA) * L * L * (1 / 3 + 1 / 4)
        abs_i = abs(wB) * L * L * (0.5 + 2 / 3 + 0.25)
        bound = 16 * U * (abs_j + abs_i)
        # every loaded row, restrained rows included (reaction rows), against the same free-row S*_f
        rows_all = {"S0.UY": [(tA[0], iA[0])], "S0.RZ": [(tA[1], iA[1])], "S1.UY": [(tA[2], iA[2]), (tB[0], iB[0])],
                    "S2.UY": [(tB[2], iB[2])], "S2.RZ": [(tB[3], iB[3])]}
        all_rows = {}
        for rk, tl in rows_all.items():
            dfc = sum(Fr(t) - i for t, i in tl)
            ni = sum(i for _, i in tl)
            th = 1e-9 * max(float(abs(ni)), mo)
            all_rows[rk] = {"restrained": True, "exact_defect": float(dfc), "net_intended": float(ni), "threshold": th,
                            "fires": float(abs(dfc)) > th}
        out[cid] = {"terms": {k: [t, float(i)] for k, t, i in rep_terms}, "nodal_input": float(nodal),
                    "restrained_rows": all_rows,
                    "net_represented": float(net_rep), "net_intended": float(net_int),
                    "exact_defect": float(defect), "threshold": thr, "fires_exact_defect": stat > thr,
                    "apriori_bound_c16": bound, "fires_apriori_bound": bound > thr,
                    "free_row": "S1.RZ (S1 RX, RY free and unloaded)"}
    return out


def part_a2():
    """S11-F's committed probe-A test (s11f_tests.rs, `probe_a`): one 2 m cantilever, root anchored,
    three uniform global-y loads on the same member, (G, 0.3, -G) and (0.3, G, -G), G = 1e8 and 1e80.
    A formed-term cancellation that is legitimate: q and -q give exactly negated terms and defects.
    The test asserts CHECKS_PASSED, so S11-G must stay silent here."""
    out = {}
    for g in (1e8, 1e80):
        for name, order in (("GnG", (1.0, 0.0, -1.0)), ("nGG", (0.0, 1.0, -1.0))):
            qs = [0.3 if f == 0.0 else f * g for f in order]
            terms = [sp_uniform_terms(q, 2.0, 0.0, 1.0) for q in qs]
            ints = [sp_uniform_intended(q, 2.0, 0.0, 1.0) for q in qs]
            rows = {}
            for key, slot in (("tip.UY", 2), ("tip.RZ", 3), ("root.UY", 0), ("root.RZ", 1)):
                dfc = sum(Fr(t[slot]) - i[slot] for t, i in zip(terms, ints))
                ni = sum(i[slot] for i in ints)
                sab = sum(abs(t[slot]) for t in terms)
                rows[key] = {"exact_defect": float(dfc), "net_intended": float(ni), "sum_abs_terms": sab}
            F, M = abs(rows["tip.UY"]["net_intended"]), abs(rows["tip.RZ"]["net_intended"])
            fo, mo = max(F, M / 2.0), max(M, 2.0 * F)
            allF = max(F, abs(rows["root.UY"]["net_intended"]))
            allM = max(M, abs(rows["root.RZ"]["net_intended"]))
            for key, r in rows.items():
                free = key.startswith("tip")
                sc = (fo if key.endswith("UY") else mo) if free else (max(allF, allM / 2) if key.endswith("UY") else max(allM, 2 * allF))
                r["threshold"] = 1e-9 * max(abs(r["net_intended"]), sc)
                r["fires_exact_defect"] = abs(r["exact_defect"]) > r["threshold"]
                r["fires_apriori_c16"] = 16 * U * r["sum_abs_terms"] * 17 > r["threshold"]
            out[f"G={g:g} {name}"] = rows
    return out


# ---------------------------------------------------------------- Part B
def part_b(root):
    rows = []
    SKIP = {"execution", "node_modules", "target", ".git", "schemas"}

    def models(o, depth=0):
        if depth > 4:
            return
        if isinstance(o, dict):
            if "load_cases" in o and "pipe_segments" in o and isinstance(o["load_cases"], list):
                yield o
                return
            for v in o.values():
                yield from models(v, depth + 1)
        elif isinstance(o, list):
            for v in o:
                yield from models(v, depth + 1)
    for dp, dn, fn in os.walk(root):
        dn[:] = sorted(x for x in dn if x not in SKIP)
        for f in sorted(fn):
            if not f.endswith(".json"):
                continue
            p = os.path.join(dp, f)
            if os.path.getsize(p) > 30 * 2 ** 20:
                continue
            try:
                d = json.load(open(p))
            except Exception:
                continue
            for m in models(d):
                rows.extend(part_b_model(root, p, d, m))
    return rows


def part_b_model(root, p, d, m):
    rows = []
    if True:
        if True:
            nodes = {n["id"] for n in m.get("nodes", [])}
            restrained = {}
            for s in m.get("supports", []):
                if W.is_rigid_support(s):
                    restrained.setdefault(s["node"], set()).update(s.get("restraints", []))
            pipes = {pp["id"]: pp for pp in m["pipe_segments"]}
            for lc in m["load_cases"]:
                formed = {}  # node -> count of formed terms (per translational/rotational family, conservative)
                inputs = {}
                for pl in lc.get("primitive_loads", []) or []:
                    tgt = pl.get("target") or {}
                    cat = pl.get("category")
                    if tgt.get("type") == "node":
                        inputs[tgt.get("node")] = inputs.get(tgt.get("node"), 0) + 1
                    elif tgt.get("type") in ("element", "pipe") and tgt.get("pipe") in pipes:
                        pp = pipes[tgt["pipe"]]
                        for nd in (pp["from"], pp["to"]):
                            formed[nd] = formed.get(nd, 0) + 1
                for es in (lc.get("analysis_state") or {}).get("element_states", []):
                    ts = es.get("thermal_state") or {}
                    if ts and ts.get("kind") != "unchanged_reference" and es.get("pipe_ref") in pipes:
                        pp = pipes[es["pipe_ref"]]
                        for nd in (pp["from"], pp["to"]):
                            formed[nd] = formed.get(nd, 0) + 1
                for reg in lc.get("pressure_regions") or []:
                    for mid in reg.get("member_pipe_ids", []):
                        if mid in pipes:
                            for nd in (pipes[mid]["from"], pipes[mid]["to"]):
                                formed[nd] = formed.get(nd, 0) + 1
                hazards = []
                for nd in sorted(nodes):
                    free = len(restrained.get(nd, set())) < 6
                    if not free:
                        continue
                    if formed.get(nd, 0) >= 2 or (formed.get(nd, 0) >= 1 and inputs.get(nd, 0) >= 1):
                        hazards.append({"node": nd, "formed_terms_meeting": formed.get(nd, 0), "inputs": inputs.get(nd, 0),
                                        "free_components": sorted({"UX", "UY", "UZ", "RX", "RY", "RZ"} - restrained.get(nd, set()))})
                if formed:
                    rows.append({"file": os.path.relpath(p, root), "case": lc["id"], "formed_nodes": len(formed),
                                 "cancellation_candidates": hazards,
                                 "magnitude_screen": safe_screen(m, d, lc, restrained)})
    return rows


def safe_screen(m, d, lc, restrained):
    try:
        return magnitude_screen(m, d, lc, restrained)
    except (KeyError, TypeError, ValueError) as e:
        return {"screen_error": repr(e)[:200], "min_margin": None}


UNITS = {"m": 1.0, "mm": 1e-3, "Pa": 1.0, "kPa": 1e3, "MPa": 1e6, "GPa": 1e9, "N": 1.0, "kN": 1e3, "N*m": 1.0,
         "N/m": 1.0, "degC": 1.0, "1": 1.0, "1/degC": 1.0}


def qv(q):
    if q.get("unit", "") not in UNITS:
        raise ValueError("unit " + str(q.get("unit")))
    return float(q["value"]) * UNITS[q["unit"]]


def magnitude_screen(m, doc, lc, restrained):
    """Sufficient no-fire screen: per free DOF, the formed terms' magnitudes |t| (upper-bounded where a
    law is not emulated) and the signed net. Since |exact defect| <= 17 * gamma_16 * sum |t| (SP's worst
    amplification 17; product chains 1), the guard cannot fire where that bound is below
    1e-9 * max(|net|, S*_f). Returns the minimum margin over free DOFs (> 1 means no fire)."""
    nodes = {n["id"]: [float(n["position"][k] if isinstance(n["position"], dict) else n["position"]["xyz".index(k)])
                       for k in "xyz"] for n in m["nodes"]}
    mats = {x["id"]: x for x in (m.get("materials") or doc.get("materials") or [])}
    pipes = {pp["id"]: pp for pp in m["pipe_segments"]}
    rows = {}  # (node, dof) -> [net, sum_abs]

    def add(nd, dof, v, formed):
        r = rows.setdefault((nd, dof), [0.0, 0.0])
        r[0] += v
        if formed:
            r[1] += abs(v)

    def geom(pid):
        pp = pipes[pid]
        a, b = nodes[pp["from"]], nodes[pp["to"]]
        dvec = [b[k] - a[k] for k in range(3)]
        L = math.sqrt(sum(x * x for x in dvec))
        od = qv(pp["section"]["outside_diameter"])
        t = qv(pp["section"]["wall_thickness"])
        A = math.pi * (od * od - (od - 2 * t) ** 2) / 4
        Ai = math.pi * (od - 2 * t) ** 2 / 4
        mat = mats.get(pp.get("material"), {})
        E = qv(mat["elastic_modulus"]) if "elastic_modulus" in mat else 2.1e11
        alpha = qv(mat["thermal_expansion_coefficient"]) if "thermal_expansion_coefficient" in mat else 2e-5
        return pp, [x / L for x in dvec], L, A, Ai, E, alpha

    def axial_pair(pid, P):
        pp, x, L, *_ = geom(pid)
        for k in range(3):
            add(pp["from"], k, -P * x[k], True)
            add(pp["to"], k, P * x[k], True)
    for pl in lc.get("primitive_loads", []) or []:
        tgt = pl.get("target") or {}
        cat = pl.get("category")
        axis = {"global_x": 0, "global_y": 1, "global_z": 2, "UX": 0, "UY": 1, "UZ": 2, "RX": 3, "RY": 4, "RZ": 5,
                "rotation_x": 3, "rotation_y": 4, "rotation_z": 5}.get(pl.get("direction"), 0)
        if tgt.get("type") == "node":
            add(tgt["node"], axis, qv(pl["magnitude"]), False)
        elif tgt.get("pipe") in pipes:
            pp, x, L, A, Ai, E, alpha = geom(tgt["pipe"])
            if cat == "thermal":
                axial_pair(tgt["pipe"], E * A * alpha * qv(pl["magnitude"]))
            elif cat == "pressure":
                axial_pair(tgt["pipe"], qv(pl["magnitude"]) * Ai)
            elif cat in ("weight", "distributed_force", "uniform"):
                w = qv(pl["magnitude"])
                dv = [0.0, 0.0, 0.0]
                dv[axis] = w
                mom = [(x[1] * dv[2] - x[2] * dv[1]) * L * L / 12, (x[2] * dv[0] - x[0] * dv[2]) * L * L / 12,
                       (x[0] * dv[1] - x[1] * dv[0]) * L * L / 12]
                for nd, sg in ((pp["from"], 1.0), (pp["to"], -1.0)):
                    add(nd, axis, w * L / 2, True)
                    for r in range(3):
                        if mom[r]:
                            add(nd, 3 + r, sg * mom[r], True)
    for reg in lc.get("pressure_regions") or []:
        p = qv(reg["pressure"])
        for mid in reg.get("member_pipe_ids", []):
            if mid in pipes:
                axial_pair(mid, p * geom(mid)[4])
    st = lc.get("analysis_state") or {}
    for es in st.get("element_states", []):
        ts = es.get("thermal_state") or {}
        if es.get("pipe_ref") not in pipes or not ts or ts.get("kind") == "unchanged_reference":
            continue
        pp, x, L, A, Ai, E, alpha = geom(es["pipe_ref"])
        strain = qv(ts["strain"]) if ts.get("kind") == "explicit_interval_strain" else 1e-2  # upper bound
        axial_pair(es["pipe_ref"], E * A * strain)
    for rc in m.get("reference_configurations") or []:
        for mr in rc.get("member_references", []):
            fit = mr.get("fit") or {}
            if mr.get("pipe_ref") in pipes and fit.get("kind") not in (None, "none"):
                pp, x, L, A, Ai, E, alpha = geom(mr["pipe_ref"])
                strain = qv(fit["strain"]) if "strain" in fit else (qv(fit["length_change"]) / L if "length_change" in fit else 1e-2)
                axial_pair(mr["pipe_ref"], E * A * abs(strain))
    free = {k: v for k, v in rows.items() if ["UX", "UY", "UZ", "RX", "RY", "RZ"][k[1]] not in restrained.get(k[0], set())}
    ext = [max(p[k] for p in nodes.values()) - min(p[k] for p in nodes.values()) for k in range(3)]
    Lb = math.sqrt(sum(e * e for e in ext))
    F = max([abs(v[0]) for k, v in free.items() if k[1] < 3] + [0.0])
    M = max([abs(v[0]) for k, v in free.items() if k[1] >= 3] + [0.0])
    fo, mo = max(F, M / Lb if Lb else 0.0), max(M, Lb * F)
    margins = []
    for (nd, dof), (net, sab) in rows.items():  # free rows and restrained (reaction) rows; S*_f from free rows
        if sab == 0:
            continue
        scale = fo if dof < 3 else mo
        bound = 17 * gamma(16) * sab
        margins.append([nd, ["UX", "UY", "UZ", "RX", "RY", "RZ"][dof], net, sab, (1e-9 * max(abs(net), scale)) / bound,
                        "free" if (nd, dof) in free else "restrained"])
    return {"S_star_f": {"force": fo, "moment": mo}, "min_margin": min([mm[4] for mm in margins] + [float("inf")]),
            "rows": margins}


# ---------------------------------------------------------------- Part C
def frame_axes(xi, xj, yref):
    d = [xj[k] - xi[k] for k in range(3)]
    L = math.sqrt(d[0] * d[0] + d[1] * d[1] + d[2] * d[2])
    s = 1.0 / L
    x = [d[0] * s, d[1] * s, d[2] * s]
    pr = sum(yref[k] * x[k] for k in range(3))
    yc = [yref[k] - x[k] * pr for k in range(3)]
    n = math.sqrt(sum(v * v for v in yc))
    y = [v / n for v in yc]
    z = [x[1] * y[2] - x[2] * y[1], x[2] * y[0] - x[0] * y[2], x[0] * y[1] - x[1] * y[0]]
    return [x, y, z], L


def local_k(sec, L):
    e, g, a, i, j = sec["E"], sec["G"], sec["A"], sec["I"], sec["J"]
    K = [[0.0] * 12 for _ in range(12)]

    def s(r, c, v):
        K[r][c] = v
        K[c][r] = v
    ax, tor = e * a / L, g * j / L
    K[0][0] = K[6][6] = ax
    s(0, 6, -ax)
    K[3][3] = K[9][9] = tor
    s(3, 9, -tor)
    for (v, th, sg) in ((1, 5, 1.0), (2, 4, -1.0)):
        k12, k6, k4, k2 = 12 * e * i / L ** 3, 6 * e * i / L ** 2, 4 * e * i / L, 2 * e * i / L
        K[v][v] = K[v + 6][v + 6] = k12
        s(v, v + 6, -k12)
        s(v, th, sg * k6)
        s(v, th + 6, sg * k6)
        s(v + 6, th, -sg * k6)
        s(v + 6, th + 6, -sg * k6)
        K[th][th] = K[th + 6][th + 6] = k4
        s(th, th + 6, k2)
    return K


def section(s):
    e = float(num(s["E"]))
    g = float(num(s["G"])) if "G" in s else e / (2.0 * (1.0 + float(num(s["nu"]))))
    od, idd = float(num(s["OD"])), float(num(s["ID"]))
    a = math.pi * (od * od - idd * idd) / 4.0
    i = math.pi * (od ** 4 - idd ** 4) / 64.0
    return dict(E=e, G=g, A=a, I=i, J=2 * i)


DOF = {"UX": 0, "UY": 1, "UZ": 2, "RX": 3, "RY": 4, "RZ": 5}


def member_rows(m, u, exact=False):
    """For each member end, (key, B, q) with q = hypot of the local bending moments and B the
    K_e.u formation-noise bound, from nodal displacements u[node] = [6]."""
    out = []
    for (mid, a, b, sid) in m["members"]:
        xi = [float(num(v)) for v in m["nodes_m"][a]]
        xj = [float(num(v)) for v in m["nodes_m"][b]]
        d = [xj[k] - xi[k] for k in range(3)]
        yref = (0.0, 1.0, 0.0) if d[0] == 0 and d[1] == 0 else (0.0, 0.0, 1.0)
        R, L = frame_axes(xi, xj, yref)
        K = local_k(section(m["sections"][sid]), L)
        ue = list(u[a]) + list(u[b])
        dl, dabs = [0.0] * 12, [0.0] * 12
        for blk in range(4):
            for r in range(3):
                dl[3 * blk + r] = sum(R[r][c] * ue[3 * blk + c] for c in range(3))
                dabs[3 * blk + r] = sum(abs(R[r][c] * ue[3 * blk + c]) for c in range(3))
        # exact recomputation from the same binary64 operands (R, u, K): the actual formation error
        dx = [sum(Fr(R[r][c]) * Fr(ue[3 * blk + c]) for c in range(3)) for blk in range(4) for r in range(3)]
        for end, (ry, rz) in (("i", (4, 5)), ("j", (10, 11))):
            my = sum(K[ry][k] * dl[k] for k in range(12))
            mz = sum(K[rz][k] * dl[k] for k in range(12))
            by = gamma(16) * sum(abs(K[ry][k]) * dabs[k] for k in range(12))
            bz = gamma(16) * sum(abs(K[rz][k]) * dabs[k] for k in range(12))
            if exact:
                mye = float(sum(Fr(K[ry][k]) * dx[k] for k in range(12)))
                mze = float(sum(Fr(K[rz][k]) * dx[k] for k in range(12)))
                err = max(abs(my - mye), abs(mz - mze))
            else:
                err = None
            off = 0 if end == "i" else 6
            fmag = math.sqrt(sum(sum(K[off + r][k] * dl[k] for k in range(12)) ** 2 for r in range(3)))
            out.append((f"Mb.{mid}.{end}", by + bz, math.hypot(my, mz), mid, err, fmag))
    return out


def body_moment_scale(m, rows):
    pts = [[float(num(v)) for v in p] for p in m["nodes_m"].values()]
    ext = [max(p[k] for p in pts) - min(p[k] for p in pts) for k in range(3)]
    Lb = math.sqrt(sum(e * e for e in ext))
    if not rows:
        return 0.0, Lb
    # S*(moment) = max(S(moment), L_b * S(force)) over the member-end actions (DESIGN 4.1.6)
    return max(max(r[2] for r in rows), Lb * max(r[5] for r in rows)), Lb


def part_c(ref, results, formation):
    pass_rows = []
    fires_a, fires_b, fires_c = [], [], []
    for case in results["cases"]:
        cid = case["id"]
        c = ref.get(cid)
        if c is None or "members" not in c.get("model", {}) or len(c["model"]["members"]) > 200:
            continue
        for mode, x in case.get("modes", {}).items():
            if x.get("verdict") != "pass":
                continue
            obs = {q[0]: float(q[3][0]) for q in x.get("quantities", []) if q[3] and q[3][0] not in (None, "")}
            names = list(c["model"]["nodes_m"])
            u = {}
            ok = True
            for nd in names:
                vec = []
                for comp in ("UX", "UY", "UZ", "RX", "RY", "RZ"):
                    key = ("u." if comp[0] == "U" else "th.") + nd + "." + comp
                    if key not in obs:
                        ok = False
                        break
                    vec.append(obs[key])
                if not ok:
                    break
                u[nd] = vec
            if not ok:
                continue
            rows = member_rows(c["model"], u, exact=True)
            smax, Lb = body_moment_scale(c["model"], rows)
            pass_rows.append((cid, mode))
            for key, B, q, mid, err, _f in rows:
                qo = obs.get(key, q)
                # R-c: actual formation error (exact recomputation), row-relative above the floor R*S*
                if err > 1e-9 * max(abs(q), 2.0 ** -34 * smax):
                    fires_c.append([cid, mode, key, err, q, smax])
                if B > 1e-9 * max(abs(qo), smax):
                    fires_a.append([cid, mode, key, B, qo])
                if B > 1e-9 * abs(qo) and abs(qo) > 1024 * B:
                    fires_b.append([cid, mode, key, B, qo])
    cases_b = sorted({(a, b) for a, b, *_ in fires_b})
    # the four INPLANE formation rows, S11-F state (exact net at N2), emulated
    inplane = {}
    for cid in ("RF-CANCEL-F-G1e80-GnG-INPLANE", "RF-CANCEL-M-G1e80-GnG-INPLANE"):
        m = ref[cid]["model"]
        names = list(m["nodes_m"])
        idx = {n: i for i, n in enumerate(names)}
        n = 6 * len(names)
        Kg = [[0.0] * n for _ in range(n)]
        Kx = [[Fr(0)] * n for _ in range(n)]
        for (mid, a, b, sid) in m["members"]:
            xi = [float(num(v)) for v in m["nodes_m"][a]]
            xj = [float(num(v)) for v in m["nodes_m"][b]]
            R, L = frame_axes(xi, xj, (0.0, 0.0, 1.0))
            Kl = local_k(section(m["sections"][sid]), L)  # axis-aligned: T is a signed permutation
            base = [6 * idx[a] + k for k in range(6)] + [6 * idx[b] + k for k in range(6)]
            for r in range(12):
                for cc in range(12):
                    Kg[base[r]][base[cc]] += Kl[r][cc]
                    Kx[base[r]][base[cc]] += Fr(Kl[r][cc])
        f = [0.0] * n
        for nd, ld in m.get("loads", {}).items():
            for key, off in (("F", 0), ("M", 3)):
                for k, v in enumerate(ld.get(key, [])):
                    f[6 * idx[nd] + off + k] += float(num(v))
        ex = {}
        for nd, dn, v in m["load_contributions_in_authored_order"]:
            dof = 6 * idx[nd] + DOF[dn]
            ex[dof] = ex.get(dof, Fr(0)) + Fr(float(num(v)))
        for dof, v in ex.items():
            f[dof] = float(Fr(f[dof]) + v)  # S11-F: the exact net, rounded once
        free = [i for i in range(n) if i // 6 != idx["N0"]]
        A = [[Kg[i][j] for j in free] + [f[i]] for i in free]
        nf = len(free)
        for k in range(nf):
            p = max(range(k, nf), key=lambda r: abs(A[r][k]))
            A[k], A[p] = A[p], A[k]
            for r in range(k + 1, nf):
                fac = A[r][k] / A[k][k]
                A[r] = [x - fac * y for x, y in zip(A[r], A[k])]
        x = [0.0] * nf
        for r in reversed(range(nf)):
            x[r] = (A[r][nf] - sum(A[r][c] * x[c] for c in range(r + 1, nf))) / A[r][r]
        u = {nd: [0.0] * 6 for nd in names}
        for r, i in enumerate(free):
            u[names[i // 6]][i % 6] = x[r]
        rows = member_rows(m, u, exact=True)
        smax, _ = body_moment_scale(m, rows)
        expd = {row[0]: float(num(row[1])) for row in ref[cid]["expected"]}
        sc = {row[0]: float(num(row[3])) for row in ref[cid]["expected"]}
        exact_from_u = {}
        for (mid, a, b, sid) in m["members"]:
            # exactly rounded K_e.u from the binary64 solution u (F2-free recovery; R is a signed permutation here)
            xi = [float(num(v)) for v in m["nodes_m"][a]]
            xj = [float(num(v)) for v in m["nodes_m"][b]]
            R, L = frame_axes(xi, xj, (0.0, 0.0, 1.0))
            Kl = [[Fr(v) for v in row] for row in local_k(section(m["sections"][sid]), L)]
            ug = [Fr(v) for v in u[a] + u[b]]
            ue = [sum(Fr(R[r][c]) * ug[3 * blk + c] for c in range(3)) for blk in range(4) for r in range(3)]
            for end, (ry, rz) in (("i", (4, 5)), ("j", (10, 11))):
                my = sum(Kl[ry][k] * ue[k] for k in range(12))
                mz = sum(Kl[rz][k] * ue[k] for k in range(12))
                exact_from_u[f"Mb.{mid}.{end}"] = math.hypot(float(my), float(mz))
        for key, B, q, mid, ferr, _f in rows:
            if key in expd:
                err = abs(q - expd[key]) / (1e-9 * max(abs(expd[key]), sc[key]))
                err_exact_recovery = abs(exact_from_u[key] - expd[key]) / (1e-9 * max(abs(expd[key]), sc[key]))
                inplane[f"{cid} {key}"] = {"q_emulated": q, "expected": expd[key], "ratio_to_criterion": err,
                                           "ratio_with_exactly_rounded_recovery_from_binary64_u": err_exact_recovery, "B": B,
                                           "fires_R_a": B > 1e-9 * max(abs(q), smax),
                                           "fires_R_b": B > 1e-9 * abs(q) and abs(q) > 1024 * B,
                                           "formation_error_exact_recompute": ferr,
                                           "fires_R_c": ferr > 1e-9 * max(abs(q), 2.0 ** -34 * smax),
                                           "formation_row": any(r[1] == cid and r[2] == key for r in formation["rows"])}
    return {"passing_case_modes": len(pass_rows), "R_a_firing_rows": len(fires_a), "R_a_firing_case_modes": sorted({(a, b) for a, b, *_ in fires_a}),
            "R_b_firing_rows": len(fires_b), "R_b_firing_case_modes": len(cases_b), "R_b_firing_cases": [list(t) for t in cases_b],
            "R_b_examples": fires_b[:12], "R_c_firing_rows": len(fires_c),
            "R_c_firing_case_modes": sorted({(a, b) for a, b, *_ in fires_c}), "R_c_examples": fires_c[:12],
            "inplane_s11f_state": inplane}


def main():
    root, outp = sys.argv[1], sys.argv[2]
    ref = json.load(open("REFERENCES/references.json"))["cases"]
    results = json.load(open("DETECTION/results.json"))
    formation = json.load(open("GATE/FORMATION_EXCEPTIONS.json"))
    out = {"python": sys.version.split()[0], "part_a_load_row_guard_R1": part_a(ref),
           "part_a2_s11f_probe_a": part_a2(),
           "part_b_committed_fixture_screen": part_b(root), "part_c_recovery_guard_candidates": part_c(ref, results, formation)}
    json.dump(out, open(outp, "w"), indent=1)
    summ = {"A": {k: {"fires": v["fires_exact_defect"], "apriori_fires": v["fires_apriori_bound"], "defect": v["exact_defect"],
                      "threshold": v["threshold"]} for k, v in out["part_a_load_row_guard_R1"].items()},
            "A2": {k: {r: [v["exact_defect"], v["threshold"], v["fires_exact_defect"], v["fires_apriori_c16"]] for r, v in rows.items()}
                   for k, rows in out["part_a2_s11f_probe_a"].items()},
            "B_models_with_formed_loads": len(out["part_b_committed_fixture_screen"]),
            "B_screen": [[r["file"], r["case"], (r["magnitude_screen"] or {}).get("min_margin")] for r in out["part_b_committed_fixture_screen"]],
            "C": {k: v for k, v in out["part_c_recovery_guard_candidates"].items() if k not in ("R_b_examples", "R_b_firing_cases", "R_c_examples")}}
    print(json.dumps(summ, indent=1))


if __name__ == "__main__":
    main()
