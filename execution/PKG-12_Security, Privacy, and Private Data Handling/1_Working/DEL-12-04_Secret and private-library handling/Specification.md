# Specification: DEL-12-04 Secret and private-library handling

## Scope

This deliverable specifies a local-first handling surface for private libraries, private paths, and credential references used by imports or private storage. It covers the private library registry concept and secret handling tests listed in `_CONTEXT.md`.

In scope:

- local-first handling for private rule packs, private material libraries, private component libraries, owner design-basis files, and project-local private library references;
- registry metadata needed to mark privacy, provenance, redistribution status, checksum status, review status, and transmission posture;
- secret-reference handling for imports or private storage without storing usable secret material in project artifacts, reports, fixtures, or public examples;
- warning, redaction, quarantine, and deny-by-default behavior at export, report, plugin, adapter, telemetry, and public contribution boundaries.
- current metadata-only release-guard evidence in `core/security/secret_private_library/`, `docs/security/secret_private_library_handling.md`, and `tests/security/test_secret_private_library_handling.py`.

Out of scope:

- cloud secret management or cloud storage operations unless separately approved;
- final operating-system credential-store integration details;
- exact secret provider, encrypted-storage default, storage roots, permission grant persistence, physical project package/container, public API transport, cloud/network behavior, external secret manager behavior, and approval choices;
- legal sufficiency, certification, approval, sealing, endorsement, or professional code-compliance claims;
- real private libraries, real credentials, real private project data, protected standards text, protected tables, proprietary formulas, material allowables, SIF/flexibility tables, or protected dimensional data.

