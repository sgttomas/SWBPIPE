# Semantic Lensing Register: DEL-17-08 GLB/glTF review geometry export

**Generated:** 2026-05-18
**Deliverable Folder:** /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-08_GLB glTF review geometry export

**Inputs Read:**
- _CONTEXT.md - deliverable identity, package scope, envelope notes, architecture basis
- _STATUS.md - current SEMANTIC_READY lifecycle state
- _SEMANTIC.md - matrices A, B, C, F, D, X, E
- Datasheet.md - target family, attributes, conditions, construction package members
- Specification.md - scope, requirements, standards, verification, documentation
- Guidance.md - purpose, principles, considerations, trade-offs, examples, conflict table
- Procedure.md - prerequisites, Phase A procedure, future profile-development and export-package review procedures
- _REFERENCES.md - governing, package, GLTF-2.0, model schema, and contract references

**Purpose:** Apply `semantic-matrix-build` matrix cells as lenses over the production documents, capturing warranted enrichment inputs for a later enrichment pass.

## Summary

- Total warranted items: 14
- By document:
  - Datasheet: 3
  - Specification: 6
  - Guidance: 3
  - Procedure: 2
- By matrix:
  - A: 1  B: 1  C: 2  F: 2  D: 2  X: 3  E: 3
- By type:
  - Conflict: 0
  - VerificationGap: 5
  - MissingSlot: 3
  - WeakStatement: 1
  - RationaleGap: 1
  - Normalization: 0
  - TBD_Question: 4
  - MatrixError: 0
- Notable conflicts: 0
- Matrix parse errors: 0

## Matrix A - Orientation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| A:[normative]:[guiding] | normative | guiding | prescriptive direction | 0 | NO_ITEMS | Boundary direction is explicit in Specification Scope and Guidance Purpose; no separate enrichment was indicated by this orientation lens. |
| A:[normative]:[applying] | normative | applying | mandatory practice | 1 | HAS_ITEMS | Mandatory `shall` statements exist, and this lens exposed a missing acceptance mapping for later verification. |
| A:[normative]:[judging] | normative | judging | compliance determination | 0 | NO_ITEMS | The documents consistently limit judgement to document and boundary verification, so this lens did not expose a conflicting determination. |
| A:[normative]:[reviewing] | normative | reviewing | regulatory audit | 0 | NO_ITEMS | The kit avoids regulatory or professional acceptance claims and cites project governance boundaries. |
| A:[operative]:[guiding] | operative | guiding | procedural direction | 0 | NO_ITEMS | Procedure separates Phase A population from future profile and package review work with clear step groups. |
| A:[operative]:[applying] | operative | applying | practical execution | 0 | NO_ITEMS | Execution is explicitly deferred from Phase A, matching the deliverable boundary. |
| A:[operative]:[judging] | operative | judging | performance assessment | 0 | NO_ITEMS | Performance-oriented checks are labeled future candidates rather than current acceptance claims. |
| A:[operative]:[reviewing] | operative | reviewing | process audit | 0 | NO_ITEMS | Run records and validation commands provide the process audit surface for this Phase A kit. |
| A:[evaluative]:[guiding] | evaluative | guiding | value orientation | 0 | NO_ITEMS | Guidance states the value priority as visual review plus identity correlation without extending authority. |
| A:[evaluative]:[applying] | evaluative | applying | merit application | 0 | NO_ITEMS | The useful review merit is tied to stable IDs and visible review context, with unsupported behavior left TBD. |
| A:[evaluative]:[judging] | evaluative | judging | worth determination | 0 | NO_ITEMS | Worth is bounded to review context, not solver or professional acceptance, across Specification and Guidance. |
| A:[evaluative]:[reviewing] | evaluative | reviewing | quality appraisal | 0 | NO_ITEMS | Guidance Conflict Table reports no source conflict, and quality appraisal remains limited to future review checks. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| A-001 | A:[normative]:[applying] | VerificationGap | Specification | Specification | Add a future req-to-check mapping for DEL-17-08 `shall` requirements, especially profile, identity, package, and coverage requirements. | Specification contains many normative requirements while Verification says implementation verification remains TBD and lists only candidate future checks. A later enrichment needs an acceptance mapping without inventing pass criteria now. | Specification.md | ## Requirements; ## Verification | NA | Specification Verification should be the preferred place for acceptance mapping, with Procedure holding execution steps. | TBD |

