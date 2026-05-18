# Semantic Lensing Register: DEL-12-01 Local-first storage and private data paths

**Generated:** 2026-04-30
**Deliverable Folder:** `execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-01_Local-first storage and private data paths`
**Warnings:** Expected setup-only TBDs remain for physical container, OS roots, executable tests, cloud exception process, and secret handling.

**Inputs Read:**
- `_CONTEXT.md` - deliverable identity, package context, scope coverage, objective support, and architecture basis injection.
- `_STATUS.md` - lifecycle state before setup refresh.
- `_SEMANTIC.md` - semantic matrices A, B, C, F, D, X, and E.
- `Datasheet.md` - recreated setup datasheet.
- `Specification.md` - recreated setup specification.
- `Guidance.md` - recreated setup guidance.
- `Procedure.md` - recreated setup procedure.
- `_REFERENCES.md` - governing references and register pointers.

**Purpose:** Apply `semantic-matrix-build` matrix cells as lenses over the production documents, capturing warranted enrichment inputs for the P3 pass and later implementation planning.

## Summary

- Total warranted items: 7
- By document:
  - Datasheet: 1
  - Specification: 2
  - Guidance: 2
  - Procedure: 1
  - Multi: 1
- By matrix:
  - A: 1  B: 1  C: 1  F: 1  D: 1  X: 1  E: 1
- By type:
  - Conflict: 1
  - VerificationGap: 1
  - MissingSlot: 2
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
| A:normative:guiding | normative | guiding | prescriptive direction | 1 | HAS_ITEMS | Container/root decisions remain bounded as guidance-only. |
| A:normative:applying | normative | applying | mandatory practice | 0 | NO_ITEMS | Requirements preserve local-first posture. |
| A:normative:judging | normative | judging | compliance determination | 0 | NO_ITEMS | No compliance or certification claim introduced. |
| A:normative:reviewing | normative | reviewing | regulatory audit | 0 | NO_ITEMS | Audit language remains project-governance only. |
| A:operative:guiding | operative | guiding | procedural direction | 0 | NO_ITEMS | Procedure records setup-only steps. |
| A:operative:applying | operative | applying | practical execution | 0 | NO_ITEMS | No storage implementation step added. |
| A:operative:judging | operative | judging | performance assessment | 0 | NO_ITEMS | Future tests are deferred. |
| A:operative:reviewing | operative | reviewing | process audit | 0 | NO_ITEMS | Run records and dependency register provide evidence. |
| A:evaluative:guiding | evaluative | guiding | value orientation | 0 | NO_ITEMS | Values align with local-first and private boundary. |
| A:evaluative:applying | evaluative | applying | merit application | 0 | NO_ITEMS | No extra item. |
| A:evaluative:judging | evaluative | judging | worth determination | 0 | NO_ITEMS | No extra item. |
| A:evaluative:reviewing | evaluative | reviewing | quality appraisal | 0 | NO_ITEMS | No extra item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| A-001 | A:normative:guiding | TBD_Question | Guidance | Guidance | Preserve `TBD`: physical project package/container and OS-specific private roots. | The architecture basis requires deterministic persistence but leaves physical container and root choices open. | `Guidance.md`; `docs/_Decomposition/SOFTWARE_DECOMP.md` | `Guidance.md#Open Issues and TBDs`; `SOFTWARE_DECOMP.md#8.2` | NA | PROPOSAL: keep symbolic classes only until human-approved implementation decision. | TBD |

