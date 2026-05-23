# Specification: DEL-17-02 Export package, profile, and stable ID map contracts

## Normative Scope

DEL-17-02 shall define common contract requirements for PKG-17 export packages, profiles, stable ID maps, manifests, and loss reports.

This deliverable shall not implement exporters, schemas, tests, parsers, executable harnesses, GUI features, persistence runtime, public API endpoints, target adapters, or target-file writers. Later deliverables shall implement concrete artifacts only after consuming this contract and their own approved source basis.

## Source Authority Requirements

| Req ID | Requirement |
|---|---|
| DEL-17-02-REQ-001 | The deliverable shall consume `DEL-17-01` as the source-basis authority for target-format claims. |
| DEL-17-02-REQ-002 | Target-specific behavior not resolved by `DEL-17-01 Source_Basis_Register.md#TBD Register` or `CAEPIPE_Question_Dossier.md#Question Register` shall remain `TBD`. |
| DEL-17-02-REQ-003 | The common contract shall cite project-owned schema/governance sources for canonical identity, hashing, data-boundary, provenance, lifecycle, and professional-boundary requirements. |
| DEL-17-02-REQ-004 | Later `DEL-17-*` deliverables shall not invent target behavior that is absent from the admitted source basis. |
| DEL-17-02-REQ-005 | Profiles shall record the source IDs consumed from `DEL-17-01`, including `CAEPIPE-IMPORT-MBF`, `CAEPIPE-EXPORT-DATA`, `CAEPIPE-EXPORT-MBF`, `CAEPIPE-BATCH`, `CAEPIPE-PCF`, `GLTF-2.0`, and relevant project references, or explicitly mark the location/source as `TBD`. |

## Architecture-Basis Requirements

| Req ID | Requirement |
|---|---|
| DEL-17-02-REQ-006 | Future concrete export contract schemas derived from this deliverable shall use JSON Schema 2020-12 unless a later approved architecture-basis change supersedes it. |
| DEL-17-02-REQ-007 | Deterministic JSON payload hashing shall use the project canonical JSON/JCS-compatible hash basis where hashes are recorded for package members or manifests. |
| DEL-17-02-REQ-008 | Schema-first command, query, and job-result envelopes remain implementation-facing constraints for later API or adapter work; this deliverable records the contract boundary only. |

## Export Package Requirements

| Req ID | Requirement |
|---|---|
| DEL-17-02-REQ-010 | An export package shall declare a package ID, source model identity, source model version or hash basis, export profile ID, and generation context. |
| DEL-17-02-REQ-011 | An export package shall list every emitted package member, including target files, JSON payloads, ID maps, manifests, loss reports, diagnostics, and sidecars. |
| DEL-17-02-REQ-012 | Package members shall have stable paths or declared path-generation rules. |
| DEL-17-02-REQ-013 | Hashes shall be recorded for deterministic text/JSON members where applicable; binary or target-owned outputs shall either record a hash or record why a hash is unavailable. |
| DEL-17-02-REQ-014 | Timestamp fields shall be explicitly declared as deterministic, normalized, omitted, or intentionally runtime-dependent. |

## Export Profile Requirements

| Req ID | Requirement |
|---|---|
| DEL-17-02-REQ-020 | An export profile shall declare target family, profile version, target version basis, source-basis IDs, and profile boundary notes. |
| DEL-17-02-REQ-021 | An export profile shall declare unit policy, coordinate policy, stable-ID policy, and loss-report policy. |
| DEL-17-02-REQ-022 | An export profile shall classify supported entity families as exported, omitted, approximated, delegated, unsupported, or `TBD`. |
| DEL-17-02-REQ-023 | A target-specific profile shall not treat unresolved `DEL-17-01` questions as support claims. |
| DEL-17-02-REQ-024 | External execution, where relevant, shall be optional and user-owned; the profile may record harness metadata but shall not require bundled commercial executables. |
| DEL-17-02-REQ-025 | Target code/check options shall be recorded only as pass-through target configuration unless a separate public rule-pack design admits local logic. |
| DEL-17-02-REQ-026 | `PLAN-EXPORT-INTEROP` may be cited for strategy and target ordering only while its precise section remains `location TBD`; it shall not substitute for target-format field evidence. |

## Stable ID Map Requirements

