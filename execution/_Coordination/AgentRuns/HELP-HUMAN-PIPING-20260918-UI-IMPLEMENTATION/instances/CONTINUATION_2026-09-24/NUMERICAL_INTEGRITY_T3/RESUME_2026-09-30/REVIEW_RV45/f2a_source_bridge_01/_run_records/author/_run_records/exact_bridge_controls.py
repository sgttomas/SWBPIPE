"""I33 finite ABSTRACT controls only. No product import, solver, model or host probe.

Run from NUM with python3 <this path>. Uses only exact standard-library arithmetic.
Closed 1x1/2x2 formulas below are mathematical witnesses, not a solver implementation.
"""
from fractions import Fraction as F
import json


def p2(e):
    return F(2**e) if e >= 0 else F(1, 2**(-e))


def rn64(x):
    if not x:
        return F(0)
    sign = 1 if x > 0 else -1
    x = abs(x)
    e = x.numerator.bit_length() - x.denominator.bit_length()
    if x < p2(e):
        e -= 1
    quantum = p2(max(e - 52, -1074))
    z = x / quantum
    q, r = divmod(z.numerator, z.denominator)
    if 2*r > z.denominator or (2*r == z.denominator and q % 2):
        q += 1
    y = q * quantum
    if y > (2-p2(-52))*p2(1023):
        raise ValueError("binary64 overflow")
    return sign*y


def allowance(n, scale):
    eps, u, h = p2(-64), p2(-53), p2(-1074)
    m = max(abs(n), scale)
    exact = eps*m*(1+p2(-21)) + u*abs(n) + h
    a0 = rn64(eps*m)
    a1 = rn64(a0*(1+p2(-21)))
    u0 = rn64(u*abs(n))
    u1 = rn64(u0+h)
    return exact, rn64(a1+u1)


def relative_pass(n, scale, h_n, y=None, h_u=None):
    y = n if y is None else y
    h_u = h_n if h_u is None else h_u
    ae, af = allowance(n, scale)
    return h_n <= ae and h_n <= af and 10**9*h_n <= abs(n) and 10**9*h_u <= abs(y)


def bridge(beta, eta, v):
    alpha = beta*eta
    if beta <= 0 or eta < 0 or v < 0 or alpha >= 1:
        return None
    return beta*v/(1-alpha)


def inv2(a, b, d, f0, f1):
    det = a*d-b*b
    assert a > 0 and det > 0
    return ((d*f0-b*f1)/det, (a*f1-b*f0)/det)


checks = []


def record(name, **values):
    checks.append({"name": name, "status": "pass", **{k: str(v) for k, v in values.items()}})


# A nonzero change of operator can be certified without changing the K output.
d = p2(-60)
tau = bridge(F(1), d, d)
truth_error = d/(1+d)
assert tau >= truth_error and relative_pass(F(1), F(1), tau)
record("scalar_success_distinct_operators", delta=d, error=truth_error, bound=tau)

# Decimal 1e-9 alone is insufficient: the sharper predicates also remain required.
d = p2(-40)
tau = bridge(F(1), d, d)
truth_error = d/(1+d)
ae, af = allowance(F(1), F(1))
assert tau >= truth_error and 10**9*tau < 1
assert truth_error > ae and truth_error > af and not relative_pass(F(1), F(1), tau)
record("scalar_sharper_refusal_despite_decimal_pass", delta=d, error=truth_error, bound=tau, A_exact=ae, A_f64=af)

# Positive rank-one mode coefficients 1 +/- d yield this SPD source matrix.
# The zero K component is genuinely zero; the source component is not.
d = p2(-52)
uk = (F(0), F(1))
assert ((1+d)+(1-d))/2 == 1 and ((1+d)-(1-d))/2 == d
ug = inv2(F(1), d, F(1), F(0), F(1))
tau = bridge(F(1), d, d)
b = p2(-64)
assert max(abs(ug[i]-uk[i]) for i in range(2)) <= tau
assert abs(ug[0]) > b and tau > b
record("zero_component_bare_b_refusal", source_zero_row=ug[0], bound=tau, bare_b=b)

# Refusal is not a singularity assertion: K_G=2 is strictly positive.
assert bridge(F(1), F(1), F(1)) is None
assert F(1, 2) > 0
record("alpha_one_refuses_while_source_is_invertible", alpha=1, source_solution=F(1,2))

