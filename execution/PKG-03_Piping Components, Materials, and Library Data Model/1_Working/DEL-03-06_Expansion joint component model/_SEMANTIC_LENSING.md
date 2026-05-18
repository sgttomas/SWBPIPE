# Semantic Lensing Register: DEL-03-06 Expansion joint component model

**Generated:** 2026-04-30
**Deliverable Folder:** /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-06_Expansion joint component model
**Warnings:** none

**Inputs Read:**
- _CONTEXT.md - deliverable identity and scope
- _STATUS.md - SEMANTIC_READY state
- _SEMANTIC.md - matrices A, B, C, F, D, X, E
- Datasheet.md - available
- Specification.md - available
- Guidance.md - available
- Procedure.md - available
- _REFERENCES.md - source pointers only

**Purpose:** Apply semantic-matrix-build matrix cells as lenses over the production documents, capturing warranted enrichment inputs for a later enrichment pass.

## Summary

- Total warranted items: 5
- By document:
  - Datasheet: 2
  - Specification: 2
  - Guidance: 1
  - Procedure: 0
- By matrix:
  - A: 0  B: 0  C: 1  F: 2  D: 1  X: 1  E: 0
- By type:
  - Conflict: 0
  - VerificationGap: 1
  - MissingSlot: 2
  - WeakStatement: 0
  - RationaleGap: 0
  - Normalization: 0
  - TBD_Question: 2
  - MatrixError: 0
- Notable conflicts: 0
- Matrix parse errors: 0

## Matrix A - Orientation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| A:normative:guiding | normative | guiding | prescriptive direction | 0 | NO_ITEMS | Covered by specification constraints. |
| A:normative:applying | normative | applying | mandatory practice | 0 | NO_ITEMS | Covered by requirements. |
| A:normative:judging | normative | judging | compliance determination | 0 | NO_ITEMS | Compliance claims are excluded. |
| A:normative:reviewing | normative | reviewing | regulatory audit | 0 | NO_ITEMS | Review remains human/proposal gated. |
| A:operative:guiding | operative | guiding | procedural direction | 0 | NO_ITEMS | Covered by Procedure.md. |
| A:operative:applying | operative | applying | practical execution | 0 | NO_ITEMS | Product execution remains future scope. |
| A:operative:judging | operative | judging | performance assessment | 0 | NO_ITEMS | Validation tests anticipated. |
| A:operative:reviewing | operative | reviewing | process audit | 0 | NO_ITEMS | Setup records present. |
| A:evaluative:guiding | evaluative | guiding | value orientation | 0 | NO_ITEMS | Guidance states principles. |
| A:evaluative:applying | evaluative | applying | merit application | 0 | NO_ITEMS | No warranted enrichment. |
| A:evaluative:judging | evaluative | judging | worth determination | 0 | NO_ITEMS | No warranted enrichment. |
| A:evaluative:reviewing | evaluative | reviewing | quality appraisal | 0 | NO_ITEMS | QA checks recorded. |

## Matrix B - Conceptualization

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| B:data:necessity | data | necessity | essential fact | 0 | NO_ITEMS | Scope facts present. |
| B:data:sufficiency | data | sufficiency | adequate evidence | 0 | NO_ITEMS | Source basis stated. |
| B:data:completeness | data | completeness | comprehensive record | 0 | NO_ITEMS | Unknowns marked TBD. |
| B:data:consistency | data | consistency | reliable measurement | 0 | NO_ITEMS | Units required, values TBD. |
| B:information:necessity | information | necessity | essential signal | 0 | NO_ITEMS | Diagnostics required. |
| B:information:sufficiency | information | sufficiency | adequate context | 0 | NO_ITEMS | Context supplied. |
| B:information:completeness | information | completeness | comprehensive account | 0 | NO_ITEMS | Complete for setup scope. |
| B:information:consistency | information | consistency | coherent message | 0 | NO_ITEMS | Terminology aligned. |
| B:knowledge:necessity | knowledge | necessity | fundamental understanding | 0 | NO_ITEMS | Package boundary stated. |
| B:knowledge:sufficiency | knowledge | sufficiency | competent expertise | 0 | NO_ITEMS | Human rulings retained. |
| B:knowledge:completeness | knowledge | completeness | thorough mastery | 0 | NO_ITEMS | No product mastery claimed. |
| B:knowledge:consistency | knowledge | consistency | coherent understanding | 0 | NO_ITEMS | Cross-doc consistency checked. |
| B:wisdom:necessity | wisdom | necessity | essential discernment | 0 | NO_ITEMS | Stop rules present. |
| B:wisdom:sufficiency | wisdom | sufficiency | adequate judgment | 0 | NO_ITEMS | Human gate preserved. |
| B:wisdom:completeness | wisdom | completeness | holistic insight | 0 | NO_ITEMS | Future implementation TBD. |
| B:wisdom:consistency | wisdom | consistency | principled reasoning | 0 | NO_ITEMS | IP/data principles consistent. |

