# Semantic Lensing Register: DEL-02-04 Plugin and Extension Domain Contracts

**Generated:** 2026-04-30
**Deliverable Folder:** `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-04_Plugin and extension domain contracts`
**Warnings:** None for missing production documents or matrix parse errors. Local consistency warnings are recorded as warranted items.

**Inputs Read:**
- _CONTEXT.md - `_CONTEXT.md#context-del-02-04`
- _STATUS.md - `_STATUS.md#status-del-02-04-plugin-and-extension-domain-contracts`
- _SEMANTIC.md - `_SEMANTIC.md#deliverable-del-02-04-plugin-and-extension-domain-contracts`
- Datasheet.md - `Datasheet.md#datasheet-plugin-and-extension-domain-contracts`
- Specification.md - `Specification.md#specification-plugin-and-extension-domain-contracts`
- Guidance.md - `Guidance.md#guidance-plugin-and-extension-domain-contracts`
- Procedure.md - `Procedure.md#procedure-plugin-and-extension-domain-contracts`
- _REFERENCES.md - `_REFERENCES.md#references-del-02-04-plugin-and-extension-domain-contracts`
- _DEPENDENCIES.md - `_DEPENDENCIES.md#dependencies-del-02-04-plugin-and-extension-domain-contracts`

**Purpose:** Apply `semantic-matrix-build` matrix cells as lenses over the production documents, capturing warranted enrichment inputs for a later enrichment pass.

## Summary

- Total warranted items: 15
- By document:
  - Datasheet: 4
  - Specification: 8
  - Guidance: 1
  - Procedure: 2
- By matrix:
  - A: 2  B: 2  C: 2  F: 2  D: 3  X: 2  E: 2
- By type:
  - Conflict: 1
  - VerificationGap: 3
  - MissingSlot: 4
  - WeakStatement: 1
  - RationaleGap: 1
  - Normalization: 2
  - TBD_Question: 3
  - MatrixError: 0
- Notable conflicts: 1
- Matrix parse errors: 0

## Matrix A - Orientation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| A:[normative]:[guiding] | normative | guiding | prescriptive direction | 1 | HAS_ITEMS | Governance owner TBD is salient under this lens. |
| A:[normative]:[applying] | normative | applying | mandatory practice | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| A:[normative]:[judging] | normative | judging | compliance determination | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| A:[normative]:[reviewing] | normative | reviewing | regulatory audit | 1 | HAS_ITEMS | Lifecycle-state traceability conflict is salient under this lens. |
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
| A-001 | A:[normative]:[guiding] | TBD_Question | Datasheet | Datasheet | TBD: identify the human or project authority responsible for plugin contract governance rulings. | Datasheet records `Responsible party` as TBD, while this deliverable intentionally leaves future permission, sandbox, transport, and registry decisions for human ruling. | Datasheet.md | Identification | NA | PROPOSAL: keep as explicit ownership TBD until the project authority assigns responsibility. | TBD |
| A-002 | A:[normative]:[reviewing] | Conflict | Procedure | Procedure | Clarify lifecycle-state wording in Procedure prerequisites so it reflects current `SEMANTIC_READY` state or explicitly marks `OPEN` as historical Pass 1+2 context. | Procedure says the Pass 1+2 run state was `OPEN`, but `_STATUS.md` now records `SEMANTIC_READY`; the mismatch can confuse audit sequencing. | Procedure.md<br>_STATUS.md | Prerequisites<br>Status: DEL-02-04 Plugin and extension domain contracts | Procedure.md#prerequisites<br>_STATUS.md#status-del-02-04-plugin-and-extension-domain-contracts | PROPOSAL: later metadata/procedure pass should reconcile lifecycle wording without changing status history. | TBD |

