---
doc_id: PIP-D62-CANDIDATE
doc_kind: governance.decision_candidate
status: candidate_not_ruled
date: 2026-07-28
expected_decision_id: D-62
frozen_basis_commit: 85ea0628fa4e57dd6aae53b06139b2b8734a9612
---

# D-62 candidate — consolidated current-state ratification of D-59, D-60, and D-61

## Candidate identity and numbering

At the frozen basis `main@85ea0628fa4e57dd6aae53b06139b2b8734a9612`, `D-61` is
the highest numbered row in
`execution/_Coordination/_DECISIONS/_REGISTER.md`, so the expected identifier
is `D-62`. The execution-time scan governs. If `D-62` is no longer free, this
candidate must be re-minted under the actual next-free identifier and returned
for exact owner acceptance; it must not be silently renumbered.

This file is a candidate only. It is not a ruling and has no authority until
the owner returns per-decision text and the owning Piping governance workflow
durably records that text in the companion ruling record
`D-62_od8_ratification_acceptance.md`.

## Purpose

`D-59`, `D-60`, and `D-61` were recorded on 2026-07-28 and are already
effective on shared `main`. Each of the three records states that the Piping
manager's recommendation stands as the owner's approval under a single
standing direction quoted in-session on 2026-07-28. None of the three carries
a per-decision owner return addressed to that decision's own content.

This candidate asks the owner to supply that missing per-decision judgment
now, over frozen and enumerated evidence, as three independently declinable
items. It exists because the acts are already effective and cannot be undone
by this instrument, not because they are in doubt.

## No retroactive cure

The three 2026-07-28 acts were authorized solely by the standing blanket
direction. That is a disclosed procedural exception, and this candidate
records it as such. Ratification here does not cure, repair, or retroactively
regularize the conditions under which those acts landed. The historical
records, their register rows, their merges, and their effective state remain
exactly as they are. What this candidate adds is fresh, present-tense,
per-decision owner judgment over the already-effective present state.

If the owner declines an item, the underlying act does not thereby become
void; it remains effective and merely remains without fresh per-decision
ratification, and any correction requires its own successor decision.

## Informed batch, not pre-authorization

This is an informed batch over frozen, enumerated, exact-SHA evidence, with a
per-line decline available for every item. It is never a pre-authorization of
unknown future content. The three items below are fully enumerated at the
frozen basis; nothing outside them is presented for approval, and no class,
category, or forward window of acts is presented for approval.

This candidate is not citable as precedent for blanket approval. A future
request that asks the owner to approve unenumerated, unfrozen, or not-yet-
authored content may not cite `D-62` as support.

## Evidence basis

Merge-and-approval provenance for the landing pull requests is recorded in the
derivative evidence package `MERGE_APPROVAL_MATRIX_2026-07-28_85EA0628`, whose
`ARTIFACT_HASHES.sha256` manifest has SHA-256
`53844bfdcedaf5bae4396241375deba5dd35cc5b6d483342efac4a28268fccc1`. That
package is cited content-addressed: it is derivative, non-authoritative
evidence prepared under the loop-readiness transition program, it rules
nothing, and it lands on its own path outside this candidate's write scope. If
its manifest hash does not reproduce, the provenance statements below must be
re-verified from git before any ruling is returned.

Across all three items the same procedural facts hold and are not disputed:

- the authorizing instrument was the 2026-07-28 standing direction alone;
- the pull-request author and the merging actor are the same account on both
  landing pull requests; and
- the landing pull requests carried zero GitHub reviews.

Non-git SHA-256 digests below identify artifact manifests and method
documents; only the 40-hex values are Git commits.

## Item 1 — ratify D-59 (SCA-008 / DAG-008 revalidation acceptance)

**Record:** `D-59_sca008_dag008_revalidation_acceptance.md`

`D-59` accepts the SCA-008 / decomposition-revision-0.11 dependency-closure
revalidation derivative and disposes DAG-008 as `CURRENT_BY_REVALIDATION`
against revision 0.11. The accepted evidence establishes that SCA-008 changed
no dependency register or DAG path, that the accepted deliverable set remains
identical to the DAG-008 node set, that the canonical row, active-edge, and
topological-wave structure is unchanged, that the graph has no orphan
endpoints, strongly connected components, duplicate active edges, or
bidirectional pairs, and that local active topology is preserved when the
approved aggregate-only duplicate retirements are retained. DAG-008 remains
immutable and `execution/_DAG/_LATEST.md` remains on DAG-008. `D-59` closes
only the SCA-008 dependency and DAG downstream rerun and expressly routes the
DEC-063 / DEC-091 / DEL-16-04 reconciliation to `D-60`.

