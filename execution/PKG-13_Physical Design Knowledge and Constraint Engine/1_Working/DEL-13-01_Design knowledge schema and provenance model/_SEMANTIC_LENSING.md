# Semantic Lensing Register: DEL-13-01 Design knowledge schema and provenance model

**Generated:** 2026-05-04
**Deliverable Folder:** `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/DEL-13-01_Design knowledge schema and provenance model`
**Warnings:** None

**Inputs Read:**
- `_CONTEXT.md` - deliverable identity and scope
- `_STATUS.md` - current lifecycle state
- `_SEMANTIC.md` - semantic lens source
- `Datasheet.md` - production document
- `Specification.md` - production document
- `Guidance.md` - production document
- `Procedure.md` - production document
- `_REFERENCES.md` - reference index

**Purpose:** Apply `semantic-matrix-build` matrix cells as lenses over the production documents, capturing warranted enrichment inputs for a later enrichment pass.

## Summary

- Total warranted items: 9
- By document:
  - Datasheet: 2
  - Specification: 3
  - Guidance: 2
  - Procedure: 2
- By matrix:
  - A: 1  B: 1  C: 1  F: 1  D: 1  X: 2  E: 2
- By type:
  - Conflict: 0
  - VerificationGap: 3
  - MissingSlot: 2
  - WeakStatement: 0
  - RationaleGap: 1
  - Normalization: 0
  - TBD_Question: 3
  - MatrixError: 0
- Notable conflicts: 0
- Matrix parse errors: 0

## Matrix A - Orientation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| A:normative:guiding | normative | guiding | prescriptive direction | 0 | NO_ITEMS | No additional warranted item. |
| A:normative:applying | normative | applying | mandatory practice | 1 | HAS_ITEMS | Schema artifact readiness needs later acceptance detail. |
| A:normative:judging | normative | judging | compliance determination | 0 | NO_ITEMS | No compliance-status conflict detected. |
| A:normative:reviewing | normative | reviewing | regulatory audit | 0 | NO_ITEMS | Public/private review is covered elsewhere. |
| A:operative:guiding | operative | guiding | procedural direction | 0 | NO_ITEMS | Procedure has setup-stage steps. |
| A:operative:applying | operative | applying | practical execution | 0 | NO_ITEMS | Implementation is explicitly TBD. |
| A:operative:judging | operative | judging | performance assessment | 0 | NO_ITEMS | No performance criteria are sourced yet. |
| A:operative:reviewing | operative | reviewing | process audit | 0 | NO_ITEMS | No process-audit conflict detected. |
| A:evaluative:guiding | evaluative | guiding | value orientation | 0 | NO_ITEMS | Boundary principles are stated. |
| A:evaluative:applying | evaluative | applying | merit application | 0 | NO_ITEMS | No additional warranted item. |
| A:evaluative:judging | evaluative | judging | worth determination | 0 | NO_ITEMS | No additional warranted item. |
| A:evaluative:reviewing | evaluative | reviewing | quality appraisal | 0 | NO_ITEMS | No additional warranted item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| A-001 | A:normative:applying | MissingSlot | Specification | Specification | TBD: record schema artifact readiness and acceptance criteria for `schemas/design_knowledge.schema.json`. | The specification names the expected artifact and verification surfaces, but the implemented schema artifact does not exist in this setup pass. Later enrichment should state readiness criteria once implementation begins. | `Specification.md` | `## Documentation`; `## Verification` | N/A | PROPOSAL: treat as implementation-readiness acceptance detail, not setup evidence. | TBD |

