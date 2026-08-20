---
run_id: HELP-HUMAN-PIPING-20260820-R6-ENGINEERING
instance_id: WI-PKG08-DEL0801
package_id: PKG-08
selected_deliverable_ids: [DEL-08-01]
graph_node: N2
status: frozen
accepted_basis: 357a58b56726feba49507534159c3fbc4656b818
dependency_authority: DAG-009
target: R5
---

# PKG-08 / DEL-08-01 Activation

Objective: close the exact `_STATUS.md ## Remaining` item “Add the cross-layer TypeScript-to-Rust component-provenance test” with the smallest coherent executable integration proof, fixing a report product seam only if the proof exposes a defect.

Representation: `SOW_V1`. `ScopeOfWork.md` is the target contract and `_STATUS.md ## Remaining` is the executable residual surface.

Dependency posture: DAG-unblocked and independent of parent graph nodes N1 and N3. The current accepted seam is the lossless local-private report-package request produced by TypeScript and consumed by the Rust report-package wire/renderer path. It supersedes the July candidate's stopped pre-package seam without amending that historical candidate.

## Authority and boundaries

- Reads: root/project instructions; ratified software workflow profile; selected deliverable kit and recent run records; DAG-009 dependency evidence; report TypeScript/Rust bridge, focused tests, and invented report fixtures.
- Product/test writes: `apps/desktop/src/features/report/**`; `core/reporting/report_package/**`; narrowly necessary report-generator bridge code; component-provenance/report-specific focused fixtures or tests.
- Evidence writes: this instance subtree; DEL-08-01 `_STATUS.md`, `MEMORY.md`, and `_run_records/**`.
- Exclusions: no `.opsproj` compatibility-policy work; no lifecycle/register/decision/DAG/decomposition/PRD/receipt/Git/push/PR work; no cloud, network, telemetry product feature, protected/private data, or cross-package write.

## Acceptance

The proof must bind TypeScript-produced component-provenance bytes or a lossless exact projection to the Rust consumer/output; cover present provenance, missing provenance as an explicit non-accepted finding/diagnostic, and malformed provenance fail-closed behavior at the Rust boundary; preserve private-only/pending/private-project-data classification; pass focused Vitest and Rust checks; pass write containment; and receive a fresh read-only `software-code-review` PASS over 100% of the frozen node diff. The exact Remaining item is removed only after all gates pass.
