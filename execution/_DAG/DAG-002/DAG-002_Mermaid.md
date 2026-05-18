---
doc_id: DAG-002-MERMAID
doc_kind: coordination.dag_visualization
status: generated_from_approved_active_edge_set
created: 2026-05-03
dag_id: DAG-002
decomposition_revision: "0.5"
approval_record: execution/_DAG/DAG-002/APPROVAL_RECORD.md
source_nodes: execution/_DAG/DAG-002/DeliverableNodes.csv
source_edges: execution/_DAG/DAG-002/DependencyEdges.csv
candidate_treatment: excluded_from_active_diagrams
---

# DAG-002 Mermaid Visualization

## Boundary

This file visualizes the approved `DAG-002` revision `0.5` active edge set.
It does not compute blocker readiness, refresh deliverable-local dependency
mirrors, change lifecycle state, dispatch Type 2 work, run `PREPARATION`, or
promote Chirality corpus material.

The package graph below collapses the 859 approved active deliverable edges
into cross-package dependency counts. Edge direction is upstream package to
dependent package, matching the approved dependency direction where
`FromDeliverableID` depends on `TargetDeliverableID`.

Candidate rows are excluded from these diagrams. The 8 candidate rows remain
non-gating, and the 1 retired proposal row is not part of the approved active
edge set.

## Graph Facts

| Fact | State |
|---|---:|
| Packages represented | 17 |
| Deliverable nodes | 92 |
| Approved active edges | 859 |
| Cross-package active edges | 764 |
| Internal same-package active edges | 95 |
| Non-gating candidate rows | 8 |
| Retired proposal row | 1 |
| Active SCCs | 0 |
| Active topological waves | 15 |

## Package-Level View

```mermaid
flowchart LR
  P00["PKG-00<br/>Software Architecture Runway"]
  P01["PKG-01<br/>Governance, IP Boundary, and Professional Responsibility"]
  P02["PKG-02<br/>Domain Model, Units, and Core Schemas"]
  P03["PKG-03<br/>Piping Components, Materials, and Library Data Model"]
  P04["PKG-04<br/>Solver Core and Numerical Methods"]
  P05["PKG-05<br/>Loads, Load Cases, and Stress Recovery"]
  P06["PKG-06<br/>Rule Packs and User-Supplied Code Check Engine"]
  P07["PKG-07<br/>Graphical User Interface and Engineering Workflow"]
  P08["PKG-08<br/>Reporting, Audit, and Reproducibility"]
  P09["PKG-09<br/>Verification, Validation, and Quality Oracles"]
  P10["PKG-10<br/>Build, Packaging, API, and Interoperability"]
  P11["PKG-11<br/>Documentation, Examples, and Education"]
  P12["PKG-12<br/>Security, Privacy, and Private Data Handling"]
  P13["PKG-13<br/>Physical Design Knowledge and Constraint Engine"]
  P14["PKG-14<br/>Model States, Analysis Runs, and Comparison"]
  P15["PKG-15<br/>Handoff and External Prover Workflow"]
  P16["PKG-16<br/>Model Operation and Agent Proposal Framework"]

  P00 -->|16| P01
  P00 -->|35| P02
  P00 -->|48| P03
  P00 -->|30| P04
  P00 -->|25| P05
  P00 -->|35| P06
  P00 -->|56| P07
  P00 -->|42| P08
  P00 -->|20| P09
  P00 -->|35| P10
  P00 -->|25| P11
  P00 -->|35| P12
  P00 -->|28| P13
  P00 -->|35| P14
  P00 -->|28| P15
  P00 -->|28| P16
  P01 -->|10| P03
  P01 -->|4| P06
  P01 -->|5| P08
  P01 -->|4| P09
  P01 -->|1| P10
  P01 -->|7| P11
  P01 -->|4| P12
  P01 -->|3| P13
  P01 -->|1| P15
  P01 -->|2| P16
  P02 -->|10| P03
  P02 -->|8| P04
  P02 -->|6| P05
  P02 -->|6| P06
  P02 -->|6| P07
  P02 -->|4| P08
  P02 -->|6| P10
  P02 -->|2| P11
  P02 -->|2| P12
  P02 -->|8| P13
  P02 -->|6| P14
  P02 -->|1| P15
  P02 -->|3| P16
  P03 -->|1| P04
  P03 -->|1| P05
  P03 -->|9| P07
  P03 -->|1| P08
  P03 -->|1| P09
  P03 -->|1| P10
  P03 -->|2| P12
  P04 -->|4| P05
  P04 -->|4| P07
  P04 -->|1| P08
  P04 -->|6| P09
  P04 -->|2| P10
  P04 -->|3| P11
  P04 -->|3| P13
  P04 -->|1| P16
  P05 -->|1| P06
  P05 -->|5| P07
  P05 -->|5| P08
  P05 -->|2| P09
  P05 -->|2| P10
  P05 -->|1| P13
  P05 -->|2| P14
  P06 -->|3| P07
  P06 -->|2| P08
  P06 -->|2| P11
  P06 -->|2| P12
  P07 -->|3| P11
  P08 -->|1| P09
  P08 -->|3| P10
  P08 -->|2| P11
  P08 -->|2| P12
  P08 -->|5| P14
  P08 -->|2| P15
  P08 -->|1| P16
  P09 -->|1| P10
  P09 -->|3| P11
  P10 -->|1| P11
  P10 -->|5| P15
  P12 -->|1| P07
  P12 -->|1| P08
  P12 -->|3| P10
  P12 -->|2| P15
  P12 -->|1| P16
  P13 -->|3| P07
  P13 -->|2| P15
  P13 -->|2| P16
  P14 -->|4| P07
  P14 -->|5| P08
  P14 -->|5| P15
  P14 -->|3| P16
  P15 -->|3| P08
  P16 -->|3| P07

  classDef arch fill:#e8eef7,stroke:#496b9f,stroke-width:1px,color:#172033;
  classDef foundation fill:#e8f5ee,stroke:#4a7c59,stroke-width:1px,color:#132318;
  classDef product fill:#fff8df,stroke:#a47b20,stroke-width:1px,color:#2a2108;
  classDef rev05 fill:#fdecea,stroke:#b65d54,stroke-width:1px,color:#2c0e0b;
  class P00 arch;
  class P01,P02 foundation;
  class P03,P04,P05,P06,P07,P08,P09,P10,P11,P12 product;
  class P13,P14,P15,P16 rev05;
```

