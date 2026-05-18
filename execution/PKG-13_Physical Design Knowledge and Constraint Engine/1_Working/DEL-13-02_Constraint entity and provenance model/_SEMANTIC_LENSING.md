# Semantic Lensing Register: DEL-13-02 Constraint entity and provenance model

**Generated:** 2026-05-04
**Deliverable Folder:** execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/DEL-13-02_Constraint entity and provenance model
**Warnings:** None

**Inputs Read:**
- _CONTEXT.md - deliverable identity and scope.
- _STATUS.md - SEMANTIC_READY after semantic generation.
- _SEMANTIC.md - matrices A, B, C, F, D, X, E parsed from result tables.
- Datasheet.md - read.
- Specification.md - read.
- Guidance.md - read.
- Procedure.md - read.
- _REFERENCES.md - read as reference index only.

**Purpose:** Apply `semantic-matrix-build` matrix cells as lenses over the production documents, capturing warranted enrichment inputs for a later enrichment pass.

## Summary

- Total warranted items: 6
- By document:
  - Datasheet: 1
  - Specification: 3
  - Guidance: 1
  - Procedure: 1
- By matrix:
  - A: 1  B: 1  C: 1  F: 1  D: 0  X: 1  E: 1
- By type:
  - Conflict: 0
  - VerificationGap: 2
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
| A:normative:guiding | normative | guiding | prescriptive direction | 0 | NO_ITEMS | No additional warranted item. |
| A:normative:applying | normative | applying | mandatory practice | 1 | HAS_ITEMS | Implementation-specific schema practice remains TBD. |
| A:normative:judging | normative | judging | compliance determination | 0 | NO_ITEMS | Professional-boundary limits are already stated. |
| A:normative:reviewing | normative | reviewing | regulatory audit | 0 | NO_ITEMS | Audit posture is covered at a high level. |
| A:operative:guiding | operative | guiding | procedural direction | 0 | NO_ITEMS | Procedure has bounded steps. |
| A:operative:applying | operative | applying | practical execution | 0 | NO_ITEMS | Execution deferred to later implementation. |
| A:operative:judging | operative | judging | performance assessment | 0 | NO_ITEMS | Verification section covers expected checks generally. |
| A:operative:reviewing | operative | reviewing | process audit | 0 | NO_ITEMS | No conflict found. |
| A:evaluative:guiding | evaluative | guiding | value orientation | 0 | NO_ITEMS | Guidance records principles and trade-offs. |
| A:evaluative:applying | evaluative | applying | merit application | 0 | NO_ITEMS | No additional warranted item. |
| A:evaluative:judging | evaluative | judging | worth determination | 0 | NO_ITEMS | No additional warranted item. |
| A:evaluative:reviewing | evaluative | reviewing | quality appraisal | 0 | NO_ITEMS | No additional warranted item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| A-001 | A:normative:applying | TBD_Question | Specification | Specification | Resolve exact schema property names, enum spellings, `$id`, and version fields. | The specification intentionally leaves implementation-level schema details as `TBD`; later enrichment needs a human-approved design or implementation source. | Specification.md | Requirements, R-13-02-010 | N/A | PROPOSAL | TBD |