## Matrix B - Conceptualization

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| B:data:necessity | data | necessity | essential fact | 0 | NO_ITEMS | Identity and scope facts are present. |
| B:data:sufficiency | data | sufficiency | adequate evidence | 0 | NO_ITEMS | Source references are present. |
| B:data:completeness | data | completeness | comprehensive record | 1 | HAS_ITEMS | Exact field taxonomy remains TBD. |
| B:data:consistency | data | consistency | reliable measurement | 0 | NO_ITEMS | Unit posture is stated. |
| B:information:necessity | information | necessity | essential signal | 0 | NO_ITEMS | No additional warranted item. |
| B:information:sufficiency | information | sufficiency | adequate context | 0 | NO_ITEMS | Context envelope is present. |
| B:information:completeness | information | completeness | comprehensive account | 0 | NO_ITEMS | No additional warranted item. |
| B:information:consistency | information | consistency | coherent message | 0 | NO_ITEMS | Terminology is consistent enough for setup. |
| B:knowledge:necessity | knowledge | necessity | fundamental understanding | 0 | NO_ITEMS | Boundary concepts are stated. |
| B:knowledge:sufficiency | knowledge | sufficiency | competent expertise | 0 | NO_ITEMS | No additional warranted item. |
| B:knowledge:completeness | knowledge | completeness | thorough mastery | 0 | NO_ITEMS | No additional warranted item. |
| B:knowledge:consistency | knowledge | consistency | coherent understanding | 0 | NO_ITEMS | No additional warranted item. |
| B:wisdom:necessity | wisdom | necessity | essential discernment | 0 | NO_ITEMS | Boundary cautions are present. |
| B:wisdom:sufficiency | wisdom | sufficiency | adequate judgment | 0 | NO_ITEMS | No additional warranted item. |
| B:wisdom:completeness | wisdom | completeness | holistic insight | 0 | NO_ITEMS | No additional warranted item. |
| B:wisdom:consistency | wisdom | consistency | principled reasoning | 0 | NO_ITEMS | No additional warranted item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| B-001 | B:data:completeness | MissingSlot | Datasheet | Datasheet | TBD: enumerate exact design-knowledge record fields and enum families. | The documents list source-grounded categories, but exact object names, field names, and enum values are intentionally unresolved. | `Datasheet.md`; `Guidance.md` | `## Construction`; `## Considerations` | N/A | PROPOSAL: leave field taxonomy unresolved until implementation sources or human architecture detail exist. | TBD |

## Matrix C - Formulation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| C:normative:necessity | normative | necessity | Binding evidence threshold | 0 | NO_ITEMS | Requirements are source-linked. |
| C:normative:sufficiency | normative | sufficiency | Adequacy control basis | 0 | NO_ITEMS | Adequacy is bounded by accessible sources. |
| C:normative:completeness | normative | completeness | Complete compliance basis | 1 | HAS_ITEMS | Required/optional field policy remains TBD. |
| C:normative:consistency | normative | consistency | Coherent compliance basis | 0 | NO_ITEMS | No conflict detected. |
| C:operative:necessity | operative | necessity | Execution input basis | 0 | NO_ITEMS | Prerequisites are listed. |
| C:operative:sufficiency | operative | sufficiency | Workable context basis | 0 | NO_ITEMS | Setup context is adequate. |
| C:operative:completeness | operative | completeness | Complete process account | 0 | NO_ITEMS | Procedure records future checks as TBD. |
| C:operative:consistency | operative | consistency | Repeatable process message | 0 | NO_ITEMS | Procedure and specification align. |
| C:evaluative:necessity | evaluative | necessity | Judgment evidence basis | 0 | NO_ITEMS | No additional warranted item. |
| C:evaluative:sufficiency | evaluative | sufficiency | Contextual merit basis | 0 | NO_ITEMS | No additional warranted item. |
| C:evaluative:completeness | evaluative | completeness | Holistic appraisal basis | 0 | NO_ITEMS | No additional warranted item. |
| C:evaluative:consistency | evaluative | consistency | Coherent quality rationale | 0 | NO_ITEMS | No additional warranted item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| C-001 | C:normative:completeness | TBD_Question | Specification | Specification | TBD: decide required vs optional field policy for each sourced category. | SOW-067 names supported knowledge categories, but accessible sources do not specify required/optional field semantics for each category. | `Specification.md` | `## Requirements`; `REQ-13-01-001` | N/A | PROPOSAL: resolve during schema implementation or a human architecture ruling. | TBD |

