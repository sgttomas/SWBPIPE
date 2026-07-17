---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-17-05
package_id: PKG-17
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@e8f59a63372f38d9e788ac39b39995558f5aba73
project_scope_refs: [SOW-030, SOW-046, SOW-075]
package_objective_refs: [OBJ-007, OBJ-009, OBJ-017, OBJ-018]
---

# Scope of Work — DEL-17-05

## Purpose and Objective Traceability

This Scope of Work defines `DEL-17-05` in service of project scope [SOW-030, SOW-046, SOW-075] and package objectives [OBJ-007, OBJ-009, OBJ-017, OBJ-018].

- **OUT-001** — An optional user-owned CAEPIPE external-run harness and bounded CSV-parser contract for regression and handoff evidence is produced.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-17-05 CAEPIPE external run harness and CSV parser

> #### Datasheet: DEL-17-05 CAEPIPE external run harness and CSV parser
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-17-05-DECL-002`.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-17-05 |
> | Package ID | PKG-17 |
> | Package | Export Format Interoperability |
> | Name | CAEPIPE external run harness and CSV parser |
> | Type | BACKEND_FEATURE_SLICE |
> | Scope Items | SOW-030, SOW-046, SOW-075 |
> | Objectives | OBJ-007, OBJ-009, OBJ-017, OBJ-018 |
> | Current phase | Phase A four-document contract only |
> | Upstream dependency | DEL-17-04 CAEPIPE MBF export profile and deterministic writer |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Required treatment |
> |---|---|
> | Harness role | Optional external execution wrapper around a user-provided, licensed CAEPIPE executable. |
> | Execution ownership | User-owned executable path, license, operating system, working directory, permissions, and any compatibility/remote Windows environment. |
> | Bundled executable | Prohibited. No CAEPIPE binary, installer, commercial example, or license-bypass mechanism is part of this deliverable. |
> | Input target | CAEPIPE MBF emitted by DEL-17-04 or a later accepted implementation tranche; exact supported MBF profile remains `TBD`. |
> | Expected target output | CAEPIPE CSV result file when the user-owned external run produces one; output filename/location behavior is source-supported but invocation profile details remain `TBD`. |
> | CSV parser role | Regression and handoff evidence extraction only. Parsed CSV rows are not professional acceptance, code compliance, formal validation, or proof of CAEPIPE compatibility. |
> | Stable identity | Correlation to OpenPipeStress canonical IDs shall use DEL-17-02/DEL-17-04 manifest and sidecar ID-map evidence where possible; direct CSV identity coverage remains `TBD`. |
> | Test posture | Automated tests without a configured executable shall skip with an explicit reason; parser-only tests shall use invented or rights-cleared fixtures. |
>

### CLM-005 — Conditions

> ##### Conditions
>
> | Condition | Source/evidence basis | Status |
> |---|---|---|
> | CAEPIPE can read MBF text input and create a model file through import. | `CAEPIPE-IMPORT-MBF`, official CAEPIPE import page, lines 0-23. | Source-supported. |
> | Command-line or batch execution can produce CSV output from an MBF input. | `CAEPIPE-IMPORT-MBF`, lines 15-20; `CAEPIPE-BATCH`, lines 40-52. | Source-supported with invocation-profile `TBD`. |
> | CAEPIPE can output model and/or result data to CSV/text through print/export surfaces. | `CAEPIPE-EXPORT-DATA`, lines 31-43. | Source-supported. |
> | CAEPIPE can export model data to MBF from the Layout window. | `CAEPIPE-EXPORT-MBF`, lines 28-39. | Source-supported for source-basis context. |
> | First supported CAEPIPE version/profile. | DEL-17-01 `TBD-17-01-001`; DEL-17-04 `TBD-17-04-001`. | `TBD`. |
> | Stable CSV sections suitable for automated parsing. | DEL-17-01 `TBD-17-01-004`; CQ-17-01-005. | `TBD`. |
> | Recommended command-line invocation shape for the first harness profile. | CQ-17-01-004; public pages describe related but not fully reconciled invocation patterns. | `TBD`. |
>

### CLM-006 — Construction

> ##### Construction
>
> | Artifact | Construction rule |
> |---|---|
> | External run harness | Future implementation shall be opt-in, disabled without user configuration, and bounded to a user-provided executable path. This Phase A document does not implement it. |
> | Run directory contract | Future implementation shall record an auditable run directory concept containing the MBF input, manifest/ID-map/loss-report evidence, target CSV output if produced, execution metadata, stdout/stderr where available, and parser diagnostics. Exact filenames remain implementation-level `TBD` unless already fixed by DEL-17-02/DEL-17-04. |
> | CSV parser | Future implementation shall parse only documented or fixture-confirmed CSV sections and shall report unrecognized, missing, or unstable sections instead of silently accepting them. |
> | Run metadata | Future implementation shall record executable path provenance without copying or redistributing the executable, target version/profile if discoverable, command shape used, timestamps or hashes according to the manifest policy, exit status, output discovery, parser status, and boundary notes. |
> | Evidence records | Future implementation shall classify evidence as regression/handoff evidence, not formal validation or professional approval. |
> | Fixtures | Public fixtures shall be invented or rights-cleared. User-provided CAEPIPE outputs may be used only under user-controlled/private handling unless redistribution rights are documented. |
>

### CLM-007 — References

> ##### References
>
> | Source ID | Source | Use |
> |---|---|---|
> | DEL-17-01 | `../DEL-17-01_CAEPIPE and export-format source basis/` | Source authority, CAEPIPE questions, TBD register, and boundary facts. |
> | DEL-17-02 | `../DEL-17-02_Export package, profile, and stable ID map contracts/` | Export manifest, stable-ID map, loss-report, profile, and external-execution policy. |
> | DEL-17-04 | `../DEL-17-04_CAEPIPE MBF export profile and deterministic writer/` | Upstream CAEPIPE MBF profile/writer contract and unresolved target-profile gates. |
> | PLAN-EXPORT-INTEROP | `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md` | Strategic source for optional external harness, run directory concept, and boundary framing. |
> | CAEPIPE-IMPORT-MBF | `https://www.sstusa.com/docs/users_manual/import_mbf_print.htm` | Public evidence for MBF import and command-line CSV behavior. |
> | CAEPIPE-EXPORT-DATA | `https://www.sstusa.com/docs/users_manual/export_data_from_caepipe.htm` | Public evidence for CAEPIPE CSV/text result and model export surfaces. |
> | CAEPIPE-BATCH | `https://www.sstusa.com/docs/users_manual/running_caepipe_caepipe_3d__in_batch_mode.htm` | Public evidence for batch-mode CSV output behavior and path/invocation caveats. |
> | CAEPIPE-EXPORT-MBF | `https://www.sstusa.com/docs/users_manual/export_mbf.htm` | Public evidence inherited through DEL-17-01/DEL-17-04 source basis for CAEPIPE MBF export from model data; this source is not listed as a direct DEL-17-05 package reference in `_REFERENCES.md`. |
> | CONTRACT | `docs/CONTRACT.md` | Project invariants for IP, authority boundary, unit-aware exports, agent TBD handling, and professional claims. |
> | IP-DATA | `docs/IP_AND_DATA_BOUNDARY.md` | Public/private data and protected-content boundaries. |

