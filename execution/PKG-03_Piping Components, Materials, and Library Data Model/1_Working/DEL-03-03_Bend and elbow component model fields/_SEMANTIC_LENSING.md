# Semantic Lensing Register: DEL-03-03 Bend and elbow component model fields

**Generated:** 2026-04-30
**Deliverable Folder:** `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-03_Bend and elbow component model fields`
**Warnings:** None.

**Inputs Read:**
- _CONTEXT.md - `./_CONTEXT.md#Context-DEL-03-03`
- _STATUS.md - `./_STATUS.md#Status-DEL-03-03-Bend-and-elbow-component-model-fields`
- _SEMANTIC.md - `./_SEMANTIC.md`
- Datasheet.md - `./Datasheet.md`
- Specification.md - `./Specification.md`
- Guidance.md - `./Guidance.md`
- Procedure.md - `./Procedure.md`
- _REFERENCES.md - `./_REFERENCES.md#Governing-References`

**Purpose:** Apply `semantic-matrix-build` matrix cells as lenses over the production documents, capturing warranted enrichment inputs for a later implementation or review pass.

## Summary

- Total warranted items: 8
- By document:
  - Datasheet: 2
  - Specification: 4
  - Guidance: 1
  - Procedure: 1
- By matrix:
  - A: 1  B: 1  C: 1  F: 2  D: 1  X: 1  E: 1
- By type:
  - Conflict: 0
  - VerificationGap: 3
  - MissingSlot: 2
  - WeakStatement: 1
  - RationaleGap: 1
  - Normalization: 0
  - TBD_Question: 1
  - MatrixError: 0
- Notable conflicts: 0
- Matrix parse errors: 0

## Matrix A - Orientation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| A:[normative]:[guiding] | normative | guiding | prescriptive direction | 0 | NO_ITEMS | No warranted item found under current docs. |
| A:[normative]:[applying] | normative | applying | mandatory practice | 1 | HAS_ITEMS | See A-001. |
| A:[normative]:[judging] | normative | judging | compliance determination | 0 | NO_ITEMS | No warranted item found under current docs. |
| A:[normative]:[reviewing] | normative | reviewing | regulatory audit | 0 | NO_ITEMS | No warranted item found under current docs. |
| A:[operative]:[guiding] | operative | guiding | procedural direction | 0 | NO_ITEMS | No warranted item found under current docs. |
| A:[operative]:[applying] | operative | applying | practical execution | 0 | NO_ITEMS | No warranted item found under current docs. |
| A:[operative]:[judging] | operative | judging | performance assessment | 0 | NO_ITEMS | No warranted item found under current docs. |
| A:[operative]:[reviewing] | operative | reviewing | process audit | 0 | NO_ITEMS | No warranted item found under current docs. |
| A:[evaluative]:[guiding] | evaluative | guiding | value orientation | 0 | NO_ITEMS | No warranted item found under current docs. |
| A:[evaluative]:[applying] | evaluative | applying | merit application | 0 | NO_ITEMS | No warranted item found under current docs. |
| A:[evaluative]:[judging] | evaluative | judging | worth determination | 0 | NO_ITEMS | No warranted item found under current docs. |
| A:[evaluative]:[reviewing] | evaluative | reviewing | quality appraisal | 0 | NO_ITEMS | No warranted item found under current docs. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| A-001 | A:[normative]:[applying] | VerificationGap | Specification | Specification | Add acceptance checks that prevent public bundled protected SIF/flexibility defaults or formulas. | Specification states the protected-content rule, but future tests need an explicit protected-content gate. | `./Specification.md; ./Guidance.md` | `Specification.md#Requirements; Guidance.md#Purpose` | N/A | PROPOSAL: Specification verification should name the protected-content gate. | TBD |

