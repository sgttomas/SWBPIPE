# Semantic Lensing Register: DEL-02-01 Canonical domain model schema

**Generated:** 2026-04-30
**Deliverable Folder:** `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-01_Canonical domain model schema`
**Warnings:** none

**Inputs Read:**
- _CONTEXT.md - `_CONTEXT.md#Context: DEL-02-01`
- _STATUS.md - `_STATUS.md#Status: DEL-02-01 Canonical domain model schema`
- _SEMANTIC.md - `_SEMANTIC.md#Deliverable: DEL-02-01 Canonical domain model schema`
- Datasheet.md - `Datasheet.md#Datasheet: DEL-02-01 Canonical domain model schema`
- Specification.md - `Specification.md#Specification: DEL-02-01 Canonical domain model schema`
- Guidance.md - `Guidance.md#Guidance: DEL-02-01 Canonical domain model schema`
- Procedure.md - `Procedure.md#Procedure: DEL-02-01 Canonical domain model schema`
- _REFERENCES.md - `_REFERENCES.md#References: DEL-02-01 Canonical domain model schema`

**Purpose:** Apply `semantic-matrix-build` matrix cells as lenses over the production documents, capturing warranted enrichment inputs for a later enrichment pass.

## Summary

- Total warranted items: 14
- By document:
  - Datasheet: 2
  - Specification: 5
  - Guidance: 5
  - Procedure: 2
- By matrix:
  - A: 1  B: 2  C: 2  F: 3  D: 2  X: 2  E: 2
- By type:
  - Conflict: 2
  - VerificationGap: 5
  - MissingSlot: 2
  - WeakStatement: 1
  - RationaleGap: 1
  - Normalization: 1
  - TBD_Question: 2
  - MatrixError: 0
- Notable conflicts: 2
- Matrix parse errors: 0

## Matrix A - Orientation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| A:[normative]:[guiding] | normative | guiding | prescriptive direction | 0 | NO_ITEMS | No warranted item under available docs. |
| A:[normative]:[applying] | normative | applying | mandatory practice | 0 | NO_ITEMS | No warranted item under available docs. |
| A:[normative]:[judging] | normative | judging | compliance determination | 0 | NO_ITEMS | No warranted item under available docs. |
| A:[normative]:[reviewing] | normative | reviewing | regulatory audit | 1 | HAS_ITEMS | Objective mapping conflict affects review traceability. |
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
| A-001 | A:[normative]:[reviewing] | Conflict | Guidance | Multi | Preserve unresolved objective-mapping ruling item before schema acceptance. | `Guidance.md` records that DEL-02-01 maps to OBJ-001 in some sources while SOW-041 also maps OBJ-012. Schema acceptance and coverage records should not silently choose a governing objective. | Guidance.md; Datasheet.md; Specification.md; Procedure.md | Guidance Conflict Table C-02-01-001; Datasheet Identification; Specification Scope; Procedure Records | Guidance.md#Conflict Table C-02-01-001 Source A; Guidance.md#Conflict Table C-02-01-001 Source B | PROPOSAL: keep OBJ-001 as deliverable-owned in draft artifacts until human ruling, while recording OBJ-012 as unresolved context. | TBD |

