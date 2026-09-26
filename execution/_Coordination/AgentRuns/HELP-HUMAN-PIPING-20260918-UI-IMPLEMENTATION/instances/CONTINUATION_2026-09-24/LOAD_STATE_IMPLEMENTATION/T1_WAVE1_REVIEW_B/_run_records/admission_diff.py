"""REVIEW_B: admitted vs frozen candidate files, 6824b6b6b..c1e130818 (run from WORKING_ROOT of a clone)."""
import json, subprocess, hashlib
PKG = 'validation/qualification/fixtures/load_reference/'
A, B = '6824b6b6b', 'c1e130818'
def show(rev, path):
    return subprocess.check_output(['git', 'show', f'{rev}:./{path}'])
changed = subprocess.check_output(['git', 'diff', '--name-only', A, B, '--', PKG], text=True).split()
print('changed paths:', len(changed))
bad = 0
expected_ref = {'readiness': ('pending_independent_review', 'ready')}
for full in changed:
    name = full.split('/')[-1]
    if name in ('MANIFEST.json', 'ADMISSION.json'):
        continue
    path = PKG + name
    old, new = json.loads(show(A, path)), json.loads(show(B, path))
    diffs = []
    def walk(o, n, ptr):
        if isinstance(o, dict) and isinstance(n, dict):
            for k in sorted(set(o) | set(n)):
                if k not in o or k not in n:
                    diffs.append((ptr + '/' + k, o.get(k, '<absent>'), n.get(k, '<absent>')))
                else:
                    walk(o[k], n[k], ptr + '/' + k)
        elif isinstance(o, list) and isinstance(n, list) and len(o) == len(n):
            for i, (x, y) in enumerate(zip(o, n)):
                walk(x, y, f'{ptr}/{i}')
        elif o != n or type(o) is not type(n):
            diffs.append((ptr, o, n))
    walk(old, new, '')
    ptrs = sorted(d[0] for d in diffs)
    if name.endswith('.reference.candidate.json'):
        ok = ptrs == ['/independent_review_ref', '/readiness'] and all(
            (d[0] == '/readiness' and d[1] == 'pending_independent_review' and d[2] == 'ready') or
            (d[0] == '/independent_review_ref' and d[1] is None and d[2] == 'execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/LOAD_STATE_IMPLEMENTATION/T1_WP6_FREEZE/RETURN.md#sha256=795b04214006160755fe39a427005022beecf94830c81e37110a8032d2574c49')
            for d in diffs)
    elif name.endswith('.criteria.candidate.json'):
        ok = ptrs == ['/tolerance_profile/profile_status', '/tolerance_profile/scope'] and all(
            (d[0].endswith('profile_status') and d[1] == 'draft_pending_independent_review' and d[2] == 'reviewed') or d[0].endswith('scope')
            for d in diffs)
    else:
        ok = False
    bad += not ok
    print(('OK  ' if ok else 'BAD ') + name, ptrs)
# Manifest re-hash: every bound sha256 in the admitted MANIFEST equals the admitted bytes.
manifest = json.loads(show(B, PKG + 'MANIFEST.json'))
checked = 0
for case in manifest['cases']:
    for role in ('runner_input', 'product_request', 'reference', 'selectors', 'criteria'):
        b = case[role]
        assert hashlib.sha256(show(B, b['path'])).hexdigest() == b['sha256'], (case['case_id'], role)
        checked += 1
    a = case['analytical_reference']
    assert hashlib.sha256(show(B, a['path'])).hexdigest() == a['sha256']
    checked += 1
old_manifest = json.loads(show(A, PKG + 'MANIFEST.json'))
def mask(m):
    m = json.loads(json.dumps(m))
    for c in m['cases']:
        for role in ('reference', 'criteria'):
            c[role]['sha256'] = None
    return m
print('manifest bindings verified:', checked, '; manifest otherwise identical:', mask(manifest) == mask(old_manifest))
print('BAD files:', bad)
