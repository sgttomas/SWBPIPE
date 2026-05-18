# Semantic Lensing Register: DEL-17-02 Export package, profile, and stable ID map contracts

**Generated:** 2026-05-18
**Deliverable Folder:** /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-02_Export package, profile, and stable ID map contracts
**Warnings:** none

**Inputs Read:**
- _CONTEXT.md - SourceRef: _CONTEXT.md#Context: DEL-17-02
- _STATUS.md - SourceRef: _STATUS.md#Status: DEL-17-02
- _SEMANTIC.md - SourceRef: _SEMANTIC.md#Semantic Matrix: DEL-17-02 Export package, profile, and stable ID map contracts
- Datasheet.md - SourceRef: Datasheet.md#Datasheet: DEL-17-02 Export package, profile, and stable ID map contracts
- Specification.md - SourceRef: Specification.md#Specification: DEL-17-02 Export package, profile, and stable ID map contracts
- Guidance.md - SourceRef: Guidance.md#Guidance: DEL-17-02 Export package, profile, and stable ID map contracts
- Procedure.md - SourceRef: Procedure.md#Procedure: DEL-17-02 Export package, profile, and stable ID map contracts
- _REFERENCES.md - SourceRef: _REFERENCES.md#References: DEL-17-02 Export package, profile, and stable ID map contracts

**Purpose:** Apply `semantic-matrix-build` matrix cells as lenses over the production documents, capturing warranted enrichment inputs for a later enrichment pass.

## Summary

- Total warranted items: 4
- By document:
  - Datasheet: 1
  - Specification: 1
  - Guidance: 1
  - Procedure: 1
- By matrix:
  - A: 1  B: 1  C: 0  F: 0  D: 0  X: 1  E: 1
- By type:
  - Conflict: 0
  - VerificationGap: 1
  - MissingSlot: 2
  - WeakStatement: 0
  - RationaleGap: 1
  - Normalization: 0
  - TBD_Question: 0
  - MatrixError: 0
- Notable conflicts: 0
- Matrix parse errors: 0

## Matrix A - Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---|---|---|
| A:[normative]:[guiding] | normative | guiding | prescriptive direction | 1 | HAS_ITEMS | prescriptive direction exposed a specific candidate tied to Specification.md. |
| A:[normative]:[applying] | normative | applying | mandatory practice | 0 | NO_ITEMS | mandatory practice checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for A-normative-applying. |
| A:[normative]:[judging] | normative | judging | compliance determination | 0 | NO_ITEMS | compliance determination checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for A-normative-judging. |
| A:[normative]:[reviewing] | normative | reviewing | regulatory audit | 0 | NO_ITEMS | regulatory audit checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for A-normative-reviewing. |
| A:[operative]:[guiding] | operative | guiding | procedural direction | 0 | NO_ITEMS | procedural direction checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for A-operative-guiding. |
| A:[operative]:[applying] | operative | applying | practical execution | 0 | NO_ITEMS | practical execution checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for A-operative-applying. |
| A:[operative]:[judging] | operative | judging | performance assessment | 0 | NO_ITEMS | performance assessment checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for A-operative-judging. |
| A:[operative]:[reviewing] | operative | reviewing | process audit | 0 | NO_ITEMS | process audit checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for A-operative-reviewing. |
| A:[evaluative]:[guiding] | evaluative | guiding | value orientation | 0 | NO_ITEMS | value orientation checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for A-evaluative-guiding. |
| A:[evaluative]:[applying] | evaluative | applying | merit application | 0 | NO_ITEMS | merit application checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for A-evaluative-applying. |
| A:[evaluative]:[judging] | evaluative | judging | worth determination | 0 | NO_ITEMS | worth determination checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for A-evaluative-judging. |
| A:[evaluative]:[reviewing] | evaluative | reviewing | quality appraisal | 0 | NO_ITEMS | quality appraisal checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for A-evaluative-reviewing. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| A-001 | A:[normative]:[guiding] | MissingSlot | Specification.md | Specification.md | Add architecture-basis requirements for JSON Schema 2020-12 contracts and canonical JSON/JCS-compatible hash basis as constraints on future concrete schemas. | _CONTEXT.md records these architecture basis constraints; Specification.md has source authority and package hash requirements but not the schema-version/hash-basis contract as an explicit requirement. | _CONTEXT.md; Specification.md | _CONTEXT.md#Architecture Basis Injection; Specification.md#Source Authority Requirements | NA | PROPOSAL - project architecture basis controls future schema/profile implementations | TBD |

