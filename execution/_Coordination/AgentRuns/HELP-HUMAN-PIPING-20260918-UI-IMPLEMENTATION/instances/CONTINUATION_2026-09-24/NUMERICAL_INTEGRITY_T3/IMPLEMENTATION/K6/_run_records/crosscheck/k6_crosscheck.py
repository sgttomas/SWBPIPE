"""K6 B2: the recorded cross-check of the 24 RF-LARGE kernel models against R1.

Usage: python3 k6_crosscheck.py <references.py> <dir of k6_observe --emit-model outputs>

R1's references.py is imported as a module (P1's load_refs method) and only its
RF-LARGE builder runs. For every case, model_json(defn, full=True) -- exactly what
`references.py --model <id>` prints (references.py main) -- is compared with the
kernel model the Rust binary emitted, each exact input rounded once with
float(Fraction):
  node order and labels, coordinates; member order, labels and end nodes;
  E and G; OD and ID through the stated section formula (A, I, J bits);
  restraints (rigid sets; no springs); every load component, and no extra load.
A difference is a stop (never edit a reference). Standard library only.
"""
import importlib.util
import json
import math
import struct
import sys
from fractions import Fraction as Fr

DOFS = ('UX', 'UY', 'UZ', 'RX', 'RY', 'RZ')


def load_refs(path):
    spec = importlib.util.spec_from_file_location('references', path)
    module = importlib.util.module_from_spec(spec)
    sys.modules['references'] = module
    spec.loader.exec_module(module)
    module.build_large()
    return module


def unhex(token):
    return struct.unpack('>d', bytes.fromhex(token))[0]


def parse(text):
    m = {'nodes': [], 'members': [], 'restraints': {}, 'loads': {}}
    for line in text.splitlines():
        t = line.split(' ')
        if t[0] == 'section':
            m['section'] = [unhex(t[k]) for k in (2, 4, 6, 8, 10, 12)]
        elif t[0] == 'node':
            m['nodes'].append((t[2], [unhex(c) for c in t[3:6]]))
        elif t[0] == 'member':
            m['members'].append((t[2], int(t[3]), int(t[4])))
        elif t[0] == 'restraint':
            m['restraints'][int(t[1])] = t[2]
        elif t[0] == 'load':
            m['loads'][int(t[1])] = unhex(t[2])
    return m


def f64(x):
    return float(Fr(x))


def section_bits(od, id_):
    od2, id2 = od * od, id_ * id_
    a = math.pi * (od2 - id2) / 4.0
    i = math.pi * (od2 * od2 - id2 * id2) / 64.0
    return [a, i, i, 2.0 * i]


def check(case_id, ref, kernel):
    problems = []
    labels = list(ref['nodes_m'].keys())
    if [l for l, _ in kernel['nodes']] != labels:
        problems.append('node order or labels differ')
    for (label, p), (rlabel, rp) in zip(kernel['nodes'], ref['nodes_m'].items()):
        if [f64(c) for c in rp] != p:
            problems.append('coordinates of %s differ' % label)
    index = {l: i for i, l in enumerate(labels)}
    ref_members = [(mid, index[a], index[b]) for (mid, a, b, _sec) in ref['members']]
    if kernel['members'] != ref_members:
        problems.append('members differ (order, labels or end nodes)')
    (sec,) = ref['sections'].values()
    E, G, OD, ID = f64(sec['E']), f64(sec['G']), f64(sec['OD']), f64(sec['ID'])
    if kernel['section'][:2] != [E, G]:
        problems.append('E or G differs')
    if kernel['section'][2:] != section_bits(OD, ID):
        problems.append('A, I or J differs from the stated formula on OD and ID')
    ref_restraints = {}
    for node, s in ref['supports'].items():
        if s.get('springs'):
            problems.append('R1 has springs at %s' % node)
        ref_restraints[index[node]] = ''.join('1' if d in s['rigid'] else '0' for d in DOFS)
    if kernel['restraints'] != ref_restraints:
        problems.append('restraints differ')
    ref_loads = {}
    for node, ld in ref['loads'].items():
        for kind, offset in (('F', 0), ('M', 3)):
            for axis, c in enumerate(ld.get(kind, ('0', '0', '0'))):
                if Fr(c) != 0:
                    ref_loads[index[node] * 6 + offset + axis] = f64(c)
    if kernel['loads'] != ref_loads:
        problems.append('loads differ')
    return problems


def main():
    refs_path, models_dir = sys.argv[1], sys.argv[2]
    refs = load_refs(refs_path)
    ids = ['RF-LARGE-%s-n%05d-%s' % (f, n, o) for f in ('CHAIN', 'TREE', 'CONT')
           for n in (10, 100, 1000, 10000) for o in ('AX', 'ROT')]
    failures = 0
    for case_id in ids:
        case = next(c for c in refs.CASES if c['id'] == case_id)
        ref = refs.model_json(case['defn'], full=True)
        with open('%s/%s.k6model' % (models_dir, case_id)) as fh:
            kernel = parse(fh.read())
        problems = check(case_id, ref, kernel)
        failures += bool(problems)
        print('%s %s nodes=%d members=%d loads=%d%s' % (
            'EQUAL' if not problems else 'DIFFERS', case_id, len(kernel['nodes']),
            len(kernel['members']), len(kernel['loads']),
            '' if not problems else ' :: ' + '; '.join(problems)))
    print('summary: %d of %d equal' % (len(ids) - failures, len(ids)))
    return 1 if failures else 0


if __name__ == '__main__':
    sys.exit(main())
