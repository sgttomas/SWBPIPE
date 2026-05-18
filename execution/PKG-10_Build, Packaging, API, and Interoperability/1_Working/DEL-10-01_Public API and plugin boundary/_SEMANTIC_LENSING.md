# Semantic Lensing Register: DEL-10-01 Public API and Plugin Boundary

**Generated:** 2026-04-30
**Deliverable Folder:** `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-01_Public API and plugin boundary`
**Warnings:** None for missing production documents or matrix parse errors. Remaining issues are explicit TBDs, not source conflicts.

**Inputs Read:**
- _CONTEXT.md - `_CONTEXT.md#context-del-10-01`
- _STATUS.md - `_STATUS.md#status-del-10-01-public-api-and-plugin-boundary`
- _SEMANTIC.md - `_SEMANTIC.md#deliverable-del-10-01-public-api-and-plugin-boundary`
- Datasheet.md - `Datasheet.md#datasheet-public-api-and-plugin-boundary`
- Specification.md - `Specification.md#specification-public-api-and-plugin-boundary`
- Guidance.md - `Guidance.md#guidance-public-api-and-plugin-boundary`
- Procedure.md - `Procedure.md#procedure-public-api-and-plugin-boundary`
- _REFERENCES.md - `_REFERENCES.md#references-del-10-01-public-api-and-plugin-boundary`
- _DEPENDENCIES.md - `_DEPENDENCIES.md#dependencies-del-10-01-public-api-and-plugin-boundary`

**Purpose:** Apply `semantic-matrix-build` matrix cells as lenses over the production documents, capturing warranted enrichment inputs for later review without rewriting documents or resolving human decisions.

## Summary

- Total warranted items: 14
- By document:
  - Datasheet: 2
  - Specification: 8
  - Guidance: 2
  - Procedure: 2
- By matrix:
  - A: 2  B: 2  C: 2  F: 2  D: 2  X: 2  E: 2
- By type:
  - Conflict: 0
  - VerificationGap: 4
  - MissingSlot: 3
  - WeakStatement: 1
  - RationaleGap: 1
  - Normalization: 1
  - TBD_Question: 4
  - MatrixError: 0
- Matrix parse errors: 0

## Matrix A - Orientation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| A:[normative]:[guiding] | normative | guiding | prescriptive direction | 1 | HAS_ITEMS | Human-ruling ownership remains salient. |
| A:[normative]:[applying] | normative | applying | mandatory practice | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| A:[normative]:[judging] | normative | judging | compliance determination | 1 | HAS_ITEMS | Professional-boundary state wording remains important. |
| A:[normative]:[reviewing] | normative | reviewing | regulatory audit | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| A:[operative]:[guiding] | operative | guiding | procedural direction | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| A:[operative]:[applying] | operative | applying | practical execution | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| A:[operative]:[judging] | operative | judging | performance assessment | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| A:[operative]:[reviewing] | operative | reviewing | process audit | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| A:[evaluative]:[guiding] | evaluative | guiding | value orientation | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| A:[evaluative]:[applying] | evaluative | applying | merit application | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| A:[evaluative]:[judging] | evaluative | judging | worth determination | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| A:[evaluative]:[reviewing] | evaluative | reviewing | quality appraisal | 0 | NO_ITEMS | No additional warranted item under tight threshold. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| A-001 | A:[normative]:[guiding] | TBD_Question | Guidance | Guidance | TBD: identify the human/project authority path for public API transport, plugin runtime, permission taxonomy, and external format rulings. | The documents intentionally preserve those choices as TBD, but later implementation needs an authority path. | Guidance.md; Specification.md | Human-Ruling Queue; Requirements REQ-13 | NA | PROPOSAL: keep as explicit human-ruling queue until project authority records decisions. | TBD |
| A-002 | A:[normative]:[judging] | Normalization | Specification | Specification | Keep status terms limited to project vocabulary and avoid future endpoint names that imply `CODE_COMPLIANT`. | `docs/TYPES.md` forbids automatic `CODE_COMPLIANT`; future API field names could drift if the contract does not keep this explicit. | Specification.md; docs/TYPES.md | Requirements REQ-10; TYPES section 4 | NA | PROPOSAL: enforce status vocabulary in later schema review. | TBD |