### CLM-008 — D-41 R5 T3 PDU-016 Evidence State

> ##### D-41 R5 T3 PDU-016 Evidence State
>
> Selected seam: CAEPIPE external-run/parser evidence. Default: private user-controlled, local-only, telemetry false. Scope excludes whole-product security assurance.

## Completion and Reliance Basis — Epistemology

### CLM-009 — Specification: DEL-17-05 CAEPIPE external run harness and CSV parser

> #### Specification: DEL-17-05 CAEPIPE external run harness and CSV parser
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-010 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-17-05-DECL-001`, `DEL-17-05-EXC-001`, `DEL-17-05-EXC-002`.
>

### CLM-011 — Scope

> ##### Scope
>
> DEL-17-05 shall define the Phase A contract for an optional CAEPIPE external run harness and CAEPIPE CSV parser used as regression and handoff evidence.
>
> This deliverable covers:
>
> - user-owned external executable boundary;
> - run directory and execution metadata requirements;
> - CSV parser coverage boundaries;
> - evidence classification for parsed CAEPIPE output;
> - skip behavior when no executable is configured;
> - carried `TBD` items for invocation profile, target version/profile, and stable CSV sections.
>
> This Phase A deliverable shall not implement code, schemas, parser logic, target fixtures, GUI workflow, public API endpoints, CAEPIPE executable discovery, CAEPIPE bundling, license validation bypasses, release behavior, compatibility claims, code-compliance claims, professional acceptance claims, or formal validation claims.
>

### CLM-012 — Requirements

> ##### Requirements
>
> | Req ID | Requirement |
> |---|---|
> | DEL-17-05-REQ-001 | The harness shall be optional and disabled unless a user supplies a local executable path and accepts responsibility for license and execution environment. |
> | DEL-17-05-REQ-002 | The project shall not bundle, download, install, redistribute, wrap for license bypass, or imply availability of CAEPIPE or CAEPIPE 3D+. |
> | DEL-17-05-REQ-003 | The first executable invocation profile shall remain `TBD` until public documentation and/or CAEPIPE support clarification reconciles the command-line and batch-mode patterns. |
> | DEL-17-05-REQ-004 | The harness shall record the command shape actually used, working directory, input MBF path, expected CSV path, observed CSV path, exit status if available, stdout/stderr capture if available, and diagnostics. |
> | DEL-17-05-REQ-005 | The harness shall treat the MBF input as an output of the accepted DEL-17-04 profile/writer tranche; if the required MBF profile is unresolved, the run record shall carry that `TBD`. |
> | DEL-17-05-REQ-006 | The run directory contract shall include or reference the export manifest, ID map, loss report, MBF input, CSV output when produced, and CAEPIPE run metadata. |
> | DEL-17-05-REQ-007 | The harness shall verify only operational evidence: input file presence, process invocation attempt, CSV discovery, parser execution, expected section/count checks when source-confirmed, and diagnostics. |
> | DEL-17-05-REQ-008 | The harness shall not classify a successful CAEPIPE run or parsed CSV output as professional acceptance, code compliance, formal validation, or compatibility proof. |
> | DEL-17-05-REQ-009 | The CSV parser shall parse only source-confirmed or fixture-confirmed sections and shall mark unsupported, unrecognized, unstable, missing, or unmapped sections as diagnostics or `TBD`. |
> | DEL-17-05-REQ-010 | Stable result correlation shall use canonical IDs through DEL-17-02/DEL-17-04 sidecars and manifests where possible; any correlation by target row order or names alone shall be marked weak or `TBD`. |
> | DEL-17-05-REQ-011 | Parser results shall be bound to source model/export/run metadata before use as regression or handoff evidence. |
> | DEL-17-05-REQ-012 | Tests that require the external executable shall skip when no executable path is configured and shall report the skip reason without failing normal public CI. |
> | DEL-17-05-REQ-013 | Public parser fixtures shall be invented, project-owned, public/permissive, or rights-cleared; proprietary CAEPIPE examples, private user projects, and protected standards-derived data shall not be committed. |
> | DEL-17-05-REQ-014 | User-provided CAEPIPE CSV outputs shall default to private/user-controlled handling unless provenance and redistribution rights are documented. |
> | DEL-17-05-REQ-015 | Diagnostics shall preserve unit, coordinate, ID-map, loss-report, and parser-coverage uncertainty instead of silently normalizing or accepting target output. |
> | DEL-17-05-REQ-016 | External-run evidence shall bind to a DEL-17-04 CAEPIPE MBF export package reference, and attempted external execution shall block unless both license responsibility and user-owned environment responsibility are acknowledged. |
>

### CLM-013 — Required Placeholders for Later Implementation

> ###### Required Placeholders for Later Implementation
>
> These fields do not close the carried CAEPIPE TBDs; they define the evidence shape a later implementation tranche must fill before claiming repeatable harness behavior.
>
> | Placeholder ID | Required placeholder | Source basis |
> |---|---|---|
> | DEL-17-05-PH-001 | Configuration surface for the executable path: `TBD` until a later tranche selects the environment variable, config key, CLI option, GUI field, or equivalent user-owned input. | REQ-001; `docs/IP_AND_DATA_BOUNDARY.md`; `CAEPIPE-IMPORT-MBF` command-line operation, lines 15-20; `CAEPIPE-BATCH` batch-mode notes, lines 40-52. |
> | DEL-17-05-PH-002 | Run-directory record shape: `TBD` filenames or manifest links for MBF input, export manifest, ID map, loss report, target CSV artifact, run metadata, and parser diagnostics. | REQ-004/006; DEL-17-02 export package contract; DEL-17-04 MBF profile contract. |
> | DEL-17-05-PH-003 | Parser coverage register shape: section name, source-or-fixture basis, supported status, expected row/field handling where known, unmapped-row handling, diagnostic severity, and fixture provenance. | REQ-009/013/015; DEL-17-01 `TBD-17-01-004`; `CAEPIPE-EXPORT-DATA`, lines 31-43. |
> | DEL-17-05-PH-004 | Skip-without-executable evidence fields: configuration key checked, configured-path absence or invalidity, skip reason, public-CI nonfailure classification, parser-only tests still executed, and boundary note. | REQ-001/012; package exclusion against bundled solvers or license bypass. |
>

### CLM-014 — TBD Closure Gates

> ###### TBD Closure Gates
>
> | Gate | Closure condition |
> |---|---|
> | Invocation profile | REQ-003 may move out of `TBD` only after support clarification, an explicitly selected public command/batch pattern, or a documented project-owned adapter decision reconciles the command-line and batch-mode source patterns. |
> | Parser section coverage | REQ-009 may move out of `TBD` for a CSV section only when the section is confirmed by public/official source text or by an invented/rights-cleared fixture with documented provenance and expected parser behavior. |
> | First live MBF profile | REQ-005 may move out of `TBD` only when DEL-17-04 identifies the accepted MBF profile/writer tranche that the harness is allowed to consume. |
>

### CLM-015 — Standards

> ##### Standards
>
> | Source | Requirement treatment |
> |---|---|
> | `docs/CONTRACT.md` | OPS-K-IP-1 through OPS-K-IP-3, OPS-K-DATA-2, OPS-K-AUTH-1, OPS-K-UNIT-1, OPS-K-REPORT-1/2, OPS-K-PRIV-1, OPS-K-AGENT-1 through OPS-K-AGENT-4 apply. |
> | `docs/IP_AND_DATA_BOUNDARY.md` | Public repository content must avoid protected standards text/tables, proprietary examples, private user data, and commercial software artifacts without redistribution rights. |
> | DEL-17-01 source basis | CAEPIPE-specific facts require official/public CAEPIPE evidence or explicit `TBD`; external runs and parsed CSVs remain non-authoritative regression/handoff evidence. |
> | DEL-17-02 export contract | Export packages require manifest, stable-ID map, loss report, boundary notes, and external execution as optional user-owned metadata. |
> | DEL-17-04 MBF profile | CAEPIPE version/profile, MBF record subset, stable-ID strategy, and unsupported-entity severity remain carried gates for the harness. |
> | Official CAEPIPE public documentation | Evidence supports MBF import/export, CSV output surfaces, and batch/command-line CSV production, but does not close parser-section coverage or OpenPipeStress compatibility. |
>

### CLM-016 — Verification

> ##### Verification
>
> | Verification ID | Verification approach |
> |---|---|
> | VER-001 | Four-document review confirms all default sections exist and all CAEPIPE-specific claims cite DEL-17-01 source IDs, DEL-17-02/DEL-17-04 contracts, or public CAEPIPE pages. |
> | VER-002 | Boundary review confirms no bundled executable, proprietary example, protected standards data, license-bypass workflow, compatibility claim, release claim, code-compliance claim, professional claim, or formal validation claim. |
> | VER-003 | TBD review confirms target version/profile, invocation profile, stable CSV sections, and stable ID correlation are not overstated. |
> | VER-004 | Future implementation review shall confirm skip-without-executable behavior and no external executable requirement in public CI, including the configuration field checked, absence/invalidity evidence, skip reason, parser-only continuation, and public-CI nonfailure classification. |
> | VER-005 | Future parser tests shall use invented or rights-cleared CSV fixtures and shall record parser coverage by section name, source-or-fixture basis, supported status, unmapped-row handling, diagnostic severity, and fixture provenance. |
> | VER-006 | Future harness tests with a user-provided executable shall remain opt-in and record run metadata, output discovery, parser diagnostics, and evidence classification. |
> | VER-007 | Future regression comparison shall bind parsed rows to canonical IDs or explicitly mark weak/unmapped evidence. |
> | VER-008 | Future acceptance review shall confirm that the user-supplied executable path, license/environment responsibility, and selected configuration surface are recorded before any live external execution occurs. |
>

### CLM-017 — Documentation

> ##### Documentation
>
> Required Phase A documentation artifacts are:
>
> - `Datasheet.md`;
> - `Specification.md`;
> - `Guidance.md`;
> - `Procedure.md`;
> - `_STATUS.md` safe update from `OPEN` to `INITIALIZED` if the P1/P2 run completes;
> - `_run_records/TASK_RUN_*.md`.
>
> Future implementation documentation shall include:
>
> - harness configuration guidance that states executable path, license, and execution environment are user-owned;
> - run directory contract;
> - CSV parser coverage register;
> - skip-without-executable test behavior;
> - fixture provenance notes;
> - evidence and limitation notes for reports or handoff records.

### CLM-018 — D-41 R5 T3 PDU-016 O7/E5 Requirement (2026-07-12)

> ##### D-41 R5 T3 PDU-016 O7/E5 Requirement (2026-07-12)
>
> User-provided CAEPIPE output evidence defaults to private/user-controlled, local-only, telemetry-disabled handling. Public classification requires an explicit later rights/provenance path; this seam does not assert global privacy sufficiency.
>

### CLM-019 — D-41 R5 T6 PDU-050 validation hold

> ##### D-41 R5 T6 PDU-050 validation hold
>
> `DEL-17-05-ACC-006` remains `VERIFIED_NOT_VALIDATED`. Parser, skipped-run,
> metadata, boundary, and invented-fixture tests verify the bounded software
> contract only. Optional live execution remains an O10 user-owned path gated by
> an executable, acknowledged responsibilities, and a selected target/MBF/
> invocation profile; those inputs are unavailable and no live validation was
> performed.

- **AC-001** — The contract preserves explicit user configuration, executable and license ownership, accepted MBF package/profile inputs, run-directory and environment metadata, documented invocation with unresolved profile gates, output discovery, bounded fixture-confirmed parser coverage, canonical-ID correlation, skip-without-executable public-CI behavior, diagnostics for unknown/unmapped sections, private-user-data handling, and regression-only evidence classification without bundling CAEPIPE or implying compatibility, code compliance, formal validation, or professional acceptance.

## Production and Verification Method — Praxeology

### CLM-020 — Procedure: DEL-17-05 CAEPIPE external run harness and CSV parser

> #### Procedure: DEL-17-05 CAEPIPE external run harness and CSV parser
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-021 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-17-05-DECL-004`.
>

