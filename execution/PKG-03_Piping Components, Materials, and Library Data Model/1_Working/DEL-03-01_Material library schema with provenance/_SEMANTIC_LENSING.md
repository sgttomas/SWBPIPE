# Semantic Lensing Register: DEL-03-01 Material library schema with provenance

**Generated:** 2026-04-30
**Deliverable Folder:** /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-01_Material library schema with provenance
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

- Total warranted items: 14
- By document:
  - Datasheet: 2
  - Specification: 7
  - Guidance: 3
  - Procedure: 2
- By matrix:
  - A: 0  B: 2  C: 2  F: 4  D: 2  X: 2  E: 2
- By type:
  - Conflict: 0
  - VerificationGap: 5
  - MissingSlot: 5
  - WeakStatement: 0
  - RationaleGap: 1
  - Normalization: 1
  - TBD_Question: 2
  - MatrixError: 0
- Notable conflicts: 0
- Matrix parse errors: 0

## Matrix A - Orientation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| A:normative:guiding | normative | guiding | prescriptive direction | 0 | NO_ITEMS | Governing direction is present in Specification and Guidance. |
| A:normative:applying | normative | applying | mandatory practice | 0 | NO_ITEMS | Mandatory practices are captured under F and X. |
| A:normative:judging | normative | judging | compliance determination | 0 | NO_ITEMS | No certification/compliance claim is made. |
| A:normative:reviewing | normative | reviewing | regulatory audit | 0 | NO_ITEMS | Review concerns are captured under later lenses. |
| A:operative:guiding | operative | guiding | procedural direction | 0 | NO_ITEMS | Procedure gives setup direction. |
| A:operative:applying | operative | applying | practical execution | 0 | NO_ITEMS | Execution details are deferred and captured under F/D. |
| A:operative:judging | operative | judging | performance assessment | 0 | NO_ITEMS | No distinct performance conflict found. |
| A:operative:reviewing | operative | reviewing | process audit | 0 | NO_ITEMS | Process audit gaps are captured under D/X/E. |
| A:evaluative:guiding | evaluative | guiding | value orientation | 0 | NO_ITEMS | Guidance states data-boundary values. |
| A:evaluative:applying | evaluative | applying | merit application | 0 | NO_ITEMS | No distinct warranted item. |
| A:evaluative:judging | evaluative | judging | worth determination | 0 | NO_ITEMS | No separate value decision issue found. |
| A:evaluative:reviewing | evaluative | reviewing | quality appraisal | 0 | NO_ITEMS | Quality review is captured under X/E. |

## Matrix B - Conceptualization

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| B:data:necessity | data | necessity | essential fact | 0 | NO_ITEMS | Identity and scope facts are present. |
| B:data:sufficiency | data | sufficiency | adequate evidence | 0 | NO_ITEMS | Evidence basis is adequate for setup. |
| B:data:completeness | data | completeness | comprehensive record | 1 | HAS_ITEMS | Field inventory remains high-level. |
| B:data:consistency | data | consistency | reliable measurement | 0 | NO_ITEMS | Unit issues captured under F/X. |
| B:information:necessity | information | necessity | essential signal | 0 | NO_ITEMS | Required signals are present. |
| B:information:sufficiency | information | sufficiency | adequate context | 0 | NO_ITEMS | Context is adequate. |
| B:information:completeness | information | completeness | comprehensive account | 0 | NO_ITEMS | No additional account gap found. |
| B:information:consistency | information | consistency | coherent message | 1 | HAS_ITEMS | Terminology around allowables/properties needs normalization. |
| B:knowledge:necessity | knowledge | necessity | fundamental understanding | 0 | NO_ITEMS | Data-boundary understanding is present. |
| B:knowledge:sufficiency | knowledge | sufficiency | competent expertise | 0 | NO_ITEMS | Engineering details remain intentionally TBD. |
| B:knowledge:completeness | knowledge | completeness | thorough mastery | 0 | NO_ITEMS | No distinct mastery item. |
| B:knowledge:consistency | knowledge | consistency | coherent understanding | 0 | NO_ITEMS | Coherence issues captured elsewhere. |
| B:wisdom:necessity | wisdom | necessity | essential discernment | 0 | NO_ITEMS | Protected-content discernment is stated. |
| B:wisdom:sufficiency | wisdom | sufficiency | adequate judgment | 0 | NO_ITEMS | Human ruling needs are captured under D. |
| B:wisdom:completeness | wisdom | completeness | holistic insight | 0 | NO_ITEMS | No separate holistic insight gap found. |
| B:wisdom:consistency | wisdom | consistency | principled reasoning | 0 | NO_ITEMS | Reasoning is consistent. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| B-001 | B:data:completeness | MissingSlot | Datasheet | Specification | Add a field-level schema inventory for material identity, temperature-indexed property records, allowable slots, units, provenance, redistribution, completeness, and diagnostics. | The documents identify record groups but do not enumerate exact schema fields. | Datasheet.md; Specification.md | Datasheet `## Construction`; Specification `## Requirements` | N/A | Specification | TBD |
| B-002 | B:information:consistency | Normalization | Guidance | Guidance | Clarify vocabulary for material property, allowable slot, actual allowable value, private library, and public fixture. | Without canonical wording, future contributors may blur schema slots with bundled protected data. | Guidance.md; Specification.md | Guidance `## Principles`; Specification `## Scope` | N/A | Guidance | TBD |

