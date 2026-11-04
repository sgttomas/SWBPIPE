"""DEL-04-09 I3a: a test-only feasibility oracle for the modal readers (Design §5.3.4,
the dependency table).

For each relation instance of a modal document, the oracle decides by search whether
ANY assignment of values within its operands' stated sets satisfies the relation. It
uses no reader code. It reads the document's JSON directly, and each operand's set comes
from its value object:

- a published finite number is that number;
- `below_normal` is a nonzero subnormal: magnitude in [2⁻¹⁰⁷⁴, 2⁻¹⁰²² − 2⁻¹⁰⁷⁴];
- `outside_canonical_profile` is finite with magnitude in [2⁵³, largest finite];
- `not_finite` is ±∞ or NaN (+∞ or NaN for a nonnegative kind);
- every other reason is a claim, checked by the relation that consumes it.

The search tries candidate points from each set (its endpoints, interior points and
its non-finite values). For each unknown operand it also SOLVES the relation for that
operand, given the others at their candidates, clamps the solution into the operand's
set, and evaluates the relation's predicate exactly (rational arithmetic, with IEEE
evaluation only for non-finite operands). That is complete for the relation shapes here:
sums, products, quotients written as products, a maximum and threshold comparisons,
each monotone on every piece of a set.

A family is infeasible when any of its instances is; a reader must fail exactly those
families. All values are invented.
"""
from __future__ import annotations

import itertools
import math
from fractions import Fraction

U = Fraction(1, 2 ** 53)
ALLOW = 16
MAX = Fraction(1.7976931348623157e308)
FLOOR = Fraction(2 ** 53)
SUB_MIN = Fraction(1, 2 ** 1074)
SUB_MAX = Fraction(1, 2 ** 1022) - SUB_MIN
INF, NAN = float("inf"), float("nan")
DIRECTIONS = ("x", "y", "z")
FACTORED = ("factor_term", "backward_error", "inverse_norm_estimate", "conditioning_product", "separation_ratio", "growth",
            "max_abs_lower_factor", "max_abs_pivot", "max_abs_scaled_diagonal")
FAMILIES = ("mass", "participation", "clusters", "sturm", "evidence", "standing")


class Domain:
    """Pieces (closed, rational), non-finite values, or a claim."""

    def __init__(self, pieces=(), specials=(), claim=None):
        self.pieces, self.specials, self.claim = list(pieces), list(specials), claim

    def point(self):
        if not self.specials and self.claim is None and len(self.pieces) == 1 and self.pieces[0][0] == self.pieces[0][1]:
            return self.pieces[0][0]
        return None


def domain(vo, signed=False):
    def both(lo, hi):
        return [(lo, hi), (-hi, -lo)] if signed else [(lo, hi)]
    if vo["value"] is not None:
        if vo["below_normal"]:
            return Domain(both(SUB_MIN, SUB_MAX))
        v = Fraction(vo["value"])
        return Domain([(v, v)])
    reason = vo["undefined_reason"]
    if reason == "outside_canonical_profile":
        return Domain(both(FLOOR, MAX))
    if reason == "not_finite":
        return Domain((), [INF, NAN] + ([-INF] if signed else []))
    return Domain(claim=reason)


def special(x):
    return isinstance(x, float) and not math.isfinite(x)


def candidates(d):
    out = []
    for lo, hi in d.pieces:
        out.append(lo)
        if lo != hi:
            out.append(hi)
            out.append((lo + hi) / 2)
            if lo > 0:
                out.append(Fraction(2.0 ** ((math.log2(lo) + math.log2(hi)) / 2)))
            if hi < 0:
                out.append(-Fraction(2.0 ** ((math.log2(-hi) + math.log2(-lo)) / 2)))
    return out + list(d.specials)


def clamp(x, d):
    return [min(max(x, lo), hi) for lo, hi in d.pieces]


