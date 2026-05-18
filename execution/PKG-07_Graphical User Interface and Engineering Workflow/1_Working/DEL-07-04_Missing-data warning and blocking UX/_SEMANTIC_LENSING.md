# Semantic Lensing Register: DEL-07-04 Missing-data warning and blocking UX

**Generated:** 2026-04-30  
**Deliverable Folder:** `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-04_Missing-data warning and blocking UX`  
**Warnings:** none

**Inputs Read:**
- `_CONTEXT.md` - deliverable identity and architecture-basis injection
- `_STATUS.md` - lifecycle state
- `_SEMANTIC.md` - matrices A, B, C, F, D, X, E
- `Datasheet.md` - production document
- `Specification.md` - production document
- `Guidance.md` - production document
- `Procedure.md` - production document
- `_REFERENCES.md` - governing source pointers

**Purpose:** Apply semantic-matrix cells as lenses over the production documents, capturing warranted enrichment inputs for later document or implementation work. This register is not engineering authority.

## Summary

- Total warranted items: 7
- By document:
  - Datasheet: 1
  - Specification: 3
  - Guidance: 2
  - Procedure: 1
- By matrix:
  - A: 1  B: 1  C: 1  F: 1  D: 1  X: 1  E: 1
- By type:
  - Conflict: 0
  - VerificationGap: 3
  - MissingSlot: 1
  - WeakStatement: 0
  - RationaleGap: 1
  - Normalization: 0
  - TBD_Question: 2
  - MatrixError: 0
- Notable conflicts: 0
- Matrix parse errors: 0

## Matrix A - Orientation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| A:normative:guiding | normative | guiding | prescriptive direction | 1 | HAS_ITEMS | Professional-boundary rationale should stay visible. |
| A:normative:applying | normative | applying | mandatory practice | 0 | NO_ITEMS | Covered by requirements. |
| A:normative:judging | normative | judging | compliance determination | 0 | NO_ITEMS | Covered by no-compliance wording. |
| A:normative:reviewing | normative | reviewing | regulatory audit | 0 | NO_ITEMS | Covered by review checks. |
| A:operative:guiding | operative | guiding | procedural direction | 0 | NO_ITEMS | Covered by Procedure. |
| A:operative:applying | operative | applying | practical execution | 0 | NO_ITEMS | Implementation deferred. |
| A:operative:judging | operative | judging | performance assessment | 0 | NO_ITEMS | Covered by verification. |
| A:operative:reviewing | operative | reviewing | process audit | 0 | NO_ITEMS | Covered by records. |
| A:evaluative:guiding | evaluative | guiding | value orientation | 0 | NO_ITEMS | Covered by Guidance principles. |
| A:evaluative:applying | evaluative | applying | merit application | 0 | NO_ITEMS | No extra item. |
| A:evaluative:judging | evaluative | judging | worth determination | 0 | NO_ITEMS | No extra item. |
| A:evaluative:reviewing | evaluative | reviewing | quality appraisal | 0 | NO_ITEMS | No extra item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| A-001 | A:normative:guiding | RationaleGap | Guidance | Guidance | Preserve why warning UX cannot imply professional approval. | The documents state the boundary, but future UX work benefits from keeping the rationale attached to warning text and status display. | `docs/DIRECTIVE.md`; `docs/TYPES.md`; `Guidance.md` | `docs/DIRECTIVE.md#2.2`; `docs/TYPES.md#4`; `Guidance.md#Purpose` |  | PROPOSAL: use `HUMAN_REVIEW_REQUIRED` as the reportable reminder, not compliance language. | TBD |

