# Semantic Lensing Register: DEL-02-02 Unit system and dimensional-analysis core contract

**Generated:** 2026-04-30
**Deliverable Folder:** `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract`
**Warnings:** None

**Inputs Read:**
- `_CONTEXT.md` - deliverable identity, package scope, architecture-basis injection.
- `_STATUS.md` - current lifecycle state observed as `SEMANTIC_READY`.
- `_SEMANTIC.md` - matrices A, B, C, F, D, X, E used as lenses only.
- `Datasheet.md` - production document read.
- `Specification.md` - production document read.
- `Guidance.md` - production document read.
- `Procedure.md` - production document read.
- `_REFERENCES.md` - local reference pointer list read; no references expanded.

**Purpose:** Apply `_SEMANTIC.md` matrix cells as non-authoritative lenses over the production documents, capturing warranted enrichment inputs for a later enrichment pass. `_SEMANTIC.md` and this register are lens/worklist artifacts only; they do not establish engineering authority.

## Summary

- Total warranted items: 12
- By document:
  - Datasheet: 2
  - Specification: 7
  - Guidance: 2
  - Procedure: 1
- By matrix:
  - A: 2  B: 3  C: 1  F: 2  D: 1  X: 2  E: 1
- By type:
  - Conflict: 1
  - VerificationGap: 2
  - MissingSlot: 3
  - WeakStatement: 0
  - RationaleGap: 0
  - Normalization: 0
  - TBD_Question: 6
  - MatrixError: 0
- Notable conflicts: 1
- Matrix parse errors: 0

## Matrix A - Orientation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| A:[normative]:[guiding] | normative | guiding | prescriptive direction | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| A:[normative]:[applying] | normative | applying | mandatory practice | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| A:[normative]:[judging] | normative | judging | compliance determination | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| A:[normative]:[reviewing] | normative | reviewing | regulatory audit | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| A:[operative]:[guiding] | operative | guiding | procedural direction | 1 | HAS_ITEMS | Human decision dependencies are not assigned. |
| A:[operative]:[applying] | operative | applying | practical execution | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| A:[operative]:[judging] | operative | judging | performance assessment | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| A:[operative]:[reviewing] | operative | reviewing | process audit | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| A:[evaluative]:[guiding] | evaluative | guiding | value orientation | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| A:[evaluative]:[applying] | evaluative | applying | merit application | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| A:[evaluative]:[judging] | evaluative | judging | worth determination | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| A:[evaluative]:[reviewing] | evaluative | reviewing | quality appraisal | 1 | HAS_ITEMS | Existing revision-basis conflict remains unresolved. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| A-001 | A:[operative]:[guiding] | MissingSlot | Procedure | Procedure | Record TBD: identify human reviewer or decision owner for open unit catalog, dimension, conversion, storage, and special-quantity decisions. | The procedure lists the human dependency list as not provided while multiple later steps require human-approved decisions. A later enrichment pass needs an owner/gate signal without inventing one. | `Procedure.md` | `## Prerequisites`; `## Steps` steps 3-6 and 11 | NA | Add a dependency-owner placeholder or review-gate reference in the procedure. | TBD |
| A-002 | A:[evaluative]:[reviewing] | Conflict | Guidance | NA | Carry the existing decomposition revision-basis conflict to human ruling; do not resolve it in the lens register. | The production guidance already records an unresolved conflict between repository overview text and the sealed context basis. The register should preserve the conflict for review rather than selecting an authority. | `Guidance.md`; `_CONTEXT.md` | `Guidance.md##Conflict Table (for human ruling)`; `_CONTEXT.md##Decomposition Reference` and `##Architecture Basis Injection` | `Guidance.md#Conflict Table (for human ruling): docs/README.md governance map text`; `_CONTEXT.md#Decomposition Reference and Architecture Basis Injection` | Use sealed context for this deliverable only after human ruling, if accepted. | TBD |

