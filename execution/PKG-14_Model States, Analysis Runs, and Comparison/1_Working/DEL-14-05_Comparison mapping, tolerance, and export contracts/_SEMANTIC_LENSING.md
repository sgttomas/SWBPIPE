# Semantic Lensing Register: DEL-14-05 Comparison mapping, tolerance, and export contracts

**Generated:** 2026-05-04
**Deliverable Folder:** execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-05_Comparison mapping, tolerance, and export contracts
**Warnings:** none

**Inputs Read:**
- _CONTEXT.md - deliverable identity and scope
- _STATUS.md - current lifecycle state
- _SEMANTIC.md - semantic matrices
- Datasheet.md - present
- Specification.md - present
- Guidance.md - present
- Procedure.md - present
- _REFERENCES.md - reference pointers only

**Purpose:** Apply `semantic-matrix-build` matrix cells as lenses over the production documents, capturing warranted enrichment inputs for a later enrichment pass.

## Summary

- Total warranted items: 8
- By document:
  - Datasheet: 2
  - Specification: 4
  - Guidance: 1
  - Procedure: 1
- By matrix:
  - A: 1  B: 1  C: 0  F: 2  D: 1  X: 1  E: 2
- By type:
  - Conflict: 0
  - VerificationGap: 1
  - MissingSlot: 5
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
| A:normative:guiding | normative | guiding | prescriptive direction | 1 | HAS_ITEMS | Contract schema prescription remains incomplete. |
| A:normative:applying | normative | applying | mandatory practice | 0 | NO_ITEMS | No additional warranted item. |
| A:normative:judging | normative | judging | compliance determination | 0 | NO_ITEMS | Professional-boundary language is present. |
| A:normative:reviewing | normative | reviewing | regulatory audit | 0 | NO_ITEMS | Audit/provenance boundaries are present. |
| A:operative:guiding | operative | guiding | procedural direction | 0 | NO_ITEMS | Procedure supplies a bounded setup path. |
| A:operative:applying | operative | applying | practical execution | 0 | NO_ITEMS | No implementation execution is claimed. |
| A:operative:judging | operative | judging | performance assessment | 0 | NO_ITEMS | Verification table records TBD status. |
| A:operative:reviewing | operative | reviewing | process audit | 0 | NO_ITEMS | Dependency mirror preservation is explicit. |
| A:evaluative:guiding | evaluative | guiding | value orientation | 0 | NO_ITEMS | Guidance states governing principles. |
| A:evaluative:applying | evaluative | applying | merit application | 0 | NO_ITEMS | No additional warranted item. |
| A:evaluative:judging | evaluative | judging | worth determination | 0 | NO_ITEMS | No additional warranted item. |
| A:evaluative:reviewing | evaluative | reviewing | quality appraisal | 0 | NO_ITEMS | No additional warranted item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| A-001 | A:normative:guiding | MissingSlot | Specification | Specification | Add the deliverable-specific contract schema body once field names are sourced. | The specification names required contract surfaces but keeps exact schema fields TBD, so a later enrichment pass needs the missing normative schema body. | Specification.md; Datasheet.md | Specification.md#Scope; Specification.md#Requirements; Datasheet.md#Construction |  | PROPOSAL: wait for sourced field definitions before drafting exact schema properties. | TBD |

