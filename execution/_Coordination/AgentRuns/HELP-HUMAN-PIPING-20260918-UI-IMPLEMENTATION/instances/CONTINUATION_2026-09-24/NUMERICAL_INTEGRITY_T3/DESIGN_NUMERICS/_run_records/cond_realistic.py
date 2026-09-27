#!/usr/bin/env python3
"""D1 D-5: equilibrated condition numbers of invented, realistic-shaped pipe models (standard library).

Uses sweep_d5_r1.py's binary64 formation emulation and the same radix equilibration as
prepare_structural, and reports the 1-norm condition number of the equilibrated reduced matrix
(main's Hager-Higham estimate targets this quantity). N section (200 mm OD x 10 mm wall steel).
Supports: anchors (all six rigid), rests (UZ rigid), guides (UY rigid), spring hangers (UZ
translational springs), and soft rotational 'stabiliser' springs where stated. Every model is
invented; nothing here is a reference or a product run.
"""
import json
import math
import sys

sys.path.insert(0, ".")
import sweep_d5_r1 as sw  # noqa: E402
import probe_skew_precision as d1  # noqa: E402

SEC = {"N": {"E": "200000000000", "G": "80000000000", "OD": "0.2", "ID": "0.18"}}


def line(points):
    nodes = {f"N{i}": [str(c) for c in p] for i, p in enumerate(points)}
    members = [[f"M{i}", f"N{i}", f"N{i + 1}", "N"] for i in range(len(points) - 1)]
    return nodes, members


