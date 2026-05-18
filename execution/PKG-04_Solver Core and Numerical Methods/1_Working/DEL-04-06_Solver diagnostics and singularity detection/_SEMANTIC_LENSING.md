# Semantic Lensing Register: DEL-04-06 Solver diagnostics and singularity detection

**Generated:** 2026-04-30
**Deliverable Folder:** `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-06_Solver diagnostics and singularity detection`
**Warnings:** None.

**Inputs Read:**
- _CONTEXT.md - `./_CONTEXT.md#Context-DEL-04-06`
- _STATUS.md - `./_STATUS.md#Status-DEL-04-06-Solver-diagnostics-and-singularity-detection`
- _SEMANTIC.md - `./_SEMANTIC.md`
- Datasheet.md - `./Datasheet.md`
- Specification.md - `./Specification.md`
- Guidance.md - `./Guidance.md`
- Procedure.md - `./Procedure.md`
- _REFERENCES.md - `./_REFERENCES.md#Governing-References`

**Purpose:** Apply `semantic-matrix-build` matrix cells as lenses over the production documents, capturing warranted enrichment inputs for a later enrichment pass.

## Summary

- Total warranted items: 8
- By document:
  - Datasheet: 1
  - Specification: 5
  - Guidance: 1
  - Procedure: 1
- By matrix:
  - A: 1  B: 1  C: 1  F: 2  D: 1  X: 1  E: 1
- By type:
  - Conflict: 0
  - VerificationGap: 2
  - MissingSlot: 2
  - WeakStatement: 1
  - RationaleGap: 1
  - Normalization: 1
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
| A-001 | A:[normative]:[applying] | VerificationGap | Specification | Specification | Add acceptance checks proving singular, ill-conditioned, nonconverged, and invalid-restraint diagnostics cannot be bypassed. | Specification defines diagnostic requirements, but future implementation will need concrete acceptance checks around blocking and warning states. | ./Specification.md | Specification.md#Requirements; Specification.md#Verification | N/A | PROPOSAL: Specification should own diagnostic acceptance checks. | TBD |

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
| B-001 | B:[data]:[completeness] | MissingSlot | Datasheet | Datasheet | Record TBD diagnostic-code taxonomy for singularity, conditioning, nonconvergence, invalid restraints, and solver status. | The documents identify diagnostic categories, but do not decide the final code registry or threshold policy. | ./Datasheet.md; ./Guidance.md | Datasheet.md#Attributes; Guidance.md#Trade-offs | N/A | PROPOSAL: Datasheet should retain descriptive TBDs until implementation scope is sealed. | TBD |

## Matrix C - Formulation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| C:[normative]:[necessity] | normative | necessity | authority premise boundary | 0 | NO_ITEMS | No warranted item found under current docs. |
| C:[normative]:[sufficiency] | normative | sufficiency | evidence threshold rule | 0 | NO_ITEMS | No warranted item found under current docs. |
| C:[normative]:[completeness] | normative | completeness | obligation coverage map | 0 | NO_ITEMS | No warranted item found under current docs. |
| C:[normative]:[consistency] | normative | consistency | coherence control principle | 0 | NO_ITEMS | No warranted item found under current docs. |
| C:[operative]:[necessity] | operative | necessity | execution entry condition | 0 | NO_ITEMS | No warranted item found under current docs. |
| C:[operative]:[sufficiency] | operative | sufficiency | action evidence standard | 0 | NO_ITEMS | No warranted item found under current docs. |
| C:[operative]:[completeness] | operative | completeness | workflow coverage model | 0 | NO_ITEMS | No warranted item found under current docs. |
| C:[operative]:[consistency] | operative | consistency | trace continuity rule | 1 | HAS_ITEMS | See C-001. |
| C:[evaluative]:[necessity] | evaluative | necessity | value relevance test | 0 | NO_ITEMS | No warranted item found under current docs. |
| C:[evaluative]:[sufficiency] | evaluative | sufficiency | merit evidence basis | 0 | NO_ITEMS | No warranted item found under current docs. |
| C:[evaluative]:[completeness] | evaluative | completeness | appraisal coverage frame | 0 | NO_ITEMS | No warranted item found under current docs. |
| C:[evaluative]:[consistency] | evaluative | consistency | quality coherence standard | 0 | NO_ITEMS | No warranted item found under current docs. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| C-001 | C:[operative]:[consistency] | Normalization | Multi | Guidance | Add vocabulary guidance for singular, ill-conditioned, nonconverged, invalid restraint, warning, and blocking diagnostic states. | The terms are used consistently enough for setup, but downstream implementation and tests would benefit from one vocabulary note. | ./Datasheet.md; ./Specification.md; ./Guidance.md; ./Procedure.md | entire document scanned | N/A | PROPOSAL: Guidance should own terminology rationale. | TBD |

## Matrix F - Requirements

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| F:[normative]:[necessity] | normative | necessity | required authority evidence | 1 | HAS_ITEMS | See F-001. |
| F:[normative]:[sufficiency] | normative | sufficiency | adequate boundary proof | 1 | HAS_ITEMS | See F-002. |
| F:[normative]:[completeness] | normative | completeness | full obligation trace | 0 | NO_ITEMS | No warranted item found under current docs. |
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
| F-001 | F:[normative]:[necessity] | TBD_Question | Specification | Specification | TBD: decide singularity, conditioning, and convergence threshold policy before implementation. | SOW-053 requires these diagnostics, but accessible sources do not define numerical thresholds. | ./Specification.md; ./Guidance.md | Specification.md#Requirements; Guidance.md#Trade-offs | N/A | PROPOSAL: Future implementation brief or human ruling should set threshold policy. | TBD |
| F-002 | F:[normative]:[sufficiency] | WeakStatement | Specification | Specification | Clarify minimum provenance fields for solver stage, affected object, sparse-solver setting, and diagnostic source. | Specification requires provenance but does not define minimum diagnostic provenance fields beyond the AB-00-06 envelope. | ./Specification.md | Specification.md#Requirements | N/A | PROPOSAL: Specification should define minimum diagnostic provenance evidence. | TBD |

