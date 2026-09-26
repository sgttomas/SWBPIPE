"""V2: model inputs for the T3 references.

Reads only the model inputs (`model`, `purpose`, `family`) of references.json, never an expected value.
Models that references.json abbreviates (n = 10000 and RF-MECH-LINE-IN-CHAIN1000) are regenerated here
from the generator rules stated in R1's README and case purposes; the generator is validated by
reproducing every complete n = 10, 100 and 1000 model in references.json exactly.
"""
import json
import math
import re
from fractions import Fraction as Fr

Q3 = [[Fr(1, 3), Fr(2, 3), Fr(2, 3)], [Fr(2, 3), Fr(1, 3), Fr(-2, 3)], [Fr(-2, 3), Fr(2, 3), Fr(-1, 3)]]
SEC_N = {"E": "200000000000", "G": "80000000000", "OD": "0.2", "ID": "0.18"}
ALL6 = ["UX", "UY", "UZ", "RX", "RY", "RZ"]
PIN3 = ["UX", "UY", "UZ"]


def s(q):
    q = Fr(q)
    return str(q.numerator) if q.denominator == 1 else '%d/%d' % (q.numerator, q.denominator)


def mv(Q, v):
    return tuple(sum(Q[r][c] * v[c] for c in range(3)) for r in range(3))


def rot_model(m):
    out = dict(m)
    out['nodes_m'] = {n: [s(c) for c in mv(Q3, [Fr(x) for x in xyz])] for n, xyz in m['nodes_m'].items()}
    out['loads'] = {n: {k: [s(c) for c in mv(Q3, [Fr(x) for x in v])] for k, v in l.items()} for n, l in m['loads'].items()}
    return out


def gen_chain(n):
    sx = round(math.log2(1.264 / n ** 2))     # stated rule: powers of two for a tip rotation near 1e-4
    s2 = round(math.log2(24.5 / n))
    # tip force 2^s (6,-3,9) N, tip moment 2^s2 (3,6,-3) N*m  (n = 10: (0.09375,-0.046875,0.140625), (6,12,-6))
    nodes = {'N%d' % i: [s(3 * i), '0', '0'] for i in range(n + 1)}
    members = [['M%d' % (i + 1), 'N%d' % i, 'N%d' % (i + 1), 'N'] for i in range(n)]
    F = [s(Fr(2) ** sx * c) for c in (6, -3, 9)]
    M = [s(Fr(2) ** s2 * c) for c in (3, 6, -3)]
    return {'nodes_m': nodes, 'sections': {'N': dict(SEC_N)}, 'members': members,
            'supports': {'N0': {'rigid': list(ALL6), 'springs': []}}, 'loads': {'N%d' % n: {'F': F, 'M': M}}}


def gen_tree(n, sc):
    h = n // 2
    nodes = {'P0': ['0', '0', '0']}
    members = []
    loads = {}
    for k in range(1, h + 1):
        nodes['P%d' % k] = [s(3 * k), '0', '0']
        f = Fr(3 * ((k % 7) - 3)) * Fr(2) ** sc
        if k % 2:
            nodes['B%d' % k] = [s(3 * k), '3', '0']
            F = ['0', '0', s(f)]
        else:
            nodes['B%d' % k] = [s(3 * k), '0', '3']
            F = ['0', s(f), '0']
        members.append(['S%d' % k, 'P%d' % (k - 1), 'P%d' % k, 'N'])
        members.append(['Q%d' % k, 'P%d' % k, 'B%d' % k, 'N'])
        if f != 0:
            loads['B%d' % k] = {'F': F}
    return {'nodes_m': nodes, 'sections': {'N': dict(SEC_N)}, 'members': members,
            'supports': {'P0': {'rigid': list(ALL6), 'springs': []}}, 'loads': loads}


def gen_cont(n):
    sp = n // 2
    ell = 6
    nodes = {}
    members = []
    for i in range(sp + 1):
        nodes['S%d' % i] = [s(i * ell), '0', '0']
    for j in range(1, sp + 1):
        nodes['C%d' % j] = [s(Fr((2 * j - 1) * ell, 2)), '0', '0']
        members.append(['A%d' % j, 'S%d' % (j - 1), 'C%d' % j, 'S'])
        members.append(['B%d' % j, 'C%d' % j, 'S%d' % j, 'S'])
    sup = {'S0': {'rigid': list(ALL6), 'springs': []}}
    for i in range(1, sp + 1):
        sup['S%d' % i] = {'rigid': list(PIN3), 'springs': []}
    loads = {}
    for j in range(1, sp + 1):
        py, pz = 96 * ((j % 5) - 2), 96 * ((j % 3) - 1)
        if py or pz:
            loads['C%d' % j] = {'F': ['0', s(py), s(pz)]}
    return {'nodes_m': nodes, 'sections': {'S': dict(SEC_N)}, 'members': members, 'supports': sup, 'loads': loads}