### CLM-022 — Purpose

> ##### Purpose
>
> This procedure defines how to populate, review, and later implement the optional CAEPIPE external run harness and CSV parser without bundling commercial software, bypassing licenses, overclaiming compatibility, or converting regression evidence into professional acceptance.
>

### CLM-023 — Prerequisites

> ##### Prerequisites
>
> For this Phase A documentation pass:
>
> 1. Read AGENTS/TASK/ORCHESTRATOR instructions and the `four-documents` skill contract.
> 2. Read DEL-17-05 local `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, and `MEMORY.md`.
> 3. Read DEL-17-01, DEL-17-02, and DEL-17-04 four-document kits.
> 4. Read DEL-17-01 `Source_Basis_Register.md` and `CAEPIPE_Question_Dossier.md`.
> 5. Read the DEL-17-05 decomposition entry, scope items, objectives, project invariants, and data-boundary policy.
> 6. Verify that DEL-17-04 is the declared upstream dependency and that unresolved DEL-17-04 target-profile gates are carried forward.
>
> For a later implementation tranche:
>
> 1. Confirm the implementation tranche has explicit write scope for code, schemas, tests, or fixtures.
> 2. Confirm the executable path is user-configured and not bundled.
> 3. Confirm the user owns license and execution-environment responsibility.
> 4. Confirm the explicit configuration surface for the executable path; until selected, carry `TBD` for the environment variable, config key, CLI option, GUI field, or equivalent.
> 5. Confirm whether the first invocation profile has been resolved or remains an explicit `TBD`.
> 6. Confirm the accepted DEL-17-04 MBF profile/writer tranche that may be used for live external-run tests; until accepted, carry the profile identifier as `TBD`.
> 7. Confirm parser fixture provenance before adding public fixtures.
> 8. Confirm no private user/project CAEPIPE output is committed without documented redistribution rights.
>

### CLM-024 — Steps

> ##### Steps
>

### CLM-025 — Phase A document population

> ###### Phase A document population
>
> 1. Create the four-document kit: `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md`.
> 2. Ground CAEPIPE-specific facts in DEL-17-01 source IDs and official/public CAEPIPE references.
> 3. Carry forward unresolved target behavior as `TBD`, especially:
>    - first CAEPIPE version/profile;
>    - first MBF profile required by the harness;
>    - exact invocation profile;
>    - stable CSV sections for parser coverage;
>    - stable canonical-ID correlation strategy for parsed rows.
> 4. Add a conflict table when public references are insufficient to choose behavior.
> 5. Avoid implementation details that would become code, schema, fixture, CLI, GUI, release, compatibility, code-compliance, or professional claims.
> 6. Run four-document and minimum-fileset validation.
> 7. Update `_STATUS.md` from `OPEN` to `INITIALIZED` only if the P1/P2 document pass completes successfully.
> 8. Record run evidence in `_run_records/TASK_RUN_*.md`.
>

### CLM-026 — Future external-run procedure

> ###### Future external-run procedure
>
> This section is a contract for later implementation, not an implementation in this tranche.
>
> 1. Load an export package produced under the accepted DEL-17-02/DEL-17-04 contract.
> 2. Confirm the MBF input exists and has associated manifest, ID-map, and loss-report records.
> 3. Resolve the user-configured CAEPIPE executable path without searching for or installing CAEPIPE; record the selected configuration surface or `TBD` if a later implementation brief has not selected one.
> 4. Build a run directory under user/project control.
> 5. Record execution-environment context: operating system, working directory, permissions relevant to invocation, shell or process-launch context, remote Windows or compatibility-layer notes if used, path quoting notes where applicable, and whether the host permits stdout/stderr capture.
> 6. Record pre-run metadata: source model/export IDs, MBF path, accepted DEL-17-04 profile ID or `TBD`, command profile, executable path provenance, working directory, environment notes, and boundary notices.
> 7. Invoke the external executable only when the user-owned configuration is present.
> 8. Capture exit status and stdout/stderr where the host environment permits.
> 9. Discover the target CSV artifact using the selected invocation profile and record missing-output diagnostics if it is absent.
> 10. Run the CSV parser on the discovered output only within the declared parser coverage.
> 11. Bind parsed evidence to canonical IDs through the manifest/ID map where possible.
> 12. Mark unparsed sections, unknown sections, unmapped rows, unit/coordinate uncertainty, and unsupported target behavior as diagnostics or `TBD`.
> 13. Write a run metadata record and parser diagnostics that classify the result as regression/handoff evidence only.
>

### CLM-027 — Future skip-without-executable procedure

> ###### Future skip-without-executable procedure
>
> 1. Check whether an executable path has been explicitly configured.
> 2. If absent, skip external execution tests and record the skip reason.
> 3. Continue parser-only and contract validation tests that do not require CAEPIPE.
> 4. Do not fail public CI solely because no CAEPIPE executable or license is present.
>

### CLM-028 — Future run-record field list

> ###### Future run-record field list
>
> The later implementation record shall be durable enough for review even when the external executable is absent or the CSV artifact is not produced. Minimum fields are:
>
> - configuration surface checked;
> - configured executable path state, recorded as present, absent, invalid, or private/redacted as appropriate;
> - license/environment responsibility acknowledgement state;
> - source model/export IDs;
> - accepted DEL-17-04 MBF profile ID or `TBD`;
> - MBF input path and manifest/ID-map/loss-report references;
> - command profile and command shape actually used, with sensitive paths redacted when needed;
> - operating system, working directory, permissions, shell/process-launch context, and compatibility or remote-Windows notes;
> - expected target CSV artifact path and observed target CSV artifact path;
> - exit status, stdout availability, stderr availability, and capture locations where allowed;
> - output discovery status;
> - parser coverage-register version or `TBD`;
> - parser status, unsupported sections, unmapped rows, missing sections, and diagnostic severity summary;
> - evidence classification stating regression/handoff evidence only;
> - boundary note confirming no bundled executable, license bypass, protected fixture, compatibility proof, code-compliance claim, formal-validation claim, or professional-acceptance claim.
>

### CLM-029 — Verification

> ##### Verification
>
> Phase A verification:
>
> ```bash
> tools/validation/check_four_documents.sh "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-05_CAEPIPE external run harness and CSV parser"
> tools/validation/check_min_viable_fileset.sh "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-05_CAEPIPE external run harness and CSV parser"
> git diff --check
> ```
>
> Manual Phase A checks:
>
> - all four default document schemas are present;
> - all target-specific unresolved behavior is `TBD`;
> - no proprietary examples or protected standards data are introduced;
> - no implementation code, schema, fixture, release, compatibility, code-compliance, formal-validation, or professional claim is introduced;
> - `_STATUS.md` changes only through the safe `OPEN` to `INITIALIZED` transition;
> - `_run_records` captures sources, outputs, validation, and remaining gaps.
>
> Future implementation checks:
>
> - skip-without-executable tests pass in public CI;
> - opt-in executable tests require explicit user configuration;
> - parser coverage register matches invented or rights-cleared fixtures;
> - run metadata records manifest, ID-map, loss-report, command profile, output discovery, parser diagnostics, and evidence classification;
> - parsed evidence is tied to canonical IDs or explicitly marked weak/unmapped.
>

### CLM-030 — Records

> ##### Records
>
> Phase A records:
>
> - four-document kit;
> - updated `_STATUS.md`;
> - `_run_records/TASK_RUN_2026-05-18_1156.md`;
> - validation command output summarized in the run record.
>
> Future implementation records:
>
> - executable configuration provenance;
> - run directory metadata;
> - external-run record;
> - stdout/stderr capture where available;
> - CSV parser coverage record;
> - parser diagnostics;
> - manifest/ID-map/loss-report references;
> - fixture provenance review;
> - skipped-test evidence when the executable is absent.

### CLM-031 — D-41 R5 T3 PDU-016 Check

> ##### D-41 R5 T3 PDU-016 Check
>
> Verify default and partial-override packages remain `private_user_controlled`, local-only, and telemetry-disabled; embedded private/protected/commercial payload remains blocking.

- **VER-001** — Validate the contract and review source parity, all run-record fields, explicit configuration and skip paths, invocation/profile and parser-section TBDs and conflicts, manifest/ID-map/loss bindings, output discovery and parser diagnostics, unit/coordinate and provenance boundaries, invented/private fixture handling, no license bypass, and all prohibited compatibility, validation, compliance, and professional-authority claims.

## Governing Values and Decisions — Axiology

### CLM-032 — Guidance: DEL-17-05 CAEPIPE external run harness and CSV parser

> #### Guidance: DEL-17-05 CAEPIPE external run harness and CSV parser
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-033 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-17-05-DECL-003`.
>

