# Semantic Lensing Register: DEL-07-06 Accessibility and usability baseline

**Generated:** 2026-04-30
**Deliverable Folder:** `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-06_Accessibility and usability baseline`
**Warnings:** none

**Inputs Read:**
- `_CONTEXT.md` - deliverable identity and scope
- `_STATUS.md` - lifecycle state
- `_SEMANTIC.md` - matrices A, B, C, F, D, X, E
- `Datasheet.md` - production document
- `Specification.md` - production document
- `Guidance.md` - production document
- `Procedure.md` - production document
- `_REFERENCES.md` - governing source pointers

**Purpose:** Apply semantic-matrix cells as lenses over the production documents, capturing warranted enrichment inputs for later document or implementation work. This register is not engineering authority.

## Summary

- Total warranted items: 11
- By document:
  - Datasheet: 1
  - Specification: 6
  - Guidance: 2
  - Procedure: 2
- By matrix:
  - A: 2  B: 1  C: 1  F: 3  D: 1  X: 2  E: 1
- By type:
  - Conflict: 1
  - VerificationGap: 6
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
| A:normative:guiding | normative | guiding | prescriptive direction | 1 | HAS_ITEMS | Final target needs human ruling. |
| A:normative:applying | normative | applying | mandatory practice | 0 | NO_ITEMS | Covered by baseline requirements. |
| A:normative:judging | normative | judging | compliance determination | 1 | HAS_ITEMS | Compliance-claim guard needed. |
| A:normative:reviewing | normative | reviewing | regulatory audit | 0 | NO_ITEMS | Covered by review boundary. |
| A:operative:guiding | operative | guiding | procedural direction | 0 | NO_ITEMS | Covered by procedure. |
| A:operative:applying | operative | applying | practical execution | 0 | NO_ITEMS | Implementation deferred. |
| A:operative:judging | operative | judging | performance assessment | 0 | NO_ITEMS | Covered by verification hooks. |
| A:operative:reviewing | operative | reviewing | process audit | 0 | NO_ITEMS | Covered by setup validation notes. |
| A:evaluative:guiding | evaluative | guiding | value orientation | 0 | NO_ITEMS | Covered by guidance principles. |
| A:evaluative:applying | evaluative | applying | merit application | 0 | NO_ITEMS | No extra item. |
| A:evaluative:judging | evaluative | judging | worth determination | 0 | NO_ITEMS | No extra item. |
| A:evaluative:reviewing | evaluative | reviewing | quality appraisal | 0 | NO_ITEMS | No extra item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| A-001 | A:normative:guiding | TBD_Question | Specification | Specification | TBD: record the final accessibility conformance target and applicability. | The scope ledger requires a baseline but explicitly leaves detailed WCAG target unresolved. | `docs/_Registers/ScopeLedger.csv`; `Specification.md` | `SOW-036`; `Specification.md#Standards` |  | PROPOSAL: require human target before measurable conformance tests. | TBD |
| A-002 | A:normative:judging | VerificationGap | Specification | Specification | Add future check that GUI/report text never claims code compliance or certification. | Accessibility and usability controls must not blur professional authority or code-compliance boundaries. | `docs/CONTRACT.md`; `docs/TYPES.md`; `Specification.md` | `OPS-K-AUTH-1`; `docs/TYPES.md#4`; `Specification.md#Verification` |  | PROPOSAL: include product-claims text checks in UI/report test plan. | TBD |

