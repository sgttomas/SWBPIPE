---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-17-04
package_id: PKG-17
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@00115c71931bcae79909602d653740d3bb72dfa1
project_scope_refs: [SOW-030, SOW-074, SOW-075]
package_objective_refs: [OBJ-009, OBJ-017, OBJ-018]
---

# Scope of Work — DEL-17-04

## Purpose and Objective Traceability

This Scope of Work defines `DEL-17-04` in service of project scope [SOW-030, SOW-074, SOW-075] and package objectives [OBJ-009, OBJ-017, OBJ-018].

- **OUT-001** — A versioned CAEPIPE MBF export-profile and deterministic writer contract with explicit supported, omitted, approximated, delegated, sidecar-mapped, unsupported, and TBD behavior is produced.
- **OUT-002** — A source-bounded MBF model importer, the inverse of the writer profile, with a declared first-tranche subset, named refusals, an import loss report, sidecar id restoration, and public round-trip and private regression checks is produced.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-17-04 CAEPIPE MBF export profile and deterministic writer

> #### Datasheet: DEL-17-04 CAEPIPE MBF export profile and deterministic writer
>

### CLM-002 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-17-04 |
> | Package | PKG-17 Export Format Interoperability |
> | Type | BACKEND_FEATURE_SLICE |
> | Source foundation | DEL-17-01 and DEL-17-02 |
> | Lifecycle role | First CAEPIPE-specific export profile/writer foundation deliverable |
>

### CLM-003 — Purpose

> ##### Purpose
>
> DEL-17-04 defines the document-level contract/design and first bounded implementation foundation for a CAEPIPE MBF export profile and deterministic writer. It is grounded in admitted public/official CAEPIPE MBF references, the DEL-17-01 source basis, and the DEL-17-02 export package/profile/loss-report contract.
>
> The implementation foundation emits a deliberately narrow invented smoke subset, a JSON package contract, a deterministic MBF text member, sidecar stable-ID mapping, diagnostics, and loss reports. It does not implement a CAEPIPE result parser, external execution harness, public API, release claim, CAEPIPE compatibility claim, code-compliance claim, or professional-acceptance claim (the desktop GUI action is `CaepipeMbfExportPanel.tsx`, a separate TypeScript preview builder).
>
> DEL-17-04 also reads MBF model data back into a SWBPIPE model document, the inverse of its writer profile (owner ruling, 2026-10-26: <!-- owner-words -->"I want to expand to go both ways. That would make any kind of iteration between the two much more effective. For the validation it isn't strictly necessary, since the proof case lives in CAEPIPE."<!-- /owner-words -->). The importer reads model input only; ingestion of CAEPIPE analysis results stays deferred under DEC-016 (REQ-012 to REQ-020; Design `caepipe-mbf-import.md`).
>
> Current implementation (at `62c1efd75b`): a desktop export action exists outside this module. The Exports section renders `apps/desktop/src/features/caepipe-mbf/CaepipeMbfExportPanel.tsx`, which builds its own preview package JSON and `.mbf` text in TypeScript (`buildCaePipeMbfExportPacket`, `document_kind: openpipestress.technical_preview.caepipe_mbf_export_package`). It does not call `core/handoff/caepipe_mbf/` and is not validated against `schemas/caepipe_mbf_export.schema.json`.
>

### CLM-004 — Source and Evidence Slots

