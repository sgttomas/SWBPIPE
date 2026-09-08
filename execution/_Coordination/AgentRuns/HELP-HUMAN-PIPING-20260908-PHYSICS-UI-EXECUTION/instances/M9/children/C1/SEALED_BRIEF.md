# M9-C1 sealed brief — independent component derivation/review

RequestedBy: M9 (`WORKING_ITEMS`, Agent 1)
RunID: `HELP-HUMAN-PIPING-20260908-PHYSICS-UI-EXECUTION`
ParentInstanceID: `M9`
ChildInstanceID: `M9-C1` (ephemeral Agent 2; role and non-delegation instruction/config asserted)
PackageID: `PKG-09`
DeliverableID: `DEL-09-01`
Configured execution: `gpt-5.6-sol`, reasoning `high`

## Objective

Independently derive and review the mechanics formulation for the exact pressure-free actual fixture identified in M9's activation. Supply the 3D Euler-Bernoulli frame stiffness/transform, consistent uniform distributed-load vector and station-cut recovery, retained ground-spring action, rigid and unilateral support actions, global force and moment balances, gap/complementarity conditions, and a branch-explicit current-normal Coulomb friction solve/check. Freeze equations, signs, inputs and expected quantities before reading any product output. You may calculate independently but must not edit the manager's calculation or compare to production.

## Declared reads

- `instances/M9/SEALED_ACTIVATION.md` and `WORK_GRAPH_V1.md`
- root/project governance and DEL-09-01 live context/SOW/status
- `fixtures/product_preview/invented_preview_model.json` as the actual input source
- accepted E1 friction decision brief and Q1 report for scope/forks only
- primary/public mechanics references; no `domains/piping-design` OCR/extracted equations

Do not read production-generated numerical outputs, fixture expected-result constants, or manager-authored results. Production source may be consulted only to identify field meaning after you have frozen the independent derivation; if consulted, list the exact purpose and preserve independence.

## Allowed tools and writes

Read/search, Python 3/NumPy scratch calculation, deterministic shell checks, and web access for primary/public references. No delegation. Write only `RUN_ROOT/instances/M9/children/C1/**`.

## Expected return

- `DERIVATION.md`: equations, coordinates/DOF order, signs, case branches, and applicability.
- `EXPECTED_RESULTS.json`: signed expected vectors and independent residual checks for the frictionless controls and the actual pressure-free composition, or an exact blocker/fork where a unique answer needs an unmade physical choice.
- `RETURN.md`: methods, reference URLs, files/hashes, validation, blockers, and recommendation to M9.

## Acceptance and escalation

All expected values trace to the input fixture plus independently stated equations. Force and moment balance are separate. Friction uses the same returned normal reaction and reports stick/slip admissibility; history/reference ambiguity stays explicit. Do not invent a tolerance or acceptance threshold. Stop only the dependent branch if input semantics require a new Owner choice; finish all independent frictionless/reference branches.