## Matrix B - Conceptualization

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| B:data:necessity | data | necessity | essential fact | 0 | NO_ITEMS | Source rows captured. |
| B:data:sufficiency | data | sufficiency | adequate evidence | 0 | NO_ITEMS | Covered by source basis. |
| B:data:completeness | data | completeness | comprehensive record | 0 | NO_ITEMS | Covered by records. |
| B:data:consistency | data | consistency | reliable measurement | 0 | NO_ITEMS | No numeric targets selected. |
| B:information:necessity | information | necessity | essential signal | 0 | NO_ITEMS | Covered by warning visibility. |
| B:information:sufficiency | information | sufficiency | adequate context | 0 | NO_ITEMS | Covered by reference sections. |
| B:information:completeness | information | completeness | comprehensive account | 0 | NO_ITEMS | Covered by four-doc kit. |
| B:information:consistency | information | consistency | coherent message | 0 | NO_ITEMS | No inconsistency found. |
| B:knowledge:necessity | knowledge | necessity | fundamental understanding | 0 | NO_ITEMS | No issue. |
| B:knowledge:sufficiency | knowledge | sufficiency | competent expertise | 0 | NO_ITEMS | Professional review boundary covered. |
| B:knowledge:completeness | knowledge | completeness | thorough mastery | 0 | NO_ITEMS | No issue. |
| B:knowledge:consistency | knowledge | consistency | coherent understanding | 0 | NO_ITEMS | No issue. |
| B:wisdom:necessity | wisdom | necessity | essential discernment | 0 | NO_ITEMS | No issue. |
| B:wisdom:sufficiency | wisdom | sufficiency | adequate judgment | 1 | HAS_ITEMS | Human target decision needs explicit owner. |
| B:wisdom:completeness | wisdom | completeness | holistic insight | 0 | NO_ITEMS | No issue. |
| B:wisdom:consistency | wisdom | consistency | principled reasoning | 0 | NO_ITEMS | No issue. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| B-001 | B:wisdom:sufficiency | TBD_Question | Guidance | Guidance | TBD: identify who records the accessibility target decision. | The target affects future verification but is not resolved in setup context. | `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`; `Guidance.md` | `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md#1`; `Guidance.md#Principles` |  | PROPOSAL: human project authority records the target before implementation acceptance. | TBD |

## Matrix C - Formulation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| C:normative:necessity | normative | necessity | binding access basis | 0 | NO_ITEMS | Covered by RQ-001 to RQ-004. |
| C:normative:sufficiency | normative | sufficiency | warranted review basis | 0 | NO_ITEMS | Covered by source basis. |
| C:normative:completeness | normative | completeness | complete visibility frame | 1 | HAS_ITEMS | Report-facing accessibility slot can be clearer. |
| C:normative:consistency | normative | consistency | coherent usability frame | 0 | NO_ITEMS | No issue. |
| C:operative:necessity | operative | necessity | required navigation basis | 0 | NO_ITEMS | Covered by keyboard requirement. |
| C:operative:sufficiency | operative | sufficiency | usable execution basis | 0 | NO_ITEMS | Implementation deferred. |
| C:operative:completeness | operative | completeness | whole workflow frame | 0 | NO_ITEMS | Covered by OBJ-006 mapping. |
| C:operative:consistency | operative | consistency | stable interaction frame | 0 | NO_ITEMS | No issue. |
| C:evaluative:necessity | evaluative | necessity | critical review basis | 0 | NO_ITEMS | Covered by verification. |
| C:evaluative:sufficiency | evaluative | sufficiency | defensible judgment basis | 0 | NO_ITEMS | No issue. |
| C:evaluative:completeness | evaluative | completeness | integral review frame | 0 | NO_ITEMS | No issue. |
| C:evaluative:consistency | evaluative | consistency | principled quality frame | 0 | NO_ITEMS | No issue. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| C-001 | C:normative:completeness | MissingSlot | Datasheet | Datasheet | Add report-facing accessibility target as a distinct TBD slot. | SOW-036 includes GUI and reports, while the datasheet can make the report-facing target more explicit. | `docs/_Registers/ScopeLedger.csv`; `Datasheet.md` | `SOW-036`; `Datasheet.md#Attributes` |  | PROPOSAL: track GUI target and report target separately until human ruling. | TBD |