> ##### Source and Evidence Slots
>
> | Source slot | Required treatment |
> |---|---|
> | DEL-17-01 source basis | Carry CAEPIPE source findings, TBD register entries, and question-dossier gates into the target profile. |
> | DEL-17-02 export contract | Carry export profile, stable-ID map, manifest, and loss-report vocabulary into the writer contract. |
> | CAEPIPE public MBF import reference | Treat as evidence for MBF text input, keyword-ordered sections, and command-line behavior only; profile coverage remains source-gated. |
> | CAEPIPE public MBF export reference | Treat as evidence that CAEPIPE exports MBF model data and version-flavoured MBF output; SWBPIPE support claims remain `TBD`. |
> | CAEPIPE public layout pages | `CAEPIPE-MBF-EXAMPLE`, `CAEPIPE-LAYOUT-OFFSETS`, `CAEPIPE-NODE`, `CAEPIPE-MBF-ABSOLUTE` and `CAEPIPE-UNITS` in DEL-17-01: evidence for the record syntax the importer reads (offsets and absolute coordinates, generated bend nodes, feet-inch lengths, weight densities) only; their text and examples are not copied into fixtures. The sources disagree on the placement of the absolute-coordinate marker. This profile selects the trailing marker shown in MBF examples; the alternative remains refused pending clarification (TBD-17-04-009). |
> | CAEPIPE public load-case and technical-reference pages | `CAEPIPE-LOAD-CASES` and `CAEPIPE-FORCE` in DEL-17-01: evidence for case-membership semantics only: which loads each CAEPIPE load case includes; and the Force technical reference documents that sustained-category forces and moments also participate in operating cases. `CAEPIPE-BEND`: evidence that CAEPIPE generates the bend's near and far end nodes with A and B suffixes, and for the section assignment on a bend row, only. |
> | Implementation evidence | `core/handoff/caepipe_mbf/`, `schemas/caepipe_mbf_export.schema.json`, `fixtures/caepipe_mbf/invented/`, and `tests/test_caepipe_mbf_export_package.py` provide bounded foundation evidence. |
>

### CLM-005 — Profile Concepts

> ##### Profile Concepts
>
> | Concept | Required treatment |
> |---|---|
> | Target family | CAEPIPE MBF profile. |
> | Target version basis | `TBD` until source-confirmed. |
> | Record-family subset | `TBD` until downstream source review and/or vendor clarification closes it. |
> | Profile-basis guardrail | The implementation foundation blocks blank or unsupported target-version and record-subset basis values; until source-confirmed closure exists, the profile must carry `TBD-17-01-001`, `TBD-17-01-002`, and `TBD-17-01-003`. |
> | Stable ID strategy | Direct MBF carrying remains `TBD`; the implementation foundation uses manifest-referenced sidecar mapping as the conservative default. |
> | Source basis trace | Each CAEPIPE-specific requirement must map to DEL-17-01, DEL-17-02, a public MBF source, or an explicit `TBD`. |
> | Source basis guardrail | The implementation foundation blocks caller-supplied profile source-basis refs that omit DEL-17-01, DEL-17-02, `CAEPIPE-IMPORT-MBF`, or `CAEPIPE-EXPORT-MBF`, and blocks `DEL-17-03` as CAEPIPE target/source authority. |
> | Loss reporting | Required for exported, omitted, approximated, delegated, unsupported, and TBD behavior. |
> | Unsupported-entity severity | Missing or malformed unsupported-entity evidence blocks the package; explicit unsupported behavior is warning-level by default unless separately marked blocking; `info` severity is not accepted for unsupported behavior. |
> | Pass-through options | May be target configuration only; not local code-checking logic. |
>

### CLM-006 — Boundary Summary

> ##### Boundary Summary
>
> - No CAEPIPE executable, commercial example, proprietary model, or copied vendor fixture is added.
> - An MBF written by CAEPIPE, even of a SWBPIPE model, is CAEPIPE output: it and every import result derived from it stay in the user's private exchange folder and are never committed or quoted in public text. Public import tests use only MBF the project writes itself from invented models.
> - No reverse-engineered behavior or license-bypass workflow is introduced.
> - No protected standards values, material allowables, SIF/flexibility values, or owner criteria are introduced.
> - MBF export work remains a source-bounded foundation until a later tranche closes target version/profile and record-family coverage.
> - Guardrail diagnostics preserve source uncertainty; they do not close CAEPIPE version/profile, record-family coverage, direct stable-ID carrying, compatibility, or professional reliance.
> - DEL-17-03 may remain historical implementation-pattern evidence, but it is not CAEPIPE MBF source-basis authority.

## Completion and Reliance Basis — Epistemology

### CLM-007 — Specification: DEL-17-04 CAEPIPE MBF export profile and deterministic writer

> #### Specification: DEL-17-04 CAEPIPE MBF export profile and deterministic writer
>

### CLM-008 — Normative Scope

