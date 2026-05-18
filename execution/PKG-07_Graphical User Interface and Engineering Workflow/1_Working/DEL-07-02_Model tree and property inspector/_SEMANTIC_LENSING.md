# Semantic Lensing Register: DEL-07-02 Model tree and property inspector

**Generated:** 2026-04-30
**Deliverable Folder:** `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector`
**Warnings:** None.

**Inputs Read:**
- `_CONTEXT.md` - deliverable identity, scope, objective, and architecture-basis injection.
- `_STATUS.md` - current state observed as `SEMANTIC_READY`.
- `_SEMANTIC.md` - matrices A, B, C, F, D, X, and E parsed as lenses.
- `Datasheet.md` - setup attributes, conditions, construction notes, references, and open setup questions.
- `Specification.md` - scope, requirements, standards, verification, documentation, and conflict table.
- `Guidance.md` - purpose, principles, considerations, trade-offs, examples, and conflict table.
- `Procedure.md` - purpose, prerequisites, steps, verification, and records.
- `_REFERENCES.md` - reference pointers read; not expanded beyond the sealed context.

**Purpose:** Apply semantic matrix cells as lenses over the production documents, capturing warranted enrichment inputs for later human or TASK enrichment.

## Summary

- Total warranted items: 7
- By document: Datasheet 2; Specification 2; Guidance 2; Procedure 1
- By matrix: A 1; B 1; C 1; F 1; D 1; X 1; E 1
- By type: Conflict 0; VerificationGap 2; MissingSlot 2; WeakStatement 0; RationaleGap 1; Normalization 0; TBD_Question 2; MatrixError 0
- Notable conflicts: 0
- Matrix parse errors: 0

## Matrix A - Orientation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| A:[normative]:[guiding] | normative | guiding | prescriptive direction | 1 | HAS_ITEMS | Ownership TBD item recorded. |
| A:[normative]:[applying] | normative | applying | mandatory practice | 0 | NO_ITEMS | No additional warranted item. |
| A:[normative]:[judging] | normative | judging | compliance determination | 0 | NO_ITEMS | No additional warranted item. |
| A:[normative]:[reviewing] | normative | reviewing | regulatory audit | 0 | NO_ITEMS | No additional warranted item. |
| A:[operative]:[guiding] | operative | guiding | procedural direction | 0 | NO_ITEMS | No additional warranted item. |
| A:[operative]:[applying] | operative | applying | practical execution | 0 | NO_ITEMS | No additional warranted item. |
| A:[operative]:[judging] | operative | judging | performance assessment | 0 | NO_ITEMS | No additional warranted item. |
| A:[operative]:[reviewing] | operative | reviewing | process audit | 0 | NO_ITEMS | No additional warranted item. |
| A:[evaluative]:[guiding] | evaluative | guiding | value orientation | 0 | NO_ITEMS | No additional warranted item. |
| A:[evaluative]:[applying] | evaluative | applying | merit application | 0 | NO_ITEMS | No additional warranted item. |
| A:[evaluative]:[judging] | evaluative | judging | worth determination | 0 | NO_ITEMS | No additional warranted item. |
| A:[evaluative]:[reviewing] | evaluative | reviewing | quality appraisal | 0 | NO_ITEMS | No additional warranted item. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| A-001 | A:[normative]:[guiding] | TBD_Question | Datasheet | Datasheet | Identify accepted owners for tree grouping, inspector field inventory, and command/query contracts. | Datasheet records open setup questions but does not name the future owning contract for each. | Datasheet.md | Open Setup Questions | NA | Keep ownership as TBD until upstream contracts or human authority assign it. | TBD |