def cond_of(nodes, members, supports):
    model = {"nodes_m": nodes, "sections": SEC, "members": members, "supports": supports, "loads": {}}
    names = list(nodes)
    idx = {nm: i for i, nm in enumerate(names)}
    n = 6 * len(names)
    K = [[0.0] * n for _ in range(n)]
    for (mid, a, b, sid) in members:
        s = sw.section_f64(SEC[sid])
        xi = tuple(float(v) for v in nodes[a])
        xj = tuple(float(v) for v in nodes[b])
        d = [xj[k] - xi[k] for k in range(3)]
        R, L = d1.f64_frame(xi, xj, sw.yref_for(d))
        Kg, T = d1.f64_global_K(sw.local_K(L, s), R)
        mp = d1.dofmap(idx[a], idx[b])
        for i in range(12):
            for j in range(12):
                K[mp[i]][mp[j]] += Kg[i][j]
    restrained = set()
    for nm, sup in supports.items():
        for dn in sup.get("rigid", []):
            restrained.add(6 * idx[nm] + sw.DOF[dn])
        for sp in sup.get("springs", []):
            dof = 6 * idx[nm] + sw.DOF[sp[0]]
            K[dof][dof] += sp[1]
    free = [i for i in range(n) if i not in restrained]
    Kf = [[K[a][b] for b in free] for a in free]
    ex = [-(math.frexp(Kf[r][r])[1] - 1) // 2 for r in range(len(free))]
    A = [[math.ldexp(Kf[r][q], ex[r] + ex[q]) for q in range(len(free))] for r in range(len(free))]
    m = len(free)
    na = max(sum(abs(A[r][q]) for r in range(m)) for q in range(m))
    Lf = [[0.0] * m for _ in range(m)]
    D = [0.0] * m
    for i in range(m):
        for j in range(i):
            s_ = A[i][j] - sum(Lf[i][k] * D[k] * Lf[j][k] for k in range(j))
            Lf[i][j] = s_ / D[j]
        s_ = A[i][i] - sum(Lf[i][k] * D[k] * Lf[i][k] for k in range(i))
        if not s_ > 0:
            return None
        D[i] = s_
    ni = 0.0
    for q in range(m):
        y = [1.0 if r == q else 0.0 for r in range(m)]
        for i in range(m):
            y[i] -= sum(Lf[i][k] * y[k] for k in range(i))
        x = [y[i] / D[i] for i in range(m)]
        for i in reversed(range(m)):
            x[i] -= sum(Lf[k][i] * x[k] for k in range(i + 1, m))
        ni = max(ni, sum(abs(v) for v in x))
    return na * ni


ANCHOR = {"rigid": ["UX", "UY", "UZ", "RX", "RY", "RZ"]}


def main():
    out = {}
    pts = [(3 * i, 0, 0) for i in range(11)]
    nodes, mem = line(pts)
    rests = {f"N{i}": {"rigid": ["UZ"]} for i in (2, 4, 6, 8)}
    out["M1 30 m run, anchor + 4 rests + guide, free end"] = cond_of(nodes, mem, {"N0": ANCHOR, **rests, "N5": {"rigid": ["UY", "UZ"]}})
    hang = {f"N{i}": {"springs": [("UZ", 5e4)]} for i in (2, 4, 6, 8)}
    out["M2 30 m run, anchors both ends, 4 spring hangers 5e4 N/m"] = cond_of(nodes, mem, {"N0": ANCHOR, "N10": ANCHOR, **hang})
    hang_only = {f"N{i}": {"springs": [("UZ", 5e4)]} for i in (3, 6, 9)}
    out["M3 30 m run, one anchor, 3 hangers 5e4 N/m, free end"] = cond_of(nodes, mem, {"N0": ANCHOR, **hang_only, "N10": {"rigid": ["UY"]}})
    pts = [(0, 0, 0), (3, 0, 0), (6, 0, 0), (9, 0, 0), (12, 0, 0), (12, 3, 0), (12, 6, 0), (12, 9, 0), (12, 9, 2), (12, 9, 4), (12, 9, 6)]
    nodes, mem = line(pts)
    out["M4 L+riser 27 m, anchors both ends, 2 hangers 3e4 N/m"] = cond_of(nodes, mem, {"N0": ANCHOR, "N10": ANCHOR, "N3": {"springs": [("UZ", 3e4)]}, "N6": {"springs": [("UZ", 3e4)]}})
    pts = [(0, 0, 0), (3, 0, 0), (6, 0, 0), (8, 2, 0), (10, 4, 0), (12, 6, 0), (12, 6, 3), (12, 6, 6)]
    nodes, mem = line(pts)
    out["M5 run with a 45-degree leg and riser, anchors both ends, 2 hangers"] = cond_of(nodes, mem, {"N0": ANCHOR, "N7": ANCHOR, "N2": {"springs": [("UZ", 3e4)]}, "N4": {"springs": [("UZ", 3e4)]}})
    out["M6 as M5, one anchor, free end held by hangers and a guide"] = cond_of(nodes, mem, {"N0": ANCHOR, "N2": {"springs": [("UZ", 3e4)]}, "N4": {"springs": [("UZ", 3e4)]}, "N7": {"rigid": ["UX", "UY"]}})
    for k in (1e4, 1e2, 1.0):
        out[f"M7 as M6 plus a {k:g} N*m/rad rotational stabiliser (RX) at the free end"] = cond_of(nodes, mem, {"N0": ANCHOR, "N2": {"springs": [("UZ", 3e4)]}, "N4": {"springs": [("UZ", 3e4)]}, "N7": {"rigid": ["UX", "UY"], "springs": [("RX", k)]}})
    pts = [(2 * i, 0, 0) for i in range(31)]
    nodes, mem = line(pts)
    out["M9 60 m run, 30 members, one anchor, hangers 5e4 N/m every 6 m, guided free end"] = cond_of(nodes, mem, {"N0": ANCHOR, **{f"N{i}": {"springs": [("UZ", 5e4)]} for i in range(3, 31, 3)}, "N30": {"rigid": ["UY"]}})
    out["M10 as M9 with anchors at both ends"] = cond_of(nodes, mem, {"N0": ANCHOR, "N30": ANCHOR, **{f"N{i}": {"springs": [("UZ", 5e4)]} for i in range(3, 30, 3)}})
    pts = [(2 * i, i, 0) for i in range(31)]
    nodes, mem = line(pts)
    out["M11 as M9 on a skew line (2,1,0)"] = cond_of(nodes, mem, {"N0": ANCHOR, **{f"N{i}": {"springs": [("UZ", 5e4)]} for i in range(3, 31, 3)}, "N30": {"rigid": ["UY"]}})
    out["M8 3 m cantilever branch, anchored (stub on a header)"] = cond_of(*line([(0, 0, 0), (1, 2, 2)]), {"N0": ANCHOR})
    print(json.dumps({k: (None if v is None else float("%.3g" % v)) for k, v in out.items()}, indent=1))


if __name__ == "__main__":
    main()
