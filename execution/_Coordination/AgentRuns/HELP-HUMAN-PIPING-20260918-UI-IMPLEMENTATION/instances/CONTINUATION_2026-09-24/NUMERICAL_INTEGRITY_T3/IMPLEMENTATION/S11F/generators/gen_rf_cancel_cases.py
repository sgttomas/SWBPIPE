#!/usr/bin/env python3
"""T3 S11-F (I4): generate the RF-CANCEL product test data from the frozen references.

Usage (standard library only; deterministic):
    python3 gen_rf_cancel_cases.py <references.json> <S11_EXCEPTIONS.json> <FORMATION_EXCEPTIONS.json> <out.json>

Inputs (recorded with their sha256 in the output):
- T3/REFERENCES/references.json at c0f14201c (sha256 7b176dbb...), R1's frozen RF-CANCEL cases;
- T3/GATE/S11_EXCEPTIONS.json, the pinned S11 "no Passed breach" exceptions, as re-pinned by ROOT on
  2026-09-27 (db665f2cb, sha256 1d8979f6...: 221 triples; the earlier 228-triple pin was 8f3687f4...);
- T3/GATE/FORMATION_EXCEPTIONS.json (db665f2cb; ruling text amended at 42b300344, rows unchanged;
  sha256 454bbc24...), the 7 formation and formed-term
  triples (14 rows with mode) that ROOT moved out of the S11 list (owned by S11-G, then F2/F3).

Nothing is hand-copied. For every RF-CANCEL case:
- the product request is authored from the case's own `model` block, each intended decimal input rounded
  once to binary64 (float(Fraction), correctly rounded), in the same shape P1's detection generator used
  (DETECTION/scripts/gen.py.txt `author_defn`: ids load:<n>, contribution:<n>, udl:<n>; nodal loads, then
  the authored contributions, then member loads);
- each published expected row becomes an acceptance interval for the published binary64 value, derived
  exactly (Fraction) from R1's unchanged criterion |obs - exp| <= 1e-9 * max(|exp|, scale) with the BINDING
  net-governed scale (ROOT_RULINGS_V2 item 1), converted exactly to the published unit and rounded INWARD
  to binary64 (lo = smallest double >= the exact lower bound, hi = largest double <= the exact upper
  bound). A value inside [lo, hi] therefore satisfies the exact predicate; the test never accepts a breach.
  Bending magnitudes (hypot of the two published local components) get the interval of My^2 + Mz^2,
  [(exp - tol)^2, (exp + tol)^2] (lower bound 0 when exp - tol <= 0), also inward, which the test checks
  exactly with the kernel's exact accumulator.
- keys the product does not publish (Mb.*.mid, tw.*, ext.*) are recorded as not published, as P1 did.
- the negative controls are copied as R1 states them (id, discriminates, the value at `at`).
The pinned exception triples are copied from S11_EXCEPTIONS.json and the formation rows from
FORMATION_EXCEPTIONS.json, with their counts.
"""
import hashlib
import json
import math
import struct
import sys
from decimal import Decimal
from fractions import Fraction as Fr

PROV = 'invented_t3_s11f_rf_cancel_input_no_library_data'
DOFS = ('UX', 'UY', 'UZ', 'RX', 'RY', 'RZ')
COMP = {'UX': 'Fx', 'UY': 'Fy', 'UZ': 'Fz', 'RX': 'Mx', 'RY': 'My', 'RZ': 'Mz'}
CRIT = Fr(1, 10 ** 9)
TWO53 = Fr(2) ** 53


def sha(path):
    return hashlib.sha256(open(path, 'rb').read()).hexdigest()


def dfr(s):
    return Fr(Decimal(s))


def f64(q):
    return float(Fr(q))


def bits(x):
    return struct.pack('>d', x).hex()


def ceil_double(q):
    """Smallest binary64 value >= q."""
    x = float(q)
    while Fr(x) < q:
        x = math.nextafter(x, math.inf)
    while True:
        y = math.nextafter(x, -math.inf)
        if Fr(y) >= q:
            x = y
        else:
            return x


def floor_double(q):
    """Largest binary64 value <= q."""
    x = float(q)
    while Fr(x) > q:
        x = math.nextafter(x, -math.inf)
    while True:
        y = math.nextafter(x, math.inf)
        if Fr(y) <= q:
            x = y
        else:
            return x