## Matrix B - Conceptualization

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| B:[data]:[necessity] | data | necessity | essential fact | 0 | NO_ITEMS | No additional warranted item. |
| B:[data]:[sufficiency] | data | sufficiency | adequate evidence | 0 | NO_ITEMS | No additional warranted item. |
| B:[data]:[completeness] | data | completeness | comprehensive record | 1 | HAS_ITEMS | Missing slot item recorded. |
| B:[data]:[consistency] | data | consistency | reliable measurement | 0 | NO_ITEMS | No additional warranted item. |
| B:[information]:[necessity] | information | necessity | essential signal | 0 | NO_ITEMS | No additional warranted item. |
| B:[information]:[sufficiency] | information | sufficiency | adequate context | 0 | NO_ITEMS | No additional warranted item. |
| B:[information]:[completeness] | information | completeness | comprehensive account | 0 | NO_ITEMS | No additional warranted item. |
| B:[information]:[consistency] | information | consistency | coherent message | 0 | NO_ITEMS | No additional warranted item. |
| B:[knowledge]:[necessity] | knowledge | necessity | fundamental understanding | 0 | NO_ITEMS | No additional warranted item. |
| B:[knowledge]:[sufficiency] | knowledge | sufficiency | competent expertise | 0 | NO_ITEMS | No additional warranted item. |
| B:[knowledge]:[completeness] | knowledge | completeness | thorough mastery | 0 | NO_ITEMS | No additional warranted item. |
| B:[knowledge]:[consistency] | knowledge | consistency | coherent understanding | 0 | NO_ITEMS | No additional warranted item. |
| B:[wisdom]:[necessity] | wisdom | necessity | essential discernment | 0 | NO_ITEMS | No additional warranted item. |
| B:[wisdom]:[sufficiency] | wisdom | sufficiency | adequate judgment | 0 | NO_ITEMS | No additional warranted item. |
| B:[wisdom]:[completeness] | wisdom | completeness | holistic insight | 0 | NO_ITEMS | No additional warranted item. |
| B:[wisdom]:[consistency] | wisdom | consistency | principled reasoning | 0 | NO_ITEMS | No additional warranted item. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| B-001 | B:[data]:[completeness] | MissingSlot | Datasheet | Datasheet | Add a controlled setup checklist for tree groups, inspector fields, unit hooks, provenance hooks, diagnostics, and test fixtures. | Datasheet lists conditions and open questions, but a checklist will make the setup slots easier to audit later. | Datasheet.md | Conditions; Open Setup Questions | NA | Record placeholders only; do not fill values. | TBD |

## Matrix C - Formulation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| C:[normative]:[necessity] | normative | necessity | binding obligation basis | 0 | NO_ITEMS | No additional warranted item. |
| C:[normative]:[sufficiency] | normative | sufficiency | warranted control basis | 1 | HAS_ITEMS | Verification gap item recorded. |
| C:[normative]:[completeness] | normative | completeness | comprehensive control frame | 0 | NO_ITEMS | No additional warranted item. |
| C:[normative]:[consistency] | normative | consistency | coherent governance frame | 0 | NO_ITEMS | No additional warranted item. |
| C:[operative]:[necessity] | operative | necessity | essential execution basis | 0 | NO_ITEMS | No additional warranted item. |
| C:[operative]:[sufficiency] | operative | sufficiency | adequate practice basis | 0 | NO_ITEMS | No additional warranted item. |
| C:[operative]:[completeness] | operative | completeness | whole workflow basis | 0 | NO_ITEMS | No additional warranted item. |
| C:[operative]:[consistency] | operative | consistency | stable process basis | 0 | NO_ITEMS | No additional warranted item. |
| C:[evaluative]:[necessity] | evaluative | necessity | critical value basis | 0 | NO_ITEMS | No additional warranted item. |
| C:[evaluative]:[sufficiency] | evaluative | sufficiency | reasoned appraisal basis | 0 | NO_ITEMS | No additional warranted item. |
| C:[evaluative]:[completeness] | evaluative | completeness | integral appraisal frame | 0 | NO_ITEMS | No additional warranted item. |
| C:[evaluative]:[consistency] | evaluative | consistency | principled appraisal frame | 0 | NO_ITEMS | No additional warranted item. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| C-001 | C:[normative]:[sufficiency] | VerificationGap | Specification | Specification | Add explicit setup verification slots for tree navigation, selection sync, inspector field groups, missing-data visibility, and privacy/provenance display. | Specification has verification areas, but a direct coverage slot list will make later UI test planning more concrete. | Specification.md | Verification | NA | Add slots without selecting framework paths or values. | TBD |

