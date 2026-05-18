# Semantic Lensing Register: DEL-06-03 Required-input completeness checker

**Generated:** 2026-04-30  
**Deliverable Folder:** `execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/DEL-06-03_Required-input completeness checker`  
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

- Total warranted items: 8
- By document:
  - Datasheet: 1
  - Specification: 4
  - Guidance: 2
  - Procedure: 1
- By matrix:
  - A: 1  B: 1  C: 1  F: 2  D: 1  X: 1  E: 1
- By type:
  - Conflict: 1
  - VerificationGap: 3
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
| A:normative:guiding | normative | guiding | prescriptive direction | 1 | HAS_ITEMS | Boundary wording needed. |
| A:normative:applying | normative | applying | mandatory practice | 0 | NO_ITEMS | Covered by requirements. |
| A:normative:judging | normative | judging | compliance determination | 1 | HAS_ITEMS | Compliance-claim guard needed. |
| A:normative:reviewing | normative | reviewing | regulatory audit | 0 | NO_ITEMS | Covered by review boundary. |
| A:operative:guiding | operative | guiding | procedural direction | 0 | NO_ITEMS | Covered by procedure. |
| A:operative:applying | operative | applying | practical execution | 0 | NO_ITEMS | Implementation deferred. |
| A:operative:judging | operative | judging | performance assessment | 0 | NO_ITEMS | Covered by verification. |
| A:operative:reviewing | operative | reviewing | process audit | 0 | NO_ITEMS | Covered by setup verification. |
| A:evaluative:guiding | evaluative | guiding | value orientation | 0 | NO_ITEMS | Covered by guidance principles. |
| A:evaluative:applying | evaluative | applying | merit application | 0 | NO_ITEMS | No extra item. |
| A:evaluative:judging | evaluative | judging | worth determination | 0 | NO_ITEMS | No extra item. |
| A:evaluative:reviewing | evaluative | reviewing | quality appraisal | 0 | NO_ITEMS | No extra item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| A-001 | A:normative:guiding | RationaleGap | Guidance | Guidance | Keep the rule-check gate separate from professional approval. | The deliverable can block rule-check status but must not imply code compliance or professional acceptance. | `docs/DIRECTIVE.md`; `docs/TYPES.md`; `Guidance.md` | `docs/DIRECTIVE.md#2.2`; `docs/TYPES.md#4`; `Guidance.md#Principles` |  | PROPOSAL: use TYPES status vocabulary as the source of terms. | TBD |
| A-002 | A:normative:judging | VerificationGap | Specification | Specification | Add verification that automatic `CODE_COMPLIANT` status is excluded. | The status vocabulary explicitly forbids automatic compliance status. | `docs/TYPES.md`; `Specification.md` | `docs/TYPES.md#4`; `Specification.md#Requirements` |  | PROPOSAL: test status names directly in future implementation. | TBD |

