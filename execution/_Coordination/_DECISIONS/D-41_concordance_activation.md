# D-41 - Concordance Activation For The Deliverable-Implementation Reconciliation Program

**Status:** PROPOSAL
**Date prepared:** 2026-07-11
**Decision ID:** D-41
**Prepared by:** agent (concordance activation-packet tranche, step 4 of the
owner's 2026-07-11 ruled sequence), under the register's PROPOSAL convention
(agents prepare packets labeled `PROPOSAL`; only the human project authority
rules — K-AUTH-1; D-GOV-04)
**Basis:** `plans/PLAN_2026-07-10_deliverable_implementation_reconciliation.md`
§4 ("Activation, start gate, and source-state control"): activation is a
recorded governance act, never an inference from the plan's existence, and is
"proposed as a new `D-XX` row (next free ID) in
`execution/_Coordination/_DECISIONS/_REGISTER.md` with a PROPOSAL packet."
D-41 is that row. This packet grants nothing: the activation RULING is not
granted here, the row lands `AWAITING_RULING`, and the owner's suspension of
piping work stands unaffected by packet preparation.

## 1. Decision Statement And Scope

Activate the deliverable–implementation concordance program for
OpenPipeStress under
`plans/PLAN_2026-07-10_deliverable_implementation_reconciliation.md` (the
self-contained operative method while the repo-root shared kernel remains
DRAFT, plan §13). The ruling must name the activated scope: **whole-corpus**
or a **named package subset**.

**Recommendation (disclosed, non-binding): whole-corpus.** Per the plan §4
start-gate text, "Under a suspended, frozen tree, whole-corpus discovery is
viable: the stability rationale for subset-scoping evaporates. Package waves
survive for tractability and checkpointing only (§8 R2), not as a stability
control."

Corpus census: the plan's §1 figures (101 deliverables; 92 `IN_PROGRESS` /
8 `CHECKING`→`IN_PROGRESS` pending transcription / 1 `ISSUED`) are declared
planning observations only. Live re-enumeration at packet preparation
(2026-07-11, post-D-40): 101 deliverables across `PKG-00`–`PKG-17`; 100
`IN_PROGRESS`; 1 `ISSUED` (`DEL-01-01`). R0 re-enumerates from the filesystem
regardless (plan §1).

Out of scope under any option: any lifecycle transition by discovery; any
DAG mutation; any cross-project edit; any agent-workflow redesign; any
release, issuance, certification, sealing, professional-approval, or
code-compliance claim (plan §3 boundaries 1–10, §12).

## 2. Precondition Status (Verified 2026-07-11)

The plan §4 "Ordering" item requires both start-gate members and the
rebaseline tranche before discovery begins. Status:

- **Canon-precedence precondition — SATISFIED.** The lifecycle-semantics
  amendment of `docs/TYPES.md` §9 (canonical lifecycle vocabulary per the
  authority table in `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`) was separately
  ruled as D-39, codified `DEC-071` in
  `execution/_Decomposition/SOFTWARE_DECOMP.md` §12, and merged to `main` in
  PR #174 (merge commit `b618ab7d8`). Ruling record:
  `D-39_RULING_2026-07-11.md`.
- **Lifecycle-rebaseline precondition — SATISFIED.** The one-time PKG-00
  rebaseline (plan §3) was ruled as D-40, codified `DEC-072`, and merged to
  `main` in PR #175 (merge commit `6ba158933`): all 8 `PKG-00` deliverables
  (`DEL-00-01`–`DEL-00-08`) are `IN_PROGRESS` (8/8 verified), and
  `DEL-01-01` stays `ISSUED` and change-managed, untouched. Ruling record:
  `D-40_RULING_2026-07-11.md`.
- **Named-ruling start gate — NOT SATISFIED BY THIS PACKET.** See §3. The
  ruling itself supplies it; that is the one gate member this packet cannot
  and does not provide.

## 3. Start Gate — Named Ruling (Supplied By The Ruling, Not This Packet)

Restated from the plan §4 ("Start gate — named ruling precondition"),
verbatim in substance: the start gate is a recorded act, not a prose
judgment. **The activation ruling must record the owner's suspension or
stable-handoff declaration with its date and the commit SHA of the
declared-stable tree.** The declaration of record the plan carries:

> On 2026-07-11, in-session, the owner (Ryan Tufts) stated: "I will suspend
> work in Chirality Piping for the time being."

The activation ruling cites this declaration (or a successor declaration)
together with the SHA at which the tree froze. This packet does not satisfy
the gate: it records that the gate exists and what the ruling must contain.
Until the ruling records the declaration and the frozen-tree SHA, no
concordance work is dispatchable.

## 4. Pinned Method Revision, HARD RULE, And Codification (Plan §4 Items 1–2)

1. **Vehicle and pin.** On ruling, codification follows the project's
   `DEC-XXX` convention in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12
   (next free DEC at ruling time; `DEC-073` as of packet preparation —
   re-derive when ruling). The ruling names the activated scope, and the
   D-41 row plus its `DEC-XXX` codification **pin the exact plan revision —
   the commit SHA on `main` — under which the run executes**; bootstrap
   items and the ruling record cite that pinned revision, and later plan
   edits do not change the method of an in-flight run absent a new ruling.
2. **HARD RULE — the ruling lands on `main` before any dispatch.** The
   ruling record and the register-row flip must be merged to `main` before
   any concordance session is dispatched. Concurrent sessions are mutually
   blind; on 2026-07-10 the sibling project suffered a real governance
   collision — contradictory owner-attributed decision states written by
   parallel sessions, reconciled only after the fact. That case study is why
   this rule is hard.

## 5. Bootstrap Mechanics (Plan §4 Item 3 — Executed At Packet Time)

The plan's bootstrap template, verbatim (with `D-XX` the activation row's
ID, here D-41):

> `- Run claim-level concordance per the reconciliation method (source: plans/PLAN_2026-07-10_deliverable_implementation_reconciliation.md §§6–8 at the D-XX-pinned main revision) (gated: D-XX)`

**Seeding timing per the plan's own text:** §4 item 3 directs seeding at
packet time — "At packet time, each in-scope deliverable's `_STATUS.md`
`## Remaining` is seeded with exactly:" [the template above] — and confirms
the pre-ruling gated existence of the items: "The ruling flips the suffixes
and supplies the pinned commit SHA the `source:` clause resolves against.
Until it does, the item stays gated and nothing under this plan is
selectable."

Accordingly, seeding was executed with this packet: the 100 `IN_PROGRESS`
deliverables' `_STATUS.md` `## Remaining` sections (section added where
absent, per the DEL-04-01 2026-07-10 precedent) each carry exactly:

`- Run claim-level concordance per the reconciliation method (source: plans/PLAN_2026-07-10_deliverable_implementation_reconciliation.md §§6–8 at the D-41-pinned main revision) (gated: D-41)`

Seeding exclusions and adjustments:

- **`DEL-01-01` (`ISSUED`) is not seeded.** Under the amended
  `docs/TYPES.md` §9 regime model, `ISSUED` changes flow only through the
  governed scope-change process; seeding its `_STATUS.md` would be an
  ungoverned edit to an `ISSUED` deliverable. Discovery reads it; any
  DEL-01-01 residual routes through the scope-change process.
- **If the ruling names a package subset**, the gated items outside the
  named scope are inert (gated: D-41, never flipped) and are removed by the
  ruling transcription tranche; the items inside the named scope get their
  suffix flipped and the pinned SHA supplied by the ruling.
- Every seeded item stays gated and unselectable until the ruling flips the
  suffix (plan §4 item 3). Seeding is not activation.

## 6. Why This Is Human-Gated

Activation authorizes the creation of an immutable evidence run
(`execution/_Reconciliation/DeliverableConcordance/<RunID>/`), names the
scope over 100+ governed deliverables, binds a pinned method revision, and
supplies the named-ruling start gate quoting the owner's own suspension
declaration with the frozen-tree SHA. `F-PIP-5` forbids new surfaces without
an owner ruling — the activation ruling is that ruling (plan §8). All of
this is owner authority (K-AUTH-1; D-GOV-04); the plan expressly "has no
authority to activate or select work."

## 7. Options

### 7.1 O-A — Activate, whole-corpus scope (recommended, non-binding)

Activate the program with scope = the whole corpus (all 101 deliverables in
discovery; the 100 `IN_PROGRESS` deliverables carry the seeded bootstrap
items; `DEL-01-01` is read-only with scope-change routing).

