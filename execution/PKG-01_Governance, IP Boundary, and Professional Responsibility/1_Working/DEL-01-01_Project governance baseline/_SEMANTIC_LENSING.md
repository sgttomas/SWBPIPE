# Semantic Lensing Register: DEL-01-01 Project governance baseline

**Generated:** 2026-04-30
**Deliverable Folder:** `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-01_Governance, IP Boundary, and Professional Responsibility/1_Working/DEL-01-01_Project governance baseline`
**Warnings:** none

**Inputs Read:**
- _CONTEXT.md - `_CONTEXT.md#Context: DEL-01-01`
- _STATUS.md - `_STATUS.md#Status: DEL-01-01 Project governance baseline`
- _SEMANTIC.md - `_SEMANTIC.md#Deliverable: DEL-01-01 Project governance baseline`
- Datasheet.md - `Datasheet.md#Datasheet: DEL-01-01 Project governance baseline`
- Specification.md - `Specification.md#Specification: DEL-01-01 Project governance baseline`
- Guidance.md - `Guidance.md#Guidance: DEL-01-01 Project governance baseline`
- Procedure.md - `Procedure.md#Procedure: DEL-01-01 Project governance baseline`
- _REFERENCES.md - `_REFERENCES.md#References: DEL-01-01 Project governance baseline`

**Purpose:** Apply `semantic-matrix-build` matrix cells as lenses over the production documents, capturing warranted enrichment inputs for a later enrichment pass.

## Summary

- Total warranted items: 7
- By document:
  - Datasheet: 1
  - Specification: 3
  - Guidance: 2
  - Procedure: 1
- By matrix:
  - A: 1  B: 1  C: 0  F: 2  D: 1  X: 1  E: 1
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

## Matrix A

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| A:[normative]:[guiding] | normative | guiding | prescriptive direction | 0 | NO_ITEMS | No warranted item under available docs. |
| A:[normative]:[applying] | normative | applying | mandatory practice | 0 | NO_ITEMS | No warranted item under available docs. |
| A:[normative]:[judging] | normative | judging | compliance determination | 0 | NO_ITEMS | No warranted item under available docs. |
| A:[normative]:[reviewing] | normative | reviewing | regulatory audit | 1 | HAS_ITEMS | The documents require open-source licensing policy while OPS-K-GOV-1 and SOW notes keep the exact license as TBD. |
| A:[operative]:[guiding] | operative | guiding | procedural direction | 0 | NO_ITEMS | No warranted item under available docs. |
| A:[operative]:[applying] | operative | applying | practical execution | 0 | NO_ITEMS | No warranted item under available docs. |
| A:[operative]:[judging] | operative | judging | performance assessment | 0 | NO_ITEMS | No warranted item under available docs. |
| A:[operative]:[reviewing] | operative | reviewing | process audit | 0 | NO_ITEMS | No warranted item under available docs. |
| A:[evaluative]:[guiding] | evaluative | guiding | value orientation | 0 | NO_ITEMS | No warranted item under available docs. |
| A:[evaluative]:[applying] | evaluative | applying | merit application | 0 | NO_ITEMS | No warranted item under available docs. |
| A:[evaluative]:[judging] | evaluative | judging | worth determination | 0 | NO_ITEMS | No warranted item under available docs. |
| A:[evaluative]:[reviewing] | evaluative | reviewing | quality appraisal | 0 | NO_ITEMS | No warranted item under available docs. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| A-001 | A:[normative]:[reviewing] | Conflict | Guidance | Multi | Keep license choice as an explicit human-ruling item. | The documents require open-source licensing policy while OPS-K-GOV-1 and SOW notes keep the exact license as TBD. | Guidance.md; Specification.md; Datasheet.md | Guidance Conflict Table C-01-01-001; Specification Requirements REQ-01-01-01; Datasheet Conditions | Guidance.md#Conflict Table C-01-01-001 Source A; Specification.md#REQ-01-01-01 | PROPOSAL: preserve TBD until human project authority records a license. | TBD |

