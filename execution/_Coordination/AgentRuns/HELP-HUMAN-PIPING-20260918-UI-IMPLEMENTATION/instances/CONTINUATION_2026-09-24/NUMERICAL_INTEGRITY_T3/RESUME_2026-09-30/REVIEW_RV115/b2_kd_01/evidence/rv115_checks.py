#!/usr/bin/env python3
"""RV115 (RV-K) independent exact checks of I94's B2-KD design.

Standard library only. Imports nothing from the repository and reads no
repository file except, at the end, I94's own run output (passed as argv[1])
for a value-by-value comparison of the combined nets. Writes only to stdout.

Checks:
 1. combined exact nets of DESIGN.md section 6's C1-C6 from my own operand
    definitions; K4LED canonical form, limb spans, K4LED bytes and sha256;
 2. my own directed 1024-bit rounding: E5's width bound, its zero case,
    C6's endpoints, and a seeded random property test over sums of binary64
    products spanning the full exponent range (including binade crossings);
 3. which specimen products are exact in binary64 (does C2 discriminate
    "exact products"?) and an alternative that does;
 4. cantilever tip flexibility by curvature integration (unit-load method),
    independent of any stiffness-matrix inversion, for two lengths;
 5. C3's operand-row discriminator with my own binary64 EA;
 6. row counts and derivative slots for the one-member specimen;
 7. a model of OriginCapacity / the proposed for_invocation reservation and
    the "dormant runs check" (counterexample and a sufficient check);
 8. magnitude of K-10's C6 nearest-rounding mutation against 1024-bit
    outward widths (row-level detectability);
 9. specimen coverage of constrained-DOF loads (the reaction-offset site).
"""
from fractions import Fraction as F
import hashlib, json, math, random, struct, sys

out = {}

def bits(x):
    return struct.unpack('>Q', struct.pack('>d', x))[0]

def floorlog2(x):
    x = abs(x)
    e = x.numerator.bit_length() - x.denominator.bit_length()
    if x < F(2) ** e:
        e -= 1
    return e

def net_form(q):
    if q == 0:
        return (False, 0, 0)
    neg = q < 0
    q = abs(q)
    n, d = q.numerator, q.denominator
    assert d & (d - 1) == 0, 'dyadic'
    e = -(d.bit_length() - 1)
    while n % 2 == 0:
        n //= 2
        e += 1
    return (neg, n, e)

def span(q):
    return net_form(q)[1].bit_length()

# ---- 1024-bit directed rounding (my own; quantum 2^(e-1023) for 1024 bits)
P = 1024
def quantum(x):
    return F(2) ** (floorlog2(x) - (P - 1))

