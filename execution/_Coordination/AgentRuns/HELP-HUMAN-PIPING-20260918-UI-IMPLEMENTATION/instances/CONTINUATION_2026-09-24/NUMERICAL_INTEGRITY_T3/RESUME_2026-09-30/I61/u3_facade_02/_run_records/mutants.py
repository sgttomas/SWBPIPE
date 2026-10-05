#!/usr/bin/env python3
"""U3 grant 1b mutants: one source change each in a disposable tree (`mut` = the
candidate; `mutstub` = the candidate plus the archive stub), the guarding tests,
killed/survived, then the bytes restored."""
import subprocess, sys, os, hashlib
W = os.environ['I61_WT']
S = W + '/scratch/i61_u3_facade_02'
TREES = {'mut': S + '/mut/projects/chirality-piping/core/product_physics',
         'mutstub': S + '/mutstub/projects/chirality-piping/core/product_physics'}
FILTERS = {'mut': ['retained_facade_tests', 'u1_', 'u2_', 'u1g2', 'i61_certificate_prefixes'],
           'mutstub': ['retained_facade_tests::u3_archive_stub']}
NOTICE_RECEIPT = '''        if let W1Fallback::Serializer(failure) = &cause {
            if let Some(detail) = receipt_encoding_detail(failure.check) {'''
