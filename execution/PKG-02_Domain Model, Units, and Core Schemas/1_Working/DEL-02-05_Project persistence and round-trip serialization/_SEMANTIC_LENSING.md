# Semantic Lensing Register: DEL-02-05 Project persistence and round-trip serialization

**Generated:** 2026-04-30
**Deliverable Folder:** /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-05_Project persistence and round-trip serialization
**Warnings:** none

**Inputs Read:**
- _CONTEXT.md - deliverable identity, scope, architecture basis injection
- _STATUS.md - lifecycle state only; not modified
- _SEMANTIC.md - matrices A, B, C, F, D, X, E parsed as lenses
- Datasheet.md - production document
- Specification.md - production document
- Guidance.md - production document
- Procedure.md - production document
- _REFERENCES.md - present; listed only, not expanded

**Purpose:** Apply `semantic-matrix-build` matrix cells as lenses over the production documents, capturing warranted enrichment inputs for a later enrichment pass.

## Summary

- Total warranted items: 21
- By document:
  - Datasheet: 1
  - Specification: 11
  - Guidance: 4
  - Procedure: 5
- By matrix:
  - A: 0  B: 3  C: 3  F: 4  D: 3  X: 4  E: 4
- By type:
  - Conflict: 0
  - VerificationGap: 9
  - MissingSlot: 8
  - WeakStatement: 0
  - RationaleGap: 0
  - Normalization: 1
  - TBD_Question: 3
  - MatrixError: 0
- Notable conflicts: 0
- Matrix parse errors: 0

## Matrix A - Orientation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| A:normative:guiding | normative | guiding | prescriptive direction | 0 | NO_ITEMS | Production documents provide direction; more specific gaps are captured under downstream requirement and verification lenses. |
| A:normative:applying | normative | applying | mandatory practice | 0 | NO_ITEMS | Mandatory practices are present at requirement level; no distinct warranted A-lens item. |
| A:normative:judging | normative | judging | compliance determination | 0 | NO_ITEMS | Professional/compliance boundary is stated consistently; no conflict surfaced. |
| A:normative:reviewing | normative | reviewing | regulatory audit | 0 | NO_ITEMS | Audit concerns are handled by verification/review lenses below. |
| A:operative:guiding | operative | guiding | procedural direction | 0 | NO_ITEMS | Procedure gives implementation direction; specific missing operational artifacts are captured under F, D, X, and E. |
| A:operative:applying | operative | applying | practical execution | 0 | NO_ITEMS | Execution gaps are represented by service contract and test-plan lenses below. |
| A:operative:judging | operative | judging | performance assessment | 0 | NO_ITEMS | No distinct performance-assessment conflict found in production docs. |
| A:operative:reviewing | operative | reviewing | process audit | 0 | NO_ITEMS | Process audit gaps are captured where procedure/review records are missing. |
| A:evaluative:guiding | evaluative | guiding | value orientation | 0 | NO_ITEMS | Guidance states the value frame; no distinct warranted item. |
| A:evaluative:applying | evaluative | applying | merit application | 0 | NO_ITEMS | Merit/application concerns appear as verification adequacy gaps below. |
| A:evaluative:judging | evaluative | judging | worth determination | 0 | NO_ITEMS | Quality judgment gaps are captured under D/X/E lenses. |
| A:evaluative:reviewing | evaluative | reviewing | quality appraisal | 0 | NO_ITEMS | Appraisal evidence gaps are captured under verification and evaluation lenses. |