> ##### Normative Scope
>
> DEL-17-04 shall define a source-bounded CAEPIPE MBF export profile and deterministic writer contract/design, and may include a first bounded implementation foundation. It shall consume DEL-17-01 source authority and DEL-17-02 package/profile/stable-ID/loss-report requirements.
>
> This deliverable's implementation foundation shall remain limited to deterministic project-owned package building, invented fixtures, schema validation, sidecar stable-ID mapping, diagnostics, and loss reports. It shall not implement a CAEPIPE result parser, external run harness, public API, GUI behavior, release claims, CAEPIPE compatibility claims, code-compliance claims, professional-acceptance claims, or full MBF coverage claims.
>
> DEL-17-04 shall also provide a source-bounded MBF model importer: the inverse of the writer profile, extended only where SWBPIPE represents an MBF construct exactly. It reads a declared subset of MBF model data into a SWBPIPE model document and refuses by name, or reports in an import loss report, everything outside that subset (REQ-012 to REQ-020).
>

### CLM-009 — Requirements

> ##### Requirements
>
> | Req ID | Requirement |
> |---|---|
> | DEL-17-04-REQ-001 | The CAEPIPE MBF profile shall declare target version basis as `TBD` until source-confirmed. |
> | DEL-17-04-REQ-002 | The first MBF record-family subset shall remain `TBD` until source evidence or human-approved clarification closes it. |
> | DEL-17-04-REQ-003 | The writer contract shall not silently approximate unsupported physical or analytical model entities. |
> | DEL-17-04-REQ-004 | The writer contract shall require a loss report for omitted, approximated, delegated, unsupported, and TBD behavior. |
> | DEL-17-04-REQ-005 | Stable canonical ID carrying inside MBF shall remain `TBD`; sidecar mapping shall be required when direct carrying is not source-confirmed. |
> | DEL-17-04-REQ-006 | Pass-through target options shall remain target configuration metadata and shall not become SWBPIPE local code-checking logic. |
> | DEL-17-04-REQ-007 | Later implementation shall use invented fixtures only. |
> | DEL-17-04-REQ-008 | The first implementation foundation shall use sidecar stable-ID mapping while `TBD-17-01-003` remains open. |
> | DEL-17-04-REQ-009 | The first implementation foundation shall expose blocking diagnostics when the invented smoke subset, sidecar stable-ID map, or loss report is missing. |
> | DEL-17-04-REQ-010 | The first implementation foundation shall expose blocking diagnostics when target-version basis, record-subset basis, or carried profile TBD references are missing, blank, or weakened without admitted source-confirmed closure. |
> | DEL-17-04-REQ-011 | The first implementation foundation shall expose blocking diagnostics when profile source-basis refs omit DEL-17-01, DEL-17-02, `CAEPIPE-IMPORT-MBF`, or `CAEPIPE-EXPORT-MBF`, or when they include DEL-17-03 as CAEPIPE target/source authority. |
> | DEL-17-04-REQ-012 | The deliverable shall provide an MBF model importer that reads MBF model input into a SWBPIPE model document of the current model-document version. It shall not read or ingest CAEPIPE analysis results, and a successful import shall not be presented as compatibility, validation or acceptance evidence. |
> | DEL-17-04-REQ-013 | The importer shall read only the declared first-tranche subset of MBF sections, records, key letters, comment keywords and fields listed in Design `caepipe-mbf-import.md`. Every other item shall be refused by name or listed in the import loss report; nothing shall be silently dropped, defaulted or inferred. |
> | DEL-17-04-REQ-014 | The importer shall read English and SI unit modes and the `RAD` option, keep each quantity in its MBF unit where the SWBPIPE unit catalog has that unit, and otherwise apply only the exact definitional conversions its Design selects (feet and inches to inches; kilogram-force to newton). A unit, notation or value it cannot carry or convert that way shall be refused by name. English imports cannot be re-exported through the current SI writer; bidirectional English iteration requires a separately selected writer extension or documented normalization to its SI profile. |
> | DEL-17-04-REQ-015 | Each import shall return diagnostics, an import loss report in the DEL-17-02 categories, and the mapping assumptions applied. Piping-code, hanger-design and analysis options shall be recorded as pass-through target configuration only and shall not become SWBPIPE rule, code-checking or acceptance logic. |
> | DEL-17-04-REQ-016 | CAEPIPE node numbers and record names shall map to SWBPIPE ids deterministically. A byte-bound sidecar shall restore identities and splits it explicitly preserves. Many-to-one mappings and unrecoverable identities shall be reported. A sidecar that is not byte-bound shall be applied only on explicit request, with every restored identity reported as unconfirmed. Each import shall emit its own id map. |
> | DEL-17-04-REQ-017 | Identical MBF bytes, sidecar, material-provenance declaration, rebinding option, source label and importer/profile version shall produce byte-identical outputs. The imported document shall validate against the model-document schema's desktop profile and shall enter the product only through the existing document open path. |
> | DEL-17-04-REQ-018 | Writing a model in the reversible subset and importing it with a bound sidecar shall reproduce the model under the canonical comparison of Design `caepipe-mbf-import.md`, within the documented and justified absolute and relative tolerances stated there. The reversible subset is defined in the Design and requires every value to remain valid as written and distinct records to retain distinct keys. The tolerances cover formatting and arithmetic error plus the writer-admitted residuals. For bends, these residuals are the tangent-length, intersection and orientation residuals. Bend-plane orientation is compared as the normalized projection of `y_reference` perpendicular to the unit chord, and its residual is computed with that same projection. A model with residuals the tolerances do not cover is excluded explicitly. Models in the byte-reproducible subset defined in the Design shall be written again byte for byte from their import, under a bound sidecar and identical writer settings. Every exclusion and representation loss shall be explicit. Every formatting collision shall be reported explicitly. An unbound import of writer output shall give its named refusals, except where an explicitly retained bounded profile decision admits the records. |
> | DEL-17-04-REQ-019 | MBF files written by CAEPIPE, and every import result derived from them, shall be handled as private data, and imported entities shall carry provenance naming the source MBF hash. Material mechanics data shall require affirmative project-invented or user-supplied provenance. A bound sidecar qualifies only when its provenance affirmatively identifies project-invented data or the user's own supplied source; vendor-library, unknown and suspected protected origins remain refused. Redistribution disposition alone is insufficient. Allowable, strength, fatigue and code data shall never enter the model. |
> | DEL-17-04-REQ-020 | A private regression check shall re-import the CAEPIPE-exported MBFs held in the user's exchange folder and fail when an import outcome moves from its recorded classification. All CAEPIPE-derived values, differences, classifications and detailed diagnostics shall remain in the private exchange folder. Standard output shall contain operational status only. The check shall skip, stating the reason, when no exchange folder is configured. |
>

