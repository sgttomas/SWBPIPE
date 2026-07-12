# Datasheet: DEL-17-05 CAEPIPE external run harness and CSV parser

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-17-05 |
| Package ID | PKG-17 |
| Package | Export Format Interoperability |
| Name | CAEPIPE external run harness and CSV parser |
| Type | BACKEND_FEATURE_SLICE |
| Scope Items | SOW-030, SOW-046, SOW-075 |
| Objectives | OBJ-007, OBJ-009, OBJ-017, OBJ-018 |
| Current phase | Phase A four-document contract only |
| Upstream dependency | DEL-17-04 CAEPIPE MBF export profile and deterministic writer |

## Attributes

| Attribute | Required treatment |
|---|---|
| Harness role | Optional external execution wrapper around a user-provided, licensed CAEPIPE executable. |
| Execution ownership | User-owned executable path, license, operating system, working directory, permissions, and any compatibility/remote Windows environment. |
| Bundled executable | Prohibited. No CAEPIPE binary, installer, commercial example, or license-bypass mechanism is part of this deliverable. |
| Input target | CAEPIPE MBF emitted by DEL-17-04 or a later accepted implementation tranche; exact supported MBF profile remains `TBD`. |
| Expected target output | CAEPIPE CSV result file when the user-owned external run produces one; output filename/location behavior is source-supported but invocation profile details remain `TBD`. |
| CSV parser role | Regression and handoff evidence extraction only. Parsed CSV rows are not professional acceptance, code compliance, formal validation, or proof of CAEPIPE compatibility. |
| Stable identity | Correlation to OpenPipeStress canonical IDs shall use DEL-17-02/DEL-17-04 manifest and sidecar ID-map evidence where possible; direct CSV identity coverage remains `TBD`. |
| Test posture | Automated tests without a configured executable shall skip with an explicit reason; parser-only tests shall use invented or rights-cleared fixtures. |

## Conditions

| Condition | Source/evidence basis | Status |
|---|---|---|
| CAEPIPE can read MBF text input and create a model file through import. | `CAEPIPE-IMPORT-MBF`, official CAEPIPE import page, lines 0-23. | Source-supported. |
| Command-line or batch execution can produce CSV output from an MBF input. | `CAEPIPE-IMPORT-MBF`, lines 15-20; `CAEPIPE-BATCH`, lines 40-52. | Source-supported with invocation-profile `TBD`. |
| CAEPIPE can output model and/or result data to CSV/text through print/export surfaces. | `CAEPIPE-EXPORT-DATA`, lines 31-43. | Source-supported. |
| CAEPIPE can export model data to MBF from the Layout window. | `CAEPIPE-EXPORT-MBF`, lines 28-39. | Source-supported for source-basis context. |
| First supported CAEPIPE version/profile. | DEL-17-01 `TBD-17-01-001`; DEL-17-04 `TBD-17-04-001`. | `TBD`. |
| Stable CSV sections suitable for automated parsing. | DEL-17-01 `TBD-17-01-004`; CQ-17-01-005. | `TBD`. |
| Recommended command-line invocation shape for the first harness profile. | CQ-17-01-004; public pages describe related but not fully reconciled invocation patterns. | `TBD`. |

## Construction

| Artifact | Construction rule |
|---|---|
| External run harness | Future implementation shall be opt-in, disabled without user configuration, and bounded to a user-provided executable path. This Phase A document does not implement it. |
| Run directory contract | Future implementation shall record an auditable run directory concept containing the MBF input, manifest/ID-map/loss-report evidence, target CSV output if produced, execution metadata, stdout/stderr where available, and parser diagnostics. Exact filenames remain implementation-level `TBD` unless already fixed by DEL-17-02/DEL-17-04. |
| CSV parser | Future implementation shall parse only documented or fixture-confirmed CSV sections and shall report unrecognized, missing, or unstable sections instead of silently accepting them. |
| Run metadata | Future implementation shall record executable path provenance without copying or redistributing the executable, target version/profile if discoverable, command shape used, timestamps or hashes according to the manifest policy, exit status, output discovery, parser status, and boundary notes. |
| Evidence records | Future implementation shall classify evidence as regression/handoff evidence, not formal validation or professional approval. |
| Fixtures | Public fixtures shall be invented or rights-cleared. User-provided CAEPIPE outputs may be used only under user-controlled/private handling unless redistribution rights are documented. |

## References

| Source ID | Source | Use |
|---|---|---|
| DEL-17-01 | `../DEL-17-01_CAEPIPE and export-format source basis/` | Source authority, CAEPIPE questions, TBD register, and boundary facts. |
| DEL-17-02 | `../DEL-17-02_Export package, profile, and stable ID map contracts/` | Export manifest, stable-ID map, loss-report, profile, and external-execution policy. |
| DEL-17-04 | `../DEL-17-04_CAEPIPE MBF export profile and deterministic writer/` | Upstream CAEPIPE MBF profile/writer contract and unresolved target-profile gates. |
| PLAN-EXPORT-INTEROP | `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md` | Strategic source for optional external harness, run directory concept, and boundary framing. |
| CAEPIPE-IMPORT-MBF | `https://www.sstusa.com/docs/users_manual/import_mbf_print.htm` | Public evidence for MBF import and command-line CSV behavior. |
| CAEPIPE-EXPORT-DATA | `https://www.sstusa.com/docs/users_manual/export_data_from_caepipe.htm` | Public evidence for CAEPIPE CSV/text result and model export surfaces. |
| CAEPIPE-BATCH | `https://www.sstusa.com/docs/users_manual/running_caepipe_caepipe_3d__in_batch_mode.htm` | Public evidence for batch-mode CSV output behavior and path/invocation caveats. |
| CAEPIPE-EXPORT-MBF | `https://www.sstusa.com/docs/users_manual/export_mbf.htm` | Public evidence inherited through DEL-17-01/DEL-17-04 source basis for CAEPIPE MBF export from model data; this source is not listed as a direct DEL-17-05 package reference in `_REFERENCES.md`. |
| CONTRACT | `docs/CONTRACT.md` | Project invariants for IP, authority boundary, unit-aware exports, agent TBD handling, and professional claims. |
| IP-DATA | `docs/IP_AND_DATA_BOUNDARY.md` | Public/private data and protected-content boundaries. |
## D-41 R5 T3 PDU-016 Evidence State

Selected seam: CAEPIPE external-run/parser evidence. Default: private user-controlled, local-only, telemetry false. Scope excludes whole-product security assurance.