### CLM-034 — Purpose

> ##### Purpose
>
> Use this deliverable to keep CAEPIPE external-run work useful without crossing the project boundary. The harness can help confirm that a user-owned CAEPIPE environment accepts a generated MBF and emits CSV output, and the parser can extract limited regression/handoff evidence from that CSV. Neither result is an engineering approval state.
>
> This Phase A kit is intentionally conservative. It records what later implementation must preserve, what remains unresolved, and what reviewers should reject.
>

### CLM-035 — Principles

> ##### Principles
>
> - External execution is opt-in and user-owned.
> - A configured executable path is evidence of user configuration, not a bundled project dependency.
> - Public CI and public fixtures must not require CAEPIPE.
> - Parsed CSV data is regression/handoff evidence only.
> - Target behavior that is not source-confirmed remains `TBD`.
> - The harness shall preserve DEL-17-02 manifest, stable-ID, loss-report, and boundary vocabulary.
> - The parser shall prefer narrow, documented coverage over broad guesswork.
> - Unsupported, missing, unstable, or unmapped output shall become diagnostics, not silent success.
>

### CLM-036 — Vocabulary

> ##### Vocabulary
>
> Use `target CSV artifact` as the preferred term for the CAEPIPE CSV file produced or expected from a user-owned external run. `Expected CSV path`, `observed CSV path`, `CSV result file`, and `CSV output` are allowed aliases only when they describe a specific state of that same artifact. Do not split these aliases into separate concepts unless a later implementation schema deliberately does so.
>

