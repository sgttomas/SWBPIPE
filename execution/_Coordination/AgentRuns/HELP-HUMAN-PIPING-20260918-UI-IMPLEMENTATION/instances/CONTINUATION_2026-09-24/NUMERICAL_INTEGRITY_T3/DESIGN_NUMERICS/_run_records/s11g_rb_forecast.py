#!/usr/bin/env python3
"""D1 S11-G revision 2 forecast: the recovery guard R-b and R-b', and the SF-4 floor (standard library only).

Usage (from T3/):
  PYTHONDONTWRITEBYTECODE=1 nice -n 19 python3 DESIGN_NUMERICS/_run_records/s11g_rb_forecast.py <P> <I4_formation_rows.json> <out.json>

<P> is `projects/chirality-piping` (run records used `../../../../../../..`). <I4_formation_rows.json> is I4's
S11-F record `IMPLEMENTATION/S11F/_run_records/formation_rows/formation_rows.json` (post-S11-F published values of
the 14 formation rows; read as sha256 c548f519..., not yet committed); pass `-` to skip.

The guard (S11G_GUARD.md revision 2, section 4). For a straight member end e on the ordinary route, with the
product's K_loc, frame T and published u:
  q = hypot(My, Mz) (the end's two published bending rows),
  B = gamma_16 * (sum_k |K[ry][k]| * sum_c |T_kc| |u_c| + the same for rz)   (formation-noise bound of K_e.u),
  R-b fires when B > 1e-9 * q and q > 2^10 * B;
  R-b' fires when R-b fires and q >= 2^-34 * S*_moment (the DESIGN V1-S8/S8-R floor, S*_moment = max(S(moment),
  L_b * S(force)) of the case's body, DESIGN 4.1.6).

Parts
  C. Committed product outputs: every committed result envelope (a JSON with `results` and `numerical_quality`),
     paired with its committed model by project id and node ids. u, q and the S* rows are the product's own
     published values, so the firing decision is the product's, up to B's K (Euler-Bernoulli pipe section, J = 2I,
     nominal OD and wall).
  E. Committed solving models that have no committed envelope: an exact (Fraction) linear solve of an emulated
     frame model (rigid supports by the product's family rule, springs, nodal loads, full-span distributed loads,
     element thermal, load-state strains and fits, prescribed boundary motions). Decisions use the exact u and
     exact q. Models with no load case, no member, no positions, nonlinear supports, or marked as not solved
     (`analysis_status.mechanics` = `not_run_*`) are listed with the reason.
  F. Frozen references (R1): every case with an explicit model and a complete exact displacement set, using R1's
     exact expected u and Mb (q), and exact S*. Today's predicate (R1's criterion with the stated scale column)
     per row from P1 (`DETECTION/results.json`, captured entry, both modes); for the 18 cases of the 221 the
     post-S11-F state is taken from S11-F's F12 result (every row inside its interval except the 14 formation
     rows); the INPLANE rows use I4's measured post-S11-F values. K-D5 overlap from `recal_d5.json`
     (K-D5 demotes a case-mode when 2 * EF_ratio_coupled_Sstar > 1; cases above 12 members are not emulated).
  S. SF-4: V1's collinear straight-run probe (four runs) and the UDL rows, without and with the S* floor
     (floor_d = 2^-10 * sum of |self-equilibrated formed terms| at row d).
"""
import collections
import json
import math
import os
import sys
from fractions import Fraction as Fr

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import withheld_rows as W  # noqa: E402  (the product's support-family rule, D2-aligned)

U = 2.0 ** -53
R_FLOOR = 2.0 ** -34
DOFN = ["UX", "UY", "UZ", "RX", "RY", "RZ"]


def gamma(k):
    return k * U / (1 - k * U)


def frame(xi, xj, yref):
    """FK frame orientation: x = d * fl(1/|d|), y = Gram-Schmidt of y_reference, z = x cross y."""
    d = [xj[k] - xi[k] for k in range(3)]
    L = math.sqrt(d[0] * d[0] + d[1] * d[1] + d[2] * d[2])
    s = 1.0 / L
    x = [d[0] * s, d[1] * s, d[2] * s]
    if abs(sum(yref[k] * x[k] for k in range(3))) > 0.999999:
        yref = [0.0, 0.0, 1.0] if abs(x[2]) < 0.9 else [0.0, 1.0, 0.0]
    pr = sum(yref[k] * x[k] for k in range(3))
    yc = [yref[k] - x[k] * pr for k in range(3)]
    n = math.sqrt(sum(v * v for v in yc))
    y = [v / n for v in yc]
    z = [x[1] * y[2] - x[2] * y[1], x[2] * y[0] - x[0] * y[2], x[0] * y[1] - x[1] * y[0]]
    return [x, y, z], L


