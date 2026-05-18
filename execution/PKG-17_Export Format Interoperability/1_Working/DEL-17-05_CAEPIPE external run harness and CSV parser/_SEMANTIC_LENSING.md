# Semantic Lensing Register: DEL-17-05 CAEPIPE external run harness and CSV parser

**Generated:** 2026-05-18
**Deliverable Folder:** /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-05_CAEPIPE external run harness and CSV parser

**Inputs Read:**
- _CONTEXT.md — _CONTEXT.md#context-del-17-05
- _STATUS.md — _STATUS.md#status-del-17-05
- _SEMANTIC.md — _SEMANTIC.md#semantic-lens-del-17-05-caepipe-external-run-harness-and-csv-parser
- Datasheet.md — Datasheet.md#datasheet-del-17-05-caepipe-external-run-harness-and-csv-parser
- Specification.md — Specification.md#specification-del-17-05-caepipe-external-run-harness-and-csv-parser
- Guidance.md — Guidance.md#guidance-del-17-05-caepipe-external-run-harness-and-csv-parser
- Procedure.md — Procedure.md#procedure-del-17-05-caepipe-external-run-harness-and-csv-parser
- _REFERENCES.md — _REFERENCES.md#references-del-17-05-caepipe-external-run-harness-and-csv-parser

**Purpose:** Apply `semantic-matrix-build` matrix cells as lenses over the production documents, capturing warranted enrichment inputs for a later enrichment pass.

## Summary

- Total warranted items: 15
- By document:
  - Datasheet: 1
  - Specification: 6
  - Guidance: 4
  - Procedure: 4
- By matrix:
  - A: 2  B: 3  C: 2  F: 2  D: 2  X: 2  E: 2
- By type:
  - Conflict: 1
  - VerificationGap: 3
  - MissingSlot: 5
  - WeakStatement: 0
  - RationaleGap: 1
  - Normalization: 2
  - TBD_Question: 3
  - MatrixError: 0
- Notable conflicts: 1
- Matrix parse errors: 0

## Matrix A — Orientation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| A:[normative]:[guiding] | normative | guiding | prescriptive direction | 0 | NO_ITEMS | Lens checked the directive framing; Specification scope and requirements keep the opt-in external-harness boundary explicit. |
| A:[normative]:[applying] | normative | applying | mandatory practice | 1 | HAS_ITEMS | See A-001 for the missing future acceptance detail around user license responsibility. |
| A:[normative]:[judging] | normative | judging | compliance determination | 0 | NO_ITEMS | Lens checked compliance classification; documents consistently reject code-compliance, compatibility, and professional-acceptance claims. |
| A:[normative]:[reviewing] | normative | reviewing | regulatory audit | 0 | NO_ITEMS | Lens checked audit posture; Specification standards and Procedure manual checks preserve the IP/data boundary. |
| A:[operative]:[guiding] | operative | guiding | procedural direction | 0 | NO_ITEMS | Lens checked procedural orientation; Procedure separates Phase A population from later implementation steps. |
| A:[operative]:[applying] | operative | applying | practical execution | 1 | HAS_ITEMS | See A-002 for the missing concrete configuration-surface slot for practical harness execution. |
| A:[operative]:[judging] | operative | judging | performance assessment | 0 | NO_ITEMS | Lens checked performance assessment; verification is framed as future operational evidence, not solver performance certification. |
| A:[operative]:[reviewing] | operative | reviewing | process audit | 0 | NO_ITEMS | Lens checked process audit; future records are identified, while implementation is outside this Phase A tranche. |
| A:[evaluative]:[guiding] | evaluative | guiding | value orientation | 0 | NO_ITEMS | Lens checked value orientation; Guidance states conservative use without turning regression evidence into authority. |
| A:[evaluative]:[applying] | evaluative | applying | merit application | 0 | NO_ITEMS | Lens checked evidence use; parser and run outputs are limited to regression and handoff support. |
| A:[evaluative]:[judging] | evaluative | judging | worth determination | 0 | NO_ITEMS | Lens checked judgment framing; the documents route unresolved target behavior to TBD rather than acceptance. |
| A:[evaluative]:[reviewing] | evaluative | reviewing | quality appraisal | 0 | NO_ITEMS | Lens checked review quality; reviewer rejection criteria directly cover the boundary risks. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| A-001 | A:[normative]:[applying] | VerificationGap | Specification | Specification | Add future acceptance criteria for how a user-supplied executable path and license/environment responsibility are evidenced. | REQ-001 requires user supply and responsibility acceptance, but verification only checks boundary absence and skip behavior, not the evidence that responsibility was accepted. | Specification.md | Requirements; Verification | NA | PROPOSAL: Specification verification should define the observable acceptance/configuration evidence. | TBD |
| A-002 | A:[operative]:[applying] | MissingSlot | Procedure | Procedure | Add a TBD slot for the explicit configuration surface used to provide the CAEPIPE executable path. | Future procedure says resolve the executable path from user configuration, but no environment variable, config key, CLI field, or equivalent placeholder is named. | Procedure.md | Future external-run procedure | NA | PROPOSAL: Procedure should carry a named configuration-surface TBD until implementation chooses it. | TBD |

