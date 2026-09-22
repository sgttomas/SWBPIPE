# Dependency mirror — DEL-08-04

**ACCEPTED DEPENDENCY MIRROR.** SCA-011 Group 3 audited the poststate and recorded DAG-011 adoption. Current graph authority is DAG-011; DAG-010 is the preserved predecessor. All added execution relations remain PENDING with actual maturity TBD; adoption does not promote source lifecycle, contract readiness or dependency satisfaction.

## Declared upstream/downstream lists

No new human-declared list is created. Existing declared lists, if any, are preserved in the quoted historical index below. Added relations are EXTRACTED from the ownership amendment and adopted into DAG-011 through the recorded Group 3 propagation. Origin=EXTRACTED records provenance and is not a human declaration.

## Extracted Dependency Register

23 rows: {'ACTIVE': 20, 'RETIRED': 3}. Classes: {'ANCHOR': 3, 'EXECUTION': 20}. Preserved local rows: 22; additive rows: 1; changed/retired rows: zero.

| Added edge | Type | Target | Satisfaction |
|---|---|---|---|
| DEP-08-04-001 | EXECUTION/INTERFACE | SCA011-IF-COMPARISON-EXPORT | PENDING |

## Run Notes

Historical preparation method: ad hoc project graph preparation with candidate-only local previews; dependency-extract was consulted for canonical schema/provenance rules, not executed outside its deliverable-local scope. Mode UPDATE-equivalent additive preview; strictness CONSERVATIVE; source documents explicitly limited to P-OWN-01–06 and its dependency table plus the preserved prior mirrors. The preparation used the SCA-011 candidate decomposition and preserved its earlier accepted basis. That preparation step did not change source documents, live local mirrors or the approved DAG. The current application/adoption state is stated at the top of this index; extraction-time candidate wording in CSV Notes is retained provenance, not a later authority claim.

New execution rows use RequiredMaturity=SEMANTIC_READY, ProposedMaturity=TBD, SatisfactionStatus=PENDING. They require actual named contract/source-owner evidence before a production dependency may be called satisfied. Existing satisfaction fields are preserved historical evidence and were not revalidated by this preparation. DOCUMENT stage bindings use repository-root-anchored evidence paths: `projects/chirality-piping/execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP/dependencies/STAGED_INTERFACE_RATIONALE.md` and `projects/chirality-piping/execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP/dependencies/StageGraph.json`. These paths remain valid from the live deliverable landing; they are not relative to this preview directory.

## Run History

- 2026-09-22: SCA-011 candidate-only additive preview; 1 added rows. Earlier local history is quoted below without rewriting its historical claims.
- SCA-011 Group 2: applied these exact additive rows under the recorded application authorization.
- SCA-011 Group 3: audited poststate accepted and DAG-011 adopted through the recorded authority propagation; execution satisfaction remains pending.

## Lifecycle Summary

Local row status: {'ACTIVE': 20, 'RETIRED': 3}. Local satisfaction counts: {'SATISFIED': 10, 'TBD': 12, 'PENDING': 1}. Exactly one active parent anchor is required and checked. These are dependency row states, not deliverable lifecycle changes.

## Downstream Handoff Notes

Group 3 audited this local mirror against the recorded applied poststate and adopted DAG-011 as current graph authority. The accepted DAG-010 predecessor remains immutable. All added execution relations remain PENDING with actual maturity TBD; baseline satisfaction evidence was preserved, not revalidated. Future satisfaction changes require actual source-owner and consumer evidence; no hold, lifecycle, engineering or release acceptance follows from graph adoption.

## Quoted historical index (not current candidate counts or graph pointer)

The following source bytes are retained as historical context; all counts and current-pointer statements inside this fence belong to their earlier record.

```markdown
# Dependencies: DEL-08-04 Result export format

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
- **Canonicalized:** 2026-06-16
- **Rows:** 22 total; 20 ACTIVE; 2 RETIRED.
- **Classes:** ANCHOR=3; EXECUTION=19.
- **Candidate rows moved to worklist:** 0.

## Canonical Dependency Types
- `CONSTRAINT`: 7
- `ENABLES`: 4
- `HANDOVER`: 1
- `INTERFACE`: 5
- `OTHER`: 3
- `PREREQUISITE`: 2

## Run Notes
- Core enum fields conform to the canonical Chirality dependency model.
- Legacy project-specific labels are preserved in `Notes` as `legacy_*` fields.
- Candidate rows remain non-gating in the candidate worklist and require explicit human approval plus graph revalidation before promotion.
- PKG-00 architecture-basis rows reviewed: 7; changed: 0.
- Anchor check: PASS; exactly one ACTIVE `IMPLEMENTS_NODE` row is present.
- Decomposition path used: `execution/_Decomposition/SOFTWARE_DECOMP.md`.
- Source documents used: `_CONTEXT.md`, `Datasheet.md`, `Specification.md`, `Procedure.md`, `Guidance.md`, and current `Dependencies.csv`.
- Warnings: none.
- This dependency refresh does not authorize implementation, lifecycle promotion, release claims, professional approval, certification, sealing, authentication, or code-compliance claims.

| Class | ACTIVE | RETIRED |
|---|---:|---:|
| ANCHOR | 3 | 0 |
| EXECUTION | 17 | 2 |

## Run History
- 2026-06-16 dependency semantic refresh: MODE=UPDATE; STRICTNESS=CONSERVATIVE; CONSUMER_CONTEXT=RECONCILIATION; ARCHITECTURE_BASIS_POLICY=PKG00_CONSISTENCY_TRACKERS; rows added 0, retired 0, changed 0; validation PASS.

## Lifecycle Summary
- ACTIVE rows: 20.
- RETIRED rows: 2.
- Satisfaction status: SATISFIED=10; TBD=12.
- Closure state: dependency register valid for reconciliation handoff; no lifecycle issuance or implementation acceptance implied.

## Downstream Handoff Notes
- Downstream `ENABLES` rows remain reconciliation context, not implementation authorization.
- PKG-00 rows remain architecture-consistency trackers only; they are not substitutes for decomposition truth.

```

## Group-2 metadata correction

Root SPEC §6.8 canonical DEP-owner identities and §6.5 verbatim local evidence have been applied to the added rows. Original SCA011 identities and proposal citations remain in CSV Notes and the ID crosswalk. The accepted ownership relations, prior rows and pending maturity are unchanged. Correction authority and exact field deltas: `projects/chirality-piping/execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP/application/dependencies/CORRECTNESS_REPAIR.md`. At correction time, graph adoption remained subject to Group 3.