def localk(E, G, A, I, J, L):
    K = [[0.0] * 12 for _ in range(12)]

    def s(r, c, v):
        K[r][c] = v
        K[c][r] = v
    ax, tor = E * A / L, G * J / L
    K[0][0] = K[6][6] = ax
    s(0, 6, -ax)
    K[3][3] = K[9][9] = tor
    s(3, 9, -tor)
    for (v, th, sg) in ((1, 5, 1.0), (2, 4, -1.0)):
        k12, k6, k4, k2 = 12 * E * I / L ** 3, 6 * E * I / L ** 2, 4 * E * I / L, 2 * E * I / L
        K[v][v] = K[v + 6][v + 6] = k12
        s(v, v + 6, -k12)
        s(v, th, sg * k6)
        s(v, th + 6, sg * k6)
        s(v + 6, th, -sg * k6)
        s(v + 6, th + 6, -sg * k6)
        K[th][th] = K[th + 6][th + 6] = k4
        s(th, th + 6, k2)
    return K


def end_bound(K, R, ue):
    """B for both ends: gamma_16 * sum |K_row| |T u| over the two bending rows."""
    dabs = [0.0] * 12
    for blk in range(4):
        for r in range(3):
            dabs[3 * blk + r] = sum(abs(R[r][c] * ue[3 * blk + c]) for c in range(3))
    out = {}
    for end, (ry, rz) in (("i", (4, 5)), ("j", (10, 11))):
        out[end] = gamma(16) * (sum(abs(K[ry][k]) * dabs[k] for k in range(12)) +
                                sum(abs(K[rz][k]) * dabs[k] for k in range(12)))
    return out


def decide(q, B, smo):
    rb = B > 1e-9 * q and q > 1024 * B
    return {"R_b": rb, "R_b_prime": rb and q >= R_FLOOR * smo, "above_floor": q >= R_FLOOR * smo,
            "B_over_1e9q": (B / (1e-9 * q)) if q else None, "q_over_B": (q / B) if B else None}


# ------------------------------------------------------------------ model walking (committed)
SKIP = {"execution", "node_modules", "target", ".git", "schemas"}


def walk_json(root):
    for dp, dn, fn in os.walk(root):
        dn[:] = sorted(x for x in dn if x not in SKIP)
        for f in sorted(fn):
            if not f.endswith(".json"):
                continue
            p = os.path.join(dp, f)
            if os.path.getsize(p) > 30 * 2 ** 20:
                continue
            try:
                yield os.path.relpath(p, root), json.load(open(p))
            except Exception:
                continue


def find(o, pred, depth=0):
    if depth > 4:
        return
    if isinstance(o, dict):
        if pred(o):
            yield o
            return
        for v in o.values():
            yield from find(v, pred, depth + 1)
    elif isinstance(o, list):
        for v in o:
            yield from find(v, pred, depth + 1)


def is_model(o):
    return "load_cases" in o and "pipe_segments" in o and isinstance(o["load_cases"], list)


def is_env(o):
    return "results" in o and isinstance(o["results"], list) and "numerical_quality" in o


UNITS = {"m": 1.0, "mm": 1e-3, "Pa": 1.0, "kPa": 1e3, "MPa": 1e6, "GPa": 1e9, "N": 1.0, "kN": 1e3, "N*m": 1.0,
         "kN*m": 1e3, "N/m": 1.0, "rad": 1.0, "1": 1.0, "degC": 1.0, "K": 1.0, "1/degC": 1.0, "1/K": 1.0,
         "N*m/rad": 1.0}


def qv(q):
    return float(q["value"]) * UNITS[q["unit"]]


def section_of(pp, mats):
    mat = mats.get(pp.get("material"), {})
    E = qv(mat["elastic_modulus"])
    if "shear_modulus" in mat:
        G = qv(mat["shear_modulus"])
    else:
        G = E / (2 * (1 + float((mat.get("poisson_ratio") or {}).get("value", 0.3))))
    od = qv(pp["section"]["outside_diameter"])
    t = qv(pp["section"]["wall_thickness"])
    idd = od - 2 * t
    A = math.pi * (od * od - idd * idd) / 4
    I = math.pi * (od ** 4 - idd ** 4) / 64
    return E, G, A, I, 2 * I, math.pi * idd * idd / 4, mat


def positions(m):
    return {n["id"]: [float(n["position"][k]) for k in "xyz"] for n in m["nodes"]}


def yref_of(pp):
    yr = pp.get("y_reference") or {"x": 0, "y": 0, "z": 1}
    return [float(yr[k]) for k in "xyz"]


