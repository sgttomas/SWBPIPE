#!/usr/bin/env python3
"""RV85 (grant 1c): run the disposable stub test (control), then permit-only mutants against it.
Each run writes into its own output folder."""
import subprocess, os, hashlib, sys
W = 'WT'
S = W + '/scratch/rv85_u3_facade_02/c1c/stub'
P = W + '/rv85/stub1c/projects/chirality-piping/core/product_physics'
RUNS = [
 ('STUB_CONTROL', None, None),
 ('SV07_w1_late_check_removed', '    if let Some(refusal) = observer.late_refusal() {\n        let refusal = refusal.clone();\n        return (ordinary, Err(W1Fallback::LateGate(refusal)));\n    }\n', ''),
 ('SV18_run_late_check_removed', '    } else if let Some(refusal) = observer.late_refusal().cloned() {\n        (ordinary, Err(W1Fallback::LateGate(refusal)))\n    } else {', '    } else if false {\n        (ordinary, Err(W1Fallback::Coexistence))\n    } else {'),
 ('SV19_run_exact_check_removed', '    let (envelope, retained) = if ordinary.source_block_recovery.is_some() {', '    let (envelope, retained) = if false {'),
 ('SV20_stack_report_dropped', 'ordinary_dispatch(request, &capture, solver_mode, Some(report), Some(Err(W1Fallback::StackReservation))),', 'ordinary_dispatch(request, &capture, solver_mode, None, Some(Err(W1Fallback::StackReservation))),'),
 ('SV21_w1_report_dropped', 'Ok(RetainedPreviewOutput { envelope, admission: Some(report), retained: Some(retained) })', 'Ok(RetainedPreviewOutput { envelope, admission: None, retained: Some(retained) })'),
 ('SV13_complete_gate_ignored', '            Some(Err(refusal)) => (ordinary, Err(W1Fallback::CompleteGate(refusal))),', '            Some(Err(_refusal)) => retained_w1(observer, ordinary, capture),'),
 ('SV15_dispatch_does_not_carry_hooks', 'on_reserved_stack(bytes, carry_test_hooks(move || {', 'on_reserved_stack(bytes, (move || {'),
 ('SV16_notice_on_complete_gate', '            Some(Err(refusal)) => (ordinary, Err(W1Fallback::CompleteGate(refusal))),', '            Some(Err(refusal)) => { let mut o = ordinary; let n = ReservedNotice::reserve(&mut o, "case").unwrap(); n.publish(o, W1Fallback::CompleteGate(refusal)) },'),
]
def sha(p): return hashlib.sha256(open(p,'rb').read()).hexdigest()
for name, old, new in RUNS:
    path = P + '/src/lib.rs'; orig = open(path).read(); before = sha(path)
    if old is not None:
        assert orig.count(old) == 1, name
        open(path, 'w').write(orig.replace(old, new))
    try:
        if subprocess.run(['pgrep', '-f', 'memguard.sh'], capture_output=True).returncode != 0: sys.exit('MEMGUARD NOT RUNNING')
        out = f'{S}/out_{name}'; os.makedirs(out, exist_ok=True)
        env = dict(os.environ, CARGO_BUILD_JOBS='4', RUST_TEST_THREADS='2', RV85_SWEEP_LIST=W + '/scratch/rv85_u3_facade_02/sweep/list.txt',
                   RV85_STUB_OUT=f'{out}/stub.out', RV85_STUB_DIR=out)
        cmd = ['perl', '-e', 'alarm shift; exec @ARGV', '1500', 'cargo', 'test', '--locked', '--offline', '--target-dir', W + '/targets/rv85/g1c/stub', '--lib', '--', 'rv85_stub_tests', '--nocapture']
        with open(f'{out}/cargo.log', 'w') as o:
            rc = subprocess.run(cmd, cwd=P, env=env, stdout=o, stderr=subprocess.STDOUT).returncode
        text = open(f'{out}/cargo.log').read()
        res = [l for l in text.splitlines() if l.startswith('test result:')]
        line = f'{name} rc={rc} {"KILLED" if (old and rc) else ("SURVIVED" if old else ("CONTROL_OK" if rc == 0 else "CONTROL_BAD"))} {res}'
    finally:
        open(path, 'w').write(orig); assert sha(path) == before
    print(line, flush=True)
    open(f'{S}/summary.txt', 'a').write(line + '\n')
