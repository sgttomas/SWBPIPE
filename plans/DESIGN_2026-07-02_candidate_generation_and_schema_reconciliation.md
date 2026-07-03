# DESIGN — Candidate generation and operation-schema reconciliation (DEC-042 item 2)

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

`DEC-042` (`projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md:612`)
sanctions, quoted exactly from the item-2 clause: "harness-INDEPENDENT, fence-free
embedded-agent PREPARATION may proceed as ordinary v0.1 prep without adopting R7 into
scope — design/spec for ... candidate-generation and operation-schema-reconciliation
design ... none of which depend on the harness runtime or any cross-repo coordination."

This document covers both halves of item 2:

- **§2–3 Candidate generation** — the design that supersedes the hardcoded sample proposal
  (BUILD item (4), "the genuinely new R7 piece; today only a hardcoded sample proposal",
  `projects/chirality-piping/plans/INIT_2026-06-18_workspace_and_agent_design_resume.md:44`).
- **§4–5 Operation-schema reconciliation** — the formal `OperationSet` vs runtime
  `EditorOperationIntent` reconciliation (BUILD item (6), same line; divergence recorded at
  `projects/chirality-piping/plans/INIT_2026-06-18_workspace_and_agent_design_resume.md:35`).

Item 3 (retrieval index) is **DONE** per `DEC-043`
(`projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md:613`); it appears
below only as the grounding substrate candidate generation consumes — it is not re-planned.
The item-1 companion (bridge + Plan extension) is
`projects/chirality-piping/plans/DESIGN_2026-07-02_proposal_apply_bridge_and_operationset_plan_extension.md`.

## 2. What candidate generation replaces

Today the only "agent proposal" in the product is a deterministic hardcoded sample:
`build_sample_agent_proposal` (`projects/chirality-piping/apps/desktop/src-tauri/src/lib.rs:1435`),
exposed as the Tauri command `sample_agent_proposal` (`:1527`). It fabricates a single
review-only `attach_design_knowledge` note against the current mechanics-preview context
(`document_kind: "openpipestress.product_preview.agent_proposal"`, `:1476`), always
`application_status: "not_applied"`, `accepted_model_mutated: false`, with const-false
professional-boundary flags. The review UI's accept action is hard-disabled
(`projects/chirality-piping/apps/desktop/src/features/agent-proposals/AgentProposalPanel.tsx:116`).

Two properties of the sample are **kept** by this design (they are boundary behavior, not
placeholder behavior): proposals carry rationale + assumptions + boundary flags, and
proposals are inert until a human acts. Everything else — the fixed target selection, the
fixed operation, the fixed prose — is what candidate generation replaces.

## 3. Candidate-generation design

### 3.1 Inputs

A generation request assembles, read-only:

| Input | Source (existing contract) |
|---|---|
| Current session model document | The same model JSON the seam validates against (`validate_operation(model, ...)`, `projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs:387`) plus its model-basis hash evidence (`ModelBasisEvidence`, cf. `projects/chirality-piping/schemas/operation_outcome.schema.json` `model_basis`) |
| Solve results / diagnostics | The mechanics preview result already consumed by the sample builder (`apps/desktop/src-tauri/src/lib.rs:1439-1470`) |
| Rule-check outcomes | `projects/chirality-piping/schemas/rule_check_run_result.schema.json` — per-check outcomes + worst-of `aggregate_status` (`USER_RULE_FAILED > RULE_INPUTS_INCOMPLETE > USER_RULE_CHECKED`); the Loop-1-published result contract the generator reads to find *what needs attention* |
| Prior operation outcomes | `projects/chirality-piping/schemas/operation_outcome.schema.json` envelopes (diff previews, diagnostics, receipts) |
| User selection/prompt | The selected review target (as today, `SelectedReviewTarget` handling `:1471-1473`) and the user's ask |
| Domain research evidence | The DEC-043 retrieval index (§3.2) |

### 3.2 Grounding via the DEC-043 retrieval index (available substrate)

The piping-design retrieval index exists (built under the DEC-042 prep lane; `DEC-043`,
`SOFTWARE_DECOMP.md:613`) and is queried locally (`tools/retrieval/query_source_index.py`).
Its use inside candidate generation is bound by the standing reliability constraint, routed
to agents at `projects/chirality-piping/AGENTS.md:42-60` ("Knowledge-source reliability"):

