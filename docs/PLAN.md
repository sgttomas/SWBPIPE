---
doc_id: OPS-PLAN
doc_kind: governance.strategy
status: current_strategy
created: 2026-06-13
---

# OpenPipeStress — Strategic Plan and Roadmap

**Epistemic status:** non-governing strategy. This document is the strategic
*map* for OpenPipeStress: the definition of "done", the current milestone
position, the roadmap posture, and how the project's authority layers relate.
It asserts no authority of its own and creates no lifecycle, release,
professional, certification, or code-compliance claim. When this document and an
authoritative surface disagree, the authority wins and this document is
corrected (the same rule the coordination record applies to the completion
plan).

This file fills the strategic-roadmap slot that, before 2026-06-13, was
distributed across `PRD.md` §22, `execution/_Coordination/_COORDINATION.md`, and
the active completion plan. It absorbs the durable strategic framing (definition
of done, milestone position, roadmap risks) so the active
`plans/PLAN_*.md` can stay a pure tactical selection instrument.

## Control-plane boundary — authoritative surfaces this map routes to

This document does not restate the authorities below; it routes to them.

| Question | Authoritative surface |
|---|---|
| Why the product exists; what must remain true | [INTENT.md](INTENT.md), [DIRECTIVE.md](DIRECTIVE.md) |
| What must be built; the release yardstick | [PRD.md](PRD.md) §10 (functional requirements), §22 (release milestones) |
| Binding invariants | [CONTRACT.md](CONTRACT.md) |
| Public/private data and protected-content boundary | [IP_AND_DATA_BOUNDARY.md](IP_AND_DATA_BOUNDARY.md), [PROFESSIONAL_BOUNDARY.md](PROFESSIONAL_BOUNDARY.md) |
| What depends on what | [execution/_Decomposition/SOFTWARE_DECOMP.md](../execution/_Decomposition/SOFTWARE_DECOMP.md), approved [DAG-006](../execution/_DAG/DAG-006/) until the canonical [DAG-007](../execution/_DAG/DAG-007/) rectification package receives human approval |
| How work executes; the current target-stage standard | [execution/_Coordination/_COORDINATION.md](../execution/_Coordination/_COORDINATION.md) ("Working Desktop Application Standard") — **authoritative for the stage gate** |
| Tactical tranche selection | the active [plans/PLAN_2026-06-17_prd_completion.md](../plans/PLAN_2026-06-17_prd_completion.md) |
| Human decision preparation and rulings | [execution/_Coordination/_DECISIONS/_REGISTER.md](../execution/_Coordination/_DECISIONS/_REGISTER.md) |

## 1. Definition of "complete per the PRD"

The PRD defines completion through two surfaces, and the project treats both as
binding:

1. **Release milestones (PRD §22).** R0–R5 exit criteria, taken verbatim. R5
   (Engineering Beta) is the terminal milestone: "External engineers can
   reproduce validation examples" and "Public repository contains no known
   protected standards data."
2. **Functional requirements (PRD §10).** All 16 `Must` requirements
   (FR-001..016) verified against their acceptance criteria; all 6 `Should`
   requirements (FR-017..022) implemented; the 3 `Could` requirements
   (FR-023..025) either implemented or **explicitly dispositioned by a human
   deferral record** at the R5 gate — silence is not a disposition.

Completion additionally requires the human-gated governance closures that the
PRD and CONTRACT make prerequisites for any release claim (unit catalog
acceptance, thresholds, CI/release authority, issuance of deliverables). These
are scheduled as decisions, not assumed — see the decision register and the
completion plan §2.

**Extended horizon (contingent).** The active completion plan additionally
scopes — contingent on a human-gated PRD scope-change (`D-21`) — the superseded
v0.2 milestones R6 (Design Knowledge and Handoff Beta) and R7 (Agent-Assisted
Design). Until that scope-change is ruled, **R5 remains the terminal milestone
of the current PRD authority** and R6/R7 carry no roadmap weight; the
decomposition already carries that surface (PKG-13/14/15/16/17, DEL-07-08,
DEL-08-06) as defined-but-unbuilt scope.