## Matrix B - Conceptualization

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| B:[data]:[necessity] | data | necessity | essential fact | 0 | NO_ITEMS | Essential facts such as target family, source basis, glTF basis, and non-authority boundary are recorded in Datasheet Identification and Attributes. |
| B:[data]:[sufficiency] | data | sufficiency | adequate evidence | 0 | NO_ITEMS | Evidence sources are named in Datasheet References and Specification Standards; no unsupported target fact was found under this lens. |
| B:[data]:[completeness] | data | completeness | comprehensive record | 1 | HAS_ITEMS | Package member records contain several exact-field and exact-location TBDs, exposing a completeness slot. |
| B:[data]:[consistency] | data | consistency | reliable measurement | 0 | NO_ITEMS | Unit basis is consistently meters or blocking diagnostic; no measurement conflict was found. |
| B:[information]:[necessity] | information | necessity | essential signal | 0 | NO_ITEMS | The documents consistently signal that GLB/glTF is review geometry only. |
| B:[information]:[sufficiency] | information | sufficiency | adequate context | 0 | NO_ITEMS | Context is adequate for Phase A because unresolved target behavior is explicitly retained as TBD. |
| B:[information]:[completeness] | information | completeness | comprehensive account | 0 | NO_ITEMS | The four documents cover data, requirements, guidance, and procedure roles without a missing Phase A account section. |
| B:[information]:[consistency] | information | consistency | coherent message | 0 | NO_ITEMS | Boundary, identity, and TBD messages are coherent across Datasheet, Specification, Guidance, and Procedure. |
| B:[knowledge]:[necessity] | knowledge | necessity | fundamental understanding | 0 | NO_ITEMS | The fundamental understanding is visual inspection plus stable identity correlation, stated in Specification Scope and Guidance Purpose. |
| B:[knowledge]:[sufficiency] | knowledge | sufficiency | competent expertise | 0 | NO_ITEMS | The kit avoids unsupported expertise claims and points to GLTF-2.0, DEL-17-01, DEL-17-02, and governance sources. |
| B:[knowledge]:[completeness] | knowledge | completeness | thorough mastery | 0 | NO_ITEMS | Phase A does not claim complete implementation mastery; deferred behavior is marked TBD. |
| B:[knowledge]:[consistency] | knowledge | consistency | coherent understanding | 0 | NO_ITEMS | The documents keep solver meaning separate from visual geometry in a coherent way. |
| B:[wisdom]:[necessity] | wisdom | necessity | essential discernment | 0 | NO_ITEMS | Essential discernment appears in the no-authority boundary and protected-data constraints. |
| B:[wisdom]:[sufficiency] | wisdom | sufficiency | adequate judgment | 0 | NO_ITEMS | Judgment is sufficiently bounded to source-grounded Phase A positions. |
| B:[wisdom]:[completeness] | wisdom | completeness | holistic insight | 0 | NO_ITEMS | The kit frames package, identity, loss reporting, and review limitations together without claiming complete downstream suitability. |
| B:[wisdom]:[consistency] | wisdom | consistency | principled reasoning | 0 | NO_ITEMS | The reasoning principle is stable: unresolved decisions stay TBD and authority claims are excluded. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| B-001 | B:[data]:[completeness] | MissingSlot | Datasheet | Datasheet | Record exact filenames, schema or storage locations, and field sets for `review_geometry_profile.json`, `id_map.json`, manifest, and loss report when authorized. | Datasheet Construction lists the expected package members but several entries remain exact filename, schema, location, or field TBDs. That is appropriate for Phase A but a clear future data slot. | Datasheet.md | ## Construction | NA | Datasheet Construction should remain the descriptive inventory for package member facts. | TBD |

