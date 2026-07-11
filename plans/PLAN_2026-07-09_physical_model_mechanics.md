# Physical-Model Mechanics Program — tactical plan

> **RETIRED AS A SELECTION SURFACE 2026-07-10** (owner-adopted consolidation; loop
> Receipt 9). P1–P4 executed (Receipts 5–6; PRs #143/#144/#145); the open residuals
> and the §5 completion criteria were rehomed into deliverable `_STATUS.md`
> `## Remaining` sections (primary: DEL-04-01, DEL-04-04, DEL-05-01, DEL-05-02,
> DEL-07-02). Do not select work from this file; it is retained as a dated record
> because `DEC-066`–`DEC-070` cite it by path.

> **Epistemic status: agent-authored PROPOSAL — non-governing.** Written
> 2026-07-09 at owner direction (verbatim in `loop/LOOP_RECEIPTS.md`
> Receipt 3; scope adoption codified as `DEC-066`–`DEC-069` in
> `execution/_Decomposition/SOFTWARE_DECOMP.md` §12). This plan routes to
> the authorities — the decision register, `_COORDINATION.md`, and the
> approved DAG — and never itself authorizes work. It carries no status;
> each loop iteration re-derives state from the live tree. Basis findings:
> `plans/ASSESSMENT_2026-07-09_physical_model_evaluation.md` (dated map).

## 1. Objective

Close the physical-model correctness gaps adopted by the owner on
2026-07-09: make the solve flexibility-correct for bends/branches
(implement, not disclose — CAND-1 of the assessment is superseded by
implementation), repair the nonlinear-support state logic, land the
workflow-physics items, and name the absent special domains in the PRD.
All work stays inside the standing fences (F-PIP-1..4): user-entered
values only, no code content, no defaults, no release/professional claims.

## 2. Phases (dependency spine)

| # | Scope | Gate / basis | Depends on |
|---|---|---|---|
| **P1** | **Bend/branch flexibility in the global stiffness** + expansion-loop thermal-bending benchmark with known-flexibility reference, k-sweep sanity evidence, provenance update superseding the "base frame stiffness unchanged" string | `D-34` (method fork, AWAITING_RULING; recommended O-A user-k-scaled bending on bend spans) | D-34 ruling |
| **P2** | **Active-set complementarity repair** (state-switched governing test) + **bounded ±μN Coulomb sliding force** + transition-case fixtures under the `DEC-046` policy convention | `D-35` ruled (`DEC-067`) | none — may run parallel to P1; merge order coordinated where both touch `nonlinear_integration` |
| **P3** | **Workflow physics**, three separable tranches: (a) modulus basis per load case with explicit range-combination basis; (b) seismic/wind static-equivalent load generation from user parameters and model mass/geometry; (c) mill-tolerance slot in section properties | `D-36` ruled (`DEC-068`) | (a) none; (b) none; (c) none — independent of P1/P2 |
| **P4** | **PRD naming of absent special domains** (docs-only) | `D-37` ruled (`DEC-069`) | none |

Suggested execution order: P1 first (largest value; starts on the D-34
ruling), P2 in parallel where write scopes stay disjoint, P3 tranches as
fill, P4 any time.

## 3. Evidence conventions (every tranche)

Per `_COORDINATION.md` validation requirements and the work class:

- New mechanics behavior lands with a hand-calc witness in
  `validation/hand_calcs/` plus a benchmark-crate fixture; tolerances per
  the `DEC-024`/`DEC-026` class-tiered policy, tighten-only.
- Nonlinear threshold changes enter as governed `DEC-046`-convention
  policy records.
- Dense/sparse parity oracle green on touched solve paths; repo-wide
  `self-check` exit 0; full practitioner-harness pytest at closeout;
  `DEC-025` five-surface sweep on a clean head for code tranches.
- Branch-first + PR; owner merges; write scopes inside
  `projects/chirality-piping/**`.

## 4. Interaction with existing authorities

- `DEC-045` remains immutable history; `D-34` carries the partial
  supersession for the solve path. The SIF stress-review multiplier and
  provenance conventions continue.
- Phase E (PRD completion plan) continues in parallel; where an E2
  validation-manual case would exercise a bend, prefer landing P1 first so
  published examples are flexibility-correct.
- `D-12` (dynamics/FEA-export disposition) is untouched by this program.
- Curved-bend macro-element (D-34 §4.2 O-B) and friction path-history are
  the named follow-on triggers, each requiring a new `D-XX`.

## 5. Completion

The program is complete when P1–P4 evidence exists per §3 and the
assessment's gap rows G1, G2, G4 (and the M2/M3 methods defects) are
closed or explicitly re-dispositioned by owner ruling. Completion is an
evidence state, not a release or lifecycle claim.
