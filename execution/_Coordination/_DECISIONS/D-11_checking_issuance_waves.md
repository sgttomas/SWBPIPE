# D-11 - Issuance Waves For The CHECKING Deliverables

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

Decide the **wave structure** under which `CHECKING`-state deliverables are
brought to the owner for `ISSUED` lifecycle closure: how many waves, which
deliverables belong to each, and what evidence bar each wave must meet before
the owner considers issuing it. This is register row `D-11` ("Issuance waves
for the `CHECKING` deliverables",
`execution/_Coordination/_DECISIONS/_REGISTER.md:38`), which blocks Phase F
governance closure and is flagged human-paced (`_REGISTER.md:68`;
`plans/PLAN_2026-06-17_prd_completion.md:92`).

Scope boundary: this packet proposes **structure only** — wave membership
criteria and per-wave evidence bars. It does not issue any deliverable, does
not advance any lifecycle state, does not set any date or schedule, and does
not bind the owner to issue any wave at any time. The owner paces issuance
entirely; a ruled wave structure defines *eligibility and evidence*, never
timing. Each actual issuance remains its own separate human lifecycle act
per existing practice.

## 2. Verified Facts (Checked Cold, 2026-07-04)

| Check | Result |
|---|---|
| Lifecycle chain | `docs/TYPES.md:242` — `OPEN → INITIALIZED → IN_PROGRESS → CHECKING → ISSUED`. `docs/TYPES.md:245`: "`ISSUED` means accepted as a development artifact by the human project authority. It does not mean professional engineering authentication of any piping calculation." |
| Current lifecycle census | Cold sweep of all 101 `execution/PKG-*/1_Working/DEL-*/_STATUS.md` `**Current State:**` headers, 2026-07-04: **`CHECKING` = 8, `IN_PROGRESS` = 92, `ISSUED` = 1**. Matches the 2026-06-21 discovery recorded at `execution/_Coordination/_COORDINATION.md:229-232`. |
| The 8 `CHECKING` deliverables | All of `PKG-00 - Software Architecture Runway` (roster in §2.1). Each `_STATUS.md:3` header reads `**Current State:** CHECKING`. |
| How PKG-00 entered `CHECKING` | Identical history entry in all eight `_STATUS.md` files (e.g. DEL-00-01 `_STATUS.md:10`): "2026-06-04 - State moved to CHECKING (human-approved corrected PKG-00 architecture-basis re-review)". See the §2.2 Unresolved note on the location of that approval record. |
| The sole `ISSUED` precedent | `DEL-01-01 Project governance baseline` — `execution/PKG-01_Governance, IP Boundary, and Professional Responsibility/1_Working/DEL-01-01_Project governance baseline/_STATUS.md:3` (`ISSUED`), `:13-14`: 2026-06-03 human Gate 5 approval accepted REVIEW recommendation `RECOMMEND_ADVANCE_TO_CHECKING` from snapshot `REV_DEL-01-01_2026-06-03_2327`; a same-day human approval then advanced `CHECKING → ISSUED` with explicit boundary language ("ISSUED is deliverable lifecycle closure only; it is not release acceptance, legal advice, professional approval, certification, sealing, authentication, or code-compliance"). |
| Review-snapshot precedent | `execution/_Reconciliation/Reviews/REV_DEL-01-01_2026-06-03_2327/` contains `Brief.md`, `QA_Report.md`, `Decision_Log.md`, `RUN_SUMMARY.md`. Sibling snapshots exist for PKG-04/PKG-05 deliverables (`REV_DEL-04-01_2026-06-05_2120`, `REV_DEL-05-01_2026-06-05_2021`, and others in the same directory), used for human-gated advances **to** `CHECKING` (Gate 5 pattern; §3 point 3). |
| Why 92 deliverables are `IN_PROGRESS`, not `CHECKING` | `execution/_Reconciliation/LifecycleCorrection/LIFECYCLE_CORRECTION_2026-07-02_2050/Decision_Log.md`: commit `28219696d` (2026-06-16) flipped 92 `_STATUS.md` headers `CHECKING → IN_PROGRESS` without history entries; the owner ruled 2026-07-02 ("Yes this is the exemplar and all shall be IN_PROGRESS") that the `IN_PROGRESS` headers are authoritative class-wide; no promotion to `CHECKING` or `ISSUED` was made or authorized. |
| Decision origin | `plans/PLAN_2026-06-17_prd_completion.md:92` (§2.2 carried decision, "Human-paced; wave alignment in §3 Phase F") and `:210-216` — Phase F "Governance closure (parallel, human-paced)"; **F1 — Issuance waves (`D-11`)**: "Prepare candidate review packets for the `CHECKING` deliverables in waves aligned to phase completions (suggested: PKG-02/03/05/06 solver-side after Phase D evidence; PKG-04 after D6/D7; PKG-07/08/13–16 app-side after C5/Phase D; PKG-09/10/12 after Phase E). Agents prepare packets; only humans change lifecycle state. `DEL-01-01` is the sole currently `ISSUED` deliverable." |
| Human-only lifecycle authority | `plans/PLAN_2026-06-17_prd_completion.md:316`: "lifecycle changes, thresholds, release claims, stage advancement, and issuance happen only at the `D-xx` decision gates and Phase F waves." `execution/_Coordination/_COORDINATION.md:242-243`: "Formal issuance remains a separate human-gated lifecycle workflow." |
| `CHECKING` semantics | `execution/_Coordination/_COORDINATION.md:238-240`: "`CHECKING` means design authority is mature; it does not mean the application has absorbed that deliverable's scope." |
| Issuance candidate selection rule | `execution/_Coordination/_COORDINATION.md:667-669` (work-loop step 5): "For human-directed lifecycle closeout, select from `CHECKING` deliverables. `ISSUED` deliverables are not work-selection scope except through a human-approved change path." |
| PKG-00 authority already flows without `ISSUED` | `execution/_Decomposition/SOFTWARE_DECOMP.md:420`: SCA-001 authorizes PKG-00 at `SEMANTIC_READY` as an architecture-basis candidate for sealed brief injection without changing lifecycle state to `ISSUED`; carried through revision 0.8 (`SOFTWARE_DECOMP.md:639`). |
| Forward-horizon (v0.2) alignment | `plans/PLAN_2026-06-17_prd_completion.md` Forward Horizon table (post-D-21/`DEC-056`, SCA-005): Phase G owns PKG-14 (+DEL-07-08, DEL-08-06), Phase H owns PKG-13/15/17, Phase I owns PKG-16. The F1 suggestion (written for the v0.1 arc) grouped PKG-13–16 "app-side"; the adopted v0.2 forward authority spreads them across G/H/I. |
| Issuance-phase framework expectation | `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md:261-266` (Phase 7: Issuance): `_COORDINATION.md` should define `ISSUED` criteria, the human approval gate, immutable evidence requirements, and the change path for already-issued deliverables. |

