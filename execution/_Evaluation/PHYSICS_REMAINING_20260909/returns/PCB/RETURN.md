# PCB remaining-physics return

## Execution identity and conclusion

- Role: bounded ephemeral Agent 2 generalist; no delegation performed.
- Requested and exposed configuration: `gpt-5.6-sol`, reasoning `high`.
- Source basis: `533332349a4607eee561d4ef90fb05a62d86519e`.
- Scope: pressure, finite-end connector mechanics, the physical-to-analytical solver bridge, and the corresponding PKG-09 verification boundary.
- Authority boundary: this is derivative engineering evidence. It recommends modelling choices under the Owner's V2 authority amendment, but it does not adopt a public contract, alter governed state, or claim comparison with an industry solver.

Two current behaviors are reproducible implementation defects. Endpoint stress recovery consumes raw i-end actions without the section-cut sign transform already used by station recovery. The finite-length expansion-joint stiffness mapping penalizes a rigid rotation and produces false energy and an uncancelled force couple. The physical-to-analytical transform is honest and fail-closed for an expansion joint, so that omission is a capability and contract gap rather than silent incorrect transformation. No product runtime consumes the Python solver-boundary adapter.

## PCB-001 — endpoint actions are not converted to a common section cut before stress recovery

**Classification:** bounded source/test repair under already accepted action and station meanings. **Severity:** high.

Current source preserves raw element actions for endpoint force rows and a common j-side cut for station rows, but feeds both raw endpoint actions directly into stress recovery:

- `core/product_physics/src/lib.rs:1561-1568` obtains mechanical end actions and applies axial-effect corrections.
- `:6847-6865` adds the equivalent pressure/thermal axial magnitude at i and subtracts it at j.
- `:6325-6353` explicitly negates all i-side components to publish the station's j-side cut.
- `:7176-7255` explicitly describes endpoint force rows as raw i/j local DOF-vector actions.
- `:7475-7504` passes each raw endpoint action directly to `ForceResultants`; station recovery at `:7506-7534` receives the already normalized cut.
- The fixed-pressure test at `:14543-14581` checks only i-end stress and omits endpoint/station parity.

For the existing fixed/fixed pressure case, zero displacement gives raw corrected axial actions

```text
[Fi,Fj] = [+p Ai,-p Ai].
```

The current endpoint stresses are `[+pAi/As,-pAi/As]`, while the limiting station cut is `-Fi=-pAi` and the station stress is `-pAi/As`. With the existing invented geometry at `p=1 MPa`, the focused arithmetic replay gave `+5.26086956522 MPa` at i, `-5.26086956522 MPa` at j, and `-5.26086956522 MPa` at the station. A uniform member state cannot have an endpoint sign discontinuity of this kind. The same proof applies without pressure: for a self-equilibrated raw action pair `[-N,+N]`, tension-positive common cuts are `Ncut_i=-Fi=+N` and `Ncut_j=+Fj=+N`.

**Selected correction.** Keep the raw endpoint force/action rows unchanged. Introduce a pure endpoint-action-to-section-cut transform before stress recovery. For a straight element in its current local basis, negate all six i-end generalized components and retain all six j-end components. Describe endpoint stress rows as section-cut quantities. For a curved macro, first rotate each chord-frame end action into its endpoint tangent section frame, then apply the cut-face sign. If that curved transform is not proved in the bounded repair, withhold curved endpoint stress rows with a blocking diagnostic; a chord-frame sign flip alone is insufficient.

This repair is independent of the broader pressure constitutive selection below and does not require a change in `core/loads/stress_recovery`.

## PCB-002 — select an exact straight-pipe pressure reference and version its meanings

**Classification:** engineering/reference selection plus public contract and migration work. **Severity:** high.

The current pressure implementation uses the physical bore thrust `pAi` (`product_physics:6610-6647`), applies a closed-end nodal pair (`:6714-6741`), subtracts it in straight force recovery (`:6847-6865`), suppresses the separate longitudinal pressure row whenever thrust is active (`:1660-1666`), and otherwise has only the thin mean-radius membrane formulas `pr/t` and `pr/(2t)` (`core/loads/stress_recovery/src/lib.rs:926-956`). It has no explicit Poisson ratio or closure-transfer topology. Straight recovery is therefore effective-force-like, while the curved pressure path documents a wall-tension result (`product_physics:6744-6753`). Simply enabling the longitudinal row would mix an approximate mean-radius stress with exact bore thrust and can double count axial pressure stress.