## Matrix B - Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---|---|---|
| B:[data]:[necessity] | data | necessity | essential fact | 0 | NO_ITEMS | essential fact checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for B-data-necessity. |
| B:[data]:[sufficiency] | data | sufficiency | adequate evidence | 0 | NO_ITEMS | adequate evidence checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for B-data-sufficiency. |
| B:[data]:[completeness] | data | completeness | comprehensive record | 1 | HAS_ITEMS | comprehensive record exposed a specific candidate tied to Datasheet.md. |
| B:[data]:[consistency] | data | consistency | reliable measurement | 0 | NO_ITEMS | reliable measurement checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for B-data-consistency. |
| B:[information]:[necessity] | information | necessity | essential signal | 0 | NO_ITEMS | essential signal checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for B-information-necessity. |
| B:[information]:[sufficiency] | information | sufficiency | adequate context | 0 | NO_ITEMS | adequate context checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for B-information-sufficiency. |
| B:[information]:[completeness] | information | completeness | comprehensive account | 0 | NO_ITEMS | comprehensive account checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for B-information-completeness. |
| B:[information]:[consistency] | information | consistency | coherent message | 0 | NO_ITEMS | coherent message checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for B-information-consistency. |
| B:[knowledge]:[necessity] | knowledge | necessity | fundamental understanding | 0 | NO_ITEMS | fundamental understanding checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for B-knowledge-necessity. |
| B:[knowledge]:[sufficiency] | knowledge | sufficiency | competent expertise | 0 | NO_ITEMS | competent expertise checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for B-knowledge-sufficiency. |
| B:[knowledge]:[completeness] | knowledge | completeness | thorough mastery | 0 | NO_ITEMS | thorough mastery checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for B-knowledge-completeness. |
| B:[knowledge]:[consistency] | knowledge | consistency | coherent understanding | 0 | NO_ITEMS | coherent understanding checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for B-knowledge-consistency. |
| B:[wisdom]:[necessity] | wisdom | necessity | essential discernment | 0 | NO_ITEMS | essential discernment checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for B-wisdom-necessity. |
| B:[wisdom]:[sufficiency] | wisdom | sufficiency | adequate judgment | 0 | NO_ITEMS | adequate judgment checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for B-wisdom-sufficiency. |
| B:[wisdom]:[completeness] | wisdom | completeness | holistic insight | 0 | NO_ITEMS | holistic insight checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for B-wisdom-completeness. |
| B:[wisdom]:[consistency] | wisdom | consistency | principled reasoning | 0 | NO_ITEMS | principled reasoning checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for B-wisdom-consistency. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| B-001 | B:[data]:[completeness] | MissingSlot | Datasheet.md | Datasheet.md | Add a local contract artifact inventory that names the four documents, semantic artifacts, Dependencies.csv, and run records as the evidence package for this contract deliverable. | Datasheet.md describes contract objects and sources but does not enumerate the local artifact set that makes the deliverable auditable. | Datasheet.md; _DEPENDENCIES.md | Datasheet.md#Contract Objects; _DEPENDENCIES.md#Extracted Dependency Register | NA | PROPOSAL - descriptive inventory only | TBD |

