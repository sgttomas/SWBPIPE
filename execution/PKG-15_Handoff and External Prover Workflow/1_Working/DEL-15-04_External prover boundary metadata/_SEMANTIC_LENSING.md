# Semantic Lensing Register: DEL-15-04 External prover boundary metadata

**Generated:** 2026-05-03
**Deliverable Folder:** `execution/PKG-15_Handoff and External Prover Workflow/1_Working/DEL-15-04_External prover boundary metadata`
**Warnings:** None

**Inputs Read:**
- _CONTEXT.md - identity, scope, objectives, architecture basis
- _STATUS.md - `SEMANTIC_READY` observed after semantic matrix build
- _SEMANTIC.md - matrices A, B, C, F, D, X, E
- Datasheet.md - production document
- Specification.md - production document
- Guidance.md - production document
- Procedure.md - production document
- _REFERENCES.md - governing reference list

**Purpose:** Apply `semantic-matrix-build` matrix cells as lenses over the production documents, capturing warranted enrichment inputs for a later enrichment pass.

## Summary

- Total warranted items: 6
- By document:
  - Datasheet: 1
  - Specification: 2
  - Guidance: 1
  - Procedure: 2
- By matrix:
  - A: 1  B: 1  C: 0  F: 1  D: 1  X: 1  E: 1
- By type:
  - Conflict: 0
  - VerificationGap: 2
  - MissingSlot: 2
  - WeakStatement: 0
  - RationaleGap: 1
  - Normalization: 1
  - TBD_Question: 0
  - MatrixError: 0
- Notable conflicts: 0
- Matrix parse errors: 0

## Matrix A - Orientation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| A:normative:guiding | normative | guiding | prescriptive direction | 0 | NO_ITEMS | No distinct warranted item. |
| A:normative:applying | normative | applying | mandatory practice | 1 | HAS_ITEMS | Schema ownership is not yet specified. |
| A:normative:judging | normative | judging | compliance determination | 0 | NO_ITEMS | Prohibited authority language is already captured. |
| A:normative:reviewing | normative | reviewing | regulatory audit | 0 | NO_ITEMS | No regulatory clause text is available or needed. |
| A:operative:guiding | operative | guiding | procedural direction | 0 | NO_ITEMS | Procedure includes setup-level production guidance. |
| A:operative:applying | operative | applying | practical execution | 0 | NO_ITEMS | Future implementation scope remains TBD. |
| A:operative:judging | operative | judging | performance assessment | 0 | NO_ITEMS | Verification hooks are present at category level. |
| A:operative:reviewing | operative | reviewing | process audit | 0 | NO_ITEMS | No additional process gap. |
| A:evaluative:guiding | evaluative | guiding | value orientation | 0 | NO_ITEMS | Professional-boundary value is explicit. |
| A:evaluative:applying | evaluative | applying | merit application | 0 | NO_ITEMS | No distinct warranted item. |
| A:evaluative:judging | evaluative | judging | worth determination | 0 | NO_ITEMS | No distinct warranted item. |
| A:evaluative:reviewing | evaluative | reviewing | quality appraisal | 0 | NO_ITEMS | No distinct warranted item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| A-001 | A:normative:applying | MissingSlot | Specification | Specification | Record TBD: concrete schema file path and owning module for external reference fields. | The deliverable anticipates external reference fields, but the four documents leave the schema path and implementation owner unresolved. | Specification.md; Datasheet.md | Specification.md#Documentation; Datasheet.md#Attributes | N/A | PROPOSAL: Resolve during implementation brief or human architecture decision. | TBD |