## Matrix B - Conceptualization

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| B:[data]:[necessity] | data | necessity | essential fact | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| B:[data]:[sufficiency] | data | sufficiency | adequate evidence | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| B:[data]:[completeness] | data | completeness | comprehensive record | 1 | HAS_ITEMS | Unit catalog and conversion source set are missing. |
| B:[data]:[consistency] | data | consistency | reliable measurement | 1 | HAS_ITEMS | Dimensionless classification is not explicit enough. |
| B:[information]:[necessity] | information | necessity | essential signal | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| B:[information]:[sufficiency] | information | sufficiency | adequate context | 1 | HAS_ITEMS | Schema/tooling placement remains TBD. |
| B:[information]:[completeness] | information | completeness | comprehensive account | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| B:[information]:[consistency] | information | consistency | coherent message | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| B:[knowledge]:[necessity] | knowledge | necessity | fundamental understanding | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| B:[knowledge]:[sufficiency] | knowledge | sufficiency | competent expertise | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| B:[knowledge]:[completeness] | knowledge | completeness | thorough mastery | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| B:[knowledge]:[consistency] | knowledge | consistency | coherent understanding | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| B:[wisdom]:[necessity] | wisdom | necessity | essential discernment | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| B:[wisdom]:[sufficiency] | wisdom | sufficiency | adequate judgment | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| B:[wisdom]:[completeness] | wisdom | completeness | holistic insight | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| B:[wisdom]:[consistency] | wisdom | consistency | principled reasoning | 0 | NO_ITEMS | No warranted enrichment item after scan. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| B-001 | B:[data]:[completeness] | TBD_Question | Datasheet | Datasheet | TBD: identify the source-backed or decision-backed minimal unit catalog and conversion source set for early tests. | The datasheet states that no authoritative unit catalog or conversion table was supplied, while the specification and guidance require provenance-backed unit and conversion records. | `Datasheet.md`; `Specification.md`; `Guidance.md` | `Datasheet.md##Attributes`; `Specification.md##Open Contract Decisions`; `Guidance.md###Unit catalog and conversion sources` | NA | Prefer a small provenance-backed catalog or explicit project decision record before implementation relies on it. | TBD |
| B-002 | B:[data]:[consistency] | MissingSlot | Specification | Specification | Add explicit classification for dimensionless quantities, ratios, percentages, unitless coefficients, and unitless-where-unit-required diagnostics. | The requirement allows explicit dimensionless fields, and the procedure names ratios and percentages, but the production docs do not yet provide a coherent classification or testable boundary. | `Specification.md`; `Datasheet.md`; `Procedure.md` | `Specification.md##Requirements` U-003; `Datasheet.md##Construction` Diagnostics; `Procedure.md##Steps` step 6 | NA | Add a dimensionless-classification subsection or decision item in the specification. | TBD |
| B-003 | B:[information]:[sufficiency] | TBD_Question | Specification | Specification | TBD: decide schema filenames, locations, and code-generation or validation tooling for JSON Schema 2020-12 quantity definitions. | The specification requires JSON Schema 2020-12 hooks but explicitly leaves exact schema location and code-generation tooling TBD, which blocks downstream consumers from finding the contract. | `Specification.md`; `Procedure.md` | `Specification.md##Requirements` U-009; `Specification.md##Documentation`; `Procedure.md##Records` | NA | Record a schema location/tooling decision before schema consumers implement against it. | TBD |

## Matrix C - Formulation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| C:[normative]:[necessity] | normative | necessity | Binding Unit Premise | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| C:[normative]:[sufficiency] | normative | sufficiency | Adequacy Rule Frame | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| C:[normative]:[completeness] | normative | completeness | Coverage Obligation Map | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| C:[normative]:[consistency] | normative | consistency | Coherence Control Rule | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| C:[operative]:[necessity] | operative | necessity | Required Runtime Evidence | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| C:[operative]:[sufficiency] | operative | sufficiency | Executable Unit Context | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| C:[operative]:[completeness] | operative | completeness | Coverage Work Pattern | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| C:[operative]:[consistency] | operative | consistency | Stable Flow Signal | 1 | HAS_ITEMS | Diagnostic code namespace is still open. |
| C:[evaluative]:[necessity] | evaluative | necessity | Review Trigger Basis | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| C:[evaluative]:[sufficiency] | evaluative | sufficiency | Acceptance Evidence Frame | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| C:[evaluative]:[completeness] | evaluative | completeness | Assurance Coverage Model | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| C:[evaluative]:[consistency] | evaluative | consistency | Quality Coherence Basis | 0 | NO_ITEMS | No warranted enrichment item after scan. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| C-001 | C:[operative]:[consistency] | TBD_Question | Specification | Specification | TBD: define the unit diagnostic code namespace and mapping to result-envelope fields. | The docs require structured diagnostics and name several unit error classes, but exact diagnostic code names remain TBD. Runtime consumers need stable codes for repeatable flow and tests. | `Specification.md`; `Datasheet.md`; `Procedure.md` | `Specification.md##Requirements` U-010; `Datasheet.md##Construction` Diagnostics; `Procedure.md##Steps` step 8 | NA | Place a diagnostic-code decision or placeholder table in the specification. | TBD |