## Matrix B - Conceptualization

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| B:data:necessity | data | necessity | essential fact | 0 | NO_ITEMS | Covered by warning classes. |
| B:data:sufficiency | data | sufficiency | adequate evidence | 0 | NO_ITEMS | Covered by provenance. |
| B:data:completeness | data | completeness | comprehensive record | 0 | NO_ITEMS | Covered by records. |
| B:data:consistency | data | consistency | reliable measurement | 0 | NO_ITEMS | No numeric values included. |
| B:information:necessity | information | necessity | essential signal | 0 | NO_ITEMS | Covered by diagnostics. |
| B:information:sufficiency | information | sufficiency | adequate context | 1 | HAS_ITEMS | Implementation-library choice remains TBD. |
| B:information:completeness | information | completeness | comprehensive account | 0 | NO_ITEMS | Covered by four docs. |
| B:information:consistency | information | consistency | coherent message | 0 | NO_ITEMS | No issue. |
| B:knowledge:necessity | knowledge | necessity | fundamental understanding | 0 | NO_ITEMS | No issue. |
| B:knowledge:sufficiency | knowledge | sufficiency | competent expertise | 0 | NO_ITEMS | No issue. |
| B:knowledge:completeness | knowledge | completeness | thorough mastery | 0 | NO_ITEMS | No issue. |
| B:knowledge:consistency | knowledge | consistency | coherent understanding | 0 | NO_ITEMS | No issue. |
| B:wisdom:necessity | wisdom | necessity | essential discernment | 0 | NO_ITEMS | No issue. |
| B:wisdom:sufficiency | wisdom | sufficiency | adequate judgment | 0 | NO_ITEMS | Human authority covered. |
| B:wisdom:completeness | wisdom | completeness | holistic insight | 0 | NO_ITEMS | No issue. |
| B:wisdom:consistency | wisdom | consistency | principled reasoning | 0 | NO_ITEMS | No issue. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| B-001 | B:information:sufficiency | TBD_Question | Specification | Specification | TBD: exact GUI component/state library remains implementation-level. | The setup docs define behavior, not concrete components or state-library details; future implementation must avoid treating the setup artifact as a source-code design. | `_CONTEXT.md`; `Specification.md` | `_CONTEXT.md#Architecture Basis Injection`; `Specification.md#Scope` |  | PROPOSAL: leave implementation library choices outside DEL-07-04 setup. | TBD |

## Matrix C - Formulation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| C:normative:necessity | normative | necessity | binding warning trigger | 0 | NO_ITEMS | Covered by Requirements. |
| C:normative:sufficiency | normative | sufficiency | evidenced warning basis | 0 | NO_ITEMS | Covered by sources. |
| C:normative:completeness | normative | completeness | full diagnostic record | 0 | NO_ITEMS | Covered by envelope fields. |
| C:normative:consistency | normative | consistency | traceable status alignment | 0 | NO_ITEMS | Covered by status mapping. |
| C:operative:necessity | operative | necessity | required UX input | 0 | NO_ITEMS | Covered by prerequisites. |
| C:operative:sufficiency | operative | sufficiency | usable warning context | 1 | HAS_ITEMS | Accessibility/display access should be promoted from verification note to requirement. |
| C:operative:completeness | operative | completeness | full workflow record | 0 | NO_ITEMS | Covered by Procedure. |
| C:operative:consistency | operative | consistency | stable interaction signal | 0 | NO_ITEMS | No issue. |
| C:evaluative:necessity | evaluative | necessity | review trigger basis | 0 | NO_ITEMS | Covered by review boundary. |
| C:evaluative:sufficiency | evaluative | sufficiency | judgment evidence basis | 0 | NO_ITEMS | No issue. |
| C:evaluative:completeness | evaluative | completeness | full assessment record | 0 | NO_ITEMS | No issue. |
| C:evaluative:consistency | evaluative | consistency | coherent review finding | 0 | NO_ITEMS | No issue. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| C-001 | C:operative:sufficiency | MissingSlot | Datasheet | Datasheet | Add condition that warning class, severity, affected object, and remediation cannot be color-only. | The verification section mentions accessibility-oriented checks, but the descriptive datasheet should record the warning-display condition. | `Specification.md`; `Datasheet.md`; `docs/SPEC.md` | `Specification.md#Verification`; `Datasheet.md#Conditions`; `docs/SPEC.md#7` |  | PROPOSAL: keep detailed accessibility gates for DEL-07-06 while recording this condition here. | TBD |

