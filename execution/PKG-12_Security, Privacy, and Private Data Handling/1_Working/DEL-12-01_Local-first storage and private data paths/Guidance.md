# Guidance: DEL-12-01 Local-first storage and private data paths

## Purpose

This deliverable keeps OpenPipeStress private-data handling aligned with the product stance: open mechanics and public schemas, but user-controlled private project, rule-pack, material, component, owner, and code/design-basis data.

The guidance remains policy and evidence level. June 7 evidence adds a metadata-only guard helper and tests, but this deliverable still does not select OS-specific roots, create real private paths, store payloads or secrets, implement encryption/key management, or close runtime storage behavior.

## Principles

| Principle | Guidance |
|---|---|
| Local-first default | Assume private engineering data remains on user-controlled local storage unless a later approved scope change says otherwise. |
| Public/private separation | Public repository content may include code, schemas, blank templates, import mechanisms, and invented examples; private rule packs, libraries, owner standards, and project models stay outside public paths by default. |
| Symbolic before physical | Use symbolic path classes in planning. SCA-003 establishes a local SQLite-backed MVP storage profile, but OS-specific roots, runtime persistence behavior, and package/container mechanics remain deferred. |
| Persistence compatibility | Do not define storage conventions that conflict with versioned schema governance, migration status, canonical JSON/JCS-compatible hashing, provenance, unit awareness, or round-trip reproducibility. |
| No silent defaults | Unknown roots, package choices, privacy statuses, or provenance fields remain `TBD` or warnings. |
| No-bypass adapters | Import/export, plugins, and private library adapters must still pass through validation, unit, provenance, diagnostic, and public/private boundary controls. |
| Human authority | Storage controls and reports support review; they do not certify, seal, approve, authenticate, or declare code compliance. |

## Considerations

### Private Data Classes

Treat these as private by default unless the user intentionally contributes or exports them with documented redistribution rights:

- project models and project-local assets;
- private rule packs and owner design bases;
- material libraries and allowable-like values;
- component libraries, manufacturer/vendor data, geometry catalogs, and stiffness data;
- code-specific formulas, interpretations, load combinations, SIFs, flexibility factors, and protected standards-derived content;
- report exports containing private engineering values.

### Path Boundary

Prefer planning language like `USER_PRIVATE_LIBRARY_ROOT` and `USER_CHOSEN_PROJECT_PATH` instead of real filesystem examples. Real examples can accidentally normalize a repository path, a cloud-synced folder, or a user-specific secret-bearing directory as the expected pattern.

### Report and Export Boundary

Local report/export destinations are still private-data risk surfaces. DEL-12-01 defines path and storage posture; DEL-12-02 owns redaction/export safeguards and should consume the path-class vocabulary from this deliverable.

### Private Library and Secret Boundary

DEL-12-04 owns secret and private-library handling. DEL-12-01 should not define credential storage, secret material, encryption, key management, or concrete private-library registry behavior beyond the local-first and public/private path boundary. The 2026-06-07 DEL-12-04 guard alignment is downstream evidence, not authority for DEL-12-01 to select real secret handling.

### Metadata-Only Guard Boundary

The June 7 `core.security.local_first_storage` helper is a metadata-only guard. It classifies explicit records, blocks unsafe public/shared or remote/direct-SQL/secret/payload markers, and returns safe metadata. It is not a runtime storage service, schema migration layer, physical package/container implementation, cloud exception mechanism, encryption feature, key-management feature, or approval workflow.

## Trade-offs

| Trade-off | Implication |
|---|---|
| User-chosen project path vs. app-managed data directory | User choice improves transparency; app-managed roots can reduce leakage risk. The project has not selected final OS roots or application data directories. |
| Local SQLite profile vs. final package mechanics | SCA-003 resolves the MVP storage profile as local SQLite-backed, but package/container mechanics, sidecar handling, and user-visible root behavior remain TBD. |
| Single project package vs. separate private library roots | A package can simplify portability; separate roots can reduce accidental sharing. The detailed package/container mechanics remain TBD. |
| Checksums in public manifests vs. private metadata | Hashes support reproducibility and tamper detection, but manifests must not reveal private values or protected source text. |
| Local-first default vs. future collaboration | MVP excludes cloud services unless approved; any later collaboration feature must preserve explicit user intent and no private data transmission by default. |

## Examples

The following are symbolic examples only:

| Scenario | Acceptable Planning Expression | Avoid |
|---|---|---|
| User project model | `USER_CHOSEN_PROJECT_PATH` plus deterministic project manifest | A real path under this repository or a cloud-synced default |
| Private rule pack | `USER_PRIVATE_LIBRARY_ROOT/rule_packs/<private-id>` as a symbolic class | A bundled public rule-pack file copied from a protected source |
| Material library | Private library reference with provenance and redistribution status | Public material allowable table without documented rights |
| Report export | User-selected local export target with future redaction controls | Public report template containing protected formulas or private values |

## Open Issues and TBDs

| Issue ID | Topic | Status | Notes |
|---|---|---|---|
| LFSP-OI-001 | Physical project package/container mechanics | TBD | SCA-003 establishes the local SQLite-backed MVP storage profile; exact package/container mechanics, sidecar behavior, and user-visible root handling remain implementation-level TBD. |
| LFSP-OI-002 | OS-specific private-data roots | TBD | This run defines symbolic path classes only. |
| LFSP-OI-003 | Executable storage tests | PARTIAL / TBD | Metadata-only guard tests exist as of 2026-06-07. Runtime storage-service, OS-root, package/container, migration, and integration tests remain future work. |
| LFSP-OI-004 | Cloud exception approval path | TBD | Cloud services are out of MVP unless separately approved. The exact approval record format is not defined here. |
| LFSP-OI-005 | Secret storage, encryption, and key management | TBD | Owned by DEL-12-04 and later implementation/governance decisions, not this DEL-12-01 alignment run. |
| LFSP-OI-006 | Runtime storage service and schema migration behavior | TBD | The metadata-only guard does not read/write project stores, open SQLite handles, or implement migrations. |
| LFSP-OI-007 | Lifecycle acceptance and approval choices | TBD | Review-readiness evidence is not `ISSUED`, release approval, security certification, professional approval, sealing, authentication, or code-compliance acceptance. |

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| LFSP-CON-001 | The deliverable title implies storage path decisions, while accepted authority only supports a local SQLite-backed MVP storage profile plus metadata-only guard evidence; package mechanics, roots, and runtime behavior remain TBD. | `docs/_Registers/Deliverables.csv` row DEL-12-01; `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 | `docs/security/local_first_storage_policy.md`; `TASK_RUN_2026-06-07_0140.md`; package fan-in `WORKING_ITEMS_RUN_2026-06-07_0150_TP-PKG12-LOCAL-PRIVACY-GUARDS-FANIN.md` | Specification Requirements; Procedure Steps; Open Issues | Preserve symbolic path classes and metadata-only guard evidence; defer physical root/container mechanics and runtime storage decisions. | TBD |