## Matrix B — Conceptualization

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| B:[data]:[necessity] | data | necessity | essential fact | 0 | NO_ITEMS | Lens checked essential facts; Datasheet identifies harness role, executable boundary, input target, expected CSV output, and dependency. |
| B:[data]:[sufficiency] | data | sufficiency | adequate evidence | 0 | NO_ITEMS | Lens checked evidence sufficiency; Conditions distinguish source-supported facts from target-profile TBDs. |
| B:[data]:[completeness] | data | completeness | comprehensive record | 1 | HAS_ITEMS | See B-001 for the unresolved run-directory filename and record-shape slot. |
| B:[data]:[consistency] | data | consistency | reliable measurement | 0 | NO_ITEMS | Lens checked measurement reliability; unit, coordinate, ID-map, and parser uncertainty are preserved as diagnostics. |
| B:[information]:[necessity] | information | necessity | essential signal | 0 | NO_ITEMS | Lens checked signal content; Scope and Attributes identify why the harness exists and what it must not imply. |
| B:[information]:[sufficiency] | information | sufficiency | adequate context | 0 | NO_ITEMS | Lens checked context; the four documents explain CAEPIPE public-source limits and DEL-17-04 dependency. |
| B:[information]:[completeness] | information | completeness | comprehensive account | 0 | NO_ITEMS | Lens checked account completeness; the documents cover Phase A, future run, future skip, and future record categories. |
| B:[information]:[consistency] | information | consistency | coherent message | 1 | HAS_ITEMS | See B-002 for target-output vocabulary that should be normalized before implementation. |
| B:[knowledge]:[necessity] | knowledge | necessity | fundamental understanding | 0 | NO_ITEMS | Lens checked fundamental understanding; Guidance explains the external-run value without broad compatibility claims. |
| B:[knowledge]:[sufficiency] | knowledge | sufficiency | competent expertise | 0 | NO_ITEMS | Lens checked expertise framing; source-supported behavior is separated from parser-section and invocation gaps. |
| B:[knowledge]:[completeness] | knowledge | completeness | thorough mastery | 0 | NO_ITEMS | Lens checked mastery claims; the documents deliberately avoid asserting full CAEPIPE behavior knowledge. |
| B:[knowledge]:[consistency] | knowledge | consistency | coherent understanding | 0 | NO_ITEMS | Lens checked understanding coherence; diagnostic handling is repeated consistently across docs. |
| B:[wisdom]:[necessity] | wisdom | necessity | essential discernment | 0 | NO_ITEMS | Lens checked discernment; boundary choices are conservative and avoid license or authority overreach. |
| B:[wisdom]:[sufficiency] | wisdom | sufficiency | adequate judgment | 1 | HAS_ITEMS | See B-003 for the missing decision authority on when unresolved CAEPIPE questions become actionable. |
| B:[wisdom]:[completeness] | wisdom | completeness | holistic insight | 0 | NO_ITEMS | Lens checked holistic framing; Guidance trade-offs cover convenience, scope, CI, identity, and data handling. |
| B:[wisdom]:[consistency] | wisdom | consistency | principled reasoning | 0 | NO_ITEMS | Lens checked reasoning consistency; the documents maintain user-owned, opt-in, non-authoritative execution language. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| B-001 | B:[data]:[completeness] | MissingSlot | Multi | Specification | Add a run-directory record-shape placeholder for required filenames or manifest links. | Datasheet and Specification require a run directory with MBF, manifest, ID map, loss report, CSV, and metadata, but exact filenames and record schema remain only described at concept level. | Datasheet.md; Specification.md | Construction; Requirements | NA | PROPOSAL: Specification should hold the required record-shape placeholder until schema/code scope exists. | TBD |
| B-002 | B:[information]:[consistency] | Normalization | Multi | Guidance | Normalize target-output vocabulary across expected CSV path, observed CSV path, CSV result file, and CSV output. | The documents use several terms for the CAEPIPE CSV artifact; without a vocabulary note, implementation could split one concept into divergent fields. | Datasheet.md; Specification.md; Procedure.md | Attributes; Requirements; Future external-run procedure | NA | PROPOSAL: Guidance should define preferred vocabulary and aliases for the target CSV artifact. | TBD |
| B-003 | B:[wisdom]:[sufficiency] | TBD_Question | Guidance | Guidance | TBD: Who can approve closure of invocation-profile and parser-section TBDs: human project authority, CAEPIPE support response, or DEL-17-01 source-basis update? | The documents preserve the TBDs, but do not name the decision path for converting them into implementation-ready scope. | Guidance.md; Specification.md | Conflict Table; Requirements | NA | PROPOSAL: Guidance should record the ruling path before implementation consumes these TBDs. | TBD |

