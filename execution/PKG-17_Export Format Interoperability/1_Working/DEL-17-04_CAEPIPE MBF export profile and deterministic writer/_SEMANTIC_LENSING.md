# Semantic Lensing Register: DEL-17-04 CAEPIPE MBF export profile and deterministic writer

**Generated:** 2026-05-18
**Deliverable Folder:** /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-04_CAEPIPE MBF export profile and deterministic writer

**Inputs Read:**
- _CONTEXT.md - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-04_CAEPIPE MBF export profile and deterministic writer/_CONTEXT.md#identity and scope
- _STATUS.md - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-04_CAEPIPE MBF export profile and deterministic writer/_STATUS.md#current state
- _SEMANTIC.md - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-04_CAEPIPE MBF export profile and deterministic writer/_SEMANTIC.md#matrices A/B/C/F/D/X/E
- Datasheet.md - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-04_CAEPIPE MBF export profile and deterministic writer/Datasheet.md#profile concepts and boundaries
- Specification.md - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-04_CAEPIPE MBF export profile and deterministic writer/Specification.md#requirements and verification
- Guidance.md - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-04_CAEPIPE MBF export profile and deterministic writer/Guidance.md#guidance and open questions
- Procedure.md - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-04_CAEPIPE MBF export profile and deterministic writer/Procedure.md#population and future implementation procedure
- _REFERENCES.md - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-04_CAEPIPE MBF export profile and deterministic writer/_REFERENCES.md#reference pointers

**Purpose:** Apply `semantic-matrix-build` matrix cells as lenses over the production documents, capturing warranted enrichment inputs for a later enrichment pass.

## Summary

- Total warranted items: 12
- By document:
  - Datasheet: 1
  - Specification: 4
  - Guidance: 4
  - Procedure: 3
- By matrix:
  - A: 2  B: 2  C: 2  F: 2  D: 2  X: 1  E: 1
- By type:
  - Conflict: 0
  - VerificationGap: 4
  - MissingSlot: 4
  - WeakStatement: 0
  - RationaleGap: 1
  - Normalization: 0
  - TBD_Question: 3
  - MatrixError: 0
- Notable conflicts: 0
- Matrix parse errors: 0

## Matrix A - Orientation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| A:[normative]:[guiding] | normative | guiding | prescriptive direction | 0 | NO_ITEMS | prescriptive direction lens found stable wording for normative/guiding in the current four-document kit. |
| A:[normative]:[applying] | normative | applying | mandatory practice | 1 | HAS_ITEMS | mandatory practice exposes 1 P3 worklist item for this deliverable. |
| A:[normative]:[judging] | normative | judging | compliance determination | 0 | NO_ITEMS | compliance determination lens found stable wording for normative/judging in the current four-document kit. |
| A:[normative]:[reviewing] | normative | reviewing | regulatory audit | 0 | NO_ITEMS | regulatory audit lens found stable wording for normative/reviewing in the current four-document kit. |
| A:[operative]:[guiding] | operative | guiding | procedural direction | 0 | NO_ITEMS | procedural direction lens found stable wording for operative/guiding in the current four-document kit. |
| A:[operative]:[applying] | operative | applying | practical execution | 0 | NO_ITEMS | practical execution lens found stable wording for operative/applying in the current four-document kit. |
| A:[operative]:[judging] | operative | judging | performance assessment | 0 | NO_ITEMS | performance assessment lens found stable wording for operative/judging in the current four-document kit. |
| A:[operative]:[reviewing] | operative | reviewing | process audit | 1 | HAS_ITEMS | process audit exposes 1 P3 worklist item for this deliverable. |
| A:[evaluative]:[guiding] | evaluative | guiding | value orientation | 0 | NO_ITEMS | value orientation lens found stable wording for evaluative/guiding in the current four-document kit. |
| A:[evaluative]:[applying] | evaluative | applying | merit application | 0 | NO_ITEMS | merit application lens found stable wording for evaluative/applying in the current four-document kit. |
| A:[evaluative]:[judging] | evaluative | judging | worth determination | 0 | NO_ITEMS | worth determination lens found stable wording for evaluative/judging in the current four-document kit. |
| A:[evaluative]:[reviewing] | evaluative | reviewing | quality appraisal | 0 | NO_ITEMS | quality appraisal lens found stable wording for evaluative/reviewing in the current four-document kit. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| A-001 | A:[normative]:[applying] | VerificationGap | Specification | Procedure | Add implementation acceptance check for source-confirmed MBF target basis before writer enablement. | The requirements hold target basis and subset as TBD, but the operational check for closing them before writer enablement is not explicit. | Specification.md | Requirements | NA | PROPOSAL | TBD |
| A-002 | A:[operative]:[reviewing] | MissingSlot | Procedure | Procedure | Add future implementation validation checklist for writer determinism, loss reports, and boundary claims. | The procedure identifies future work but lacks a compact validation checklist for the implementation tranche. | Procedure.md | Future Implementation Procedure | NA | PROPOSAL | TBD |

