# Dependencies: DEL-01-03 Contributor certification workflow

## Generated Dependency Register
- **Status:** REFRESHED_BY_TP_DAG_004
- **Source of Truth:** Deliverable-local evidence extraction; aggregate DAG artifacts remain outside this refresh authority.
- **Previous Mirror Source:** `execution/_DAG/DAG-002/DependencyEdges.csv`
- **Local Register:** `Dependencies.csv`
- **Rows:** 14 total; 14 ACTIVE; 0 RETIRED.
- **Generated:** 2026-05-10

## Authority Boundary
- Aggregate DAG artifacts remain the sequencing and blocker-computation authority within their approval boundary.
- This local register is a deliverable-local evidence surface, not an independent project graph authority.
- Candidate, proposal, or uncertain rows remain non-gating until later RECONCILIATION plus CHANGE approval.
- Existing `DAG-002-*` rows were preserved and normalized to v3.1 enum values; original aggregate meanings are retained in row notes.
- `PKG-00` architecture-basis rows are preserved here as injected context evidence; `PKG-00` does not receive local dependency registers.

## Extracted Dependency Register

### Counts
| Metric | Count |
|---|---:|
| Total rows | 14 |
| ACTIVE rows | 14 |
| RETIRED rows | 0 |
| ANCHOR rows | 4 |
| EXECUTION rows | 10 |
| DECLARED rows | 6 |
| EXTRACTED rows | 8 |

### Compact Table
| DependencyID | Class | Direction | Type | TargetType | TargetRefID | TargetName | Status | Satisfaction |
|---|---|---|---|---|---|---|---|---|
| DAG-002-E0009 | EXECUTION | UPSTREAM | OTHER | DELIVERABLE | DEL-00-01 | Architecture decision record baseline | ACTIVE | SATISFIED |
| DAG-002-E0010 | EXECUTION | UPSTREAM | OTHER | DELIVERABLE | DEL-00-02 | Repository and module boundary architecture | ACTIVE | SATISFIED |
| DAG-002-E0011 | EXECUTION | UPSTREAM | OTHER | DELIVERABLE | DEL-00-06 | Diagnostics, warning, and result-envelope contract | ACTIVE | SATISFIED |
| DAG-002-E0012 | EXECUTION | UPSTREAM | OTHER | DELIVERABLE | DEL-00-08 | Layered software test and acceptance strategy | ACTIVE | SATISFIED |
| DAG-002-E0390 | EXECUTION | UPSTREAM | OTHER | DELIVERABLE | DEL-01-01 | Project governance baseline | ACTIVE | TBD |
| DAG-002-E0391 | EXECUTION | UPSTREAM | OTHER | DELIVERABLE | DEL-01-02 | Copyright and protected-data boundary policy | ACTIVE | TBD |
| DEL-01-03-A001 | ANCHOR | UPSTREAM | OTHER | PACKAGE | PKG-01 | Governance, IP Boundary, and Professional Responsibility | ACTIVE | SATISFIED |
| DEL-01-03-A002 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | SOW-028 | Contributor governance and IP controls | ACTIVE | SATISFIED |
| DEL-01-03-A003 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | SOW-048 | Open-source license governance release and maintainer policies | ACTIVE | SATISFIED |
| DEL-01-03-A004 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | OBJ-002 | Protect standards-body and vendor intellectual property | ACTIVE | SATISFIED |
| DEL-01-03-E001 | EXECUTION | UPSTREAM | PREREQUISITE | DOCUMENT | OPS-CONTRACT | CONTRACT invariant catalog | ACTIVE | SATISFIED |
| DEL-01-03-E002 | EXECUTION | UPSTREAM | PREREQUISITE | DOCUMENT | OPS-IP-DATA-BOUNDARY | IP and Data Boundary Policy | ACTIVE | SATISFIED |
| DEL-01-03-E003 | EXECUTION | UPSTREAM | PREREQUISITE | DOCUMENT | OPS-SOFTWARE-DECOMP | Software decomposition current basis | ACTIVE | SATISFIED |
| DEL-01-03-E004 | EXECUTION | DOWNSTREAM | HANDOVER | DOCUMENT | CONTRIBUTING.md | Future CONTRIBUTING.md section | ACTIVE | PENDING |