# ------------------------------------------------------------------ Part C: committed envelopes
def part_c(root, models):
    out = []
    for path, doc in walk_json(root):
        for e in find(doc, is_env):
            res = e["results"]
            ref = e.get("model_ref")
            nodes = {r["entity_ref"] for r in res if r["kind"].startswith("global_nodal_")}
            if not nodes:
                out.append({"envelope": path, "skipped": "no nodal displacement rows"})
                continue
            cand = [(mp, m, d) for mp, m, d in models if (m.get("project") or {}).get("id") == ref
                    and nodes <= {n["id"] for n in m["nodes"]} and all("position" in n for n in m["nodes"])]
            if not cand:
                cand = [(mp, m, d) for mp, m, d in models if nodes <= {n["id"] for n in m["nodes"]}
                        and all("position" in n for n in m["nodes"])]
            if not cand:
                out.append({"envelope": path, "skipped": "no committed model with node positions (synthetic receipt control)"})
                continue
            cand.sort(key=lambda c: -len(os.path.commonprefix([os.path.dirname(c[0]), os.path.dirname(path)])))
            mp, m, d = cand[0]
            mats = {x["id"]: x for x in (m.get("materials") or d.get("materials") or [])}
            pos = positions(m)
            curved = {(c.get("geometry") or {}).get("bend_pipe_ref") for c in m.get("components") or []}
            bycase = collections.defaultdict(list)
            for r in res:
                if r.get("basis_ref"):
                    bycase[r["basis_ref"]["ref_id"]].append(r)
            qual = {c.get("basis_ref", {}).get("ref_id"): c.get("solve_quality") for c in e["numerical_quality"].get("cases") or []}
            for case, rows in sorted(bycase.items()):
                comp = {"global_nodal_displacement_x": 0, "global_nodal_displacement_y": 1, "global_nodal_displacement_z": 2,
                        "global_nodal_rotation_x": 3, "global_nodal_rotation_y": 4, "global_nodal_rotation_z": 5}
                u = collections.defaultdict(lambda: [None] * 6)
                mom = collections.defaultdict(dict)
                smom = sfor = 0.0
                for r in rows:
                    md = r.get("metadata") or {}
                    if r["kind"] in comp:
                        u[r["entity_ref"]][comp[r["kind"]]] = r["value"] * (1e-3 if r["unit"] == "mm" else 1.0)
                    if r["kind"] in ("element_local_bending_moment_y", "element_local_bending_moment_z"):
                        mom[(r["entity_ref"], md.get("location"))][r["kind"][-1]] = r["value"]
                    if r["unit"] == "N*m" and r["kind"] in ("element_local_torsional_moment", "element_local_bending_moment_y",
                                                            "element_local_bending_moment_z", "support_reaction_component_v2",
                                                            "support_reaction_moment_magnitude_v2"):
                        smom = max(smom, abs(r["value"]))
                    if r["unit"] == "N" and r["kind"] in ("element_local_axial_force", "element_local_shear_force_y",
                                                          "element_local_shear_force_z", "support_reaction_component_v2",
                                                          "support_reaction_force_magnitude_v2", "pipe_wall_axial_force_v2",
                                                          "pipe_effective_axial_force_v2"):
                        sfor = max(sfor, abs(r["value"]))
                ext = [max(v[k] for v in pos.values()) - min(v[k] for v in pos.values()) for k in range(3)]
                Lb = math.sqrt(sum(x * x for x in ext))
                smo = max(smom, Lb * sfor)
                ends = []
                for pp in m["pipe_segments"]:
                    if pp["id"] in curved:
                        continue
                    a, b = pp["from"], pp["to"]
                    if None in u[a] or None in u[b]:
                        continue
                    E_, G_, A, I, J, _, _ = section_of(pp, mats)
                    R, L = frame(pos[a], pos[b], yref_of(pp))
                    Bs = end_bound(localk(E_, G_, A, I, J, L), R, u[a] + u[b])
                    for end in ("i", "j"):
                        mm = mom.get((pp["id"], "end_" + end))
                        if not mm or "y" not in mm or "z" not in mm:
                            continue
                        q = math.hypot(mm["y"], mm["z"])
                        dd = decide(q, Bs[end], smo)
                        ends.append(dict({"pipe": pp["id"], "end": end, "q": q, "B": Bs[end], "My": mm["y"], "Mz": mm["z"]}, **dd))
                out.append({"envelope": path, "model": mp, "case": case, "solve_quality": qual.get(case), "S_star_moment": smo,
                            "ends": len(ends), "R_b": [x for x in ends if x["R_b"]], "R_b_prime": [x for x in ends if x["R_b_prime"]],
                            "near": [x for x in ends if not x["R_b"] and x["q"] and x["B"] > 1e-12 * x["q"] and x["q"] > 16 * x["B"]]})
    return out