## Matrix B - Conceptualization

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| B:[data]:[necessity] | data | necessity | essential fact | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| B:[data]:[sufficiency] | data | sufficiency | adequate evidence | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| B:[data]:[completeness] | data | completeness | comprehensive record | 1 | HAS_ITEMS | Extension point registry remains assumption-level. |
| B:[data]:[consistency] | data | consistency | reliable measurement | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| B:[information]:[necessity] | information | necessity | essential signal | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| B:[information]:[sufficiency] | information | sufficiency | adequate context | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| B:[information]:[completeness] | information | completeness | comprehensive account | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| B:[information]:[consistency] | information | consistency | coherent message | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| B:[knowledge]:[necessity] | knowledge | necessity | fundamental understanding | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| B:[knowledge]:[sufficiency] | knowledge | sufficiency | competent expertise | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| B:[knowledge]:[completeness] | knowledge | completeness | thorough mastery | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| B:[knowledge]:[consistency] | knowledge | consistency | coherent understanding | 1 | HAS_ITEMS | Decomposition revision wording differs between local references and current context. |
| B:[wisdom]:[necessity] | wisdom | necessity | essential discernment | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| B:[wisdom]:[sufficiency] | wisdom | sufficiency | adequate judgment | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| B:[wisdom]:[completeness] | wisdom | completeness | holistic insight | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| B:[wisdom]:[consistency] | wisdom | consistency | principled reasoning | 0 | NO_ITEMS | No additional warranted item under tight threshold. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| B-001 | B:[data]:[completeness] | MissingSlot | Datasheet | Datasheet | Add an approved registry slot or explicit TBD/open-issue pointer for extension point families. | Candidate families are recorded as ASSUMPTION and the Procedure says concrete names need later approval; the data record has no approved registry or decision pointer. | Datasheet.md<br>Procedure.md<br>Specification.md | Construction<br>Steps, step 4<br>Documentation | NA | PROPOSAL: keep current candidate families non-authoritative until an approved registry is cited. | TBD |
| B-002 | B:[knowledge]:[consistency] | Normalization | Datasheet | TBD | Normalize the local decomposition revision wording for DEL-02-04 references. | `_CONTEXT.md` and Datasheet cite accepted revision 0.4, while `_REFERENCES.md` still says accepted v0.2; the discrepancy is traceability drift. | _CONTEXT.md<br>Datasheet.md<br>_REFERENCES.md | Decomposition Reference<br>References<br>Decomposition and Registers | NA | PROPOSAL: route through a later metadata/reference cleanup because `_REFERENCES.md` is outside this run's write scope. | TBD |

## Matrix C - Formulation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| C:[normative]:[necessity] | normative | necessity | binding obligation basis | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| C:[normative]:[sufficiency] | normative | sufficiency | warranted control basis | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| C:[normative]:[completeness] | normative | completeness | comprehensive control frame | 1 | HAS_ITEMS | Manifest contract concept list lacks an approved field model. |
| C:[normative]:[consistency] | normative | consistency | coherent governance frame | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| C:[operative]:[necessity] | operative | necessity | essential execution basis | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| C:[operative]:[sufficiency] | operative | sufficiency | adequate practice basis | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| C:[operative]:[completeness] | operative | completeness | whole workflow basis | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| C:[operative]:[consistency] | operative | consistency | stable process basis | 1 | HAS_ITEMS | Boundary-equivalence wording is implementation-significant. |
| C:[evaluative]:[necessity] | evaluative | necessity | critical value basis | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| C:[evaluative]:[sufficiency] | evaluative | sufficiency | reasoned appraisal basis | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| C:[evaluative]:[completeness] | evaluative | completeness | integral appraisal frame | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| C:[evaluative]:[consistency] | evaluative | consistency | principled appraisal frame | 0 | NO_ITEMS | No additional warranted item under tight threshold. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| C-001 | C:[normative]:[completeness] | MissingSlot | Specification | Specification | Add a manifest contract field-model slot or keep each listed manifest concept explicitly marked TBD. | Specification lists required manifest concepts but leaves exact schema layout and field names TBD; downstream schema work needs a clear placeholder inventory. | Specification.md<br>Datasheet.md | Documentation<br>Construction | NA | PROPOSAL: later enrichment should add only placeholders or approved fields, not invented schema syntax. | TBD |
| C-002 | C:[operative]:[consistency] | WeakStatement | Specification | Specification | Clarify what qualifies as an `equivalent governed service boundary` for mutating plugin operations. | REQ-09 allows equivalent governed boundaries but does not define equivalence criteria; that ambiguity could affect implementation decisions. | Specification.md | Requirements, DEL-02-04-REQ-09 | NA | PROPOSAL: define equivalence in terms of validation, diagnostics, envelope, provenance, and report-control preservation. | TBD |

