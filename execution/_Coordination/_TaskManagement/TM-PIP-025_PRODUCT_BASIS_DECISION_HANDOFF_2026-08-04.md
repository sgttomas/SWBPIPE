# TM-PIP-025 product-basis decision handoff — 2026-08-04

Status: `DERIVATIVE DECISION SUPPORT — ROUTED HANDOFF — NO RULING`

Run: `HELP-HUMAN-PIPING-TM025-20260804-R25`

Invoking loop: Chirality Piping

This file is a rebuildable decision-support package prepared by
`TASK_MANAGEMENT`. It is not a product-basis act, decision packet, PRD,
decomposition amendment, scope change, lifecycle act, work selection, or
Task Management disposition. `TM-PIP-025` remains `DEFERRED`. The accountable
human alone may select an option.

## 1. Commission and hard fence

The owner commissioned preparation of a Piping product-basis decision surface
over both of these intent records, without deciding their disposition:

1. `projects/chirality-piping/execution/_Coordination/OWNER_INTENT_2026-07-31_DESIGN_TOOL_BOUNDARY.md`,
   Git blob `2c3d9aca71844052734c232a3f342f6d71391ad6`;
2. `execution/_Coordination/OWNER_INTENT_2026-08-02_PRODUCT_DELIVERY_DIRECTION.md`,
   Git blob `4b1bdafdb4a8e39e6942e74ebaf4028162436be3`.

The surface must offer exactly `ADOPT`, `AMEND`, `DEFER`, and `DECLINE`.
Those records remain coordination, not authority, until the owner rules a
Piping-owned product-basis instrument. This handoff does not reserve a
decision identifier or write the Piping decision register.

## 2. Mandatory federation preflight

Before resolution planning, the deterministic `taskmgmt federation` helper
surveyed the Piping invocation against committed
`cdc76a1d398231267f1379e7143b4de27abaa01b`:

```text
taskmgmt federation COMPLETE: 4 register(s), 46 finding(s), 45 presented
coverage: COMPLETE; register_writes: 0
PEC: OPEN=17 DEFERRED=3 ELEVATED=0 CLOSED=1; archived=4
ROOT: OPEN=12 DEFERRED=11 ELEVATED=0 CLOSED=0; archived=99
APP: OPEN=11 DEFERRED=3 ELEVATED=0 CLOSED=0; archived=26
PIP: OPEN=7 DEFERRED=26 ELEVATED=0 CLOSED=0; archived=4
```

All four canonical live registers and archives were tracked, readable, and
schema-valid. There were no invalid or unreadable inputs, unresolved
ambiguities, operational errors, or discovered lookalikes. The helper
declared its standard excluded classes. Before/after hashes proved zero
register writes. `COMPLETE` describes survey coverage only; it does not
accept the findings or authorize any effect.

## 3. Exact evidence basis

| Evidence | Git blob | SHA-256 | Use |
|---|---|---|---|
| 2026-07-31 Piping design-tool-boundary intent | `2c3d9aca71844052734c232a3f342f6d71391ad6` | `c35f119adc4199751d5db3af62c01c1b2ba84be42b31424b3d1a2e411dc57eee` | Limited-purpose design tool; engineering judgment and validation stay external; agent use is query/convey oriented at this boundary |
| 2026-08-02 product-delivery intent | `4b1bdafdb4a8e39e6942e74ebaf4028162436be3` | `9bbb67556765c6c83d6a35a1ace297e4d693d5169281c620dc9b2673229c7e03` | OpenPipeStress as first domain-specific application target; robust UI; semantically equivalent agent API; Piping-tailored operative runtime/harness |
| Piping runtime-surface response | `a71145ec0952cc5ad62b1b12635be44deebffbd3` | `e38c5614351ce45d77535c4bb234580bbbb1916a68a482660b6c3f4e230235e7` | Current inventory, Piping-local meanings, candidate generic primitives, gaps, and no-effect fence; exact response basis bound by the App rider |
| App D-APP-91 ruling | `e6d7e979204b978e3fde4f84b57e9bbbe49a55fe` | `ea3eb83128db26d25677e0823d294b477904e46d9783c62474454fd1436a6237` | Option C planning baseline and exact material-amendment rider |
| App D-APP-91 proposal packet | `82b6aed9cacb0ce2ef1bf1095f7caa7fd1aaac6e` | `ff57b0b3947c15dcc49f7ac978dac736a2f03a0fc71f3bd270f378f2e9a51a9e` | Selected build-time product-profile architecture and six bounded slots |
| Piping live register at launch | `39748c43eb0dbd82424dd8eb9d6b98339a773758` | `a57199e9d120edbfd60ff9f131a9f7a18ab48ada57e9d6bf3a0dfc78311e86a9` | Exact `TM-PIP-025` row and trigger |

