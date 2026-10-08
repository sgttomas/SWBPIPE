#!/usr/bin/env python3
"""RV115 ADDENDUM_02 checks of DEF-C's numerical content (standard library only).

argv: DEF-O path, DEF-C draft path, the R-7 fixture path.
Reads those three committed or recorded files only; imports nothing from the
repository (the canonical form is re-implemented here, not the checked-JSON
executable); writes only to stdout.

1. R-10: H(domain, definition) = sha256(JCS({"domain": domain, "payload": definition})),
   with DEF-O's a7ed7ca0... as the control; DEF-C under the reused domain and
   under the alternative domain; raw bytes equal to their canonical form.
2. DEF-C against DEF-O, member by member: every changed leaf path, and the
   members the proof relies on that must be byte-identical.
3. R-7 on the fixture, counted independently: n, m, g from distinct entity
   references (not from the case-row arithmetic), per-member site coverage,
   unique row keys, absent families, and the case's extra rows.
"""
import hashlib, json, sys
from collections import Counter, defaultdict

def jcs(value):
    # JCS for this value class: no floats, ASCII-sortable keys (asserted below).
    return json.dumps(value, ensure_ascii=False, separators=(',', ':'), sort_keys=True, allow_nan=False)

def no_floats(v):
    if isinstance(v, float):
        return False
    if isinstance(v, dict):
        return all(isinstance(k, str) and k.isascii() for k in v) and all(no_floats(x) for x in v.values())
    if isinstance(v, list):
        return all(no_floats(x) for x in v)
    return True

def H(domain, definition):
    return hashlib.sha256(jcs({'domain': domain, 'payload': definition}).encode('utf-8')).hexdigest()

def leaves(v, path=''):
    if isinstance(v, dict):
        out = {}
        for k in v:
            out.update(leaves(v[k], f'{path}/{k}'))
        return out
    return {path: json.dumps(v, sort_keys=True)}

def_o_path, def_c_path, fixture_path = sys.argv[1:4]
raw_o, raw_c = open(def_o_path, 'rb').read(), open(def_c_path, 'rb').read()
o, c = json.loads(raw_o), json.loads(raw_c)
out = {}
assert no_floats(o) and no_floats(c)
out['1_hash'] = {
    'DEF_O_raw_sha256': hashlib.sha256(raw_o).hexdigest(),
    'DEF_O_H_formation_v1': H('retained_precision_formation_v1', o),
    'DEF_O_control_matches_a7ed7ca0': H('retained_precision_formation_v1', o).startswith('a7ed7ca0bf0bba6e'),
    'DEF_C_raw_sha256': hashlib.sha256(raw_c).hexdigest(),
    'DEF_C_raw_is_canonical': jcs(c).encode('utf-8') == raw_c,
    'DEF_O_raw_is_canonical': jcs(o).encode('utf-8') == raw_o,
    'DEF_C_ascii_only': all(b < 128 for b in raw_c),
    'DEF_C_H_formation_v1': H('retained_precision_formation_v1', c),
    'DEF_C_H_alternative_domain': H('retained_precision_combination_formation_v1', c),
    'DEF_C_operand_definition_equals_H_DEF_O': c['inherits']['operand_definition']['sha256'] == H('retained_precision_formation_v1', o),
}

lo, lc = leaves(o), leaves(c)
changed = sorted(p for p in set(lo) | set(lc) if lo.get(p) != lc.get(p))
must_equal = [p for p in lo if p.startswith(('/precision', '/projection', '/acceptance/final', '/acceptance/native',
              '/acceptance/standing', '/acceptance/warrant', '/acceptance/scoped', '/lanes/order', '/lanes/per_lane',
              '/lanes/private_only', '/lanes/reuse', '/lanes/annular_seed', '/lanes/annular_source', '/rows/component',
              '/rows/displacement_magnitude', '/rows/support_magnitude', '/hash_domains/publication',
              '/hash_domains/receipt', '/hash_domains/source', '/hash_domains/definition', '/scope/materials',
              '/scope/supports', '/scope/source', '/scope/entry', '/scope/requires', '/scope/scope_limit', '/trust/G7',
              '/trust/attestation', '/trust/validation', '/work/auxiliary', '/work/finalization', '/inherits/base_contract',
              '/inherits/canonicalization', '/inherits/facade_policy', '/inherits/kernel_policy', '/inherits/method',
              '/inherits/projection_policy', '/inherits/semantic_contract', '/inherits/work_policy'))]