### 2.1 The Current `CHECKING` Cohort (Checked Cold, 2026-07-04)

All paths relative to
`execution/PKG-00_Software Architecture Runway/1_Working/`; each `_STATUS.md`
carries the header `**Current State:** CHECKING` at line 3 and the 2026-06-04
CHECKING history entry quoted in §2.

| Deliverable | Folder |
|---|---|
| DEL-00-01 | `DEL-00-01_Architecture decision record baseline/` |
| DEL-00-02 | `DEL-00-02_Repository and module boundary architecture/` |
| DEL-00-03 | `DEL-00-03_Application service command-query-job model/` |
| DEL-00-04 | `DEL-00-04_Persistence and schema versioning architecture/` |
| DEL-00-05 | `DEL-00-05_GUI state and interaction architecture/` |
| DEL-00-06 | `DEL-00-06_Diagnostics, warning, and result-envelope contract/` |
| DEL-00-07 | `DEL-00-07_API boundary and adapter contract map/` |
| DEL-00-08 | `DEL-00-08_Layered software test and acceptance strategy/` |

All eight share the same recorded path: `OPEN` (2026-04-30) → `INITIALIZED`
(2026-04-30) → `SEMANTIC_READY` (2026-04-30) → `CHECKING` (2026-06-04), per
each `_STATUS.md:7-10`.

### 2.2 Unresolved Notes (K-INVENT-1)

1. The 2026-06-04 "human-approved corrected PKG-00 architecture-basis
   re-review" is asserted only inside the eight PKG-00 `_STATUS.md`
   histories. A standalone review-snapshot record for that approval was not
   located under `execution/_Reconciliation/` during this cold check — the
   same-day `TASK_RUN_2026-06-04_authority-refresh-0.7-dag006_DEL-00-0X` run
   records carry `lifecycle-changes: not_authorized` and record pre-run state
   `SEMANTIC_READY`, so they are not it. Consequence drawn here: the proposed
   Wave 1 evidence bar (§5) requires **fresh** per-deliverable review
   snapshots rather than relying on the 2026-06-04 record. The transition
   itself is not disputed; only the durable record's location is unresolved.