### CLM-037 — Considerations

> ##### Considerations
>
> The public CAEPIPE import documentation supports MBF as a text model input path and describes command-line CSV creation from an MBF input. The batch-mode documentation also describes CSV output from MBF input, but its command shape includes a batch-mode argument pattern. DEL-17-05 should therefore carry the exact first invocation profile as `TBD` until the project chooses a documented profile or receives support clarification.
>
> The export-data documentation supports CSV/text output surfaces for model and result data. It does not, by itself, define which CSV sections are stable enough for automated parser coverage in the first OpenPipeStress harness. Parser coverage should be explicit, small, and test-backed.
>
> The harness should not attempt to infer CAEPIPE solver validity. It can record operational facts such as input file written, executable invoked, CSV discovered, parser completed, counts matched where meaningful, and rows correlated to canonical IDs where possible.
>
> Invocation-profile and parser-section TBDs should move to implementation scope only when the project has enough evidence to avoid hidden target-behavior claims. The ruling path is: first use public/official CAEPIPE documentation where it is explicit; otherwise use CAEPIPE support clarification or a human project-authority decision recorded in the deliverable's accepted implementation brief. Fixture-confirmed parser behavior may support parser coverage only when the fixture is invented, rights-cleared, or private/user-controlled with documented handling.
>

