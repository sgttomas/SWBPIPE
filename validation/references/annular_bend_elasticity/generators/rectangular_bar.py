"""The exact plane-stress solution of a curved bar of rectangular section under a radial end force (the classical
Golovin/Ribiere field, as in Timoshenko and Goodier, Theory of Elasticity), and its strain energy per unit angle.
It imports the Python standard library and numpy, and no product code.

Field (unit thickness, inner radius a, outer b, end force P):
  sigma_r   = (2 A r - 2 B / r^3 + D / r) sin(theta)
  sigma_th  = (6 A r + 2 B / r^3 + D / r) sin(theta)
  tau_rth   = -(2 A r - 2 B / r^3 + D / r) cos(theta)
  A = P/(2N), B = -P a^2 b^2/(2N), D = -P (a^2 + b^2)/N, N = a^2 - b^2 + (a^2 + b^2) ln(b/a)

`exact_energies` gives the normal (sin^2) and shear (cos^2) parts of the energy per unit angle, for the check of
the section solver on a thin plate. `total_energy_errors` compares, on a quarter circle (equal sin^2 and cos^2
weights), the elementary beam energy built from the field's own resultants with the exact energy: thin bending
without shear, and thin bending with shear (rectangular shear coefficient 5/6).
"""
from __future__ import annotations

import math

import numpy as np

SAMPLED = (0.02, 0.05, 0.1, 0.2, 0.3, 0.4, 0.6, 0.8, 1.0, 1.2, 1.6)


def _coefficients(hr, P=1.0):
    R = 1.0
    a, b = R - hr / 2, R + hr / 2
    Nn = a * a - b * b + (a * a + b * b) * math.log(b / a)
    return a, b, P / (2 * Nn), -P * a * a * b * b / (2 * Nn), -P * (a * a + b * b) / Nn


def exact_energies(hr, nu=0.3, P=1.0, Em=1.0):
    """Exact plane-stress energies per unit angle, unit thickness, at sin = 1 (normal part) and cos = 1 (shear part)."""
    Gm = Em / (2 * (1 + nu))
    a, b, A_, B_, D_ = _coefficients(hr, P)
    g, w = np.polynomial.legendre.leggauss(64)
    r = a + (b - a) * (g + 1) / 2
    w = w * (b - a) / 2
    sr = 2 * A_ * r - 2 * B_ / r ** 3 + D_ / r
    st = 6 * A_ * r + 2 * B_ / r ** 3 + D_ / r
    tau = -(2 * A_ * r - 2 * B_ / r ** 3 + D_ / r)
    shear = np.sum(w * tau ** 2 / (2 * Gm) * r)
    normal = np.sum(w * (sr ** 2 + st ** 2 - 2 * nu * sr * st) / (2 * Em) * r)
    return normal, shear


def _simpson(f, lo, hi, n=4000):
    h = (hi - lo) / n
    s = f(lo) + f(hi)
    for k in range(1, n):
        s += f(lo + k * h) * (4 if k % 2 else 2)
    return s * h / 3


def total_energy_errors(hr, nu=0.3):
    """(thin bending without shear, with shear) total energy over the exact total, minus one."""
    E, P = 1.0, 1.0
    G = E / (2 * (1 + nu))
    R = 1.0
    h = hr * R
    a, b, A_, B_, D_ = _coefficients(hr, P)
    sr = lambda r: 2 * A_ * r - 2 * B_ / r ** 3 + D_ / r
    st = lambda r: 6 * A_ * r + 2 * B_ / r ** 3 + D_ / r
    tau = lambda r: -(2 * A_ * r - 2 * B_ / r ** 3 + D_ / r)
    Nres = _simpson(st, a, b)
    Vres = _simpson(tau, a, b)
    Mres = _simpson(lambda r: st(r) * (r - R), a, b)
    assert abs(Mres + R * Nres) < 1e-9 * abs(R * Nres), "the end force must pass through the centre"
    shear_exact = _simpson(lambda r: tau(r) ** 2 / (2 * G) * r, a, b)
    normal_exact = _simpson(lambda r: (sr(r) ** 2 + st(r) ** 2 - 2 * nu * sr(r) * st(r)) / (2 * E) * r, a, b)
    Ar, I = h, h ** 3 / 12
    shear_k = Vres ** 2 * R / (2 * (5.0 / 6.0) * G * Ar)
    thin = Mres ** 2 * R / (2 * E * I) + Nres ** 2 * R / (2 * E * Ar)
    total = normal_exact + shear_exact
    return thin / total - 1, (thin + shear_k) / total - 1