## Matrix F - Requirements

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| F:[normative]:[necessity] | normative | necessity | Mandated Unit Conditions | 1 | HAS_ITEMS | Dimension basis requires a human-approved decision. |
| F:[normative]:[sufficiency] | normative | sufficiency | Rule Evidence Threshold | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| F:[normative]:[completeness] | normative | completeness | Contract Coverage Standard | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| F:[normative]:[consistency] | normative | consistency | Governed Coherence Standard | 1 | HAS_ITEMS | Alias ambiguity lacks explicit verification. |
| F:[operative]:[necessity] | operative | necessity | Runtime Unit Preconditions | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| F:[operative]:[sufficiency] | operative | sufficiency | Executable Evidence Threshold | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| F:[operative]:[completeness] | operative | completeness | Workflow Coverage Standard | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| F:[operative]:[consistency] | operative | consistency | Deterministic Signal Standard | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| F:[evaluative]:[necessity] | evaluative | necessity | Review Evidence Preconditions | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| F:[evaluative]:[sufficiency] | evaluative | sufficiency | Acceptance Evidence Threshold | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| F:[evaluative]:[completeness] | evaluative | completeness | Assurance Coverage Standard | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| F:[evaluative]:[consistency] | evaluative | consistency | Quality Coherence Standard | 0 | NO_ITEMS | No warranted enrichment item after scan. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| F-001 | F:[normative]:[necessity] | TBD_Question | Specification | Specification | TBD: obtain human-approved base dimension vector, derived-dimension rules, and dimension identifier vocabulary before schema/API freeze. | The specification requires stable dimensional signatures but leaves the base dimensions, derived dimensions, and identifier vocabulary unresolved. | `Specification.md`; `Datasheet.md`; `Procedure.md` | `Specification.md##Requirements` U-004; `Specification.md##Open Contract Decisions`; `Datasheet.md##Attributes` Dimensional basis; `Procedure.md##Steps` step 4 | NA | Treat the dimension basis as an explicit design decision before freezing schemas or APIs. | TBD |
| F-002 | F:[normative]:[consistency] | VerificationGap | Specification | Specification | Add acceptance tests or checks for unit identifier aliases and ambiguous parsing rejection. | Alias ambiguity is named as an open decision, but the verification matrix does not explicitly call out alias parsing or ambiguous alias rejection. | `Specification.md`; `Procedure.md`; `Guidance.md` | `Specification.md##Open Contract Decisions`; `Specification.md##Verification`; `Procedure.md##Steps` step 5; `Guidance.md##Trade-offs` | NA | Add parser/alias verification once the unit identifier namespace is chosen. | TBD |

## Matrix D - Objectives

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| D:[normative]:[guiding] | normative | guiding | Directed Unit Obligation | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| D:[normative]:[applying] | normative | applying | Enforced Evidence Practice | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| D:[normative]:[judging] | normative | judging | Coverage Decision Rule | 1 | HAS_ITEMS | Storage/canonicalization choice remains proposal-only. |
| D:[normative]:[reviewing] | normative | reviewing | Coherence Audit Standard | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| D:[operative]:[guiding] | operative | guiding | Execution Preconditions Path | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| D:[operative]:[applying] | operative | applying | Runtime Evidence Practice | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| D:[operative]:[judging] | operative | judging | Coverage Assessment Method | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| D:[operative]:[reviewing] | operative | reviewing | Deterministic Process Check | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| D:[evaluative]:[guiding] | evaluative | guiding | Review Basis Orientation | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| D:[evaluative]:[applying] | evaluative | applying | Acceptance Evidence Use | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| D:[evaluative]:[judging] | evaluative | judging | Assurance Decision Basis | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| D:[evaluative]:[reviewing] | evaluative | reviewing | Quality Appraisal Check | 0 | NO_ITEMS | No warranted enrichment item after scan. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| D-001 | D:[normative]:[judging] | TBD_Question | Datasheet | Specification | TBD: obtain human ruling on persisted shape: entered unit, canonical calculation representation, or both, including hash canonicalization behavior. | The datasheet and guidance describe a proposal or recommended draft direction, while the procedure says the persisted representation is still a decision. This is not a conflict, but it is a consequential open contract decision. | `Datasheet.md`; `Guidance.md`; `Procedure.md`; `Specification.md` | `Datasheet.md##Construction` Storage convention; `Guidance.md###Storage and reproducibility`; `Procedure.md##Steps` step 3; `Specification.md##Open Contract Decisions` | NA | Resolve storage/canonicalization as a specification decision or ADR before persistence-facing contracts depend on it. | TBD |