def gen_line_in_chain1000():
    m = gen_chain(1000)
    for i in range(6):
        m['nodes_m']['L%d' % i] = [s(i), s(100 + 2 * i), s(2 * i)]
        m['supports']['L%d' % i] = {'rigid': list(PIN3), 'springs': []}
    for i in range(1, 6):
        m['members'].append(['LM%d' % i, 'L%d' % (i - 1), 'L%d' % i, 'N'])
    m['loads']['L5'] = {'M': ['0.001', '0.002', '0.002']}
    return m


def same_model(a, b):
    """Compare two JSON models by exact value."""
    P = lambda x: Fr(x) if '^' not in x else None
    if set(a['nodes_m']) != set(b['nodes_m']):
        return False
    for k in a['nodes_m']:
        if [Fr(x) for x in a['nodes_m'][k]] != [Fr(x) for x in b['nodes_m'][k]]:
            return False
    if [list(x) for x in a['members']] != [list(x) for x in b['members']]:
        return False
    la = {n: {k: [Fr(x) for x in v] for k, v in l.items()} for n, l in a['loads'].items()}
    lb = {n: {k: [Fr(x) for x in v] for k, v in l.items()} for n, l in b['loads'].items()}
    z = [Fr(0)] * 3
    for n in set(la) | set(lb):
        for k in ('F', 'M'):
            if la.get(n, {}).get(k, z) != lb.get(n, {}).get(k, z):
                return False
    ra = {n: sorted(x['rigid']) for n, x in a['supports'].items()}
    rb = {n: sorted(x['rigid']) for n, x in b['supports'].items()}
    return ra == rb and a['sections'] == b['sections']


def load_cases(path):
    """Return {case_id: {'family', 'model', 'purpose'}} with complete models; nothing else is read."""
    d = json.load(open(path))
    out = {}
    for cid, c in d['cases'].items():
        out[cid] = {'family': c['family'], 'model': c['model'], 'purpose': c['purpose'], 'flags': c.get('flags')}
    # regenerate abbreviated models
    for cid, c in out.items():
        if 'nodes_m' in c['model']:
            continue
        m = re.match(r'RF-LARGE-(CHAIN|TREE|CONT)-n(\d+)-(AX|ROT)$', cid)
        if m:
            kind, n, ax = m.group(1), int(m.group(2)), m.group(3)
            if kind == 'CHAIN':
                gm = gen_chain(n)
            elif kind == 'TREE':
                src = out['RF-LARGE-TREE-n%05d-AX' % n]['purpose']
                sc = int(re.search(r'\*2\^(-?\d+) N', src).group(1))
                gm = gen_tree(n, sc)
            else:
                gm = gen_cont(n)
            if ax == 'ROT':
                gm = rot_model(gm)
        elif cid == 'RF-MECH-LINE-IN-CHAIN1000':
            gm = gen_line_in_chain1000()
        else:
            raise KeyError(cid)
        c['model'] = gm
        c['generated'] = True
    return out


def validate_generators(cases):
    """Reproduce every complete RF-LARGE model (n = 10, 100, 1000) and RF-MECH-DISC chain from the rules."""
    res = []
    for cid, c in cases.items():
        m = re.match(r'RF-LARGE-(CHAIN|TREE|CONT)-n(\d+)-(AX|ROT)$', cid)
        if not m or c.get('generated'):
            continue
        kind, n, ax = m.group(1), int(m.group(2)), m.group(3)
        if kind == 'CHAIN':
            gm = gen_chain(n)
        elif kind == 'TREE':
            sc = int(re.search(r'\*2\^(-?\d+) N', cases['RF-LARGE-TREE-n%05d-AX' % n]['purpose']).group(1))
            gm = gen_tree(n, sc)
        else:
            gm = gen_cont(n)
        if ax == 'ROT':
            gm = rot_model(gm)
        res.append((cid, same_model(gm, c['model'])))
    return res