## Matrix B - Conceptualization

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| B:data:necessity | data | necessity | essential fact | 1 | HAS_ITEMS | Required-input declaration source needed. |
| B:data:sufficiency | data | sufficiency | adequate evidence | 0 | NO_ITEMS | Covered by provenance requirement. |
| B:data:completeness | data | completeness | comprehensive record | 0 | NO_ITEMS | Covered by dependency artifacts. |
| B:data:consistency | data | consistency | reliable measurement | 0 | NO_ITEMS | No numeric values included. |
| B:information:necessity | information | necessity | essential signal | 0 | NO_ITEMS | Covered by warnings. |
| B:information:sufficiency | information | sufficiency | adequate context | 0 | NO_ITEMS | Covered by references. |
| B:information:completeness | information | completeness | comprehensive account | 0 | NO_ITEMS | Covered by four docs. |
| B:information:consistency | information | consistency | coherent message | 0 | NO_ITEMS | No issue. |
| B:knowledge:necessity | knowledge | necessity | fundamental understanding | 0 | NO_ITEMS | No issue. |
| B:knowledge:sufficiency | knowledge | sufficiency | competent expertise | 0 | NO_ITEMS | Professional boundary covered. |
| B:knowledge:completeness | knowledge | completeness | thorough mastery | 0 | NO_ITEMS | No issue. |
| B:knowledge:consistency | knowledge | consistency | coherent understanding | 0 | NO_ITEMS | No issue. |
| B:wisdom:necessity | wisdom | necessity | essential discernment | 0 | NO_ITEMS | No issue. |
| B:wisdom:sufficiency | wisdom | sufficiency | adequate judgment | 0 | NO_ITEMS | Human authority covered. |
| B:wisdom:completeness | wisdom | completeness | holistic insight | 0 | NO_ITEMS | No issue. |
| B:wisdom:consistency | wisdom | consistency | principled reasoning | 0 | NO_ITEMS | No issue. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| B-001 | B:data:necessity | TBD_Question | Specification | Specification | TBD: exact required-input declaration fields from DEL-06-01. | The checker cannot be implemented until the rule-pack schema names the machine-checkable required-input contract. | `docs/SPEC.md`; `Specification.md` | `docs/SPEC.md#6`; `Specification.md#External Inputs` |  | PROPOSAL: keep as external dependency until DEL-06-01 is ready. | TBD |

## Matrix C - Formulation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| C:normative:necessity | normative | necessity | binding input threshold | 0 | NO_ITEMS | Covered by R-001. |
| C:normative:sufficiency | normative | sufficiency | evidenced rule basis | 1 | HAS_ITEMS | Provenance warning distinction needed. |
| C:normative:completeness | normative | completeness | full governance record | 0 | NO_ITEMS | Covered by records. |
| C:normative:consistency | normative | consistency | traceable rule alignment | 0 | NO_ITEMS | Covered by dependencies. |
| C:operative:necessity | operative | necessity | required execution input | 0 | NO_ITEMS | Covered by prerequisites. |
| C:operative:sufficiency | operative | sufficiency | usable evidence basis | 0 | NO_ITEMS | Covered by verification. |
| C:operative:completeness | operative | completeness | full workflow record | 0 | NO_ITEMS | Covered by procedure. |
| C:operative:consistency | operative | consistency | stable implementation signal | 0 | NO_ITEMS | Implementation deferred. |
| C:evaluative:necessity | evaluative | necessity | review trigger basis | 0 | NO_ITEMS | Covered by human review notes. |
| C:evaluative:sufficiency | evaluative | sufficiency | judgment evidence basis | 0 | NO_ITEMS | No issue. |
| C:evaluative:completeness | evaluative | completeness | full assessment record | 0 | NO_ITEMS | No issue. |
| C:evaluative:consistency | evaluative | consistency | coherent review finding | 0 | NO_ITEMS | No issue. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| C-001 | C:normative:sufficiency | MissingSlot | Datasheet | Datasheet | Record condition for value-present/provenance-weak. | Completeness is not only presence; provenance may affect whether a rule-pack input can be relied upon. | `docs/CONTRACT.md`; `Datasheet.md` | `OPS-K-DATA-3`; `Datasheet.md#Conditions` |  | PROPOSAL: classify as provenance warning, not missing value. | TBD |