| Req ID | Requirement |
|---|---|
| DEL-17-02-REQ-030 | The ID map shall preserve links from canonical OpenPipeStress IDs to emitted target records, files, metadata fields, or sidecar rows. |
| DEL-17-02-REQ-031 | The ID map shall record unmapped canonical IDs with omission or unsupported reasons. |
| DEL-17-02-REQ-032 | The ID map shall be stable across deterministic re-export when the source model identity and profile are unchanged. |
| DEL-17-02-REQ-033 | A target file that cannot carry canonical IDs shall be paired with a sidecar mapping referenced by the manifest. |
| DEL-17-02-REQ-034 | ID-map entries shall distinguish source canonical identity from target-generated identity. |
| DEL-17-02-REQ-035 | ID-map entries shall classify the carrier as `direct_target_carried`, `metadata_carried`, `sidecar_mapping`, or `omitted_target_entry`. |
| DEL-17-02-REQ-036 | A downstream profile shall not claim a direct target-carried ID unless the target carrier is source-confirmed for that profile. |
| DEL-17-02-REQ-037 | CAEPIPE MBF direct ID carrying remains `TBD-17-01-003`; sidecar mapping is the conservative fallback until downstream evidence closes the carrier question. |

## Manifest Requirements

| Req ID | Requirement |
|---|---|
| DEL-17-02-REQ-040 | The manifest shall record source model reference, export package ID, export profile ID, package member inventory, and source-basis references. |
| DEL-17-02-REQ-041 | The manifest shall include diagnostics and boundary notes sufficient to audit unsupported behavior without opening every package member. |
| DEL-17-02-REQ-042 | The manifest shall indicate whether target-specific fields are source-confirmed, delegated, unsupported, or `TBD`. |
| DEL-17-02-REQ-043 | The manifest shall identify sidecar ID-map and loss-report members whenever a target artifact cannot carry canonical identity or loss categories directly. |

## Loss Report Requirements

| Req ID | Requirement |
|---|---|
| DEL-17-02-REQ-050 | The loss report shall be mandatory for every target exporter, including native project-owned exports. |
| DEL-17-02-REQ-051 | Loss report entries shall include affected canonical IDs, category, severity, target artifact if any, reason, source-basis reference, and downstream implication. |
| DEL-17-02-REQ-052 | Approximation entries shall describe the approximation at a contract level and shall not hide target limitations. |
| DEL-17-02-REQ-053 | Unsupported and `TBD` entries shall be blocking when they affect solver-ready or compatibility-sensitive output. |
| DEL-17-02-REQ-054 | Loss reports shall use the categories `exported`, `omitted`, `approximated`, `delegated`, `unsupported`, and `tbd`. |
| DEL-17-02-REQ-055 | `Delegated` entries shall identify the target configuration or external workflow that receives the behavior without claiming local interpretation. |
| DEL-17-02-REQ-056 | `TBD` entries shall name the governing TBD ID when the gap maps to `TBD-17-01-001` through `TBD-17-01-006`. |

## Downstream Requirements

| Downstream deliverable | Required use of DEL-17-02 |
|---|---|
| DEL-17-03 | Use the common package/profile/ID-map/loss-report contract for the native open JSON package. |
| DEL-17-04 | Use the contract to define deterministic CAEPIPE MBF writer behavior without hidden unsupported approximations. |
| DEL-17-05 | Use manifest and profile metadata for optional CAEPIPE external-run harness records and CSV parser boundaries. |
| DEL-17-06 | Use loss reports and stable IDs for stress-neutral CSV/JSON result packages. |
| DEL-17-07 | Use profile and loss-report rules for conservative PCF subset exports. |
| DEL-17-08 | Use stable IDs and metadata/sidecar policy for GLB/glTF review geometry. |
| DEL-17-09 | Use the common contract as the adapter SDK target contract. |

## Boundary Requirements

| Req ID | Requirement |
|---|---|
| DEL-17-02-REQ-090 | The contract shall not include protected standards values, proprietary target examples, private client data, or reverse-engineered target behavior. |
| DEL-17-02-REQ-091 | The contract shall not claim CAEPIPE compatibility, PCF completeness, glTF solver validity, release readiness, code compliance, professional acceptance, or formal validation. |
| DEL-17-02-REQ-092 | All later target deliverables shall carry forward unresolved `DEL-17-01` TBDs until source evidence or human scope authority resolves them. |
| DEL-17-02-REQ-093 | External target execution shall remain optional, user-owned, environment-specific, and license-bound; this contract shall not require or bundle target executors. |
| DEL-17-02-REQ-094 | Public artifacts shall not include protected standards text, protected tables, commercial examples, proprietary model files, private owner criteria, or license-bypass guidance. |

## Acceptance Requirements

DEL-17-02 is acceptable when:

- the four-document kit exists;
- semantic and dependency artifacts exist;
- `Dependencies.csv` validates against the v3.1 dependency schema;
- the DAG-005/root blocker queue uses the active evidence register and records `DEL-17-01` as committed;
- `DEL-17-03` through `DEL-17-09` remain unpopulated beyond existing scaffolds in this tranche;
- no unauthorized schema, code, lifecycle, release, compatibility, or professional claim is made.
