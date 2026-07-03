# DESIGN — Agent proposal→apply bridge and the OperationSet→Plan schema extension (DEC-042 item 1)

> **Epistemic status: agent-authored DESIGN PROPOSAL — not authority.** Written at owner
> direction (Ryan Tufts, K-AUTH-1) on 2026-07-02 under the `DEC-042` harness-independent
> preparation sanction (`projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md:612`).
> This document proposes; it adopts nothing, implements nothing, and advances no lifecycle.
> NOT adopted. Live agent binding, R7 scope adoption, and app-dev dependency consumption
> remain gated: `D-21` is HELD (`projects/chirality-piping/execution/_Coordination/_DECISIONS/_REGISTER.md:45`),
> the `DEC-041` automation condition stands (`projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md:611`),
> and app-dev fence F3 applies to any live binding (register row D-24,
> `projects/chirality-piping/execution/_Coordination/_DECISIONS/_REGISTER.md:48`).
> Sources govern on any disagreement. All paths are repo-relative to the monorepo root.

## 1. Sanction and scope

`DEC-042` (`projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md:612`) holds
`D-21` and carves one exception, quoted exactly:

> "**Sanctioned exception:** harness-INDEPENDENT, fence-free embedded-agent PREPARATION may
> proceed as ordinary v0.1 prep without adopting R7 into scope — design/spec for the agent
> proposal→apply bridge and the `OperationSet`→Plan schema extension (step ordering + bridging
> rationale + a non-mutating step variant), candidate-generation and operation-schema-reconciliation
> design, and the piping-design domain retrieval index — none of which depend on the harness
> runtime or any cross-repo coordination. No live agent binding, no R7 scope adoption, and no
> app-dev dependency consumption proceed under this sanction (those stay behind `D-21` and the
> `DEC-041` automation condition)."

This document is the item-1 design: the **agent proposal→apply bridge** and the
**`OperationSet`→Plan schema extension** (step ordering + bridging rationale + a
non-mutating step variant). Item 2 (candidate generation and schema reconciliation) is the
companion document `projects/chirality-piping/plans/DESIGN_2026-07-02_candidate_generation_and_schema_reconciliation.md`.
Item 3 (the piping-design retrieval index) is **DONE** per `DEC-043`
(`projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md:613`) and is treated
here only as an available substrate — it is not re-planned.

## 2. What exists today (grounding)

The proposal→apply pipeline exists end-to-end for **user** intents and is deliberately
severed for **agent** proposals:

| Piece | Where | State |
|---|---|---|
| Formal operation schema (`OperationSet`/`ModelOperationRecord`) | `projects/chirality-piping/schemas/model_operation.schema.json:495` / `:230` | Exists (DEL-16-01) |
| Runtime intent (`EditorOperationIntent`) | `projects/chirality-piping/apps/desktop/src/types.ts:517` | Exists; the live seam input |
| Non-mutating validation | `validate_operation`, `projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs:387` ("Never mutates or returns a model", `:385-386`) | Exists |
| Controlled apply (clone, never in-place) | `apply_operation`, `projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs:397` | Exists |
| Result envelope `OperationOutcome` | `projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs:119`; published as `projects/chirality-piping/schemas/operation_outcome.schema.json` (Rust source governs, per its `description`) | Exists |
| Rule-check result contract | `projects/chirality-piping/schemas/rule_check_run_result.schema.json` | Exists (DEL-10-03) |
| User-intent apply path | `handleApplyIntent`, `projects/chirality-piping/apps/desktop/src/App.tsx:480`; queued-intent driver `:552` | Live |
| Agent proposal accept | `projects/chirality-piping/apps/desktop/src/features/agent-proposals/AgentProposalPanel.tsx:116` — `data-testid="accept-proposal-disabled"`, `disabled`, "Review-only until accepted mutation is implemented" | **Hard-disabled** |
| Agent proposal content | hardcoded sample, `build_sample_agent_proposal`, `projects/chirality-piping/apps/desktop/src-tauri/src/lib.rs:1435` (command `sample_agent_proposal` `:1527`) | Placeholder (item-2 territory) |

The gap this design addresses is exactly the one enumerated as BUILD item (2) in
`projects/chirality-piping/plans/INIT_2026-06-18_workspace_and_agent_design_resume.md:44`:
"the agent-proposal → `EditorOperationIntent` bridge (the disabled accept — highest-value,
fully harness-independent)". The two-schema divergence it must bridge is recorded at
`projects/chirality-piping/plans/INIT_2026-06-18_workspace_and_agent_design_resume.md:35`.