## Matrix B - Conceptualization

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| B:data:necessity | data | necessity | essential fact | 0 | NO_ITEMS | Essential identity and scope facts are present. |
| B:data:sufficiency | data | sufficiency | adequate evidence | 0 | NO_ITEMS | Evidence basis is cited at a high level; actionable gaps are captured in narrower lenses. |
| B:data:completeness | data | completeness | comprehensive record | 1 | HAS_ITEMS | Field-level schema inventory remains underspecified. |
| B:data:consistency | data | consistency | reliable measurement | 0 | NO_ITEMS | Measurement/unit concerns are captured under C and X. |
| B:information:necessity | information | necessity | essential signal | 0 | NO_ITEMS | No distinct missing signal found. |
| B:information:sufficiency | information | sufficiency | adequate context | 0 | NO_ITEMS | Context is adequate for current worklist generation. |
| B:information:completeness | information | completeness | comprehensive account | 0 | NO_ITEMS | No separate comprehensive-account item beyond schema/test gaps below. |
| B:information:consistency | information | consistency | coherent message | 1 | HAS_ITEMS | Project persistence terminology can drift without normalization. |
| B:knowledge:necessity | knowledge | necessity | fundamental understanding | 0 | NO_ITEMS | No distinct missing knowledge basis found. |
| B:knowledge:sufficiency | knowledge | sufficiency | competent expertise | 0 | NO_ITEMS | Implementation expertise decisions are intentionally deferred. |
| B:knowledge:completeness | knowledge | completeness | thorough mastery | 0 | NO_ITEMS | No distinct mastery-gap item beyond open decisions. |
| B:knowledge:consistency | knowledge | consistency | coherent understanding | 0 | NO_ITEMS | Coherence issues are captured under terminology and migration items. |
| B:wisdom:necessity | wisdom | necessity | essential discernment | 0 | NO_ITEMS | No distinct discernment item found. |
| B:wisdom:sufficiency | wisdom | sufficiency | adequate judgment | 1 | HAS_ITEMS | The physical package/container decision needs a human judgment path. |
| B:wisdom:completeness | wisdom | completeness | holistic insight | 0 | NO_ITEMS | No additional holistic insight gap found. |
| B:wisdom:consistency | wisdom | consistency | principled reasoning | 0 | NO_ITEMS | Reasoning is consistent where documented. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| B-001 | B:data:completeness | MissingSlot | Datasheet | Specification | Add a field-level project envelope inventory for schema version, project identity, unit system reference, model payload, load payloads, rule-pack references, provenance metadata, and validation or migration status. | The docs identify required payload categories, but do not enumerate the concrete persisted slots needed for a complete schema contract. | Datasheet.md; Specification.md | Datasheet `## Construction`; Specification `## Requirements` REQ-02-05-003 through REQ-02-05-006 | N/A | Specification | TBD |
| B-002 | B:information:consistency | Normalization | Guidance | Guidance | Clarify canonical vocabulary for project file, project document, project envelope, and project package/container. | The terms are used in related but not identical ways, which can blur the boundary between logical schema contract and physical storage container. | Specification.md; Datasheet.md; Guidance.md; Procedure.md | Specification `## Scope`; Datasheet `## Construction`; Guidance `## Considerations`; Procedure `## Steps` step 4 | N/A | Guidance | TBD |
| B-003 | B:wisdom:sufficiency | TBD_Question | Guidance | Guidance | TBD: Who owns the decision for physical project package/container, and what criteria should resolve single JSON file versus packaged container? | Multiple documents state the container is TBD; the later enrichment pass needs an explicit decision path rather than an inferred implementation choice. | Datasheet.md; Specification.md; Guidance.md | Datasheet `## Attributes` Project package/container; Specification `## Scope`; Guidance `## Trade-offs` | N/A | Guidance | TBD |

