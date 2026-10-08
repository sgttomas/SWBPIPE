"""RV118 addendum 01: independent checks of I97's B2-C revision 01 (read-only; standard library only).

It imports nothing from I96, I97 or production code.
  1. JCS hashes (my canonicalizer, as in RV118's rv118_hashes.py): DEF-C r1's H under retained_precision_formation_v1 and
     under R-10's alternative domain, with DEF-O's pinned H as the control; DEF-C r1 is its own JCS form.
  2. The member deltas: DEF-C v0 -> r1 (every changed leaf path), DEF-C r1 against DEF-O for support_magnitude, and
     PTABLE r1 against the v0 draft and against main; receipt_bindings against XTABLE; the formation hashes recompute.
  3. S-2: my own model of the producer's append order (PP lib.rs: the case rows; then, per combination in authored
     order, that combination's rows in one iteration; then, per subtraction or range combination in authored order,
     one combination_modulus_basis_record per operand), over every in-domain shape (c + z <= 3, z >= 1; mechanics with
     distinct cases, mechanics with a repeated case (rowless, C-1), range over each non-empty operand set, subtraction
     over each ordered pair). Revision 01's T-6' rule against v0's.
  4. S-4 (a): the 64-epsilon guard of G7's combination_magnitudes, |p - r| <= 64*eps*max(|p|, MIN_POSITIVE), where p
     and r are binary64 hypot(hypot(x,y),z) of the same three published components, each hypot call taken faithful
     (the correctly rounded value or one ulp away). Random triples over the normal and subnormal ranges, with signed
     zeros and equal or widely separated magnitudes; every combination of per-call perturbations for p and for r.

Usage: python rv118_r1_checks.py <P> <DEF-C v0> <statics r1 dir> <PTABLE v0 draft> <XTABLE> <out_json>
"""
import hashlib
import itertools
import json
import math
import random
import sys
from pathlib import Path

EPS = 2.0 ** -52
MIN_POSITIVE = 2.0 ** -1022


def jcs_string(s):
    out = ['"']
    for ch in s:
        o = ord(ch)
        if ch == '"':
            out.append('\\"')
        elif ch == '\\':
            out.append('\\\\')
        elif o == 0x08:
            out.append('\\b')
        elif o == 0x0C:
            out.append('\\f')
        elif o == 0x0A:
            out.append('\\n')
        elif o == 0x0D:
            out.append('\\r')
        elif o == 0x09:
            out.append('\\t')
        elif o < 0x20:
            out.append('\\u%04x' % o)
        else:
            out.append(ch)
    out.append('"')
    return ''.join(out)


def jcs(v):
    if v is None:
        return 'null'
    if v is True:
        return 'true'
    if v is False:
        return 'false'
    if type(v) is int:
        if abs(v) > 9007199254740991:
            raise ValueError('unsafe integer')
        return str(v)
    if type(v) is float:
        raise ValueError('binary64 value present')
    if type(v) is str:
        return jcs_string(v)
    if type(v) is list:
        return '[' + ','.join(jcs(x) for x in v) + ']'
    if type(v) is dict:
        return '{' + ','.join(jcs_string(k) + ':' + jcs(v[k]) for k in sorted(v, key=lambda s: s.encode('utf-16-be'))) + '}'
    raise TypeError(type(v))


def H(domain, payload):
    return hashlib.sha256(jcs({'domain': domain, 'payload': payload}).encode('utf-8')).hexdigest()


def sha(b):
    return hashlib.sha256(b).hexdigest()


def leaves(a, b, prefix=''):
    out = []
    for k in sorted(set(a) | set(b)):
        x, y = a.get(k), b.get(k)
        if isinstance(x, dict) and isinstance(y, dict):
            out += leaves(x, y, prefix + k + '.')
        elif x != y:
            out.append(prefix + k + (' (added)' if k not in a else ' (removed)' if k not in b else ''))
    return out


# ---- 3. S-2 ---------------------------------------------------------------------------------------------------
def combination_types(c):
    cases = ['A', 'B'][:c]
    types = [('mech', None), ('mech_repeated', None)]
    for r in range(1, c + 1):
        for subset in itertools.combinations(cases, r):
            types.append(('range', subset))
    if c == 2:
        types += [('sub', ('A', 'B')), ('sub', ('B', 'A'))]
    return types


