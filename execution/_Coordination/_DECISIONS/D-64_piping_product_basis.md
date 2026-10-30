# D-64 — Piping Product-Basis Disposition

Status: `PROPOSAL — AWAITING HUMAN RULING`

DecisionID: `D-64`

Date prepared: `2026-08-04`

Owning loop: `Chirality Piping`

Accountable owner: `Ryan Tufts`

Related Action Item: `TM-PIP-025`

OwnerCaseSelection: `NONE`

This packet is a proposal-only Piping decision surface. It offers four lawful
owner options and records a non-binding recommendation. It makes no selection
and creates no product-basis, PRD, decomposition, scope, planning,
implementation, lifecycle, release, publication, reliance, Task Management,
App, Root, or Git effect before an exact owner return is recorded through the
Piping decision instrument.

## 1. Decision required

How shall Chirality Piping treat, as one product-basis pair, the exact
2026-07-31 design-tool-boundary intent and the exact 2026-08-02
product-delivery intent: `ADOPT`, `AMEND`, `DEFER`, or `DECLINE`?

The two intent records remain coordination, not authority, unless and until
the owner selects an option here. Their existence alone does not affect PRD,
decomposition, scope, planning, implementation, or lifecycle state.

## 2. Exact evidence basis

| Evidence | Git blob | SHA-256 | Decision use |
|---|---|---|---|
| `projects/chirality-piping/execution/_Coordination/OWNER_INTENT_2026-07-31_DESIGN_TOOL_BOUNDARY.md` | `2c3d9aca71844052734c232a3f342f6d71391ad6` | `c35f119adc4199751d5db3af62c01c1b2ba84be42b31424b3d1a2e411dc57eee` | Limited-purpose design tool; engineering judgment and validation external; query/convey-oriented agent boundary |
| `execution/_Coordination/OWNER_INTENT_2026-08-02_PRODUCT_DELIVERY_DIRECTION.md` | `4b1bdafdb4a8e39e6942e74ebaf4028162436be3` | `9bbb67556765c6c83d6a35a1ace297e4d693d5169281c620dc9b2673229c7e03` | OpenPipeStress as first domain-specific application target; robust UI; semantically equivalent agent API; Piping-tailored operative runtime/harness |
| `projects/chirality-piping/execution/_Coordination/COORDINATION_RESPONSE_2026-08-02_PIPING_RUNTIME_SURFACE_NEEDS.md` | `a71145ec0952cc5ad62b1b12635be44deebffbd3` | `e38c5614351ce45d77535c4bb234580bbbb1916a68a482660b6c3f4e230235e7` | Current inventory, Piping-local meanings, candidate generic primitives, recorded gaps, and no-effect fence; exact App-bound response basis |
| `projects/chirality-app-dev/execution/_Coordination/_DECISIONS/D-APP-91_RULING_BUILD_TIME_PRODUCT_PROFILES_SIX_BOUNDED_SLOTS_2026-08-03.md` | `e6d7e979204b978e3fde4f84b57e9bbbe49a55fe` | `ea3eb83128db26d25677e0823d294b477904e46d9783c62474454fd1436a6237` | Ruled Option C planning baseline and exact material-amendment rider |
| `projects/chirality-app-dev/execution/_Coordination/_DECISIONS/D-APP-91_PACKET_DUAL_TARGET_ARCHITECTURE_SELECTION_2026-08-03.md` | `82b6aed9cacb0ce2ef1bf1095f7caa7fd1aaac6e` | `ff57b0b3947c15dcc49f7ac978dac736a2f03a0fc71f3bd270f378f2e9a51a9e` | Selected build-time product-profile architecture and six bounded slots |

The D-APP-91 same-ruling rider is operative exactly as follows: if the
eventual `TM-PIP-025` product-basis act materially amends response Git blob
`a71145ec0952cc5ad62b1b12635be44deebffbd3`, the App first-domain lane must
reverify every affected evidence claim against the amended basis before its
exact-requirements packet issues. Absent material amendment, no
reverification is owed.

D-APP-91 Option C is a planning baseline only: one shared Woven shell, an
immutable product manifest, generated build-time profiles, and six bounded
slots: `NavigationWorkspaceSlot`, `StructuredInformationSlot`, `WorkflowSlot`,
`DecisionGateSlot`, `TypedAgentReviewSlot`, and `UiAgentConformanceSlot`.

## 3. Interpretation boundary

`Response-basis effect` below concerns App D-APP-91's use of response blob
`a71145ec...`. It does not rewrite or erase that historical response. The
response's implemented/current/candidate distinctions and its Piping-local
meaning boundaries remain exact unless the owner expressly amends them.

The decision is owner-class because it touches product purpose, the external
engineering-judgment boundary, delivery posture, and potentially scope. No
agent may select an option under standing delegation.

## 4. Owner options

