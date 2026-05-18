# Semantic Lensing Register: DEL-11-04 Invented educational example models

**Generated:** 2026-04-30  
**Deliverable Folder:** `execution/PKG-11_Documentation, Examples, and Education/1_Working/DEL-11-04_Invented educational example models`  
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

**Purpose:** Apply `semantic-matrix-build` matrix cells as lenses over the production documents, capturing warranted enrichment inputs for a later enrichment pass. This register is not engineering authority.

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
  - RationaleGap: 2
  - Normalization: 0
  - TBD_Question: 1
  - MatrixError: 0
- Notable conflicts: 1
- Matrix parse errors: 0

## Matrix A - Orientation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| A:normative:guiding | normative | guiding | prescriptive direction | 1 | HAS_ITEMS | Non-reliance direction should be explicit. |
| A:normative:applying | normative | applying | mandatory practice | 0 | NO_ITEMS | Covered by requirements. |
| A:normative:judging | normative | judging | compliance determination | 0 | NO_ITEMS | Compliance claim excluded. |
| A:normative:reviewing | normative | reviewing | regulatory audit | 0 | NO_ITEMS | Covered by review boundary. |
| A:operative:guiding | operative | guiding | procedural direction | 0 | NO_ITEMS | Covered by procedure. |
| A:operative:applying | operative | applying | practical execution | 0 | NO_ITEMS | External files deferred. |
| A:operative:judging | operative | judging | performance assessment | 0 | NO_ITEMS | Covered by verification. |
| A:operative:reviewing | operative | reviewing | process audit | 0 | NO_ITEMS | Covered by setup checks. |
| A:evaluative:guiding | evaluative | guiding | value orientation | 0 | NO_ITEMS | Covered by principles. |
| A:evaluative:applying | evaluative | applying | merit application | 0 | NO_ITEMS | No extra item. |
| A:evaluative:judging | evaluative | judging | worth determination | 0 | NO_ITEMS | No extra item. |
| A:evaluative:reviewing | evaluative | reviewing | quality appraisal | 0 | NO_ITEMS | No extra item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| A-001 | A:normative:guiding | RationaleGap | Guidance | Guidance | Add or preserve a non-reliance notice for all future examples. | Invented examples can be mistaken for engineering templates unless the professional boundary is explicit. | `docs/DIRECTIVE.md`; `docs/TYPES.md`; `Guidance.md` | `docs/DIRECTIVE.md#3`; `docs/TYPES.md#4`; `Guidance.md#Non-Reliance Notice` |  | PROPOSAL: use the professional-boundary vocabulary from TYPES. | TBD |

## Matrix B - Conceptualization

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| B:data:necessity | data | necessity | essential fact | 1 | HAS_ITEMS | Future schema source needed. |
| B:data:sufficiency | data | sufficiency | adequate evidence | 0 | NO_ITEMS | Covered by provenance. |
| B:data:completeness | data | completeness | comprehensive record | 0 | NO_ITEMS | Covered by required artifacts. |
| B:data:consistency | data | consistency | reliable measurement | 0 | NO_ITEMS | No numeric example values included. |
| B:information:necessity | information | necessity | essential signal | 0 | NO_ITEMS | Covered by notices. |
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
| B-001 | B:data:necessity | TBD_Question | Specification | Specification | TBD: exact future model file schema and persistence format from DEL-02-01/DEL-02-05. | Actual example model files cannot be safely materialized until the canonical schema and persistence contract exist. | `docs/SPEC.md`; `Specification.md` | `docs/SPEC.md#2`; `Specification.md#External Inputs` |  | PROPOSAL: keep external example files deferred. | TBD |