# ------------------------------------------------------------------ Part E: emulated exact solve
def emulate(m, doc, lc):
    """Exact linear solve of the emulated frame model for one load case. Returns (u exact per node, K/R per member)."""
    mats = {x["id"]: x for x in (m.get("materials") or doc.get("materials") or [])}
    pos = positions(m)
    ids = [n["id"] for n in m["nodes"]]
    idx = {n: i for i, n in enumerate(ids)}
    ndof = 6 * len(ids)
    K = collections.defaultdict(Fr)
    f = collections.defaultdict(Fr)
    members = []
    state = lc.get("analysis_state") or {}
    strains = {}
    for es in state.get("element_states", []):
        ts = es.get("thermal_state") or {}
        k = ts.get("kind")
        if k == "explicit_interval_strain":
            strains[es["pipe_ref"]] = strains.get(es["pipe_ref"], 0.0) + qv(ts["strain"])
        elif k in ("constant_alpha_interval", "free_length_state"):
            # emulation: a representative axial strain (the axial field decouples from bending in a linear frame)
            strains[es["pipe_ref"]] = strains.get(es["pipe_ref"], 0.0) + 1.0e-3
    for rc in m.get("reference_configurations") or []:
        if rc.get("id") != state.get("reference_configuration_ref"):
            continue
        for mr in rc.get("member_references", []):
            fit = mr.get("fit") or {}
            if fit.get("kind") == "fit_strain":
                strains[mr["pipe_ref"]] = strains.get(mr["pipe_ref"], 0.0) - qv(fit["strain"])
    for pp in m["pipe_segments"]:
        a, b = pp["from"], pp["to"]
        E_, G_, A, I, J, Ai, mat = section_of(pp, mats)
        R, L = frame(pos[a], pos[b], yref_of(pp))
        Kl = localk(E_, G_, A, I, J, L)
        T = [[0.0] * 12 for _ in range(12)]
        for blk in range(4):
            for r in range(3):
                for c in range(3):
                    T[3 * blk + r][3 * blk + c] = R[r][c]
        TF = [[Fr(v) for v in row] for row in T]
        KF = [[Fr(v) for v in row] for row in Kl]
        KT = [[sum(KF[r][k] * TF[k][c] for k in range(12) if TF[k][c]) for c in range(12)] for r in range(12)]
        Kg = [[sum(TF[k][r] * KT[k][c] for k in range(12) if TF[k][r]) for c in range(12)] for r in range(12)]
        dofs = [6 * idx[a] + k for k in range(6)] + [6 * idx[b] + k for k in range(6)]
        for r in range(12):
            for c in range(12):
                if Kg[r][c]:
                    K[(dofs[r], dofs[c])] += Kg[r][c]
        members.append((pp, a, b, Kl, R, L))
        eps = strains.get(pp["id"], 0.0)
        if eps:
            N = Fr(E_) * Fr(A) * Fr(eps)
            for k in range(3):
                f[dofs[k]] -= N * Fr(R[0][k])
                f[dofs[6 + k]] += N * Fr(R[0][k])
    axis = {"global_x": 0, "global_y": 1, "global_z": 2, "UX": 0, "UY": 1, "UZ": 2, "RX": 3, "RY": 4, "RZ": 5,
            "rotation_x": 3, "rotation_y": 4, "rotation_z": 5}
    pipes = {pp["id"]: pp for pp in m["pipe_segments"]}
    for pl in lc.get("primitive_loads") or []:
        tgt = pl.get("target") or {}
        ax = axis.get(pl.get("direction"))
        if ax is None:
            return None, "load direction " + str(pl.get("direction"))
        if tgt.get("type") == "node":
            f[6 * idx[tgt["node"]] + ax] += Fr(qv(pl["magnitude"]))
        elif tgt.get("pipe") in pipes and pl.get("category") in ("distributed_force", "weight"):
            pp = pipes[tgt["pipe"]]
            a, b = pp["from"], pp["to"]
            R, L = frame(pos[a], pos[b], yref_of(pp))
            w = Fr(qv(pl["magnitude"]))
            Lf = Fr(L)
            dv = [Fr(0)] * 3
            dv[ax] = w
            xh = [Fr(v) for v in R[0]]
            mom = [(xh[1] * dv[2] - xh[2] * dv[1]) * Lf * Lf / 12, (xh[2] * dv[0] - xh[0] * dv[2]) * Lf * Lf / 12,
                   (xh[0] * dv[1] - xh[1] * dv[0]) * Lf * Lf / 12]
            for nd, sg in ((a, 1), (b, -1)):
                f[6 * idx[nd] + ax] += w * Lf / 2
                for r in range(3):
                    f[6 * idx[nd] + 3 + r] += sg * mom[r]
        elif tgt.get("pipe") in pipes and pl.get("category") == "thermal":
            pp = pipes[tgt["pipe"]]
            E_, G_, A, I, J, Ai, mat = section_of(pp, mats)
            alpha = qv(mat.get("thermal_expansion_coefficient", {"value": 1.2e-5, "unit": "1/degC"}))
            R, L = frame(pos[pp["from"]], pos[pp["to"]], yref_of(pp))
            N = Fr(E_) * Fr(A) * Fr(alpha) * Fr(qv(pl["magnitude"]))
            for k in range(3):
                f[6 * idx[pp["from"]] + k] -= N * Fr(R[0][k])
                f[6 * idx[pp["to"]] + k] += N * Fr(R[0][k])
        else:
            return None, "element load category " + str(pl.get("category"))
    if lc.get("pressure_regions"):
        return None, "pressure regions (not emulated; axial only)"
    restrained = {}
    for s in m.get("supports") or []:
        fam = s.get("family") or s.get("kind") or ""
        if fam in ("nonlinear", "line_stop") or s.get("nonlinear"):
            return None, "nonlinear support family " + fam
        if fam in ("spring",) and s.get("stiffness"):
            st = s["stiffness"]
            dof = 6 * idx[s["node"]] + DOFN.index(st["dof"])
            K[(dof, dof)] += Fr(qv(st["value"]))
            continue
        if W.is_rigid_support(s):
            for r in s.get("restraints", []):
                restrained[6 * idx[s["node"]] + DOFN.index(r)] = Fr(0)
        elif fam:
            return None, "support family " + fam
    for ss in state.get("support_states", []):
        sup = [s for s in m.get("supports") or [] if s["id"] == ss.get("support_ref")]
        for bm in ss.get("boundary_motion", []) if sup else []:
            dof = 6 * idx[sup[0]["node"]] + DOFN.index(bm["dof"])
            restrained[dof] = Fr(qv(bm["value"]))
    free = [i for i in range(ndof) if i not in restrained]
    # exact Gaussian elimination on the free system with prescribed values moved to the right-hand side
    n = len(free)
    pos_of = {d: i for i, d in enumerate(free)}
    Aexact = [[Fr(0)] * n for _ in range(n)]
    rhs = [f[d] for d in free]
    for (r, c), v in K.items():
        if r in pos_of:
            if c in pos_of:
                Aexact[pos_of[r]][pos_of[c]] += v
            else:
                rhs[pos_of[r]] -= v * restrained[c]
    for k in range(n):
        p = next((r for r in range(k, n) if Aexact[r][k] != 0), None)
        if p is None:
            return None, "singular emulated system (mechanism)"
        Aexact[k], Aexact[p] = Aexact[p], Aexact[k]
        rhs[k], rhs[p] = rhs[p], rhs[k]
        for r in range(k + 1, n):
            if Aexact[r][k]:
                fac = Aexact[r][k] / Aexact[k][k]
                Aexact[r] = [x - fac * y for x, y in zip(Aexact[r], Aexact[k])]
                rhs[r] -= fac * rhs[k]
    x = [Fr(0)] * n
    for r in reversed(range(n)):
        x[r] = (rhs[r] - sum(Aexact[r][c] * x[c] for c in range(r + 1, n))) / Aexact[r][r]
    u = [restrained.get(d, Fr(0)) for d in range(ndof)]
    for d, v in zip(free, x):
        u[d] = v
    return (u, idx, members), None