### CLM-010 — Verification Requirements

> ##### Verification Requirements
>
> | Check | Requirement |
> |---|---|
> | Source trace | CAEPIPE-specific statements cite DEL-17-01 source IDs or remain `TBD`. |
> | Boundary trace | No proprietary examples or protected data are introduced. |
> | Loss trace | Unsupported and TBD behavior is visible in the loss-report contract. |
> | Profile-basis trace | Target-version, record-subset, and direct stable-ID uncertainty remain visible through carried TBD refs until source-confirmed closure exists. |
> | Source-basis guardrail trace | Profile and manifest source-basis refs carry the required authority set and do not promote implementation-pattern evidence to CAEPIPE source authority. |
> | Import subset trace | Each imported MBF item in the Design is cited to an admitted DEL-17-01 source, except selected interpretations the Design labels as such: the trailing absolute-coordinate marker (TBD-17-04-009), and any explicitly retained bounded profile decision, which the Design delimits as an extension. Each selected interpretation is traced to its documented part and its open TBD. Behaviour learned from the target tool is stated generically, citing private notes. Each named refusal and loss entry has a test that checks its code, location and effect. |
> | Import round-trip trace | Invented models in the defined reversible subset pass canonical equality within the derived tolerances, identity restoration with a bound sidecar and reporting of unrecoverable identities, byte reproduction of the listed byte-reproducible models under a bound sidecar and identical writer settings with formatting collisions reported, determinism, and solve equality of global displacements and reactions against a provisional threshold. |
> | Import unit trace | English and SI imports of the same invented model agree. Their unit handling is checked against an evaluation independent of the importer, and the numerical-meaning changes have independent review. |
> | Import boundary trace | The data-boundary guard fails on any tracked MBF or import report that lacks an invented-provenance sidecar, and the private regression check reads only the configured exchange folder. |
> | Dependency trace | `Dependencies.csv` validates as v3.1. |
>

### CLM-011 — Acceptance Criteria

