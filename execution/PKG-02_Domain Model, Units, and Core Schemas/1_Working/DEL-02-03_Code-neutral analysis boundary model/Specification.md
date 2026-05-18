# Specification: DEL-02-03 Code-neutral Analysis Boundary Model

## Scope

This specification defines the draft data-model boundary for `DEL-02-03`: states and interfaces that distinguish mechanics solve, user-rule check, and project-specific human professional acceptance.

In scope:

- `analysis_status` vocabulary and authority semantics.
- Boundary interfaces between mechanics solving, rule-pack checking, and human acceptance records.
- Minimum provenance, diagnostics, and evidence hooks needed to avoid status ambiguity.
- Documentation guidance for a future `docs/SPEC.md` state-model section.

Out of scope:

- Numerical solver implementation.
- Rule-pack expression grammar or evaluator implementation.
- GUI workflow behavior.
- Report rendering.
- External code, standard, or certification logic.
- Protected standards/code data, code tables, allowables, SIF/flexibility factors, or proprietary examples.

Primary governing sources are `_CONTEXT.md`, `SOW-002`, `docs/TYPES.md` section 4, `docs/CONTRACT.md`, `docs/DIRECTIVE.md`, `docs/PRD.md` sections 6.1/6.2/12/17.4, and `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4.

## Requirements

| ID | Requirement | Source |
|---|---|---|
| DEL-02-03-R01 | The boundary model shall represent the status vocabulary from `docs/TYPES.md` section 4: `MODEL_INCOMPLETE`, `MECHANICS_SOLVED`, `RULE_INPUTS_INCOMPLETE`, `USER_RULE_CHECKED`, `USER_RULE_FAILED`, `HUMAN_REVIEW_REQUIRED`, and `HUMAN_APPROVED_FOR_PROJECT`. | `docs/TYPES.md` section 4 |
| DEL-02-03-R02 | The model shall not include `CODE_COMPLIANT` as an automatic software status and shall not claim certification, sealing, approval, authentication, or engineering code compliance for reliance. | `docs/TYPES.md` section 4; `docs/CONTRACT.md` `OPS-K-AUTH-1`; `INIT.md` Agent rule |
| DEL-02-03-R03 | The mechanics solve boundary shall communicate that solver output is a mechanics result only. | `docs/CONTRACT.md` `OPS-K-MECH-2`; `docs/PRD.md` section 6.1; `docs/INTENT.md` mechanics/rule-pack boundary |
| DEL-02-03-R04 | The rule-check boundary shall require user-supplied rule-pack data for acceptability checks and shall not bundle proprietary or protected code content into the public model. | `SOW-002`; `docs/PRD.md` sections 6.1 and 12.1; `docs/IP_AND_DATA_BOUNDARY.md` sections 2-3 |
| DEL-02-03-R05 | Missing solve-required and rule-check-required values shall surface as explicit statuses or diagnostics, never as silent defaults. | `docs/CONTRACT.md` `OPS-K-DATA-2`; `docs/PRD.md` section 6.2 |
| DEL-02-03-R06 | The model shall keep user-rule pass/fail outcomes distinct from human professional acceptance. | `docs/DIRECTIVE.md` sections 2.2 and 3; `docs/TYPES.md` section 4 |
| DEL-02-03-R07 | A human acceptance record, if represented, shall be outside solver authority and bound to the specific model/rule/report evidence that was reviewed. | `docs/CONTRACT.md` `OPS-K-AUTH-2`; `docs/TYPES.md` section 4 |
| DEL-02-03-R08 | Status envelopes and diagnostics shall carry provenance sufficient to identify the source/evidence for the status. | `docs/_Decomposition/SOFTWARE_DECOMP.md` section 8.1 `AB-00-06`; `docs/SPEC.md` sections 7-8 |
| DEL-02-03-R09 | Where a public schema/interchange artifact is produced, it shall align with the JSON Schema 2020-12 baseline from SCA-001. | `_CONTEXT.md` Architecture Basis Injection; `docs/_Decomposition/SOFTWARE_DECOMP.md` section 8.2 |
| DEL-02-03-R10 | The model shall preserve application-service and API boundary separation; adapters/plugins shall not bypass governance, validation, diagnostics, or public/private data boundaries. | `AB-00-02`; `AB-00-07`; `docs/SPEC.md` section 1 |
| DEL-02-03-R11 | The model shall leave exact dependency versions, solver numerical library, rule expression grammar/library, public API transport, and physical project container as implementation-level TBD unless a later human project-authority decision resolves them. | `_CONTEXT.md` Still TBD; `docs/_Decomposition/SOFTWARE_DECOMP.md` section 8.2 |
| DEL-02-03-R12 | ASSUMPTION: If `analysis_status` remains a single enum, `USER_RULE_FAILED` represents an evaluated user-defined rule failure and `USER_RULE_CHECKED` represents completed user-rule evaluation without a blocking missing-input condition. A richer outcome field may be needed; exact split is TBD. | Inference from `docs/TYPES.md` section 4 and anticipated artifact `analysis_status enum` |

### Status Authority and Evidence Model

The following draft maps refine R01-R08 for downstream schema work. Exact schema names and storage paths remain `TBD`; the obligations below describe the minimum boundary evidence that must not be lost.

Minimum provenance sufficiency:

| Evidence area | Minimum contents | Source |
|---|---|---|
| Any emitted status | `analysis_status`, the authority/producer basis for that status, and a reference to the evidence that justifies it. | `docs/TYPES.md` section 4; `docs/CONTRACT.md` `OPS-K-AUTH-1`, `OPS-K-MECH-2` |
| Diagnostic-backed status | Diagnostic code, class, severity, source, affected object, message, remediation, and provenance. | `docs/_Decomposition/SOFTWARE_DECOMP.md` `AB-00-06`; `docs/SPEC.md` section 7 |
| User-rule status | Rule-pack identity, version/checksum, source/provenance note, redistribution/private status, and evaluation or missing-input evidence. | `docs/SPEC.md` section 6; `docs/PRD.md` section 12.2; `docs/IP_AND_DATA_BOUNDARY.md` section 7 |
| Human acceptance status | Pointer to a project-specific human acceptance record outside solver core and to the reviewed model/rule/report evidence. Storage location and stale-state field remain `TBD`. | `docs/TYPES.md` section 4; `docs/CONTRACT.md` `OPS-K-AUTH-2` |
| Reproducibility binding | Model/result hash or input manifest when available; exact hash scope remains `TBD`. | `docs/_Decomposition/SOFTWARE_DECOMP.md` `AB-00-04`; `docs/SPEC.md` section 8 |

Field-level obligation map:

| Field | Obligation | Applies when | Notes |
|---|---|---|---|
| `analysis_status` | Required. | Every boundary status record. | Values limited to the `docs/TYPES.md` section 4 vocabulary unless the human project authority later amends it. |
| `status_authority` | Required or derivable without ambiguity. | Every boundary status record. | Authority basis follows the status vocabulary: software finding, solver result only, rule-pack finding/computation, always-required review, or human record only. |
| `status_evidence_ref` | Required. | Every emitted status. | References the diagnostic, solver result envelope, rule-pack evaluation, missing-input finding, or human acceptance record; exact pointer format is `TBD`. |
| `diagnostics` | Required when a missing, blocking, warning, assumption, nonconvergence, IP-boundary, or failed-check condition is present. | Incomplete, blocking, warning, assumption, and failure conditions. | Must preserve provenance and affected-object context; clean states may still carry informational diagnostics. |
| `rule_pack_ref` | Required for rule-check statuses. | `RULE_INPUTS_INCOMPLETE`, `USER_RULE_CHECKED`, `USER_RULE_FAILED`; optional/TBD where a rule pack is selected but not yet run. | Must identify at least name/id, version or checksum, source/provenance, and private/redistribution status when available. |
| `human_acceptance_ref` | Required only for `HUMAN_APPROVED_FOR_PROJECT`; prohibited as a solver-generated result. | Human acceptance record represented in project data. | Storage location, permissions, and stale-state representation are `TBD`. |
| `model_or_result_hash` | Required when binding a human record or producing audit/reproducibility output with available hashes. | Human acceptance, audit/report, and reproducibility contexts. | Exact hash scope remains `TBD`; JSON payload hash basis follows the SCA-001 architecture baseline. |

Producer authority and evidence map:

| Status | Authorized producer / actor | Authority level | Minimum evidence |
|---|---|---|---|
| `MODEL_INCOMPLETE` | Model validation or solve precheck. | Software finding. | Solve-blocking diagnostic identifying missing physical data. |
| `MECHANICS_SOLVED` | Mechanics solver through result envelope. | Solver result only. | Solver result envelope and model/result evidence reference. |
| `RULE_INPUTS_INCOMPLETE` | Rule-pack completeness checker/evaluator. | Rule-pack finding. | Selected rule-pack reference plus missing rule-input diagnostic. |
| `USER_RULE_CHECKED` | Rule-pack evaluator using user-supplied data. | Software computation using user data. | Rule-pack reference, evaluation evidence, and any nonblocking diagnostics. |
| `USER_RULE_FAILED` | Rule-pack evaluator using user-supplied data. | Software computation using user data. | Rule-pack reference, failed-check evidence, and diagnostic/result context. |
| `HUMAN_REVIEW_REQUIRED` | Boundary/status model or reporting surface. | Always true for professional use. | Status presentation or report notice that professional reliance requires competent human review. |
| `HUMAN_APPROVED_FOR_PROJECT` | Human reviewer or project record outside solver core. | Human record only. | Human acceptance record pointer bound to reviewed model/rule/report evidence; not emitted by solver or rule-pack code. |

## Standards

No external engineering code or protected standard is a governing source for this deliverable. The governing standards for this draft are project-internal governance and architecture basis documents:

| Source | Applicable content |
|---|---|
| `docs/CONTRACT.md` | Binding invariants for protected content, user-supplied data, status authority, solver/rule separation, and agent boundaries. |
| `docs/TYPES.md` | Analysis-status vocabulary, epistemic labels, domain terms, and lifecycle states. |
| `docs/DIRECTIVE.md` | Product boundaries, no silent defaults, human authority, and stop rules. |
| `docs/SPEC.md` | Layered architecture, rule-pack evaluator requirements, warning classes, report/audit boundaries, and acceptance semantics. |
| `docs/_Decomposition/SOFTWARE_DECOMP.md` | Accepted revision 0.4 decomposition, `SOW-002`, `DEL-02-03`, objectives, and SCA-001 architecture basis. |
| `docs/PRD.md` | Open mechanics/private code data, no silent engineering defaults, rule-pack requirements, private data handling, and disclaimer requirements. |
| `docs/IP_AND_DATA_BOUNDARY.md` | Public/private data boundary, provenance fields, quarantine rule, and report boundary. |

## Verification

| Verification ID | Requirement(s) | Check |
|---|---|---|
| V01 | R01, R02 | Confirm the status vocabulary matches `docs/TYPES.md` section 4 and excludes automatic `CODE_COMPLIANT`. |
| V02 | R03, R04, R06 | Confirm the mechanics, rule-pack, and human acceptance interfaces are documented separately. |
| V03 | R05, R08 | Confirm missing values are represented as explicit findings/statuses/diagnostics with provenance hooks. |
| V04 | R07 | Confirm human acceptance is modeled as a human record tied to specific evidence, not a solver output. |
| V05 | R09, R10 | Confirm future schema/API work references JSON Schema 2020-12 and no-bypass adapter constraints without selecting unresolved implementation details. |
| V06 | R11 | Confirm all unresolved implementation-level choices remain `TBD`. |
| V07 | All | Confirm no protected standards text, tables, proprietary data, or code-compliance/certification claims appear in the artifact. |
| V08 | R02, R07 | Confirm `HUMAN_APPROVED_FOR_PROJECT` is presented only as a project-specific human record and never as software approval, certification, or code compliance. |
| V09 | R01, R05, R08 | Confirm each status has the required actor/authority level, evidence reference, and diagnostic or rule-pack hook from the producer authority and evidence map. |
| V10 | R08 | Confirm minimum provenance contents identify status evidence, rule-pack source/version/checksum where applicable, diagnostic provenance where present, and hash/input-manifest binding when available. |
| V11 | R10 | Confirm every adapter, plugin, API surface, or schema surface that can observe or mutate analysis status routes through validation, diagnostics/provenance, and public/private data-boundary controls; no unvalidated status write path is permitted. |

Per-status verification coverage:

| Status | Acceptance check |
|---|---|
| `MODEL_INCOMPLETE` | Missing physical data is represented by a solve-blocking diagnostic, not a silent default. |
| `MECHANICS_SOLVED` | Result presentation says mechanics only and does not imply a rule-pack pass/fail or human acceptance. |
| `RULE_INPUTS_INCOMPLETE` | Selected rule-pack identity is present and missing rule-pack inputs are explicit. |
| `USER_RULE_CHECKED` | Rule-pack evaluation evidence is present and language avoids external code-compliance claims. |
| `USER_RULE_FAILED` | Failed-check evidence is present and remains a user-rule result, not professional authentication. |
| `HUMAN_REVIEW_REQUIRED` | Presentation treats competent human review as required before professional reliance. |
| `HUMAN_APPROVED_FOR_PROJECT` | A human acceptance record is referenced, bound to reviewed evidence, and clearly outside solver/rule-pack authority. |

## Documentation

Required documentation outputs for this deliverable tranche:

- `Datasheet.md` describing the boundary model attributes and construction.
- `Specification.md` defining requirements and verification hooks.
- `Guidance.md` explaining conservative use and trade-offs.
- `Procedure.md` giving operational steps for producing/updating the model.
- Future integration target: a `docs/SPEC.md` state-model section. This run does not edit `docs/SPEC.md` because the sealed write scope is deliverable-local.
- Future schema target: `analysis_status` enum/schema location TBD.
