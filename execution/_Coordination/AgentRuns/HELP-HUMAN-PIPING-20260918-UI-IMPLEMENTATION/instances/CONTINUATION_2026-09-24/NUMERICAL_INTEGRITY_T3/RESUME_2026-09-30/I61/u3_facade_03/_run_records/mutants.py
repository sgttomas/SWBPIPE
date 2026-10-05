#!/usr/bin/env python3
"""U3 grant 1c mutants: one source change each in a disposable tree (`mut` = the
candidate; `mutstub` = the candidate plus the archive stub), the guarding tests,
killed/survived, then the bytes restored."""
import subprocess, sys, os, hashlib
W = os.environ['I61_WT']
S = W + '/scratch/i61_u3_facade_03'
TREES = {'mut': S + '/mut/projects/chirality-piping/core/product_physics',
         'mutstub': S + '/mutstub/projects/chirality-piping/core/product_physics'}
FILTERS = {'mut': ['retained_facade_tests', 'u1_', 'u2_', 'u1g2', 'i61_certificate_prefixes'],
           'mutstub': ['retained_facade_tests::u3_archive_stub']}
NOTICE_RECEIPT = '''        if let W1Fallback::Serializer(failure) = &cause {
            if let Some(detail) = receipt_encoding_detail(failure.check) {'''