## Matrix B - Conceptualization

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| B:[data]:[necessity] | data | necessity | essential fact | 1 | HAS_ITEMS | essential fact exposes 1 P3 worklist item for this deliverable. |
| B:[data]:[sufficiency] | data | sufficiency | adequate evidence | 0 | NO_ITEMS | adequate evidence lens found stable wording for data/sufficiency in the current four-document kit. |
| B:[data]:[completeness] | data | completeness | comprehensive record | 0 | NO_ITEMS | comprehensive record lens found stable wording for data/completeness in the current four-document kit. |
| B:[data]:[consistency] | data | consistency | reliable measurement | 0 | NO_ITEMS | reliable measurement lens found stable wording for data/consistency in the current four-document kit. |
| B:[information]:[necessity] | information | necessity | essential signal | 0 | NO_ITEMS | essential signal lens found stable wording for information/necessity in the current four-document kit. |
| B:[information]:[sufficiency] | information | sufficiency | adequate context | 0 | NO_ITEMS | adequate context lens found stable wording for information/sufficiency in the current four-document kit. |
| B:[information]:[completeness] | information | completeness | comprehensive account | 1 | HAS_ITEMS | comprehensive account exposes 1 P3 worklist item for this deliverable. |
| B:[information]:[consistency] | information | consistency | coherent message | 0 | NO_ITEMS | coherent message lens found stable wording for information/consistency in the current four-document kit. |
| B:[knowledge]:[necessity] | knowledge | necessity | fundamental understanding | 0 | NO_ITEMS | fundamental understanding lens found stable wording for knowledge/necessity in the current four-document kit. |
| B:[knowledge]:[sufficiency] | knowledge | sufficiency | competent expertise | 0 | NO_ITEMS | competent expertise lens found stable wording for knowledge/sufficiency in the current four-document kit. |
| B:[knowledge]:[completeness] | knowledge | completeness | thorough mastery | 0 | NO_ITEMS | thorough mastery lens found stable wording for knowledge/completeness in the current four-document kit. |
| B:[knowledge]:[consistency] | knowledge | consistency | coherent understanding | 0 | NO_ITEMS | coherent understanding lens found stable wording for knowledge/consistency in the current four-document kit. |
| B:[wisdom]:[necessity] | wisdom | necessity | essential discernment | 0 | NO_ITEMS | essential discernment lens found stable wording for wisdom/necessity in the current four-document kit. |
| B:[wisdom]:[sufficiency] | wisdom | sufficiency | adequate judgment | 0 | NO_ITEMS | adequate judgment lens found stable wording for wisdom/sufficiency in the current four-document kit. |
| B:[wisdom]:[completeness] | wisdom | completeness | holistic insight | 0 | NO_ITEMS | holistic insight lens found stable wording for wisdom/completeness in the current four-document kit. |
| B:[wisdom]:[consistency] | wisdom | consistency | principled reasoning | 0 | NO_ITEMS | principled reasoning lens found stable wording for wisdom/consistency in the current four-document kit. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| B-001 | B:[data]:[necessity] | TBD_Question | Guidance | Guidance | TBD: confirm first CAEPIPE target version/profile and public citation target. | The target version/profile is explicitly unresolved and gates target-specific support wording. | Guidance.md | Open Questions | NA | PROPOSAL | TBD |
| B-002 | B:[information]:[completeness] | MissingSlot | Datasheet | Datasheet | Add source-basis slots for DEL-17-01 findings, DEL-17-02 contract, and public MBF import/export references. | The datasheet names source foundations but does not expose the source-basis identifiers needed by later implementation. | Datasheet.md | Profile Concepts | NA | PROPOSAL | TBD |

