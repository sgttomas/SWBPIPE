# Semantic Lensing Register: DEL-12-05 Security threat model

**Generated:** 2026-04-30
**Deliverable Folder:** execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-05_Security threat model
**Warnings:** NONE

**Inputs Read:**
- _CONTEXT.md — deliverable identity, scope, context envelope, and architecture-basis injection
- _STATUS.md — lifecycle state
- _SEMANTIC.md — refreshed semantic lens
- Datasheet.md — production setup document
- Specification.md — production setup document
- Guidance.md — production setup document
- Procedure.md — production setup document
- _REFERENCES.md — source pointer list

**Purpose:** Apply `semantic-matrix-build` matrix cells as lenses over the production documents, capturing warranted enrichment inputs for a later enrichment pass. The matrices are lenses, not authority.

## Summary

- Total warranted items: 6
- By document:
  - Datasheet: 0
  - Specification: 3
  - Guidance: 2
  - Procedure: 1
- By matrix:
  - A: 1  B: 0  C: 1  F: 1  D: 1  X: 1  E: 1
- By type:
  - Conflict: 0
  - VerificationGap: 2
  - MissingSlot: 2
  - WeakStatement: 1
  - RationaleGap: 0
  - Normalization: 0
  - TBD_Question: 1
  - MatrixError: 0
- Notable conflicts: 0
- Matrix parse errors: 0

## Matrix A — Orientation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| A:normative:guiding | normative | guiding | prescriptive direction | 0 | NO_ITEMS | Covered by requirements and principles. |
| A:normative:applying | normative | applying | mandatory practice | 0 | NO_ITEMS | Covered by requirements. |
| A:normative:judging | normative | judging | compliance determination | 0 | NO_ITEMS | Authority boundary avoids compliance claims. |
| A:normative:reviewing | normative | reviewing | regulatory audit | 0 | NO_ITEMS | Human review boundary stated. |
| A:operative:guiding | operative | guiding | procedural direction | 0 | NO_ITEMS | Procedure covers workflow. |
| A:operative:applying | operative | applying | practical execution | 1 | HAS_ITEMS | Update-trigger execution needed strengthening. |
| A:operative:judging | operative | judging | performance assessment | 0 | NO_ITEMS | Verification table covers checks. |
| A:operative:reviewing | operative | reviewing | process audit | 0 | NO_ITEMS | Records and validation noted. |
| A:evaluative:guiding | evaluative | guiding | value orientation | 0 | NO_ITEMS | Guidance principles cover values. |
| A:evaluative:applying | evaluative | applying | merit application | 0 | NO_ITEMS | Trade-offs covered. |
| A:evaluative:judging | evaluative | judging | worth determination | 0 | NO_ITEMS | Risk posture table covers judgement. |
| A:evaluative:reviewing | evaluative | reviewing | quality appraisal | 0 | NO_ITEMS | Verification expectations present. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| A-001 | A:operative:applying | MissingSlot | Guidance; Procedure | Guidance | Add update triggers for threat-model refresh. | The setup docs identify an evolving architecture but need an operational trigger list so future changes reopen the model. | _CONTEXT.md; Guidance.md; Procedure.md | _CONTEXT.md#Architecture Basis Injection; Guidance.md#Considerations; Procedure.md#Steps | N/A | Add conservative trigger list without resolving implementation TBDs. | TBD |