The smallest defensible complete straight-pipe choice is the exact circular-annulus, homogeneous-isotropic, small-strain generalized-plane-strain reduction, limited to uniform pressure and sections away from end discontinuities:

```text
Ai = pi ri^2                         As = pi(ro^2-ri^2)
Pc = pi Ai - pe Ae                   A = Pc/As
sigma_z = E(epsilon_z-alpha DeltaT) + 2 nu A
Nw = E As(epsilon_z-alpha DeltaT) + 2 nu Pc
S = Nw + pe Ae - pi Ai
```

`Nw` is the tension-positive axial resultant carried by the wall. `S` is a separately typed effective axial force. Axial wall stress is `Nw/As` exactly once. Raw endpoint actions, section cuts, `Nw`, and `S` must remain different typed quantities. Actual bore and annulus areas are already available; no thin-wall applicability threshold is needed for this bounded exact reference.

Require an authored dimensionless, temperature-aware `poisson_ratio`. An `E/G` inference is allowed only under an explicitly selected homogeneous-isotropic compatibility rule with a visible precedence/consistency diagnostic. Require pressure topology that distinguishes `closed_transferred` from `separate_closure`; a vented physical terminal is not represented by relabeling a uniformly pressurized barrel. Keep expansion-joint effective area as a separate load-path input and never reuse it as pipe-wall membrane area.