> ##### Acceptance Criteria
>
> | Criterion | Acceptance treatment |
> |---|---|
> | Source basis mapping | Each CAEPIPE-specific requirement is traced to DEL-17-01, DEL-17-02, an admitted public MBF reference, or an explicit `TBD` closure path. |
> | Target version closure | REQ-001 remains unresolved until the first CAEPIPE version/profile is source-confirmed or human-approved as a bounded profile decision. |
> | Record subset closure | REQ-002 remains unresolved until MBF record families and required fields are enumerated from admitted evidence. |
> | Stable ID preservation | REQ-005 is accepted only when direct MBF carrying is source-confirmed or the writer uses manifest-referenced sidecar mapping. |
> | Loss category coverage | REQ-004 is accepted only when exported, omitted, approximated, delegated, unsupported, and `TBD` behavior are all visible in the loss report. |
> | Diagnostic classification | `TBD-17-04-004` is addressed for this foundation: malformed unsupported refs, missing unsupported loss coverage, and `info`-severity unsupported losses block; explicit `warning` or `blocking` unsupported losses are accepted as classification evidence. |
> | Profile-basis guardrail | REQ-010 is accepted when diagnostics block blank or unsupported `target_version_basis`, `record_subset_basis`, or carried-TBD evidence while the foundation remains source-bounded. |
> | Source-basis guardrail | REQ-011 is accepted when diagnostics and schema validation block weakened CAEPIPE source-basis refs while preserving DEL-17-03 only as historical implementation-pattern evidence. |
> | Fixture provenance | REQ-007 is accepted only with invented fixtures and no vendor, proprietary, protected, or owner/project examples. |
> | Foundation implementation | REQ-008 and REQ-009 are accepted when schema, builder, fixture, and tests demonstrate sidecar-first IDs, deterministic MBF text, and blocking diagnostics for incomplete packages. |
> | Import subset and refusals | REQ-012, REQ-013 and REQ-015 are accepted when the Design lists the first-tranche subset with its sources, and tests show every listed item read, every named refusal blocking, and every loss entry and pass-through option reported without entering SWBPIPE logic. |
> | Import units | REQ-014 is accepted when tests cover both unit modes, every documented English length form, the `RAD` option, both exact conversions, and the named refusals, with English and SI imports of one invented model agreeing, and the SI writer's refusal of an English import is stated in its result. |
> | Import round trip | REQ-016, REQ-017 and REQ-018 are accepted when the following are shown. The Design defines the reversible subset (including validity of every value as written and preservation of distinct record keys), the byte-reproducible subset and their exclusions, and states each tolerance with its derivation, including the writer-admitted bend residuals and the bend-plane comparison by projection perpendicular to the unit chord. Bound imports of the invented pilot-case and writer-test models in the reversible subset pass canonical equality within those tolerances, and so does the invented bend boundary case. Unbound imports of writer output give their named refusals, except where an explicitly retained bounded profile decision admits the records. Bound imports restore the identities and splits the sidecar preserves and report every unrecoverable identity. Determinism, schema validation and solve equality within its provisional threshold pass. Every model listed in the byte-reproducible subset is written again byte for byte under a bound sidecar and identical writer settings. Every formatting collision and exclusion is reported. |
> | Import data boundary | REQ-019 and REQ-020 are accepted when the boundary guard, the material-provenance refusals and the private check's skip and operational-only output are tested publicly, and no CAEPIPE-written MBF or result derived from one is tracked. |
>

### CLM-012 — Downstream Use

> ##### Downstream Use
>
> DEL-17-05 may consume this deliverable later for CAEPIPE external run harness and CSV parser work. DEL-17-05 remains blocked until DEL-17-04 has committed evidence.
>
> DEL-17-05 may also use the importer and its id map to read CAEPIPE-exported MBFs in its private exchange folder and to bind CAEPIPE results to imported ids. A desktop MBF import action would belong to DEL-07-12 and is not part of this deliverable.

- **AC-001** — The contract preserves DEL-17-01 source authority and DEL-17-02 common package/profile/ID-map/loss contracts, a selected-profile gate, deterministic text generation, stable canonical-ID mapping, units and coordinates, pass-through target options without local code logic, invented/rights-cleared fixtures, diagnostics, loss reporting, and unresolved MBF record, field, version, line-ending, encoding, and external-validation questions without claiming broad compatibility or acceptance.
- **AC-002** — The importer reads only its declared subset and refuses or reports everything else by name, including TBD-dependent records in unbound imports. It keeps or exactly converts units, restores the identities a byte-bound sidecar preserves and reports the rest, round-trips the defined reversible subset within derived tolerances, and reproduces the byte-reproducible subset byte for byte. It requires affirmative material provenance, keeps CAEPIPE-written MBFs and everything derived from them private, and makes no compatibility, validation or acceptance claim.

