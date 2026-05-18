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
| Lifecycle Target | SEMANTIC_READY after setup gates pass |

## Attributes

| Attribute | Value |
|---|---|
| Product posture | Local-first by default |
| Protected private data classes | Private rule packs; private material data; private component data; project models; user-owned code/design-basis data |
| Public repository boundary | Public code, schemas, workflows, and invented examples only; no protected standards data or user-private engineering data |
| Cloud posture | Out of MVP unless separately approved by the human project authority |
| Persistence baseline | Versioned JSON-schema-governed persistence with canonical JSON/JCS-compatible hashes for JSON payloads |
| Physical project container | TBD |
| Implementation status | Not implemented by this setup run |
| Real private path creation | None |
| Secret handling | Out of scope for this deliverable run; related work is DEL-12-04 |

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

This setup artifact constructs a documentation-level storage boundary, not a storage implementation. The deliverable output is limited to:

- local-first storage policy requirements in `Specification.md`;
- private path convention guidance in this datasheet and `Guidance.md`;
- future verification expectations in `Procedure.md`;
- semantic matrix/lensing and dependency setup artifacts.

No storage code, schema files, real private directories, sample secrets, test files, cloud service assumptions, or physical project package/container choice are created by this run.

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
| `docs/_Decomposition/SOFTWARE_DECOMP.md` | PKG-12, DEL-12-01, SOW-029, OBJ-010, AB-00 architecture basis. |
| `docs/_Registers/Deliverables.csv` | Deliverable identity, anticipated artifacts, context/risk notes. |
| `docs/_Registers/ScopeLedger.csv` | Scope ledger row for SOW-029. |