## Matrix X - Verification

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| X:[guiding]:[necessity] | guiding | necessity | Initiation Evidence Gate | 1 | HAS_ITEMS | Special-quantity semantics gate remains open. |
| X:[guiding]:[sufficiency] | guiding | sufficiency | Context Readiness Gate | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| X:[guiding]:[completeness] | guiding | completeness | Coverage Readiness Gate | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| X:[guiding]:[consistency] | guiding | consistency | Coherence Readiness Gate | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| X:[applying]:[necessity] | applying | necessity | Practice Entry Gate | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| X:[applying]:[sufficiency] | applying | sufficiency | Execution Evidence Gate | 1 | HAS_ITEMS | Conversion tests depend on unresolved constants/tolerances. |
| X:[applying]:[completeness] | applying | completeness | Workflow Coverage Gate | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| X:[applying]:[consistency] | applying | consistency | Signal Stability Gate | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| X:[judging]:[necessity] | judging | necessity | Decision Evidence Gate | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| X:[judging]:[sufficiency] | judging | sufficiency | Assessment Context Gate | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| X:[judging]:[completeness] | judging | completeness | Coverage Decision Gate | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| X:[judging]:[consistency] | judging | consistency | Coherence Decision Gate | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| X:[reviewing]:[necessity] | reviewing | necessity | Audit Evidence Gate | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| X:[reviewing]:[sufficiency] | reviewing | sufficiency | Process Context Gate | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| X:[reviewing]:[completeness] | reviewing | completeness | Audit Coverage Gate | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| X:[reviewing]:[consistency] | reviewing | consistency | Process Coherence Gate | 0 | NO_ITEMS | No warranted enrichment item after scan. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| X-001 | X:[applying]:[sufficiency] | VerificationGap | Specification | Specification | Add gated deterministic conversion test placeholders that cannot become executable until exact constants and tolerances are approved. | The verification matrix requires deterministic conversion cases but leaves exact constants and tolerances TBD. The gap should stay visible so tests are not filled with unsupported values. | `Specification.md`; `Guidance.md`; `Procedure.md` | `Specification.md##Verification`; `Specification.md##Open Contract Decisions`; `Guidance.md##Trade-offs`; `Procedure.md##Verification` | NA | Gate executable conversion tests on approved constants, representation, and tolerance policy. | TBD |
| X-002 | X:[guiding]:[necessity] | TBD_Question | Specification | Specification | TBD: resolve offset temperature versus interval, gauge versus absolute pressure, angle/rotation, ratios, and percentages before enabling related conversions. | Special quantity semantics are repeatedly called out as unresolved; enabling related conversions without accepted semantics would create hidden assumptions. | `Specification.md`; `Guidance.md`; `Procedure.md` | `Specification.md##Open Contract Decisions`; `Guidance.md###Offset and reference quantities`; `Procedure.md##Steps` step 6 | NA | Treat special-quantity semantics as preconditions for conversion support. | TBD |

## Matrix E - Evaluation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| E:[guiding]:[data] | guiding | data | Fact Basis Assurance | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| E:[guiding]:[information] | guiding | information | Context Basis Assurance | 1 | HAS_ITEMS | Examples remain absent pending safe sources and decisions. |
| E:[guiding]:[knowledge] | guiding | knowledge | Expertise Basis Assurance | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| E:[guiding]:[wisdom] | guiding | wisdom | Discernment Basis Assurance | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| E:[applying]:[data] | applying | data | Fact Practice Assurance | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| E:[applying]:[information] | applying | information | Context Practice Assurance | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| E:[applying]:[knowledge] | applying | knowledge | Expertise Practice Assurance | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| E:[applying]:[wisdom] | applying | wisdom | Judgment Practice Assurance | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| E:[judging]:[data] | judging | data | Fact Decision Assurance | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| E:[judging]:[information] | judging | information | Context Decision Assurance | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| E:[judging]:[knowledge] | judging | knowledge | Expertise Decision Assurance | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| E:[judging]:[wisdom] | judging | wisdom | Reasoning Decision Assurance | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| E:[reviewing]:[data] | reviewing | data | Fact Audit Assurance | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| E:[reviewing]:[information] | reviewing | information | Context Audit Assurance | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| E:[reviewing]:[knowledge] | reviewing | knowledge | Expertise Audit Assurance | 0 | NO_ITEMS | No warranted enrichment item after scan. |
| E:[reviewing]:[wisdom] | reviewing | wisdom | Reasoning Audit Assurance | 0 | NO_ITEMS | No warranted enrichment item after scan. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| E-001 | E:[guiding]:[information] | MissingSlot | Guidance | Guidance | Add source-safe invented examples only after schema, unit catalog, and conversion decisions are accepted. | The guidance examples section is explicitly TBD while listing example categories needed for future understanding and QA. The item should remain bounded by provenance and protected-content constraints. | `Guidance.md`; `Specification.md`; `Procedure.md` | `Guidance.md##Examples`; `Specification.md##Requirements` U-014; `Procedure.md##Records` | NA | Use original, invented, public-domain, or permissively licensed fixtures after prerequisite unit decisions are approved. | TBD |