def part_e(models, env_models):
    seen = {}
    out = []
    for mp, m, d in models:
        key = json.dumps({k: m.get(k) for k in ("nodes", "pipe_segments", "materials", "supports", "components",
                                                "load_cases", "reference_configurations")}, sort_keys=True)
        if key in seen:
            seen[key].append(mp)
            continue
        seen[key] = [mp]
        rec = {"model": mp, "copies": seen[key]}
        mech = str((m.get("analysis_status") or {}).get("mechanics", ""))
        if mp in env_models:
            rec["covered_by_envelope"] = True
        if not m["pipe_segments"] or not m["load_cases"]:
            rec["skipped"] = "no member or no load case"
        elif not all("position" in n for n in m["nodes"]):
            rec["skipped"] = "no node positions (synthetic receipt control)"
        elif mech.startswith("not_run"):
            rec["skipped"] = "not solved by any committed path (analysis_status.mechanics = %s)" % mech
        elif len(m["nodes"]) > 60:
            rec["skipped"] = "more than 60 nodes (exact emulation not attempted)"
        if "skipped" in rec:
            out.append(rec)
            continue
        cases = []
        for lc in m["load_cases"]:
            sol, why = emulate(m, d, lc)
            if sol is None:
                cases.append({"case": lc["id"], "not_emulated": why})
                continue
            u, idx, members = sol
            smom = sfor = Fr(0)
            ends = []
            for pp, a, b, Kl, R, L in members:
                ue = u[6 * idx[a]:6 * idx[a] + 6] + u[6 * idx[b]:6 * idx[b] + 6]
                Bs = end_bound(Kl, R, [float(v) for v in ue])
                dl = [sum(Fr(R[r][c]) * ue[3 * blk + c] for c in range(3)) for blk in range(4) for r in range(3)]
                fl_ = [sum(Fr(Kl[r][k]) * dl[k] for k in range(12)) for r in range(12)]
                for end, (ry, rz), fr_ in (("i", (4, 5), (0, 1, 2)), ("j", (10, 11), (6, 7, 8))):
                    q = math.hypot(float(fl_[ry]), float(fl_[rz]))
                    ends.append((pp["id"], end, q, Bs[end]))
                    smom = max(smom, abs(fl_[ry]), abs(fl_[rz]), abs(fl_[3 if end == "i" else 9]))
                    sfor = max(sfor, *(abs(fl_[k]) for k in fr_))
            pos = positions(m)
            ext = [max(v[k] for v in pos.values()) - min(v[k] for v in pos.values()) for k in range(3)]
            Lb = math.sqrt(sum(x * x for x in ext))
            smo = max(float(smom), Lb * float(sfor))
            dec = [dict({"pipe": p, "end": e, "q": q, "B": B}, **decide(q, B, smo)) for p, e, q, B in ends]
            cases.append({"case": lc["id"], "S_star_moment": smo, "ends": len(dec), "R_b": [x for x in dec if x["R_b"]],
                          "R_b_prime": [x for x in dec if x["R_b_prime"]]})
        rec["cases"] = cases
        out.append(rec)
    return out