## 3. Step-ordering design: the proposal lifecycle mapped onto components

The governing lifecycle is the ADOPTED profile's operation-proposal contract
(`_DomainEngines/profiles/open_pipe_stress.yaml:112-118`):

```yaml
operation_proposal_contract:
  lifecycle: ["draft", "ready_for_review", "accepted", "rejected", "applied"]
  accepted_or_applied_requires:
    - "human approval bound to git SHA per K-AUTH-2"
    - "domain-engine-controlled apply or external terminal acceptance record (instance: HUMAN_APPROVED_FOR_PROJECT)"
```

and `K-DOMAIN-3` (`docs/CONTRACT.md:139`): a proposal is `proposal_only` until validated by
a declared deterministic tool and accepted by a human; application occurs only through a
domain-engine-controlled apply. The profile marks this on the desktop seam:
`requires_human_confirmation: true` (`_DomainEngines/profiles/open_pipe_stress.yaml:85`).

The design maps each lifecycle state to concrete component interactions:

| Lifecycle state (profile `:113`) | Component interaction | Data state |
|---|---|---|
| `draft` | Candidate generator (item 2) emits a formal `OperationSet` with `author_type: "agent"` (`projects/chirality-piping/schemas/model_operation.schema.json:306-315`), every operation `operation_status: "proposed"` (`:542-552`), `application_status: "not_applied"` (`:582-590`) | Formal record only; nothing shown as actionable |
| `ready_for_review` | For each operation change, the bridge derives an `EditorOperationIntent` (§4) and runs `validate_operation` (`projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs:387`) — the non-mutating path — producing an `OperationOutcome` with `diff_preview` rows and `diagnostics` (`:119-141`). Only when no blocking diagnostic exists does the proposal surface with an enabled review affordance (today's disabled button at `AgentProposalPanel.tsx:116` becomes the review entry point). Blocking findings hold the proposal in `draft` with visible diagnostics — findings, not silent fallbacks (crate boundary rule, `lib.rs:10-12`) | `operation_status: "ready_for_user_review"` (`schemas/model_operation.schema.json:548`); diff preview generated |
| `accepted` | **Human act only.** The user accepts in the review UI after seeing the diff preview. Acceptance produces an audit record via the DEL-16-03 acceptance/audit surface (explicit user acceptance required — `execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-03_User acceptance and operation audit trail/Specification.md`, DEL-16-03-REQ-006). No agent, timer, or default may produce this transition | Acceptance recorded; still `not_applied` |
| `applied` | The accepted, bridged intent(s) enter the same apply path user intents use: `handleApplyIntent` (`projects/chirality-piping/apps/desktop/src/App.tsx:480`) → `apply_operation` (`lib.rs:397`, clone-and-apply, input model never mutated in place) → session model replaced, solve results cleared, undo checkpoint pushed (existing App.tsx behavior) | `OperationOutcome` with `applied_model`, `input_model_unchanged: true`, honest `acceptance` receipt (`lib.rs:112-116`) |
| `rejected` | Human act in the review UI; the proposal is retained as a record (audit trail), never applied | `operation_status: "rejected"` (`schemas/model_operation.schema.json:549`) |

Ordering invariants the design fixes (these are design commitments, not implementations):

1. **No state skipping.** `draft → ready_for_review` requires a completed `validate_operation`
   pass; `ready_for_review → accepted` requires a human action; `accepted → applied` requires
   the domain-engine-controlled apply. There is no `draft → applied` edge.
2. **Validation is re-run at apply time.** `apply_operation` internally re-validates
   (`Mode::Apply` runs the same checks; blocking findings prevent application — `lib.rs:395-403`),
   so a stale acceptance cannot apply against a changed model: the intent's
   `claimed_model_hash` / the record's `required_current_hashes`
   (`schemas/model_operation.schema.json:463-480`) must still match the live model basis.
3. **Multi-operation sets apply in declared order, one change at a time,** because the seam
   is single-change (§4). A blocking outcome at step k halts the remainder of the set; already
   applied steps stay applied with their receipts, and the halt is surfaced as a finding.
   (Alternative — transactional all-or-nothing via a scratch clone chain — is listed as OPEN-1.)

Note a vocabulary seam the design must respect rather than paper over: the profile lifecycle
has `accepted`/`applied`, but the formal schema's `OperationStatus` enum
(`schemas/model_operation.schema.json:542-552`) does not — acceptance/application live in
`OperationValidation.application_status` (`:582-590`) and downstream DEL-16-03 records
(`OperationContractStatus`, `:375-406`: `user_acceptance_boundary: "downstream_user_acceptance_required"`,
`audit_trail_binding: "downstream_DEL-16-03"`). The design keeps that split: lifecycle state
is *derived* (status enum + validation block + acceptance record), not stored as a single
mutable field. OPEN-2 asks whether the enum should gain explicit terminal values.

## 4. The proposal→EditorOperationIntent bridge: rationale

### 4.1 The two schemas (both quoted)

**Formal (DEL-16-01)** — `projects/chirality-piping/schemas/model_operation.schema.json`:
an `OperationSet` (`:495-541`) contains `operations[]` of `ModelOperationRecord` (`:230-305`),
each with `operation_kind` from `add | move | modify | delete | reconnect | constraint | load |
support | design_knowledge | TBD` (`:407-421`), `changes[]` (`minItems: 1`) of `OperationChange`
(`:316-374`) with `change_kind` from `add_object | remove_object | set_field | move_geometry |
reconnect | update_constraint | update_load | update_support | attach_design_knowledge | TBD`
(`:332-345`), a structured `OperationValuePayload` (`:593-645`), full `Provenance` (`:690-734`),
`OperationPrecondition` with `required_current_hashes` (`:450-494`), and const-false
`ProfessionalBoundary` flags (`:658-688`).

**Runtime (live seam)** — `projects/chirality-piping/apps/desktop/src/types.ts:517-581`:

```ts
export type EditorOperationIntent = {
  queue_id?: string;
  operation_id: string;
  operation_kind: "create" | "connect" | "delete" | "insert" | "modify";
  operation_status: "proposed";
  author_type: "user";
  source?: EditorOperationSource;
  target: { object_type: EditorOperationObjectType; ref: string };
  change: { change_id: string; change_kind: /* 20 kinds, :530-550 */;
            field_label: string; field_path: string; before: string; after: string;
            unit: string; dimension: string; source_note: string };
  validation: { ... };            // :559-565
  audit_boundary: { ... };        // :566-571 (structured_operations_only, etc.)
  professional_boundary: { ... }; // :572-579 (const flags)
  rationale: string;              // :580
};
```

validated hand-rolled by `check_kinds` (`projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs:1228-1251`),
which accepts exactly the 20 runtime `change_kind` values.

### 4.2 Divergence table

| Facet | Formal `OperationSet` record | Runtime `EditorOperationIntent` | Bridge consequence |
|---|---|---|---|
| Cardinality | Set → N operations → N changes each (`schema:495`, `:270-276`) | Exactly one change per intent (`types.ts:528`) | 1 proposal ⇒ ordered sequence of intents (the Plan, §5) |
| `operation_kind` | 9 domain kinds + TBD (`schema:407-421`) | 5 editor verbs (`types.ts:520`) | Crosswalk needed (item-2 doc §4 owns the full table) |
| `change_kind` | 9 abstract kinds (`schema:332-345`) | 20 concrete kinds (`types.ts:530-550` = `check_kinds` list `lib.rs:1228`) | Runtime vocabulary is what the applier executes; bridge must land on it |
| `author_type` | enum incl. `"agent"` (`schema:306-315`) | const `"user"` (`types.ts:522`) | **Blocking divergence** — an agent-authored intent is unrepresentable today (OPEN-3) |
| Value payload | Structured `OperationValuePayload` (quantities with unit+dimension+provenance, `schema:593-645`) | Flat `before`/`after` strings + `unit` + `dimension` (`types.ts:551-557`) | Bridge flattens; loses structured provenance unless carried in `source_note`/`rationale` |
| Preconditions | `required_current_hashes` (min 1, model_state_record scope, `schema:463-480`) | `claimed_model_hash` is a separate optional argument to `validate_operation`/`apply_operation` (`lib.rs:387-401`) | Bridge maps the record's model-state hash to the call argument; MUST pass it, never omit |
| Status | 6-value `OperationStatus` (`schema:542-552`) | const `"proposed"` (`types.ts:521`) | Lifecycle lives outside the intent (§3 derivation) |
| Provenance | Full `Provenance` object (`schema:690-734`) | `source_note: string` + optional `EditorOperationSource` (`types.ts:511-515`) | Formal record remains the provenance system of record; intent carries a pointer |
| Boundary blocks | `ProfessionalBoundary` consts (`schema:658-688`) | identical const shape (`types.ts:572-579`) + `audit_boundary` (`:566-571`) | Aligned — carried through unchanged |

### 4.3 Bridging rationale (why bridge down, not replace)

The design **keeps the formal `OperationSet` as the proposal record and bridges each
`OperationChange` down to one `EditorOperationIntent` at review/apply time**, rather than
teaching the applier to consume `OperationSet` directly or having agents emit raw intents:

1. **The seam is proven and honest.** `validate_operation`/`apply_operation` are the DEC-020
   validated seam with deterministic diff previews, hash-bound model basis, and receipts
   (`lib.rs:1-15`, `:119-141`). Reusing the exact path user edits take means agent-applied
   changes get identical validation, identical diff rows, identical undo behavior
   (`App.tsx:480-550`) — no second apply path to audit.
2. **The formal record is the audit/provenance carrier.** Proposals need multi-change
   structure, assumptions, preconditions, and full provenance — all of which the runtime
   intent deliberately lacks. Discarding the formal record would orphan DEL-16-01/-16-03.
3. **Agents must not get a wider channel than humans.** Bridging down guarantees the agent's
   effective write vocabulary is a subset of the human editor's (`check_kinds` list), which
   is the locked frame: the agent "acts through the SAME structured-operation seam the human
   uses, as PROPOSALS" (`projects/chirality-piping/plans/INIT_2026-06-18_workspace_and_agent_design_resume.md:24`).

Bridge function (design signature, not code):
`bridge(record: ModelOperationRecord, change: OperationChange, basis: OperationModelBasis) → EditorOperationIntent + bridging_note`,
total over the crosswalk-covered kinds and **failing closed** otherwise: an unmappable kind
yields a blocking diagnostic on the proposal (consistent with the applier's own posture, e.g.
`OP-KIND-UNSUPPORTED` `lib.rs:1255` and the no-invention guard `lib.rs:1300-1311`),
never a best-effort guess. Each derived intent's `rationale` (`types.ts:580`) carries the
record's rationale, which is subject to the DEL-16-04 prohibited-claims scan
(`projects/chirality-piping/core/model_operations/agent_rationale/engine.py:43-74`).

## 5. The OperationSet→Plan extension

### 5.1 The app-dev harness "Plan" concept — what actually exists

Verified against the live tree: **there is no formal `Plan`/step data type in
`projects/chirality-app-dev/frontend/src/lib/harness/` today.** A search across that
directory finds "plan" only as: (a) the operator mode `plan/ask` in the normalized
permission-mode vocabulary `readOnly | workspaceWrite | dontAsk | ask | bypass`
(`projects/chirality-app-dev/frontend/src/lib/harness/permission-overlay.ts:23-28`; mode
semantics at `projects/chirality-app-dev/plans/DESIGN_2026-06-18_agent_orchestration_ui.md:42`
and `:82`), and (b) the *deferred design-stage* "task management / workflow co-composition
(human-alone authoring → loose plan + ongoing Type-1 session → a composable, non-programmatic
agent todo-list)" (`projects/chirality-app-dev/plans/DESIGN_2026-06-18_agent_orchestration_ui.md:18`).
(`scaffold.ts` has a `PackagePlan` type, but that is decomposition scaffolding, unrelated.)
The closest *executable* structures are the ordered event stream — `HarnessEvent` with the
43-type vocabulary including `tool.queued/started/progress/completed/failed`
(`projects/chirality-app-dev/frontend/src/lib/harness/event-schema.ts:3-47`, `:51-60`; count
per `D-28`/`DEC-055`, `projects/chirality-piping/execution/_Coordination/_DECISIONS/_REGISTER.md:52`) —
and the per-tool `humanGate` field on `HarnessToolDescriptor`
(`projects/chirality-app-dev/frontend/src/lib/harness/tool-descriptor.ts:84-105`).

Consequence for this design: the "Plan" this extension targets is a **piping-side schema
extension shaped to be alignable with that harness direction**, not an import of an app-dev
type (none exists to import, and consuming app-dev packages is gated anyway — `DEC-041`
automation condition, `SOFTWARE_DECOMP.md:611`; `DEC-042` "no app-dev dependency consumption",
`:612`). This preserves harness-independence exactly as the sanction requires.

### 5.2 Proposed Plan envelope (extension over `OperationSet`)

The design proposes a `plan` object added alongside (not inside) the existing `OperationSet`
definition — a pure ordering/gating overlay that references operations by id and adds nothing
to their content:

| Proposed Plan field | Type / values | Maps from / rationale |
|---|---|---|
| `plan_id` | Id | New; sibling of `operation_set_id` (`schemas/model_operation.schema.json:509`) |
| `operation_set_ref` | Reference | Binds the plan to exactly one `OperationSet` (`:495-541`) |
| `model_basis` | (by reference) | Reuses `OperationModelBasis` (`:422-449`) — a plan is only meaningful against a hash-pinned basis |
| `steps[]` (ordered, min 1) | PlanStep | The extension's core; order is the array order, no implicit reordering |
| `steps[].step_id` | Id | New |
| `steps[].operation_ref` / `change_ref` | Ids | Points at one `ModelOperationRecord.operation_id` (`:249`) + one `change_id` (`:329`) — i.e., one bridgeable unit (§4 cardinality) |
| `steps[].step_kind` | `validate_only` \| `apply` \| `review_note` | The **non-mutating step variant** (§5.3) |
| `steps[].bridging_rationale` | string | Human-readable record of how the formal change maps to the runtime intent (the §4 crosswalk decision for this step); required — a step whose mapping cannot be stated is not `ready_for_review` |
| `steps[].depends_on[]` | step_ids | Explicit ordering constraints beyond array order (e.g., `create_node` before `connect_pipe_run` — both runtime kinds, `types.ts:534/:536`) |
| `steps[].human_gate` | const `"ask"` | Every mutating step is human-gated (K-DOMAIN-3, `docs/CONTRACT.md:139`; profile `:85`, `:116-118`). Shaped like — but not imported from — the harness `humanGate` descriptor field (`tool-descriptor.ts:96`). A tunable `auto` tier for reversible/cosmetic steps is the locked-frame direction (`INIT_2026-06-18...md:25`) but is NOT proposed here; it needs its own ruling (OPEN-4) |
| `steps[].expected_outcome_ref` | Reference | Slot for binding the step to its `OperationOutcome` / diff-preview checksum after validation (`DiffPreviewRef`, `schemas/model_operation.schema.json:195-224`) |
| `provenance`, `professional_boundary` | existing `$defs` | Same const-false boundary flags as the set (`:658-688`) |

Execution semantics (design commitments): steps run strictly in order; each `apply` step is
individually human-accepted (or the human accepts the plan and then confirms each
engineering-significant step — presentation detail left OPEN-4); any blocking
`OperationOutcome` halts the plan (§3 invariant 3); every step emission is loggable as an
event pair shaped like `tool.started`/`tool.completed`/`tool.failed`
(`event-schema.ts:23-26`) so a future live-loop UI can render plans without a new vocabulary —
alignment by shape, not by dependency.

### 5.3 The non-mutating step variant

Grounded directly in the applier's existing dual entry points:

- `step_kind: "validate_only"` executes `validate_operation`
  (`projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs:387-393`) —
  documented "Never mutates or returns a model" (`:385-386`) — and yields a full
  `OperationOutcome` (diff preview + diagnostics + model-basis evidence) with
  `application_status` never reaching applied and `applied_model: None` (`:119-141`).
- `step_kind: "review_note"` covers non-operation steps: present retrieval evidence, a framed
  choice, or an interpretation flag (the three human-judgment shapes,
  `INIT_2026-06-18...md:26`) with **no** seam call at all. Today's sample proposal is
  precisely this shape (`attach_design_knowledge` review note, `apps/desktop/src-tauri/src/lib.rs:1476-1524`).

Uses: (a) a whole-plan dry-run is the plan with every `apply` downgraded to `validate_only`;
(b) an agent may only ever *emit* plans whose steps are `validate_only`/`review_note` —
upgrading a step to `apply` is part of the human acceptance act, never the proposal. This
gives the profile lifecycle a concrete mechanical meaning: `draft` plans contain no live
`apply` steps.

Limitation, stated honestly: `validate_only` steps all validate against the *same* base
model, so a multi-step dry-run cannot see step k's effect on step k+1. Sequential simulation
would need to chain the `applied_model` outputs of `apply_operation` on a scratch clone
(never accepted state — `input_model_unchanged` receipts, `lib.rs:135`). Whether that
scratch-chain dry-run is worth specifying is OPEN-1.

## 6. Open questions (labeled OPEN — not resolved here)

- **OPEN-1 — plan atomicity.** Halt-and-keep (proposed default) vs all-or-nothing via a
  scratch-clone chain vs per-step user choice. Interacts with the session undo stack
  (checkpointing per applied intent, existing App.tsx behavior).
- **OPEN-2 — lifecycle vocabulary.** Should `OperationStatus`
  (`schemas/model_operation.schema.json:542-552`) gain `accepted`/`applied` terminal values,
  or does derived state (status + validation + DEL-16-03 acceptance record) remain the
  contract? The profile lifecycle (`open_pipe_stress.yaml:113`) and the schema enum currently
  differ; either answer is a schema-touching governed act.
- **OPEN-3 — `author_type` on the runtime intent.** `EditorOperationIntent.author_type` is
  const `"user"` (`types.ts:522`). Options: widen to `"user" | "agent"`; keep const and record
  agent authorship only in the formal record + `source`/`rationale`; or a distinct
  `accepted_agent_intent` wrapper. Audit-trail implications (DEL-16-03 actor metadata) make
  this an owner-visible choice. The item-2 reconciliation doc carries the field-level detail.
- **OPEN-4 — gate granularity.** Per-plan vs per-step acceptance presentation, and whether a
  future tunable `auto` tier for reversible steps (locked-frame direction,
  `INIT_2026-06-18...md:25`) is introduced with the plan schema or strictly later. The
  profile's PROPOSED risk-class axis (`open_pipe_stress.yaml:120-125`, explicitly "not in
  engine today") is the natural hook but is itself unadopted in the engine.
- **OPEN-5 — where the Plan envelope lives.** Same schema file as a new `$defs.Plan` +
  optional top-level property, vs a separate `model_operation_plan.schema.json`. Affects
  DEL-16-01's strict `additionalProperties: false` envelope (`schemas/model_operation.schema.json:7`).

## 7. What this design does NOT do

- No schema edits: `projects/chirality-piping/schemas/model_operation.schema.json` and all
  other `schemas/**` files are untouched; the Plan envelope above is a proposal only.
- No code: no changes to `core/**`, `apps/**`; the disabled accept button
  (`AgentProposalPanel.tsx:116`) stays disabled.
- No new deliverables, no `execution/**` writes, no `_STATUS.md` or lifecycle claims, no
  review-finding dispositions.
- No R7 scope adoption, no live agent binding, no app-dev package consumption — all remain
  behind `D-21` (held), the `DEC-041` automation condition, and app-dev fence F3
  (`SOFTWARE_DECOMP.md:611-612`; `_REGISTER.md:45`, `:48`).
- No professional, certification, sealing, authentication, approval, or code-compliance
  claim — the const-false boundary (`schemas/model_operation.schema.json:658-688`) is
  preserved by construction everywhere above.

## 8. Where this would land if adopted (owner's choice — not made here)

Both routes are viable; this document deliberately does not choose:

1. **Amend DEL-16-01's Specification** (`execution/PKG-16_Model Operation and Agent Proposal
   Framework/1_Working/DEL-16-01_Structured model operation schema/Specification.md`) to add
   the Plan envelope and bridge contract as new requirements. Fit: the Plan is a data-model
   extension, and DEL-16-01's scope already owns the operation envelope; but its current
   Scope explicitly excludes application/acceptance behavior (owned by DEL-16-02/-16-03), so
   the bridge's *behavioral* half would spill into DEL-16-02 (validation/preview,
   `.../DEL-16-02_Operation validation and diff preview/Specification.md`) and DEL-16-03
   (acceptance/audit, `.../DEL-16-03_User acceptance and operation audit trail/Specification.md`),
   with DEL-16-04 governing the rationale/boundary controls
   (`.../DEL-16-04_Agent rationale and professional-boundary controls/Specification.md`).
2. **A new design deliverable via governed decomposition** (a PKG-16 addition or successor
   package) that owns proposal→apply bridging and plan orchestration end-to-end, referencing
   DEL-16-01..04 as upstream.

Verified in this loop's discovery: **no existing PKG-16 Specification scopes DEC-042 items
1–2** — DEL-16-01 is the schema envelope only; DEL-16-02 excludes schema ownership and
application; DEL-16-03 is acceptance/audit recording; DEL-16-04 is rationale/boundary
controls and explicitly excludes the other three. Either landing route is therefore a real
scope act (human-gated), not a clerical edit.
