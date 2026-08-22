# WI-PKG12-001 Launch Brief v1

- RequestedBy: `HELP_HUMAN`
- RunID: `HELP-HUMAN-PIPING-20260821-LOCAL-FIRST-RUNTIME`
- ParentInstanceID: `HELP_HUMAN`
- InstanceID: `WI-PKG12-001`
- PackageID: `PKG-12`
- DeliverableID: `DEL-12-01`
- Objective: close as much as technically warranted of the open DEL-12-01 residual: "Bind all applicable adapter/plugin and result/report runtime surfaces to governed local-first/private-data enforcement; the selected deny-only declaration gate does not create those runtime bindings."
- AcceptedBasis: `main@1b375af4f`; Receipt 124; DAG-010; DEL-12-01 `ScopeOfWork.md`, `_STATUS.md`, `MEMORY.md`, `_CONTEXT.md`, `_REFERENCES.md`, dependencies and recent run records; project `software-workflow.json`; F-PIP-1..5.
- Dependencies: `CHANGE-001` branch setup must complete first; all DEL-12-01 upstream execution edges are `SATISFIED` in DAG-010.
- DeclaredReads: all repo paths needed to trace current adapter/plugin, result/report, persistence, security-guard, tests, and registered check surfaces.
- AllowedWriteTargets: relevant product and test files under `projects/chirality-piping/**`; DEL-12-01 local `_STATUS.md`, `MEMORY.md`, and `_run_records/**`; this RunID's instance/control records. No `_DomainEngines/**`, other projects, root governance, root agents/skills/tools, decision register, DAG, PRD, lifecycle promotion, publication/release surfaces, or owner-held policy values.
- Tools: read, search, patch, shell tests, managed Agent 2 dispatch.
- RequiredMethod: inspect first; freeze the intra-package graph; implement product behavior; add/extend focused tests; for any `core/**` or `apps/desktop/src/**` change obtain a fresh read-only `TASK + software-code-review` over 100% of the frozen diff with refreshed hashes; remediate and repeat without limit until PASS.
- AcceptanceCriteria: applicable current runtime consumers invoke the governed local-first guard or fail closed; no payload inspection or network/cloud/telemetry behavior; compatibility preserved where authority requires it; focused and full registered tests PASS; terminal review PASS; exact write containment; truthful deliverable residual update.
- ExpectedReturn: validated package return naming files, behavior, tests, review attempts/hashes, remaining residuals, blockers, reruns, and derivative status.
- Escalation: new owner policy/value, hard fence, cross-package write need, destructive action, or scope outside this residual.
