# Semantic Lensing Register: DEL-02-03 Code-neutral Analysis Boundary Model

**Generated:** 2026-04-30
**Deliverable Folder:** `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-03_Code-neutral analysis boundary model`
**Warnings:** None.

**Inputs Read:**
- _CONTEXT.md - `./_CONTEXT.md#Context-DEL-02-03`
- _STATUS.md - `./_STATUS.md#Status-DEL-02-03-Code-neutral-analysis-boundary-model`
- _SEMANTIC.md - `./_SEMANTIC.md`
- Datasheet.md - `./Datasheet.md`
- Specification.md - `./Specification.md`
- Guidance.md - `./Guidance.md`
- Procedure.md - `./Procedure.md`
- _REFERENCES.md - `./_REFERENCES.md#Governing-References`

**Purpose:** Apply `semantic-matrix-build` matrix cells as lenses over the production documents, capturing warranted enrichment inputs for a later enrichment pass.

## Summary

- Total warranted items: 10
- By document:
  - Datasheet: 1
  - Specification: 6
  - Guidance: 2
  - Procedure: 1
- By matrix:
  - A: 1  B: 1  C: 1  F: 2  D: 2  X: 2  E: 1
- By type:
  - Conflict: 1
  - VerificationGap: 3
  - MissingSlot: 2
  - WeakStatement: 1
  - RationaleGap: 1
  - Normalization: 1
  - TBD_Question: 1
  - MatrixError: 0
- Notable conflicts: 1
- Matrix parse errors: 0

## Matrix A - Orientation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| A:[normative]:[guiding] | normative | guiding | prescriptive direction | 0 | NO_ITEMS | No warranted item found under current docs. |
| A:[normative]:[applying] | normative | applying | mandatory practice | 0 | NO_ITEMS | No warranted item found under current docs. |
| A:[normative]:[judging] | normative | judging | compliance determination | 1 | HAS_ITEMS | See A-001. |
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
| A-001 | A:[normative]:[judging] | VerificationGap | Specification | Specification | Add an acceptance check that `HUMAN_APPROVED_FOR_PROJECT` wording remains a project-specific human record and never implies software approval or code compliance. | The status name contains "approved", while the deliverable also prohibits approval, certification, and code-compliance claims by software. Existing verification checks the boundary generally, but does not directly test presentation of this status label. | `./Specification.md; ./Guidance.md` | `Specification.md#Requirements; Specification.md#Verification; Guidance.md#Considerations` | N/A | PROPOSAL: Specification verification should guard non-certifying status presentation. | TBD |

## Matrix B - Conceptualization

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| B:[data]:[necessity] | data | necessity | essential fact | 0 | NO_ITEMS | No warranted item found under current docs. |
| B:[data]:[sufficiency] | data | sufficiency | adequate evidence | 0 | NO_ITEMS | No warranted item found under current docs. |
| B:[data]:[completeness] | data | completeness | comprehensive record | 0 | NO_ITEMS | No warranted item found under current docs. |
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
| B:[wisdom]:[completeness] | wisdom | completeness | holistic insight | 1 | HAS_ITEMS | See B-001. |
| B:[wisdom]:[consistency] | wisdom | consistency | principled reasoning | 0 | NO_ITEMS | No warranted item found under current docs. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| B-001 | B:[wisdom]:[completeness] | RationaleGap | Guidance | Guidance | Add decision criteria for when the single `analysis_status` enum should remain coarse versus split into separate solve, rule, and human axes. | Guidance identifies the trade-off and says a richer outcome field may be needed, but does not give criteria for future human review to decide the split. | `./Guidance.md; ./Specification.md` | `Guidance.md#Considerations; Guidance.md#Trade-offs; Specification.md#Requirements` | N/A | PROPOSAL: Guidance should hold rationale and decision triggers. | TBD |