## Matrix F - Requirements

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| F:normative:necessity | normative | necessity | Required evidence boundary | 1 | HAS_ITEMS | Executable schema validation criteria remain future work. |
| F:normative:sufficiency | normative | sufficiency | Sufficient proof package | 0 | NO_ITEMS | Evidence sources are listed. |
| F:normative:completeness | normative | completeness | Complete obligation record | 0 | NO_ITEMS | No additional warranted item. |
| F:normative:consistency | normative | consistency | Consistent control rationale | 0 | NO_ITEMS | No additional warranted item. |
| F:operative:necessity | operative | necessity | Required input pathway | 0 | NO_ITEMS | Procedure lists prerequisites. |
| F:operative:sufficiency | operative | sufficiency | Sufficient execution context | 0 | NO_ITEMS | No additional warranted item. |
| F:operative:completeness | operative | completeness | Complete workflow record | 0 | NO_ITEMS | No additional warranted item. |
| F:operative:consistency | operative | consistency | Consistent process rationale | 0 | NO_ITEMS | No additional warranted item. |
| F:evaluative:necessity | evaluative | necessity | Required review basis | 0 | NO_ITEMS | Boundary review is stated. |
| F:evaluative:sufficiency | evaluative | sufficiency | Sufficient judgment context | 0 | NO_ITEMS | No additional warranted item. |
| F:evaluative:completeness | evaluative | completeness | Complete assessment record | 0 | NO_ITEMS | No additional warranted item. |
| F:evaluative:consistency | evaluative | consistency | Consistent appraisal rationale | 0 | NO_ITEMS | No additional warranted item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| F-001 | F:normative:necessity | VerificationGap | Specification | Specification | Add concrete JSON Schema validation acceptance criteria once the schema artifact exists. | The setup specification maps verification approaches, but there is no executable schema artifact or concrete acceptance fixture yet. | `Specification.md` | `## Verification`; `## Documentation` | N/A | PROPOSAL: defer executable criteria until implementation evidence exists. | TBD |

## Matrix D - Objectives

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| D:normative:guiding | normative | guiding | Controlled obligation direction | 0 | NO_ITEMS | No additional warranted item. |
| D:normative:applying | normative | applying | Enforced practice basis | 0 | NO_ITEMS | No additional warranted item. |
| D:normative:judging | normative | judging | Determinate compliance basis | 0 | NO_ITEMS | No additional warranted item. |
| D:normative:reviewing | normative | reviewing | Auditable control record | 0 | NO_ITEMS | Boundary review is represented. |
| D:operative:guiding | operative | guiding | Actionable workflow direction | 0 | NO_ITEMS | Procedure is present. |
| D:operative:applying | operative | applying | Validated execution basis | 1 | HAS_ITEMS | Geometry policy remains unresolved. |
| D:operative:judging | operative | judging | Measured performance basis | 0 | NO_ITEMS | No additional warranted item. |
| D:operative:reviewing | operative | reviewing | Auditable process record | 0 | NO_ITEMS | No additional warranted item. |
| D:evaluative:guiding | evaluative | guiding | Principled value direction | 0 | NO_ITEMS | No additional warranted item. |
| D:evaluative:applying | evaluative | applying | Reasoned merit basis | 0 | NO_ITEMS | No additional warranted item. |
| D:evaluative:judging | evaluative | judging | Determinate worth basis | 0 | NO_ITEMS | No additional warranted item. |
| D:evaluative:reviewing | evaluative | reviewing | Auditable quality record | 0 | NO_ITEMS | No additional warranted item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| D-001 | D:operative:applying | TBD_Question | Guidance | Specification | TBD: choose geometry representation and coordinate-frame policy for corridors, zones, no-go volumes, and equipment interfaces. | The guidance identifies geometry/coordinate policy as unresolved, and later schema fields cannot be precise without that decision. | `Guidance.md` | `## Considerations`; rows for no-go/supportable zones and equipment interfaces | N/A | PROPOSAL: resolve before freezing field-level schema. | TBD |

