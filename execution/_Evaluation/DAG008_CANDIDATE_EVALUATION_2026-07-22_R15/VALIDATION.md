# Validation — DAG-008 candidate evaluation

## Deterministic candidate checks

All checks ran against `bundle/DAG-008/` from repository root.

| Check | Result |
|---|---|
| Canonical dependency schema | PASS — 31 columns (29 required + `EstimateImpactClass`, `ConsumerHint`), 1,480 rows |
| Strict canonical DAG audit | PASS — 101 nodes, 972 unique directed active edges, 0 SCCs, 0 duplicate active edges, 0 bidirectional pairs, 0 endpoint issues, 0 ragged rows, 0 canonical findings |
| JSON parse | PASS — `dag.json`, `DAG_Audit.json`, `PROVENANCE.json` |
| Candidate manifest | PASS — all 15 non-manifest bundle files match `MANIFEST.sha256` |
| Exact delta | PASS — exactly 13 IDs differ from DAG-007; seven have the authorized three fields and six have the authorized four fields |
| Local source equivalence | PASS — candidate target fields equal the refreshed local rows for 13/13 IDs; DAG-007 extension columns preserved |
| Node identity | PASS — `DeliverableNodes.csv` is byte-identical to DAG-007 |
| Machine graph identity | PASS — all 972 `dag.json.edges` objects equal DAG-007; only proposal identity/provenance metadata differs |
| Topology | PASS — 101 nodes in 15 waves; wave membership equals independent recomputation and DAG-007 |
| Cycle/SCC | PASS — active and active-plus-candidate SCC count 0 |
| Row identity/uniqueness | PASS — 1,480 unique dependency IDs; 1,395 ACTIVE; 85 RETIRED; 0 candidate rows; 30 inherited duplicate-worklist rows |
| Satisfaction enumeration | PASS — all rows: SATISFIED 788, TBD 329, PENDING 123, NOT_APPLICABLE 240; ACTIVE rows: SATISFIED 771, TBD 273, PENDING 112, NOT_APPLICABLE 239 |
| Proposed maturity enumeration | PASS — SEMANTIC_READY 1,323; TBD 57; all remaining recorded values preserved |
| Dependency class/type enumeration | PASS — ANCHOR 378, EXECUTION 1,102; ACTIVE types unchanged (`PREREQUISITE=550`, `OTHER=397`, `CONSTRAINT=325`, `INTERFACE=105`, `HANDOVER=11`, `ENABLES=7`) |
| Approval/pointer boundary | PASS — approval remains TBD/proposed; root `_DAG/_LATEST.md` not modified |
| Path anchors / portability | PASS — 915 files checked, 0 findings, 0 unacknowledged controls, 0 active unclassified artifacts, 0 policy issues |
| Claims-language taxonomy | PASS — 264 files scanned; DEC-081 taxonomy satisfied |
| `git diff --check` | PASS |

## Independent Agent 2 fan-in

| Return | Required scope | Fan-in verdict |
|---|---|---|
| `returns/EDGE_CROSSCHECK/RETURN.md` | 13 exact edge deltas and meaning/scope/decomposition test | VALID — 13/13, no semantic ambiguity, no write |
| `returns/SCHEMA_CROSSCHECK/RETURN.md` | candidate roles, counts, topology, tool hazards | VALID — required artifact roles and all expected counts independently matched, no write |

The returns agree with each other and with deterministic outputs. One non-edge note ambiguity in DEL-10-05 `_DEPENDENCIES.md` has no target-row or candidate effect and remains visible in the edge cross-check.

## Repository-level checks

- Registered full practitioner-harness pytest: PASS — `311 passed in 85.59s`, recorded in `CHECK_registered-harness.json`.
- Practitioner-harness `self-check`: exit 0; no BLOCK findings. Existing unrelated live-tree findings remain visible (`REVIEW=3`, `WARN=8`, `INFO=15`, `NOT_APPLICABLE=2`), including stale references to the retired `AGENT_ORCHESTRATOR.md` name after the live Agent1 rename to PROJECT_SETUP. No such finding was introduced or repaired by this evaluation.

## Tool-hazard disposition

- `dependency_type_rectification.py` was not used as a builder. It is DAG-007-specific and would update `LastSeen` on all 1,480 rows.
- `audit_dag.py` supplied computed JSON facts. Its hard-coded DAG-001/DEV-001 Markdown labels were not used; `DAG_Audit.md` was rendered with DAG-008 candidate identity while preserving the tool's computed facts.

## Write containment

Subject writes remain exactly the six pre-existing PROJECT_SETUP refresh paths. EVALUATION wrote only under `execution/_Evaluation/DAG008_CANDIDATE_EVALUATION_2026-07-22_R15/`. DAG-007, root `_DAG/_LATEST.md`, deliverable lifecycle/status, decomposition, receipts, product, and Git state remain unchanged by EVALUATION.
