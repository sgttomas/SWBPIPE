"""RV4 gate re-run of the 14 formation rows (standard library).
For every case named in rf_cancel_cases.json `formation_exceptions`, on every entry that accepts it
(`captured_refused_at_capture` excludes the captured entry) and both modes, the base and candidate
fixdiff probes are compared: the case's integrity code (base / candidate), which S11-G guard fired,
whether any `/results` leaf differs, and every differing leaf.
Usage: rv4_gate.py <rf_cancel_cases.json> <fixdiff_base> <fixdiff_cand> <workdir>"""
import json, os, subprocess, sys
data = json.load(open(sys.argv[1])); base_bin, cand_bin, work = sys.argv[2:5]
cases = {c['id']: c for c in data['cases']}
rows = {}
for entry, case, key, mode in data['formation_exceptions']:
    rows.setdefault(case, set()).add(key)
def probe(binary, path, mode, entry):
    out = subprocess.run([binary, 'probe', path, mode, entry], capture_output=True, text=True, check=True).stdout
    return json.loads(out)
def leaves(a, b, path=''):
    if isinstance(a, dict) and isinstance(b, dict):
        for k in sorted(set(a) | set(b)):
            if k not in a or k not in b: yield path + '/' + k
            else: yield from leaves(a[k], b[k], path + '/' + k)
    elif isinstance(a, list) and isinstance(b, list):
        if len(a) != len(b): yield path + '[len]'
        for i, (x, y) in enumerate(zip(a, b)): yield from leaves(x, y, '%s/%d' % (path, i))
    elif a != b: yield path
def integrity(env):
    return next(d for d in env['diagnostics'] if d['id'] == 'diagnostic:numerical-integrity:case')
bad = 0
print('| case | rows | entry | mode | base | candidate | guard | /results differ | differing leaves |')
print('|---|---|---|---|---|---|---|---|---|')
for case in sorted(rows):
    c = cases[case]; path = os.path.join(work, case + '.request.json'); json.dump(c['request'], open(path, 'w'))
    entries = ['typed'] if c.get('captured_refused_at_capture') else ['captured', 'typed']
    for entry in entries:
        for mode in ['dense_scrutiny', 'sparse_interactive']:
            b = probe(base_bin, path, mode, entry); n = probe(cand_bin, path, mode, entry)
            ib, ic = integrity(b), integrity(n)
            guard = 'load-row' if 'S11-G formation-noise guard' in ic['message'] else ("R-b'" if "S11-G recovery guard (R-b')" in ic['message'] else 'none')
            diff = sorted(set(leaves(b, n)))
            results_differ = any(d.startswith('/results') for d in diff)
            ok = ib['code'] == 'NUMERICAL_INTEGRITY_CHECKS_PASSED' and ic['code'] == 'NUMERICAL_INTEGRITY_SENSITIVE' and not results_differ and n['numerical_quality']['status'] != 'checks_passed'
            bad += not ok
            print(f"| {case} | {', '.join(sorted(rows[case]))} | {entry} | {mode} | {ib['code']} | {ic['code']} | {guard} | {results_differ} | {', '.join(diff)} |")
print(f"\nrows checked: {sum(len(v) for v in rows.values())} row keys over {len(rows)} cases; failures: {bad}")