## Matrix C - Formulation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| C:normative:necessity | normative | necessity | Binding Contract Basis | 0 | NO_ITEMS | Binding requirements exist; specific open contract slots are captured below. |
| C:normative:sufficiency | normative | sufficiency | Defensible Evidence Basis | 0 | NO_ITEMS | Evidence sufficiency concerns are captured under F and X. |
| C:normative:completeness | normative | completeness | Comprehensive Contract Record | 0 | NO_ITEMS | Record completeness is represented by B/F/D items. |
| C:normative:consistency | normative | consistency | Controlled Conformance Logic | 1 | HAS_ITEMS | Unit-system conformance needs round-trip equality criteria. |
| C:operative:necessity | operative | necessity | Executable Workflow Basis | 0 | NO_ITEMS | Workflow basis is present in Procedure. |
| C:operative:sufficiency | operative | sufficiency | Usable Evidence Context | 0 | NO_ITEMS | Service/test evidence gaps are captured under F and X. |
| C:operative:completeness | operative | completeness | Complete Process Account | 0 | NO_ITEMS | Process account gaps are captured under D and X. |
| C:operative:consistency | operative | consistency | Stable Execution Meaning | 1 | HAS_ITEMS | Hash/canonicalization scope needs precise stability semantics. |
| C:evaluative:necessity | evaluative | necessity | Value Discernment Basis | 0 | NO_ITEMS | No distinct value-basis item. |
| C:evaluative:sufficiency | evaluative | sufficiency | Adequate Appraisal Ground | 0 | NO_ITEMS | Appraisal sufficiency is captured in F and E. |
| C:evaluative:completeness | evaluative | completeness | Holistic Quality Account | 1 | HAS_ITEMS | Provenance completeness criteria are not yet enumerated. |
| C:evaluative:consistency | evaluative | consistency | Principled Merit Logic | 0 | NO_ITEMS | No additional consistency issue found. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| C-001 | C:normative:consistency | VerificationGap | Specification | Specification | Add round-trip equality criteria for explicit units, declared unit-system references, missing units, and incompatible unit metadata. | The requirement forbids silent unit defaults and requires unit-aware persistence, but the acceptance criteria do not define what counts as equivalent after serialization. | Specification.md; Procedure.md | Specification `## Requirements` REQ-02-05-007 and `## Verification`; Procedure `## Verification` Unit/provenance check | N/A | Specification | TBD |
| C-002 | C:operative:consistency | VerificationGap | Guidance | Specification | Define hash payload partitioning and excluded volatile/session fields for deterministic JSON comparisons. | The docs warn that hashing volatile or session-only fields undermines reproducibility, but the exact payload subset and manifest split remain TBD. | Guidance.md; Procedure.md; Specification.md | Guidance `## Considerations` and `## Trade-offs`; Procedure `## Steps` step 6; Specification `## Verification` | N/A | Specification | TBD |
| C-003 | C:evaluative:completeness | VerificationGap | Specification | Specification | Enumerate provenance completeness criteria by record type, including materials, components, SIF/flexibility values, allowables, rule-pack values, and engineering-reliance data. | The docs require missing provenance to produce findings, but do not define the minimum provenance fields per affected record class. | Specification.md; Datasheet.md | Specification `## Requirements` REQ-02-05-008 and `## Verification`; Datasheet `## Attributes` Core data objects affected and Diagnostic behavior | N/A | Specification | TBD |

## Matrix F - Requirements

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| F:normative:necessity | normative | necessity | Required Contract Foundation | 0 | NO_ITEMS | Core persistence requirements are present. |
| F:normative:sufficiency | normative | sufficiency | Evidence Closure Standard | 0 | NO_ITEMS | Evidence closure items are represented under verification lenses. |
| F:normative:completeness | normative | completeness | Complete Obligation Record | 1 | HAS_ITEMS | Migration status semantics remain incomplete. |
| F:normative:consistency | normative | consistency | Stable Rule Coherence | 0 | NO_ITEMS | Rule-coherence issues are captured under E. |
| F:operative:necessity | operative | necessity | Required Service Capability | 0 | NO_ITEMS | Required service capability is named. |
| F:operative:sufficiency | operative | sufficiency | Adequate Workflow Proof | 1 | HAS_ITEMS | Service operation signatures are not yet specified. |
| F:operative:completeness | operative | completeness | Complete State Preservation | 0 | NO_ITEMS | State preservation gaps are represented under X. |
| F:operative:consistency | operative | consistency | Deterministic Process Behavior | 1 | HAS_ITEMS | Diagnostic taxonomy needs deterministic mapping. |
| F:evaluative:necessity | evaluative | necessity | Required Assurance Basis | 0 | NO_ITEMS | Assurance basis is present at policy level. |
| F:evaluative:sufficiency | evaluative | sufficiency | Defensible Appraisal Ground | 1 | HAS_ITEMS | Review-gate evidence template is missing. |
| F:evaluative:completeness | evaluative | completeness | Complete Quality Evidence | 0 | NO_ITEMS | Quality evidence gaps are captured under E and X. |
| F:evaluative:consistency | evaluative | consistency | Coherent Assurance Logic | 0 | NO_ITEMS | No distinct assurance-logic conflict found. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| F-001 | F:normative:completeness | VerificationGap | Specification | Specification | Define migration status states and required diagnostics for unsupported, stale, newer, failed, and successfully migrated project versions. | The requirement says migration status must detect unsupported, stale, or failed migrations, but the status model and acceptance cases are not enumerated. | Specification.md; Procedure.md; Datasheet.md | Specification `## Requirements` REQ-02-05-006 and `## Verification`; Procedure `## Verification` Migration check; Datasheet `## Attributes` Migration mechanism | N/A | Specification | TBD |
| F-002 | F:operative:sufficiency | MissingSlot | Specification | Specification | Add create, open, save, validate, version, and migrate operation signatures with inputs, outputs, result envelopes, and boundary checks. | The service contract is an anticipated artifact, and the docs describe expected operations without concrete interface shape. | Datasheet.md; Specification.md; Procedure.md | Datasheet `## Construction` Persistence service contract; Specification `## Verification`; Procedure `## Steps` step 5 | N/A | Specification | TBD |
| F-003 | F:operative:consistency | MissingSlot | Specification | Specification | Add a diagnostic code namespace or class taxonomy for validation, migration, protected-content, private-data, unit, and provenance failures. | Structured diagnostic fields are required, but deterministic codes/classes and mappings are not yet specified. | Specification.md; Procedure.md | Specification `## Requirements` REQ-02-05-011; Procedure `## Steps` step 5 and `## Verification` Service-contract check | N/A | Specification | TBD |
| F-004 | F:evaluative:sufficiency | VerificationGap | Procedure | Procedure | Add a protected-content/private-data review gate record template for fixture provenance, redistribution status, and professional-boundary checks. | The docs require public-data safety review, but no concrete review evidence artifact or pass/fail template is defined. | Specification.md; Procedure.md | Specification `## Verification`; Procedure `## Steps` step 8 and `## Records` | N/A | Procedure | TBD |

