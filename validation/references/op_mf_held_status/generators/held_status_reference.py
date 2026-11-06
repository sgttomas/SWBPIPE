"""Independent exact references for the held-status solve (`minuend_final`).

DEL-05-02 Work `operating-case.md` §7.2 and §7.9. Python standard library only (`fractions`); no product code is read.

The model is a 2D Euler-Bernoulli frame in the XZ plane, with DOFs (ux, uz, th) per node, solved exactly in rationals.
Each model carries one nonlinear support. A case's own state is the unique admissible state of the static,
history-free problem, found by enumerating every candidate state; the generator refuses a case with zero or several. A
held solve fixes the source's final state, is oriented by the source's final iterate (slip sense and derived-normal
sign), and applies §7.2's classifier rules once. `released` is a changed state or an inadmissible derived-normal branch.

Run in this directory: `python3 -I generators/held_status_reference.py` rebuilds every output in memory and compares
it with `references.json`; `--write` rewrites it. Every value is written as an exact fraction and as the nearest
binary64 (`float.hex()`).
"""
from fractions import Fraction as Fr
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "..", "references.json")


def solve_lin(a, b):
    n = len(a)
    m = [row[:] + [b[i]] for i, row in enumerate(a)]
    for c in range(n):
        p = next(r for r in range(c, n) if m[r][c] != 0)
        m[c], m[p] = m[p], m[c]
        for r in range(n):
            if r != c and m[r][c] != 0:
                f = m[r][c] / m[c][c]
                m[r] = [x - f * y for x, y in zip(m[r], m[c])]
    return [m[i][n] / m[i][i] for i in range(n)]


class Frame:
    """Nodes (x, z); elements with EA, EI; global DOF 3*i + (0 ux, 1 uz, 2 th)."""

    def __init__(self, nodes):
        self.nodes = nodes
        self.n = 3 * len(nodes)
        self.k = [[Fr(0)] * self.n for _ in range(self.n)]
        self.elems = []

    def element(self, i, j, ea, ei):
        (xi, zi), (xj, zj) = self.nodes[i], self.nodes[j]
        dx, dz = Fr(xj - xi), Fr(zj - zi)
        assert dx == 0 or dz == 0, "axis-aligned members only"
        length = abs(dx) + abs(dz)
        c, s = dx / length, dz / length
        a, b1, b2, b3 = Fr(ea) / length, 12 * Fr(ei) / length ** 3, 6 * Fr(ei) / length ** 2, Fr(ei) / length
        kl = [[a, 0, 0, -a, 0, 0], [0, b1, b2, 0, -b1, b2], [0, b2, 4 * b3, 0, -b2, 2 * b3],
              [-a, 0, 0, a, 0, 0], [0, -b1, -b2, 0, b1, -b2], [0, b2, 2 * b3, 0, -b2, 4 * b3]]
        t = [[c, s, 0, 0, 0, 0], [-s, c, 0, 0, 0, 0], [0, 0, 1, 0, 0, 0],
             [0, 0, 0, c, s, 0], [0, 0, 0, -s, c, 0], [0, 0, 0, 0, 0, 1]]
        kg = [[sum(t[p][r] * kl[p][q] * t[q][w] for p in range(6) for q in range(6)) for w in range(6)] for r in range(6)]
        dofs = [3 * i, 3 * i + 1, 3 * i + 2, 3 * j, 3 * j + 1, 3 * j + 2]
        for r in range(6):
            for w in range(6):
                self.k[dofs[r]][dofs[w]] += kg[r][w]
        self.elems.append((Fr(ea), c, s, length, dofs))

    def thermal(self, eps):
        f = [Fr(0)] * self.n
        for e, ep in eps.items():
            ea, c, s, _, dofs = self.elems[e]
            p = ea * ep
            f[dofs[0]] -= p * c
            f[dofs[1]] -= p * s
            f[dofs[3]] += p * c
            f[dofs[4]] += p * s
        return f

    def axial(self, e, u, eps):
        ea, c, s, length, dofs = self.elems[e]
        du = (u[dofs[3]] - u[dofs[0]]) * c + (u[dofs[4]] - u[dofs[1]]) * s
        return ea * (du / length - eps.get(e, 0))

    def solve(self, prescribed, force, applied=None, coupled=None):
        """`applied` {dof: phi} constant; `coupled` (d, n, c): phi at free d equals c * R_n (n restrained).
        Returns (u, R, phi): R = K u - f, with every applied force added back (the reported convention)."""
        applied = dict(applied or {})
        free = [d for d in range(self.n) if d not in prescribed]
        idx = {d: q for q, d in enumerate(free)}
        m = len(free) + (1 if coupled else 0)
        a = [[Fr(0)] * m for _ in range(m)]
        b = [Fr(0)] * m
        for d in free:
            for w in free:
                a[idx[d]][idx[w]] = self.k[d][w]
            b[idx[d]] = force[d] + applied.get(d, 0) - sum(self.k[d][w] * v for w, v in prescribed.items())
        if coupled:
            d, nsrc, c = coupled
            a[idx[d]][m - 1] = Fr(-1)
            a[m - 1][m - 1] = Fr(1)
            for w in free:
                a[m - 1][idx[w]] = -c * self.k[nsrc][w]
            b[m - 1] = c * (sum(self.k[nsrc][w] * v for w, v in prescribed.items()) - force[nsrc])
        x = solve_lin(a, b)
        u = [Fr(0)] * self.n
        for d, v in prescribed.items():
            u[d] = Fr(v)
        for d in free:
            u[d] = x[idx[d]]
        phi = Fr(0)
        if coupled:
            phi = x[m - 1]
            applied[coupled[0]] = applied.get(coupled[0], 0) + phi
        elif applied:
            phi = sum(applied.values())
        total = [force[r] + applied.get(r, 0) for r in range(self.n)]
        r_vec = [sum(self.k[r][w] * u[w] for w in range(self.n)) - total[r] for r in range(self.n)]
        for dof, v in applied.items():
            r_vec[dof] += v
        return u, r_vec, phi


