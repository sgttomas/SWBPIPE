# Datasheet: DEL-02-03 Code-neutral Analysis Boundary Model

## Identification

| Field | Value | Source |
|---|---|---|
| Deliverable ID | `DEL-02-03` | `_CONTEXT.md` |
| Name | Code-neutral analysis boundary model | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row `DEL-02-03` |
| Package | `PKG-02` Domain Model, Units, and Core Schemas | `_CONTEXT.md`; `docs/_Decomposition/SOFTWARE_DECOMP.md` section 7 |
| Deliverable type | `DATA_MODEL_CHANGE` | `_CONTEXT.md`; `docs/TYPES.md` section 3 |
| Scope item | `SOW-002` | `_CONTEXT.md`; `docs/_Registers/ScopeLedger.csv` row `SOW-002` |
| Objectives | `OBJ-001`, `OBJ-011` | `_CONTEXT.md`; `docs/_Decomposition/SOFTWARE_DECOMP.md` section 5 |
| Anticipated artifacts | `analysis_status` enum; `docs/SPEC.md` state model | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row `DEL-02-03` |
| Context envelope | `S`; risk `OK` | `_CONTEXT.md`; `docs/_Registers/ContextBudgetQA.csv` row `DEL-02-03` |

## Attributes

| Attribute | Draft value | Source / notes |
|---|---|---|
| Boundary purpose | Separate mechanics solve, user-supplied rule-pack check, and project-specific human professional acceptance. | `INIT.md` agent rule; `docs/DIRECTIVE.md` section 2.2; `docs/_Decomposition/SOFTWARE_DECOMP.md` sections 1 and 7 |
| Code-neutral scope | Solver computes mechanics; user-supplied rule packs evaluate acceptability. | `SOW-002`; `docs/PRD.md` section 6.1; `docs/INTENT.md` "Rule-pack intent" |
| Public data boundary | Public artifacts must not contain protected standards text, tables, examples, code-derived formulas, allowables, SIF/flexibility tables, or proprietary commercial data. | `docs/CONTRACT.md` `OPS-K-IP-1`; `docs/IP_AND_DATA_BOUNDARY.md` sections 2-3 |
| Status vocabulary basis | `MODEL_INCOMPLETE`, `MECHANICS_SOLVED`, `RULE_INPUTS_INCOMPLETE`, `USER_RULE_CHECKED`, `USER_RULE_FAILED`, `HUMAN_REVIEW_REQUIRED`, `HUMAN_APPROVED_FOR_PROJECT`. | `docs/TYPES.md` section 4 |
| Prohibited automatic status | `CODE_COMPLIANT` must not be used as an automatic software status. | `docs/TYPES.md` section 4; `docs/CONTRACT.md` `OPS-K-AUTH-1` |
| Authority levels | Software finding, solver result only, rule-pack finding, software computation using user data, always-required professional review, and human record only. | `docs/TYPES.md` section 4 |
| Result-envelope basis | Commands, queries, jobs, reproducibility metadata, diagnostics, and result envelopes must preserve the mechanics/rule/human distinction. | `docs/_Decomposition/SOFTWARE_DECOMP.md` section 8.1 `AB-00-03`; `docs/SPEC.md` sections 1 and 11 |
| Diagnostics basis | Diagnostics/result envelopes carry code, class, severity, source, affected object, message, remediation, and provenance. | `docs/_Decomposition/SOFTWARE_DECOMP.md` section 8.1 `AB-00-06` |
| Schema basis | Public schemas/interchange use JSON Schema 2020-12 where schema artifacts are produced. | `_CONTEXT.md` Architecture Basis Injection; `docs/_Decomposition/SOFTWARE_DECOMP.md` section 8.2 |
| Human acceptance binding | Human acceptance records, if used, bind to specific model/rule/report hashes and do not survive content changes without re-review. | `docs/CONTRACT.md` `OPS-K-AUTH-2` |

## Conditions

- The model applies only to the semantic and data-interface boundary for `DEL-02-03`; it does not implement numerical solving, rule-pack evaluation, GUI views, reports, or human-acceptance workflow screens. Source: `_CONTEXT.md` Package Exclusions and anticipated artifacts.
- Missing solve-required or rule-check-required values must be explicit findings, not silent defaults. Source: `docs/CONTRACT.md` `OPS-K-DATA-2`; `docs/PRD.md` section 6.2.
- A mechanics-only solve may exist before a rule-pack check, but it must not be represented as a code pass/fail state. Source: `docs/PRD.md` section 6.2.
- A user rule-pack result is a software computation using user data; it is not professional authentication. Source: `docs/TYPES.md` section 4; `docs/DIRECTIVE.md` section 3.
- Human professional acceptance is outside solver authority and may only be represented as a project-specific human record. Source: `docs/TYPES.md` section 4; `docs/CONTRACT.md` `OPS-K-AUTH-1`.
- ASSUMPTION: `analysis_status` is a coarse boundary enum for serialized result/state surfaces. Detailed rule-check outcomes, diagnostic arrays, and human acceptance record fields may be separate fields to avoid overloading the enum. Exact schema file path and field layout are TBD.

