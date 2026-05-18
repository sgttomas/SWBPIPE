# CHANGE Publication Guidance Constraints

## Purpose

This document defines the minimum constraints any publication workflow must satisfy when using the CHANGE agent. It is constraints-based, not a mandated branching model.

## Governing Sources

- `docs/DIRECTIVE.md` Section 2.1 (`filesystem is the database`)
- `docs/DIRECTIVE.md` Section 2.2 (`git is the event store`)
- `docs/CONTRACT.md` K-AUTH-2, K-MERGE-1, K-HIER-1, K-SNAP-1, K-STALE-1, K-STALE-2, K-VAL-1
- `docs/SPEC.md` Section 1 (execution root and tool roots), Section 11 (snapshot immutability)
- `agents/AGENT_CHANGE.md`

## Mandatory Constraints

| ID | Constraint | Invariant/Source |
|---|---|---|
| C-PUB-01 | Publication approval must bind to a specific git SHA. Content changes after approval void approval. | K-AUTH-2 |
| C-PUB-02 | Every execution approval token must include the approved SHA and explicit action list. | K-AUTH-2, AGENT_CHANGE Approval Gate |
| C-PUB-03 | Before executing approved actions, CHANGE must re-check HEAD and refuse execution if `HEAD != approved SHA`. | K-AUTH-2, AGENT_CHANGE Approval-SHA validation |
| C-PUB-04 | Merge to `main` is allowed only when current branch `HEAD == approved SHA` for the run. | K-MERGE-1 |
| C-PUB-05 | Publication artifacts must remain within the flat package/deliverable hierarchy; no extra hierarchy layers may be introduced. | K-HIER-1, SPEC Section 1 |
| C-PUB-06 | State authority remains filesystem + git. Side logs are supplemental evidence, not authoritative state. | DIRECTIVE Section 2.1/2.2 |
| C-PUB-07 | `_Change/` records use immutable snapshots. `_LATEST.md` may move; prior snapshot folders must not be overwritten. | K-SNAP-1, SPEC Section 11 |
| C-PUB-08 | If governed inputs changed since last approved SHA, impacted deliverables must be triaged by a human using `no impact`, `needs rework`, or `needs review`. | K-STALE-1, K-STALE-2, K-VAL-1 |

## Minimum Evidence for a Publish Decision

1. Candidate SHA (`HEAD`) and branch name.
2. Explicit approval token referencing that SHA.
3. Confirmation that the post-approval `HEAD` check passed.
4. Merge precondition check result (`HEAD == approved SHA`).
5. Staleness triage summary for impacted deliverables when applicable.
6. Optional `_Change/` snapshot path if a session log was produced.

## Non-Binding Reference Workflow

1. Capture candidate publish SHA from branch HEAD.
2. Present diff/evidence and staleness advisory to the human.
3. Human approves with explicit token, e.g. `APPROVE: SHA=<sha>; <actions>`.
4. Re-check `HEAD` immediately before execution.
5. Execute approved actions exactly.
6. If merge is included, verify `HEAD == approved SHA` at merge time.
7. Record session evidence in `_Change/` snapshot (optional but recommended).

## Out of Scope

- CI automation of merge gates.
- Branch naming conventions beyond these constraints.
- Runtime dependency/staleness automation tooling.
