# D-21 - PRD Scope-Change to the v0.2 Milestone Set (Governed SCOPE_CHANGE Preparation)

> **Epistemic status: agent-prepared PROPOSAL — not authority.** Prepared at owner
> direction (Ryan Tufts, K-AUTH-1) on 2026-07-02 under the bridge work loop
> (`_DomainEngines/bridge/WORKPLAN_2026-07-02_bridge_loop.md`). Preparation does
> not adopt: the `D-21` adoption hold stands until the owner rules in §7. Sources
> govern on any disagreement with this packet.

**Date prepared:** 2026-07-02
**Prepared by:** bridge work loop agent, executing the owner's 2026-07-02
queue-item-4 lane direction (the standing bridge plan recorded `D-21` packet
preparation as "execution-ready on your word alone",
`_DomainEngines/bridge/WORKPLAN_2026-07-02_bridge_loop.md:175-178`, with "prep
permitted, adoption held", `:124-127`).
**Epistemic status:** PROPOSAL (non-governing). Only the human project
authority rules. Nothing here changes lifecycle state, adopts v0.2/R6/R7 scope,
issues deliverables, creates release readiness, professional approval,
certification, sealing, authentication, code-compliance acceptance, or asserts
that any PRD milestone is met until the human records the ruling.

---

## 1. Decision Statement And Scope

Decide whether to adopt the **PRD scope-change to the v0.2 milestone set** via
the governed `SCOPE_CHANGE` path, exactly as the register scopes it
(`execution/_Coordination/_DECISIONS/_REGISTER.md:45`):

- adopt the v0.2 release milestones **R6** (Design Knowledge and Handoff Beta)
  and **R7** (Agent-Assisted Design and Candidate Generation) plus the
  **inserted v0.2 R3** (States, Runs, and Generic Comparison), promoting
  `docs/_ScopeChange/OpenPipeStress_PRD_v0.2.md` (or its delta) to authority
  over the current governing `docs/PRD.md` (v0.1);
- adopt the **v0.2 FR renumber** — the flat `FR-001..FR-025` set is replaced by
  the namespaced families `FR-MOD/KNOW/GUI/SOL/RULE/CMP/HAND/AGENT/REP` — a
  **traceability-breaking** change that **requires an FR crosswalk**
  (`plans/PLAN_2026-06-17_prd_completion.md:263`). The mandatory crosswalk is
  Annex A of this packet.

This packet is preparation only. It does not adopt the scope-change, does not
run the separate `SCOPE_CHANGE` execution workflow (§3.4), does not bind a live
agent, does not consume app-dev packages, and does not advance any lifecycle or
target stage. The decision blocks the Extended Horizon (Phases G/H/I) per the
register row.

## 2. Verified Facts (Checked Cold, 2026-07-02)

All paths are repo-relative to `projects/chirality-piping/` unless they start
with `_DomainEngines/`.

| Check | Result |
|---|---|
| Register row state | `D-21` is `NOT_PREPARED (held)` at `execution/_Coordination/_DECISIONS/_REGISTER.md:45`; scope as restated in §1; Blocks = "Extended Horizon (Phases G/H/I)". |
| Prerequisite `D-27` | RULED (`_REGISTER.md:51`), and its register row names itself "prerequisite to `D-21` preparation". Ruling `DEC-054` (`execution/_Decomposition/SOFTWARE_DECOMP.md:624`) accepted a conditional R4 gate and advanced the current target stage to PRD R5. The `D-27` packet holds `D-21` explicitly: "Held `D-21` remains held. Accepting R4 exit does not adopt v0.2/R6/R7 scope" (`D-27_r4_exit_clearance_stage_advancement.md:106-108`) and "`D-21` remains held until the governed `SCOPE_CHANGE` packet is separately prepared and ruled" (`:179-180`). |
| `DEC-042` posture | `execution/_Decomposition/SOFTWARE_DECOMP.md:612`: `D-21` "remain HELD until the R4/R5 lead-up"; only harness-independent embedded-agent design/spec preparation was sanctioned; the row "stays NOT_PREPARED (no packet is prepared while held)". The R4/R5 lead-up has now arrived (`DEC-054`, R5 current), and the owner's 2026-07-02 bridge-loop direction supersedes the no-packet clause **for preparation only** (`_DomainEngines/bridge/WORKPLAN_2026-07-02_bridge_loop.md:124-127`, `:175-178`); adoption stays held. |
| Timing guidance | The completion plan recommended preparing this packet "only when R4/R5 are near exit, so the governing arc is not destabilized mid-flight" (`plans/PLAN_2026-06-17_prd_completion.md:100`). R4 exit is cleared (`DEC-054`) and R5 is the current target, so the guidance is satisfied. |
| Current stage | R5 Engineering Beta is the current ordinary in-stage target; the R5 row lists "`D-21` scope-change preparation when selected" among Phase E work (`docs/PLAN.md:84`; stage authority `DEC-054` per `docs/PLAN.md:74`). |
| Governing v0.1 PRD | `docs/PRD.md` is "Document version: 0.1 Draft" (`docs/PRD.md:5`). Flat FR set `FR-001..FR-025` at `docs/PRD.md:334-358`. Release milestones are §22 R0–R5 only (`docs/PRD.md:1158-1253`); §23 begins at `:1257`; **no R6 or R7 exists in the governing PRD**. |
| v0.2 scope source | `docs/_ScopeChange/OpenPipeStress_PRD_v0.2.md` is "Document version: 0.2 Draft", "Status: Replacement PRD / product definition" (`:5-7`), and declares supersession of the v0.1 framing (`:32-38`). Namespaced FR families in §11 (`:416-528`). Renumbered milestones in §24 (`:1448-1585`): R0 (`:1450`), R1 (`:1467`), R2 (`:1485`), **inserted R3 States/Runs/Comparison** (`:1503`), R4 Rule Packs and Private Libraries (`:1520`), R5 Piping Components and Nonlinear Supports (`:1537`), **new R6 Design Knowledge and Handoff Beta** (`:1554`), **new R7 Agent-Assisted Design and Candidate Generation** (`:1571`). |
| Milestone renumber shape | v0.1 R3 (rule packs, `docs/PRD.md:1207`) becomes v0.2 R4 (`:1520`); v0.1 R4 (components/nonlinear, `docs/PRD.md:1223`) becomes v0.2 R5 (`:1537`); the v0.2 R3 insertion shifts everything after R2 by one. Milestone tokens in ruled history (`DEC-048`, `DEC-053`, `DEC-054`) speak v0.1 numbering and are immutable history — adoption breaks stage-token traceability as well as FR traceability (read old records against this packet's Annex A and §2, never rewrite them). |
| v0.1 R5 content delta | v0.1 R5 Engineering Beta deliverables (`docs/PRD.md:1239-1253`) include validation manual, full report package, IP contribution process, public issue templates, private-data redaction workflow, and signed releases. v0.2 R6 (`:1554-1569`) carries the validation manual and the "external reviewers can reproduce validation examples" exit, but the release-machinery items (signed releases, issue templates, redaction workflow, IP contribution process) appear in v0.2 prose (§19–§21) rather than any v0.2 §24 milestone list. Adoption must not silently drop them (§4 consequences). |
| R6/R7 carry no roadmap weight today | The governing PRD contains no R6/R7 (`docs/PRD.md:1158-1253`); the Extended Horizon "Phases G/H/I are entirely gated on `D-21`" (`plans/PLAN_2026-06-17_prd_completion.md:277`; scale estimates `:279` are conditional "if `D-21` adopted"). |
| FR crosswalk mandate | "Any plan revision adopting `D-21` must add the full crosswalk" (`plans/PLAN_2026-06-17_prd_completion.md:263`); the same note records that the registers already trace to the v0.2 FRs, that FR-023 becomes the narrow ancestor of the FR-HAND-* family, that FR-024 is de-emphasized in v0.2, and that FR-025 survives as the distinct local-FEA sub-model export (DEL-10-03). |
| Live-binding gate | `D-21` is one of the x4 live-binding gates: "Live binding (L2-L3) gated x4: tier-0 adoption, app-dev F3, piping D-21, DEC-041 automation condition" (`_DomainEngines/profiles/open_pipe_stress.yaml:143`, ADOPTED profile, read-only cite). Ruling `D-21` — in any direction — moves exactly one of the four gates. |
| Embedded-agent substrate | `DEC-041` (`execution/_Decomposition/SOFTWARE_DECOMP.md:611`) rules harness-as-versioned-packages with execution "gated behind `D-21` AND the automation condition"; `DEC-055` (`:625`) binds the event-vocabulary count to the live app-dev source for any future pin. Neither is reopened by this packet. |
| SCA workflow separate | The `SCOPE_CHANGE` execution workflow is a separate, later act: "SCOPE_CHANGE does not write production deliverable content, code, schemas, tests, lifecycle promotion beyond PREPARATION `OPEN`, dependency-extract registers, commits, release claims, or professional claims" (`execution/_ScopeChange/SCA-004_2026-05-18_0000/Propagation_Plan.md:28`). This packet does not run it. |

## 3. Constraints: What Adoption Must NOT Weaken

### 3.1 Professional boundary and agent gating

The v0.2 PRD keeps — and any adoption ruling inherits — the professional
boundary as a hard invariant:

- Prohibited claims (v0.2 §21.2, `docs/_ScopeChange/OpenPipeStress_PRD_v0.2.md:1302-1314`):
  the product must not claim or imply that it certifies, seals, approves, or
  authenticates engineering work, declares code compliance for professional
  reliance, or replaces the engineer of record.
- The FR-AGENT family is human-gated by construction: agent output is proposed
  operations, not direct mutations (`FR-AGENT-001`, `:514`); proposals must pass
  schema/constraint validation (`FR-AGENT-002`, `:515`); changes are shown as
  diffs and require user acceptance (`FR-AGENT-003`, `:516`); accepted
  operations record rationale and assumptions (`FR-AGENT-004`, `:517`); and —
  the only **Must** in the family — "Agent output cannot claim engineering
  acceptance, certification, or code compliance for reliance"
  (`FR-AGENT-005`, `:518`).
- The ADOPTED tier-0 profile mirrors the same ceiling: the
  `professional_boundary.agent_must_not_claim` list and the reserved human act
  `HUMAN_APPROVED_FOR_PROJECT` (external, SHA-bound, K-AUTH-2)
  (`_DomainEngines/profiles/open_pipe_stress.yaml:127-137`).

### 3.2 Adoption is not live binding

Ruling `D-21` in favor of v0.2 clears exactly one of the four live-binding
gates (`_DomainEngines/profiles/open_pipe_stress.yaml:143`). App-dev F3, the
`DEC-041` automation condition ("must be consumable as a highly-automated
package pull", `execution/_Decomposition/SOFTWARE_DECOMP.md:611`), and the
tier-0 sequence remain independent. No live agent binding, package consumption,
or R7 execution follows from adoption alone.

### 3.3 Immutable ruled history

Ruled `DEC` entries and prior packets speak v0.1 FR and milestone numbering and
are never edited in place (the `D-28` packet §3 records this convention and its
`DEC-029`/`DEC-035` supersession precedent,
`D-28_event_vocabulary_count_reconciliation.md:49-60`). Adoption re-keys the
*forward* traceability basis via Annex A; it rewrites nothing.

### 3.4 The SCA workflow stays separate and later

Adoption authorizes — but is not — propagation. Conforming edits (the PRD swap
or delta, the plan revision carrying the crosswalk, register/coordination
re-keying) execute under the governed `SCOPE_CHANGE` workflow with its own
non-write boundary (`execution/_ScopeChange/SCA-004_2026-05-18_0000/Propagation_Plan.md:28`),
after and only after the §7 ruling.

### 3.5 Standing engine invariants

The status vocabulary (no compliance/reliance tokens), the code-neutral
boundary (no protected standards data; all component factors user-entered), and
the no-silent-defaults rule carry through every phase regardless of numbering
(`plans/PLAN_2026-06-17_prd_completion.md:285-287`). The FR renumber changes
identifiers, not these invariants.

## 4. Options

| ID | Option | Consequence |
|---|---|---|
| **O-A** | **Adopt the full v0.2 milestone set** (R6 + R7 + inserted v0.2 R3; full FR renumber with Annex A as the governing crosswalk). | `docs/PRD.md` v0.1 is superseded per the v0.2 supersession clause (`OpenPipeStress_PRD_v0.2.md:32-38`) through the SCA workflow (§3.4). Phases G/H/I become plannable ordinary-gated work (plan `:277`; scale `:279`). Traceability re-keys forward via Annex A; ruled history stays immutable (§3.3). R7 execution remains gated by the remaining x3 live-binding gates (§3.2). The ruling should name a disposition for the v0.1 R5 release-machinery deliverables that have no v0.2 §24 milestone home (§2, "v0.1 R5 content delta") — e.g. carry them as explicit R6-entry residuals. |
| **O-B** | **Decline — terminate the PRD arc at R5.** | v0.1 stays governing; the PRD arc completes at R5 Engineering Beta (`docs/PRD.md:1239-1253`). R6/R7 content remains non-authoritative under `docs/_ScopeChange/`. The piping `D-21` live-binding gate can never clear, so embedded-agent live binding stays permanently unlawful under the current gate structure (`open_pipe_stress.yaml:143`) unless the owner separately re-rules the gates; the `DEC-041` substrate ruling and the `DEC-042`-sanctioned prep remain valid but stranded short of execution. No crosswalk is adopted; flat FR numbering persists. |
| **O-C** | **Adopt a delta** — e.g. the inserted v0.2 R3 + R6 (with the FR renumber for the non-agent families), defer R7 and the FR-AGENT family. | States/Comparison and Design-Knowledge/Handoff scope become plannable; the agent horizon stays unadopted. Costs: the v0.2 document is written as a whole replacement (`:7`), so a delta requires authoring a new scope-change artifact (partial-adoption text + partial crosswalk) — more SCA work, not less; a second traceability break occurs if/when R7 is adopted later; the x4 live-binding gate structure still shows `D-21` unresolved-in-part, which the owner would need to re-word in the profile's open-issues row (`open_pipe_stress.yaml:143`). |

## 5. Recommended Disposition (PROPOSAL)

Recommend **O-A** — adopt the full v0.2 milestone set with Annex A as the
mandatory crosswalk — **non-binding; the recommendation confers nothing**.

Rationale:

1. **The timing condition is met.** The 2026-06-17 horizon guidance held this
   packet until R4/R5 were near exit (`PLAN_2026-06-17_prd_completion.md:100`);
   `DEC-054` cleared R4 and made R5 current (`SOFTWARE_DECOMP.md:624`,
   `docs/PLAN.md:74`), and the register's prerequisite (`D-27`) is RULED
   (`_REGISTER.md:51`).
2. **The tree already leans v0.2.** Execution registers trace to the v0.2 FRs
   and SOW rows (`PLAN_2026-06-17_prd_completion.md:263`); the States/
   Comparison, handoff, and agent seams exist in schemas and code as
   harness-independent prep (`DEC-042`, `SOFTWARE_DECOMP.md:612`). Declining
   (O-B) would leave the working traceability basis of the registers pointing
   at a permanently non-authoritative document.
3. **Adoption is cheap in risk terms.** It moves one gate of four (§3.2),
   weakens no boundary (§3), and executes through the separate SCA workflow
   (§3.4) — every consequential edit remains individually governed.
4. **O-C buys little.** The deferred-R7 delta still requires the
   traceability-breaking renumber and a bespoke partial scope-change artifact,
   then a second break later; the agent family it defers is already inert
   without the other three live-binding gates.

If O-A is selected, the recommendation includes: name the v0.1 R5
release-machinery deliverables (signed releases, issue templates, redaction
workflow, IP contribution process; `docs/PRD.md:1239-1253`) as explicit carried
residuals so the v0.2 §24 milestone list does not silently orphan them.

## 6. Consequence For Readers

While §7 is unruled: `docs/PRD.md` (v0.1) remains the sole governing PRD and
the flat `FR-001..FR-025` numbering remains the governing FR identity;
`docs/_ScopeChange/OpenPipeStress_PRD_v0.2.md` remains a scope *source*, not
authority; R6/R7 carry no roadmap weight and Phases G/H/I select no work
(`PLAN_2026-06-17_prd_completion.md:277`); Annex A is a *proposed* crosswalk
with no traceability force. Agents must not cite v0.2 FR identifiers as
governing requirements. The register row reads `AWAITING_RULING (adoption
held)` — packet drafted, adoption held with the owner. Nothing about the
embedded-agent gates changes: live binding remains gated x4
(`open_pipe_stress.yaml:143`).

## 7. Human Ruling And Disposition

**Ruling recorded:** OPEN — no ruling is recorded. Adoption is held with the
owner; preparation of this packet does not advance it.

- **Ruling (option or direct):** ____________________
- **Ruled by:** ____________________
- **Date:** ____________________
- **Conditions / riders:** ____________________

## 8. Ruling And Recording Mechanism

Per existing practice (the `D-28` §8 pattern,
`D-28_event_vocabulary_count_reconciliation.md:102-109`), the human project
authority selects an option or rules directly. On ruling:

1. The ruling is appended to `execution/_Decomposition/SOFTWARE_DECOMP.md` §12
   as the next free `DEC` entry citing this packet — **`DEC-056`** as of
   2026-07-02 (the live table ends at `DEC-055`, `SOFTWARE_DECOMP.md:625`;
   re-check the live file at recording time).
2. The register row `D-21` (`_REGISTER.md:45`) moves from
   `AWAITING_RULING (adoption held)` to `RULED` with the decision pointer.
3. Only then may the separate governed `SCOPE_CHANGE` workflow execute
   propagation (PRD supersession or delta, plan revision carrying Annex A,
   coordination/stage-token re-keying), under its own non-write boundary
   (`execution/_ScopeChange/SCA-004_2026-05-18_0000/Propagation_Plan.md:28`)
   and the `SCA-` id space (`DEC-040` D-CDR-4, `SOFTWARE_DECOMP.md:610`).

No PRD, plan, coordination, decomposition, or profile file is edited under this
packet. This packet writes exactly itself plus the register-row state update.

---

## Annex A. Mandatory FR Crosswalk (v0.1 flat FRs → v0.2 namespaced families)

This is the traceability-breaking heart of the decision
(`_REGISTER.md:45`; `PLAN_2026-06-17_prd_completion.md:263`). Every flat v0.1
FR (`docs/PRD.md:334-358`) is mapped to its v0.2 successor(s)
(`docs/_ScopeChange/OpenPipeStress_PRD_v0.2.md`, §11 tables `:416-528`), with
splits, merges, narrowings, and unmapped rows labeled explicitly. Where no
clean successor exists the row says so rather than forcing a mapping. Adopted
only if §7 rules for adoption; until then this table is proposal-grade.

**Mapping classes:** DIRECT (one clean successor) · SPLIT (multiple
successors) · MERGE (shares a successor with another v0.1 FR) · NARROWED
(successor covers less at FR granularity; remainder in v0.2 prose) · UNMAPPED
(no v0.2 §11 FR row; disposition stated).

| v0.1 FR (docs/PRD.md) | v0.2 successor(s) (OpenPipeStress_PRD_v0.2.md) | Class | Notes |
|---|---|---|---|
| FR-001 create/open/save/version projects (`:334`) | FR-MOD-001 canonical project schema (`:422`); FR-CMP-001 named immutable model states (`:488`) | SPLIT | Lossless round-trip → FR-MOD-001; the "version" verb is realized by named immutable states/runs (v0.2 §5.7 `:146-148`). |
| FR-002 explicit unit systems (`:335`) | FR-MOD-001 units preserved in serialization (`:422`); FR-RULE-002 unit-aware evaluator rejects incompatible units (`:477`) | SPLIT / NARROWED | v0.2 §11 has no single dedicated units FR; the system-wide rule lives in design principle §5.6 Unit Safety (`:142-144`) and the MVP "unit system" item (`:1411`). |
| FR-003 3D node/element modeling (`:336`) | FR-MOD-002 schema-backed physical model (`:423`); FR-GUI-004 direct model editing (`:447`); FR-SOL-003 straight pipe elements (`:458`); FR-SOL-004 rigid elements (`:459`) | SPLIT | v0.1 bundled schema representation, GUI editing, and element behavior; v0.2 separates them by family. |
| FR-004 six DOF per node (`:337`) | FR-SOL-001 (`:456`) | DIRECT | — |
| FR-005 pipe section property calculations (`:338`) | — no v0.2 §11 FR row | UNMAPPED | Substance survives in prose: analytical model includes section properties (§8.3 `:240`), pipe section library records (§18.2 `:1130-1143`), and the MVP "material and section entry" item (`:1425`). No forced mapping; if adopted, the SCA plan revision should note FR-005 as prose-carried. |
| FR-006 user-defined materials (`:339`) | FR-RULE-005 private material libraries (`:480`); provenance slots via FR-MOD-005 (`:426`) | MERGE | Merges with FR-022 into FR-RULE-005 (materials half). |
| FR-007 basic load cases (`:340`) | FR-SOL-006 (`:461`) | DIRECT | — |
| FR-008 linear static solve + benchmarks (`:341`) | FR-SOL-002 (`:457`) | DIRECT | — |
| FR-009 recover element forces/moments (`:342`) | FR-SOL-007 (`:462`) | DIRECT | — |
| FR-010 recover fundamental stresses (`:343`) | FR-SOL-008 (`:463`) | DIRECT | — |
| FR-011 rule-pack schema (`:344`) | FR-RULE-001 (`:476`) | DIRECT | — |
| FR-012 block incomplete code checks (`:345`) | FR-RULE-003 (`:478`) | DIRECT | — |
| FR-013 graphical 3D modeler (`:346`) | FR-GUI-001 3D viewport (`:444`); FR-GUI-004 direct editing (`:447`) | SPLIT | Support/load authoring substance flows model-side to FR-SOL-005 (`:460`)/FR-SOL-006 (`:461`). |
| FR-014 model tree + property editor (`:347`) | FR-GUI-002 model tree (`:445`); FR-GUI-003 property inspector (`:446`) | SPLIT | — |
| FR-015 results visualization (`:348`) | FR-GUI-001 result overlays/deformed shapes (`:444`) | NARROWED | Result tables / governing-ratio views have no dedicated v0.2 §11 FR; carried by §14.1 "results browser" (`:788-806`). |
| FR-016 calculation reports (`:349`) | FR-REP-001 design/analysis report (`:524`); FR-REP-003 provenance summary (`:526`) | SPLIT | — |
| FR-017 bend objects (`:350`) | FR-SOL-010 (`:465`) | DIRECT | — |
| FR-018 branch connection objects (`:351`) | FR-SOL-011 (`:466`) | DIRECT | — |
| FR-019 valves/flanges/reducers (`:352`) | FR-SOL-012 (`:467`) | DIRECT | — |
| FR-020 expansion joints (`:353`) | FR-SOL-013 (`:468`) | DIRECT | — |
| FR-021 nonlinear restraints (`:354`) | FR-SOL-014 (`:469`) | DIRECT | — |
| FR-022 private material/component libraries (`:355`) | FR-RULE-005 materials (`:480`); FR-RULE-006 components (`:481`) | SPLIT / MERGE | Merges with FR-006 on FR-RULE-005. |
| FR-023 import/export open formats (`:356`) | FR-HAND-001..005 handoff family (`:504-508`); FR-CMP-010 comparison export (`:497`) | SPLIT | Per `PLAN_2026-06-17_prd_completion.md:263`, FR-023 is "the narrow ancestor of the FR-HAND-* handoff family". |
| FR-024 dynamic analysis modules (`:357`) | — no v0.2 §11 FR row | UNMAPPED (de-scoped) | v0.2 de-emphasizes dynamics: "Later modules may add … dynamics" (`:542`); MVP excludes "advanced nonlinear/dynamic modules unless otherwise prioritized" (`:1444`); not on any v0.2 milestone (`:1448-1585`). Plan `:263` concurs ("de-emphasized in v0.2, not on the milestone path"). The v0.1 `D-12` disposition row (`_REGISTER.md:30`) remains the vehicle for explicit deferral. |
| FR-025 local FEA export (`:358`) | — no v0.2 §11 FR row | UNMAPPED (survives narrowly) | Survives as v0.2 secondary goal 6, "local analysis handoff to external FEA tools" (`:186`), and per plan `:263` "as the distinct local-FEA sub-model export (DEL-10-03)". Distinct from the FR-HAND-* handoff package; `D-12` also dispositions this row. |

### A.2 New v0.2 FRs with no flat v0.1 ancestor (labeled NEW)

| v0.2 family | NEW members | Substance |
|---|---|---|
| FR-MOD (`:422-428`) | FR-MOD-003 stable IDs (`:424`), FR-MOD-004 names/tags/notes (`:425`), FR-MOD-005 provenance fields (`:426`), FR-MOD-006 unresolved assumptions (`:427`), FR-MOD-007 physical→analytical transformation (`:428`) | Model-identity and auditability layer; no flat-FR ancestor (v0.1 carried fragments in prose only). |
| FR-KNOW (`:434-438`) | Entire family FR-KNOW-001..005 | Design knowledge/constraints/candidate generation — the R6 substance (`:1554`). |
| FR-GUI (`:444-450`) | FR-GUI-005 undo/redo (`:448`), FR-GUI-006 validation-message classes (`:449`), FR-GUI-007 comparison views (`:450`) | — |
| FR-SOL (`:456-470`) | FR-SOL-005 linear supports (`:460`), FR-SOL-009 load-case algebra (`:464`), FR-SOL-015 numerical diagnostics (`:470`) | FR-SOL-009 promotes the v0.1 post-MVP roadmap item "Load-case algebra and range cases" (`docs/PRD.md:321`) to FR rank. |
| FR-CMP (`:488-498`) | Entire family FR-CMP-001..011 (except the FR-001 "version" thread and FR-023's export thread noted above) | The inserted v0.2 R3 States/Runs/Comparison substance (`:1503`). |
| FR-HAND (`:504-508`) | FR-HAND-002 manifest, FR-HAND-003 target mapping, FR-HAND-004 unsupported-target flags, FR-HAND-005 future exchange formats (`:505-508`) | Beyond FR-023's narrow ancestry. |
| FR-AGENT (`:514-518`) | Entire family FR-AGENT-001..005 | The R7 substance (`:1571`); human-gated per §3.1. |
| FR-REP (`:524-528`) | FR-REP-002 professional-boundary notice (`:525`), FR-REP-004 comparison sections (`:527`), FR-REP-005 no protected content in public templates (`:528`) | FR-REP-002/005 promote v0.1 prose boundary requirements (v0.1 §14/§21 equivalents) to FR rank. |

### A.3 Dropped at FR rank

Only FR-024 (dynamics) is dropped from the FR set without a named narrow
successor (A.1 row FR-024). FR-005 and FR-025 are UNMAPPED at FR rank but
survive in prose / secondary goals / deliverable guidance as their rows state.
No other v0.1 FR loses its substance under v0.2.