## Matrix C - Formulation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| C:normative:necessity | normative | necessity | public boundary threshold | 0 | NO_ITEMS | Covered. |
| C:normative:sufficiency | normative | sufficiency | provenance evidence basis | 1 | HAS_ITEMS | Provenance label should be explicit. |
| C:normative:completeness | normative | completeness | complete notice record | 0 | NO_ITEMS | Covered. |
| C:normative:consistency | normative | consistency | consistent example framing | 0 | NO_ITEMS | Covered. |
| C:operative:necessity | operative | necessity | required example input | 0 | NO_ITEMS | Covered. |
| C:operative:sufficiency | operative | sufficiency | usable tutorial basis | 0 | NO_ITEMS | Covered. |
| C:operative:completeness | operative | completeness | full fixture record | 0 | NO_ITEMS | Covered. |
| C:operative:consistency | operative | consistency | stable demonstration signal | 0 | NO_ITEMS | Covered. |
| C:evaluative:necessity | evaluative | necessity | review trigger basis | 0 | NO_ITEMS | Covered. |
| C:evaluative:sufficiency | evaluative | sufficiency | audit evidence basis | 0 | NO_ITEMS | Covered. |
| C:evaluative:completeness | evaluative | completeness | assessment coverage record | 0 | NO_ITEMS | Covered. |
| C:evaluative:consistency | evaluative | consistency | coherent testing finding | 0 | NO_ITEMS | Covered. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| C-001 | C:normative:sufficiency | MissingSlot | Datasheet | Datasheet | Record `PUBLIC_DOMAIN_OR_ORIGINAL` or equivalent provenance expectation for future data. | Future invented examples need explicit public-data provenance so invented values are not mistaken for hidden standards data. | `docs/TYPES.md`; `Datasheet.md` | `docs/TYPES.md#7`; `Datasheet.md#Attributes` |  | PROPOSAL: use provenance labels from TYPES. | TBD |

## Matrix F - Requirements

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| F:normative:necessity | normative | necessity | mandatory example gate | 1 | HAS_ITEMS | Protected-content check needed. |
| F:normative:sufficiency | normative | sufficiency | verified source gate | 0 | NO_ITEMS | Covered by review. |
| F:normative:completeness | normative | completeness | full artifact gate | 0 | NO_ITEMS | Covered by records. |
| F:normative:consistency | normative | consistency | traceable demo gate | 0 | NO_ITEMS | Covered by dependencies. |
| F:operative:necessity | operative | necessity | execution readiness gate | 0 | NO_ITEMS | Covered. |
| F:operative:sufficiency | operative | sufficiency | usable teaching gate | 1 | HAS_ITEMS | Fake-rule-pack schema dependency. |
| F:operative:completeness | operative | completeness | workflow coverage gate | 0 | NO_ITEMS | Covered. |
| F:operative:consistency | operative | consistency | stable fixture gate | 0 | NO_ITEMS | Covered. |
| F:evaluative:necessity | evaluative | necessity | review readiness gate | 0 | NO_ITEMS | Covered. |
| F:evaluative:sufficiency | evaluative | sufficiency | judgment evidence gate | 0 | NO_ITEMS | Covered. |
| F:evaluative:completeness | evaluative | completeness | assessment coverage gate | 0 | NO_ITEMS | Covered. |
| F:evaluative:consistency | evaluative | consistency | finding trace gate | 0 | NO_ITEMS | Covered. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| F-001 | F:normative:necessity | VerificationGap | Specification | Specification | Add setup verification that no external example files or tutorials were created. | The sealed brief permits setup/document production only despite register-listed future artifacts. | User brief; `Specification.md` | `Write scope`; `Specification.md#Verification` |  | PROPOSAL: verify file list and scoped git status. | TBD |
| F-002 | F:operative:sufficiency | VerificationGap | Specification | Specification | Add future check that fake-rule-pack demos wait for DEL-06 schema/example boundaries. | Fake-rule-pack demonstrations cannot safely define executable rule-pack artifacts before the schema and invented-rule-pack pattern exist. | `docs/SPEC.md`; `Specification.md` | `docs/SPEC.md#6`; `Specification.md#External Inputs` |  | PROPOSAL: record DEL-06-01 and DEL-06-05 dependencies. | TBD |

