---
run_id: HELP-HUMAN-PIPING-20260820-R6-ENGINEERING
instance_id: WI-PKG14-DEL1404
agent_role: WORKING_ITEMS
package_id: PKG-14
selected_deliverables: [DEL-14-04]
graph_node: N3
status: frozen
plan_version: 1
---

# PKG-14 activation — DEL-14-04 result-category bindings

Accepted basis: repository HEAD `357a58b56726feba49507534159c3fbc4656b818`, approved `DAG-009`, current target `R5`, root/project instructions, `docs/SOFTWARE_WORKFLOW_PROFILE.md`, `projects/chirality-piping/software-workflow.json`, and the current `SOW_V1` DEL-14-04 kit.

Objective: close or materially narrow the exact `_STATUS.md ## Remaining` item “Exercise every named result category and bind comparison outputs separately” by implementing the smallest coherent product/test change that makes each currently supported analysis-run result family explicit, exercised, deterministically ordered, and independently bound in the comparison result.

Scope and acceptance:

- Exercise every family in `SUPPORTED_RESULT_FAMILIES`: `displacement`, `rotation`, `force`, `moment`, `reaction`, `stress`, and `ratio`.
- Keep category bindings deterministic and separate while preserving the existing aggregate `result_deltas` compatibility surface.
- Report unsupported, mismatched, and missing categories explicitly using the accepted diagnostic behavior.
- Do not create or claim an authoritative comparison-result/export schema and do not alter the PDU-011 hold.
- Do not introduce validation, suitability, tolerance, conversion, or engineering-acceptance claims and do not alter the PDU-047 hold.
- Focused pytest checks pass; the registered affected check is `piping-pytest`, with the bounded focused test run authorized for implementation feedback.
- Any `core/**` change receives a fresh, read-only `software-code-review` over 100% of the frozen node diff before manager fan-in.

Declared reads: repository and project instructions; software workflow profile; selected deliverable kit and recent records; current analysis-run comparison engine/contracts; focused analysis-run comparison tests.

Allowed writes:

1. This instance subtree.
2. `projects/chirality-piping/core/comparison/analysis_run/**`.
3. `projects/chirality-piping/apps/desktop/src/features/comparison/**` only if required by the bound output seam.
4. Analysis-run-comparison-specific schemas, fixtures, and focused tests, including `projects/chirality-piping/tests/test_analysis_run_comparison.py`.
5. DEL-14-04 `_STATUS.md`, `MEMORY.md`, and `_run_records/**`.

Exclusions: no tolerance or policy invention; no comparison-result/export schema invention; no engineering validation/suitability claim; no lifecycle, register, decision, DAG, decomposition, PRD, receipt, Git, push, or PR changes; no cloud, network, telemetry, protected/private data, or sibling/shared-run-root writes.

Execution attribution: manager and dispatched children use the inherited Codex runtime model; the runtime does not expose a more specific model identifier to this record.