# The full free-to-constrained perturbation is essential.
d = p2(-45)
uk = F(3, 2)
ug = F(3, 2)*(1+d)
tau = bridge(F(1, 2), F(0), 3*d)
assert tau == abs(ug-uk) and bridge(F(1,2), F(0), F(0)) == 0
record("prescribed_coupling_term", exact_error=abs(ug-uk), correct_bound=tau, omitted_term_bound=0)

# Parallel branch actions depend on stiffness; their sum is fixed by equilibrium.
d = p2(-48)
uk, ug = F(1,2), 1/(2+d)
tau = bridge(F(1,2), d, d*abs(uk))
qk, qg = uk, (1+d)*ug
action_bound = (1+d)*tau + d*abs(uk)
assert abs(qg-qk) <= action_bound and qg != qk and uk != ug
assert (1+d)*ug + ug == 1
record("action_functional_and_equilibrium_scope", displacement_bound=tau, branch_error=abs(qg-qk), branch_bound=action_bound, total_action=1)

# S belongs to the certified verification operator, and error is mapped back by S.
d = p2(-50)
scale = (F(2), F(1,2))
uk = (F(1,2), F(1,8))
ug = (uk[0]/(1+d), uk[1]/(1+d))
tau = bridge(F(1,2), 8*d, 2*d)
assert all(abs(ug[i]-uk[i]) <= scale[i]*tau for i in range(2))
record("radix_scaled_components", beta=F(1,2), eta=8*d, residual_majorant=2*d, tau=tau)

# The bridge does not treat the published displacement as the exact K solution.
d, r = p2(-60), p2(-70)
x, uk, ug = 1+r, F(1), 1/(1+d)
tau = bridge(F(1), d, d*(abs(x)+r))
assert abs(x-uk) <= r and abs(ug-uk) <= tau and abs(x-ug) <= r+tau
assert relative_pass(x, x, r+tau)
record("publication_radius_in_residual_majorant", x=x, R_K=r, source_error=abs(x-ug), H=r+tau)

# B1C's already reviewed fixed pi endpoints, reused only as exact abstract constants.
pl = F(int("3243f6a8885a308d313198a2e03707344a4093822299f31d0082efa98ec4e6c89452821e638d01377be5466cf34e90c6cc0ac29b7c97c50dd3f84d5b5b5470917",16), 2**512)
pu = pl+p2(-512)
D, t, E, nu = F(4), F(1), F(1), F(1,4)
c, ri = D/2, D/2-t
P, Q = t*(D-t), c*c+ri*ri
area = (pl*P, pu*P)
inertia = (pl*P*Q/4, pu*P*Q/4)
polar = (2*inertia[0], 2*inertia[1])
g_source = E/(2*(1+nu))
g_k = rn64(g_source)
j_k = rn64((polar[0]+polar[1])/2)
assert 3 < pl < pu < 4 and g_source != g_k
torsion = (g_source*polar[0], g_source*polar[1])
k_torsion = g_k*j_k
delta = max(abs(torsion[0]-k_torsion), abs(torsion[1]-k_torsion))
assert delta > 0 and area[0] > 0 and inertia[0] > 0 and torsion[0] > 0
assert all(abs(v-k_torsion) <= delta for v in torsion)
record("source_geometry_and_derived_G_difference", exact_G=g_source, admitted_G=g_k, torsion_difference_bound=delta)

# Simple interval validity/source identity checks are separate from a zero radius.
assert not (F(-1) > -1)  # nu=-1: denominator zero, refuse before evaluation.
assert not (F(2) < F(4)/2)  # t=D/2 violates the unchanged annulus premise.
assert None != F(0)
record("nonpositive_operand_and_absence_refusals", invalid_nu=-1, invalid_D=4, invalid_t=2)

# The identical load is the exact ledger, not a nearest-rounded sequential fold.
terms = [F(1), F(-1), p2(-80)]
assert sum(terms, F(0)) == p2(-80)
other_terms = [F(1), p2(-80), F(-1)]
fold = F(0)
for term in other_terms:
    fold = rn64(fold+term)
assert fold == 0 and sum(other_terms, F(0)) == p2(-80)
record("exact_load_terms_not_rounded_net", exact_net=sum(terms,F(0)), rounded_fold=fold)

# Example of arithmetic-domain refusal; no truncation or silent zero substitution.
def fits(q, cap):
    return max(abs(q.numerator).bit_length(), q.denominator.bit_length()) <= cap

assert fits(F(1,3), 64) and not fits(p2(-128), 64)
record("checked_arithmetic_cap_refusal", abstract_cap_bits=64, refused_denominator_bits=129)

print(json.dumps({"kind":"abstract exact design controls; no realized source/solver reach claim", "groups":len(checks), "checks":checks}, indent=2))
