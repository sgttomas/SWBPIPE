"""V2: re-derive every all-zero class scale from R1's stated rule (README section 4) with V2's own member
lengths and section stiffnesses.  Usage: python3 v2_zeroscale.py <references.json>
Rule: force = moment scale / L_c (or max(F_ref, M_ref/L_c) when both vanish); moment = force scale * L_c
(or max(M_ref, F_ref*L_c)); translation = rotation scale * L_c; rotation = translation scale / L_c;
twist = moment scale * max(L/GJ); extension = force scale * max(L/EA).  L_c = longest member of the case
(RF-WEAK: of the region)."""
import json
import re
import sys
from decimal import Decimal as D, getcontext
import v2_frame as vf
from v2_models import load_cases

getcontext().prec = 60
ref = json.load(open(sys.argv[1]))['cases']
cases = load_cases(sys.argv[1])
R = vf.Num('dec')
n = bad = 0
for cid, c in ref.items():
    if not c.get('scales'):
        continue
    m = vf.load_model(cases[cid]['model'])
    props = {k: vf.section_props(s, R) for k, s in m['sections'].items()}
    L = {}
    for name, a, b, sec in m['members']:
        d = [R.conv(m['nodes'][b][i] - m['nodes'][a][i]) for i in range(3)]
        L[name] = (sum(x * x for x in d).sqrt(), props[sec])
    sc = {k: D(v['value']) for k, v in c['scales'].items()}
    for cl, v in c['scales'].items():
        der = v['derivation']
        if 'class all zero' not in der:
            continue
        base = cl.split('@')[0]
        region = cl.split('@')[1] if '@' in cl else None
        if region:
            continue   # RF-WEAK regional all-zero classes: none published with this template
        m_lc = re.search(r'L_c = ([0-9.e+-]+) m', der)
        Lc = max(x[0] for x in L.values())
        if m_lc:
            assert abs(D(m_lc.group(1)) - Lc) <= D('1e-10') * Lc, (cid, cl, m_lc.group(1), Lc)
        if der.startswith('moment scale / L_c'):
            want = sc['moment'] / Lc
        elif der.startswith('force scale * L_c'):
            want = sc['force'] * Lc
        elif der.startswith('rotation scale * L_c'):
            want = sc['rotation'] * Lc
        elif der.startswith('translation scale / L_c'):
            want = sc['translation'] / Lc
        elif der.startswith('force scale * max(L/EA)'):
            want = sc['force'] * max(l / p[0] for l, p in L.values())
        elif der.startswith('moment scale * max(L/GJ)'):
            want = sc['moment'] * max(l / p[2] for l, p in L.values())
        else:
            print('unhandled', cid, cl, der)
            continue
        n += 1
        if abs(want - sc[cl]) > D('1e-23') * sc[cl]:
            bad += 1
            print('MISMATCH', cid, cl, sc[cl], want)
print('all-zero class scales re-derived: %d, mismatches: %d' % (n, bad))
