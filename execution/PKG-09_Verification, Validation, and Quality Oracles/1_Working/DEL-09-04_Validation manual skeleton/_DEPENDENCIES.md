# Dependencies: DEL-09-04 Validation manual skeleton

## Extracted Dependency Register

- **Status:** REFRESHED_TP_DAG_004
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer Context:** RECONCILIATION
- **Source of Truth:** deliverable-local extraction from assigned source documents plus `execution/_Decomposition/SOFTWARE_DECOMP.md`
- **Local Register:** `Dependencies.csv`
- **Rows:** 12 total; 12 ACTIVE; 0 RETIRED.
- **Generated:** 2026-05-11

| Class | Direction | Type | Target | Count |
|---|---|---|---|---:|
| ANCHOR | UPSTREAM | OTHER | WBS_NODE / REQUIREMENT | 3 |
| EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | 5 |
| EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | 3 |
| EXECUTION | DOWNSTREAM | ENABLES | DELIVERABLE | 1 |

## Active Dependency Rows

| DependencyID | Class | Direction | Type | Target | Satisfaction | Confidence |
|---|---|---|---|---|---|---|
| DEL-09-04-A001 | ANCHOR | UPSTREAM | OTHER | SOW-027 | NOT_APPLICABLE | HIGH |
| DEL-09-04-A002 | ANCHOR | UPSTREAM | OTHER | OBJ-008 | NOT_APPLICABLE | HIGH |
| DEL-09-04-A003 | ANCHOR | UPSTREAM | OTHER | OBJ-011 | NOT_APPLICABLE | HIGH |
| DAG-002-E0286 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-01 / AB-00-01 | SATISFIED | HIGH |
| DAG-002-E0287 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-02 / AB-00-02 | SATISFIED | HIGH |
| DAG-002-E0288 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-06 / AB-00-06 | SATISFIED | HIGH |
| DAG-002-E0289 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-08 / AB-00-08 | SATISFIED | HIGH |
| DAG-002-E0543 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-09-01 | TBD | MEDIUM |
| DAG-002-E0544 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-09-02 | TBD | MEDIUM |
| DAG-002-E0545 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-09-03 | TBD | MEDIUM |
| DAG-002-E0546 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-01-04 | TBD | HIGH |
| DEL-09-04-E001 | EXECUTION | DOWNSTREAM | ENABLES | DEL-09-05 | TBD | HIGH |

## Run Notes

- Defaults used: `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`.
- Read scope used: assigned DEL-09-04 deliverable documents, `AGENTS.md`, `docs/CONTRACT.md`, `skills/dependency-extract/SKILL.md`, and `execution/_Decomposition/SOFTWARE_DECOMP.md`.
- The prior local register mirrored DAG-002 rows with project-specific values such as `ARCHITECTURE_BASIS`, `VALIDATION_PREDECESSOR`, `GOVERNANCE_PREDECESSOR`, `INFERRED_DIRECT`, `CONTEXT`, `DECOMPOSITION`, and `UNKNOWN`; these failed the current enum validator.
- This refresh normalized those rows into accepted enums while preserving original classification context in `Notes`.
- `tools/validation/validate_id_format.sh` still expects three-digit package/deliverable IDs and rejects current decomposition IDs such as `PKG-09` and `DEL-09-04`; canonical decomposition IDs were preserved.
- No source, status, memory, code, schema, test, DAG, or coordination artifact was edited.

## Lifecycle Summary

- ACTIVE rows: 12
- RETIRED rows: 0
- Anchor rows: 3
- Execution rows: 9
- Satisfaction: 4 SATISFIED; 5 TBD; 3 NOT_APPLICABLE
- Confidence: 9 HIGH; 3 MEDIUM
- Origin: 12 EXTRACTED; 0 DECLARED

## Downstream Handoff Notes

- RECONCILIATION should treat the three validation benchmark prerequisites (`DEL-09-01`, `DEL-09-02`, `DEL-09-03`) as retained but conservative: evidence is implied by manual section requirements and resolved through decomposition, not by explicit deliverable IDs in the source prose.
- `DEL-09-05` was added as a downstream enabled deliverable because `VAL-REQ-008` explicitly constrains release-gate language and decomposition resolves `DEL-09-05` as the release quality gate checklist.
- Architecture-basis rows remain local evidence only; they do not change PKG-00 issuance status or DAG authority.
- Professional-boundary dependency on `DEL-01-04` remains active because DEL-09-04 source text explicitly prohibits certification, approval, sealing, authentication, and code-compliance claims.

## Run History

- 2026-05-11: TP-DAG-004 dependency-extract refresh; mode `UPDATE`; strictness `CONSERVATIVE`; decomposition path `execution/_Decomposition/SOFTWARE_DECOMP.md`; warnings: enum normalization required for prior DAG mirror rows; legacy ID-format helper rejects current two-digit decomposition IDs; active rows 12; retired rows 0.