## Matrix C - Formulation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| C:[normative]:[necessity] | normative | necessity | format boundary basis | 0 | NO_ITEMS | Source and Boundary Requirements define the permissible basis for target claims and leave unconfirmed behavior TBD. |
| C:[normative]:[sufficiency] | normative | sufficiency | identity policy threshold | 1 | HAS_ITEMS | The threshold for direct glTF metadata versus sidecar identity is intentionally undecided and needs a future ruling. |
| C:[normative]:[completeness] | normative | completeness | package obligation record | 0 | NO_ITEMS | The package obligation set is stated in Specification Export Package Requirements and Datasheet Construction. |
| C:[normative]:[consistency] | normative | consistency | target claim coherence | 0 | NO_ITEMS | Target claims are consistently constrained to GLTF-2.0 and project source-basis evidence. |
| C:[operative]:[necessity] | operative | necessity | geometry export trigger | 1 | HAS_ITEMS | The exact export service boundary remains TBD, so the trigger/interface is not yet defined. |
| C:[operative]:[sufficiency] | operative | sufficiency | profile execution basis | 0 | NO_ITEMS | Procedure Future profile-development steps identify the execution basis that must exist before writing artifacts. |
| C:[operative]:[completeness] | operative | completeness | package assembly coverage | 0 | NO_ITEMS | Package assembly members are named; the missing exact fields are captured under B-001 instead of duplicated here. |
| C:[operative]:[consistency] | operative | consistency | deterministic handoff flow | 0 | NO_ITEMS | Hashing and timestamp determinism are recognized in requirements, with detailed review gaps captured under later verification lenses. |
| C:[evaluative]:[necessity] | evaluative | necessity | visual review concern | 0 | NO_ITEMS | Visual review concern is clearly bounded by the non-authority statements. |
| C:[evaluative]:[sufficiency] | evaluative | sufficiency | identity adequacy rationale | 0 | NO_ITEMS | Identity adequacy is framed through canonical IDs, direct metadata candidates, sidecar fallback, and loss reporting. |
| C:[evaluative]:[completeness] | evaluative | completeness | limitation coverage frame | 0 | NO_ITEMS | Limitations are repeatedly tied to loss reporting and TBD preservation. |
| C:[evaluative]:[consistency] | evaluative | consistency | boundary judgment alignment | 0 | NO_ITEMS | Boundary judgment is aligned across the four documents and no conflict was identified. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| C-001 | C:[normative]:[sufficiency] | TBD_Question | Specification | Specification | TBD: decide when `name`, `extras`, or extensions are sufficient for direct identity metadata versus when sidecar ID mapping is mandatory; consult the future GLB/glTF profile and consumer-risk evidence. | The documents name candidate metadata locations and require sidecar fallback, but final direct-versus-sidecar policy remains TBD. This affects stable identity sufficiency. | Datasheet.md; Specification.md; Guidance.md | Datasheet ## Conditions; Specification ### Identity and Metadata Requirements; Guidance ### Preserve identity before appearance polish | NA | Specification should hold the requirement threshold; Guidance can explain rationale. | TBD |
| C-002 | C:[operative]:[necessity] | TBD_Question | Specification | Specification | TBD: identify the export service boundary and trigger/interface that will consume the canonical model before writer implementation begins. | Datasheet says the exact export service boundary remains TBD, while Specification excludes an API endpoint or writer implementation in Phase A. The future trigger must be sourced before operational work. | Datasheet.md; Specification.md | Datasheet ## Conditions; Specification ## Scope | NA | Specification Scope or a later interface contract should define the service boundary before Procedure adds execution steps. | TBD |

