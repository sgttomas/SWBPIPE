# Remaining physics evaluation report

## Basis

This bounded evaluation used source commit `533332349a4607eee561d4ef90fb05a62d86519e`, the frozen P0 launch controls, and the Owner's engineering-direction V2. The V2 direction authorizes a defensible engineering selection with assumptions and refutation instead of leaving every modelling choice as an Owner hold.

Accepted 2026-09-08 pressure four-case, canonical-design, and mixed-reference packages were treated as candidate evidence. They were not treated as implementation adoption. No extracted equation under `domains/piping-design/` was used as authority, and historical N7 was not reopened.

The evaluation criterion was whether current behavior is mechanically admissible and internally coherent under its declared small-displacement/static scope, whether a gap is a current repair or a new contract/capability, and whether the proposed behavior has a bounded independent refutation plan. No scoring rubric was accepted, so no score is produced.

## Method

EVALUATION P0 ran the frozen protocol and dispatched two nondelegating Agent 2 generalists at the required `gpt-5.6-sol` / `high` configuration:

- NS reviewed nonlinear supports, active-set integration, numerical scaling, and diagnostics.
- PCB reviewed pressure, connectors, the physical-to-analytical bridge, and PKG-09 coverage.

Evidence collection used focused source trace, exact arithmetic probes, narrowly selected current tests, and primary technical references. One root-coordinated Rust slot executed the NS-01 current-source probe once. The manager reran only `tests/test_physical_to_analytical_transform.py` with the supplied qualified Python after PCB's system Python lacked `jsonschema`; it passed with exit 0. No blanket test, full build, GUI run, or external industry-solver comparison was performed.

After fan-in, root-routed independent reviewers challenged the provisional friction, endpoint, and pressure designs. Their refinements are incorporated below and indexed in `POST_FAN_IN_REFINEMENTS.md`. The frozen provisional packets remain historical inputs; they are not the current unqualified implementation contracts.

## Coverage

The assessment covered the remaining concerns in PKG-04/05/09/13 that could materially affect the first useful physics tranche: static friction branch consistency, contact initialization, solver scaling and pivots, endpoint stress signs, straight pressure semantics, finite-end connector kinematics, canonical bridge status, and independent oracle coverage.

It did not audit the full application, code compliance, allowables, pressure gradients, bend ovalization or pressure stiffening, plasticity, large displacement, manufacturer-specific nonlinear connector laws, general contact subset enumeration, or every deliverable statement. Curved endpoint stress was inspected only enough to fence the required tangent-frame proof.

## Validated-return inventory

| Return | Fan-in verdict | Material evidence |
|---|---|---|
| `returns/NS/` | PASS after one schema correction requested by P0; source basis/model/effort/write scope verified | Four solver findings plus exact algebraic probes and an exit-0 current-SHA Rust reproduction of seed-dependent assisting friction. |
| `returns/PCB/` | PASS; source basis/model/effort/write scope and ten-column register verified | Five pressure/connector/bridge/verification findings; focused adapter test passed and fail-closed transform behavior reproduced. |

PCB's full transform test originally stopped only because system Python lacked `jsonschema>=4,<5`. P0 used the launch-provided qualified environment and obtained exit 0. This resolves the environment-only test gap; it does not turn the Python adapter into a production runtime bridge.

## Findings

The authoritative manager register is `FINDINGS.csv`.

