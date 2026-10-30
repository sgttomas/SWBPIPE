# Candidate dependency mirror — DEL-07-03

**PROPOSED; NOT ADOPTED.** Prepared under SCA-011 group 2. Live graph authority remains DAG-010. This file describes only the adjacent candidate CSV; no source lifecycle or contract readiness is promoted.

## Declared upstream/downstream lists

No new human-declared list is created. Existing declared lists, if any, are preserved in the quoted historical index below. Candidate relations are EXTRACTED from the ownership proposal, and are not declared owner-approved dependency rows.

## Extracted Dependency Register

20 rows: {'ACTIVE': 20}. Classes: {'ANCHOR': 3, 'EXECUTION': 17}. Preserved local rows: 17; additive rows: 3; changed/retired rows: zero.

| Candidate edge | Type | Target | Satisfaction |
|---|---|---|---|
| SCA011-E076 | EXECUTION/PREREQUISITE | DEL-05-01 | PENDING |
| SCA011-E077 | EXECUTION/PREREQUISITE | DEL-05-02 | PENDING |
| SCA011-E078 | EXECUTION/PREREQUISITE | DEL-16-06 | PENDING |

## Run Notes

Historical preparation method: ad hoc project graph preparation with candidate-only local previews; dependency-extract was consulted for canonical schema/provenance rules, not executed outside its deliverable-local scope. Mode UPDATE-equivalent additive preview; strictness CONSERVATIVE; source documents explicitly limited to P-OWN-01–06 and its dependency table plus the preserved prior mirrors. The preparation used the SCA-011 candidate decomposition and preserved its earlier accepted basis. That preparation step did not change source documents, live local mirrors or the approved DAG. The current application/adoption state is stated at the top of this index; extraction-time candidate wording in CSV Notes is retained provenance, not a later authority claim.

New execution rows use RequiredMaturity=SEMANTIC_READY, ProposedMaturity=TBD, SatisfactionStatus=PENDING. They require actual named contract/source-owner evidence before a production dependency may be called satisfied. Existing satisfaction fields are preserved historical evidence and were not revalidated by this preparation. DOCUMENT stage bindings use repository-root-anchored evidence paths: `projects/chirality-piping/execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP/dependencies/STAGED_INTERFACE_RATIONALE.md` and `projects/chirality-piping/execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP/dependencies/StageGraph.json`. These paths remain valid from the live deliverable landing; they are not relative to this preview directory.

## Run History

- 2026-09-22: SCA-011 candidate-only additive preview; 3 added rows. Earlier local history is quoted below without rewriting its historical claims.

## Lifecycle Summary

Candidate row status: {'ACTIVE': 20}. Candidate satisfaction counts: {'NOT_APPLICABLE': 3, 'SATISFIED': 13, 'PENDING': 4}. Exactly one active parent anchor is required and checked. These are dependency row states, not deliverable lifecycle changes.

## Downstream Handoff Notes

Revalidate this preview against the exact candidate source documents and current local baseline under a bounded owning TASK before application. No new source-witness/readiness conclusion follows from this file. Graph authority adoption and pointer changes remain separate. The aggregate preserves DAG-010; it does not copy unrelated local drift back into authority.

## Quoted historical index (not current candidate counts or graph pointer)

The following source bytes are retained as historical context; all counts and current-pointer statements inside this fence belong to their earlier record.

