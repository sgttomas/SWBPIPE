#!/usr/bin/env python3
"""RV85 mutants on a disposable git-archive copy of bee3dc07ca (WT/rv85/mut).
Each mutant applies one exact replacement, runs its scope, records killed/survived
(excluding the known Mac failure t13 via --skip), and restores the bytes.
I-set: I61's committed-tree mutants, verbatim, with I61's own filter.
R08b: RV82's mutant, verbatim, on the whole PP --lib.
V-set: RV85's own mutants on the dispatch, the fallback and the frozen split."""
import subprocess, sys, os, hashlib
W = 'WT'
S = W + '/scratch/rv85_u3_facade_01'
M = W + '/rv85/mut/projects/chirality-piping/core/product_physics'
T = W + '/targets/rv85/mut'
SKIP = ['--skip', 't13_committed_fallback_uz_is_byte_identical']
I61F = ['retained_facade_tests', 'u1_', 'u2_', 'u1g2', 'i61_certificate_prefixes']
SCOPES = {'i61': ['--lib', '--'] + I61F, 'lib': ['--lib', '--'] + SKIP, 'all': ['--'] + SKIP}
sys.path.insert(0, S + '/mutants')
from mutant_defs import MUTANTS
def sha(p): return hashlib.sha256(open(p,'rb').read()).hexdigest()
only = sys.argv[1:]
for name, scope, f, old, new in MUTANTS:
    if only and name not in only: continue
    path = M + '/src/' + f; orig = open(path).read(); before = sha(path)
    if old is not None:
        assert orig.count(old) == 1, (name, orig.count(old))
        open(path, 'w').write(orig.replace(old, new))
    try:
        if subprocess.run(['pgrep', '-f', 'memguard.sh'], capture_output=True).returncode != 0: sys.exit('MEMGUARD NOT RUNNING')
        log = f'{S}/mutants/logs/{name}.log'
        cmd = ['perl', '-e', 'alarm shift; exec @ARGV', '1500', 'cargo', 'test', '--locked', '--offline', '--no-fail-fast', '--target-dir', T] + SCOPES[scope]
        env = dict(os.environ, CARGO_BUILD_JOBS='4', RUST_TEST_THREADS='2')
        with open(log, 'w') as out:
            rc = subprocess.run(cmd, cwd=M, env=env, stdout=out, stderr=subprocess.STDOUT).returncode
        text = open(log).read()
        failed = sorted({l.split()[1] for l in text.splitlines() if l.startswith('test ') and l.endswith('FAILED')})
        compile_error = 'error[' in text or 'error: could not compile' in text
        crashed = ("process didn't exit successfully" in text) and not failed
        passed = sum(int(l.split()[3]) for l in text.splitlines() if l.startswith('test result:'))
        verdict = ('CONTROL_OK' if rc == 0 else 'CONTROL_BAD') if old is None else ('KILLED' if rc != 0 else 'SURVIVED')
        line = f'{name} [{scope}] {verdict} rc={rc} passed={passed} compile_error={compile_error} crashed={crashed} failed={failed}'
    finally:
        open(path, 'w').write(orig)
        assert sha(path) == before
    print(line, flush=True)
    open(f'{S}/mutants/summary.txt', 'a').write(line + '\n')