def y_reference(pi, pj):
    e = [dfr(pj[k]) - dfr(pi[k]) for k in range(3)]
    mags = [abs(c) for c in e]
    best = min((1, 2, 0), key=lambda k: mags[k])
    v = [0, 0, 0]
    v[best] = 1
    return {'x': v[0], 'y': v[1], 'z': v[2]}


def nodal_load(lid, node, comp, value):
    if comp < 3:
        return {'id': lid, 'category': 'concentrated_force', 'target': {'type': 'node', 'node': node},
                'direction': 'global_' + 'xyz'[comp], 'magnitude': {'value': value, 'unit': 'N'},
                'dimension': 'force', 'provenance': PROV}
    return {'id': lid, 'category': 'concentrated_moment', 'target': {'type': 'node', 'node': node},
            'direction': DOFS[comp], 'magnitude': {'value': value, 'unit': 'N*m'},
            'dimension': 'moment', 'provenance': PROV}


def author(cid, md):
    model = {
        'schema_version': '0.1.0', 'document_kind': 'openpipestress.product_preview.model',
        'analysis_status': {'mechanics': 'ready_for_preview_diagnostics',
                            'rule_check': 'not_performed_user_rule_inputs_missing',
                            'professional_acceptance': 'not_provided'},
        'project': {'id': 'invented:t3-s11f:' + cid,
                    'units': {'length': 'm', 'force': 'N', 'angle': 'rad', 'pressure': 'Pa',
                              'temperature': 'degC', 'stress': 'Pa'}},
        'nodes': [], 'pipe_segments': [], 'materials': [], 'supports': [],
        'load_cases': [{'id': 'case', 'label': cid, 'kind': 'primitive_user_load', 'primitive_loads': [],
                        'provenance': PROV}],
        'combinations': [],
    }
    for nid, p in md['nodes_m'].items():
        model['nodes'].append({'id': nid, 'position': {'x': f64(dfr(p[0])), 'y': f64(dfr(p[1])), 'z': f64(dfr(p[2]))},
                               'provenance': PROV})
    for sid, s in md['sections'].items():
        E = dfr(s['E'])
        G = dfr(s['G']) if 'G' in s else E / (2 * (1 + dfr(s['nu'])))
        model['materials'].append({'id': 'mat:' + sid, 'elastic_modulus': {'value': f64(E), 'unit': 'Pa'},
                                   'shear_modulus': {'value': f64(G), 'unit': 'Pa'}, 'provenance': PROV})
    for (mid, i, j, sid) in md['members']:
        s = md['sections'][sid]
        OD, ID = dfr(s['OD']), dfr(s['ID'])
        model['pipe_segments'].append({
            'id': mid, 'from': i, 'to': j, 'material': 'mat:' + sid,
            'y_reference': y_reference(md['nodes_m'][i], md['nodes_m'][j]),
            'section': {'outside_diameter': {'value': f64(OD), 'unit': 'm'},
                        'wall_thickness': {'value': f64((OD - ID) / 2), 'unit': 'm'}},
            'provenance': PROV})
    rigid_of = {}
    for nid, s in md.get('supports', {}).items():
        assert not s.get('springs'), cid
        rigid = list(s.get('rigid', []))
        if rigid:
            sup = {'id': 'rigid:' + nid, 'node': nid, 'restraints': rigid, 'provenance': PROV}
            if any(d[0] == 'R' for d in rigid):
                sup['family'] = 'anchor'
            model['supports'].append(sup)
            rigid_of[nid] = 'rigid:' + nid
    loads = model['load_cases'][0]['primitive_loads']
    inputs = []
    n = 0
    for nid, ld in md.get('loads', {}).items():
        for comp in range(6):
            vec = ld.get('F' if comp < 3 else 'M', ('0', '0', '0'))
            v = dfr(vec[comp % 3])
            if v != 0:
                loads.append(nodal_load('load:%d' % n, nid, comp, f64(v)))
                inputs.append(v)
                n += 1
    for (nid, dof, v) in md.get('load_contributions_in_authored_order', []):
        loads.append(nodal_load('contribution:%d' % n, nid, DOFS.index(dof), f64(dfr(v))))
        inputs.append(dfr(v))
        n += 1
    for mid, q in md.get('member_uniform_loads_N_per_m_global', {}).items():
        for a in range(3):
            if dfr(q[a]) != 0:
                loads.append({'id': 'udl:%d' % n, 'category': 'distributed_force',
                              'target': {'type': 'element', 'pipe': mid}, 'direction': 'global_' + 'xyz'[a],
                              'magnitude': {'value': f64(dfr(q[a])), 'unit': 'N/m'},
                              'dimension': 'force_per_length', 'provenance': PROV})
                inputs.append(dfr(q[a]))
                n += 1
    captured_refused = any(abs(Fr(f64(v))) >= TWO53 for v in inputs)
    return {'model': model, 'materials': []}, rigid_of, captured_refused


