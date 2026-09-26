"""V2 backcheck of R1 revision 2: structural diff against revision 1, input exactness, F3/F4/F9 texts.

Usage: python3 bc_diff.py <rev1 references.json> <rev2 references.json> <out.json>
Standard library only.  Revision 1 is the file V2 checked (sha256 57e3254e...).
"""
import json
import re
import sys
from decimal import Decimal, getcontext
from fractions import Fraction as F

getcontext().prec = 80
D = Decimal


def parse(s):
    s = str(s).strip()
    m = re.fullmatch(r'(.+)\*2\^(-?\d+)', s)
    if m:
        return parse(m.group(1)) * F(2) ** int(m.group(2))
    return F(s)


def walk(a, b, path, out):
    if type(a) != type(b):
        out.append((path, 'type', repr(a)[:80], repr(b)[:80]))
    elif isinstance(a, dict):
        for k in sorted(set(a) | set(b)):
            if k not in a:
                out.append((path + '.' + k, 'added', None, json.dumps(b[k])[:200]))
            elif k not in b:
                out.append((path + '.' + k, 'removed', json.dumps(a[k])[:200], None))
            else:
                walk(a[k], b[k], path + '.' + k, out)
    elif isinstance(a, list):
        if len(a) != len(b):
            out.append((path, 'length', len(a), len(b)))
        for i, (x, y) in enumerate(zip(a, b)):
            walk(x, y, '%s[%d]' % (path, i), out)
    elif a != b:
        out.append((path, 'value', a if not isinstance(a, str) else a[:200], b if not isinstance(b, str) else b[:200]))


def classify(path):
    p = re.sub(r'\[\d+\]', '[]', path)
    p = re.sub(r'^\.cases\.[^.]+', '.cases.<id>', p)
    p = re.sub(r'\.nodes_m\.[^.\[]+', '.nodes_m.<n>', p)
    p = re.sub(r'\.supports\.[^.]+', '.supports.<n>', p)
    p = re.sub(r'\.loads\.[^.]+', '.loads.<n>', p)
    p = re.sub(r'\.sections\.[^.]+', '.sections.<s>', p)
    p = re.sub(r'\.member_uniform_loads_N_per_m_global\.[^.\[]+', '.member_uniform_loads_N_per_m_global.<m>', p)
    return p