The exact-annulus choice is supported by the [MIT 22.314 structural mechanics note L.4](https://ocw.mit.edu/courses/22-314j-structural-mechanics-in-nuclear-power-technology-fall-2006/137e6469e37e9d347b7b3b69292da2f3_l4_2.pdf) (accessed 2026-09-09). The [Abaqus pipe-pressure documentation](https://docs.software.vt.edu/abaqusv2025/English/SIMACAEPRCRefMap/simaprc-c-loaddistributed.htm) independently confirms the need to distinguish closed-end loading and explicit open-end compensation, and its [beam section-force documentation](https://docs.software.vt.edu/abaqusv2025/English/SIMACAEELMRefMap/simaelm-r-beamlibrary.htm) independently distinguishes effective axial force (both accessed 2026-09-09). These sources support the mechanics and terminology; they are not evidence that Chirality has passed another solver's validation.

**Compatibility selection.** Add a new contract version with typed rows for `wall_axial_force`, `effective_axial_force`, `element_end_action`, and `section_cut`, plus exact inner/outer radial and hoop stresses and wall-average axial stress. Preserve legacy generic rows only as deprecated rows with their old basis stated. Existing documents may migrate to an explicit `closed_transferred_legacy_inference` with a visible assumption; newly authored pressure loads must state topology. Unknown versions must block instead of invoking heuristic fallback.

Curved-pipe Poisson coupling, pressure stiffening, ovalization, expansion-joint pressure interaction, large displacement, plasticity, pressure gradients, and code allowables remain outside this straight-pipe tranche and require separate proof.

## PCB-003 — finite-end expansion-joint stiffness is nonobjective

**Classification:** reproduced mechanics defect plus schema/compatibility repair. **Severity:** high.

`core/solver/frame_kernel/src/lib.rs:606-678` defines a finite-end `UserStiffnessElement`. Its matrix at `:1018-1039` applies six uncoupled springs to raw endpoint translation and rotation differences. The product maps an expansion joint to the two ends of a referenced pipe and infers the remote end (`core/product_physics/src/lib.rs:3538-3631`). Under a small rigid rotation, the endpoint translations differ by the rotation cross the finite chord, while the equal rotations produce no rotational spring action. The connector therefore resists rigid motion.

For a 2 m x-directed connector, `k_lateral=2000 N/m`, and rigid z-rotation `epsilon=0.001`, the focused replay gives raw `Delta uy=0.002 m`, force `4 N`, energy `0.004 J`, and an uncancelled `8 N*m` force couple. The objective relative displacement is zero. The accepted-candidate M9 mixed reference independently reports a `658.4367607916422 N*m` uncancelled literal-adapter connector couple; it labels that value diagnostic rather than a complete physical oracle.

**Selected formulation.** Adopt a symmetric, small-displacement, energy-derived two-frame connector. Declare nodal reference positions `xi,xj`; attachment offsets `ai,aj` in each node's initial frame and their global forms `ai_g,aj_g`; initial attachment points `pi0=xi+ai_g`, `pj0=xj+aj_g`; `r0=pj0-pi0`; and a right-handed initial connector triad `Q0` whose first axis follows the declared i-to-j reference axis. For nodal DOFs `d=[ui,theta_i,uj,theta_j]`, use

```text
di = ui + theta_i x ai_g
dj = uj + theta_j x aj_g
theta_c = (theta_i + theta_j)/2
q_t = Q0^T[(dj-di) - theta_c x r0]
q_r = Q0^T(theta_j-theta_i)
q = [q_t;q_r].
```

With `S(r)v=r x v`, the exact small-displacement operator is

```text
B_t = Q0^T[-I, S(ai_g)+0.5S(r0), I, -S(aj_g)+0.5S(r0)]
B_r = Q0^T[ 0,              -I, 0,                 I].
```

For installed zero-force deformation `q0` and a symmetric positive-semidefinite connector matrix `Kc`, define

```text
Pi = 0.5 (q-q0)^T Kc (q-q0)
f  = B^T Kc (q-q0)
Ke = B^T Kc B.
```

This construction gives all six infinitesimal rigid modes zero deformation, energy, and action; end forces and lever-arm moments follow from virtual work. The symmetric midpoint rotation is selected because it treats the two attachment frames equally and passes endpoint reversal when the frame is canonicalized. It is limited to small rotations; a later large-rotation connector would need a corotational finite-rotation contract.

The new schema must declare both attachment refs/offsets, units, `Q0` and handedness, end order, `q0`, installed reference temperature, `Kc` component order and mixed block units, provenance/measurement restraints, validity ranges, and topology (`replaces_span`, `series_between_end_planes`, or explicit parallel). A diagonal `Kc=diag(k_ax,k_lat,k_lat,k_tor,k_ang,k_ang)` is valid only in a new explicit isotropic-uncoupled mode. Full 6x6 coupling must be available because manufacturer/reference-plane definitions can couple degrees of freedom. The current four numbers must not be silently reinterpreted.

The [Abaqus connector-element documentation](https://docs.software.vt.edu/abaqusv2025/English/SIMACAEELMRefMap/simaelm-c-connectiontypeuse.htm) (accessed 2026-09-09) supports the requirements that connector relative motions and forces/moments be work-conjugate, that orientations corotate, and that reference lengths/angles define the zero-force state. The formulation above is project-selected mechanics, not copied vendor behavior.

Until the new mode exists, product consumption of finite-length expansion joints should fail closed with a specific diagnostic. If the current element is retained, name it as an external-frame relative spring and require explicit selection; do not present it as physical expansion-joint mechanics.

## PCB-004 — the physical-to-analytical path is fail-closed but not an end-to-end solver bridge

**Classification:** capability, versioning, and public-contract gap; no silent transform defect reproduced. **Severity:** high.

`core/model_transform/physical_to_analytical/contract.py:113-121` promises deterministic omission with diagnostics. Component support at `:16-23` excludes expansion joints; `:336-389` emits `PTA-COMPONENT-TYPE-UNSUPPORTED`; and `:392-453` blocks the linked element. A focused mutation of the canonical fixture to an expansion-joint `component_link` emitted no analytical component or element, emitted the expected component/element/target diagnostics, and returned `has_blocking_findings=true`. This is honest failure, not an incorrect expansion-joint solve.

The downstream adapter states that it is not a public API or runtime (`_solver_boundary_adapter.py:1-6`), has contract ID 0.1 (`:18`), accepts only straight pipes (`:312-351`), only binds `E`, `G`, area and basic section values (`:63-75,474-527`), and supports only nodal forces/moments, element point forces, and uniform distributed forces (`:22-61`). Focused repository search found adapter use in tests/evidence only, not in the Rust product runtime. The live deliverable itself records GUI/runtime/API integration as `TBD` (`ScopeOfWork.md:123-141`) and remains `IN_PROGRESS` with broader mechanics suitability and runtime result-envelope work held (`_STATUS.md:1-9`).

**Selected bridge contract.** Preserve the canonical physical model as the editable source. Add a new immutable transform version and a matching solver-adapter version. Every source record must produce either a typed analytical/solver DTO with source record ID, canonical SI value, original unit/provenance, source-model state hash, transform version, and trace chain, or a blocking typed disposition. The solver command must reject blockers and unknown version pairs before invoking physics. Carry the selected pressure topology/Poisson semantics and connector frames/topology/stiffness basis without inference. Retain transform/adapter 0.1 for replay only; do not make new writers emit it. Retire parallel direct product mappings only after round-trip and compatibility tests establish one authoritative route.

This is a bounded capability tranche, not evidence that SOW-066 is complete.

## PCB-005 — PKG-09 needs independent pressure, connector, and bridge oracles

**Classification:** verification coverage gap. **Severity:** medium.

The current physics-audit regression README explicitly excludes pressure and connector constitutive semantics (`validation/benchmarks/physics_audit_regression/README.md:1-13`). The 2026-09-08 pressure work is a complete candidate investigation and M9 is a pressure-free mixed reference. Neither is current implementation adoption or an independent full end-to-end oracle.

Freeze expected results before each source change:

- Pressure: free closed, restrained closed, separately supported closures, and mixed thermal/pressure; `p=0`, `nu=0`, and `DeltaT=0`; orientation reversal; two collinear segments; endpoint/station parity; support equilibrium; wall/effective/action/cut type identities; mutations for omitted/doubled thrust and wrong area.
- Connector: all six rigid modes; six single generalized-deformation modes; at least one coupled `Kc`; finite differences of `q`, energy, and virtual work; symmetry/PSD; moment balance about arbitrary origins; endpoint reversal; translated/rotated installations and nonzero offsets; explicit topology; mutation restoring raw translation difference.
- Bridge: every source record mapped or blocked; unknown-version rejection; source hash and provenance; original and canonical units; unsupported component fail-closed; pressure and connector fields preserved exactly; no solver invocation after a blocking transform.

Use exact identities or publish raw residuals. Do not invent a universal numeric tolerance. Run dense and sparse product paths where applicable, then arrange the separately governed industry-solver comparison. No such external comparison was performed here.

## Prioritized bounded implementation/test fence for WORKING_ITEMS

1. **P0, independent of contract adoption:** repair straight endpoint action-to-cut stress recovery in `core/product_physics/src/lib.rs`; preserve raw action rows; add the focused parity and mutation tests. Either prove the curved tangent transform in the same bounded review or block curved endpoint stress.
2. **P0, safety then mechanics:** stop current finite-length expansion-joint solver consumption with a typed diagnostic. Implement the objective two-frame element in `core/solver/frame_kernel/src/lib.rs`, its explicit schema/adapter in the physical/component contract and `core/product_physics/src/lib.rs`, and the focused rigid-mode/energy tests. Keep an explicitly named legacy external-frame spring only if compatibility requires it.
3. **P1, pressure capability:** add the exact straight circular-annulus/generalized-plane-strain contract, explicit Poisson ratio and pressure topology, and typed versioned results across `core/product_physics`, `core/loads/stress_recovery`, model/results schemas, and focused fixtures. Do not extend the rule to bends or expansion joints without separate proof.
4. **P1, bridge after schemas stabilize:** implement one version-negotiated physical-to-analytical-to-solver route, carry the pressure/connector meanings losslessly, and block on omissions or version mismatch. Keep the Python 0.1 route as replay evidence until migration completes.
5. **PKG-09 throughout:** freeze independent oracles before implementation, run focused source tests and dense/sparse product cases after implementation, require independent physics review, and schedule industry-solver comparison as a later validation phase.

## Remaining limits

- This return selects a defensible straight-pipe pressure reference but does not select pressure-gradient, bend ovalization, pressure-stiffening, large-rotation connector, manufacturer-specific nonlinear stiffness, or code-allowable models.
- Manufacturer stiffness data can require different reference planes or coupling. The new schema exposes those meanings; it does not invent missing provenance.
- The full transform test could not run because `jsonschema>=4,<5` is absent. The focused adapter test passed and the unsupported-expansion-joint transform reproduction passed.
- No shared build, blanket suite, GUI route, external solver, or industry benchmark was run.