def sgn(x):
    return (x > 0) - (x < 0)


class Support:
    """kind: one_way (sense +1/-1) | gap (sense, c) | friction (mu, normal ('explicit', N) or ('derived', dof))."""

    def __init__(self, dof, kind, **kw):
        self.dof, self.kind, self.kw = dof, kind, kw


def boundary(model, sup, st, ground):
    pres = dict(model["base"])
    if sup.kind == "one_way" and st == "active":
        pres[sup.dof] = ground
    if sup.kind == "gap" and st == "active":
        sig = sup.kw["sense"]
        pres[sup.dof] = sig * (sup.kw["c"] + sig * ground)
    if sup.kind == "friction" and st == "sticking":
        pres[sup.dof] = ground
    return pres


def run(model, sup, case, st, direction=None, beta=None, frozen_normal=None):
    pres = boundary(model, sup, st, case["ground"])
    for d, v in case.get("base", {}).items():
        pres[d] = v
    applied, coupled = {}, None
    if sup.kind == "friction" and st == "sliding" and direction:
        mu = sup.kw["mu"]
        src, val = sup.kw["normal"]
        if frozen_normal is not None:
            applied[sup.dof] = -direction * mu * abs(frozen_normal)
        elif src == "explicit":
            # The step forms fl(-dir*mu*|N|) and applies it only when nonzero; a binary64 product that
            # underflows to zero (H-15c) applies no force.
            phi = -direction * mu * abs(Fr(val))
            if phi != 0 and float(-direction * float(mu)) * float(abs(Fr(val))) != 0.0:
                applied[sup.dof] = phi
        elif mu > 0:
            coupled = (sup.dof, val, -direction * mu * beta)
    u, r, phi = model["frame"].solve(pres, case["force"], applied, coupled)
    return {"u": u, "R": r, "phi": phi, "state": st, "direction": direction, "beta": beta, "ground": case["ground"]}


def normal_of(sup, r):
    src, val = sup.kw["normal"]
    return (abs(Fr(val)), Fr(val)) if src == "explicit" else (abs(r[val]), r[val])


