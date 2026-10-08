#!/usr/bin/env python3
"""I94 B2-KD design checks (standard library only; no repository imports).

Feasibility facts for DESIGN.md section 6 (the oracle specification):
1. exact combined-ledger nets (K4LED form: sign, odd magnitude, exponent) for
   the proposed combination specimens;
2. the significant-bit span of each net (does it fit 1024 bits?);
3. the cantilever tip flexibility from a textbook 3D frame element solved
   exactly with Fractions (signs of the coupling terms);
4. the C03-style discriminator: the binary64 sum of operand rows versus the
   exact combined response.
Nothing here reads or writes repository files.
"""
from fractions import Fraction as F
import json, struct

def f(x):
    return F(x)

def bits(x):
    return struct.unpack('>Q', struct.pack('>d', x))[0]

def net_form(q):
    """(negative, odd magnitude, exponent) as ledger.rs LedgerNet, or zero."""
    if q == 0:
        return (False, 0, 0)
    neg = q < 0
    q = abs(q)
    num, den = q.numerator, q.denominator
    assert den & (den - 1) == 0, 'dyadic'
    e = -(den.bit_length() - 1)
    while num % 2 == 0:
        num //= 2
        e += 1
    return (neg, num, e)

def span_bits(q):
    neg, m, e = net_form(q)
    return m.bit_length()

# Operand terms: list of (dof, value) at the tip node; dof 0..5 = Ux..Rz.
A = [(0, 1.0), (1, 1.0), (3, 1.0)]
B = [(2, 3.0), (4, -2.0), (1, 0.5), (1, -0.5)]
A2 = list(A)
P = [(0, 1.0), (0, 2.0 ** -60)]
Q = [(0, -1.0)]
H1 = [(1, 2.0 ** 600), (1, 2.0 ** -600)]

combos = {
    'C1_A+B-A2': [(1.0, A), (1.0, B), (-1.0, A2)],
    'C2_0.1A+3B': [(0.1, A), (3.0, B)],
    'C3_P+Q': [(1.0, P), (1.0, Q)],
    'C4_2B(prepared first)+A': [(2.0, B), (1.0, A)],
    'C5_2^700A-2^700A2+2^-700B': [(2.0 ** 700, A), (-(2.0 ** 700), A2), (2.0 ** -700, B)],
    'C6_H1': [(1.0, H1)],
}

out = {}
for name, ops in combos.items():
    nets = {}
    nonzero = {}
    for c, terms in ops:
        for dof, v in terms:
            nets[dof] = nets.get(dof, F(0)) + f(c) * f(v)
            nonzero[dof] = nonzero.get(dof, False) or (c != 0 and v != 0)
    out[name] = {
        str(d): {
            'net': str(nets[d]),
            'zero': nets[d] == 0,
            'individual_nonzero': nonzero[d],
            'span_bits': span_bits(nets[d]),
            'fits_1024': span_bits(nets[d]) <= 1024,
        }
        for d in sorted(nets)
    }

# Cantilever: textbook 3D Euler-Bernoulli frame element along global X,
# local axes = global axes (y_ref = global Y). Tip DOFs only (root fixed).
def tip_stiffness(EA, GJ, EIy, EIz, L):
    K = [[F(0)] * 6 for _ in range(6)]
    K[0][0] = EA / L
    K[3][3] = GJ / L
    # bending in x-y plane (uy, rz) about z
    K[1][1] = 12 * EIz / L ** 3; K[1][5] = -6 * EIz / L ** 2
    K[5][1] = -6 * EIz / L ** 2; K[5][5] = 4 * EIz / L
    # bending in x-z plane (uz, ry) about y
    K[2][2] = 12 * EIy / L ** 3; K[2][4] = 6 * EIy / L ** 2
    K[4][2] = 6 * EIy / L ** 2; K[4][4] = 4 * EIy / L
    return K
# Tip-node block of the standard element matrix (node j): the signs below are
# the j-j block (Przemieniecki): k_yy=12EIz/L^3, k_y,rz=-6EIz/L^2 at node j;
# k_zz=12EIy/L^3, k_z,ry=+6EIy/L^2 at node j.

def solve(K, f):
    n = len(f)
    M = [row[:] + [f[i]] for i, row in enumerate(K)]
    for c in range(n):
        p = next(r for r in range(c, n) if M[r][c] != 0)
        M[c], M[p] = M[p], M[c]
        for r in range(n):
            if r != c and M[r][c] != 0:
                t = M[r][c] / M[c][c]
                M[r] = [a - t * b for a, b in zip(M[r], M[c])]
    return [M[i][n] / M[i][i] for i in range(n)]

EA, GJ, EI, L = F(7), F(5), F(3), F(1)
K = tip_stiffness(EA, GJ, EI, EI, L)
flex = {}
for j in range(6):
    e = [F(0)] * 6
    e[j] = F(1)
    flex[j] = solve(K, e)
coupling = {
    'uy_per_Mz': str(flex[5][1]), 'rz_per_Fy': str(flex[1][5]),
    'uz_per_My': str(flex[4][2]), 'ry_per_Fz': str(flex[2][4]),
    'uy_per_Fy': str(flex[1][1]), 'rz_per_Mz': str(flex[5][5]),
}
# Expected closed forms with EI=3, L=1: uy/Fy = 1/9, rz/Mz = 1/3, |uy/Mz| = 1/6.

# C03 discriminator on C3 = P + Q (Fx only): exact ux = 2^-60/(EA);
# binary64 operand rows ux_P = RN64((1+2^-60)/EA_b), ux_Q = RN64(-1/EA_b).
EA_b = 210e9 * 0.0014922565104551517  # an illustrative positive binary64 EA
ux_P = (1.0 + 2.0 ** -60) / EA_b
ux_Q = -1.0 / EA_b
sum_rows = ux_P + ux_Q
exact = F(2) ** -60 / F(EA_b)
disc = {
    'one_plus_2^-60_is_one_in_binary64': (1.0 + 2.0 ** -60) == 1.0,
    'sum_of_operand_rows': sum_rows,
    'exact_combined_ux': float(exact),
    'relative_loss': 'total' if sum_rows == 0.0 else float(abs(F(sum_rows) - exact) / exact),
}

print(json.dumps({'nets': out, 'cantilever_coupling_EA7_GJ5_EI3_L1': coupling,
                  'c03_discriminator': disc,
                  'factor_0.1_bits': hex(bits(0.1))}, indent=1))
