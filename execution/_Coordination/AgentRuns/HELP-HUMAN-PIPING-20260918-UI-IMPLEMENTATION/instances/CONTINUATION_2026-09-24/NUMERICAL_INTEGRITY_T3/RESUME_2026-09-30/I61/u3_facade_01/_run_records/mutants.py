#!/usr/bin/env python3
"""U3 grant 1 mutants: each applies one source change in a disposable mutant tree
(`mut` = candidate; `mutstub` = candidate plus the archive stub), runs the tests
that guard the branch, records killed/survived, and restores the bytes."""
import subprocess, sys, os, hashlib
W = os.environ['I61_WT']
S = W + '/scratch/i61_u3_facade_01'
TREES = {'mut': S + '/mut/projects/chirality-piping/core/product_physics',
         'mutstub': S + '/mutstub/projects/chirality-piping/core/product_physics'}
FILTERS = {'mut': ['retained_facade_tests', 'u1_', 'u2_', 'u1g2', 'i61_certificate_prefixes'],
           'mutstub': ['retained_facade_tests::u3_archive_stub']}
MUTANTS = [
 # Committed tests (the private driver over the actual single ordinary run).
 ('X01_coexistence_guard', 'mut', 'lib.rs', 'if ordinary.source_block_recovery.is_some() {\n        return (ordinary, Err(W1Fallback::Coexistence));', 'if false {\n        return (ordinary, Err(W1Fallback::Coexistence));'),
 ('X02_preparation_cause', 'mut', 'lib.rs', 'Err(failure) => return (failure.ordinary, Err(W1Fallback::Preparation)),', 'Err(failure) => return (failure.ordinary, Err(W1Fallback::Candidate)),'),
 ('X03_native_guard', 'mut', 'lib.rs', 'if prepared.solve_native().is_err() {', 'if { let _ = prepared.solve_native(); false } {'),
 ('X04_candidate_cause', 'mut', 'lib.rs', 'Err(refusal) => return (refusal.ordinary, Err(W1Fallback::Candidate)),', 'Err(refusal) => return (refusal.ordinary, Err(W1Fallback::Native)),'),
 ('X05_serialize_ordinary_not_staged', 'mut', 'lib.rs', 'retained_wire::serialize_frozen(&frozen, &staged, capture);', 'retained_wire::serialize_frozen(&frozen, frozen.ordinary(), capture);'),
 ('X06_serializer_fallback_staged', 'mut', 'lib.rs', 'Err(failure) => return (frozen.into_ordinary(), Err(W1Fallback::Serializer(failure))),', 'Err(failure) => return (frozen.staged_envelope(), Err(W1Fallback::Serializer(failure))),'),
 ('X07_precommit_skipped', 'mut', 'lib.rs', 'validate(&successor, Some(&invocation)) {', 'validate(&successor, Some(&invocation)).map(|_| ()).or_else(|e| if e.gate.is_empty() { Err(e) } else { Ok(()) }) {'),
 ('X08_precommit_unbound', 'mut', 'lib.rs', 'validate(&successor, Some(&invocation)) {', 'validate(&successor, None) {'),
 ('X09_precommit_fallback_staged', 'mut', 'lib.rs', 'return (frozen.into_ordinary(), Err(W1Fallback::Precommit { gate: error.gate, code: error.code }));', 'return (frozen.staged_envelope(), Err(W1Fallback::Precommit { gate: error.gate, code: error.code }));'),
 ('X10_transfer_publishes_staged', 'mut', 'lib.rs', '(frozen.into_ordinary(), Ok(RetainedSuccessor(successor)))', '(frozen.staged_envelope(), Ok(RetainedSuccessor(successor)))'),
 ('X11_stack_size_ignored', 'mut', 'lib.rs', 'std::thread::Builder::new().stack_size(bytes).spawn_scoped(scope, work)', 'std::thread::Builder::new().stack_size(bytes.min(2 << 20)).spawn_scoped(scope, work)'),
 ('X12_panic_payload_lost', 'mut', 'lib.rs', 'Err(payload) => std::panic::resume_unwind(payload),', 'Err(_payload) => panic!("reserved-stack work panicked"),'),
 ('X13_spawn_failure_not_reported', 'mut', 'lib.rs', '            Err(_) => None,\n        }\n    })', '            Err(error) => panic!("{error}"),\n        }\n    })'),
 ('X14_admission_report_dropped', 'mut', 'lib.rs', 'Some(Err(report)) => Some(report),', 'Some(Err(_report)) => None,'),
 ('X15_staging_overlay_skipped', 'mut', 'retained_product.rs', 'let mut staged=self.ordinary.clone();\n        apply_prepared_overlay(&mut staged,&self.payload);', 'let staged=self.ordinary.clone();'),
 ('X16_commit_overlay_skipped', 'mut', 'retained_product.rs', 'let mut envelope=self.ordinary;\n        apply_prepared_overlay(&mut envelope,&self.payload);', 'let envelope=self.ordinary;'),
 ('X17_serialize_frozen_reads_ordinary', 'mut', 'retained_wire.rs', '    serialize_selected_from(candidate, staged, invocation)\n', '    let _ = staged;\n    serialize_selected_from(candidate, candidate.ordinary(), invocation)\n'),
 ('X18_freeze_mutates_ordinary', 'mut', 'retained_product.rs', 'Some(certificate)=>Ok(FrozenCandidate{ordinary,payload,prepared:self,certificate}),', 'Some(certificate)=>{let mut ordinary=ordinary;apply_prepared_overlay(&mut ordinary,&payload);Ok(FrozenCandidate{ordinary,payload,prepared:self,certificate})},'),
 # ROOT's additions (RV82 N1' and N9).
 ('R08b_run_after_check_call_site', 'mut', 'retained_wire.rs', '(after_conserved(e, before, increment, after), "cases[].run.invocation_after"),', '(true, "cases[].run.invocation_after"),'),
 ('R08_run_after_check_helper', 'mut', 'retained_wire.rs', '    e.sum(&[before, increment], "cases[].run.invocation_after") == after\n', '    let _ = (before, increment, after);\n    true\n'),
 ('N1a_case_charge_call_site', 'mut', 'retained_wire.rs', '(total("case_charge", "cases[].run.case_charge") == case, "cases[].run.case_charge"),', '(true, "cases[].run.case_charge"),'),
 ('N1b_increment_call_site', 'mut', 'retained_wire.rs', '(total("invocation_increment", "cases[].run.invocation_increment") == increment, "cases[].run.invocation_increment"),', '(true, "cases[].run.invocation_increment"),'),
 ('N9a_second_parse_in_permitted_run', 'mut', 'lib.rs', '    let ordinary = run_linear_static_preview_observed(request, solver_mode, Some(capture), &mut budget, Some(&mut observer));', '    let (request, _) = source_receipt::CapturedInvocation::parse(capture.borrowed_raw().clone(), solver_mode).map_err(|e| e.0)?;\n    let ordinary = run_linear_static_preview_observed(request, solver_mode, Some(capture), &mut budget, Some(&mut observer));'),
 ('N9b_request_rederived_from_capture', 'mut', 'lib.rs', '    let ordinary = run_linear_static_preview_observed(request, solver_mode, Some(capture), &mut budget, Some(&mut observer));', '    let request: LinearStaticPreviewRequest = serde_json::from_value(capture.borrowed_raw().clone()).map_err(|e| e.to_string())?;\n    let ordinary = run_linear_static_preview_observed(request, solver_mode, Some(capture), &mut budget, Some(&mut observer));'),
 ('N9c_dispatch_parses_twice', 'mut', 'lib.rs', '    let (request, capture) = source_receipt::CapturedInvocation::parse(actual_request, solver_mode)', '    let (_, capture) = source_receipt::CapturedInvocation::parse(actual_request.clone(), solver_mode)\n        .map_err(|error| error.0)?;\n    let (request, _) = source_receipt::CapturedInvocation::parse(actual_request, solver_mode)'),
 # Unreachable without a permit: killed only behind the disposable stub (grant 2 commits these).
 ('Y01_late_gate_ignored', 'mutstub', 'retained_product.rs', 'if let Err(refusal)=permit.check_late(&facts) {self.late_refusal=Some(refusal);return;}', 'let _=permit.check_late(&facts);'),
 ('Y02_late_refusal_not_checked', 'mutstub', 'lib.rs', 'if let Some(refusal) = observer.late_refusal() {', 'if let Some(refusal) = observer.late_refusal().filter(|_| false) {'),
 ('Y03_complete_gate_ignored', 'mutstub', 'lib.rs', 'match permit.check_complete(&retained_memory::CompleteFacts { ordinary: &ordinary }) {', 'match permit.check_complete(&retained_memory::CompleteFacts { ordinary: &ordinary }).or(Ok::<(), retained_memory::PhaseRefusal>(())) {'),
 ('Y04_stack_failure_not_ordinary', 'mutstub', 'lib.rs', '(None, Some(request)) => ordinary_dispatch(request, &capture, solver_mode, None, Some(Err(W1Fallback::StackReservation))),', '(None, Some(_request)) => Err("reserved stack unavailable".into()),'),
 ('Y05_permit_dispatch_bypassed', 'mutstub', 'lib.rs', 'Some(Ok(permit)) => return permitted_dispatch(permit, request, capture, solver_mode),', 'Some(Ok(_permit)) => None,'),
 ('Y06_domain_guard_removed', 'mutstub', 'lib.rs', 'if case_state::is_load_state(&request.model) || pressure_runtime::is_exact(&request.model) {\n        return ordinary_dispatch(request, capture, solver_mode, None, Some(Err(W1Fallback::Domain)));', 'if false {\n        return ordinary_dispatch(request, capture, solver_mode, None, Some(Err(W1Fallback::Domain)));'),
 ('Y07_finalization_check_removed', 'mutstub', 'lib.rs', '    if source_finalization_failed(&ordinary) {\n        return Err("SOURCE_BLOCKS_FINALIZATION_FAILED".into());\n    }\n    let (envelope, retained)', '    let (envelope, retained)'),
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
        cmd = ['perl', '-e', 'alarm shift; exec @ARGV', '1200', 'cargo', 'test', '--locked', '--offline', '--target-dir', W + '/targets/i61-u3/' + tree, '--lib', '--'] + FILTERS[tree]
        env = dict(os.environ, CARGO_BUILD_JOBS='4', RUST_TEST_THREADS='2')
        if subprocess.run(['pgrep', '-f', 'memguard.sh'], capture_output=True).returncode != 0: sys.exit('MEMGUARD NOT RUNNING')
        with open(log, 'w') as out:
            rc = subprocess.run(cmd, cwd=M, env=env, stdout=out, stderr=subprocess.STDOUT).returncode
        text = open(log).read()
        failed = sorted({l.split()[1] for l in text.splitlines() if l.startswith('test ') and l.endswith('FAILED')})
        compile_error = 'error[' in text or 'error: could not compile' in text
        crashed = 'process didn\'t exit successfully' in text and not failed
        verdict = 'KILLED' if rc != 0 else 'SURVIVED'
        line = f'{name} [{tree}] {verdict} rc={rc} compile_error={compile_error} crashed={crashed} failed={failed}'
    finally:
        open(path, 'w').write(orig)
        assert sha(path) == before
    print(line, flush=True); results.append(line)
open(f'{S}/logs/mutants_summary.txt', 'a').write('\n'.join(results) + '\n')
