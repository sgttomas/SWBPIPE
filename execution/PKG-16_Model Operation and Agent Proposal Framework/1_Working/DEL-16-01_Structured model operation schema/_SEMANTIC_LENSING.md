# Semantic Lensing Register: DEL-16-01 Structured model operation schema

**Generated:** 2026-05-04
**Deliverable Folder:** `execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-01_Structured model operation schema`
**Warnings:** None

**Inputs Read:**
- `_CONTEXT.md` - deliverable identity and scope context.
- `_STATUS.md` - current state observed as `SEMANTIC_READY`.
- `_SEMANTIC.md` - matrices A, B, C, F, D, X, E parsed.
- `Datasheet.md` - production document read.
- `Specification.md` - production document read.
- `Guidance.md` - production document read.
- `Procedure.md` - production document read.
- `_REFERENCES.md` - reference list read; pointers not expanded by this lensing pass.

**Purpose:** Apply `semantic-matrix-build` matrix cells as lenses over the production documents, capturing warranted enrichment inputs for a later enrichment pass.

## Summary

- Total warranted items: 5
- By document:
  - Datasheet: 1
  - Specification: 2
  - Guidance: 1
  - Procedure: 1
- By matrix:
  - A: 0  B: 0  C: 1  F: 1  D: 1  X: 1  E: 1
- By type:
  - Conflict: 0
  - VerificationGap: 2
  - MissingSlot: 1
  - WeakStatement: 0
  - RationaleGap: 0
  - Normalization: 0
  - TBD_Question: 2
  - MatrixError: 0
- Notable conflicts: 0
- Matrix parse errors: 0

## Matrix A - Orientation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| A:normative:guiding | normative | guiding | prescriptive direction | 0 | NO_ITEMS | No additional warranted item beyond current TBD markings. |
| A:normative:applying | normative | applying | mandatory practice | 0 | NO_ITEMS | No additional warranted item beyond current TBD markings. |
| A:normative:judging | normative | judging | compliance determination | 0 | NO_ITEMS | Professional-boundary constraint is already present. |
| A:normative:reviewing | normative | reviewing | regulatory audit | 0 | NO_ITEMS | Audit/provenance boundary is already identified at setup level. |
| A:operative:guiding | operative | guiding | procedural direction | 0 | NO_ITEMS | Procedure covers setup workflow at a conservative level. |
| A:operative:applying | operative | applying | practical execution | 0 | NO_ITEMS | Implementation steps are intentionally TBD. |
| A:operative:judging | operative | judging | performance assessment | 0 | NO_ITEMS | Verification section already records future checks as TBD. |
| A:operative:reviewing | operative | reviewing | process audit | 0 | NO_ITEMS | No conflict detected across production docs. |
| A:evaluative:guiding | evaluative | guiding | value orientation | 0 | NO_ITEMS | Guidance records project-boundary principles. |
| A:evaluative:applying | evaluative | applying | merit application | 0 | NO_ITEMS | No warranted enrichment beyond current TBDs. |
| A:evaluative:judging | evaluative | judging | worth determination | 0 | NO_ITEMS | No conflict detected across production docs. |
| A:evaluative:reviewing | evaluative | reviewing | quality appraisal | 0 | NO_ITEMS | Setup quality posture is explicit. |

