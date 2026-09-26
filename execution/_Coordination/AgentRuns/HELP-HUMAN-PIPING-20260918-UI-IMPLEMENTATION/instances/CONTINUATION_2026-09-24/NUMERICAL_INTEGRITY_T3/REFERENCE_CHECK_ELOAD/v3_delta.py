"""V3 delta check of RF-ELOAD revision 1 against revision 0 (V3's own check, independent of preserve_check_rev1.py).

Usage: python3 v3_delta.py <rev0 references_eload.json> <rev1 references_eload.json> <out v3_delta.json>
* Byte identity (as JSON strings) of every pre-existing case field except `negative_controls`, `notes` and
  the one allowed new field `cancellation.gross_scale_status`; every expected, represented and
  represented-vs-intended row compared row by row as exact strings.
* Negative controls: every rev0 control present in rev1 with identical numerical fields (discriminates,
  values_failing, worst_key, worst ratio, observed, expected, class/gross flags); lists controls removed,
  added, and every changed text or label.
* Lists the new cases and the top-level fields that changed.
"""
import json
import sys

NUM = ('discriminates', 'values_failing', 'worst_key', 'worst_ratio_to_criterion', 'worst_observed', 'worst_expected',
       'discriminates_under_class_scale', 'discriminates_under_gross_scale')


def main():
    r0 = json.load(open(sys.argv[1]))
    r1 = json.load(open(sys.argv[2]))
    c0 = {c['id']: c for c in r0['cases']}
    c1 = {c['id']: c for c in r1['cases']}
    out = {'new_cases': sorted(set(c1) - set(c0)), 'missing_cases': sorted(set(c0) - set(c1)),
           'top_level_changed': sorted(k for k in set(r0) | set(r1) if k != 'cases' and r0.get(k) != r1.get(k)),
           'field_differences': [], 'row_counts': {'expected': 0, 'expected_represented': 0, 'rep_vs_int': 0},
           'row_differences': [], 'controls_removed': [], 'controls_added': [], 'control_numeric_changes': [],
           'control_text_changes': [], 'other_control_field_changes': []}
    for cid, a in c0.items():
        b = c1[cid]
        for k in sorted(set(a) | set(b)):
            if k in ('negative_controls', 'notes'):
                continue
            va, vb = a.get(k), b.get(k)
            if k == 'cancellation' and isinstance(vb, dict):
                extra = set(vb) - set(va)
                vb2 = {x: y for x, y in vb.items() if x != 'gross_scale_status'}
                if extra - {'gross_scale_status'}:
                    out['field_differences'].append((cid, k, 'unexpected new keys', sorted(extra)))
                if 'gross_scale_status' in vb:
                    out.setdefault('gross_scale_status', {})[cid] = vb['gross_scale_status']
                vb = vb2
            if json.dumps(va, sort_keys=True) != json.dumps(vb, sort_keys=True):
                out['field_differences'].append((cid, k))
        for fld, key in (('expected', 'expected'), ('expected_represented', 'expected_represented'),
                         ('represented_vs_intended_per_quantity', 'rep_vs_int')):
            ra, rb = a.get(fld) or [], b.get(fld) or []
            out['row_counts'][key] += len(ra)
            if len(ra) != len(rb):
                out['row_differences'].append((cid, fld, 'length', len(ra), len(rb)))
            for x, y in zip(ra, rb):
                if x != y:
                    out['row_differences'].append((cid, fld, x, y))
        if a.get('notes') != b.get('notes'):
            out.setdefault('notes_changed', {})[cid] = b.get('notes')
        na = {n['id']: n for n in a['negative_controls']}
        nb = {n['id']: n for n in b['negative_controls']}
        out['controls_removed'] += [(cid, x) for x in na if x not in nb]
        out['controls_added'] += [(cid, x) for x in nb if x not in na]
        for x in na:
            if x not in nb:
                continue
            for f in NUM:
                if na[x].get(f) != nb[x].get(f):
                    out['control_numeric_changes'].append((cid, x, f, na[x].get(f), nb[x].get(f)))
            for f in sorted(set(na[x]) | set(nb[x])):
                if f in NUM or f == 'id':
                    continue
                if na[x].get(f) != nb[x].get(f):
                    (out['control_text_changes'] if f == 'defect' else out['other_control_field_changes']).append(
                        (cid, x, f, na[x].get(f), nb[x].get(f)))
    json.dump(out, open(sys.argv[3], 'w'), indent=1)
    print('new cases:', out['new_cases'], ' missing:', out['missing_cases'])
    print('top-level fields changed:', out['top_level_changed'])
    print('pre-existing rows compared:', out['row_counts'])
    print('field differences (excluding negative_controls, notes, gross_scale_status):', out['field_differences'])
    print('row differences:', out['row_differences'])
    print('gross_scale_status added in:', sorted(out.get('gross_scale_status', {})))
    print('notes changed in:', sorted(out.get('notes_changed', {})))
    print('controls removed:', out['controls_removed'])
    print('controls added:', out['controls_added'])
    print('control numeric changes:', out['control_numeric_changes'])
    print('control text changes:', len(out['control_text_changes']))
    print('other control field changes:', len(out['other_control_field_changes']),
          sorted({(x[1], x[2]) for x in out['other_control_field_changes']}))


if __name__ == '__main__':
    main()