RUN_W1 = '        Some(Ok(())) => retained_w1(observer, ordinary, capture),'
MUTANTS = [
 # RV85 S3: every permitted output keeps the G-A report.
 ('C01_stack_report_dropped', 'mut', 'lib.rs', 'ordinary_dispatch(request, &capture, solver_mode, Some(report), Some(Err(W1Fallback::StackReservation))),', 'ordinary_dispatch(request, &capture, solver_mode, None, Some(Err(W1Fallback::StackReservation))),'),
 ('C02_domain_report_dropped', 'mut', 'lib.rs', 'ordinary_dispatch(request, capture, solver_mode, Some(report), Some(Err(W1Fallback::Domain)));', 'ordinary_dispatch(request, capture, solver_mode, None, Some(Err(W1Fallback::Domain)));'),
 ('C03_w1_report_dropped', 'mut', 'lib.rs', 'Ok(RetainedPreviewOutput { envelope, admission: Some(report), retained: Some(retained) })', 'Ok(RetainedPreviewOutput { envelope, admission: None, retained: Some(retained) })'),
 ('C01s_stack_report_dropped', 'mutstub', 'lib.rs', 'ordinary_dispatch(request, &capture, solver_mode, Some(report), Some(Err(W1Fallback::StackReservation))),', 'ordinary_dispatch(request, &capture, solver_mode, None, Some(Err(W1Fallback::StackReservation))),'),
 ('C02s_domain_report_dropped', 'mutstub', 'lib.rs', 'ordinary_dispatch(request, capture, solver_mode, Some(report), Some(Err(W1Fallback::Domain)));', 'ordinary_dispatch(request, capture, solver_mode, None, Some(Err(W1Fallback::Domain)));'),
 ('C03s_w1_report_dropped', 'mutstub', 'lib.rs', 'Ok(RetainedPreviewOutput { envelope, admission: Some(report), retained: Some(retained) })', 'Ok(RetainedPreviewOutput { envelope, admission: None, retained: Some(retained) })'),
 # RV85 N1: G-C only after exact selection and G-B's outcome.
 ('C04_exact_check_removed', 'mut', 'lib.rs', '    let (envelope, retained) = if ordinary.source_block_recovery.is_some() {', '    let (envelope, retained) = if false {'),
 ('C04s_exact_check_removed', 'mutstub', 'lib.rs', '    let (envelope, retained) = if ordinary.source_block_recovery.is_some() {', '    let (envelope, retained) = if false {'),
 ('C05s_late_check_removed', 'mutstub', 'lib.rs', '    } else if let Some(refusal) = observer.late_refusal().cloned() {', '    } else if let Some(refusal) = observer.late_refusal().cloned().filter(|_| false) {'),
 # RV85 N6: the typed staging fallback.
 ('C06_staging_fault_ignored', 'mut', 'lib.rs', '        Err(fault) => return notice.publish(frozen.into_ordinary(), W1Fallback::Staging(fault)),', '        Err(_fault) => frozen.ordinary().clone(),'),
 ('C07_bad_patch_skipped', 'mut', 'retained_product.rs', '        let object=extrema.get_mut(patch.evidence_index).and_then(|x|x.as_object_mut()).ok_or(StagingFault("pipe_stress_extrema[]"))?;', '        let Some(object)=extrema.get_mut(patch.evidence_index).and_then(|x|x.as_object_mut()) else { continue };'),
 ('C08_staging_cause', 'mut', 'lib.rs', '        Err(fault) => return notice.publish(frozen.into_ordinary(), W1Fallback::Staging(fault)),', '        Err(_fault) => return notice.publish(frozen.into_ordinary(), W1Fallback::Candidate),'),
 ('C09_staging_without_notice', 'mut', 'lib.rs', '        Err(fault) => return notice.publish(frozen.into_ordinary(), W1Fallback::Staging(fault)),', '        Err(fault) => { drop(notice); return (frozen.into_ordinary(), Err(W1Fallback::Staging(fault))) }'),
 # RV85 N7: the strengthened custody guard.
 ('C10_raw_reread', 'mut', 'lib.rs', '    let mut budget = SourceRecoveryBudget::default();\n    // The permit moves into the observer', '    let _raw = capture.borrowed_raw().clone();\n    let mut budget = SourceRecoveryBudget::default();\n    // The permit moves into the observer'),
 ('C11_from_str_rederive', 'mut', 'lib.rs', '    let mut budget = SourceRecoveryBudget::default();\n    // The permit moves into the observer', '    let _again: Option<serde_json::Value> = serde_json::from_str("null").ok();\n    let mut budget = SourceRecoveryBudget::default();\n    // The permit moves into the observer'),
 ('C12_second_dispatch_call', 'mut', 'lib.rs', '    ordinary_dispatch(request, &capture, solver_mode, admission, None)\n}', '    #[allow(unreachable_code)]\n    if false { let _ = permitted_dispatch(todo!(), todo!(), todo!(), todo!(), todo!()); }\n    ordinary_dispatch(request, &capture, solver_mode, admission, None)\n}'),
 # The permit-only branches re-run on the grant-1c text, behind the stub.
 ('Y01_late_gate_ignored', 'mutstub', 'retained_product.rs', 'if let Err(refusal)=permit.check_late(&facts) {self.late_refusal=Some(refusal);return;}', 'let _=permit.check_late(&facts);'),
 ('Y03_complete_gate_ignored', 'mutstub', 'lib.rs', 'permit.check_complete(&retained_memory::CompleteFacts { ordinary: &ordinary })) {', 'permit.check_complete(&retained_memory::CompleteFacts { ordinary: &ordinary }).or(Ok::<(), retained_memory::PhaseRefusal>(()))) {'),
 ('Y04_stack_failure_not_ordinary', 'mutstub', 'lib.rs', '(None, Some(request)) => ordinary_dispatch(request, &capture, solver_mode, Some(report), Some(Err(W1Fallback::StackReservation))),', '(None, Some(_request)) => Err("reserved stack unavailable".into()),'),
 ('Y05_permit_dispatch_bypassed', 'mutstub', 'lib.rs', 'Some(Ok((permit, report))) => return permitted_dispatch(permit, report, request, capture, solver_mode),', 'Some(Ok((_permit, report))) => Some(report),'),
 ('Y06_domain_guard_removed', 'mutstub', 'lib.rs', 'if case_state::is_load_state(&request.model) || pressure_runtime::is_exact(&request.model) {', 'if false {'),
 ('Y07_finalization_check_removed', 'mutstub', 'lib.rs', '    if source_finalization_failed(&ordinary) {\n        return Err("SOURCE_BLOCKS_FINALIZATION_FAILED".into());\n    }\n    // RV85 N1', '    // RV85 N1'),
 ('Y08_permitted_observer_unbound', 'mutstub', 'retained_product.rs', 'Self {prepared_probe:true,permit:Some(permit),..Self::default()}', 'Self {prepared_probe:true,permit:{let _=permit;None},..Self::default()}'),
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
        cmd = ['perl', '-e', 'alarm shift; exec @ARGV', '1200', 'cargo', 'test', '--locked', '--offline', '--target-dir', W + '/targets/i61-u3/' + tree + '-1c', '--lib', '--'] + FILTERS[tree]
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
