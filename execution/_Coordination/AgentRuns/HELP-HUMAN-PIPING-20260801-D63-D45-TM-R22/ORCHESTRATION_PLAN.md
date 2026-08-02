---
doc_id: R22-D63-D45-TM-ORCHESTRATION-PLAN
doc_kind: coordination.orchestration_plan
status: ACTIVE_SERIAL_EXECUTION
created: 2026-08-01
version: 3
---

# R22 D-63, D-45, and Task Management execution plan

## Activation and accepted basis

- Parent: `HELP_HUMAN`.
- Managed roles: `HELPS_HUMANS`, `TASK_MANAGEMENT`, `SOFTWARE_DECOMP`, and
  `CHANGE` as assigned by the graph.
- Run ID: `HELP-HUMAN-PIPING-20260801-D63-D45-TM-R22`.
- Frozen Git basis:
  `3c2e816f1072295de15fdcdf924c19b4b66497bc`.
- Branch authorized by the owner and active:
  `codex/piping-d63-d45-rulings`.
- Selected committed loop plan:
  `projects/chirality-piping/loop/WORKPLAN_2026-07-18b_piping_loop.md`,
  loaded through `LOOP_INIT.md` from committed `HEAD`.
- Selection authority: `HUMAN`.
- Descriptive posture: `MIXED`.
- Human rulings in scope:
  - `1) D-63 ruling: Option A.`
  - `2) D-45 ruling: O-B.`
- Human Task Management direction in scope: promote the surfaced D-62
  register-currency concern as a Decisions-domain row, then perform a narrow
  D-62 register-currency correction without reopening or reinterpreting D-62.

The posture is descriptive only. The graph is fully serialized because its
nodes touch shared coordination, decision, decomposition, receipt, and Git
surfaces. No concurrent writes are permitted.

## Amendment lineage

Version 3 supersedes version 2 through
`amendments/HELPS-HUMANS-R22-INTEGRATION/V3.md`. It adds the adoption packet
to N6's write scope solely for an evidence-backed execution-status and pointer
currency correction, and explicitly identifies the N6 closeout outputs. No
accepted ruling, effect, boundary, ordering, risk, or completed node changes.

Version 2 supersedes version 1 through
`amendments/HELPS-HUMANS-R22-INTEGRATION/V2.md`. It corrects only the role and
write ownership of the former `N4_D62_CURRENCY_REPAIR`: K-TM ownership
requires `TASK_MANAGEMENT` to write the owner-promoted Piping register row,
while `HELPS_HUMANS` retains the separate narrow D-62 decision-register
currency repair. The former node is therefore replaced by serialized `N4A`
and `N4B`. Accepted objective, scope, human authority, ordering, risk,
acceptance fences, and all completed N1-N3 state remain unchanged.

## Objective

Execute the owner's D-63 and D-45 rulings, the adopted Task Management
migration, and the directed D-62 register-currency repair; then close the
affected Task Management rows, route the ordinary root-loop notice, validate
the complete tranche, append the governed loop receipt, and hand Git closeout
to `CHANGE`.

## Authority and artifact classes

- The D-63 and D-45 chat statements are human rulings and must be transcribed
  without strengthening them.
- The D-63 adoption packet is proposal history plus its appended ruling
  pointer. It is not a replacement for the ruling record.
- The Piping Task Management register is a loop-owned coordination surface
  authorized by D-63 under F-PIP-5. Its rows are non-authoritative action-item
  records governed by K-TM-1..6.
- Linked migrations cite root rows as sources; they do not modify or close the
  root register.
- Any notice to the root loop is ordinary coordination evidence. The root loop
  alone may use it to close `TM-ROOT-099` or amend its own rows.
- The D-45 codification may implement only O-B's decision semantics; product
  implementation remains separately governed.
- The D-62 repair is register currency only. The D-62 ruling record is not
  reopened, rewritten, or reinterpreted.

## Serialized work graph and ownership

