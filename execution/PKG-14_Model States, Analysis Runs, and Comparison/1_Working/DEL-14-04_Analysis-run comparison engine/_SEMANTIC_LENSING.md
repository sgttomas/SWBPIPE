# Semantic Lensing Register: DEL-14-04 Analysis-run comparison engine

**Generated:** 2026-05-03
**Deliverable Folder:** execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-04_Analysis-run comparison engine
**Warnings:** none

**Inputs Read:**
- _CONTEXT.md - deliverable identity and scope
- _STATUS.md - SEMANTIC_READY after semantic-matrix-build
- _SEMANTIC.md - matrix source
- Datasheet.md - production document
- Specification.md - production document
- Guidance.md - production document
- Procedure.md - production document
- _REFERENCES.md - governing pointer list

**Purpose:** Apply `semantic-matrix-build` matrix cells as lenses over the production documents, capturing warranted enrichment inputs for a later enrichment pass.

## Summary

- Total warranted items: 5
- By document:
  - Datasheet: 0
  - Specification: 3
  - Guidance: 1
  - Procedure: 1
- By matrix:
  - A: 0  B: 0  C: 1  F: 1  D: 1  X: 1  E: 1
- By type:
  - Conflict: 0
  - VerificationGap: 2
  - MissingSlot: 1
  - WeakStatement: 0
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
| A:[normative]:[guiding] | normative | guiding | prescriptive direction | 0 | NO_ITEMS | Production docs already identify governing direction conservatively. |
| A:[normative]:[applying] | normative | applying | mandatory practice | 0 | NO_ITEMS | No separate warranted item beyond existing requirements. |
| A:[normative]:[judging] | normative | judging | compliance determination | 0 | NO_ITEMS | Boundary language prevents compliance overclaim. |
| A:[normative]:[reviewing] | normative | reviewing | regulatory audit | 0 | NO_ITEMS | Audit boundary is stated as diagnostic only. |
| A:[operative]:[guiding] | operative | guiding | procedural direction | 0 | NO_ITEMS | Procedure includes conservative production steps. |
| A:[operative]:[applying] | operative | applying | practical execution | 0 | NO_ITEMS | Implementation details remain TBD where unsupported. |
| A:[operative]:[judging] | operative | judging | performance assessment | 0 | NO_ITEMS | Verification hooks exist at draft level. |
| A:[operative]:[reviewing] | operative | reviewing | process audit | 0 | NO_ITEMS | Records section captures audit evidence categories. |
| A:[evaluative]:[guiding] | evaluative | guiding | value orientation | 0 | NO_ITEMS | Guidance states purpose and principles. |
| A:[evaluative]:[applying] | evaluative | applying | merit application | 0 | NO_ITEMS | No additional warranted item. |
| A:[evaluative]:[judging] | evaluative | judging | worth determination | 0 | NO_ITEMS | Professional boundary prevents overclaim. |
| A:[evaluative]:[reviewing] | evaluative | reviewing | quality appraisal | 0 | NO_ITEMS | Open questions are captured elsewhere. |

## Matrix B - Conceptualization

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| B:[data]:[necessity] | data | necessity | essential fact | 0 | NO_ITEMS | Identification and scope facts are present. |
| B:[data]:[sufficiency] | data | sufficiency | adequate evidence | 0 | NO_ITEMS | Source basis is listed. |
| B:[data]:[completeness] | data | completeness | comprehensive record | 0 | NO_ITEMS | Missing implementation values are marked TBD. |
| B:[data]:[consistency] | data | consistency | reliable measurement | 0 | NO_ITEMS | Unit constraints are consistently stated. |
| B:[information]:[necessity] | information | necessity | essential signal | 0 | NO_ITEMS | Scope signals are present. |
| B:[information]:[sufficiency] | information | sufficiency | adequate context | 0 | NO_ITEMS | Decomposition and register context is present. |
| B:[information]:[completeness] | information | completeness | comprehensive account | 0 | NO_ITEMS | Draft account is conservative. |
| B:[information]:[consistency] | information | consistency | coherent message | 0 | NO_ITEMS | Terminology is consistent. |
| B:[knowledge]:[necessity] | knowledge | necessity | fundamental understanding | 0 | NO_ITEMS | Purpose and boundaries are present. |
| B:[knowledge]:[sufficiency] | knowledge | sufficiency | competent expertise | 0 | NO_ITEMS | No unsupported engineering expertise is claimed. |
| B:[knowledge]:[completeness] | knowledge | completeness | thorough mastery | 0 | NO_ITEMS | Missing contract details remain TBD. |
| B:[knowledge]:[consistency] | knowledge | consistency | coherent understanding | 0 | NO_ITEMS | Cross-document terminology is aligned. |
| B:[wisdom]:[necessity] | wisdom | necessity | essential discernment | 0 | NO_ITEMS | Professional-boundary discernment is explicit. |
| B:[wisdom]:[sufficiency] | wisdom | sufficiency | adequate judgment | 0 | NO_ITEMS | Human/product decisions remain outside this draft. |
| B:[wisdom]:[completeness] | wisdom | completeness | holistic insight | 0 | NO_ITEMS | Open questions preserve missing decisions. |
| B:[wisdom]:[consistency] | wisdom | consistency | principled reasoning | 0 | NO_ITEMS | Governance constraints are consistently applied. |