def admissible(sup, sol):
    u, r, d, g = sol["u"], sol["R"], sup.dof, sol["ground"]
    delta = u[d] - g
    if sup.kind == "one_way":
        s = sup.kw["sense"]
        return s * r[d] > 0 if sol["state"] == "active" else s * delta >= 0
    if sup.kind == "gap":
        s, c = sup.kw["sense"], sup.kw["c"]
        return -s * r[d] >= 0 if sol["state"] == "active" else s * delta < c
    n_abs, n_signed = normal_of(sup, r)
    mu = sup.kw["mu"]
    if sol["state"] == "inactive":
        return n_abs <= 0 if sup.kw["normal"][0] == "explicit" else n_signed == 0
    if n_abs <= 0:
        return False
    if sol["state"] == "sticking":
        return abs(r[d]) <= mu * n_abs
    if sup.kw["normal"][0] == "derived" and mu > 0 and sgn(n_signed) != sol["beta"]:
        return False
    return sgn(delta) == sol["direction"] and (mu * n_abs == 0 or sol["phi"] * delta < 0)


def own(model, sup, case):
    if sup.kind in ("one_way", "gap"):
        cands = [("active", None, None), ("inactive", None, None)]
    else:
        betas = (1, -1) if sup.kw["normal"][0] == "derived" else (None,)
        cands = [("inactive", None, None), ("sticking", None, None)] + [
            ("sliding", d, b) for d in (1, -1) for b in betas]
    ok = [s for s in (run(model, sup, case, *c) for c in cands) if admissible(sup, s)]
    assert len(ok) == 1, [(s["state"], s["direction"], s["beta"]) for s in ok]
    return ok[0]


def limit_of(mu, n_abs):
    """The classifier's limit fl(mu*N): zero exactly when the binary64 product is zero (mu = 0, or an underflow)."""
    return mu * n_abs if float(mu) * float(n_abs) != 0.0 else Fr(0)


def classify(sup, sol):
    """§7.2's classifier rules: (classified state, derived branch admissible)."""
    u, r, d, g, st = sol["u"], sol["R"], sup.dof, sol["ground"], sol["state"]
    delta = u[d] - g
    if sup.kind == "one_way":
        s = sup.kw["sense"]
        if st == "active":
            return ("active" if s * r[d] > 0 else "inactive"), True
        return ("active" if s * delta < 0 else "inactive"), True
    if sup.kind == "gap":
        s, c = sup.kw["sense"], sup.kw["c"]
        if st == "active":
            return ("active" if -s * r[d] >= 0 else "inactive"), True
        return ("active" if s * delta >= c else "inactive"), True
    n_abs, n_signed = normal_of(sup, r)
    mu = sup.kw["mu"]
    if n_abs <= 0:
        return "inactive", True
    if st == "sliding":
        if sup.kw["normal"][0] == "derived" and mu > 0 and sol["direction"]:
            b = sol["beta"]
            if not ((n_signed == 0) if b == 0 else (b * n_signed >= 0)):
                return "sliding", False
        phi = sol["phi"]
        lim = limit_of(mu, n_abs)
        if lim == 0:
            return ("sliding" if phi == 0 and delta != 0 else "sticking"), True
        return ("sliding" if delta != 0 and phi * delta < 0 and r[d] * delta < 0 else "sticking"), True
    return ("sticking" if abs(r[d]) <= mu * n_abs else "sliding"), True


def held(model, sup, source, target):
    st, d = source["state"], sup.dof
    direction = beta = None
    if sup.kind == "friction" and st == "sliding":
        direction = sgn(source["u"][d] - source["ground"])
        if sup.kw["normal"][0] == "derived":
            beta = sgn(source["R"][sup.kw["normal"][1]])
    sol = run(model, sup, target, st, direction, beta)
    sol["classified"], sol["branch_ok"] = classify(sup, sol)
    return sol


def v(x):
    x = Fr(x)
    return {"exact": f"{x.numerator}/{x.denominator}" if x.denominator != 1 else str(x.numerator), "binary64": float(x).hex()}


