# Sealed implementation child brief v1 — A2_HANGER_IMPORT

RequestedBy: WORKING_ITEMS N4_WI_PKG03; Parent: HELP_HUMAN.
RunID: HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY.
ChildInstanceID: A2_HANGER_IMPORT. Role: TASK Agent2, no delegation or sibling messaging. TaskSkill: software-bounded-implementation. ApplyEdits: true ONLY after explicit parent release dispatch. This prepared brief does not launch execution.

## Objective, authority and scope

Implement the accepted row23 hanger-library schema and provenance import boundary, preserving Python/Rust parity. Scope PKG03 DEL0302/0307, SCA009 row23 and section1.4 / DEC049. Contract proposal v2 and invented payload v2 in parent instance are accepted interface basis following independent PASS_DESIGN_ONLY and parent acceptance. DAG010 / decomposition0.12 / initial HEAD740569598f9d remain accepted upstream references; capture actual source hashes at dispatch and return source drift before edits if conflicting. Stable target criteria DEL0302 OUT001/AC001/VER001 (SOW018 OBJ004) and DEL0307 OUT001/AC001/VER001 (SOW019/044 OBJ002/004). Preserve OPS-K-IP1..3/DATA1..3/UNIT1/PRIV1. No lifecycle closure.

## Reads and tools

Read root/project AGENTS, agents/AGENT_TASK.md, skills/software-bounded-implementation/SKILL.md and any companions it requires; docs/SOFTWARE_WORKFLOW_PROFILE.md, project software-workflow.json, selected ScopeOfWork/context/status/references, parent CONTRACT_PROPOSAL_v2.md and INTERFACE_BINDING_v2.json, N_CROSS_CONTRACT_REFUTE returns, existing core/library_import, schemas/component.schema.yaml, existing library tests and product_physics hanger structs/validation/units. Read N2 contract for integration only. Tools read/write/bash within fence, registered check selection/scope validators. No network, Git mutations, delegation, protected data, source scraping or telemetry containing private engineering data.

## Exact source fence (project-relative)

- schemas/hanger.schema.yaml (NEW)
- core/library_import/provenance_checker.py
- core/library_import/README.md
- core/library_import/library_import_document/src/lib.rs
- core/library_import/library_import_document/tests/provenance_parity.rs
- core/library_import/library_import_document/README.md
- tests/test_hanger_library_schema.py (NEW)
- tests/test_library_import_provenance.py
- fixtures/hanger/** (NEW, strictly invented)
- Own A2_HANGER_IMPORT/RETURN.md, STATUS.json, CHECKS.json, SOURCE_HASHES.json only.

No Cargo manifest/lock changes, model schema, product_physics, operation applier, desktop or shared types. Parent N2 owns native src-tauri lib.rs adapter; N1 owns UI library types/manager; propose needed corrections through WORKING_ITEMS only. Any additional source file requires amended brief.

## Implementation contract

Add hanger LibraryKind / hanger_library / hanger_records. Strict standalone JSON Schema2020-12, schema_version1.0.0, explicit library_id/hanger_id identities and names; duplicate IDs reject. Accepted V2 ImportedQuantity magnitude/unit/dimension/provenance maps later to product value/unit but PKG03 does not implement operation generation. Allow incomplete optional hanger values for explicit solve-readiness findings, not hidden defaults. Canonical hanger types variable_spring_hanger/constant_effort_support; source_reference required nonblank. Explicit translational stiffness DOF only. Numeric finite positive checks and compatible force/length/force_per_length units, matching existing unit vocabulary; do not silently assume unit or dimension. All nested numeric quantities need ALL seven provenance fields, applicable disposition checks and quarantine; generic legacy traversal existence check alone is insufficient. User-provided unknown source/license remains review metadata per current accepted checker semantics, not legal acceptance. Hanger path must reject malformed fields/types, unsupported families/DOFs, unit mismatch and invalid nested provenance in both Python/Rust; preserve existing material/section/component outcomes.

Only invented fixtures, no bundled selectable catalog. Snapshot provenance and atomic support configuration replacement are downstream obligations documented in v2; do not implement a competing support resolver. Future operation selection must clear top-level stiffness/nonlinear visibly with whole before/after and retain source snapshot in supported provenance/source_note strings. Add README explanation of limitations and ready-to-consume interface.

## Checks and return

Create tests covering valid invented private-local hanger, library/record/value provenance omissions, nested protected suspicion and public/private disposition, type/unknown-field rejection, compatible/incompatible quantities, duplicate IDs, missing optional values and malformed required source. Cross-language parity of outcomes/codes/paths/severity including hanger cases; existing library regressions unchanged. Do not run unregistered checks absent explicit parent authorization; return exact focused recommended commands (`python3 -m pytest -q tests/test_hanger_library_schema.py tests/test_library_import_provenance.py`; `cargo test --offline --manifest-path core/library_import/library_import_document/Cargo.toml`) for manager execution/authorization. Registered profile applicable checks piping-pytest/evidence-sweep are parent-owned; do not start broad sweep. Scope/hash inspection allowed. If parent release authorizes focused commands, capture full command/exit/output and isolated target path.

Return source enumeration/hashes, implemented behavior, acceptance mapping, tests/evidence, omitted checks and reasons, unresolved defects, source fence compliance, downstream integration patch requirements. No PASS without actual evidence. Fresh read-only review of full frozen source diff follows implementation under manager ownership; this child cannot review its own implementation as independent acceptance. Runtime class delegated-harness-native; role/nondelegation instruction+config asserted, not mechanically enforced; model unavailable unless runtime exposes it. No commits/push/receipt/authority pointers.
