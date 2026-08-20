---
run_id: HELP-HUMAN-PIPING-20260820-R6-ENGINEERING
plan_version: 1
selection_authority: HUMAN+AGENT_0
posture: MIXED
fan_out: terminal
status: frozen
---

# Orchestration Plan v1

Accepted basis: `357a58b56726feba49507534159c3fbc4656b818`, `DAG-009`, target `R5`.

The run has three independent, concurrently selectable engineering nodes. There are no inter-node dependency edges. Each node is owned by one package-scoped `WORKING_ITEMS` instance and writes only its frozen product/evidence scope. `WI-PKG02-DEL0204` owns the run-root shared records and performs the single fan-in write; sibling managers write only beneath their own instance folders.

1. **N1 — PKG-02 / DEL-02-04:** close the executable adapter/plugin verification-layer residual. Checks run in-session through the registered focused adapter/plugin tests and applicable registered evidence-sweep selectors; any `core/**` or desktop product-source change also requires a fresh read-only `software-code-review` over 100% of the frozen node diff.
2. **N2 — PKG-08 / DEL-08-01:** execute the independently activated DEL-08-01 product node under its sealed manager brief. Checks run on the in-session and/or host-capability surfaces named by that brief; the node is selectable only because its manager brief binds an executable check surface.
3. **N3 — PKG-14 / DEL-14-04:** execute the independently activated DEL-14-04 product node under its sealed manager brief. Checks run on the in-session and/or host-capability surfaces named by that brief; the node is selectable only because its manager brief binds an executable check surface.

Integrated fan-in gates: each node returns write-containment evidence, exact changed paths, its registered check results, blockers/reruns, and truthful deliverable residual state; source-changing nodes return the mandatory fresh read-only review result; Agent 0 accepts only non-conflicting node returns and then routes one tranche to CHANGE for integrated review/closeout. Shared receipts, registers, completion logs, and other shared coordination surfaces are not written per node.

Escalation points: authority expansion, cross-package writes, protected/private data, unavailable declared check surfaces, acceptance-policy ambiguity, or shared/gate-surface overlap return to Agent 0 without silent replanning.