## Matrix B - Conceptualization

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| B:data:necessity | data | necessity | essential fact | 0 | NO_ITEMS | Required identity facts are present. |
| B:data:sufficiency | data | sufficiency | adequate evidence | 0 | NO_ITEMS | Source citations are present. |
| B:data:completeness | data | completeness | comprehensive record | 1 | HAS_ITEMS | CSV field record is incomplete. |
| B:data:consistency | data | consistency | reliable measurement | 0 | NO_ITEMS | Unit boundary is present. |
| B:information:necessity | information | necessity | essential signal | 0 | NO_ITEMS | Scope signal is present. |
| B:information:sufficiency | information | sufficiency | adequate context | 0 | NO_ITEMS | Context envelope is present. |
| B:information:completeness | information | completeness | comprehensive account | 0 | NO_ITEMS | Missing details already marked TBD. |
| B:information:consistency | information | consistency | coherent message | 0 | NO_ITEMS | Terminology is consistent. |
| B:knowledge:necessity | knowledge | necessity | fundamental understanding | 0 | NO_ITEMS | Professional boundary is stated. |
| B:knowledge:sufficiency | knowledge | sufficiency | competent expertise | 0 | NO_ITEMS | No expertise claim is introduced. |
| B:knowledge:completeness | knowledge | completeness | thorough mastery | 0 | NO_ITEMS | Full mastery is not claimed. |
| B:knowledge:consistency | knowledge | consistency | coherent understanding | 0 | NO_ITEMS | No additional warranted item. |
| B:wisdom:necessity | wisdom | necessity | essential discernment | 0 | NO_ITEMS | Human authority boundaries are present. |
| B:wisdom:sufficiency | wisdom | sufficiency | adequate judgment | 0 | NO_ITEMS | No additional warranted item. |
| B:wisdom:completeness | wisdom | completeness | holistic insight | 0 | NO_ITEMS | No additional warranted item. |
| B:wisdom:consistency | wisdom | consistency | principled reasoning | 0 | NO_ITEMS | No additional warranted item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| B-001 | B:data:completeness | MissingSlot | Datasheet | Datasheet | Record TBD: CSV column set, ordering, and serialization rules. | CSV export semantics are assigned to the deliverable, but the documents correctly do not invent column details. The missing slot should remain tracked. | Datasheet.md; Specification.md | Datasheet.md#Conditions; Specification.md#Requirements |  | PROPOSAL: define columns only after the comparison result shape is sourced. | TBD |

## Matrix C - Formulation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| C:normative:necessity | normative | necessity | binding evidence basis | 0 | NO_ITEMS | Evidence basis is cited. |
| C:normative:sufficiency | normative | sufficiency | adequate compliance support | 0 | NO_ITEMS | Compliance claims are avoided. |
| C:normative:completeness | normative | completeness | full obligation record | 0 | NO_ITEMS | Open obligations are marked TBD. |
| C:normative:consistency | normative | consistency | coherent control basis | 0 | NO_ITEMS | Controls align across docs. |
| C:operative:necessity | operative | necessity | required execution inputs | 0 | NO_ITEMS | Prerequisites are recorded. |
| C:operative:sufficiency | operative | sufficiency | usable workflow context | 0 | NO_ITEMS | Setup procedure is bounded. |
| C:operative:completeness | operative | completeness | complete action record | 0 | NO_ITEMS | Procedure lists setup steps. |
| C:operative:consistency | operative | consistency | repeatable process signal | 0 | NO_ITEMS | No process contradiction found. |
| C:evaluative:necessity | evaluative | necessity | essential review criteria | 0 | NO_ITEMS | Review criteria are TBD where unsupported. |
| C:evaluative:sufficiency | evaluative | sufficiency | adequate judgment basis | 0 | NO_ITEMS | Human judgment boundary is present. |
| C:evaluative:completeness | evaluative | completeness | complete appraisal record | 0 | NO_ITEMS | No additional warranted item. |
| C:evaluative:consistency | evaluative | consistency | coherent quality rationale | 0 | NO_ITEMS | No additional warranted item. |