## 2. Current milestone position

Position summary as of 2026-06-21. This table moves only at milestone
boundaries (human-ruled stage gates), not per tranche; per-tranche progress
lives in the completion plan and `plans/PLAN_COMPLETION_LOG.md`. The
**authoritative current target stage is R4** (`DEC-048`, recorded in
`_COORDINATION.md`); this table is a position summary, not the stage authority.

| Milestone | Status | Blocking residual |
|---|---|---|
| R0 Architecture Prototype | Met in substance | R0's deferred "Unit system" is pulled onto the critical path as Phase B (B1 units crate landed 2026-06-12; B2/B3 unit-aware I/O in progress) |
| R1 Core Solver MVP | Substantially met (linear static, dense solve) | Tolerance thresholds **RULED** (`DEC-024`/`DEC-026`, D-04) |
| R2 GUI MVP | Met in substance | Exit chain verified ([VERIFICATION_2026-06-12_r2_exit_chain.md](../plans/VERIFICATION_2026-06-12_r2_exit_chain.md)); stage advanced R2→R3 (`DEC-035`). The two R2-carried residuals were closed for C5 by the 2026-06-20 replacement-criterion ruling (`DEC-047`) |
| R3 Rule packs + private libraries | Evidence accepted for target-stage advancement | C1-C4 landed; C5.8 assembled [VERIFICATION_2026-06-20_r3_exit_chain.md](../plans/VERIFICATION_2026-06-20_r3_exit_chain.md); `D-23` ruled O-A by `DEC-048` |
| R4 Components + nonlinear supports (current target) | D1 bend, D2 branch, D3 rigid/semi-rigid, and D4 expansion-joint invented-preview paths landed across app visibility, user-entered evidence, diagnostics, native-package/report provenance, bend/branch stress-recovery review rows, and EJ user-stiffness macro-element review rows (`TP-R4-D1-BENDVIS-001`, `TP-R4-D1-BENDSTRESS-001`, `TP-R4-D2-BRANCHSTRESS-001`, `TP-R4-D3-RIGIDVIS-001`, `TP-R4-D4-EJSTIFF-001`); D6 loop-core crate and invented product/result-envelope sidecar landed (`TP-R4-D6-LOOPCORE-001`, `TP-R4-D6-PHYSINTEG-001`), with product gap/lift-off live-loop coverage (`TP-R4-D6-LIVECOVER-001`), derived friction normal-source evidence (`TP-R4-D6-FRICTIONNORMAL-001`), and mixed one-way/gap/friction product live-loop coverage (`TP-R4-D6-LIVEBUNDLE-001`); assembled validation seeds landed through explicit/derived-normal friction evidence, the PRD §16.2 branch-assembly benchmark, structured convergence observations, and the governed active-set-count validation-seed policy (`TP-R4-D9-ASSEMBLEDSEED-001`, `TP-R4-D9-FRICTIONSEED-001`, `TP-R4-D9-FRICTIONSLIDE-001`, `TP-R4-D6-FRICTIONNORMAL-001`, `TP-R4-D9-BRANCHASSEMBLY-001`, `TP-R4-D9-CONVOBS-001`, `TP-R4-D9-CONVPOLICY-001`); D8 component provenance now flows into the hash-bound rendered report (`TP-R4-D8-COMPPROVREPORT-001`); sparse-solver strategy decided (`DEC-023`, first slice landed) | No spring-hanger constant-effort/variable-rate behavior; sparse live path not bound; non-seed convergence thresholds and broader R4 validation/exit evidence remain (Phase D) |
| R5 Engineering Beta | Distant | All release machinery TBD; validation manual; redaction workflow; signed releases (Phase E) |

## 3. How the strategic layers relate

The product is built by turning the PRD yardstick into evidenced, bounded
tranches through a fixed chain of surfaces:

- **Intent → Requirements.** `INTENT.md` (why / what must remain true) and the
  `PRD.md` (what must be built, §10 FRs and §22 milestones) define the target.