## Matrix F - Requirements

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| F:[normative]:[necessity] | normative | necessity | target authority gate | 1 | HAS_ITEMS | A source authority gate exists, and this lens exposed the need for profile-level trace evidence. |
| F:[normative]:[sufficiency] | normative | sufficiency | metadata evidence threshold | 0 | NO_ITEMS | Metadata evidence threshold issues are captured more directly by C-001 and E-002. |
| F:[normative]:[completeness] | normative | completeness | package closure standard | 0 | NO_ITEMS | Closure obligations are listed for manifest, ID map, loss report, diagnostics, and target artifact. |
| F:[normative]:[consistency] | normative | consistency | claim coherence control | 0 | NO_ITEMS | Claim coherence is enforced by repeated exclusions of compatibility, validation, release, and professional claims. |
| F:[operative]:[necessity] | operative | necessity | writer readiness gate | 0 | NO_ITEMS | Writer readiness is deliberately out of Phase A and gated by future profile decisions. |
| F:[operative]:[sufficiency] | operative | sufficiency | profile support basis | 0 | NO_ITEMS | The procedure enumerates the support basis needed before profile development proceeds. |
| F:[operative]:[completeness] | operative | completeness | artifact coverage record | 1 | HAS_ITEMS | Fixture and generated artifact records are deferred, leaving a future artifact coverage slot. |
| F:[operative]:[consistency] | operative | consistency | handoff alignment control | 0 | NO_ITEMS | Handoff alignment is bounded by manifest, ID map, loss report, diagnostics, and stable IDs. |
| F:[evaluative]:[necessity] | evaluative | necessity | review priority basis | 0 | NO_ITEMS | Review priority is identity correlation before appearance polish, with no further Phase A edit indicated. |
| F:[evaluative]:[sufficiency] | evaluative | sufficiency | traceability adequacy test | 0 | NO_ITEMS | Traceability adequacy gaps are handled under X-002 and X-003 to avoid duplicate items. |
| F:[evaluative]:[completeness] | evaluative | completeness | limitation coverage appraisal | 0 | NO_ITEMS | Limitation classes are present in Specification and Datasheet; the entity-family coverage TBD is captured under X-001. |
| F:[evaluative]:[consistency] | evaluative | consistency | boundary alignment test | 0 | NO_ITEMS | Boundary alignment is consistent and the Guidance conflict table reports no source conflict. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| F-001 | F:[normative]:[necessity] | VerificationGap | Specification | Specification | Add a future check that each GLB/glTF profile target fact traces to GLTF-2.0, DEL-17-01, DEL-17-02, or project governance evidence. | Specification requires unconfirmed target behavior to remain TBD and lists source grounding as a Phase A check, but future profile evidence checks are not mapped to target facts. | Specification.md | ### Source and Boundary Requirements; ## Verification | NA | Specification Verification should define the source-evidence check for future profile facts. | TBD |
| F-002 | F:[operative]:[completeness] | MissingSlot | Datasheet | Datasheet | Record the future fixture inventory, rights basis, and fixture-to-check coverage when fixture creation is authorized. | Datasheet states geometry fixtures are TBD and none are created in Phase A, while Specification and Guidance identify fixture comparison and rights-cleared examples as future needs. | Datasheet.md; Specification.md; Guidance.md | Datasheet ## Construction; Specification ## Verification; Guidance ## Examples | NA | Datasheet should list fixture facts; Procedure should later describe fixture checks. | TBD |

