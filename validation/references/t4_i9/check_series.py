"""Check each generated load series against direct numerical integration (mpmath-free:
composite Gauss-Legendre 10-point in double-double via Decimal 40 digits) at phi = 0.3, 0.8, 1.04."""
import importlib.util, sys
from decimal import Decimal as D, getcontext
getcontext().prec = 45
spec = importlib.util.spec_from_file_location("g", "load_series_gen.py"); g = importlib.util.module_from_spec(spec); spec.loader.exec_module(g)
import math
def dsin(x):
    s, t, n = D(0), x, 1
    while abs(t) > D(10) ** -44:
        s += t; t = -t * x * x / ((n + 1) * (n + 2)); n += 2
    return s
def dcos(x):
    s, t, n = D(0), D(1), 0
    while abs(t) > D(10) ** -44:
        s += t; t = -t * x * x / ((n + 1) * (n + 2)); n += 2
    return s
F = {
 "LOAD_P0X": lambda t, p: (dsin(t) - dsin(p)) * (dsin(p) - dsin(t) - (p - t) * dcos(t)),
 "LOAD_P0Y": lambda t, p: (dsin(t) - dsin(p)) * (dcos(t) - dcos(p) - (p - t) * dsin(t)),
 "LOAD_P1X": lambda t, p: (dcos(p) - dcos(t)) * (dsin(p) - dsin(t) - (p - t) * dcos(t)),
 "LOAD_P1Y": lambda t, p: (dcos(p) - dcos(t)) * (dcos(t) - dcos(p) - (p - t) * dsin(t)),
 "LOAD_P5X": lambda t, p: (dsin(p) - dsin(t) - (p - t) * dcos(t)),
 "LOAD_P5Y": lambda t, p: (dcos(t) - dcos(p) - (p - t) * dsin(t)),
 "LOAD_Q2": lambda t, p: dsin(p - t) * (1 - dcos(p - t)),
 "LOAD_Q3": lambda t, p: dcos(t) * (1 - dcos(p - t)),
 "LOAD_Q4": lambda t, p: dsin(t) * (1 - dcos(p - t)),
 "LOAD_T2": lambda t, p: (1 - dcos(p - t)) * ((p - t) - dsin(p - t)),
 "LOAD_T3": lambda t, p: dsin(t) * ((p - t) - dsin(p - t)),
 "LOAD_T4": lambda t, p: dcos(t) * ((p - t) - dsin(p - t)),
 "LOAD_ASC": lambda t, p: (p - t) * dsin(t) * dcos(t),
 "LOAD_ASS": lambda t, p: (p - t) * dsin(t) ** 2,
 "LOAD_ACC": lambda t, p: (p - t) * dcos(t) ** 2,
}
# Simpson with Richardson is plenty: integrands are polynomial-smooth; use 400 intervals x Boole
def integ(f, p, n=200):
    h = p / n; s = D(0)
    for i in range(n):
        a = h * i
        xs = [a + h * k / 4 for k in range(5)]
        w = [7, 32, 12, 32, 7]
        s += sum(D(wk) * f(x, p) for wk, x in zip(w, xs)) * h / 90
    return s
from fractions import Fraction as Q
for name, f, _ in g.INTEGRANDS:
    c = g.integrate(f)
    for p in (D("0.3"), D("0.8"), D("1.04")):
        ser = sum(D(v.numerator) / D(v.denominator) * p ** n for n, v in c.items())
        num = integ(F[name], p)
        print(name, p, "rel diff %.2e" % float(abs(ser - num) / abs(num)))
