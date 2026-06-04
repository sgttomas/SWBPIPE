# Dependencies: DEL-16-04 Agent rationale and professional-boundary controls

## Generated Dependency Register
- **Status:** REFRESHED_TP_DAG_004
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer Context:** RECONCILIATION
- **Approved Graph Authority:** `execution/_DAG/DAG-006/`
- **Local Register:** `Dependencies.csv`
- **Rows:** 13 total; 13 ACTIVE; 0 RETIRED.

## Extracted Dependency Register

| Count | DependencyClass | AnchorType | Direction | DependencyType | TargetType | Status |
|---:|---|---|---|---|---|---|
| 7 | EXECUTION | DELIVERABLE | UPSTREAM | ARCHITECTURE_BASIS | DELIVERABLE | ACTIVE |
| 2 | EXECUTION | DELIVERABLE | UPSTREAM | GOVERNANCE_PREDECESSOR | DELIVERABLE | ACTIVE |
| 1 | EXECUTION | DELIVERABLE | UPSTREAM | SECURITY_PREDECESSOR | DELIVERABLE | ACTIVE |
| 1 | ANCHOR | IMPLEMENTS_NODE | UPSTREAM | OTHER | WBS_NODE | ACTIVE |
| 2 | ANCHOR | TRACES_TO_REQUIREMENT | UPSTREAM | OTHER | REQUIREMENT | ACTIVE |

## Authority Boundary
- Aggregate `APPROVED_DAG002` remains the sequencing and blocker-computation authority within its approval boundary.
- `DAG-003` is not approved or promoted by this refresh.
- This local register is a refreshed deliverable evidence surface for later reconciliation, not an independent graph authority.
- `CANDIDATE` rows remain non-gating until later RECONCILIATION plus CHANGE approval; none were introduced here.
- `PKG-00` architecture-basis rows are preserved as injected context evidence; `PKG-00` does not receive local dependency registers.

## Run Notes
- Defaults used: `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`.
- Anchor source selected: `_CONTEXT.md`, cross-checked against `execution/_Decomposition/SOFTWARE_DECOMP.md`.
- Execution sources reviewed: `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_REFERENCES.md`, existing `Dependencies.csv`, and approved `execution/_DAG/DAG-002` edges for `DEL-16-04`.
- Preserved all 10 approved DAG-002 mirror execution rows and refreshed `LastSeen` to `2026-05-11`.
- Added one active parent anchor to `SOW-070` and two active trace anchors to `OBJ-015` and `OBJ-018` from explicit local context/decomposition evidence.
- No downstream active execution edges were added.
- No candidate rows were promoted; no active cycles were introduced.
- [WARNING] ENUM_TOOL_MISMATCH: `tools/validation/validate_enum.py` does not include approved DAG-002 local mirror values such as `ARCHITECTURE_BASIS`, `GOVERNANCE_PREDECESSOR`, `SECURITY_PREDECESSOR`, `CONTEXT`, `GRAPH_REVIEW`, or `INFERRED_DIRECT`; enum validation is therefore degraded for preserved mirror rows.
- [WARNING] ID_FORMAT_TOOL_MISMATCH: `tools/validation/validate_id_format.sh` expects legacy three-digit package/deliverable and four-digit SOW formats, while this project uses IDs such as `PKG-16`, `DEL-16-04`, and `SOW-070`.

## Run History
- 2026-05-03: Synchronized from approved DAG-006 mirror; 10 ACTIVE rows.
- 2026-05-11: TP-DAG-004 dependency surface refresh; mode `UPDATE`; strictness `CONSERVATIVE`; decomposition `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.5; approved graph authority `execution/_DAG/DAG-006`; 13 ACTIVE rows; warnings: enum/id-format validator mismatch with current project IDs and DAG-002 mirror taxonomy.

## Lifecycle Summary
- ACTIVE: 13
- RETIRED: 0
- CANDIDATE: 0
- SatisfactionStatus: `SATISFIED` 7; `UNKNOWN` 3; `NOT_APPLICABLE` 3
- RequiredMaturity: `SEMANTIC_READY` 13
- ProposedMaturity: `SEMANTIC_READY` 13

## Downstream Handoff Notes
- For RECONCILIATION: treat the 10 execution rows as preserved approved DAG-002 mirror evidence and the 3 anchor rows as newly extracted local definition traces.
- Validator warnings should be handled as tool/schema drift, not as evidence that preserved DAG-002 rows are invalid sequencing authority.
