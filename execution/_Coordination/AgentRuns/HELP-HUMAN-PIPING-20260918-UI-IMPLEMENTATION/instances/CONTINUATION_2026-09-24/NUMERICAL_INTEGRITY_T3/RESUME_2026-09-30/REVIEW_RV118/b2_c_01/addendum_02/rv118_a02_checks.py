"""RV118 addendum 02: independent checks of I97's B2-C revision 02 (read-only; standard library only).

It imports nothing from I96, I97 or production code. It loads, by path and unchanged, RV118's own sealed scripts:
addendum_01/rv118_r1_checks.py (the JCS canonicalizer, H and the leaf diff) and addendum_01/rv118_a01_option_ii.py
(rn64_sqrt and exact_square_sum).
  1. DEF-C r2: its H under retained_precision_formation_v1 and R-10's alternative domain, with DEF-O's H as the
     control; DEF-C r2 is its own JCS form, ASCII, no newline; operand_definition.sha256 == H(DEF-O); support_magnitude
     == DEF-O's; the leaf paths that change from r1; the two changed texts appear verbatim in REVISION_02.md.
  2. PTABLE r2: the leaf paths that change from r1; the formation hashes recompute (DEF-O's and DEF-C r2's H);
     receipt_bindings == XTABLE's (JCS); indent-2 ASCII with one trailing newline; the scope text is r1's.
  3. The J1 SCHEMA text contains neither H(DEF-C r2) nor the raw hashes of DEF-C r2 or PTABLE r2.
  4. exact_norm_vectors.json: every vector recomputed with RV118's rn64_sqrt (an overflow raised by its final ldexp,
     or a rounded value of 2^1024 or more, counts as refused); bits and tie flags compared.
  5. NC-3's TS test (REVISION_02 §4.5): for the vectors it names, whether p*(1 + 2^-40) leaves the reader guard
     |p' - r| <= 64*eps*max(|p'|, MIN_POSITIVE), r = this host's nested hypot (the RE/PY/TS formula).
  6. REVISION_02 §2.2 step 4 at y0 = 2^-1022: a triple whose exact norm lies between the true lower midpoint
     (2^-1022 - 2^-1075) and the "one half as far" one (2^-1022 - 2^-1076); whether exact_norm_vectors.json has it.

Usage: python rv118_a02_checks.py <P> <addendum_01 dir> <I97 b2_c_01 dir> <XTABLE> <out_json>
"""
import importlib.util
import json
import math
import struct
import sys
from pathlib import Path

EPS = 2.0 ** -52
MIN_POSITIVE = 2.0 ** -1022


def load(path, name):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def f64(h):
    return struct.unpack('>d', bytes.fromhex(h))[0]


def hx(v):
    return struct.pack('>d', v).hex()


def mine(rv, x, y, z):
    try:
        p, tie = rv.rn64_sqrt(rv.exact_square_sum(x, y, z))
    except OverflowError:
        return 'refused', False
    if not math.isfinite(p):
        return 'refused', False
    return hx(p), tie


def guard_ok(p, x, y, z):
    r = math.hypot(math.hypot(x, y), z)
    return abs(p - r) <= 64 * EPS * max(abs(p), MIN_POSITIVE)