def main():
    r1 = json.load(open(sys.argv[1]))
    r2 = json.load(open(sys.argv[2]))
    diffs = []
    walk(r1, r2, '', diffs)
    kinds = {}
    for d in diffs:
        kinds.setdefault((classify(d[0]), d[1]), []).append(d)
    res = {'n_diffs': len(diffs), 'by_path_class': {'%s (%s)' % k: len(v) for k, v in sorted(kinds.items())}}
    # which classes are allowed by the revision's stated scope
    allowed = re.compile(r'^(\.criterion|\.status|\.conventions\.model inputs|\.families.*'
                         r'|\.cases\.<id>\.model\..*|\.cases\.<id>\.purpose'
                         r'|\.cases\.<id>\.cancellation\.(recommended_scale|gross_scale|gross)'
                         r'|\.cases\.<id>\.negative_controls\[\]\.(description|defect_form|other_forms))$')
    res['outside_stated_scope'] = [d for d in diffs if not allowed.match(classify(d[0]))]
    # F1: every changed model input: new string parses to a rational within 1e-26 of the old printed value,
    # and is exact in the sense of not being a 28-significant-digit rounding
    changed_inputs = []
    for d in diffs:
        if '.model.' in d[0] and d[1] == 'value':
            old, new = parse(d[2]), parse(d[3])
            rel = abs(new - old) / abs(new) if new else abs(old)
            changed_inputs.append((d[0], d[2][:48], d[3][:60], format(D(rel.numerator) / D(rel.denominator), '.2e')))
    res['F1_changed_inputs'] = changed_inputs
    res['F1_changed_cases'] = sorted({re.match(r'\.cases\.([^.]+)', x[0]).group(1) for x in changed_inputs})
    # every model input string in revision 2 parses; count significant digits of decimal mantissas
    long_mant = []
    nparsed = 0

    def inputs(x, path):
        nonlocal nparsed
        if isinstance(x, str):
            if re.fullmatch(r'[-+0-9.e/*^]+', x) and re.search(r'\d', x):
                parse(x)
                nparsed += 1
                m = re.fullmatch(r'-?([0-9.]+)(e[-+]?\d+)?(\*2\^-?\d+)?', x)
                if m:
                    digs = m.group(1).replace('.', '').strip('0')
                    if len(digs) >= 28:
                        long_mant.append((path, x[:60], len(digs)))
        elif isinstance(x, list):
            for i, y in enumerate(x):
                inputs(y, path + '[%d]' % i)
        elif isinstance(x, dict):
            for k, v in x.items():
                if k in ('generator',):
                    continue
                inputs(v, path + '.' + k)
    for cid, c in r2['cases'].items():
        inputs(c['model'], cid + '.model')
    res['F1_inputs_parsed'] = nparsed
    res['F1_long_mantissas_(>=28 sig digits)'] = long_mant
    # F3
    rec = [(cid, c['cancellation']['recommended_scale']) for cid, c in r2['cases'].items() if c['family'] == 'RF-CANCEL']
    need = ['BINDING', 'never exceeds the class scale', 'exactly relative', 'never below |exp|']
    res['F3_criterion'] = r2['criterion']
    res['F3_recommended_texts'] = len(rec)
    res['F3_texts_missing_phrases'] = [(cid, [p for p in need if p not in t]) for cid, t in rec if any(p not in t for p in need)]
    below = 0
    above_class = 0
    for cid, c in r2['cases'].items():
        if c['family'] != 'RF-CANCEL':
            continue
        sc = {k: D(v['value']) for k, v in c['scales'].items()}
        for k, e, cl, rs, gs, gov in c['expected']:
            e, rs = D(e), D(rs)
            if rs < abs(e) * (1 - D('1e-20')):
                below += 1
                assert gov == 'mixed', (cid, k, gov)
            if rs > sc[cl] * (1 + D('1e-20')):
                above_class += 1
    res['F3_rows_rec_below_exp_(all mixed)'] = below
    res['F3_rows_rec_above_class'] = above_class
    # F4, F9 texts
    res['F4'] = {cid: {k: v for k, v in n.items() if k in ('description', 'discriminates', 'defect_form', 'other_forms', 'at',
                                                             'max_violation_ratio')}
                 for cid, c in r2['cases'].items() if cid.startswith('RF-WEAK-W-L')
                 for n in c['negative_controls'] if n['id'] == 'NC-WRONG-TRANSFORM'}
    res['F9'] = {cid: n['description'] for cid, c in r2['cases'].items() if c['family'] == 'RF-MECH'
                 for n in c['negative_controls'] if n['id'] == 'NC-RESTRAINT-COUNT'}
    json.dump(res, open(sys.argv[3], 'w'), indent=1, default=str)
    print('differences: %d' % len(diffs))
    for k, v in res['by_path_class'].items():
        print('  %6d  %s' % (v, k))
    print('outside stated scope: %d' % len(res['outside_stated_scope']))
    for d in res['outside_stated_scope'][:20]:
        print('   ', d)
    print('F1 changed model inputs: %d in %d cases: %s' % (len(changed_inputs), len(res['F1_changed_cases']), res['F1_changed_cases']))
    print('F1 max relative change of a changed input: %s' % max((x[3] for x in changed_inputs), key=lambda s: D(s)))
    print('F1 revision-2 model input strings parsed: %d; mantissas with >= 28 significant digits: %d' % (nparsed, len(long_mant)))
    for x in long_mant[:10]:
        print('   ', x)
    print('F3 recommended_scale texts: %d; missing phrases: %s' % (len(rec), res['F3_texts_missing_phrases']))
    print('F3 rows with recommended < |exp|: %d (all mixed); rows with recommended > class: %d' % (below, above_class))


if __name__ == '__main__':
    main()