## Matrix C - Formulation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| C:[normative]:[necessity] | normative | necessity | authority premise boundary | 0 | NO_ITEMS | No warranted item found under current docs. |
| C:[normative]:[sufficiency] | normative | sufficiency | evidence threshold rule | 0 | NO_ITEMS | No warranted item found under current docs. |
| C:[normative]:[completeness] | normative | completeness | obligation coverage map | 0 | NO_ITEMS | No warranted item found under current docs. |
| C:[normative]:[consistency] | normative | consistency | coherence control principle | 1 | HAS_ITEMS | See C-001. |
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
| C-001 | C:[normative]:[consistency] | Normalization | Guidance | Guidance | Add a vocabulary note defining the preferred human-boundary term and accepted aliases for human professional approval, human professional acceptance, and human acceptance record. | The documents use several close terms for the human side of the boundary. They are not contradictory, but a normalization note would reduce drift in downstream schema, UI, and report text. | `./Datasheet.md; ./Specification.md; ./Guidance.md; ./Procedure.md` | `Datasheet.md#Conditions; Specification.md#Scope; Guidance.md#Purpose; Procedure.md#Steps` | N/A | PROPOSAL: Guidance should own terminology rationale, with production docs following it. | TBD |

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
| F-001 | F:[normative]:[sufficiency] | WeakStatement | Specification | Specification | Clarify what "provenance sufficient to identify the source/evidence for the status" minimally includes. | Requirement R08 uses a sufficiency standard, but the verification hook only says to confirm provenance hooks. Without a minimum, later implementations could pass with weak identifiers. | `./Specification.md` | `Specification.md#Requirements; Specification.md#Verification` | N/A | PROPOSAL: Specification should define minimum provenance evidence for statuses and diagnostics. | TBD |
| F-002 | F:[normative]:[completeness] | MissingSlot | Specification | Specification | Add a field-level obligation map for `analysis_status`, `status_authority`, `status_evidence_ref`, `diagnostics`, `rule_pack_ref`, `human_acceptance_ref`, and `model_or_result_hash`. | Datasheet lists minimum draft fields, while Specification requirements do not yet map which fields are required, optional, or TBD for each boundary state. | `./Datasheet.md; ./Specification.md` | `Datasheet.md#Construction; Specification.md#Requirements` | N/A | PROPOSAL: Specification should carry the obligation map; Datasheet can remain descriptive. | TBD |

## Matrix D - Objectives

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| D:[normative]:[guiding] | normative | guiding | authority closure direction | 0 | NO_ITEMS | No warranted item found under current docs. |
| D:[normative]:[applying] | normative | applying | mandatory boundary practice | 0 | NO_ITEMS | No warranted item found under current docs. |
| D:[normative]:[judging] | normative | judging | compliance finding separation | 0 | NO_ITEMS | No warranted item found under current docs. |
| D:[normative]:[reviewing] | normative | reviewing | audit-ready authority record | 1 | HAS_ITEMS | See D-002. |
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
| D-001 | D:[operative]:[applying] | MissingSlot | Specification | Specification | Add a minimal transition or actor-authority table showing which actor may produce each `analysis_status` and what evidence is required. | Procedure Step 4 defines evidence expectations status by status, but Specification has no explicit transition or producer table for implementers to execute against. | `./Specification.md; ./Procedure.md` | `Specification.md#Requirements; Specification.md#Documentation; Procedure.md#Steps` | N/A | PROPOSAL: Specification should hold state-transition and authority semantics for future schema work. | TBD |
| D-002 | D:[normative]:[reviewing] | TBD_Question | Datasheet | Datasheet | Record TBD: where is the human acceptance record stored, and how is it marked stale when bound model, rule, or report evidence changes? | The documents require a human record bound to evidence and note invalidation on content change, but the storage location and stale-state representation remain unresolved. | `./Datasheet.md; ./Guidance.md; ./Specification.md` | `Datasheet.md#Construction; Guidance.md#Considerations; Specification.md#Requirements` | N/A | PROPOSAL: Datasheet can record the unresolved field or pointer; Specification should later constrain it. | TBD |

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
| X:[applying]:[consistency] | applying | consistency | practice trace check | 1 | HAS_ITEMS | See X-002. |
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
| X-001 | X:[reviewing]:[completeness] | VerificationGap | Specification | Specification | Expand verification to check every status against required actor, authority level, evidence reference, and diagnostic or rule-pack hook. | Verification currently checks vocabulary, separated interfaces, and provenance generally. It does not yet give complete per-status coverage for the evidence model described in Procedure Step 4. | `./Specification.md; ./Procedure.md` | `Specification.md#Verification; Procedure.md#Steps` | N/A | PROPOSAL: Specification verification should include a per-status evidence coverage table. | TBD |
| X-002 | X:[applying]:[consistency] | VerificationGap | Specification | Specification | Add a no-bypass acceptance check for adapters, plugins, and API surfaces that can observe or mutate analysis status. | Requirement R10 states adapters and plugins must not bypass governance, validation, diagnostics, or data boundaries, but the verification hook only confirms references to no-bypass constraints. | `./Specification.md; ./Guidance.md` | `Specification.md#Requirements; Specification.md#Verification; Guidance.md#Considerations` | N/A | PROPOSAL: Specification should define the no-bypass acceptance condition for later API/plugin work. | TBD |

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
| E-001 | E:[reviewing]:[data] | Conflict | Procedure | Procedure | Update or clarify the Procedure prerequisite note that says the run state was `OPEN`; current `_STATUS.md` records `SEMANTIC_READY`. | Procedure carries a run-state fact that is stale against the current deliverable status. This is a trace-record conflict, not a technical boundary-model conflict. | `./Procedure.md; ./_STATUS.md` | `Procedure.md#Prerequisites; _STATUS.md#Status-DEL-02-03-Code-neutral-analysis-boundary-model` | `./Procedure.md#Prerequisites; ./_STATUS.md#Status-DEL-02-03-Code-neutral-analysis-boundary-model` | PROPOSAL: Current `_STATUS.md` should govern lifecycle state; Procedure should avoid stale run-specific state claims. | TBD |

