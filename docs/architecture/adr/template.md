# ADR-NNNN: <short decision title>

| Field | Value |
|---|---|
| Status | proposed \| accepted \| superseded by ADR-NNNN \| rejected |
| Date | YYYY-MM-DD (date of the status shown) |
| Human authority | Required for `accepted`/`rejected`: cite the human ruling (who ruled in role terms, date, and where the ruling is recorded). Without a cited human ruling the decision stays `proposed` and the choice remains `TBD`. |
| Decision log cross-reference | `DEC-NNN` row in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12, or `TBD` |

## Context

What problem or fork forced a decision. State facts with their sources; label
anything unverified `ASSUMPTION` or `TBD`. Record what the status quo bought
(why it existed) as well as why it is insufficient.

## Decision

The decision itself, stated so an implementation agent can act on it without
reinterpreting. Record only what was actually ruled; downstream details that
remain open stay `TBD` here and route to the human-ruling queue.

## Alternatives Considered

Each option that was on the table, with the reason it was rejected or
deferred. An accepted decision with no recorded alternatives is suspect.

## Consequences

What this decision costs and changes: new prerequisites, build/test posture
changes, performance or size impacts, and any new obligations on later work.
Include negative consequences; an ADR with only benefits is incomplete.

## Affected Packages

Which packages/deliverables this decision touches (per the decomposition in
`execution/_Decomposition/SOFTWARE_DECOMP.md`).

## Reconsideration Triggers

Concrete conditions under which this decision must be re-opened and routed
back to human ruling. Every accepted decision must list at least one.

## Source References

Documents, rulings, code locations, and registers this ADR's claims trace to.

## Boundary Note

ADRs record software development decisions only. They are not
release-readiness, professional-approval, certification, sealing,
authentication, or engineering-code-compliance claims. ADRs must stay
code-neutral (no protected standards text, tables, formulas, or proprietary
values) per `docs/CONTRACT.md`.
