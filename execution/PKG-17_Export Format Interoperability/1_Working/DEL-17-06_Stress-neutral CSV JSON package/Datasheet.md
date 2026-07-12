# Datasheet: DEL-17-06 Stress-neutral CSV/JSON package

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-17-06 |
| Package | PKG-17 Export Format Interoperability |
| Name | Stress-neutral CSV/JSON package |
| Type | BACKEND_FEATURE_SLICE |
| Scope Items | SOW-046, SOW-074 |
| Objectives | OBJ-007, OBJ-017, OBJ-018 |
| Runtime pass | Phase A four-document P1/P2 only |

## Attributes

| Attribute | Current value | Source |
|---|---|---|
| Package purpose | Project-controlled stress-oriented neutral package for review, debugging, community adapter development, and comparison across target exporters. It is not a vendor format and not a replacement for a commercial solver input deck. | `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md` section "Stress Neutral CSV/JSON Package" |
| Formats | CSV for inspection/spreadsheet review and JSON for structured import/export. | `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md` section "Stress Neutral CSV/JSON Package" |
| Required relationship to result exports | Result exports are for review, regression comparison, report consumption, headless automation, and governed downstream tooling; they preserve units, diagnostics, provenance, hashes, and professional-boundary limits. | `docs/SPEC.md` result export section; `docs/TYPES.md` `ResultExportEnvelope` |
| Required relationship to DEL-17-02 | Use loss reports and stable IDs for stress-neutral CSV/JSON result packages. | `DEL-17-02/Specification.md` "Downstream Requirements" |
| Required relationship to DEL-17-01 | Do not use CAEPIPE CSV/text behavior as professional acceptance; target-specific CSV coverage remains `TBD` unless source-confirmed. | `DEL-17-01/Specification.md` boundary and downstream rules |

## Conditions

| Condition | Status |
|---|---|
| Source model reference | Required by result-export and handoff scope; exact source model identifier, version, and package reference path are `TBD`. |
| Source model hash basis | Required where the package or manifest records deterministic source evidence; exact payload scope and hash field are `TBD`. |
| Analysis-run reference | Required when exported values come from an analysis run; exact analysis-run identifier, solver version reference, load-case or combination basis, and package reference path are `TBD`. |
| Package member path references | Required by the common export package contract; exact CSV, JSON, manifest, ID-map, loss-report, and validation-report paths are `TBD`. |
| Unit and dimensional metadata | Required for exported result values; exact CSV column names and JSON property names are `TBD`. |
| Stable ID preservation | Required through direct fields or sidecar mapping; exact stress-neutral ID-map layout is `TBD`. |
| Diagnostics and boundary notes | Required so review consumers can see limitations, unresolved assumptions, warnings, and non-authoritative status. |
| Protected/private content handling | Public fixtures and examples must not include protected standards data, proprietary examples, private project data, code allowables, SIF/flexibility tables, or owner criteria. |
| Professional boundary | The package is evidence for review/regression/downstream tooling only; it must not declare code compliance, certification, sealing, professional approval, formal validation, or engineering acceptance. |

## Construction

| Candidate package member | Required role | Current detail level |
|---|---|---|
| Node table | Reviewable node identity and coordinate/result-reference surface. | Table fields `TBD`. |
| Element table | Reviewable element identity and connectivity/result-reference surface. | Table fields `TBD`. |
| Component table | Reviewable component identity and relationship to elements/nodes. | Table fields `TBD`. |
| Restraint/support table | Reviewable support/restraint identity and mapping to model/result records. | Table fields `TBD`; nonlinear/support-state behavior remains source-dependent. |
| Equipment/nozzle table | Reviewable terminal/equipment interface identity and mapping surface. | Table fields `TBD`. |
| Material table | Reference identity/provenance surface only; no bundled proprietary or protected values. | Table fields `TBD`. |
| Section table | Reference identity/provenance surface only; no protected dimensional tables or catalog defaults. | Table fields `TBD`. |
| Load/design case table | Reviewable load/design case identity and basis. | Table fields `TBD`. |
| Units metadata | Declares unit system, dimensions, and conversion basis needed by CSV and JSON consumers. | Exact layout `TBD`. |
| ID map | Maps canonical OpenPipeStress IDs to package rows, JSON objects, or sidecars. | Exact layout `TBD`; consume DEL-17-02 rules. |
| Validation report | Records parse/shape checks, loss entries, warnings, and boundary notes. | Exact layout `TBD`; machine-readable and human-readable forms expected where applicable. |