# ------------------------------------------------------------------ Part F: frozen references
from sweep_d5_r1 import num as _num  # noqa: E402  (R1's exact decimal and 'a*2^b' strings)


def num(s):
    v = _num(s)
    return v if isinstance(v, Fr) else Fr(v)


def part_f(T3, I4):
    refs = json.load(open(os.path.join(T3, "REFERENCES/references.json")))["cases"]
    p1 = {c["id"]: c for c in json.load(open(os.path.join(T3, "DETECTION/results.json")))["cases"]}
    s11 = json.load(open(os.path.join(T3, "GATE/S11_EXCEPTIONS.json")))
    s11_cases = {t[1] for t in s11["triples"]}
    form = json.load(open(os.path.join(T3, "GATE/FORMATION_EXCEPTIONS.json")))
    form_rows = {(r[1], r[2], r[3]) for r in form["rows"]}
    recal = json.load(open(os.path.join(HERE, "recal_d5.json")))["cases"]
    i4 = {}
    if I4:
        for r in I4:
            i4[(r["case"], r["key"], r["mode"])] = r
    out = {"evaluated": 0, "not_evaluated": [], "cases": {}}
    for cid, c in refs.items():
        m = c["model"]
        if "expected" not in c or "nodes_m" not in m:
            out["not_evaluated"].append([cid, "no explicit model or no expected values (mechanism or generator case)"])
            continue
        exp = {e[0]: e for e in c["expected"]}
        need = [("u." if k < 3 else "th.") + n + "." + DOFN[k] for n in m["nodes_m"] for k in range(6)]
        if any(k not in exp for k in need):
            out["not_evaluated"].append([cid, "exact displacement set incomplete (large case; P1: %s)" %
                                         ",".join(sorted({x.get("verdict", "?") for x in p1.get(cid, {}).get("modes", {}).values()}))])
            continue
        out["evaluated"] += 1
        u = {n: [float(num(exp[("u." if k < 3 else "th.") + n + "." + DOFN[k]][1])) for k in range(6)] for n in m["nodes_m"]}
        smom = max([abs(num(e[1])) for e in c["expected"] if e[2] == "moment"] + [Fr(0)])
        sfor = max([abs(num(e[1])) for e in c["expected"] if e[2] == "force"] + [Fr(0)])
        pts = [[float(num(v)) for v in p] for p in m["nodes_m"].values()]
        ext = [max(p[k] for p in pts) - min(p[k] for p in pts) for k in range(3)]
        Lb = math.sqrt(sum(x * x for x in ext))
        smo = max(float(smom), Lb * float(sfor))
        obs_u = {}
        for mode in ("dense_scrutiny", "sparse_interactive"):
            qrows = {q[0]: q for q in p1.get(cid, {}).get("modes", {}).get(mode, {}).get("quantities", []) if q[3] and q[3][0] not in (None, "")}
            if all(k in qrows for k in need):
                obs_u[mode] = ({n: [float(qrows[("u." if k < 3 else "th.") + n + "." + DOFN[k]][3][0]) for k in range(6)]
                                for n in m["nodes_m"]}, qrows)
        fires = []
        for (mid, a, b, sid) in m["members"]:
            s = m["sections"][sid]
            E_ = float(num(s["E"]))
            G_ = float(num(s["G"])) if "G" in s else E_ / (2 * (1 + float(num(s["nu"]))))
            od, idd = float(num(s["OD"])), float(num(s["ID"]))
            A = math.pi * (od * od - idd * idd) / 4
            I = math.pi * (od ** 4 - idd ** 4) / 64
            xi = [float(num(v)) for v in m["nodes_m"][a]]
            xj = [float(num(v)) for v in m["nodes_m"][b]]
            dvec = [xj[k] - xi[k] for k in range(3)]
            yref = [0.0, 1.0, 0.0] if dvec[0] == 0 and dvec[1] == 0 else [0.0, 0.0, 1.0]
            R, L = frame(xi, xj, yref)
            Kl = localk(E_, G_, A, I, 2 * I, L)
            Bs = end_bound(Kl, R, u[a] + u[b])
            Bobs = {mode: end_bound(Kl, R, ou[a] + ou[b]) for mode, (ou, _) in obs_u.items()}
            for end in ("i", "j"):
                key = "Mb.%s.%s" % (mid, end)
                if key not in exp:
                    continue
                q = abs(float(num(exp[key][1])))
                dd = decide(q, Bs[end], smo)
                dobs = {}
                for mode, (ou, qrows) in obs_u.items():
                    if key in qrows:
                        qo = abs(float(qrows[key][3][0]))
                        dobs[mode] = dict({"q_obs": qo, "B_obs": Bobs[mode][end]}, **decide(qo, Bobs[mode][end], smo))
                if dd["R_b"] or any(v["R_b"] for v in dobs.values()):
                    fires.append(dict({"row": key, "q_exact": q, "B": Bs[end], "p1_observed": dobs,
                                       "borderline": (dd["B_over_1e9q"] or 0) < 2 or any((v["B_over_1e9q"] or 0) < 2 for v in dobs.values())
                                                     or any(v["R_b"] != dd["R_b"] for v in dobs.values()),
                                       "scale": exp[key][3] if len(exp[key]) > 3 else ((c.get("scales") or {}).get(exp[key][2]) or {}).get("value")}, **dd))
        if not fires:
            continue
        rec = {"family": c["family"], "class": "realistic scale" if cid.startswith("RF-LARGE") else "synthetic",
               "S_star_moment": smo, "rows": fires, "modes": {}}
        for mode in ("dense_scrutiny", "sparse_interactive"):
            x = p1.get(cid, {}).get("modes", {}).get(mode, {})
            ident = x.get("identity") or {}
            qrows = {q[0]: q for q in x.get("quantities", [])}
            mrec = {"p1_captured_verdict": x.get("verdict"), "p1_quality": ident.get("numerical_quality_status")}
            verdicts = {}
            for fr in fires:
                key = fr["row"]
                if (cid, key, mode) in form_rows or (cid, key, mode) in {(k[0], k[1], k[2]) for k in i4}:
                    r4 = i4.get((cid, key, mode))
                    verdicts[key] = {"today": "breach (formation row, post-S11-F)", "I4_ratio": r4["cand"]["ratio"] if r4 else None}
                elif cid in s11_cases:
                    verdicts[key] = {"today": "correct (S11 case; post-S11-F every non-formation row inside its interval)"}
                elif key in qrows:
                    qq = qrows[key]
                    verdicts[key] = {"today": "correct" if qq[6] == "pass" else ("breach" if qq[6] == "MISMATCH" else qq[6]),
                                     "p1_ratio": qq[5], "row_relative_error_over_1e-9": (abs(float(qq[3][0]) - float(num(qq[2]))) /
                                                                                          (1e-9 * abs(float(num(qq[2]))))) if qq[3] and float(num(qq[2])) else None}
                else:
                    verdicts[key] = {"today": "not published on the captured entry (%s)" % x.get("verdict")}
            mrec["rows"] = verdicts
            rr = recal.get(cid, {}).get("dense" if mode == "dense_scrutiny" else "sparse")
            mrec["K_D5_demotes"] = (2 * rr["EF_ratio_coupled_Sstar"] > 1) if rr and rr.get("EF_ratio_coupled_Sstar") is not None else None
            rec["modes"][mode] = mrec
        out["cases"][cid] = rec
    return out


