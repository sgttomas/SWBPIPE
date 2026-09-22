# Dependency mirror — DEL-16-06

**ACCEPTED DEPENDENCY MIRROR.** SCA-011 Group 3 audited the poststate and recorded DAG-011 adoption. Current graph authority is DAG-011; DAG-010 is the preserved predecessor. All added execution relations remain PENDING with actual maturity TBD; adoption does not promote source lifecycle, contract readiness or dependency satisfaction.

## Declared upstream/downstream lists

No new human-declared list is created. Existing declared lists, if any, are preserved in the quoted historical index below. Added relations are EXTRACTED from the ownership amendment and adopted into DAG-011 through the recorded Group 3 propagation. Origin=EXTRACTED records provenance and is not a human declaration.

## Extracted Dependency Register

12 rows: {'ACTIVE': 12}. Classes: {'ANCHOR': 4, 'EXECUTION': 8}. Preserved local rows: 0; additive rows: 12; changed/retired rows: zero.

| Added edge | Type | Target | Satisfaction |
|---|---|---|---|
| DEP-16-06-001 | ANCHOR/OTHER | DEL-16-06 | NOT_APPLICABLE |
| DEP-16-06-002 | ANCHOR/OTHER | SOW-069 | NOT_APPLICABLE |
| DEP-16-06-003 | ANCHOR/OTHER | SOW-070 | NOT_APPLICABLE |
| DEP-16-06-004 | ANCHOR/OTHER | OBJ-015 | NOT_APPLICABLE |
| DEP-16-06-005 | EXECUTION/PREREQUISITE | DEL-16-01 | PENDING |
| DEP-16-06-006 | EXECUTION/PREREQUISITE | DEL-16-02 | PENDING |
| DEP-16-06-007 | EXECUTION/PREREQUISITE | DEL-16-03 | PENDING |
| DEP-16-06-008 | EXECUTION/PREREQUISITE | DEL-02-01 | PENDING |
| DEP-16-06-009 | EXECUTION/PREREQUISITE | DEL-02-02 | PENDING |
| DEP-16-06-010 | EXECUTION/PREREQUISITE | DEL-02-05 | PENDING |
| DEP-16-06-011 | EXECUTION/CONSTRAINT | DEL-00-03 | PENDING |
| DEP-16-06-012 | EXECUTION/CONSTRAINT | DEL-00-07 | PENDING |

## Run Notes

Historical preparation method: ad hoc project graph preparation with candidate-only local previews; dependency-extract was consulted for canonical schema/provenance rules, not executed outside its deliverable-local scope. Mode UPDATE-equivalent additive preview; strictness CONSERVATIVE; source documents explicitly limited to P-OWN-01–06 and its dependency table plus the preserved prior mirrors. The preparation used the SCA-011 candidate decomposition and preserved its earlier accepted basis. That preparation step did not change source documents, live local mirrors or the approved DAG. The current application/adoption state is stated at the top of this index; extraction-time candidate wording in CSV Notes is retained provenance, not a later authority claim.

New execution rows use RequiredMaturity=SEMANTIC_READY, ProposedMaturity=TBD, SatisfactionStatus=PENDING. They require actual named contract/source-owner evidence before a production dependency may be called satisfied. Existing satisfaction fields are preserved historical evidence and were not revalidated by this preparation. DOCUMENT stage bindings use repository-root-anchored evidence paths: `projects/chirality-piping/execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP/dependencies/STAGED_INTERFACE_RATIONALE.md` and `projects/chirality-piping/execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP/dependencies/StageGraph.json`. These paths remain valid from the live deliverable landing; they are not relative to this preview directory.

## Run History

- 2026-09-22: SCA-011 candidate-only additive preview; 12 added rows. Earlier local history is quoted below without rewriting its historical claims.
- SCA-011 Group 2: applied these exact additive rows under the recorded application authorization.
- SCA-011 Group 3: audited poststate accepted and DAG-011 adopted through the recorded authority propagation; execution satisfaction remains pending.

## Lifecycle Summary

Local row status: {'ACTIVE': 12}. Local satisfaction counts: {'NOT_APPLICABLE': 4, 'PENDING': 8}. Exactly one active parent anchor is required and checked. These are dependency row states, not deliverable lifecycle changes.

## Downstream Handoff Notes

Group 3 audited this local mirror against the recorded applied poststate and adopted DAG-011 as current graph authority. The accepted DAG-010 predecessor remains immutable. All added execution relations remain PENDING with actual maturity TBD; baseline satisfaction evidence was preserved, not revalidated. Future satisfaction changes require actual source-owner and consumer evidence; no hold, lifecycle, engineering or release acceptance follows from graph adoption.

## Group-2 metadata correction

Root SPEC §6.8 canonical DEP-owner identities and §6.5 verbatim local evidence have been applied to the added rows. Original SCA011 identities and proposal citations remain in CSV Notes and the ID crosswalk. The accepted ownership relations, prior rows and pending maturity are unchanged. Correction authority and exact field deltas: `projects/chirality-piping/execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP/application/dependencies/CORRECTNESS_REPAIR.md`. At correction time, graph adoption remained subject to Group 3.