def main():
    p, a01, o, x_path, out = (Path(a) for a in sys.argv[1:6])
    c = load(a01 / 'rv118_r1_checks.py', 'rv118_r1_checks')
    rv = load(a01 / 'rv118_a01_option_ii.py', 'rv118_a01_option_ii')
    res = p / 'fixtures/results'
    raw = {
        'DEF-O': (res / 'retained_precision_prepared_ordinary_v1.json').read_bytes(),
        'DEF-C r1': (o / 'statics/r1/retained_precision_prepared_combination_v1.json').read_bytes(),
        'DEF-C r2': (o / 'statics/r2/retained_precision_prepared_combination_v1.json').read_bytes(),
        'PTABLE r1': (o / 'statics/r1/semantic_contract_v0_3_preview_physics_retained_1.json').read_bytes(),
        'PTABLE r2': (o / 'statics/r2/semantic_contract_v0_3_preview_physics_retained_1.json').read_bytes(),
        'SCHEMA J1': (o / 'statics/retained_precision_mp_v2.schema.json').read_bytes(),
        'XTABLE': x_path.read_bytes(),
    }
    j = {k: json.loads(v) for k, v in raw.items()}
    rev2 = (o / 'REVISION_02.md').read_text(encoding='utf-8')
    h_defo = c.H('retained_precision_formation_v1', j['DEF-O'])
    h_defc2 = c.H('retained_precision_formation_v1', j['DEF-C r2'])
    r = {'inputs': {k: c.sha(v) for k, v in raw.items()}}
    d1, d2 = j['DEF-C r1'], j['DEF-C r2']
    r['DEF-C r2'] = {
        'H (my JCS)': h_defc2,
        'H, alternative domain': c.H('retained_precision_combination_formation_v1', d2),
        'DEF-O H (control)': h_defo,
        'raw is its own JCS': c.jcs(d2).encode() == raw['DEF-C r2'],
        'ascii only, no trailing newline': all(b < 128 for b in raw['DEF-C r2']) and not raw['DEF-C r2'].endswith(b'\n'),
        'operand_definition.sha256 == H(DEF-O)': d2['inherits']['operand_definition']['sha256'] == h_defo,
        'support_magnitude == DEF-O support_magnitude': d2['rows']['support_magnitude'] == j['DEF-O']['rows']['support_magnitude'],
        'r1_to_r2_changed_leaves': c.leaves(d1, d2),
        'rows.displacement_magnitude verbatim in REVISION_02 §1.1': '"' + d2['rows']['displacement_magnitude'] + '"' in rev2,
        'stages.observables verbatim in REVISION_02 §3': '"' + d2['stages']['observables'] + '"' in rev2,
        'stages.entered': d2['stages']['entered'],
    }
    p1, p2 = j['PTABLE r1'], j['PTABLE r2']
    r['PTABLE r2'] = {
        'r1_to_r2_changed_leaves': c.leaves(p1, p2),
        'formation hashes recompute': [e['sha256'] == {'RP-PREPARED-ORDINARY-DUAL-v1': h_defo,
                                                       'RP-PREPARED-COMBINATION-DUAL-v1': h_defc2}.get(e['id'])
                                       for e in p2['product_formation_definitions']],
        'receipt_bindings == XTABLE (JCS)': c.jcs(p2['receipt_bindings']) == c.jcs(j['XTABLE']['receipt_bindings']),
        'scope == r1 scope': p2['accuracy_classification']['scope'] == p1['accuracy_classification']['scope'],
        'indent-2 ASCII, one trailing newline': (json.dumps(p2, indent=2, ensure_ascii=True) + '\n').encode() == raw['PTABLE r2'],
    }
    schema_text = raw['SCHEMA J1'].decode('ascii')
    r['SCHEMA J1'] = {name: (needle[:8] in schema_text) for name, needle in
                      (('H(DEF-C r2) present', h_defc2), ('DEF-C r2 raw present', c.sha(raw['DEF-C r2'])),
                       ('PTABLE r2 raw present', c.sha(raw['PTABLE r2'])))}
    vec = json.loads((o / '_run_records/r2/exact_norm_vectors.json').read_text())['vectors']
    diffs, refused, ties = [], 0, 0
    for v in vec:
        x, y, z = (f64(v[k]) for k in 'xyz')
        got, tie = mine(rv, x, y, z)
        refused += got == 'refused'
        ties += tie
        if got != v['p'] or tie != v['tie']:
            diffs.append({'label': v['label'], 'theirs': [v['p'], v['tie']], 'mine': [got, tie]})
    r['vectors'] = {'count': len(vec), 'agree': len(vec) - len(diffs), 'refused': refused, 'ties': ties,
                    'differences': diffs}
    named = []
    for v in vec:
        lab = v['label']
        if v['p'] == 'refused':
            continue
        if not ('A-5 midpoint, ties to even' == lab or 'A-5 midpoint plus 2^-1074' in lab or 'subnormal' in lab.lower()
                or '1e200' in lab or '1e-200' in lab or '1e308' in lab or 'MIN_POSITIVE' in lab):
            continue
        x, y, z = (f64(v[k]) for k in 'xyz')
        pv = f64(v['p'])
        edited = pv * (1 + 2.0 ** -40)
        named.append({'label': lab, 'p': v['p'], 'p_is_subnormal_or_zero': pv < MIN_POSITIVE,
                      'p accepted by the guard': guard_ok(pv, x, y, z),
                      'p*(1+2^-40) equals p': edited == pv,
                      'p*(1+2^-40) refused by the guard': not guard_ok(edited, x, y, z),
                      'p + 256 subnormal ulps refused': not guard_ok(pv + 256 * 2.0 ** -1074, x, y, z)})
    r['nc3_ts_test_vectors'] = {'vectors': named,
                                'edit_not_refused': [n['label'] for n in named if not n['p*(1+2^-40) refused by the guard']]}
    # 6. REVISION_02 §2.2 step 4 at y0 = 2^-1022: the lower neighbour is the largest subnormal, one full quantum
    #    (2^-1074) below, so the lower midpoint is 2^-1022 - 2^-1075; "one half as far" would put it at 2^-1022 - 2^-1076.
    from fractions import Fraction
    x, y, z = math.nextafter(MIN_POSITIVE, 0.0), 2.0 ** -1048, 0.0
    s = rv.exact_square_sum(x, y, z)
    p_edge, tie_edge = rv.rn64_sqrt(s)
    m_true = Fraction(MIN_POSITIVE) - Fraction(1, 2 ** 1075)
    m_half = Fraction(MIN_POSITIVE) - Fraction(1, 2 ** 1076)
    r['step4_edge_at_min_positive'] = {
        'x': hx(x), 'y': hx(y), 'z': hx(z), 'rn64': hx(p_edge), 'tie': tie_edge,
        'rn64 == MIN_POSITIVE': p_edge == MIN_POSITIVE,
        'lower neighbour of MIN_POSITIVE': hx(math.nextafter(MIN_POSITIVE, 0.0)),
        'S >= (2^-1022 - 2^-1075)^2 (true midpoint: stay at MIN_POSITIVE)': s >= m_true * m_true,
        'S < (2^-1022 - 2^-1076)^2 (the half-as-far midpoint: a step down)': s < m_half * m_half,
        'in exact_norm_vectors.json': any((v['x'], v['y'], v['z']) == (hx(x), hx(y), hx(z)) for v in vec),
    }
    text = json.dumps(r, indent=1, sort_keys=True) + '\n'
    Path(out).write_text(text, encoding='ascii')
    print(json.dumps({'DEF-C r2': {k: r['DEF-C r2'][k] for k in ('H (my JCS)', 'r1_to_r2_changed_leaves')},
                      'PTABLE r2': r['PTABLE r2']['r1_to_r2_changed_leaves'],
                      'vectors': {k: r['vectors'][k] for k in ('count', 'agree', 'refused', 'ties')},
                      'nc3 edit_not_refused': r['nc3_ts_test_vectors']['edit_not_refused']}, indent=1))


if __name__ == '__main__':
    main()
