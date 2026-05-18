# TASK RUN - TP-VERIFY-012D Adapter/Source-Chain Traceability Gap Triage

## Identity

- Requested by: WORKING_ITEMS orchestrator
- Agent role: canonical `TASK`
- DeliverableID: DEL-13-04
- PackageID: PKG-13
- TaskProfile: DELIVERABLE_TASK
- Purpose: adapter/source-chain traceability gap triage
- Timestamp: 2026-05-17T13:03:27-0600
- Initial git state: `## main...origin/main`

## Required Reads

Read before acting:

- `AGENTS.md`
- `docs/CONTRACT.md`
- `_CONTEXT.md`
- `_STATUS.md`
- `_REFERENCES.md`
- `_DEPENDENCIES.md`
- `Dependencies.csv`
- `MEMORY.md`
- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`

Additional read-only evidence inspected:

- `core/model_transform/physical_to_analytical/contract.py`
- `core/model_transform/physical_to_analytical/_solver_boundary_adapter.py`
- `tests/test_analytical_solver_boundary_adapter.py`
- `validation/benchmarks/mechanics/fixtures/tp_phys_014_canonical_analytical_payload.json`
- `validation/benchmarks/mechanics/src/lib.rs`
- `validation/hand_calcs/mechanics/tp_phys_015a_canonical_solve_result_envelope.md`
- `schemas/model.schema.yaml`
- `schemas/results.schema.yaml`
- `schemas/headless_runner.schema.yaml`
- TP-PHYS-015 worker run records for DEL-09-01, DEL-13-04, DEL-09-02, DEL-08-04, and DEL-10-05
- TP-PHYS-015 parent fan-in run record

## Triage Findings

DEL-13-04 has strong local traceability evidence through the transform and
adapter boundary:

- `transform_physical_to_analytical` preserves the physical source as
  `source_model_ref`, derives an `analytical_solver_model`, and emits stable
  object-level `traceability_links` from physical records to analytical
  records or diagnostics.
- The internal solver-boundary adapter preserves `source_model_ref` at result
  root and emits deterministic DTO records for nodes, straight-pipe
  connectivity, property bindings, supports, load-case records, load
  applications, and diagnostics.
- TP-PHYS-015B proves invalid or unsupported adapter load records remain
  explicit `load_case_diagnostics` with `blocked_by_diagnostics`, no emitted
  load applications, and no hidden `user_loads` fallback surface.
- TP-PHYS-015A records the validation-local end-to-end chain from physical
  source ref to analytical payload, solver DTOs, load vector, and result
  envelope evidence.

The remaining gaps are integration and vocabulary gaps, not evidence of hidden
defaults inside DEL-13-04.

## Gap Classification Matrix

| Gap | Classification | Recommended owner | Rationale | Non-goals |
|---|---|---|---|---|
| Per-result-value multi-hop trace chain from physical source -> analytical model -> adapter DTO -> solver input -> result evidence is not first-class in the result schema. | READY_FOR_SCHEMA_TRANCHE | DEL-08-04 Result export format, with DEL-13-04 as upstream trace-source reviewer | TP-PHYS-015A can describe the chain and existing records carry `object_ref`, `basis_ref`, `station_ref`, provenance, and envelope refs, but there is no governed per-value chain field. | Do not add schema fields in DEL-13-04; do not make public export/runtime behavior claims. |
| Canonical TP-PHYS-014 standalone analytical payload fixture has `source_model_ref` and per-quantity provenance, but its `traceability_links` array is empty. | READY_FOR_RUNTIME_TEST_TRANCHE | DEL-09-01 Mechanics benchmark suite, with DEL-13-04 fixture/source-chain review | The generated transform path can emit trace links, while the validation-local payload fixture is standalone evidence. A focused benchmark/test tranche can require nonempty or externally referenced trace links without changing schemas. | Do not retrofit fixture data in this triage slice; do not treat empty fixture links as a lifecycle blocker. |
| Adapter DTOs preserve source refs at object/load-record level, but they do not carry a dedicated adapter DTO identity/hash per DTO for downstream result refs. | NEEDS_CROSS_DELIVERABLE_RULING | DEL-13-04 plus DEL-08-04 and DEL-10-05 | DTO identity affects adapter internals, result-export refs, and headless pass-through checksum vocabulary. The owner boundary should be decided before adding IDs or hashes. | Do not invent adapter DTO hash policy; do not modify headless or result-export checksum vocabularies here. |
| Adapter diagnostics are explicit locally, but there is no approved mapping from `ASBA-*` diagnostics into DEL-08-04 result-envelope diagnostic classes or DEL-10-05 runner diagnostics. | NEEDS_CROSS_DELIVERABLE_RULING | DEL-08-04 and DEL-10-05, with DEL-13-04 diagnostic-code source ownership | TP-PHYS-015B proves the adapter diagnostics exist; TP-PHYS-015D1/D2 record adjacent diagnostic vocabulary differences. A shared or bridged vocabulary needs cross-deliverable authority. | Do not collapse adapter diagnostics into runner/result diagnostics without approved vocabulary. |
| Physical-source traceability is object-level, not field-level for every copied quantity, coordinate, orientation vector, support property, or load quantity. | KEEP_AS_TBD | DEL-13-04 | Current DEL-13-04 requirements require traceability from records to analytical outcomes, diagnostics, omissions, or assumptions. Field-level traceability would expand the contract beyond current accepted evidence. | Do not add field-level traceability unless a later schema/contract tranche explicitly authorizes it. |
| Solver input to result evidence is validation-local in TP-PHYS-015A, not an application/runtime contract from adapter DTOs into a public solver result envelope. | READY_FOR_RUNTIME_TEST_TRANCHE | DEL-09-01 Mechanics benchmark suite or a later solver/result integration tranche | Existing benchmarks prove the path can be represented and validated in memory. A later test-only integration tranche can strengthen the runtime boundary without adding public CLI/API behavior. | Do not add public runtime commands, GUI, persistence, reports, or release behavior. |
| Result evidence can reference physical and analytical sources, but load-vector evidence and station-resultant set typing remain generic result-export vocabulary. | READY_FOR_SCHEMA_TRANCHE | DEL-08-04 Result export format | TP-PHYS-015D1 already records this as a result-export vocabulary gap. It is downstream of DEL-13-04 and should not be solved inside the adapter contract. | Do not add result-export schema vocabulary from DEL-13-04. |

## Recommended Next Work

1. Open a narrow DEL-08-04 schema tranche for result-boundary vocabulary if the
   project wants first-class load-vector, station-resultant, and per-value
   multi-hop trace-chain fields.
2. Open a DEL-09-01 runtime/test tranche if the project wants benchmark
   evidence that every canonical TP-PHYS-015 result value links back to a
   generated transform trace link or an explicit external source-chain ref.
3. Open a cross-deliverable ruling before adding adapter DTO identity/hashes or
   reconciling adapter/result/runner diagnostic vocabularies.

## Validation

- `git status --short --branch` before edits: `## main...origin/main`.
- `git diff --check` passed after edits.
- Final status review showed this TASK changed only this run record and
  deliverable `MEMORY.md` within DEL-13-04; concurrent TP-VERIFY-012A/B edits
  in DEL-08-04 and DEL-10-05 were observed and left untouched.
- No schemas, code, tests, lifecycle/status files, dependency registers, DAG or
  blocker files, review findings, release records, acceptance records, public
  API/CLI/runtime/report/persistence behavior, protected content, or
  professional/code-compliance surfaces were changed.

## Closeout

TP-VERIFY-012D completed as DEL-13-04 evidence-only triage. The adapter/source
chain is adequate for object-level deterministic transform and adapter
diagnostic evidence, while first-class per-result trace chains, DTO identity,
and diagnostic-vocabulary reconciliation require separately approved schema,
runtime-test, or cross-deliverable work.
