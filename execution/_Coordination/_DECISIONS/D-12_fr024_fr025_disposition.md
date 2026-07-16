# D-12 - Disposition Of FR-024 (Dynamics) And FR-025 (Local FEA Export)

**Date prepared:** 2026-07-04
**Prepared by:** bridge work loop agent at owner direction (Ryan Tufts,
2026-07-04) preparing all eight open register rows.
**Epistemic status:** PROPOSAL (non-governing). Only the human project
authority rules. Nothing here changes lifecycle state, issues deliverables,
creates release readiness, professional approval, certification, sealing,
authentication, code-compliance acceptance, or asserts that any PRD milestone
is met until the human records the ruling.

---

## 1. Decision Statement And Scope

Decide the disposition of the two v0.1 functional requirements that the final
PRD-completeness claim cannot honestly close over while they sit undecided:

- **FR-024 (v0.1) — dynamic analysis modules** ("Support dynamic analysis
  modules | Could | Modal and response-spectrum modules pass validation
  benchmarks", `docs/PRD.md:376`), and
- **FR-025 (v0.1) — local FEA export** ("Support local FEA export | Could |
  Selected local geometry and loads can be exported for external shell/solid
  analysis", `docs/PRD.md:377`).

For each FR independently, the decision is: **implement post-beta** (with a
named placement in the milestone map) or **record an explicit deferral**
(with wording that keeps the final PRD-completeness claim honest). The
decision origin is the completion plan's decision register: "D-12 |
Disposition of FR-024 (dynamics) and FR-025 (local FEA export) — implement
post-beta or record explicit deferral | Final PRD-completeness claim | At the
R5 gate" (`plans/PLAN_2026-06-17_prd_completion.md:93`), reinforced at the
Phase E exit description: "at the R5 gate `D-12` dispositions FR-024
(dynamics) and FR-025 (local FEA export) — implement post-beta or record an
explicit deferral (**silence is not a disposition**)"
(`plans/PLAN_2026-06-17_prd_completion.md:208`). The register row is
`execution/_Coordination/_DECISIONS/_REGISTER.md:39`, and the recommended
preparation order places D-12 "at the R5 gate" (`_REGISTER.md:69`). The
current target stage is PRD R5 (v0.1 token; advanced 2026-06-23 by `DEC-054`,
`execution/_Coordination/_COORDINATION.md:262-270`), so this gate's timing is
now live.

This is a disposition record only. It does not implement anything, does not
edit the PRD (v0.1 or v0.2), does not advance any stage, does not touch the
D-21/DEC-056 ruling or the accepted SCA-005 propagation, and creates no
lifecycle, release-readiness, professional approval, certification, sealing,
authentication, or code-compliance claim. Whichever option is ruled, the
follow-on plan/decomposition edits execute as ordinary governed work citing
the ruling, not under this packet.

## 2. Verified Facts (Checked Cold, 2026-07-04)

| Check | Result |
|---|---|
| Register row (historical identifiers) | `_REGISTER.md:39`: "D-12 \| Disposition of FR-024 (dynamics) and FR-025 (local FEA export): implement post-beta or record explicit deferral \| Final PRD-completeness claim \| NOT_PREPARED". The row speaks the flat v0.1 FR tokens. |
| Decision origin | `plans/PLAN_2026-06-17_prd_completion.md:93` (§2.2 open-decisions table, timing "At the R5 gate") and `:208` (Phase E exit: "silence is not a disposition"). Plan `:216` (Phase F hygiene) requires any `D-12` deferral to get "a recorded SCA/DEC entry, not silent drift". |
| v0.1 FR text (historical) | FR-024 at `docs/PRD.md:376`; FR-025 at `docs/PRD.md:377`; both priority **Could**. The v0.1 post-MVP roadmap lists both: "Dynamic modules: modal, response spectrum, harmonic, and time-history support." (`docs/PRD.md:341`) and "Local FEA export workflows." (`docs/PRD.md:342`). v0.1 secondary goal: "Provide optional local-analysis handoff to external FEA tools for nozzles, trunnions, lugs, branch details, or special components." (`docs/PRD.md:109`). |
| v0.1 PRD status today | `docs/PRD.md:5-8`: "Document version: 0.1 Draft, preserved as historical text"; "Status: Historical v0.1 product definition; forward authority delegated by SCA-005". Forward Authority Note at `docs/PRD.md:12-27` points forward work to `docs/_ScopeChange/OpenPipeStress_PRD_v0.2.md` and to D-21 Annex A for FR traceability. |
| v0.2 adoption + propagation | D-21 ruled 2026-07-02 as `DEC-056` (`execution/_Decomposition/SOFTWARE_DECOMP.md:634`; register `_REGISTER.md:54`). SCA-005 propagation accepted 2026-07-04 by the owner (`execution/_ScopeChange/SCA-005_2026-07-04_0000/ACCEPTANCE_RECORD.md`, `accepted: 2026-07-04`, `accepted_by: Ryan Tufts`); the FR-vocabulary supersession row is `SCA-005_2026-07-04_0000/Supersession_Map.csv:3` ("v0.2 namespaced FR families are the forward vocabulary; D-21 Annex A governs flat-to-namespaced traceability … Historical records keep their original v0.1 tokens."). |
| Current numbering: FR-024 | **UNMAPPED (de-scoped)** — no v0.2 §11 FR row exists for dynamics (v0.2 §11 tables at `OpenPipeStress_PRD_v0.2.md:416-528` contain no dynamics FR; checked cold by search). D-21 Annex A row: "FR-024 dynamic analysis modules … — no v0.2 §11 FR row \| UNMAPPED (de-scoped)" (`D-21_prd_scope_change_v0_2_milestone_set.md:261`). Annex A.3: "Only FR-024 (dynamics) is dropped from the FR set without a named narrow successor" (`:279-280`). |
| Current numbering: FR-025 | **UNMAPPED (survives narrowly)** — no v0.2 §11 FR row, but the substance survives as v0.2 secondary goal 6: "Support local analysis handoff to external FEA tools for nozzles, lugs, trunnions, branches, or other local details." (`OpenPipeStress_PRD_v0.2.md:186`) and as deliverable guidance DEL-10-03. D-21 Annex A row at `D-21_prd_scope_change_v0_2_milestone_set.md:262`; A.3 at `:280-281`. Distinct from the FR-HAND-* handoff family (same row, "Distinct from the FR-HAND-* handoff package"). |
| v0.2 dynamics posture | "Later modules may add geometric nonlinearity, dynamics, or specialized component behavior." (`OpenPipeStress_PRD_v0.2.md:542`); MVP exclusion "advanced nonlinear/dynamic modules unless otherwise prioritized" (`:1444`); no dynamics deliverable on any v0.2 milestone R0-R7 (`:1450-1585`, checked cold). |
| v0.2 milestone coverage of local FEA export | None. The R6 "handoff package generator" (`:1561`) is the FR-HAND downstream-modeling package, which Annex A distinguishes from the local-FEA sub-model export (`D-21…md:262`). No v0.2 milestone names a local FEA export deliverable (`:1450-1585`). |
| Plan crosswalk note | `plans/PLAN_2026-06-17_prd_completion.md:280-286`: "FR-024 (dynamics) is de-emphasized in v0.2 (not on the milestone path); FR-025 survives as the distinct local-FEA sub-model export (DEL-10-03)." |
| Implementation reality: FR-024 | **Greenfield — nothing exists.** Plan FR map: "FR-024 dynamic analysis modules \| Could \| Not implemented \| `D-12` disposition at R5 gate" (`plans/PLAN_2026-06-17_prd_completion.md:277`). No decomposition deliverable covers dynamics; the only decomposition mention is the SOW-013 note "Dynamic modules may be deferred." (`execution/_Decomposition/SOFTWARE_DECOMP.md:111`). Code search (core/apps/api/schemas/tools) for modal / response-spectrum / time-history / eigen surfaces found no dynamics module; the sole eigenvalue routine is `jacobi_symmetric_eigenvalues` in `core/solver/performance_harness/src/lib.rs:980`, a static condition-number evidence helper, not dynamics. |
| Implementation reality: FR-025 | **Contract-stage carrier exists; export itself not implemented.** Decomposition row: "DEL-10-03 \| Local FEA handoff data contract \| API_CONTRACT \| SOW-031,SOW-049 \| … Guidance only, no external FEA implementation." (`SOFTWARE_DECOMP.md:343`, under PKG-10 `:215`; SOW-031 at `:129`). Deliverable folder exists with a populated document kit; `_STATUS.md` current state **IN_PROGRESS** (last updated 2026-07-02) at `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-03_Local FEA handoff data contract/_STATUS.md:3-4`. Plan FR map: "the distinct local FEA submodel export required by FR-025 is not implemented (separate from the Phase H handoff package; DEL-10-03 guidance only)" (`plans/PLAN_2026-06-17_prd_completion.md:278`). |
| Forward-horizon placement surface | Plan §3 Forward Horizon (`plans/PLAN_2026-06-17_prd_completion.md:218-236`): Phase G = inserted v0.2 R3 (states/comparison), Phase H = v0.2 R6 (design knowledge and handoff beta, PKG-13/15/17), Phase I = v0.2 R7 (agent-assisted design). Rough scale Phase H ~10-16 tranches (`:305`). |

### 2.1 Additional verified detail: FR-024 (dynamics) is greenfield

The greenfield finding in the table above was checked at three layers, since
the disposition options depend on it:

- **PRD layer.** In v0.1, dynamics is a Could-priority FR (`docs/PRD.md:376`)
  and a post-MVP roadmap bullet (`:341`); it appears on none of the v0.1 §22
  release milestones R0-R5. In the adopted v0.2, dynamics has no FR row at
  all (§11, `OpenPipeStress_PRD_v0.2.md:416-528`), is an explicit MVP
  exclusion ("advanced nonlinear/dynamic modules unless otherwise
  prioritized", `:1444`), appears on no milestone R0-R7 (`:1450-1585`), and
  survives only as a solver-formulation aside ("Later modules may add
  geometric nonlinearity, dynamics, or specialized component behavior.",
  `:542`).
- **Decomposition layer.** No PKG or DEL row in
  `execution/_Decomposition/SOFTWARE_DECOMP.md` covers dynamics. The single
  mention is the SOW-013 scope note "Dynamic modules may be deferred."
  (`SOFTWARE_DECOMP.md:111`), which anticipated exactly this decision.
- **Code layer.** A search of `core/`, `apps/`, `api/`, `schemas/`, and
  `tools/` for modal, response-spectrum, harmonic, time-history, and
  eigen-solver surfaces found no dynamics module. The only eigenvalue code
  in the solver tree is `jacobi_symmetric_eigenvalues`
  (`core/solver/performance_harness/src/lib.rs:980`), used to compute a true
  condition number for the static sparse-promotion evidence packet
  (`DEC-053` lineage) — static conditioning diagnostics, not dynamics.

An "implement" ruling for FR-024 would therefore be a from-zero numerical
domain (mass matrices, eigen-extraction, spectral combination, benchmark
fixtures) with no existing deliverable to own it.

### 2.2 Additional verified detail: FR-025 has a contract-stage carrier only

- **PRD layer.** In v0.1, local FEA export is a Could-priority FR
  (`docs/PRD.md:377`), a post-MVP roadmap bullet (`:342`), and a secondary
  goal (`:109`). In the adopted v0.2 it survives as secondary goal 6
  (`OpenPipeStress_PRD_v0.2.md:186`) but has no FR row and no milestone
  deliverable. The v0.2 §16 handoff workflow and the `FR-HAND-001..005`
  family (`:504-508`) concern the downstream-modeling handoff package —
  Annex A explicitly separates that family (descended from v0.1 FR-023)
  from the local-FEA sub-model export
  (`D-21_prd_scope_change_v0_2_milestone_set.md:262`).
- **Decomposition layer.** DEL-10-03 "Local FEA handoff data contract"
  exists under PKG-10 as an `API_CONTRACT` deliverable, scope-noted
  "Guidance only, no external FEA implementation."
  (`SOFTWARE_DECOMP.md:343`), tracing to SOW-031 ("The product shall support
  optional handoff of selected local-detail problems to external shell/solid
  FEA workflows.", `:129`).
- **Deliverable state.** The DEL-10-03 kit is populated and its `_STATUS.md`
  reads **IN_PROGRESS** (last updated 2026-07-02, after a human K-CONFLICT-1
  lifecycle correction); its history records exchange format, adapter, mesh,
  invocation, and runtime integration as still open
  (`execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-03_Local FEA handoff data contract/_STATUS.md`).
- **Plan layer.** The FR map is explicit that earlier summaries overstated
  this row: "app export/review and handoff panels exist, but the distinct
  local FEA submodel export required by FR-025 is not implemented (separate
  from the Phase H handoff package; DEL-10-03 guidance only)"
  (`plans/PLAN_2026-06-17_prd_completion.md:278`).

An "implement" ruling for FR-025 is therefore not greenfield in the FR-024
sense: the contract authority exists and is in work, but the export
implementation itself does not exist anywhere.

## 3. Identifier Anchoring After The v0.2 Renumber

Per the register's numbering caveat, the identifiers were checked cold
against the post-SCA-005 state rather than assumed:

- The register row and this packet's title speak the **historical v0.1
  tokens** `FR-024` / `FR-025`, which remain valid for ruled history and for
  this row per the SCA-005 supersession note ("Historical records keep their
  original v0.1 tokens", `Supersession_Map.csv:3`).
- In the **current forward authority**
  (`docs/_ScopeChange/OpenPipeStress_PRD_v0.2.md`, promoted by SCA-005 on
  2026-07-04), **neither FR exists as a numbered requirement**. There is no
  renumbered successor token to cite: D-21 Annex A classifies FR-024 as
  UNMAPPED (de-scoped) and FR-025 as UNMAPPED (survives narrowly)
  (`D-21_prd_scope_change_v0_2_milestone_set.md:261-262`). The nearby v0.2
  `FR-HAND-001..005` family (`OpenPipeStress_PRD_v0.2.md:504-508`) is the
  downstream-modeling handoff package descended from v0.1 FR-023 — it is
  **not** a successor to FR-025 and must not be conflated with it (Annex A
  `:262`; plan `:278`).
- Annex A itself notes that "The v0.1 `D-12` disposition row … remains the
  vehicle for explicit deferral" for FR-024, and "`D-12` also dispositions"
  the FR-025 row (`:261-262`). This packet is that vehicle.

Consequence: this decision cannot be discharged by pointing at any v0.2 FR
row. It is a disposition of two v0.1 identifiers whose substance is,
respectively, dropped (FR-024) and carried in prose/deliverable-guidance form
(FR-025) under the adopted v0.2 authority.

## 4. Relationship To Ruled History And Open Gates

Nothing in this packet touches ruled history, and no option requires editing
it:

- **`DEC-056` (D-21) stays as ruled.** The v0.2 adoption, its Annex A
  crosswalk, and its release-machinery rider (v0.1 R5 release-machinery
  deliverables carried as explicit R6-entry residuals,
  `SOFTWARE_DECOMP.md:634`) are immutable. Options O-A and O-B operate
  entirely inside the posture that ruling adopted; only O-C would require a
  *new* scope-change artifact (it re-scopes dynamics that DEC-056's adopted
  document de-scoped), and even then DEC-056 itself would remain unedited —
  supersession, never rewriting, per the D-28/`DEC-055` precedent.
- **SCA-005 stays as accepted.** The 2026-07-04 acceptance
  (`execution/_ScopeChange/SCA-005_2026-07-04_0000/ACCEPTANCE_RECORD.md`)
  propagated forward-authority pointers only; this packet cites the
  propagated surfaces (`docs/PRD.md` Forward Authority Note, the plan
  crosswalk note) exactly as they now stand.
- **Ruled milestone tokens are read, not rewritten.** Stage tokens in this
  packet ("R5 gate", current target stage PRD R5) are the v0.1 tokens the
  register, plan, and `_COORDINATION.md:262` actually use; the v0.2
  renumber's milestone shift (v0.1 R3 → v0.2 R4, v0.1 R4 → v0.2 R5, per the
  D-21 packet's verified-facts row) does not alter what those historical
  surfaces say.
- **The residual-row convention applies forward.** Under the owner-ruled
  2026-07-03 convention (`_REGISTER.md:17-24`), once D-12 is ruled the row
  is immutable; any later change of posture (e.g. reviving dynamics, or
  re-deferring the FR-025 Phase H item) becomes a **new** `D-XX` row citing
  the D-12 ruling as its basis, never a reopening of this one.

## 5. What A Disposition Must Achieve

The blocked item is the **final PRD-completeness claim** (`_REGISTER.md:39`).
For that claim to be honest:

1. Every v0.1 FR must be either implemented, or carry a recorded human
   disposition. FR-024 and FR-025 are the only two FR-map rows whose
   disposition column still points at an unruled `D-12`
   (`plans/PLAN_2026-06-17_prd_completion.md:277-278`).
2. A deferral must be **explicit and recorded** — plan `:208` ("silence is
   not a disposition") and plan `:216` (a `D-12` deferral "gets a recorded
   SCA/DEC entry, not silent drift").
3. The record must not overstate: both FRs are priority **Could**
   (`docs/PRD.md:376-377`), so neither blocks any Must/Should completeness
   reading; what they block is the *unqualified* form of the claim.
4. The record must survive the vocabulary change: the disposition is written
   against the v0.1 tokens (historical) with the Annex A anchoring in §3, so
   future readers of either PRD generation can trace it.

## 6. Options

Options may treat the two FRs differently; O-B and O-C differ only in the
FR-024 half. "Implement post-beta" placements reference the plan §3 forward
horizon (`plans/PLAN_2026-06-17_prd_completion.md:218-236`).

| ID | Option | Consequence |
|---|---|---|
| **O-A** | **Explicit deferral of both.** FR-024: deferred and de-scoped, consistent with its UNMAPPED (de-scoped) Annex A class; any future dynamics scope re-enters only through a new governed scope change with its own decision packet. FR-025: deferred; DEL-10-03 stays a guidance-only API contract; the local-FEA sub-model export is not scheduled on any phase; the surviving v0.2 secondary-goal-6 substance remains prose. | Final PRD-completeness claim unblocks immediately with two recorded deferrals. Cheapest and fully honest. Cost: the FR-025 substance that the adopted v0.2 authority deliberately kept alive (goal 6, `:186`; DEL-10-03) is left unscheduled — a later revival needs a new decision even though a carrier deliverable already exists IN_PROGRESS. |
| **O-B** | **Split: defer FR-024; implement FR-025 post-beta.** FR-024 as in O-A. FR-025: scheduled as a named post-beta work item in the **Phase H (v0.2 R6) lane**, carried by DEL-10-03 (PKG-10) as the contract authority, explicitly distinct from the FR-HAND downstream-modeling package; the concrete implementation deliverable(s) get scoped at the Phase H lead-up decomposition revision citing this ruling. | Final PRD-completeness claim unblocks: FR-024 by recorded deferral, FR-025 by recorded post-beta placement. Matches the adopted v0.2 posture exactly (dynamics dropped; local FEA handoff kept as a goal). Cost: adds a named item to an already ~10-16-tranche Phase H; the item stays "Could"-priority and can still be re-deferred at the Phase H lead-up if the owner then rules so. |
| **O-C** | **Implement both post-beta.** FR-025 as in O-B. FR-024: dynamics re-entered as a post-R7 module family (modal + response spectrum first, per the v0.1 acceptance criterion). | Because FR-024 is UNMAPPED (de-scoped) in the adopted v0.2 FR set and appears on no v0.2 milestone (`:1450-1585`), scheduling it is itself a scope change: it requires a new SCA-grade artifact and new PKG-04/PKG-05 deliverables for a greenfield numerical domain (eigen-solvers, spectra, mass matrices) with no present demand signal in the repo. Heaviest option; contradicts the two-day-old DEC-056/SCA-005 posture without new information. |
| **O-D** | **No disposition (status quo).** | Explicitly disallowed as a *silent* end-state by plan `:208`; presented only to record its consequence: the final PRD-completeness claim stays blocked indefinitely and the register row stays open. |

## 7. Recommended Disposition (PROPOSAL)

Recommend **O-B** — **non-binding; the recommendation confers nothing**:

1. **FR-024 (dynamics): record explicit deferral**, with proposed wording for
   the ruling entry:

   > FR-024 (v0.1, dynamic analysis modules) is explicitly deferred and
   > de-scoped. No dynamics code, deliverable, or milestone exists in the
   > repository or in the adopted v0.2 PRD (D-21 Annex A: UNMAPPED,
   > de-scoped). Any future dynamics scope re-enters only through a new
   > governed scope change with its own human decision packet. The final
   > PRD-completeness claim reads FR-024 as "dispositioned by explicit
   > deferral" — never as implemented, and never silently dropped.

2. **FR-025 (local FEA export): implement post-beta**, placed in the
   **Phase H (v0.2 R6) lane** as a named work item distinct from the
   FR-HAND-* handoff package, with DEL-10-03 (`SOFTWARE_DECOMP.md:343`,
   currently IN_PROGRESS) as the contract authority; the implementing
   deliverable(s) are scoped at the Phase H lead-up decomposition revision
   citing this ruling. Priority stays "Could": if Phase H pressure demands,
   the owner may re-defer it at that lead-up without reopening this row
   (per the 2026-07-03 residual-row convention, that would be a new row).

Rationale: the two FRs are in genuinely different states and the adopted
authority already says so. Dynamics is greenfield with zero code, zero
deliverables, no v0.2 FR, and no milestone — deferral merely records the
posture the owner adopted via DEC-056 and SCA-005 accepted two days ago.
Local FEA export, by contrast, has a live IN_PROGRESS carrier deliverable, a
surviving v0.2 secondary goal, and a natural milestone home (R6/Phase H);
recording a placement rather than a deferral keeps that deliberately-kept
substance scheduled instead of orphaned, at near-zero present cost since
Phase H work has not begun.

## 8. Consequence For Readers

If O-A or O-B is ruled, the final PRD-completeness claim may thereafter say,
without qualification games: *every v0.1 functional requirement is either
implemented or carries a recorded human disposition; FR-024 and FR-025 are
dispositioned by `DEC` entry, not silently dropped.* Readers of the v0.2
authority lose nothing: neither FR exists there, and the Annex A rows for
both point at this packet's row as the disposition vehicle
(`D-21_prd_scope_change_v0_2_milestone_set.md:261-262`). Readers of ruled
v0.1-token history (plans, `DEC-048`/`DEC-053`/`DEC-054`, the register) find
the historical tokens intact and this record explaining their fate. No PRD
text changes under any option in this packet.

## 9. Unresolved Notes (K-INVENT-1)

- **Which PRD generation the "final PRD-completeness claim" is claimed
  against** is not defined by any file read for this packet. The register row
  and plan speak v0.1 tokens; forward authority is v0.2. This packet keeps
  the disposition honest under either reading (v0.1 arc: both FRs carry
  recorded dispositions; v0.2: neither FR exists, and the Annex A rows point
  here). If the owner intends the completeness claim to be v0.2-only, O-A
  and O-B are equally sufficient for it; the difference is planning posture,
  not claim honesty.
- **Exact FR-025 implementation shape** (export formats, mesh/boundary
  payload, target tools) is not asserted here: DEL-10-03's own kit records
  exchange format, adapter, mesh, invocation, and runtime integration as
  open (`_STATUS.md` history, 2026-05-11 entry). O-B deliberately defers
  that shaping to the Phase H lead-up rather than inventing it now.
- No other unverified facts are relied on; every claim above cites a file
  read cold on 2026-07-04.

## 10. Human Ruling And Disposition

**Ruling recorded (2026-07-15, Ryan Tufts, human project authority,
in-session decision slate):** Option **O-B**, approved as recommended —
FR-024 explicit deferral (de-scoped, §7 wording); FR-025 implement
post-beta in the Phase H (v0.2 R6) lane per §7. Verbatim slate ruling over
the rows presented in the order D-42, D-38, D-12, D-07b (with D-06b noted
as NOT_PREPARED):

> My rulings:
>
> O-A, O-B, O-B, O-B, we don't need to act on D-06b right now.

Mapping for this row: D-12 → O-B. Recorded by agent at owner direction
(K-AUTH-1; D-GOV-04). Codified as `DEC-078` in
`execution/_Decomposition/SOFTWARE_DECOMP.md` §12; register row D-12 →
RULED. Follow-on edits execute per §11.

## 11. Ruling Mechanism

Per existing practice, the human project authority selects an option (or
rules directly, including a per-FR mix not listed). The ruling is appended to
`execution/_Decomposition/SOFTWARE_DECOMP.md` §12 as the next `DEC` entry
citing this packet; the `D-12` register row in
`execution/_Coordination/_DECISIONS/_REGISTER.md` then moves from
`AWAITING_RULING` to `RULED` with the decision pointer. Follow-on edits (plan
FR-map rows `:277-278`, any Phase H work-item naming, any decomposition
revision) execute as ordinary governed work citing that `DEC` entry — nothing
propagates under this packet itself.
