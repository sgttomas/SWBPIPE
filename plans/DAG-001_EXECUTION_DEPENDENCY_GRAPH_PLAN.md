---
doc_id: PLAN-DAG-001-EXECUTION-DEPENDENCY-GRAPH
doc_kind: plan.workflow
status: draft
created: 2026-04-30
authority: human_authorized
active_decomposition: docs/_Decomposition/SOFTWARE_DECOMP.md
active_revision: "0.4"
---

# DAG-001 Execution Dependency Graph Plan

## Purpose

Create the first governed deliverable-level execution DAG for OpenPipeStress software-product development.

The graph represents what predecessor work must exist before each deliverable can be safely executed as product development work. It is built from the seed governance documents, the Chirality agent framework, the v0.4 software decomposition, SCA-001 architecture-basis propagation, and the current execution folders.

This pass answers:

> What depends on what, so every future Type 2 deliverable can receive predecessor work that satisfies its execution dependencies?

This pass does not answer:

> Which deliverables are currently blocked, unblocked, highest priority, or scheduled next?

Blocked/unblocked queue computation remains disabled until a human approves an acyclic DAG.

## Governance Boundary

DAG-001 is a coordination artifact, not product implementation.

Allowed work:

- read all governance, decomposition, register, context, dependency, semantic, review, reconciliation, and audit evidence;
- infer candidate deliverable-to-deliverable execution edges where evidence supports the dependency;
- record edge evidence and confidence;
- detect unknowns, cycles, missing evidence, and unresolved cross-package dependency questions;
- produce graph artifacts under `execution/_DAG/DAG-001/`;
- run dependency schema, cycle, and consistency checks.

Not allowed:

- implement product code;
- edit deliverable production docs as part of DAG authoring;
- mark `PKG-00` as `ISSUED`;
- compute blocked/unblocked queues before DAG approval;
- introduce protected standards/code data, copied proprietary formulas, or code-compliance claims;
- silently resolve cycles or uncertain dependencies.

## Source Hierarchy

Use sources in this order of authority:

1. `AGENTS.md` and applicable agent instructions.
2. `docs/CONTRACT.md`.
3. `docs/IP_AND_DATA_BOUNDARY.md`.
4. `docs/_Decomposition/SOFTWARE_DECOMP.md` revision `0.4`.
5. `docs/_Registers/Deliverables.csv`, `ScopeLedger.csv`, and `ContextBudgetQA.csv`.
6. Deliverable `_CONTEXT.md` files.
7. Deliverable `_DEPENDENCIES.md` files.
8. Existing production four-document kits, where present.
9. `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, `_REVIEW.md`, and `Review_Findings.csv` as evidence aids.
10. `execution/_Reconciliation/PKG-02_FourDocInitialization/` reconciliation and audit summaries.

If sources conflict, preserve the conflict in DAG evidence and route it to `RECONCILIATION`. Do not flatten a conflict into a silent dependency decision.

## Required Prerequisite Reading

Read in this order before acting:

1. `INIT.md`
2. `AGENTS.md`
3. `agents/AGENT_ORCHESTRATOR.md`
4. `agents/AGENT_TASK.md`
5. `agents/AGENT_DELIVERABLE_TASK.md`
6. `agents/AGENT_REVIEW.md`
7. `agents/AGENT_RECONCILIATION.md`
8. `agents/AGENT_AUDIT_DEP_CLOSURE.md`
9. `docs/CONTRACT.md`
10. `docs/IP_AND_DATA_BOUNDARY.md`
11. `docs/VALIDATION_STRATEGY.md`
12. `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`
13. `docs/_Decomposition/SOFTWARE_DECOMP.md`
14. `docs/_Registers/ScopeLedger.csv`
15. `docs/_Registers/Deliverables.csv`
16. `docs/_Registers/ContextBudgetQA.csv`
17. `execution/_Coordination/_COORDINATION.md`
18. `execution/_Coordination/NEXT_INSTANCE_PROMPT.md`
19. `execution/_Coordination/NEXT_INSTANCE_STATE.md`
20. `execution/_ScopeChange/SCA-001_2026-04-30_0045/RUN_SUMMARY.md`
21. `execution/_ScopeChange/SCA-001_2026-04-30_0045/Handoff_State.md`
22. `execution/_Reconciliation/PKG-02_FourDocInitialization/RECONCILIATION_SUMMARY.md`
23. `execution/_Reconciliation/PKG-02_FourDocInitialization/AUDIT_SUMMARY.md`
24. `plans/SCA-001_DOWNSTREAM_FOUR_DOC_REFRESH_PLAN.md`
25. `plans/DAG-001_EXECUTION_DEPENDENCY_GRAPH_PLAN.md`

Do not use `docs/INIT.md`; it is retired.

## Definitions

| Term | Meaning in DAG-001 |
|---|---|
| Node | One deliverable from `docs/_Registers/Deliverables.csv`. |
| Edge | A predecessor dependency between two deliverables. |
| Upstream | Work that the current deliverable needs before safe execution. |
| Downstream | Work that consumes the current deliverable. |
| Execution dependency | A dependency required for product-development execution, not merely conceptual relation. |
| Architecture-basis dependency | A downstream dependency on one or more `PKG-00` architecture-basis deliverables accepted by SCA-001. |
| Evidence-backed edge | An edge with a cited source file, source reference, statement, explicitness, and confidence. |
| Candidate edge | An inferred edge that needs review because evidence is indirect or incomplete. |
| Approved DAG | Acyclic edge set accepted by the human project authority after REVIEW/RECONCILIATION/AUDIT. |

## Output Location

Create:

`execution/_DAG/DAG-001/`

Recommended files:

| File | Purpose |
|---|---|
| `DeliverableNodes.csv` | One row per deliverable node. |
| `DependencyEdges.csv` | Canonical DAG-001 edge register. |
| `dag.json` | Machine-readable graph with nodes, edges, and metadata. |
| `TopologicalWaves.md` | Human-readable execution waves if the graph is acyclic. |
| `Cycle_Report.md` | Cycle and strongly connected component report. |
| `DAG_Audit.md` | Audit summary, source coverage, assumptions, unresolved questions, and acceptance status. |
| `_LATEST.md` | Pointer to the active DAG run folder if the root is reused for later DAGs. |

If additional scratch evidence is useful, place it under `execution/_DAG/DAG-001/evidence/`.

## Node Register

`DeliverableNodes.csv` must include all 73 deliverables from `docs/_Registers/Deliverables.csv`.

Required columns:

- `NodeID`
- `PackageID`
- `DeliverableID`
- `DeliverableName`
- `DeliverableType`
- `ScopeItems`
- `Objectives`
- `ContextEnvelope`
- `LifecycleState`
- `ExecutionPath`
- `ContextPath`
- `DependenciesPath`
- `HasFourDocumentKit`
- `HasSemanticMatrix`
- `HasSemanticLensing`
- `HasReview`
- `SourceRegister`
- `Notes`

Rules:

- `NodeID` equals `DeliverableID`.
- Lifecycle state comes only from `_STATUS.md`.
- `HasFourDocumentKit` is true only when all four production docs exist.
- Missing deliverable folders or missing status files must be reported in `DAG_Audit.md`, not guessed.

## Edge Register

`DependencyEdges.csv` should reuse the existing dependency-register v3.1 schema where possible so project tools can be reused or adapted.

Required v3.1 columns:

- `RegisterSchemaVersion`
- `DependencyID`
- `FromPackageID`
- `FromDeliverableID`
- `FromDeliverableName`
- `DependencyClass`
- `AnchorType`
- `Direction`
- `DependencyType`
- `TargetType`
- `TargetPackageID`
- `TargetDeliverableID`
- `TargetRefID`
- `TargetName`
- `TargetLocation`
- `Statement`
- `EvidenceFile`
- `SourceRef`
- `EvidenceQuote`
- `Explicitness`
- `RequiredMaturity`
- `ProposedMaturity`
- `SatisfactionStatus`
- `Confidence`
- `Origin`
- `FirstSeen`
- `LastSeen`
- `Status`
- `Notes`

DAG-001 conventions:

- `RegisterSchemaVersion`: `v3.1`
- `DependencyClass`: `EXECUTION`
- `AnchorType`: `DELIVERABLE`
- `Direction`: use `UPSTREAM` from the dependent deliverable to its prerequisite.
- `TargetType`: `DELIVERABLE`
- `DependencyType`: one of `ARCHITECTURE_BASIS`, `SCHEMA_CONTRACT`, `DOMAIN_MODEL`, `UNIT_CONTRACT`, `PERSISTENCE_CONTRACT`, `SERVICE_API`, `DIAGNOSTICS_CONTRACT`, `SOLVER_PREDECESSOR`, `LOAD_STRESS_PREDECESSOR`, `RULE_PACK_PREDECESSOR`, `GUI_PREDECESSOR`, `REPORTING_PREDECESSOR`, `VALIDATION_PREDECESSOR`, `SECURITY_PREDECESSOR`, `GOVERNANCE_PREDECESSOR`, `INTEROP_PREDECESSOR`, `DOCS_PREDECESSOR`, `REVIEW_RECONCILIATION`.
- `Explicitness`: `EXPLICIT`, `INFERRED_DIRECT`, or `INFERRED_INDIRECT`.
- `RequiredMaturity`: `SEMANTIC_READY`, `INITIALIZED`, `REVIEWED`, or `ISSUED`.
- `ProposedMaturity`: the maturity expected before downstream execution begins.
- `SatisfactionStatus`: `UNKNOWN`, `SATISFIED`, `UNSATISFIED`, or `CONFLICTED`.
- `Confidence`: `HIGH`, `MEDIUM`, or `LOW`.
- `Origin`: `REGISTER`, `CONTEXT`, `DEPENDENCIES_MD`, `DECOMPOSITION`, `ARCHITECTURE_BASIS`, `REVIEW`, `RECONCILIATION`, or `AGENT_INFERENCE`.
- `Status`: `ACTIVE`, `CANDIDATE`, `CONFLICTED`, or `REJECTED`.

Edge direction means:

`FromDeliverableID` depends on `TargetDeliverableID`.

Example:

`DEL-04-02` depends on `DEL-04-01`, so the row is `FromDeliverableID=DEL-04-02`, `TargetDeliverableID=DEL-04-01`, `Direction=UPSTREAM`.

## Edge Authoring Rules

Create edges conservatively. A useful DAG is sparse, evidence-backed, and acyclic after review.

Always create:

- downstream architecture-basis edges from relevant deliverables to applicable `PKG-00` deliverables;
- schema/data deliverables to the core domain/persistence/unit predecessors they require;
- solver/load/stress deliverables to required unit, domain, diagnostics, and solver predecessor contracts;
- GUI deliverables to required service, domain, diagnostics, and result predecessors;
- reporting/export deliverables to required result, audit, provenance, and report-governance predecessors;
- validation deliverables to the implementation surfaces they test;
- packaging/API/headless runner deliverables to the public API, service, schema, and test-gate predecessors they require;
- security/privacy deliverables to persistence, plugin/adapter, private data, and report/export surfaces they govern.

Do not create:

- edges based only on broad thematic similarity;
- priority edges that mean "should happen earlier" but not "is required by";
- blocker or readiness states;
- hidden package-wide all-to-all dependencies;
- dependencies on protected standards or proprietary engineering data.

When unsure:

- record a `CANDIDATE` edge with `Confidence=LOW` and a precise note; or
- record the unresolved question in `DAG_Audit.md` and route to `RECONCILIATION`.

## Expected High-Level Dependency Shape

Use this shape as a starting hypothesis, not a substitute for evidence:

1. `PKG-00` architecture runway precedes downstream implementation contracts where applicable.
2. `PKG-01` governance/IP/professional-boundary work precedes contribution, protected-content, report-claims, documentation, and public-data acceptance surfaces.
3. `PKG-02` domain model, units, analysis-boundary, plugin contracts, and persistence are foundational for most product implementation.
4. `PKG-03` material/component/library models build on `PKG-02` schemas and units.
5. `PKG-04` solver core builds on `PKG-02` units/domain/diagnostics and consumes component/section properties from `PKG-03`.
6. `PKG-05` load/stress work builds on `PKG-02`, `PKG-03`, and `PKG-04`.
7. `PKG-06` rule-pack work builds on `PKG-02`, `PKG-05`, governance boundaries, and private lifecycle constraints.
8. `PKG-07` GUI work builds on service contracts, domain schemas, persistence, diagnostics, solver/load/result surfaces, and private-data boundaries.
9. `PKG-08` reporting/export work builds on persistence, results, diagnostics, provenance, rule-pack checksum, and professional-boundary policy.
10. `PKG-09` validation work builds on the implementation surfaces being benchmarked and on release quality gates.
11. `PKG-10` API, adapters, headless runner, and packaging build on service/API boundaries, schemas, persistence, tests, security, and result exports.
12. `PKG-11` docs/examples build on the product behavior being documented and governance constraints for examples.
13. `PKG-12` security/privacy work cross-cuts persistence, plugins/adapters, reports/exports, private libraries, telemetry, and threat model concerns.

If this high-level shape conflicts with evidence, prefer the evidence and record the deviation.

## Execution Phases

### Phase 1 - Inventory

Produce the node inventory from `docs/_Registers/Deliverables.csv` and the filesystem.

Required checks:

- 73 deliverables represented;
- 13 package IDs represented;
- every node maps to an execution folder;
- every available `_STATUS.md`, `_CONTEXT.md`, and `_DEPENDENCIES.md` is detected;
- `PKG-02` review/reconciliation artifacts are recorded as available evidence.

### Phase 2 - Evidence Extraction

Extract candidate dependencies from:

- `_DEPENDENCIES.md` files;
- `_CONTEXT.md` architecture-basis blocks;
- deliverable descriptions and anticipated artifacts;
- package descriptions and exclusions;
- scope/objective relationships;
- SCA-001 architecture-basis requirements;
- review/reconciliation/audit outputs.

Record source file and source reference for every edge.

### Phase 3 - Edge Normalization

Normalize all edges into `DependencyEdges.csv`.

Rules:

- no duplicate active edges with identical `FromDeliverableID`, `TargetDeliverableID`, and `DependencyType`;
- no self-dependencies;
- all target deliverables must exist in `DeliverableNodes.csv`;
- uncertain dependencies remain `CANDIDATE`, not `ACTIVE`, unless evidence is strong enough.

### Phase 4 - Graph Construction

Build `dag.json` from active execution edges.

Minimum JSON structure:

```json
{
  "metadata": {
    "dag_id": "DAG-001",
    "created": "2026-04-30",
    "decomposition_revision": "0.4",
    "node_count": 73
  },
  "nodes": [],
  "edges": []
}
```

Each node should include `id`, `package_id`, `name`, `type`, `lifecycle_state`, and `path`.

Each edge should include `id`, `from`, `to`, `type`, `status`, `confidence`, and `evidence_file`.

### Phase 5 - Cycle And Closure Checks

Run deterministic checks:

- all nodes represented;
- all active edge endpoints valid;
- no self-dependencies;
- duplicate edge detection;
- active-edge cycle detection;
- candidate-edge cycle detection as a separate warning layer;
- orphan detection for nodes with no edges;
- high-degree hub scan;
- bidirectional-pair detection.

Use existing tools where applicable:

- `tools/validation/validate_dependencies_schema.py`
- `tools/evaluation/check_dependency_schema.sh`
- `tools/coordination/analyze_dep_closure.py`

If tool assumptions require per-deliverable `Dependencies.csv` files, either generate compatible aggregate/per-deliverable files as part of the DAG output or record the limitation in `DAG_Audit.md`.

### Phase 6 - Topological Waves

If active edges are acyclic, produce `TopologicalWaves.md`.

Wave semantics:

- a deliverable may appear in a wave only after all active upstream dependencies appear in earlier waves;
- candidate edges do not gate wave placement but must be listed as caveats;
- waves are dependency order, not schedule, priority, staffing, or blocked/unblocked state.

If active edges contain cycles:

- do not produce execution waves as authoritative;
- produce a cycle-focused `TopologicalWaves.md` stub that says waves are deferred pending cycle resolution;
- route cycles to `RECONCILIATION`.

### Phase 7 - REVIEW, RECONCILIATION, AUDIT

After the initial graph is authored:

- run `REVIEW` against edge evidence quality and schema conformance;
- run `RECONCILIATION` for cycles, conflicts, cross-package ambiguities, and stale assumptions;
- run `AUDIT_DEP_CLOSURE` or equivalent dependency-closure audit;
- record all findings in `DAG_Audit.md`.

Human approval is required before DAG-001 becomes the accepted coordination basis.

## Acceptance Criteria

DAG-001 is complete when:

- all 73 deliverables are represented as nodes;
- all active edges cite evidence;
- active edge endpoints are valid deliverables;
- no self-dependencies exist;
- duplicate active edges are removed or justified;
- active cycles are either absent or explicitly reported;
- `TopologicalWaves.md` is produced if and only if active edges are acyclic;
- `Cycle_Report.md` reports active and candidate-cycle results;
- `DAG_Audit.md` records source coverage, unresolved questions, tool results, and review requirements;
- blocker computation remains disabled pending human approval.

## Completion Report

Report concisely:

- node count and package coverage;
- active edge count and candidate edge count;
- source coverage by evidence type;
- cycle status;
- whether topological waves were produced;
- unresolved dependency questions;
- REVIEW/RECONCILIATION/AUDIT handoffs;
- explicit confirmation that no blocker queue was computed.

