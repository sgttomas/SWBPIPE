# Guidance: DEL-16-01 Structured model operation schema

## Purpose

This deliverable makes model edits explicit, reviewable, and controllable before they change accepted model state. The current schema evidence is `schemas/model_operation.schema.json`; current invented fixtures are under `fixtures/model_operations/`.

Sources: `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-069, OBJ-015, PKG-16; `_CONTEXT.md`; `schemas/model_operation.schema.json`; `tests/test_model_operation_schema.py`.

## Principles

| Principle | Guidance |
|---|---|
| Operation records are proposals until downstream controls run | Treat GUI and agent edits as operation data, not as direct persisted-project mutation. The schema contract sets `direct_model_mutation_allowed = false`. |
| Use the implemented envelope | Current operation documents use `schema_version`, `deliverable_id`, `package_id`, `scope_item`, `objectives`, `operation_contract_status`, and `operation_set` at the root. |
| Use the implemented operation record structure | Current records include operation identity/kind/status/author, target references, preconditions, changes, validation, diagnostics, diff-preview refs, assumptions, provenance, and professional-boundary fields. |
| Preserve source boundaries | Public fixtures and examples must remain invented or otherwise cleared, with provenance, redistribution, review, and privacy-classification fields. |
| Prefer references over duplicate model structure | Use `Reference` objects and model-basis/precondition hashes rather than embedding the full canonical physical model in operation records. |
| Expose missing inputs | Missing unit metadata or other solve/rule-required information should surface as diagnostics or blocked validation, never hidden defaults. |
| Keep validation ownership separate | DEL-16-01 defines schema structure. DEL-16-02 owns validator and diff-preview behavior even when its tests consume DEL-16-01 fixtures. |
| Keep audit and acceptance ownership separate | DEL-16-03 owns acceptance and operation audit trail behavior. DEL-16-01 records boundary hooks only. |

Sources: `docs/CONTRACT.md` OPS-K-DATA-2, OPS-K-AUTH-1, OPS-K-AGENT-1, OPS-K-AGENT-4; `schemas/model_operation.schema.json`; `tests/test_operation_validation_preview.py`.

## Considerations

- The current schema establishes the concrete path `schemas/model_operation.schema.json`, root envelope fields, operation/change enums, reference/precondition/hash hooks, provenance requirements, validation placeholders, unit requirements, and professional-boundary flags.
- The fixture `fixtures/model_operations/invented_operation_set_valid.json` covers the declared operation kinds and change kinds using invented public example data.
- The fixture `fixtures/model_operations/invented_accepted_model_state.json` provides the invented physical source-of-truth model state and `sha256:invented-state-001` hash referenced by operation-set model basis and preconditions.
- Exact persistence granularity, physical project package/container behavior, and broad hash partitioning remain TBD beyond the current model-state hash hooks.
- Runtime validation, deterministic diff preview, blocking behavior, and controlled application remain DEL-16-02 behavior. `tests/test_operation_validation_preview.py` is useful adjacent evidence but does not transfer ownership to DEL-16-01.
- User acceptance, audit trail records, and human review dispositions remain DEL-16-03/review-governance behavior. Existing `Review_Findings.csv` human dispositions are not changed by this evidence-alignment pass.
- Agent operation autonomy remains bounded by proposal status, downstream user acceptance, and professional-boundary controls.

## Trade-offs

| Topic | Current Position |
|---|---|
| Broad operation enum vs specialized records | The implemented schema uses an `OperationKind` enum plus structured `OperationChange` records. More granular future edit classes remain TBD. |
| Patch-like payload vs command-like payload | The implemented schema supports `value_kind` values including `structured_patch`, `quantity`, `reference_list`, and `text`; it does not settle a universal patch grammar. |
| Embedding snapshots vs referencing targets | Current schema uses target references, model-basis references, accepted model-state hash, and required current hashes rather than embedding full model snapshots. |
| Agent-authored operations | Agent authoring is represented by `author_type = agent`, but records remain proposals and carry required professional-boundary negatives. |
| Fixtures | Current fixtures are invented public examples with provenance and privacy classification. They are schema evidence, not engineering defaults. |

## Examples

Current example evidence is the invented operation set at `fixtures/model_operations/invented_operation_set_valid.json`. It includes:

- `op:add-node` using `operation_kind = add` and `change_kind = add_object`;
- `op:resize` using `operation_kind = modify` and `change_kind = set_field`;
- `op:move-node` using `operation_kind = move` and `change_kind = move_geometry`;
- `op:delete-load` using `operation_kind = delete` and `change_kind = remove_object`;
- `op:reconnect-component` using `operation_kind = reconnect` and `change_kind = reconnect`;
- `op:update-constraint`, `op:update-load`, `op:update-support`, and `op:attach-design-knowledge` for the remaining declared categories.

These examples remain invented fixture evidence only. They must not be treated as professional approval, code compliance, or reusable engineering defaults.

## Conflict Table (for human ruling)

| Conflict ID | Conflict (short statement) | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling (TBD) |
|---|---|---|---|---|---|---|
| CT-001 | Operation granularity is required by SOW-069, but exact future edit-class granularity is not fully settled beyond the current schema enums. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-069 | `schemas/model_operation.schema.json` `$defs.OperationKind` and `$defs.OperationChange` | Specification Requirements; Procedure Steps | Treat current enums/fixtures as implemented DEL-16-01 evidence; keep broader operation granularity TBD. | TBD |
| CT-002 | Agent edits are in scope, but autonomy level is unresolved beyond proposal and human-review boundaries. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-069 | `schemas/model_operation.schema.json` `OperationAuthorType`, `OperationContractStatus`, and `ProfessionalBoundary` | Specification Requirements; Procedure Verification | Allow `author_type = agent` operation proposals only; preserve downstream acceptance/audit and professional-boundary controls. | TBD |
| CT-003 | Hash binding is implemented for accepted model state, but broader persistence/hash partitioning remains unsettled. | `schemas/model_operation.schema.json` `$defs.OperationModelBasis` and `$defs.OperationPrecondition` | `_CONTEXT.md` Architecture Basis Injection still lists physical project package/container as TBD | Datasheet Conditions; Specification R010; Procedure Verification | Record current model-state hash hooks as fact; keep exact persistence granularity and broader hash partitioning TBD. | TBD |