1. **P0-001 — reproduced high defect:** static no-history Coulomb behavior is seed dependent. The current-SHA executable returned `u=0`, sticking, for a sticking seed and `u=-0.02`, sliding with `T=-3` and `T*u=+0.06`, for a sliding seed under the same sub-limit load. The independent review accepts the static model after corrections: use `F_t*u_t <= 0`; require strict opposition only for positive limit and nonzero motion; retain zero-limit frictionless and no-contact exceptions; identify exact-boundary sticking as an application convention; update simultaneous inconsistent rows together; preserve PR 760 current-normal coupling and honest nonconvergence.
2. **P0-002 — reproduced high defect:** endpoint stress consumes raw element actions while station stress consumes a common section cut. The independent implementation brief selects existing `straight_section_resultants` at fractions 0/1, and the curved arc-equilibrium helper at 0/1, before stress recovery. Raw action rows remain unchanged. Endpoint and curved-station metadata use existing canonical enums, with exact frame/equilibrium meaning in `sign_convention`.
3. **P0-003 — reproduced high defect:** the finite-end expansion-joint element strains under rigid rotation. The energy-derived two-frame small-displacement connector in `PROVISIONAL_OBJECTIVE_CONNECTOR_RECOMMENDATION_V1.md` is a future design recommendation requiring independent refutation before source selection. It requires explicit frames, offsets, installed reference, stiffness basis, and topology.
4. **P0-004 — high engineering implementation gap:** straight pressure results lack the information and mechanics needed to distinguish wall force, effective force, closure transfer, and Poisson-coupled strain. Independent refutation confirms the exact-annulus/four-case mathematics but rejects the provisional runtime contract. A current source tranche is not selected. First freeze a pure exact kernel; then redesign the runtime contract around authoritative `(E,nu)` with derived `G`, connected pressure regions and terminal transfer, a real `legacy_pressure_v1` mode, the live desktop/headless DTO and migration paths, and every typed-result consumer.
5. **P0-005 — medium bounded capability defect:** a singular inactive first contact trial can prevent discovery of an admissible active state. Select one deterministic all-contact recovery trial for the declared bounded class; do not add artificial stiffness or claim general enumeration.
6. **P0-006 — medium numerical/diagnostic defect:** absolute pivot guards reject well-conditioned scaled systems, while nonpositive sparse pivots remain metadata. Select symmetric equilibration, positive-definite factorization, original-system backward residuals, and blocking product diagnostics.
7. **P0-007 — medium capability gap:** the transform is honestly fail-closed but is not an end-to-end product solver bridge. Add a version-negotiated mapped-or-blocked route only after pressure and connector meanings stabilize.
8. **P0-008 — medium verification gap:** independent pre-implementation oracles and later industry-solver comparison remain required.

## Conflicts / unknowns

- The endpoint straight transform is settled; the curved endpoint tangent-frame transform still needs independent derivation. Curved endpoint stress should be withheld if that proof is not in the repair.
- The connector recommendation is coherent for small displacement but has not yet received the independent design refutation required before source selection. Manufacturer data may use different reference planes, coupling, restraints, or temperature basis; the new contract must carry those authored meanings and cannot infer missing provenance.
- The exact straight-pressure mathematics survived independent refutation, while its provisional runtime contract did not. It also does not extend to curved pressure coupling, ovalization, pressure stiffening, gradients, expansion-joint pressure interaction, or end-discontinuity stress.
- Exact public version numbers, deprecation duration, and release timing remain workflow/accountability choices. The technical migration rule is new writer/new version, visible legacy replay, and fail-closed unknown versions.
- Numerical warning thresholds remain unset. Arithmetic correctness can be repaired using scale-relative factorization and raw residuals without inventing a universal engineering acceptance threshold.
- No external industry-solver validation has occurred.

## Recommendations

1. Implement P0-001 under the independently refined FREFUTE contract and P0-002 under the PREVIEW endpoint brief and its metadata/validation refinements. Their source fences are disjoint and both correct current physical results without requiring broader pressure or connector public contracts.
2. Independently refute P0-003 before selecting source effect. If accepted, first add the objective connector primitive and rigid-mode/energy proofs; then add the explicit schema and adapter; keep ambiguous finite-end legacy expansion-joint consumption fail-closed.
3. Implement P0-005 as a bounded contact-initialization recovery with a visible diagnostic.
4. Revise P0-004 under the pressure refutation's `CHANGES_REQUIRED` verdict. Freeze a private exact kernel first. Select runtime source only after the `(E,nu)->G` material rule, region-terminal topology, legacy mode, live DTO/migration fence, and typed-result consumers form one coherent contract.
5. Implement P0-006 as one numerical-policy tranche so factorization and product diagnostics cannot diverge.
6. Implement P0-007 only after the pressure and connector schemas freeze; keep every omission and version mismatch fail-closed.
7. For every tranche, freeze exact PKG-09 expectations before source edits, use focused dense/sparse tests where applicable, perform independent physics review, and reserve the industry-solver comparison for the declared later validation phase.

## Decision queue

The Owner's V2 direction resolves the engineering-selection authority. The current source-ready contracts are the independently refined static-friction and endpoint-cut repairs. Contact recovery and numerical factorization are bounded evaluation recommendations for later independent review. Connector and pressure remain future design work: connector awaits refutation, and pressure refutation requires a revised runtime contract despite accepting its exact mathematics.

The remaining queue for HELP_HUMAN/WORKING_ITEMS is execution sequencing, public version identifiers and deprecation timing, and authorization of the later external-solver validation matrix. No further engineering-choice approval is required merely to prepare or independently review the bounded implementations.

## Handoff summary

The evaluation is complete with three reproduced high mechanics defects, one high engineering implementation gap, three medium capability/numerical gaps, and one medium verification gap. P0-001 and P0-002 now point to independently refined source-ready contracts. Connector remains design-only pending refutation; pressure mathematics is accepted but its runtime contract is `CHANGES_REQUIRED`. `HANDOFF.md` records the owners, blockers, and rerun requirements.