## Matrix D - Objectives

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| D:[normative]:[guiding] | normative | guiding | format boundary direction | 0 | NO_ITEMS | Format boundary direction is explicit: use GLTF-2.0 facts without extending into solver or professional claims. |
| D:[normative]:[applying] | normative | applying | mandatory metadata practice | 1 | HAS_ITEMS | Timestamp and generator metadata policy has a requirement but lacks a corresponding package review step. |
| D:[normative]:[judging] | normative | judging | claim boundary decision | 0 | NO_ITEMS | Claim boundary decisions are expressed through exclusions and source-basis constraints without contradiction. |
| D:[normative]:[reviewing] | normative | reviewing | package audit trail | 0 | NO_ITEMS | Package audit trail gaps for binary hashes are captured under E-003, so this row is not duplicated. |
| D:[operative]:[guiding] | operative | guiding | export workflow direction | 0 | NO_ITEMS | Future profile-development and package-review procedures provide workflow direction at the allowed level. |
| D:[operative]:[applying] | operative | applying | geometry writing practice | 0 | NO_ITEMS | Geometry writing practice is excluded from Phase A and must wait for an authorized implementation task. |
| D:[operative]:[judging] | operative | judging | handoff performance assessment | 0 | NO_ITEMS | Handoff performance checks are named as future candidates; the verification gap is captured under X-003. |
| D:[operative]:[reviewing] | operative | reviewing | process review record | 0 | NO_ITEMS | Procedure Records and run records provide the Phase A process review record. |
| D:[evaluative]:[guiding] | evaluative | guiding | review value framing | 0 | NO_ITEMS | Review value framing is sufficiently stated as lightweight visual inspection plus ID correlation. |
| D:[evaluative]:[applying] | evaluative | applying | traceability merit use | 0 | NO_ITEMS | Traceability merit is linked to canonical IDs and sidecar fallback, with unresolved policy noted elsewhere. |
| D:[evaluative]:[judging] | evaluative | judging | limitation fitness decision | 1 | HAS_ITEMS | The choice between centerline, tube, surface, or other detail levels lacks decision rationale. |
| D:[evaluative]:[reviewing] | evaluative | reviewing | quality appraisal trail | 0 | NO_ITEMS | Quality appraisal trail remains future-facing and is not asserted as current validation. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| D-001 | D:[normative]:[applying] | VerificationGap | Procedure | Procedure | Add a future export-package review step for timestamp or generator metadata policy: deterministic, normalized, omitted, or intentionally runtime-dependent. | Specification REQ-034 requires timestamp or generator metadata policy, but the future export-package review procedure does not explicitly check it. | Specification.md; Procedure.md | Specification ### Export Package Requirements; Procedure ### Future export-package review procedure | NA | Procedure should carry the operational review check for the Specification requirement. | TBD |
| D-002 | D:[evaluative]:[judging] | RationaleGap | Guidance | Guidance | Add rationale criteria for selecting centerline-only, simplified tube/surface, or other review geometry detail levels. | Guidance lists trade-offs and leaves options TBD, but does not state how a later profile should judge the fitness of one level of detail over another. | Guidance.md | ## Trade-offs | NA | Guidance should hold rationale for level-of-detail decisions; Specification can hold resulting requirements. | TBD |

