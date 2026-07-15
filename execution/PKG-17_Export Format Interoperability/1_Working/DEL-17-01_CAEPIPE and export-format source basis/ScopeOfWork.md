---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-17-01
package_id: PKG-17
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@e8f59a63372f38d9e788ac39b39995558f5aba73
project_scope_refs: [SOW-030, SOW-074, SOW-075]
package_objective_refs: [OBJ-009, OBJ-017, OBJ-018]
---

# Scope of Work — DEL-17-01

## Purpose and Objective Traceability

This Scope of Work defines `DEL-17-01` in service of project scope [SOW-030, SOW-074, SOW-075] and package objectives [OBJ-009, OBJ-017, OBJ-018].

- **OUT-001** — A governed CAEPIPE and export-format source-basis contract with admitted sources, source-grounded findings, downstream-consumption rules, and an explicit unanswered-question register is produced.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-17-01 CAEPIPE and export-format source basis

> #### Datasheet: DEL-17-01 CAEPIPE and export-format source basis
>

### CLM-002 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | DeliverableID | DEL-17-01 |
> | PackageID | PKG-17 |
> | Name | CAEPIPE and export-format source basis |
> | Type | DOC_UPDATE |
> | Scope Items | SOW-030, SOW-074, SOW-075 |
> | Objectives | OBJ-009, OBJ-017, OBJ-018 |
> | Lifecycle Role | First source-basis deliverable for the SCA-004 export interoperability workflow |
>

### CLM-003 — Purpose

> ##### Purpose
>
> This deliverable records the admitted source basis for later export-format work. It is not an exporter, schema, implementation, compatibility claim, release claim, or professional validation artifact.
>
> Later `DEL-17-*` deliverables must consume this source basis before making target-format assumptions about CAEPIPE MBF, CAEPIPE batch execution, CAEPIPE CSV results, PCF translation, GLB/glTF review geometry, or adapter SDK behavior.
>

### CLM-004 — Admitted Source Set

> ##### Admitted Source Set
>
> | Source ID | Source | Use in PKG-17 |
> |---|---|---|
> | PLAN-EXPORT-INTEROP | `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md` | Human strategic basis for export priorities, boundaries, and target ordering. |
> | CAEPIPE-IMPORT-MBF | Official CAEPIPE MBF import/manual page; `IMPORT MBF`, `Command line operation`, `Format of .MBF` | Public evidence for MBF as a text model input path and command-line behavior. |
> | CAEPIPE-EXPORT-DATA | Official CAEPIPE model/result export page; `Export data from CAEPIPE` | Public evidence for model/result export surfaces including CSV/text-style outputs. |
> | CAEPIPE-EXPORT-MBF | Official CAEPIPE MBF export page; `EXPORT MBF` | Public evidence that CAEPIPE can export model data to MBF. |
> | CAEPIPE-BATCH | Official CAEPIPE batch-mode page; `RUNNING CAEPIPE/CAEPIPE 3D+ IN BATCH MODE` | Public evidence for external executable invocation constraints. |
> | CAEPIPE-PCF | Official CAEPIPE PCF translator document; `Reference`, `PCF to CAEPIPE component Mapping` | Public evidence for PCF translation limits, mappings, defaults, and translator-dependent behavior. |
> | GLTF-2.0 | Khronos glTF 2.0 specification; `2.4 glTF Basics`, `3.2 Asset`, `3.3 Indices and Names`, `3.4 Coordinate System and Units`, `4 GLB File Format Specification` | Public evidence for review-geometry export format semantics. |
> | CONTRACT / IP-DATA / SPEC | Project governance and technical docs | Binding constraints for no protected data, no professional claim, no hidden defaults, and no-bypass adapter boundaries. |
>

### CLM-005 — Source-Basis Findings