## Matrix F - Requirements

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| F:normative:necessity | normative | necessity | mandatory warning gate | 1 | HAS_ITEMS | Future warning-class tests need explicit gate. |
| F:normative:sufficiency | normative | sufficiency | verified basis gate | 0 | NO_ITEMS | Covered by verification. |
| F:normative:completeness | normative | completeness | full diagnostic gate | 0 | NO_ITEMS | Covered by R-004. |
| F:normative:consistency | normative | consistency | traceable status gate | 0 | NO_ITEMS | Covered by R-005. |
| F:operative:necessity | operative | necessity | execution readiness gate | 0 | NO_ITEMS | Covered. |
| F:operative:sufficiency | operative | sufficiency | usable context gate | 0 | NO_ITEMS | Covered. |
| F:operative:completeness | operative | completeness | workflow coverage gate | 0 | NO_ITEMS | Covered. |
| F:operative:consistency | operative | consistency | stable process gate | 0 | NO_ITEMS | Covered. |
| F:evaluative:necessity | evaluative | necessity | review readiness gate | 0 | NO_ITEMS | Covered. |
| F:evaluative:sufficiency | evaluative | sufficiency | judgment basis gate | 0 | NO_ITEMS | Covered. |
| F:evaluative:completeness | evaluative | completeness | assessment coverage gate | 0 | NO_ITEMS | Covered. |
| F:evaluative:consistency | evaluative | consistency | finding trace gate | 0 | NO_ITEMS | Covered. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| F-001 | F:normative:necessity | VerificationGap | Specification | Specification | Make future tests assert every warning class is preserved. | The requirement names all classes; acceptance needs explicit per-class test coverage to avoid silent collapse into generic alerts. | `Specification.md`; `docs/SPEC.md`; `docs/_Decomposition/SOFTWARE_DECOMP.md` | `Specification.md#Requirements`; `docs/SPEC.md#7`; `SOFTWARE_DECOMP.md#AB-00-06` |  | PROPOSAL: require representative diagnostics for all six classes. | TBD |

## Matrix D - Objectives

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| D:normative:guiding | normative | guiding | source-bound warning direction | 0 | NO_ITEMS | Covered. |
| D:normative:applying | normative | applying | required blocking practice | 0 | NO_ITEMS | Covered. |
| D:normative:judging | normative | judging | status decision boundary | 0 | NO_ITEMS | Covered. |
| D:normative:reviewing | normative | reviewing | audit trail boundary | 0 | NO_ITEMS | Covered. |
| D:operative:guiding | operative | guiding | workflow direction | 0 | NO_ITEMS | Covered. |
| D:operative:applying | operative | applying | executable warning path | 1 | HAS_ITEMS | Surface-by-surface behavior remains implementation detail. |
| D:operative:judging | operative | judging | readiness assessment path | 0 | NO_ITEMS | Covered. |
| D:operative:reviewing | operative | reviewing | process evidence audit | 0 | NO_ITEMS | Covered. |
| D:evaluative:guiding | evaluative | guiding | review framing basis | 0 | NO_ITEMS | Covered. |
| D:evaluative:applying | evaluative | applying | judgment action path | 0 | NO_ITEMS | Covered. |
| D:evaluative:judging | evaluative | judging | finding decision boundary | 0 | NO_ITEMS | Covered. |
| D:evaluative:reviewing | evaluative | reviewing | quality review basis | 0 | NO_ITEMS | Covered. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| D-001 | D:operative:applying | TBD_Question | Guidance | Guidance | TBD: exact editor/solve/results/report warning placements. | The setup docs state warning behavior but intentionally do not lay out concrete component placement or interaction copy. | `Guidance.md`; `Procedure.md`; `_CONTEXT.md` | `Guidance.md#Trade-offs`; `Procedure.md#Steps`; `_CONTEXT.md#Architecture Basis Injection` |  | PROPOSAL: defer surface layout and copy to future GUI implementation under DEL-07-04. | TBD |