## Matrix B - Conceptualization

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| B:[data]:[necessity] | data | necessity | essential fact | 0 | NO_ITEMS | No warranted item found under current docs. |
| B:[data]:[sufficiency] | data | sufficiency | adequate evidence | 0 | NO_ITEMS | No warranted item found under current docs. |
| B:[data]:[completeness] | data | completeness | comprehensive record | 1 | HAS_ITEMS | See B-001. |
| B:[data]:[consistency] | data | consistency | reliable measurement | 0 | NO_ITEMS | No warranted item found under current docs. |
| B:[information]:[necessity] | information | necessity | essential signal | 0 | NO_ITEMS | No warranted item found under current docs. |
| B:[information]:[sufficiency] | information | sufficiency | adequate context | 0 | NO_ITEMS | No warranted item found under current docs. |
| B:[information]:[completeness] | information | completeness | comprehensive account | 0 | NO_ITEMS | No warranted item found under current docs. |
| B:[information]:[consistency] | information | consistency | coherent message | 0 | NO_ITEMS | No warranted item found under current docs. |
| B:[knowledge]:[necessity] | knowledge | necessity | fundamental understanding | 0 | NO_ITEMS | No warranted item found under current docs. |
| B:[knowledge]:[sufficiency] | knowledge | sufficiency | competent expertise | 0 | NO_ITEMS | No warranted item found under current docs. |
| B:[knowledge]:[completeness] | knowledge | completeness | thorough mastery | 0 | NO_ITEMS | No warranted item found under current docs. |
| B:[knowledge]:[consistency] | knowledge | consistency | coherent understanding | 0 | NO_ITEMS | No warranted item found under current docs. |
| B:[wisdom]:[necessity] | wisdom | necessity | essential discernment | 0 | NO_ITEMS | No warranted item found under current docs. |
| B:[wisdom]:[sufficiency] | wisdom | sufficiency | adequate judgment | 0 | NO_ITEMS | No warranted item found under current docs. |
| B:[wisdom]:[completeness] | wisdom | completeness | holistic insight | 0 | NO_ITEMS | No warranted item found under current docs. |
| B:[wisdom]:[consistency] | wisdom | consistency | principled reasoning | 0 | NO_ITEMS | No warranted item found under current docs. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| B-001 | B:[data]:[completeness] | MissingSlot | Datasheet | Datasheet | Add a later field inventory for exact geometry slots, factor slots, provenance slots, and validation flags. | Datasheet intentionally holds draft field concepts, but exact implementation fields are still TBD. | `./Datasheet.md` | `Datasheet.md#Construction` | N/A | PROPOSAL: Datasheet should host descriptive field inventory once implementation is authorized. | TBD |

## Matrix C - Formulation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| C:[normative]:[necessity] | normative | necessity | authority premise boundary | 1 | HAS_ITEMS | See C-001. |
| C:[normative]:[sufficiency] | normative | sufficiency | evidence threshold rule | 0 | NO_ITEMS | No warranted item found under current docs. |
| C:[normative]:[completeness] | normative | completeness | obligation coverage map | 0 | NO_ITEMS | No warranted item found under current docs. |
| C:[normative]:[consistency] | normative | consistency | coherence control principle | 0 | NO_ITEMS | No warranted item found under current docs. |
| C:[operative]:[necessity] | operative | necessity | execution entry condition | 0 | NO_ITEMS | No warranted item found under current docs. |
| C:[operative]:[sufficiency] | operative | sufficiency | action evidence standard | 0 | NO_ITEMS | No warranted item found under current docs. |
| C:[operative]:[completeness] | operative | completeness | workflow coverage model | 0 | NO_ITEMS | No warranted item found under current docs. |
| C:[operative]:[consistency] | operative | consistency | trace continuity rule | 0 | NO_ITEMS | No warranted item found under current docs. |
| C:[evaluative]:[necessity] | evaluative | necessity | value relevance test | 0 | NO_ITEMS | No warranted item found under current docs. |
| C:[evaluative]:[sufficiency] | evaluative | sufficiency | merit evidence basis | 0 | NO_ITEMS | No warranted item found under current docs. |
| C:[evaluative]:[completeness] | evaluative | completeness | appraisal coverage frame | 0 | NO_ITEMS | No warranted item found under current docs. |
| C:[evaluative]:[consistency] | evaluative | consistency | quality coherence standard | 0 | NO_ITEMS | No warranted item found under current docs. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| C-001 | C:[normative]:[necessity] | WeakStatement | Specification | Specification | Clarify whether SIF and flexibility factor fields are single values, directional values, or structured groups. | The scope requires support for user-entered SIFs and flexibility factors, but exact cardinality and direction/basis semantics are still TBD. | `./Specification.md; ./Datasheet.md` | `Specification.md#Requirements; Datasheet.md#Construction` | N/A | PROPOSAL: Specification should define cardinality once the implementation field model is chosen. | TBD |

