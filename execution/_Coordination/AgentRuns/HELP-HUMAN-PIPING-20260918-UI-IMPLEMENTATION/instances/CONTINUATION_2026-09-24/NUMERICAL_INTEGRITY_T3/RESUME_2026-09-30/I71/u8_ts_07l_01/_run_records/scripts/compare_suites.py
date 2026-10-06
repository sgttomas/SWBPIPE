"""I71: compare the desktop vitest suite test by test, base (d449097085 archive copy) against candidate
(WT/f2a-u8), from vitest's JSON reports. A test's key is (file relative to apps/desktop, fullName, occurrence), since a few files repeat a name.
Usage: compare_suites.py BASE_JSON BASE_DESKTOP CAND_JSON CAND_DESKTOP OUT_TSV"""
import json, os, sys, collections
def load(path, root):
    d = json.load(open(path)); out = {}; files = {}
    for r in d['testResults']:
        rel = os.path.relpath(r['name'], root); files[rel] = r['status']
        seen = collections.Counter()
        for a in r['assertionResults']:
            seen[a['fullName']] += 1
            out[(rel, a['fullName'], str(seen[a['fullName']]))] = a['status']
    totals = {k: d[k] for k in ('numTotalTests', 'numPassedTests', 'numFailedTests', 'numPendingTests', 'numTodoTests', 'numTotalTestSuites', 'numPassedTestSuites', 'numFailedTestSuites', 'success')}
    return out, files, totals
b, bf, bt = load(sys.argv[1], sys.argv[2]); c, cf, ct = load(sys.argv[3], sys.argv[4])
rows = []
for key in sorted(set(b) | set(c)):
    sb, sc = b.get(key, 'absent'), c.get(key, 'absent')
    if sb != sc: rows.append((key[0], key[1], key[2], sb, sc))
with open(sys.argv[5], 'w') as f:
    f.write('file\ttest\toccurrence\tbase\tcandidate\n')
    for r in rows: f.write('\t'.join(r) + '\n')
print('base totals', bt); print('candidate totals', ct)
print('files: base', len(bf), 'candidate', len(cf), 'same set', set(bf) == set(cf))
print('file status changes', [(k, bf.get(k), cf.get(k)) for k in sorted(set(bf) | set(cf)) if bf.get(k) != cf.get(k)])
print('tests: base', len(b), 'candidate', len(c), 'common', len(set(b) & set(c)))
print('status counts base', collections.Counter(b.values()), 'candidate', collections.Counter(c.values()))
print('differences', len(rows), collections.Counter((r[3], r[4]) for r in rows))
print('differing files', collections.Counter(r[0] for r in rows))