# ------------------------------------------------------------------ Part S: SF-4 floor
def part_s():
    def frame_x(xi, xj):
        d = [xj[k] - xi[k] for k in range(3)]
        s = 1.0 / math.sqrt(d[0] * d[0] + d[1] * d[1] + d[2] * d[2])
        return [d[0] * s, d[1] * s, d[2] * s]

    def run(end, stations, N):
        pts = [[round(float(Fr(str(t)) * Fr(str(e))), 6) for e in end] for t in stations]
        rows = {}
        for a in range(len(pts) - 1):
            x = frame_x(pts[a], pts[a + 1])
            for node, sgn in ((a, -1.0), (a + 1, 1.0)):
                for k in range(3):
                    v = sgn * (N * x[k])
                    rows.setdefault((node, k), []).append((v, Fr(sgn) * Fr(N) * Fr(x[k])))
        free = {key: val for key, val in rows.items() if 0 < key[0] < len(pts) - 1}
        nets = {key: sum(i for _, i in val) for key, val in free.items()}
        Sf = max(abs(v) for v in nets.values())
        worst_no = worst_floor = 0.0
        for key, val in free.items():
            E = sum(Fr(v) - i for v, i in val)
            thr = Fr(1, 10 ** 9) * max(abs(nets[key]), Sf)
            floor = Fr(2) ** -10 * sum(abs(Fr(v)) for v, _ in val)  # every term here is a self-equilibrated axial pair term
            thr_f = Fr(1, 10 ** 9) * max(abs(nets[key]), Sf, floor)
            worst_no = max(worst_no, float(abs(E) / thr) if thr else (math.inf if E else 0.0))
            worst_floor = max(worst_floor, float(abs(E) / thr_f) if thr_f else (math.inf if E else 0.0))
        return {"end": end, "stations": stations, "N": N, "S_star_free": float(Sf),
                "worst_without_floor": worst_no, "fires_without_floor": worst_no > 1,
                "worst_with_floor": worst_floor, "fires_with_floor": worst_floor > 1}
    runs = [run(e, s, 1.296e6) for e, s in (((12.0, 5.0, 0.0), (0, 0.13, 0.4, 0.55, 0.81, 1)),
                                           ((10.0, 3.7, 2.2), (0, 0.3, 0.55, 0.7, 1)),
                                           ((6.0, 6.0, 0.0), (0, 0.25, 0.5, 0.75, 1)),
                                           ((9.0, 0.0, 0.0), (0, 0.13, 0.4, 0.55, 0.81, 1)))]
    # a pure-pressure run: same geometry, thrust P*A_i = 2 MPa * pi*0.09^2 (the floor argument is identical)
    runs += [run(e, s, 2e6 * math.pi * 0.09 ** 2) for e, s in (((12.0, 5.0, 0.0), (0, 0.13, 0.4, 0.55, 0.81, 1)),)]
    return {"runs": runs,
            "udl_rows_unchanged": "UDL-W1e5/-W1e8/-W1e80 and probe A carry no self-equilibrated formed term, so their floor is 0 "
                                  "and their statistic and threshold are those of s11g_forecast.py Part A and A2 (W1e8 47.99, "
                                  "W1e80 2.6e81 fire; W1e5 0.0395 and probe A silent)."}


