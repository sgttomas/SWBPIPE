#!/usr/bin/env python3
"""U3 grant 1d mutants: one source change each in a disposable tree (`mut` = the
candidate; `mutstub` = the candidate plus the archive stub), the guarding tests,
killed/survived, then the bytes restored."""
import subprocess, sys, os, hashlib
W = os.environ['I61_WT']
S = W + '/scratch/i61_u3_facade_04'
TREES = {'mut': S + '/mut/projects/chirality-piping/core/product_physics',
         'mutstub': S + '/mutstub/projects/chirality-piping/core/product_physics'}
FILTERS = {'mut': ['retained_facade_tests', 'u1_', 'u2_', 'u1g2', 'i61_certificate_prefixes'],
           'mutstub': ['retained_facade_tests::u3_archive_stub']}
NOTICE_RECEIPT = '''        if let W1Fallback::Serializer(failure) = &cause {
            if let Some(detail) = receipt_encoding_detail(failure.check) {'''
MUTANTS = [
 # RV85 T1: its own patches, verbatim.
 ('W01_message_reservation_short', 'mut', 'lib.rs', 'const RECEIPT_ENCODING_DETAIL_MAX: usize = 25;', 'const RECEIPT_ENCODING_DETAIL_MAX: usize = 0;'),
 ('W02_message_not_reserved', 'mut', 'lib.rs', '        message.try_reserve_exact(RETAINED_UNAVAILABLE_NOTICE.len() + RECEIPT_ENCODING_REASON.len() + RECEIPT_ENCODING_DETAIL_MAX + 1).ok()?;\n', ''),
 # RV85 U2: the hand-back.
 ('U2a_worker_does_not_hand_back', 'mut', 'lib.rs', '        retained_tests_hooks::hand_back(caller);\n', ''),
 ('U2b_unrun_work_drops_faults', 'mut', 'lib.rs', '            if let Some(armed) = self.armed.take() {\n                record(self.caller, armed);\n            }', '            let _ = self.armed.take();'),
 ('U2c_reclaim_drops_handed_back', 'mut', 'lib.rs', '        for faults in mine {\n            arm(|a| {', '        for faults in mine.into_iter().filter(|_| false) {\n            arm(|a| {'),
 ('U2d_dispatch_does_not_reclaim', 'mutstub', 'lib.rs', '    #[cfg(test)]\n    retained_tests_hooks::reclaim_handed_back();\n    match (ran.flatten(), slot) {', '    match (ran.flatten(), slot) {'),
]
def sha(p): return hashlib.sha256(open(p,'rb').read()).hexdigest()
only = sys.argv[1:]
results = []
for name, tree, f, old, new in MUTANTS:
    if only and name not in only: continue
    M = TREES[tree]; path = M + '/src/' + f; orig = open(path).read(); before = sha(path)
    assert orig.count(old) == 1, (name, orig.count(old))
    open(path, 'w').write(orig.replace(old, new))
    try:
        log = f'{S}/logs/mutant_{name}.log'
        cmd = ['perl', '-e', 'alarm shift; exec @ARGV', '1200', 'cargo', 'test', '--locked', '--offline', '--target-dir', W + '/targets/i61-u3/' + tree + '-1d', '--lib', '--'] + FILTERS[tree]
        env = dict(os.environ, CARGO_BUILD_JOBS='4', RUST_TEST_THREADS='2')
        if subprocess.run(['pgrep', '-f', 'memguard.sh'], capture_output=True).returncode != 0: sys.exit('MEMGUARD NOT RUNNING')
        with open(log, 'w') as out:
            rc = subprocess.run(cmd, cwd=M, env=env, stdout=out, stderr=subprocess.STDOUT).returncode
        text = open(log).read()
        failed = sorted({l.split()[1] for l in text.splitlines() if l.startswith('test ') and l.endswith('FAILED')})
        compile_error = 'error[' in text or 'error: could not compile' in text
        crashed = "process didn't exit successfully" in text and not failed
        verdict = 'KILLED' if rc != 0 else 'SURVIVED'
        line = f'{name} [{tree}] {verdict} rc={rc} compile_error={compile_error} crashed={crashed} failed={failed}'
    finally:
        open(path, 'w').write(orig)
        assert sha(path) == before
    print(line, flush=True); results.append(line)
open(f'{S}/logs/mutants_summary.txt', 'a').write('\n'.join(results) + '\n')