## Matrix C - Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---|---|---|
| C:[normative]:[necessity] | normative | necessity | contract basis | 0 | NO_ITEMS | contract basis checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for C-normative-necessity. |
| C:[normative]:[sufficiency] | normative | sufficiency | evidence threshold | 0 | NO_ITEMS | evidence threshold checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for C-normative-sufficiency. |
| C:[normative]:[completeness] | normative | completeness | scope closure | 0 | NO_ITEMS | scope closure checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for C-normative-completeness. |
| C:[normative]:[consistency] | normative | consistency | alignment control | 0 | NO_ITEMS | alignment control checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for C-normative-consistency. |
| C:[operative]:[necessity] | operative | necessity | execution prerequisite | 0 | NO_ITEMS | execution prerequisite checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for C-operative-necessity. |
| C:[operative]:[sufficiency] | operative | sufficiency | implementation proof | 0 | NO_ITEMS | implementation proof checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for C-operative-sufficiency. |
| C:[operative]:[completeness] | operative | completeness | artifact closure | 0 | NO_ITEMS | artifact closure checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for C-operative-completeness. |
| C:[operative]:[consistency] | operative | consistency | workflow coherence | 0 | NO_ITEMS | workflow coherence checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for C-operative-consistency. |
| C:[evaluative]:[necessity] | evaluative | necessity | boundary rationale | 0 | NO_ITEMS | boundary rationale checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for C-evaluative-necessity. |
| C:[evaluative]:[sufficiency] | evaluative | sufficiency | review confidence | 0 | NO_ITEMS | review confidence checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for C-evaluative-sufficiency. |
| C:[evaluative]:[completeness] | evaluative | completeness | coverage assurance | 0 | NO_ITEMS | coverage assurance checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for C-evaluative-completeness. |
| C:[evaluative]:[consistency] | evaluative | consistency | quality alignment | 0 | NO_ITEMS | quality alignment checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for C-evaluative-consistency. |

## Matrix F - Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---|---|---|
| F:[normative]:[necessity] | normative | necessity | authority gate | 0 | NO_ITEMS | authority gate checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for F-normative-necessity. |
| F:[normative]:[sufficiency] | normative | sufficiency | source adequacy | 0 | NO_ITEMS | source adequacy checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for F-normative-sufficiency. |
| F:[normative]:[completeness] | normative | completeness | contract closure | 0 | NO_ITEMS | contract closure checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for F-normative-completeness. |
| F:[normative]:[consistency] | normative | consistency | governance alignment | 0 | NO_ITEMS | governance alignment checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for F-normative-consistency. |
| F:[operative]:[necessity] | operative | necessity | readiness gate | 0 | NO_ITEMS | readiness gate checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for F-operative-necessity. |
| F:[operative]:[sufficiency] | operative | sufficiency | implementation evidence | 0 | NO_ITEMS | implementation evidence checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for F-operative-sufficiency. |
| F:[operative]:[completeness] | operative | completeness | package completion | 0 | NO_ITEMS | package completion checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for F-operative-completeness. |
| F:[operative]:[consistency] | operative | consistency | process coherence | 0 | NO_ITEMS | process coherence checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for F-operative-consistency. |
| F:[evaluative]:[necessity] | evaluative | necessity | review basis | 0 | NO_ITEMS | review basis checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for F-evaluative-necessity. |
| F:[evaluative]:[sufficiency] | evaluative | sufficiency | decision confidence | 0 | NO_ITEMS | decision confidence checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for F-evaluative-sufficiency. |
| F:[evaluative]:[completeness] | evaluative | completeness | assurance closure | 0 | NO_ITEMS | assurance closure checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for F-evaluative-completeness. |
| F:[evaluative]:[consistency] | evaluative | consistency | fitness coherence | 0 | NO_ITEMS | fitness coherence checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for F-evaluative-consistency. |