- **Requirements → Structure.** `SOFTWARE_DECOMP.md` decomposes the PRD into a
  flat package/deliverable set; approved `DAG-006` records what depends on what
  until the canonical `DAG-007` rectification package is human-approved.
- **Structure → Execution.** `_COORDINATION.md` is the operational control
  plane: the Application Integration And Issuance Loop, the staged Working
  Desktop Application Standard (the current target stage lives here), bounded
  workers, validation, evidence, and git closeout.
- **Execution → Selection.** The active `plans/PLAN_*.md` orders the work toward
  the PRD yardstick — phase plan, dependency spine, FR completion map — as
  *selection guidance*, not authority. Landed detail compresses into
  `plans/PLAN_COMPLETION_LOG.md`.
- **Human gates throughout.** `_DECISIONS/_REGISTER.md` tracks the human-gated
  decisions (`D-01..D-12` and successors); only humans rule them, captured as
  `DEC`/`SCA` records in `SOFTWARE_DECOMP.md`.

When a lower layer and a higher authority disagree, the discrepancy is surfaced
and the lower layer (this map, the completion plan, a register) is corrected —
never the authority.

### Dependency epistemics — how the DAG is built and maintained

A dependency graph is not a fact to be discovered but a model relative to a
stated objective and edge semantics; individual edges are frequently a judgment
call. The one objective invariant, once semantics are fixed, is the
strongly-connected-component (SCC) decomposition: contracting each SCC yields a
unique condensation that is always a DAG. The project therefore builds and
maintains its dependency graph by *cycle-driven resolution* — treating SCCs as
the diagnostic of where ordering is undecided, resolving each by an explicit,
recorded move (decompose / invert / merge / cut), and holding cycle-participating
edges non-gating until resolved. This localizes the subjective calls to
cycle-closing edges and keeps them auditable. `DAG-007` was generated for the
2026-06-16 dependency type-system rectification, preserving the approved
`DAG-006` active topology while moving current write-form registers to the
canonical enum model; `_DAG/_LATEST.md` stays on `DAG-006` until human
approval. Later DAG versions remain event-driven by a decomposition revision /
SCA or another explicit governance rectification, not periodic. The canonical
doctrine is the shared repo-root
[CYCLE_DRIVEN_RESOLUTION.md](../../../docs/CYCLE_DRIVEN_RESOLUTION.md); the
operative practice lives in
[_COORDINATION.md](../execution/_Coordination/_COORDINATION.md); and this
project's adoption, rollout, and historical DAG re-derivation trigger notes are in
[PLAN_2026-06-13_cycle_driven_resolution_doctrine.md](../plans/PLAN_2026-06-13_cycle_driven_resolution_doctrine.md).

## 4. Current roadmap posture

- **Target stage: R4 / Phase D** (`DEC-048`, after `DEC-035` advanced to R3).
  Ordinary development completed the Phase C implementation spine, C5
  exit-readiness program, and `TP-R3VERIFY-001` R3 exit-chain packet; the human
  accepted `D-23` Option O-A for R3 exit evidence and target-stage advancement.
- **D1 bend objects, D2 branch objects, D3 rigid/semi-rigid components, D4
  expansion joints, the first D6 product-preview nonlinear sidecar, D6
  mixed live nonlinear bundle, D8
  rendered-report component provenance, and the D9 assembled validation seeds
  through explicit/derived-normal friction plus the branch-assembly benchmark
  and structured convergence observations
  landed for the invented preview path**
  through
  `TP-R4-D1-BENDVIS-001`, `TP-R4-D1-BENDSTRESS-001`,
  `TP-R4-D2-BRANCHSTRESS-001`, `TP-R4-D3-RIGIDVIS-001`, and
  `TP-R4-D4-EJSTIFF-001`, `TP-R4-D6-PHYSINTEG-001`,
  `TP-R4-D6-LIVECOVER-001`, `TP-R4-D6-LIVEBUNDLE-001`,
  `TP-R4-D8-COMPPROVREPORT-001`,
  `TP-R4-D9-ASSEMBLEDSEED-001`, and
  `TP-R4-D9-FRICTIONSEED-001`, and
  `TP-R4-D9-FRICTIONSLIDE-001`, `TP-R4-D6-FRICTIONNORMAL-001`,
  `TP-R4-D9-BRANCHASSEMBLY-001`, `TP-R4-D9-CONVOBS-001`, and
  `TP-R4-D9-CONVPOLICY-001`; D5 is gated by `D-15`, whose packet is
  prepared and awaiting human ruling.
