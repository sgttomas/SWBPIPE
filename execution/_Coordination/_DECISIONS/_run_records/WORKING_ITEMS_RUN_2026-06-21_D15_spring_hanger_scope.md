# WORKING_ITEMS Run Record - D-15 Spring-Hanger Scope Packet

- Date: 2026-06-21
- Agent: WORKING_ITEMS (Type 1 persona)
- Tranche: TP-DECIDE-D15-SPRINGHANGER-001
- Repo HEAD at preparation: `79338fab3e`

## Trigger

The human project authority approved `D-23` Option O-A: accept the R3 exit
evidence and advance the current target stage to R4. That ruling was already
recorded as `DEC-048` in `execution/_Decomposition/SOFTWARE_DECOMP.md`, with
the decision register, `docs/PLAN.md`, `_COORDINATION.md`, and
`NEXT_INSTANCE_PROMPT.md` already showing R4 as the current target stage.

The next Phase D item after landed D1-D4 work is D5 spring hangers. D5 was
gated by unprepared decision `D-15`, so this run executed the coordination
loop's decision-escalation step.

## Scope

Prepared a PROPOSAL-only decision packet for:

- whether existing generic `spring` support behavior is sufficient for the R4
  spring-hanger deliverable;
- whether D5 requires dedicated variable spring hanger and constant-effort
  support data contracts;
- what boundaries prevent catalog sizing, protected standards defaults, and
  professional/code-compliance claims.

No implementation, schema, solver, UI, fixture, report, or release behavior
changed in this tranche.

## Evidence Sources Reviewed

- `docs/PRD.md` for R4 spring-hanger scope and support/restraint vocabulary.
- `docs/SPEC.md` for the current linear/nonlinear support boundaries.
- `docs/CONTRACT.md` for no-protected-data, no-default, and no-claim invariants.
- `schemas/model.schema.yaml` for current `Support.support_type` and generic
  support properties.
- `docs/_Registers/ScopeLedger.csv` and `docs/_Registers/Deliverables.csv` for
  DEL-04-03 / DEL-04-04 scope split.
- `core/solver/linear_supports/src/lib.rs` for current generic spring mechanics.
- `core/product_physics/src/lib.rs` and
  `core/product_physics/src/validation.rs` for preview support mapping and unit
  validation.
- `core/solver/nonlinear_supports/src/lib.rs` for the nonlinear classifier
  boundary.
- `fixtures/product_preview/invented_preview_model.json` for current invented
  support examples.
- `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-03_Linear support and restraint models/_CONTEXT.md`
  for DEL-04-03 local scope.

## Outputs

- Added `execution/_Coordination/_DECISIONS/D-15_spring_hanger_scope.md`.
- Updated `execution/_Coordination/_DECISIONS/_REGISTER.md` row D-15 from
  `NOT_PREPARED` to `AWAITING_RULING`.
- Updated `plans/PLAN_2026-06-17_prd_completion.md`, `docs/PLAN.md`,
  `execution/_Coordination/_COORDINATION.md`, and
  `execution/_Coordination/NEXT_INSTANCE_PROMPT.md` so they no longer instruct
  the next agent to prepare the packet again.

## Packet Recommendation

The packet recommends Option B: a minimal dedicated spring-hanger model for
user-entered variable spring hanger and constant-effort support
data/provenance. Generic `spring` alone is treated as insufficient unless the
human project authority explicitly rules otherwise.

This recommendation is not a ruling. D5 remains blocked until the human rules
`D-15`. If the ruling is still pending, the next unblocked Phase D
implementation item is D6 assembled nonlinear solve under `DEC-044` and
`DEC-046`.

## Validation

- `git diff --check` - passed.
- `rg -n "D-15|D-15_spring_hanger_scope|AWAITING_RULING|D5 is gated|next unblocked Phase D implementation item is D6|DEC-048|DEC-044|DEC-046" ...` - passed; expected references found.
- cited-file existence check over PRD, SPEC, CONTRACT, registers, schema, core
  crates, fixture, decomposition record, and DEL-04-03 context - passed.

## Boundary Review

Packet is `PROPOSAL` / `AWAITING_RULING` only. It creates no lifecycle
transition, release-readiness claim, professional approval, certification,
sealing, authentication, code-compliance acceptance, protected-data use, or
catalog/default value authorization.