def layout(c, combos):
    rows = [('case', x) for x in ['A', 'B'][:c] for _ in range(3)]
    for i, (kind, ops) in enumerate(combos):
        if kind != 'mech_repeated':
            rows += [('comb', i)] * 3
    for i, (kind, ops) in enumerate(combos):
        if kind in ('range', 'sub'):
            rows += [('comb', i)] * len(ops)
    return rows


def contiguous(rows, i):
    pos = [k for k, r in enumerate(rows) if r == ('comb', i)]
    return not pos or pos == list(range(pos[0], pos[0] + len(pos)))


def case_block_ok(rows):
    first = next((k for k, r in enumerate(rows) if r[0] == 'comb'), len(rows))
    return all(r[0] == 'case' for r in rows[:first]) and all(r[0] == 'comb' for r in rows[first:])


def s2():
    shapes = 0
    r1_fail, v0_fail = [], []
    for c in (1, 2):
        for z in range(1, 4 - c):
            for combos in itertools.product(combination_types(c), repeat=z):
                shapes += 1
                rows = layout(c, combos)
                r1 = case_block_ok(rows) and all(contiguous(rows, i) for i, (k, _) in enumerate(combos) if k == 'mech')
                v0 = case_block_ok(rows) and all(contiguous(rows, i) for i in range(z))
                label = f'c={c} ' + ' , '.join(k + ('' if o is None else '(' + ','.join(o) + ')') for k, o in combos)
                if not r1:
                    r1_fail.append(label)
                if not v0:
                    v0_fail.append(label)
    return {'shapes_enumerated': shapes, 'revision_01_rule_fails': r1_fail, 'v0_rule_fails': v0_fail}


# ---- 4. S-4 (a) -----------------------------------------------------------------------------------------------
def faithful(v):
    return [v, math.nextafter(v, math.inf), math.nextafter(v, -math.inf)] if v != 0.0 else [0.0, math.nextafter(0.0, 1.0)]


def nested_variants(x, y, z):
    out = set()
    for inner in faithful(math.hypot(x, y)):
        if inner < 0 or not math.isfinite(inner):
            continue
        for outer in faithful(math.hypot(inner, z)):
            if outer >= 0 and math.isfinite(outer):
                out.add(outer)
    return sorted(out)


def s4(seed, count):
    rnd = random.Random(seed)
    worst, triples, pairs, failures = 0.0, 0, 0, []

    def draw():
        kind = rnd.random()
        if kind < 0.1:
            return rnd.choice([0.0, -0.0])
        if kind < 0.25:
            return rnd.choice([-1, 1]) * rnd.random() * 2.0 ** rnd.randint(-1074, -1022)
        return rnd.choice([-1, 1]) * rnd.random() * 2.0 ** rnd.randint(-1000, 1000)

    cases = []
    for _ in range(count):
        base = draw()
        mode = rnd.random()
        if mode < 0.3:
            cases.append((base, base * (1 + rnd.random() * 1e-3), -base))
        elif mode < 0.5:
            cases.append((base, base * 2.0 ** rnd.randint(-80, -20), base * 2.0 ** rnd.randint(-80, -20)))
        else:
            cases.append((draw(), draw(), draw()))
    for x, y, z in cases:
        if not all(math.isfinite(v) for v in (x, y, z)):
            continue
        variants = nested_variants(x, y, z)
        if not variants:
            continue
        triples += 1
        for p, r in itertools.product(variants, repeat=2):
            pairs += 1
            allowance = 64 * EPS * max(abs(p), MIN_POSITIVE)
            ratio = abs(p - r) / allowance
            worst = max(worst, ratio)
            if ratio > 1:
                failures.append([x.hex(), y.hex(), z.hex(), p.hex(), r.hex()])
    return {'triples': triples, 'pairs': pairs, 'largest_fraction_of_allowance': worst, 'failures': failures[:5],
            'failure_count': len(failures)}