## Matrix C - Formulation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| C:normative:necessity | normative | necessity | source obligation | 0 | NO_ITEMS | Source obligations stated. |
| C:normative:sufficiency | normative | sufficiency | evidence threshold | 0 | NO_ITEMS | Evidence threshold conservative. |
| C:normative:completeness | normative | completeness | trace completeness | 0 | NO_ITEMS | Traceability present. |
| C:normative:consistency | normative | consistency | controlled coherence | 0 | NO_ITEMS | Terms aligned. |
| C:operative:necessity | operative | necessity | required input | 1 | HAS_ITEMS | Input taxonomy remains TBD. |
| C:operative:sufficiency | operative | sufficiency | workable basis | 0 | NO_ITEMS | Setup basis workable. |
| C:operative:completeness | operative | completeness | workflow coverage | 0 | NO_ITEMS | Procedure covers setup workflow. |
| C:operative:consistency | operative | consistency | process coherence | 0 | NO_ITEMS | No drift found. |
| C:evaluative:necessity | evaluative | necessity | decision need | 0 | NO_ITEMS | Human rulings identified. |
| C:evaluative:sufficiency | evaluative | sufficiency | judgment support | 0 | NO_ITEMS | No added item. |
| C:evaluative:completeness | evaluative | completeness | review coverage | 0 | NO_ITEMS | QA checks present. |
| C:evaluative:consistency | evaluative | consistency | appraisal coherence | 0 | NO_ITEMS | No added item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| C-001 | C:operative:necessity | TBD_Question | Datasheet | Datasheet | TBD: exact stiffness field shape and DOF mapping. | SOW-010 requires stiffness support, but setup sources do not define field shape. | Datasheet.md; Specification.md | Attributes; Requirements | N/A | PROPOSAL | TBD |

## Matrix F - Requirements

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| F:normative:necessity | normative | necessity | source requirement | 0 | NO_ITEMS | Supplied-data rule present. |
| F:normative:sufficiency | normative | sufficiency | acceptance basis | 1 | HAS_ITEMS | Acceptance criteria need future precision. |
| F:normative:completeness | normative | completeness | record closure | 0 | NO_ITEMS | Records described. |
| F:normative:consistency | normative | consistency | normalized control | 0 | NO_ITEMS | No conflict. |
| F:operative:necessity | operative | necessity | input requirement | 1 | HAS_ITEMS | Required categories listed, exact required/optional split TBD. |
| F:operative:sufficiency | operative | sufficiency | usable basis | 0 | NO_ITEMS | Basis adequate for setup. |
| F:operative:completeness | operative | completeness | complete workflow | 0 | NO_ITEMS | Procedure adequate for setup. |
| F:operative:consistency | operative | consistency | stable execution | 0 | NO_ITEMS | No conflict. |
| F:evaluative:necessity | evaluative | necessity | evaluation input | 0 | NO_ITEMS | Evaluation inputs stated. |
| F:evaluative:sufficiency | evaluative | sufficiency | defensible judgment | 0 | NO_ITEMS | Human judgment preserved. |
| F:evaluative:completeness | evaluative | completeness | full review | 0 | NO_ITEMS | No added item. |
| F:evaluative:consistency | evaluative | consistency | consistent review | 0 | NO_ITEMS | No added item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| F-001 | F:normative:sufficiency | VerificationGap | Specification | Specification | Add future acceptance criteria for unit/dimension validation and missing-data diagnostics. | Requirements exist, but implementation-level acceptance detail remains TBD. | Specification.md | Verification | N/A | PROPOSAL | TBD |
| F-002 | F:operative:necessity | MissingSlot | Datasheet | Datasheet | Record TBD: required vs optional expansion joint fields. | Data categories are identified, but required/optional taxonomy is not available from setup sources. | Datasheet.md | Attributes | N/A | PROPOSAL | TBD |