1. Retrieval evidence may ground **concepts, terminology, and approach** — e.g. "long
   vertical runs near equipment nozzles commonly get a spring support; here are the corpus
   passages" — attached to a candidate as cited rationale/evidence.
2. **Extracted equations are never presented as authoritative**, and each cited artifact's
   review status (cleared vs unverified, per the maintainer's per-artifact JSON review
   status) is surfaced with the citation.
3. **No physics values from the corpus.** Corpus content never supplies numeric inputs to a
   candidate operation (allowables, factors, stiffnesses, dimensions). Numbers come from the
   model, the user, or vetted project data — or the field stays empty and the candidate is
   emitted as blocked/incomplete (§3.3).

The index is consumed as a local tool; this creates no harness or cross-repo dependency
(the sanction's own boundary, `SOFTWARE_DECOMP.md:612`).

### 3.3 Generation constraints (the guards, stated as design invariants)

- **Values are never invented.** This mirrors the applier's boundary rules verbatim:
  "blocked operations are findings, not silent fallbacks (no invented values, no ungoverned
  unit conversion, no geometry defaults)"
  (`projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs:10-12`), and
  its concrete guard behavior — e.g. underspecified viewport gestures are *held*, not
  completed ("values are not invented on the user's behalf",
  `OP-GEOMETRY-INPUT-INCOMPLETE`, `lib.rs:1303-1308`). The generator inherits the same
  posture one stage earlier: a candidate missing a required value is emitted as a `draft`
  with an explicit incomplete-input finding (or as a `review_note`/framed choice asking the
  human for the value), never with a guessed value. The generator must not become the layer
  that launders invented numbers past the applier's guards.
- **Structured output only.** Candidates are emitted as formal `OperationSet` records
  (`projects/chirality-piping/schemas/model_operation.schema.json:495-541`) with
  `author_type: "agent"` (`:306-315`), `operation_status: "proposed"` (`:542-552`), full
  `Provenance` (`:690-734`) on every change and quantity, and const-false
  `ProfessionalBoundary` flags (`:658-688`). Free-text-only "suggestions" are not proposals.
- **Self-validation before presentation.** Every candidate change is bridged (item-1 doc §4)
  and run through `validate_operation` (`lib.rs:387`, never mutates). Blocking outcomes
  either loop back into regeneration (bounded retries) or surface as a blocked draft with
  its diagnostics. Nothing reaches the review surface without a generated diff preview
  (`OperationOutcome.diff_preview`, `lib.rs:119-141`).
- **Rationale passes the boundary scan.** Candidate rationale/assumption text is checked
  against the DEL-16-04 prohibited-claims patterns — compliance/certification/sealing/
  authentication/professional-approval/external-validation/autonomous-acceptance
  (`PROHIBITED_CLAIM_PATTERNS`,
  `projects/chirality-piping/core/model_operations/agent_rationale/engine.py:43-74`). A hit
  blocks presentation, it does not get reworded silently.
- **Human gate before any apply.** Per `K-DOMAIN-3` (`docs/CONTRACT.md:139`) and the ADOPTED
  profile's operation-proposal contract — lifecycle
  `draft → ready_for_review → accepted/rejected → applied` with
  `accepted_or_applied_requires` human approval
  (`_DomainEngines/profiles/open_pipe_stress.yaml:112-118`;
  `requires_human_confirmation: true` on the desktop seam, `:85`) — the generator's terminal
  state is `ready_for_review`. It has no path to `accepted` or `applied`; those transitions
  belong to the human via the item-1 bridge. This matches DEL-16-03's default posture
  (explicit user acceptance required; agent-only signals yield `held_for_user_acceptance`,
  DEL-16-03-REQ-006, `execution/PKG-16_Model Operation and Agent Proposal
  Framework/1_Working/DEL-16-03_User acceptance and operation audit trail/Specification.md`).
- **Three output shapes, not one.** Following the locked frame
  (`plans/INIT_2026-06-18_workspace_and_agent_design_resume.md:26`), a generation turn may
  yield: a **proposal** (OperationSet candidates), a **framed choice** (two candidate sets
  with a trade-off statement), or an **interpretation flag** (no operation at all — a
  borderline finding deferred to human judgment). The framed-choice and flag shapes reuse
  the item-1 `review_note` non-mutating step variant.

### 3.4 Pipeline (design, provider-agnostic)

