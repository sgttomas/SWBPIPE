#!/usr/bin/env python3
"""T3 D1 revision 2 probe for V1-B1 and V1-S8 (standard library; no product code).

Uses D1's revision-1 emulation (probe_skew_precision.py, imported unchanged) and
adds three things:

  load_mode   "fold_p": revision 1 (nodal contributions added one by one at p);
              "ledger": the revision-2 rule (exact sum of the binary64
              contributions per DOF, rounded once to p).
  combine     "exact": sum_i c_i*u_i formed exactly over the p-bit retained states,
              rounded once to p, then actions recovered at p from that state;
              "fold_p": the same sum accumulated term by term at p (the V1-B1 defect).
              Combination outputs go through the same stop rule as case outputs.
  sections    per-member section scaling, for a weak-coupling control (V1-S8).

Checks:
  B1-L   V1's check L on D1's N05-class skew case (tip moment contributions
         (1e80, 1e-8, -1e80) on RX, plus 2e-8 on RY): fold_p versus ledger.
  B1-C   absorbed-term combination A + B - A2 (A = A2 = tip moment 1e80, B = 1e-8):
         combine fold_p versus exact, both under the stop rule.
  B1-E   combination whose true value needs escalation: A = (P, eps) with
         eps/P = 1e-45, minus B = P. Exact combination: 128 must be rejected and 256
         accepted.
  S8-W   weak coupling: a stiff member, then a member whose E is scaled by s
         (1e-10, 1e-14) to a node grounded by stiff springs; the far node's
         quantities sit far below the body scale. Reports whether 128 is accepted,
         the strict relative error of every nonzero quantity, and which quantities
         fall below the stated floor R*S* with R = 2^-64/1e-9.

Large-magnitude loads (1e80) are arithmetic stress only, outside any small-rotation
physical claim. Every value is invented; nothing here is a frozen reference.
"""
from fractions import Fraction as Fr
import json
import math
import sys

sys.path.insert(0, ".")
import probe_skew_precision as d1  # noqa: E402

G = Fr(1, 2 ** 64)
CRIT = Fr(1, 10 ** 9)
FLOOR_R = G / CRIT  # about 5.42e-11


def section_scaled(s):
    sec = dict(d1.SECTION)
    sec["E"] = sec["E"] * Fr(s)
    sec["G"] = sec["G"] * Fr(s)
    return sec


def solve(case, p, load_mode="ledger"):
    """Returns (u, elems, K, f) at precision p (None = exact)."""
    ar = d1.A(p)
    n = case["n"]
    K = [[Fr(0)] * n for _ in range(n)]
    elems = []
    for mem in case["members"]:
        i, j, xi, xj, yref = mem[:5]
        sec = mem[5] if len(mem) > 5 else d1.SECTION
        R, L = d1.element_frame(xi, xj, yref, ar)
        T = d1.transform12(R)
        Bl = d1.basic_B_local(L, ar)
        Bg = d1.matmul(Bl, T, ar)
        D = d1.basic_D(sec, L, ar)
        Ke = d1.matmul(d1.transpose(Bg), d1.matmul(D, Bg, ar), ar)
        m = d1.dofmap(i, j)
        for a in range(12):
            for b in range(12):
                if Ke[a][b] != 0:
                    K[m[a]][m[b]] = ar.add(K[m[a]][m[b]], Ke[a][b])
        elems.append((m, T, Bl, D))
    for dof, k in case["springs"]:
        K[dof][dof] = ar.add(K[dof][dof], d1.F(k))
    f = [Fr(0)] * n
    if load_mode == "fold_p":
        for dof, v in case["loads"]:
            f[dof] = ar.add(f[dof], d1.F(v))
    else:
        exact = {}
        for dof, v in case["loads"]:
            exact[dof] = exact.get(dof, Fr(0)) + d1.F(v)
        for dof, s in exact.items():
            f[dof] = d1.rnd(s, p)
    free = [d for d in range(n) if d not in case["restrained"]]
    x, err = d1.ldl_solve([[K[a][b] for b in free] for a in free], [f[a] for a in free], ar)
    if err:
        return None, err
    u = [Fr(0)] * n
    for a, v in zip(free, x):
        u[a] = v
    return (u, elems, K, f, ar), None