## Matrix B — Conceptualization

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| B:data:necessity | data | necessity | essential fact | 0 | NO_ITEMS | Scope facts present. |
| B:data:sufficiency | data | sufficiency | adequate evidence | 0 | NO_ITEMS | Source basis cited. |
| B:data:completeness | data | completeness | comprehensive record | 0 | NO_ITEMS | Four-doc kit records content. |
| B:data:consistency | data | consistency | reliable measurement | 0 | NO_ITEMS | No numeric values introduced. |
| B:information:necessity | information | necessity | essential signal | 0 | NO_ITEMS | Threat signals present. |
| B:information:sufficiency | information | sufficiency | adequate context | 0 | NO_ITEMS | Context envelope cited. |
| B:information:completeness | information | completeness | comprehensive account | 0 | NO_ITEMS | Threat surfaces covered. |
| B:information:consistency | information | consistency | coherent message | 0 | NO_ITEMS | Terms consistent. |
| B:knowledge:necessity | knowledge | necessity | fundamental understanding | 0 | NO_ITEMS | Principles explain boundary. |
| B:knowledge:sufficiency | knowledge | sufficiency | competent expertise | 0 | NO_ITEMS | Human authority preserved. |
| B:knowledge:completeness | knowledge | completeness | thorough mastery | 0 | NO_ITEMS | Open questions marked TBD. |
| B:knowledge:consistency | knowledge | consistency | coherent understanding | 0 | NO_ITEMS | No conflicts found. |
| B:wisdom:necessity | wisdom | necessity | essential discernment | 0 | NO_ITEMS | Stop rules cited. |
| B:wisdom:sufficiency | wisdom | sufficiency | adequate judgment | 0 | NO_ITEMS | Trade-offs documented. |
| B:wisdom:completeness | wisdom | completeness | holistic insight | 0 | NO_ITEMS | Scope plus exclusions present. |
| B:wisdom:consistency | wisdom | consistency | principled reasoning | 0 | NO_ITEMS | Professional boundary maintained. |

## Matrix C — Formulation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| C:normative:necessity | normative | necessity | evidence requirement | 0 | NO_ITEMS | Requirements cite sources. |
| C:normative:sufficiency | normative | sufficiency | adequate coverage | 0 | NO_ITEMS | Scope rows cover core surfaces. |
| C:normative:completeness | normative | completeness | full scope record | 0 | NO_ITEMS | Scope, assets, and controls present. |
| C:normative:consistency | normative | consistency | coherent evidence | 0 | NO_ITEMS | Evidence labels consistent. |
| C:operative:necessity | operative | necessity | required action | 0 | NO_ITEMS | Procedure steps present. |
| C:operative:sufficiency | operative | sufficiency | adequate action | 0 | NO_ITEMS | Validation commands listed. |
| C:operative:completeness | operative | completeness | full workflow | 0 | NO_ITEMS | Setup phases covered. |
| C:operative:consistency | operative | consistency | stable workflow | 0 | NO_ITEMS | Status boundary stated. |
| C:evaluative:necessity | evaluative | necessity | risk priority | 0 | NO_ITEMS | High-risk rows present. |
| C:evaluative:sufficiency | evaluative | sufficiency | measured adequacy | 0 | NO_ITEMS | Quantitative scoring remains TBD. |
| C:evaluative:completeness | evaluative | completeness | complete risk picture | 1 | HAS_ITEMS | Supply-chain surface needed explicit threat row. |
| C:evaluative:consistency | evaluative | consistency | aligned judgment | 0 | NO_ITEMS | Trade-offs align with sources. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| C-001 | C:evaluative:completeness | MissingSlot | Specification | Specification | Add supply-chain exposure to the threat inventory. | The brief names supply chain exposure and architecture basis requires layered security/release gates, so the threat list should not stop at local/report/plugin surfaces. | Specification.md; _CONTEXT.md; docs/_Decomposition/SOFTWARE_DECOMP.md | Specification.md#Threat Inventory; _CONTEXT.md#Architecture Basis Injection; SOFTWARE_DECOMP.md#8.1 Architecture basis register | N/A | Add supply-chain row with implementation details left TBD. | TBD |