## Matrix C - Formulation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| C:[normative]:[necessity] | normative | necessity | required evidence basis | 1 | HAS_ITEMS | required evidence basis exposes 1 P3 worklist item for this deliverable. |
| C:[normative]:[sufficiency] | normative | sufficiency | sufficient source support | 0 | NO_ITEMS | sufficient source support lens found stable wording for normative/sufficiency in the current four-document kit. |
| C:[normative]:[completeness] | normative | completeness | complete claim boundary | 0 | NO_ITEMS | complete claim boundary lens found stable wording for normative/completeness in the current four-document kit. |
| C:[normative]:[consistency] | normative | consistency | consistent authority trace | 0 | NO_ITEMS | consistent authority trace lens found stable wording for normative/consistency in the current four-document kit. |
| C:[operative]:[necessity] | operative | necessity | required writer inputs | 0 | NO_ITEMS | required writer inputs lens found stable wording for operative/necessity in the current four-document kit. |
| C:[operative]:[sufficiency] | operative | sufficiency | adequate execution basis | 0 | NO_ITEMS | adequate execution basis lens found stable wording for operative/sufficiency in the current four-document kit. |
| C:[operative]:[completeness] | operative | completeness | complete action record | 0 | NO_ITEMS | complete action record lens found stable wording for operative/completeness in the current four-document kit. |
| C:[operative]:[consistency] | operative | consistency | coherent diagnostic flow | 1 | HAS_ITEMS | coherent diagnostic flow exposes 1 P3 worklist item for this deliverable. |
| C:[evaluative]:[necessity] | evaluative | necessity | required review basis | 0 | NO_ITEMS | required review basis lens found stable wording for evaluative/necessity in the current four-document kit. |
| C:[evaluative]:[sufficiency] | evaluative | sufficiency | adequate judgment support | 0 | NO_ITEMS | adequate judgment support lens found stable wording for evaluative/sufficiency in the current four-document kit. |
| C:[evaluative]:[completeness] | evaluative | completeness | complete audit posture | 0 | NO_ITEMS | complete audit posture lens found stable wording for evaluative/completeness in the current four-document kit. |
| C:[evaluative]:[consistency] | evaluative | consistency | coherent quality signal | 0 | NO_ITEMS | coherent quality signal lens found stable wording for evaluative/consistency in the current four-document kit. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| C-001 | C:[normative]:[necessity] | VerificationGap | Specification | Specification | Add acceptance criteria mapping CAEPIPE-specific requirements to source-basis IDs or TBD closure. | The source trace check is present, but acceptance criteria should make requirement-to-source mapping explicit. | Specification.md | Verification Requirements | NA | PROPOSAL | TBD |
| C-002 | C:[operative]:[consistency] | TBD_Question | Guidance | Guidance | TBD: classify unsupported entities as blocking export or non-blocking diagnostics. | The open questions identify this classification as unresolved, and implementation decisions depend on it. | Guidance.md | Open Questions | NA | PROPOSAL | TBD |