## Production and Verification Method — Praxeology

### CLM-013 — Procedure: DEL-17-04 CAEPIPE MBF export profile and deterministic writer

> #### Procedure: DEL-17-04 CAEPIPE MBF export profile and deterministic writer
>

### CLM-014 — Population Procedure

> ##### Population Procedure
>
> 1. Read DEL-17-01 source-basis dossier and CAEPIPE question dossier.
> 2. Read DEL-17-02 export package/profile/stable-ID/loss-report contract.
> 3. Draft the CAEPIPE MBF profile/writer `ScopeOfWork.md` at contract/design level only.
> 4. Generate `_SEMANTIC.md` with `semantic-matrix-build`.
> 5. Generate `_SEMANTIC_LENSING.md` with `lens-register`.
> 6. Apply warranted P3 enrichment through `four-documents P3_ONLY`.
> 7. Extract dependencies into `Dependencies.csv`.
> 8. Record memory and run evidence.
>

### CLM-015 — Foundation Implementation Procedure

> ##### Foundation Implementation Procedure
>
> Bounded foundation implementation work must:
>
> 1. Close or explicitly carry target version and MBF record-family TBDs.
> 2. Define the deterministic writer subset from admitted evidence.
> 3. Emit a loss report for exported, unsupported, omitted, approximated, delegated, and TBD behavior.
> 4. Preserve stable IDs through manifest-referenced sidecars unless direct MBF carrying is source-confirmed.
> 5. Block blank or unsupported profile-basis values until target version, record subset, and direct stable-ID carrying are closed by admitted source evidence.
> 6. Block missing or unsafe profile source-basis refs so DEL-17-01, DEL-17-02, `CAEPIPE-IMPORT-MBF`, and `CAEPIPE-EXPORT-MBF` remain visible, and DEL-17-03 is not treated as CAEPIPE target/source authority.
> 7. Use invented fixtures only.
> 8. Avoid CAEPIPE compatibility, release, code-compliance, and professional-acceptance claims.
> 9. Keep external execution, CSV parsing, target result interpretation, public API, and GUI integration out of scope unless separately authorized.
>

### CLM-016 — Implementation Readiness Checks

> ##### Implementation Readiness Checks
>
> Before a later writer tranche begins, re-read DEL-17-01, DEL-17-02, the admitted public MBF references, and this `ScopeOfWork.md`. The tranche must record:
>
> 1. Target version/profile closure or explicit carried `TBD`.
> 2. MBF record-family and required-field subset closure or explicit carried `TBD`.
> 3. Stable-ID direct-carry evidence or manifest-referenced sidecar policy.
> 4. Loss-report coverage for exported, omitted, approximated, delegated, unsupported, and `TBD` behavior.
> 5. Blocking versus non-blocking diagnostic classification: unsupported behavior defaults to warning-level evidence unless explicitly blocking, while missing coverage, malformed refs, or `info` severity block.
> 6. Profile-basis diagnostics proving target-version, record-subset, and carried-TBD evidence cannot be silently omitted or weakened.
> 7. Source-basis diagnostics proving required CAEPIPE source authority cannot be silently omitted or replaced by implementation-pattern evidence.
> 8. Invented-fixture provenance.
> 9. Validation results for the deliverable-local document kit, semantic artifacts, dependency schema, and diff hygiene.
>

### CLM-017 — Semantic Enrichment Verification