## Matrix B - Conceptualization

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| B:data:necessity | data | necessity | essential fact | 0 | NO_ITEMS | Identity facts are present. |
| B:data:sufficiency | data | sufficiency | adequate evidence | 0 | NO_ITEMS | Evidence sources are listed. |
| B:data:completeness | data | completeness | comprehensive record | 0 | NO_ITEMS | Missing implementation particulars are marked TBD. |
| B:data:consistency | data | consistency | reliable measurement | 0 | NO_ITEMS | No numeric measurements are introduced. |
| B:information:necessity | information | necessity | essential signal | 0 | NO_ITEMS | Scope signal is present. |
| B:information:sufficiency | information | sufficiency | adequate context | 0 | NO_ITEMS | Context is sufficient for setup. |
| B:information:completeness | information | completeness | comprehensive account | 0 | NO_ITEMS | Account is conservative. |
| B:information:consistency | information | consistency | coherent message | 1 | HAS_ITEMS | Hyphenated and unhyphenated term forms should be normalized later. |
| B:knowledge:necessity | knowledge | necessity | fundamental understanding | 0 | NO_ITEMS | Boundary understanding is explicit. |
| B:knowledge:sufficiency | knowledge | sufficiency | competent expertise | 0 | NO_ITEMS | No expert value is asserted. |
| B:knowledge:completeness | knowledge | completeness | thorough mastery | 0 | NO_ITEMS | Completeness beyond setup remains TBD. |
| B:knowledge:consistency | knowledge | consistency | coherent understanding | 0 | NO_ITEMS | No distinct warranted item. |
| B:wisdom:necessity | wisdom | necessity | essential discernment | 0 | NO_ITEMS | Human authority boundary is present. |
| B:wisdom:sufficiency | wisdom | sufficiency | adequate judgment | 0 | NO_ITEMS | Judgment is deferred to human review. |
| B:wisdom:completeness | wisdom | completeness | holistic insight | 0 | NO_ITEMS | No additional warranted item. |
| B:wisdom:consistency | wisdom | consistency | principled reasoning | 0 | NO_ITEMS | No additional warranted item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| B-001 | B:information:consistency | Normalization | Multi | Guidance | Choose and document the canonical written form for `external-prover` / `external prover` when schema names are selected. | The sources use both natural-language and hyphenated forms. This is harmless for setup prose but can cause naming drift in a schema. | _CONTEXT.md; Specification.md; Guidance.md | _CONTEXT.md#Description; Specification.md#Scope; Guidance.md#Purpose | N/A | PROPOSAL: Keep prose natural-language and reserve a single machine-name convention for schema fields. | TBD |

## Matrix C - Formulation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| C:normative:necessity | normative | necessity | binding evidence threshold | 0 | NO_ITEMS | Prohibited-status threshold is present. |
| C:normative:sufficiency | normative | sufficiency | adequate authority basis | 0 | NO_ITEMS | Authority sources are cited. |
| C:normative:completeness | normative | completeness | comprehensive compliance record | 0 | NO_ITEMS | Compliance overclaim is avoided. |
| C:normative:consistency | normative | consistency | coherent audit standard | 0 | NO_ITEMS | No distinct warranted item. |
| C:operative:necessity | operative | necessity | essential execution basis | 0 | NO_ITEMS | Setup-level prerequisites are present. |
| C:operative:sufficiency | operative | sufficiency | adequate workflow context | 0 | NO_ITEMS | Workflow context is adequate for setup. |
| C:operative:completeness | operative | completeness | complete process record | 0 | NO_ITEMS | Later implementation process remains TBD by design. |
| C:operative:consistency | operative | consistency | reliable process signal | 0 | NO_ITEMS | No distinct warranted item. |
| C:evaluative:necessity | evaluative | necessity | essential judgment basis | 0 | NO_ITEMS | Human judgment boundary is present. |
| C:evaluative:sufficiency | evaluative | sufficiency | adequate appraisal rationale | 0 | NO_ITEMS | Rationale is source-grounded. |
| C:evaluative:completeness | evaluative | completeness | holistic quality record | 0 | NO_ITEMS | No distinct warranted item. |
| C:evaluative:consistency | evaluative | consistency | coherent merit standard | 0 | NO_ITEMS | No distinct warranted item. |

