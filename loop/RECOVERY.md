---
name: swbpipe-recovery-note
description: Where SWBPIPE's work in flight stands and how to resume it. One replaceable note.
metadata:
  kind: recovery
  surface: operating-entry
---
# SWBPIPE recovery note

Written 2026-11-10 at the close of an Agent 0 session whose managers have also
been retired. Start new sessions for this work; none of the earlier ones
carries on. Replace this note when its content stops being true, and delete it
when nothing here is still needed. Current capability is in
[`docs/RELEASE_TARGET_v1.0.md` §2](../docs/RELEASE_TARGET_v1.0.md#2-capability-rows).

## Dated work: CAEPIPE

The CAEPIPE re-verification is on **November 21 to 24, 2026**. The CAEPIPE trial
ends on **November 25, 2026**, and capture is not possible after that. That date
was counted by Agent 0 and not read from the application, so check it when the
guest next starts.

The re-verification needs the pressure-elongation switch in the shipped build,
so [#217](https://github.com/sgttomas/SWBPIPE/pull/217) comes first. Then:

1. Run the offline re-verification against the shipped commit and runner.
2. Before November 25, finish any further CAEPIPE capture: the integrated model
   after #217 and the `lib-seismic` case set, which failed its gate on an extra
   SL+SO column. Agent 0 decides whether to accept it, change the parser or
   re-run it. Recount the attempt reserve first.

The campaign is private and stays out of Git. Its recovery record, on the
development host, is
`/Users/ryan/dev/chirality-t3/caepipe-oracle/experiments/t1-campaign-20261109/RECOVERY-20261110.md`. The
pinned checkout and runner it names must be kept until the re-verification is
done. Benchmark evaluation stays private
([`docs/RELEASE_TARGET_v1.0.md` §1 and §3](../docs/RELEASE_TARGET_v1.0.md)).

## Path to the target

Lanes, in the order that matters. Each draft's description holds its recovery
note: start there. The Agent 0 that resumes a draft takes the merge duty the
Standing Git grant gives to the session that opened it. `harness` currently
fails on #217 and #214; #217's item 1 lists its known failures, and #214's have
not been examined.

| PR | Row | Where it stands | Resume at |
|---|---|---|---|
| [#217](https://github.com/sgttomas/SWBPIPE/pull/217) (draft) | RT-L2 | Pressure elongation switch, not integrated. Top priority. | Its recovery note. Decision 3's statement selection (item 6) comes first, then the fixtures (1), anchor (2), D3 re-pricing (3), independent numerical scrutiny (4) and AC-004 (7). The MBF writer (5) is a separate PR and is free to start now that #197 has merged |
| [#214](https://github.com/sgttomas/SWBPIPE/pull/214) (draft) | RT-K1 | Curved-bend transverse shear, paused until #217 merges | Its recovery note, starting with the merge of `main` after #217 |
| [#213](https://github.com/sgttomas/SWBPIPE/pull/213) (draft) | RT-A7 | Modal integrity step ledger, documentation only. R8 and the FMA decision are recorded; the replacements wait for #217 | Independent review round 2 at its head, then mark ready |

§2 of the release target is the complete list of open Must rows. The larger
ones outside these drafts include hydrotest (RT-A6), response spectrum (RT-A8),
reference-system interchange (RT-R7, RT-R8), usability and performance (RT-U1,
RT-U4) and the embedded agent's qualification (RT-E1).

## Needed from the owner

Bring these before the work they affect, each with its recorded recommendation.
- **Selections that are the owner's.** Whether to start the hanger lane (`execution/PKG-04/DEL-04-11/Work/hanger-sizing.md`), H1 (`execution/PKG-05/DEL-05-01/Work/hydrostatic-test.md`) and the dormant build guest (`execution/PKG-10/DEL-10-04/Work/windows-port.md`). None starts until the owner selects it.
- **Leave to launch the app** for #217's native recordings, and an AC-004 window for its final head.
- **The supplier-only HC-1 capture** for the embedded agent's qualification (RT-E1; `execution/PKG-16/DEL-16-06/Work/agent-d4-capture-proposal.md`).
- **Choices the governing text records no answer for.** Telemetry privacy (`execution/PKG-12/DEL-12-03/Work/telemetry-privacy-decision.md`), the Sensitive certificate notice (`execution/PKG-05/DEL-05-02/Work/pf-nonlinear-operand-standing.md` §5), operation-ID pairs (`execution/PKG-07/DEL-07-03/Work/combination-editor.md` §5) and affected-reference navigation (`execution/PKG-07/DEL-07-04/Work/affected-reference-navigation.md`). These were raised with the owner on 2026-11-08. Check that no answer has landed before asking again.
- **Before v1.0.** The signed D-20/DEC-058 repository scan, including the recovery branches published on 2026-11-08 (RT-V8); accepting any limitation; acceptance, issuance and release; the copyright holder's name for the licence (RT-V6); the check of the CAEPIPE licence for benchmark clauses before agreement is published (§1, §3); and the batch disposition of the 23 holds, which does not gate v1.0 (§1).

## Merging and the nightly arrangement

`main` requires the `harness` check and no longer requires an up-to-date branch.
Auto-merge is allowed. GitHub's merge queue is not available on a repository
owned by a personal account; moving to an organization is parked by the owner.
To run CI once for several ready, non-overlapping PRs, merge their reviewed heads
into one integration branch and merge that, as #220 did.

A full run of `main` follows every merge, and runs at 09:00 UTC if `main` has not
been verified. A failed run opens an issue labelled `nightly` naming the merge.
[`docs/CI_STRATEGY.md`](../docs/CI_STRATEGY.md) describes it, and the
[Standing Git grant](../AGENTS.md#standing-git-grant) has the arming, overlap and
triage rules.

## Small follow-ups

- The stale text "Unintegrated S6 and constant-effort work" in DEL-04-07
  `Work/pressure-contract.md`, and "R-4 S2–S8/P remain unbuilt" in
  `Work/route-unification-r4.md`.
- "S4" means a hydrotest stage in RT-A3 and condition-set resolution in RT-G5 and
  RT-L1.
- DEL-08-01 `Work/report-section-composer.md` says the composer serves exit
  criterion 4, which is the shareable report (RT-R5).
- `docs/COMPLETENESS_MAP.md` is an older second status account. ScopeOfWork files
  cite its §4.2 for the 2026-10-23 rulings, so cut it down with care.
- The preview-required message shows a raw operation id, `reconnect_pipe_run`
  (RT-U1).
- `AGENTS.md` is the place to say what a PR description must read like at merge.
  A reconciliation round found drafts merged with "not ready" descriptions, a PR
  naming no deliverable, and review evidence cited where none existed.

## Host conditions and files kept outside Git

Heavy jobs on the development host run through
`/Users/ryan/dev/chirality-t3/tools/t3_slot.sh`, and `guard/memguard.sh` there
watches memory. The drafts need no credentials.

Under `/Users/ryan/dev/chirality-t3/scratch/`:
`rt-a3-s6-final/ac004` (cited by hash in DEL-04-08 `Work/retained-pricing-rules.md`),
`s6-standalone-20261108/native-resume/ac004` (read-only, cited by nothing),
`swbpipe-ce193-ac004`, and the evidence that #214's and #213's recovery notes
name. The local branch `claude/rt-e1-catalog-expansion` is unpushed work in
progress for what merged as #27; nothing needs it.
