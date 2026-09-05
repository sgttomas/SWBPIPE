# Manager independent spec review — Phase A

Basis: actual AGENT_WORKING_ITEMS.md, project AGENTS.md, software-workflow.json, root SOFTWARE_WORKFLOW_PROFILE; DEL0709 context; DEL0701/02/03 ScopeOfWork; SCA009 Vocabulary_Annex; DAG latest resolved to approved DAG010. DEL0709 anticipated artifact scaffold has no ScopeOfWork, explicitly accepted by parent for contract production. Other implementation surfaces are SOW_V1. No status/lifecycle edits made.

DEL0701 OUT001/AC001/VER001 require stable identity, unit-aware centerline geometry, command-routed edits and explicit missing/protected data. DEL0702 OUT001/AC001/VER001 preserve current editable/read-only distinctions, transient vs durable state, provenance and typed dimensions. DEL0703 OUT001/AC001/VER001 forbid private/protected content and direct persistence bypass. DEL0709 SOW077 owns organization/coverage only. Implementation must be attributed to landing deliverables, not DEL0709.

Applicable CONTRACT invariants: OPS-K-DATA-1/2/3 (no bundled engineering defaults; missing values explicit; provenance), OPS-K-UNIT-1 (dimensional checks), OPS-K-PRIV-1/2 (private data and telemetry), OPS-K-AGENT-1..4 (evidence and sealed scope, draft/acceptance distinction).

The annex historical header remains candidate, but accepted downstream context records rev0.12 / DEC094 acceptance; derivative contracts must cite acceptance rather than reclassifying historical text. Historical rows14/15/16 closures must not be overwritten by broader current usability gaps.

Observed UI seams: PipeViewport command bar exposes five tool choices; PropertyInspector contains support, material and section creation, so feature splitting must give that file a single integrator. OperationApplyPanel calls common validate/apply route and undo/redo; no new direct mutation surface permitted. App/types integration overlaps multiple prospective children and must follow terminal child returns or be owned by one integration child. Test provisioning owned by parent.

Provisional implementation sequence: (1) accepted capability/organization contract; (2) existing-capability wiring (support/hanger/nonlinear, materials, wind, section/deletion) after PKG16 interface return; (3) Tier3 geometry/boundary/self-weight/hanger-library/unit display after upstream semantics return. D58 runtime integration, roadmap R1-R3, engineering defaults and lifecycle acceptance stay gated.

Phase A checks: deterministic repository discovery saved DISCOVERY.json; source-backed manual comparison. Full profile checks will run at integrated implementation boundary; no claim of test pass in this phase.