## Matrix F — Requirements

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| F:normative:necessity | normative | necessity | threat evidence | 0 | NO_ITEMS | Threat rows source-grounded. |
| F:normative:sufficiency | normative | sufficiency | sufficient coverage | 0 | NO_ITEMS | Core sources cited. |
| F:normative:completeness | normative | completeness | complete coverage | 0 | NO_ITEMS | Out-of-scope stated. |
| F:normative:consistency | normative | consistency | consistent evidence | 0 | NO_ITEMS | No source conflict found. |
| F:operative:necessity | operative | necessity | disclosure action | 1 | HAS_ITEMS | Export/report verification needed explicit handling. |
| F:operative:sufficiency | operative | sufficiency | adequate mitigation | 0 | NO_ITEMS | Mitigations listed. |
| F:operative:completeness | operative | completeness | complete mitigation | 0 | NO_ITEMS | Controls cover surfaces. |
| F:operative:consistency | operative | consistency | stable mitigation | 0 | NO_ITEMS | Status unchanged. |
| F:evaluative:necessity | evaluative | necessity | priority threat | 0 | NO_ITEMS | High-risk rows present. |
| F:evaluative:sufficiency | evaluative | sufficiency | sufficient threat | 0 | NO_ITEMS | Risk posture qualitative only. |
| F:evaluative:completeness | evaluative | completeness | complete threat | 0 | NO_ITEMS | Threat inventory covers requested areas. |
| F:evaluative:consistency | evaluative | consistency | aligned threat | 0 | NO_ITEMS | No contradiction found. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| F-001 | F:operative:necessity | VerificationGap | Specification | Specification | Add acceptance signal for report/export redaction and warning controls. | SOW-040 requires redaction/export safeguards; the verification table should explicitly check that path. | Specification.md; docs/_Decomposition/SOFTWARE_DECOMP.md; docs/PRD.md | Specification.md#Verification; SOFTWARE_DECOMP.md#SOW-040; PRD.md#18.3 Private Data Protection | N/A | Add a verification row for redaction, warning, and protected-content lint controls. | TBD |

## Matrix D — Objectives

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| D:normative:guiding | normative | guiding | disclosure guidance | 0 | NO_ITEMS | Guidance clear. |
| D:normative:applying | normative | applying | control application | 0 | NO_ITEMS | Requirements table present. |
| D:normative:judging | normative | judging | risk judgment | 0 | NO_ITEMS | No compliance overclaim. |
| D:normative:reviewing | normative | reviewing | review guidance | 0 | NO_ITEMS | Human review boundary present. |
| D:operative:guiding | operative | guiding | action guidance | 0 | NO_ITEMS | Procedure present. |
| D:operative:applying | operative | applying | mitigation practice | 1 | HAS_ITEMS | Plugin/import no-bypass language needed precision. |
| D:operative:judging | operative | judging | operational judgment | 0 | NO_ITEMS | Verification rows present. |
| D:operative:reviewing | operative | reviewing | review practice | 0 | NO_ITEMS | Records present. |
| D:evaluative:guiding | evaluative | guiding | risk framing | 0 | NO_ITEMS | Risk framing present. |
| D:evaluative:applying | evaluative | applying | measured control | 0 | NO_ITEMS | Implementation scoring TBD. |
| D:evaluative:judging | evaluative | judging | complete judgment | 0 | NO_ITEMS | No active conflict. |
| D:evaluative:reviewing | evaluative | reviewing | aligned review | 0 | NO_ITEMS | Quality checks listed. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| D-001 | D:operative:applying | WeakStatement | Specification; Guidance | Specification | Clarify that plugins/imports cannot bypass validation, provenance, diagnostics, sandboxing, or report controls. | The architecture basis is specific; weak wording would make plugin/import threat treatment ambiguous. | _CONTEXT.md; Specification.md; Guidance.md | _CONTEXT.md#Architecture Basis Injection; Specification.md#Requirements; Guidance.md#Trust Boundaries | N/A | Use AB-00-07 no-bypass language and keep exact permission model TBD. | TBD |