def satisfiable(operands, predicate, solvers=None):
    """operands: {name: Domain}. Whether some assignment satisfies `predicate`; a claim
    operand makes the instance infeasible (the claim states no value)."""
    if any(d.claim is not None for d in operands.values()):
        return False
    cands = {n: candidates(d) for n, d in operands.items()}
    unknown = [n for n in operands if len(cands[n]) > 1]
    fixed = {n: cands[n][0] for n in operands if n not in unknown}
    for combo in itertools.product(*(cands[n] for n in unknown)):
        if predicate({**fixed, **dict(zip(unknown, combo))}):
            return True
    for v in unknown:
        solve = (solvers or {}).get(v)
        if solve is None:
            continue
        others = [n for n in unknown if n != v]
        for combo in itertools.product(*(cands[n] for n in others)):
            a = {**fixed, **dict(zip(others, combo))}
            if any(special(x) for x in a.values()):
                continue
            for target in solve(a):
                for x in clamp(target, operands[v]):
                    if predicate({**a, v: x}):
                        return True
    return False


def _float(x):
    return x if isinstance(x, float) else float(x)


def formula(published, inputs, combine, tolerance):
    """A value the producer forms by + and ·: non-finite exactly when an input is or its
    binary64 result overflows; else within `tolerance` of the exact result."""
    if any(special(x) for x in inputs):
        return special(published)
    try:
        rounded = combine([_float(x) for x in inputs])
    except OverflowError:
        rounded = INF
    if not math.isfinite(rounded):
        return special(published)
    if special(published):
        return False
    exact = combine(inputs)
    return abs(published - exact) <= tolerance(exact, published)


def quotient(values, holds):
    """A relation the producer forms through a quotient: a non-finite operand leaves it
    open; otherwise its exact predicate."""
    if any(special(x) for x in values):
        return True
    return holds()