2. Vocabulary nuance, noted not resolved: PKG-00 histories pass through
   `SEMANTIC_READY`, a state not present in the `docs/TYPES.md:242` chain
   (`OPEN → INITIALIZED → IN_PROGRESS → CHECKING → ISSUED`). No PKG-00
   deliverable ever recorded `IN_PROGRESS`.

## 3. Constraints Carried Into Any Wave Structure

1. **Only the human changes lifecycle state.** Every wave, however grouped,
   resolves into individual human issuance acts (or an explicit human cohort
   approval enumerating deliverable IDs). Agents prepare packets and evidence
   only (`plans/PLAN_2026-06-17_prd_completion.md:214,316`).
2. **`ISSUED` freezes the deliverable.** Issued deliverables leave ordinary
   work-selection scope and reopen only through a human-approved change path
   (`execution/_Coordination/_COORDINATION.md:667-669`). For PKG-00 this
   bites hardest: it is the live architecture authority injected into sealed
   briefs (`SOFTWARE_DECOMP.md:420`), and DEL-00-01 is a standing ADR
   surface — the D-13 ruling stood up "the DEL-00-01 ADR surface with this as
   its first accepted entry" (`_REGISTER.md:40`, ruled `DEC-020`). Issuing
   PKG-00 converts every future architecture evolution into a formal
   change-path event. That is a legitimate owner choice, not a defect — but
   the wave structure must present it, not bury it.
3. **Only `CHECKING` deliverables are issuance candidates.** The 92
   `IN_PROGRESS` deliverables enter a wave only after they individually
   return to `CHECKING` through the existing human-gated review path. That
   path is precedented, not invented: a REVIEW snapshot under
   `execution/_Reconciliation/Reviews/` recommending
   `RECOMMEND_ADVANCE_TO_CHECKING`, accepted by explicit human Gate 5
   approval — the recorded route for DEL-01-01 (2026-06-03, its
   `_STATUS.md:13`) and for the PKG-05 cohort (2026-06-05 entries citing
   `REV_DEL-05-01_2026-06-05_2021`, `REV_DEL-05-02_2026-06-05_2120`, and
   siblings). No wave definition may imply promotion to `CHECKING`.
4. **No schedule pressure.** The register and plan mark D-11 human-paced
   throughout (`_REGISTER.md:68`; plan `:92,298`). Waves are
   eligibility-and-evidence containers. This packet proposes no dates, and a
   ruled structure creates no obligation to issue any wave at any time.

## 4. Candidate Wave Structure (Basis For The Options)

Grounded in the F1 suggestion, adapted to the post-`DEC-056`/SCA-005 forward
horizon, and checked against the cold census (8 `CHECKING` + 92 `IN_PROGRESS`
+ 1 `ISSUED` = 101 deliverables):

| Wave | Members (count) | Entry condition (eligibility, not schedule) |
|---|---|---|
| **W1 — Architecture runway** | PKG-00: DEL-00-01..08 (8) | Already `CHECKING` (since 2026-06-04). Eligible once the §5 evidence bar is met; the owner decides if and when — including deliberately holding W1 open while PKG-00 remains live architecture authority (§3 point 2). |
| **W2 — Solver-side foundations** | PKG-02 (5), PKG-03 (8), PKG-05 (5), PKG-06 (5) = 23 | Members back in `CHECKING` after Phase D evidence (per F1). |
| **W3 — Solver core** | PKG-04 (6) | Members back in `CHECKING` after D6/D7 (per F1). |
| **W4 — App-side v0.1** | PKG-07 (8), PKG-08 (6) = 14 | Members back in `CHECKING` after C5/Phase D app absorption (per F1, narrowed — see the note below). |
| **W5 — Release-side** | PKG-09 (5), PKG-10 (5), PKG-12 (5) = 15 | Members back in `CHECKING` after Phase E (per F1). Phase E itself gates on open rows `D-06`/`D-05b` (E5), `D-20` (E7), `D-04b` (E8) per plan §2/§3 — those rulings are prerequisites for W5 *members maturing*, not part of this decision. |
| **W6 — Governance and documentation closure** | PKG-01 remainder (3: DEL-01-02..04), PKG-11 (5) = 8 | Members back in `CHECKING`; natural terminal wave at governance closure. F1 left these packages unassigned; this packet proposes the home. |
| **W7 — Forward horizon v0.2** | PKG-14 (5) with Phase G; PKG-13 (4), PKG-15 (4), PKG-17 (9) with Phase H; PKG-16 (4) with Phase I = 26 | Members back in `CHECKING` after their owning v0.2 phase; sub-waves W7a (G), W7b (H), W7c (I) if the owner prefers finer batches. |