### CLM-038 — Trade-offs

> ##### Trade-offs
>
> | Decision area | Conservative choice | Trade-off |
> |---|---|---|
> | Executable discovery | Require explicit user configuration. | Less convenient, but avoids bundled dependency and license-bypass implications. |
> | Invocation profile | Keep exact command profile `TBD` until reconciled. | Slower implementation, but prevents a false support claim. |
> | Parser scope | Parse only known/fixture-confirmed sections. | Narrow evidence at first, but clearer diagnostics. |
> | CSV identity | Use manifest/ID-map correlation where possible. | Requires sidecars and run metadata, but avoids row-order assumptions. |
> | CI behavior | Skip external-run tests without configured executable. | Public CI will not exercise live CAEPIPE, but remains legal and reproducible. |
> | User CSV handling | Default to private unless redistribution rights are recorded. | Fewer shared fixtures, but preserves data/IP boundaries. |
> | TBD closure | Require explicit ruling evidence before implementing invocation or parser-section assumptions. | Slower closure, but keeps target behavior and authority boundaries reviewable. |
>

### CLM-039 — Examples

> ##### Examples
>
> Acceptable Phase A examples:
>
> - A document-level run record listing the MBF input path, expected CSV path, parser status, and boundary note as fields to capture later.
> - An invented parser fixture that exercises CSV header handling without copying a vendor or client file.
> - A skipped external-run test record that states no user-owned executable path was configured.
>
> Not acceptable:
>
> - A committed CAEPIPE executable, installer, commercial example model, or copied vendor fixture.
> - A claim that OpenPipeStress is CAEPIPE-compatible because a CSV file was produced.
> - A claim that a successful CAEPIPE run proves code compliance, professional acceptance, or formal validation.
> - A parser that silently treats unknown sections or unmapped rows as supported.
>