## Future Detail Slots

These slots are placeholders for later schema, writer, fixture, and validation work. They are not populated by this Phase A/Pass 3 task.

| Slot | Future content | Current disposition | Source basis |
|---|---|---|---|
| CSV fields | Exact table names, column names, ordering, quoting/null rules, and per-table required/optional status. | `TBD` until schema work begins. | Stress-neutral package contents from `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md`; package-member inventory rules from `DEL-17-02/Specification.md`. |
| JSON properties | Exact object layout, property names, required/optional status, schema identifier, and validation profile. | `TBD` until JSON Schema 2020-12 contract work begins. | Architecture basis in `_CONTEXT.md`; `DEL-17-02/Specification.md` architecture-basis requirements. |
| Manifest layout | Package ID, source model/run basis, profile ID, member inventory, hashes where applicable, diagnostics, and boundary notes. | `TBD` layout; required concept retained. | `DEL-17-02/Specification.md` manifest requirements; `docs/SPEC.md` result export and audit-manifest sections. |
| ID-map layout | Canonical ID, package member path, row/object reference, target/generated identity where any exists, and omission/unsupported reason. | `TBD` layout; stable-ID role retained. | `DEL-17-02/Specification.md` stable ID map requirements; export plan stable identity rules. |
| Loss-report layout | Exported, omitted, approximated, delegated, unsupported, and `TBD` behavior with affected IDs and downstream implication. | `TBD` layout; loss report remains mandatory for native project-owned exports. | `DEL-17-02/Specification.md` loss report requirements. |
| Validation-report layout | Shape checks, synchronization checks, source-evidence checks, protected-content checks, diagnostics, and boundary findings. | `TBD`; future implementation evidence only. | `docs/SPEC.md` result export boundary; `docs/IP_AND_DATA_BOUNDARY.md` contribution and public-data policy. |

## References

| Source | Use |
|---|---|
| `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md` | Stress-neutral package purpose, recommended contents, CSV/JSON format split, export risk framing. |
| `execution/_Decomposition/SOFTWARE_DECOMP.md` | DEL-17-06 scope, SOW-046/SOW-074 assignment, PKG-17 exclusions, architecture-basis context. |
| `docs/SPEC.md` | Result export envelope boundary, required result-export metadata, `TBD` status for additional formats. |
| `docs/TYPES.md` | Result/export, stable identity, data-boundary, no-bypass, and professional-boundary vocabulary. |
| `docs/CONTRACT.md` | IP, data, unit, and professional-responsibility invariants. |
| `docs/IP_AND_DATA_BOUNDARY.md` | Public/private/protected data boundary and contribution constraints. |
| `DEL-17-01` four-document kit | Source-basis and target-claim boundaries for PKG-17. |
| `DEL-17-02` four-document kit | Common export package, profile, stable ID map, manifest, and loss-report contract consumed by DEL-17-06. |
## D-41 R5 T2A canonicalization evidence (2026-07-12)

JSON checksum records emitted by DEL-17-06 use `deterministic_sorted_compact_json_payload_hash`: UTF-8 Python JSON with lexicographically sorted keys, compact separators, and ASCII escaping. This is a deterministic local byte contract, not an RFC 8785/JCS claim. CSV members retain `normalized_ascii_lf_text`.