## Matrix D - Objectives

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| D:normative:guiding | normative | guiding | Contract Direction Closure | 1 | HAS_ITEMS | Schema layout/tooling decision path is still open. |
| D:normative:applying | normative | applying | Mandatory Evidence Practice | 0 | NO_ITEMS | Mandatory evidence practice appears in verification plan; specific gaps are in X. |
| D:normative:judging | normative | judging | Conformance Decision Closure | 0 | NO_ITEMS | No separate conformance decision gap found. |
| D:normative:reviewing | normative | reviewing | Audit Trail Closure | 0 | NO_ITEMS | Audit trail gaps are represented in E/X. |
| D:operative:guiding | operative | guiding | Workflow Direction Closure | 0 | NO_ITEMS | Procedure direction is present. |
| D:operative:applying | operative | applying | Service Execution Closure | 0 | NO_ITEMS | Service execution specifics are captured under F. |
| D:operative:judging | operative | judging | Behavior Assessment Closure | 0 | NO_ITEMS | Behavior assessment gaps are in X. |
| D:operative:reviewing | operative | reviewing | Process Trace Closure | 1 | HAS_ITEMS | Open-decision process trace lacks an artifact path. |
| D:evaluative:guiding | evaluative | guiding | Value Direction Closure | 0 | NO_ITEMS | Value direction is stated in Guidance. |
| D:evaluative:applying | evaluative | applying | Merit Practice Closure | 0 | NO_ITEMS | No separate merit-practice gap found. |
| D:evaluative:judging | evaluative | judging | Quality Decision Closure | 1 | HAS_ITEMS | Example/fixture quality evidence remains undeveloped. |
| D:evaluative:reviewing | evaluative | reviewing | Appraisal Trace Closure | 0 | NO_ITEMS | Review trace gaps are captured under E. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| D-001 | D:normative:guiding | TBD_Question | Specification | Guidance | TBD: Where should the exact schema file layout and code-generation/tooling decision be recorded, and who can approve it? | The documents explicitly leave schema layout and tooling TBD, but downstream implementation needs a controlled decision path. | Specification.md; Procedure.md | Specification `## Requirements` REQ-02-05-004 and `## Documentation`; Procedure `## Steps` step 4 and `## Records` | N/A | Guidance | TBD |
| D-002 | D:operative:reviewing | MissingSlot | Procedure | Procedure | Add an open-decision log artifact path, owner, and ruling workflow for physical container, migration tooling, schema layout, dependency versions, and hash partitioning. | Procedure requires open items to be recorded, but the specific record location and ruling mechanism are not named. | Specification.md; Procedure.md | Specification `## Documentation`; Procedure `## Steps` step 9 and `## Records` | N/A | Procedure | TBD |
| D-003 | D:evaluative:judging | MissingSlot | Guidance | Procedure | Add fixture/example provenance and redistribution fields for future public project examples. | Guidance says examples are TBD, while the specification and procedure require invented or permissively licensed fixtures with provenance. | Guidance.md; Specification.md; Procedure.md | Guidance `## Examples`; Specification `## Requirements` REQ-02-05-014; Procedure `## Steps` steps 7 and 8 | N/A | Procedure | TBD |