## Matrix C — Formulation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| C:[normative]:[necessity] | normative | necessity | authority boundary | 0 | NO_ITEMS | Lens checked authority boundary; Specification explicitly forbids bundling, bypass, formal validation, and professional claims. |
| C:[normative]:[sufficiency] | normative | sufficiency | evidence threshold | 1 | HAS_ITEMS | See C-001 for the missing threshold for treating source or fixture confirmation as parser coverage. |
| C:[normative]:[completeness] | normative | completeness | coverage mandate | 0 | NO_ITEMS | Lens checked mandate coverage; requirements span external execution, parser boundaries, diagnostics, CI skip, and data rights. |
| C:[normative]:[consistency] | normative | consistency | traceable constraint | 0 | NO_ITEMS | Lens checked traceable constraints; Standards table ties project invariants to the CAEPIPE-specific source basis. |
| C:[operative]:[necessity] | operative | necessity | execution prerequisite | 1 | HAS_ITEMS | See C-002 for unresolved prerequisite status of the first supported MBF profile. |
| C:[operative]:[sufficiency] | operative | sufficiency | workable evidence | 0 | NO_ITEMS | Lens checked workable evidence; future run steps capture invocation attempt, CSV discovery, parser execution, and diagnostics. |
| C:[operative]:[completeness] | operative | completeness | process coverage | 0 | NO_ITEMS | Lens checked process coverage; Procedure includes Phase A, future external-run, and skip-without-executable paths. |
| C:[operative]:[consistency] | operative | consistency | repeatable method | 0 | NO_ITEMS | Lens checked repeatability; future procedure records metadata before invocation and diagnostics after parser execution. |
| C:[evaluative]:[necessity] | evaluative | necessity | review basis | 0 | NO_ITEMS | Lens checked review basis; Verification V-001 through V-007 state review surfaces for document and future implementation work. |
| C:[evaluative]:[sufficiency] | evaluative | sufficiency | judgment threshold | 0 | NO_ITEMS | Lens checked judgment threshold; Guidance reviewer rejections are concrete enough for this Phase A contract. |
| C:[evaluative]:[completeness] | evaluative | completeness | appraisal coverage | 0 | NO_ITEMS | Lens checked appraisal coverage; manual and future implementation checks cover fixtures, metadata, and evidence classification. |
| C:[evaluative]:[consistency] | evaluative | consistency | coherent rationale | 0 | NO_ITEMS | Lens checked rationale coherence; conservative choices are tied to explicit trade-offs. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| C-001 | C:[normative]:[sufficiency] | VerificationGap | Specification | Specification | Add acceptance criteria for what qualifies a CSV section as source-confirmed or fixture-confirmed. | REQ-009 limits parsing to source-confirmed or fixture-confirmed sections, but Verification V-005 does not define minimum evidence for confirming a section. | Specification.md | Requirements; Verification | NA | PROPOSAL: Specification should define the parser-section evidence threshold. | TBD |
| C-002 | C:[operative]:[necessity] | TBD_Question | Procedure | Procedure | TBD: Which DEL-17-04 MBF profile is the first prerequisite accepted for live external-run tests? | Datasheet and Specification carry the MBF profile as unresolved; Procedure future steps assume an export package can be loaded. | Datasheet.md; Specification.md; Procedure.md | Attributes; Requirements; Future external-run procedure | NA | PROPOSAL: Procedure should require the accepted DEL-17-04 profile identifier before live external execution. | TBD |