## Matrix F - Requirements

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| F:[normative]:[necessity] | normative | necessity | required authority evidence | 0 | NO_ITEMS | No warranted item found under current docs. |
| F:[normative]:[sufficiency] | normative | sufficiency | adequate boundary proof | 1 | HAS_ITEMS | See F-001. |
| F:[normative]:[completeness] | normative | completeness | full obligation trace | 1 | HAS_ITEMS | See F-002. |
| F:[normative]:[consistency] | normative | consistency | stable authority semantics | 0 | NO_ITEMS | No warranted item found under current docs. |
| F:[operative]:[necessity] | operative | necessity | required execution evidence | 0 | NO_ITEMS | No warranted item found under current docs. |
| F:[operative]:[sufficiency] | operative | sufficiency | adequate action context | 0 | NO_ITEMS | No warranted item found under current docs. |
| F:[operative]:[completeness] | operative | completeness | full workflow trace | 0 | NO_ITEMS | No warranted item found under current docs. |
| F:[operative]:[consistency] | operative | consistency | stable process semantics | 0 | NO_ITEMS | No warranted item found under current docs. |
| F:[evaluative]:[necessity] | evaluative | necessity | required review evidence | 0 | NO_ITEMS | No warranted item found under current docs. |
| F:[evaluative]:[sufficiency] | evaluative | sufficiency | adequate merit context | 0 | NO_ITEMS | No warranted item found under current docs. |
| F:[evaluative]:[completeness] | evaluative | completeness | full appraisal trace | 0 | NO_ITEMS | No warranted item found under current docs. |
| F:[evaluative]:[consistency] | evaluative | consistency | stable quality semantics | 0 | NO_ITEMS | No warranted item found under current docs. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| F-001 | F:[normative]:[sufficiency] | VerificationGap | Specification | Specification | Add tests for minimum provenance metadata on geometry, SIF, and flexibility inputs. | Requirements call for provenance fields, but the minimum acceptable evidence per input group is still not enumerated. | `./Specification.md; ./Datasheet.md` | `Specification.md#Requirements; Datasheet.md#Attributes` | N/A | PROPOSAL: Specification should hold minimum provenance acceptance criteria. | TBD |
| F-002 | F:[normative]:[completeness] | MissingSlot | Specification | Specification | Add an obligation map linking each required model concept to a test class. | Requirements list tests generally, but not a complete field-to-test coverage table. | `./Specification.md; ./Procedure.md` | `Specification.md#Verification; Procedure.md#Verification` | N/A | PROPOSAL: Specification verification should include field-to-test coverage. | TBD |

## Matrix D - Objectives

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| D:[normative]:[guiding] | normative | guiding | source boundary direction | 0 | NO_ITEMS | No warranted item found under current docs. |
| D:[normative]:[applying] | normative | applying | protected-data practice | 0 | NO_ITEMS | No warranted item found under current docs. |
| D:[normative]:[judging] | normative | judging | authority separation finding | 0 | NO_ITEMS | No warranted item found under current docs. |
| D:[normative]:[reviewing] | normative | reviewing | audit-ready provenance | 1 | HAS_ITEMS | See D-001. |
| D:[operative]:[guiding] | operative | guiding | procedure closure route | 0 | NO_ITEMS | No warranted item found under current docs. |
| D:[operative]:[applying] | operative | applying | executable field practice | 0 | NO_ITEMS | No warranted item found under current docs. |
| D:[operative]:[judging] | operative | judging | validation finding channel | 0 | NO_ITEMS | No warranted item found under current docs. |
| D:[operative]:[reviewing] | operative | reviewing | process trace record | 0 | NO_ITEMS | No warranted item found under current docs. |
| D:[evaluative]:[guiding] | evaluative | guiding | value closure orientation | 0 | NO_ITEMS | No warranted item found under current docs. |
| D:[evaluative]:[applying] | evaluative | applying | merit use discipline | 0 | NO_ITEMS | No warranted item found under current docs. |
| D:[evaluative]:[judging] | evaluative | judging | worth finding boundary | 0 | NO_ITEMS | No warranted item found under current docs. |
| D:[evaluative]:[reviewing] | evaluative | reviewing | quality appraisal record | 0 | NO_ITEMS | No warranted item found under current docs. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| D-001 | D:[normative]:[reviewing] | TBD_Question | Datasheet | Datasheet | Record TBD: exact provenance fields and redistribution-status vocabulary for bend/elbow library data. | The setup kit requires provenance and source metadata, but exact vocabulary remains implementation-level. | `./Datasheet.md; ./Specification.md` | `Datasheet.md#Construction; Specification.md#Requirements` | N/A | PROPOSAL: Datasheet can record unresolved field vocabulary until schema work resolves it. | TBD |