## Matrix X - Verification

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| X:guiding:necessity | guiding | necessity | Directive Evidence Basis | 0 | NO_ITEMS | Directive evidence basis is present. |
| X:guiding:sufficiency | guiding | sufficiency | Contextual Direction Proof | 0 | NO_ITEMS | Contextual proof is adequate for current stage. |
| X:guiding:completeness | guiding | completeness | Comprehensive Direction Record | 0 | NO_ITEMS | Direction record gaps are captured under D. |
| X:guiding:consistency | guiding | consistency | Stable Direction Meaning | 0 | NO_ITEMS | Stable direction issue captured under B terminology. |
| X:applying:necessity | applying | necessity | Practice Readiness Basis | 0 | NO_ITEMS | Practice readiness is described at procedure level. |
| X:applying:sufficiency | applying | sufficiency | Executable Evidence Proof | 0 | NO_ITEMS | Executable proof gaps are narrowed in other X cells. |
| X:applying:completeness | applying | completeness | Complete Practice Record | 1 | HAS_ITEMS | Fixture inventory and expected outputs are missing. |
| X:applying:consistency | applying | consistency | Stable Practice Meaning | 1 | HAS_ITEMS | Round-trip semantic equality criteria are too high-level. |
| X:judging:necessity | judging | necessity | Decision Evidence Basis | 1 | HAS_ITEMS | Version diagnostics require explicit test cases. |
| X:judging:sufficiency | judging | sufficiency | Assessment Proof Ground | 0 | NO_ITEMS | No separate sufficiency issue found. |
| X:judging:completeness | judging | completeness | Complete Decision Record | 0 | NO_ITEMS | Decision-record gaps covered by migration and reproducibility items. |
| X:judging:consistency | judging | consistency | Coherent Decision Rationale | 0 | NO_ITEMS | No distinct rationale conflict found. |
| X:reviewing:necessity | reviewing | necessity | Trace Evidence Basis | 0 | NO_ITEMS | Trace basis is present. |
| X:reviewing:sufficiency | reviewing | sufficiency | Audit Proof Ground | 1 | HAS_ITEMS | Reproducibility manifest audit criteria need definition. |
| X:reviewing:completeness | reviewing | completeness | Complete Trace Record | 0 | NO_ITEMS | Complete trace concerns are covered by E review items. |
| X:reviewing:consistency | reviewing | consistency | Coherent Audit Rationale | 0 | NO_ITEMS | No distinct audit-rationale inconsistency found. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| X-001 | X:applying:completeness | VerificationGap | Procedure | Procedure | Add a fixture inventory with valid and invalid project fixtures plus expected canonical JSON and hash outputs. | Round-trip and canonicalization tests are required, but the procedure does not yet list concrete fixtures or expected outputs. | Specification.md; Procedure.md | Specification `## Verification`; Procedure `## Steps` step 7 and `## Records` | N/A | Procedure | TBD |
| X-002 | X:judging:necessity | VerificationGap | Procedure | Procedure | Add explicit tests for unsupported, stale, newer, failed migration, and successful migration-status cases. | Migration diagnostics are required, but the test plan only names the category and does not define cases. | Specification.md; Procedure.md | Specification `## Requirements` REQ-02-05-006 and `## Verification`; Procedure `## Verification` Migration check | N/A | Procedure | TBD |
| X-003 | X:reviewing:sufficiency | VerificationGap | Specification | Specification | Define model-hash and input-manifest compatibility criteria for reproducibility metadata. | Reproducibility metadata and manifest compatibility are required, but the docs do not specify the fields or comparison rules. | Specification.md; Procedure.md | Specification `## Requirements` REQ-02-05-012 and `## Verification`; Procedure `## Verification` Reproducibility records | N/A | Specification | TBD |
| X-004 | X:applying:consistency | VerificationGap | Specification | Specification | Enumerate semantic equality checks for model content, unit metadata, loads, rule-pack references, and provenance metadata after parse-normalize-serialize-parse. | The round-trip requirement lists preserved categories but does not define per-category comparison criteria. | Specification.md; Procedure.md | Specification `## Requirements` REQ-02-05-002 and `## Verification`; Procedure `## Steps` step 7 | N/A | Specification | TBD |