## Matrix F — Requirements

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| F:[normative]:[necessity] | normative | necessity | required boundary | 1 | HAS_ITEMS | See F-001 for the missing external ruling source for the first invocation profile. |
| F:[normative]:[sufficiency] | normative | sufficiency | accepted substantiation | 0 | NO_ITEMS | Lens checked substantiation; cited public CAEPIPE pages support only bounded MBF/CSV behavior. |
| F:[normative]:[completeness] | normative | completeness | governed coverage | 0 | NO_ITEMS | Lens checked governed coverage; data/IP, authority, unit, reporting, privacy, and agent constraints are listed. |
| F:[normative]:[consistency] | normative | consistency | stable constraint | 0 | NO_ITEMS | Lens checked stable constraints; requirements consistently keep formal validation and compatibility proof out of scope. |
| F:[operative]:[necessity] | operative | necessity | runnable precondition | 0 | NO_ITEMS | Lens checked runnable preconditions; executable path, license, environment, and MBF input prerequisites are present. |
| F:[operative]:[sufficiency] | operative | sufficiency | adequate execution proof | 1 | HAS_ITEMS | See F-002 for missing concrete skip-record evidence fields. |
| F:[operative]:[completeness] | operative | completeness | documented workflow span | 0 | NO_ITEMS | Lens checked workflow span; future external run and skip procedures cover setup through records. |
| F:[operative]:[consistency] | operative | consistency | repeatable evidence path | 0 | NO_ITEMS | Lens checked evidence path; records include manifest, ID map, loss report, command profile, and diagnostics. |
| F:[evaluative]:[necessity] | evaluative | necessity | defensible review basis | 0 | NO_ITEMS | Lens checked review defensibility; reviewer rejection criteria identify boundary violations. |
| F:[evaluative]:[sufficiency] | evaluative | sufficiency | sufficient appraisal ground | 0 | NO_ITEMS | Lens checked appraisal ground; future checks distinguish parser-only tests from opt-in executable tests. |
| F:[evaluative]:[completeness] | evaluative | completeness | complete assurance frame | 0 | NO_ITEMS | Lens checked assurance frame; the documents explicitly avoid assurance beyond regression and handoff evidence. |
| F:[evaluative]:[consistency] | evaluative | consistency | coherent judgment logic | 0 | NO_ITEMS | Lens checked judgment logic; TBDs and diagnostics are the chosen response to unsupported target behavior. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| F-001 | F:[normative]:[necessity] | TBD_Question | Specification | Specification | TBD: What evidence source closes the first invocation profile: selected public command pattern, support clarification, or project-owned adapter decision? | REQ-003 keeps the first executable invocation profile TBD, but the normative closure source is not identified. | Specification.md; Guidance.md | Requirements; Conflict Table | NA | PROPOSAL: Specification should state the permitted closure source for REQ-003. | TBD |
| F-002 | F:[operative]:[sufficiency] | VerificationGap | Specification | Specification | Add expected fields for skip-without-executable evidence, including configured-path absence and public-CI nonfailure. | REQ-012 and Procedure skip steps require explicit skip reasons, but acceptance details for the skip record are not specified. | Specification.md; Procedure.md | Requirements; Future skip-without-executable procedure | NA | PROPOSAL: Specification verification should name the skip evidence fields. | TBD |

