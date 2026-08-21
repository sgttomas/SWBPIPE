# WORKING_ITEMS activation — PKG-05 / DEL-05-04

- RunID: `HELP-HUMAN-PIPING-20260820-R9-STALE-HASH`
- InstanceID: `WORKING-ITEMS-PKG05-DEL0504-STALE-HASH`
- Parent: `HELP_HUMAN`
- Role: `WORKING_ITEMS` (Agent 1)
- PackageID: `PKG-05`
- Selected deliverables: `DEL-05-04` only
- Scope representation: `SOW_V1`
- Branch: `codex/piping-product-20260820b`
- Base and activation HEAD: `cd823be3badd034c86390f2707dcf01952c782f0`
- Accepted basis: owner-directed R9 implementation authority; approved `DAG-009`; `DEC-020` operation seam; current R5 stage; `Receipt-120`; the exact open `DEL-05-04` Remaining item.
- Objective: make the authoritative Rust operation-applier runtime fail closed when a supplied claimed model hash cannot be meaningfully compared with, or does not equal, the current backend RFC8785/JCS model hash. Preserve no-claim behavior, normal matching-hash validation/application, and the professional-boundary claims.
- Authority boundary: implementation and evidence only. No lifecycle, review disposition, release, publication, issuance, professional acceptance, certification, sealing, authentication, or code-compliance authority.
- Upstream dependencies: DAG-009 records six active upstream execution rows for DEL-05-04; all are `SATISFIED` at required `SEMANTIC_READY` maturity (`DEL-00-01`, `DEL-00-02`, `DEL-00-03`, `DEL-00-06`, `DEL-00-08`, `DEL-02-03`).
- Downstream consumers: DAG-009 downstream rows are evidence context only and do not expand this activation.
- Writable targets:
  - `projects/chirality-piping/core/model_operations/operation_applier/**`
  - `projects/chirality-piping/fixtures/model_operations/contract_corpus/**` only if needed for parity
  - `projects/chirality-piping/apps/desktop/src/services/operationContractCorpus.test.ts` and `operationService.test.ts` only if needed
  - `projects/chirality-piping/execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-04_Analysis status semantics/_STATUS.md`
  - `projects/chirality-piping/execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-04_Analysis status semantics/MEMORY.md`
  - that deliverable's `_run_records/**`
  - this AgentRuns root
- Exclusions: shared loop receipt, decision register, DAG, PRD, root files, other projects, commits, pushes, and PR operations.
- Checks: focused Rust tests and registered core/profile checks in-session; native-to-Wasm contract corpus parity in-session; DEC-025 surface 4 on host/CI as part of the final five-surface sweep. A declined escalation is recorded as `HOST_RERUN_REQUIRED`; sandbox denial alone is not a substrate limitation.
- Review gate: because `core/**` changes, a fresh read-only `TASK + software-code-review` child must review 100% of the frozen diff. Actionable findings require remediation and fresh re-review to PASS.
- Return contract: validated uncommitted changes, exact commands/results, review evidence, host rerun requirements, deliverable effects, blockers, derivative disposition, and handoff to `HELP_HUMAN`.
- Runtime attribution: inherited GPT-5-based Codex runtime; no substitution recorded at activation.

Standard claim fence applies (F-PIP-2; DEC-081 claims taxonomy).