Count check: 8 + 23 + 6 + 14 + 15 + 8 + 26 = 100, plus `DEL-01-01` already
`ISSUED` = 101 (cold census, §2).

Note on W4/W7: F1's suggestion (2026-06-17, v0.1 arc) grouped PKG-13–16 into
the app-side wave. After D-21/`DEC-056` adopted the v0.2 milestone set and
SCA-005 propagated it, PKG-13/14/15/16/17 are owned by forward Phases G/H/I
(plan Forward Horizon table). This packet therefore proposes them as W7,
keeping W4 to PKG-07/08. This is an adaptation of the F1 suggestion to the
current forward authority, offered for ruling — not a claim that F1 was
wrong when written.

Cross-deliverable members: the Forward Horizon table places DEL-07-08 and
DEL-08-06 with Phase G scope. Under this structure they stay with their
owning packages (W4), but the owner may re-seat those two rows into W7a at
ruling time; nothing else in the structure depends on that choice.

## 5. Evidence Bar (Proposed, Per Wave)

Before the owner considers issuing any deliverable in any wave, each member
carries — following the DEL-01-01 precedent, not invented ceremony:

1. **A fresh review snapshot** under `execution/_Reconciliation/Reviews/`
   (`REV_DEL-XX-YY_<date>` with `Brief.md`, `QA_Report.md`,
   `Decision_Log.md`, `RUN_SUMMARY.md` — the recorded DEL-01-01 shape),
   prepared against the **current** project authority: the current
   `SOFTWARE_DECOMP.md` revision, the current approved DAG pointer
   (`DAG-007` as of 2026-06-22, `_COORDINATION.md` work-loop step 6), and
   the SCA-005 forward authority. The snapshot recommends an issuance
   disposition the human is free to reject.
2. **Status hygiene:** `_STATUS.md` header and history consistent — no
   `STATUS_HISTORY_MISMATCH` drift, per the class the 2026-07-02 lifecycle
   correction resolved
   (`execution/_Reconciliation/LifecycleCorrection/LIFECYCLE_CORRECTION_2026-07-02_2050/Decision_Log.md`).
3. **Dependency closure check** against the current approved DAG pointer for
   edges into the deliverable, recorded in the snapshot.
4. **Boundary language** in the issuance history entry, per the DEL-01-01
   wording: ISSUED is deliverable lifecycle closure only; it is not release
   acceptance, legal advice, professional approval, certification, sealing,
   authentication, or code-compliance.
5. **A human approval record per deliverable**, or one explicit human cohort
   approval enumerating the deliverable IDs it covers. Cohort-level human
   action has precedent (the 2026-06-04 PKG-00 entry), and the missing
   standalone record for that act (§2.2) is exactly why this bar asks for
   durable snapshot records this time.

For W1 specifically, given §3 point 2 (issuance freezes the live
architecture authority), the W1 snapshots must state what future
architecture evolution would then require — the human-approved change path —
so the owner rules on the freeze knowingly.

## 6. Options