## Matrix B - Conceptualization

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| B:[data]:[necessity] | data | necessity | essential fact | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| B:[data]:[sufficiency] | data | sufficiency | adequate evidence | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| B:[data]:[completeness] | data | completeness | comprehensive record | 1 | HAS_ITEMS | Manifest concept inventory remains non-fielded. |
| B:[data]:[consistency] | data | consistency | reliable measurement | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| B:[information]:[necessity] | information | necessity | essential signal | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| B:[information]:[sufficiency] | information | sufficiency | adequate context | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| B:[information]:[completeness] | information | completeness | comprehensive account | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| B:[information]:[consistency] | information | consistency | coherent message | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| B:[knowledge]:[necessity] | knowledge | necessity | fundamental understanding | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| B:[knowledge]:[sufficiency] | knowledge | sufficiency | competent expertise | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| B:[knowledge]:[completeness] | knowledge | completeness | thorough mastery | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| B:[knowledge]:[consistency] | knowledge | consistency | coherent understanding | 1 | HAS_ITEMS | API artifact equivalence needs persistent trace. |
| B:[wisdom]:[necessity] | wisdom | necessity | essential discernment | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| B:[wisdom]:[sufficiency] | wisdom | sufficiency | adequate judgment | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| B:[wisdom]:[completeness] | wisdom | completeness | holistic insight | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| B:[wisdom]:[consistency] | wisdom | consistency | principled reasoning | 0 | NO_ITEMS | No additional warranted item under tight threshold. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| B-001 | B:[data]:[completeness] | MissingSlot | Specification | Specification | Add a future schema-field inventory when the repository-level API artifact is authorized. | Specification lists concept slots but intentionally leaves exact field names/layout TBD; downstream schema work needs a later field inventory. | Specification.md | Documentation, manifest concept inventory | NA | PROPOSAL: add field-level schema only after transport/schema layout ruling. | TBD |
| B-002 | B:[knowledge]:[consistency] | TBD_Question | Datasheet | Datasheet | TBD: record the eventual location of the equivalent API artifact when repository write scope allows it. | The anticipated artifact mentions `api/openapi.yaml or equivalent`, while current write scope only permits deliverable-local documents. | Datasheet.md; _CONTEXT.md | Identification; Anticipated Artifacts | NA | PROPOSAL: keep deliverable-local kit as setup equivalent and defer repo-level path. | TBD |

## Matrix C - Formulation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| C:[normative]:[necessity] | normative | necessity | binding obligation basis | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| C:[normative]:[sufficiency] | normative | sufficiency | warranted control basis | 1 | HAS_ITEMS | No-bypass equivalence needs later acceptance evidence. |
| C:[normative]:[completeness] | normative | completeness | comprehensive control frame | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| C:[normative]:[consistency] | normative | consistency | coherent governance frame | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| C:[operative]:[necessity] | operative | necessity | essential execution basis | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| C:[operative]:[sufficiency] | operative | sufficiency | adequate practice basis | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| C:[operative]:[completeness] | operative | completeness | whole workflow basis | 1 | HAS_ITEMS | Job/export workflow artifacts are concept-level only. |
| C:[operative]:[consistency] | operative | consistency | stable process basis | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| C:[evaluative]:[necessity] | evaluative | necessity | critical value basis | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| C:[evaluative]:[sufficiency] | evaluative | sufficiency | reasoned appraisal basis | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| C:[evaluative]:[completeness] | evaluative | completeness | integral appraisal frame | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| C:[evaluative]:[consistency] | evaluative | consistency | principled appraisal frame | 0 | NO_ITEMS | No additional warranted item under tight threshold. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| C-001 | C:[normative]:[sufficiency] | VerificationGap | Specification | Specification | Add future acceptance evidence criteria for proving a plugin boundary is no-bypass. | Requirements state no-bypass obligations, but concrete tests and proofs can only exist once implementation artifacts exist. | Specification.md | Requirements REQ-03, REQ-05, REQ-12; Verification | NA | PROPOSAL: later plugin/API implementation must provide schema, unit, provenance, diagnostics, privacy, sandbox, and report-boundary evidence. | TBD |
| C-002 | C:[operative]:[completeness] | MissingSlot | Procedure | Procedure | Add concrete records for solve-job/export-job progress and cancellation once job schema exists. | Procedure identifies job concepts but cannot name concrete records before schema layout and transport are approved. | Procedure.md | Steps 3-4; Records | NA | PROPOSAL: retain concept-level record list until schema work begins. | TBD |