## Matrix F - Requirements

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| F:normative:necessity | normative | necessity | mandatory input gate | 1 | HAS_ITEMS | Future test needed. |
| F:normative:sufficiency | normative | sufficiency | verified basis gate | 1 | HAS_ITEMS | Future test needed. |
| F:normative:completeness | normative | completeness | full record gate | 0 | NO_ITEMS | Covered by artifacts. |
| F:normative:consistency | normative | consistency | traceable rule gate | 0 | NO_ITEMS | Covered by dependencies. |
| F:operative:necessity | operative | necessity | execution readiness gate | 0 | NO_ITEMS | Covered. |
| F:operative:sufficiency | operative | sufficiency | usable context gate | 0 | NO_ITEMS | Covered. |
| F:operative:completeness | operative | completeness | workflow coverage gate | 0 | NO_ITEMS | Covered. |
| F:operative:consistency | operative | consistency | stable process gate | 0 | NO_ITEMS | Covered. |
| F:evaluative:necessity | evaluative | necessity | review readiness gate | 0 | NO_ITEMS | Covered. |
| F:evaluative:sufficiency | evaluative | sufficiency | judgment basis gate | 0 | NO_ITEMS | Covered. |
| F:evaluative:completeness | evaluative | completeness | assessment coverage gate | 0 | NO_ITEMS | Covered. |
| F:evaluative:consistency | evaluative | consistency | finding trace gate | 0 | NO_ITEMS | Covered. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| F-001 | F:normative:necessity | VerificationGap | Specification | Specification | Add future test for missing declared required input. | Validation strategy names rule-pack missing-input tests as a release gate expectation. | `docs/VALIDATION_STRATEGY.md`; `Specification.md` | `docs/VALIDATION_STRATEGY.md#4`; `Specification.md#Verification` |  | PROPOSAL: use invented/non-code fixtures only. | TBD |
| F-002 | F:normative:sufficiency | VerificationGap | Specification | Specification | Add future test for unit/provenance mismatch classification. | Completeness should distinguish absent value, unit mismatch, and weak provenance. | `docs/CONTRACT.md`; `docs/SPEC.md`; `Specification.md` | `OPS-K-UNIT-1`; `docs/SPEC.md#7`; `Specification.md#Verification` |  | PROPOSAL: split by diagnostic class. | TBD |

## Matrix D - Objectives

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| D:normative:guiding | normative | guiding | source-bound direction | 0 | NO_ITEMS | Covered. |
| D:normative:applying | normative | applying | required gate practice | 0 | NO_ITEMS | Covered. |
| D:normative:judging | normative | judging | status decision boundary | 0 | NO_ITEMS | Covered. |
| D:normative:reviewing | normative | reviewing | audit trail boundary | 0 | NO_ITEMS | Covered. |
| D:operative:guiding | operative | guiding | workflow direction | 0 | NO_ITEMS | Covered. |
| D:operative:applying | operative | applying | executable gating path | 0 | NO_ITEMS | Covered. |
| D:operative:judging | operative | judging | readiness assessment path | 0 | NO_ITEMS | Covered. |
| D:operative:reviewing | operative | reviewing | process evidence audit | 0 | NO_ITEMS | Covered. |
| D:evaluative:guiding | evaluative | guiding | review framing basis | 1 | HAS_ITEMS | Conflict retained. |
| D:evaluative:applying | evaluative | applying | judgment application path | 0 | NO_ITEMS | Covered. |
| D:evaluative:judging | evaluative | judging | finding decision boundary | 0 | NO_ITEMS | Covered. |
| D:evaluative:reviewing | evaluative | reviewing | quality review basis | 0 | NO_ITEMS | Covered. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| D-001 | D:evaluative:guiding | Conflict | Guidance | Guidance | Keep expression grammar/library unresolved. | The architecture/decomposition notes leave the exact rule expression grammar/library as TBD, so setup docs cannot prescribe it. | `docs/_Decomposition/SOFTWARE_DECOMP.md`; `Guidance.md` | `OI-006`; `Guidance.md#Conflict Table` | `docs/_Decomposition/SOFTWARE_DECOMP.md#OI-006`; `Guidance.md#Conflict Table` | PROPOSAL: defer to future human architecture decision. | TBD |