The operative App rider is exact: if the eventual `TM-PIP-025` product-basis
act materially amends response Git blob `a71145ec...`, the App first-domain
lane must reverify affected evidence before its exact-requirements packet
issues; absent material amendment, no reverification is owed. D-APP-91 Option
C remains a planning baseline only: one shared Woven shell, immutable product
manifest, generated build-time profile, and six bounded slots
(`NavigationWorkspaceSlot`, `StructuredInformationSlot`, `WorkflowSlot`,
`DecisionGateSlot`, `TypedAgentReviewSlot`, `UiAgentConformanceSlot`).

## 4. Nine-domain completeness scan

| Domain | TM-PIP-025 resolution requirement |
|---|---|
| Action Item | Decide whether the two exact intent records are adopted, amended, deferred, or declined as a Piping product basis. Until that owner act, neither record has PRD, decomposition, scope, planning, implementation, or lifecycle effect. |
| Assignment | `TASK_MANAGEMENT` prepares and routes this handoff; `HELP_HUMAN` owns cross-manager fan-in and may dispatch one bounded packet author; the Piping decision instrument owns the proposal/ruling record; `SCOPE_CHANGE` owns any later PRD/decomposition/scope propagation. Ryan Tufts is the sole accountable decision maker; no agent is `A`. |
| Prioritization | The row has `Priority=TBD` and `Status=DEFERRED`. This session's commissioning authorizes packet preparation, not a priority change or disposition. |
| Deliverables | A new proposal-only Piping decision packet under `projects/chirality-piping/execution/_Coordination/_DECISIONS/`, registered in `_DECISIONS/_REGISTER.md`, followed only after owner selection by a separate ruling record. No product deliverable is amended by preparation. |
| Work | Bind both source blobs; present the four options below; preserve the design-tool/judgment boundary and the runtime response's current-vs-candidate distinctions; carry the D-APP-91 rider per option; provide exact owner-return forms and on-ruling mechanics. |
| Planning | Packet author verifies the next unused Piping decision ID, writes proposal/register entry, and returns it for owner ruling. After ruling, record the decision; apply the App-rider route below; invoke `SCOPE_CHANGE` only if the ruled product basis is to alter PRD, decomposition, or scope; then return separately to `TASK_MANAGEMENT` for an owner-ruled row disposition. |
| Approval | This is owner-class: it touches product purpose, external-judgment boundary, delivery posture, and potentially scope. `OwnerCaseSelection=NONE`; no standing delegation or agent recommendation can select it. |
| Checking | Verify both intent Git blobs, the response blob, and the D-APP-91 rider; verify all four options are present; fail closed on unspecified amendments; verify the selected ruling is recorded byte-for-byte; run decision-register and ordinary repository checks before publication. |
| Decisions | The new Piping decision record carries the product-basis ruling. A later SCA decision carries any scope propagation. The Task Management row only records the eventual human disposition and exact evidence; it is never the product-basis authority. |

## 5. Required owner-decision surface

The decision statement should be:

> How shall Chirality Piping treat, as one product-basis pair, the exact
> 2026-07-31 design-tool-boundary intent and the exact 2026-08-02
> product-delivery intent: ADOPT, AMEND, DEFER, or DECLINE?

`Response-basis effect` below concerns App D-APP-91's use of response blob
`a71145ec...`. It does not rewrite or erase the historical response bytes.

| Option | Exact effect if selected | Response-basis effect | App-side reverification before first-domain exact-requirements issuance |
|---|---|---|---|
| `ADOPT` | Adopt both exact intent records as the Piping product basis: OpenPipeStress is the first domain-specific application delivery target with robust UI, semantically equivalent agent-facing API, and a Piping-tailored operative runtime/harness, all bounded as a limited-purpose design tool with engineering judgment and validation external. Current response statements remain inventory/candidate statements unless separately propagated. | `UNCHANGED`. Exact adoption adds authority to the intent pair but neither contradicts nor replaces the response's inventory, Piping-local meanings, gaps, or candidate/implemented distinctions. | `NO`. The D-APP-91 rider says no reverification is owed absent material amendment. |
| `AMEND` | Adopt only the owner's exact amended product-basis text. No amendment is supplied or inferred by this handoff. | `UNDETERMINED_FAIL_CLOSED` until the owner supplies exact text, effect classification, and rationale using section 6. It may be `MATERIAL_AMENDMENT`, `INVALIDATES`, or `UNCHANGED`. | `BLOCKED_PENDING_CLASSIFICATION`. `YES` for `MATERIAL` (including `MATERIAL_AMENDMENT` or `INVALIDATES`); `NO` only for a consistent `NON_MATERIAL / UNCHANGED` return. |
| `DEFER` | Make no Piping product-basis adoption now. The ruling must name a checkable future trigger or review date. Both intent records and the response retain their current coordination/inventory-only standing. | `UNCHANGED`. Deferral changes neither the response nor App's current response-bound planning evidence. | `NO`. The owner has not materially amended the response basis. This does not itself authorize App exact-requirements issuance or Piping work. |
| `DECLINE` | Decline both intent records as the Piping product basis. Existing accepted Piping scope is not silently reversed; any desired scope removal requires its own governed change. | `INVALIDATES_FOR_FIRST_DOMAIN_USE`. The response remains valid historical inventory, but its use as evidence for the D-APP-91 first-domain product-profile target is no longer supported by a Piping product-basis direction. | `YES`, and issuance remains held until App re-verifies the affected evidence and records whether a replacement first-domain basis exists. Reverification is not permission to invent one. |