## Matrix F - Requirements

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| F:[normative]:[necessity] | normative | necessity | binding evidence threshold | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| F:[normative]:[sufficiency] | normative | sufficiency | warranted assurance threshold | 1 | HAS_ITEMS | Layered verification lacks concrete sufficiency criteria. |
| F:[normative]:[completeness] | normative | completeness | full control coverage | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| F:[normative]:[consistency] | normative | consistency | coherent compliance basis | 1 | HAS_ITEMS | Schema TBDs need an interim review gate. |
| F:[operative]:[necessity] | operative | necessity | required action evidence | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| F:[operative]:[sufficiency] | operative | sufficiency | adequate execution proof | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| F:[operative]:[completeness] | operative | completeness | whole practice coverage | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| F:[operative]:[consistency] | operative | consistency | stable workflow assurance | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| F:[evaluative]:[necessity] | evaluative | necessity | essential appraisal evidence | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| F:[evaluative]:[sufficiency] | evaluative | sufficiency | reasoned merit assurance | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| F:[evaluative]:[completeness] | evaluative | completeness | integral appraisal coverage | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| F:[evaluative]:[consistency] | evaluative | consistency | principled quality assurance | 0 | NO_ITEMS | No additional warranted item under tight threshold. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| F-001 | F:[normative]:[sufficiency] | VerificationGap | Specification | Specification | Add acceptance criteria for layered plugin/adapter verification gates. | REQ-14 requires schema, unit, provenance, diagnostics, protected-content, and regression checks, but the verification table does not define pass/fail criteria or severity mapping. | Specification.md | Requirements, DEL-02-04-REQ-14; Verification | NA | PROPOSAL: later enrichment should add criteria only at the level supported by approved schemas and gates. | TBD |
| F-002 | F:[normative]:[consistency] | VerificationGap | Specification | Specification | Add an interim review gate for schema layout and code-generation TBDs before contract issuance. | REQ-11 leaves concrete schema layout and code-generation tooling TBD, and verification is deferred until concrete schemas exist; there is no interim control to keep those TBDs from being silently bypassed. | Specification.md | Requirements, DEL-02-04-REQ-11; Verification; Documentation | NA | PROPOSAL: record a pre-implementation review checkpoint rather than resolving the schema design here. | TBD |

## Matrix D - Objectives

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| D:[normative]:[guiding] | normative | guiding | controlled decision charter | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| D:[normative]:[applying] | normative | applying | enforceable practice boundary | 1 | HAS_ITEMS | Permission and sandbox TBDs need a ruling path. |
| D:[normative]:[judging] | normative | judging | adjudicated closure basis | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| D:[normative]:[reviewing] | normative | reviewing | audit closure record | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| D:[operative]:[guiding] | operative | guiding | executable direction charter | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| D:[operative]:[applying] | operative | applying | governed action protocol | 1 | HAS_ITEMS | Capability class lists are not yet normalized. |
| D:[operative]:[judging] | operative | judging | measured performance basis | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| D:[operative]:[reviewing] | operative | reviewing | process audit record | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| D:[evaluative]:[guiding] | evaluative | guiding | value-aligned direction frame | 1 | HAS_ITEMS | Extension point family assumptions lack rationale. |
| D:[evaluative]:[applying] | evaluative | applying | merit-grounded practice frame | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| D:[evaluative]:[judging] | evaluative | judging | defensible appraisal basis | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| D:[evaluative]:[reviewing] | evaluative | reviewing | quality appraisal record | 0 | NO_ITEMS | No additional warranted item under tight threshold. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| D-001 | D:[normative]:[applying] | TBD_Question | Specification | Specification | TBD: identify decision inputs and approval path for exact permission names and sandbox mechanism. | Datasheet, Specification, Guidance, and Procedure all preserve sandbox and permission details as TBD; the later decision path itself is not specified. | Datasheet.md<br>Specification.md<br>Guidance.md<br>Procedure.md | Attributes and Construction<br>Requirements, DEL-02-04-REQ-15<br>Considerations and Trade-offs<br>Steps, step 6 | NA | PROPOSAL: keep deny-by-default as current principle while routing exact mechanism to human-approved security design. | TBD |
| D-002 | D:[operative]:[applying] | Normalization | Datasheet | Datasheet | Normalize candidate capability class lists or label one list as an intentional subset. | Datasheet lists a shorter candidate declaration set, while Procedure lists additional classes such as read-only query access, background job execution, and rule-pack evaluator integration. | Datasheet.md<br>Procedure.md | Construction<br>Steps, step 6 | NA | PROPOSAL: later enrichment should align the lists without approving any capability by default. | TBD |
| D-003 | D:[evaluative]:[guiding] | RationaleGap | Guidance | Guidance | Add concise rationale or decision source for candidate extension point families. | Candidate families are named as assumptions, but Guidance does not explain why those families are in scope or whether any are intentionally excluded. | Datasheet.md<br>Procedure.md<br>Guidance.md | Construction<br>Steps, step 4<br>Considerations | NA | PROPOSAL: add rationale only if supported by human ruling or later architecture/API work. | TBD |

