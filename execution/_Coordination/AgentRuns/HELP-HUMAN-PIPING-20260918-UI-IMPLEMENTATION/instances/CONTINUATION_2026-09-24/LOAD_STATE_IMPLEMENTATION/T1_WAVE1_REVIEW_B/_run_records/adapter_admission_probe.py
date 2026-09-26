"""REVIEW_B: the admitted WP6 package passes the WP5 adapter's per-case admission (no process, no product)."""
import json, sys
from pathlib import Path
sys.path.insert(0, 'tools/validation')
import qualification_load_reference as lr
import qualification_gate as gate
project = Path('.').resolve()
manifest_bytes = (project / 'validation/qualification/fixtures/load_reference/MANIFEST.json').read_bytes()
manifest = lr.admit_manifest(manifest_bytes)
table = gate.strict_json((project / lr.TABLE).read_bytes())
pos = neg = 0
for purpose in ('development_comparison', 'harness_development'):
    for case in manifest['cases']:
        inv = lr.case_inventory(case, project)
        assert inv['refusal'] is None, (case['case_id'], inv['refusal'])
        prepared = lr.prepare_case(case, project, purpose, table['rows'])
        if purpose == 'development_comparison':
            pos += sum(i['polarity'] == 'positive' for i in prepared['selected']); neg += sum(i['polarity'] == 'negative' for i in prepared['selected'])
    for mode in lr.MODES:
        led = lr.predeclare(manifest, mode, {c['case_id']: lr.case_inventory(c, project) for c in manifest['cases']})
        print(purpose, mode, 'cases required', len(led['cases']), 'excluded', led['cases_not_required_in_mode'])
print('admitted cases', len(manifest['cases']), 'positive', pos, 'negative', neg)