## Matrix E - Evaluation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| E:guiding:data | guiding | data | Directive Fact Trace | 0 | NO_ITEMS | Directive fact trace is present. |
| E:guiding:information | guiding | information | Directive Context Signal | 1 | HAS_ITEMS | Local-first/private-data operation context needs clarification. |
| E:guiding:knowledge | guiding | knowledge | Directive Expertise Frame | 0 | NO_ITEMS | No distinct expertise-frame gap found. |
| E:guiding:wisdom | guiding | wisdom | Directive Discernment Frame | 0 | NO_ITEMS | Decision discernment is represented by B and D items. |
| E:applying:data | applying | data | Practice Fact Trace | 0 | NO_ITEMS | Practice fact trace is adequate at current stage. |
| E:applying:information | applying | information | Execution Context Signal | 0 | NO_ITEMS | Execution context gaps are captured under service diagnostics. |
| E:applying:knowledge | applying | knowledge | Service Expertise Frame | 1 | HAS_ITEMS | Rule-pack reference field semantics need specification. |
| E:applying:wisdom | applying | wisdom | Practice Discernment Frame | 0 | NO_ITEMS | No distinct practice discernment gap found. |
| E:judging:data | judging | data | Decision Fact Trace | 0 | NO_ITEMS | Decision facts are covered elsewhere. |
| E:judging:information | judging | information | Assessment Context Signal | 0 | NO_ITEMS | No additional assessment context issue found. |
| E:judging:knowledge | judging | knowledge | Decision Expertise Frame | 1 | HAS_ITEMS | Human-review authority-label semantics are missing. |
| E:judging:wisdom | judging | wisdom | Assessment Discernment Frame | 0 | NO_ITEMS | No separate discernment item found. |
| E:reviewing:data | reviewing | data | Audit Fact Trace | 1 | HAS_ITEMS | Review evidence artifact naming is missing. |
| E:reviewing:information | reviewing | information | Audit Context Signal | 0 | NO_ITEMS | No separate audit context gap found. |
| E:reviewing:knowledge | reviewing | knowledge | Audit Expertise Frame | 0 | NO_ITEMS | No distinct audit expertise gap found. |
| E:reviewing:wisdom | reviewing | wisdom | Audit Discernment Frame | 0 | NO_ITEMS | No separate audit discernment item found. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| E-001 | E:applying:knowledge | MissingSlot | Specification | Specification | Define required rule-pack reference fields and validation behavior for version, checksum, source note, privacy status, and missing/private references. | Rule-pack references are required, but the exact persisted reference shape and diagnostics are not yet specified. | Specification.md; Guidance.md; Datasheet.md | Specification `## Requirements` REQ-02-05-009; Guidance `## Principles`; Datasheet `## Construction` | N/A | Specification | TBD |
| E-002 | E:judging:knowledge | MissingSlot | Specification | Specification | Add schema semantics for optional human review records, authority labels, hashes, and non-compliance-claim boundaries. | Guidance allows persistence to record user-rule check states or human review records with labels and hashes, but the schema-level meaning is not defined. | Guidance.md; Specification.md | Guidance `## Principles`; Specification `## Requirements` REQ-02-05-012 and REQ-02-05-016 | N/A | Specification | TBD |
| E-003 | E:reviewing:data | MissingSlot | Procedure | Procedure | Add a named review evidence artifact for no protected standards/code data, no proprietary values, and no professional approval claims. | The production docs require this review, but do not name the evidence artifact that would make the audit trace durable. | Procedure.md; Specification.md; Datasheet.md | Procedure `## Records`; Specification `## Verification`; Datasheet `## Conditions` | N/A | Procedure | TBD |
| E-004 | E:guiding:information | TBD_Question | Specification | Specification | TBD: Do create/open/save operations enforce local-first private-data safeguards directly, or is the local-first rule limited to public fixture and publication policy? | The docs state private data must remain local-first and not be transmitted or committed publicly by default, but do not clarify the persistence operation behavior needed to enforce that boundary. | Specification.md; Datasheet.md; Guidance.md | Specification `## Requirements` REQ-02-05-013; Datasheet `## Conditions`; Guidance `## Considerations` | N/A | Specification | TBD |