## Matrix F - Requirements

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| F:[normative]:[necessity] | normative | necessity | binding evidence threshold | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| F:[normative]:[sufficiency] | normative | sufficiency | warranted assurance threshold | 1 | HAS_ITEMS | Rule-hook sandbox evidence remains future work. |
| F:[normative]:[completeness] | normative | completeness | full control coverage | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| F:[normative]:[consistency] | normative | consistency | coherent compliance basis | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| F:[operative]:[necessity] | operative | necessity | required action evidence | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| F:[operative]:[sufficiency] | operative | sufficiency | adequate execution proof | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| F:[operative]:[completeness] | operative | completeness | whole practice coverage | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| F:[operative]:[consistency] | operative | consistency | stable workflow assurance | 1 | HAS_ITEMS | External format list is intentionally unresolved. |
| F:[evaluative]:[necessity] | evaluative | necessity | essential appraisal evidence | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| F:[evaluative]:[sufficiency] | evaluative | sufficiency | reasoned merit assurance | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| F:[evaluative]:[completeness] | evaluative | completeness | integral appraisal coverage | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| F:[evaluative]:[consistency] | evaluative | consistency | principled quality assurance | 0 | NO_ITEMS | No additional warranted item under tight threshold. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| F-001 | F:[normative]:[sufficiency] | VerificationGap | Specification | Specification | Define future sandbox test evidence for rule-pack-facing API hooks. | Requirements prohibit arbitrary code execution, but exact expression grammar/library and sandbox mechanism remain TBD. | Specification.md; Guidance.md | Requirements REQ-08, REQ-12, REQ-13; Human-Ruling Queue | NA | PROPOSAL: require deterministic/unit/security tests once PKG-06/PKG-12 decisions exist. | TBD |
| F-002 | F:[operative]:[consistency] | TBD_Question | Guidance | Guidance | TBD: link future external format decisions to adapter validation gates. | External formats are intentionally not selected; each future format needs unit/provenance/redistribution/protected-content controls. | Guidance.md; docs/_Decomposition/SOFTWARE_DECOMP.md | Human-Ruling Queue; OI-004 | NA | PROPOSAL: route format choices through PKG-10 adapter deliverables and human decision. | TBD |

## Matrix D - Objectives

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| D:[normative]:[guiding] | normative | guiding | controlled decision charter | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| D:[normative]:[applying] | normative | applying | enforceable practice boundary | 1 | HAS_ITEMS | Permission policy needs future ruling. |
| D:[normative]:[judging] | normative | judging | adjudicated closure basis | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| D:[normative]:[reviewing] | normative | reviewing | audit closure record | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| D:[operative]:[guiding] | operative | guiding | executable direction charter | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| D:[operative]:[applying] | operative | applying | governed action protocol | 1 | HAS_ITEMS | Command/query/job examples are not schemas. |
| D:[operative]:[judging] | operative | judging | measured performance basis | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| D:[operative]:[reviewing] | operative | reviewing | process audit record | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| D:[evaluative]:[guiding] | evaluative | guiding | value-aligned direction frame | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| D:[evaluative]:[applying] | evaluative | applying | merit-grounded practice frame | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| D:[evaluative]:[judging] | evaluative | judging | defensible appraisal basis | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| D:[evaluative]:[reviewing] | evaluative | reviewing | quality appraisal record | 0 | NO_ITEMS | No additional warranted item under tight threshold. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| D-001 | D:[normative]:[applying] | TBD_Question | Specification | Specification | TBD: identify approval owner and review state for plugin capability grants. | The boundary is deny-by-default, but exact permission names, grant workflow, and owner are not defined. | Specification.md; Guidance.md | Requirements REQ-12, REQ-13; Human-Ruling Queue | NA | PROPOSAL: defer to security/privacy and architecture review. | TBD |
| D-002 | D:[operative]:[applying] | WeakStatement | Guidance | Specification | Clarify that invented examples in Guidance are conceptual examples, not approved envelope schemas. | Guidance lists acceptable invented boundary examples; downstream readers could mistake them for approved schema fields without a warning. | Guidance.md; Specification.md | Examples; Documentation | NA | PROPOSAL: retain examples as conceptual and require later schema approval. | TBD |