## Matrix B - Conceptualization

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| B:data:necessity | data | necessity | essential fact | 0 | NO_ITEMS | No additional warranted item. |
| B:data:sufficiency | data | sufficiency | adequate evidence | 0 | NO_ITEMS | No additional warranted item. |
| B:data:completeness | data | completeness | comprehensive record | 0 | NO_ITEMS | No additional warranted item. |
| B:data:consistency | data | consistency | reliable measurement | 1 | HAS_ITEMS | Schema identity/version slots are still missing. |
| B:information:necessity | information | necessity | essential signal | 0 | NO_ITEMS | No additional warranted item. |
| B:information:sufficiency | information | sufficiency | adequate context | 0 | NO_ITEMS | No additional warranted item. |
| B:information:completeness | information | completeness | comprehensive account | 0 | NO_ITEMS | No additional warranted item. |
| B:information:consistency | information | consistency | coherent message | 0 | NO_ITEMS | No additional warranted item. |
| B:knowledge:necessity | knowledge | necessity | fundamental understanding | 0 | NO_ITEMS | No additional warranted item. |
| B:knowledge:sufficiency | knowledge | sufficiency | competent expertise | 0 | NO_ITEMS | No additional warranted item. |
| B:knowledge:completeness | knowledge | completeness | thorough mastery | 0 | NO_ITEMS | No additional warranted item. |
| B:knowledge:consistency | knowledge | consistency | coherent understanding | 0 | NO_ITEMS | No additional warranted item. |
| B:wisdom:necessity | wisdom | necessity | essential discernment | 0 | NO_ITEMS | No additional warranted item. |
| B:wisdom:sufficiency | wisdom | sufficiency | adequate judgment | 0 | NO_ITEMS | No additional warranted item. |
| B:wisdom:completeness | wisdom | completeness | holistic insight | 0 | NO_ITEMS | No additional warranted item. |
| B:wisdom:consistency | wisdom | consistency | principled reasoning | 0 | NO_ITEMS | No additional warranted item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| B-001 | B:data:consistency | MissingSlot | Datasheet | Datasheet | Add concrete schema identity/version attributes after the schema path exists. | Datasheet construction names `$id`, `$schema`, and version fields as `TBD`; these are needed for stable schema identity once implementation begins. | Datasheet.md | Construction | N/A | PROPOSAL | TBD |

## Matrix C - Formulation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| C:normative:necessity | normative | necessity | binding evidence basis | 0 | NO_ITEMS | No additional warranted item. |
| C:normative:sufficiency | normative | sufficiency | sufficient compliance proof | 0 | NO_ITEMS | No additional warranted item. |
| C:normative:completeness | normative | completeness | complete obligation record | 0 | NO_ITEMS | No additional warranted item. |
| C:normative:consistency | normative | consistency | coherent control basis | 0 | NO_ITEMS | No additional warranted item. |
| C:operative:necessity | operative | necessity | required execution input | 1 | HAS_ITEMS | Unit-aware quantity representation remains unresolved. |
| C:operative:sufficiency | operative | sufficiency | usable workflow evidence | 0 | NO_ITEMS | No additional warranted item. |
| C:operative:completeness | operative | completeness | complete work record | 0 | NO_ITEMS | No additional warranted item. |
| C:operative:consistency | operative | consistency | stable execution message | 0 | NO_ITEMS | No additional warranted item. |
| C:evaluative:necessity | evaluative | necessity | review evidence basis | 0 | NO_ITEMS | No additional warranted item. |
| C:evaluative:sufficiency | evaluative | sufficiency | adequate appraisal context | 0 | NO_ITEMS | No additional warranted item. |
| C:evaluative:completeness | evaluative | completeness | complete judgment record | 0 | NO_ITEMS | No additional warranted item. |
| C:evaluative:consistency | evaluative | consistency | coherent review rationale | 0 | NO_ITEMS | No additional warranted item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| C-001 | C:operative:necessity | TBD_Question | Specification | Specification | Resolve unit-aware quantity representation for clearance, slope, elevation, and related physical values. | The specification requires unit-aware values but does not yet identify the exact quantity/reference shape for constraint fields. | Specification.md | Requirements, R-13-02-005 | N/A | PROPOSAL | TBD |