def _sqrt(x):
    """√x as a rational with about 64 significant bits, for any positive rational."""
    if x <= 0:
        return Fraction(0)
    shift = max(0, 128 - (x.numerator.bit_length() - x.denominator.bit_length()))
    shift += shift % 2
    return Fraction(math.isqrt((x.numerator << shift) // x.denominator), 1 << (shift // 2))


# --- The document --------------------------------------------------------------------------

class Doc:
    def __init__(self, doc):
        t = doc["tables"]
        self.doc, self.t = doc, t
        dofs = t["dofs"]["rows"]
        self.dof_index = {r["row_id"]: i for i, r in enumerate(dofs)}
        self.free = [r["free_index"] is not None for r in dofs]
        mm = t["mass_matrix"]
        self.mass = None if mm.get("status") == "withheld" else [
            (self.dof_index[r["row_dof_ref"]], self.dof_index[r["col_dof_ref"]], r["value"]) for r in mm["rows"]]
        self.totals = {r["direction"]: r for r in t["mass_totals"]["rows"]}
        self.modes = t["modes"]["rows"]
        sh = t["shapes"]
        n, ndof = len(self.modes), len(dofs)
        self.shapes = None if sh.get("status") == "withheld" else [
            [sh["rows"][m * ndof + d]["value"] for d in range(ndof)] for m in range(n)]
        self.participation = [[t["participation"]["rows"][3 * m + k] for k in range(3)] for m in range(n)]
        self.computed = t["computed_modes"]["rows"]
        self.clusters = t["clusters"]["rows"]
        self.candidates = t["sturm_candidates"]["rows"]
        self.identity = t["identity"]["rows"]
        self.params = doc["binding"]["integrity_parameters"]
        s = doc["sturm"]
        self.count, self.limit = s["count"], Fraction(s["conditioning_limit"]["value"])
        self.ks, self.g_max = Fraction(s["separation_multiple"]["value"]), Fraction(s["growth_limit"]["value"])
        pl = s["placement"]
        self.placement = pl["kind"]
        self.lower = Fraction(pl["lower"]["value"])
        self.upper = None if pl["upper"] is None else Fraction(pl["upper"]["value"])
        self.ortho = doc["orthogonality"]["max_ratio"]


def _cluster_of(ref):
    return int(ref.split(":")[1])


# --- The families ----------------------------------------------------------------------------

def mass_family(d):
    n = len(d.modes)
    for k in DIRECTIONS:
        for key in ("total_mass", "free_dof_mass"):
            vo = d.totals[k][key]
            if domain(vo).claim is not None or (vo["value"] is not None and vo["value"] < 0):
                return False
    for k in range(3):
        operands = {"T": domain(d.totals[DIRECTIONS[k]]["total_mass"])}
        for m in range(n):
            dm = domain(d.participation[m][k]["effective_mass"])
            if dm.claim is None:
                operands[f"e{m}"] = dm

        def bessel(a, k=k):
            values = list(a.values())
            if any(special(x) for x in values):
                return True
            return sum(a[f"e{m}"] for m in range(n) if f"e{m}" in a) <= a["T"] * (1 + ALLOW * (n + 2) * U)
        if not satisfiable(operands, bessel):
            return False
    if d.mass is None:
        return True
    for i, j, vo in d.mass:
        if domain(vo, signed=i != j).claim is not None or (i == j and vo["value"] is not None and vo["value"] < 0):
            return False
    for k in range(3):
        for key, free_only in (("total_mass", False), ("free_dof_mass", True)):
            block = [(Fraction(1 if i == j else 2), vo, i, j) for i, j, vo in d.mass
                     if i % 6 == k and j % 6 == k and not (free_only and not (d.free[i] and d.free[j]))]
            if not _sum_relation(d.totals[DIRECTIONS[k]][key], block, len(block)):
                return False
    return True


def _sum_relation(published_vo, terms, count):
    """published ≈ Σ c·x within A(count+4)u·Σ|c·x|; terms: (c, vo, i, j)."""
    operands = {"P": domain(published_vo)}
    for t, (c, vo, i, j) in enumerate(terms):
        operands[f"x{t}"] = domain(vo, signed=i != j)
    allowance = ALLOW * (count + 4) * U

    def holds(a):
        xs = [a[f"x{t}"] for t in range(len(terms))]
        return formula(a["P"], xs, lambda v: sum(c * x for (c, *_), x in zip(terms, v)),
                       lambda exact, p: allowance * sum(abs(c * x) for (c, *_), x in zip(terms, xs)))

    def solve_x(t):
        def s(a):
            rest = sum(c * a[f"x{u}"] for u, (c, *_) in enumerate(terms) if u != t)
            return [(a["P"] - rest) / terms[t][0]]
        return s
    solvers = {f"x{t}": solve_x(t) for t in range(len(terms))}
    solvers["P"] = lambda a: [sum(c * a[f"x{t}"] for t, (c, *_) in enumerate(terms))]
    return satisfiable(operands, holds, solvers)


def participation_family(d):
    n = len(d.modes)
    for m, mode in enumerate(d.modes):
        gm = mode["generalized_mass"]
        if domain(gm).claim is not None or (gm["value"] is not None and not gm["below_normal"] and gm["value"] <= 0):
            return False
        for k in range(3):
            row, total = d.participation[m][k], d.totals[DIRECTIONS[k]]["total_mass"]
            g, e = row["participation_factor"], row["effective_mass"]
            if domain(g, True).claim is not None or domain(e).claim is not None or domain(total).claim is not None:
                return False
            if e["value"] is not None and e["value"] < 0:
                return False
            tol = ALLOW * U
            ops = {"g": domain(g, True), "m": domain(gm), "e": domain(e)}

            def g2m(a):
                return quotient(a.values(), lambda: abs(a["g"] ** 2 * a["m"] - a["e"]) <= tol * max(abs(a["g"] ** 2 * a["m"]), abs(a["e"])))
            if not satisfiable(ops, g2m, {"e": lambda a: [a["g"] ** 2 * a["m"]],
                                          "m": lambda a: [a["e"] / a["g"] ** 2] if a["g"] != 0 else [],
                                          "g": lambda a: [_sqrt(a["e"] / a["m"]), -_sqrt(a["e"] / a["m"])] if a["m"] != 0 else []}):
                return False
            f, c = row["effective_mass_fraction"], row["cumulative_effective_mass_fraction"]
            zero = [vo["undefined_reason"] == "total_mass_zero" for vo in (f, c)]
            dt = domain(total)
            if any(zero):
                if not all(zero) or dt.point() != 0 or domain(e).point() != 0:
                    return False
                continue
            if dt.point() == 0 or domain(f).claim is not None or (f["value"] is not None and f["value"] < 0):
                return False
            ops = {"f": domain(f), "T": dt, "e": domain(e)}

            def ft(a):
                return quotient(a.values(), lambda: abs(a["f"] * a["T"] - a["e"]) <= tol * max(abs(a["f"] * a["T"]), abs(a["e"])))
            if not satisfiable(ops, ft, {"e": lambda a: [a["f"] * a["T"]],
                                         "f": lambda a: [a["e"] / a["T"]] if a["T"] != 0 else [],
                                         "T": lambda a: [a["e"] / a["f"]] if a["f"] != 0 else []}):
                return False
    for k in range(3):
        for m in range(n):
            rows = [d.participation[j][k] for j in range(m + 1)]
            if any(r["effective_mass_fraction"]["undefined_reason"] == "total_mass_zero" for r in rows):
                break
            c = rows[-1]["cumulative_effective_mass_fraction"]
            if domain(c).claim is not None or (c["value"] is not None and c["value"] < 0):
                return False
            terms = [(Fraction(1), r["effective_mass_fraction"], 0, 0) for r in rows]
            allowance = ALLOW * (m + 2) * U
            operands = {"P": domain(c), **{f"x{t}": domain(vo) for t, (_, vo, _, _) in enumerate(terms)}}

            def holds(a, terms=terms, allowance=allowance):
                xs = [a[f"x{t}"] for t in range(len(terms))]
                return formula(a["P"], xs, sum, lambda exact, p: allowance * abs(exact))
            solvers = {"P": lambda a, terms=terms: [sum(a[f"x{t}"] for t in range(len(terms)))]}
            for t in range(len(terms)):
                solvers[f"x{t}"] = lambda a, t=t, terms=terms: [a["P"] - sum(a[f"x{u}"] for u in range(len(terms)) if u != t)]
            if not satisfiable(operands, holds, solvers):
                return False
    if d.mass is None or d.shapes is None:
        return True
    for m, mode in enumerate(d.modes):
        phi = [Fraction(vo["value"]) for vo in d.shapes[m]]
        assert not any(vo["below_normal"] for vo in d.shapes[m]), "the oracle's fixtures have no below-normal shape"
        pairs = [(i, j, vo) for i, j, vo in d.mass if d.free[i] and d.free[j]]
        terms = [(Fraction(1 if i == j else 2) * phi[i] * phi[j], vo, i, j) for i, j, vo in pairs]
        if not _sum_relation(mode["generalized_mass"], terms, len(pairs)):
            return False
        for k in range(3):
            terms = []
            for i, j, vo in d.mass:
                if d.free[i] and j % 6 == k:
                    terms.append((phi[i], vo, i, j))
                if i != j and d.free[j] and i % 6 == k:
                    terms.append((phi[j], vo, i, j))
            row = d.participation[m][k]
            if not _excitation(mode["generalized_mass"], row["participation_factor"], row["effective_mass"], terms):
                return False
    return True


def _excitation(gm, g, e, terms):
    """Γ·m ≈ s and m_eff·m ≈ s², s = Σ c·x (the payload's φᵀp_k), each a quotient
    relation: e = A(n+6)u, a = Σ|c·x|."""
    eps = ALLOW * (len(terms) + 6) * U
    base = {"m": domain(gm), **{f"x{t}": domain(vo, signed=i != j) for t, (_, vo, i, j) in enumerate(terms)}}

    def s_of(a):
        return sum(c * a[f"x{t}"] for t, (c, *_) in enumerate(terms)), sum(abs(c * a[f"x{t}"]) for t, (c, *_) in enumerate(terms))

    def solve_x(target_fn):
        out = {}
        for t in range(len(terms)):
            def s(a, t=t):
                if terms[t][0] == 0:
                    return []
                rest = sum(c * a[f"x{u}"] for u, (c, *_) in enumerate(terms) if u != t)
                return [(target - rest) / terms[t][0] for target in target_fn(a)]
            out[f"x{t}"] = s
        return out

    def gamma(a):
        def holds():
            s, mag = s_of(a)
            gm_ = a["g"] * a["m"]
            return abs(gm_ - s) <= eps * (mag + abs(gm_))
        return quotient(a.values(), holds)
    solvers = solve_x(lambda a: [a["g"] * a["m"]])
    solvers["g"] = lambda a: [s_of(a)[0] / a["m"]] if a["m"] != 0 else []
    solvers["m"] = lambda a: [s_of(a)[0] / a["g"]] if a["g"] != 0 else []
    if not satisfiable({**base, "g": domain(g, True)}, gamma, solvers):
        return False

    def effective(a):
        def holds():
            s, mag = s_of(a)
            em = a["e"] * a["m"]
            return abs(em - s * s) <= eps * (2 * abs(s) * mag + abs(em)) + (eps * mag) ** 2
        return quotient(a.values(), holds)
    solvers = solve_x(lambda a: [_sqrt(a["e"] * a["m"]), -_sqrt(a["e"] * a["m"])])
    solvers["e"] = lambda a: [s_of(a)[0] ** 2 / a["m"]] if a["m"] != 0 else []
    solvers["m"] = lambda a: [s_of(a)[0] ** 2 / a["e"]] if a["e"] != 0 else []
    return satisfiable({**base, "e": domain(e)}, effective, solvers)


def clusters_family(d):
    for c, row in enumerate(d.clusters):
        cid = c + 1
        members = [r for r in d.computed if _cluster_of(r["cluster_ref"]) == cid]
        operands = {"C": domain(row["error_estimate"]), **{f"x{t}": domain(r["error_estimate"]) for t, r in enumerate(members)}}

        def holds(a, n=len(members)):
            xs = [a[f"x{t}"] for t in range(n)]
            if any(special(x) for x in xs):
                return special(a["C"])
            return not special(a["C"]) and a["C"] == max(xs)
        solvers = {"C": lambda a, n=len(members): [max(a[f"x{t}"] for t in range(n))]}
        for t in range(len(members)):
            solvers[f"x{t}"] = lambda a: [a["C"]]
        if not satisfiable(operands, holds, solvers):
            return False
    for mode in d.modes:
        own = d.clusters[_cluster_of(mode["cluster_ref"]) - 1]["error_estimate"]
        mine = mode["cluster_error_estimate"]
        if any(mine[k] != own[k] for k in ("value", "below_normal", "undefined_reason")):
            return False
    return True


def _truths(d, test):
    """The truth values a producer comparison can take over a domain's candidates."""
    return {t for x in candidates(d) for t in test(x)}


def sturm_family(d):
    ks, g_max, limit = d.ks, d.g_max, d.limit
    for index, c in enumerate(d.candidates):
        if c["negative_pivot_count"] is None:
            if any(c[f]["value"] is not None or c[f]["undefined_reason"] != "not_factored" for f in FACTORED):
                return False
            if c["decision"] in ("accepted", "growth_limit", "ill_conditioned"):
                return False
            continue
        doms = {f: domain(c[f]) for f in ("formation_term",) + FACTORED}
        if any(x.claim is not None for x in doms.values()):
            return False
        for out, ins, combine in (("backward_error", ("factor_term", "formation_term"), lambda v: v[0] + v[1]),
                                  ("conditioning_product", ("backward_error", "inverse_norm_estimate"), lambda v: v[0] * v[1])):
            operands = {"P": doms[out], "a": doms[ins[0]], "b": doms[ins[1]]}
            published = all(x.point() is not None for x in operands.values())

            def holds(a, combine=combine, published=published):
                if published:
                    return _float(a["P"]) == combine([_float(a["a"]), _float(a["b"])])
                return formula(a["P"], [a["a"], a["b"]], combine, lambda exact, p: 8 * U * abs(exact))
            solvers = {"P": lambda a, combine=combine: [combine([a["a"], a["b"]])]}
            if out == "backward_error":
                solvers["a"] = lambda a: [a["P"] - a["b"]]
                solvers["b"] = lambda a: [a["P"] - a["a"]]
            else:
                solvers["a"] = lambda a: [a["P"] / a["b"]] if a["b"] != 0 else []
                solvers["b"] = lambda a: [a["P"] / a["a"]] if a["a"] != 0 else []
            if not satisfiable(operands, holds, solvers):
                return False
        separated = _truths(doms["separation_ratio"], lambda r: {True} if special(r) else
                            ({True} if r >= ks * (1 - 8 * U) else set()) | ({False} if r < ks * (1 + 8 * U) else set()))
        within = _truths(doms["growth"], lambda g: {not special(g) and g <= g_max})
        conditioned = _truths(doms["conditioning_product"], lambda x: {not special(x) and x <= limit})
        decision = c["decision"]
        if decision == "accepted":
            if c["negative_pivot_count"] != d.count or True not in separated or True not in within or True not in conditioned:
                return False
        elif decision == "growth_limit" and (True not in separated or False not in within):
            return False
        elif decision == "ill_conditioned" and (True not in separated or True not in within or False not in conditioned):
            return False
        elif decision == "unseparated" and False not in separated:
            return False
    shift = domain(d.candidates[-1]["shift"])
    if shift.claim is not None or not shift.pieces:
        return False
    top = d.upper if d.placement == "gap" else MAX
    if not any(lo <= top and d.lower <= hi for lo, hi in shift.pieces):
        return False
    return True


def evidence_family(d):
    for c in d.computed:
        vos = (c["backward_error"], c["condition"], c["error_estimate"])
        doms = [domain(vo) for vo in vos]
        if any(x.claim is not None for x in doms) or any(vo["value"] is not None and vo["value"] < 0 for vo in vos):
            return False
        operands = {"eta": doms[0], "kappa": doms[1], "e": doms[2]}

        def holds(a):
            return formula(a["e"], [a["eta"], a["kappa"]], lambda v: v[0] * v[1],
                           lambda exact, p: 8 * U * max(abs(exact), abs(p)))
        if not satisfiable(operands, holds, {"e": lambda a: [a["eta"] * a["kappa"]],
                                             "eta": lambda a: [a["e"] / a["kappa"]] if a["kappa"] != 0 else [],
                                             "kappa": lambda a: [a["e"] / a["eta"]] if a["eta"] != 0 else []}):
            return False
    tolerance = d.params["identity_tolerance"]
    if tolerance["absolute"] == 0.0:
        rel = Fraction(tolerance["relative"])
        for row in d.identity:
            vo = row["relative"]
            x = domain(vo)
            if x.claim is not None or (vo["value"] is not None and vo["value"] < 0):
                return False
            if not satisfiable({"d": x}, lambda a: special(a["d"]) or a["d"] <= rel):
                return False
    o = domain(d.ortho)
    if o.claim is not None or (d.ortho["value"] is not None and d.ortho["value"] < 0):
        return False
    return True


def standing_family(d):
    tau_r, tau_o = Fraction(d.params["residual_threshold"]), Fraction(d.params["orthogonality_threshold"])

    def above(x, threshold):
        # The producer's η > τ_r: true for +∞, false for NaN.
        return {x > 0} if special(x) and not math.isnan(x) else ({False} if special(x) else {x > threshold})
    for m, mode in enumerate(d.modes):
        eta = domain(d.computed[m]["backward_error"])
        if eta.claim is not None:
            return False
        if (mode["standing"]["token"] == "sensitive") not in _truths(eta, lambda x: above(x, tau_r)):
            return False
    o = domain(d.ortho)
    if o.claim is not None:
        return False
    if any(mode["standing"]["token"] == "sensitive" for mode in d.modes):
        possible = {True}
    else:
        possible = _truths(o, lambda x: above(x, tau_o))
    return (d.doc["standing"]["token"] == "sensitive") in possible


def verdict(doc):
    """{family: "feasible" | "infeasible"} for a solved document."""
    d = Doc(doc)
    checks = {"mass": mass_family, "participation": participation_family, "clusters": clusters_family,
              "sturm": sturm_family, "evidence": evidence_family, "standing": standing_family}
    return {name: "feasible" if check(d) else "infeasible" for name, check in checks.items()}
