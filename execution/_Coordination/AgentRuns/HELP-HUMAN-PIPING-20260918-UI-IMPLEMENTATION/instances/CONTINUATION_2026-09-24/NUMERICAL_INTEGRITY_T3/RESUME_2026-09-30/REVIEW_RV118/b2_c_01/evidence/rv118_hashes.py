"""RV118 (RV-C): independent hash and statics checks of I97's B2-C draft statics (read-only).

Standard library only. It imports no I96 or I97 code and no production code; its canonicalizer is
written here from RFC 8785 (JCS) for the value classes these statics contain (objects, arrays,
strings, safe integers, booleans, null; no binary64 fraction is accepted).

Usage: python rv118_hashes.py <P> <statics dir> <XTABLE> <out_json>
  <P>        NUM's projects/chirality-piping (committed bytes)
  <statics>  a folder holding I97's five statics (the committed record folder, or a rebuild)
  <XTABLE>   I96's statics/r1/semantic_contract_v0_3_physics_retained_1.json
"""
import hashlib
import json
import sys
from pathlib import Path


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


def utf16_key(s):
    return s.encode('utf-16-be')


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
        raise ValueError('binary64 value present; not expected in these statics')
    if type(v) is str:
        return jcs_string(v)
    if type(v) is list:
        return '[' + ','.join(jcs(x) for x in v) + ']'
    if type(v) is dict:
        keys = sorted(v, key=utf16_key)
        return '{' + ','.join(jcs_string(k) + ':' + jcs(v[k]) for k in keys) + '}'
    raise TypeError(type(v))


def H(domain, payload):
    return hashlib.sha256(jcs({'domain': domain, 'payload': payload}).encode('utf-8')).hexdigest()


def sha(b):
    return hashlib.sha256(b).hexdigest()


def main():
    p, statics, xpath, out = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3]), Path(sys.argv[4])
    res = p / 'fixtures/results'
    defo_raw = (res / 'retained_precision_prepared_ordinary_v1.json').read_bytes()
    defc_raw = (statics / 'retained_precision_prepared_combination_v1.json').read_bytes()
    pt_raw = (res / 'semantic_contract_v0_3_preview_physics_retained_1.json').read_bytes()
    pt2_raw = (statics / 'semantic_contract_v0_3_preview_physics_retained_1.json').read_bytes()
    x_raw = xpath.read_bytes()
    defo, defc, pt, pt2, x = (json.loads(b) for b in (defo_raw, defc_raw, pt_raw, pt2_raw, x_raw))
    r = {}
    r['inputs'] = {'DEF-O raw': sha(defo_raw), 'DEF-C raw': sha(defc_raw), 'PTABLE (main)': sha(pt_raw),
                   'PTABLE revised': sha(pt2_raw), 'XTABLE': sha(x_raw)}
    r['DEF-O'] = {'raw_is_jcs': jcs(defo).encode() == defo_raw,
                  'H_formation_v1': H('retained_precision_formation_v1', defo)}
    r['DEF-C'] = {'raw_is_jcs': jcs(defc).encode() == defc_raw, 'bytes': len(defc_raw),
                  'ascii_only': all(b < 128 for b in defc_raw), 'trailing_newline': defc_raw.endswith(b'\n'),
                  'H_formation_v1': H('retained_precision_formation_v1', defc),
                  'H_alt_domain': H('retained_precision_combination_formation_v1', defc),
                  'inherits_operand_definition': defc['inherits'].get('operand_definition'),
                  'operand_definition_sha_equals_DEF-O_H':
                      defc['inherits'].get('operand_definition', {}).get('sha256') == H('retained_precision_formation_v1', defo)}
    # Member-by-member delta against DEF-O (two levels).
    delta = {'top_added': sorted(set(defc) - set(defo)), 'top_removed': sorted(set(defo) - set(defc)), 'changed': []}
    for k in sorted(set(defc) & set(defo)):
        a, b = defo[k], defc[k]
        if a == b:
            continue
        if isinstance(a, dict) and isinstance(b, dict):
            for kk in sorted(set(a) | set(b)):
                if a.get(kk) != b.get(kk):
                    delta['changed'].append(f'{k}.{kk}' + (' (added)' if kk not in a else ' (removed)' if kk not in b else ''))
        else:
            delta['changed'].append(k)
    r['DEF-C_vs_DEF-O'] = delta
    r['excludes'] = {'DEF-O': defo['scope']['excludes'], 'DEF-C': defc['scope']['excludes']}
    # PTABLE.
    changed = [k for k in pt if pt2.get(k) != pt[k]]
    added = [k for k in pt2 if k not in pt]
    r['PTABLE'] = {
        'members_changed': changed, 'members_added': added, 'members_removed': [k for k in pt if k not in pt2],
        'pfd': pt2['product_formation_definitions'],
        'pfd_hashes_recomputed': [e['sha256'] == {'RP-PREPARED-ORDINARY-DUAL-v1': H('retained_precision_formation_v1', defo),
                                                    'RP-PREPARED-COMBINATION-DUAL-v1': H('retained_precision_formation_v1', defc)}.get(e['id'])
                                  for e in pt2['product_formation_definitions']],
        'accuracy_classification_keys_changed': [k for k in pt['accuracy_classification']
                                                 if pt2['accuracy_classification'].get(k) != pt['accuracy_classification'][k]],
        'warrant_type': type(pt2['formation_warrant']).__name__,
        'warrant_first_equals_old': pt2['formation_warrant'][0] == pt['formation_warrant'],
        'warrant_second_diff_paths': [k for k in pt['formation_warrant'] if pt2['formation_warrant'][1].get(k) != pt['formation_warrant'][k]],
        'receipt_bindings_equal_XTABLE_jcs': jcs(pt2['receipt_bindings']) == jcs(x['receipt_bindings']),
        'receipt_bindings_equal_XTABLE_indented_text': json.dumps(pt2['receipt_bindings'], indent=2) == json.dumps(x['receipt_bindings'], indent=2),
        'receipt_bindings_key_order': list(pt2['receipt_bindings']),
        'XTABLE_key_order': list(x['receipt_bindings']),
        'serialization_indent2_ascii_newline': (json.dumps(pt2, indent=2, ensure_ascii=True) + '\n').encode() == pt2_raw,
        'receipt_bindings_vs_PTABLE_policy': {
            'receipt_policy': pt2['receipt_policy'], 'facade_policy': pt2['accuracy_classification']['policy']},
        'XTABLE_warrant_type': type(x['formation_warrant']).__name__,
    }
    out.write_text(json.dumps(r, indent=2, sort_keys=True) + '\n')
    print(json.dumps(r, indent=1, sort_keys=True))


if __name__ == '__main__':
    main()
