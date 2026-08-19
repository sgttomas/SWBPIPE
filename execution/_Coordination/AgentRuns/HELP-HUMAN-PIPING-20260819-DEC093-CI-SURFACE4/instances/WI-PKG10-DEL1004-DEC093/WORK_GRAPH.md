# Frozen intra-package work graph — version 5

- RunID: `HELP-HUMAN-PIPING-20260819-DEC093-CI-SURFACE4`
- InstanceID: `WI-PKG10-DEL1004-DEC093`
- PackageID: `PKG-10`
- SelectedDeliverables: `DEL-10-04`
- SelectionAuthority: Agent 0 sealed launch brief applying D-65/DEC-093
- Posture: `MIXED` (manager implementation followed by terminal read-only fan-in)
- IntegrationOwner: `WI-PKG10-DEL1004-DEC093`

## Nodes and edges

1. `N1-MANAGER-IMPLEMENT` — WORKING_ITEMS directly implements the bounded
   CI-binding path, focused regression coverage, and deliverable reconciliation.
   Writes: `tools/release/**`, affected source-controlled tests under `tests/**`,
   the DEL-10-04 folder, and this instance. Checks: focused pytest and
   `git diff --check` in-session. Return: frozen N1 diff plus check evidence.
2. `N2-TASK-REVIEW-001` — fresh read-only `TASK + software-code-review` reviews
   100% of the frozen N1 diff and its verification evidence. Writes: runtime-
   owned child status/return only. Result: `FAIL`, four blocking findings.
3. `N3-MANAGER-REMEDIATE` — released after N2. Agent 0 amendments V2/V3 cured
   the exact test-path authorization gap; the manager repaired strict v2
   validation, partial-evidence exclusion, and release-packaging validation,
   then reran focused checks. Result: 90 focused tests passed, one environment-
   unavailable jsonschema case deselected.
4. `N4-TASK-REVIEW-002` — fresh read-only reviewer over 100% of the newly
   frozen integrated diff. Result: interrupted without verdict after a
   post-freeze validation hardening edit invalidated its hash basis; no review
   result from this node is accepted.
5. `N5-TASK-REVIEW-003` — independently instantiated fresh read-only reviewer
   over 100% of the newly frozen integrated diff. Result: `FAIL`, three
   downstream evidence-integrity findings after all 24 hashes and 100% scope
   were verified.
6. `N6-MANAGER-REMEDIATE` — released after N5. The manager rejected
   Git-unverified sweep state in packaging, moved strict summary validation
   before release-gate Git dereference, typed/parsed UTC evidence timestamps,
   added focused regressions, and reran checks. Result: 93 focused tests passed,
   one environment-unavailable jsonschema case deselected.
7. `N7-TASK-REVIEW-004` — separately instantiated fresh read-only reviewer over
   100% of the newly frozen integrated diff. Result: `FAIL`, one contradictory
   dirty-flag/path evidence-integrity finding after all 28 hashes and 100% scope
   were verified.
8. `N8-MANAGER-REMEDIATE` — released after N7. The manager enforced exact
   consistency between `working_tree_dirty` and `dirty_paths`, required empty
   paths when status capture fails, added validator and downstream regressions,
   and reran checks. Result: 95 focused tests passed, one environment-
   unavailable jsonschema case deselected.
9. `N9-TASK-REVIEW-005` — separately instantiated fresh read-only reviewer over
   100% of the final newly frozen integrated diff. Gate: terminal `PASS` with no
   actionable findings.

Edges: `N1-MANAGER-IMPLEMENT -> N2-TASK-REVIEW-001 ->
N3-MANAGER-REMEDIATE -> N4-TASK-REVIEW-002 -> N5-TASK-REVIEW-003 ->
N6-MANAGER-REMEDIATE -> N7-TASK-REVIEW-004 -> N8-MANAGER-REMEDIATE ->
N9-TASK-REVIEW-005`.

## Fan-in and escalation

Fan-in requires write containment, focused checks passing, exact D-65 binding
validation, unchanged host semantics, and accepted independent review. The
committed-HEAD DEC-025 sweep and repository/harness closeout remain with Agent
0/CHANGE. Scope expansion, new evidence semantics, or a required workflow edit
escalates without execution.