> ##### Semantic Enrichment Verification
>
> Pass 3 checked `_SEMANTIC_LENSING.md` against DEL-17-01 CAEPIPE TBDs, DEL-17-02 loss-report/stable-ID requirements, and the public MBF source boundary. Applied TP-EXPORT-004R items added source-basis slots, acceptance criteria, classification guidance, pass-through-option rationale, and implementation readiness checks. No P3 item authorized implementation in this tranche.
>
> | Register Item | Disposition |
> |---|---|
> | A-001 | Incorporated through implementation readiness checks for source-confirmed MBF target basis before writer enablement. |
> | A-002 | Incorporated through the future implementation validation checklist for writer determinism, loss reports, and boundary claims. |
> | B-001 | Incorporated as a carried `TBD` for first CAEPIPE target version/profile and public citation target. |
> | B-002 | Incorporated through source and evidence slots for DEL-17-01, DEL-17-02, and public MBF references. |
> | C-001 | Incorporated through acceptance criteria requiring CAEPIPE-specific requirements to map to admitted evidence or explicit `TBD` closure. |
> | C-002 | Incorporated as diagnostic classification guidance and an open question for unsupported-entity severity. |
> | F-001 | Incorporated through profile acceptance criteria for target version, record subset, stable-ID policy, loss categories, and invented fixtures. |
> | F-002 | Incorporated through future implementation loss-report coverage checks. |
> | D-001 | Incorporated through fixture provenance acceptance criteria. |
> | D-002 | Incorporated as an open diagnostic policy question for blocker versus warning behavior. |
> | X-001 | Incorporated through implementation readiness checks with required rereads and validation evidence. |
> | E-001 | Incorporated through pass-through-option rationale in guidance. |

- **VER-001** — Validate the contract and review source parity, source/profile gates, deterministic writer and byte-output requirements, complete behavior classification and loss reporting, stable-ID carrier/sidecar treatment, unit/coordinate and encoding boundaries, fixture provenance, every retained MBF TBD, and absence of bundled/proprietary material, local code-checking logic, compatibility overclaim, or professional authority.
- **VER-002** — Run the importer's grammar, refusal, unit, round-trip, schema and boundary tests on invented data. Review its source citations, exact conversions and canonical comparison with independent scrutiny, and run the private regression check where an exchange folder is configured.

## Governing Values and Decisions — Axiology

### CLM-018 — Guidance: DEL-17-04 CAEPIPE MBF export profile and deterministic writer

> #### Guidance: DEL-17-04 CAEPIPE MBF export profile and deterministic writer
>

### CLM-019 — Design Guidance

> ##### Design Guidance
>
> Treat CAEPIPE MBF as a narrow, source-confirmed target profile. The purpose is not to claim compatibility; it is to define and test what a bounded deterministic writer foundation may safely emit, what it must omit, and what it must report.
>
> The safe posture is conservative: if a record family, required field, stable ID carrier, option, or target behavior is not confirmed by admitted source evidence, mark it `TBD` and require a loss-report or profile diagnostic. The first implementation foundation therefore uses sidecar stable-ID mapping and carries direct MBF ID carrying as `TBD-17-01-003`.
>
> Profile-basis fields are guardrails, not target-support claims. Until later source-confirmed closure exists, blank values or informal target-version/record-subset labels should block the package because they would make unresolved CAEPIPE scope look settled.
>
> Source-basis refs are also guardrails. A caller-provided CAEPIPE MBF profile should not be able to omit DEL-17-01, DEL-17-02, `CAEPIPE-IMPORT-MBF`, or `CAEPIPE-EXPORT-MBF`, and should not promote DEL-17-03 implementation-pattern evidence into CAEPIPE target/source authority.
>

### CLM-020 — Interpretation Guidance

> ##### Interpretation Guidance
>
> - Use DEL-17-01 for CAEPIPE source facts and unanswered questions.
> - Use DEL-17-02 for manifest, profile, ID-map, and loss-report structure.
> - Do not treat MBF text output as equivalent to professional acceptance.
> - Do not turn pass-through target options into local code-checking logic.
> - Do not infer undocumented MBF behavior from the target name or common practice.
> - Treat generated MBF text as a deterministic invented smoke artifact, not as target acceptance evidence.
> - Treat a successful MBF import as a faithful reading of the declared subset of that file, not as evidence that CAEPIPE and SWBPIPE analyse the model alike.
>

### CLM-021 — Classification Guidance

> ##### Classification Guidance
>
> Unsupported target behavior must be classified before implementation as blocking, diagnostic-only, delegated, omitted, approximated, or `TBD`. Until that classification exists, the writer contract should preserve the uncertainty rather than enabling a broad MBF support claim.
>
> For this foundation, explicit unsupported behavior defaults to warning-level loss-report evidence unless the loss entry marks it blocking. Missing unsupported loss coverage, malformed unsupported references, or `info` severity for unsupported behavior are blocking because they would hide material target-scope limits.
>
> Pass-through CAEPIPE options are metadata for the target profile because they may name target-side analysis settings. They must not become SWBPIPE local code-checking logic, local professional acceptance logic, or a substitute for user-supplied rule packs and human review.
>

