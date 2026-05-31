# NEXT INSTANCE STATE

Last updated: 2026-05-31
Updated by: WORKING_ITEMS

## Authority Pointers

- Decomposition authority:
  `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7`.
- Graph authority: `execution/_DAG/DAG-005/`, as pointed to by
  `execution/_DAG/_LATEST.md`.
- Graph approval record: `execution/_DAG/DAG-005/APPROVAL_RECORD.md`.
- Coordination record: `execution/_Coordination/_COORDINATION.md`.
- Implementation evidence:
  `execution/_Coordination/DEV-001_IMPLEMENTATION_EVIDENCE.csv`.
- Blocker queue derivatives:
  `execution/_Coordination/DEV-001_BLOCKER_QUEUE.md` and `.csv`.
- Handoff prompt: `execution/_Coordination/NEXT_INSTANCE_PROMPT.md`.

## Current Program State

- `SOFTWARE_DECOMP` says what must be built and why.
- `DAG-005` says what depends on what, using approved active edges.
- `DEV-001` says what is currently unblocked or blocked for implementation
  based on committed evidence.
- `_COORDINATION.md` defines the Integrated Verification and Tranche Selection
  Loop: authority intake, state verification, optional read-only integrated
  verification snapshot, gap-to-tranche selection, human approval, bounded
  execution, fan-in, validation, evidence updates, and handoff.
- `DAG-005` remains approved active graph authority. Candidate rows are
  non-gating unless later promoted by explicit human gate and graph
  revalidation.
- `DEV-001_BLOCKER_QUEUE.md` reports 101 implementation-unblocked deliverables
  and 0 blocked deliverables using active `DAG-005` edges only.

## Recent Context

- `TP-VERIFY-017` remediated the three `TP-VERIFY-016` release-readiness gaps:
  witness tooling import surface, DEL-12-04 security wording, and DEL-10-04
  coordination regression command surface.
- `TP-VERIFY-017` recorded `TP-VERIFY-017-RESIDUAL-001`: the DEL-11-04
  invented fake-rule model fixture no longer validated against the current
  model schema and persistence-envelope checks.
- `TP-DEL1104-SCHEMA-ALIGN-001_2026-05-31` was approved and executed by
  WORKING_ITEMS. The tranche aligned the two DEL-11-04 invented model fixtures
  with current straight-pipe orientation and load-record schema requirements,
  recomputed their project hashes, and recorded closeout in
  `execution/PKG-11_Documentation, Examples, and Education/1_Working/DEL-11-04_Invented educational example models/_run_records/TP-DEL1104-SCHEMA-ALIGN-001_2026-05-31.md`.
- Focused validation passed for the DEL-11-04 schema-example remediation:
  `python3 -m pytest tests/test_invented_example_models.py -q` (`7 passed`),
  `python3 -m pytest tests/test_model_schema.py -q` (`4 passed`),
  `python3 tools/coordination/maintain_dev001_coordination.py --dag DAG-005 --check`
  (`VALID`), and `git diff --check`.
- Integrated release-readiness is still not globally closed by this focused
  tranche. No lifecycle state, DAG artifact, DEV-001 evidence row, blocker
  queue, release record, acceptance record, professional claim, or
  code-compliance claim was changed.
- `execution/_Coordination/_COORDINATION.md` now records the canonical
  Integrated Verification and Tranche Selection Loop for steering development
  through remaining objectives.
- `execution/_Coordination/NEXT_INSTANCE_PROMPT.md` was updated to initiate the
  loop before proposing work.

## Immediate Next Actions

1. Read `NEXT_INSTANCE_PROMPT.md`.
2. Read `_COORDINATION.md` and this `NEXT_INSTANCE_STATE.md`.
3. Read `SOFTWARE_DECOMP`, `DAG-005`, and `DEV-001` surfaces as stipulated.
4. Apply the Integrated Verification and Tranche Selection Loop from
   `_COORDINATION.md`.
5. Run `python3 tools/coordination/maintain_dev001_coordination.py --dag DAG-005 --check`
   and record `git status --short` before coordination-sensitive tranche
   selection.
6. Because the pinned DEL-11-04 residual was locally remediated, propose the
   next bounded read-only integrated verification snapshot under
   `execution/_Aggregation/TP-INTEGRATED-VERIFY-###_YYYY-MM-DD/` unless a newer
   authoritative artifact identifies a more specific pinned residual.
7. Do not implement until approved.

## Active Residuals

| ID | Owner | Summary | Required next action |
|---|---|---|---|
| `TP-VERIFY-017-RESIDUAL-001` | `DEL-11-04` / schema-example alignment | Locally remediated by approved `TP-DEL1104-SCHEMA-ALIGN-001_2026-05-31`; focused tests now pass. | Verify through the next read-only integrated verification snapshot before making broader release-readiness claims. |

## Do Not Change Without Explicit Human Approval

- lifecycle `_STATUS.md` files;
- DAG artifacts or dependency registers;
- candidate-edge promotion or graph authority;
- `DEV-001_IMPLEMENTATION_EVIDENCE.csv`;
- blocker queues except through approved deterministic coordination workflow;
- release records or acceptance records;
- professional, certification, sealing, authentication, code-compliance, or
  release-readiness-for-reliance claims.
