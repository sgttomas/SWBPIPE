# Proposed register mutation manifest — TM-PIP-040

Status: `EXACT PROPOSAL — NOT APPLIED`

## Preconditions for any later application

Every condition below must hold at application time or the application must
stop without register mutation:

1. the owner returns the exact `CLOSE` ruling from
   `OWNER_CLOSURE_DECISION_PACKET.md`;
2. the live and closed-register identities equal the frozen hashes in
   `RUN_BASIS.md` or are re-evaluated with an amended owner-ready proposal;
3. exactly one live `TM-PIP-040` row exists, still `OPEN`, field-identical to
   the frozen current row, and no same-ID archive row exists;
4. both evidence paths exist at the exact Git blobs below;
5. all federation, schema, containment, and preservation checks pass; and
6. the later session has explicit write authority for both registers and the
   ordinary receipt/closeout surfaces it will touch.

This manifest confers none of that later authority.

## Exact logical field mutation

The current row has the canonical schema-1.0 25-column order. Preserve every
field except the eight listed below:

| Field | Current exact value | Proposed exact value after owner `CLOSE` ruling |
| --- | --- | --- |
| `Status` | `OPEN` | `CLOSED` |
| `Disposition` | empty | `RESOLVED_BY_DECISION` |
| `EvidenceRef` | empty | `execution/_Reconciliation/DeliverableConcordance/TM_PIP_038_040_TREATMENT_2026-08-09/OWNER_RULING_2026-08-09_TM_PIP_038_040/OWNER_TREATMENT_RULINGS.md; execution/_Reconciliation/DeliverableConcordance/TM_PIP_038_040_TREATMENT_2026-08-09/OWNER_RULING_2026-08-09_TM_PIP_038_040/TM_PIP_040_LOST_OUTCOME_RECORD.md` |
| `EvidenceSha` | empty | `dfc3b8faf0cfe336f4c8a47e4593ea9add134c9b; cc7770df165286d4fb523131f28b7340d41216b8` |
| `EvidenceQuote` | empty | `TM-PIP-040 outcome selection: LOST; Propose closure of TM-PIP-040 as RESOLVED_BY_DECISION at the next TASK_MANAGEMENT closure session, citing this ruling and the recorded outcome; disposition applies only on my closure ruling.` |
| `LastReviewed` | `2026-08-08` | `2026-08-10` |
| `Closed` | empty | `2026-08-10` |
| `Notes` | frozen current value below | frozen current value plus the exact appended clause below |

`EvidenceSha` uses the committed Git blob identities required by the live
schema and established Piping archive practice. `Closed` and `LastReviewed`
mean the as-of date of the new explicit owner closure ruling, not the earlier
investigation, treatment, or `LOST` outcome date. If that ruling is not made
on `2026-08-10`, this proposal must be amended before application.

### Frozen current `Notes`

```text
OWNER_RULING_2026-08-08_HARVEST.md: PROMOTE exactly as recommended. Resolution requires a bounded evidence investigation and an evidence-bound owner disposition; the absent worktree alone proves neither restoration nor loss. No investigation dispatch, reconciliation write, or disposition is authorized before the closeout-routed handoff and its owning gates.
```

### Exact clause to append to `Notes`

```text
; OWNER_CLOSURE_RULING_2026-08-10: CLOSED / RESOLVED_BY_DECISION on the accepted owner LOST outcome recorded in OWNER_TREATMENT_RULINGS.md and TM_PIP_040_LOST_OUTCOME_RECORD.md. Closure records disposition of this attention row only: the original ignored objects remain unavailable, further recovery is declined, and historical test results and ledger encodings remain evidence of record. No TM-PIP-038/039 closure, lifecycle, release, reliance, scope, reconstruction, recovery, product-validation, or professional-approval effect is created.
```

## Exact storage mutation and archive semantics

After the owner ruling and exact field mutation above:

1. `REGISTER.csv` contains one `TM-PIP-040` row in `CLOSED` state long enough
   for schema validation and an archive dry-run.
2. `taskmgmt archive --dry-run` must predict exactly one move.
3. The deterministic archive operation removes exactly that row from
   `REGISTER.csv` and appends the complete, field-identical CLOSED row to
   `REGISTER_CLOSED.csv` without reordering or changing any pre-existing row.
4. Final expected layout, if no unrelated authorized row change intervenes:
   live 33 rows (`OPEN=9`, `DEFERRED=24`, `ELEVATED=0`, `CLOSED=0`);
   archive 7 rows, all `CLOSED`.
5. Combined live-plus-archive identity remains 40 rows. The only semantic
   field changes are the eight named above on `TM-PIP-040`; storage location
   changes from live to archive.

The archive move is mechanical storage layout only. The new explicit owner
ruling is the closure act; this manifest and the helper make no judgment.

## Zero-act statement

Neither register was edited, copied, staged, or archived in this proposal
run. No receipt was appended. This file is a schema-bound decision-support
manifest, not an executable instruction or owner act.