## Matrix D — Objectives

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| D:[normative]:[guiding] | normative | guiding | bounded directive | 0 | NO_ITEMS | Lens checked bounded directives; Scope and Requirements confine this tranche to Phase A contract language. |
| D:[normative]:[applying] | normative | applying | controlled obligation | 0 | NO_ITEMS | Lens checked controlled obligations; executable, license, and private-data responsibilities stay user-owned. |
| D:[normative]:[judging] | normative | judging | acceptance boundary | 1 | HAS_ITEMS | See D-001 for the surfaced invocation-profile conflict requiring human ruling. |
| D:[normative]:[reviewing] | normative | reviewing | compliance oversight | 0 | NO_ITEMS | Lens checked compliance oversight; boundary review rejects prohibited binaries, claims, and protected data. |
| D:[operative]:[guiding] | operative | guiding | workflow instruction | 0 | NO_ITEMS | Lens checked workflow instruction; Phase A and future execution paths are separated. |
| D:[operative]:[applying] | operative | applying | executable practice | 0 | NO_ITEMS | Lens checked executable practice; later implementation steps condition execution on explicit configuration. |
| D:[operative]:[judging] | operative | judging | operational proof | 0 | NO_ITEMS | Lens checked operational proof; future checks focus on invocation attempt, output discovery, parser run, and diagnostics. |
| D:[operative]:[reviewing] | operative | reviewing | process traceability | 1 | HAS_ITEMS | See D-002 for a missing durable run-record field list. |
| D:[evaluative]:[guiding] | evaluative | guiding | value frame | 0 | NO_ITEMS | Lens checked value frame; Guidance uses conservative choices to preserve legal and professional boundaries. |
| D:[evaluative]:[applying] | evaluative | applying | evidence use | 0 | NO_ITEMS | Lens checked evidence use; parsed CSV is limited to regression and handoff evidence. |
| D:[evaluative]:[judging] | evaluative | judging | fitness determination | 0 | NO_ITEMS | Lens checked fitness determination; documents do not convert a successful external run into compatibility proof. |
| D:[evaluative]:[reviewing] | evaluative | reviewing | quality appraisal | 0 | NO_ITEMS | Lens checked quality appraisal; review guidance gives rejection triggers for later work. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| D-001 | D:[normative]:[judging] | Conflict | Multi | Guidance | Preserve the invocation-profile conflict as a human-ruling item before any implementation chooses command syntax. | Guidance records related public CAEPIPE command-line and batch-mode patterns without selecting the first OpenPipeStress profile. | Guidance.md; Specification.md | Conflict Table; Requirements | Guidance.md#Conflict Table source A: CAEPIPE-IMPORT-MBF command-line operation; Guidance.md#Conflict Table source B: CAEPIPE-BATCH batch-mode notes | PROPOSAL: Guidance conflict table remains the ruling surface until support clarification or human decision. | TBD |
| D-002 | D:[operative]:[reviewing] | MissingSlot | Procedure | Procedure | Add a future run-record field list for stdout/stderr availability, exit status, output discovery, parser status, and boundary note. | Procedure names categories of future records, but no durable field list is stated for later review to audit repeatability. | Procedure.md; Datasheet.md | Records; Construction | NA | PROPOSAL: Procedure should hold the reviewable field list until implementation defines a schema. | TBD |