## Matrix C - Formulation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| C:normative:necessity | normative | necessity | Binding Data Boundary | 0 | NO_ITEMS | Boundary is explicit. |
| C:normative:sufficiency | normative | sufficiency | Defensible Provenance Basis | 1 | HAS_ITEMS | Provenance minimum fields remain unresolved. |
| C:normative:completeness | normative | completeness | Complete Governance Record | 0 | NO_ITEMS | Governance records are described at setup level. |
| C:normative:consistency | normative | consistency | Controlled Provenance Logic | 0 | NO_ITEMS | No distinct logic conflict. |
| C:operative:necessity | operative | necessity | Executable Schema Basis | 0 | NO_ITEMS | Schema basis is stated. |
| C:operative:sufficiency | operative | sufficiency | Usable Validation Context | 0 | NO_ITEMS | Validation context captured under F. |
| C:operative:completeness | operative | completeness | Complete Schema Account | 1 | HAS_ITEMS | Material editor fixture strategy is incomplete. |
| C:operative:consistency | operative | consistency | Stable Validation Meaning | 0 | NO_ITEMS | No distinct consistency item. |
| C:evaluative:necessity | evaluative | necessity | Risk Discernment Basis | 0 | NO_ITEMS | Risk basis is stated. |
| C:evaluative:sufficiency | evaluative | sufficiency | Adequate Review Ground | 0 | NO_ITEMS | Review grounds captured under E. |
| C:evaluative:completeness | evaluative | completeness | Holistic Quality Account | 0 | NO_ITEMS | No distinct item. |
| C:evaluative:consistency | evaluative | consistency | Principled Review Logic | 0 | NO_ITEMS | No distinct item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| C-001 | C:normative:sufficiency | MissingSlot | Specification | Specification | Define minimum provenance fields per material value class and allowable slot. | The docs require provenance but leave exact minimum fields as TBD. | Specification.md; Datasheet.md | Specification `## Requirements`; Datasheet `## Construction` | N/A | Specification | TBD |
| C-002 | C:operative:completeness | TBD_Question | Guidance | Guidance | TBD: What fixture source policy permits public material editor fixtures without protected or proprietary data? | Fixtures are anticipated but exact public-safe source policy is not ruled. | Guidance.md; Procedure.md | Guidance `## Examples`; Procedure `## Steps` | N/A | Guidance | TBD |

## Matrix F - Requirements

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| F:normative:necessity | normative | necessity | Required Boundary Foundation | 0 | NO_ITEMS | Core boundary requirements are present. |
| F:normative:sufficiency | normative | sufficiency | Provenance Closure Standard | 1 | HAS_ITEMS | Closure criteria need acceptance detail. |
| F:normative:completeness | normative | completeness | Complete Boundary Record | 0 | NO_ITEMS | Boundary records are stated. |
| F:normative:consistency | normative | consistency | Stable Governance Coherence | 0 | NO_ITEMS | No conflict found. |
| F:operative:necessity | operative | necessity | Required Schema Capability | 1 | HAS_ITEMS | JSON Schema shape remains undeveloped. |
| F:operative:sufficiency | operative | sufficiency | Adequate Fixture Proof | 1 | HAS_ITEMS | Fixture expected outcomes are not defined. |
| F:operative:completeness | operative | completeness | Complete Field Coverage | 0 | NO_ITEMS | Field coverage gap already captured by B/C. |
| F:operative:consistency | operative | consistency | Deterministic Validation Behavior | 1 | HAS_ITEMS | Diagnostic taxonomy is high-level. |
| F:evaluative:necessity | evaluative | necessity | Required Review Basis | 0 | NO_ITEMS | Review basis is present. |
| F:evaluative:sufficiency | evaluative | sufficiency | Defensible Quarantine Ground | 0 | NO_ITEMS | Quarantine is noted; specifics captured under E. |
| F:evaluative:completeness | evaluative | completeness | Complete Review Evidence | 0 | NO_ITEMS | No distinct review evidence issue. |
| F:evaluative:consistency | evaluative | consistency | Coherent Gate Logic | 0 | NO_ITEMS | No distinct logic conflict. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| F-001 | F:normative:sufficiency | VerificationGap | Specification | Specification | Add acceptance criteria for provenance completeness and redistribution status closure. | Requirements name provenance and rights metadata, but not pass/fail closure criteria. | Specification.md | Specification `## Verification` | N/A | Specification | TBD |
| F-002 | F:operative:necessity | MissingSlot | Specification | Specification | Add schema object outlines for material record, property series, allowable series, provenance, redistribution, completeness, and diagnostics. | Anticipated artifact is a material schema, but setup docs only list groups. | Specification.md; Datasheet.md | Specification `## Documentation`; Datasheet `## Construction` | N/A | Specification | TBD |
| F-003 | F:operative:sufficiency | VerificationGap | Procedure | Procedure | Add fixture inventory and expected results for missing units, missing provenance, suspected protected content, and private-data warnings. | Procedure names checks but not concrete fixture inventory. | Procedure.md | Procedure `## Verification` | N/A | Procedure | TBD |
| F-004 | F:operative:consistency | MissingSlot | Specification | Specification | Add diagnostic code/class taxonomy for provenance, protected-content, unit, completeness, and privacy findings. | AB-00-06 requires structured diagnostics; local docs do not enumerate codes/classes. | Specification.md; Datasheet.md | Specification `## Requirements`; Datasheet `## Construction` | N/A | Specification | TBD |