## Matrix C - Formulation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| C:[normative]:[necessity] | normative | necessity | Binding input basis | 0 | NO_ITEMS | Required input basis is identified. |
| C:[normative]:[sufficiency] | normative | sufficiency | Adequate proof basis | 0 | NO_ITEMS | Evidence posture is conservative. |
| C:[normative]:[completeness] | normative | completeness | Whole record basis | 0 | NO_ITEMS | Required metadata categories are named. |
| C:[normative]:[consistency] | normative | consistency | Coherent control basis | 0 | NO_ITEMS | Boundary controls are consistent. |
| C:[operative]:[necessity] | operative | necessity | Required execution input | 0 | NO_ITEMS | Prerequisites are present. |
| C:[operative]:[sufficiency] | operative | sufficiency | Workable evidence basis | 0 | NO_ITEMS | Source basis is stated. |
| C:[operative]:[completeness] | operative | completeness | Full workflow record | 1 | HAS_ITEMS | Concrete output schema/API record shape remains TBD. |
| C:[operative]:[consistency] | operative | consistency | Reliable process signal | 0 | NO_ITEMS | Procedure and specification align. |
| C:[evaluative]:[necessity] | evaluative | necessity | Critical appraisal input | 0 | NO_ITEMS | Diagnostic/audit role is explicit. |
| C:[evaluative]:[sufficiency] | evaluative | sufficiency | Judged evidence basis | 0 | NO_ITEMS | No unsupported validation claim. |
| C:[evaluative]:[completeness] | evaluative | completeness | Complete review basis | 0 | NO_ITEMS | Open questions cover missing decisions. |
| C:[evaluative]:[consistency] | evaluative | consistency | Stable appraisal message | 0 | NO_ITEMS | Professional-boundary language is stable. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| C-001 | C:[operative]:[completeness] | MissingSlot | Specification | Specification | Add TBD slot for final comparison output schema/API shape. | The production docs require deterministic output and result delta tests, but the concrete comparison record shape is still unresolved. | Specification.md; Procedure.md | Specification.md#Documentation; Procedure.md#Records | NA | PROPOSAL | TBD |

## Matrix F - Requirements

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| F:[normative]:[necessity] | normative | necessity | Enforceable input condition | 0 | NO_ITEMS | Requirements identify input basis. |
| F:[normative]:[sufficiency] | normative | sufficiency | Evidenced acceptance basis | 1 | HAS_ITEMS | Acceptance criteria need concrete fixtures later. |
| F:[normative]:[completeness] | normative | completeness | Exhaustive obligation record | 0 | NO_ITEMS | Scope categories are recorded. |
| F:[normative]:[consistency] | normative | consistency | Harmonized control rule | 0 | NO_ITEMS | Governance constraints align. |
| F:[operative]:[necessity] | operative | necessity | Actionable work condition | 0 | NO_ITEMS | Procedure prerequisites are present. |
| F:[operative]:[sufficiency] | operative | sufficiency | Proven execution basis | 0 | NO_ITEMS | Execution basis remains source-grounded. |
| F:[operative]:[completeness] | operative | completeness | Whole delivery account | 0 | NO_ITEMS | Records section names expected evidence. |
| F:[operative]:[consistency] | operative | consistency | Stable workflow rule | 0 | NO_ITEMS | Steps follow specification terms. |
| F:[evaluative]:[necessity] | evaluative | necessity | Decisive appraisal condition | 0 | NO_ITEMS | Boundary condition is explicit. |
| F:[evaluative]:[sufficiency] | evaluative | sufficiency | Warranted review basis | 0 | NO_ITEMS | Review evidence remains non-authoritative. |
| F:[evaluative]:[completeness] | evaluative | completeness | Integrated judgment record | 0 | NO_ITEMS | No additional item. |
| F:[evaluative]:[consistency] | evaluative | consistency | Aligned quality rule | 0 | NO_ITEMS | Quality rule remains draft/TBD where needed. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| F-001 | F:[normative]:[sufficiency] | VerificationGap | Multi | Specification | Add acceptance criteria for result delta fixtures once upstream contracts are available. | Verification lists deterministic, mapping, unit, diagnostics, settings, and tolerance checks, but concrete fixtures and commands remain TBD. | Specification.md; Procedure.md | Specification.md#Verification; Procedure.md#Verification | NA | PROPOSAL | TBD |

