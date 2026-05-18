# Semantic Lensing Register: DEL-15-02 Target mapping and unsupported-behavior contract

**Generated:** 2026-05-03
**Deliverable Folder:** /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-15_Handoff and External Prover Workflow/1_Working/DEL-15-02_Target mapping and unsupported-behavior contract
**Warnings:** None

**Inputs Read:**
- _CONTEXT.md - full file
- _STATUS.md - current lifecycle state
- _SEMANTIC.md - matrices A, B, C, F, D, X, E
- Datasheet.md - full file
- Specification.md - full file
- Guidance.md - full file
- Procedure.md - full file
- _REFERENCES.md - full file

**Purpose:** Apply `semantic-matrix-build` matrix cells as lenses over the production documents, capturing warranted enrichment inputs for a later enrichment pass.

## Summary

- Total warranted items: 7
- By document:
  - Datasheet: 1
  - Specification: 2
  - Guidance: 2
  - Procedure: 2
- By matrix:
  - A: 1  B: 1  C: 1  F: 1  D: 1  X: 1  E: 1
- By type:
  - Conflict: 1
  - VerificationGap: 2
  - MissingSlot: 1
  - WeakStatement: 0
  - RationaleGap: 1
  - Normalization: 0
  - TBD_Question: 2
  - MatrixError: 0
- Notable conflicts: 1
- Matrix parse errors: 0

## Matrix A - Orientation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| A:[normative]:[guiding] | normative | guiding | prescriptive direction | 1 | HAS_ITEMS | warranted item recorded |
| A:[normative]:[applying] | normative | applying | mandatory practice | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| A:[normative]:[judging] | normative | judging | compliance determination | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| A:[normative]:[reviewing] | normative | reviewing | regulatory audit | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| A:[operative]:[guiding] | operative | guiding | procedural direction | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| A:[operative]:[applying] | operative | applying | practical execution | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| A:[operative]:[judging] | operative | judging | performance assessment | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| A:[operative]:[reviewing] | operative | reviewing | process audit | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| A:[evaluative]:[guiding] | evaluative | guiding | value orientation | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| A:[evaluative]:[applying] | evaluative | applying | merit application | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| A:[evaluative]:[judging] | evaluative | judging | worth determination | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| A:[evaluative]:[reviewing] | evaluative | reviewing | quality appraisal | 0 | NO_ITEMS | no warranted enrichment item under this lens |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| A-001 | A:[normative]:[guiding] | TBD_Question | Specification | Specification | TBD: confirm target list and target-specific mapping strategy before hardening normative contract language. | The production docs correctly preserve OI-015 as TBD, but the contract cannot be completed until this governing decision is resolved. | Specification.md; Guidance.md | Specification.md#Standards; Guidance.md#Principles | NA | PROPOSAL: human product decision or CHANGE-approved decomposition amendment | TBD |

## Matrix B - Conceptualization

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| B:[data]:[necessity] | data | necessity | essential fact | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| B:[data]:[sufficiency] | data | sufficiency | adequate evidence | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| B:[data]:[completeness] | data | completeness | comprehensive record | 1 | HAS_ITEMS | warranted item recorded |
| B:[data]:[consistency] | data | consistency | reliable measurement | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| B:[information]:[necessity] | information | necessity | essential signal | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| B:[information]:[sufficiency] | information | sufficiency | adequate context | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| B:[information]:[completeness] | information | completeness | comprehensive account | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| B:[information]:[consistency] | information | consistency | coherent message | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| B:[knowledge]:[necessity] | knowledge | necessity | fundamental understanding | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| B:[knowledge]:[sufficiency] | knowledge | sufficiency | competent expertise | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| B:[knowledge]:[completeness] | knowledge | completeness | thorough mastery | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| B:[knowledge]:[consistency] | knowledge | consistency | coherent understanding | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| B:[wisdom]:[necessity] | wisdom | necessity | essential discernment | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| B:[wisdom]:[sufficiency] | wisdom | sufficiency | adequate judgment | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| B:[wisdom]:[completeness] | wisdom | completeness | holistic insight | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| B:[wisdom]:[consistency] | wisdom | consistency | principled reasoning | 0 | NO_ITEMS | no warranted enrichment item under this lens |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| B-001 | B:[data]:[completeness] | MissingSlot | Datasheet | Datasheet | Add final schema path, schema identifier, and property-name source once accepted. | Datasheet Construction leaves the target mapping schema path and property names as TBD, so the descriptive data slot remains incomplete. | Datasheet.md | Datasheet.md#Construction | NA | PROPOSAL: accepted implementation artifact or decomposition amendment | TBD |