## Matrix B - Conceptualization

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| B:[data]:[necessity] | data | necessity | essential fact | 0 | NO_ITEMS | No warranted item under available docs. |
| B:[data]:[sufficiency] | data | sufficiency | adequate evidence | 0 | NO_ITEMS | No warranted item under available docs. |
| B:[data]:[completeness] | data | completeness | comprehensive record | 0 | NO_ITEMS | No warranted item under available docs. |
| B:[data]:[consistency] | data | consistency | reliable measurement | 1 | HAS_ITEMS | Revision metadata conflict affects reliable basis identification. |
| B:[information]:[necessity] | information | necessity | essential signal | 0 | NO_ITEMS | No warranted item under available docs. |
| B:[information]:[sufficiency] | information | sufficiency | adequate context | 0 | NO_ITEMS | No warranted item under available docs. |
| B:[information]:[completeness] | information | completeness | comprehensive account | 0 | NO_ITEMS | No warranted item under available docs. |
| B:[information]:[consistency] | information | consistency | coherent message | 1 | HAS_ITEMS | Object-family naming needs normalization before schema vocabulary freezes. |
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
| B-001 | B:[data]:[consistency] | Conflict | Guidance | Multi | Preserve stale-revision metadata discrepancy as a human-ruling dependency. | `Guidance.md` records conflicting decomposition revision pointers: some metadata says v0.2 or v0.3 while `_CONTEXT.md` and the current drafting basis identify revision 0.4. The register should surface this basis conflict without editing metadata. | Guidance.md; Datasheet.md; _CONTEXT.md; _REFERENCES.md | Guidance Conflict Table C-02-01-002; Datasheet Identification and References; _CONTEXT Decomposition Reference; _REFERENCES Decomposition and Registers | Guidance.md#Conflict Table C-02-01-002 Source A; Guidance.md#Conflict Table C-02-01-002 Source B | PROPOSAL: use `_CONTEXT.md` revision 0.4 for this run and leave governance pointer edits to a human-approved metadata pass. | TBD |
| B-002 | B:[information]:[consistency] | Normalization | Datasheet | Guidance | Clarify canonical naming relationships among Load, LoadCase, load cases, combinations, Support, Support/Restraint, and Section. | Production documents use a mix of slash forms, singular labels, plural prose, and adjacent object labels. Without a vocabulary note, future schema definitions and any `docs/TYPES.md` update could drift. | Datasheet.md; Specification.md; Guidance.md; Procedure.md | Datasheet Attributes and Construction; Specification Scope and Requirements; Guidance Considerations and Examples; Procedure Steps 2 and 5 | Datasheet.md#Attributes; Specification.md#Scope; Procedure.md#Steps | PROPOSAL: add a non-authoritative vocabulary normalization note before promoting names into schema definitions or `docs/TYPES.md`. | TBD |

## Matrix C - Formulation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| C:[normative]:[necessity] | normative | necessity | binding schema basis | 0 | NO_ITEMS | No warranted item under available docs. |
| C:[normative]:[sufficiency] | normative | sufficiency | authorized evidence frame | 0 | NO_ITEMS | No warranted item under available docs. |
| C:[normative]:[completeness] | normative | completeness | controlled coverage frame | 1 | HAS_ITEMS | Required object-family coverage needs an explicit crosswalk. |
| C:[normative]:[consistency] | normative | consistency | governed coherence standard | 0 | NO_ITEMS | No warranted item under available docs. |
| C:[operative]:[necessity] | operative | necessity | actionable model basis | 0 | NO_ITEMS | No warranted item under available docs. |
| C:[operative]:[sufficiency] | operative | sufficiency | workable context frame | 1 | HAS_ITEMS | Adjacent-object boundary wording is materially ambiguous. |
| C:[operative]:[completeness] | operative | completeness | executable coverage map | 0 | NO_ITEMS | No warranted item under available docs. |
| C:[operative]:[consistency] | operative | consistency | stable workflow signal | 0 | NO_ITEMS | No warranted item under available docs. |
| C:[evaluative]:[necessity] | evaluative | necessity | value-grounded basis | 0 | NO_ITEMS | No warranted item under available docs. |
| C:[evaluative]:[sufficiency] | evaluative | sufficiency | judgment readiness frame | 0 | NO_ITEMS | No warranted item under available docs. |
| C:[evaluative]:[completeness] | evaluative | completeness | review maturity frame | 0 | NO_ITEMS | No warranted item under available docs. |
| C:[evaluative]:[consistency] | evaluative | consistency | principled coherence basis | 0 | NO_ITEMS | No warranted item under available docs. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| C-001 | C:[normative]:[completeness] | MissingSlot | Specification | Specification | Add an object-family coverage crosswalk from required and adjacent families to schema definitions, references, or deferrals. | `Specification.md` requires coverage review for Project, Model, Node, Element, Material, Component, Load/LoadCase, Result, and Report, while `Guidance.md` and `Procedure.md` also raise adjacent families. No table currently records coverage or justified deferral by family. | Specification.md; Guidance.md; Procedure.md | Specification Requirements REQ-02-01-02 and Verification; Guidance Considerations; Procedure Steps 2 and 5 | NA | PROPOSAL: add the crosswalk as acceptance-supporting schema planning content, not as implemented schema evidence. | TBD |
| C-002 | C:[operative]:[sufficiency] | WeakStatement | Guidance | Guidance | Clarify what `at the level authorized for DEL-02-01` permits for adjacent objects. | `Procedure.md` tells future work to define Material, Component, Section, Support/Restraint, LoadCase, Combination, Result, and Report records at the authorized level, but the production documents do not define that level. This ambiguity can change implementation scope. | Guidance.md; Procedure.md; Datasheet.md | Guidance Considerations; Procedure Step 5; Datasheet Conditions | NA | PROPOSAL: record boundary criteria for shared references versus package-owned details before schema drafting. | TBD |