## Matrix F - Requirements

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| F:[normative]:[necessity] | normative | necessity | binding evidence threshold | 0 | NO_ITEMS | No additional warranted item. |
| F:[normative]:[sufficiency] | normative | sufficiency | warranted assurance threshold | 1 | HAS_ITEMS | Verification gap item recorded. |
| F:[normative]:[completeness] | normative | completeness | full control coverage | 0 | NO_ITEMS | No additional warranted item. |
| F:[normative]:[consistency] | normative | consistency | coherent compliance basis | 0 | NO_ITEMS | No additional warranted item. |
| F:[operative]:[necessity] | operative | necessity | required action evidence | 0 | NO_ITEMS | No additional warranted item. |
| F:[operative]:[sufficiency] | operative | sufficiency | adequate execution proof | 0 | NO_ITEMS | No additional warranted item. |
| F:[operative]:[completeness] | operative | completeness | whole practice coverage | 0 | NO_ITEMS | No additional warranted item. |
| F:[operative]:[consistency] | operative | consistency | stable workflow assurance | 0 | NO_ITEMS | No additional warranted item. |
| F:[evaluative]:[necessity] | evaluative | necessity | essential appraisal evidence | 0 | NO_ITEMS | No additional warranted item. |
| F:[evaluative]:[sufficiency] | evaluative | sufficiency | reasoned merit assurance | 0 | NO_ITEMS | No additional warranted item. |
| F:[evaluative]:[completeness] | evaluative | completeness | integral appraisal coverage | 0 | NO_ITEMS | No additional warranted item. |
| F:[evaluative]:[consistency] | evaluative | consistency | principled quality assurance | 0 | NO_ITEMS | No additional warranted item. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| F-001 | F:[normative]:[sufficiency] | VerificationGap | Procedure | Procedure | Add record-level evidence requirements for fixture provenance, protected-content review, and command/query boundary checks. | Procedure names records, but explicit evidence slots reduce later ambiguity. | Procedure.md | Verification; Records | NA | Keep evidence placeholders explicit. | TBD |

## Matrix D - Objectives

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| D:[normative]:[guiding] | normative | guiding | controlled decision charter | 0 | NO_ITEMS | No additional warranted item. |
| D:[normative]:[applying] | normative | applying | enforceable practice boundary | 0 | NO_ITEMS | No additional warranted item. |
| D:[normative]:[judging] | normative | judging | adjudicated closure basis | 0 | NO_ITEMS | No additional warranted item. |
| D:[normative]:[reviewing] | normative | reviewing | audit closure record | 0 | NO_ITEMS | No additional warranted item. |
| D:[operative]:[guiding] | operative | guiding | executable direction charter | 0 | NO_ITEMS | No additional warranted item. |
| D:[operative]:[applying] | operative | applying | governed action protocol | 1 | HAS_ITEMS | Boundary TBD item recorded. |
| D:[operative]:[judging] | operative | judging | measured performance basis | 0 | NO_ITEMS | No additional warranted item. |
| D:[operative]:[reviewing] | operative | reviewing | process audit record | 0 | NO_ITEMS | No additional warranted item. |
| D:[evaluative]:[guiding] | evaluative | guiding | value-aligned direction frame | 0 | NO_ITEMS | No additional warranted item. |
| D:[evaluative]:[applying] | evaluative | applying | merit-grounded practice frame | 0 | NO_ITEMS | No additional warranted item. |
| D:[evaluative]:[judging] | evaluative | judging | defensible appraisal basis | 0 | NO_ITEMS | No additional warranted item. |
| D:[evaluative]:[reviewing] | evaluative | reviewing | quality appraisal record | 0 | NO_ITEMS | No additional warranted item. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| D-001 | D:[operative]:[applying] | TBD_Question | Guidance | Guidance | Clarify split with DEL-07-03 editor work and DEL-07-04 missing-data blocking UX. | Guidance names the scope split, but the exact implementation handoff remains unresolved. | Guidance.md | Trade-offs | NA | Route final split to sealed implementation briefs or human ruling. | TBD |