## Matrix F - Requirements

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| F:normative:necessity | normative | necessity | mandatory contract preconditions | 1 | HAS_ITEMS | Tolerance decision remains unresolved. |
| F:normative:sufficiency | normative | sufficiency | sufficient obligation evidence | 0 | NO_ITEMS | Source citations are present. |
| F:normative:completeness | normative | completeness | complete requirement coverage | 1 | HAS_ITEMS | Unmatched enum details remain absent. |
| F:normative:consistency | normative | consistency | consistent control rule | 0 | NO_ITEMS | No control conflict found. |
| F:operative:necessity | operative | necessity | required workflow inputs | 0 | NO_ITEMS | Inputs are summarized. |
| F:operative:sufficiency | operative | sufficiency | sufficient execution evidence | 0 | NO_ITEMS | Execution evidence remains future work. |
| F:operative:completeness | operative | completeness | complete procedure coverage | 0 | NO_ITEMS | Procedure is complete for setup pass. |
| F:operative:consistency | operative | consistency | repeatable workflow rule | 0 | NO_ITEMS | No additional warranted item. |
| F:evaluative:necessity | evaluative | necessity | required review criteria | 0 | NO_ITEMS | Review criteria are future validation work. |
| F:evaluative:sufficiency | evaluative | sufficiency | sufficient appraisal evidence | 0 | NO_ITEMS | No additional warranted item. |
| F:evaluative:completeness | evaluative | completeness | complete review coverage | 0 | NO_ITEMS | No additional warranted item. |
| F:evaluative:consistency | evaluative | consistency | consistent judgment rule | 0 | NO_ITEMS | No additional warranted item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| F-001 | F:normative:necessity | TBD_Question | Specification | Specification | Resolve TBD: comparison tolerance defaults and mapping workflow authority. | The specification preserves OI-014, but implementation cannot freeze tolerance profile defaults or mapping workflow rules until a governing source or human ruling exists. | Specification.md; Guidance.md | Specification.md#Requirements; Guidance.md#Principles |  | PROPOSAL: require solver/result schema prototype evidence or human product decision. | TBD |
| F-002 | F:normative:completeness | MissingSlot | Specification | Specification | Define unmatched classification enum values or keep an explicit extension slot. | The contract includes unmatched classifications, but the accessible sources do not supply the enum set. | Specification.md; Datasheet.md | Specification.md#Requirements; Datasheet.md#Construction |  | PROPOSAL: do not invent enums during setup; source or approve them later. | TBD |

## Matrix D - Objectives

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| D:normative:guiding | normative | guiding | binding contract direction | 0 | NO_ITEMS | Contract direction is explicit. |
| D:normative:applying | normative | applying | mandatory contract execution | 0 | NO_ITEMS | No implementation execution is claimed. |
| D:normative:judging | normative | judging | compliance decision basis | 0 | NO_ITEMS | Compliance authority is constrained. |
| D:normative:reviewing | normative | reviewing | governed audit basis | 0 | NO_ITEMS | Audit basis is present. |
| D:operative:guiding | operative | guiding | workflow direction basis | 0 | NO_ITEMS | Procedure gives setup direction. |
| D:operative:applying | operative | applying | practical delivery path | 1 | HAS_ITEMS | Engine-to-export handoff detail is absent. |
| D:operative:judging | operative | judging | performance check basis | 0 | NO_ITEMS | Verification is marked TBD. |
| D:operative:reviewing | operative | reviewing | process audit trail | 0 | NO_ITEMS | Dependency mirror check is present. |
| D:evaluative:guiding | evaluative | guiding | review value basis | 0 | NO_ITEMS | Review value is stated. |
| D:evaluative:applying | evaluative | applying | merit delivery use | 0 | NO_ITEMS | No additional warranted item. |
| D:evaluative:judging | evaluative | judging | quality decision basis | 0 | NO_ITEMS | No additional warranted item. |
| D:evaluative:reviewing | evaluative | reviewing | quality audit view | 0 | NO_ITEMS | No additional warranted item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| D-001 | D:operative:applying | MissingSlot | Procedure | Procedure | Clarify which DEL-14-03 and DEL-14-04 comparison outputs feed each exporter. | The procedure names upstream comparison engines indirectly, but the export handoff shape is not available in the current sources. | Procedure.md; Specification.md | Procedure.md#Steps; Specification.md#Scope |  | PROPOSAL: add handoff steps after engine output contracts are sourced. | TBD |

