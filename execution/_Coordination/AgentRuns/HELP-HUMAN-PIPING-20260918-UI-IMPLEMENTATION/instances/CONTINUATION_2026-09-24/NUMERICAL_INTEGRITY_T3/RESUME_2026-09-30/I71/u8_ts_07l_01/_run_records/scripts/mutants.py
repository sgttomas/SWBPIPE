"""I71 discrimination evidence (records only). Runs only in the scratch arena ARCH = WT/scratch/i71_u8/base (the
d449097085 archive copy), after its suite run, with the candidate's corpus (07l) and test file installed by sha256.
Reader mutants (RMx) patch ARCH's copy of retainedPrecision.ts; test mutants (TMx) patch ARCH's copy of the corpus.
Each run executes retainedPrecision.test.ts under vitest and records the failing test names; every file is
restored and sha256-checked afterwards. Usage: mutants.py ARCH_P S_LOGS TMPDIR."""
import hashlib, json, os, subprocess, sys
P, LOGS, TMP = sys.argv[1:4]
DESK = os.path.join(P, 'apps/desktop')
READER = os.path.join(DESK, 'src/features/results/retainedPrecision.ts')
CORPUS = os.path.join(P, 'fixtures/results/retained_precision_cases.json')
sha = lambda p: hashlib.sha256(open(p, 'rb').read()).hexdigest()
def text_patch(old, new):
    def apply(src):
        assert src.count(old) == 1, old
        return src.replace(old, new)
    return apply
READER_MUTANTS = {
    'RM1_a_exclusion_removed': text_patch("    if (A.some((v, k) => v && !nonInput[k])) continue;\n", ""),
    'RM2_feasibility_coupled_at_l0': text_patch("fail(stopFeasible(e.stop, facts.present[bi], facts.nonInput[bi], coupled, floors));", "fail(stopFeasible(e.stop, facts.present[bi], facts.nonInput[bi], true, floors));"),
    'RM3_hats_coupled_at_l0': text_patch("    if (coupled) hats = [hats[0] || hats[1], hats[0] || hats[1]];", "    if (true) hats = [hats[0] || hats[1], hats[0] || hats[1]];"),
    'RM4_no_free_dof_rule_removed': text_patch("  if (!facts.free[bi]) need(!e.has_data, 'G5a', 'SCALE_MISMATCH');\n", ""),
}
def corpus_patch(edit):
    def apply(src):
        c = json.loads(src); edit(c); return json.dumps(c, indent=2) + '\n'
    return apply
def drop_dense(c):
    c['cases'] = [x for x in c['cases'] if x['id'] != 'u8_l0_isolated_node_dense_scrutiny']
    c['mutations'] = [x for x in c['mutations'] if x['base'] != 'u8_l0_isolated_node_dense_scrutiny']
    c['must_pass'] = [x for x in c['must_pass'] if x['base'] != 'u8_l0_isolated_node_dense_scrutiny']
def masked(c):
    m = next(x for x in c['mutations'] if x['id'] == 'isolated_has_data_sparse_interactive')
    m['expected_by_reader'] = {'typescript': m['expected']}; m['expected'] = {'gate': 'G5', 'code': 'RETAINED_PRECISION_ATTEMPT_MISMATCH'}
def swapped(c):
    c['mutations'][278], c['mutations'][279] = c['mutations'][279], c['mutations'][278]
TEST_MUTANTS = {
    'TM1_drop_last_must_pass': corpus_patch(lambda c: c['must_pass'].pop()),
    'TM2_drop_dense_base_and_its_entries': corpus_patch(drop_dense),
    'TM3_reexpected_behind_a_typescript_override': corpus_patch(masked),
    'TM4_two_appended_mutations_swapped': corpus_patch(swapped),
}
def run(tag):
    out = os.path.join(LOGS, f'mutant_{tag}.json')
    env = dict(os.environ, TMPDIR=TMP)
    rc = subprocess.run(['../../node_modules/.bin/vitest', 'run', '--reporter=json', '--outputFile.json=' + out, 'src/features/results/retainedPrecision.test.ts'],
                        cwd=DESK, env=env, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL).returncode
    d = json.load(open(out))
    results = [a for r in d['testResults'] for a in r['assertionResults']]
    failed = sorted(a['fullName'] for a in results if a['status'] != 'passed')
    os.remove(out)
    return {'mutant': tag, 'rc': rc, 'tests': len(results), 'failed': failed}
before = {READER: sha(READER), CORPUS: sha(CORPUS)}
with open(os.path.join(LOGS, 'mutants.jsonl'), 'w') as log:
    log.write(json.dumps(run('none_control')) + '\n')
    for group, path in ((READER_MUTANTS, READER), (TEST_MUTANTS, CORPUS)):
        for tag, patch in group.items():
            original = open(path).read()
            try:
                open(path, 'w').write(patch(original)); log.write(json.dumps(run(tag)) + '\n'); log.flush()
            finally:
                open(path, 'w').write(original)
            assert sha(path) == before[path], tag
print('restored', all(sha(p) == h for p, h in before.items()))