Sources: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` PKG-12 and row `DEL-12-04`; `docs/PRD.md` sections 17.3, 18.1, and 18.3.

## Requirements

| ID | Requirement | Source | Verification |
|---|---|---|---|
| DEL-12-04-R1 | Private libraries and rule packs shall be stored outside public example directories by default and marked private in metadata. | `docs/PRD.md` section 17.3; `docs/IP_AND_DATA_BOUNDARY.md` section 6 | Registry tests inspect default classification and public-example exclusion behavior. |
| DEL-12-04-R2 | Private rule packs, component libraries, material data, project files, and calculation results shall not be transmitted by telemetry by default. | `docs/PRD.md` section 18.2; `docs/architecture/plugin_boundary.md` Private Data Handling | Telemetry exclusion tests verify absent private fields. |
| DEL-12-04-R3 | Private library registry records shall include privacy classification, provenance/source summary, redistribution status, review status, and checksum status where available. | `docs/IP_AND_DATA_BOUNDARY.md` section 4; `docs/PRD.md` sections 13.4 and 18.3; `docs/architecture/plugin_boundary.md` Checksums and Provenance | Registry schema/fixture tests validate required metadata presence without protected content. |
| DEL-12-04-R4 | Project, registry, report, and test artifacts shall store opaque credential references only and shall not store usable credential values. | ASSUMPTION from `docs/PRD.md` section 18.3 and `docs/architecture/plugin_boundary.md` Permission Model Skeleton | Secret handling tests reject secret-like values and accept non-sensitive reference markers only. |
| DEL-12-04-R5 | Export, report, issue/bug-report, and shared-model paths shall warn before exposing private data and shall redact or omit protected/private values by default. | `docs/PRD.md` sections 15.2, 17.3, and 18.3; SOW-040 | Export/redaction tests verify default omission and warning diagnostics. |
| DEL-12-04-R6 | If protected standards data or proprietary source content is suspected in a private-library import or public contribution path, ingestion/publication shall stop and route to quarantine/human review. | `docs/IP_AND_DATA_BOUNDARY.md` section 5; `docs/DIRECTIVE.md` section 5 | Quarantine-routing tests verify diagnostic emission and no public artifact write. |
| DEL-12-04-R7 | Plugins and adapters shall be denied private-library, private-rule-pack, filesystem, and network access unless an explicit permission grant exists. | `docs/architecture/plugin_boundary.md` Permission Model Skeleton; `docs/architecture/extension_domain_contracts.md` Denied-By-Default Behavior | Permission tests verify fail-closed access behavior. |
| DEL-12-04-R8 | Plugins, adapters, imports, exports, and private storage shall not bypass schema validation, unit checks, provenance checks, privacy controls, protected-content screening, diagnostics, checksums, or report controls. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-02, AB-00-06, AB-00-07; `docs/architecture/extension_domain_contracts.md` No-Bypass Rules | Boundary tests verify routed control checks and diagnostics. |
| DEL-12-04-R9 | Reports and exported result payloads may identify private rule packs or libraries by name/version/checksum/source note without exposing protected formulas, private values, or proprietary source content in public templates. | `docs/PRD.md` sections 15.2 and 17.3; `docs/architecture/plugin_boundary.md` Checksums and Provenance | Report/export tests inspect metadata-only references and protected-content omission. |
| DEL-12-04-R10 | Missing provenance, unknown redistribution status, or uncertain privacy classification shall be surfaced as explicit diagnostics or findings, not silent defaults. | OPS-K-DATA-2/3; `docs/PRD.md` section 13.5; `docs/SPEC.md` section 7 | Validation tests verify `PROVENANCE_WARNING` or `IP_BOUNDARY_WARNING` class behavior. |

## Standards

No protected standards text, tables, figures, or proprietary values are needed to execute this deliverable. Governing project references are:

- `docs/CONTRACT.md` invariants OPS-K-IP-1/2/3, OPS-K-DATA-1/2/3, OPS-K-AUTH-1/2, OPS-K-PRIV-1/2, OPS-K-RULE-3, and OPS-K-AGENT-1/2/3/4.
- `docs/DIRECTIVE.md` non-negotiable product principles and stop rules.
- `docs/IP_AND_DATA_BOUNDARY.md` public/private data and quarantine policy.
- `docs/PRD.md` sections 12, 13, 15, 17, and 18.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 architecture basis AB-00-01/02/03/04/06/07/08.

## Verification

| Verification item | Acceptance signal |
|---|---|
| Four-document source boundary | No real credentials, private libraries, protected standards text, proprietary source content, or usable credential examples appear in deliverable artifacts. |
| Registry metadata coverage | Current helper/test fixtures cover privacy classification, provenance, redistribution status, checksum status, credential-reference posture, transmission default, and review/quarantine status as metadata-only records; exact registry schema/storage placement remains `TBD`. |
| Secret-reference safety | Current tests fail when a project or registry artifact contains secret material instead of an opaque reference or non-sensitive sentinel marker. |
| Local-first default | Modeling, solving, rule checking, reporting, and private-library use require no cloud service by default. |
| Redaction/export path | June 7 guard evidence aligns with DEL-12-02 redaction/export markers for metadata-only report/shared/downstream release contexts; runtime report/export route integration remains `TBD`. |
| Permission path | Plugin/adapter access semantics are denied by default in the documented no-bypass control surface; exact permission grant persistence remains `TBD`. |
| Diagnostic path | Missing provenance, unknown redistribution, suspected protected content, payload markers, cloud/network markers, external secret-manager markers, direct SQL/raw SQLite markers, storage-bypass markers, and concrete path indicators produce structured diagnostics or safe metadata reduction in current focused tests. |

## Documentation

Required deliverable-local artifacts:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_run_records/*` phase records

Current implementation-facing evidence:

- `core/security/secret_private_library/controls.py` and `__init__.py` provide metadata-only reference classification and release-guard helpers;
- `docs/security/secret_private_library_handling.md` documents guard behavior and non-authority boundaries;
- `tests/security/test_secret_private_library_handling.py` provides focused invented-fixture coverage;
- `_run_records/TASK_RUN_2026-06-07_0942_secret-private-library-alignment.md` and package fan-in `WORKING_ITEMS_RUN_2026-06-07_0957_TP-PKG12-REDACTION-SECRET-GUARD-CLOSEOUT.md` record passing focused and paired PKG-12 validation.

Remaining implementation details not resolved by this specification include exact secret provider, encrypted-storage default, storage roots, permission grant persistence, physical project package/container, public API transport, cloud/network behavior, external secret manager behavior, and approval choices.