## Matrix B

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| B:[data]:[necessity] | data | necessity | essential fact | 0 | NO_ITEMS | No warranted item under available docs. |
| B:[data]:[sufficiency] | data | sufficiency | adequate evidence | 0 | NO_ITEMS | No warranted item under available docs. |
| B:[data]:[completeness] | data | completeness | comprehensive record | 0 | NO_ITEMS | No warranted item under available docs. |
| B:[data]:[consistency] | data | consistency | reliable measurement | 0 | NO_ITEMS | No warranted item under available docs. |
| B:[information]:[necessity] | information | necessity | essential signal | 0 | NO_ITEMS | No warranted item under available docs. |
| B:[information]:[sufficiency] | information | sufficiency | adequate context | 0 | NO_ITEMS | No warranted item under available docs. |
| B:[information]:[completeness] | information | completeness | comprehensive account | 1 | HAS_ITEMS | The kit identifies these as TBD but does not yet provide a structured table ready for the maintainer skeleton. |
| B:[information]:[consistency] | information | consistency | coherent message | 0 | NO_ITEMS | No warranted item under available docs. |
| B:[knowledge]:[necessity] | knowledge | necessity | fundamental understanding | 0 | NO_ITEMS | No warranted item under available docs. |
| B:[knowledge]:[sufficiency] | knowledge | sufficiency | competent expertise | 0 | NO_ITEMS | No warranted item under available docs. |
| B:[knowledge]:[completeness] | knowledge | completeness | thorough mastery | 0 | NO_ITEMS | No warranted item under available docs. |
| B:[knowledge]:[consistency] | knowledge | consistency | coherent understanding | 0 | NO_ITEMS | No warranted item under available docs. |
| B:[wisdom]:[necessity] | wisdom | necessity | essential discernment | 0 | NO_ITEMS | No warranted item under available docs. |
| B:[wisdom]:[sufficiency] | wisdom | sufficiency | adequate judgment | 0 | NO_ITEMS | No warranted item under available docs. |
| B:[wisdom]:[completeness] | wisdom | completeness | holistic insight | 0 | NO_ITEMS | No warranted item under available docs. |
| B:[wisdom]:[consistency] | wisdom | consistency | principled reasoning | 0 | NO_ITEMS | No warranted item under available docs. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| B-001 | B:[information]:[completeness] | MissingSlot | Datasheet | Datasheet | Add explicit slots for maintainer roster, quorum, release authority, and signing process. | The kit identifies these as TBD but does not yet provide a structured table ready for the maintainer skeleton. | Datasheet.md; Guidance.md; Procedure.md | Datasheet Conditions; Guidance Conflict Table C-01-01-002; Procedure Steps | NA | PROPOSAL: add a governance decision surface table. | TBD |

## Matrix C

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| C:[normative]:[necessity] | normative | necessity | binding governance basis | 0 | NO_ITEMS | No warranted item under available docs. |
| C:[normative]:[sufficiency] | normative | sufficiency | authorized evidence frame | 0 | NO_ITEMS | No warranted item under available docs. |
| C:[normative]:[completeness] | normative | completeness | controlled coverage frame | 0 | NO_ITEMS | No warranted item under available docs. |
| C:[normative]:[consistency] | normative | consistency | governed coherence standard | 0 | NO_ITEMS | No warranted item under available docs. |
| C:[operative]:[necessity] | operative | necessity | actionable policy basis | 0 | NO_ITEMS | No warranted item under available docs. |
| C:[operative]:[sufficiency] | operative | sufficiency | workable context frame | 0 | NO_ITEMS | No warranted item under available docs. |
| C:[operative]:[completeness] | operative | completeness | executable coverage map | 0 | NO_ITEMS | No warranted item under available docs. |
| C:[operative]:[consistency] | operative | consistency | stable workflow signal | 0 | NO_ITEMS | No warranted item under available docs. |
| C:[evaluative]:[necessity] | evaluative | necessity | value-grounded basis | 0 | NO_ITEMS | No warranted item under available docs. |
| C:[evaluative]:[sufficiency] | evaluative | sufficiency | judgment readiness frame | 0 | NO_ITEMS | No warranted item under available docs. |
| C:[evaluative]:[completeness] | evaluative | completeness | review maturity frame | 0 | NO_ITEMS | No warranted item under available docs. |
| C:[evaluative]:[consistency] | evaluative | consistency | principled coherence basis | 0 | NO_ITEMS | No warranted item under available docs. |