## Matrix X - Verification

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| X:guiding:necessity | guiding | necessity | Required direction evidence | 0 | NO_ITEMS | No additional warranted item. |
| X:guiding:sufficiency | guiding | sufficiency | Adequate direction context | 0 | NO_ITEMS | No additional warranted item. |
| X:guiding:completeness | guiding | completeness | Complete direction record | 0 | NO_ITEMS | No additional warranted item. |
| X:guiding:consistency | guiding | consistency | Coherent direction rationale | 0 | NO_ITEMS | No additional warranted item. |
| X:applying:necessity | applying | necessity | Required practice evidence | 1 | HAS_ITEMS | Validation command/check remains future work. |
| X:applying:sufficiency | applying | sufficiency | Adequate execution context | 0 | NO_ITEMS | No additional warranted item. |
| X:applying:completeness | applying | completeness | Complete practice record | 0 | NO_ITEMS | No additional warranted item. |
| X:applying:consistency | applying | consistency | Coherent execution rationale | 0 | NO_ITEMS | No additional warranted item. |
| X:judging:necessity | judging | necessity | Required decision evidence | 0 | NO_ITEMS | No additional warranted item. |
| X:judging:sufficiency | judging | sufficiency | Adequate assessment context | 0 | NO_ITEMS | No additional warranted item. |
| X:judging:completeness | judging | completeness | Complete decision record | 0 | NO_ITEMS | No additional warranted item. |
| X:judging:consistency | judging | consistency | Coherent assessment rationale | 0 | NO_ITEMS | No additional warranted item. |
| X:reviewing:necessity | reviewing | necessity | Required audit evidence | 0 | NO_ITEMS | No additional warranted item. |
| X:reviewing:sufficiency | reviewing | sufficiency | Adequate inspection context | 0 | NO_ITEMS | No additional warranted item. |
| X:reviewing:completeness | reviewing | completeness | Complete audit record | 0 | NO_ITEMS | No additional warranted item. |
| X:reviewing:consistency | reviewing | consistency | Coherent audit rationale | 1 | HAS_ITEMS | Review-gate evidence remains future work. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| X-001 | X:applying:necessity | VerificationGap | Procedure | Procedure | Add validation command/check for JSON Schema syntax once `schemas/design_knowledge.schema.json` exists. | Procedure step 8 names schema validation but has no concrete command because the schema artifact does not exist yet. | `Procedure.md` | `## Steps`; step 8 | N/A | PROPOSAL: add exact command during implementation. | TBD |
| X-002 | X:reviewing:consistency | VerificationGap | Procedure | Procedure | Add protected-content/private-data review evidence requirements when fixtures or examples exist. | Boundary review is required, but setup docs cannot name fixture-level evidence because no examples or implementation artifacts exist. | `Procedure.md`; `Specification.md` | `## Verification`; `## Verification` | N/A | PROPOSAL: bind review evidence to future public examples/fixtures. | TBD |

## Matrix E - Evaluation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| E:guiding:data | guiding | data | Source-grounded orientation | 1 | HAS_ITEMS | Schema identity/version policy remains TBD. |
| E:guiding:information | guiding | information | Contextual orientation brief | 0 | NO_ITEMS | No additional warranted item. |
| E:guiding:knowledge | guiding | knowledge | Competent orientation model | 0 | NO_ITEMS | No additional warranted item. |
| E:guiding:wisdom | guiding | wisdom | Principled orientation rationale | 0 | NO_ITEMS | No additional warranted item. |
| E:applying:data | applying | data | Source-grounded action basis | 0 | NO_ITEMS | No additional warranted item. |
| E:applying:information | applying | information | Contextual execution brief | 0 | NO_ITEMS | No additional warranted item. |
| E:applying:knowledge | applying | knowledge | Competent execution model | 0 | NO_ITEMS | No additional warranted item. |
| E:applying:wisdom | applying | wisdom | Principled execution rationale | 0 | NO_ITEMS | No additional warranted item. |
| E:judging:data | judging | data | Source-grounded decision basis | 0 | NO_ITEMS | No additional warranted item. |
| E:judging:information | judging | information | Contextual assessment brief | 0 | NO_ITEMS | No additional warranted item. |
| E:judging:knowledge | judging | knowledge | Competent assessment model | 0 | NO_ITEMS | No additional warranted item. |
| E:judging:wisdom | judging | wisdom | Principled decision rationale | 0 | NO_ITEMS | No additional warranted item. |
| E:reviewing:data | reviewing | data | Source-grounded audit basis | 0 | NO_ITEMS | No additional warranted item. |
| E:reviewing:information | reviewing | information | Contextual audit brief | 0 | NO_ITEMS | No additional warranted item. |
| E:reviewing:knowledge | reviewing | knowledge | Competent audit model | 0 | NO_ITEMS | No additional warranted item. |
| E:reviewing:wisdom | reviewing | wisdom | Principled audit rationale | 1 | HAS_ITEMS | Boundary review ownership needs future routing. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| E-001 | E:guiding:data | TBD_Question | Datasheet | Datasheet | TBD: assign schema `$id`, `$schema`, versioning, and namespace policy. | Datasheet records JSON Schema 2020-12 as a baseline, but exact schema identifiers and versioning fields are not available in accessible sources. | `Datasheet.md` | `## Construction` | N/A | PROPOSAL: resolve in schema implementation, preserving JSON Schema 2020-12 unless changed by human decision. | TBD |
| E-002 | E:reviewing:wisdom | RationaleGap | Guidance | Guidance | Clarify review ownership for design-knowledge taxonomy and public-example boundary. | Guidance states that examples and taxonomy details are TBD and must avoid protected/private content, but it does not identify the later review route. | `Guidance.md`; `Procedure.md` | `## Examples`; `## Steps` step 7 | N/A | PROPOSAL: route later public-example/taxonomy review through PKG-13 plus IP/security review. | TBD |