## Matrix X - Verification

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| X:guiding:necessity | guiding | necessity | essential verification basis | 0 | NO_ITEMS | Verification basis is present. |
| X:guiding:sufficiency | guiding | sufficiency | adequate review evidence | 0 | NO_ITEMS | Review evidence remains future work. |
| X:guiding:completeness | guiding | completeness | complete check record | 0 | NO_ITEMS | No additional warranted item. |
| X:guiding:consistency | guiding | consistency | coherent verification signal | 0 | NO_ITEMS | No inconsistency found. |
| X:applying:necessity | applying | necessity | required test inputs | 0 | NO_ITEMS | Test inputs remain TBD with schema bodies. |
| X:applying:sufficiency | applying | sufficiency | sufficient execution proof | 0 | NO_ITEMS | No additional warranted item. |
| X:applying:completeness | applying | completeness | complete test coverage | 1 | HAS_ITEMS | Export acceptance checks are not yet concrete. |
| X:applying:consistency | applying | consistency | repeatable test result | 0 | NO_ITEMS | Determinism tests are identified as future work. |
| X:judging:necessity | judging | necessity | essential acceptance criteria | 0 | NO_ITEMS | Acceptance criteria are intentionally TBD. |
| X:judging:sufficiency | judging | sufficiency | adequate decision proof | 0 | NO_ITEMS | No additional warranted item. |
| X:judging:completeness | judging | completeness | complete decision record | 0 | NO_ITEMS | No additional warranted item. |
| X:judging:consistency | judging | consistency | consistent acceptance logic | 0 | NO_ITEMS | No additional warranted item. |
| X:reviewing:necessity | reviewing | necessity | required audit evidence | 0 | NO_ITEMS | Audit evidence requirements are identified. |
| X:reviewing:sufficiency | reviewing | sufficiency | sufficient trace proof | 0 | NO_ITEMS | No additional warranted item. |
| X:reviewing:completeness | reviewing | completeness | complete audit package | 0 | NO_ITEMS | No additional warranted item. |
| X:reviewing:consistency | reviewing | consistency | coherent audit trail | 0 | NO_ITEMS | No additional warranted item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| X-001 | X:applying:completeness | VerificationGap | Specification | Specification | Add concrete acceptance checks for JSON, CSV, and report-section exporters after schemas are materialized. | The verification table names approaches but leaves concrete checks TBD because schema bodies and export layouts are not yet defined. | Specification.md | Specification.md#Verification |  | PROPOSAL: bind checks to schema/fixture evidence in a later pass. | TBD |

## Matrix E - Evaluation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| E:guiding:data | guiding | data | verified evidence facts | 1 | HAS_ITEMS | Example payloads are absent. |
| E:guiding:information | guiding | information | contextual review signals | 0 | NO_ITEMS | Review context is present. |
| E:guiding:knowledge | guiding | knowledge | validated contract understanding | 0 | NO_ITEMS | No validation claim is overstated. |
| E:guiding:wisdom | guiding | wisdom | principled review rationale | 0 | NO_ITEMS | Principles are stated. |
| E:applying:data | applying | data | tested execution facts | 0 | NO_ITEMS | Tested facts remain future work. |
| E:applying:information | applying | information | usable workflow signals | 0 | NO_ITEMS | No additional warranted item. |
| E:applying:knowledge | applying | knowledge | validated delivery expertise | 0 | NO_ITEMS | No expertise claim is introduced. |
| E:applying:wisdom | applying | wisdom | practical decision rationale | 0 | NO_ITEMS | No additional warranted item. |
| E:judging:data | judging | data | accepted criteria facts | 0 | NO_ITEMS | Accepted criteria are not claimed. |
| E:judging:information | judging | information | decision context signals | 0 | NO_ITEMS | No additional warranted item. |
| E:judging:knowledge | judging | knowledge | validated assessment expertise | 0 | NO_ITEMS | No additional warranted item. |
| E:judging:wisdom | judging | wisdom | principled acceptance rationale | 0 | NO_ITEMS | No acceptance authority is asserted. |
| E:reviewing:data | reviewing | data | auditable evidence facts | 0 | NO_ITEMS | Audit facts are source-linked. |
| E:reviewing:information | reviewing | information | traceable audit context | 1 | HAS_ITEMS | Report-section layout remains absent. |
| E:reviewing:knowledge | reviewing | knowledge | verified audit understanding | 0 | NO_ITEMS | No additional warranted item. |
| E:reviewing:wisdom | reviewing | wisdom | principled audit rationale | 0 | NO_ITEMS | No additional warranted item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| E-001 | E:guiding:data | TBD_Question | Guidance | Guidance | Add source-safe example payloads only after approved fixtures exist. | The guidance examples section is correctly marked TBD because no approved mapping, tolerance, CSV, JSON, or report examples are available. | Guidance.md | Guidance.md#Examples |  | PROPOSAL: use invented or source-cleared fixtures only. | TBD |
| E-002 | E:reviewing:information | MissingSlot | Datasheet | Datasheet | Record TBD: report-section layout and required comparison section fields. | The documents require report-section export semantics but do not define the section layout or exact field set. | Datasheet.md; Specification.md; Procedure.md | Datasheet.md#Conditions; Specification.md#Requirements; Procedure.md#Steps |  | PROPOSAL: define layout after report-section contract evidence is available. | TBD |
