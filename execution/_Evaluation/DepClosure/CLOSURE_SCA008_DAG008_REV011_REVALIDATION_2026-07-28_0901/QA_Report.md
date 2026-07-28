# QA report

| Check | Result | Evidence |
|---|---|---|
| Local register discovery | PASS — 93 | `Evidence/coverage.csv` |
| Local schema validation | PASS — 93 valid, 0 invalid | `Evidence/local_registers/validation_results.csv` |
| Local row coverage | PASS — 1,480/1,480 | `Evidence/closure_summary.json` |
| Anchor coverage | PASS — 93/93 `IMPLEMENTS_NODE` | `Evidence/closure_summary.json` |
| Orphan endpoints | PASS — 0 | `Evidence/orphans.csv` |
| SCCs / cycles | PASS — 0 | `Evidence/scc_summary.csv`; `Evidence/cycles_sample.csv` |
| Bidirectional pairs | PASS — 0 | `Evidence/bidirectional_pairs.csv` |
| Aggregate schema | PASS — 31 columns, 1,480 rows | `Evidence/dag008_dependency_schema.txt` |
| Canonical aggregate audit | PASS — 0 findings | `Evidence/dag008_audit.json` |
| DAG-008 manifest | PASS — 15/15 | `Evidence/sca008_dag008_compatibility.json` |
| Node parity | PASS — 101/101 | `Evidence/sca008_dag008_compatibility.json` |
| Dependency-ID parity | PASS — 1,480/1,480 | `Evidence/sca008_dag008_compatibility.json` |
| Non-status/notes field parity | PASS — 0 mismatches | `Evidence/sca008_dag008_compatibility.json` |
| Accepted duplicate-retirement delta | PASS — exact 30 | `Evidence/sca008_dag008_compatibility.json` |
| Unique active directed topology | PASS — 972 local / 972 aggregate | `Evidence/sca008_dag008_compatibility.json` |
| Topological waves | PASS — 15 | `Evidence/sca008_dag008_compatibility.json` |

The aggregate carries two extension columns absent from local registers:
`EstimateImpactClass` and `ConsumerHint`. Those extensions are not treated as
local-field mismatches. The 30 status differences are not drift: each local
row remains `ACTIVE`, its aggregate duplicate is deliberately `RETIRED`, and
the set equals `DAG-008_DuplicateEdgeWorklist.csv`.