## Matrix F - Requirements

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| F:[normative]:[necessity] | normative | necessity | binding source gates | 0 | NO_ITEMS | binding source gates lens found stable wording for normative/necessity in the current four-document kit. |
| F:[normative]:[sufficiency] | normative | sufficiency | sufficient evidence gates | 0 | NO_ITEMS | sufficient evidence gates lens found stable wording for normative/sufficiency in the current four-document kit. |
| F:[normative]:[completeness] | normative | completeness | complete profile bounds | 1 | HAS_ITEMS | complete profile bounds exposes 1 P3 worklist item for this deliverable. |
| F:[normative]:[consistency] | normative | consistency | consistent claim limits | 0 | NO_ITEMS | consistent claim limits lens found stable wording for normative/consistency in the current four-document kit. |
| F:[operative]:[necessity] | operative | necessity | required emission inputs | 0 | NO_ITEMS | required emission inputs lens found stable wording for operative/necessity in the current four-document kit. |
| F:[operative]:[sufficiency] | operative | sufficiency | adequate writer controls | 0 | NO_ITEMS | adequate writer controls lens found stable wording for operative/sufficiency in the current four-document kit. |
| F:[operative]:[completeness] | operative | completeness | complete loss routing | 1 | HAS_ITEMS | complete loss routing exposes 1 P3 worklist item for this deliverable. |
| F:[operative]:[consistency] | operative | consistency | coherent sidecar policy | 0 | NO_ITEMS | coherent sidecar policy lens found stable wording for operative/consistency in the current four-document kit. |
| F:[evaluative]:[necessity] | evaluative | necessity | required audit signals | 0 | NO_ITEMS | required audit signals lens found stable wording for evaluative/necessity in the current four-document kit. |
| F:[evaluative]:[sufficiency] | evaluative | sufficiency | adequate review criteria | 0 | NO_ITEMS | adequate review criteria lens found stable wording for evaluative/sufficiency in the current four-document kit. |
| F:[evaluative]:[completeness] | evaluative | completeness | complete risk register | 0 | NO_ITEMS | complete risk register lens found stable wording for evaluative/completeness in the current four-document kit. |
| F:[evaluative]:[consistency] | evaluative | consistency | coherent readiness basis | 0 | NO_ITEMS | coherent readiness basis lens found stable wording for evaluative/consistency in the current four-document kit. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| F-001 | F:[normative]:[completeness] | MissingSlot | Specification | Specification | Add profile acceptance criteria for target version, record subset, stable ID policy, loss categories, and invented fixtures. | The specification has requirements, but the acceptance surface is not yet grouped around the profile completion gates. | Specification.md | Verification Requirements | NA | PROPOSAL | TBD |
| F-002 | F:[operative]:[completeness] | VerificationGap | Procedure | Procedure | Check that the future loss report covers exported, omitted, approximated, delegated, unsupported, and TBD behavior. | The procedure requires a loss report but should verify the complete category vocabulary inherited from DEL-17-02. | Procedure.md | Future Implementation Procedure | NA | PROPOSAL | TBD |

## Matrix D - Objectives

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| D:[normative]:[guiding] | normative | guiding | bounded profile direction | 0 | NO_ITEMS | bounded profile direction lens found stable wording for normative/guiding in the current four-document kit. |
| D:[normative]:[applying] | normative | applying | enforceable export contract | 1 | HAS_ITEMS | enforceable export contract exposes 1 P3 worklist item for this deliverable. |
| D:[normative]:[judging] | normative | judging | source claim decision | 0 | NO_ITEMS | source claim decision lens found stable wording for normative/judging in the current four-document kit. |
| D:[normative]:[reviewing] | normative | reviewing | boundary audit trail | 0 | NO_ITEMS | boundary audit trail lens found stable wording for normative/reviewing in the current four-document kit. |
| D:[operative]:[guiding] | operative | guiding | planned writer route | 0 | NO_ITEMS | planned writer route lens found stable wording for operative/guiding in the current four-document kit. |
| D:[operative]:[applying] | operative | applying | deterministic emission practice | 0 | NO_ITEMS | deterministic emission practice lens found stable wording for operative/applying in the current four-document kit. |
| D:[operative]:[judging] | operative | judging | diagnostic decision path | 1 | HAS_ITEMS | diagnostic decision path exposes 1 P3 worklist item for this deliverable. |
| D:[operative]:[reviewing] | operative | reviewing | process evidence audit | 0 | NO_ITEMS | process evidence audit lens found stable wording for operative/reviewing in the current four-document kit. |
| D:[evaluative]:[guiding] | evaluative | guiding | review value frame | 0 | NO_ITEMS | review value frame lens found stable wording for evaluative/guiding in the current four-document kit. |
| D:[evaluative]:[applying] | evaluative | applying | merit-based application | 0 | NO_ITEMS | merit-based application lens found stable wording for evaluative/applying in the current four-document kit. |
| D:[evaluative]:[judging] | evaluative | judging | acceptance signal test | 0 | NO_ITEMS | acceptance signal test lens found stable wording for evaluative/judging in the current four-document kit. |
| D:[evaluative]:[reviewing] | evaluative | reviewing | quality review record | 0 | NO_ITEMS | quality review record lens found stable wording for evaluative/reviewing in the current four-document kit. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| D-001 | D:[normative]:[applying] | VerificationGap | Specification | Specification | Add invented-fixture provenance as an acceptance check. | The requirements require invented fixtures, but verification does not yet check fixture provenance. | Specification.md | Requirements | NA | PROPOSAL | TBD |
| D-002 | D:[operative]:[judging] | TBD_Question | Guidance | Guidance | TBD: define blocker versus warning diagnostic policy for unsupported target behavior. | The diagnostic decision path is a gating design choice and remains explicitly unresolved. | Guidance.md | Open Questions | NA | PROPOSAL | TBD |

