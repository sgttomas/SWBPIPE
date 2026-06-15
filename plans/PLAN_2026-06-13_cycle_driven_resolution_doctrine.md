# Cycle-Driven Resolution Doctrine — chirality-piping adoption + rollout record

> **Canonical doctrine relocated (2026-06-15).** The project-neutral doctrine now
> lives at the shared **`docs/CYCLE_DRIVEN_RESOLUTION.md`** (adopted `DEC-040`).
> This file is retained as **chirality-piping's adoption, rollout, and DAG-007
> re-derivation record**: the surface-by-surface rollout (§3), the worked example
> (§4), the §7 / `DEC-040` ruling, and the 2026-06-15 DAG-007 trigger amendment
> are piping-specific and stay here. For the reusable principle itself, read the
> shared doctrine.

**Date:** 2026-06-13 (adopted `DEC-040`, 2026-06-15)
**Epistemic status:** Adopted, non-governing record (originally a PROPOSAL
authored by `WORKING_ITEMS`). This plan proposed amendments to authority and
tool-root surfaces (strategy, coordination, the decision register, the shared
`agents/` framework, a skill, and the DAG audit tool); the accepted subset was
applied under `DEC-040` and the principle promoted to the shared doctrine above.
The §6 scope/authority map and §7 open decisions are retained as the adoption
audit trail.

**Provenance:** distilled from the 2026-06-13 design discussion on dependency
modeling. Core insight: a DAG is objective-relative and its edges are often
subjective, but the **strongly-connected-component (SCC) condensation is unique
and always a DAG**. So the durable improvement is to make *cycle-driven
resolution* an explicit, reusable competence of the operating system rather than
an ad-hoc audit footnote.

---

## 1. Motivation

The project already keeps its *active* dependency graph acyclic and quarantines
cycles to a non-gating candidate layer (see [DAG-006 audit](../execution/_DAG/DAG-006/DAG_Audit.md):
2 candidate-layer SCCs, 0 active SCCs). What is **not** written down is the
*method* by which tangled work is turned into an order, or the reasoning habit
that should apply whenever any complex work — not just the DAG-006 artifact —
looks circular. This plan codifies and generalizes that method.

The value is threefold: (a) it **localizes and minimizes** the subjective calls
to the few edges that actually close cycles; (b) it turns DAG construction into
**diagnose-and-repair** instead of global authoring; and (c) it makes each
cycle-breaking decision **auditable** (a recorded move + rationale), which fits
this project's "no silent reconciliation" ethos exactly. This is codification and
generalization, **not** a fix to a broken graph.

---

## 2. The doctrine (the liftable kernel)

> **Cycle-Driven Resolution Doctrine**
>
> 1. **No canonical DAG a priori.** A dependency graph is relative to a stated
>    *objective* and *edge semantics* (build-order ≠ runtime ≠ knowledge ≠
>    data-flow ≠ deployment). Fix both before drawing edges; never conflate
>    graphs drawn for different objectives.
> 2. **SCCs are the objective skeleton.** Under fixed semantics the SCC
>    condensation is unique and always a DAG. Treat the SCC set — not the
>    hand-drawn edges — as the authoritative signal of *where ordering is
>    genuinely undecided*. Edges in no cycle need no adjudication; their
>    direction is consistent with some valid global order regardless of
>    interpretation.
> 3. **Localize and record the subjective calls.** Interpretive decisions are
>    required only on cycle-closing edges. Resolve each non-trivial SCC by one
>    of four named moves, with a recorded rationale — never reconcile a cycle
>    silently:
>    - **Decompose** a too-coarse node (the usual root cause — split interface
>      from implementation, concern from concern).
>    - **Invert** a dependency behind a contract/interface so the edge reverses.
>    - **Merge** the cluster and accept it as one indivisible unit.
>    - **Cut** an edge by reclassifying it as out-of-objective (e.g.,
>      runtime/test/optional, not sequencing).
>    *Decompose* and *Invert* are design refinements an agent may propose;
>    *Cut* and *Merge* encode a subjective interpretation into authority and are
>    therefore **human-gated**.
> 4. **Cycle-participating edges stay non-gating until resolved.** (Already true
>    for candidate edges; this names it a rule.)
> 5. **Proportionality.** Obvious resolutions get a one-line note; contested
>    ones (cut/merge, or objective-dependent) get a decision packet. Apply the
>    lens when work is tangled or the audit flags an SCC — do not ritualize it
>    everywhere.