## Matrix F

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| F:[normative]:[necessity] | normative | necessity | binding policy prerequisites | 0 | NO_ITEMS | No warranted item under available docs. |
| F:[normative]:[sufficiency] | normative | sufficiency | evidence acceptance boundary | 1 | HAS_ITEMS | REQ-01-01-03 names review gates, but the minimum evidence fields are not enumerated as acceptance criteria. |
| F:[normative]:[completeness] | normative | completeness | coverage closure standard | 0 | NO_ITEMS | No warranted item under available docs. |
| F:[normative]:[consistency] | normative | consistency | coherence control rule | 0 | NO_ITEMS | No warranted item under available docs. |
| F:[operative]:[necessity] | operative | necessity | implementation readiness basis | 0 | NO_ITEMS | No warranted item under available docs. |
| F:[operative]:[sufficiency] | operative | sufficiency | workflow evidence threshold | 0 | NO_ITEMS | No warranted item under available docs. |
| F:[operative]:[completeness] | operative | completeness | execution coverage closure | 1 | HAS_ITEMS | Procedure verification names checks but does not state the durable evidence record content. |
| F:[operative]:[consistency] | operative | consistency | release coherence guard | 0 | NO_ITEMS | No warranted item under available docs. |
| F:[evaluative]:[necessity] | evaluative | necessity | review readiness basis | 0 | NO_ITEMS | No warranted item under available docs. |
| F:[evaluative]:[sufficiency] | evaluative | sufficiency | judgment evidence threshold | 0 | NO_ITEMS | No warranted item under available docs. |
| F:[evaluative]:[completeness] | evaluative | completeness | assurance coverage closure | 0 | NO_ITEMS | No warranted item under available docs. |
| F:[evaluative]:[consistency] | evaluative | consistency | rationale coherence guard | 0 | NO_ITEMS | No warranted item under available docs. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| F-001 | F:[normative]:[sufficiency] | VerificationGap | Specification | Specification | Add acceptance criteria for contribution review evidence fields. | REQ-01-01-03 names review gates, but the minimum evidence fields are not enumerated as acceptance criteria. | Specification.md; Guidance.md | Specification Requirements REQ-01-01-03; Guidance Considerations | NA | PROPOSAL: enumerate source, provenance, redistribution, certification, review disposition, quarantine status, and privacy risk. | TBD |
| F-002 | F:[operative]:[completeness] | VerificationGap | Procedure | Procedure | Add run-record evidence target for no repo-level edits and no protected content. | Procedure verification names checks but does not state the durable evidence record content. | Procedure.md; Specification.md | Procedure Verification and Records; Specification Verification | NA | PROPOSAL: include these checks in `_run_records`. | TBD |

## Matrix D

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| D:[normative]:[guiding] | normative | guiding | authoritative governance direction | 1 | HAS_ITEMS | The documents require human authority but the specific authority is not recorded. |
| D:[normative]:[applying] | normative | applying | mandatory evidence closure | 0 | NO_ITEMS | No warranted item under available docs. |
| D:[normative]:[judging] | normative | judging | controlled acceptance basis | 0 | NO_ITEMS | No warranted item under available docs. |
| D:[normative]:[reviewing] | normative | reviewing | audit-ready coherence | 0 | NO_ITEMS | No warranted item under available docs. |
| D:[operative]:[guiding] | operative | guiding | actionable workflow direction | 0 | NO_ITEMS | No warranted item under available docs. |
| D:[operative]:[applying] | operative | applying | implementation evidence closure | 0 | NO_ITEMS | No warranted item under available docs. |
| D:[operative]:[judging] | operative | judging | execution acceptance basis | 0 | NO_ITEMS | No warranted item under available docs. |
| D:[operative]:[reviewing] | operative | reviewing | process coherence assurance | 0 | NO_ITEMS | No warranted item under available docs. |
| D:[evaluative]:[guiding] | evaluative | guiding | value-screened direction | 0 | NO_ITEMS | No warranted item under available docs. |
| D:[evaluative]:[applying] | evaluative | applying | judgment evidence closure | 0 | NO_ITEMS | No warranted item under available docs. |
| D:[evaluative]:[judging] | evaluative | judging | assurance acceptance basis | 0 | NO_ITEMS | No warranted item under available docs. |
| D:[evaluative]:[reviewing] | evaluative | reviewing | quality coherence assurance | 0 | NO_ITEMS | No warranted item under available docs. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| D-001 | D:[normative]:[guiding] | TBD_Question | Guidance | Guidance | TBD: who is the human project authority for license, maintainer, release, and policy acceptance decisions? | The documents require human authority but the specific authority is not recorded. | Guidance.md; Specification.md; Datasheet.md | Guidance Principles; Specification Requirements REQ-01-01-05; Datasheet Conditions | NA | PROPOSAL: route to human project authority before repo-level governance artifacts are issued. | TBD |