## Matrix X — Verification

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| X:[guiding]:[necessity] | guiding | necessity | source basis | 1 | HAS_ITEMS | See X-001 for the local reference normalization gap around CAEPIPE-EXPORT-MBF. |
| X:[guiding]:[sufficiency] | guiding | sufficiency | proof threshold | 0 | NO_ITEMS | Lens checked proof threshold; verification distinguishes source-confirmed facts from unresolved TBDs. |
| X:[guiding]:[completeness] | guiding | completeness | boundary coverage | 0 | NO_ITEMS | Lens checked boundary coverage; no bundled binary, license bypass, protected fixture, or authority claim is permitted. |
| X:[guiding]:[consistency] | guiding | consistency | stable trace | 0 | NO_ITEMS | Lens checked stable trace; DEL-17-02 and DEL-17-04 sidecars are used for canonical ID correlation where possible. |
| X:[applying]:[necessity] | applying | necessity | input readiness | 0 | NO_ITEMS | Lens checked input readiness; Procedure requires MBF input with manifest, ID map, and loss-report records. |
| X:[applying]:[sufficiency] | applying | sufficiency | run evidence | 0 | NO_ITEMS | Lens checked run evidence; future steps capture process attempt, output discovery, parser execution, and diagnostics. |
| X:[applying]:[completeness] | applying | completeness | workflow coverage | 0 | NO_ITEMS | Lens checked workflow coverage; Phase A, external-run, and skip workflows are each documented. |
| X:[applying]:[consistency] | applying | consistency | reproducible path | 0 | NO_ITEMS | Lens checked reproducible path; run metadata records command profile, working directory, timestamps or hashes, and environment notes. |
| X:[judging]:[necessity] | judging | necessity | acceptance basis | 0 | NO_ITEMS | Lens checked acceptance basis; acceptance is restricted to document/future implementation review, not CAEPIPE validation. |
| X:[judging]:[sufficiency] | judging | sufficiency | diagnostic proof | 0 | NO_ITEMS | Lens checked diagnostic proof; unsupported and unmapped sections become diagnostics or TBD. |
| X:[judging]:[completeness] | judging | completeness | coverage finding | 1 | HAS_ITEMS | See X-002 for missing parser coverage-register fields. |
| X:[judging]:[consistency] | judging | consistency | reasoned verdict | 0 | NO_ITEMS | Lens checked verdict reasoning; formal validation and professional acceptance are consistently excluded. |
| X:[reviewing]:[necessity] | reviewing | necessity | audit basis | 0 | NO_ITEMS | Lens checked audit basis; manual checks include protected data and claim-boundary review. |
| X:[reviewing]:[sufficiency] | reviewing | sufficiency | evidence trail | 0 | NO_ITEMS | Lens checked evidence trail; future implementation records include executable provenance and fixture provenance review. |
| X:[reviewing]:[completeness] | reviewing | completeness | checklist coverage | 0 | NO_ITEMS | Lens checked checklist coverage; verification rows span four-document, boundary, TBD, skip, parser, harness, and regression reviews. |
| X:[reviewing]:[consistency] | reviewing | consistency | traceable finding | 0 | NO_ITEMS | Lens checked traceable finding; source basis and limitations are tied to DEL-17-01, DEL-17-02, DEL-17-04, and public CAEPIPE pages. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| X-001 | X:[guiding]:[necessity] | Normalization | Multi | Datasheet | Align the local source list for CAEPIPE-EXPORT-MBF or remove it from Datasheet references if not part of this deliverable's source basis. | Datasheet lists CAEPIPE-EXPORT-MBF, while local _REFERENCES.md does not list that source ID among package references. | Datasheet.md; _REFERENCES.md | References; Package References | NA | PROPOSAL: Datasheet reference list and _REFERENCES should be reconciled by a future authorized pass. | TBD |
| X-002 | X:[judging]:[completeness] | MissingSlot | Specification | Specification | Add parser coverage-register fields for section name, source or fixture basis, supported status, unmapped-row handling, and diagnostic severity. | Specification requires parser coverage and diagnostics, but no future register fields are described for review completeness. | Specification.md; Procedure.md | Requirements; Future implementation checks | NA | PROPOSAL: Specification should define the minimum parser coverage-register shape. | TBD |