## Matrix X - Verification

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| X:[guiding]:[necessity] | guiding | necessity | required direction evidence | 0 | NO_ITEMS | required direction evidence lens found stable wording for guiding/necessity in the current four-document kit. |
| X:[guiding]:[sufficiency] | guiding | sufficiency | adequate boundary rationale | 0 | NO_ITEMS | adequate boundary rationale lens found stable wording for guiding/sufficiency in the current four-document kit. |
| X:[guiding]:[completeness] | guiding | completeness | complete profile rationale | 0 | NO_ITEMS | complete profile rationale lens found stable wording for guiding/completeness in the current four-document kit. |
| X:[guiding]:[consistency] | guiding | consistency | coherent source posture | 0 | NO_ITEMS | coherent source posture lens found stable wording for guiding/consistency in the current four-document kit. |
| X:[applying]:[necessity] | applying | necessity | required execution input | 0 | NO_ITEMS | required execution input lens found stable wording for applying/necessity in the current four-document kit. |
| X:[applying]:[sufficiency] | applying | sufficiency | adequate writer evidence | 0 | NO_ITEMS | adequate writer evidence lens found stable wording for applying/sufficiency in the current four-document kit. |
| X:[applying]:[completeness] | applying | completeness | complete emission record | 0 | NO_ITEMS | complete emission record lens found stable wording for applying/completeness in the current four-document kit. |
| X:[applying]:[consistency] | applying | consistency | coherent package assembly | 0 | NO_ITEMS | coherent package assembly lens found stable wording for applying/consistency in the current four-document kit. |
| X:[judging]:[necessity] | judging | necessity | required decision evidence | 0 | NO_ITEMS | required decision evidence lens found stable wording for judging/necessity in the current four-document kit. |
| X:[judging]:[sufficiency] | judging | sufficiency | adequate diagnostic basis | 0 | NO_ITEMS | adequate diagnostic basis lens found stable wording for judging/sufficiency in the current four-document kit. |
| X:[judging]:[completeness] | judging | completeness | complete loss assessment | 0 | NO_ITEMS | complete loss assessment lens found stable wording for judging/completeness in the current four-document kit. |
| X:[judging]:[consistency] | judging | consistency | coherent support classification | 0 | NO_ITEMS | coherent support classification lens found stable wording for judging/consistency in the current four-document kit. |
| X:[reviewing]:[necessity] | reviewing | necessity | required audit evidence | 0 | NO_ITEMS | required audit evidence lens found stable wording for reviewing/necessity in the current four-document kit. |
| X:[reviewing]:[sufficiency] | reviewing | sufficiency | adequate trace package | 1 | HAS_ITEMS | adequate trace package exposes 1 P3 worklist item for this deliverable. |
| X:[reviewing]:[completeness] | reviewing | completeness | complete review record | 0 | NO_ITEMS | complete review record lens found stable wording for reviewing/completeness in the current four-document kit. |
| X:[reviewing]:[consistency] | reviewing | consistency | coherent closure signal | 0 | NO_ITEMS | coherent closure signal lens found stable wording for reviewing/consistency in the current four-document kit. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| X-001 | X:[reviewing]:[sufficiency] | MissingSlot | Procedure | Procedure | Add implementation readiness evidence checklist with required rereads and local validation commands. | The procedure records population steps but needs a future-readiness checklist for trace package sufficiency. | Procedure.md | Population Procedure | NA | PROPOSAL | TBD |

