# Activation and routing record — TM-PIP-038 through TM-PIP-040

## Status

`PENDING_SHARED_BASELINE_COMMIT — NOT ACTIVE`

This is the minimal durable record of the owner's 2026-08-09 routing act. It
does not activate discovery, treatment, repair, register disposition, or an
evidence-outcome decision by itself. Under
`agents/AGENT_RECONCILIATION.md` R0 and
`docs/DELIVERABLE_CONCORDANCE_METHOD.md` section 6, this record and the ruling
it records must land on the shared baseline before any RECONCILIATION
discovery or dispatch begins.

## Managed-run identity

- Run ID: `HELP-HUMAN-PIPING-20260809-DUAL-LANE-R26`
- Parent instance: `HELP-HUMAN-R26`
- Manager instance: `RECON-TM-PIP-038-040-R26`
- Manager role: `RECONCILIATION`
- Canonical run root:
  `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/TM_PIP_038_040_TREATMENT_2026-08-09/`
- Preparation branch: `codex/piping-reconciliation-tm038-040-20260809`
- Pinned pre-activation base:
  `81c376b41a1e181d3edb0737d4f3c9e398527dbe`
- Selection authority: `HUMAN`
- Posture: `MIXED`

## Routed handoff binding

- Path:
  `projects/chirality-piping/execution/_Coordination/_TaskManagement/DRAFT_RECONCILIATION_HANDOFF_2026-08-08_TM-PIP-038_040.md`
- SHA-256 at the pinned pre-activation base:
  `7bca6073f2ba9aa1c4350ee694e979fb1b04fee561ab09329ba09a1ae3ebdd30`
- Receipt posture: received byte-for-byte; no modification authorized or made.

## Owner routing ruling of record

Source: owner direction in the supervising HELP_HUMAN session, 2026-08-09.

> I am exercising the human/Agent 0 routing gate named in
> DRAFT_RECONCILIATION_HANDOFF_2026-08-08_TM-PIP-038_040.md. Route that
> handoff, unmodified, to a RECONCILIATION instance for adopt/amend/defer/
> decline treatment under its own instrument. Constraints carried with the
> routing: TM-PIP-038 is a bounded factual correction (discovery-pilot
> attribution in the three cited PACKAGE_SUMMARIES files) with exact
> before/after evidence preserved; TM-PIP-039 is a supersession record only;
> TM-PIP-040 terminates in a decision packet for my ruling, not a disposal
> act. RECONCILIATION's treatment verdicts return to me; register
> dispositions remain with TASK_MANAGEMENT under a separate closure ruling;
> the handoff itself confers no register or evidence authority, exactly as
> its own status block states.

## Activated scope upon shared-baseline landing

The ruling activates a source-state-bound, narrower-than-corpus treatment of
only `TM-PIP-038`, `TM-PIP-039`, and `TM-PIP-040` under the ratified shared
method and the RECONCILIATION instrument at the accepted shared-baseline
revision. It does not authorize corpus closure.

- `TM-PIP-038`: evaluate and, only if permitted by preservation rules, execute
  the bounded factual discovery-pilot attribution correction in the three
  cited package summaries, preserving exact before/after evidence.
- `TM-PIP-039`: produce a supersession record only; preserve the historical
  owner quote and act verbatim; do not reinterpret them.
- `TM-PIP-040`: investigate committed provenance and produce an owner-ready
  decision packet only. The absent worktree proves neither restoration nor
  loss. Do not perform a `RESTORED`, `LOST`, or `UNDETERMINED` disposal act.

## Authority and write fences

- Persistent writes are limited to
  `projects/chirality-piping/execution/_Reconciliation/**`.
- The routed handoff confers no register or evidence authority.
- TASK_MANAGEMENT retains register disposition under a separate owner closure
  ruling.
- No lifecycle, release, reliance, merge, or issuance implication is made.
- No stage, commit, push, merge, PR creation, or loop-receipt append is part of
  the RECONCILIATION manager's authority in this run.

## Frozen method and source-state rule

The ratified shared method is
`docs/DELIVERABLE_CONCORDANCE_METHOD.md`; the manager contract is
`agents/AGENT_RECONCILIATION.md`; project reliability and execution riders are
in `projects/chirality-piping/AGENTS.md`. Their accepted revisions, the
activation commit, current decision/decomposition/DAG/lifecycle pointers, and
overlapping-work state must be frozen in `RUN_BASIS.md` only after this
record lands on the shared baseline. The activation commit, not the
pre-activation base named above, will become the discovery source-state
anchor.

## Activation check and required next action

At pre-activation base `81c376b41a1e181d3edb0737d4f3c9e398527dbe`:

- the routed handoff hash matches the binding above;
- Task Management rows `TM-PIP-038` through `TM-PIP-040` exist and remain
  `OPEN`;
- no committed record contains the 2026-08-09 routing ruling, this activated
  scope, its pinned method/source state, and this run pointer together.

Required next action: route this single-file activation record through CHANGE,
land it on the shared baseline, and then resume `RECON-TM-PIP-038-040-R26`
from that accepted commit. Until then, the manager return is
`NEEDS_CHANGE_COMMIT_BEFORE_DISCOVERY`.