## Matrix B - Conceptualization

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| B:data:necessity | data | necessity | essential fact | 0 | NO_ITEMS | Identification facts are present. |
| B:data:sufficiency | data | sufficiency | adequate evidence | 0 | NO_ITEMS | Sources are listed; missing details are marked TBD. |
| B:data:completeness | data | completeness | comprehensive record | 0 | NO_ITEMS | Setup docs do not claim complete implementation data. |
| B:data:consistency | data | consistency | reliable measurement | 0 | NO_ITEMS | No numeric values are introduced. |
| B:information:necessity | information | necessity | essential signal | 0 | NO_ITEMS | Scope signal is present. |
| B:information:sufficiency | information | sufficiency | adequate context | 0 | NO_ITEMS | Context is enough for setup, not implementation. |
| B:information:completeness | information | completeness | comprehensive account | 0 | NO_ITEMS | Missing implementation decisions are explicitly TBD. |
| B:information:consistency | information | consistency | coherent message | 0 | NO_ITEMS | Terminology is consistent across docs. |
| B:knowledge:necessity | knowledge | necessity | fundamental understanding | 0 | NO_ITEMS | Boundary principles are present. |
| B:knowledge:sufficiency | knowledge | sufficiency | competent expertise | 0 | NO_ITEMS | No professional judgment is asserted. |
| B:knowledge:completeness | knowledge | completeness | thorough mastery | 0 | NO_ITEMS | Full schema expertise remains future work. |
| B:knowledge:consistency | knowledge | consistency | coherent understanding | 0 | NO_ITEMS | No conflict detected. |
| B:wisdom:necessity | wisdom | necessity | essential discernment | 0 | NO_ITEMS | Conservative source-bound posture is visible. |
| B:wisdom:sufficiency | wisdom | sufficiency | adequate judgment | 0 | NO_ITEMS | Human-ruling areas are marked TBD. |
| B:wisdom:completeness | wisdom | completeness | holistic insight | 0 | NO_ITEMS | Not forced beyond available evidence. |
| B:wisdom:consistency | wisdom | consistency | principled reasoning | 0 | NO_ITEMS | Professional/IP boundary is consistent. |

## Matrix C - Formulation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| C:normative:necessity | normative | necessity | binding input rationale | 1 | HAS_ITEMS | Operation granularity and field layout remain unresolved. |
| C:normative:sufficiency | normative | sufficiency | accepted proof threshold | 0 | NO_ITEMS | Existing docs mark proof/validation details as TBD. |
| C:normative:completeness | normative | completeness | covered obligation record | 0 | NO_ITEMS | Scope obligations are traced at setup level. |
| C:normative:consistency | normative | consistency | stable conformance message | 0 | NO_ITEMS | No terminology conflict detected. |
| C:operative:necessity | operative | necessity | required execution input | 0 | NO_ITEMS | Procedure prerequisites are present. |
| C:operative:sufficiency | operative | sufficiency | workable evidence basis | 0 | NO_ITEMS | Conservative source basis is present. |
| C:operative:completeness | operative | completeness | finished activity account | 0 | NO_ITEMS | Setup procedure lists expected records. |
| C:operative:consistency | operative | consistency | repeatable process signal | 0 | NO_ITEMS | No repeated-process conflict detected. |
| C:evaluative:necessity | evaluative | necessity | critical appraisal basis | 0 | NO_ITEMS | Guidance marks human-ruling items. |
| C:evaluative:sufficiency | evaluative | sufficiency | balanced judgment support | 0 | NO_ITEMS | No unsupported judgment is asserted. |
| C:evaluative:completeness | evaluative | completeness | full review perspective | 0 | NO_ITEMS | Setup does not claim complete implementation review. |
| C:evaluative:consistency | evaluative | consistency | coherent quality rationale | 0 | NO_ITEMS | Quality rationale is consistent with sources. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| C-001 | C:normative:necessity | TBD_Question | Multi | Specification | TBD: resolve operation granularity and operation variant/field layout. | Production docs identify required operation categories but available sources do not decide subtype structure, payload grammar, or exact fields. | `Specification.md`; `Guidance.md` | `## Requirements`; `## Conflict Table (for human ruling)` CT-001 |  | PROPOSAL: keep field-level schema decisions TBD until architecture/human ruling. | TBD |