## Matrix D - Objectives

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| D:[normative]:[guiding] | normative | guiding | Authoritative closure path | 0 | NO_ITEMS | Closure is constrained by governance. |
| D:[normative]:[applying] | normative | applying | Binding execution closure | 0 | NO_ITEMS | Requirements preserve boundaries. |
| D:[normative]:[judging] | normative | judging | Compliance closure rationale | 0 | NO_ITEMS | Compliance overclaim is avoided. |
| D:[normative]:[reviewing] | normative | reviewing | Audit closure record | 0 | NO_ITEMS | Audit records are non-authoritative. |
| D:[operative]:[guiding] | operative | guiding | Directed work closure | 0 | NO_ITEMS | Work steps are present. |
| D:[operative]:[applying] | operative | applying | Practical completion route | 1 | HAS_ITEMS | Mapping/tolerance workflow is open. |
| D:[operative]:[judging] | operative | judging | Performance closure evidence | 0 | NO_ITEMS | Verification evidence is identified. |
| D:[operative]:[reviewing] | operative | reviewing | Process closure trail | 0 | NO_ITEMS | Records section exists. |
| D:[evaluative]:[guiding] | evaluative | guiding | Value closure compass | 0 | NO_ITEMS | Purpose is clear. |
| D:[evaluative]:[applying] | evaluative | applying | Merited action closure | 0 | NO_ITEMS | No unsupported value decision. |
| D:[evaluative]:[judging] | evaluative | judging | Worth closure finding | 0 | NO_ITEMS | Human decision rights preserved. |
| D:[evaluative]:[reviewing] | evaluative | reviewing | Quality closure appraisal | 0 | NO_ITEMS | No additional warranted item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| D-001 | D:[operative]:[applying] | TBD_Question | Specification | Specification | TBD: accepted tolerance defaults and mapping workflow source. | Guidance records OI-014 as open, and requirements depend on tolerance profiles and manual mappings. | Guidance.md; Specification.md | Guidance.md#Open Questions; Specification.md#Requirements | NA | PROPOSAL | TBD |