## Matrix F - Requirements

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| F:normative:necessity | normative | necessity | binding input obligation | 0 | NO_ITEMS | No additional warranted item. |
| F:normative:sufficiency | normative | sufficiency | proof threshold rule | 0 | NO_ITEMS | No additional warranted item. |
| F:normative:completeness | normative | completeness | complete compliance envelope | 1 | HAS_ITEMS | Provenance acceptance criteria are still general. |
| F:normative:consistency | normative | consistency | stable requirement meaning | 0 | NO_ITEMS | No additional warranted item. |
| F:operative:necessity | operative | necessity | required action precondition | 0 | NO_ITEMS | No additional warranted item. |
| F:operative:sufficiency | operative | sufficiency | workable evidence threshold | 0 | NO_ITEMS | No additional warranted item. |
| F:operative:completeness | operative | completeness | complete execution contract | 0 | NO_ITEMS | No additional warranted item. |
| F:operative:consistency | operative | consistency | stable workflow basis | 0 | NO_ITEMS | No additional warranted item. |
| F:evaluative:necessity | evaluative | necessity | reviewable decision basis | 0 | NO_ITEMS | No additional warranted item. |
| F:evaluative:sufficiency | evaluative | sufficiency | judgment proof threshold | 0 | NO_ITEMS | No additional warranted item. |
| F:evaluative:completeness | evaluative | completeness | complete appraisal envelope | 0 | NO_ITEMS | No additional warranted item. |
| F:evaluative:consistency | evaluative | consistency | stable review criterion | 0 | NO_ITEMS | No additional warranted item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| F-001 | F:normative:completeness | VerificationGap | Specification | Specification | Add acceptance criteria for minimum provenance fields once schema fields are chosen. | R-13-02-003 requires user/project/import/agent/source provenance where known, but the current verification text does not yet define field-level acceptance criteria. | Specification.md | Requirements R-13-02-003; Verification | N/A | PROPOSAL | TBD |

## Matrix D - Objectives

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| D:normative:guiding | normative | guiding | authoritative constraint direction | 0 | NO_ITEMS | No additional warranted item. |
| D:normative:applying | normative | applying | enforced schema practice | 0 | NO_ITEMS | Covered by A-001. |
| D:normative:judging | normative | judging | compliance finding basis | 0 | NO_ITEMS | Professional-boundary language avoids compliance claims. |
| D:normative:reviewing | normative | reviewing | audit-ready control record | 0 | NO_ITEMS | No additional warranted item. |
| D:operative:guiding | operative | guiding | procedure-ready direction | 0 | NO_ITEMS | Procedure exists. |
| D:operative:applying | operative | applying | controlled execution method | 0 | NO_ITEMS | No additional warranted item. |
| D:operative:judging | operative | judging | performance finding basis | 0 | NO_ITEMS | No additional warranted item. |
| D:operative:reviewing | operative | reviewing | process evidence record | 0 | NO_ITEMS | No additional warranted item. |
| D:evaluative:guiding | evaluative | guiding | value-framed rationale | 0 | NO_ITEMS | Guidance exists. |
| D:evaluative:applying | evaluative | applying | judgment application method | 0 | NO_ITEMS | No additional warranted item. |
| D:evaluative:judging | evaluative | judging | merit finding basis | 0 | NO_ITEMS | No additional warranted item. |
| D:evaluative:reviewing | evaluative | reviewing | quality evidence record | 0 | NO_ITEMS | No additional warranted item. |