## Matrix X - Verification

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| X:guiding:necessity | guiding | necessity | governing input proof | 0 | NO_ITEMS | Covered. |
| X:guiding:sufficiency | guiding | sufficiency | direction evidence proof | 0 | NO_ITEMS | Covered. |
| X:guiding:completeness | guiding | completeness | full guidance trace | 0 | NO_ITEMS | Covered. |
| X:guiding:consistency | guiding | consistency | stable guidance record | 0 | NO_ITEMS | Covered. |
| X:applying:necessity | applying | necessity | implementation input proof | 0 | NO_ITEMS | Covered. |
| X:applying:sufficiency | applying | sufficiency | execution evidence proof | 0 | NO_ITEMS | Covered. |
| X:applying:completeness | applying | completeness | full application trace | 0 | NO_ITEMS | Covered. |
| X:applying:consistency | applying | consistency | stable application record | 0 | NO_ITEMS | Covered. |
| X:judging:necessity | judging | necessity | decision input proof | 1 | HAS_ITEMS | Blocking classification needed. |
| X:judging:sufficiency | judging | sufficiency | assessment evidence proof | 0 | NO_ITEMS | Covered. |
| X:judging:completeness | judging | completeness | full decision trace | 0 | NO_ITEMS | Covered. |
| X:judging:consistency | judging | consistency | stable decision record | 0 | NO_ITEMS | Covered. |
| X:reviewing:necessity | reviewing | necessity | audit input proof | 0 | NO_ITEMS | Covered. |
| X:reviewing:sufficiency | reviewing | sufficiency | inspection evidence proof | 0 | NO_ITEMS | Covered. |
| X:reviewing:completeness | reviewing | completeness | full audit trace | 0 | NO_ITEMS | Covered. |
| X:reviewing:consistency | reviewing | consistency | stable inspection record | 0 | NO_ITEMS | Covered. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| X-001 | X:judging:necessity | VerificationGap | Procedure | Procedure | Verify `RULE_CHECK_BLOCKING` remains distinct from `SOLVE_BLOCKING`. | SPEC separates missing physical data from missing rule-pack data. | `docs/SPEC.md`; `Procedure.md` | `docs/SPEC.md#7`; `Procedure.md#Verification` |  | PROPOSAL: future tests assert warning class. | TBD |

## Matrix E - Evaluation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| E:guiding:data | guiding | data | governed input notice | 0 | NO_ITEMS | Covered. |
| E:guiding:information | guiding | information | guided context notice | 0 | NO_ITEMS | Covered. |
| E:guiding:knowledge | guiding | knowledge | traced understanding notice | 0 | NO_ITEMS | Covered. |
| E:guiding:wisdom | guiding | wisdom | reliance boundary notice | 1 | HAS_ITEMS | Public example caution. |
| E:applying:data | applying | data | execution input notice | 0 | NO_ITEMS | Covered. |
| E:applying:information | applying | information | application context notice | 0 | NO_ITEMS | Covered. |
| E:applying:knowledge | applying | knowledge | implementation understanding notice | 0 | NO_ITEMS | Covered. |
| E:applying:wisdom | applying | wisdom | use boundary notice | 0 | NO_ITEMS | Covered. |
| E:judging:data | judging | data | decision input finding | 0 | NO_ITEMS | Covered. |
| E:judging:information | judging | information | assessment context finding | 0 | NO_ITEMS | Covered. |
| E:judging:knowledge | judging | knowledge | evaluation understanding finding | 0 | NO_ITEMS | Covered. |
| E:judging:wisdom | judging | wisdom | professional boundary finding | 0 | NO_ITEMS | Covered. |
| E:reviewing:data | reviewing | data | audit input record | 0 | NO_ITEMS | Covered. |
| E:reviewing:information | reviewing | information | audit context record | 0 | NO_ITEMS | Covered. |
| E:reviewing:knowledge | reviewing | knowledge | quality understanding record | 0 | NO_ITEMS | Covered. |
| E:reviewing:wisdom | reviewing | wisdom | acceptance boundary record | 0 | NO_ITEMS | Covered. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| E-001 | E:guiding:wisdom | TBD_Question | Guidance | Guidance | TBD: future public examples must remain invented/non-code. | DEL-06-03 itself should not create examples, but future tests need fixtures that preserve the IP boundary. | `docs/VALIDATION_STRATEGY.md`; `Guidance.md` | `docs/VALIDATION_STRATEGY.md#5`; `Guidance.md#Examples` |  | PROPOSAL: route examples to DEL-06-05 or validation work. | TBD |
