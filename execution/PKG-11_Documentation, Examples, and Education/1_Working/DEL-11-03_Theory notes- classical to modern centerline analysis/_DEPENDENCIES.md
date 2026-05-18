# Dependencies: DEL-11-03 Theory notes: classical to modern centerline analysis

## Declared Dependency Notes

- No human-declared dependency list was present in the prior container.
- Prior `SYNCHRONIZED_FROM_DAG_002` rows are preserved in `Dependencies.csv` with their original `DAG-002-*` IDs where matchable.
- Prior DAG-specific enum values were normalized to v3.1 dependency-extract enums for local validation. Original DAG vocabulary is retained in `Notes`.

## Extracted Dependency Register

- **Status:** REFRESHED_TP_DAG_004
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer Context:** RECONCILIATION
- **Source of Truth:** deliverable-local extraction from assigned source documents plus `execution/_Decomposition/SOFTWARE_DECOMP.md`; approved graph cross-check from `execution/_DAG/DAG-002`
- **Local Register:** `Dependencies.csv`
- **Rows:** 12 total; 12 ACTIVE; 0 RETIRED.
- **Generated:** 2026-05-10

| Class | Direction | Type | Target | Count |
|---|---|---|---|---:|
| ANCHOR | UPSTREAM | OTHER | WBS_NODE / REQUIREMENT | 3 |
| EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | 6 |
| EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | 3 |

## Active Dependency Rows

| DependencyID | Class | Direction | Type | Target | Satisfaction | Confidence |
|---|---|---|---|---|---|---|
| DEL-11-03-A001 | ANCHOR | UPSTREAM | OTHER | SOW-033 | NOT_APPLICABLE | HIGH |
| DEL-11-03-A002 | ANCHOR | UPSTREAM | OTHER | OBJ-001 | NOT_APPLICABLE | HIGH |
| DEL-11-03-A003 | ANCHOR | UPSTREAM | OTHER | OBJ-003 | NOT_APPLICABLE | HIGH |
| DAG-002-E0339 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-01 / AB-00-01 | SATISFIED | HIGH |
| DAG-002-E0340 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-02 / AB-00-02 | SATISFIED | HIGH |
| DAG-002-E0341 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-06 / AB-00-06 | SATISFIED | HIGH |
| DAG-002-E0342 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-07 / AB-00-07 | SATISFIED | HIGH |
| DAG-002-E0343 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-08 / AB-00-08 | SATISFIED | HIGH |
| DAG-002-E0586 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-04-01 | TBD | MEDIUM |
| DAG-002-E0587 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-04-02 | TBD | MEDIUM |
| DAG-002-E0588 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-09-01 | TBD | MEDIUM |
| DAG-002-E0589 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-01-02 | TBD | HIGH |

## Run Notes

- Defaults recorded: `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`, `MODE=UPDATE`, `STRICTNESS=CONSERVATIVE`, `CONSUMER_CONTEXT=RECONCILIATION`.
- Chosen anchor evidence: `Datasheet.md`, with decomposition validation from `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`.
- Chosen execution evidence order: `Specification.md`, `Guidance.md`, `Datasheet.md`, then contextual cross-checks in `_CONTEXT.md` and `_REFERENCES.md`.
- Approved graph authority used for preservation/cross-check only: `execution/_DAG/DAG-002`. `DAG-003` was not approved, promoted, or used as authority.
- ACTIVE rows use only v3.1 canonical dependency enums.
- Prior DAG-002 synchronized rows used non-v3.1 values in enum-controlled fields: `AnchorType=DELIVERABLE`, `DependencyType=ARCHITECTURE_BASIS/DOCS_PREDECESSOR/GOVERNANCE_PREDECESSOR`, `Explicitness=INFERRED_DIRECT`, `SatisfactionStatus=UNKNOWN`, and `Origin=CONTEXT/DECOMPOSITION`. These values were normalized while preserving the source context in `Notes`.
- No `[WARNING] FLOATING_NODE`: one ACTIVE `IMPLEMENTS_NODE` anchor is present.
- No `[WARNING] AMBIGUOUS_ANCHOR`: exactly one ACTIVE `IMPLEMENTS_NODE` anchor is present.
- No `[WARNING] MISSING_DECOMPOSITION`: decomposition path was available and read.
- Validation warning: `tools/validation/validate_id_format.sh` expects legacy three-digit package/deliverable and four-digit SOW patterns and rejects canonical decomposition IDs used in this project revision such as `PKG-11`, `DEL-11-03`, and `SOW-033`. IDs were preserved from the decomposition.
- No source documents, status files, memory files, code, schemas, tests, aggregate DAG artifacts, coordination files, lifecycle state, or package registers were edited.

## Run History

| Timestamp | Mode | Strictness | Consumer | Decomposition | Rows | Warnings |
|---|---|---|---|---|---:|---|
| 2026-04-30 1155 | UPDATE | CONSERVATIVE | NONE | available | 9 ACTIVE | prior dependency setup |
| 2026-05-03 | DAG-002 sync | N/A | coordination | DAG-002 | 9 ACTIVE | non-v3.1 enum vocabulary later detected |
| 2026-05-10 2322 | UPDATE | CONSERVATIVE | RECONCILIATION | available | 12 ACTIVE; 0 RETIRED | enum normalization required for prior DAG mirror rows; legacy ID-format helper pattern mismatch |

## Lifecycle Summary

- ACTIVE rows: 12
- RETIRED rows: 0
- Anchor rows: 3
- Execution rows: 9
- Satisfaction: 5 SATISFIED; 4 TBD; 3 NOT_APPLICABLE
- Confidence: 9 HIGH; 3 MEDIUM
- Origin: 12 EXTRACTED; 0 DECLARED

## Downstream Handoff Notes

- For RECONCILIATION, consume ACTIVE rows as the current local dependency-extract surface for `DEL-11-03`.
- The architecture-basis rows remain local evidence only; they do not change PKG-00 issuance status or aggregate DAG authority.
- `DEL-04-01`, `DEL-04-02`, and `DEL-09-01` remain active but conservative prerequisites retained from approved DAG-002. Local source text supports frame, line-element, and mechanics-results concepts, while exact deliverable targeting is still decomposition/DAG-resolved.
- `DEL-01-02` remains a high-confidence constraint because local requirements explicitly require public/permissive sources and prohibit protected standards text, examples, figures, tables, code-specific formulas, and proprietary commercial data.
- Public/permissive mechanics source selection, exact citation sections, detailed historical claims, numerical examples, and formula-level content remain `TBD` for future sealed documentation work.