def record(sup, sol, extra=None):
    d = sup.dof
    out = {"state": sol["state"], "u": v(sol["u"][d]), "reaction": v(sol["R"][d])}
    if sol["direction"] is not None:
        out["slip_sense"] = sol["direction"]
    if sup.kind == "friction":
        out["friction_force"] = v(sol["phi"])
        if sup.kw["normal"][0] == "derived":
            out["normal_signed"] = v(sol["R"][sup.kw["normal"][1]])
            if sol["beta"] is not None:
                out["beta"] = sol["beta"]
    for k, x in (extra or {}).items():
        out[k] = v(x)
    return out


# ---------------------------------------------------------------- models
K_EA, L_BAR, K_EI = Fr(2 ** 22), 4, Fr(2 ** 10)   # the bar: k = EA/L = 2^20 N/m
ALPHA = Fr(1, 2 ** 16)


def eps(dt):
    return ALPHA * dt


def bar(tip_uz=False):
    fr = Frame([(0, 0), (L_BAR, 0)])
    fr.element(0, 1, K_EA, K_EI)
    base = {0: 0, 1: 0, 2: 0}
    if tip_uz:
        base[4] = 0
    return {"frame": fr, "base": base, "axial": 0}


def bar_case(model, fx=(), dt=0, fz=0, f_anchor=0, ground=0):
    fr = model["frame"]
    f = fr.thermal({0: Fr(eps(dt))}) if dt else [Fr(0)] * fr.n
    for p in fx:
        f[3] += Fr(p)
    f[4] += Fr(fz)
    f[0] += Fr(f_anchor)
    return {"force": f, "ground": Fr(ground), "eps": {0: Fr(eps(dt))}}


def case_record(model, sup, name, description, o, s, sh, oc, sc, outcome_note, mutants=None):
    def ax(sol, c):
        return {"axial": model["frame"].axial(model["axial"], sol["u"], c["eps"])} if model.get("axial") is not None else {}
    rec = {
        "description": description,
        "source_own": record(sup, o, ax(o, oc)),
        "target_own": record(sup, s, ax(s, sc)),
        "target_held": dict(record(sup, sh, ax(sh, sc)), classified_state=sh["classified"],
                            derived_branch_admissible=sh["branch_ok"],
                            released=sh["classified"] != sh["state"] or not sh["branch_ok"]),
        "source_minus_held": {"du": v(o["u"][sup.dof] - sh["u"][sup.dof]), "dreaction": v(o["R"][sup.dof] - sh["R"][sup.dof])},
        "outcome": outcome_note,
    }
    if model.get("axial") is not None:
        rec["source_minus_held"]["daxial"] = v(model["frame"].axial(model["axial"], o["u"], oc["eps"])
                                               - model["frame"].axial(model["axial"], sh["u"], sc["eps"]))
    if mutants:
        rec["mutants"] = mutants
    return name, rec