- **The two R2-carried R3-exit residuals are closed for C5** by `DEC-047`.
  The stale TP-MAC-189 path remains bypassed, not passed.
- **Later stages** follow the completion plan: R5 (engineering beta and
  release machinery).
- Stage advancement is human-ruled: agents propose with evidence; only a
  human-approved coordination update advances the target stage recorded in
  `_COORDINATION.md`. If `D-15` is not yet ruled, the current Phase D D9
  readiness packet is
  [VERIFICATION_2026-06-21_r4_exit_gap.md](../plans/VERIFICATION_2026-06-21_r4_exit_gap.md),
  which records **not ready** rather than R4 closure. The next unblocked
  dependency-spine item is `D-17` sparse live-path decision-packet preparation
  or the next unblocked D6/D9 nonlinear residual under `DEC-044` and
  `DEC-046`, unless superseded by a later human-approved tranche.

## 5. Non-goals and boundary prohibitions

The headline prohibitions, in full, live in the authorities — this map only
names them and routes:

- **Code-neutral boundary:** no populated code tables, no protected standards
  constants or copied formulas; all component factors are user-entered
  ([INTENT.md](INTENT.md) §Non-goals, [IP_AND_DATA_BOUNDARY.md](IP_AND_DATA_BOUNDARY.md), [CONTRACT.md](CONTRACT.md)).
- **No professional/reliance claims:** the software computes user-defined
  checks; it never makes a compliance, certification, sealing, approval,
  authentication, or code-compliance claim. Human acceptance stays external and
  hash-bound ([PROFESSIONAL_BOUNDARY.md](PROFESSIONAL_BOUNDARY.md), CONTRACT).
- **Local-first, private-by-default:** no cloud, daemon, network, telemetry, or
  repository-default private-data writes; user-created models stay in local
  project storage and are never committed; bundled examples are invented
  (`_COORDINATION.md` Working Desktop Application Standard, IP boundary).
- **No silent defaults:** unknown values are explicit `TBD`, surfaced not
  guessed (CONTRACT).

## 6. Top risks to the roadmap

| Risk | Mitigation |
|---|---|
| D-01 (units) stalls and silently re-blocks R2 value | Led with the decision-prep tranche (**RULED** `DEC-018`); Phase A proceeds SI-labeled, so any stall is visible, not hidden |
| Edit-loop (A1–A3) underestimated — it is the product hinge | Smallest-possible tranche slicing; Playwright (A8) lands early against each increment, not at phase end |
| Nonlinear iteration (D6) convergence quality | Validation-first: D8 cases drafted alongside D6; existing diagnostics taxonomy reused, per PRD §24 mitigation row |
| GUI test debt compounds | A8 makes the R2 journey itself the regression spine before C/D add surface area |
| Release machinery decisions (D-05..07) deferred until they block R5 | Decision register makes them explicit mid-plan items with RGAP traceability, not end-loaded surprises |
| Scope drift via app tranches crossing deliverable boundaries | Coordination loop rule retained: tranches name the app-owned slice and allowed write targets; register hygiene in completion-plan F3 |

## 7. Tranche selection

This map does not select tranches. The active completion plan
[plans/PLAN_2026-06-17_prd_completion.md](../plans/PLAN_2026-06-17_prd_completion.md)
holds the phase tables, dependency spine, FR completion map, and decision
register that drive tranche selection under the `_COORDINATION.md` loop. When a
successor completion plan is accepted, update the pointer here and in
`_COORDINATION.md`.