## Matrix X - Verification

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| X:[guiding]:[necessity] | guiding | necessity | chartered evidence gate | 0 | NO_ITEMS | No additional warranted item. |
| X:[guiding]:[sufficiency] | guiding | sufficiency | charter assurance gate | 0 | NO_ITEMS | No additional warranted item. |
| X:[guiding]:[completeness] | guiding | completeness | charter coverage gate | 0 | NO_ITEMS | No additional warranted item. |
| X:[guiding]:[consistency] | guiding | consistency | charter coherence gate | 0 | NO_ITEMS | No additional warranted item. |
| X:[applying]:[necessity] | applying | necessity | practice evidence gate | 0 | NO_ITEMS | No additional warranted item. |
| X:[applying]:[sufficiency] | applying | sufficiency | practice assurance gate | 0 | NO_ITEMS | No additional warranted item. |
| X:[applying]:[completeness] | applying | completeness | practice coverage gate | 1 | HAS_ITEMS | Missing slot item recorded. |
| X:[applying]:[consistency] | applying | consistency | practice coherence gate | 0 | NO_ITEMS | No additional warranted item. |
| X:[judging]:[necessity] | judging | necessity | decision evidence gate | 0 | NO_ITEMS | No additional warranted item. |
| X:[judging]:[sufficiency] | judging | sufficiency | decision assurance gate | 0 | NO_ITEMS | No additional warranted item. |
| X:[judging]:[completeness] | judging | completeness | decision coverage gate | 0 | NO_ITEMS | No additional warranted item. |
| X:[judging]:[consistency] | judging | consistency | decision coherence gate | 0 | NO_ITEMS | No additional warranted item. |
| X:[reviewing]:[necessity] | reviewing | necessity | audit evidence gate | 0 | NO_ITEMS | No additional warranted item. |
| X:[reviewing]:[sufficiency] | reviewing | sufficiency | audit assurance gate | 0 | NO_ITEMS | No additional warranted item. |
| X:[reviewing]:[completeness] | reviewing | completeness | audit coverage gate | 0 | NO_ITEMS | No additional warranted item. |
| X:[reviewing]:[consistency] | reviewing | consistency | audit coherence gate | 0 | NO_ITEMS | No additional warranted item. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| X-001 | X:[applying]:[completeness] | MissingSlot | Specification | Specification | Add coverage slots for transient/durable state separation and command-backed edit behavior. | These are central AB-00-05 constraints and should be visible in the verification surface. | Specification.md | Requirements; Verification | NA | Add slots without choosing a state library. | TBD |

## Matrix E - Evaluation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| E:[guiding]:[data] | guiding | data | chartered fact validation | 0 | NO_ITEMS | No additional warranted item. |
| E:[guiding]:[information] | guiding | information | chartered signal validation | 0 | NO_ITEMS | No additional warranted item. |
| E:[guiding]:[knowledge] | guiding | knowledge | chartered expertise validation | 0 | NO_ITEMS | No additional warranted item. |
| E:[guiding]:[wisdom] | guiding | wisdom | chartered judgment validation | 0 | NO_ITEMS | No additional warranted item. |
| E:[applying]:[data] | applying | data | practice fact validation | 0 | NO_ITEMS | No additional warranted item. |
| E:[applying]:[information] | applying | information | practice signal validation | 1 | HAS_ITEMS | Rationale gap item recorded. |
| E:[applying]:[knowledge] | applying | knowledge | practice expertise validation | 0 | NO_ITEMS | No additional warranted item. |
| E:[applying]:[wisdom] | applying | wisdom | practice judgment validation | 0 | NO_ITEMS | No additional warranted item. |
| E:[judging]:[data] | judging | data | decision fact validation | 0 | NO_ITEMS | No additional warranted item. |
| E:[judging]:[information] | judging | information | decision signal validation | 0 | NO_ITEMS | No additional warranted item. |
| E:[judging]:[knowledge] | judging | knowledge | decision expertise validation | 0 | NO_ITEMS | No additional warranted item. |
| E:[judging]:[wisdom] | judging | wisdom | decision judgment validation | 0 | NO_ITEMS | No additional warranted item. |
| E:[reviewing]:[data] | reviewing | data | audit fact validation | 0 | NO_ITEMS | No additional warranted item. |
| E:[reviewing]:[information] | reviewing | information | audit signal validation | 0 | NO_ITEMS | No additional warranted item. |
| E:[reviewing]:[knowledge] | reviewing | knowledge | audit expertise validation | 0 | NO_ITEMS | No additional warranted item. |
| E:[reviewing]:[wisdom] | reviewing | wisdom | audit judgment validation | 0 | NO_ITEMS | No additional warranted item. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| E-001 | E:[applying]:[information] | RationaleGap | Guidance | Guidance | Add rationale for command-backed editing rather than direct durable model mutation from UI state. | Guidance states the principle, but a short rationale will help future UI workers preserve the boundary. | Guidance.md | Principles; Trade-offs | NA | Cite AB-00-05 and command/query boundaries when available. | TBD |