## Matrix E - Evaluation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| E:[guiding]:[data] | guiding | data | source fact orientation | 0 | NO_ITEMS | source fact orientation lens found stable wording for guiding/data in the current four-document kit. |
| E:[guiding]:[information] | guiding | information | contextual direction trace | 0 | NO_ITEMS | contextual direction trace lens found stable wording for guiding/information in the current four-document kit. |
| E:[guiding]:[knowledge] | guiding | knowledge | expertise boundary frame | 0 | NO_ITEMS | expertise boundary frame lens found stable wording for guiding/knowledge in the current four-document kit. |
| E:[guiding]:[wisdom] | guiding | wisdom | principled direction basis | 0 | NO_ITEMS | principled direction basis lens found stable wording for guiding/wisdom in the current four-document kit. |
| E:[applying]:[data] | applying | data | factual emission check | 0 | NO_ITEMS | factual emission check lens found stable wording for applying/data in the current four-document kit. |
| E:[applying]:[information] | applying | information | contextual writer use | 0 | NO_ITEMS | contextual writer use lens found stable wording for applying/information in the current four-document kit. |
| E:[applying]:[knowledge] | applying | knowledge | expert execution basis | 0 | NO_ITEMS | expert execution basis lens found stable wording for applying/knowledge in the current four-document kit. |
| E:[applying]:[wisdom] | applying | wisdom | principled practice guard | 1 | HAS_ITEMS | principled practice guard exposes 1 P3 worklist item for this deliverable. |
| E:[judging]:[data] | judging | data | factual decision trace | 0 | NO_ITEMS | factual decision trace lens found stable wording for judging/data in the current four-document kit. |
| E:[judging]:[information] | judging | information | contextual diagnostic basis | 0 | NO_ITEMS | contextual diagnostic basis lens found stable wording for judging/information in the current four-document kit. |
| E:[judging]:[knowledge] | judging | knowledge | expert claim screen | 0 | NO_ITEMS | expert claim screen lens found stable wording for judging/knowledge in the current four-document kit. |
| E:[judging]:[wisdom] | judging | wisdom | principled acceptance boundary | 0 | NO_ITEMS | principled acceptance boundary lens found stable wording for judging/wisdom in the current four-document kit. |
| E:[reviewing]:[data] | reviewing | data | factual audit trail | 0 | NO_ITEMS | factual audit trail lens found stable wording for reviewing/data in the current four-document kit. |
| E:[reviewing]:[information] | reviewing | information | contextual review package | 0 | NO_ITEMS | contextual review package lens found stable wording for reviewing/information in the current four-document kit. |
| E:[reviewing]:[knowledge] | reviewing | knowledge | expert quality screen | 0 | NO_ITEMS | expert quality screen lens found stable wording for reviewing/knowledge in the current four-document kit. |
| E:[reviewing]:[wisdom] | reviewing | wisdom | principled closure rationale | 0 | NO_ITEMS | principled closure rationale lens found stable wording for reviewing/wisdom in the current four-document kit. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| E-001 | E:[applying]:[wisdom] | RationaleGap | Guidance | Guidance | Explain why pass-through target options remain metadata rather than OpenPipeStress code-checking logic. | The boundary is stated, but the rationale matters because MBF options include code-like target settings. | Guidance.md | Interpretation Guidance | NA | PROPOSAL | TBD |