## Matrix C - Formulation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| C:[normative]:[necessity] | normative | necessity | binding evidence basis | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| C:[normative]:[sufficiency] | normative | sufficiency | adequate rule context | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| C:[normative]:[completeness] | normative | completeness | complete compliance record | 1 | HAS_ITEMS | warranted item recorded |
| C:[normative]:[consistency] | normative | consistency | coherent audit basis | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| C:[operative]:[necessity] | operative | necessity | required execution input | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| C:[operative]:[sufficiency] | operative | sufficiency | workable context package | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| C:[operative]:[completeness] | operative | completeness | complete process record | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| C:[operative]:[consistency] | operative | consistency | stable process message | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| C:[evaluative]:[necessity] | evaluative | necessity | essential value criterion | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| C:[evaluative]:[sufficiency] | evaluative | sufficiency | adequate appraisal context | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| C:[evaluative]:[completeness] | evaluative | completeness | complete quality account | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| C:[evaluative]:[consistency] | evaluative | consistency | coherent merit rationale | 0 | NO_ITEMS | no warranted enrichment item under this lens |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| C-001 | C:[normative]:[completeness] | VerificationGap | Specification | Specification | Add acceptance criteria for unsupported behavior taxonomy completeness. | Specification requires unsupported-target and approximate behavior representation, but exact taxonomy values and completeness criteria are TBD. | Specification.md | Specification.md#Requirements; Specification.md#Verification | NA | PROPOSAL: validation-qa or human-approved taxonomy source | TBD |

## Matrix F - Requirements

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| F:[normative]:[necessity] | normative | necessity | mandatory evidence threshold | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| F:[normative]:[sufficiency] | normative | sufficiency | supported conformance basis | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| F:[normative]:[completeness] | normative | completeness | exhaustive compliance trace | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| F:[normative]:[consistency] | normative | consistency | stable authority record | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| F:[operative]:[necessity] | operative | necessity | input readiness gate | 1 | HAS_ITEMS | warranted item recorded |
| F:[operative]:[sufficiency] | operative | sufficiency | usable execution context | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| F:[operative]:[completeness] | operative | completeness | full workflow record | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| F:[operative]:[consistency] | operative | consistency | repeatable transfer basis | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| F:[evaluative]:[necessity] | evaluative | necessity | review criterion basis | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| F:[evaluative]:[sufficiency] | evaluative | sufficiency | reasoned appraisal support | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| F:[evaluative]:[completeness] | evaluative | completeness | comprehensive trace trail | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| F:[evaluative]:[consistency] | evaluative | consistency | defensible merit rationale | 0 | NO_ITEMS | no warranted enrichment item under this lens |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| F-001 | F:[operative]:[necessity] | TBD_Question | Procedure | Procedure | TBD: identify which predecessor contract source slices must be available before final contract authoring. | Procedure lists ACTIVE predecessor mirror rows, but this workflow was not allowed to inspect sibling DEL folders and therefore cannot confirm predecessor contract contents. | Procedure.md | Procedure.md#Prerequisites | NA | PROPOSAL: later sealed Type 2 brief or human-provided source slices | TBD |

## Matrix D - Objectives

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| D:[normative]:[guiding] | normative | guiding | directive closure basis | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| D:[normative]:[applying] | normative | applying | enforceable practice contract | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| D:[normative]:[judging] | normative | judging | determinate compliance verdict | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| D:[normative]:[reviewing] | normative | reviewing | auditable control record | 1 | HAS_ITEMS | warranted item recorded |
| D:[operative]:[guiding] | operative | guiding | executable process direction | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| D:[operative]:[applying] | operative | applying | verified execution path | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| D:[operative]:[judging] | operative | judging | measured performance closure | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| D:[operative]:[reviewing] | operative | reviewing | inspected workflow evidence | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| D:[evaluative]:[guiding] | evaluative | guiding | value rationale closure | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| D:[evaluative]:[applying] | evaluative | applying | applied merit basis | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| D:[evaluative]:[judging] | evaluative | judging | resolved worth finding | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| D:[evaluative]:[reviewing] | evaluative | reviewing | quality appraisal closure | 0 | NO_ITEMS | no warranted enrichment item under this lens |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| D-001 | D:[normative]:[reviewing] | Conflict | Guidance | NA | Keep dependency mirror unchanged; record dependency-extract enum conflict for human ruling. | Guidance records a conflict between dependency-extract normalization rules and the explicit project rule to preserve approved DAG-002 mirror rows as ACTIVE. | Guidance.md | Guidance.md#Conflict Table | Guidance.md#Conflict Table Source A cell; Guidance.md#Conflict Table Source B cell | PROPOSAL: user task rule for this run | TBD |