## Matrix X - Verification

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| X:[guiding]:[necessity] | guiding | necessity | boundary premise check | 0 | NO_ITEMS | No warranted item found under current docs. |
| X:[guiding]:[sufficiency] | guiding | sufficiency | boundary proof check | 0 | NO_ITEMS | No warranted item found under current docs. |
| X:[guiding]:[completeness] | guiding | completeness | boundary coverage check | 0 | NO_ITEMS | No warranted item found under current docs. |
| X:[guiding]:[consistency] | guiding | consistency | boundary coherence check | 0 | NO_ITEMS | No warranted item found under current docs. |
| X:[applying]:[necessity] | applying | necessity | practice readiness check | 0 | NO_ITEMS | No warranted item found under current docs. |
| X:[applying]:[sufficiency] | applying | sufficiency | practice evidence check | 0 | NO_ITEMS | No warranted item found under current docs. |
| X:[applying]:[completeness] | applying | completeness | practice coverage check | 1 | HAS_ITEMS | See X-001. |
| X:[applying]:[consistency] | applying | consistency | practice trace check | 0 | NO_ITEMS | No warranted item found under current docs. |
| X:[judging]:[necessity] | judging | necessity | finding readiness check | 0 | NO_ITEMS | No warranted item found under current docs. |
| X:[judging]:[sufficiency] | judging | sufficiency | finding proof check | 0 | NO_ITEMS | No warranted item found under current docs. |
| X:[judging]:[completeness] | judging | completeness | finding coverage check | 0 | NO_ITEMS | No warranted item found under current docs. |
| X:[judging]:[consistency] | judging | consistency | finding coherence check | 0 | NO_ITEMS | No warranted item found under current docs. |
| X:[reviewing]:[necessity] | reviewing | necessity | record readiness check | 0 | NO_ITEMS | No warranted item found under current docs. |
| X:[reviewing]:[sufficiency] | reviewing | sufficiency | record proof check | 0 | NO_ITEMS | No warranted item found under current docs. |
| X:[reviewing]:[completeness] | reviewing | completeness | record coverage check | 0 | NO_ITEMS | No warranted item found under current docs. |
| X:[reviewing]:[consistency] | reviewing | consistency | record coherence check | 0 | NO_ITEMS | No warranted item found under current docs. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| X-001 | X:[applying]:[completeness] | VerificationGap | Procedure | Procedure | Add a later checklist that each implementation test maps to geometry, factors, provenance, diagnostics, unit awareness, and protected-content gates. | Procedure names test groups but does not yet show complete execution checklist detail. | `./Procedure.md` | `Procedure.md#Steps; Procedure.md#Verification` | N/A | PROPOSAL: Procedure should hold execution checklist detail after implementation plan exists. | TBD |

## Matrix E - Evaluation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| E:[guiding]:[data] | guiding | data | fact boundary assurance | 0 | NO_ITEMS | No warranted item found under current docs. |
| E:[guiding]:[information] | guiding | information | signal boundary assurance | 0 | NO_ITEMS | No warranted item found under current docs. |
| E:[guiding]:[knowledge] | guiding | knowledge | understanding boundary assurance | 0 | NO_ITEMS | No warranted item found under current docs. |
| E:[guiding]:[wisdom] | guiding | wisdom | discernment boundary assurance | 0 | NO_ITEMS | No warranted item found under current docs. |
| E:[applying]:[data] | applying | data | fact practice assurance | 0 | NO_ITEMS | No warranted item found under current docs. |
| E:[applying]:[information] | applying | information | signal practice assurance | 0 | NO_ITEMS | No warranted item found under current docs. |
| E:[applying]:[knowledge] | applying | knowledge | understanding practice assurance | 0 | NO_ITEMS | No warranted item found under current docs. |
| E:[applying]:[wisdom] | applying | wisdom | discernment practice assurance | 0 | NO_ITEMS | No warranted item found under current docs. |
| E:[judging]:[data] | judging | data | fact finding assurance | 0 | NO_ITEMS | No warranted item found under current docs. |
| E:[judging]:[information] | judging | information | signal finding assurance | 0 | NO_ITEMS | No warranted item found under current docs. |
| E:[judging]:[knowledge] | judging | knowledge | understanding finding assurance | 0 | NO_ITEMS | No warranted item found under current docs. |
| E:[judging]:[wisdom] | judging | wisdom | discernment finding assurance | 0 | NO_ITEMS | No warranted item found under current docs. |
| E:[reviewing]:[data] | reviewing | data | fact record assurance | 0 | NO_ITEMS | No warranted item found under current docs. |
| E:[reviewing]:[information] | reviewing | information | signal record assurance | 0 | NO_ITEMS | No warranted item found under current docs. |
| E:[reviewing]:[knowledge] | reviewing | knowledge | understanding record assurance | 0 | NO_ITEMS | No warranted item found under current docs. |
| E:[reviewing]:[wisdom] | reviewing | wisdom | discernment record assurance | 1 | HAS_ITEMS | See E-001. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| E-001 | E:[reviewing]:[wisdom] | RationaleGap | Guidance | Guidance | Add future decision criteria for when bend/elbow field vocabulary should be shared with generic component schemas versus bend-specific. | Guidance identifies the trade-off but does not yet provide selection criteria for downstream schema design. | `./Guidance.md` | `Guidance.md#Trade-offs` | N/A | PROPOSAL: Guidance should host this rationale after schema options are known. | TBD |
