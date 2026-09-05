# Proposed exact private integration API v1
Module boundary_association.rs:
- pub(crate) struct BoundaryError { pub(crate) code: &'static str, pub(crate) message: String }
- pub(crate) struct ValidatedBoundaryBatch { private fields only; no Deserialize; constructor only preflight_batch below }
- pub(crate) fn preflight_batch(base_model:&Value, batch:&Value)->Result<ValidatedBoundaryBatch,BoundaryError>. Checks all optional boundary member payloads in batch.operations against original base and freezes exact canonical member payloads plus base/batch hash in private fields. Empty boundary set allowed.
- pub(crate) fn validate_create(current_model:&Value, support_payload:&Value, context:Option<&ValidatedBoundaryBatch>)->Result<(),BoundaryError>. None enforces single-route group-newness. Some permits existing new-group prefix only when exact validated canonical payload in token; still checks association/member shape and existing support-ID collision. It verifies existing same-group records are only earlier allowed token member payloads, rejecting drift/noncanonical injected members.
- pub(crate) fn validate_association(association:&Value)->Result<(),BoundaryError>.

Module atomic_batch.rs:
- pub fn validate_operation_batch(model:&Value,batch:&Value,claimed:Option<&Value>)->Value; pub fn apply_operation_batch(...) ->Value.
- common implementation first strict import preflight + initial hash check, then boundary_association::preflight_batch once.
- each private step invokes crate::apply_operation_in_batch(&current_model,&intent,Some(&fresh_claim),&context)->OperationOutcome.

Root lib.rs manager-only:
- existing public validate_operation/apply_operation unchanged; run internal dispatch always context None.
- crate-private fn apply_operation_in_batch(model:&Value,intent:&Value,claimed:Option<&Value>,context:&ValidatedBoundaryBatch)->OperationOutcome routes same internal run implementation with context Some. Not public export, not Wasm command, not JSON parameter.
- shared internal run context forwarded only into resolve_create_support. Metadata preserved exactly in ordinary support construction after validate_create.
- public batch functions reexported from atomic_batch; native/Wasm facade has batch+claimed only, never context.

Module children may not edit lib.rs, each other's module, or native wrapper; manager integrates these exact names/signatures. Refutation/acceptance precedes source.

## V2 required basis helper
- boundary_association::requires_model_hash(model:&Value,intent:&Value)->bool returns true for any operation targeting an existing associated support and for create_support payload carrying association. Root shared run invokes this before resolution; missing/null claim yields blocking OP-BOUNDARY-MODEL-HASH-REQUIRED, and existing check_claimed_model_hash validates any provided complete claim. This applies equally to public single and private batch entry. No input flag can disable it.
Tests add missing/malformed/mismatched claims for associated creation/configuration/scalar edit/deletion, changed association with same configuration.before, and legacy unassociated behavior unchanged.