## Matrix X - Verification

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| X:[guiding]:[necessity] | guiding | necessity | Directed evidence need | 0 | NO_ITEMS | Evidence need is named. |
| X:[guiding]:[sufficiency] | guiding | sufficiency | Sufficient proof route | 0 | NO_ITEMS | Proof route remains TBD where unsupported. |
| X:[guiding]:[completeness] | guiding | completeness | Whole coverage path | 0 | NO_ITEMS | Coverage is listed by category. |
| X:[guiding]:[consistency] | guiding | consistency | Aligned instruction trace | 0 | NO_ITEMS | Cross-document terms align. |
| X:[applying]:[necessity] | applying | necessity | Executable input check | 0 | NO_ITEMS | Input checks are included. |
| X:[applying]:[sufficiency] | applying | sufficiency | Proven application basis | 0 | NO_ITEMS | Verification remains source-bound. |
| X:[applying]:[completeness] | applying | completeness | Total work coverage | 0 | NO_ITEMS | Test categories are listed. |
| X:[applying]:[consistency] | applying | consistency | Stable action trace | 0 | NO_ITEMS | Procedure steps are stable. |
| X:[judging]:[necessity] | judging | necessity | Decisive proof test | 0 | NO_ITEMS | Determinism test is named. |
| X:[judging]:[sufficiency] | judging | sufficiency | Accepted evidence test | 0 | NO_ITEMS | Evidence remains TBD where needed. |
| X:[judging]:[completeness] | judging | completeness | Comprehensive verdict basis | 0 | NO_ITEMS | Verification table is complete at category level. |
| X:[judging]:[consistency] | judging | consistency | Coherent decision trail | 0 | NO_ITEMS | Boundary checks are coherent. |
| X:[reviewing]:[necessity] | reviewing | necessity | Audit evidence need | 0 | NO_ITEMS | Audit evidence categories are present. |
| X:[reviewing]:[sufficiency] | reviewing | sufficiency | Adequate trace proof | 0 | NO_ITEMS | Trace proof remains implementation-dependent. |
| X:[reviewing]:[completeness] | reviewing | completeness | Exhaustive inspection record | 0 | NO_ITEMS | Records are named. |
| X:[reviewing]:[consistency] | reviewing | consistency | Reliable audit trail | 1 | HAS_ITEMS | Deterministic ordering/hash fixture evidence remains TBD. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| X-001 | X:[reviewing]:[consistency] | VerificationGap | Procedure | Procedure | Add concrete deterministic ordering/hash comparison fixture checks. | Procedure requires deterministic output but leaves fixture set, paths, commands, and hash/output basis as TBD. | Procedure.md; Specification.md | Procedure.md#Verification; Specification.md#Verification | NA | PROPOSAL | TBD |

## Matrix E - Evaluation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| E:[guiding]:[data] | guiding | data | Directed fact evidence | 0 | NO_ITEMS | Facts are source-cited. |
| E:[guiding]:[information] | guiding | information | Framed context proof | 0 | NO_ITEMS | Context is framed conservatively. |
| E:[guiding]:[knowledge] | guiding | knowledge | Interpreted mastery trace | 0 | NO_ITEMS | No unsupported mastery claim. |
| E:[guiding]:[wisdom] | guiding | wisdom | Discerned rationale path | 0 | NO_ITEMS | Rationale path is present. |
| E:[applying]:[data] | applying | data | Applied fact check | 0 | NO_ITEMS | Operational facts are present. |
| E:[applying]:[information] | applying | information | Contextual execution proof | 0 | NO_ITEMS | Execution context is present. |
| E:[applying]:[knowledge] | applying | knowledge | Mastered work evidence | 0 | NO_ITEMS | Implementation evidence remains TBD. |
| E:[applying]:[wisdom] | applying | wisdom | Judged action rationale | 0 | NO_ITEMS | Decisions are not invented. |
| E:[judging]:[data] | judging | data | Verdict fact basis | 0 | NO_ITEMS | Result classification remains bounded. |
| E:[judging]:[information] | judging | information | Message proof basis | 0 | NO_ITEMS | Boundary message is stable. |
| E:[judging]:[knowledge] | judging | knowledge | Understanding verdict trace | 0 | NO_ITEMS | No additional item. |
| E:[judging]:[wisdom] | judging | wisdom | Reasoned merit basis | 0 | NO_ITEMS | Human ruling preserved. |
| E:[reviewing]:[data] | reviewing | data | Audit fact trail | 0 | NO_ITEMS | Audit facts are recorded. |
| E:[reviewing]:[information] | reviewing | information | Context audit proof | 0 | NO_ITEMS | Audit context is present. |
| E:[reviewing]:[knowledge] | reviewing | knowledge | Mastery audit record | 0 | NO_ITEMS | No implementation evidence invented. |
| E:[reviewing]:[wisdom] | reviewing | wisdom | Principled audit rationale | 1 | HAS_ITEMS | Export/report consumer rationale remains bounded by upstream work. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| E-001 | E:[reviewing]:[wisdom] | RationaleGap | Guidance | Guidance | Clarify downstream export/report consumer rationale after upstream contracts mature. | The documents correctly state diagnostic/audit boundaries, but final report/export consumer behavior depends on DEL-08-04 and DEL-14-05 contracts. | Guidance.md; Datasheet.md | Guidance.md#Considerations; Datasheet.md#Construction | NA | PROPOSAL | TBD |
