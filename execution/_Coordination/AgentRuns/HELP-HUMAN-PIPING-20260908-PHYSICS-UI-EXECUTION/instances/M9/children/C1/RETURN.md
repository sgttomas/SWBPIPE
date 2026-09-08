# M9-C1 return — independent component derivation/review

Status: `COMPLETE_WITH_EXPLICIT_BRANCH_BLOCKERS`
Disposition: `DERIVATIVE_CANDIDATE_NOT_ACCEPTED_AUTHORITY`
Parent: `M9` (`WORKING_ITEMS`, Agent 1)
Role: ephemeral Agent 2; role mechanically unenforced and instruction asserted
Delegation: none; non-delegation is instruction/config asserted
Configured execution reported by parent dispatch: `gpt-5.6-sol`, reasoning `high`; no deeper runtime self-introspection asserted
Branch/base: `codex/piping-physics-ui-execution-20260908` at `779dedb8670625b36af07b89fc5557470e47c50e`

## Result

The independent frame-plus-retained-zero-reference-spring branch is complete for both pressure-free fixture load cases. It includes the full 3D Euler-Bernoulli element stiffness and transformation, consistent uniform-load vector, thermal initial-strain vector, rigid and one-way supports, spring action, local element-end and five-station cut vectors, separate global force and moment balances, and branch-explicit current-normal Coulomb friction.

Under the explicitly stated interpretation that the one-way support blocks positive N-140 UY motion, its active branch is admissible in both cases. The open controls move in positive UY and violate that gap side. Zero-reference stick is inadmissible in both cases. Each case has exactly one admissible current-normal sliding branch:

| Quantity | L-100 pressure-free | L-200 pressure-free |
|---|---:|---:|
| N-130 UZ | -0.00664392679739249 m | -0.00332196339869625 m |
| S-130 UY action | -52.3732819873299 N | -26.1866409936650 N |
| Current normal magnitude | 52.3732819873299 N | 26.1866409936650 N |
| Friction UZ action | +0.523732819873299 N | +0.261866409936650 N |
| NL-140 UY action | -297.626718012670 N | -98.8133590063350 N |
| SH-140 UZ action | +279.008518427111 N | +139.504259213555 N |

The L-100 thermal primitive produces the expected free P-120 axial growth `0.00066 m`. Raw global force residuals are `[0,0,-2.60644e-10] N` and `[0,0,-1.30152e-10] N`; raw global moment residuals about origin are `[-6.24569e-10,1.97414e-9,0] N m` and `[-3.12000e-10,9.86162e-10,0] N m`. These are observations, not adopted comparison tolerances.

## Method and independence

`derive.py` parses only the declared input fixture and assembles the reference from NumPy primitives. It does not import or inspect product solver code. Equations and sign conventions are in `DERIVATION.md`; the machine-readable full vectors and every accepted/rejected branch are in `EXPECTED_RESULTS.json`.

No product-generated output, bundled expected-result constant, manager calculation/result, or production source was read before or after freeze. No `domains/piping-design` OCR/extracted equation was used. The E1 friction brief and Q1 report were used only for accepted scope, current-normal/static-history forks, dependency posture, and claim limits. Public references were limited to OpenSees documentation for 3D elastic-frame parameterization/transformation/uniform-load context and COMSOL documentation already cited by E1 for the static-versus-incremental friction distinction:

- https://opensees.github.io/OpenSeesDocumentation/user/manual/model/elements/elasticBeamColumn.html
- https://opensees.github.io/OpenSeesDocumentation/user/manual/model/geomTransf/Linear.html
- https://opensees.github.io/OpenSeesDocumentation/user/manual/model/pattern/PlainPatternloadcommands/eleLoad.html
- https://doc.comsol.com/6.4/doc/com.comsol.help.sme/sme_ug_theory.06.088.html

## Exact blockers/forks