> ##### Source-Basis Findings
>
> | Finding ID | Finding | Source Basis | Status |
> |---|---|---|---|
> | F-17-01-001 | CAEPIPE MBF is the preferred first solver-specific target because it is a documented text model input/output path and is closer to CAEPIPE model data than PCF translation. | PLAN-EXPORT-INTEROP location TBD; CAEPIPE-IMPORT-MBF `IMPORT MBF` / `Format of .MBF`; CAEPIPE-EXPORT-MBF `EXPORT MBF` | ADMITTED |
> | F-17-01-002 | CAEPIPE external execution may be supported only as an optional user-owned harness using a user-provided executable and license. | PLAN-EXPORT-INTEROP location TBD; CAEPIPE-IMPORT-MBF `Command line operation`; CAEPIPE-BATCH `RUNNING CAEPIPE/CAEPIPE 3D+ IN BATCH MODE`; IP-DATA `6. Private user data` | ADMITTED_WITH_BOUNDARY |
> | F-17-01-003 | CAEPIPE CSV or text results may be parsed for regression/handoff evidence, but parsed outputs remain non-authoritative and do not become professional acceptance. | PLAN-EXPORT-INTEROP location TBD; CAEPIPE-EXPORT-DATA `Export data from CAEPIPE`; CAEPIPE-BATCH `Run the Analysis and Output Results in CSV Format`; CONTRACT `1.9 Provenance and Epistemic Integrity` | ADMITTED_WITH_BOUNDARY |
> | F-17-01-004 | PCF is useful for broader interoperability, but CAEPIPE PCF translation has mapping/default behavior that makes it unsuitable as the first deterministic exchange backbone. | PLAN-EXPORT-INTEROP location TBD; CAEPIPE-PCF `Reference`; `PCF to CAEPIPE component Mapping` | ADMITTED_WITH_LIMIT |
> | F-17-01-005 | GLB/glTF is a review-geometry target, not solver geometry or solver-fidelity evidence. | GLTF-2.0 `2.4 glTF Basics`; `3.2 Asset`; `3.3 Indices and Names`; `3.4 Coordinate System and Units`; PLAN-EXPORT-INTEROP location TBD | ADMITTED_WITH_BOUNDARY |
> | F-17-01-006 | Later export deliverables must report exported, omitted, approximated, delegated, unsupported, and TBD behavior rather than silently accepting target losses. | PLAN-EXPORT-INTEROP location TBD; CONTRACT `1.9 Provenance and Epistemic Integrity`; DAG-005 `Approval Conditions` / `Immediate Follow-Up Boundary` | ADMITTED |
>

### CLM-006 — Boundary Facts

> ##### Boundary Facts
>
> - No CAEPIPE binary, commercial example, proprietary model file, copied report, or vendor fixture is admitted.
> - No protected standards text, standards-derived tables, code allowables, SIF/flexibility tables, or owner design criteria are admitted.
> - Pass-through target fields may name solver options only as downstream target configuration, not as local code-checking logic.
> - A successful export or external run is regression/handoff evidence only; it is not professional engineering acceptance.
>

### CLM-007 — Downstream Consumers

> ##### Downstream Consumers
>
> | Consumer | Dependency on DEL-17-01 |
> |---|---|
> | DEL-17-02 | Uses this source basis to define common export package/profile/ID-map/loss-report contracts. |
> | DEL-17-04 | Uses this source basis before defining CAEPIPE MBF writer assumptions. |
> | DEL-17-05 | Uses this source basis before defining CAEPIPE external harness and CSV parser behavior. |
> | DEL-17-06 | Uses this source basis before defining automated CAEPIPE CSV/text result parser coverage. |
> | DEL-17-07 | Uses this source basis before defining conservative PCF subset behavior. |
> | DEL-17-08 | Uses this source basis before defining review geometry export assumptions. |
>

### CLM-008 — Open TBD Register

> ##### Open TBD Register
>
> | TBD ID | Question | Blocks |
> |---|---|---|
> | TBD-17-01-001 | Which CAEPIPE version/profile is the first supported target? | DEL-17-04, DEL-17-05 |
> | TBD-17-01-002 | Which MBF record families are required for the first deterministic writer subset? | DEL-17-04 |
> | TBD-17-01-003 | Which MBF fields can safely carry stable canonical IDs, and which require sidecar maps? | DEL-17-02, DEL-17-04 |
> | TBD-17-01-004 | What CSV result sections are stable enough for automated parsing in a first harness? | DEL-17-05, DEL-17-06 |
> | TBD-17-01-005 | What PCF subset is conservative enough to avoid hidden translator defaults? | DEL-17-07 |
> | TBD-17-01-006 | What geometry fields are sufficient for glTF review without implying solver fidelity? | DEL-17-08 |

## Completion and Reliance Basis — Epistemology

### CLM-009 — Specification: DEL-17-01 CAEPIPE and export-format source basis

> #### Specification: DEL-17-01 CAEPIPE and export-format source basis
>

### CLM-010 — Normative Scope