### CLM-022 — Open Questions

> ##### Open Questions
>
> | TBD | Question | Later closure path |
> |---|---|---|
> | TBD-17-04-001 | Which CAEPIPE version/profile is the first target? | Public documentation review or CAEPIPE developer/support clarification. |
> | TBD-17-04-002 | Which MBF record families and required fields are in the first subset? | Source review and bounded implementation design. |
> | TBD-17-04-003 | Can MBF carry stable canonical IDs directly? | Source review; otherwise sidecar-only mapping. |
> | TBD-17-04-004 | Which unsupported entities block export versus produce non-blocking diagnostics? | This foundation uses warning by default with explicit blocking allowed; later source-confirmed profiles may specialize entity-specific severity. |
> | TBD-17-04-005 | The public import page (`CAEPIPE-IMPORT-MBF`, OPMISC) lists a gravitational acceleration and a gravitational direction field for version 15.10 and later. How is the direction encoded, and what are the defaults of both fields? | DEL-17-01 CQ-17-01-011. Until admitted evidence or an explicit bounded profile decision resolves it, a present value is refused with a named TBD diagnostic in every import; the writer does not write the field. |
> | TBD-17-04-006 | Which text encoding, foot and inch mark characters, and numeric precision does CAEPIPE's MBF export write? | DEL-17-01 CQ-17-01-012. Until closed, the importer reads only strict UTF-8 and the marks the public pages show, refuses other encodings and characters by name, and reads values as written without assuming a precision; no tolerance is widened for export precision. |
> | TBD-17-04-007 | Is the layout comment `L=<load name>` the documented load reference, carried until changed, which load case does a force or moment without `FLC` join, and are both stable across versions? | DEL-17-01 CQ-17-01-013. Bound imports apply the writer's conventions. Unbound imports refuse `L=` and FLC-less FOR/MOM with `MBF-IMPORT-TBD-17-04-007` pending documentation. Unbound imports may admit both where an explicitly retained bounded profile decision does so. |
> | TBD-17-04-008 | Does CAEPIPE's MBF export keep node numbers, row order and tangent-intersection bend rows as imported, and does it write offsets or absolute coordinates? | DEL-17-01 CQ-17-01-014. Until closed, an unbound sidecar is applied only on explicit request and every identity it restores is reported as unconfirmed; geometry is read only in its documented forms. |
> | TBD-17-04-009 | Does the absolute-coordinate marker `*` precede or follow the node number in an MBF layout row? The public sources disagree. | DEL-17-01 CQ-17-01-015. Until closed, the profile selects the trailing marker shown in MBF examples; any other placement is refused by name. |
>

### CLM-023 — Foundation Guidance

> ##### Foundation Guidance
>
> The first implementation foundation should remain sidecar-first and loss-report-first:
>
> - deterministic text output is acceptable only for the invented smoke subset;
> - every canonical identity must appear in a sidecar mapping or an explicit loss entry;
> - every unsupported entity must be a stable reference and must have an explicit unsupported loss entry;
> - target-version, record-subset, and direct stable-ID uncertainty must remain carried as `TBD-17-01-001`, `TBD-17-01-002`, and `TBD-17-01-003`;
> - missing smoke-subset inputs, sidecar IDs, or loss reports should block the package;
> - missing, blank, or weakened profile-basis evidence should block the package;
> - missing or unsafe CAEPIPE source-basis refs should block the package;
> - external execution, CSV parsing, target result interpretation, and target code/check logic remain out of scope.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-030 SOW-074 SOW-075 OBJ-009 OBJ-017 OBJ-018 | CLM-007 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
| OUT-002 | SOW-030 SOW-075 OBJ-009 OBJ-017 OBJ-018 | CLM-008, REQ-012 to REQ-020 | AC-002 | VER-002 | Importer tests on invented data, independent review of numerical meaning, and private regression where configured |

## Product references

- **CAEPIPE-IMPORT-MBF:** `https://www.sstusa.com/docs/users_manual/import_mbf_print.htm` - Official/public CAEPIPE MBF import, MBF format, and command-line CSV result behavior reference.
- **CAEPIPE-EXPORT-MBF:** `https://www.sstusa.com/docs/users_manual/export_mbf.htm` - Official/public CAEPIPE MBF export reference.

Source: `_REFERENCES.md` in DEL-17-04 at `6228147ac5` (before folder migration).