## Matrix X - Verification

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| X:[guiding]:[necessity] | guiding | necessity | chartered evidence gate | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| X:[guiding]:[sufficiency] | guiding | sufficiency | charter assurance gate | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| X:[guiding]:[completeness] | guiding | completeness | charter coverage gate | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| X:[guiding]:[consistency] | guiding | consistency | charter coherence gate | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| X:[applying]:[necessity] | applying | necessity | practice evidence gate | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| X:[applying]:[sufficiency] | applying | sufficiency | practice assurance gate | 1 | HAS_ITEMS | Protected-content import checks need future tests. |
| X:[applying]:[completeness] | applying | completeness | practice coverage gate | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| X:[applying]:[consistency] | applying | consistency | practice coherence gate | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| X:[judging]:[necessity] | judging | necessity | decision evidence gate | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| X:[judging]:[sufficiency] | judging | sufficiency | decision assurance gate | 1 | HAS_ITEMS | Transport/schema TBD gate needs review evidence. |
| X:[judging]:[completeness] | judging | completeness | decision coverage gate | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| X:[judging]:[consistency] | judging | consistency | decision coherence gate | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| X:[reviewing]:[necessity] | reviewing | necessity | audit evidence gate | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| X:[reviewing]:[sufficiency] | reviewing | sufficiency | audit assurance gate | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| X:[reviewing]:[completeness] | reviewing | completeness | audit coverage gate | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| X:[reviewing]:[consistency] | reviewing | consistency | audit coherence gate | 0 | NO_ITEMS | No additional warranted item under tight threshold. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| X-001 | X:[applying]:[sufficiency] | VerificationGap | Specification | Specification | Add future adapter/plugin tests for protected-content quarantine and provenance rejection. | Requirements state the checks, but test fixtures and gates are future implementation artifacts. | Specification.md; docs/IP_AND_DATA_BOUNDARY.md | Requirements REQ-05, REQ-06, REQ-07; Quarantine rule | NA | PROPOSAL: require invented fixtures and protected-content lint in adapter/plugin test work. | TBD |
| X-002 | X:[judging]:[sufficiency] | VerificationGap | Specification | Specification | Add a review gate requiring transport/schema decisions before endpoint-level API docs are treated as implementable. | Transport and endpoint syntax are TBD; without a gate, future work could imply approval prematurely. | Specification.md; Guidance.md | Requirements REQ-13; Human-Ruling Queue | NA | PROPOSAL: make endpoint-level docs dependent on human-approved transport/schema layout. | TBD |

## Matrix E - Evaluation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| E:[guiding]:[data] | guiding | data | chartered fact validation | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| E:[guiding]:[information] | guiding | information | chartered signal validation | 1 | HAS_ITEMS | Human-ruling queue needs durable issue linkage. |
| E:[guiding]:[knowledge] | guiding | knowledge | chartered expertise validation | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| E:[guiding]:[wisdom] | guiding | wisdom | chartered judgment validation | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| E:[applying]:[data] | applying | data | practice fact validation | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| E:[applying]:[information] | applying | information | practice signal validation | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| E:[applying]:[knowledge] | applying | knowledge | practice expertise validation | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| E:[applying]:[wisdom] | applying | wisdom | practice judgment validation | 1 | HAS_ITEMS | Privacy/telemetry posture needs explicit later gate. |
| E:[judging]:[data] | judging | data | decision fact validation | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| E:[judging]:[information] | judging | information | decision signal validation | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| E:[judging]:[knowledge] | judging | knowledge | decision expertise validation | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| E:[judging]:[wisdom] | judging | wisdom | decision judgment validation | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| E:[reviewing]:[data] | reviewing | data | audit fact validation | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| E:[reviewing]:[information] | reviewing | information | audit signal validation | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| E:[reviewing]:[knowledge] | reviewing | knowledge | audit expertise validation | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| E:[reviewing]:[wisdom] | reviewing | wisdom | audit judgment validation | 0 | NO_ITEMS | No additional warranted item under tight threshold. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| E-001 | E:[guiding]:[information] | MissingSlot | Guidance | Guidance | Add durable open-issue or decision-log references once project authority creates them. | The Human-Ruling Queue is clear but lacks issue IDs beyond decomposition open issues for some decisions. | Guidance.md; docs/_Decomposition/SOFTWARE_DECOMP.md | Human-Ruling Queue; Open issues | NA | PROPOSAL: link future decisions to OI/DEC IDs rather than embedding decisions here. | TBD |
| E-002 | E:[applying]:[wisdom] | RationaleGap | Specification | Specification | Add privacy/security rationale when any plugin capability could expose private project, rule-pack, material, component, or result data. | The deny-by-default rule is present, but later exceptions require rationale and explicit approval evidence. | Specification.md; docs/PRD.md | Requirements REQ-12; PRD sections 18.2 and 18.3 | NA | PROPOSAL: defer exception rationale to PKG-12/security review and human ruling. | TBD |

## Register Closure

Every cell in matrices A, B, C, F, D, X, and E from `_SEMANTIC.md` is represented above. Warranted items are candidate inputs only; they do not authorize edits, resolve TBDs, approve transport/plugin runtime choices, or establish engineering authority.