## Matrix B - Conceptualization

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| B:data:necessity | data | necessity | essential fact | 1 | HAS_ITEMS | Private data classes require explicit path treatment. |
| B:data:sufficiency | data | sufficiency | adequate evidence | 0 | NO_ITEMS | Evidence references are recorded. |
| B:data:completeness | data | completeness | comprehensive record | 0 | NO_ITEMS | Datasheet records symbolic classes. |
| B:data:consistency | data | consistency | reliable measurement | 0 | NO_ITEMS | No numeric values introduced. |
| B:information:necessity | information | necessity | essential signal | 0 | NO_ITEMS | No extra item. |
| B:information:sufficiency | information | sufficiency | adequate context | 0 | NO_ITEMS | Context supplied. |
| B:information:completeness | information | completeness | comprehensive account | 0 | NO_ITEMS | No extra item. |
| B:information:consistency | information | consistency | coherent message | 0 | NO_ITEMS | Terms are consistent. |
| B:knowledge:necessity | knowledge | necessity | fundamental understanding | 0 | NO_ITEMS | No extra item. |
| B:knowledge:sufficiency | knowledge | sufficiency | competent expertise | 0 | NO_ITEMS | No extra item. |
| B:knowledge:completeness | knowledge | completeness | thorough mastery | 0 | NO_ITEMS | No extra item. |
| B:knowledge:consistency | knowledge | consistency | coherent understanding | 0 | NO_ITEMS | No extra item. |
| B:wisdom:necessity | wisdom | necessity | essential discernment | 0 | NO_ITEMS | No extra item. |
| B:wisdom:sufficiency | wisdom | sufficiency | adequate judgment | 0 | NO_ITEMS | No extra item. |
| B:wisdom:completeness | wisdom | completeness | holistic insight | 0 | NO_ITEMS | No extra item. |
| B:wisdom:consistency | wisdom | consistency | principled reasoning | 0 | NO_ITEMS | No extra item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| B-001 | B:data:necessity | MissingSlot | Datasheet | Datasheet | Add symbolic classes for private roots rather than real filesystem paths. | The deliverable must protect private data paths, but real paths are not authorized in setup. | `Datasheet.md`; `_CONTEXT.md` | `Datasheet.md#Symbolic Path Classes`; `_CONTEXT.md#Architecture Basis Injection` | NA | PROPOSAL: use symbolic path classes and leave OS roots TBD. | TBD |

## Matrix C - Formulation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| C:normative:necessity | normative | necessity | Binding Contract Basis | 1 | HAS_ITEMS | Local-first requirement and no-cloud boundary need explicit preservation. |
| C:normative:sufficiency | normative | sufficiency | Defensible Evidence Basis | 0 | NO_ITEMS | Sources cited. |
| C:normative:completeness | normative | completeness | Comprehensive Contract Record | 0 | NO_ITEMS | No extra item. |
| C:normative:consistency | normative | consistency | Controlled Conformance Logic | 0 | NO_ITEMS | No compliance claim introduced. |
| C:operative:necessity | operative | necessity | Executable Workflow Basis | 0 | NO_ITEMS | Procedure exists. |
| C:operative:sufficiency | operative | sufficiency | Usable Evidence Context | 0 | NO_ITEMS | No extra item. |
| C:operative:completeness | operative | completeness | Complete Process Account | 0 | NO_ITEMS | No extra item. |
| C:operative:consistency | operative | consistency | Stable Execution Meaning | 0 | NO_ITEMS | No extra item. |
| C:evaluative:necessity | evaluative | necessity | Value Discernment Basis | 0 | NO_ITEMS | No extra item. |
| C:evaluative:sufficiency | evaluative | sufficiency | Adequate Appraisal Ground | 0 | NO_ITEMS | No extra item. |
| C:evaluative:completeness | evaluative | completeness | Holistic Quality Account | 0 | NO_ITEMS | No extra item. |
| C:evaluative:consistency | evaluative | consistency | Principled Merit Logic | 0 | NO_ITEMS | No extra item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| C-001 | C:normative:necessity | RationaleGap | Guidance | Guidance | Explain why cloud storage remains excluded unless separately approved. | The scope note says cloud is out of MVP, but future readers need the rationale tied to private-data protection. | `Guidance.md`; `docs/_Registers/ScopeLedger.csv` | `Guidance.md#Principles`; `ScopeLedger.csv` row SOW-029 | NA | PROPOSAL: keep a local-first rationale and defer approval mechanics. | TBD |

