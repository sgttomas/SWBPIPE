#!/usr/bin/env python3
"""Seeded faults against independent_recompute.py (run from a scratch WORKING_ROOT copy).

Each mutant edits one package file in the scratch copy, runs the checker and
expects a nonzero exit; the file is restored afterwards. Proves the checker
detects the fault classes it claims to cover. Never run on the real tree.
"""
import json, math, subprocess, sys
from pathlib import Path

P = Path('validation/qualification/fixtures/load_reference')
CHECKER = sys.argv[1]


def edit(name, fn):
    f = P / name
    orig = f.read_bytes()
    d = json.loads(orig)
    fn(d)
    f.write_text(json.dumps(d, indent=2) + '\n')
    return f, orig


def val(case, aid, fn, key='values'):
    def m(d):
        for v in d[key]:
            if v['assertion_id'] == aid:
                v['value'] = fn(v['value'])
                return
        raise KeyError(aid)
    return f'{case}.reference.candidate.json', m


def rule(case, rid, field, fn):
    def m(d):
        for r in d['tolerance_profile']['rules']:
            if r['rule_id'] == rid:
                r[field] = fn(r[field])
                return
        raise KeyError(rid)
    return f'{case}.criteria.candidate.json', m


MUTANTS = {
    'value +1 ulp (two-bar middle UX)': val('prescribed_translation_two_bar', 'case:prescribed-root-translation.disp.node:middle.ux', lambda x: math.nextafter(x, math.inf)),
    'negate dropped (two-bar pipe:1 end_i)': val('prescribed_translation_two_bar', 'case:prescribed-root-translation.axial.pipe:1.end_i', lambda x: -x),
    'm->mm factor dropped (free-tip tip UY)': val('prescribed_rotation_free_tip', 'case:prescribed-root-rotation.disp.node:far.uy', lambda x: x / 1000),
    'one_plus dropped (thermal install stretch)': val('thermal_datum_ratio.fixed', 'case:annular-three-point.evidence.member.pipe:thermal.installation_datum_stretch', lambda x: x - 1),
    'area ratio dropped (fit hot additive negative)': val('signed_fit_states.fixed', 'case:hot.axial.pipe:fit.midspan.not.hot_additive_strains', lambda x: -90000.0, key='wrong_values'),
    'equilibrium zero made nonzero': val('persistent_source_once', 'case:combined.reaction.support:root.Fy', lambda x: 1e-12),
    'zero absolute x10 (fit cold/return strain rule)': rule('signed_fit_states.fixed', 'criterion:load_reference_strain:dimensionless:1:zero_scale:hot_thermal_strain', 'absolute_tolerance_value', lambda x: x * 10),
    'relative relaxed to 1e-8': rule('thermal_datum_ratio.free', 'criterion:displacement:length:mm:relative_1e-9', 'relative_tolerance_value', lambda x: 1e-8),
    'negative equal to reference (serial)': val('shared_material_serial_companion', 'case:shared-material.disp.node:middle.ux.not.material_ID_only_E_override', lambda x: -0.3333333333333333, key='wrong_values'),
}

baseline = subprocess.run([sys.executable, CHECKER], capture_output=True, text=True)
print('baseline exit', baseline.returncode, baseline.stdout.strip().splitlines()[-1])
killed = 0
for name, (fname, fn) in MUTANTS.items():
    f, orig = edit(fname, fn)
    try:
        r = subprocess.run([sys.executable, CHECKER], capture_output=True, text=True)
    finally:
        f.write_bytes(orig)
    ok = r.returncode != 0
    killed += ok
    first = next((l for l in r.stdout.splitlines() if l.startswith('FAIL')), r.stderr.strip().splitlines()[-1] if r.stderr.strip() else '')
    print(('KILLED ' if ok else 'SURVIVED ') + name + ' :: ' + first[:160])
print(f'{killed} of {len(MUTANTS)} killed')
sys.exit(0 if killed == len(MUTANTS) and baseline.returncode == 0 else 1)