def recover(case, state, u=None, f=None):
    uu, elems, K, ff, ar = state
    u = uu if u is None else u
    f = ff if f is None else f
    element_F = []
    for (m, T, Bl, D) in elems:
        ue = [u[d] for d in m]
        dl = [ar.dot(T[r], ue) for r in range(12)]
        e = [ar.dot(Bl[r], dl) for r in range(6)]
        Q = [ar.dot(D[r], e) for r in range(6)]
        mi = ar.sqrt(ar.add(ar.mul(Q[2], Q[2]), ar.mul(Q[4], Q[4])))
        mj = ar.sqrt(ar.add(ar.mul(Q[3], Q[3]), ar.mul(Q[5], Q[5])))
        element_F.append(dict(N=Q[0], T=Q[1], Mi=mi, Mj=mj))
    reactions = {}
    for dof, k in case["springs"]:
        reactions[dof] = ar.mul(-d1.F(k), u[dof])
    for dof in case["restrained"]:
        reactions[dof] = ar.sub(ar.dot(K[dof], u), f[dof])
    return d1.published(case, u, element_F, reactions)


def publish(case, p, load_mode="ledger"):
    st, err = solve(case, p, load_mode)
    return (None, err) if err else (recover(case, st), None)


def combination(case_list, factors, p, mode, load_mode="ledger"):
    """Combination of retained states (same operator), then recovery at p."""
    states = []
    for c in case_list:
        st, err = solve(c, p, load_mode)
        if err:
            return None, err
        states.append(st)
    base = states[0]
    n = case_list[0]["n"]
    ar = base[4]
    u = [Fr(0)] * n
    f = [Fr(0)] * n
    for idx in range(n):
        if mode == "exact":
            su = sum((d1.F(c) * st[0][idx] for c, st in zip(factors, states)), Fr(0))
            sf = sum((d1.F(c) * st[3][idx] for c, st in zip(factors, states)), Fr(0))
            u[idx] = d1.rnd(su, p)
            f[idx] = d1.rnd(sf, p)
        else:
            su = Fr(0)
            sf = Fr(0)
            for c, st in zip(factors, states):
                su = ar.add(su, ar.mul(d1.F(c), st[0][idx]))
                sf = ar.add(sf, ar.mul(d1.F(c), st[3][idx]))
            u[idx], f[idx] = su, sf
    return recover(case_list[0], base, u, f), None


def strict_relative(pub, ref):
    worst, at, fails = Fr(0), None, 0
    for key, (_, _, e) in ref.items():
        if Fr(e) == 0:
            continue
        o = Fr(float(Fr(pub[key][2])))
        ep = Fr(float(Fr(e)))
        r = abs(o - ep) / abs(ep)
        if r > CRIT:
            fails += 1
        if r > worst:
            worst, at = r, key
    return {"max_relative_error_nonzero": float(worst), "at": at, "failing": fails}


def below_floor(pub, case):
    sc = d1.scales(pub, case)
    return sorted(k for k, (_, _, v) in pub.items() if Fr(v) != 0 and abs(Fr(v)) < FLOOR_R * sc[k])


def report(name, pubs, ref, case):
    out = {"case": name}
    for p, pub in pubs.items():
        if isinstance(pub, str):
            out[f"p{p}"] = pub
            continue
        out[f"p{p}"] = {"body_scale": d1.compare(pub, ref, case), "strict": strict_relative(pub, ref),
                        "nonzero_below_floor": below_floor(pub, case)}
    for p in list(pubs):
        if 2 * p in pubs and not isinstance(pubs[p], str) and not isinstance(pubs[2 * p], str):
            out[f"stop_rule_p{p}_vs_p{2 * p}"] = d1.agreement(pubs[p], pubs[2 * p], case)
    return out