## Matrix D - Objectives

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| D:normative:guiding | normative | guiding | source-bound direction | 0 | NO_ITEMS | Covered. |
| D:normative:applying | normative | applying | required example practice | 0 | NO_ITEMS | Covered. |
| D:normative:judging | normative | judging | status boundary decision | 0 | NO_ITEMS | Covered. |
| D:normative:reviewing | normative | reviewing | audit notice boundary | 0 | NO_ITEMS | Covered. |
| D:operative:guiding | operative | guiding | workflow direction | 0 | NO_ITEMS | Covered. |
| D:operative:applying | operative | applying | executable demo path | 0 | NO_ITEMS | Covered. |
| D:operative:judging | operative | judging | readiness assessment path | 0 | NO_ITEMS | Covered. |
| D:operative:reviewing | operative | reviewing | process evidence audit | 0 | NO_ITEMS | Covered. |
| D:evaluative:guiding | evaluative | guiding | review framing basis | 1 | HAS_ITEMS | Scope conflict retained. |
| D:evaluative:applying | evaluative | applying | judgment application path | 0 | NO_ITEMS | Covered. |
| D:evaluative:judging | evaluative | judging | finding boundary decision | 0 | NO_ITEMS | Covered. |
| D:evaluative:reviewing | evaluative | reviewing | quality review basis | 0 | NO_ITEMS | Covered. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| D-001 | D:evaluative:guiding | Conflict | Guidance | Guidance | Keep external `examples/models/invented/*` and tutorial creation deferred. | The register anticipates external artifacts while the sealed brief forbids creating them in this setup session. | `docs/_Registers/Deliverables.csv`; `Guidance.md` | `DEL-11-04 row`; `Guidance.md#Conflict Table` | `docs/_Registers/Deliverables.csv#DEL-11-04`; `User brief#Write scope` | PROPOSAL: setup docs only in this run. | TBD |

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
| X:judging:necessity | judging | necessity | decision input proof | 0 | NO_ITEMS | Covered. |
| X:judging:sufficiency | judging | sufficiency | assessment evidence proof | 0 | NO_ITEMS | Covered. |
| X:judging:completeness | judging | completeness | full decision trace | 0 | NO_ITEMS | Covered. |
| X:judging:consistency | judging | consistency | stable decision record | 0 | NO_ITEMS | Covered. |
| X:reviewing:necessity | reviewing | necessity | audit input proof | 1 | HAS_ITEMS | Dependency register and status check needed. |
| X:reviewing:sufficiency | reviewing | sufficiency | inspection evidence proof | 0 | NO_ITEMS | Covered. |
| X:reviewing:completeness | reviewing | completeness | full audit trace | 0 | NO_ITEMS | Covered. |
| X:reviewing:consistency | reviewing | consistency | stable inspection record | 0 | NO_ITEMS | Covered. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| X-001 | X:reviewing:necessity | VerificationGap | Procedure | Procedure | Verify `Dependencies.csv`, `_DEPENDENCIES.md`, and `_STATUS.md` before finalizing setup. | SEMANTIC_READY should only remain after setup gates pass. | User brief; `Procedure.md` | `Acceptance/risk notes`; `Procedure.md#Verification` |  | PROPOSAL: run schema and file-scope checks. | TBD |

## Matrix E - Evaluation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| E:guiding:data | guiding | data | governed input notice | 0 | NO_ITEMS | Covered. |
| E:guiding:information | guiding | information | guided context notice | 0 | NO_ITEMS | Covered. |
| E:guiding:knowledge | guiding | knowledge | traced understanding notice | 0 | NO_ITEMS | Covered. |
| E:guiding:wisdom | guiding | wisdom | reliance boundary notice | 1 | HAS_ITEMS | Tutorial notice should remain explicit. |
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
| E-001 | E:guiding:wisdom | RationaleGap | Guidance | Guidance | Preserve tutorial wording that examples are invented and not design bases. | Future tutorials are register-listed but cannot imply engineering reliance. | `docs/DIRECTIVE.md`; `Guidance.md` | `docs/DIRECTIVE.md#3`; `Guidance.md#Non-Reliance Notice` |  | PROPOSAL: require notice in every future example. | TBD |