def build():
    cases = {}

    def add(item):
        cases[item[0]] = item[1]

    # H-1 / H-1b: the V-2 cantilever, L = 2, P = 48 at midspan, EI = 40, so s* = 5PL^3/(48EI) = 1; one-way tip prop
    # (positive sense) on UZ; the operating anchor rise s is prescribed at A's UZ.
    def v2(rise):
        fr = Frame([(0, 0), (1, 0), (2, 0)])
        fr.element(0, 1, Fr(2 ** 30), Fr(40))
        fr.element(1, 2, Fr(2 ** 30), Fr(40))
        model = {"frame": fr, "base": {0: 0, 1: 0, 2: 0}, "axial": None}
        sup = Support(7, "one_way", sense=1)
        f = [Fr(0)] * fr.n
        f[4] = Fr(-48)
        oc = {"force": f, "ground": Fr(0), "base": {1: Fr(rise)}, "eps": {}}
        sc = {"force": f, "ground": Fr(0), "eps": {}}
        return model, sup, oc, sc

    for name, rise, note in (("H-1", 2, "R (changed: penetration)"), ("H-1b", Fr(1, 2), "E or N (kept; held = own)")):
        model, sup, oc, sc = v2(rise)
        o, s = own(model, sup, oc), own(model, sup, sc)
        sh = held(model, sup, o, sc)
        rec = case_record(model, sup, name, f"V-2 cantilever, units P = 48, L = 2, s* = 1; operating rise s = {rise} s*",
                          o, s, sh, oc, sc, note)[1]
        for tag, sol in (("source_own", o), ("target_own", s), ("target_held", sh)):
            rec[tag]["anchor_moment_over_PL"] = v(abs(sol["R"][2]) / 96)
            rec[tag]["prop_reaction_over_P"] = v(sol["R"][7] / 48)
            rec[tag]["tip_u_over_sstar"] = v(sol["u"][7])
        cases[name] = rec

    m = bar()
    sup = Support(3, "one_way", sense=1)
    oc, sc = bar_case(m, fx=[512], dt=-16), bar_case(m, fx=[512])
    o, s = own(m, sup, oc), own(m, sup, sc)
    add(case_record(m, sup, "H-2", "bar, tip one-way +; S: F = 512 N; O: F, dT = -16 K", o, s, held(m, sup, o, sc), oc, sc,
                    "R (changed: tensile held reaction)"))

    m = bar()
    sup = Support(3, "friction", mu=Fr(1, 4), normal=("explicit", 4096))
    sc = bar_case(m, fx=[2048])
    s = own(m, sup, sc)
    for name, dt, note in (("H-3", -24, "R (changed: stick capacity)"), ("H-4", -8, "E or N (kept)"),
                           ("H-5", -64, "R (changed: slip sense)")):
        oc = bar_case(m, fx=[2048], dt=dt)
        o = own(m, sup, oc)
        add(case_record(m, sup, name, f"bar, tip friction, explicit N = 4096, mu = 1/4; S: F = 2048; O: F, dT = {dt} K",
                        o, s, held(m, sup, o, sc), oc, sc, note))

    m = bar(tip_uz=True)
    sup = Support(3, "friction", mu=Fr(1, 4), normal=("derived", 4))
    for name, fs, dt, note in (("H-6", 2048, -80, "R (changed)"), ("H-6b", 3072, -8, "E or N (kept)")):
        oc, sc = bar_case(m, fx=[fs], fz=-8192, dt=dt), bar_case(m, fx=[fs], fz=-4096)
        o, s = own(m, sup, oc), own(m, sup, sc)
        sh = held(m, sup, o, sc)
        frozen = run(m, sup, sc, "sliding", sh["direction"], sh["beta"], frozen_normal=o["R"][4])
        add(case_record(m, sup, name, f"tip-UZ bar, derived normal, mu = 1/4; S: V = 4096 down, F = {fs}; O: V + V' = 4096 down, F, dT = {dt} K",
                        o, s, sh, oc, sc, note,
                        {"frozen_source_normal": {"friction_force": v(frozen["phi"]), "u": v(frozen["u"][3])}}))
    oc, sc = bar_case(m, fx=[2048], fz=4096 - 8192, dt=-8), bar_case(m, fx=[2048], fz=4096)
    o, s = own(m, sup, oc), own(m, sup, sc)
    add(case_record(m, sup, "H-13", "tip-UZ bar, derived; S: U = 4096 up, F = 2048; O: U, V' = 8192 down, F, dT = -8 K",
                    o, s, held(m, sup, o, sc), oc, sc, "R (branch: the target's normal reverses)"))

    # H-7: the V-1 gap bar (closing +, c = 2^-9).
    m = bar()
    sup = Support(3, "gap", sense=1, c=Fr(1, 2 ** 9))
    oc, sc = bar_case(m, fx=[1536], dt=16), bar_case(m, fx=[1536])
    o, s = own(m, sup, oc), own(m, sup, sc)
    add(case_record(m, sup, "H-7", "V-1 bar, gap closing +, c = 2^-9; S: F = 1536; O: F, dT = +16 K", o, s,
                    held(m, sup, o, sc), oc, sc, "G and R if the source is strict-gap Qualified; otherwise SOURCE_UNQUALIFIED"))

    # H-7g: NI p_tests gap_chain(1) as a product model: a unit cantilever, EA = EI = 100, tip gap closing in -, c = 2^-12.
    fr = Frame([(0, 0), (1, 0)])
    fr.element(0, 1, Fr(100), Fr(100))
    m7 = {"frame": fr, "base": {0: 0, 1: 0, 2: 0, 3: 0}, "axial": None}
    sup = Support(4, "gap", sense=-1, c=Fr(1, 2 ** 12))

    def cant(loads):
        f = [Fr(0)] * fr.n
        for p in loads:
            f[4] -= Fr(p)
        return {"force": f, "ground": Fr(0), "eps": {}}

    oc, sc = cant([Fr(1, 64), 1]), cant([Fr(1, 64)])
    o, s = own(m7, sup, oc), own(m7, sup, sc)
    add(case_record(m7, sup, "H-7g", "unit cantilever, E = 100, A = 1, I = 1, tip gap closing -, c = 2^-12; S: 2^-6 N toward the gap; O: S, +1 N",
                    o, s, held(m7, sup, o, sc), oc, sc, "G and R (the product witness for G)"))

    # H-14 / H-14r: the strongly coupled L-frame.
    fl = Frame([(0, 0), (1, 0), (1, -1)])
    fl.element(0, 1, Fr(2 ** 24), Fr(2 ** 20))
    fl.element(1, 2, Fr(2 ** 24), Fr(2 ** 20))
    ml = {"frame": fl, "base": {0: 0, 1: 0, 2: 0, 7: 0}, "axial": None}
    sup = Support(6, "friction", mu=Fr(1, 4), normal=("derived", 7))
    f = [Fr(0)] * fl.n
    f[6] = Fr(1)
    g = fl.solve(dict(ml["base"]), f)[1][7]

    def lcase(ps, dt=0):
        ff = fl.thermal({0: eps(dt)}) if dt else [Fr(0)] * fl.n
        ff[4] -= Fr(4096)
        for p in ps:
            ff[6] += Fr(p)
        return {"force": ff, "ground": Fr(0), "eps": {0: eps(dt)}}

    for name, oc, note in (("H-14", lcase([2048], 64), "E or N (kept; k = 1)"), ("H-14r", lcase([2048, -4096]), "R (changed: slip sense)")):
        sc = lcase([2048])
        o, s = own(ml, sup, oc), own(ml, sup, sc)
        sh = held(ml, sup, o, sc)
        frozen = run(ml, sup, sc, "sliding", sh["direction"], sh["beta"], frozen_normal=o["R"][7])
        base_r = fl.solve(dict(ml["base"]), sc["force"])[1][7]
        unc_phi = -sh["direction"] * Fr(1, 4) * sh["beta"] * base_r
        unc = fl.solve(dict(ml["base"]), sc["force"], {6: unc_phi})
        rec = case_record(ml, sup, name, "L-frame A(0,0) B(1,0) C(1,-1), EA = 2^24, EI = 2^20; C: UZ restraint (normal), UX friction mu = 1/4; "
                          "S: W = 4096 down at B, P = 2048 +X at C; O: " + ("S, dT = +64 K on AB" if name == "H-14" else "S, P' = -4096 at C"),
                          o, s, sh, oc, sc, note,
                          {"frozen_source_normal": {"friction_force": v(frozen["phi"]), "u": v(frozen["u"][6])},
                           "uncoupled_normal": {"friction_force": v(unc_phi), "u": v(unc[0][6])}})[1]
        rec["coupling_g"] = v(g)
        cases[name] = rec

    # H-15a/b/c: mu = 0 and a positive mu with a zero binary64 limit.
    m = bar()
    for name, mu, n_exp, oc_kw, sc_kw, note in (
            ("H-15a", Fr(0), 4096, dict(fx=[512], dt=-16), dict(fx=[512]), "E or N (kept: zero limit, slip)"),
            ("H-15b", Fr(0), 4096, dict(f_anchor=512, dt=-16), dict(f_anchor=512), "R (changed: no slip)"),
            ("H-15c", Fr(2) ** -1074, Fr(1, 4), dict(fx=[512], dt=-16), dict(fx=[512]), "E or N (kept: fl(mu N) = 0)")):
        sup = Support(3, "friction", mu=mu, normal=("explicit", n_exp))
        oc, sc = bar_case(m, **oc_kw), bar_case(m, **sc_kw)
        # H-15c: the binary64 product mu*N = 2^-1076 rounds to +0, so the step forms no force and the classifier's
        # zero-limit rule applies; own states are those of the system the binary64 step solves (mu*N = 0).
        if name == "H-15c":
            assert float(mu) * float(n_exp) == 0.0 and float(mu) > 0.0
            sup.kw["mu"] = Fr(0)
            o, s = own(m, sup, oc), own(m, sup, sc)
            sup.kw["mu"] = mu
        else:
            o, s = own(m, sup, oc), own(m, sup, sc)
        sh = held(m, sup, o, sc)
        cases[name] = case_record(m, sup, name, f"bar, explicit N = {n_exp}, mu = {mu if name != 'H-15c' else '2^-1074'}", o, s, sh, oc, sc, note)[1]

    # H-16a/b: inactive friction.
    sup = Support(3, "friction", mu=Fr(1, 4), normal=("explicit", 0))
    oc, sc = bar_case(m, fx=[512], dt=-16), bar_case(m, fx=[512])
    o, s = own(m, sup, oc), own(m, sup, sc)
    add(case_record(m, sup, "H-16a", "bar, explicit N = 0 (an input decision), mu = 1/4", o, s, held(m, sup, o, sc), oc, sc,
                    "E (input decision)"))
    mz = bar(tip_uz=True)
    sup = Support(3, "friction", mu=Fr(1, 4), normal=("derived", 4))
    oc, sc = bar_case(mz, fx=[512], fz=-4096 + 4096), bar_case(mz, fx=[512], fz=-4096)
    o, s = own(mz, sup, oc), own(mz, sup, sc)
    add(case_record(mz, sup, "H-16b", "tip-UZ bar, derived; S: V = 4096 down, F = 512; O: V, U = 4096 up, F (R_n = 0)",
                    o, s, held(mz, sup, o, sc), oc, sc, "R (changed)"))

    # H-17 / H-17c: sticking with a moved ground (callable level, and the combination-level twin).
    sup = Support(3, "friction", mu=Fr(1, 4), normal=("explicit", 4096))
    oc = bar_case(m, fx=[768, 1280], ground=Fr(3, 2 ** 11))
    o = own(m, sup, oc)
    for name, g_t, note in (("H-17", Fr(1, 2 ** 11), "E or N (callable level: no G-1)"), ("H-17c", Fr(0), "E or N")):
        sc = bar_case(m, fx=[768], ground=g_t)
        s = own(m, sup, sc)
        sh = held(m, sup, o, sc)
        mut = run(m, sup, {"force": sc["force"], "ground": o["u"][3], "eps": {}}, "sticking")
        add(case_record(m, sup, name, f"bar, explicit N = 4096; source: F = 768 + 1280, shoe ground 3*2^-11; target: F = 768, ground {g_t}",
                        o, s, sh, oc, sc, note, {"hold_source_displacement": {"u": v(mut["u"][3]), "reaction": v(mut["R"][3])}}))

    # H-18: weak contact (zero separation, kept; its separation ball contains 0, so N).
    sup = Support(3, "one_way", sense=1)
    oc, sc = bar_case(m, f_anchor=512, dt=16), bar_case(m, f_anchor=512)
    o, s = own(m, sup, oc), own(m, sup, sc)
    add(case_record(m, sup, "H-18", "bar, one-way +; S: 512 N at the anchor only; O: S, dT = +16 K", o, s,
                    held(m, sup, o, sc), oc, sc, "N (separation not established)"))
    return {"document_kind": "openpipestress.op_mf_held_status.references", "units": "N, m, rad; H-1/H-1b in P, PL and s*",
            "cases": cases}


def main():
    out = json.dumps(build(), indent=1, sort_keys=True) + "\n"
    if "--write" in sys.argv:
        with open(OUT, "w") as fh:
            fh.write(out)
        print("written", OUT)
        return
    with open(OUT) as fh:
        if fh.read() != out:
            raise SystemExit("references.json differs from the generator's output")
    print("references.json matches the generator")


if __name__ == "__main__":
    main()
