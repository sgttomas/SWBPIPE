# Semantic Lensing Register: DEL-13-04 Physical-to-analytical transformation contract

**Generated:** 2026-05-03
**Deliverable Folder:** execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/DEL-13-04_Physical-to-analytical transformation contract
**Warnings:** none

**Inputs Read:**
- _CONTEXT.md - deliverable identity, SOW-066, OBJ-014
- _STATUS.md - SEMANTIC_READY after semantic-matrix-build
- _SEMANTIC.md - matrices A, B, C, F, D, X, E parsed as lenses
- Datasheet.md - present
- Specification.md - present
- Guidance.md - present
- Procedure.md - present
- _REFERENCES.md - present; pointers listed without expanding write scope

**Purpose:** Apply `semantic-matrix-build` matrix cells as lenses over the production documents, capturing warranted enrichment inputs for a later enrichment pass.

## Summary

- Total warranted items: 13
- By document:
  - Datasheet: 1
  - Specification: 5
  - Guidance: 5
  - Procedure: 2
- By matrix:
  - A: 2  B: 2  C: 2  F: 2  D: 1  X: 2  E: 2
- By type:
  - Conflict: 0
  - VerificationGap: 6
  - MissingSlot: 3
  - WeakStatement: 0
  - RationaleGap: 1
  - Normalization: 1
  - TBD_Question: 2
  - MatrixError: 0
- Notable conflicts: 0
- Matrix parse errors: 0

## Matrix A - Orientation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| A:normative:guiding | normative | guiding | prescriptive direction | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| A:normative:applying | normative | applying | mandatory practice | 1 | HAS_ITEMS | warranted item recorded |
| A:normative:judging | normative | judging | compliance determination | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| A:normative:reviewing | normative | reviewing | regulatory audit | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| A:operative:guiding | operative | guiding | procedural direction | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| A:operative:applying | operative | applying | practical execution | 1 | HAS_ITEMS | warranted item recorded |
| A:operative:judging | operative | judging | performance assessment | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| A:operative:reviewing | operative | reviewing | process audit | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| A:evaluative:guiding | evaluative | guiding | value orientation | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| A:evaluative:applying | evaluative | applying | merit application | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| A:evaluative:judging | evaluative | judging | worth determination | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| A:evaluative:reviewing | evaluative | reviewing | quality appraisal | 0 | NO_ITEMS | no warranted enrichment item after document scan |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| A-001 | A:normative:applying | VerificationGap | Specification | Specification.md | Add exact pass/fail acceptance criteria for deterministic transform repeatability. | The specification requires deterministic repeat behavior, but fixture basis and equivalence criteria are still TBD. | Specification.md | Verification |  | PROPOSAL: Specification owns acceptance criteria. | TBD |
| A-002 | A:operative:applying | MissingSlot | Datasheet | Datasheet.md | TBD: define implementation module/path and command or service boundary. | The datasheet and procedure identify the transform contract but no accessible source fixes the implementation boundary. | Datasheet.md; Procedure.md | Construction; Steps |  | PROPOSAL: Datasheet records descriptive location once approved. | TBD |

## Matrix B - Conceptualization

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| B:data:necessity | data | necessity | essential fact | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| B:data:sufficiency | data | sufficiency | adequate evidence | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| B:data:completeness | data | completeness | comprehensive record | 1 | HAS_ITEMS | warranted item recorded |
| B:data:consistency | data | consistency | reliable measurement | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| B:information:necessity | information | necessity | essential signal | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| B:information:sufficiency | information | sufficiency | adequate context | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| B:information:completeness | information | completeness | comprehensive account | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| B:information:consistency | information | consistency | coherent message | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| B:knowledge:necessity | knowledge | necessity | fundamental understanding | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| B:knowledge:sufficiency | knowledge | sufficiency | competent expertise | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| B:knowledge:completeness | knowledge | completeness | thorough mastery | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| B:knowledge:consistency | knowledge | consistency | coherent understanding | 1 | HAS_ITEMS | warranted item recorded |
| B:wisdom:necessity | wisdom | necessity | essential discernment | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| B:wisdom:sufficiency | wisdom | sufficiency | adequate judgment | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| B:wisdom:completeness | wisdom | completeness | holistic insight | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| B:wisdom:consistency | wisdom | consistency | principled reasoning | 0 | NO_ITEMS | no warranted enrichment item after document scan |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| B-001 | B:data:completeness | TBD_Question | Guidance | Guidance.md | TBD: locate PRD v0.2 section 8.3 / FR-MOD-007 or keep PRD-specific clauses excluded. | The decomposition cites PRD material, but that source is not locally available through this deliverable reference set. | Guidance.md; Specification.md | Source Access Gaps; Standards |  | PROPOSAL: Unavailable PRD text remains non-authoritative. | TBD |
| B-002 | B:knowledge:consistency | Normalization | Guidance | Guidance.md | Clarify canonical vocabulary for transformation warning, diagnostic, omission, assumption, and traceability record. | The documents use warning/diagnostic language conservatively, but final canonical naming remains open and could drift during implementation. | Guidance.md; Specification.md | Diagnostic Placement; Requirements |  | PROPOSAL: Guidance should hold vocabulary notes after source approval. | TBD |

