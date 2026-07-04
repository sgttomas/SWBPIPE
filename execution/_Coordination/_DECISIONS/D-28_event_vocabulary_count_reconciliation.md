# D-28 - Event Vocabulary Count Reconciliation (DEC-041 Forward Note)

**Date prepared:** 2026-07-01
**Prepared by:** WORKING_ITEMS, discharging the cross-project reconciliation
flag raised by the app-dev governance loop on 2026-06-24 and surfaced again by
the 2026-07-01 seed-to-leaf consistency audit.
**Epistemic status:** PROPOSAL (non-governing). Only the human project
authority rules. Nothing here changes lifecycle state, issues deliverables,
creates release readiness, professional approval, certification, sealing,
authentication, code-compliance acceptance, or asserts that any PRD milestone
is met until the human records the ruling.

---

## 1. Decision Statement And Scope

Decide whether to adopt this record as the **forward reconciliation** of the
stale "42-type event vocabulary" phrase inside the ruled entry `DEC-041`
(`execution/_Decomposition/SOFTWARE_DECOMP.md` §12, line 611), binding the
event-vocabulary count for all future consumers to the authoritative app-dev
source file instead of editing ruled history in place.

This is a doc-only reconciliation record. It does not reopen or re-rule
`DEC-041` (the harness-as-versioned-packages substrate ruling stands
unchanged), does not adopt `D-21` or v0.2/R7 scope, does not relax the
`DEC-041` automation condition, does not bind any live agent, and does not
consume any app-dev package. No lifecycle, release-readiness, professional
approval, certification, sealing, authentication, or code-compliance claim is
created.

## 2. Verified Facts (Checked Cold, 2026-07-01)

| Check | Result |
|---|---|
| Authoritative source, current count | `projects/chirality-app-dev/frontend/packages/harness-contract/src/event-schema.ts` (`HARNESS_EVENT_TYPES`) enumerates **43** event types at app-dev HEAD `0d2956ccb` (2026-07-01); the final array member is `runtime.mirror.error`. D-APP-47 later made the package path the forward source surface by retiring the frontend harness shims. |
| `DEC-041` text | `execution/_Decomposition/SOFTWARE_DECOMP.md:611` reads "the 42-type event vocabulary" — **stale** against the live source. |
| Was `DEC-041` wrong when ruled? | No. The file held 42 types from `e39d07827` (2026-06-13) through `2df56aa18` (2026-06-21); `DEC-041` was ruled 2026-06-18, so "42-type" was accurate at ruling time and went stale afterward. |
| What changed, and when | The 43rd member, `turn.interrupted` (`:20`), was added by commit `86e934f39` ("Complete ADQ-05 runtime taxonomy reconciliation", 2026-06-21) — the last commit touching the file. `runtime.mirror.error` has been present since `6b23eb96c` (2026-06-13) and is the final member by position, not the newest addition. |
| Cross-project flag (correction note) | `projects/chirality-app-dev/plans/artifacts/bridge_appdev_contribution_for_tier0_2026-06-21.md:67-72` — the 2026-06-24 D-APP-46 hygiene correction: the cell originally read "exactly 42"; the true count is **43**; "any version-pinning must use 43"; the piping-side `DEC-041` stale count is explicitly flagged for reconciliation. |
| Cross-project flag (handoff) | `projects/chirality-app-dev/plans/artifacts/handoff_tier0_governance_residual_cleanup_2026-06-24.md` Residual 6 assigns the "piping half" of the count reconciliation to this loop. |

Precision note on the correction note: its count correction (42 → 43) verifies
cold and is adopted here. Its attribution of the new member to
`runtime.mirror.error` does not verify — the member added after the artifact
(and after `DEC-041`) was authored is `turn.interrupted`; `runtime.mirror.error`
merely shifted to `:46`. This detail changes nothing about the reconciliation:
the keystone count is 43 either way.

## 3. Constraint: `DEC-041` Is Immutable Ruling History

`DEC-041` sits in the ruled decisions table of
`execution/_Decomposition/SOFTWARE_DECOMP.md` §12 and is the ruling record
pointed to by register row `D-22` (RULED, 2026-06-18). Ruled `DEC` entries are
treated as immutable ruling history: existing practice supersedes rather than
edits (the `D-14` hold `DEC-029` was superseded by `DEC-035`, not rewritten),
and the same snapshot-immutability convention is what the app-dev contract
names `K-SNAP-1`. `DEC-041` therefore must **not** be edited in place, even to
fix a count that has since gone stale. The handoff's suggested in-place fix
("reconcile DEC-041's '42-type' to 43") is discharged by this forward record
instead, which preserves the ruling text as ruled.

## 4. Options

| ID | Option | Consequence |
|---|---|---|
| **O-A** | **Adopt this record as the forward reconciliation; `DEC-041` stays unedited.** | The event-vocabulary count binds to the live source (§5 rule). The app-dev 2026-06-24 flag is discharged. Ruling history stays immutable. |
| **O-B** | **Edit `DEC-041` in place to read "43-type".** | Rewrites a ruled entry; breaks the immutability convention and the `D-22` pointer's fidelity to what was actually ruled; the count would go stale again at the next vocabulary change. |
| **O-C** | **Take no piping action.** | The cross-project flag stays open; a future Flow-A version pin or contract extraction may inherit the stale prose count. |

## 5. Recommended Disposition (PROPOSAL)

Recommend **O-A**, with the following binding rule if accepted:

> Any version-pinning, contract-package extraction, or schema work relying on
> the harness event vocabulary (the `DEC-041` keystone dependency, whose
> execution remains gated behind `D-21` and the `DEC-041` automation
> condition) takes the count from the live enumeration in
> `projects/chirality-app-dev/frontend/packages/harness-contract/src/event-schema.ts` at the
> commit being pinned — **43** as of 2026-07-01 (app-dev HEAD `0d2956ccb`;
> count established by `86e934f39`) — never from the prose of `DEC-041` or any
> other decision record. Prose counts in decision records are snapshots of
> ruling-time state, not contracts.

Rationale: the source file is the single authoritative enumeration; `DEC-041`
was accurate when ruled and its prose is history, not a pin; the app-dev
bridge artifact's 2026-06-24 correction note is the cross-project flag this
record discharges, and its substantive content (true count 43, pin against 43)
verifies cold.

## 6. Consequence For Readers

`DEC-041` remains immutable history and continues to read "the 42-type event
vocabulary" — that phrase was accurate on 2026-06-18 and is not an error in
the ruling. Readers encountering it are directed here: the authoritative count
is whatever `event-schema.ts` enumerates at the relevant commit (43 as of
2026-07-01). Register row `D-22` is untouched.

## 7. Human Ruling And Disposition

**Ruling recorded:** O-A approved as recommended by owner (Ryan Tufts), 2026-07-01.

## 8. Ruling Mechanism

Per existing practice, the human project authority selects an option or rules
directly. The ruling is appended to
`execution/_Decomposition/SOFTWARE_DECOMP.md` §12 as the next `DEC` entry
citing this packet; this register row then moves from `AWAITING_RULING` to
`RULED` with the decision pointer. No edit to `DEC-041`, `SOFTWARE_DECOMP.md`
history, or any app-dev file occurs under this packet.
