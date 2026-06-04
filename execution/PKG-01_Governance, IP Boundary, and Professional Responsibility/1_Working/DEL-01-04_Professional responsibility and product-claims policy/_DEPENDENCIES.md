# Dependencies: DEL-01-04 Professional responsibility and product-claims policy

## Generated Dependency Register

- **Status:** CURRENT_BASIS_REFRESHED_BY_WORKING_ITEMS
- **Source of Truth:** Deliverable-local evidence refresh; aggregate DAG artifacts remain outside this refresh authority.
- **Previous Mirror Source:** `execution/_DAG/DAG-002/DependencyEdges.csv`
- **Approved Graph Context:** `execution/_DAG/DAG-006/`
- **Local Register:** `Dependencies.csv`
- **Rows:** 17 total; 16 ACTIVE; 1 RETIRED.
- **Generated:** 2026-05-10; refreshed 2026-06-04 by `TP-DEL-01-04-CURRENT-BASIS-REFRESH-001`

## Authority Boundary

- Aggregate `DAG-006` remains the sequencing and blocker-computation authority within its approval boundary.
- This local register is a deliverable-local evidence surface, not an independent project graph authority.
- Candidate, proposal, or uncertain rows remain non-gating until later RECONCILIATION plus CHANGE approval.
- Existing `DAG-002-*` rows were preserved and normalized to v3.1 enum values; original aggregate meanings are retained in row notes.
- `PKG-00` architecture-basis rows are preserved here as injected context evidence; `PKG-00` does not receive local dependency registers.

## Extracted Dependency Register

### Counts

| Metric | Count |
|---|---:|
| Total rows | 17 |
| ACTIVE rows | 16 |
| RETIRED rows | 1 |
| ANCHOR rows | 5 |
| EXECUTION rows | 12 |
| EXTRACTED rows | 17 |
| SATISFIED rows | 14 |
| PENDING rows | 2 |
| TBD rows | 1 |

### Compact Table

| DependencyID | Class | Direction | Type | TargetType | TargetRefID | TargetName | Status | Satisfaction |
|---|---|---|---|---|---|---|---|---|
| DEP-DEL-01-04-A001 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | SOW-034 | Professional responsibility boundaries | ACTIVE | SATISFIED |
| DEP-DEL-01-04-A002 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | OBJ-011 | Preserve professional responsibility | ACTIVE | SATISFIED |
| DEP-DEL-01-04-A003 | ANCHOR | UPSTREAM | OTHER | PACKAGE | PKG-01 | Governance, IP Boundary, and Professional Responsibility | ACTIVE | SATISFIED |
| DEP-DEL-01-04-A004 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | SOW-064 | Design-engine non-authoritative reliance boundary | ACTIVE | SATISFIED |
| DEP-DEL-01-04-A005 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | OBJ-018 | Preserve professional and IP boundaries across design-engine workflows | ACTIVE | SATISFIED |
| DAG-002-E0013 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-01 | Architecture decision record baseline | ACTIVE | SATISFIED |
| DAG-002-E0014 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-02 | Repository and module boundary architecture | ACTIVE | SATISFIED |
| DAG-002-E0015 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-06 | Diagnostics, warning, and result-envelope contract | ACTIVE | SATISFIED |
| DAG-002-E0016 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-08 | Layered software test and acceptance strategy | ACTIVE | SATISFIED |
| DAG-002-E0392 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-01-01 | Project governance baseline | RETIRED | TBD |
| DEP-DEL-01-04-E001 | EXECUTION | UPSTREAM | PREREQUISITE | DOCUMENT | OPS-CONTRACT | Invariant catalog | ACTIVE | SATISFIED |
| DEP-DEL-01-04-E002 | EXECUTION | UPSTREAM | CONSTRAINT | DOCUMENT | OPS-IP-DATA-BOUNDARY | Protected-content and private-data boundary | ACTIVE | SATISFIED |
| DEP-DEL-01-04-E005 | EXECUTION | UPSTREAM | PREREQUISITE | DOCUMENT | OPS-SOFTWARE-DECOMP | Software decomposition current basis | ACTIVE | SATISFIED |
| DEP-DEL-01-04-E003 | EXECUTION | DOWNSTREAM | HANDOVER | DOCUMENT | docs/PROFESSIONAL_BOUNDARY.md | Professional-boundary policy document | ACTIVE | SATISFIED |
| DEP-DEL-01-04-E004 | EXECUTION | DOWNSTREAM | HANDOVER | DOCUMENT | OPS-REPORT-NOTICE-TEMPLATE | Report notice template | ACTIVE | SATISFIED |
| DEP-DEL-01-04-E006 | EXECUTION | UPSTREAM | PREREQUISITE | EXTERNAL | HUMAN_LEGAL_PROFESSIONAL_REVIEW | Human legal/professional review | ACTIVE | PENDING |
| DEP-DEL-01-04-E007 | EXECUTION | UPSTREAM | PREREQUISITE | EXTERNAL | HUMAN_PROJECT_AUTHORITY | Human project authority acceptance | ACTIVE | PENDING |