## Matrix D - Objectives

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| D:[normative]:[guiding] | normative | guiding | authority closure direction | 0 | NO_ITEMS | No warranted item found under current docs. |
| D:[normative]:[applying] | normative | applying | mandatory boundary practice | 0 | NO_ITEMS | No warranted item found under current docs. |
| D:[normative]:[judging] | normative | judging | compliance finding separation | 0 | NO_ITEMS | No warranted item found under current docs. |
| D:[normative]:[reviewing] | normative | reviewing | audit-ready authority record | 0 | NO_ITEMS | No warranted item found under current docs. |
| D:[operative]:[guiding] | operative | guiding | procedure closure route | 0 | NO_ITEMS | No warranted item found under current docs. |
| D:[operative]:[applying] | operative | applying | executable boundary practice | 1 | HAS_ITEMS | See D-001. |
| D:[operative]:[judging] | operative | judging | performance finding channel | 0 | NO_ITEMS | No warranted item found under current docs. |
| D:[operative]:[reviewing] | operative | reviewing | process trace record | 0 | NO_ITEMS | No warranted item found under current docs. |
| D:[evaluative]:[guiding] | evaluative | guiding | value closure orientation | 0 | NO_ITEMS | No warranted item found under current docs. |
| D:[evaluative]:[applying] | evaluative | applying | merit use discipline | 0 | NO_ITEMS | No warranted item found under current docs. |
| D:[evaluative]:[judging] | evaluative | judging | worth finding boundary | 0 | NO_ITEMS | No warranted item found under current docs. |
| D:[evaluative]:[reviewing] | evaluative | reviewing | quality appraisal record | 0 | NO_ITEMS | No warranted item found under current docs. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| D-001 | D:[operative]:[applying] | MissingSlot | Procedure | Procedure | Add future implementation step for diagnostic fixture selection after solver-library and threshold decisions are sealed. | Procedure gives setup steps but does not yet outline how future singular-model and conditioning fixtures should be selected. | ./Procedure.md; ./Guidance.md | Procedure.md#Steps; Guidance.md#Trade-offs | N/A | PROPOSAL: Procedure should capture future diagnostic-fixture workflow. | TBD |

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
| X:[applying]:[completeness] | applying | completeness | practice coverage check | 0 | NO_ITEMS | No warranted item found under current docs. |
| X:[applying]:[consistency] | applying | consistency | practice trace check | 0 | NO_ITEMS | No warranted item found under current docs. |
| X:[judging]:[necessity] | judging | necessity | finding readiness check | 0 | NO_ITEMS | No warranted item found under current docs. |
| X:[judging]:[sufficiency] | judging | sufficiency | finding proof check | 0 | NO_ITEMS | No warranted item found under current docs. |
| X:[judging]:[completeness] | judging | completeness | finding coverage check | 0 | NO_ITEMS | No warranted item found under current docs. |
| X:[judging]:[consistency] | judging | consistency | finding coherence check | 0 | NO_ITEMS | No warranted item found under current docs. |
| X:[reviewing]:[necessity] | reviewing | necessity | record readiness check | 0 | NO_ITEMS | No warranted item found under current docs. |
| X:[reviewing]:[sufficiency] | reviewing | sufficiency | record proof check | 0 | NO_ITEMS | No warranted item found under current docs. |
| X:[reviewing]:[completeness] | reviewing | completeness | record coverage check | 1 | HAS_ITEMS | See X-001. |
| X:[reviewing]:[consistency] | reviewing | consistency | record coherence check | 0 | NO_ITEMS | No warranted item found under current docs. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| X-001 | X:[reviewing]:[completeness] | VerificationGap | Specification | Specification | Add release-gate checks confirming singular-model tests and conditioning-warning tests are present before release use. | Verification mentions deterministic tests, but future acceptance should explicitly link solver diagnostics to release gates. | ./Specification.md | Specification.md#Verification | N/A | PROPOSAL: Specification should require diagnostic test gate coverage. | TBD |

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
| E:[reviewing]:[data] | reviewing | data | fact record assurance | 1 | HAS_ITEMS | See E-001. |
| E:[reviewing]:[information] | reviewing | information | signal record assurance | 0 | NO_ITEMS | No warranted item found under current docs. |
| E:[reviewing]:[knowledge] | reviewing | knowledge | understanding record assurance | 0 | NO_ITEMS | No warranted item found under current docs. |
| E:[reviewing]:[wisdom] | reviewing | wisdom | discernment record assurance | 0 | NO_ITEMS | No warranted item found under current docs. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| E-001 | E:[reviewing]:[data] | RationaleGap | Guidance | Guidance | Explain why diagnostic thresholds remain TBD and how future values must be justified without implying compliance. | Guidance marks thresholds as TBD, but future contributors need rationale to avoid inventing or overclaiming diagnostic policy. | ./Guidance.md | Guidance.md#Considerations; Guidance.md#Trade-offs | N/A | PROPOSAL: Guidance should carry threshold-rationale boundaries. | TBD |