## Matrix X - Verification

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| X:[guiding]:[necessity] | guiding | necessity | directive evidence check | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| X:[guiding]:[sufficiency] | guiding | sufficiency | contextual instruction proof | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| X:[guiding]:[completeness] | guiding | completeness | full instruction trace | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| X:[guiding]:[consistency] | guiding | consistency | aligned control check | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| X:[applying]:[necessity] | applying | necessity | practice readiness proof | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| X:[applying]:[sufficiency] | applying | sufficiency | executable adequacy check | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| X:[applying]:[completeness] | applying | completeness | complete practice evidence | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| X:[applying]:[consistency] | applying | consistency | repeatable execution check | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| X:[judging]:[necessity] | judging | necessity | compliance evidence test | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| X:[judging]:[sufficiency] | judging | sufficiency | verdict support proof | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| X:[judging]:[completeness] | judging | completeness | determination trace record | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| X:[judging]:[consistency] | judging | consistency | coherent decision check | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| X:[reviewing]:[necessity] | reviewing | necessity | audit evidence gate | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| X:[reviewing]:[sufficiency] | reviewing | sufficiency | audit context proof | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| X:[reviewing]:[completeness] | reviewing | completeness | complete audit trail | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| X:[reviewing]:[consistency] | reviewing | consistency | audit coherence check | 1 | HAS_ITEMS | warranted item recorded |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| X-001 | X:[reviewing]:[consistency] | VerificationGap | Procedure | Procedure | Add a durable record location for dependency schema validation output. | Procedure requires running the schema validator, but no deliverable-local record file for validator output is defined beyond the final workflow report. | Procedure.md | Procedure.md#Verification; Procedure.md#Records | NA | PROPOSAL: final workflow report or future validation evidence file | TBD |

## Matrix E - Evaluation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| E:[guiding]:[data] | guiding | data | directive fact trace | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| E:[guiding]:[information] | guiding | information | contextual signal trail | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| E:[guiding]:[knowledge] | guiding | knowledge | instruction mastery evidence | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| E:[guiding]:[wisdom] | guiding | wisdom | principled instruction rationale | 1 | HAS_ITEMS | warranted item recorded |
| E:[applying]:[data] | applying | data | practice fact proof | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| E:[applying]:[information] | applying | information | executable signal context | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| E:[applying]:[knowledge] | applying | knowledge | skilled practice trace | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| E:[applying]:[wisdom] | applying | wisdom | reasoned practice basis | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| E:[judging]:[data] | judging | data | compliance fact evidence | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| E:[judging]:[information] | judging | information | verdict signal support | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| E:[judging]:[knowledge] | judging | knowledge | determination mastery proof | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| E:[judging]:[wisdom] | judging | wisdom | principled verdict rationale | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| E:[reviewing]:[data] | reviewing | data | audit fact record | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| E:[reviewing]:[information] | reviewing | information | audit signal context | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| E:[reviewing]:[knowledge] | reviewing | knowledge | audit mastery trace | 0 | NO_ITEMS | no warranted enrichment item under this lens |
| E:[reviewing]:[wisdom] | reviewing | wisdom | principled audit rationale | 0 | NO_ITEMS | no warranted enrichment item under this lens |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| E-001 | E:[guiding]:[wisdom] | RationaleGap | Guidance | Guidance | Clarify rationale for approximate behavior flag semantics once taxonomy values exist. | Guidance marks approximate behavior semantics as ASSUMPTION/TBD, so later users will need rationale for interpreting non-exact target representation safely. | Guidance.md | Guidance.md#Examples; Guidance.md#Considerations | NA | PROPOSAL: approved taxonomy plus professional-boundary policy | TBD |