def main():
    p, defc0_path, r1_dir, pt0_path, x_path, out = (Path(a) for a in sys.argv[1:7])
    res = p / 'fixtures/results'
    defo_raw = (res / 'retained_precision_prepared_ordinary_v1.json').read_bytes()
    pt_main_raw = (res / 'semantic_contract_v0_3_preview_physics_retained_1.json').read_bytes()
    defc0_raw = defc0_path.read_bytes()
    defc1_raw = (r1_dir / 'retained_precision_prepared_combination_v1.json').read_bytes()
    pt1_raw = (r1_dir / 'semantic_contract_v0_3_preview_physics_retained_1.json').read_bytes()
    pt0_raw = pt0_path.read_bytes()
    x_raw = x_path.read_bytes()
    defo, defc0, defc1, pt_main, pt0, pt1, x = (json.loads(b) for b in
                                                (defo_raw, defc0_raw, defc1_raw, pt_main_raw, pt0_raw, pt1_raw, x_raw))
    r = {'inputs': {'DEF-O': sha(defo_raw), 'DEF-C v0': sha(defc0_raw), 'DEF-C r1': sha(defc1_raw),
                    'PTABLE main': sha(pt_main_raw), 'PTABLE v0 draft': sha(pt0_raw), 'PTABLE r1': sha(pt1_raw),
                    'XTABLE': sha(x_raw)}}
    h_defo = H('retained_precision_formation_v1', defo)
    h_defc1 = H('retained_precision_formation_v1', defc1)
    r['hashes'] = {
        'DEF-O H (control)': h_defo,
        'DEF-C v0 H': H('retained_precision_formation_v1', defc0),
        'DEF-C r1 H': h_defc1,
        'DEF-C r1 H, alternative domain': H('retained_precision_combination_formation_v1', defc1),
        'DEF-C r1 raw is its own JCS': jcs(defc1).encode() == defc1_raw,
        'DEF-C r1 ascii only, no trailing newline': all(b < 128 for b in defc1_raw) and not defc1_raw.endswith(b'\n'),
        'DEF-C r1 operand_definition.sha256 == H(DEF-O)': defc1['inherits']['operand_definition']['sha256'] == h_defo,
    }
    r['DEF-C'] = {
        'v0_to_r1_changed_leaves': leaves(defc0, defc1),
        'r1 support_magnitude == DEF-O support_magnitude': defc1['rows']['support_magnitude'] == defo['rows']['support_magnitude'],
        'r1 displacement_magnitude': defc1['rows']['displacement_magnitude'],
        'r1 stages.observables': defc1['stages'].get('observables'),
        'r1 stages.entered': defc1['stages']['entered'],
        'r1 scope.operand_equality': defc1['scope']['operand_equality'],
        'r1 lanes.loads': defc1['lanes']['loads'],
    }
    r['PTABLE'] = {
        'v0_draft_to_r1_changed_leaves': leaves(pt0, pt1),
        'main_to_r1_members_changed': [k for k in pt_main if pt1.get(k) != pt_main[k]],
        'main_to_r1_members_added': [k for k in pt1 if k not in pt_main],
        'accuracy_classification keys changed from main': [k for k in pt_main['accuracy_classification']
                                                           if pt1['accuracy_classification'].get(k) != pt_main['accuracy_classification'][k]],
        'scope r1': pt1['accuracy_classification']['scope'],
        'product_formation_definitions': pt1['product_formation_definitions'],
        'pfd hashes recompute': [e['sha256'] == {'RP-PREPARED-ORDINARY-DUAL-v1': h_defo,
                                                 'RP-PREPARED-COMBINATION-DUAL-v1': h_defc1}.get(e['id'])
                                 for e in pt1['product_formation_definitions']],
        'receipt_bindings == XTABLE (JCS)': jcs(pt1['receipt_bindings']) == jcs(x['receipt_bindings']),
        'formation_warrant == v0 draft': pt1['formation_warrant'] == pt0['formation_warrant'],
        'indent-2 ASCII, one trailing newline': (json.dumps(pt1, indent=2, ensure_ascii=True) + '\n').encode() == pt1_raw,
    }
    r['s2_layout'] = s2()
    r['s4_guard'] = s4(118, 20000)
    out.write_text(json.dumps(r, indent=2, sort_keys=True) + '\n')
    print(json.dumps({'hashes': r['hashes'], 'DEF-C changed': r['DEF-C']['v0_to_r1_changed_leaves'],
                      'PTABLE v0->r1': r['PTABLE']['v0_draft_to_r1_changed_leaves'],
                      's2': {k: v for k, v in r['s2_layout'].items()},
                      's4': {k: v for k, v in r['s4_guard'].items() if k != 'failures'}}, indent=1))


if __name__ == '__main__':
    main()