## Matrix E — Evaluation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| E:[guiding]:[data] | guiding | data | factual authority | 0 | NO_ITEMS | Lens checked factual authority; CAEPIPE facts are bounded to DEL-17-01 source IDs and official/public references. |
| E:[guiding]:[information] | guiding | information | contextual direction | 0 | NO_ITEMS | Lens checked contextual direction; Package and deliverable context describe optional external validation evidence. |
| E:[guiding]:[knowledge] | guiding | knowledge | governed expertise | 0 | NO_ITEMS | Lens checked governed expertise; the documents defer specialized target behavior to source-confirmed or support-clarified inputs. |
| E:[guiding]:[wisdom] | guiding | wisdom | principled boundary | 0 | NO_ITEMS | Lens checked principled boundary; conservative trade-offs preserve license, fixture, and authority limits. |
| E:[applying]:[data] | applying | data | input fact pattern | 0 | NO_ITEMS | Lens checked input fact pattern; future run starts from MBF input plus manifest, ID map, and loss report. |
| E:[applying]:[information] | applying | information | execution context | 1 | HAS_ITEMS | See E-001 for missing operating-environment detail around configured execution context. |
| E:[applying]:[knowledge] | applying | knowledge | practiced handling | 0 | NO_ITEMS | Lens checked practiced handling; implementation is deferred, and future steps define the intended handling sequence. |
| E:[applying]:[wisdom] | applying | wisdom | prudent operation | 0 | NO_ITEMS | Lens checked prudent operation; no executable search, install, redistribution, or bypass behavior is allowed. |
| E:[judging]:[data] | judging | data | evidence basis | 0 | NO_ITEMS | Lens checked evidence basis; parsed rows must bind to metadata and canonical IDs or be marked weak/unmapped. |
| E:[judging]:[information] | judging | information | diagnostic meaning | 0 | NO_ITEMS | Lens checked diagnostic meaning; unrecognized, missing, unstable, or unsupported sections become diagnostics or TBD. |
| E:[judging]:[knowledge] | judging | knowledge | assessment expertise | 0 | NO_ITEMS | Lens checked assessment expertise; the documents avoid claiming CAEPIPE solver validity. |
| E:[judging]:[wisdom] | judging | wisdom | reasoned decision | 1 | HAS_ITEMS | See E-002 for the missing rationale on decision gates for closing carried TBDs. |
| E:[reviewing]:[data] | reviewing | data | audit trace | 0 | NO_ITEMS | Lens checked audit trace; records include source model/export IDs, executable provenance, output discovery, and fixture provenance. |
| E:[reviewing]:[information] | reviewing | information | record context | 0 | NO_ITEMS | Lens checked record context; future records distinguish executable configuration from parser-only public fixtures. |
| E:[reviewing]:[knowledge] | reviewing | knowledge | appraisal insight | 0 | NO_ITEMS | Lens checked appraisal insight; review guidance highlights overclaiming, protected-data, and uncertainty-hiding failures. |
| E:[reviewing]:[wisdom] | reviewing | wisdom | quality rationale | 0 | NO_ITEMS | Lens checked quality rationale; guidance explains the trade-offs behind conservative parser and CI choices. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| E-001 | E:[applying]:[information] | MissingSlot | Multi | Procedure | Add execution-environment context fields for OS, working directory, permissions, and remote Windows or compatibility notes. | Datasheet identifies execution environment as user-owned, and Procedure says record environment notes, but the specific context fields are not enumerated. | Datasheet.md; Procedure.md | Attributes; Future external-run procedure | NA | PROPOSAL: Procedure should list the minimum execution-context fields for later implementation. | TBD |
| E-002 | E:[judging]:[wisdom] | RationaleGap | Guidance | Guidance | Add rationale for the gate that decides when invocation-profile and parser-section TBDs may move from documentation to implementation scope. | Guidance preserves the TBDs and gives trade-offs, but not the decision rationale for when the project has adequate judgment to close them. | Guidance.md; Specification.md | Considerations; Requirements | NA | PROPOSAL: Guidance should state the decision rationale and authority before implementation closes the TBDs. | TBD |