**Evidence:**

- Landing pull request #403; branch head commit
  `eba66bfe6ef00aa07d9cfac75eee79965218fa43`; merge commit
  `21e8e54e1f5648b7d3db29228271aaa8c7d8904f` (the branch head is the second
  parent of the merge).
- Examined basis stated in the record:
  `4cd25b348196f7e6dfa925d8c7938184924cb383`.
- Derivative production basis: `404e47c16a88e7ffdc6d1fc5fac61ebb6864211e`.
- Upstream SCA-008 application commit
  `9b52076701c218f69255afbedcfc52025bd47fa3` and effective merge commit
  `380ea2a794588075b83fe8cc0108ab7ce74b6b33` (pull request #390).
- Derivative
  `execution/_Evaluation/DepClosure/CLOSURE_SCA008_DAG008_REV011_REVALIDATION_2026-07-28_0901/`
  at manifest SHA-256
  `3be1e5a5d5f629b39be656799a7e100cd7fe7a615e6188cd78198ae81e3876f0`.
- The merge-approval matrix records this landing as satisfying its
  approved-SHA identity check: the run record names the application commit,
  which reproduces as the merge's second parent.

**What the blanket direction supplied:** the entire authorization. The Piping
Agent 1 recommended acceptance; under the standing direction that
recommendation was recorded as the owner's approval. No owner text addressed
to the DAG-008 revalidation content exists.

**What ratification would add:** fresh present-tense owner judgment that the
`CURRENT_BY_REVALIDATION` disposition and its stated boundaries are the
owner's own, on the frozen evidence above. The owner's return for this item is
transcribed in the companion ruling record inside the fence
`<!-- BEGIN OWNER RULING VERBATIM D-62-ITEM-D-59 -->` /
`<!-- END OWNER RULING VERBATIM D-62-ITEM-D-59 -->`.

## Item 2 — ratify D-60 (scoped reconciliation derivative activation)

**Record:** `D-60_sca008_current_effect_reconciliation_activation.md`

`D-60` activates and unfreezes only the SCA-008 downstream `RECONCILIATION`
rerun covering current-effect authority relations among `DEC-063`, `DEC-091`,
and `DEL-16-04`, plus exactly the two affected historical claim identities
`DEL-16-04-REQ-009` and `DEL-16-04-DECL-005`. It authorizes one new immutable
sibling derivative at
`execution/_Reconciliation/DeliverableConcordance/SCOPED_SCA008_DEC091_DEL1604_CURRENT_EFFECT_2026-07-28/`
under a fixed core-artifact contract, requires DEC-063 to remain immutable
historical authority rather than a current reliance basis, requires DEC-091 to
govern current effect, holds App F3 necessary and the automation-condition
mechanism unresolved, keeps `DEL-16-04` in its existing lifecycle state with
its exact current Remaining items and releases no selectable work, keeps
Piping outside the Root-runtime and App-harness client sets, leaves PRD R7 and
Piping product outcomes unchanged, and permits closure only as
`CURRENT_EFFECT_RECONCILED / CLOSED_WITH_RELIANCE_HOLD`. The complete
historical 2026-07-11 concordance package must remain byte-identical, no
reconciliation pointer may be created or edited, and no discovery or
derivative write could begin before durable merge.

**Evidence:**

- Landing pull request #403 (the same landing as item 1); branch head commit
  `eba66bfe6ef00aa07d9cfac75eee79965218fa43`; merge commit
  `21e8e54e1f5648b7d3db29228271aaa8c7d8904f`.
- Activation basis stated in the record:
  `4cd25b348196f7e6dfa925d8c7938184924cb383`.
- Method `docs/DELIVERABLE_CONCORDANCE_METHOD.md` revision 1 at SHA-256
  `abf3e78fce606c4557d61cdbfbdb7292a3d858838f6526da6b433d1bcd0ef627`.
- Run identity `SCOPED_SCA008_DEC091_DEL1604_CURRENT_EFFECT_2026-07-28`; the
  derivative's `RUN_BASIS.md` names the activation application commit
  `eba66bfe6ef00aa07d9cfac75eee79965218fa43` and the durable merge
  `21e8e54e1f5648b7d3db29228271aaa8c7d8904f`.

**What the blanket direction supplied:** the entire authorization. The Piping
Agent 1 recommended the bounded activation; under the standing direction that
recommendation was recorded as the owner's approval. No owner text addressed
to the activation scope, the write fence, or the permitted closure verdict
exists.

**What ratification would add:** fresh present-tense owner judgment that the
activation scope, the preservation and write fence, and the single permitted
closure verdict are the owner's own. The owner's return for this item is
transcribed in the companion ruling record inside the fence
`<!-- BEGIN OWNER RULING VERBATIM D-62-ITEM-D-60 -->` /
`<!-- END OWNER RULING VERBATIM D-62-ITEM-D-60 -->`.

## Item 3 — ratify D-61 (reconciliation acceptance and reliance hold)

**Record:** `D-61_sca008_current_effect_reconciliation_acceptance.md`, with the
additive `D-61_EFFECTIVE_STATE_CLOSEOUT_2026-07-28.md`

`D-61` accepts the `D-60` derivative with `CurrentEffectState =
CURRENT_EFFECT_RECONCILED` and `ClosureVerdict = CLOSED_WITH_RELIANCE_HOLD`.
Under that acceptance `DEC-063` is preserved as immutable historical authority
and is not a current reliance basis, `DEC-091` governs current effect, App F3
remains necessary, the automation-condition mechanism remains unresolved,
`DEL-16-04` remains `IN_PROGRESS` with its exact three Remaining items, and no
selectable work is released. The two affected claims retain the bounded
dispositions `DEL-16-04-REQ-009 = PARTIALLY_IMPLEMENTED / MEDIUM / OWNER / NO`
and `DEL-16-04-DECL-005 = ALIGNED / HIGH / NO / NO`. The historical
2026-07-11 concordance package remains byte-identical, no reconciliation
pointer is created or edited, Piping remains outside the Root-runtime and
App-harness client sets, and PRD R7 and the Piping product outcomes remain
unchanged.

**Evidence:**

- Landing pull request #404; branch head commit
  `ee683ef9d99abe41ad91f275605884c74b27da38`; merge commit
  `ecfddf92a945d6f5b83b9bddc28bc2b42f65fdbc` (the branch head is the second
  parent of the merge).
- Execution basis stated in the record:
  `21e8e54e1f5648b7d3db29228271aaa8c7d8904f`.
- Derivative artifact manifest SHA-256
  `d6f5598f087c4d49c3f1ad5406d8c80d02d79be13f59cd6b127323a949a86ea3`.
- Validation result SHA-256
  `0df4e84c3846abc79e6a3c16669e6e282222eb51107c74deffbe6ae78a759ab1`.
- The merge-approval matrix records this landing as **not** satisfying its
  approved-SHA identity check: the branch head commit appears nowhere in the
  checkout, and the effective-state closeout names only the pull-request
  number and the merge commit. The merge-level identity reproduces in git;
  the record-level approved-HEAD identity does not. This asymmetry against
  item 1 is disclosed here rather than smoothed over, and it is a reason the
  owner may weigh in declining or qualifying this item.

**What the blanket direction supplied:** the entire authorization. The Piping
`RECONCILIATION` manager recommended acceptance after the derivative validator
passed; under the standing direction that recommendation was recorded as the
owner's approval. No owner text addressed to the closure verdict, the reliance
hold, or the two claim dispositions exists.

**What ratification would add:** fresh present-tense owner judgment that
`CURRENT_EFFECT_RECONCILED / CLOSED_WITH_RELIANCE_HOLD`, the DEC-063 /
DEC-091 / DEL-16-04 dispositions, and the two claim dispositions are the
owner's own. The owner's return for this item is transcribed in the companion
ruling record inside the fence
`<!-- BEGIN OWNER RULING VERBATIM D-62-ITEM-D-61 -->` /
`<!-- END OWNER RULING VERBATIM D-62-ITEM-D-61 -->`.

## Decline semantics

Each of the three items is independently declinable. Declining one item does
not weaken, qualify, or condition the other two, and approving one item does
not carry any inference toward the others.

A decline on any item means only this: the underlying act has not received
fresh per-decision owner ratification. It does not void the act, does not
reopen the merged record, does not alter the effective state on shared `main`,
and does not authorize any edit to the ruled record, its register row, its
derivative, or its merge. A declined item that the owner wants changed
requires its own successor decision, prepared and ruled separately.

Silence is not approval. An item with no owner return in its fence remains
unratified, and the ruling record must not be completed as if it were ruled.

## Ruling mechanics

Owner returns arrive later, in a separate owner session. Each return is
transcribed verbatim into its own fence in
`D-62_od8_ratification_acceptance.md`. Until then each fence carries only its
placeholder token, and every other byte of that record is final as authored.

Per the Piping convention, the completion commit adds, immediately after each
fence, the canonical byte count of the exact UTF-8 bytes between that fence's
markers, excluding marker lines and adjacent delimiter newlines and with no
trailing newline, together with the SHA-256 of exactly those bytes. Those
statements are pending and cannot be authored before the returns exist.

## Explicit exclusions

This candidate does not:

- alter, reopen, annotate, or rewrite `D-59`, `D-60`, `D-61`, their register
  rows, their derivatives, or their merges;
- change the reliance hold, which remains active by design and untouched;
- change `DEL-16-04` lifecycle state, its Remaining items, or release any
  selectable work;
- decide a successor mechanism for the unresolved automation condition;
- change Piping's client status with respect to Root runtime or the App
  harness;
- cure, repair, or retroactively regularize the historical nonconformance it
  discloses;
- create precedent for blanket or unenumerated approval;
- change any PRD, decomposition, scope, package, deliverable, objective,
  requirement, dependency, DAG, source, schema, runtime, profile, lifecycle,
  stage, release, estimate, schedule, professional-reliance, certification,
  sealing, authentication, or code-compliance state; or
- authorize any Git application, push, or merge.

## Authorized consequence if adopted

Adoption of any item authorizes only durable recording of that owner return
through the normal Piping governance and `CHANGE` path, as the three-path
landing described in the companion ruling record, plus the single
`LOOP_RECEIPTS.md` append drafted below. Nothing else follows from adoption.

## Receipt draft

The following `Receipt-82` is drafted here and is **not** appended by the
tranche that lands this candidate. `projects/chirality-piping/loop/LOOP_RECEIPTS.md`
ends at `Receipt-81` at the frozen basis, so `82` is the next free identifier;
the execution-time scan governs. The append happens only in the completion
commit that follows the owner session, and the completion commit may finalize
the `Gate-Outcome` and `Pointers` text to reflect the owner's actual returns.
Any such finalization is governed by the returned text, never by this draft.

```text
- **2026-07-28 — Receipt 82** (D-62 consolidated current-state ratification candidate).
  - Receipt-ID: `Receipt-82`
  - Examined-Through: `85ea0628fa4e57dd6aae53b06139b2b8734a9612`
  - Parent-Receipt: `Receipt-81`
  - Owner-Direction: CHAT_TRANSCRIPTION — EVIDENCE, NOT RULING. The
    2026-07-28 standing direction quoted in the D-59, D-60, and D-61 records
    is the sole authorization those acts carried; it is reproduced there and
    is not restated as authority here.
  - Pointers: `D-62_od8_ratification_proposal.md`;
    `D-62_od8_ratification_acceptance.md`; the new D-62 decision-register
    row; merge-provenance evidence
    `MERGE_APPROVAL_MATRIX_2026-07-28_85EA0628` at manifest SHA-256
    `53844bfdcedaf5bae4396241375deba5dd35cc5b6d483342efac4a28268fccc1`.
  - Checks: candidate identity is next-free at the frozen basis; the cited
    commits for pull requests #403 and #404 resolve and reproduce as merge
    second parents; the piping receipt-contract validator reports the ledger
    conformant; the candidate touches no ruled record and no reliance hold.
  - Model-Attribution: Anthropic Claude bounded Agent 2 author under the
    Piping owning-manager lane of the loop-readiness transition program;
    exact runtime model string was not exposed to the record.
  - Gate-Outcome: `AWAITING_OWNER` — the candidate is immutable and complete
    except for three empty owner-return fences. Each item is independently
    declinable; silence is not approval. No cure of historical
    nonconformance, no reliance-hold change, no DEL-16-04 lifecycle change,
    no successor mechanism, no client-status change, and no product, runtime,
    dependency, DAG, implementation, repin, release, estimate, schedule,
    selectable-work, or professional-reliance effect is created.
```

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