| Option | Exact effect if selected | App response-basis classification | App-side reverification before first-domain exact-requirements issuance |
|---|---|---|---|
| `ADOPT` | Adopt both exact intent records as the Piping product basis: OpenPipeStress is the first domain-specific application delivery target with robust UI, semantically equivalent agent-facing API, and a Piping-tailored operative runtime/harness, all bounded as a limited-purpose design tool with engineering judgment and validation external. The response remains inventory/candidate evidence unless separately propagated. | `UNCHANGED`. Exact adoption neither contradicts nor replaces the response's inventory, Piping-local meanings, gaps, or candidate/implemented distinctions. | `NO`. The D-APP-91 rider requires none absent material amendment. |
| `AMEND` | Adopt only the owner's exact amended product-basis text. This packet supplies and infers no amendment. | `UNDETERMINED_FAIL_CLOSED` until the owner supplies the complete section 5 return. The result must be classified `MATERIAL_AMENDMENT`, `INVALIDATES`, or `UNCHANGED`. | `BLOCKED_PENDING_CLASSIFICATION`. `YES` for `MATERIAL`, including `MATERIAL_AMENDMENT` or `INVALIDATES`; `NO` only for a consistent `NON_MATERIAL / UNCHANGED` return. |
| `DEFER` | Make no Piping product-basis adoption now. The return must name a checkable trigger or review date. Both intents and the response retain coordination/inventory-only standing. | `UNCHANGED`. Deferral changes neither the response nor App's current response-bound planning evidence. | `NO`. No material amendment occurred. This does not itself authorize App issuance or Piping work. |
| `DECLINE` | Decline both intent records as the Piping product basis. Existing accepted Piping scope is not silently reversed; any desired removal requires a separately governed change. | `INVALIDATES_FOR_FIRST_DOMAIN_USE`. The response remains valid historical inventory, but D-APP-91 may no longer use it as evidence for a Piping-backed first-domain product-profile target. | `YES`, and issuance remains held until App re-verifies affected evidence and records whether a replacement first-domain basis exists. Reverification cannot invent one. |

## 5. Fail-closed AMEND return

An `AMEND` return is incomplete and has no selection effect unless the owner
supplies every field below:

```text
APPROVE D-64 OPTION AMEND
EXACT_AMENDMENT_TEXT:
<complete replacement/addition/deletion text, identifying each changed intent clause>

APP_EFFECT: MATERIAL | NON_MATERIAL
RESPONSE_BASIS_EFFECT: MATERIAL_AMENDMENT | INVALIDATES | UNCHANGED
RATIONALE:
<why the exact amendment has that effect on Git blob a71145ec0952cc5ad62b1b12635be44deebffbd3>
OWNER: Ryan Tufts
DATE: YYYY-MM-DD
```

`MATERIAL_AMENDMENT` and `INVALIDATES` require `APP_EFFECT: MATERIAL` and
App-side reverification. `UNCHANGED` requires `APP_EFFECT: NON_MATERIAL` and
a rationale explaining why no response evidence claim changes. Missing,
contradictory, or placeholder-bearing fields return to the owner without a
ruling record, scope act, register disposition, or App issuance inference.

## 6. Non-binding recommendation

`PROPOSAL — ADOPT` both exact intent records without added semantics.

This is recommended because it preserves the limited-purpose design-tool and
external-engineering-judgment boundary while giving the domain-application,
robust-UI, semantic-API, and tailored-runtime direction an explicit Piping
product-basis home. It leaves response blob `a71145ec...` unchanged, so the
D-APP-91 rider requires no App-side reverification.

This recommendation is not a selection. All four options remain lawful.

## 7. Exact owner return tokens

For `ADOPT`, return exactly:

```text
APPROVE D-64 OPTION ADOPT — ADOPT BOTH EXACT INTENT RECORDS AS THE PIPING PRODUCT BASIS WITHOUT AMENDMENT — Ryan Tufts YYYY-MM-DD
```

For `AMEND`, return the complete section 5 form; no shorter token is valid.

For `DEFER`, replace both angle-bracket fields and return exactly:

```text
APPROVE D-64 OPTION DEFER — TRIGGER: <checkable condition or review date> — Ryan Tufts <YYYY-MM-DD>
```

For `DECLINE`, return exactly:

```text
APPROVE D-64 OPTION DECLINE — DECLINE BOTH EXACT INTENT RECORDS AS THE PIPING PRODUCT BASIS — Ryan Tufts YYYY-MM-DD
```

Silence, paraphrase, an unfilled placeholder, or a response selecting more
than one option is not a ruling.

## 8. On-ruling mechanism

1. Record the exact owner return in a separate Piping `D-64` ruling record;
   update this decision's register row from `AWAITING_RULING` to `RULED` with
   a pointer. Do not rewrite this proposal packet.
2. For `ADOPT`, or an `AMEND` intended to alter accepted PRD, decomposition,
   or scope, prepare an SCA intake for `SCOPE_CHANGE`. The product-basis
   ruling itself does not silently rewrite those surfaces.
3. For `MATERIAL_AMENDMENT` or `INVALIDATES`, route a SHA-bound Piping-to-App
   notice and hold App first-domain exact-requirements issuance until App
   records reverification. For `UNCHANGED`, route a coordination notice
   stating why the rider requires no reverification; that notice creates no
   App authority.
4. For `DEFER`, preserve the exact trigger/date in the ruling. For `DECLINE`,
   preserve accepted Piping state until a separate governed change alters it.
5. Return the ruling's exact `EvidenceRef`, byte identity, and quote to Piping
   `TASK_MANAGEMENT`. Any proposal to disposition `TM-PIP-025` remains a
   separate owner act; the decision register never disposes that row.
6. Ordinary validation, CHANGE, receipt, publication, and merge gates remain
   separate and human-gated.

## 9. No-effect fence

Before an exact owner return, this packet authorizes nothing. After a return,
the selected product-basis effect is limited to the option's exact text and
the mechanics above. No option by itself writes PRD, decomposition, scope,
DAG, deliverables, source, tests, lifecycle, release, publication, reliance,
professional approval, Root/App state, or Task Management state. No contract
bytes, generic runtime semantics, cross-consumer compatibility, engineering
judgment, validation authority, or agent implementation are created here.