out['2_members'] = {
    'changed_leaf_paths': changed,
    'proof_members_byte_identical_to_DEF_O': {p: lo[p] == lc.get(p) for p in sorted(must_equal)},
    'all_proof_members_identical': all(lo[p] == lc.get(p) for p in must_equal),
    'DEF_O_preparation_domain_present': 'preparation' in o['hash_domains'],
    'DEF_C_preparation_domain_present': 'preparation' in c['hash_domains'],
    'DEF_C_excludes_minus_DEF_O': sorted(set(c['scope']['excludes']) - set(o['scope']['excludes'])),
    'DEF_O_excludes_minus_DEF_C': sorted(set(o['scope']['excludes']) - set(c['scope']['excludes'])),
}

env = json.load(open(fixture_path, encoding='utf-8'))
fixture_sha = hashlib.sha256(open(fixture_path, 'rb').read()).hexdigest()
by = defaultdict(list)
for r in env['results']:
    b = r.get('basis_ref') or {}
    by[(b.get('ref_type'), b.get('ref_id'))].append(r)
cases = [k for k in by if k[0] == 'load_case']
combos = [k for k in by if k[0] == 'combination']
def census(rows):
    nodes = {r['entity_ref'] for r in rows if r['kind'].startswith('global_nodal_')}
    pipes = {r['entity_ref'] for r in rows if r['kind'].startswith('element_local_')}
    supports = {r['entity_ref'] for r in rows if r['kind'].startswith('support_reaction_')}
    return len(nodes), len(pipes), len(supports)
res3 = {'fixture_sha256': fixture_sha, 'summary': {k: env['summary'].get(k) for k in ('node_count', 'segment_count', 'support_count', 'load_case_count', 'component_stress_modifier_count', 'spring_hanger_user_input_count')}}
for key in combos:
    rows = by[key]
    n, m, g = census(rows)
    keys = [(r['kind'], r['entity_ref'], (r.get('metadata') or {}).get('location'), (r.get('metadata') or {}).get('component')) for r in rows]
    sites = defaultdict(Counter)
    for r in rows:
        if r['kind'].startswith('element_local_'):
            sites[r['entity_ref']][(r.get('metadata') or {}).get('location')] += 1
    fam = Counter()
    for r in rows:
        k = r['kind']
        if k.startswith('global_nodal_'): fam['displacement_components'] += 1
        elif k == 'displacement_magnitude': fam['displacement_magnitudes'] += 1
        elif k.startswith('element_local_') and '_stress' in k: fam['stresses'] += 1
        elif k.startswith('element_local_'): fam['actions'] += 1
        elif k == 'support_reaction_component_v2': fam['support_components'] += 1
        elif k.startswith('support_reaction_') and 'magnitude' in k: fam['support_magnitudes'] += 1
        else: fam['other:' + k] += 1
    res3[key[1].encode('ascii', 'backslashreplace').decode()] = {
        'n_m_g_from_entities': [n, m, g], 'rows': len(rows), '7n_50m_8g': 7 * n + 50 * m + 8 * g,
        'families': dict(fam),
        'expected_families': {'displacement_components': 6 * n, 'displacement_magnitudes': n, 'actions': 30 * m,
                              'stresses': 20 * m, 'support_components': 6 * g, 'support_magnitudes': 2 * g},
        'unique_row_keys': len(set(keys)) == len(keys),
        'unique_ids': len({r['id'] for r in rows}) == len(rows),
        'sites_per_member': {e.encode('ascii', 'backslashreplace').decode(): dict(s) for e, s in sites.items()},
    }
for key in cases:
    rows = by[key]
    n, m, g = census(rows)
    res3['case:' + key[1].encode('ascii', 'backslashreplace').decode()] = {
        'n_m_g_from_entities': [n, m, g], 'rows': len(rows), '7n_51m_8g': 7 * n + 51 * m + 8 * g,
        'kinds_outside_7n_51m_8g': {k: v for k, v in Counter(r['kind'] for r in rows).items()
                                     if k in ('linear_solver_mode_basis', 'component_equal_factor_intensified_bending_stress_v1',
                                              'modulus_basis_record', 'sparse_live_path_dense_parity_relative_delta')}}
out['3_R7'] = res3
for key in combos:
    e = res3[key[1].encode('ascii', 'backslashreplace').decode()]
    assert e['rows'] == e['7n_50m_8g'] and e['families'] == e['expected_families'] and e['unique_row_keys']
print(json.dumps(out, indent=1, sort_keys=True, ensure_ascii=True))
