# Semantic Lensing Register: DEL-04-03 Linear support and restraint models

**Generated:** 2026-04-30
**Deliverable Folder:** `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-03_Linear support and restraint models`
**Warnings:** None.

**Inputs Read:**
- `_CONTEXT.md`
- `_STATUS.md`
- `_SEMANTIC.md`
- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_REFERENCES.md`

**Purpose:** Apply `semantic-matrix-build` matrix cells as lenses over the production documents, capturing warranted enrichment inputs for later enrichment without rewriting authority from the lens.

## Summary

- Total warranted items: 7
- By document: Datasheet 1; Specification 3; Guidance 1; Procedure 2.
- By matrix: A 1; B 1; C 1; F 1; D 1; X 1; E 1.
- By type: VerificationGap 2; MissingSlot 2; WeakStatement 1; RationaleGap 1; TBD_Question 1; Conflict 0; Normalization 0; MatrixError 0.
- Matrix parse errors: 0.

## Matrix A - Orientation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| A:[normative]:[guiding] | normative | guiding | prescriptive direction | 0 | NO_ITEMS | No warranted item found. |
| A:[normative]:[applying] | normative | applying | mandatory practice | 1 | HAS_ITEMS | See A-001. |
| A:[normative]:[judging] | normative | judging | compliance determination | 0 | NO_ITEMS | No warranted item found. |
| A:[normative]:[reviewing] | normative | reviewing | regulatory audit | 0 | NO_ITEMS | No warranted item found. |
| A:[operative]:[guiding] | operative | guiding | procedural direction | 0 | NO_ITEMS | No warranted item found. |
| A:[operative]:[applying] | operative | applying | practical execution | 0 | NO_ITEMS | No warranted item found. |
| A:[operative]:[judging] | operative | judging | performance assessment | 0 | NO_ITEMS | No warranted item found. |
| A:[operative]:[reviewing] | operative | reviewing | process audit | 0 | NO_ITEMS | No warranted item found. |
| A:[evaluative]:[guiding] | evaluative | guiding | value orientation | 0 | NO_ITEMS | No warranted item found. |
| A:[evaluative]:[applying] | evaluative | applying | merit application | 0 | NO_ITEMS | No warranted item found. |
| A:[evaluative]:[judging] | evaluative | judging | worth determination | 0 | NO_ITEMS | No warranted item found. |
| A:[evaluative]:[reviewing] | evaluative | reviewing | quality appraisal | 0 | NO_ITEMS | No warranted item found. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| A-001 | A:[normative]:[applying] | VerificationGap | Specification | Specification | Add acceptance checks for no silent defaults in support stiffness, directions, and imposed displacement data. | Requirements state the rule; implementation acceptance needs explicit checks. | ./Specification.md | Specification.md#Verification | N/A | PROPOSAL: Specification owns acceptance criteria. | TBD |

## Matrix B - Conceptualization

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| B:[data]:[necessity] | data | necessity | essential fact | 0 | NO_ITEMS | No warranted item found. |
| B:[data]:[sufficiency] | data | sufficiency | adequate evidence | 0 | NO_ITEMS | No warranted item found. |
| B:[data]:[completeness] | data | completeness | comprehensive record | 1 | HAS_ITEMS | See B-001. |
| B:[data]:[consistency] | data | consistency | reliable measurement | 0 | NO_ITEMS | No warranted item found. |
| B:[information]:[necessity] | information | necessity | essential signal | 0 | NO_ITEMS | No warranted item found. |
| B:[information]:[sufficiency] | information | sufficiency | adequate context | 0 | NO_ITEMS | No warranted item found. |
| B:[information]:[completeness] | information | completeness | comprehensive account | 0 | NO_ITEMS | No warranted item found. |
| B:[information]:[consistency] | information | consistency | coherent message | 0 | NO_ITEMS | No warranted item found. |
| B:[knowledge]:[necessity] | knowledge | necessity | fundamental understanding | 0 | NO_ITEMS | No warranted item found. |
| B:[knowledge]:[sufficiency] | knowledge | sufficiency | competent expertise | 0 | NO_ITEMS | No warranted item found. |
| B:[knowledge]:[completeness] | knowledge | completeness | thorough mastery | 0 | NO_ITEMS | No warranted item found. |
| B:[knowledge]:[consistency] | knowledge | consistency | coherent understanding | 0 | NO_ITEMS | No warranted item found. |
| B:[wisdom]:[necessity] | wisdom | necessity | essential discernment | 0 | NO_ITEMS | No warranted item found. |
| B:[wisdom]:[sufficiency] | wisdom | sufficiency | adequate judgment | 0 | NO_ITEMS | No warranted item found. |
| B:[wisdom]:[completeness] | wisdom | completeness | holistic insight | 0 | NO_ITEMS | No warranted item found. |
| B:[wisdom]:[consistency] | wisdom | consistency | principled reasoning | 0 | NO_ITEMS | No warranted item found. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| B-001 | B:[data]:[completeness] | MissingSlot | Datasheet | Datasheet | Record TBD for exact field taxonomy by support family. | The setup lists families, but exact schema/API fields are not settled. | ./Datasheet.md | Datasheet.md#Attributes | N/A | PROPOSAL: Datasheet retains descriptive TBD slots. | TBD |

## Matrix C - Formulation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| C:[normative]:[necessity] | normative | necessity | boundary mandate basis | 0 | NO_ITEMS | No warranted item found. |
| C:[normative]:[sufficiency] | normative | sufficiency | evidence threshold rule | 0 | NO_ITEMS | No warranted item found. |
| C:[normative]:[completeness] | normative | completeness | obligation coverage map | 0 | NO_ITEMS | No warranted item found. |
| C:[normative]:[consistency] | normative | consistency | coherence control principle | 0 | NO_ITEMS | No warranted item found. |
| C:[operative]:[necessity] | operative | necessity | execution entry condition | 0 | NO_ITEMS | No warranted item found. |
| C:[operative]:[sufficiency] | operative | sufficiency | action evidence standard | 0 | NO_ITEMS | No warranted item found. |
| C:[operative]:[completeness] | operative | completeness | workflow coverage model | 0 | NO_ITEMS | No warranted item found. |
| C:[operative]:[consistency] | operative | consistency | trace continuity rule | 1 | HAS_ITEMS | See C-001. |
| C:[evaluative]:[necessity] | evaluative | necessity | value relevance test | 0 | NO_ITEMS | No warranted item found. |
| C:[evaluative]:[sufficiency] | evaluative | sufficiency | merit evidence basis | 0 | NO_ITEMS | No warranted item found. |
| C:[evaluative]:[completeness] | evaluative | completeness | appraisal coverage frame | 0 | NO_ITEMS | No warranted item found. |
| C:[evaluative]:[consistency] | evaluative | consistency | quality coherence standard | 0 | NO_ITEMS | No warranted item found. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| C-001 | C:[operative]:[consistency] | WeakStatement | Guidance | Guidance | Clarify coordinate-frame convention remains TBD and must be decided before implementation acceptance. | The docs mark convention TBD, but the risk should stay visible for deterministic testing. | ./Guidance.md | Guidance.md#Considerations | N/A | PROPOSAL: Guidance owns interpretation risk. | TBD |

## Matrix F - Requirements

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| F:[normative]:[necessity] | normative | necessity | required boundary evidence | 1 | HAS_ITEMS | See F-001. |
| F:[normative]:[sufficiency] | normative | sufficiency | adequate model proof | 0 | NO_ITEMS | No warranted item found. |
| F:[normative]:[completeness] | normative | completeness | full obligation trace | 0 | NO_ITEMS | No warranted item found. |
| F:[normative]:[consistency] | normative | consistency | stable authority semantics | 0 | NO_ITEMS | No warranted item found. |
| F:[operative]:[necessity] | operative | necessity | required execution evidence | 0 | NO_ITEMS | No warranted item found. |
| F:[operative]:[sufficiency] | operative | sufficiency | adequate action context | 0 | NO_ITEMS | No warranted item found. |
| F:[operative]:[completeness] | operative | completeness | full workflow trace | 0 | NO_ITEMS | No warranted item found. |
| F:[operative]:[consistency] | operative | consistency | stable process semantics | 0 | NO_ITEMS | No warranted item found. |
| F:[evaluative]:[necessity] | evaluative | necessity | required review evidence | 0 | NO_ITEMS | No warranted item found. |
| F:[evaluative]:[sufficiency] | evaluative | sufficiency | adequate merit context | 0 | NO_ITEMS | No warranted item found. |
| F:[evaluative]:[completeness] | evaluative | completeness | full appraisal trace | 0 | NO_ITEMS | No warranted item found. |
| F:[evaluative]:[consistency] | evaluative | consistency | stable quality semantics | 0 | NO_ITEMS | No warranted item found. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| F-001 | F:[normative]:[necessity] | TBD_Question | Specification | Specification | TBD: decide support direction coordinate basis before schema/API implementation. | Six-DOF basis is known, but support direction convention is not settled. | ./Specification.md | Specification.md#Requirements | N/A | PROPOSAL: Future implementation brief or decision record resolves it. | TBD |

## Matrix D - Objectives

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| D:[normative]:[guiding] | normative | guiding | boundary closure direction | 0 | NO_ITEMS | No warranted item found. |
| D:[normative]:[applying] | normative | applying | mandatory restraint practice | 0 | NO_ITEMS | No warranted item found. |
| D:[normative]:[judging] | normative | judging | compliance separation boundary | 0 | NO_ITEMS | No warranted item found. |
| D:[normative]:[reviewing] | normative | reviewing | audit-ready authority record | 0 | NO_ITEMS | No warranted item found. |
| D:[operative]:[guiding] | operative | guiding | procedure closure route | 0 | NO_ITEMS | No warranted item found. |
| D:[operative]:[applying] | operative | applying | executable support practice | 1 | HAS_ITEMS | See D-001. |
| D:[operative]:[judging] | operative | judging | mechanics finding channel | 0 | NO_ITEMS | No warranted item found. |
| D:[operative]:[reviewing] | operative | reviewing | process trace record | 0 | NO_ITEMS | No warranted item found. |
| D:[evaluative]:[guiding] | evaluative | guiding | value closure orientation | 0 | NO_ITEMS | No warranted item found. |
| D:[evaluative]:[applying] | evaluative | applying | merit use discipline | 0 | NO_ITEMS | No warranted item found. |
| D:[evaluative]:[judging] | evaluative | judging | worth finding boundary | 0 | NO_ITEMS | No warranted item found. |
| D:[evaluative]:[reviewing] | evaluative | reviewing | quality appraisal record | 0 | NO_ITEMS | No warranted item found. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| D-001 | D:[operative]:[applying] | MissingSlot | Procedure | Procedure | Add future implementation step to review singularity/overconstraint diagnostics with DEL-04-06. | Linear restraints can create singular or overconstrained models; diagnostics ownership is adjacent. | ./Procedure.md | Procedure.md#Steps | N/A | PROPOSAL: Procedure records handoff awareness without cross-package edit. | TBD |

## Matrix X - Verification

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| X:[guiding]:[necessity] | guiding | necessity | boundary premise check | 0 | NO_ITEMS | No warranted item found. |
| X:[guiding]:[sufficiency] | guiding | sufficiency | boundary proof check | 0 | NO_ITEMS | No warranted item found. |
| X:[guiding]:[completeness] | guiding | completeness | boundary coverage check | 0 | NO_ITEMS | No warranted item found. |
| X:[guiding]:[consistency] | guiding | consistency | boundary coherence check | 0 | NO_ITEMS | No warranted item found. |
| X:[applying]:[necessity] | applying | necessity | practice readiness check | 0 | NO_ITEMS | No warranted item found. |
| X:[applying]:[sufficiency] | applying | sufficiency | practice evidence check | 1 | HAS_ITEMS | See X-001. |
| X:[applying]:[completeness] | applying | completeness | practice coverage check | 0 | NO_ITEMS | No warranted item found. |
| X:[applying]:[consistency] | applying | consistency | practice trace check | 0 | NO_ITEMS | No warranted item found. |
| X:[judging]:[necessity] | judging | necessity | finding readiness check | 0 | NO_ITEMS | No warranted item found. |
| X:[judging]:[sufficiency] | judging | sufficiency | finding proof check | 0 | NO_ITEMS | No warranted item found. |
| X:[judging]:[completeness] | judging | completeness | finding coverage check | 0 | NO_ITEMS | No warranted item found. |
| X:[judging]:[consistency] | judging | consistency | finding coherence check | 0 | NO_ITEMS | No warranted item found. |
| X:[reviewing]:[necessity] | reviewing | necessity | record readiness check | 0 | NO_ITEMS | No warranted item found. |
| X:[reviewing]:[sufficiency] | reviewing | sufficiency | record proof check | 0 | NO_ITEMS | No warranted item found. |
| X:[reviewing]:[completeness] | reviewing | completeness | record coverage check | 0 | NO_ITEMS | No warranted item found. |
| X:[reviewing]:[consistency] | reviewing | consistency | record coherence check | 0 | NO_ITEMS | No warranted item found. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| X-001 | X:[applying]:[sufficiency] | VerificationGap | Procedure | Procedure | Add explicit unit/dimensional test checklist for support stiffness and imposed displacement values. | Procedure mentions verification but could make support-specific unit checks more direct. | ./Procedure.md | Procedure.md#Verification | N/A | PROPOSAL: Procedure owns operational checklists. | TBD |

## Matrix E - Evaluation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| E:[necessity]:[normative] | necessity | normative | required support boundary | 1 | HAS_ITEMS | See E-001. |
| E:[necessity]:[operative] | necessity | operative | required solve pathway | 0 | NO_ITEMS | No warranted item found. |
| E:[necessity]:[evaluative] | necessity | evaluative | required review concern | 0 | NO_ITEMS | No warranted item found. |
| E:[sufficiency]:[normative] | sufficiency | normative | adequate support evidence | 0 | NO_ITEMS | No warranted item found. |
| E:[sufficiency]:[operative] | sufficiency | operative | adequate solve context | 0 | NO_ITEMS | No warranted item found. |
| E:[sufficiency]:[evaluative] | sufficiency | evaluative | adequate review basis | 0 | NO_ITEMS | No warranted item found. |
| E:[completeness]:[normative] | completeness | normative | complete support trace | 0 | NO_ITEMS | No warranted item found. |
| E:[completeness]:[operative] | completeness | operative | complete process record | 0 | NO_ITEMS | No warranted item found. |
| E:[completeness]:[evaluative] | completeness | evaluative | complete appraisal basis | 0 | NO_ITEMS | No warranted item found. |
| E:[consistency]:[normative] | consistency | normative | consistent support semantics | 0 | NO_ITEMS | No warranted item found. |
| E:[consistency]:[operative] | consistency | operative | consistent solve trace | 0 | NO_ITEMS | No warranted item found. |
| E:[consistency]:[evaluative] | consistency | evaluative | consistent quality basis | 0 | NO_ITEMS | No warranted item found. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| E-001 | E:[necessity]:[normative] | RationaleGap | Specification | Specification | Note that rigid-restraint numerical treatment and solver assembly are TBD and must not be inferred from setup wording. | The setup intentionally avoids implementation; downstream readers need the boundary preserved. | ./Specification.md | Specification.md#Scope | N/A | PROPOSAL: Specification owns normative boundary language. | TBD |