---

## 3. Proposed changes by surface

Each item gives the target file, the insertion point, and the proposed
text/behavior. Scope is flagged: **[in-project]** = within
`projects/chirality-piping/`, `WORKING_ITEMS` may apply after a human ruling;
**[cross-project]** = lives in the shared framework above this project and needs
explicit owner authorization.

### 3.1 Strategy — `docs/PLAN.md` **[in-project]**

Insertion: a new short section after §3 ("How the strategic layers relate"),
titled **"Dependency epistemics (how the DAG is built and maintained)."**
Proposed text:

```markdown
## Dependency epistemics — how the DAG is built and maintained

A dependency graph is not a fact to be discovered but a model relative to a
stated objective and edge semantics; individual edges are frequently a judgment
call. The one objective invariant, once semantics are fixed, is the
strongly-connected-component (SCC) decomposition: contracting each SCC yields a
unique condensation that is always a DAG. The project therefore builds and
maintains its dependency graph by *cycle-driven resolution* — treating SCCs as
the diagnostic of where ordering is undecided, resolving each by an explicit,
recorded move (decompose / invert / merge / cut), and holding cycle-participating
edges non-gating until resolved. This keeps the subjective decisions localized to
cycle-closing edges and auditable. The operative practice lives in
`_COORDINATION.md`; the full doctrine and rationale are recorded in
`plans/PLAN_2026-06-13_cycle_driven_resolution_doctrine.md`.
```

### 3.2 Practice — `execution/_Coordination/_COORDINATION.md` **[in-project, coordination root — propose, human rules]**

Three edits:

**(a) "State Tracking Rules" — augment the DAG-authority bullet** (currently:
"`DAG-006/` says what depends on what, using approved active edges only.
Candidate rows remain non-gating…"). Append:

```markdown
   SCCs are the primary diagnostic of undecided ordering: the audit reports
   strongly-connected components, and any edge that participates in a cycle is
   non-gating until the SCC is resolved by a recorded move (decompose / invert /
   merge / cut). The active edge set is kept acyclic by construction; cycles
   live only in the candidate layer pending resolution.
```

**(b) Loop step 4 "Decision escalation" — add an SCC-resolution branch.** After
the existing decision-preparation bullets, add:

```markdown
   - when an unresolved SCC blocks or is relevant to the selected work, the
     tranche is an SCC-resolution tranche: list the SCC's internal edges, choose
     a resolution move per the cycle-driven resolution doctrine, and record it.
     Decompose/Invert moves are agent-proposable design refinements; Cut/Merge
     moves encode a subjective interpretation and are prepared as a human-gated
     decision packet in `_DECISIONS/` (a cycle-resolution row, §below). Trivial
     resolutions are recorded as a one-line run-record note (proportionality).
```

**(c) Loop step 9 "Handoff and git closeout" and the session-summary rule —**
add "open SCCs awaiting resolution" alongside the pending-rulings summary, so
each session surfaces the unresolved cycles the same way it surfaces pending
human decisions.

### 3.3 Decision record — `execution/_Coordination/_DECISIONS/_REGISTER.md` **[in-project]**

Add a **cycle-resolution row convention** so cut/merge interpretations are
auditable like `SCA`/`DEC` entries. Proposed: a parallel id space `SCC-XXX`
(or reuse `SCA-` for the human-gated cut/merge cases) with columns *SCC id →
member nodes → chosen move → rationale → ruling record*. State machine mirrors
the existing `NOT_PREPARED → AWAITING_RULING → RULED`. Decompose/Invert
resolutions may be recorded directly (agent-proposable); Cut/Merge require a
ruling pointer.

### 3.4 Agent habit — `agents/AGENT_WORKING_ITEMS.md` **[cross-project — shared framework, owner-gated]**

Two edits, both *generalizing the existing* "Conflict transparency" invariant
from content contradictions to structural cycles:

**(a) Non-negotiable invariants — extend "Conflict transparency":**

```markdown
- **Conflict transparency.** When sources or documents contradict, present a
  Conflict Table and request a ruling. A **structural cycle** (a strongly-
  connected component in a dependency or sequencing graph) is a conflict of the
  same kind: surface it, do not silently linearize it. Resolve it by an explicit
  move (decompose / invert / merge / cut) with a recorded rationale; cut/merge
  are human-gated.
```

**(b) PROTOCOL — when framing/decomposing complex work (Phase 1) and when
dispatching bounded sub-tasks (Phase 4):** add a note that tangled work is
modeled SCC-first — surface the strongly-connected subset, resolve it into a DAG
by a named move, *then* sequence — rather than forcing a linear plan over a
genuinely circular structure. (Optionally add a "Cycle/SCC resolution" companion
to the Phase 3 Conflict Table mechanism.)

### 3.5 Worker habit — `agents/AGENT_TASK.md` **[cross-project — not yet read; verify insertion on implementation]**

Parallel to 3.4: a bounded worker that encounters a mutual dependency / cycle
within its slice surfaces it as an SCC with the four-move options and returns it
in its run summary, rather than silently choosing an order. *Note: I have not
read `AGENT_TASK.md` this session; the exact insertion point must be confirmed
against its actual structure before drafting final text.*

### 3.6 Procedure / skill — "decompose-and-de-cycle" **[location TBD]**

A step-by-step operational artifact: fix objective + edge semantics → draw edges
liberally → compute SCCs (Tarjan, linear) → the condensation is the working DAG
→ for each non-trivial SCC choose decompose/invert/merge/cut with rationale →
re-run, iterate to trivial SCCs. *Open question:* whether project-local skills
exist here or whether this should be a `docs/` procedure (e.g.,
`docs/DEPENDENCY_DECOMPOSITION.md`); to confirm before placement.

### 3.7 Tooling — `tools/coordination/audit_dag.py` **[in-project — not yet read internally; behavior proposal]**

It already computes and reports SCCs. Proposed behavior additions:
- Promote the SCC list from an audit *warning* to an actionable **backlog**
  (e.g., a `--list-sccs` mode and/or an `Open SCCs` section in the audit output).
- A per-SCC **edge-lister** (e.g., `--scc <id>`) that prints the internal edges
  of a component, so a resolution decision can name exactly which edge it
  cuts/inverts. *Note: confirm the tool's current CLI/structure before
  implementing.*

---

## 4. Worked example — the two live candidate SCCs

Applying the doctrine to the components the [DAG-006 audit](../execution/_DAG/DAG-006/DAG_Audit.md)
already flags (illustrative; real resolution requires reading each SCC's edges
via 3.7 and is human-gated for any cut/merge):

- **SCC-C-001:** `DEL-10-02` (import/export adapter framework) ↔ `DEL-12-01`
  (local-first storage / private paths) ↔ `DEL-12-05` (security threat model).
  Likely candidates: *invert* (a storage/security contract the adapter depends
  on, one-directionally) or *cut* (if a flagged edge is runtime/policy, not
  sequencing).
- **SCC-C-002:** the `DEL-08` reporting cluster + `DEL-09-05` (release-quality
  gate) + `DEL-10-04/05` (build/CI, headless runner). Likely candidate:
  *decompose* (separate the report-contract from the report-emit concern) or
  *merge* (accept the reporting/evidence cluster as one release-together unit).

These become the **first application** of the doctrine and clear the two
standing SCC warnings — each as a recorded cycle-resolution decision.

---

## 5. Guardrails

- **Proportionality is load-bearing** (doctrine §5): the failure mode is
  ossifying into "every cycle needs a packet." Trivial decompose/invert → a
  one-line note; only cut/merge or objective-dependent calls → a packet.
- **Objective-scoping:** the doctrine is applied *per objective + edge
  semantics*; it does not unify graphs across objectives, and it catches
  *circular* subjectivity, not *ordering/priority* subjectivity (an acyclic-but-
  "wrong" graph is not flagged by SCC analysis). State this so it is not
  oversold.
- **Hairball caution:** a single giant SCC reduces "resolve into a DAG" to a
  feedback-arc-set problem (NP-hard); the tool should report SCC sizes so large
  components get human attention rather than a naive auto-cut.

---

## 6. Scope, authority, and rollout order

| Surface | Scope | Authority to apply |
|---|---|---|
| `docs/PLAN.md` (§3.1) | in-project | human ruling on this proposal |
| `_COORDINATION.md` (§3.2) | in-project, coordination root | human ruling (coordination-instruction edit) |
| `_DECISIONS/_REGISTER.md` (§3.3) | in-project | human ruling |
| `agents/AGENT_WORKING_ITEMS.md` (§3.4) | **cross-project** (shared `agents/`) | framework owner authorization |
| `agents/AGENT_TASK.md` (§3.5) | **cross-project** | framework owner authorization |
| skill / `docs/` procedure (§3.6) | TBD | human ruling once located |
| `tools/coordination/audit_dag.py` (§3.7) | in-project | human ruling |

Recommended rollout (smallest coherent unit first):
1. **Principle + practice + register** (§3.1–3.3) — the in-project "principle →
   practice → record" spine; self-contained and immediately usable.
2. **Tooling** (§3.7) — makes the practice operable (SCC backlog + edge-lister).
3. **First application** (§4) — resolve SCC-C-001/002 to prove the loop.
4. **Agent + worker habit** (§3.4–3.5) and **skill** (§3.6) — the generalization
   beyond the DAG artifact; requires cross-project authorization.

---

## 7. Open decisions for human ruling

- **D-CDR-1:** Accept the doctrine kernel (§2) as project posture? (gates all
  §3 edits)
- **D-CDR-2:** Which surfaces, and how far? (rollout tiers in §6)
- **D-CDR-3:** Authorize the `agents/` framework edits (§3.4–3.5), which are
  above this project's write scope?
- **D-CDR-4:** Register convention — new `SCC-XXX` id space vs. reuse `SCA-`
  for cut/merge (§3.3)?
- **D-CDR-5:** Skill vs. `docs/` procedure, and where, for §3.6?

**Ruling 2026-06-15 (`DEC-040`, human project authority).** Doctrine kernel
adopted as posture (**D-CDR-1**); rollout = the in-project routing + the
cross-project agent-instruction habit, with the operational machinery deferred
(**D-CDR-2**); `agents/` framework edits authorized (**D-CDR-3**); future
cut/merge reuse the `SCA-` id space, no new namespace (**D-CDR-4**); a standalone
skill/procedure is deferred, a `docs/` procedure preferred if ever (**D-CDR-5**).
**Applied this sitting:** `docs/PLAN.md` dependency-epistemics subsection;
`_COORDINATION.md` State-Tracking DAG bullet (+ the re-derivation trigger);
completion-plan Phase D DAG-007 trigger note; `agents/AGENT_WORKING_ITEMS.md`
(Conflict-transparency invariant extended to structural cycles + a Phase 3
companion); `agents/AGENT_TASK.md` (an epistemic-controls SCC bullet); and the
`D-CDR` register row. **Deferred to the next decomposition revision / SCA (the
DAG-007 trigger, §Amendment 2026-06-15):** the loop step-4 SCC-resolution branch
(§3.2b), per-session open-SCCs surfacing (§3.2c), the register SCC-row convention
(§3.3), the `audit_dag.py` backlog/edge-lister flags (§3.7), and the standalone
skill/procedure (§3.6) — deferred on §5 proportionality (0 active SCCs; the
graph matches its rev-0.7 basis), not on authority.

---

## 8. Verification (how we would know it is working)

- The DAG audit reports an **Open SCCs** backlog and each session's handoff
  surfaces it alongside pending rulings.
- Every cycle resolution appears as a recorded move + rationale (one-line note
  for trivial, register row for cut/merge) — i.e., no cycle is silently
  linearized.
- The two live candidate SCCs are resolved or explicitly merged, dropping the
  audit's active-plus-candidate SCC warning count.
- Spot-check: an agent handed a tangled decomposition surfaces the SCC and a
  named move rather than emitting an arbitrary linear order.

---

## Amendment 2026-06-15 — DAG re-derivation (DAG-007): not warranted now; same trigger as this doctrine

**Author:** `WORKING_ITEMS`. **Status:** PROPOSAL (non-governing), consistent with
the parent plan. Records a 2026-06-15 assessment of whether a new dependency-graph
run (`DAG-007`) is warranted, and folds two corrections back into §3.7 and §4.

**Finding — `DAG-007` is not warranted at this time.** The approved graph
authority `DAG-006` is bound to decomposition revision `0.7` (its
`APPROVAL_RECORD`), and `SOFTWARE_DECOMP.md` is still at revision `0.7`: every
change since the 2026-06-03 freeze is a decision-log append (DEC-022…039), not a
structural decomposition change. The last scope-change record is dated
2026-05-03; no SCA-005 / v0.8 exists. New DAG versions are **event-driven by a
decomposition/scope change** (DAG-002←SCA-002, 004←SCA-003, 005←SCA-004; 006 was
a metadata-only cleanup over the 005 edge set), not periodic. The inputs to a
re-derivation are therefore essentially static — only 3 of ~101
`Dependencies.csv` registers changed since the freeze, and the touched source
docs were lifecycle/evidence refreshes, not edge changes — so a run would
reproduce `DAG-006`'s topology (945 active edges, 0 active SCCs, 2 candidate
SCCs) at human-gated cost, informing no current decision. App-integration
consumes mature `CHECKING` design authority and writes code; it does not
re-author the decomposition the DAG models, and `_COORDINATION.md` already
demotes the DAG to design-context discovery only during this phase.

