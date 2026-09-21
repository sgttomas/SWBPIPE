# D-73 — Activation of the whole-corpus deliverable reconciliation

**Status:** PROPOSAL
**Date prepared:** 2026-09-21
**Decision ID:** D-73
**Prepared by:** HELP_HUMAN Agent 0 (Claude Code) under the register's PROPOSAL
convention. Agents prepare packets; only the owner rules (K-AUTH-1; D-GOV-04).
This packet grants nothing. The row lands `AWAITING_RULING`.
**Run record:** `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260921-RECONCILIATION/`
(owner directions verbatim with hashes in `OWNER_DIRECTIONS.md`; approved
Agent 0 plan in `PLAN.md`).

## 1. What is proposed

Activate a reconciliation of **what actually transpired in the code** against
**the deliverables as they stand**, to determine what must change in the
deliverables and whether anything must change in the code. The method is the
root `reconciliation` workflow run as written, adopted for Piping by a new
project profile. Code and tests are evidence, not authority. Discovery
(R0–R3) writes only inside this run's record and evidence folders. The owner
rules direction-of-change at R4.

Why now: 98 of 101 deliverable scope surfaces were last changed in July 2026,
while 979 Piping commits have landed since the July reconciliation froze its
tree, including the desktop application (+81K lines), v0.2 result, run and
export schemas, and substantial physics, operations, runner and solver work.
The owner deliberately deferred deliverable upkeep during that development
and recorded the intent to reconcile once the application reached a first
functional form (active UI run `WORK_GRAPH.json`, `deliverableBindingNote`
and `ownerDirections`). Every loop receipt since has disclaimed deliverable
reconciliation.

## 2. Items for ruling

Items 1–3 restate directions the owner has already given in session
(Directions 1–3 of the run record). The ruling confirms them as register
acts. Items 4–6 are new.

### Item 1 — Scope, frozen source state and suspension

- **Scope:** whole corpus. All 102 deliverables across `PKG-00`–`PKG-17`
  (100 `IN_PROGRESS`, 1 `ISSUED`, 1 `OPEN`). `DEL-01-01` (`ISSUED`) is
  read-only in discovery; any residual routes through the governed
  scope-change process. `DEL-07-09` (`OPEN`) is in scope.