1. `C-150` cannot be inserted uniquely: no second connector node, rigid offset/insertion length, or series/replacement/parallel rule is supplied. The component-stiffness-inclusive matrix and vectors therefore require a physical/topology choice.
2. `SH-140` has no stated force-free/reference displacement and does not identify installed/cold/hot load as preload. The completed numerical branch is explicitly stiffness-only with zero displacement reference.
3. `NL-130-FRIC.initial_state=sliding` supplies no committed slip displacement or load-step history. The completed branch is the accepted current-normal, static zero-reference interpretation; incremental/history results remain undefined.
4. `NL-140.active_when=negative_reaction` fixes the active reaction sign but does not expressly define the allowed displacement side. Both active and open solutions are returned, with gap feasibility stated under the block-positive-UY interpretation.
5. No accepted tolerance exists. No new threshold was invented.

These blockers stop only the dependent component/preload/history/opposite-gap branches. They do not block the completed frictionless controls or current-normal zero-reference results.

## Files and hashes

Written only under the sealed target:

| File | SHA-256 |
|---|---|
| `derive.py` | `2f1775aa450bcf3103f7c7fc303590d4dac76b8db84d3cbf4b6969dea9b9c55c` |
| `DERIVATION.md` | `df853f1181becf888f0b5dccf50ffa3cf4b76b0e6820cc6e89c0cf0def0c0e7d` |
| `EXPECTED_RESULTS.json` | `da9a142548a2660e6637aef8588295a469d2218fa0115b0e02252e073ef704e8` |

Key input hashes:

| Input | SHA-256 |
|---|---|
| fixture | `986c055944776ca8d0d849d552678e1f2bfb6c4559bbb9ff8a1928157a4f871c` |
| M9 activation | `577431f8ca2d2521d322b0db93ffa65bb1b2e8aaffdc4c548ace767676b6c416` |
| M9 work graph | `30c08355d4ca8d6e6ebe4df0c0780504fe68e18f89839c67c87a3bc28129ba56` |
| C1 sealed brief | `4a460472a3ea7efb4b2df891d5f1109d66189c2c25ae12df49e89e1847edc2e1` |
| E1 friction decision brief | `79fdd9e18d42a0f2594131030cdc3b9263822583d0e44beb80b0473261dd87fb` |
| Q1 report | `cd5b067dd7cfce88dcc7f678885cdbeb7aac2f28faa1e7395fd1baefdd1153ba` |
| DEL-09-01 context / SOW / status | `dad32d9c...` / `2f5a13ab...` / `59897592...` |
| project contract | `60cd0b5893a7b218a5f75c22d480bf03cb4f93c9134f118adad6c0dcc3960fd3` |

## Validation

- Default system Python correctly failed before execution because NumPy was unavailable: `ModuleNotFoundError: No module named 'numpy'`.
- The bundled workspace Python with NumPy compiled and ran `derive.py` successfully; host resolution is recorded only in the manager's structural execution record.
- JSON schema/key and branch assertions passed: two pressure-free cases, one admissible current-normal branch per case, both active stick trials inadmissible, and five station records per element.
- Assembled global stiffness symmetry maximum absolute difference: `0.0`.
- Station endpoint identity maximum absolute difference (`C(0)=-a_i`, `C(L)=a_j`): `3.183231456205249e-12` in N/N m units.
- Free-DOF equilibrium and separate global force/moment closure residuals are recorded for every branch in the JSON.
- `jq` parsed `EXPECTED_RESULTS.json`; output size is 380411 bytes.

No pass/fail tolerance was selected or implied by those raw numerical observations.

## Recommendation to M9

Use the unique admissible `admissible_current_normal_zero_reference_branches[0]` result for each load case as the frozen independent comparison branch, while preserving all four blockers in the manager handoff. Compare production only after binding these exact bytes/hashes. Report absolute and relative errors without pass/fail classification until an authorized tolerance exists. Do not treat agreement with this branch as validation of expansion-joint topology, spring preload, friction history, the opposite unilateral gap side, lifecycle closure, or engineering acceptance.