## Matrix X

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| X:[guiding]:[necessity] | guiding | necessity | directed validation basis | 0 | NO_ITEMS | No warranted item under available docs. |
| X:[guiding]:[sufficiency] | guiding | sufficiency | evidence alignment threshold | 0 | NO_ITEMS | No warranted item under available docs. |
| X:[guiding]:[completeness] | guiding | completeness | coverage alignment proof | 0 | NO_ITEMS | No warranted item under available docs. |
| X:[guiding]:[consistency] | guiding | consistency | coherence trace standard | 0 | NO_ITEMS | No warranted item under available docs. |
| X:[applying]:[necessity] | applying | necessity | execution validation basis | 0 | NO_ITEMS | No warranted item under available docs. |
| X:[applying]:[sufficiency] | applying | sufficiency | implementation evidence threshold | 0 | NO_ITEMS | No warranted item under available docs. |
| X:[applying]:[completeness] | applying | completeness | workflow coverage proof | 0 | NO_ITEMS | No warranted item under available docs. |
| X:[applying]:[consistency] | applying | consistency | process coherence trace | 0 | NO_ITEMS | No warranted item under available docs. |
| X:[judging]:[necessity] | judging | necessity | acceptance validation basis | 0 | NO_ITEMS | No warranted item under available docs. |
| X:[judging]:[sufficiency] | judging | sufficiency | decision evidence threshold | 0 | NO_ITEMS | No warranted item under available docs. |
| X:[judging]:[completeness] | judging | completeness | acceptance coverage proof | 0 | NO_ITEMS | No warranted item under available docs. |
| X:[judging]:[consistency] | judging | consistency | decision coherence trace | 0 | NO_ITEMS | No warranted item under available docs. |
| X:[reviewing]:[necessity] | reviewing | necessity | audit validation basis | 0 | NO_ITEMS | No warranted item under available docs. |
| X:[reviewing]:[sufficiency] | reviewing | sufficiency | audit evidence threshold | 0 | NO_ITEMS | No warranted item under available docs. |
| X:[reviewing]:[completeness] | reviewing | completeness | assurance coverage proof | 1 | HAS_ITEMS | The principle appears in Guidance, but Procedure verification can better preserve the release/professional-boundary distinction. |
| X:[reviewing]:[consistency] | reviewing | consistency | audit coherence trace | 0 | NO_ITEMS | No warranted item under available docs. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| X-001 | X:[reviewing]:[completeness] | RationaleGap | Procedure | Procedure | Add rationale that release validation disclosures are communication controls, not professional reliance approval. | The principle appears in Guidance, but Procedure verification can better preserve the release/professional-boundary distinction. | Guidance.md; Procedure.md | Guidance Principles and Considerations; Procedure Verification | NA | PROPOSAL: add a verification note preserving OPS-K-AUTH-1. | TBD |

## Matrix E

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| E:[guiding]:[data] | guiding | data | directed fact assurance | 0 | NO_ITEMS | No warranted item under available docs. |
| E:[guiding]:[information] | guiding | information | context trace assurance | 0 | NO_ITEMS | No warranted item under available docs. |
| E:[guiding]:[knowledge] | guiding | knowledge | expertise coverage assurance | 0 | NO_ITEMS | No warranted item under available docs. |
| E:[guiding]:[wisdom] | guiding | wisdom | reasoned closure assurance | 0 | NO_ITEMS | No warranted item under available docs. |
| E:[applying]:[data] | applying | data | implementation fact assurance | 0 | NO_ITEMS | No warranted item under available docs. |
| E:[applying]:[information] | applying | information | context execution assurance | 0 | NO_ITEMS | No warranted item under available docs. |
| E:[applying]:[knowledge] | applying | knowledge | expertise execution assurance | 0 | NO_ITEMS | No warranted item under available docs. |
| E:[applying]:[wisdom] | applying | wisdom | reasoned execution assurance | 0 | NO_ITEMS | No warranted item under available docs. |
| E:[judging]:[data] | judging | data | acceptance fact assurance | 0 | NO_ITEMS | No warranted item under available docs. |
| E:[judging]:[information] | judging | information | decision context assurance | 0 | NO_ITEMS | No warranted item under available docs. |
| E:[judging]:[knowledge] | judging | knowledge | expertise acceptance assurance | 0 | NO_ITEMS | No warranted item under available docs. |
| E:[judging]:[wisdom] | judging | wisdom | reasoned acceptance assurance | 0 | NO_ITEMS | No warranted item under available docs. |
| E:[reviewing]:[data] | reviewing | data | audit fact assurance | 0 | NO_ITEMS | No warranted item under available docs. |
| E:[reviewing]:[information] | reviewing | information | audit context assurance | 0 | NO_ITEMS | No warranted item under available docs. |
| E:[reviewing]:[knowledge] | reviewing | knowledge | expertise audit assurance | 0 | NO_ITEMS | No warranted item under available docs. |
| E:[reviewing]:[wisdom] | reviewing | wisdom | principled audit assurance | 1 | HAS_ITEMS | OPS-K-GOV-3 requires disclosure, but exact release wording and maturity labels remain undefined. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| E-001 | E:[reviewing]:[wisdom] | TBD_Question | Specification | Specification | TBD: define release maturity label taxonomy and validation status wording. | OPS-K-GOV-3 requires disclosure, but exact release wording and maturity labels remain undefined. | Specification.md; Guidance.md | Specification Requirements REQ-01-01-04; Guidance Trade-offs | NA | PROPOSAL: defer exact taxonomy to human release-governance decision. | TBD |