## Matrix X - Verification

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| X:[guiding]:[necessity] | guiding | necessity | profile basis requirement | 0 | NO_ITEMS | The profile basis requirement is present through GLTF-2.0 version and source-basis declarations. |
| X:[guiding]:[sufficiency] | guiding | sufficiency | evidence adequacy basis | 0 | NO_ITEMS | Evidence adequacy for target facts is captured under F-001. |
| X:[guiding]:[completeness] | guiding | completeness | coverage boundary map | 1 | HAS_ITEMS | Entity-family coverage remains TBD and needs a future profile map. |
| X:[guiding]:[consistency] | guiding | consistency | target alignment signal | 0 | NO_ITEMS | Target alignment is constrained to glTF basis, meters, +Y-up, and declared profile policy. |
| X:[applying]:[necessity] | applying | necessity | metadata control requirement | 1 | HAS_ITEMS | Canonical ID family availability is a prerequisite but not enumerated locally. |
| X:[applying]:[sufficiency] | applying | sufficiency | support readiness basis | 0 | NO_ITEMS | Support readiness remains tied to declared coverage, diagnostics, and loss reporting. |
| X:[applying]:[completeness] | applying | completeness | artifact coverage map | 0 | NO_ITEMS | Artifact coverage is addressed by B-001 and F-002 rather than repeating the same package-member and fixture slots. |
| X:[applying]:[consistency] | applying | consistency | workflow coherence signal | 0 | NO_ITEMS | Workflow coherence is maintained by prerequisites, future profile steps, and package review steps. |
| X:[judging]:[necessity] | judging | necessity | claim decision basis | 0 | NO_ITEMS | Claim decisions are bounded by source-basis requirements and no conflict was identified. |
| X:[judging]:[sufficiency] | judging | sufficiency | performance evidence test | 1 | HAS_ITEMS | Future implementation verification is acknowledged but not yet turned into checks. |
| X:[judging]:[completeness] | judging | completeness | limitation coverage audit | 0 | NO_ITEMS | Limitation coverage audit uses loss report classifications; family coverage TBD is captured under X-001. |
| X:[judging]:[consistency] | judging | consistency | boundary coherence finding | 0 | NO_ITEMS | Boundary coherence is stable across all four documents. |
| X:[reviewing]:[necessity] | reviewing | necessity | audit trace basis | 0 | NO_ITEMS | Audit trace basis is present in manifest, source-basis references, and run records. |
| X:[reviewing]:[sufficiency] | reviewing | sufficiency | package evidence adequacy | 0 | NO_ITEMS | Package evidence adequacy is partly captured by hash and profile checks under D-001 and E-003. |
| X:[reviewing]:[completeness] | reviewing | completeness | record coverage appraisal | 0 | NO_ITEMS | Records are Phase A only, and future records are explicitly listed as TBD. |
| X:[reviewing]:[consistency] | reviewing | consistency | risk alignment audit | 0 | NO_ITEMS | Risk alignment is consistently expressed through protected-data and no-authority boundaries. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| X-001 | X:[guiding]:[completeness] | MissingSlot | Specification | Specification | Declare first review-geometry coverage by canonical entity family, including emitted, omitted, approximated, unsupported, delegated, or TBD status. | Specification REQ-040 requires family coverage declaration, and REQ-041 leaves centerlines, bends, branches, components, supports, interfaces, labels, load indicators, and diagnostics overlays TBD. | Specification.md | ### Review Geometry Coverage Requirements | NA | Specification should own the coverage requirement; Datasheet can later summarize selected coverage. | TBD |
| X-002 | X:[applying]:[necessity] | TBD_Question | Datasheet | Datasheet | TBD: identify the canonical ID families and schema source that must be mapped for emitted and intentionally omitted review entities; consult project-owned schema/contracts. | Procedure says canonical ID families must be available from schema/contracts, while Specification requires stable canonical identity for every emitted or intentionally omitted entity. The local kit does not enumerate the family set. | Procedure.md; Specification.md | Procedure ## Prerequisites; Specification ### Identity and Metadata Requirements | NA | Datasheet should record the descriptive family list once the schema source is selected. | TBD |
| X-003 | X:[judging]:[sufficiency] | VerificationGap | Specification | Specification | Define future implementation verification checks for glTF asset structure, GLB container handling, deterministic hashing, geometry counts, stable-ID correlation, and invented fixture comparison. | Specification Verification lists these as candidate future checks and states implementation verification remains TBD. They need later acceptance criteria before implementation can be judged. | Specification.md | ## Verification | NA | Specification Verification should convert candidate future checks into explicit acceptance checks in a later task. | TBD |