## Matrix F - Requirements

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| F:normative:necessity | normative | necessity | mandatory access threshold | 1 | HAS_ITEMS | Keyboard/name check needed. |
| F:normative:sufficiency | normative | sufficiency | review assurance threshold | 1 | HAS_ITEMS | Contrast target check deferred. |
| F:normative:completeness | normative | completeness | full visibility coverage | 0 | NO_ITEMS | Covered by requirements. |
| F:normative:consistency | normative | consistency | coherent baseline criteria | 0 | NO_ITEMS | No issue. |
| F:operative:necessity | operative | necessity | required navigation evidence | 0 | NO_ITEMS | Covered by future tests. |
| F:operative:sufficiency | operative | sufficiency | adequate workflow proof | 0 | NO_ITEMS | Covered by validation plan. |
| F:operative:completeness | operative | completeness | complete interaction coverage | 1 | HAS_ITEMS | End-to-end workflow check needed. |
| F:operative:consistency | operative | consistency | stable workflow assurance | 0 | NO_ITEMS | No issue. |
| F:evaluative:necessity | evaluative | necessity | essential review evidence | 0 | NO_ITEMS | Covered. |
| F:evaluative:sufficiency | evaluative | sufficiency | reasoned usability assurance | 0 | NO_ITEMS | No issue. |
| F:evaluative:completeness | evaluative | completeness | holistic review coverage | 0 | NO_ITEMS | No issue. |
| F:evaluative:consistency | evaluative | consistency | principled quality assurance | 0 | NO_ITEMS | No issue. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| F-001 | F:normative:necessity | VerificationGap | Specification | Specification | Add future test coverage for keyboard paths and accessible names. | PRD section 21 lists keyboard navigation and labels/tooltips as baseline needs. | `docs/PRD.md`; `Specification.md` | `docs/PRD.md#21`; `Specification.md#Verification` |  | PROPOSAL: use UI/accessibility-tree tests once implementation is authorized. | TBD |
| F-002 | F:normative:sufficiency | VerificationGap | Specification | Specification | Add future contrast/readability test tied to selected target. | High-contrast result visualization is required, but exact measured threshold is TBD. | `docs/PRD.md`; `ScopeLedger.csv`; `Specification.md` | `docs/PRD.md#21`; `SOW-036`; `Specification.md#Verification` |  | PROPOSAL: keep test pending until target decision. | TBD |
| F-003 | F:operative:completeness | VerificationGap | Specification | Specification | Add end-to-end engineering review workflow validation. | OBJ-006 requires model creation, missing data, results, and assumptions to stay visible. | `docs/_Decomposition/SOFTWARE_DECOMP.md`; `Specification.md` | `OBJ-006`; `Specification.md#Verification` |  | PROPOSAL: validate a future walkthrough rather than isolated widgets only. | TBD |

## Matrix D - Objectives

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| D:normative:guiding | normative | guiding | governed access charter | 1 | HAS_ITEMS | Existing conflict needs to remain visible. |
| D:normative:applying | normative | applying | enforceable interaction boundary | 0 | NO_ITEMS | Covered by requirements. |
| D:normative:judging | normative | judging | judged readiness basis | 0 | NO_ITEMS | Covered by verification. |
| D:normative:reviewing | normative | reviewing | audit evidence record | 0 | NO_ITEMS | Covered by records. |
| D:operative:guiding | operative | guiding | executable navigation charter | 0 | NO_ITEMS | Covered by procedure. |
| D:operative:applying | operative | applying | governed workflow protocol | 0 | NO_ITEMS | Covered. |
| D:operative:judging | operative | judging | measured usability basis | 0 | NO_ITEMS | Target TBD. |
| D:operative:reviewing | operative | reviewing | process evidence record | 0 | NO_ITEMS | Covered. |
| D:evaluative:guiding | evaluative | guiding | value-aligned review frame | 0 | NO_ITEMS | Covered by principles. |
| D:evaluative:applying | evaluative | applying | merit-grounded usability frame | 0 | NO_ITEMS | No issue. |
| D:evaluative:judging | evaluative | judging | defensible review basis | 0 | NO_ITEMS | No issue. |
| D:evaluative:reviewing | evaluative | reviewing | quality appraisal record | 0 | NO_ITEMS | No issue. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| D-001 | D:normative:guiding | Conflict | Specification | Specification | Preserve conflict row for baseline requirement vs. target TBD. | The deliverable cannot satisfy a final measurable accessibility target until that target is selected. | `docs/_Registers/ScopeLedger.csv`; `Specification.md`; `Guidance.md` | `SOW-036`; `Specification.md#Conflict Table`; `Guidance.md#Conflict Table` | `ScopeLedger.csv#SOW-036`; `Specification.md#Conflict Table`; `Guidance.md#Conflict Table` | PROPOSAL: defer measurable conformance target to human project authority. | TBD |