## Matrix F - Requirements

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| F:normative:necessity | normative | necessity | controlling schema prerequisite | 0 | NO_ITEMS | Schema prerequisite is stated. |
| F:normative:sufficiency | normative | sufficiency | reviewable evidence threshold | 0 | NO_ITEMS | Review evidence is TBD but acknowledged. |
| F:normative:completeness | normative | completeness | full obligation coverage | 1 | HAS_ITEMS | Acceptance criteria need concrete completion once schema exists. |
| F:normative:consistency | normative | consistency | stable rule alignment | 0 | NO_ITEMS | No rule-alignment conflict detected. |
| F:operative:necessity | operative | necessity | executable input gate | 0 | NO_ITEMS | Input gate is described, not implemented. |
| F:operative:sufficiency | operative | sufficiency | validated action basis | 0 | NO_ITEMS | Validation behavior remains DEL-16-02. |
| F:operative:completeness | operative | completeness | whole workflow coverage | 0 | NO_ITEMS | Workflow split is explicit. |
| F:operative:consistency | operative | consistency | repeatable application behavior | 0 | NO_ITEMS | Controlled-application language is consistent. |
| F:evaluative:necessity | evaluative | necessity | decision basis checkpoint | 0 | NO_ITEMS | Human-ruling needs are marked. |
| F:evaluative:sufficiency | evaluative | sufficiency | balanced review support | 0 | NO_ITEMS | No unsupported review outcome is asserted. |
| F:evaluative:completeness | evaluative | completeness | end-to-end appraisal record | 0 | NO_ITEMS | Appraisal record remains future implementation evidence. |
| F:evaluative:consistency | evaluative | consistency | coherent acceptance rationale | 0 | NO_ITEMS | Acceptance boundary is consistent. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| F-001 | F:normative:completeness | VerificationGap | Specification | Specification | Add acceptance criteria for schema validation and category fixtures once schema exists. | The verification table lists future schema and fixture checks, but concrete commands and pass criteria cannot be supplied until the schema artifact exists. | `Specification.md` | `## Verification` |  | PROPOSAL: later enrichment should add exact validator command, fixture inventory, and expected outcomes. | TBD |

## Matrix D - Objectives

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| D:normative:guiding | normative | guiding | bounded directive closure | 0 | NO_ITEMS | Boundary is conservative. |
| D:normative:applying | normative | applying | controlled practice gate | 0 | NO_ITEMS | Controlled route is explicit. |
| D:normative:judging | normative | judging | verified acceptance decision | 0 | NO_ITEMS | Software acceptance claims are excluded. |
| D:normative:reviewing | normative | reviewing | traceable audit posture | 0 | NO_ITEMS | Audit posture is noted as future evidence. |
| D:operative:guiding | operative | guiding | repeatable work direction | 0 | NO_ITEMS | Procedure steps are repeatable at setup level. |
| D:operative:applying | operative | applying | validated execution route | 0 | NO_ITEMS | Validation implementation remains future work. |
| D:operative:judging | operative | judging | measured outcome check | 0 | NO_ITEMS | No measures are invented. |
| D:operative:reviewing | operative | reviewing | process evidence review | 0 | NO_ITEMS | Process evidence is listed as future record. |
| D:evaluative:guiding | evaluative | guiding | principled review orientation | 1 | HAS_ITEMS | Agent autonomy remains unresolved. |
| D:evaluative:applying | evaluative | applying | reasoned judgment use | 0 | NO_ITEMS | Human judgment boundary is visible. |
| D:evaluative:judging | evaluative | judging | quality decision basis | 0 | NO_ITEMS | No software quality decision is overclaimed. |
| D:evaluative:reviewing | evaluative | reviewing | evidence appraisal posture | 0 | NO_ITEMS | Evidence posture is conservative. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| D-001 | D:evaluative:guiding | TBD_Question | Guidance | Guidance | TBD: resolve agent operation autonomy and acceptance boundary for proposed operations. | Guidance records CT-002 because agent edits are in scope while autonomy level remains unresolved by available sources. | `Guidance.md` | `## Conflict Table (for human ruling)` CT-002 |  | PROPOSAL: default to proposal-only operations requiring controlled application and later acceptance workflow. | TBD |

