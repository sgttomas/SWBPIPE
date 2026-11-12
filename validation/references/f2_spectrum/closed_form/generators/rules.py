"""Spectrum table evaluation and the combination rules, written from DEL-04-10 CLM-001.

Table evaluation
  A table is a list of (abscissa, ordinate) points with strictly increasing
  abscissae. The abscissa is frequency (Hz) or period (s); the ordinate is a
  pseudo-spectral acceleration in m/s^2 or a multiple of a user-entered g.
  The rule is linear or logarithmic on each axis. A query is a frequency f.

  Reading "tabulated axis" (the primary reading): the interpolation runs on
  the table's own abscissa, so a period table is interpolated in T = 1/f.
  Reading "frequency axis" (published only where it differs, that is for a
  period table with a linear abscissa rule): the tabulated periods are first
  converted to frequencies 1/T and the interpolation runs in f.

  On segment [x0, x1] with ordinates y0, y1 the parameter is
    t = (x - x0)/(x1 - x0)          (linear abscissa)
    t = ln(x/x0)/ln(x1/x0)          (logarithmic abscissa)
  and the ordinate is
    S = y0 + (y1 - y0) t            (linear ordinate)
    S = y0 exp(t ln(y1/y0))         (logarithmic ordinate).
  A query outside [x_first, x_last] is refused (no extrapolation). A query at
  a tabulated abscissa returns the tabulated ordinate exactly. With linear
  rules on both axes and a rational query the value is an exact rational.

Combination rules (each applied to one response quantity's signed values)
  SRSS      sqrt(sum R_i^2)
  CQC       sqrt(sum_i sum_j rho_ij R_i R_j),
            rho_ij = 8 z^2 (1 + r) r^(3/2) / ((1 - r^2)^2 + 4 z^2 r (1 + r)^2), r = w_j/w_i
  GROUPING  modes in ascending order form groups in which each frequency is
            within 10 % of the group's lowest ((f - f_low) <= f_low/10);
            sqrt(sum R_i^2 + 2 sum_groups sum_{i<j in group} |R_i R_j|)
  TEN_PERCENT  the ten percent method: sqrt(sum R_i^2 + 2 sum |R_i R_j|), the
            second sum over every pair i < j (ascending frequency) with
            (f_j - f_i)/f_i <= 1/10, whether or not the two fall in one group
  ABSSUM    the absolute sum, sum |R_i|
  These are the five modal rules of DEL-04-10 CLM-001. The rules are applied to
  the values given; where a cluster of repeated modes is summed first (CLM-001,
  Repeated modes) the caller passes the cluster sums.
"""

from decimal import Decimal
from fractions import Fraction

from numerics import dec, dexp, dln, dsqrt

TENTH = Fraction(1, 10)


# --------------------------------------------------------------------------
# Tables


def table_points(table, reading="tabulated"):
    """(xs, ys, kind) with xs increasing, as Fractions, ordinate in m/s^2 units unscaled."""
    xs = [Fraction(v) for v in table["abscissae"]]
    ys = [Fraction(v) for v in table["ordinates"]]
    if table["abscissa"] == "period" and reading == "frequency":
        xs = [1 / x for x in reversed(xs)]
        ys = list(reversed(ys))
        return xs, ys, "frequency"
    return xs, ys, table["abscissa"]


def _to_x(f, kind):
    return (1 / f) if kind == "period" else f


def _segment(xs, x):
    if x < xs[0] or x > xs[-1]:
        return None
    for i in range(len(xs) - 1):
        if xs[i] <= x <= xs[i + 1]:
            return i
    return None


def _g_factor(table):
    return Fraction(table["g"]) if table["ordinate"] == "g" else Fraction(1)


def evaluate(table, f, reading="tabulated", alt=False):
    """S(f) in m/s^2 (unscaled), as Fraction when exact, Decimal otherwise; None if refused.

    alt=True evaluates the same rule from the segment's other end (a second,
    differently arranged computation used as a cross-check).
    """
    xs, ys, kind = table_points(table, reading)
    xr, yr = table["rule"]
    exact = isinstance(f, Fraction)
    x = _to_x(f, kind)
    i = _segment(xs, x)
    if i is None:
        return None
    gf = _g_factor(table)
    for k in (i, i + 1):
        if x == xs[k]:
            return ys[k] * gf
    x0, x1, y0, y1 = xs[i], xs[i + 1], ys[i], ys[i + 1]
    if xr == "linear" and yr == "linear" and exact:
        if alt:
            return (y1 - (y1 - y0) * (x1 - x) / (x1 - x0)) * gf
        return (y0 + (y1 - y0) * (x - x0) / (x1 - x0)) * gf
    X, X0, X1, Y0, Y1 = (dec(v) if not isinstance(v, Decimal) else v for v in (x, x0, x1, y0, y1))
    if xr == "linear":
        t = (X1 - X) / (X1 - X0) if alt else (X - X0) / (X1 - X0)
    else:
        t = dln(X1 / X) / dln(X1 / X0) if alt else dln(X / X0) / dln(X1 / X0)
    if yr == "linear":
        S = Y1 - (Y1 - Y0) * t if alt else Y0 + (Y1 - Y0) * t
    else:
        S = Y1 * dexp(-t * dln(Y1 / Y0)) if alt else Y0 * dexp(t * dln(Y1 / Y0))
    return S * dec(gf)