## Matrix X - Verification

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| X:[guiding]:[necessity] | guiding | necessity | chartered evidence gate | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| X:[guiding]:[sufficiency] | guiding | sufficiency | charter assurance gate | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| X:[guiding]:[completeness] | guiding | completeness | charter coverage gate | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| X:[guiding]:[consistency] | guiding | consistency | charter coherence gate | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| X:[applying]:[necessity] | applying | necessity | practice evidence gate | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| X:[applying]:[sufficiency] | applying | sufficiency | practice assurance gate | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| X:[applying]:[completeness] | applying | completeness | practice coverage gate | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| X:[applying]:[consistency] | applying | consistency | practice coherence gate | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| X:[judging]:[necessity] | judging | necessity | decision evidence gate | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| X:[judging]:[sufficiency] | judging | sufficiency | decision assurance gate | 1 | HAS_ITEMS | Rule-hook sandbox verification lacks acceptance detail. |
| X:[judging]:[completeness] | judging | completeness | decision coverage gate | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| X:[judging]:[consistency] | judging | consistency | decision coherence gate | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| X:[reviewing]:[necessity] | reviewing | necessity | audit evidence gate | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| X:[reviewing]:[sufficiency] | reviewing | sufficiency | audit assurance gate | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| X:[reviewing]:[completeness] | reviewing | completeness | audit coverage gate | 1 | HAS_ITEMS | Reproducibility evidence artifacts are not named. |
| X:[reviewing]:[consistency] | reviewing | consistency | audit coherence gate | 0 | NO_ITEMS | No additional warranted item under tight threshold. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| X-001 | X:[judging]:[sufficiency] | VerificationGap | Specification | Specification | Add acceptance criteria for rule-pack-facing sandbox tests. | REQ-13 and the verification table require no arbitrary code, filesystem, or network access, but do not define the test evidence needed to judge determinism or isolation. | Specification.md<br>Procedure.md | Requirements, DEL-02-04-REQ-13; Verification<br>Steps, steps 6 and 9 | NA | PROPOSAL: leave exact sandbox technology TBD while defining evidence categories for later tests. | TBD |
| X-002 | X:[reviewing]:[completeness] | MissingSlot | Procedure | Procedure | Name the records expected from manifest canonicalization and hash verification. | REQ-12 requires canonical JSON/JCS-compatible hashing when used, and verification mentions hash tests, but Procedure Records does not name the expected reproducibility evidence artifacts. | Specification.md<br>Procedure.md | Requirements, DEL-02-04-REQ-12; Verification<br>Records | NA | PROPOSAL: add record placeholders only after concrete schema artifacts are approved. | TBD |

## Matrix E - Evaluation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| E:[guiding]:[data] | guiding | data | chartered fact validation | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| E:[guiding]:[information] | guiding | information | chartered signal validation | 1 | HAS_ITEMS | Human-ruling log is required but not instantiated. |
| E:[guiding]:[knowledge] | guiding | knowledge | chartered expertise validation | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| E:[guiding]:[wisdom] | guiding | wisdom | chartered judgment validation | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| E:[applying]:[data] | applying | data | practice fact validation | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| E:[applying]:[information] | applying | information | practice signal validation | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| E:[applying]:[knowledge] | applying | knowledge | practice expertise validation | 0 | NO_ITEMS | No additional warranted item under tight threshold. |
| E:[applying]:[wisdom] | applying | wisdom | practice judgment validation | 1 | HAS_ITEMS | Plugin-facing privacy/telemetry status needs explicit ruling. |
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
| E-001 | E:[guiding]:[information] | MissingSlot | Specification | Guidance | Add a human-ruling log entry or open-issue pointer for public API transport, extension registry, permission taxonomy, import/export formats, and sandbox technology. | Specification requires a human-ruling log or future open-issue references, but no concrete entries or pointers are present in the production documents. | Specification.md<br>Guidance.md<br>_DEPENDENCIES.md | Documentation<br>Conflict Table<br>Coordination | NA | PROPOSAL: use a future approved issue/ruling index; do not resolve the decisions in this register. | TBD |
| E-002 | E:[applying]:[wisdom] | TBD_Question | Specification | Specification | TBD: decide whether plugins can ever receive telemetry-facing data; if yes, add explicit privacy requirement and verification coverage. | Datasheet and Guidance mention telemetry/private-by-default constraints, but Specification has no plugin-facing privacy or telemetry requirement beyond general data-boundary language. | Datasheet.md<br>Guidance.md<br>Specification.md | Conditions<br>Considerations<br>Requirements scanned | NA | PROPOSAL: preserve telemetry off-by-default and private-data prohibition unless later human privacy/security ruling expands plugin exposure. | TBD |
