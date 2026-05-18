---
doc_id: AUDIT-DEP-CLOSURE-DECISION-LOG-PKG-12-POST-SETUP-2026-04-30
doc_kind: audit.decision_log
status: completed
created: 2026-04-30
---

# Decision Log

| Decision | Basis | Result |
|---|---|---|
| Use full-workspace analyzer | Existing deterministic tool discovers all dependency CSVs under `execution/`. | Accepted; PKG-12-specific interpretation recorded separately. |
| Treat schema/anchor/evidence results as gates | Dependency-extract QA contract requires v3.1 schema, anchors, and evidence. | PASS. |
| Treat global SCC/bidirectional pairs as warnings | They are outside PKG-12 and predate this package finish. | Warning only for this run. |
| Treat PKG-12 PROPOSAL rows as non-binding | Dependency notes label them as proposals. | Human ruling required before blocker use. |
| Treat DEL-12-03 and DEL-12-05 isolation as warning | Telemetry no-op and threat-model doc can be valid isolated setup deliverables, but graph intent should be confirmed. | Human ruling recommended. |
