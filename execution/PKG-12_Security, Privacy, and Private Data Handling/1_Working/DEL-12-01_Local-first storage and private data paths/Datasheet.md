# Datasheet: DEL-12-01 Local-first storage and private data paths

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-12-01 |
| Deliverable Name | Local-first storage and private data paths |
| Package ID | PKG-12 |
| Package Name | Security, Privacy, and Private Data Handling |
| Deliverable Type | SECURITY_CONTROL |
| Scope Item | SOW-029 |
| Objective | OBJ-010 |
| Setup Run Date | 2026-04-30 |
| Readiness Evidence Basis | 2026-06-07 metadata-only guard code, policy documentation, tests, and fan-in evidence; no lifecycle promotion by this alignment run |

## Attributes

| Attribute | Value |
|---|---|
| Product posture | Local-first by default |
| Protected private data classes | Private rule packs; private material data; private component data; project models; user-owned code/design-basis data |
| Public repository boundary | Public code, schemas, workflows, and invented examples only; no protected standards data or user-private engineering data |
| Cloud posture | Out of MVP unless separately approved by the human project authority |
| Persistence baseline | Versioned JSON-schema-governed persistence with canonical JSON/JCS-compatible hashes for JSON payloads |
| Physical project package/container mechanics | TBD; SCA-003 establishes the local SQLite-backed storage profile, but OS roots, runtime persistence behavior, and package mechanics remain deferred |
| Implementation status | Metadata-only local-first storage guard helper exists as of 2026-06-07; runtime storage implementation remains deferred |
| Real private path creation | None |
| Secret handling | TBD and owned by DEL-12-04; no real secret storage, encryption, or key-management behavior is selected here |

## Conditions

| Condition | Constraint |
|---|---|
| Local-first storage | Private project and library data remain user-controlled by default and are not transmitted or committed publicly by default. |
| Deterministic persistence | Storage conventions must remain compatible with deterministic round-trip serialization, schema versions, migrations, units, provenance, and reproducibility manifests. |
| Path convention level | This deliverable records symbolic path classes only; OS-specific roots and the physical package/container remain implementation-level TBD. |
| Public/private separation | Public repository paths must not be used as default private-library or private-project storage locations. |
| Export/report boundary | Export and report paths can expose private data and must defer redaction/export controls to DEL-12-02. |
| Professional boundary | Storage status, rule-pack presence, or report export must not be framed as certification, sealing, approval, or code compliance. |

## Symbolic Path Classes

| Symbolic Path Class | Intended Use | Boundary |
|---|---|---|
| `PUBLIC_REPO_ROOT` | Public source, schemas, documentation, invented examples, and validation artifacts safe for redistribution. | Must not hold private rule packs, owner standards, project models, private material libraries, private component libraries, credentials, or protected standards data. |
| `USER_CHOSEN_PROJECT_PATH` | User-selected local project model storage. | Private by default; actual folder and physical container remain TBD. |
| `USER_PRIVATE_LIBRARY_ROOT` | User-controlled local location for private rule packs, material libraries, component libraries, and owner/company design-basis data. | Must be outside default public repository paths and must not be committed publicly by default. |
| `PROJECT_PRIVATE_ASSET_ROOT` | Project-scoped private attachments, local manifests, and non-public supporting files. | Symbolic only in this deliverable; physical packaging and migration strategy remain TBD. |
| `REPORT_EXPORT_TARGET` | Local report/export destination selected by the user. | May contain private values; redaction/export safeguards belong to DEL-12-02. |
| `APP_CACHE_OR_SESSION_STATE` | Transient GUI/session/cache state. | Must not become an ungoverned durable private-data store. |

## Construction

This artifact began as a documentation-level storage boundary. As of 2026-06-07, readiness evidence also includes a metadata-only guard helper and focused tests. The deliverable output is limited to:

- local-first storage policy requirements in `Specification.md`;
- private path convention guidance in this datasheet and `Guidance.md`;
- verification expectations and current metadata-guard evidence in `Procedure.md`;
- semantic matrix/lensing and dependency setup artifacts.
- product-level policy documentation in `docs/security/local_first_storage_policy.md`;
- metadata-only guard code in `core/security/local_first_storage/`;
- focused tests in `tests/security/test_local_first_storage_policy.py`.

The June 7 guard helper classifies explicit storage metadata only. It does not create a runtime storage service, choose OS-specific roots, create real private directories, store payloads or secrets, open SQLite handles, authorize direct SQL, implement encryption or key management, create cloud service assumptions, or make approval, security-certification, professional, sealing, authentication, or code-compliance claims.

## References

| Source | Use |
|---|---|
| `INIT.md` | Bootstrap boundaries: open mechanics, private data, human authority, no certification claims. |
| `AGENTS.md` | Type 2 scoped execution and write-scope constraints. |
| `docs/DIRECTIVE.md` | Local/private data principles, hidden cloud/telemetry exclusion, professional boundary. |
| `docs/CONTRACT.md` | OPS-K-IP, OPS-K-DATA, OPS-K-PRIV, OPS-K-AUTH, OPS-K-UNIT, and agent invariants. |
| `docs/TYPES.md` | SECURITY_CONTROL type, private/user-supplied data vocabulary, canonical object registry. |
| `docs/SPEC.md` | Layered architecture, project storage-policy field, private rule-pack schema, report and agentic acceptance semantics. |
| `docs/IP_AND_DATA_BOUNDARY.md` | Public/private data policy and private user data boundary. |
| `docs/security/local_first_storage_policy.md` | Product-level local-first storage policy and metadata-only guard helper boundary. |
| `core/security/local_first_storage/` | Metadata-only storage guard helper evidence; not a runtime storage implementation. |
| `tests/security/test_local_first_storage_policy.py` | Focused invented-fixture tests for metadata classification and public/private leakage blocking. |
| `execution/_Decomposition/SOFTWARE_DECOMP.md` | PKG-12, DEL-12-01, SOW-029, OBJ-010, AB-00 architecture basis. |
| `docs/_Registers/Deliverables.csv` | Deliverable identity, anticipated artifacts, context/risk notes. |
| `docs/_Registers/ScopeLedger.csv` | Scope ledger row for SOW-029. |
| `TASK_RUN_2026-06-07_0140.md` and package fan-in run `WORKING_ITEMS_RUN_2026-06-07_0150_TP-PKG12-LOCAL-PRIVACY-GUARDS-FANIN.md` | June 7 implementation and fan-in evidence; no lifecycle, approval, storage-schema, runtime-storage, cloud, encryption, or certification claim. |

PDU-036 does not change the storage evidence inventory: the trace-gap fixture is cross-deliverable verification evidence, not a storage runtime, migration, round-trip, or report/export fixture.