## Construction

Minimum draft status fields for downstream schema design:

| Field | Purpose | Status |
|---|---|---|
| `analysis_status` | One value from the status vocabulary basis. | Required by anticipated artifact; exact serialization TBD. |
| `status_authority` | Indicates software finding, solver result, rule-pack finding/computation, or human record. | ASSUMPTION, derived from `docs/TYPES.md` section 4. |
| `status_evidence_ref` | References solver result, rule-pack evaluation, missing-input diagnostic, or human acceptance record. | TBD. |
| `diagnostics` | Carries blocking/warning information with provenance. | Required in principle by `AB-00-06`; exact schema TBD. |
| `rule_pack_ref` | Identifies user rule-pack ID, version, checksum, source/provenance, and redistribution status when a rule check is attempted. | Supported by `docs/SPEC.md` section 6 and `docs/PRD.md` section 12.2. |
| `human_acceptance_ref` | Optional pointer to a human acceptance record outside solver core. | Governed by `OPS-K-AUTH-2`; exact storage location TBD. |
| `model_or_result_hash` | Binds status/evidence to specific content when hashing is available. | Supported by `AB-00-04`; exact hash scope TBD. |

Open storage and stale-state questions:

| Question | Current disposition | Source / notes |
|---|---|---|
| Where is a human acceptance record stored? | TBD. The pointer may reference an external project record, report/audit manifest entry, or future persistence object; this deliverable does not choose the storage location. | `docs/CONTRACT.md` `OPS-K-AUTH-2`; `docs/TYPES.md` section 4; `_CONTEXT.md` "Still TBD" implementation choices. |
| How is human acceptance marked stale after bound evidence changes? | TBD. The invariant is that the record does not survive model/rule/report content changes without re-review; the exact stale-state field or invalidation mechanism remains unresolved. | `docs/CONTRACT.md` `OPS-K-AUTH-2`; `docs/_Decomposition/SOFTWARE_DECOMP.md` section 8.2 hash/persistence TBDs. |

Minimum boundary interfaces:

| Interface | Inputs | Outputs | Boundary rule |
|---|---|---|---|
| Mechanics solve boundary | Unit-aware model data and solver settings | Mechanical result envelope or solve-blocking diagnostic | May produce `MECHANICS_SOLVED` but not rule pass/fail or human acceptance. |
| Rule-pack check boundary | Mechanical result envelope plus user-supplied rule pack and required inputs | Rule-check result, missing-input finding, or failed-check finding | May produce user-rule statuses only; no professional acceptance or automatic code-compliance claim. |
| Human acceptance boundary | Specific model/result/rule/report evidence and external human review record | Optional `HUMAN_APPROVED_FOR_PROJECT` record | Human record only; not emitted by solver core. |

## References

- `_CONTEXT.md` revision 0.4 for sealed deliverable identity, scope, artifacts, and architecture-basis injection.
- `docs/_Registers/Deliverables.csv` row `DEL-02-03`.
- `docs/_Registers/ScopeLedger.csv` row `SOW-002`.
- `docs/_Registers/ContextBudgetQA.csv` row `DEL-02-03`.
- `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4 sections 5, 7, 8, and 9.
- `docs/CONTRACT.md` section 1 invariants `OPS-K-IP-*`, `OPS-K-DATA-*`, `OPS-K-AUTH-*`, `OPS-K-MECH-2`, `OPS-K-AGENT-*`.
- `docs/TYPES.md` sections 4, 5, 6, and 8.
- `docs/DIRECTIVE.md` sections 2, 3, 4, and 5.
- `docs/SPEC.md` sections 1, 6, 7, 8, 9, and 11.
- `docs/PRD.md` sections 6.1, 6.2, 12, 17.3, and 17.4.
- `docs/INTENT.md` "Rule-pack intent" and mechanics/rule-pack boundary notes.
- `docs/IP_AND_DATA_BOUNDARY.md` sections 2-7.