## Run Notes
- **TaskSkill:** `dependency-extract`
- **Mode:** `UPDATE`
- **Strictness:** `CONSERVATIVE`
- **Consumer Context:** `RECONCILIATION`
- **Scope:** `DEL-01-03`
- **Scope Path:** `execution/PKG-01_Governance, IP Boundary, and Professional Responsibility/1_Working/DEL-01-03_Contributor certification workflow`
- **Run Root:** `/Users/ryan/ai-env/projects/chirality-piping/execution`
- **Decomposition Path:** `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`
- **Decomposition Status:** located; frontmatter revision `0.5`.
- **SOURCE_DOCS:** `AUTO`; scanned `Datasheet.md`, `Specification.md`, `Procedure.md`, `Guidance.md`, `_CONTEXT.md`, `_REFERENCES.md`, and current dependency artifacts.
- **DOC_ROLE_MAP:** `DEFAULT`.
- **ANCHOR_DOC:** `Datasheet.md`.
- **EXECUTION_DOC_ORDER:** `Procedure.md`, `Specification.md`, `Guidance.md`, `_CONTEXT.md`.
- **Defaults Applied:** conservative extraction; no inferred protected standards content; unresolved legal/governance decisions preserved as `TBD`; uncertain future handoff marked `PROPOSAL` in notes and `PENDING` in closure lifecycle.
- **Enum Normalization:** Existing synchronized DAG rows contained values outside the v3.1 enum sets. They were normalized locally: `AnchorType=DELIVERABLE` to `NOT_APPLICABLE`, architecture/governance dependency subtypes to `OTHER`, `INFERRED_DIRECT` to `IMPLICIT`, `UNKNOWN` satisfaction to `TBD`, and non-v3.1 origins to `DECLARED`. Original values remain in row notes.
- **Warnings:** Source documents refer to `docs/_Decomposition/SOFTWARE_DECOMP.md` revision `0.4`, while the assigned decomposition path is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.5`. This refresh used the assigned path and recorded the stale-source condition on `DEL-01-03-E003`.
- **Warnings:** `tools/validation/validate_id_format.sh` rejects project IDs such as `DEL-01-03` because it expects `^DEL-[0-9]{3}-[0-9]{2}$`. Project IDs were instead checked against the assigned decomposition and preserved unchanged.
- **Warnings:** No `[WARNING] FLOATING_NODE`; exactly one ACTIVE `IMPLEMENTS_NODE` anchor is present.
- **Warnings:** No `[WARNING] AMBIGUOUS_ANCHOR`; only one ACTIVE parent anchor is present.
- **Warnings:** No `[WARNING] MISSING_DECOMPOSITION`; the assigned decomposition file was available.
- **Failed Inputs:** none.

## Run History
| Timestamp | Mode | Strictness | Decomposition | Warnings | ACTIVE Counts |
|---|---|---|---|---|---|
| 2026-05-03 | DAG mirror sync | n/a | `execution/_DAG/DAG-002/DependencyEdges.csv` | Aggregate authority boundary applies. | 6 execution; 0 anchor |
| 2026-05-10 21:40 America/Edmonton | UPDATE | CONSERVATIVE | `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.5 | Stale local source reference to decomposition revision 0.4; normalized legacy/non-v3.1 enum values; ID-format helper regex mismatch with project IDs. | 10 execution; 4 anchor |

## Lifecycle Summary
| Lifecycle | Count |
|---|---:|
| ACTIVE | 14 |
| RETIRED | 0 |
| SATISFIED | 11 |
| PENDING | 1 |
| TBD | 2 |
| IN_PROGRESS | 0 |
| WAIVED | 0 |
| NOT_APPLICABLE | 0 |

## Downstream Handoff Notes
- `RECONCILIATION` should treat this file as deliverable-local evidence only. It must not promote `DEL-01-03-E004` or any normalized aggregate mirror row to aggregate authority without its own review and CHANGE approval.
- The two governance predecessor rows from DAG-002 remain `TBD` in satisfaction because local evidence did not prove closure of DEL-01-01 or DEL-01-02.
- The future `CONTRIBUTING.md` handoff is explicit in the local procedure but remains pending human approval and repo-level governance action.
- The stale decomposition revision mention in local source documents may require a later source-document refresh outside this task's write scope.
