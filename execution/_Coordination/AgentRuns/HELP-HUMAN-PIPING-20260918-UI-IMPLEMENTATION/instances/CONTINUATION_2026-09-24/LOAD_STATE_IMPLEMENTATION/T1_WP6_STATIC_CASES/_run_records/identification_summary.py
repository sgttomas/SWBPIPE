"""Summarize identification-only producer runs without any result value (T1_WP6_STATIC_CASES).

Usage: python identification_summary.py <raw dir> [<inventory out dir>]
Prints, per case and mode: semantic contract, profile, mechanics status,
numerical standing, blocking and all diagnostic codes, published method and
row count. With an inventory directory it writes the row inventory (every row
field except `value`) used by author_package.py; inventories are scratch only.
"""
import glob
import json
import os
import sys

raw = sys.argv[1]
inventory_dir = sys.argv[2] if len(sys.argv) > 2 else None
paths = sorted(glob.glob(os.path.join(raw, '*.json')), key=lambda p: (not p.endswith('.sparse.json'), p))
skip = ('linear_solver_mode_basis', 'sparse_live_path_dense_parity_relative_delta')
inventories = {}
for path in paths:
    case, mode = os.path.basename(path)[:-5].rsplit('.', 1)
    envelope = json.load(open(path))
    inventory = [{k: v for k, v in row.items() if k != 'value'} for row in envelope['results']]
    comparable = [r for r in inventory if r['kind'] not in skip]
    if mode == 'sparse':
        inventories[case] = comparable
        if inventory_dir:
            json.dump(inventory, open(os.path.join(inventory_dir, f'{case}.inventory.json'), 'w'), indent=1)
    else:
        assert inventories[case] == comparable, f'{case}: selected row inventory differs between modes'
    evidence = envelope['contract_evidence']['load_reference_states']
    diagnostics = envelope['diagnostics']
    print(case, mode, json.dumps({
        'contract': envelope['producer'].get('semantic_contract_id'),
        'profile': envelope['formulation_basis']['profile_id'],
        'mechanics': envelope['status']['mechanics'],
        'standing': envelope['numerical_quality']['status'],
        'case_standing': sorted(set(c['solve_quality'] for c in envelope['numerical_quality']['cases'])),
        'blocking': [d['code'] for d in diagnostics if d.get('severity') == 'blocking'],
        'codes': sorted(set(d['code'] for d in diagnostics)),
        'recovery': sorted(set(r['solve']['recovery_method'] for r in evidence)),
        'source_recovery': sorted(set(r['source_recovery']['status'] for r in evidence)),
        'source_block_recovery_present': 'source_block_recovery' in envelope,
        'rows': len(envelope['results'])}))
print('row inventories identical between modes (excluding solver-mode and parity rows) for', len(inventories), 'cases')