## 6. Fail-closed AMEND return contract

An `AMEND` return is incomplete and has no selection effect unless it supplies
all fields below:

```text
OPTION: AMEND
EXACT_AMENDMENT_TEXT:
<complete replacement/addition/deletion text, identifying which intent clause(s) it changes>

APP_EFFECT: MATERIAL | NON_MATERIAL
RESPONSE_BASIS_EFFECT: MATERIAL_AMENDMENT | INVALIDATES | UNCHANGED
RATIONALE:
<why the exact amendment has that effect on Git blob a71145ec0952cc5ad62b1b12635be44deebffbd3>
OWNER: Ryan Tufts
DATE: YYYY-MM-DD
```

Consistency rule: `MATERIAL_AMENDMENT` and `INVALIDATES` require
`APP_EFFECT: MATERIAL` and App-side reverification. `UNCHANGED` requires
`APP_EFFECT: NON_MATERIAL`; the rationale must explain why no response
evidence claim changes. Missing, contradictory, or merely paraphrased fields
return to the owner without a ruling record, scope act, register disposition,
or App issuance inference.

## 7. Non-binding recommendation

`PROPOSAL — ADOPT` both records exactly, with the product-basis wording in
section 5 and no additional semantics. It is the most coherent option because
it preserves the owner's limited-purpose/external-judgment boundary while
giving the recorded domain-application, robust-UI, semantic-API, and tailored
runtime direction an explicit Piping home. It also preserves the exact
runtime-surface response basis used by D-APP-91, so no App-side reverification
is owed under the rider.

This recommendation is not a selection. `AMEND`, `DEFER`, and `DECLINE`
remain lawful owner choices with the effects stated above.

## 8. On-ruling mechanics

1. The decision-instrument manager records the exact owner return in a
   separate ruling record and updates only the Piping decision register.
2. For `ADOPT` or an `AMEND` intended to alter accepted PRD, decomposition,
   or scope, route an SCA intake to `SCOPE_CHANGE`; the product-basis ruling
   itself does not silently rewrite those surfaces.
3. For `MATERIAL_AMENDMENT` or `INVALIDATES`, route a SHA-bound Piping-to-App
   notice and hold App first-domain exact-requirements issuance until App
   records reverification. For `UNCHANGED`, route a coordination notice
   stating why the rider does not require reverification; the notice creates
   no App authority.
4. For `DEFER`, record the exact trigger/date in the decision ruling. For
   `DECLINE`, preserve existing accepted Piping state until separately changed.
5. Return to Piping `TASK_MANAGEMENT` with the ruling `EvidenceRef`, exact
   byte identity, and quote. The non-binding closure proposal is
   `RESOLVED_BY_DECISION` because the row tracks whether the product-basis
   question received an owner act; no row change occurs without that owner
   disposition.
6. Ordinary CHANGE and receipt closeout follow only after the owning
   instruments complete their checks and the human authorizes Git effects.

## 9. Routed handoff state

- Accepted upstream snapshots: repository basis
  `cdc76a1d398231267f1379e7143b4de27abaa01b`; the six exact evidence objects
  in section 3.
- Derivative status: this file is decision support only and is not a
  substitute for either intent record, the runtime response, D-APP-91, or a
  future Piping decision.
- Closure verdict: `PREPARATION_COMPLETE / TM-PIP-025_UNCHANGED_DEFERRED`.
- Next gate: `HELP_HUMAN` routes this package to a bounded Piping decision
  packet author under the loop's decision-escalation instrument; the human
  rules the resulting packet.
- Rerun requirements: re-hash all cited objects and re-check the next unused
  decision ID immediately before packet authoring; reclassify App effect if
  any source bytes or the owner-supplied amendment differ.
- Remaining blockers: no owner option selected; no decision packet/ruling
  exists; no SCA is authorized; no Task Management disposition is authorized.