### CLM-040 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | DEL-17-05-CONF-001 | Public CAEPIPE pages describe related MBF-to-CSV execution patterns, but the exact first OpenPipeStress invocation profile is not selected. | `CAEPIPE-IMPORT-MBF`, command-line operation, lines 15-20. | `CAEPIPE-BATCH`, batch-mode notes, lines 40-52. | `Specification.md` REQ-003/004; `Procedure.md` run-profile steps. | Carry invocation profile as `TBD` until support clarification or human profile decision. | TBD |
> | DEL-17-05-CONF-002 | CSV export surfaces are documented, but stable parser-section coverage for automated regression is not selected. | `CAEPIPE-EXPORT-DATA`, export list, lines 31-43. | DEL-17-01 `TBD-17-01-004` and CQ-17-01-005. | `Specification.md` REQ-009; parser coverage records. | Keep first parser coverage `TBD`; require fixture-confirmed parser coverage before implementation claims. | TBD |
>

### CLM-041 — Review Guidance

> ##### Review Guidance
>
> Reviewers should reject later DEL-17-05 work if it:
>
> - executes CAEPIPE without explicit user configuration;
> - introduces bundled binaries, installers, proprietary examples, or license-bypass behavior;
> - treats public documentation as permission to claim broad compatibility;
> - treats CSV parsing as professional acceptance or code compliance;
> - commits private/user CAEPIPE outputs without provenance and redistribution review;
> - hides target version, invocation, parser-section, unit, coordinate, or ID-map uncertainty;
> - expands into schema/code implementation outside an approved implementation tranche.

### CLM-042 — D-41 R5 T3 PDU-016 Boundary

> ##### D-41 R5 T3 PDU-016 Boundary
>
> Invented public fixtures remain rights-safe test inputs, but user-provided CSV/runtime evidence must not inherit their public classification by default.

### CLM-043 — DEC-080 validation-posture currency note (2026-07-16)

> ##### DEC-080 validation-posture currency note (2026-07-16)
>
> Per `DEC-080` (D-47 O-A, SCA-007, 2026-07-16), external-prover correlation (e.g., CAEPIPE) is now the amended PRD's principal validation posture (v0.3 §22.5) rather than merely future/optional validation evidence; activation remains owner-gated on lawful tool procurement, and this deliverable's bounded contract — optional user-owned harness, O10 gates, and regression/handoff classification of raw runs and parsed CSVs short of accepted correlation comparisons — is unchanged, with no scope expansion.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-030 SOW-046 SOW-075 OBJ-007 OBJ-009 OBJ-017 OBJ-018 | CLM-009 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