## Matrix F - Requirements

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| F:normative:necessity | normative | necessity | required warrant basis | 0 | NO_ITEMS | Requirement warrant is cited. |
| F:normative:sufficiency | normative | sufficiency | defensible authority record | 0 | NO_ITEMS | Authority record is adequate for setup. |
| F:normative:completeness | normative | completeness | complete conformance dossier | 0 | NO_ITEMS | No conformance claim is made. |
| F:normative:consistency | normative | consistency | stable compliance rationale | 1 | HAS_ITEMS | Negative status acceptance criteria need implementation-level detail. |
| F:operative:necessity | operative | necessity | required workflow input | 0 | NO_ITEMS | Inputs are listed. |
| F:operative:sufficiency | operative | sufficiency | workable evidence package | 0 | NO_ITEMS | Evidence package is local and bounded. |
| F:operative:completeness | operative | completeness | complete execution dossier | 0 | NO_ITEMS | No distinct warranted item. |
| F:operative:consistency | operative | consistency | stable process rationale | 0 | NO_ITEMS | No distinct warranted item. |
| F:evaluative:necessity | evaluative | necessity | required review basis | 0 | NO_ITEMS | Review basis is present. |
| F:evaluative:sufficiency | evaluative | sufficiency | defensible appraisal record | 0 | NO_ITEMS | No distinct warranted item. |
| F:evaluative:completeness | evaluative | completeness | complete review dossier | 0 | NO_ITEMS | No distinct warranted item. |
| F:evaluative:consistency | evaluative | consistency | stable merit rationale | 0 | NO_ITEMS | No distinct warranted item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| F-001 | F:normative:consistency | VerificationGap | Specification | Specification | Add implementation-level acceptance criteria for prohibited status labels and equivalent authority terms. | Specification requires negative tests, but exact matching/equivalence behavior is intentionally not defined in this setup pass. | Specification.md; Procedure.md | Specification.md#Verification; Procedure.md#Verification | N/A | PROPOSAL: Resolve in boundary validation tests once schema names are known. | TBD |

## Matrix D - Objectives

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| D:normative:guiding | normative | guiding | bounded policy direction | 0 | NO_ITEMS | Policy direction is present. |
| D:normative:applying | normative | applying | enforced practice boundary | 0 | NO_ITEMS | Practice boundary is present. |
| D:normative:judging | normative | judging | defensible conformance ruling | 0 | NO_ITEMS | No automatic conformance ruling is asserted. |
| D:normative:reviewing | normative | reviewing | traceable authority audit | 0 | NO_ITEMS | No distinct warranted item. |
| D:operative:guiding | operative | guiding | bounded workflow direction | 0 | NO_ITEMS | Workflow direction is present. |
| D:operative:applying | operative | applying | controlled execution path | 1 | HAS_ITEMS | Attachment/reference storage behavior is unresolved. |
| D:operative:judging | operative | judging | evidence performance ruling | 0 | NO_ITEMS | No distinct warranted item. |
| D:operative:reviewing | operative | reviewing | traceable process audit | 0 | NO_ITEMS | No distinct warranted item. |
| D:evaluative:guiding | evaluative | guiding | bounded value direction | 0 | NO_ITEMS | Boundary values are present. |
| D:evaluative:applying | evaluative | applying | reasoned merit use | 0 | NO_ITEMS | No distinct warranted item. |
| D:evaluative:judging | evaluative | judging | defensible worth ruling | 0 | NO_ITEMS | No distinct warranted item. |
| D:evaluative:reviewing | evaluative | reviewing | traceable quality audit | 0 | NO_ITEMS | No distinct warranted item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| D-001 | D:operative:applying | MissingSlot | Procedure | Procedure | Record TBD: attachment storage/reference behavior and external artifact hash treatment. | Procedure explicitly leaves attachment storage and external artifact hashing unresolved, yet attachments are an in-scope metadata category. | Procedure.md; Datasheet.md | Procedure.md#Steps; Datasheet.md#Construction | N/A | PROPOSAL: Decide whether attachments are embedded, referenced, hashed, or all of these in the implementation task. | TBD |

