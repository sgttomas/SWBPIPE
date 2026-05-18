# Datasheet: DEL-12-04 Secret and private-library handling

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-12-04 |
| Deliverable name | Secret and private-library handling |
| Package ID | PKG-12 |
| Package name | Security, Privacy, and Private Data Handling |
| Type | SECURITY_CONTROL |
| Scope items | SOW-040, SOW-029 |
| Objective | OBJ-010 |
| Context envelope | M |
| Lifecycle state | SEMANTIC_READY |

Source: `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row `DEL-12-04`; `docs/_Registers/ScopeLedger.csv` rows `SOW-040` and `SOW-029`.

## Attributes

| Attribute | Value | Source |
|---|---|---|
| Primary product posture | Local-first; no cloud service required for modeling, solving, rule checking, or reporting | `docs/PRD.md` section 18.1; `docs/_Decomposition/SOFTWARE_DECOMP.md` SOW-029 |
| Private assets in scope | Private rule packs, private material data, private component data, private libraries, project models, paths, and credential references | `docs/PRD.md` sections 17.3 and 18.3; `_CONTEXT.md` Description |
| Public repository boundary | Public artifacts may define schemas, importers, mechanisms, and invented examples, but must not contain user private rule packs, owner standards, company design bases, protected tables, or proprietary source content | `docs/IP_AND_DATA_BOUNDARY.md` sections 2, 3, and 6 |
| Registry artifact | Private library registry | `_CONTEXT.md` Anticipated Artifacts |
| Test artifact | Secret handling tests | `_CONTEXT.md` Anticipated Artifacts |
| Secret storage posture | Store references to secrets, not secret material, in project, registry, report, or test artifacts | ASSUMPTION from `docs/PRD.md` section 18.3 and `docs/architecture/plugin_boundary.md` Permission Model Skeleton |
| Hash/checksum posture | Rule packs, libraries, exports, reports, and relevant JSON payloads should be checksum-addressable using the accepted canonical JSON/JCS-compatible basis where applicable | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04; `docs/architecture/plugin_boundary.md` Checksums and Provenance |

## Conditions

| Condition | Required handling |
|---|---|
| A private library path is registered | The registry records an opaque path reference, privacy classification, provenance summary, redistribution status, checksum status, and default transmission prohibition. Exact path storage mechanics remain `TBD`. |
| A credential is needed for import or private storage | The product records an opaque credential reference and required permission, not a credential value. The concrete local secret provider remains `TBD`. |
| A private library is exported, attached to a bug report, or included in a shared model/report | The product warns, redacts or omits private values unless the user explicitly requests inclusion under their responsibility. |
| Provenance or redistribution status is missing | The product emits a provenance/private-data diagnostic and does not silently treat the asset as public. |
| Protected or proprietary content is suspected | Ingestion or publication is stopped, the artifact is quarantined outside public examples, and human/legal review is required. |
| A plugin or adapter requests private-library access | Access is denied unless a permission grant exists; plugins and adapters cannot bypass schema, unit, provenance, privacy, protected-content, diagnostic, or report controls. |
| Cloud transmission or telemetry is proposed | It is out of this deliverable unless separately approved; telemetry is off by default and must not include private engineering data. |

Sources: `docs/PRD.md` sections 17.3, 18.2, and 18.3; `docs/IP_AND_DATA_BOUNDARY.md` sections 5 and 6; `docs/architecture/plugin_boundary.md` Private Data Handling; `docs/architecture/extension_domain_contracts.md` Denied-By-Default Behavior.

## Construction

### Private Library Registry Record

Minimum registry fields for this deliverable are descriptive and implementation-facing, not final schema commitments:

| Field | Purpose | Status |
|---|---|---|
| `library_id` | Stable local identifier for a private library record | PROPOSAL |
| `library_kind` | Private rule pack, material library, component library, owner design basis, or project-local library category | PROPOSAL |
| `path_ref` | Opaque local path or storage reference; does not expose secret material | PROPOSAL |
| `privacy_classification` | Marks private/public/export posture | PROPOSAL |
| `redistribution_status` | Records private-only, public-permissive, unknown, or protected-suspected posture using project vocabulary | PROPOSAL |
| `source_provenance` | Source/provenance summary without protected tables or formulas | PROPOSAL |
| `checksum` | Hash/checksum reference when available | PROPOSAL |
| `credential_ref` | Optional opaque credential reference; never a credential value | PROPOSAL |
| `default_transmission_allowed` | Defaults to false for private assets | PROPOSAL |
| `review_status` | Pending, accepted, rejected, quarantined, or `TBD` review disposition | PROPOSAL |

### Secret Handling Tests

Tests should verify behaviors, not use real secrets or private libraries:

| Test area | Expected evidence |
|---|---|
| Registry rejects secret values | Fixture contains only non-secret sentinel markers and opaque references. |
| Export redaction | Private values are omitted or replaced by non-sensitive markers in report/share paths. |
| Telemetry exclusion | Private-library and credential-reference fields are not included in telemetry payloads. |
| Permission denial | Plugin/adapter access to private libraries and secret references fails closed without explicit grant. |
| Quarantine routing | Suspected protected/private content entering a public contribution path emits a diagnostic and routes to review. |

## References

- `_CONTEXT.md` for deliverable identity, scope, anticipated artifacts, and architecture basis injection.
- `docs/CONTRACT.md` for OPS-K-IP, OPS-K-DATA, OPS-K-AUTH, OPS-K-PRIV, OPS-K-RULE, and OPS-K-AGENT invariants.
- `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4 for SOW-029, SOW-040, OBJ-010, PKG-12, and AB-00-01/02/03/04/06/07/08.
- `docs/_Registers/Deliverables.csv`, `docs/_Registers/ScopeLedger.csv`, and `docs/_Registers/ContextBudgetQA.csv` for machine-readable scope.
- `docs/PRD.md` sections 12, 13, 15, 17, and 18 for rule-pack, private-library, report, IP, local-first, telemetry, and private-data handling requirements.
- `docs/IP_AND_DATA_BOUNDARY.md` sections 2 through 7 for public/private data boundary, provenance, quarantine, and report limits.
- `docs/architecture/plugin_boundary.md`, `docs/architecture/extension_domain_contracts.md`, and `docs/architecture/persistence_contract.md` for architecture-basis constraints on permissions, private-data markers, checksums, and no-bypass behavior.