## Matrix F - Requirements

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| F:normative:necessity | normative | necessity | Required Contract Foundation | 0 | NO_ITEMS | Requirements present. |
| F:normative:sufficiency | normative | sufficiency | Evidence Closure Standard | 0 | NO_ITEMS | Verification rows present. |
| F:normative:completeness | normative | completeness | Complete Obligation Record | 0 | NO_ITEMS | No extra item. |
| F:normative:consistency | normative | consistency | Stable Rule Coherence | 0 | NO_ITEMS | No extra item. |
| F:operative:necessity | operative | necessity | Required Service Capability | 0 | NO_ITEMS | No implementation required. |
| F:operative:sufficiency | operative | sufficiency | Adequate Workflow Proof | 1 | HAS_ITEMS | Test expectations exist but executable tests are absent by design. |
| F:operative:completeness | operative | completeness | Complete State Preservation | 0 | NO_ITEMS | No extra item. |
| F:operative:consistency | operative | consistency | Deterministic Process Behavior | 0 | NO_ITEMS | Deterministic persistence referenced. |
| F:evaluative:necessity | evaluative | necessity | Required Assurance Basis | 0 | NO_ITEMS | No extra item. |
| F:evaluative:sufficiency | evaluative | sufficiency | Defensible Appraisal Ground | 0 | NO_ITEMS | No extra item. |
| F:evaluative:completeness | evaluative | completeness | Complete Quality Evidence | 0 | NO_ITEMS | No extra item. |
| F:evaluative:consistency | evaluative | consistency | Coherent Assurance Logic | 0 | NO_ITEMS | No extra item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| F-001 | F:operative:sufficiency | VerificationGap | Specification | Specification | Record that executable storage tests are future implementation artifacts, not setup outputs. | The register anticipates tests, but this run is setup/document production only. | `Specification.md`; `_CONTEXT.md` | `Specification.md#Verification`; `_CONTEXT.md#Anticipated Artifacts` | NA | PROPOSAL: keep test expectations and defer test files. | TBD |

## Matrix D - Objectives

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| D:normative:guiding | normative | guiding | Contract Direction Closure | 0 | NO_ITEMS | No extra item. |
| D:normative:applying | normative | applying | Mandatory Evidence Practice | 0 | NO_ITEMS | No extra item. |
| D:normative:judging | normative | judging | Conformance Decision Closure | 0 | NO_ITEMS | No extra item. |
| D:normative:reviewing | normative | reviewing | Audit Trail Closure | 0 | NO_ITEMS | No extra item. |
| D:operative:guiding | operative | guiding | Workflow Direction Closure | 0 | NO_ITEMS | No extra item. |
| D:operative:applying | operative | applying | Service Execution Closure | 1 | HAS_ITEMS | Setup must not drift into implementation. |
| D:operative:judging | operative | judging | Behavior Assessment Closure | 0 | NO_ITEMS | No extra item. |
| D:operative:reviewing | operative | reviewing | Process Trace Closure | 0 | NO_ITEMS | No extra item. |
| D:evaluative:guiding | evaluative | guiding | Value Direction Closure | 0 | NO_ITEMS | No extra item. |
| D:evaluative:applying | evaluative | applying | Merit Practice Closure | 0 | NO_ITEMS | No extra item. |
| D:evaluative:judging | evaluative | judging | Quality Decision Closure | 0 | NO_ITEMS | No extra item. |
| D:evaluative:reviewing | evaluative | reviewing | Appraisal Trace Closure | 0 | NO_ITEMS | No extra item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| D-001 | D:operative:applying | MissingSlot | Procedure | Procedure | Add explicit setup-only step boundaries: no source, schema, storage, test, path, or secret creation. | The human brief constrains this run to setup/document production only. | `Procedure.md`; user sealed brief | `Procedure.md#Steps`; `Procedure.md#Future Implementation Procedure` | NA | PROPOSAL: make non-implementation checks part of setup verification. | TBD |