- **Frozen source state:** `00115c71931bcae79909602d653740d3bb72dfa1`
  (`origin/main` after PR #835). The Piping tree at that commit is the same
  tree object as at the PR #834 merge `620ff6387b211c774c0de7dabec0a170acdf6017`
  (`38cbfc64be7e55126c90b82a266846815aa52efb`); PR #835 changed only root
  public-export files.
- **Suspension declaration (Direction 1, owner's words):** "Yes piping is
  suspended at that merger.  The pause will remain in effect until you're
  done." and "No other work will proceed in the project". The suspension
  holds until this run closes or the owner lifts it.

### Item 2 — Pinned method

The run executes under these exact bytes:

| Path (repository root) | Git blob at the frozen state | SHA-256 |
|---|---|---|
| `workflows/reconciliation/WORKFLOW.md` | `b7aa037d8565956e0bde6612fd29ae4599fe95f3` | `75948a77e7ee3ebd9cd9d8a2089c9aff75ca76b49d87260793e6fbdd152fa380` |
| `workflows/reconciliation/execution.json` | `4774e20015cdaafe8772cbc0f7099802111aab8e` | `35e108bccac694cac738db03c61709e3db1886f84a3ee5545a1bbc5bdd4f3ab2` |
| `workflows/reconciliation/resources/contract.md` | `dd997a40757d1d7d0297c6fcce53de6f9e029737` | `da47475cdf12422416d9d0ce16d2aa6bfb744dfed9731a08cf7ca66a05a6e4d5` |
| `workflows/reconciliation/resources/method.md` | `8bccdfb428891c88e7ee21739c831b6a2c42237e` | `d7b5e22a71a7de72445f8d2718d28b43764f9a7e4f069c674aecae1f33ccb86d` |
| `docs/DELIVERABLE_CONCORDANCE_METHOD.md` | `137209cb37e8d8204a7f2bd78114b4b5753c6c2e` | `abf3e78fce606c4557d61cdbfbdb7292a3d858838f6526da6b433d1bcd0ef627` |

plus the project adoption profile `projects/chirality-piping/docs/RECONCILIATION_PROFILE.md`
proposed with this packet, pinned by the SHA-256 recorded in the ruling
record. Per the owner's direction, no earlier project reconciliation plan is
part of the method. Later edits to any pinned file do not change this run's
method without a new ruling.

Extensions to the written method (sealed forward and reverse passes, cause
tags, authority tiers, cause-clustered decision packets, the extended ledger
schema) are **not** adopted by this ruling. They are proposed at R0 in the run's
method addendum and conventions, and take effect only when the owner rules
them there, as the workflow's "human-calibrated conventions" invariant
requires.

### Item 3 — Variance: no `## Remaining` seeding

Kernel §6 seeds a gated item into each in-scope deliverable's `## Remaining`
at activation. **This run seeds nothing.** No deliverable file is written by
activation or discovery. Program state lives where kernel §5 already puts it:
this register row (open/closed, with a pointer to the run) and the run
folders. Owner's words (Direction 1): "I waive the `## Remaining` boostrap
seeding as a ruled variance, because it's stale." Clarified (Direction 2):
"seeding new items was never in my thoughts".

Related, and carried to R0 as a candidate convention rather than ruled here:
existing `## Remaining` entries are stale and are never cited as authority
for what is open or done; they are audited as declared-state claims.

### Item 4 — Phases this ruling covers

- **O-A (recommended): R0–R4.** Discovery, synthesis and the decision gate.
  R5 repair tranches and the R6 closing backcheck need a separate owner
  authorization after the owner has seen the R3 findings and ruled R4.
  Rationale: nothing about R5's content is knowable before R3, and the owner
  has said deliverable changes are a separate act.
- **O-B: R0–R6.** R5 then executes only edits the owner's R4 rulings name;
  every R5 write still needs an R4 ruling. Fewer register acts, but the
  authorization precedes any knowledge of its content.

Under either option, structural outcomes (retire, merge, split or create
deliverables) are handed to scope-change, and implementation defects become
candidate briefs for separate authorization. Neither is an R5 edit.

### Item 5 — Surfaces

*(Amended before ruling, at the owner's request, from a list of new surfaces
only to the complete write and read-only boundary below.)*

**5a. New surfaces**, authorized as ruled program surfaces. They select no
work and are not status surfaces:

- the project profile `docs/RECONCILIATION_PROFILE.md` (kernel §5's recurring
  process asset for this project);
- the run's orchestration record
  `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260921-RECONCILIATION/`
  (work graph, briefs, returns, owner directions, handoff);
- the run's evidence folder
  `execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/`
  (run basis, gate transcript, inventories, ledgers, verifications,
  summaries, findings, decision packets), created at R0.

**5b. Existing surfaces the run writes**, each only as stated:

| Surface | Writes |
|---|---|
| `execution/_Coordination/_DECISIONS/_REGISTER.md` and `D-73_*` | This ruling; the R4 closure note; a new D-row only when an R4 act needs one |
| `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 | `DEC-110` for this ruling; codification of the owner's R4 rulings |
| `loop/LOOP_RECEIPTS.md` | Short versioned receipts at meaningful pauses and at close, as `loop/LOOP_INIT.md` requires |
| `execution/_Coordination/NOTICE_*.md` | Only when another loop is affected |
| GitHub pull requests from this run's branches | Activation, R0 and R3/R4 pull requests under the standing Git authorization |

**5c. Writes outside the repository:**

- a detached scratch checkout of the frozen source state in the session's
  scratch directory, for the gate transcript, builds and tests;
- shared user caches (`~/.cargo`, `~/.npm`, Playwright browsers) that a fresh
  install may populate, with network access for package fetches.

**5d. Named read-only surfaces** (no permission may be inferred):

- every deliverable folder, including `_STATUS.md` and `## Remaining`, until
  a separate R5 authorization;
- `docs/CHECKING_ENTRY_PROFILES.md` (R3 may propose maturity feedback; any
  write waits for a later ruling);
- the Task Management register (R3 may propose dispositions of its rows);
- the in-tree `validation/evidence/sweeps/` (the fresh sweep writes outside
  the tree);
- `execution/_DAG/**`, `execution/_ScopeChange/**`, `docs/_ScopeChange/**`,
  and all code, tests, schemas and fixtures;
- other branches and worktrees, in particular
  `codex/swbpipe-continuation-20260919`.

### Item 6 — Execution parameters on record

Already directed by the owner and recorded here so the register carries them:

- Topology: Agent 0 (HELP_HUMAN, this session) → one WORKING_ITEMS manager
  per package → TASK workers, one owning worker per deliverable; TASK never
  delegates. Nested delegation confirmed by the owner (Direction 2).
- Models: `opus-5`, high reasoning, all roles (Direction 1).
- Concurrency: at most 16 live agents at any instant, counting Agent 0 and
  every agent at any nesting depth; the owner may revise it mid-run
  (Directions 1 and 4). Per-phase allocations are in `PLAN.md`.
- Gate transcript: one fresh run of the registered checks at R0 in a scratch
  checkout of the frozen state outside the repository (Direction 2).
- Only Agent 0 commits. Independent fresh-context review covers the
  activation, R0, R3/R4 and any R5/R6 pull requests.

## 3. Boundaries (all options)

No lifecycle transition, dependency or DAG change, decomposition change,
scope change, product or test change, instruction or workflow change,
cross-project edit, or lifting of any owner hold (including PDU-045 and
PDU-046, the C4 and live-control decisions, and D-68/D-72 criteria) follows
from this ruling. DEC-043's equation-source exclusion and F-PIP-1 to F-PIP-4
apply throughout. Standard claim fence applies (F-PIP-2; claims taxonomy per
DEC-081).

## 4. Risks

- **Size.** About 3,000 claim blocks and a reverse inventory of roughly 1,300
  implementation files. Mitigated by deterministic claim keys, script
  validation after each sub-batch, and capacity-bounded waves.
- **Containment.** The project folder is large and carries ignored build
  state. All builds and tests run in the external scratch checkout with
  caches and outputs redirected; the evidence tree is checked for ignored as
  well as tracked changes.
- **Receipt numbering.** An unmerged records-only branch
  (`codex/swbpipe-continuation-20260919`) already holds a Receipt-161. This
  run numbers its receipts to avoid it and does not touch that branch.
- **One pre-merge launch.** Piping requires independent review before merge,
  and this activation PR is itself a mergeable slice. Its one fresh read-only
  reviewer is the only agent launched before the merge. It reviews this diff
  only and does no reconciliation work.

## 5. On-ruling mechanism

1. Write `D-73_RULING_2026-09-21.md` with the owner's verbatim words and their
   SHA-256, the selected Item 4 option, and the profile's SHA-256.
2. Update this register row to `RULED` with the pointer; append the owner's
   words to §6 below.
3. Append `DEC-110` to `execution/_Decomposition/SOFTWARE_DECOMP.md` §12.
4. Record the ruling in the run record's work graph.
5. Independent fresh-context review of the complete diff; repair and backcheck
   any findings; run the registered checks for the touched paths and the
   receipt validator.
6. Open the PR, pass CI, merge to `main` under the standing Git authorization.
   **No reconciliation agent is dispatched before that merge.**
7. R0 begins from the merged `main`.

## 6. Human ruling and disposition

**RULED 2026-09-21** — owner (Ryan Tufts), in-session decision slate and a
follow-up. Items 1–3 "Rule as written (Recommended)"; Item 4 "O-A: R0–R4
(Recommended)"; Item 6 accepted ("The roles and caps, fresh gate at R0, and
only Agent 0 commits, are all acceptable."); Item 5 amended at the owner's
request and then accepted ("I accept your recommendation for Item 5").
Ruling record with verbatim acts and hashes:
[D-73_RULING_2026-09-21.md](D-73_RULING_2026-09-21.md); codification
`DEC-110` in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12. This section
transcribes the owner's act without broadening it.
