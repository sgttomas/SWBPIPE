"""Read-only compatibility probe of the WP6 package against WP5's in-progress adapter.

T1_WP6_STATIC_CASES TASK. Run from WORKING_ROOT. It imports
tools/validation/qualification_load_reference.py as it stands at run time and
applies its selector-file admission, row/evidence semantics and rule binding
to every assertion. Readiness and review status are not probed: the candidates
are intentionally pending. The WP5 file is concurrent work, so this result is
dated evidence only.
"""
import collections
import hashlib
import json
import sys

sys.path.insert(0, 'tools/validation')
sys.path.insert(0, '.')
import qualification_load_reference as lr  # noqa: E402

print('adapter sha256', hashlib.sha256(open('tools/validation/qualification_load_reference.py', 'rb').read()).hexdigest())
table = json.load(open('fixtures/results/semantic_contract_v0_3_load_reference_1.json'))['rows']
manifest = json.load(open('validation/qualification/fixtures/load_reference/MANIFEST.json'))
errors, checked = collections.Counter(), 0
for case in manifest['cases']:
    selectors = json.load(open(case['selectors']['path']))
    try:
        lr.admit_selectors(selectors, case)
    except Exception as exc:  # noqa: BLE001
        errors[(case['case_id'], 'admit_selectors', str(exc))] += 1
    rules = json.load(open(case['criteria']['path']))['tolerance_profile']['rules']
    for assertion in selectors['assertions'] + selectors.get('negative_assertions', []):
        selector = assertion['selector']
        checked += 1
        try:
            family = lr.evidence_semantics(selector) if 'namespace' in selector else lr.row_semantics(selector, table)
            lr._rule(rules, assertion['criterion_rule_id'], selector['dimension'], family, selector['unit'])
        except Exception as exc:  # noqa: BLE001
            errors[(case['case_id'], 'semantics', str(exc))] += 1
for key, count in sorted(errors.items()):
    print(count, key)
print(f'{checked} assertions probed; {sum(errors.values())} refusals')
