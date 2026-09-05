# B4 batch design refutation

Verdict: REVISE. Internal replay is a sound mechanism: apply_operation takes an immutable model, clones it, and returns the candidate without session/persistence side effects. Reusing it on a private evolving model preserves one mutation implementation. The batch wrapper must add three explicit contracts below before the design is ready for implementation.

## B4-C1 — Sanitize every nested acceptance claim, not only models

Source: operation_applier/src/lib.rs:1649–1707. Successful internal apply results carry mode=apply, validation.application_status=applied_to_session_model, acceptance.acceptance_basis=user_initiated_apply_in_local_session, acceptance.persistence_status=session_state_only_not_yet_saved, and audit_boundary.applied_model_is_new_document=true. They also carry applied_model_backend_hash. Merely stripping applied_model leaves false accepted-session evidence during validation and on rolled-back predecessors.

Correction: specify a distinct simulated-step projection, rather than serializing raw OperationOutcome and removing one field. Remove both applied_model and applied_model_backend_hash; label nested mode/status as internal simulation, replace acceptance with none_internal_simulation/no_session_application, and remove/rewrite applied_model_is_new_document. Preserve diagnostic and hypothetical diff facts; intermediate model-basis hashes may remain only as explicitly hypothetical replay evidence. Even in a successful apply batch, only the top-level result owns final application/acceptance. Test recursively that validation and failed apply contain no applied model, applied model hash, applied-to-session status, or accepted-local-session receipt at any depth.

## B4-C2 — Define imported intent preflight; existing applier does not enforce it

Source: operation_applier/src/lib.rs:1750–1813. check_intent_structure checks presence of operation_status and professional_boundary but does not enforce proposed status, author_type, source/rationale, or the individual professional booleans. check_audit_boundary checks only mutation_route and direct_model_mutation_allowed. Reusing this applier therefore does not fulfill B4's proposed offline guard by itself.

Correction: add an explicit backend preflight for every batch member before replay: author_type exactly user or agent for this path; operation_status exactly proposed; explicit structured-only route; direct mutation false; requires_user_acceptance true; mutates_accepted_model_state false; human_review_required true; all five software_makes_* claims false; nonempty operation/change IDs; correctly typed object/change fields. Agent source must specify concrete nonempty source_ref/source_channel/source_role and nonempty rationale; retain submitted fields without rewriting attribution as user. Reject malformed types/unknown author values and claimed approval rather than coercing/defaulting. Treat source fields as asserted source metadata, not verified actor identity.

schemas/model_operation.schema.json ModelOperationRecord and EditorOperationIntent are different wire shapes (target_refs/changes versus target/change). Pin the exact adapter validation contract and its relationship to canonical schema; do not claim full canonical JSON Schema validation merely because the partial editor checker returns schema_validation=passed. This is especially relevant to DEL-16-02 REQ-16-02-002 and DEL-16-03 audit acceptance.

## B4-C3 — Bind final UI commit and checkpoint to the same input snapshot

The initial backend hash check correctly binds its supplied model, but cannot detect a newer UI model while the asynchronous command runs. App.tsx:843–893 shows the present await-then-commit pattern. B4 currently states only one UI checkpoint per successful final result.

Correction: carry initial input hash and current session generation through dispatch. Before publishing result/receipt/checkpoint, compare against current session generation/model hash; if changed by load/undo/redo/another edit, reject the stale completion with no commit, receipt, or checkpoint. Alternatively explicitly lock every model-changing entry point through the apply; a disabled apply button alone is insufficient. The accepted checkpoint must be the actual still-current input model, and one undo must restore the whole batch atomically. Test an in-flight batch followed by model replacement/undo, plus successful one-checkpoint undo/redo.

## Hash/source invariants to retain

Require complete initial hash metadata before simulation; the existing single-operation optional-hash fallback must never be used at the batch boundary. Validate initial claim against original complete raw document before generating any internal hash. Internal fresh claims are implementation evidence, not a replacement for the submitted initial basis. Preserve original claim alongside input_backend_hash for audit. If imported proposals include a declared basis/preconditions, honor them; do not silently rebind them to the UI's fresh hash.

Specify batch_hash as SHA-256 of canonical submitted batch including ordered operations and all retained source/rationale fields, and retain a separate input-model hash. Reordering operations must alter the batch hash even though object-key ordering does not. Keep exact submitted JSON values for audit; parsing cannot preserve byte whitespace, so any exactness claim must be about parsed values unless original bytes are also retained. A successful replay step must have applied status, no blocking diagnostics, and a returned candidate model; otherwise roll back the entire private chain.

## D58 and authority

Offline imported intents do not require a live provider binding and fit SCA009 row11's single operation route. Source attribution is an assertion retained for review. D58 remains unchanged: Piping is not adopted as Root-runtime/App-harness client and no automation-condition successor is established. Local acceptance is not professional approval or identity verification. These claims are appropriately bounded in the candidate.

## Handoff

Return to HELP_HUMAN for N2 amendment and UI integration routing. No code, tests, or lifecycle changes. This is derivative design evidence against the captured source hashes, not implementation acceptance. Rerun on amended B4 design or relevant hash/audit/checkpoint consumer drift; later fresh frozen-source review and deterministic tests remain required. Actual inherited model identifier unavailable; no delegation.