## Matrix X — Verification

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| X:guiding:necessity | guiding | necessity | disclosure check | 0 | NO_ITEMS | Verification table covers disclosure. |
| X:guiding:sufficiency | guiding | sufficiency | coverage check | 0 | NO_ITEMS | Coverage complete. |
| X:guiding:completeness | guiding | completeness | complete check | 0 | NO_ITEMS | Default sections present. |
| X:guiding:consistency | guiding | consistency | aligned check | 0 | NO_ITEMS | Consistency pass recorded. |
| X:applying:necessity | applying | necessity | control check | 0 | NO_ITEMS | Controls in threat table. |
| X:applying:sufficiency | applying | sufficiency | adequate check | 0 | NO_ITEMS | Validation command included. |
| X:applying:completeness | applying | completeness | complete control | 0 | NO_ITEMS | Scope surfaces covered. |
| X:applying:consistency | applying | consistency | stable control | 0 | NO_ITEMS | Status remains SEMANTIC_READY. |
| X:judging:necessity | judging | necessity | risk check | 0 | NO_ITEMS | Risk posture qualitative. |
| X:judging:sufficiency | judging | sufficiency | adequate risk | 0 | NO_ITEMS | No numeric scoring invented. |
| X:judging:completeness | judging | completeness | complete risk | 0 | NO_ITEMS | Threat inventory broad enough. |
| X:judging:consistency | judging | consistency | coherent risk | 0 | NO_ITEMS | No conflict found. |
| X:reviewing:necessity | reviewing | necessity | review check | 0 | NO_ITEMS | Review records listed. |
| X:reviewing:sufficiency | reviewing | sufficiency | adequate review | 0 | NO_ITEMS | Run records required. |
| X:reviewing:completeness | reviewing | completeness | complete review | 0 | NO_ITEMS | Procedure covers phases. |
| X:reviewing:consistency | reviewing | consistency | stable review | 1 | HAS_ITEMS | Final validation checklist needed explicit semantic/dependency/status checks. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| X-001 | X:reviewing:consistency | VerificationGap | Procedure | Procedure | Add final validation checks for semantic audit, dependency schema, status, and write scope. | The brief gives explicit acceptance criteria that need procedure-level verification hooks. | Procedure.md; _CONTEXT.md | Procedure.md#Verification; _CONTEXT.md#Context Budget QA | N/A | Add validation checklist aligned to sealed brief acceptance criteria. | TBD |

## Matrix E — Evaluation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| E:guiding:data | guiding | data | disclosure datum | 0 | NO_ITEMS | Asset classes present. |
| E:guiding:information | guiding | information | disclosure context | 0 | NO_ITEMS | Disclosure boundary present. |
| E:guiding:knowledge | guiding | knowledge | disclosure knowledge | 0 | NO_ITEMS | Guidance principles present. |
| E:guiding:wisdom | guiding | wisdom | disclosure judgment | 1 | HAS_ITEMS | Open implementation questions should stay explicit. |
| E:applying:data | applying | data | control datum | 0 | NO_ITEMS | Control themes present. |
| E:applying:information | applying | information | control context | 0 | NO_ITEMS | Controls cite sources. |
| E:applying:knowledge | applying | knowledge | control knowledge | 0 | NO_ITEMS | No-bypass rule included. |
| E:applying:wisdom | applying | wisdom | control judgment | 0 | NO_ITEMS | Trade-offs present. |
| E:judging:data | judging | data | risk datum | 0 | NO_ITEMS | Threat rows present. |
| E:judging:information | judging | information | risk context | 0 | NO_ITEMS | Risk context present. |
| E:judging:knowledge | judging | knowledge | risk knowledge | 0 | NO_ITEMS | Source-grounded. |
| E:judging:wisdom | judging | wisdom | risk judgment | 0 | NO_ITEMS | No overclaim. |
| E:reviewing:data | reviewing | data | review datum | 0 | NO_ITEMS | Records listed. |
| E:reviewing:information | reviewing | information | review context | 0 | NO_ITEMS | Run records required. |
| E:reviewing:knowledge | reviewing | knowledge | review knowledge | 0 | NO_ITEMS | Verification criteria present. |
| E:reviewing:wisdom | reviewing | wisdom | review judgment | 0 | NO_ITEMS | Human authority preserved. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| E-001 | E:guiding:wisdom | TBD_Question | Guidance | Guidance | Record unresolved encryption, plugin permission, API transport, secret storage, and project package questions as TBD. | The architecture basis explicitly leaves implementation choices unresolved; the threat model should expose these rather than imply decisions. | _CONTEXT.md; Guidance.md; docs/PRD.md | _CONTEXT.md#Architecture Basis Injection; Guidance.md#Open Questions; PRD.md#18.3 Private Data Protection | N/A | Keep unresolved implementation choices as open questions until human-approved design decisions exist. | TBD |

