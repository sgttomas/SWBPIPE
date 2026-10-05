# RV85 grant-1b mutant definitions: (name, scope, file under PP/src, old, new). old=None is a control.
from i61_set import I61_SET
CONTROLS = [
 ('NONE_lib', 'lib', 'lib.rs', None, None),
 ('NONE_i61', 'i61', 'lib.rs', None, None),
]
# RV85 grant-1 survivors, re-run on 4b31bbf23a (same text).
SURVIVORS = [
 ('V02_finalization_predicate_drops_load_reference', 'all', 'lib.rs',
  'SOURCE_BLOCKS_SEMANTIC_CONTRACT_ID | PHYSICS_SOURCE_SEMANTIC_CONTRACT_ID | LOAD_REFERENCE_SOURCE_SEMANTIC_CONTRACT_ID)\n        && envelope.source_block_recovery.is_none()',
  'SOURCE_BLOCKS_SEMANTIC_CONTRACT_ID | PHYSICS_SOURCE_SEMANTIC_CONTRACT_ID)\n        && envelope.source_block_recovery.is_none()'),
 ('V07_late_refusal_after_preparation', 'lib', 'lib.rs',
  '    if let Some(refusal) = observer.late_refusal() {\n        let refusal = refusal.clone();\n        return (ordinary, Err(W1Fallback::LateGate(refusal)));\n    }\n', ''),
 ('V12_F4_none_branch_unreachable', 'lib', 'retained_product.rs',
  '                    None=>{self.trace.fail_entered();Err(PreparedCandidateRefusal{ordinary,prepared:self,\n                        error:PreparedCandidateError::Capture("frozen candidate without certificate".into()),certificate:None,values:Some(payload.values)})}',
  '                    None=>unreachable!("RV85: certified Ok always carries its certificate"),'),
]
OWN = [
 # The notice (R-2).
 ('W01_message_reservation_short', 'lib', 'lib.rs',
  'const RECEIPT_ENCODING_DETAIL_MAX: usize = 25;', 'const RECEIPT_ENCODING_DETAIL_MAX: usize = 0;'),
 ('W02_message_not_reserved', 'lib', 'lib.rs',
  '        message.try_reserve_exact(RETAINED_UNAVAILABLE_NOTICE.len() + RECEIPT_ENCODING_REASON.len() + RECEIPT_ENCODING_DETAIL_MAX + 1).ok()?;\n', ''),
 ('W03_notice_first_not_after_prefix', 'lib', 'lib.rs',
  '        ordinary.diagnostics.push(notice);\n        (ordinary, Err(cause))', '        ordinary.diagnostics.insert(0, notice);\n        (ordinary, Err(cause))'),
 ('W04_notice_severity_warning', 'lib', 'lib.rs',
  '            severity: "info".to_owned(),\n            message,', '            severity: "warning".to_owned(),\n            message,'),
 ('W05_notice_names_invocation_not_case', 'lib', 'lib.rs',
  '            affected_refs: vec![case_id.to_owned()],', '            affected_refs: vec!["invocation".to_owned()],'),
 ('W06_reason_without_detail_dot', 'lib', 'lib.rs',
  "                notice.message.push_str(detail);\n                notice.message.push('.');", "                notice.message.push_str(detail);"),
 ('W07_w1_case_any_first_case', 'lib', 'lib.rs',
  '        (Some([case]), 0) => case["id"].as_str(),', '        (Some([case, ..]), 0) => case["id"].as_str(),'),
 # The carrier (R-1).
 ('W08_successor_variant_carries_ordinary', 'lib', 'lib.rs',
  '            Some(Ok(successor)) => RetainedPublication::Successor(successor.0),', '            Some(Ok(_successor)) => RetainedPublication::Successor(serde_json::to_value(&self.envelope).unwrap()),'),
 ('W09_successor_accessor_on_fallback', 'lib', 'lib.rs',
  '            Some(Ok(successor)) => Some(&successor.0),\n            _ => None,', '            Some(Ok(successor)) => Some(&successor.0),\n            Some(Err(_)) => Some(&serde_json::Value::Null),\n            None => None,'),
 # The carried hooks (S2).
 ('W10_carry_drops_ceiling_install', 'lib', 'lib.rs',
  '        super::DENSE_SCRUTINY_CEILING_OVERRIDE.with(|c| c.set(armed.ceiling));\n', ''),
]
MUTANTS = CONTROLS + SURVIVORS + I61_SET + OWN
