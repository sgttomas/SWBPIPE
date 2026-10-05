#!/usr/bin/env python3
"""RV85: run the disposable stub test (control), then two permit-only mutants against it."""
import subprocess, os, hashlib, sys
W = 'WT'
S = W + '/scratch/rv85_u3_facade_02'
P = W + '/rv85/stub/projects/chirality-piping/core/product_physics'
RUNS = [
 ('STUB_CONTROL', None, None, None),
 ('SV07_late_refusal_check_removed', 'lib.rs', '    if let Some(refusal) = observer.late_refusal() {\n        let refusal = refusal.clone();\n        return (ordinary, Err(W1Fallback::LateGate(refusal)));\n    }\n', ''),
 ('SV13_complete_gate_ignored', 'lib.rs', '        Some(Err(refusal)) => (ordinary, Err(W1Fallback::CompleteGate(refusal))),', '        Some(Err(_refusal)) => retained_w1(observer, ordinary, capture),'),
 ('SV14_stack_failure_runs_nothing', 'lib.rs', '(None, Some(request)) => ordinary_dispatch(request, &capture, solver_mode, None, Some(Err(W1Fallback::StackReservation))),', '(None, Some(_request)) => Err("RV85: reserved stack unavailable".into()),'),
 ('SV15_dispatch_does_not_carry_hooks', 'lib.rs', 'on_reserved_stack(bytes, carry_test_hooks(move || {\n        pending.take().map(|request| permitted_run(permit, request, captured, solver_mode))\n    }));', 'on_reserved_stack(bytes, move || {\n        pending.take().map(|request| permitted_run(permit, request, captured, solver_mode))\n    });'),
 ('SV16_notice_on_complete_gate', 'lib.rs', '        Some(Err(refusal)) => (ordinary, Err(W1Fallback::CompleteGate(refusal))),', '        Some(Err(refusal)) => { let mut o = ordinary; let n = ReservedNotice::reserve(&mut o, "case").unwrap(); n.publish(o, W1Fallback::CompleteGate(refusal)) },'),
 ('SV17_permit_unbound_proceeds', 'lib.rs', '        None => (ordinary, Err(W1Fallback::PermitUnbound)),', '        None => retained_w1(observer, ordinary, capture),'),
]
def sha(p): return hashlib.sha256(open(p,'rb').read()).hexdigest()
for name, f, old, new in RUNS:
    path = P + '/src/' + (f or 'lib.rs'); orig = open(path).read(); before = sha(path)
    if old is not None:
        assert orig.count(old) == 1, name
        open(path, 'w').write(orig.replace(old, new))
    try:
        if subprocess.run(['pgrep', '-f', 'memguard.sh'], capture_output=True).returncode != 0: sys.exit('MEMGUARD NOT RUNNING')
        log = f'{S}/stub/{name}.log'
        env = dict(os.environ, CARGO_BUILD_JOBS='4', RUST_TEST_THREADS='2', RV85_SWEEP_LIST=S + '/sweep/list.txt',
                   RV85_STUB_OUT=f'{S}/stub/{name}.out', RV85_STUB_DIR=S + '/stub')
        cmd = ['perl', '-e', 'alarm shift; exec @ARGV', '1500', 'cargo', 'test', '--locked', '--offline', '--target-dir', W + '/targets/rv85/g1b/stub', '--lib', '--', 'rv85_stub_tests', '--nocapture']
        with open(log, 'w') as out:
            rc = subprocess.run(cmd, cwd=P, env=env, stdout=out, stderr=subprocess.STDOUT).returncode
        text = open(log).read()
        res = [l for l in text.splitlines() if l.startswith('test result:')]
        line = f'{name} rc={rc} {"KILLED" if (old and rc) else ("SURVIVED" if old else ("CONTROL_OK" if rc == 0 else "CONTROL_BAD"))} {res}'
    finally:
        open(path, 'w').write(orig); assert sha(path) == before
    print(line, flush=True)
    open(f'{S}/stub/summary.txt', 'a').write(line + '\n')