## Matrix F - Requirements

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| F:[normative]:[necessity] | normative | necessity | binding data prerequisites | 0 | NO_ITEMS | No warranted item under available docs. |
| F:[normative]:[sufficiency] | normative | sufficiency | evidence acceptance boundary | 1 | HAS_ITEMS | Provenance/status acceptance boundary needs explicit reusable-field criteria. |
| F:[normative]:[completeness] | normative | completeness | coverage closure standard | 0 | NO_ITEMS | No warranted item under available docs. |
| F:[normative]:[consistency] | normative | consistency | coherence control rule | 1 | HAS_ITEMS | Unit-aware requirement lacks field-level acceptance mapping. |
| F:[operative]:[necessity] | operative | necessity | implementation readiness basis | 0 | NO_ITEMS | No warranted item under available docs. |
| F:[operative]:[sufficiency] | operative | sufficiency | workflow evidence threshold | 0 | NO_ITEMS | No warranted item under available docs. |
| F:[operative]:[completeness] | operative | completeness | execution coverage closure | 1 | HAS_ITEMS | Protected-content and provenance gate evidence needs a concrete run record target. |
| F:[operative]:[consistency] | operative | consistency | serialization coherence guard | 0 | NO_ITEMS | No warranted item under available docs. |
| F:[evaluative]:[necessity] | evaluative | necessity | review readiness basis | 0 | NO_ITEMS | No warranted item under available docs. |
| F:[evaluative]:[sufficiency] | evaluative | sufficiency | judgment evidence threshold | 0 | NO_ITEMS | No warranted item under available docs. |
| F:[evaluative]:[completeness] | evaluative | completeness | assurance coverage closure | 0 | NO_ITEMS | No warranted item under available docs. |
| F:[evaluative]:[consistency] | evaluative | consistency | rationale coherence guard | 0 | NO_ITEMS | No warranted item under available docs. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| F-001 | F:[normative]:[consistency] | VerificationGap | Specification | Specification | Add acceptance mapping for which schema fields require units or unit references. | REQ-02-01-04 requires dimensional values to be unit-aware and tests to reject incompatible or missing units where required, but the production documents do not identify the field set or acceptance matrix. | Specification.md; Procedure.md; Datasheet.md | Specification Requirements REQ-02-01-04 and Verification; Procedure Steps 4 and 9; Datasheet Construction | NA | PROPOSAL: require field-level unit applicability evidence before claiming the unit check is covered. | TBD |
| F-002 | F:[normative]:[sufficiency] | VerificationGap | Specification | Specification | Add minimal acceptance criteria for provenance, redistribution/private status, contributor/review status, and privacy metadata. | REQ-02-01-05 requires provenance/status fields or reusable definitions, but the documents do not define the minimum reusable record shape or how each engineering-reliance family is checked. | Specification.md; Guidance.md; Procedure.md | Specification Requirements REQ-02-01-05 and Verification; Guidance Principles; Procedure Step 4 | NA | PROPOSAL: use the future schema review to verify reusable provenance/status definitions against each relying object family. | TBD |
| F-003 | F:[operative]:[completeness] | VerificationGap | Procedure | Procedure | Add concrete protected-content and provenance gate evidence records for schemas, examples, and fixtures. | The documents require protected-content/provenance gates, but the procedure does not name the evidence artifact, scan target list, or pass/fail record needed for review closure. | Procedure.md; Specification.md; Datasheet.md | Procedure Verification and Records; Specification Requirements REQ-02-01-06 and REQ-02-01-11; Datasheet Construction | NA | PROPOSAL: record expected gate output paths or review records before implementation proceeds. | TBD |