**Connection to this doctrine — same trigger, first live application.** A
`DAG-007` run and this doctrine's operational machinery (§3.2b/c, §3.3) share one
trigger: the **next decomposition revision / SCA** — most concretely Phase D (R4)
component deliverables and the assembled nonlinear-solve seam, a D-12 disposition
that changes the deliverable set, or newly decomposed Phase E deliverables. At
that event the correct cycle is: bump the revision under an SCA → re-run
`dependency-extract` on the changed deliverables → AGGREGATION →
`audit_dag.py` / `analyze_dep_closure.py` closure audit → `DAG-007` + human
`APPROVAL_RECORD`. That closure audit is exactly where the doctrine earns its
first real use: any *new active* SCC is resolved by a recorded
decompose/invert/merge/cut move, and the two standing candidate SCCs (§4) become
co-resolution candidates at the same sitting because their clusters are the
Phase D/E surfaces. **Adopt the principle now; let both the cycle-resolution
machinery and graph re-derivation activate together at that event — not before.**
This is §5 proportionality applied to the graph itself.

**Correction to §3.7.** `tools/coordination/audit_dag.py` **exists** — at the
shared repo-root `tools/coordination/` (per the `DAG-006` approval record's
validation commands), alongside `analyze_dep_closure.py`,
`materialize_local_dependencies.py`, and `tools/aggregation/`. It already
computes SCCs (Tarjan) and generated `DAG_Audit.md`. The §3.7 premise stands; its
proposed additions (`--list-sccs`, `--scc <id>`, an "Open SCCs" backlog section)
are feasible edits to an existing tool, not new tooling. (An earlier reviewer
note that the file was absent reflected a check of the project-local `tools/`
rather than the shared repo-root `tools/`.)

**Caution carried from doctrine principle #1 (do not conflate objectives).** If
the impulse to re-run is "the frozen graph no longer reflects the application,"
note that the application's structure is a **code/build dependency graph** — a
different objective and edge semantics from the decomposition-dependency DAG this
doctrine and `DAG-006` concern. That would be a new artifact with its own
justification, not a `DAG-007` successor, and is not currently needed (cargo/npm
already encode build deps; selection runs off the completion plan and app
surfaces). "Re-validate the graph" must not be silently satisfied by a `DAG-007`
run.

---

*This plan is a proposal. Nothing here is applied. It makes no lifecycle,
release, professional, certification, or code-compliance claim.*