def main():
    root, i4p, outp = sys.argv[1], sys.argv[2], sys.argv[3]
    T3 = os.getcwd()
    models = []
    for path, doc in walk_json(root):
        for m in find(doc, is_model):
            models.append((path, m, doc))
    I4 = json.load(open(i4p)) if i4p != "-" else None
    C = part_c(root, models)
    env_models = {c["model"] for c in C if "model" in c}
    E = part_e(models, env_models)
    F = part_f(T3, I4)
    S = part_s()
    out = {"python": sys.version.split()[0], "part_c_committed_envelopes": C, "part_e_committed_models_emulated": E,
           "part_f_frozen_references": F, "part_s_sf4_floor": S}
    json.dump(out, open(outp, "w"), indent=1)
    summ = {
        "C_case_envelopes": len([c for c in C if "case" in c]), "C_ends": sum(c.get("ends", 0) for c in C),
        "C_R_b": [[c["envelope"], c["case"], c["solve_quality"], [(x["pipe"], x["end"], x["q"], round(x["B_over_1e9q"], 1), round(x["q_over_B"])) for x in c["R_b"]]] for c in C if c.get("R_b")],
        "C_R_b_prime": [[c["envelope"], c["case"]] for c in C if c.get("R_b_prime")],
        "C_skipped": [[c["envelope"], c["skipped"]] for c in C if "skipped" in c],
        "E_distinct_models": len(E), "E_emulated_case_count": sum(1 for e in E for c in e.get("cases", []) if "ends" in c),
        "E_not_emulated": [[e["model"], c["case"], c["not_emulated"]] for e in E for c in e.get("cases", []) if "not_emulated" in c],
        "E_skipped": [[e["model"], e["skipped"]] for e in E if "skipped" in e],
        "E_R_b": [[e["model"], c["case"], [(x["pipe"], x["end"]) for x in c["R_b"]]] for e in E for c in e.get("cases", []) if c.get("R_b")],
        "E_R_b_prime": [[e["model"], c["case"]] for e in E for c in e.get("cases", []) if c.get("R_b_prime")],
        "F_evaluated": F["evaluated"], "F_not_evaluated": F["not_evaluated"],
        "F_firing": {cid: {"class": r["class"], "rows": [(x["row"], x["R_b"], x["R_b_prime"], round(x["B_over_1e9q"], 2), round(x["q_over_B"]),
                                                          {mo: (v["R_b"], v["R_b_prime"], round(v["B_over_1e9q"], 2)) for mo, v in x["p1_observed"].items()}, x["borderline"]) for x in r["rows"]],
                           "p1_quality": {mo: mr["p1_quality"] for mo, mr in r["modes"].items()},
                           "today": {mo: {k: v["today"] for k, v in mr["rows"].items()} for mo, mr in r["modes"].items()},
                           "K_D5": {mo: mr["K_D5_demotes"] for mo, mr in r["modes"].items()}} for cid, r in F["cases"].items()},
        "S": [[r["end"], r["worst_without_floor"], r["worst_with_floor"]] for r in S["runs"]],
    }
    print(json.dumps(summ, indent=1))


if __name__ == "__main__":
    main()