## Matrix D - Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---|---|---|
| D:[normative]:[guiding] | normative | guiding | controlled contract direction | 0 | NO_ITEMS | controlled contract direction checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for D-normative-guiding. |
| D:[normative]:[applying] | normative | applying | mandatory profile practice | 0 | NO_ITEMS | mandatory profile practice checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for D-normative-applying. |
| D:[normative]:[judging] | normative | judging | boundary decision basis | 0 | NO_ITEMS | boundary decision basis checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for D-normative-judging. |
| D:[normative]:[reviewing] | normative | reviewing | audit-ready governance | 0 | NO_ITEMS | audit-ready governance checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for D-normative-reviewing. |
| D:[operative]:[guiding] | operative | guiding | procedure planning basis | 0 | NO_ITEMS | procedure planning basis checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for D-operative-guiding. |
| D:[operative]:[applying] | operative | applying | implementation work package | 0 | NO_ITEMS | implementation work package checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for D-operative-applying. |
| D:[operative]:[judging] | operative | judging | evidence assessment basis | 0 | NO_ITEMS | evidence assessment basis checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for D-operative-judging. |
| D:[operative]:[reviewing] | operative | reviewing | workflow audit trail | 0 | NO_ITEMS | workflow audit trail checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for D-operative-reviewing. |
| D:[evaluative]:[guiding] | evaluative | guiding | rationale direction basis | 0 | NO_ITEMS | rationale direction basis checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for D-evaluative-guiding. |
| D:[evaluative]:[applying] | evaluative | applying | merit application basis | 0 | NO_ITEMS | merit application basis checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for D-evaluative-applying. |
| D:[evaluative]:[judging] | evaluative | judging | acceptance decision basis | 0 | NO_ITEMS | acceptance decision basis checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for D-evaluative-judging. |
| D:[evaluative]:[reviewing] | evaluative | reviewing | quality appraisal basis | 0 | NO_ITEMS | quality appraisal basis checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for D-evaluative-reviewing. |

## Matrix X - Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---|---|---|
| X:[guiding]:[necessity] | guiding | necessity | traceable contract basis | 0 | NO_ITEMS | traceable contract basis checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for X-guiding-necessity. |
| X:[guiding]:[sufficiency] | guiding | sufficiency | adequate source package | 0 | NO_ITEMS | adequate source package checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for X-guiding-sufficiency. |
| X:[guiding]:[completeness] | guiding | completeness | complete scope record | 0 | NO_ITEMS | complete scope record checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for X-guiding-completeness. |
| X:[guiding]:[consistency] | guiding | consistency | aligned boundary message | 0 | NO_ITEMS | aligned boundary message checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for X-guiding-consistency. |
| X:[applying]:[necessity] | applying | necessity | executable readiness proof | 0 | NO_ITEMS | executable readiness proof checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for X-applying-necessity. |
| X:[applying]:[sufficiency] | applying | sufficiency | sufficient artifact evidence | 0 | NO_ITEMS | sufficient artifact evidence checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for X-applying-sufficiency. |
| X:[applying]:[completeness] | applying | completeness | complete work package | 0 | NO_ITEMS | complete work package checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for X-applying-completeness. |
| X:[applying]:[consistency] | applying | consistency | coherent process package | 0 | NO_ITEMS | coherent process package checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for X-applying-consistency. |
| X:[judging]:[necessity] | judging | necessity | decision evidence basis | 0 | NO_ITEMS | decision evidence basis checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for X-judging-necessity. |
| X:[judging]:[sufficiency] | judging | sufficiency | assessment confidence package | 0 | NO_ITEMS | assessment confidence package checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for X-judging-sufficiency. |
| X:[judging]:[completeness] | judging | completeness | complete acceptance record | 0 | NO_ITEMS | complete acceptance record checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for X-judging-completeness. |
| X:[judging]:[consistency] | judging | consistency | coherent ruling basis | 0 | NO_ITEMS | coherent ruling basis checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for X-judging-consistency. |
| X:[reviewing]:[necessity] | reviewing | necessity | audit evidence basis | 0 | NO_ITEMS | audit evidence basis checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for X-reviewing-necessity. |
| X:[reviewing]:[sufficiency] | reviewing | sufficiency | review confidence package | 0 | NO_ITEMS | review confidence package checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for X-reviewing-sufficiency. |
| X:[reviewing]:[completeness] | reviewing | completeness | complete audit trail | 0 | NO_ITEMS | complete audit trail checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for X-reviewing-completeness. |
| X:[reviewing]:[consistency] | reviewing | consistency | consistent assurance record | 1 | HAS_ITEMS | consistent assurance record exposed a specific candidate tied to Procedure.md. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| X-001 | X:[reviewing]:[consistency] | VerificationGap | Procedure.md | Procedure.md | Add semantic matrix and lens-register validators to the validation command list and closeout checklist. | Procedure.md requires semantic artifacts in closeout but its validation command list does not include the semantic or lens-register validators. | Procedure.md | Procedure.md#Validation Commands; Procedure.md#Closeout Procedure | NA | PROPOSAL - local closeout validation completeness | TBD |