- Pro: the plan's own start-gate analysis — under a suspended, frozen tree
  whole-corpus discovery is viable and subset-scoping loses its stability
  rationale; waves remain as tractability/checkpointing partitions (§8 R2);
  one ruling, one pinned revision, no re-activation cycles.
- Con: largest single authorization; R0 calibration (DEL-04-01, DEL-10-05,
  DEL-12-02) must be trusted to catch method defects before wave scale-out.

### 7.2 O-B — Activate, named package subset

Activate with a ruling-named subset (e.g. wave-1 packages `PKG-00`–`PKG-03`
per §8 R2). Items outside the named scope are removed by the ruling
transcription; a later ruling extends scope.

- Pro: smaller first authorization; natural checkpoint before corpus-wide
  commitment.
- Con: the stability rationale for subsetting is gone under the frozen tree
  (plan §4); repeated activation rulings and re-pinning overhead; split
  census risks partial-coverage ambiguity at R6/QA (§10 requires 100%
  indexing of live deliverables regardless).

### 7.3 O-C — Defer activation

Rule nothing now; the row stays open, the seeded items stay gated and inert,
the suspension continues with no concordance program.

- Pro: zero commitment while suspension priorities are elsewhere.
- Con: the corpus's stale setup-era declarations persist unreconciled; the
  warranted-`## Remaining` path back to `CHECKING` (plan §§1, 3) stays
  closed; gated bootstrap items sit indefinitely in 100 status files.

## 8. Risks And Validation Implications

- **Discovery is read-only.** R0–R4 change only run artifacts inside the
  immutable run folder; deliverable and product surfaces change only in
  separately authorized R5 repair tranches (plan §§4, 8).
- **No lifecycle transitions.** Discovery may record
  `LIFECYCLE_REASSESSMENT_REQUIRED` but never applies it; every transition
  remains an owner act under its human gate (plan §3 boundary 5).
- **Suspension unaffected.** Packet preparation and the ruling itself move
  no product work; the suspension declaration is what the ruling records,
  not what it revokes.
- **Run evidence immutable and source-state-bound.** Runs bind to the
  recorded frozen source state; any material source change marks affected
  rows `STALE_INPUT` and forces a rerun, never a memory patch (plan §4).
- **Provenance baselines re-verified, not trusted.** `DAG-007` and the
  `DEC`/lock-review artifacts are provenance baselines, not current truth:
  R0/R1 resolve the live `execution/_DAG/_LATEST.md` pointer and re-verify
  current rows and state against the frozen tree before relying on them
  (plan §3 boundary 6, §5).
- **No new standing surfaces** beyond the three ruled program-state homes
  (run folder; register-row ruling cell; the ruled docs profile surface this
  project selects at activation) (plan §§8–9).

## 9. On-Ruling Mechanism

On ruling: record the owner's suspension (or successor) declaration with
date and frozen-tree SHA in the ruling record (§3); name the activated
scope (§7); pin the plan revision `main` SHA (§4); append the `DEC-XXX`
codification row to `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 citing
this packet; update the `_REGISTER.md` D-41 row to RULED with the pointer;
flip the seeded items' `(gated: D-41)` suffixes and supply the pinned SHA
for in-scope deliverables (removing out-of-scope items if O-B); merge all of
it to `main` **before** any concordance session is dispatched (§4 HARD
RULE). Discovery then begins at R0 under the plan's source-state control.

## 10. Human Ruling And Disposition

**RULED 2026-07-11** — owner (Ryan Tufts), in-session decision slate. Recorded
answers, verbatim: scope — "O-A: Whole-corpus (Recommended)"; declaration —
"Affirm the 2026-07-11 declaration" (reaffirming: "I will suspend work in
Chirality Piping for the time being."); frozen-tree SHA — "551f84ef6 — current
origin/main (Recommended)" = `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.
Pinned plan revision: the same SHA. Ruling record:
[D-41_RULING_2026-07-11.md](D-41_RULING_2026-07-11.md); codification `DEC-073`
in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12; session context
`projects/chirality-piping/loop/LOOP_RECEIPTS.md` Receipt 13. This section
transcribes the owner's act without broadening it.