> ##### Normative Scope
>
> DEL-17-01 shall define the source authority, target-behavior boundaries, unanswered-question register, and downstream consumption rules for PKG-17 export-format work.
>
> This deliverable shall not implement exporters, parsers, harnesses, schemas, tests, GUI behavior, persistence behavior, or public API surfaces.
>

### CLM-011 — Source Authority Requirements

> ##### Source Authority Requirements
>
> | Req ID | Requirement |
> |---|---|
> | DEL-17-01-REQ-001 | The deliverable shall use only public, official, project-owned, or otherwise admitted source references listed in `_REFERENCES.md` and `PKG-17/0_References/_REFERENCE_INDEX.md`. |
> | DEL-17-01-REQ-002 | CAEPIPE-specific facts shall be grounded in official/public CAEPIPE documentation or explicitly marked `TBD`. |
> | DEL-17-01-REQ-002A | Every non-trivial governed claim in the source-basis register and CAEPIPE question dossier shall include a source path plus best-effort section/heading reference, or explicit `location TBD`. |
> | DEL-17-01-REQ-003 | PCF-specific facts shall distinguish documented translator behavior from OpenPipeStress export intentions. |
> | DEL-17-01-REQ-004 | glTF/GLB facts shall be limited to review-geometry export semantics and shall not imply solver geometry equivalence. |
> | DEL-17-01-REQ-005 | Later `DEL-17-*` deliverables shall consume this deliverable before making target-format claims. |
>

### CLM-012 — Boundary Requirements

> ##### Boundary Requirements
>
> | Req ID | Requirement |
> |---|---|
> | DEL-17-01-REQ-010 | The source basis shall not include bundled CAEPIPE binaries, copied commercial examples, proprietary model files, reverse-engineered binary behavior, or license-bypass instructions. |
> | DEL-17-01-REQ-011 | The source basis shall not include protected standards text, protected tables, proprietary formulas, standards-derived examples, material allowables, SIF/flexibility tables, or owner criteria. |
> | DEL-17-01-REQ-012 | A CAEPIPE run, import, export, or parsed CSV result shall be described only as non-authoritative regression/handoff evidence unless a later human professional review process separately accepts it. |
> | DEL-17-01-REQ-013 | Target solver options may be recorded as pass-through configuration, but shall not become OpenPipeStress local code-checking logic. |
> | DEL-17-01-REQ-014 | Unknown or version-sensitive target behavior shall remain `TBD` and shall gate implementation claims in downstream deliverables. |
>

### CLM-013 — Required Registers

> ##### Required Registers
>
> | Register | Required Content |
> |---|---|
> | Source basis register | Source ID, source location, admitted use, boundary, downstream consumers. |
> | Finding register | Source-grounded facts separated from assumptions and TBDs. |
> | CAEPIPE question dossier | Questions for CAEPIPE developer/support clarification, current public evidence, affected deliverables, and gating impact. |
> | TBD register | Unanswered target-format behavior, blocked deliverables, and closure route. |
>

### CLM-014 — Downstream Consumption Rules

> ##### Downstream Consumption Rules
>
> - `DEL-17-02` shall use this deliverable as the source authority for common export package/profile/ID-map/loss-report contracts.
> - `DEL-17-04` shall not define a CAEPIPE MBF writer subset until its unsupported, omitted, approximated, delegated, and sidecar-mapped behaviors are traceable to this deliverable.
> - `DEL-17-05` shall not treat CAEPIPE external runs as mandatory or bundled; the executable path and license remain user-owned.
> - `DEL-17-06` shall carry forward CSV/text parser TBDs until stable result sections and parser scope are explicitly selected.
> - `DEL-17-07` shall treat PCF translation as conservative interoperability, not as the first deterministic exchange backbone.
> - `DEL-17-08` shall treat GLB/glTF as visual review geometry only.
>

### CLM-015 — Acceptance Requirements

> ##### Acceptance Requirements
>
> DEL-17-01 is acceptable when:
>
> - the four-document kit exists and passes the local four-document check;
> - source findings cite admitted source IDs;
> - public facts, assumptions, and TBDs are separated;
> - the CAEPIPE developer-team question dossier exists;
> - no protected or proprietary source material is copied into the repository;
> - no compatibility, release, code-compliance, or professional-acceptance claim is made.