def rd(x):
    if x == 0:
        return F(0)
    q = quantum(x)
    k = x / q
    return F(k.numerator // k.denominator) * q

def ru(x):
    if x == 0:
        return F(0)
    q = quantum(x)
    k = x / q
    return F(-((-k.numerator) // k.denominator)) * q

def rn(x):
    if x == 0:
        return F(0)
    q = quantum(x)
    k = x / q
    fl = k.numerator // k.denominator
    rem = k - fl
    if rem > F(1, 2) or (rem == F(1, 2) and fl % 2):
        fl += 1
    return F(fl) * q

def representable(x):
    return x == 0 or span(x) <= P

# ---- 1. operands and combinations (DOF index at the tip node: 0..5 = Fx..Mz)
A = [(0, 1.0), (1, 1.0), (3, 1.0)]
B = [(2, 3.0), (4, -2.0), (1, 0.5), (1, -0.5)]
A2 = [(0, 1.0), (1, 1.0), (3, 1.0)]
Pp = [(0, 1.0), (0, 2.0 ** -60)]
Q = [(0, -1.0)]
H = [(1, 2.0 ** 600), (1, 2.0 ** -600)]
combos = {
    'C1': [(1.0, A), (1.0, B), (-1.0, A2)],
    'C2': [(0.1, A), (3.0, B)],
    'C3': [(1.0, Pp), (1.0, Q)],
    'C4': [(2.0, B), (1.0, A)],
    'C5': [(2.0 ** 700, A), (-(2.0 ** 700), A2), (2.0 ** -700, B)],
    'C6': [(1.0, H)],
}

def nets_of(ops):
    nets, flag = {}, {}
    for c, terms in ops:
        for g, v in terms:
            nets[g] = nets.get(g, F(0)) + F(c) * F(v)
            flag[g] = flag.get(g, False) or (c != 0.0 and v != 0.0)
    return nets, flag

def k4led(nets, node=1):
    b = bytearray(b'K4LED\x01')
    b += struct.pack('<I', len(nets))
    for g in sorted(nets):
        neg, m, e = net_form(nets[g])
        limbs = []
        while m:
            limbs.append(m & (2 ** 64 - 1))
            m >>= 64
        b += struct.pack('<I', node) + bytes([g, 1 if neg else 0]) + struct.pack('<q', e if limbs else 0)
        b += struct.pack('<I', len(limbs))
        for l in limbs:
            b += struct.pack('<Q', l)
    return bytes(b)

res1 = {}
all_nets = {}
for name, ops in combos.items():
    nets, flag = nets_of(ops)
    all_nets[name] = nets
    res1[name] = {
        'nets': {str(g): str(nets[g]) for g in sorted(nets)},
        'k4led_form': {str(g): [net_form(nets[g])[0], hex(net_form(nets[g])[1]), net_form(nets[g])[2]] for g in sorted(nets)},
        'span_bits': {str(g): span(nets[g]) for g in sorted(nets)},
        'individual_nonzero': {str(g): flag[g] for g in sorted(nets)},
        'k4led_sha256_tip_node1': hashlib.sha256(k4led(nets)).hexdigest(),
    }
# spot facts
assert all_nets['C1'][0] == 0 and all_nets['C1'][1] == 0 and all_nets['C1'][3] == 0
assert all_nets['C1'][2] == 3 and all_nets['C1'][4] == -2
assert all_nets['C2'][1] == F(0.1) and all_nets['C2'][2] == 9 and all_nets['C2'][4] == -6
assert all_nets['C3'][0] == F(2) ** -60
assert all_nets['C4'] == {0: 1, 1: 1, 2: 6, 3: 1, 4: -4}
assert all_nets['C5'][2] == 3 * F(2) ** -700 and all_nets['C5'][4] == -F(2) ** -699
assert all_nets['C6'][1] == F(2) ** 600 + F(2) ** -600 and span(all_nets['C6'][1]) == 1201
out['1_nets'] = res1

# ---- 2. E5
c6 = all_nets['C6'][1]
lo, hi, near = rd(c6), ru(c6), rn(c6)
e = floorlog2(c6)
e5 = {
    'C6_floorlog2': e,
    'C6_RD_le_N_le_RU': lo <= c6 <= hi,
    'C6_width_log2': floorlog2(hi - lo),
    'C6_width_equals_2^(e-1023)': hi - lo == F(2) ** (e - 1023),
    'C6_RN_equals_2^600': near == F(2) ** 600,
    'C6_RN_excludes_truth': near != c6,
}
rng = random.Random(115)
def rand_b64(rng):
    while True:
        x = struct.unpack('>d', struct.pack('>Q', rng.getrandbits(64)))[0]
        if math.isfinite(x) and x != 0.0:
            return x
trials = 0
viol = 0
zero_ok = 0
crossings = 0
max_rel = F(0)
for _ in range(4000):
    k = rng.randint(1, 6)
    N = sum((F(rand_b64(rng)) * F(rand_b64(rng)) for _ in range(k)), F(0))
    if N == 0:
        continue
    trials += 1
    l, h = rd(N), ru(N)
    w = h - l
    bound = F(2) ** (floorlog2(N) - 1023)
    if not (l <= N <= h) or w > bound or ((w == 0) != representable(N)):
        viol += 1
    if w == 0:
        zero_ok += 1
    if abs(h) >= F(2) ** (floorlog2(N) + 1) or (l != 0 and abs(l) < F(2) ** floorlog2(N)):
        crossings += 1
    if w:
        max_rel = max(max_rel, w / abs(N))
# explicit binade-crossing cases: N just below a power of two
for k in [0, 600, -700, 1023, -1074]:
    for s in [1, -1]:
        N = s * (F(2) ** k - F(2) ** (k - 1100))
        l, h = rd(N), ru(N)
        bound = F(2) ** (floorlog2(N) - 1023)
        trials += 1
        crossings += 1
        if not (l <= N <= h) or h - l > bound:
            viol += 1
e5.update({'random_trials': trials, 'violations': viol, 'exact_cases_width0': zero_ok,
           'binade_crossings_checked': crossings, 'max_width_over_abs_N_log2': floorlog2(max_rel) if max_rel else None})
assert viol == 0
out['2_E5'] = e5

# ---- 3. product exactness
def product_exact(c, v):
    return F(c) * F(v) == F(c * v)
exact_products = {}
for name, ops in combos.items():
    exact_products[name] = all(product_exact(c, v) for c, terms in ops for _, v in terms)
alt = {'operand': 'B with factor 0.1 (0.1*3 at Fz)', 'exact': str(F(0.1) * F(3.0)),
       'binary64': str(F(0.1 * 3.0)), 'exact_span_bits': span(F(0.1) * F(3.0)),
       'binary64_equals_exact': F(0.1) * F(3.0) == F(0.1 * 3.0),
       'relative_difference_log2': floorlog2(abs(F(0.1) * F(3.0) - F(0.1 * 3.0)) / (F(0.1) * F(3.0)))}
out['3_products'] = {'every_product_binary64_exact': exact_products, 'alternative': alt}
assert all(exact_products.values())
assert not alt['binary64_equals_exact']

# ---- 4. cantilever tip flexibility by curvature integration (unit-load method)
# Small-rotation kinematics: du_y/dx = +theta_z, du_z/dx = -theta_y.
# Internal moment at x from a tip action at x = L:
#   tip Fy: Mz(x) = +Fy (L - x);  tip Fz: My(x) = -Fz (L - x)  (r x F = (L-x) x^ x Fz z^);
#   tip Mz: Mz(x) = Mz;            tip My: My(x) = My.
# Curvature: d theta_z/dx = Mz/EIz, d theta_y/dx = My/EIy; integrate from the fixed root.
def poly_int(p):  # p: list of Fraction coefficients in x; returns integral from 0
    return [F(0)] + [c / (i + 1) for i, c in enumerate(p)]
def poly_eval(p, x):
    return sum(c * x ** i for i, c in enumerate(p))
def tip(EA, GJ, EIy, EIz, L, load):
    Fx, Fy, Fz, Mx, My, Mz = load
    mz = [Fy * L + Mz, -Fy]            # Mz(x)
    my = [-Fz * L + My, Fz]            # My(x)
    tz = poly_int([c / EIz for c in mz])
    ty = poly_int([c / EIy for c in my])
    uy = poly_int(tz)
    uz = poly_int([-c for c in ty])
    return [Fx * L / EA, poly_eval(uy, L), poly_eval(uz, L), Mx * L / GJ, poly_eval(ty, L), poly_eval(tz, L)]
flex = {}
for L in (F(1), F(2)):
    EA, GJ, EI = F(11), F(13), F(17)
    unit = lambda j: [F(int(i == j)) for i in range(6)]
    cols = [tip(EA, GJ, EI, EI, L, unit(j)) for j in range(6)]
    flex[str(L)] = {
        'uy_per_Fy': str(cols[1][1]), 'expected_L3_3EI': str(L ** 3 / (3 * EI)),
        'uy_per_Mz': str(cols[5][1]), 'rz_per_Fy': str(cols[1][5]), 'expected_+L2_2EI': str(L ** 2 / (2 * EI)),
        'uz_per_My': str(cols[4][2]), 'ry_per_Fz': str(cols[2][4]), 'expected_-L2_2EI': str(-L ** 2 / (2 * EI)),
        'rz_per_Mz': str(cols[5][5]), 'expected_L_EI': str(L / EI),
        'symmetric': all(cols[i][j] == cols[j][i] for i in range(6) for j in range(6)),
    }
    assert cols[5][1] == cols[1][5] == L ** 2 / (2 * EI)
    assert cols[4][2] == cols[2][4] == -L ** 2 / (2 * EI)
    assert cols[1][1] == cols[2][2] == L ** 3 / (3 * EI)
out['4_cantilever'] = flex

# ---- 5. C3 discriminator with my own binary64 EA (D=0.1, t=0.005, E=210e9; pi as binary64)
Ab = float(F(math.pi) * (F(0.1) ** 2 - (F(0.1) - 2 * F(0.005)) ** 2) / 4)
EAb = 210e9 * Ab
row_P = (1.0 + 2.0 ** -60) / EAb
row_Q = -1.0 / EAb
out['5_C3'] = {'A_binary64': Ab, 'EA_binary64': EAb, 'one_plus_2^-60_binary64_is_one': (1.0 + 2.0 ** -60) == 1.0,
               'sum_of_operand_rows': row_P + row_Q,
               'exact_ux_log2': floorlog2(F(2) ** -60 / F(EAb)),
               'relative_loss_total': (row_P + row_Q) == 0.0}

# ---- 6. row counts (recover.rs layout; final_case.rs row_scales slots)
nodes, members, stations, springs, constraints, supports = 2, 1, 3, 0, 6, 1
native = 6 * nodes + nodes + 12 * members + 6 * stations + springs + constraints + 2 * supports
case_final = 6 * nodes + nodes + 12 * members + 6 * stations + 6 * supports + 2 * supports + 20 * members + members
combo_final = case_final - members
slots = sorted({mi * 21 + site * 4 + comp for mi in range(members) for site in range(5) for comp in range(4)})
out['6_rows'] = {'native_rows': native, 'case_final_rows_without_records': case_final,
                 'formula_7n_51m_8g': 7 * nodes + 51 * members + 8 * supports,
                 'combination_final_rows': combo_final, 'formula_7n_50m_8g': 7 * nodes + 50 * members + 8 * supports,
                 'stress_slots_per_member': len(slots), 'stress_slots': [slots[0], slots[-1]], 'maximum_slot': 20}
assert native == 52 and case_final == 73 and combo_final == 72

# ---- 7. capacity model
def for_calls(batches, combos_):
    cases = sum(batches); combs = len(combos_)
    return dict(calls=len(batches) + combs, cases=cases, combinations=combs, operands=sum(combos_),
                runs=cases + combs, builds=7 * (cases + combs))
def for_invocation(batches, combos_, prepared):
    c = for_calls(batches, combos_)
    c['cases'] += prepared   # native case ordinals, batch + prepared (DESIGN 1.2)
    return c                 # runs = sum(batches) + combinations, builds = 7 runs
cap0 = for_calls([2], [2]); cap1 = for_invocation([2], [2], 0)
cap = for_invocation([2], [2], 1)
# Adversarial order: one batch of 3 (spending the prepared ordinal), then a combination.
used_cases, runs_used, used_comb = 0, 0, 0
batch = 3
ok_cases = used_cases + batch <= cap['cases']
ok_runs_as_stated = runs_used + batch <= cap['runs']                 # "case runs never exceed runs"
ok_runs_sufficient = runs_used + batch + (cap['combinations'] - used_comb) <= cap['runs']
used_cases += batch; runs_used += batch
comb_admitted_today = used_comb < cap['combinations']
runs_after_combination = runs_used + 1
out['7_capacity'] = {
    'for_calls_equals_for_invocation_zero': cap0 == cap1,
    'sources_reservation_cases_plus_combinations_equals_runs_under_for_calls': cap0['cases'] + cap0['combinations'] == cap0['runs'],
    'invocation_[2],[2],1': cap,
    'batch_of_3_passes_case_ordinal_check': ok_cases,
    'batch_of_3_passes_check_as_worded': ok_runs_as_stated,
    'then_combination_admitted_by_existing_checks': comb_admitted_today,
    'runs_after_combination': runs_after_combination, 'runs_reserved': cap['runs'],
    'overflows_reservation': runs_after_combination > cap['runs'],
    'sufficient_check_refuses_batch': not ok_runs_sufficient,
}
# The sufficient check is dormant under for_calls: runs_used <= used_cases + used_comb and
# used_cases + n <= cases imply runs_used + n + (combs - used_comb) <= cases + combs = runs.
dormant = True
for batches in ([1], [2, 3], [4, 1, 1]):
    for cs in ([], [2], [1, 3]):
        c = for_calls(batches, cs)
        for used_comb in range(len(cs) + 1):
            uc = 0
            for n in batches:
                runs_used_max = uc + used_comb
                if not (runs_used_max + n + (c['combinations'] - used_comb) <= c['runs']):
                    dormant = False
                uc += n
out['7_capacity']['sufficient_check_dormant_under_for_calls'] = dormant
assert out['7_capacity']['overflows_reservation'] and dormant

# ---- 8. K-10's C6 nearest-rounding mutation at row level
omitted = abs(c6 - near)                       # 2^-600
k_width = F(2) ** (floorlog2(c6) - 1023)       # one 1024-bit outward step at |N| (K lane)
pi_rel = F(2) ** -512                          # DEF-O's pi bracket is a 2^-512 grid (G lane)
out['8_C6_mutation'] = {
    'omitted_log2': floorlog2(omitted),
    'one_K_lane_1024bit_step_at_N_log2': floorlog2(k_width),
    'ratio_omitted_over_K_step_log2': floorlog2(omitted / k_width),
    'ratio_omitted_over_G_lane_pi_width_at_N_log2': floorlog2(omitted / (pi_rel * c6)),
}

# ---- 9. constrained-DOF loads in the specimen
root_terms = 0  # every operand term in DESIGN 6 is a tip term; the root (node 0) is fully restrained
R = [(1, 5.0)]  # an alternative non-representative operand with Fy = 5 at the ROOT (constrained)
out['9_constrained_loads'] = {'specimen_terms_at_constrained_dofs': root_terms,
    'alternative': 'operand R: Fy = 5 at the restrained root, used as a non-first operand, e.g. A + R',
    'representative_loads_mutation_changes_root_Fy_reaction_by': str(F(5))}

# ---- compare with I94's run output, if given
if len(sys.argv) > 1:
    i94 = json.load(open(sys.argv[1]))['nets']
    keymap = {'C1': 'C1_A+B-A2', 'C2': 'C2_0.1A+3B', 'C3': 'C3_P+Q', 'C4': 'C4_2B(prepared first)+A',
              'C5': 'C5_2^700A-2^700A2+2^-700B', 'C6': 'C6_H1'}
    same = {}
    for k, v in keymap.items():
        theirs = {g: F(d['net']) for g, d in i94[v].items()}
        mine = {str(g): n for g, n in all_nets[k].items()}
        spans = all(i94[v][g]['span_bits'] == span(mine[g]) for g in mine)
        same[k] = theirs == mine and spans
    out['compare_I94'] = same
    assert all(same.values())

print(json.dumps(out, indent=1, sort_keys=True))
