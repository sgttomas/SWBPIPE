# RV85 grant-1c mutant definitions: (name, scope, file under PP/src, old, new). old=None is a control.
from i61_set import I61_SET
CONTROLS = [
 ('NONE_lib', 'lib', 'lib.rs', None, None),
 ('NONE_i61', 'i61', 'lib.rs', None, None),
]
OWN = [
 # N6: the overlay's other typed sites.
 ('Q01_values_site_fault_ignored', 'lib', 'retained_product.rs',
  'row.value=*payload.values.value(i).ok_or(StagingFault("values"))?;', 'if let Some(v)=payload.values.value(i){row.value=*v;}'),
 ('Q02_key_site_fault_ignored', 'lib', 'retained_product.rs',
  '*object.get_mut(key).ok_or(StagingFault("pipe_stress_extrema[].key"))?=serde_json::Value::Number(number.clone());',
  'if let Some(slot)=object.get_mut(key){*slot=serde_json::Value::Number(number.clone());}'),
 # N7: a re-derivation through a helper defined outside the guard's section.
 ('Q03_rederive_via_outside_helper', 'lib', 'lib.rs',
  'fn source_finalization_failed(envelope: &MechanicsEnvelope) -> bool {',
  'fn rv85_requested(c: &source_receipt::CapturedInvocation) -> LinearStaticPreviewRequest { serde_json::from_value(c.borrowed_raw().clone()).expect("rv85") }\n\nfn source_finalization_failed(envelope: &MechanicsEnvelope) -> bool {'),
]
# Q03 needs two edits (the helper and the call): a list of pairs.
OWN[2] = ('Q03_rederive_via_outside_helper', 'lib', 'lib.rs', [
  ('fn source_finalization_failed(envelope: &MechanicsEnvelope) -> bool {',
   'fn rv85_requested(c: &source_receipt::CapturedInvocation) -> LinearStaticPreviewRequest { serde_json::from_value(c.borrowed_raw().clone()).expect("rv85") }\n\nfn source_finalization_failed(envelope: &MechanicsEnvelope) -> bool {'),
  ('    let ordinary = run_linear_static_preview_observed(request, solver_mode, Some(capture), &mut budget, Some(&mut observer));',
   '    let _ = request;\n    let request = rv85_requested(capture);\n    let ordinary = run_linear_static_preview_observed(request, solver_mode, Some(capture), &mut budget, Some(&mut observer));'),
 ], None)
MUTANTS = CONTROLS + I61_SET + OWN