def dlnS_dlnf(table, f, reading="tabulated"):
    """d ln S / d ln f inside a segment (None at a tabulated abscissa or outside)."""
    xs, ys, kind = table_points(table, reading)
    xr, yr = table["rule"]
    x = _to_x(f, kind)
    i = _segment(xs, x)
    if i is None or x in (xs[i], xs[i + 1]):
        return None
    X, X0, X1, Y0, Y1 = (dec(v) if not isinstance(v, Decimal) else v for v in (x, xs[i], xs[i + 1], ys[i], ys[i + 1]))
    S = evaluate(table, f, reading)
    S = dec(S) if not isinstance(S, Decimal) else S
    S = S / dec(_g_factor(table))
    if xr == "linear" and yr == "linear":
        d = X * (Y1 - Y0) / (X1 - X0) / S
    elif xr == "log" and yr == "linear":
        d = (Y1 - Y0) / dln(X1 / X0) / S
    elif xr == "linear" and yr == "log":
        d = X * dln(Y1 / Y0) / (X1 - X0)
    else:
        d = dln(Y1 / Y0) / dln(X1 / X0)
    return -d if kind == "period" else d


def table_doc(table):
    from numerics import inp
    doc = {
        "id": table["id"],
        "abscissa": table["abscissa"] + (" (Hz)" if table["abscissa"] == "frequency" else " (s)"),
        "ordinate": "pseudo-spectral acceleration, " + ("m/s^2" if table["ordinate"] == "acceleration"
                                                       else "multiple of the user-entered g"),
        "abscissae": [inp(v) for v in table["abscissae"]],
        "ordinates": [inp(v) for v in table["ordinates"]],
        "damping_ratio": inp(table["damping"]),
        "interpolation": {"abscissa": table["rule"][0], "ordinate": table["rule"][1]},
    }
    if table["ordinate"] == "g":
        doc["g_m_per_s2"] = inp(table["g"])
    return doc


# --------------------------------------------------------------------------
# Modal combination


def D(x):
    return x if isinstance(x, Decimal) else dec(x)


def srss(vals):
    return dsqrt(sum((D(v) ** 2 for v in vals), Decimal(0)))


def abssum(vals):
    return sum((abs(D(v)) for v in vals), Decimal(0))


def rho(wi, wj, zeta):
    wi, wj, z = D(wi), D(wj), D(zeta)
    r = wj / wi
    num = 8 * z * z * (1 + r) * r * dsqrt(r)
    den = (1 - r * r) ** 2 + 4 * z * z * r * (1 + r) ** 2
    return num / den


def cqc(vals, omegas, zeta):
    n = len(vals)
    s = Decimal(0)
    for i in range(n):
        for j in range(n):
            s += rho(omegas[i], omegas[j], zeta) * D(vals[i]) * D(vals[j])
    if s < 0:
        raise ValueError("negative CQC double sum")
    return dsqrt(s)


def groups(freqs):
    """Index groups under the grouping method's closeness rule (ascending order)."""
    order = sorted(range(len(freqs)), key=lambda i: D(freqs[i]))
    out = []
    cur = []
    low = None
    for i in order:
        f = D(freqs[i])
        if cur and f - low <= low / 10:
            cur.append(i)
        else:
            if cur:
                out.append(cur)
            cur, low = [i], f
    if cur:
        out.append(cur)
    return out


def grouping(vals, freqs):
    s = sum((D(v) ** 2 for v in vals), Decimal(0))
    for g in groups(freqs):
        for a in range(len(g)):
            for b in range(a + 1, len(g)):
                s += 2 * abs(D(vals[g[a]]) * D(vals[g[b]]))
    return dsqrt(s)


def close_pairs(freqs):
    order = sorted(range(len(freqs)), key=lambda i: D(freqs[i]))
    out = []
    for a in range(len(order)):
        for b in range(a + 1, len(order)):
            i, j = order[a], order[b]
            fi, fj = D(freqs[i]), D(freqs[j])
            if (fj - fi) <= fi / 10:
                out.append((i, j))
    return out


def pairwise10(vals, freqs):
    s = sum((D(v) ** 2 for v in vals), Decimal(0))
    for i, j in close_pairs(freqs):
        s += 2 * abs(D(vals[i]) * D(vals[j]))
    return dsqrt(s)


ten_percent = pairwise10

MODAL_RULES = ["SRSS", "CQC", "GROUPING", "ABSSUM", "TEN_PERCENT"]


def combine_all(vals, freqs, omegas, zeta):
    return {
        "SRSS": srss(vals),
        "CQC": cqc(vals, omegas, zeta),
        "GROUPING": grouping(vals, freqs),
        "ABSSUM": abssum(vals),
        "TEN_PERCENT": pairwise10(vals, freqs),
    }


def with_rigid(periodic, rigid):
    p, r = D(periodic), D(rigid)
    return {"SRSS": dsqrt(p * p + r * r), "ABSSUM": abs(p) + abs(r)}