## Matrix C - Formulation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| C:normative:necessity | normative | necessity | binding prerequisite | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| C:normative:sufficiency | normative | sufficiency | evidence mandate | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| C:normative:completeness | normative | completeness | coverage obligation | 1 | HAS_ITEMS | warranted item recorded |
| C:normative:consistency | normative | consistency | coherence rule | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| C:operative:necessity | operative | necessity | execution prerequisite | 1 | HAS_ITEMS | warranted item recorded |
| C:operative:sufficiency | operative | sufficiency | context adequacy | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| C:operative:completeness | operative | completeness | workflow coverage | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| C:operative:consistency | operative | consistency | process coherence | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| C:evaluative:necessity | evaluative | necessity | review criterion | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| C:evaluative:sufficiency | evaluative | sufficiency | judgment basis | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| C:evaluative:completeness | evaluative | completeness | appraisal coverage | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| C:evaluative:consistency | evaluative | consistency | quality coherence | 0 | NO_ITEMS | no warranted enrichment item after document scan |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| C-001 | C:normative:completeness | MissingSlot | Guidance | Specification.md | TBD: define transform-loss taxonomy when architecture detail is approved. | Guidance notes OI-012: transformation loss classes require technical architecture detail before implementation briefs are sealed. | Guidance.md | Transform-Loss Classes |  | PROPOSAL: Specification should receive normative taxonomy only after approval. | TBD |
| C-002 | C:operative:necessity | VerificationGap | Procedure | Procedure.md | Add concrete negative fixture list once source-cleared examples exist. | The procedure requires warning tests but exact fixtures are TBD because no source-grounded examples are available. | Procedure.md; Guidance.md | Define Warning Tests; Examples |  | PROPOSAL: Procedure owns fixture execution steps. | TBD |

## Matrix F - Requirements

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| F:normative:necessity | normative | necessity | requirement basis | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| F:normative:sufficiency | normative | sufficiency | proof threshold | 1 | HAS_ITEMS | warranted item recorded |
| F:normative:completeness | normative | completeness | closure standard | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| F:normative:consistency | normative | consistency | alignment rule | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| F:operative:necessity | operative | necessity | input readiness | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| F:operative:sufficiency | operative | sufficiency | working adequacy | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| F:operative:completeness | operative | completeness | execution closure | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| F:operative:consistency | operative | consistency | repeatable process | 1 | HAS_ITEMS | warranted item recorded |
| F:evaluative:necessity | evaluative | necessity | acceptance basis | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| F:evaluative:sufficiency | evaluative | sufficiency | review adequacy | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| F:evaluative:completeness | evaluative | completeness | assurance coverage | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| F:evaluative:consistency | evaluative | consistency | appraisal coherence | 0 | NO_ITEMS | no warranted enrichment item after document scan |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| F-001 | F:normative:sufficiency | VerificationGap | Specification | Specification.md | Add traceability coverage acceptance rule for every output, warning, omission, and assumption. | Traceability is required, but exact coverage threshold and failure behavior are not yet specified. | Specification.md | Requirements DEL-13-04-REQ-004; Verification |  | PROPOSAL: Specification owns coverage criteria. | TBD |
| F-002 | F:operative:consistency | VerificationGap | Procedure | Procedure.md | Define stable ordering, canonicalization, or hash basis if analytical output comparison needs it. | The procedure records deterministic behavior but leaves sorting/canonicalization/hash rules TBD. | Procedure.md | Define Determinism Rules |  | PROPOSAL: Procedure records operational comparison method after architecture source approval. | TBD |

## Matrix D - Objectives

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| D:normative:guiding | normative | guiding | authoritative direction | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| D:normative:applying | normative | applying | enforced practice | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| D:normative:judging | normative | judging | conformance closure | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| D:normative:reviewing | normative | reviewing | audit closure | 1 | HAS_ITEMS | warranted item recorded |
| D:operative:guiding | operative | guiding | procedural closure | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| D:operative:applying | operative | applying | field execution | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| D:operative:judging | operative | judging | performance closure | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| D:operative:reviewing | operative | reviewing | process assurance | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| D:evaluative:guiding | evaluative | guiding | value resolution | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| D:evaluative:applying | evaluative | applying | merit enactment | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| D:evaluative:judging | evaluative | judging | worth closure | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| D:evaluative:reviewing | evaluative | reviewing | quality assurance | 0 | NO_ITEMS | no warranted enrichment item after document scan |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| D-001 | D:normative:reviewing | RationaleGap | Guidance | Guidance.md | Record authority source for final warning class catalog before implementation. | The documents reference project warning classes and transform warnings, but final transform warning class authority is not yet sourced. | Guidance.md; docs/SPEC.md | Diagnostic Placement; missing-data warning classes |  | PROPOSAL: Guidance records rationale; Specification records final classes. | TBD |