## Matrix D - Objectives

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| D:[normative]:[guiding] | normative | guiding | authoritative schema direction | 1 | HAS_ITEMS | Schema identity and tooling decisions remain explicit TBDs. |
| D:[normative]:[applying] | normative | applying | mandatory evidence closure | 0 | NO_ITEMS | No warranted item under available docs. |
| D:[normative]:[judging] | normative | judging | controlled acceptance basis | 0 | NO_ITEMS | No warranted item under available docs. |
| D:[normative]:[reviewing] | normative | reviewing | audit-ready coherence | 0 | NO_ITEMS | No warranted item under available docs. |
| D:[operative]:[guiding] | operative | guiding | actionable workflow direction | 0 | NO_ITEMS | No warranted item under available docs. |
| D:[operative]:[applying] | operative | applying | implementation evidence closure | 0 | NO_ITEMS | No warranted item under available docs. |
| D:[operative]:[judging] | operative | judging | execution acceptance basis | 0 | NO_ITEMS | No warranted item under available docs. |
| D:[operative]:[reviewing] | operative | reviewing | process coherence assurance | 1 | HAS_ITEMS | Persistence and migration boundary remains unresolved. |
| D:[evaluative]:[guiding] | evaluative | guiding | value-screened direction | 0 | NO_ITEMS | No warranted item under available docs. |
| D:[evaluative]:[applying] | evaluative | applying | judgment evidence closure | 0 | NO_ITEMS | No warranted item under available docs. |
| D:[evaluative]:[judging] | evaluative | judging | assurance acceptance basis | 0 | NO_ITEMS | No warranted item under available docs. |
| D:[evaluative]:[reviewing] | evaluative | reviewing | quality coherence assurance | 0 | NO_ITEMS | No warranted item under available docs. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| D-001 | D:[normative]:[guiding] | TBD_Question | Datasheet | TBD | TBD: who decides schema file layout, `$id` URI, code-generation tooling, and fixture organization? | These items are explicitly TBD in `Datasheet.md` and also called out for human review in `Specification.md`. They are prerequisite direction for the future schema artifact but cannot be invented here. | Datasheet.md; Specification.md; Guidance.md | Datasheet Conditions; Specification Documentation; Guidance Principles | NA | PROPOSAL: route to human architecture or package authority before schema artifact creation. | TBD |
| D-002 | D:[operative]:[reviewing] | TBD_Question | Guidance | Guidance | TBD: define the handoff between this schema contract and persistence, migration, physical project packaging, and DEL-02-05 ownership. | REQ-02-01-09 requires versioned, migration-aware, round-trip-testable persistence compatibility, while `Datasheet.md` and `Guidance.md` leave physical package and migration framework outside this deliverable unless approved. | Guidance.md; Datasheet.md; Specification.md | Guidance Trade-offs; Datasheet Conditions; Specification Requirements REQ-02-01-09 | NA | PROPOSAL: record compatibility constraints here and defer implementation ownership to the approved persistence/package deliverable. | TBD |

