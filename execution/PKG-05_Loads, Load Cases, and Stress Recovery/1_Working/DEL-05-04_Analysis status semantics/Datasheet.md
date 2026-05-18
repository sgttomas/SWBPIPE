# Datasheet: DEL-05-04 Analysis status semantics

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-05-04 |
| Name | Analysis status semantics |
| Package | PKG-05 Loads, Load Cases, and Stress Recovery |
| Type | DATA_MODEL_CHANGE |
| Scope items | SOW-047 |
| Objectives | OBJ-005, OBJ-011 |
| Context envelope | S |

## Attributes

| Attribute | Setup value | Source |
|---|---|---|
| Primary subject | Explicit result/status semantics that separate mechanics solve state, user rule-pack check state, missing input state, and human acceptance state. | `_CONTEXT.md`; `docs/_Decomposition/SOFTWARE_DECOMP.md` row DEL-05-04 |
| Mechanics status | `MECHANICS_SOLVED` describes completed numerical mechanics computation for a model snapshot; it is not a code-compliance statement. | `docs/TYPES.md` section 4; `docs/architecture/analysis_status_semantics.md` Usage Rules |
| Incomplete-data statuses | `MODEL_INCOMPLETE` and `RULE_INPUTS_INCOMPLETE` distinguish solve-required physical data gaps from rule-pack-required user/code data gaps. | `docs/TYPES.md` section 4; OPS-K-DATA-2 |
| Rule-pack statuses | `USER_RULE_CHECKED` and `USER_RULE_FAILED` describe user-rule-pack evaluation outcomes using user data and rule definitions. | `docs/TYPES.md` section 4; `docs/architecture/analysis_status_semantics.md` Authority Boundary |
| Human review status | `HUMAN_REVIEW_REQUIRED` is reportable for professional use; `HUMAN_APPROVED_FOR_PROJECT` is human-record-only and not emitted automatically by software. | `docs/TYPES.md` section 4; OPS-K-AUTH-1; OPS-K-AUTH-2 |
| Result-envelope interface | Status fields belong with schema-first command/query/job result envelopes and diagnostics, preserving mechanics/rule/human distinctions. | AB-00-03; AB-00-06 |
| Human acceptance workflow | TBD. External human acceptance records and storage/presentation ownership remain outside this setup deliverable. | `docs/_Decomposition/SOFTWARE_DECOMP.md` OI-007; `docs/architecture/analysis_status_semantics.md` Remaining TBDs |

## Conditions

- Status records describe what software computed or found; they must not claim certification, sealing, approval, authentication, or professional code compliance. Source: OPS-K-AUTH-1 and OPS-K-MECH-2.
- Missing solve-required or rule-check-required values must be explicit findings and never silent defaults. Source: OPS-K-DATA-2.
- Human acceptance records, if used later, must bind to specific model, rule-pack, result, and report hashes and must not survive content changes without re-review. Source: OPS-K-AUTH-2.
- Reports must disclose solver/version, rule-pack checksum, warnings, assumptions, limitations, source/provenance notes, and professional-boundary notices. Source: OPS-K-REPORT-1.
- Setup artifacts are drafts/proposals until accepted by a human gate. Source: OPS-K-AGENT-4.

## Construction

| Artifact | Description | Setup status |
|---|---|---|
| Analysis status enum | Enum vocabulary for model incomplete, mechanics solved, rule inputs incomplete, user rule checked/failed, human review required, and human-approved record references. | Setup evidence only; implementation TBD |
| API/result fields | Result-envelope fields that separate software status from external human acceptance record state. | Setup evidence only; schema location TBD |
| Diagnostics linkage | Diagnostic class/status wiring for solve blocking, rule-check blocking, provenance, assumptions, and reportable limitations. | Setup evidence only |
| Tests | Future tests that prove mechanics/rule/human statuses are distinct and no automatic compliance/approval status is emitted. | Required in principle; fixtures TBD |

## Status Vocabulary Register

| Status | Setup meaning | Authority level |
|---|---|---|
| `MODEL_INCOMPLETE` | Solve-required physical inputs are missing. | Software finding |
| `MECHANICS_SOLVED` | Numerical mechanics solve completed for a specific snapshot. | Solver result only |
| `RULE_INPUTS_INCOMPLETE` | Mechanics may be solved, but rule-pack-required user/code values are missing. | Rule-pack finding |
| `USER_RULE_CHECKED` | User-defined rule pack evaluated the result. | Software computation using user data |
| `USER_RULE_FAILED` | User-defined rule pack produced a failing result. | Software computation using user data |
| `HUMAN_REVIEW_REQUIRED` | Professional use requires competent human review. | Report/professional-boundary notice |
| `HUMAN_APPROVED_FOR_PROJECT` | A human recorded project-specific acceptance outside solver authority. | Human record only; automatic software emission prohibited |

## References

- `_CONTEXT.md`
- `_REFERENCES.md`
- `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4
- `docs/_Registers/Deliverables.csv` row DEL-05-04
- `docs/_Registers/ScopeLedger.csv` row SOW-047
- `docs/_Registers/ContextBudgetQA.csv` row DEL-05-04
- `docs/CONTRACT.md`
- `docs/TYPES.md`
- `docs/SPEC.md`
- `docs/architecture/analysis_status_semantics.md`