## Run Notes

- **TaskSkill:** `dependency-extract`
- **Mode:** `UPDATE`
- **Strictness:** `CONSERVATIVE`
- **Consumer Context:** `RECONCILIATION`
- **Scope:** `DEL-01-04`
- **Scope Path:** `execution/PKG-01_Governance, IP Boundary, and Professional Responsibility/1_Working/DEL-01-04_Professional responsibility and product-claims policy`
- **Run Root:** `/Users/ryan/ai-env/projects/chirality-piping/execution`
- **Decomposition Path:** `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`
- **Decomposition Status:** located; frontmatter revision `0.7`.
- **SOURCE_DOCS:** `AUTO`; scanned `Datasheet.md`, `Specification.md`, `Procedure.md`, `Guidance.md`, `_CONTEXT.md`, `_REFERENCES.md`, and current dependency artifacts.
- **DOC_ROLE_MAP:** `DEFAULT`.
- **ANCHOR_DOC:** `Datasheet.md`.
- **EXECUTION_DOC_ORDER:** `Procedure.md`, `Specification.md`, `Guidance.md`, `_CONTEXT.md`.
- **Defaults Applied:** conservative refresh; no inferred protected standards content; unresolved legal/professional and human-acceptance decisions preserved as `TBD`/`PENDING`; draft repo-level artifacts recorded as present without final acceptance.
- **Warnings:** Historical run records still preserve old `docs/_Decomposition/SOFTWARE_DECOMP.md` and revision `0.4`/`0.5` references as history. Active local source documents now cite `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7` and approved `DAG-006`.
- **Warnings:** The retired historical `DAG-002-E0392` row remains preserved as retired evidence and was not promoted into local or aggregate active authority.
- **Warnings:** No lifecycle edit, aggregate DAG edit, candidate promotion, repo-level policy edit, release claim, professional approval, certification, sealing, authentication, or code-compliance claim was made.
- **Failed Inputs:** none.

## Run History

| Timestamp | Mode | Strictness | Decomposition | Warnings | ACTIVE Counts |
|---|---|---|---|---|---|
| 2026-05-03 | DAG mirror sync | n/a | `execution/_DAG/DAG-002/DependencyEdges.csv` | Aggregate authority boundary applies. | 5 execution; 0 anchor |
| 2026-05-10 21:40 America/Edmonton | UPDATE | CONSERVATIVE | `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.5 | Stale local source reference to decomposition revision 0.4; normalized legacy/non-v3.1 enum values; prior inferred governance-predecessor row retired. | 8 execution; 2 anchor |
| 2026-06-04 | CURRENT_BASIS_REFRESH | CONSERVATIVE | `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 | Active local docs aligned to `DAG-006`; SOW-064/OBJ-018 anchors added; draft repo-level policy/template artifacts recorded as present; no aggregate DAG or lifecycle edit. | 11 execution; 5 anchor |

## Lifecycle Summary

| Lifecycle | Count |
|---|---:|
| ACTIVE | 16 |
| RETIRED | 1 |
| SATISFIED | 14 |
| PENDING | 2 |
| TBD | 1 |
| IN_PROGRESS | 0 |
| WAIVED | 0 |
| NOT_APPLICABLE | 0 |

## Downstream Handoff Notes

- `RECONCILIATION` should treat this file as deliverable-local evidence only. It must not promote any normalized aggregate mirror row to aggregate authority without its own review and CHANGE approval.
- `docs/PROFESSIONAL_BOUNDARY.md` and `docs/report_notice_template.md` exist as draft repo-level governance/report-notice artifacts. This satisfies local handoff evidence but does not issue or accept those artifacts.
- Jurisdiction-specific legal/professional wording, exact acceptance-record storage/invalidation workflow, release-label vocabulary, final release policy language, legal-review authority, and final human acceptance remain unresolved.
- This refresh does not edit aggregate `DAG-006`, lifecycle `_STATUS.md`, repo-level policy artifacts, release records, or professional/legal acceptance records.
