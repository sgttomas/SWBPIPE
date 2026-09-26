"""REVIEW_B backcheck of F4 (cad59d01e): run from WORKING_ROOT of a clone at or after cad59d01e."""
import json, subprocess, hashlib, importlib.util
from pathlib import Path
PKG = 'validation/qualification/fixtures/load_reference/'
A, B = 'c1e130818', 'cad59d01e'
def show(rev, path): return subprocess.check_output(['git', 'show', f'{rev}:./{path}'])
def sha(b): return hashlib.sha256(b).hexdigest()
changed = subprocess.check_output(['git', 'diff', '--name-only', A, B, '--', PKG], text=True).split()
names = sorted(p.split('/')[-1] for p in changed)
print('changed package files:', len(names), names[:3], '...')
non_criteria = [n for n in names if not n.endswith('.criteria.candidate.json')]
print('non-criteria changed:', non_criteria)
bad = 0
admitted_review = None
for n in names:
    if not n.endswith('.criteria.candidate.json'): continue
    old, new = json.loads(show(A, PKG + n)), json.loads(show(B, PKG + n))
    ro, rn = old['tolerance_profile'].pop('rules'), new['tolerance_profile'].pop('rules')
    ok = old == new and len(ro) == len(rn)
    for x, y in zip(ro, rn):
        rv = y.pop('review'); x.pop('review')
        admitted_review = admitted_review or rv
        ok &= (x == y) and rv == admitted_review
    bad += not ok
print('criteria files differing beyond rule review text:', bad, '; review =', admitted_review)
# references and selectors untouched
for kind in ('reference', 'selectors', 'runner_input', 'preview_request'):
    diff = subprocess.run(['git', 'diff', '--quiet', A, B, '--', PKG + f'*.{kind}*.json'])
    print(f'{kind} files unchanged:', diff.returncode == 0)
# manifest bindings and PROVENANCE currency at B
m = json.loads(show(B, PKG + 'MANIFEST.json')); n = 0
for case in m['cases']:
    for role in ('runner_input', 'product_request', 'reference', 'selectors', 'criteria'):
        assert sha(show(B, case[role]['path'])) == case[role]['sha256']; n += 1
    assert sha(show(B, case['analytical_reference']['path'])) == case['analytical_reference']['sha256']; n += 1
print('manifest bindings verified:', n)
p = json.loads(show(B, PKG + 'PROVENANCE.json'))
stale = [o['path'] for o in p['generator_outputs'] if sha(show(B, o['path'])) != o['sha256']]
print('PROVENANCE status:', p['status'], '; generator current:', sha(show(B, p['generator']['path'])) == p['generator']['sha256'],
      '; outputs stale:', len(stale), 'of', len(p['generator_outputs']))
for k in ('record', 'independent_freeze', 'freeze_change_R1_script'):
    print('PROVENANCE admission', k, 'current:', sha(show(B, p['admission'][k]['path'])) == p['admission'][k]['sha256'])
for o in p['authoring_records']:
    print('authoring record kept:', o['path'].split('/')[-1], sha(show(B, o['path'])) == o['sha256'])
# generator reproduces B with ADMISSION.json, and the pre-admission 6824b6b6b state without it (values/rules invariance)
spec = importlib.util.spec_from_file_location('gen', PKG + 'generate_reference_values.py')
gen = importlib.util.module_from_spec(spec); spec.loader.exec_module(gen)
out = gen.generate()
print('generator with ADMISSION.json reproduces', B, ':', sum(show(B, PKG + q.name) == d for q, d in out.items()), 'of', len(out))
adm = Path(PKG + 'ADMISSION.json'); aside = Path(PKG).parent / 'ADMISSION.reviewB'
adm.rename(aside)
try:
    out = gen.generate()
    print('generator without ADMISSION.json reproduces 6824b6b6b:', sum(show('6824b6b6b', PKG + q.name) == d for q, d in out.items()), 'of', len(out))
finally:
    aside.rename(adm)