## Matrix X - Verification

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| X:guiding:necessity | guiding | necessity | directive input basis | 0 | NO_ITEMS | Required source inputs are identified. |
| X:guiding:sufficiency | guiding | sufficiency | instruction evidence threshold | 0 | NO_ITEMS | Evidence threshold remains TBD where unsupported. |
| X:guiding:completeness | guiding | completeness | coverage signal map | 0 | NO_ITEMS | Coverage map is documented at setup level. |
| X:guiding:consistency | guiding | consistency | stable direction message | 0 | NO_ITEMS | No direction conflict detected. |
| X:applying:necessity | applying | necessity | action prerequisite check | 0 | NO_ITEMS | Prerequisites are present. |
| X:applying:sufficiency | applying | sufficiency | execution proof basis | 1 | HAS_ITEMS | Procedure lacks concrete validator command because schema does not exist yet. |
| X:applying:completeness | applying | completeness | workflow coverage record | 0 | NO_ITEMS | Procedure lists expected records. |
| X:applying:consistency | applying | consistency | repeatable enactment signal | 0 | NO_ITEMS | No workflow contradiction detected. |
| X:judging:necessity | judging | necessity | decision input test | 0 | NO_ITEMS | Decision inputs remain future criteria. |
| X:judging:sufficiency | judging | sufficiency | determination evidence basis | 0 | NO_ITEMS | No unsupported determination is asserted. |
| X:judging:completeness | judging | completeness | assessment coverage record | 0 | NO_ITEMS | Assessment coverage is future work. |
| X:judging:consistency | judging | consistency | coherent decision signal | 0 | NO_ITEMS | Decision language is consistent. |
| X:reviewing:necessity | reviewing | necessity | audit prerequisite basis | 0 | NO_ITEMS | Audit prerequisites are identified at setup level. |
| X:reviewing:sufficiency | reviewing | sufficiency | inspection evidence threshold | 0 | NO_ITEMS | Inspection evidence remains future work. |
| X:reviewing:completeness | reviewing | completeness | inspection coverage record | 0 | NO_ITEMS | No additional item beyond F-001. |
| X:reviewing:consistency | reviewing | consistency | stable audit message | 0 | NO_ITEMS | No audit-message conflict detected. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| X-001 | X:applying:sufficiency | VerificationGap | Procedure | Procedure | Add concrete validation command and expected result after schema artifact exists. | Procedure verification names checks but cannot provide command-level evidence or expected output until `schemas/model_operation.schema.json` is implemented. | `Procedure.md` | `## Verification` |  | PROPOSAL: later enrichment should cite validator command/output rather than inventing it now. | TBD |

## Matrix E - Evaluation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| E:guiding:data | guiding | data | directive evidence trace | 0 | NO_ITEMS | Directive evidence is cited. |
| E:guiding:information | guiding | information | instruction context frame | 0 | NO_ITEMS | Context is present. |
| E:guiding:knowledge | guiding | knowledge | practice insight basis | 0 | NO_ITEMS | Insight stays source-bounded. |
| E:guiding:wisdom | guiding | wisdom | principled direction sense | 0 | NO_ITEMS | No unsupported principle is introduced. |
| E:applying:data | applying | data | execution fact trace | 0 | NO_ITEMS | No implementation facts are claimed. |
| E:applying:information | applying | information | work context frame | 0 | NO_ITEMS | Work context is adequate for setup. |
| E:applying:knowledge | applying | knowledge | practical capability basis | 0 | NO_ITEMS | Practical capability remains future implementation. |
| E:applying:wisdom | applying | wisdom | situated action judgment | 0 | NO_ITEMS | No software judgment is overclaimed. |
| E:judging:data | judging | data | decision evidence trace | 0 | NO_ITEMS | Human-ruling areas are marked TBD. |
| E:judging:information | judging | information | assessment context frame | 0 | NO_ITEMS | Assessment context is conservative. |
| E:judging:knowledge | judging | knowledge | determination expertise basis | 0 | NO_ITEMS | No expert compliance claim is asserted. |
| E:judging:wisdom | judging | wisdom | balanced decision judgment | 0 | NO_ITEMS | Decision boundary remains human-governed. |
| E:reviewing:data | reviewing | data | audit evidence trace | 1 | HAS_ITEMS | Future fixture provenance review record is not yet present because fixtures do not exist. |
| E:reviewing:information | reviewing | information | inspection context frame | 0 | NO_ITEMS | Inspection context is listed in expected records. |
| E:reviewing:knowledge | reviewing | knowledge | appraisal capability basis | 0 | NO_ITEMS | Appraisal remains future review. |
| E:reviewing:wisdom | reviewing | wisdom | principled audit judgment | 0 | NO_ITEMS | No audit conclusion is overclaimed. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| E-001 | E:reviewing:data | MissingSlot | Datasheet | Datasheet | Add public-fixture provenance/protected-content review record when fixtures are created. | Production docs require protected-content/provenance review for fixtures, but no fixture artifact or review evidence exists in this setup pass. | `Datasheet.md`; `Procedure.md`; `Specification.md` | `## Construction`; `## Records`; `## Documentation` |  | PROPOSAL: later enrichment should add review evidence only after fixtures exist. | TBD |