## Matrix X - Verification

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| X:guiding:necessity | guiding | necessity | chartered access proof | 0 | NO_ITEMS | Covered. |
| X:guiding:sufficiency | guiding | sufficiency | charter assurance proof | 0 | NO_ITEMS | Covered. |
| X:guiding:completeness | guiding | completeness | charter coverage proof | 0 | NO_ITEMS | Covered. |
| X:guiding:consistency | guiding | consistency | charter coherence proof | 0 | NO_ITEMS | Covered. |
| X:applying:necessity | applying | necessity | interaction evidence proof | 0 | NO_ITEMS | Covered. |
| X:applying:sufficiency | applying | sufficiency | workflow assurance proof | 0 | NO_ITEMS | Covered. |
| X:applying:completeness | applying | completeness | workflow coverage proof | 0 | NO_ITEMS | Covered. |
| X:applying:consistency | applying | consistency | workflow coherence proof | 0 | NO_ITEMS | Covered. |
| X:judging:necessity | judging | necessity | readiness evidence proof | 1 | HAS_ITEMS | Warning separation test needed. |
| X:judging:sufficiency | judging | sufficiency | readiness assurance proof | 0 | NO_ITEMS | Covered. |
| X:judging:completeness | judging | completeness | status coverage proof | 0 | NO_ITEMS | Covered. |
| X:judging:consistency | judging | consistency | status coherence proof | 0 | NO_ITEMS | Covered. |
| X:reviewing:necessity | reviewing | necessity | audit evidence proof | 0 | NO_ITEMS | Covered. |
| X:reviewing:sufficiency | reviewing | sufficiency | audit assurance proof | 1 | HAS_ITEMS | Fixture/content review check needed. |
| X:reviewing:completeness | reviewing | completeness | audit coverage proof | 0 | NO_ITEMS | Covered. |
| X:reviewing:consistency | reviewing | consistency | audit coherence proof | 0 | NO_ITEMS | Covered. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| X-001 | X:judging:necessity | VerificationGap | Procedure | Procedure | Verify solve-blocking and rule-check-blocking warnings stay distinct. | SPEC section 7 and AB-00-06 preserve different warning classes with different review meanings. | `docs/SPEC.md`; `docs/_Decomposition/SOFTWARE_DECOMP.md`; `Procedure.md` | `docs/SPEC.md#7`; `AB-00-06`; `Procedure.md#Verification` |  | PROPOSAL: future tests assert warning class labels directly. | TBD |
| X-002 | X:reviewing:sufficiency | VerificationGap | Procedure | Procedure | Verify public fixtures/screenshots/reports contain no protected or private data. | Accessibility evidence often uses screenshots and reports, which can leak protected or private data if not checked. | `docs/IP_AND_DATA_BOUNDARY.md`; `Procedure.md` | `docs/IP_AND_DATA_BOUNDARY.md#3`; `Procedure.md#Verification` |  | PROPOSAL: require protected-content/private-data review for fixtures. | TBD |

## Matrix E - Evaluation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| E:guiding:data | guiding | data | guided fact review | 0 | NO_ITEMS | Covered. |
| E:guiding:information | guiding | information | guided signal review | 0 | NO_ITEMS | Covered. |
| E:guiding:knowledge | guiding | knowledge | guided expertise review | 0 | NO_ITEMS | Covered. |
| E:guiding:wisdom | guiding | wisdom | guided judgment review | 0 | NO_ITEMS | Covered. |
| E:applying:data | applying | data | applied fact review | 0 | NO_ITEMS | Covered. |
| E:applying:information | applying | information | applied context review | 0 | NO_ITEMS | Covered. |
| E:applying:knowledge | applying | knowledge | applied expertise review | 0 | NO_ITEMS | Covered. |
| E:applying:wisdom | applying | wisdom | applied judgment review | 0 | NO_ITEMS | Covered. |
| E:judging:data | judging | data | readiness fact finding | 0 | NO_ITEMS | Covered. |
| E:judging:information | judging | information | readiness context finding | 0 | NO_ITEMS | Covered. |
| E:judging:knowledge | judging | knowledge | readiness expertise finding | 0 | NO_ITEMS | Covered. |
| E:judging:wisdom | judging | wisdom | readiness judgment finding | 0 | NO_ITEMS | Covered. |
| E:reviewing:data | reviewing | data | audit fact record | 0 | NO_ITEMS | Covered. |
| E:reviewing:information | reviewing | information | audit context record | 0 | NO_ITEMS | Covered. |
| E:reviewing:knowledge | reviewing | knowledge | audit expertise record | 0 | NO_ITEMS | Covered. |
| E:reviewing:wisdom | reviewing | wisdom | audit judgment record | 1 | HAS_ITEMS | Professional boundary note should remain explicit. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| E-001 | E:reviewing:wisdom | RationaleGap | Guidance | Guidance | Keep accessibility/usability framed as decision support, not professional acceptance. | Clearer rationale helps prevent a good review workflow from being mistaken for professional approval. | `docs/DIRECTIVE.md`; `docs/TYPES.md`; `Guidance.md` | `docs/DIRECTIVE.md#2.2`; `docs/TYPES.md#4`; `Guidance.md#Principles` |  | PROPOSAL: retain professional-boundary sentence in future UI/report wording. | TBD |