def with_loads(case, loads):
    c = dict(case)
    c["loads"] = loads
    return c


def main():
    res = {"python": sys.version.split()[0], "floor_R": float(FLOOR_R), "checks": []}
    skew = (3, 4, 0)
    base = d1.pin_case(skew, 1e-4, (0.0, 0.0, 0.0))
    tip = 6
    # B1-L: V1 check L on D1's N05-class skew case.
    caseL = with_loads(base, [(tip + 3, 1e80), (tip + 3, 1e-8), (tip + 3, -1e80), (tip + 4, 2e-8)])
    ref, err, _ = d1.method_exact_reference(caseL)
    assert err is None
    for mode in ("fold_p", "ledger"):
        pubs = {}
        for p in (128, 256, 512):
            pub, err = publish(caseL, p, mode)
            pubs[p] = err or pub
        res["checks"].append(report(f"B1-L check L, loads {mode}", pubs, ref, caseL))
    # B1-C: absorbed-term combination A + B - A2.
    A = with_loads(base, [(tip + 3, 1e80)])
    B = with_loads(base, [(tip + 3, 1e-8)])
    A2 = with_loads(base, [(tip + 3, 1e80)])
    refB, err, _ = d1.method_exact_reference(B)  # true combination equals case B
    for mode in ("fold_p", "exact"):
        pubs = {}
        for p in (128, 256, 512):
            pub, err = combination([A, B, A2], [1.0, 1.0, -1.0], p, mode)
            pubs[p] = err or pub
        res["checks"].append(report(f"B1-C combination A+B-A2, combine {mode}", pubs, refB, B))
    # B1-E: combination needing escalation: (P, eps) - P with eps/P = 1e-45.
    P, eps = 1e-8, 1e-53
    Ae = with_loads(base, [(tip + 3, P), (tip + 3, eps)])
    Be = with_loads(base, [(tip + 3, P)])
    refE, err, _ = d1.method_exact_reference(with_loads(base, [(tip + 3, eps)]))
    pubs = {}
    for p in (128, 256, 512):
        pub, err = combination([Ae, Be], [1.0, -1.0], p, "exact")
        pubs[p] = err or pub
    res["checks"].append(report("B1-E combination (P,eps)-P, eps/P=1e-45, combine exact", pubs, refE,
                                with_loads(base, [(tip + 3, eps)])))
    # S8-W: weak coupling through a soft member to a stiffly grounded node.
    for s in (1e-10, 1e-14):
        n0, n1, n2 = (0.0, 0.0, 0.0), (3.0, 4.0, 0.0), (6.0, 8.0, 0.0)
        members = [(0, 1, n0, n1, (0.0, 0.0, 1.0)), (1, 2, n1, n2, (0.0, 0.0, 1.0), section_scaled(s))]
        springs = [(12 + a, 1e12) for a in range(6)]
        caseW = d1.build_case(members, springs, list(range(6)), [(6 + 1, 1000.0), (6 + 5, 300.0)], 3)
        refW, err = solve(caseW, None)
        assert err is None
        refWp = recover(caseW, refW)
        pubs = {}
        for p in (128, 256):
            pub, err = publish(caseW, p)
            pubs[p] = err or pub
        rep = report(f"S8-W weak coupling, soft member E scaled {s:g}", pubs, refWp, caseW)
        sc = d1.scales(refWp, caseW)
        far = {k: {"value": float(Fr(v)), "over_scale": float(abs(Fr(v)) / sc[k]) if sc[k] else None}
               for k, (_, _, v) in refWp.items() if k.startswith("u:2:") and Fr(v) != 0}
        rep["far_node_quantities_exact"] = far
        res["checks"].append(rep)
    print(json.dumps(res, indent=1))


if __name__ == "__main__":
    main()