| ID | Option | Consequence |
|---|---|---|
| **O-A** | **Phase-aligned cohort waves: adopt the §4 seven-wave structure with the §5 evidence bar, owner-paced.** | One ruling closes D-11's structural question. Bounded, coherent human review batches aligned to phase evidence; W1 (PKG-00) is the only wave that can become eligible without further lifecycle movement. Future waves activate only as members return to `CHECKING` via the existing Gate 5 path. Wave boundaries remain adjustable by a later decision record if phase reality diverges. |
| **O-B** | **Single omnibus terminal wave: hold all issuance (including PKG-00) until every deliverable reaches `CHECKING`, then one project-wide issuance event.** | Minimal ruling overhead now; maximal review pile later — one ~100-deliverable human review event, the "surprise blocker" shape Phase F scheduling exists to avoid (plan `:212`). Architecture authority stays unfrozen longest (a benefit for PKG-00 evolution), but Phase F closure defers entirely to project end. |
| **O-C** | **Per-deliverable issuance, no waves: repeat the DEL-01-01 single-deliverable pattern (~100 times) as items mature.** | Maximum granularity and precision; heaviest owner load — each issuance is its own full review-and-ruling cycle with no batching benefit. Precedented (DEL-01-01), but the precedent was one deliverable, not one hundred. D-11's wave question is answered by declining waves. |
| **O-D** | **Rule W1 scope only now (PKG-00 as the sole defined wave + the §5 evidence bar); defer all future-wave structure until the next cohort actually reaches `CHECKING`.** | Smallest binding surface; nothing speculative is ruled about the 92 `IN_PROGRESS` deliverables. D-11 closes only partially — a residual-work row per the 2026-07-03 register convention (`_REGISTER.md:17-24`) carries the future-wave structure, and Phase F closure keeps an open structural question. |

Under every option, each issuance act remains individually human-gated and
owner-paced; the options differ only in how the eligibility structure is
batched.

## 7. Recommended Disposition (PROPOSAL)

Recommend **O-A**: the seven-wave, phase-aligned structure of §4 with the §5
evidence bar, explicitly owner-paced.

Rationale: it is the only option that both closes the D-11 structural
question in one ruling (unlike O-D, which leaves a residual row, and O-C,
which declines the question) and keeps human review batches bounded and
evidence-coherent (unlike O-B's terminal pile-up). It is also the option
closest to the plan's own recorded intent — this packet's W1–W7 is F1's wave
alignment (`plans/PLAN_2026-06-17_prd_completion.md:214`), extended to cover
the packages F1 left unassigned (PKG-01 remainder, PKG-11, PKG-17) and
re-seated on the adopted v0.2 forward horizon — so it follows existing
practice rather than inventing structure. The recommendation is non-binding;
O-D is the natural fallback if the owner prefers to rule nothing speculative
about packages still `IN_PROGRESS`.

Explicitly **not** recommended by this packet: issuing W1 (or anything else)
now. Whether and when PKG-00's freeze trade-off (§3 point 2) is worth taking
is entirely the owner's call, made wave by wave, at the owner's pace. A
ruled O-A leaves the owner free to hold every wave indefinitely.

## 8. Consequence For Readers

If a wave structure is ruled, the lifecycle truth surfaces do not move:
deliverable-local `_STATUS.md` files remain the state authority
(`_COORDINATION.md:226-229`), the census stays `CHECKING=8, IN_PROGRESS=92,
ISSUED=1` until the owner acts, and `DEL-01-01` remains the sole `ISSUED`
deliverable. Wave membership confers no authority on any deliverable's
content: `CHECKING` continues to mean design authority is mature, not that
the application has absorbed the scope (`_COORDINATION.md:238-240`), and
PKG-00's architecture basis keeps flowing through sealed briefs under
SCA-001 exactly as before (`SOFTWARE_DECOMP.md:420`) whether or not W1 is
ever issued.

## 9. Human Ruling And Disposition

**Ruling recorded:** O-A approved as recommended by owner (Ryan Tufts), 2026-07-04 — in-session blanket slate direction ('proceed as recommended through the "Decision slate"'; verbatim in `_DomainEngines/bridge/LOOP_RECEIPTS.md` Receipt 20): the seven-wave, phase-aligned structure of §4 with the §5 evidence bar, explicitly owner-paced. Codified as `DEC-062` in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12; register row D-11 → RULED. No wave is issued and no lifecycle state changes under this ruling; the census stays CHECKING=8 / IN_PROGRESS=92 / ISSUED=1 until the owner acts wave by wave, and the owner remains free to hold every wave indefinitely.

## 10. Ruling Mechanism

Per existing practice, the human project authority selects an option or
rules a modified structure directly. The ruling is appended to
`execution/_Decomposition/SOFTWARE_DECOMP.md` §12 as the next `DEC` entry
citing this packet; the D-11 register row then moves from `AWAITING_RULING`
to `RULED` with the decision pointer. No lifecycle state changes under this
packet: any subsequent wave issuance is its own separate human act on the
deliverable-local `_STATUS.md` surfaces per the DEL-01-01 precedent, and any
future re-batching of waves is a new decision record, not an edit to this
one.