- **AC-001** — The contract preserves official/public and project-owned source authority, best-effort locations, CAEPIPE MBF, external-run and CSV boundaries, conservative PCF and review-only GLB/glTF classifications, downstream gating, protected/proprietary-content exclusions, explicit target-behavior TBDs, and professional non-authority without implementing or claiming compatibility.

## Production and Verification Method — Praxeology

### CLM-016 — Procedure: DEL-17-01 CAEPIPE and export-format source basis

> #### Procedure: DEL-17-01 CAEPIPE and export-format source basis
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-017 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-17-01-DECL-004`.
>

### CLM-018 — Procedure Purpose

> ##### Procedure Purpose
>
> This procedure defines how to maintain and use the DEL-17-01 source basis before downstream export deliverables are populated or implemented.
>

### CLM-019 — Source Intake

> ##### Source Intake
>
> 1. Confirm that a proposed source is listed in `_REFERENCES.md` or `PKG-17/0_References/_REFERENCE_INDEX.md`.
> 2. Classify the source as one of:
>    - official/public vendor documentation;
>    - public standards/specification documentation for an exchange format;
>    - project-owned governance/schema/plan material;
>    - user/private material excluded from public repository content.
> 3. Reject or quarantine any source that appears to include proprietary examples, protected standards data, private project data, or commercial files without redistribution rights.
> 4. Record admitted use and boundary in `Source_Basis_Register.md`.
>

### CLM-020 — Finding Extraction

> ##### Finding Extraction
>
> 1. Extract only source-grounded facts that are needed by downstream export work.
> 2. Paraphrase vendor/public documentation rather than copying long excerpts.
> 3. Attach every finding to one or more source IDs.
> 4. Attach a best-effort source section/heading reference; if the location cannot be narrowed during the run, record `location TBD`.
> 5. Mark any inference as `INFERENCE`, not as a source fact.
> 6. Mark unresolved target behavior as `TBD`.
>

### CLM-021 — CAEPIPE Question Dossier Maintenance

> ##### CAEPIPE Question Dossier Maintenance
>
> 1. Add a question when public documentation is insufficient for deterministic exporter or harness behavior.
> 2. Include the affected downstream deliverables.
> 3. Include the current public evidence basis.
> 4. State the gating impact.
> 5. Do not request proprietary internals, protected standards content, commercial examples, or license-bypass instructions.
> 6. When a question is answered, record the answer source and update affected TBD entries.
>

### CLM-022 — Downstream Consumption

> ##### Downstream Consumption
>
> Before a downstream `DEL-17-*` deliverable is populated or modified:
>
> 1. Read this four-document kit.
> 2. Read `Source_Basis_Register.md`.
> 3. Read `CAEPIPE_Question_Dossier.md`.
> 4. Confirm all relevant TBDs are either closed or explicitly carried forward.
> 5. Confirm the downstream deliverable is named in the source-basis register, a dependency register, or a current package plan before treating DEL-17-01 as a gating input.
> 6. Confirm the downstream deliverable consumes only the relevant source-basis findings, not a blanket claim that every DEL-17 item consumes every target-format source.
> 7. Preserve the professional/IP/export boundaries from this deliverable.
>

### CLM-023 — Validation

> ##### Validation
>
> Run:
>
> ```text
> tools/validation/check_four_documents.sh execution/PKG-17_Export\ Format\ Interoperability/1_Working/DEL-17-01_CAEPIPE\ and\ export-format\ source\ basis
> tools/validation/check_min_viable_fileset.sh execution/PKG-17_Export\ Format\ Interoperability/1_Working/DEL-17-01_CAEPIPE\ and\ export-format\ source\ basis
> git diff --check -- "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-01_CAEPIPE and export-format source basis"
> ```
>
> If an unscoped worktree check reports findings outside this project's write scope, record the finding as external-scope noise and do not treat it as a blocker for DEL-17-01 closeout. Scoped deliverable checks remain the controlling diff-hygiene evidence for this deliverable.
>
> Manual review must check:
>
> - no proprietary examples;
> - no copied protected standards data;
> - no CAEPIPE compatibility overclaims;
> - no code-compliance or professional-acceptance claims;
> - admitted source pointers are still reachable or are marked stale/location TBD before reuse;
> - unresolved target behavior remains marked `TBD`.
>

### CLM-024 — Semantic Enrichment Verification

> ##### Semantic Enrichment Verification
>
> Pass 3 semantic-lensing enrichment checked `_SEMANTIC_LENSING.md` items `C-001`, `D-001`, and `E-001` against `Datasheet.md` downstream consumers, `Procedure.md` downstream consumption and validation checks, `_REFERENCES.md` package references, and `Source_Basis_Register.md` finding/downstream-use rows before applying the warranted operational updates.
>
> Later relevant downstream deliverables must cite admitted source IDs, carry forward unresolved questions, and report exported, omitted, approximated, delegated, unsupported, and TBD behavior without converting those records into compatibility, release, code-compliance, or professional-acceptance claims.

- **VER-001** — Validate the contract and review source parity, admitted-source and finding registers, source-location discipline, downstream-consumer gates, all CAEPIPE/PCF/GLB boundaries, the complete unanswered-question register, protected-content exclusions, and absence of implementation, compatibility, release, code-compliance, or professional-acceptance claims.

## Governing Values and Decisions — Axiology

### CLM-025 — Guidance: DEL-17-01 CAEPIPE and export-format source basis

> #### Guidance: DEL-17-01 CAEPIPE and export-format source basis
>

### CLM-026 — Working Guidance

> ##### Working Guidance
>
> Use DEL-17-01 as a source authority and guardrail document. Its value is not in deciding the final exporter design; its value is preventing downstream exporter work from inventing target behavior or silently crossing IP/professional boundaries.
>

### CLM-027 — How to Use the Source Basis

> ##### How to Use the Source Basis
>
> - Treat official/public vendor documentation as evidence for documented interfaces only.
> - Treat `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md` as the accepted human-authored strategy, not as a substitute for target-format details.
> - Treat project governance docs as binding constraints whenever source material is silent, ambiguous, or tempting to overstate.
> - Treat any unsupported, version-sensitive, undocumented, or inferred target behavior as `TBD`.
>

### CLM-028 — CAEPIPE Guidance

> ##### CAEPIPE Guidance
>
> CAEPIPE is the first CAEPIPE-focused source-basis target because the admitted references support a text model handoff and optional external execution workflow. This does not make OpenPipeStress a CAEPIPE replacement, does not bundle CAEPIPE, and does not create a claim that exported models are accepted engineering work.
>
> For downstream work:
>
> - prefer MBF over PCF for the first deterministic CAEPIPE exchange backbone;
> - keep all CAEPIPE-specific assumptions versioned and profile-scoped;
> - require loss reports for unsupported or approximate target behavior;
> - keep executable paths, licenses, and execution environments user-owned;
> - record parsed CSV results as regression/handoff evidence only.
>

### CLM-029 — PCF Guidance

> ##### PCF Guidance
>
> PCF should be treated as broader interoperability, not the first exchange backbone. The CAEPIPE PCF documentation indicates translator-dependent mappings and defaults. Downstream PCF work should therefore begin from a conservative subset and report limitations explicitly.
>

### CLM-030 — GLB/glTF Guidance

> ##### GLB/glTF Guidance
>
> GLB/glTF should be used for visual review and lightweight geometry inspection. It should not be described as solver input, stress-model proof, or professional acceptance evidence. Any stable ID mapping that cannot be carried directly in the geometry file should be preserved in sidecar records.
>

### CLM-031 — Question-Dossier Guidance

> ##### Question-Dossier Guidance
>
> The CAEPIPE developer-team question dossier should ask for clarification only where public documentation is insufficient for deterministic, supportable behavior. Questions should avoid requesting proprietary internals, reverse-engineered behavior, licensed examples, or protected code/compliance content.
>
> Good questions are about:
>
> - supported public interface behavior;
> - version/profile expectations;
> - allowed command-line invocation shape;
> - stable CSV sections suitable for automated parsing;
> - MBF field/record interpretation boundaries;
> - recommended citation targets for public documentation.
>
> Poor questions are about:
>
> - proprietary solver algorithms;
> - hidden translator internals;
> - commercial example files;
> - code-compliance calculations;
> - protected standards content;
> - license-bypass behavior.
>

### CLM-032 — Review Guidance

> ##### Review Guidance
>
> Reviewers should reject this deliverable if it:
>
> - overstates CAEPIPE compatibility;
> - treats PCF translation as deterministic without qualification;
> - implies successful export equals engineering acceptance;
> - embeds proprietary examples or protected standards data;
> - fails to mark unknown target behavior as `TBD`;
> - lets later DEL-17 deliverables proceed without consuming this source basis.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-030 SOW-074 SOW-075 OBJ-009 OBJ-017 OBJ-018 | CLM-009 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