## Matrix X - Verification

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| X:guiding:necessity | guiding | necessity | required policy evidence | 0 | NO_ITEMS | Policy evidence is cited. |
| X:guiding:sufficiency | guiding | sufficiency | adequate direction context | 0 | NO_ITEMS | No distinct warranted item. |
| X:guiding:completeness | guiding | completeness | complete rationale record | 0 | NO_ITEMS | No distinct warranted item. |
| X:guiding:consistency | guiding | consistency | stable direction signal | 0 | NO_ITEMS | No distinct warranted item. |
| X:applying:necessity | applying | necessity | required practice input | 0 | NO_ITEMS | Practice inputs are present. |
| X:applying:sufficiency | applying | sufficiency | adequate execution evidence | 0 | NO_ITEMS | No distinct warranted item. |
| X:applying:completeness | applying | completeness | complete implementation record | 0 | NO_ITEMS | No implementation record exists yet by scope. |
| X:applying:consistency | applying | consistency | stable action rationale | 0 | NO_ITEMS | No distinct warranted item. |
| X:judging:necessity | judging | necessity | required ruling basis | 0 | NO_ITEMS | No automatic ruling is asserted. |
| X:judging:sufficiency | judging | sufficiency | adequate finding evidence | 0 | NO_ITEMS | No distinct warranted item. |
| X:judging:completeness | judging | completeness | complete decision record | 0 | NO_ITEMS | Human decisions remain TBD. |
| X:judging:consistency | judging | consistency | stable ruling rationale | 0 | NO_ITEMS | No distinct warranted item. |
| X:reviewing:necessity | reviewing | necessity | required audit input | 0 | NO_ITEMS | Audit inputs are listed. |
| X:reviewing:sufficiency | reviewing | sufficiency | adequate inspection evidence | 0 | NO_ITEMS | No distinct warranted item. |
| X:reviewing:completeness | reviewing | completeness | complete audit trail | 1 | HAS_ITEMS | Review command/evidence path remains unspecified. |
| X:reviewing:consistency | reviewing | consistency | stable audit rationale | 0 | NO_ITEMS | No distinct warranted item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| X-001 | X:reviewing:completeness | VerificationGap | Procedure | Procedure | Add the selected protected-content/private-data review command or evidence path once fixtures exist. | Procedure requires protected-content/private-data review checks, but no concrete command, fixture path, or evidence artifact exists in setup scope. | Procedure.md; Specification.md | Procedure.md#Verification; Specification.md#Verification | N/A | PROPOSAL: Link to the actual linter/review evidence in the implementation deliverable. | TBD |

## Matrix E - Evaluation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| E:guiding:data | guiding | data | verified fact direction | 0 | NO_ITEMS | Facts are cited. |
| E:guiding:information | guiding | information | coherent signal direction | 0 | NO_ITEMS | No distinct warranted item. |
| E:guiding:knowledge | guiding | knowledge | competent rationale direction | 0 | NO_ITEMS | Rationale is present at setup level. |
| E:guiding:wisdom | guiding | wisdom | principled judgment direction | 1 | HAS_ITEMS | Comparison-link semantics need future clarification. |
| E:applying:data | applying | data | verified practice fact | 0 | NO_ITEMS | No distinct warranted item. |
| E:applying:information | applying | information | coherent action signal | 0 | NO_ITEMS | No distinct warranted item. |
| E:applying:knowledge | applying | knowledge | competent execution rationale | 0 | NO_ITEMS | No distinct warranted item. |
| E:applying:wisdom | applying | wisdom | principled use judgment | 0 | NO_ITEMS | No distinct warranted item. |
| E:judging:data | judging | data | verified finding fact | 0 | NO_ITEMS | No distinct warranted item. |
| E:judging:information | judging | information | coherent ruling signal | 0 | NO_ITEMS | No automatic ruling is asserted. |
| E:judging:knowledge | judging | knowledge | competent decision rationale | 0 | NO_ITEMS | No distinct warranted item. |
| E:judging:wisdom | judging | wisdom | principled ruling judgment | 0 | NO_ITEMS | No distinct warranted item. |
| E:reviewing:data | reviewing | data | verified audit fact | 0 | NO_ITEMS | No distinct warranted item. |
| E:reviewing:information | reviewing | information | coherent inspection signal | 0 | NO_ITEMS | No distinct warranted item. |
| E:reviewing:knowledge | reviewing | knowledge | competent audit rationale | 0 | NO_ITEMS | No distinct warranted item. |
| E:reviewing:wisdom | reviewing | wisdom | principled audit judgment | 0 | NO_ITEMS | No distinct warranted item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| E-001 | E:guiding:wisdom | RationaleGap | Guidance | Guidance | Clarify whether comparison report links are references only or typed relations when the schema is selected. | Guidance says comparison reports may support workflows but are diagnostic/handoff support only; the later schema still needs a precise relationship model. | Guidance.md; Specification.md | Guidance.md#Considerations; Specification.md#Requirements | N/A | PROPOSAL: Treat links as references until a schema/API contract defines a typed relation. | TBD |