## Matrix X - Verification

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| X:guiding:necessity | guiding | necessity | governing warning proof | 0 | NO_ITEMS | Covered. |
| X:guiding:sufficiency | guiding | sufficiency | direction evidence proof | 0 | NO_ITEMS | Covered. |
| X:guiding:completeness | guiding | completeness | full guidance trace | 0 | NO_ITEMS | Covered. |
| X:guiding:consistency | guiding | consistency | stable guidance record | 0 | NO_ITEMS | Covered. |
| X:applying:necessity | applying | necessity | blocking input proof | 0 | NO_ITEMS | Covered. |
| X:applying:sufficiency | applying | sufficiency | execution evidence proof | 0 | NO_ITEMS | Covered. |
| X:applying:completeness | applying | completeness | full action trace | 0 | NO_ITEMS | Covered. |
| X:applying:consistency | applying | consistency | stable action record | 0 | NO_ITEMS | Covered. |
| X:judging:necessity | judging | necessity | decision input proof | 1 | HAS_ITEMS | Solve/rule separation needs test proof. |
| X:judging:sufficiency | judging | sufficiency | assessment evidence proof | 0 | NO_ITEMS | Covered. |
| X:judging:completeness | judging | completeness | full decision trace | 0 | NO_ITEMS | Covered. |
| X:judging:consistency | judging | consistency | stable decision record | 0 | NO_ITEMS | Covered. |
| X:reviewing:necessity | reviewing | necessity | audit input proof | 0 | NO_ITEMS | Covered. |
| X:reviewing:sufficiency | reviewing | sufficiency | inspection evidence proof | 0 | NO_ITEMS | Covered. |
| X:reviewing:completeness | reviewing | completeness | full audit trace | 0 | NO_ITEMS | Covered. |
| X:reviewing:consistency | reviewing | consistency | stable inspection record | 0 | NO_ITEMS | Covered. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| X-001 | X:judging:necessity | VerificationGap | Procedure | Procedure | Verify `SOLVE_BLOCKING` remains distinct from `RULE_CHECK_BLOCKING`. | SPEC and TYPES separate missing physical input from missing rule-pack input; the procedure should preserve that test gate. | `docs/SPEC.md`; `docs/TYPES.md`; `Procedure.md` | `docs/SPEC.md#7`; `docs/TYPES.md#4`; `Procedure.md#Verification` |  | PROPOSAL: future tests assert mechanics and rule-check statuses separately. | TBD |

## Matrix E - Evaluation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| E:guiding:data | guiding | data | governed warning notice | 0 | NO_ITEMS | Covered. |
| E:guiding:information | guiding | information | guided context notice | 0 | NO_ITEMS | Covered. |
| E:guiding:knowledge | guiding | knowledge | traced understanding notice | 0 | NO_ITEMS | Covered. |
| E:guiding:wisdom | guiding | wisdom | reliance boundary notice | 0 | NO_ITEMS | Covered. |
| E:applying:data | applying | data | blocking input notice | 0 | NO_ITEMS | Covered. |
| E:applying:information | applying | information | action context notice | 0 | NO_ITEMS | Covered. |
| E:applying:knowledge | applying | knowledge | implementation understanding notice | 0 | NO_ITEMS | Covered. |
| E:applying:wisdom | applying | wisdom | use boundary notice | 0 | NO_ITEMS | Covered. |
| E:judging:data | judging | data | decision input finding | 0 | NO_ITEMS | Covered. |
| E:judging:information | judging | information | assessment context finding | 0 | NO_ITEMS | Covered. |
| E:judging:knowledge | judging | knowledge | evaluation understanding finding | 0 | NO_ITEMS | Covered. |
| E:judging:wisdom | judging | wisdom | professional boundary finding | 0 | NO_ITEMS | Covered. |
| E:reviewing:data | reviewing | data | audit input record | 0 | NO_ITEMS | Covered. |
| E:reviewing:information | reviewing | information | audit context record | 0 | NO_ITEMS | Covered. |
| E:reviewing:knowledge | reviewing | knowledge | quality understanding record | 0 | NO_ITEMS | Covered. |
| E:reviewing:wisdom | reviewing | wisdom | acceptance boundary record | 1 | HAS_ITEMS | IP/export/report warning needs verification. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| E-001 | E:reviewing:wisdom | VerificationGap | Specification | Specification | Add explicit future test for `IP_BOUNDARY_WARNING` in public export/report paths. | The setup requires IP-boundary warning behavior, but future tests should prove public-facing paths cannot leak protected/private data. | `Specification.md`; `docs/IP_AND_DATA_BOUNDARY.md`; `docs/SPEC.md` | `Specification.md#Requirements`; `docs/IP_AND_DATA_BOUNDARY.md#5`; `docs/SPEC.md#8` |  | PROPOSAL: use invented fixtures and protected-content linting only. | TBD |