def interval(e, scale, factor):
    tol = CRIT * max(abs(e), scale)
    return ceil_double((e - tol) * factor), floor_double((e + tol) * factor)


def rows_of(rec, rigid_of):
    out = []
    for (key, exp, cls, scale, _gross, gov) in rec['expected']:
        e, s = dfr(exp), dfr(scale)
        p = key.split('.')
        kind = p[0]
        row = {'key': key, 'governed_by': gov}
        if kind == 'u':
            lo, hi = interval(e, s, Fr(1000))
            row.update(check='value', id='result:disp:%s:%s' % (p[1], p[2].lower()), unit='mm')
        elif kind == 'th':
            lo, hi = interval(e, s, Fr(1))
            row.update(check='value', id='result:disp:%s:%s' % (p[1], p[2].lower()), unit='rad')
        elif kind == 'R':
            lo, hi = interval(e, s, Fr(1))
            row.update(check='support', support=rigid_of[p[1]], component=COMP[p[2]],
                       unit='N' if p[2][0] == 'U' else 'N*m')
        elif kind in ('N', 'T'):
            lo, hi = interval(e, s, Fr(1))
            row.update(check='pair', member=p[1],
                       component='axial_force' if kind == 'N' else 'torsional_moment',
                       unit='N' if kind == 'N' else 'N*m')
        elif kind == 'Mb' and p[2] in ('i', 'j'):
            tol = CRIT * max(abs(e), s)
            lower = e - tol
            lo = 0.0 if lower <= 0 else ceil_double(lower * lower)
            hi = floor_double((e + tol) * (e + tol))
            row.update(check='hypot', member=p[1], location='end_' + p[2], unit='N*m')
        else:
            row.update(check='not_published')
            out.append(row)
            continue
        row.update(lo_bits=bits(lo), hi_bits=bits(hi))
        out.append(row)
    return out


def main():
    ref_path, exc_path, form_path, out_path = sys.argv[1:5]
    refs = json.load(open(ref_path))
    exc = json.load(open(exc_path))
    form = json.load(open(form_path))
    cases = []
    for cid in sorted(c for c in refs['cases'] if c.startswith('RF-CANCEL-')):
        rec = refs['cases'][cid]
        request, rigid_of, refused = author(cid, rec['model'])
        cases.append({
            'id': cid,
            'request': request,
            'captured_refused_at_capture': refused,
            'rows': rows_of(rec, rigid_of),
            'negative_controls': [
                {'id': nc['id'], 'discriminates': nc['discriminates'], 'at': nc.get('at'),
                 'value_at': (nc.get('values') or {}).get(nc.get('at')) if nc.get('at') else None}
                for nc in rec['negative_controls']],
        })
    out = {
        'generator': 'T3/IMPLEMENTATION/S11F/generators/gen_rf_cancel_cases.py',
        'inputs': {'references_json_sha256': sha(ref_path), 's11_exceptions_json_sha256': sha(exc_path),
                   'formation_exceptions_json_sha256': sha(form_path)},
        'criterion': '|obs - exp| <= 1e-9 * max(|exp|, scale), binding net-governed scale; intervals rounded inward',
        'exception_counts': exc['counts'],
        'exceptions': exc['triples'],
        'formation_exception_counts': form['counts'],
        'formation_exceptions': form['rows'],
        'cases': cases,
    }
    with open(out_path, 'w') as fh:
        json.dump(out, fh, indent=1, sort_keys=True)
        fh.write('\n')


if __name__ == '__main__':
    main()