## Matrix E - Evaluation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| E:[guiding]:[data] | guiding | data | profile fact trace | 0 | NO_ITEMS | Profile fact trace is addressed by source-basis and GLTF-2.0 requirements, with detailed future check captured under F-001. |
| E:[guiding]:[information] | guiding | information | target context basis | 1 | HAS_ITEMS | Viewer-specific target context remains TBD and affects how metadata and compatibility statements are worded. |
| E:[guiding]:[knowledge] | guiding | knowledge | boundary understanding map | 0 | NO_ITEMS | Boundary understanding is well mapped: review geometry only, not solver, validation, compliance, or professional acceptance. |
| E:[guiding]:[wisdom] | guiding | wisdom | judgment framing rationale | 0 | NO_ITEMS | Judgment rationale for geometry detail is captured under D-002. |
| E:[applying]:[data] | applying | data | metadata fact control | 0 | NO_ITEMS | Metadata fact control is represented by direct metadata candidates, sidecar fallback, and protected-data limits. |
| E:[applying]:[information] | applying | information | package context handling | 0 | NO_ITEMS | Package context handling is covered by manifest, member inventory, boundary notes, and loss reports. |
| E:[applying]:[knowledge] | applying | knowledge | writer expertise record | 0 | NO_ITEMS | Writer expertise is outside Phase A; no current authoring edit is warranted. |
| E:[applying]:[wisdom] | applying | wisdom | judgment use boundary | 1 | HAS_ITEMS | The phrase "reliable enough" for direct metadata needs a profile-defined threshold. |
| E:[judging]:[data] | judging | data | claim fact audit | 0 | NO_ITEMS | Claim fact audit uses public/project sources and excludes unsupported target claims. |
| E:[judging]:[information] | judging | information | evidence message test | 0 | NO_ITEMS | Evidence messaging issues are represented by E-001 and F-001 rather than a separate row item. |
| E:[judging]:[knowledge] | judging | knowledge | limitation mastery review | 0 | NO_ITEMS | Limitation mastery is appropriately not claimed in Phase A; behavior remains TBD or loss-report driven. |
| E:[judging]:[wisdom] | judging | wisdom | reasoning boundary decision | 0 | NO_ITEMS | Reasoning boundaries are explicit in no-authority and no-private-data constraints. |
| E:[reviewing]:[data] | reviewing | data | package audit trail | 1 | HAS_ITEMS | Binary/non-JSON hash handling has a requirement but lacks an explicit package review check. |
| E:[reviewing]:[information] | reviewing | information | account review basis | 0 | NO_ITEMS | Account review basis is provided through manifest, package inventory, source references, and records. |
| E:[reviewing]:[knowledge] | reviewing | knowledge | quality understanding audit | 0 | NO_ITEMS | Quality understanding is limited to document/boundary verification in Phase A. |
| E:[reviewing]:[wisdom] | reviewing | wisdom | reasoning appraisal trail | 0 | NO_ITEMS | The reasoning appraisal trail preserves TBDs and leaves human rulings open. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| E-001 | E:[guiding]:[information] | TBD_Question | Guidance | Guidance | TBD: resolve viewer-specific rendering, selection, metadata display, and metadata stripping behavior before making viewer compatibility or metadata visibility statements. | Guidance marks viewer behavior TBD and Specification excludes target compatibility and compatibility profiles in Phase A. Later wording needs source-grounded viewer evidence. | Guidance.md; Specification.md | Guidance ## Considerations; Specification ## Scope | NA | Guidance should retain viewer interpretation cautions; Specification should hold any later compatibility limits. | TBD |
| E-002 | E:[applying]:[wisdom] | WeakStatement | Guidance | Guidance | Clarify the phrase "reliable enough" for direct metadata by tying it to profile-declared consumer-risk criteria. | Guidance says to use direct metadata only where the profile says it is reliable enough, and Specification requires the profile to record target location and stripping/consumer-risk policy. The threshold phrase is consequential but not defined. | Guidance.md; Specification.md | Guidance ### Preserve identity before appearance polish; Specification ### Identity and Metadata Requirements | NA | Guidance should explain the judgment threshold; Specification should encode the resulting requirement. | TBD |
| E-003 | E:[reviewing]:[data] | VerificationGap | Procedure | Procedure | Add a future export-package review check that binary GLB or other non-JSON members have hashes or an explicit reason why a hash is unavailable. | Specification REQ-033 requires hash handling for binary or non-JSON members, but the future package review procedure checks inventory and limitations without explicitly checking hashes. | Specification.md; Procedure.md | Specification ### Export Package Requirements; Procedure ### Future export-package review procedure | NA | Procedure should carry the package audit check that operationalizes the Specification requirement. | TBD |