## Matrix D - Objectives

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| D:normative:guiding | normative | guiding | bounded directive | 0 | NO_ITEMS | Scope boundary clear. |
| D:normative:applying | normative | applying | enforceable practice | 0 | NO_ITEMS | Requirements clear. |
| D:normative:judging | normative | judging | closure determination | 0 | NO_ITEMS | No compliance claim. |
| D:normative:reviewing | normative | reviewing | control review | 0 | NO_ITEMS | Human gate preserved. |
| D:operative:guiding | operative | guiding | execution route | 0 | NO_ITEMS | Procedure clear. |
| D:operative:applying | operative | applying | implementable operation | 1 | HAS_ITEMS | Hardware enumeration remains TBD. |
| D:operative:judging | operative | judging | workflow assessment | 0 | NO_ITEMS | Future tests noted. |
| D:operative:reviewing | operative | reviewing | process assurance | 0 | NO_ITEMS | Setup QA noted. |
| D:evaluative:guiding | evaluative | guiding | review intent | 0 | NO_ITEMS | No item. |
| D:evaluative:applying | evaluative | applying | judgment use | 0 | NO_ITEMS | No item. |
| D:evaluative:judging | evaluative | judging | review decision | 0 | NO_ITEMS | No item. |
| D:evaluative:reviewing | evaluative | reviewing | appraisal control | 0 | NO_ITEMS | No item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| D-001 | D:operative:applying | TBD_Question | Guidance | Guidance | TBD: hardware flag/enumeration taxonomy. | SOW-010 names hardware data but not a stable enumeration. | Guidance.md; Specification.md | Considerations; Requirements | N/A | PROPOSAL | TBD |

## Matrix X - Verification

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| X:guiding:necessity | guiding | necessity | required check | 0 | NO_ITEMS | Checks identified. |
| X:guiding:sufficiency | guiding | sufficiency | proof check | 0 | NO_ITEMS | No item. |
| X:guiding:completeness | guiding | completeness | coverage check | 0 | NO_ITEMS | No item. |
| X:guiding:consistency | guiding | consistency | alignment check | 0 | NO_ITEMS | No item. |
| X:applying:necessity | applying | necessity | input check | 0 | NO_ITEMS | No item. |
| X:applying:sufficiency | applying | sufficiency | usability check | 1 | HAS_ITEMS | Movement-limit validation taxonomy TBD. |
| X:applying:completeness | applying | completeness | implementation check | 0 | NO_ITEMS | Future scope. |
| X:applying:consistency | applying | consistency | normalization check | 0 | NO_ITEMS | No item. |
| X:judging:necessity | judging | necessity | finding check | 0 | NO_ITEMS | No item. |
| X:judging:sufficiency | judging | sufficiency | acceptance check | 0 | NO_ITEMS | No item. |
| X:judging:completeness | judging | completeness | decision coverage | 0 | NO_ITEMS | No item. |
| X:judging:consistency | judging | consistency | decision alignment | 0 | NO_ITEMS | No item. |
| X:reviewing:necessity | reviewing | necessity | audit check | 0 | NO_ITEMS | No item. |
| X:reviewing:sufficiency | reviewing | sufficiency | review proof | 0 | NO_ITEMS | No item. |
| X:reviewing:completeness | reviewing | completeness | audit coverage | 0 | NO_ITEMS | No item. |
| X:reviewing:consistency | reviewing | consistency | audit alignment | 0 | NO_ITEMS | No item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| X-001 | X:applying:sufficiency | MissingSlot | Specification | Specification | Record TBD: movement-limit validation classes. | Movement limits are in scope, but setup sources do not define limit classes or validation categories. | Specification.md; Datasheet.md | Requirements; Attributes | N/A | PROPOSAL | TBD |

## Matrix E - Evaluation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| E:guiding:data | guiding | data | data trace | 0 | NO_ITEMS | Provenance noted. |
| E:guiding:information | guiding | information | information trace | 0 | NO_ITEMS | Context trace present. |
| E:guiding:knowledge | guiding | knowledge | knowledge trace | 0 | NO_ITEMS | Boundary knowledge present. |
| E:guiding:wisdom | guiding | wisdom | judgment trace | 0 | NO_ITEMS | Human gate present. |
| E:applying:data | applying | data | field evidence | 0 | NO_ITEMS | Field categories present. |
| E:applying:information | applying | information | interface evidence | 0 | NO_ITEMS | Future interfaces TBD. |
| E:applying:knowledge | applying | knowledge | implementation evidence | 0 | NO_ITEMS | Product work excluded. |
| E:applying:wisdom | applying | wisdom | judgment evidence | 0 | NO_ITEMS | No item. |
| E:judging:data | judging | data | finding evidence | 0 | NO_ITEMS | Findings expected for missing data. |
| E:judging:information | judging | information | acceptance context | 0 | NO_ITEMS | No item. |
| E:judging:knowledge | judging | knowledge | decision rationale | 0 | NO_ITEMS | No item. |
| E:judging:wisdom | judging | wisdom | acceptance rationale | 0 | NO_ITEMS | No item. |
| E:reviewing:data | reviewing | data | audit evidence | 0 | NO_ITEMS | Records noted. |
| E:reviewing:information | reviewing | information | review context | 0 | NO_ITEMS | No item. |
| E:reviewing:knowledge | reviewing | knowledge | audit rationale | 0 | NO_ITEMS | No item. |
| E:reviewing:wisdom | reviewing | wisdom | review rationale | 0 | NO_ITEMS | No item. |

