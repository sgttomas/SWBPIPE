# RV85 mutant definitions: (name, scope, file under PP/src, old, new). old=None is a control.
from i61_set import I61_SET
CONTROLS = [
 ('NONE_lib', 'lib', 'lib.rs', None, None),
 ('NONE_all', 'all', 'lib.rs', None, None),
 ('NONE_i61', 'i61', 'lib.rs', None, None),
]
# RV82's R08b, verbatim (REVIEW_RV82/u1_serializer_02/evidence/mutants_rv82_g2.py:71), whole PP --lib.
RV82 = [
 ('R08b_rv82_verbatim_whole_lib', 'lib', 'retained_wire.rs', '(after_conserved(e, before, increment, after), "cases[].run.invocation_after"),', '(true, "cases[].run.invocation_after"),'),
]
OWN = [
 # The dispatch (reachable).
 ('V01_ordinary_finalization_check_removed', 'all', 'lib.rs',
  '    let result = run_linear_static_preview_captured(request, solver_mode, Some(capture), &mut budget);\n    if source_finalization_failed(&result) {',
  '    let result = run_linear_static_preview_captured(request, solver_mode, Some(capture), &mut budget);\n    if false && source_finalization_failed(&result) {'),
 ('V02_finalization_predicate_drops_load_reference', 'all', 'lib.rs',
  'SOURCE_BLOCKS_SEMANTIC_CONTRACT_ID | PHYSICS_SOURCE_SEMANTIC_CONTRACT_ID | LOAD_REFERENCE_SOURCE_SEMANTIC_CONTRACT_ID)\n        && envelope.source_block_recovery.is_none()',
  'SOURCE_BLOCKS_SEMANTIC_CONTRACT_ID | PHYSICS_SOURCE_SEMANTIC_CONTRACT_ID)\n        && envelope.source_block_recovery.is_none()'),
 ('V03_no_permit_route_reports_w1_result', 'all', 'lib.rs',
  'ordinary_dispatch(request, &capture, solver_mode, admission, None)\n}',
  'ordinary_dispatch(request, &capture, solver_mode, admission, Some(Err(W1Fallback::Domain)))\n}'),
 ('V04_ordinary_exact_pressure_budget_dropped', 'all', 'lib.rs',
  '    if pressure_runtime::is_exact(&request.model) {\n        budget.per_case_limit = PHYSICS_SOURCE_WORK_LIMIT;\n    }\n    #[cfg(test)]\n    retained_memory::tests::ordinary_dispatch_entered();',
  '    #[cfg(test)]\n    retained_memory::tests::ordinary_dispatch_entered();'),
 # The fallback (reachable through the committed retained_w1 tests).
 ('V05_coexistence_predicate_inverted', 'lib', 'lib.rs',
  'if ordinary.source_block_recovery.is_some() {\n        return (ordinary, Err(W1Fallback::Coexistence));',
  'if ordinary.source_block_recovery.is_none() {\n        return (ordinary, Err(W1Fallback::Coexistence));'),
 ('V06_precommit_invocation_without_mode', 'lib', 'lib.rs',
  'serde_json::json!({"request": capture.borrowed_raw(), "solver_mode": capture.mode().as_str()});',
  'serde_json::json!({"request": capture.borrowed_raw()});'),
 ('V07_late_refusal_after_preparation', 'lib', 'lib.rs',
  '    if let Some(refusal) = observer.late_refusal() {\n        let refusal = refusal.clone();\n        return (ordinary, Err(W1Fallback::LateGate(refusal)));\n    }\n',
  ''),
 # The frozen split.
 ('V08_freeze_skips_trace_snapshot', 'lib', 'retained_product.rs',
  '                // The adapter snapshot is final here: no adapter event follows.\n                self.trace.freeze(&self.capture);\n',
  ''),
 ('V09_frozen_typed_trace_drops_proof_work', 'lib', 'retained_product.rs',
  '            trace::ResultRef::Ready,Some(self.certificate.work()),None,None,costs)\n    }\n}\n\n#[cfg(test)]\nimpl PreparedCandidateRefusal {',
  '            trace::ResultRef::Ready,None,None,None,costs)\n    }\n}\n\n#[cfg(test)]\nimpl PreparedCandidateRefusal {'),
 ('V10_commit_private_not_marked', 'lib', 'retained_product.rs',
  'self.prepared.trace.private_committed=true;', 'self.prepared.trace.private_committed=false;'),
 ('V11_staged_overlay_from_payload_maxima_skipped', 'lib', 'retained_product.rs',
  '    for patch in &payload.maxima {\n        let object=extrema[patch.evidence_index].as_object_mut().unwrap();',
  '    for patch in payload.maxima.iter().take(0) {\n        let object=extrema[patch.evidence_index].as_object_mut().unwrap();'),
 ('V12_F4_none_branch_unreachable', 'lib', 'retained_product.rs',
  '                    None=>{self.trace.fail_entered();Err(PreparedCandidateRefusal{ordinary,prepared:self,\n                        error:PreparedCandidateError::Capture("frozen candidate without certificate".into()),certificate:None,values:Some(payload.values)})}',
  '                    None=>unreachable!("RV85: certified Ok always carries its certificate"),'),
]
MUTANTS = CONTROLS + RV82 + I61_SET + OWN