## Matrix X - Verification

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| X:[guiding]:[necessity] | guiding | necessity | directed validation basis | 0 | NO_ITEMS | No warranted item under available docs. |
| X:[guiding]:[sufficiency] | guiding | sufficiency | evidence alignment threshold | 0 | NO_ITEMS | No warranted item under available docs. |
| X:[guiding]:[completeness] | guiding | completeness | coverage alignment proof | 0 | NO_ITEMS | No warranted item under available docs. |
| X:[guiding]:[consistency] | guiding | consistency | coherence trace standard | 0 | NO_ITEMS | No warranted item under available docs. |
| X:[applying]:[necessity] | applying | necessity | execution validation basis | 0 | NO_ITEMS | No warranted item under available docs. |
| X:[applying]:[sufficiency] | applying | sufficiency | implementation evidence threshold | 0 | NO_ITEMS | No warranted item under available docs. |
| X:[applying]:[completeness] | applying | completeness | workflow coverage proof | 1 | HAS_ITEMS | Fixture provenance manifest needs coverage. |
| X:[applying]:[consistency] | applying | consistency | process coherence trace | 0 | NO_ITEMS | No warranted item under available docs. |
| X:[judging]:[necessity] | judging | necessity | acceptance validation basis | 0 | NO_ITEMS | No warranted item under available docs. |
| X:[judging]:[sufficiency] | judging | sufficiency | decision evidence threshold | 0 | NO_ITEMS | No warranted item under available docs. |
| X:[judging]:[completeness] | judging | completeness | acceptance coverage proof | 0 | NO_ITEMS | No warranted item under available docs. |
| X:[judging]:[consistency] | judging | consistency | decision coherence trace | 0 | NO_ITEMS | No warranted item under available docs. |
| X:[reviewing]:[necessity] | reviewing | necessity | audit validation basis | 0 | NO_ITEMS | No warranted item under available docs. |
| X:[reviewing]:[sufficiency] | reviewing | sufficiency | audit evidence threshold | 0 | NO_ITEMS | No warranted item under available docs. |
| X:[reviewing]:[completeness] | reviewing | completeness | assurance coverage proof | 0 | NO_ITEMS | No warranted item under available docs. |
| X:[reviewing]:[consistency] | reviewing | consistency | audit coherence trace | 1 | HAS_ITEMS | Hash/canonicalization scope needs explicit acceptance criteria. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| X-001 | X:[reviewing]:[consistency] | VerificationGap | Specification | Specification | Add acceptance criteria for when JCS-compatible hash metadata applies and what payload scope is hashed. | The documents require JCS-compatible hashing where JSON payload hashes are used, but do not define the payload boundary, metadata fields, or deferral rule for cases where hashes are not yet used. | Specification.md; Datasheet.md; Guidance.md | Specification Requirements REQ-02-01-09 and Verification; Datasheet Attributes and Construction; Guidance Trade-offs | NA | PROPOSAL: require hash-scope documentation or explicit deferral in schema review evidence. | TBD |
| X-002 | X:[applying]:[completeness] | VerificationGap | Procedure | Procedure | Add a fixture provenance manifest expectation for public, invented, or permissively licensed examples. | REQ-02-01-11 and the procedure require safe example data, but no fixture manifest or provenance label convention is specified for future validation evidence. | Procedure.md; Specification.md; Guidance.md | Procedure Step 8 and Records; Specification Requirements REQ-02-01-11; Guidance Public examples trade-off | NA | PROPOSAL: define a minimal fixture provenance manifest before accepting example-based tests. | TBD |

## Matrix E - Evaluation

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
| E:[judging]:[wisdom] | judging | wisdom | reasoned acceptance assurance | 1 | HAS_ITEMS | Human-ruling capture path needs rationale. |
| E:[reviewing]:[data] | reviewing | data | audit fact assurance | 0 | NO_ITEMS | No warranted item under available docs. |
| E:[reviewing]:[information] | reviewing | information | audit context assurance | 1 | HAS_ITEMS | Diagnostics/result/report crosswalk is missing. |
| E:[reviewing]:[knowledge] | reviewing | knowledge | expertise audit assurance | 0 | NO_ITEMS | No warranted item under available docs. |
| E:[reviewing]:[wisdom] | reviewing | wisdom | reasoned audit assurance | 0 | NO_ITEMS | No warranted item under available docs. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| E-001 | E:[reviewing]:[information] | MissingSlot | Specification | Specification | Add a crosswalk from diagnostic/result envelope fields to Result and Report schema records. | REQ-02-01-08 lists required diagnostic/result-envelope compatibility, and `Datasheet.md` lists result/report records, but no crosswalk shows where code, class, severity, source, affected object, message, remediation, and provenance land. | Specification.md; Datasheet.md; Guidance.md | Specification Requirements REQ-02-01-08 and Verification; Datasheet Attributes and Construction; Guidance Examples | NA | PROPOSAL: add a field-level crosswalk or explicit reference before asserting downstream report compatibility. | TBD |
| E-002 | E:[judging]:[wisdom] | RationaleGap | Guidance | Guidance | Explain where human rulings for conflicts and TBD schema decisions are captured before schema acceptance. | `Guidance.md` has unresolved human-ruling rows and `Specification.md` asks for human review notes, but the production documents do not define the durable ruling record or how acceptance evidence references it. | Guidance.md; Specification.md; Procedure.md | Guidance Conflict Table; Specification Documentation; Procedure Step 10 and Records | NA | PROPOSAL: add a ruling-record reference or handoff note before downstream acceptance claims depend on those decisions. | TBD |
