# R2 wave plan

This plan is Agent 0's, made under the R0 ruling. It adopts `R0_REVIEW.md` §7,
which says: first wave of 12 deliverables across two packages, one of them
PKG-07; verification at double sampling; scale out only if firm false
alignment is 5% or less and no shared-situation conflicts remain; otherwise
return to the owner.

## Wave 1 (gate wave)

| Package | Deliverables | Worker groups (3–4 each) |
|---|---|---|
| PKG-07 | DEL-07-01 to DEL-07-09 (9) | G1: DEL-07-02, DEL-07-09, DEL-07-01 · G2: DEL-07-06, DEL-07-03, DEL-07-04 · G3: DEL-07-05, DEL-07-07, DEL-07-08 |
| PKG-16 | DEL-16-01 to DEL-16-04 (4) | G1: DEL-16-01, DEL-16-02, DEL-16-03, DEL-16-04 |

**Disclosure.** Wave 1 has 13 deliverables, not 12 (confirmed by the owner,
Direction 6). PKG-07 has 9 deliverables
and no package has exactly 3, so two whole packages cannot total 12. PKG-16 is
chosen because its operation schema, validation and preview, and audit trail
are coupled to PKG-07's palette and editing surfaces, and to the DEL-07-09
vocabulary.

**Grouping.** G1 holds three of the four PKG-07 deliverables that share
Remaining text. DEL-07-06, the fourth, shares text with G1, so the manager's
batch check and the verifier's 100% shared-body sampling cover that pair.

**Topology.**

| Agent | Count | Live agents |
|---|---|---|
| Agent 0 | 1 | 1 |
| WORKING_ITEMS managers (one per package) | 2 | 2 |
| Workers (budget: PKG-07 three, PKG-16 one) | 4 | 4 |
| Total while workers run | | 7 |

After the managers return, Agent 0 launches one fresh verifier per package at
double sampling.

**Gate.** Wave 1 passes when all of these hold:

1. both verifiers return `ACCEPT` or `ACCEPT WITH CONTESTED ROWS`;
2. package firm false alignment is 5% or less;
3. batch mode shows no unresolved shared-situation conflict;
4. every deliverable validates.

**Verifier reruns.** When a verifier returns `RERUN <DEL list>`, Agent 0
relaunches that package's manager (fresh, same brief) with only the named
deliverables as its assignments; the manager's workers take the rerun clause
(old files moved to `superseded_<n>/`, never patched). Agent 0 then launches
a fresh verifier over only those deliverables, at the same sampling. One
rerun cycle per deliverable counts towards the gate: the gate is judged on
the re-verified result. A deliverable still named for rerun after that cycle
goes to the owner with the verifier's findings; it does not block the rest of
the package.

When the gate passes, Agent 0 shows the owner the canonical situation table
with the wave 1 results (R0 ruling item 1) and scales out. When it fails, the
failure and a proposed remedy go to the owner.

**Owner items at the wave 1 checkpoint.** Presented with the canonical table:
(1) any verifier rerun that went to the owner; (2) contested rows. (The
13-deliverable wave was confirmed in Direction 6.)

## Later waves (after the gate)

**Concurrency (owner Direction 6).** Keep 16 live agents, counting Agent 0
and every subagent at any depth, whenever work is available. Run fewer only
for a stated reason (a reserved review slot, or a sequencing dependency such
as the wave 1 gate). After the gate, the remaining 89 deliverables run as one
rolling queue rather than fixed waves:

- Queue order: PKG-00 to PKG-03, then PKG-04 to PKG-06, PKG-08 to PKG-12,
  PKG-13 to PKG-15 and PKG-17 (the order approved at the start).
- Up to 3 managers live. A manager's worker budget is set so that Agent 0 +
  managers + all workers + verifiers = 16. Worker groups stay at 3–4
  deliverables from one package (`R0_REVIEW.md` §7, shared text judged by one
  mind); with 89 deliverables queued this never limits concurrency.
- When a worker returns, its manager launches the next group at once. When a
  package's workers are done, Agent 0 launches its verifier and the next
  package's manager as slots free, rebalancing budgets so the total stays at
  16.
- Each package's verifier runs at standard sampling.

Wave 1 runs 7 live agents: its 13 deliverables form 4 worker groups of 3–4,
and nothing else may start before the gate. That is the stated reason for
running under 16 during wave 1.

Before every sub-batch, Agent 0 runs the stale check:
`git diff --stat 00115c71931bcae79909602d653740d3bb72dfa1 origin/main -- projects/chirality-piping ':!projects/chirality-piping/execution'`.
Any product change marks the affected rows `STALE_INPUT`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