## Wave-Level View

```mermaid
flowchart LR
  W01["Wave 1<br/>8 deliverables<br/>PKG-00:8"]
  W02["Wave 2<br/>2 deliverables<br/>PKG-01:1, PKG-02:1"]
  W03["Wave 3<br/>4 deliverables<br/>PKG-01:2, PKG-02:2"]
  W04["Wave 4<br/>7 deliverables<br/>PKG-01:1, PKG-02:2, PKG-04:1, PKG-05:1, PKG-06:1, PKG-13:1"]
  W05["Wave 5<br/>12 deliverables<br/>PKG-03:2, PKG-04:2, PKG-06:3, PKG-10:1, PKG-11:1, PKG-12:1, PKG-13:1, PKG-16:1"]
  W06["Wave 6<br/>17 deliverables<br/>PKG-03:6, PKG-04:2, PKG-05:1, PKG-06:1, PKG-07:2, PKG-08:1, PKG-11:1, PKG-12:2, PKG-13:1"]
  W07["Wave 7<br/>12 deliverables<br/>PKG-04:1, PKG-05:2, PKG-07:3, PKG-08:1, PKG-09:1, PKG-10:1, PKG-12:1, PKG-13:1, PKG-14:1"]
  W08["Wave 8<br/>2 deliverables<br/>PKG-05:1, PKG-09:1"]
  W09["Wave 9<br/>6 deliverables<br/>PKG-07:1, PKG-08:2, PKG-09:1, PKG-10:1, PKG-11:1"]
  W10["Wave 10<br/>7 deliverables<br/>PKG-07:1, PKG-08:1, PKG-09:1, PKG-10:1, PKG-11:1, PKG-12:1, PKG-14:1"]
  W11["Wave 11<br/>4 deliverables<br/>PKG-09:1, PKG-11:1, PKG-14:1, PKG-15:1"]
  W12["Wave 12<br/>4 deliverables<br/>PKG-10:1, PKG-14:2, PKG-15:1"]
  W13["Wave 13<br/>2 deliverables<br/>PKG-15:1, PKG-16:1"]
  W14["Wave 14<br/>2 deliverables<br/>PKG-15:1, PKG-16:1"]
  W15["Wave 15<br/>3 deliverables<br/>PKG-07:1, PKG-08:1, PKG-16:1"]
  W01 --> W02
  W02 --> W03
  W03 --> W04
  W04 --> W05
  W05 --> W06
  W06 --> W07
  W07 --> W08
  W08 --> W09
  W09 --> W10
  W10 --> W11
  W11 --> W12
  W12 --> W13
  W13 --> W14
  W14 --> W15
  classDef wave fill:#eef4ff,stroke:#5277a3,stroke-width:1px,color:#172033;
  class W01,W02,W03,W04,W05,W06,W07,W08,W09,W10,W11,W12,W13,W14,W15 wave;
```

## Internal Same-Package Active Edges

| Package | Internal active edges |
|---|---:|
| `PKG-01` | 4 |
| `PKG-02` | 8 |
| `PKG-03` | 8 |
| `PKG-04` | 8 |
| `PKG-05` | 5 |
| `PKG-06` | 5 |
| `PKG-07` | 10 |
| `PKG-08` | 8 |
| `PKG-09` | 6 |
| `PKG-10` | 4 |
| `PKG-12` | 6 |
| `PKG-13` | 6 |
| `PKG-14` | 7 |
| `PKG-15` | 6 |
| `PKG-16` | 4 |

## Notes

- `PKG-00` architecture-basis edges dominate the package view because every
  non-`PKG-00` package receives applicable architecture context.
- `PKG-13` through `PKG-16` are revision `0.5` additions and are highlighted
  in the package diagram.
- The wave diagram is dependency-order evidence only. It is not schedule,
  staffing, priority, readiness, or dispatch authority.
