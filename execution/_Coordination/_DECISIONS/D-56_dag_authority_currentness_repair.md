# D-56 — Project-Instruction DAG Authority Currentness Repair

**Status:** `PROPOSAL` / `AWAITING_RULING`
**Date:** 2026-07-25
**Prepared by:** HELPS_HUMANS for HELP_HUMAN
**Register row:** `execution/_Coordination/_DECISIONS/_REGISTER.md` D-56
**Managed run:**
`execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260725-CANDIDATE-BRIEFS-R18/`

## 1. Question and effect

How should stale dependency-authority wording in project `AGENTS.md` be
repaired now that the owner-accepted committed pointer names DAG-008, while
making the instruction resistant to the same drift on a future successor?

This packet is a non-governing `PROPOSAL`. It decides nothing, changes no
authority, and does not authorize an `AGENTS.md` edit.

## 2. Live accepted basis

At committed basis `2f8d35ceb30da734ca6dff24dcab36dded8c9b35`:

- `execution/_DAG/_LATEST.md` names `DAG-008` as the approved graph authority.
- `execution/_DAG/DAG-008/APPROVAL_RECORD.md` records the human owner's acts
  accepting DAG-008 as immutable successor to DAG-007 and authorizing the
  pointer move.
- The approval is limited to dependency authority; lifecycle, selection,
  brief adoption, implementation, D-45, and release remain separate acts.
- Project `AGENTS.md` still names `DAG-007` as current and DAG-001..006 as
  historical.

These are live-tree facts. The project-instruction sentence is stale; there is
no authority conflict about the current DAG because the accepted pointer and
approval record govern.

## 3. Classification — fast reject

Project `AGENTS.md` is a governance/instruction surface. Selecting its durable
currentness rule changes authoritative instructions even though the motivating
fact correction is overdetermined. D-49/`DEC-082` excludes adoption, ruling,
and direction; D-52/`DEC-085` and D-54/`DEC-087` preserve owner gates for new
or changed normative content, authority conflict, and ambiguous fast-reject
classification.

**Result: OWNER GATE.** No four-lens discretionary selection is activated.
HELPS_HUMANS may draft and recommend; the human must select and rule. This
classification has no authority effect.

## 4. Options

### O-A — Minimal identifier refresh

Replace only the stale two-sentence currentness statement with:

> `DAG-008` is the current approved dependency graph authority and the
> satisfaction-currency successor to the canonical type-system rectification
> graph `DAG-007`. `DAG-001` through `DAG-007` remain immutable historical or
> superseded snapshots.

This is the smallest conceptual change, but it will stale on the next DAG.

### O-B — Pointer-resolved currentness rule

Replace only the stale two-sentence currentness statement with:

> The current approved dependency graph authority is the immutable snapshot
> named by `execution/_DAG/_LATEST.md`; consumers must resolve that committed
> pointer rather than pinning a DAG identifier in this instruction. At this
> packet's 2026-07-25 basis, the pointer names `DAG-008`; earlier DAGs remain
> immutable historical or superseded snapshots.

This adds an explicit pointer-resolution instruction and resists identifier
drift while retaining a dated audit fact.

### O-C — Defer

Leave project `AGENTS.md` unchanged. This avoids an edit but preserves the
known contradiction for every later operator to rediscover.

## 5. Recommendation — `PROPOSAL`

Recommend **O-B**. The committed loop plan already treats `_LATEST.md` as the
dependency-authority pointer, and DAG-008's approval separately authorizes it.
This is advisory only, not a ruling, adoption, direction, or edit authority.

## 6. Exact on-ruling mechanism

For **O-A** or **O-B**:

1. record the owner ruling verbatim in a governed D-56 ruling record and bind
   its exact bytes/hash under existing practice;
2. replace only the project `AGENTS.md` two sentences beginning “`DAG-007` is
   the current…” and ending “…historical snapshots.” with the exact chosen
   block in §4;
3. change only D-56 from `AWAITING_RULING` to `RULED` with the ruling pointer;
4. run read-only instruction/path/claims/whitespace/self-check and containment
   checks, then route Git closeout separately.

For **O-C**, record the ruled deferral and update only the register
state/pointer; do not edit project `AGENTS.md`.

Mixed wording or any wider change requires a separately bounded amendment.
Nothing here changes `_LATEST.md`, DAG-008, decomposition, lifecycle, stage,
product, deliverables, scope, or acceptance criteria.

## 7. Preserved boundaries

D-56 remains `AWAITING_RULING`; project `AGENTS.md` remains unchanged.
DAG-008's existing owner acceptance and pointer authorization remain the sole
basis for its currentness. No brief adoption, implementation, activation,
lifecycle, stage, release, issuance, professional, evidence-posture, prover,
publication, merge, push, network, or external act occurs.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