## Matrix E - Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---|---|---|
| E:[guiding]:[data] | guiding | data | source trace record | 0 | NO_ITEMS | source trace record checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for E-guiding-data. |
| E:[guiding]:[information] | guiding | information | context direction package | 0 | NO_ITEMS | context direction package checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for E-guiding-information. |
| E:[guiding]:[knowledge] | guiding | knowledge | expert contract basis | 0 | NO_ITEMS | expert contract basis checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for E-guiding-knowledge. |
| E:[guiding]:[wisdom] | guiding | wisdom | principled boundary rationale | 0 | NO_ITEMS | principled boundary rationale checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for E-guiding-wisdom. |
| E:[applying]:[data] | applying | data | artifact execution record | 0 | NO_ITEMS | artifact execution record checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for E-applying-data. |
| E:[applying]:[information] | applying | information | context work package | 0 | NO_ITEMS | context work package checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for E-applying-information. |
| E:[applying]:[knowledge] | applying | knowledge | expert implementation basis | 0 | NO_ITEMS | expert implementation basis checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for E-applying-knowledge. |
| E:[applying]:[wisdom] | applying | wisdom | judgment execution rationale | 0 | NO_ITEMS | judgment execution rationale checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for E-applying-wisdom. |
| E:[judging]:[data] | judging | data | acceptance fact record | 0 | NO_ITEMS | acceptance fact record checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for E-judging-data. |
| E:[judging]:[information] | judging | information | context assessment package | 0 | NO_ITEMS | context assessment package checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for E-judging-information. |
| E:[judging]:[knowledge] | judging | knowledge | expert decision basis | 0 | NO_ITEMS | expert decision basis checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for E-judging-knowledge. |
| E:[judging]:[wisdom] | judging | wisdom | principled ruling rationale | 0 | NO_ITEMS | principled ruling rationale checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for E-judging-wisdom. |
| E:[reviewing]:[data] | reviewing | data | audit fact record | 0 | NO_ITEMS | audit fact record checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for E-reviewing-data. |
| E:[reviewing]:[information] | reviewing | information | context audit package | 0 | NO_ITEMS | context audit package checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for E-reviewing-information. |
| E:[reviewing]:[knowledge] | reviewing | knowledge | expert assurance basis | 1 | HAS_ITEMS | expert assurance basis exposed a specific candidate tied to Guidance.md. |
| E:[reviewing]:[wisdom] | reviewing | wisdom | principled audit rationale | 0 | NO_ITEMS | principled audit rationale checked across Datasheet, Specification, Guidance, and Procedure; contract treatment stays stable for E-reviewing-wisdom. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| E-001 | E:[reviewing]:[knowledge] | RationaleGap | Guidance.md | Guidance.md | Add a reviewer-checklist prompt that asks whether semantic matrix and lens-register validation passed before downstream consumption. | Guidance.md has reviewer prompts for target statements, TBDs, identity, loss reporting, and consumers, but not for the semantic/lensing readiness evidence that this tranche relies on. | Guidance.md; Procedure.md | Guidance.md#Reviewer Checklist; Procedure.md#Semantic Enrichment Verification | NA | PROPOSAL - checklist aid only | TBD |