## Matrix X - Verification

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| X:guiding:necessity | guiding | necessity | directional evidence need | 0 | NO_ITEMS | No additional warranted item. |
| X:guiding:sufficiency | guiding | sufficiency | instruction proof threshold | 0 | NO_ITEMS | No additional warranted item. |
| X:guiding:completeness | guiding | completeness | complete direction record | 0 | NO_ITEMS | No additional warranted item. |
| X:guiding:consistency | guiding | consistency | coherent guidance trace | 0 | NO_ITEMS | No additional warranted item. |
| X:applying:necessity | applying | necessity | implementation input basis | 0 | NO_ITEMS | No additional warranted item. |
| X:applying:sufficiency | applying | sufficiency | practice evidence threshold | 1 | HAS_ITEMS | Concrete validator command is unavailable until schema exists. |
| X:applying:completeness | applying | completeness | complete action record | 0 | NO_ITEMS | No additional warranted item. |
| X:applying:consistency | applying | consistency | stable practice trace | 0 | NO_ITEMS | No additional warranted item. |
| X:judging:necessity | judging | necessity | decision evidence need | 0 | NO_ITEMS | No additional warranted item. |
| X:judging:sufficiency | judging | sufficiency | finding proof threshold | 0 | NO_ITEMS | No additional warranted item. |
| X:judging:completeness | judging | completeness | complete decision record | 0 | NO_ITEMS | No additional warranted item. |
| X:judging:consistency | judging | consistency | coherent finding trace | 0 | NO_ITEMS | No additional warranted item. |
| X:reviewing:necessity | reviewing | necessity | audit input basis | 0 | NO_ITEMS | No additional warranted item. |
| X:reviewing:sufficiency | reviewing | sufficiency | audit evidence threshold | 0 | NO_ITEMS | No additional warranted item. |
| X:reviewing:completeness | reviewing | completeness | complete audit record | 0 | NO_ITEMS | No additional warranted item. |
| X:reviewing:consistency | reviewing | consistency | stable review trace | 0 | NO_ITEMS | No additional warranted item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| X-001 | X:applying:sufficiency | VerificationGap | Procedure | Procedure | Add concrete validator command/check once `schemas/constraint.schema.json` exists. | Procedure verification defines expected checks, but no executable validation command can be named before the schema artifact exists. | Procedure.md | Verification | N/A | PROPOSAL | TBD |

## Matrix E - Evaluation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| E:guiding:data | guiding | data | source direction basis | 1 | HAS_ITEMS | Example source decision remains missing. |
| E:guiding:information | guiding | information | context direction basis | 0 | NO_ITEMS | No additional warranted item. |
| E:guiding:knowledge | guiding | knowledge | expert direction basis | 0 | NO_ITEMS | No additional warranted item. |
| E:guiding:wisdom | guiding | wisdom | principled direction basis | 0 | NO_ITEMS | No additional warranted item. |
| E:applying:data | applying | data | source practice basis | 0 | NO_ITEMS | No additional warranted item. |
| E:applying:information | applying | information | context practice basis | 0 | NO_ITEMS | No additional warranted item. |
| E:applying:knowledge | applying | knowledge | expert practice basis | 0 | NO_ITEMS | No additional warranted item. |
| E:applying:wisdom | applying | wisdom | principled practice basis | 0 | NO_ITEMS | No additional warranted item. |
| E:judging:data | judging | data | source finding basis | 0 | NO_ITEMS | No additional warranted item. |
| E:judging:information | judging | information | context finding basis | 0 | NO_ITEMS | No additional warranted item. |
| E:judging:knowledge | judging | knowledge | expert finding basis | 0 | NO_ITEMS | No additional warranted item. |
| E:judging:wisdom | judging | wisdom | principled finding basis | 0 | NO_ITEMS | No additional warranted item. |
| E:reviewing:data | reviewing | data | source audit basis | 0 | NO_ITEMS | No additional warranted item. |
| E:reviewing:information | reviewing | information | context audit basis | 0 | NO_ITEMS | No additional warranted item. |
| E:reviewing:knowledge | reviewing | knowledge | expert audit basis | 0 | NO_ITEMS | No additional warranted item. |
| E:reviewing:wisdom | reviewing | wisdom | principled audit basis | 0 | NO_ITEMS | No additional warranted item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| E-001 | E:guiding:data | MissingSlot | Guidance | Guidance | Record example-payload decision: invented/public-permissive example or no example. | Guidance states that no source-grounded example payload is available and examples are `TBD`; later work should decide whether to add cleared examples or explicitly keep none. | Guidance.md | Examples | N/A | PROPOSAL | TBD |