```markdown
# Dependencies: DEL-07-03 Material, component, and rule-pack editors

## Coordination Mode
- **Mode:** FULL_GRAPH
- **Graph Authority:** `execution/_DAG/DAG-007/` is the current approved canonical graph authority.
- **Authority Boundary:** Candidate/non-gating edges are not represented through `Status=CANDIDATE` in current canonical registers.

## Declared Upstream Dependencies
- None recorded.

## Declared Downstream Dependencies
- None recorded.

## Extracted Dependency Register
- **Local Register:** `Dependencies.csv`
- **Register schema version:** `v3.1`
- **Semantic refreshed:** 2026-06-16
- **Rows:** 17 total; 17 ACTIVE; 0 RETIRED.
- **Classes:** ANCHOR=3; EXECUTION=14.
- **Parent anchors:** 1 ACTIVE `IMPLEMENTS_NODE` row(s).
- **Rows added this run:** 2
- **Rows retired this run:** 0
- **Rows changed this run:** 1
- **PKG-00 rows reviewed/changed:** 7/0

| DependencyID | Class | Direction | Type | TargetType | Target | Status |
|---|---|---|---|---|---|---|
| DEP-DEL-07-03-001 | ANCHOR | UPSTREAM | OTHER | WBS_NODE | PKG-07 | ACTIVE |
| DEP-DEL-07-03-002 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | SOW-021 | ACTIVE |
| DEP-DEL-07-03-003 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | OBJ-006 | ACTIVE |
| DAG-002-E0204 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-01 | ACTIVE |
| DAG-002-E0205 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-02 | ACTIVE |
| DAG-002-E0206 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-03 | ACTIVE |
| DAG-002-E0207 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-05 | ACTIVE |
| DAG-002-E0208 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-06 | ACTIVE |
| DAG-002-E0209 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-07 | ACTIVE |
| DAG-002-E0210 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-08 | ACTIVE |
| DAG-002-E0489 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-03-01 | ACTIVE |
| DAG-002-E0490 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-03-02 | ACTIVE |
| DAG-002-E0491 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-03-07 | ACTIVE |
| DAG-002-E0492 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-06-01 | ACTIVE |
| DAG-002-E0493 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-06-04 | ACTIVE |
| DAG-002-E0494 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-12-01 | ACTIVE |
| DEV-001-STAGE2-DEL-07-03-PKG02-001 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-02-02 | ACTIVE |

## Canonical Dependency Types
- `OTHER`: 3
- `PREREQUISITE`: 14

## Run Notes
- Mode `UPDATE`; strictness `CONSERVATIVE`; consumer context `RECONCILIATION`; architecture-basis policy `PKG00_CONSISTENCY_TRACKERS`.
- Decomposition path used: `execution/_Decomposition/SOFTWARE_DECOMP.md`.
- Anchor document: `_CONTEXT.md`; execution documents reviewed as needed: `Datasheet.md`, `Specification.md`, `Procedure.md`, `Guidance.md`, existing `Dependencies.csv`, and `_DEPENDENCIES.md`.
- PKG-00 architecture-basis rows were reviewed against `execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-01`, `DEL-00-02`, `DEL-00-03`, `DEL-00-05`, `DEL-00-06`, `DEL-00-07`, and `DEL-00-08` source documents; supported rows were retained.
- Core enum fields conform to the canonical Chirality dependency model.
- Legacy project-specific labels are preserved in `Notes` as `legacy_*` fields where previously present.
- Candidate rows remain non-gating in the candidate worklist and require explicit human approval plus graph revalidation before promotion.
- This dependency refresh does not authorize implementation, lifecycle promotion, release claims, professional approval, certification, sealing, authentication, or code-compliance claims.

## Warnings
- None.

## Run History
- 2026-06-16: dependency semantic refresh; mode `UPDATE`; strictness `CONSERVATIVE`; decomposition `execution/_Decomposition/SOFTWARE_DECOMP.md`; rows 17 total, 17 ACTIVE, 0 RETIRED; warnings 0.

- 2026-08-02: R23 dependency-currency patch; five execution rows closed by independent target-maturity plus consumer-integration evidence; DAG-002-E0491 held unchanged.

## Lifecycle Summary
- ACTIVE rows: 17
- RETIRED rows: 0
- Satisfaction status counts: {'NOT_APPLICABLE': 3, 'PENDING': 1, 'SATISFIED': 13}

## Downstream Handoff Notes
- R23 is a deliverable-local satisfaction refresh only; DAG-008 remains graph authority and this patch does not update execution/_DAG/_LATEST.md.
- Consume this register as a deliverable-local semantic refresh shard only. It is not graph authority and does not update `_DAG/_LATEST.md`.

```