## Matrix X - Verification

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| X:guiding:necessity | guiding | necessity | Directive Evidence Basis | 0 | NO_ITEMS | No extra item. |
| X:guiding:sufficiency | guiding | sufficiency | Contextual Direction Proof | 0 | NO_ITEMS | No extra item. |
| X:guiding:completeness | guiding | completeness | Comprehensive Direction Record | 0 | NO_ITEMS | No extra item. |
| X:guiding:consistency | guiding | consistency | Stable Direction Meaning | 0 | NO_ITEMS | No extra item. |
| X:applying:necessity | applying | necessity | Practice Readiness Basis | 0 | NO_ITEMS | No extra item. |
| X:applying:sufficiency | applying | sufficiency | Executable Evidence Proof | 0 | NO_ITEMS | No extra item. |
| X:applying:completeness | applying | completeness | Complete Practice Record | 1 | HAS_ITEMS | Run records should show every required setup step. |
| X:applying:consistency | applying | consistency | Stable Practice Meaning | 0 | NO_ITEMS | No extra item. |
| X:judging:necessity | judging | necessity | Decision Evidence Basis | 0 | NO_ITEMS | No extra item. |
| X:judging:sufficiency | judging | sufficiency | Assessment Proof Ground | 0 | NO_ITEMS | No extra item. |
| X:judging:completeness | judging | completeness | Complete Decision Record | 0 | NO_ITEMS | No extra item. |
| X:judging:consistency | judging | consistency | Coherent Decision Rationale | 0 | NO_ITEMS | No extra item. |
| X:reviewing:necessity | reviewing | necessity | Trace Evidence Basis | 0 | NO_ITEMS | No extra item. |
| X:reviewing:sufficiency | reviewing | sufficiency | Audit Proof Ground | 0 | NO_ITEMS | No extra item. |
| X:reviewing:completeness | reviewing | completeness | Complete Trace Record | 0 | NO_ITEMS | No extra item. |
| X:reviewing:consistency | reviewing | consistency | Coherent Audit Rationale | 0 | NO_ITEMS | No extra item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| X-001 | X:applying:completeness | TBD_Question | Multi | Multi | Confirm setup evidence remains sufficient without executable storage tests. | This run intentionally does not create tests, but the deliverable catalog anticipates tests for the eventual security control. | `Procedure.md`; `docs/_Registers/Deliverables.csv` | `Procedure.md#Verification`; `Deliverables.csv` row DEL-12-01 | NA | PROPOSAL: accept setup readiness and leave implementation tests as future work. | TBD |

## Matrix E - Evaluation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| E:guiding:data | guiding | data | Directive Fact Trace | 0 | NO_ITEMS | No extra item. |
| E:guiding:information | guiding | information | Directive Context Signal | 0 | NO_ITEMS | No extra item. |
| E:guiding:knowledge | guiding | knowledge | Directive Expertise Frame | 0 | NO_ITEMS | No extra item. |
| E:guiding:wisdom | guiding | wisdom | Directive Discernment Frame | 0 | NO_ITEMS | No extra item. |
| E:applying:data | applying | data | Practice Fact Trace | 0 | NO_ITEMS | No extra item. |
| E:applying:information | applying | information | Execution Context Signal | 0 | NO_ITEMS | No extra item. |
| E:applying:knowledge | applying | knowledge | Service Expertise Frame | 0 | NO_ITEMS | No extra item. |
| E:applying:wisdom | applying | wisdom | Practice Discernment Frame | 0 | NO_ITEMS | No extra item. |
| E:judging:data | judging | data | Decision Fact Trace | 0 | NO_ITEMS | No extra item. |
| E:judging:information | judging | information | Assessment Context Signal | 0 | NO_ITEMS | No extra item. |
| E:judging:knowledge | judging | knowledge | Decision Expertise Frame | 0 | NO_ITEMS | No extra item. |
| E:judging:wisdom | judging | wisdom | Assessment Discernment Frame | 0 | NO_ITEMS | No extra item. |
| E:reviewing:data | reviewing | data | Audit Fact Trace | 1 | HAS_ITEMS | Conflict table should retain human-ruling status. |
| E:reviewing:information | reviewing | information | Audit Context Signal | 0 | NO_ITEMS | No extra item. |
| E:reviewing:knowledge | reviewing | knowledge | Audit Expertise Frame | 0 | NO_ITEMS | No extra item. |
| E:reviewing:wisdom | reviewing | wisdom | Audit Discernment Frame | 0 | NO_ITEMS | No extra item. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| E-001 | E:reviewing:data | Conflict | Guidance | Guidance | Keep conflict table entry for title/implementation implication vs. physical-container TBD. | The deliverable title and catalog artifact list can imply implementation, while the sealed brief and architecture basis keep this run setup-only and the container TBD. | `Guidance.md`; `_CONTEXT.md`; `docs/_Decomposition/SOFTWARE_DECOMP.md` | `Guidance.md#Conflict Table`; `_CONTEXT.md#Architecture Basis Injection`; `SOFTWARE_DECOMP.md#8.2` | `Guidance.md#Conflict Table`; `_CONTEXT.md#Architecture Basis Injection`; `docs/_Decomposition/SOFTWARE_DECOMP.md#8.2` | PROPOSAL: human should decide the container/root in a later implementation or architecture decision. | TBD |