MUTANTS = [
 # Grant 1b: R-1.
 ('B01_successor_never', 'mut', 'lib.rs', '            Some(Ok(successor)) => Some(&successor.0),\n            _ => None,', '            Some(Ok(_successor)) => None,\n            _ => None,'),
 ('B02_publication_always_ordinary', 'mut', 'lib.rs', '            Some(Ok(successor)) => RetainedPublication::Successor(successor.0),', '            Some(Ok(_successor)) => RetainedPublication::Ordinary(self.envelope),'),
 # Grant 1b: R-2 (N1).
 ('B03_notice_not_appended', 'mut', 'lib.rs', '        ordinary.diagnostics.push(notice);\n        (ordinary, Err(cause))', '        drop(notice);\n        (ordinary, Err(cause))'),
 ('B04_receipt_detail_never', 'mut', 'lib.rs', NOTICE_RECEIPT, '        if let W1Fallback::Serializer(failure) = &cause {\n            if let Some(detail) = receipt_encoding_detail(failure.check).filter(|_| false) {'),
 ('B05_receipt_detail_every_check', 'mut', 'lib.rs', '        _ => None,\n    }\n}\n\n/// R-2 (N1): the notice whose space', '        token => Some(token),\n    }\n}\n\n/// R-2 (N1): the notice whose space'),
 ('B06_slot_not_reserved', 'mut', 'lib.rs', '        ordinary.diagnostics.try_reserve_exact(1).ok()?;\n', ''),
 ('B07_duplicate_id_not_refused', 'mut', 'lib.rs', '        if ordinary.diagnostics.iter().any(|d| d.id == id) {\n            return None;\n        }\n', ''),
 ('B08_combinations_not_checked', 'mut', 'lib.rs', "model[\"combinations\"].as_array().map_or(0, Vec::len)) {", "0usize) {"),
 ('B09_any_case_count', 'mut', 'lib.rs', '        (Some([case]), 0) => case["id"].as_str(),', '        (Some([case, ..]), 0) => case["id"].as_str(),'),
 ('B10_preparation_without_notice', 'mut', 'lib.rs', 'Err(failure) => return notice.publish(failure.ordinary, W1Fallback::Preparation),', 'Err(failure) => { drop(notice); return (failure.ordinary, Err(W1Fallback::Preparation)) }'),
 ('B11_native_without_notice', 'mut', 'lib.rs', '        return notice.publish(prepared.into_ordinary(), W1Fallback::Native);', '        drop(notice);\n        return (prepared.into_ordinary(), Err(W1Fallback::Native));'),
 ('B12_candidate_without_notice', 'mut', 'lib.rs', 'Err(refusal) => return notice.publish(refusal.ordinary, W1Fallback::Candidate),', 'Err(refusal) => { drop(notice); return (refusal.ordinary, Err(W1Fallback::Candidate)) }'),
 ('B13_serializer_without_notice', 'mut', 'lib.rs', 'Err(failure) => return notice.publish(frozen.into_ordinary(), W1Fallback::Serializer(failure)),', 'Err(failure) => { drop(notice); return (frozen.into_ordinary(), Err(W1Fallback::Serializer(failure))) }'),
 ('B14_precommit_without_notice', 'mut', 'lib.rs', '        return notice.publish(frozen.into_ordinary(), W1Fallback::Precommit { gate: error.gate, code: error.code });', '        drop(notice);\n        return (frozen.into_ordinary(), Err(W1Fallback::Precommit { gate: error.gate, code: error.code }));'),
 ('B15_notice_on_success', 'mut', 'lib.rs', '    drop(notice);\n    (frozen.into_ordinary(), Ok(RetainedSuccessor(successor)))', '    let (published, _) = notice.publish(frozen.into_ordinary(), W1Fallback::Candidate);\n    (published, Ok(RetainedSuccessor(successor)))'),
 # Grant 1b: the hooks follow the reserved-stack thread.
 ('B16_dispatch_does_not_carry', 'mut', 'lib.rs', 'on_reserved_stack(bytes, carry_test_hooks(move || {\n        pending.take().map(|request| permitted_run(permit, request, captured, solver_mode))\n    }));', 'on_reserved_stack(bytes, move || {\n        pending.take().map(|request| permitted_run(permit, request, captured, solver_mode))\n    });'),
 ('B17_carry_does_not_install', 'mut', 'lib.rs', '        retained_tests_hooks::install_armed(armed);\n        work()', '        let _ = armed;\n        work()'),
 ('B18_take_leaves_armed', 'mut', 'lib.rs', '        let mut armed = ARMED.with(|a| a.take());', '        let mut armed = ARMED.with(|a| a.get());'),
 ('B19_ceiling_taken', 'mut', 'lib.rs', '        armed.ceiling = super::DENSE_SCRUTINY_CEILING_OVERRIDE.with(|c| c.get());', '        armed.ceiling = super::DENSE_SCRUTINY_CEILING_OVERRIDE.with(|c| c.take());'),
 # Grant 1b: the linear permit.
 ('B20_permit_copy_again', 'mut', 'retained_memory.rs', '/// reserved-stack thread and into the observer, which uses it for G-B and G-C.\npub(super) struct CapturePermit {', '/// reserved-stack thread and into the observer, which uses it for G-B and G-C.\n#[derive(Clone, Copy)]\npub(super) struct CapturePermit {'),
 # Grant 1 mutants re-expressed on the grant-1b text.
 ('X01_coexistence_guard', 'mut', 'lib.rs', 'if ordinary.source_block_recovery.is_some() {\n        return (ordinary, Err(W1Fallback::Coexistence));', 'if false {\n        return (ordinary, Err(W1Fallback::Coexistence));'),
 ('X02_preparation_cause', 'mut', 'lib.rs', 'notice.publish(failure.ordinary, W1Fallback::Preparation)', 'notice.publish(failure.ordinary, W1Fallback::Candidate)'),
 ('X03_native_guard', 'mut', 'lib.rs', 'if prepared.solve_native().is_err() {', 'if { let _ = prepared.solve_native(); false } {'),
 ('X05_serialize_ordinary_not_staged', 'mut', 'lib.rs', 'retained_wire::serialize_frozen(&frozen, &staged, capture);', 'retained_wire::serialize_frozen(&frozen, frozen.ordinary(), capture);'),
 ('X06_serializer_fallback_staged', 'mut', 'lib.rs', 'notice.publish(frozen.into_ordinary(), W1Fallback::Serializer(failure))', 'notice.publish(frozen.staged_envelope(), W1Fallback::Serializer(failure))'),
 ('X07_precommit_skipped', 'mut', 'lib.rs', 'validate(&successor, Some(&invocation)) {', 'validate(&successor, Some(&invocation)).map(|_| ()).or_else(|e| if e.gate.is_empty() { Err(e) } else { Ok(()) }) {'),
 ('X08_precommit_unbound', 'mut', 'lib.rs', 'validate(&successor, Some(&invocation)) {', 'validate(&successor, None) {'),
 ('X10_transfer_publishes_staged', 'mut', 'lib.rs', '(frozen.into_ordinary(), Ok(RetainedSuccessor(successor)))', '(frozen.staged_envelope(), Ok(RetainedSuccessor(successor)))'),
 ('X12_panic_payload_lost', 'mut', 'lib.rs', 'Err(payload) => std::panic::resume_unwind(payload),', 'Err(_payload) => panic!("reserved-stack work panicked"),'),
 ('X14_admission_report_dropped', 'mut', 'lib.rs', 'Some(Err(report)) => Some(report),', 'Some(Err(_report)) => None,'),
 # Permit-only branches: behind the disposable stub.
 ('Y01_late_gate_ignored', 'mutstub', 'retained_product.rs', 'if let Err(refusal)=permit.check_late(&facts) {self.late_refusal=Some(refusal);return;}', 'let _=permit.check_late(&facts);'),
 ('Y02_late_refusal_not_checked', 'mutstub', 'lib.rs', 'if let Some(refusal) = observer.late_refusal() {', 'if let Some(refusal) = observer.late_refusal().filter(|_| false) {'),
 ('Y03_complete_gate_ignored', 'mutstub', 'lib.rs', 'permit.check_complete(&retained_memory::CompleteFacts { ordinary: &ordinary }));', 'permit.check_complete(&retained_memory::CompleteFacts { ordinary: &ordinary }).or(Ok::<(), retained_memory::PhaseRefusal>(())));'),
 ('Y04_stack_failure_not_ordinary', 'mutstub', 'lib.rs', '(None, Some(request)) => ordinary_dispatch(request, &capture, solver_mode, None, Some(Err(W1Fallback::StackReservation))),', '(None, Some(_request)) => Err("reserved stack unavailable".into()),'),
 ('Y05_permit_dispatch_bypassed', 'mutstub', 'lib.rs', 'Some(Ok(permit)) => return permitted_dispatch(permit, request, capture, solver_mode),', 'Some(Ok(_permit)) => None,'),
 ('Y06_domain_guard_removed', 'mutstub', 'lib.rs', 'if case_state::is_load_state(&request.model) || pressure_runtime::is_exact(&request.model) {\n        return ordinary_dispatch(request, capture, solver_mode, None, Some(Err(W1Fallback::Domain)));', 'if false {\n        return ordinary_dispatch(request, capture, solver_mode, None, Some(Err(W1Fallback::Domain)));'),
 ('Y07_finalization_check_removed', 'mutstub', 'lib.rs', '    if source_finalization_failed(&ordinary) {\n        return Err("SOURCE_BLOCKS_FINALIZATION_FAILED".into());\n    }\n    // G-C', '    // G-C'),
 ('Y08_permitted_observer_unbound', 'mutstub', 'retained_product.rs', 'Self {prepared_probe:true,permit:Some(permit),..Self::default()}', 'Self {prepared_probe:true,permit:{let _=permit;None},..Self::default()}'),
 ('Y09_permit_unbound_proceeds', 'mutstub', 'lib.rs', '        None => (ordinary, Err(W1Fallback::PermitUnbound)),', '        None => retained_w1(observer, ordinary, capture),'),
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
        cmd = ['perl', '-e', 'alarm shift; exec @ARGV', '1200', 'cargo', 'test', '--locked', '--offline', '--target-dir', W + '/targets/i61-u3/' + tree + '-1b', '--lib', '--'] + FILTERS[tree]
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