| Node | Owner | Depends on | Writes | Required return / gate |
|---|---|---|---|---|
| `N1_D63_RECORD` | `HELPS_HUMANS` | frozen basis and owner ruling | this plan, `WORK_GRAPH.json`, D-63 ruling record, exactly one D-63 row, adoption-packet status/pointer | Option A faithfully recorded; F-PIP-5 satisfied; no `LOOP_INIT.md` or scope effect |
| `N2_TM_REGISTER_MIGRATION` | `TASK_MANAGEMENT` | `N1_D63_RECORD` | Piping `_TaskManagement/REGISTER.csv` and unique R22 instance records only | schema 1.0 / 25 columns; linked migration of exactly `TM-ROOT-037`, `TM-ROOT-077`–`TM-ROOT-097`, and open `TM-ROOT-053`; no root-register write |
| `N3_D45_CODIFICATION` | `SOFTWARE_DECOMP` | `N2_TM_REGISTER_MIGRATION` | D-45 ruling/application surfaces and exact authorized decomposition/deliverable consequences | O-B only; DEC-077 method and live implementation inputs reverified; no unruled product-direction expansion |
| `N4A_TM_D62_PROMOTION` | `TASK_MANAGEMENT` | `N3_D45_CODIFICATION` | Piping `_TaskManagement/REGISTER.csv` and unique N4A R22 instance records only | owner-directed Decisions-domain row promoted with cited source provenance; no decision-register write or authority effect |
| `N4B_D62_CURRENCY_REPAIR` | `HELPS_HUMANS` | `N4A_TM_D62_PROMOTION` | exactly the D-62 row text in `_DECISIONS/_REGISTER.md` and unique N4B R22 instance records only | current ruling facts reflected without reopening, rewriting, or reinterpreting D-62; no Task Management register write |
| `N5_TM_CLOSURE_NOTICE` | `TASK_MANAGEMENT` | `N4B_D62_CURRENCY_REPAIR` | Piping Task Management closure updates and an ordinary root-loop notice under the Piping coordination surface | dispositions evidence-bound; root register unchanged; notice sufficient for root-loop review of `TM-ROOT-099` |
| `N6_VALIDATE_RECEIPT` | `HELPS_HUMANS` | `N5_TM_CLOSURE_NOTICE` | R22 N6 closeout records, `HANDOFF_STATE.md`, plan/graph closeout status, adoption-packet execution-status/pointer currency only, and exactly one governed Piping loop receipt | graph/row/schema/path/receipt validators pass; blockers and derivative status explicit |
| `N7_GIT_CLOSEOUT` | `CHANGE` | `N6_VALIDATE_RECEIPT` | Git index/commit/push/PR only as separately authorized | containment and staged checks pass; Git action creates no semantic approval |

## Concurrency and failure policy

Every node is serialized against its predecessor. There are no sibling nodes
eligible for concurrent execution and no shared-write exception. A failed or
partial node blocks every declared dependant. The parent may preserve
independent read-only inspection, but any changed objective, scope, authority,
write ownership, risk, or acceptance criterion requires a versioned plan and
brief amendment; consequential changes return to the human.

## Decision and scope fences

1. D-63 selects Option A only: mint the register and use on-demand or
   owner-scheduled `TASK_MANAGEMENT`; do not amend `loop/LOOP_INIT.md`.
2. D-63 creates no product, decomposition, package, deliverable, dependency,
   lifecycle, stage, release, professional-reliance, or code-compliance
   effect.
3. Read
   `execution/_Coordination/OWNER_INTENT_2026-07-31_DESIGN_TOOL_BOUNDARY.md`
   before proposing product direction. It is intent of record, not scope, and
   is not expanded here.
4. The historical maturity survey remains candidate input rather than
   findings of record; any fact acted on must be reverified from live accepted
   sources.
5. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
6. No root-register write, lifecycle transition, release, publication,
   network action, or merge is authorized by this plan.

## Fan-in and terminus

Each manager return must name exact changed paths, checks, blockers, derivative
status, rerun requirements, and the next lawful owner. `HELP_HUMAN` validates
the serialized fan-in. The run terminates only after `N7_GIT_CLOSEOUT`
returns, or earlier with an explicit blocked handoff that names the first
unsatisfied gate and preserves all later nodes as blocked.
