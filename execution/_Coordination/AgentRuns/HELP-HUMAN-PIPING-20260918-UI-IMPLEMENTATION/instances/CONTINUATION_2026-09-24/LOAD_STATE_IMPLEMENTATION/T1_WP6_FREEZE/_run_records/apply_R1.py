#!/usr/bin/env python3
"""Apply freeze change R1 to a package copy (run from that copy's WORKING_ROOT).

R1: the cold/return `thermal_strain` zero expectations of both signed-fit cases
use the protected test's strain scale |hot total_eigenstrain| (0.00059984)
instead of |hot thermal_strain| (0.0008), so the absolute floor becomes
5.9984e-13 rather than the looser 8e-13.

After this script: run `generate_reference_values.py --write`, then update the
four changed sha256 values in MANIFEST.json and PROVENANCE.json
(see required_change_R1.diff), then `--check` and check_package.py.
"""
import json
from pathlib import Path

P = Path('validation/qualification/fixtures/load_reference')
NEWTAG = 'hot_total_eigenstrain'
NEWPTR = '/cases/signed_fit_states/variants/annular_companion/expected/hot/total_eigenstrain'
TARGETS = {
    'signed_fit_states.fixed': ('case:cold.evidence.member.pipe:fit.thermal_strain',
                                'case:return.evidence.member.pipe:fit.thermal_strain'),
    'signed_fit_states.released': ('case:cold.evidence.member.pipe:fit.thermal_strain',
                                   'case:return.evidence.member.pipe:fit.thermal_strain'),
}
for case, ids in TARGETS.items():
    f = P / f'{case}.selectors.candidate.json'
    s = json.loads(f.read_text())
    hit = 0
    for a in s['assertions']:
        if a['id'] not in ids:
            continue
        assert a['criterion_rule_id'] == 'criterion:load_reference_strain:dimensionless:1:zero_scale:hot_thermal_strain'
        a['criterion_rule_id'] = 'criterion:load_reference_strain:dimensionless:1:zero_scale:' + NEWTAG
        zs = a['selector_origin']['reference_origin']['zero_scale']
        zs['tag'] = NEWTAG
        zs['base']['pointer'] = NEWPTR
        hit += 1
    assert hit == 2, (case, hit)
    f.write_text(json.dumps(s, indent=2, ensure_ascii=False) + '\n')
    print('updated', f)