## Matrix X - Verification

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| X:guiding:necessity | guiding | necessity | directional evidence | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| X:guiding:sufficiency | guiding | sufficiency | context threshold | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| X:guiding:completeness | guiding | completeness | coverage guide | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| X:guiding:consistency | guiding | consistency | coherence guide | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| X:applying:necessity | applying | necessity | practice evidence | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| X:applying:sufficiency | applying | sufficiency | implementation threshold | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| X:applying:completeness | applying | completeness | closure support | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| X:applying:consistency | applying | consistency | stable execution | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| X:judging:necessity | judging | necessity | decision evidence | 1 | HAS_ITEMS | warranted item recorded |
| X:judging:sufficiency | judging | sufficiency | assessment threshold | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| X:judging:completeness | judging | completeness | closure proof | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| X:judging:consistency | judging | consistency | coherent decision | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| X:reviewing:necessity | reviewing | necessity | audit evidence | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| X:reviewing:sufficiency | reviewing | sufficiency | assurance threshold | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| X:reviewing:completeness | reviewing | completeness | audit coverage | 1 | HAS_ITEMS | warranted item recorded |
| X:reviewing:consistency | reviewing | consistency | coherent assurance | 0 | NO_ITEMS | no warranted enrichment item after document scan |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| X-001 | X:judging:necessity | VerificationGap | Specification | Specification.md | Add a text/schema check that transform outputs avoid professional or code-compliance claims. | The professional boundary is specified, but an explicit contract-level verification check is still additive and warranted. | Specification.md; docs/CONTRACT.md | Requirements DEL-13-04-REQ-009; OPS-K-AUTH-1 |  | PROPOSAL: Specification owns verification requirement. | TBD |
| X-002 | X:reviewing:completeness | VerificationGap | Specification | Specification.md | Add protected-content review evidence for public transform fixtures. | The protected-content boundary is required, but fixture review evidence is not yet defined. | Specification.md; Procedure.md | Verification; Review Data Boundary |  | PROPOSAL: Specification owns evidence requirement; Procedure owns review step. | TBD |

## Matrix E - Evaluation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| E:guiding:data | guiding | data | source orientation | 1 | HAS_ITEMS | warranted item recorded |
| E:guiding:information | guiding | information | context orientation | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| E:guiding:knowledge | guiding | knowledge | expert direction | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| E:guiding:wisdom | guiding | wisdom | reasoned direction | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| E:applying:data | applying | data | source execution | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| E:applying:information | applying | information | context execution | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| E:applying:knowledge | applying | knowledge | expert practice | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| E:applying:wisdom | applying | wisdom | reasoned practice | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| E:judging:data | judging | data | source assessment | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| E:judging:information | judging | information | context assessment | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| E:judging:knowledge | judging | knowledge | expert determination | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| E:judging:wisdom | judging | wisdom | reasoned determination | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| E:reviewing:data | reviewing | data | source assurance | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| E:reviewing:information | reviewing | information | context assurance | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| E:reviewing:knowledge | reviewing | knowledge | expert appraisal | 0 | NO_ITEMS | no warranted enrichment item after document scan |
| E:reviewing:wisdom | reviewing | wisdom | reasoned appraisal | 1 | HAS_ITEMS | warranted item recorded |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| E-001 | E:guiding:data | MissingSlot | Guidance | Guidance.md | Add source-cleared transform example only when invented or permissive fixture exists. | Guidance currently records that no source-grounded examples are locally available. | Guidance.md | Examples |  | PROPOSAL: Guidance may host examples after source clearance. | TBD |
| E-002 | E:reviewing:wisdom | TBD_Question | Specification | Specification.md | TBD: human decision needed on final loss-class taxonomy and schema field ownership. | The setup draft cannot resolve ownership of detailed transform-loss classes or schema fields without approved architecture/detail sources. | Guidance.md; Specification.md | Source Access Gaps; Scope |  | PROPOSAL: Human/architecture ruling required before normative closure. | TBD |

## Register Boundary

This register is a candidate worklist only. It does not authorize changes, resolve TBDs, override the production documents, or supply engineering particulars. Human rulings remain TBD unless an approved source is cited in a later pass.