## Matrix D - Objectives

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| D:normative:guiding | normative | guiding | Boundary Direction Closure | 0 | NO_ITEMS | Direction closure is present. |
| D:normative:applying | normative | applying | Mandatory Provenance Practice | 0 | NO_ITEMS | Practice is captured by requirements. |
| D:normative:judging | normative | judging | Boundary Decision Closure | 0 | NO_ITEMS | No conflict found. |
| D:normative:reviewing | normative | reviewing | Governance Review Closure | 1 | HAS_ITEMS | Human review owner is not named. |
| D:operative:guiding | operative | guiding | Schema Direction Closure | 0 | NO_ITEMS | Direction is present. |
| D:operative:applying | operative | applying | Validation Execution Closure | 0 | NO_ITEMS | Execution details captured under F/X. |
| D:operative:judging | operative | judging | Field Assessment Closure | 0 | NO_ITEMS | No distinct item. |
| D:operative:reviewing | operative | reviewing | Validation Trace Closure | 1 | HAS_ITEMS | Review record path remains TBD. |
| D:evaluative:guiding | evaluative | guiding | Review Direction Closure | 0 | NO_ITEMS | Review direction is present. |
| D:evaluative:applying | evaluative | applying | Quarantine Practice Closure | 0 | NO_ITEMS | Quarantine captured under E. |
| D:evaluative:judging | evaluative | judging | Quality Decision Closure | 0 | NO_ITEMS | No separate item. |
| D:evaluative:reviewing | evaluative | reviewing | Gate Appraisal Closure | 0 | NO_ITEMS | No separate item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| D-001 | D:normative:reviewing | TBD_Question | Procedure | Guidance | TBD: Who owns protected-content and public fixture acceptance rulings for material libraries? | The docs require human review but do not identify the ruling authority. | Procedure.md; Guidance.md | Procedure `## Records`; Guidance `## Principles` | N/A | Guidance | TBD |
| D-002 | D:operative:reviewing | MissingSlot | Procedure | Procedure | Add record path for schema review notes, fixture provenance review, protected-content disposition, and human rulings. | Procedure lists records but leaves their locations TBD. | Procedure.md | Procedure `## Records` | N/A | Procedure | TBD |