```
context assembly ─► retrieval grounding ─► candidate drafting ─► self-validation ─► boundary scan ─► ready_for_review
   (§3.1 inputs)      (§3.2, cited,          (formal OperationSet,   (validate_operation      (agent_rationale        (review UI;
                       review-status          author_type=agent,      per bridged change;      PROHIBITED_CLAIM        human accepts /
                       surfaced)              no invented values)     bounded repair loop)     _PATTERNS)              edits / rejects)
```

The drafting stage is deliberately unspecified as to *model/provider*: under `DEC-051`
(`SOFTWARE_DECOMP.md:621`) the runtime residency posture is open (local / Anthropic / other
providers, owner-configured), and under `DEC-041`/`D-21` no live binding is designed here.
Everything upstream and downstream of the drafting box is deterministic, local, and
implementable/testable harness-independently (fixture-driven drafting stubs stand in for a
live model — the same pattern as today's deterministic sample).

### 3.5 Risks (candidate generation)

| Risk | Mitigation in this design |
|---|---|
| Plausible-but-wrong engineering suggestions | Human gate is structural (§3.3); rationale must cite model/results/corpus evidence with review status; interpretation-flag shape exists precisely so the generator can decline |
| Corpus equation contamination | DEC-043 constraints hard-wired into the grounding stage (§3.2); equations never authoritative, never numeric inputs |
| Prompt-injected or degenerate rationale making authority claims | DEL-16-04 scan blocks presentation (§3.3) |
| Generator drift into direct mutation | Output type is `OperationSet` only; the only apply path is the item-1 bridge through `apply_operation`; `direct_model_mutation_allowed: false` is const in both schemas (`model_operation.schema.json:393-395`; `types.ts:568`) |
| Volume/noise (proposal spam) | OPEN-B (budget/ranking policy) |

## 4. Operation-schema reconciliation design

### 4.1 The two surfaces

The project carries **two operation schemas that differ**
(`plans/INIT_2026-06-18_workspace_and_agent_design_resume.md:35`): the formal DEL-16-01
contract (`projects/chirality-piping/schemas/model_operation.schema.json`) and the live
runtime intent (`projects/chirality-piping/apps/desktop/src/types.ts:517-581`) hand-validated
by `check_kinds` (`projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs:1228-1251`).
Reconciliation means: one documented crosswalk, a declared winner per facet, and a staged
path toward a single source of truth — not a big-bang rewrite.

### 4.2 Per-field reconciliation table (which surface wins where)

| Facet | Formal (`model_operation.schema.json`) | Runtime (`types.ts` / `check_kinds`) | Winner + rationale |
|---|---|---|---|
| Envelope / cardinality | `OperationSet` → `operations[]` → `changes[]` (`:495`, `:270-276`) | one change per intent (`types.ts:528`) | **Formal** for records/proposals; runtime stays the single-change *execution* unit. Sets are ordered into per-change steps (item-1 Plan) |
| `operation_kind` | `add/move/modify/delete/reconnect/constraint/load/support/design_knowledge/TBD` (`:407-421`) | `create/connect/delete/insert/modify` (`types.ts:520`) | **Formal** names the domain taxonomy; runtime verbs are retained as an editor-facing alias set. Crosswalk: `add→create`, `reconnect→connect`, `delete→delete`, `modify/constraint/load/support→modify` (kind refined by `change_kind`), `move→modify` on geometry fields, `insert→add` specialization (component symbols), `design_knowledge→` review-note (non-mutating; no runtime apply kind today). Each mapping is recorded per-step as `bridging_rationale` (item-1 doc §5.2); unmappable ⇒ fail closed |
| `change_kind` | 9 abstract kinds (`:332-345`) | 20 concrete kinds (`types.ts:530-550` = `check_kinds` `lib.rs:1228-1251`) | **Runtime** wins the executable vocabulary — it is what the applier actually validates/applies, entity by entity. The formal enum is the *classification*; reconciliation adds a documented many-to-one map (e.g. `add_object→create_node/create_section/create_material/create_support/create_load_case/create_primitive_load/create_combination/create_combination_term`; `remove_object→delete_*`; `set_field→set_field`; `update_load/update_support→update_load/update_support`; `reconnect→connect_pipe_run`; `move_geometry→set_field` on position fields; `attach_design_knowledge→` no runtime kind — review-note only; `update_constraint→` no runtime kind today — OPEN-C) |
| `author_type` | `user/agent/import_adapter/project_template/TBD` (`:306-315`) | const `"user"` (`types.ts:522`) | **Formal** is the target; the runtime const is the current enforcement point that keeps agent intents out of the apply path. Reconciliation options (owner-visible, not chosen here): widen the const to `"user" \| "agent"` with DEL-16-03 actor metadata carried through; or keep const `"user"` where the *acceptance act* re-authors the intent as user-authored with the agent recorded in `source`/`rationale`/audit record. The second preserves "only user-queued intents apply" (`App.tsx:480/:552`) at the cost of encoding authorship indirection |
| Value payload | structured `OperationValuePayload` (quantities w/ unit+dimension+provenance, `:593-645`) | flat `before/after` strings + `unit` + `dimension` + `source_note` (`types.ts:551-557`) | **Formal** for provenance and multi-value payloads; **runtime** for the seam call (the applier resolves fields from the flat shape). Bridge flattens deterministically; the formal record remains attached so nothing is lost |
| Preconditions / hash binding | `required_current_hashes` (min 1, `model_state_record` scope, `:463-480`) | optional `claimed_model_hash` argument (`lib.rs:387-401`) | **Formal** semantics win: the bridge must always pass the record's accepted-model-state hash as `claimed_model_hash`; "optional" at the seam is a permissiveness the agent path never uses |
| Status / lifecycle | `OperationStatus` 6 values, no accepted/applied (`:542-552`) + `OperationValidation.application_status` (`:582-590`) | const `"proposed"` + free-string `validation` block (`types.ts:521`, `:559-565`) | **Profile** lifecycle governs both (`open_pipe_stress.yaml:113`); state is derived, not stored twice (item-1 doc §3). Schema enum growth is OPEN (item-1 OPEN-2) |
| Provenance | full `Provenance` object required everywhere (`:690-734`) | `source_note` string + optional `EditorOperationSource` (`types.ts:511-515`, `:557`) | **Formal** — system of record. Runtime carries a reference, never the substitute |
| Boundary flags | `ProfessionalBoundary` consts (`:658-688`) | identical consts + `audit_boundary` (`types.ts:566-579`) | Already aligned; frozen. Any change is a contract-level human act |
| Result envelope | (downstream refs: `diff_preview_binding: downstream_DEL-16-02`, `audit_trail_binding: downstream_DEL-16-03`, `:399-404`) | `OperationOutcome` (`lib.rs:119-141`) | **Rust source governs** — already declared by `schemas/operation_outcome.schema.json` (`description`: "on any disagreement the Rust source governs"). Reconciliation follows that precedent for the intent schema too |

### 4.3 Migration sketch (all steps are future governed acts, none performed here)

1. **Document the crosswalk (this design).** No repo surface changes; the table above is the
   proposal.
2. **Publish the runtime intent as a schema.** Add an `editor_operation_intent.schema.json`
   derived field-by-field from `types.ts:517-581` + `check_kinds` (`lib.rs:1228-1251`) with
   the same "Rust/TS source governs on disagreement" posture as
   `operation_outcome.schema.json`. This makes the divergence machine-checkable before it is
   shrunk. (Schema write — DEL-16-01-adjacent governed act.)
3. **Extend the formal schema minimally.** Encode the `change_kind` crosswalk (either widen
   the formal enum or add a `runtime_change_kind` mapping annotation) and resolve
   `author_type` per the OPEN decision. (Schema + spec amendment, human-gated.)
4. **Single-source the types.** Generate the TS types (or a conformance test) from the
   published schemas so `types.ts` cannot drift silently; `check_kinds` gains a conformance
   test against the same enumeration. (Code + test tranche.)
5. **Retire ad-hoc shapes.** The sample proposal's one-off JSON shape
   (`apps/desktop/src-tauri/src/lib.rs:1476`) is replaced by the formal `OperationSet` +
   Plan envelope (item-1 doc §5.2) once candidate generation lands.

### 4.4 Risks (reconciliation)

| Risk | Note |
|---|---|
| Enum drift between the three enumerations (formal enum, TS union, `check_kinds` match) | Exists today; step 2/4 make it test-visible. Until then the crosswalk table is the only guard |
| `author_type` widening weakens the "only user-queued intents apply" invariant (`App.tsx:480/:552`) | Both options in §4.2 keep a human act between agent authorship and apply; the choice changes *where* authorship is recorded, not *whether* the gate exists |
| Hash-binding mismatch (`required_current_hashes` required vs `claimed_model_hash` optional) | Closed by bridge rule (always pass); a conformance test should enforce it at step 4 |
| Double bookkeeping (formal record + derived intent + outcome) diverging mid-lifecycle | The formal record is append-only through the lifecycle; intents are derived, disposable, and re-derived after any model-basis change |
| Crosswalk gaps (`update_constraint`, `design_knowledge` have no runtime kinds) | Fail closed at the bridge (blocking finding); OPEN-C tracks whether runtime kinds are added or the formal kinds stay non-executable |

## 5. Open questions (labeled OPEN — not resolved here)

- **OPEN-A — drafting engine location.** In-process deterministic templates first
  (harness-independent, testable now) vs LLM-drafted from the start (gated by `D-21`/
  `DEC-041` for live binding). The pipeline (§3.4) is valid for both; the sequencing choice
  is the owner's.
- **OPEN-B — candidate budget/ranking.** How many candidates per turn, ranked how, and
  whether low-confidence candidates surface as interpretation flags instead. No grounding
  exists to decide this yet.
- **OPEN-C — crosswalk gaps.** `update_constraint` and executable `design_knowledge` have no
  runtime `change_kind` today (§4.2). Add runtime kinds (applier work) or declare those
  formal kinds non-executable-by-design?
- **OPEN-D — where the reconciliation artifacts live.** Step-2's published intent schema:
  new `schemas/` file under DEL-16-01's umbrella vs DEL-10-03's published-result-schema
  precedent. Interacts with the landing choice in §7.
- **OPEN-E — retrieval evidence format in proposals.** How corpus citations + per-artifact
  review status attach to a proposal record — `assumptions`/`reference_values`
  (`model_operation.schema.json:626-631`) vs a dedicated evidence block. Any answer is a
  schema-touching act.

## 6. What this design does NOT do

- No schema edits (`schemas/**` untouched), no code (`core/**`, `apps/**` untouched): the
  hardcoded sample (`apps/desktop/src-tauri/src/lib.rs:1435`) keeps running and the accept
  button stays disabled (`AgentProposalPanel.tsx:116`) until a governed implementation act.
- No new deliverables, no `execution/**` writes, no `_STATUS.md` edits, no lifecycle claims,
  no review-finding dispositions.
- No R7 scope adoption, no live agent binding, no app-dev dependency consumption — gated by
  `D-21` (held), the `DEC-041` automation condition, and app-dev fence F3
  (`SOFTWARE_DECOMP.md:611-612`; `_REGISTER.md:45`, `:48`).
- No re-planning of the retrieval index (item 3, DONE per `DEC-043`,
  `SOFTWARE_DECOMP.md:613`) and no relaxation of its equation-reliability constraint.
- No professional, certification, sealing, authentication, approval, or code-compliance
  claim; the const-false boundary flags and the DEL-16-04 scan are load-bearing in every
  section above.

## 7. Where this would land if adopted (owner's choice — not made here)

Verified in this loop's discovery: **no existing PKG-16 Specification scopes DEC-042 items
1–2.** Per-element natural homes, without choosing the vehicle:

| Design element | Nearest existing deliverable surface |
|---|---|
| Published runtime-intent schema + formal-schema extensions (§4.3 steps 2–3) | DEL-16-01 (`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-01_Structured model operation schema/Specification.md` — owns the operation data-model contract; its Scope currently covers only the existing envelope) |
| Self-validation stage, diff-preview obligations (§3.3) | DEL-16-02 (`.../DEL-16-02_Operation validation and diff preview/Specification.md`) |
| Acceptance gating, audit/actor metadata for agent-authored records (§3.3, §4.2 author_type) | DEL-16-03 (`.../DEL-16-03_User acceptance and operation audit trail/Specification.md`) |
| Rationale scan integration, evidence/review-status surfacing, boundary controls (§3.2–3.3) | DEL-16-04 (`.../DEL-16-04_Agent rationale and professional-boundary controls/Specification.md` — its Excluded list currently defers "final UI/agent workflow presentation") |
| Candidate-generation pipeline as a whole (§3) | **No existing deliverable owns it.** Owner options: a new design deliverable via governed decomposition (PKG-16 addition or successor package), or amendments distributing it across DEL-16-01..04 as above |

Both routes — new deliverable via decomposition, or DEL-16-01(+siblings) Specification
amendment — are real scope acts requiring the human decomposition/adoption path; this
document provides the material for either and selects neither.