## Matrix X - Verification

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| X:guiding:necessity | guiding | necessity | Directive Boundary Basis | 0 | NO_ITEMS | Directive basis is present. |
| X:guiding:sufficiency | guiding | sufficiency | Contextual Boundary Proof | 0 | NO_ITEMS | No distinct item. |
| X:guiding:completeness | guiding | completeness | Comprehensive Direction Record | 0 | NO_ITEMS | Direction record present at setup level. |
| X:guiding:consistency | guiding | consistency | Stable Direction Meaning | 0 | NO_ITEMS | No distinct item. |
| X:applying:necessity | applying | necessity | Practice Readiness Basis | 0 | NO_ITEMS | Readiness basis present. |
| X:applying:sufficiency | applying | sufficiency | Executable Provenance Proof | 1 | HAS_ITEMS | Executable checks need expected results. |
| X:applying:completeness | applying | completeness | Complete Practice Record | 0 | NO_ITEMS | Captured under F/D. |
| X:applying:consistency | applying | consistency | Stable Practice Meaning | 0 | NO_ITEMS | No distinct item. |
| X:judging:necessity | judging | necessity | Decision Evidence Basis | 0 | NO_ITEMS | Decision evidence basis present. |
| X:judging:sufficiency | judging | sufficiency | Assessment Proof Ground | 0 | NO_ITEMS | No distinct item. |
| X:judging:completeness | judging | completeness | Complete Decision Record | 0 | NO_ITEMS | No distinct item. |
| X:judging:consistency | judging | consistency | Coherent Decision Rationale | 0 | NO_ITEMS | No distinct item. |
| X:reviewing:necessity | reviewing | necessity | Review Evidence Basis | 0 | NO_ITEMS | Review evidence basis present. |
| X:reviewing:sufficiency | reviewing | sufficiency | Audit Proof Ground | 1 | HAS_ITEMS | Protected-content gate criteria need detail. |
| X:reviewing:completeness | reviewing | completeness | Complete Trace Record | 0 | NO_ITEMS | Trace gap captured under D. |
| X:reviewing:consistency | reviewing | consistency | Coherent Audit Rationale | 0 | NO_ITEMS | No distinct item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| X-001 | X:applying:sufficiency | VerificationGap | Procedure | Procedure | Add expected pass/fail outcomes for each negative validation fixture category. | Procedure lists check categories but not expected observable outcomes. | Procedure.md | Procedure `## Verification` | N/A | Procedure | TBD |
| X-002 | X:reviewing:sufficiency | VerificationGap | Specification | Specification | Define protected-content gate acceptance criteria for suspected material tables, source notes, and public fixture contributions. | Gate existence is clear, but acceptance criteria remain implicit. | Specification.md; Guidance.md | Specification `## Verification`; Guidance `## Trade-offs` | N/A | Specification | TBD |

## Matrix E - Evaluation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| E:guiding:data | guiding | data | Directive Fact Trace | 0 | NO_ITEMS | Directive facts present. |
| E:guiding:information | guiding | information | Directive Context Signal | 0 | NO_ITEMS | No distinct item. |
| E:guiding:knowledge | guiding | knowledge | Directive Expertise Frame | 0 | NO_ITEMS | No distinct item. |
| E:guiding:wisdom | guiding | wisdom | Directive Discernment Frame | 1 | HAS_ITEMS | Human legal/rights judgment criteria are open. |
| E:applying:data | applying | data | Practice Fact Trace | 0 | NO_ITEMS | No distinct item. |
| E:applying:information | applying | information | Provenance Context Signal | 0 | NO_ITEMS | Provenance context captured under C/F. |
| E:applying:knowledge | applying | knowledge | Schema Expertise Frame | 1 | HAS_ITEMS | Schema/fixture examples need rationale. |
| E:applying:wisdom | applying | wisdom | Practice Discernment Frame | 0 | NO_ITEMS | No distinct item. |
| E:judging:data | judging | data | Decision Fact Trace | 0 | NO_ITEMS | Decision facts captured. |
| E:judging:information | judging | information | Assessment Context Signal | 0 | NO_ITEMS | No distinct item. |
| E:judging:knowledge | judging | knowledge | Decision Expertise Frame | 0 | NO_ITEMS | No distinct item. |
| E:judging:wisdom | judging | wisdom | Assessment Discernment Frame | 0 | NO_ITEMS | No distinct item. |
| E:reviewing:data | reviewing | data | Audit Fact Trace | 0 | NO_ITEMS | Audit facts are present. |
| E:reviewing:information | reviewing | information | Audit Context Signal | 0 | NO_ITEMS | No distinct item. |
| E:reviewing:knowledge | reviewing | knowledge | Audit Expertise Frame | 0 | NO_ITEMS | No distinct item. |
| E:reviewing:wisdom | reviewing | wisdom | Audit Discernment Frame | 0 | NO_ITEMS | No distinct item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| E-001 | E:guiding:wisdom | RationaleGap | Guidance | Guidance | Explain decision criteria for public fixture acceptance versus quarantine/escalation. | The docs identify stop rules but do not explain how to choose among accept, reject, quarantine, or private-only handling. | Guidance.md; Procedure.md | Guidance `## Principles`; Procedure `## Steps` | N/A | Guidance | TBD |
| E-002 | E:applying:knowledge | VerificationGap | Specification | Specification | Add example schema fragments using invented placeholder values only, or defer examples explicitly. | Future implementers need examples, but examples risk implying engineering data unless constrained. | Specification.md; Guidance.md | Specification `## Documentation`; Guidance `## Examples` | N/A | Specification | TBD |

