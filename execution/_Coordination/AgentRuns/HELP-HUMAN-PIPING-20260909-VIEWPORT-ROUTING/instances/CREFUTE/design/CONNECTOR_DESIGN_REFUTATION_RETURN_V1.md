# Connector design refutation return V1

## Verdict

`CHANGES_REQUIRED` for `PROVISIONAL_OBJECTIVE_CONNECTOR_RECOMMENDATION_V1.md` as an implementation-ready recommendation.

The proposed `B` algebra is correct for the stated infinitesimal, symmetric midpoint convention. Independent calculation passes all six stress-free rigid modes, nonzero offsets, rotated and translated installations, finite-difference kinematics and energy gradients, symmetry, positive-semidefinite assembled stiffness, arbitrary-origin equilibrium, canonical endpoint reversal, all six generalized components, and a mutation restoring raw `u_j-u_i`. The recommendation is not ready for activation because its `q0` wording and rigid-mode oracle conflict, it omits the initial residual required for nonzero `q0`, topology tokens are insufficient to construct the promised graphs, and its source/contract/result-consumer fence does not cover the live runtime.

This is derivative engineering review evidence. It does not adopt a model contract, change source, or claim comparison with an external industry solver.

## Bound identity and inputs

- Runtime instance: `/root/connector_design_refutation`; local role label: `CREFUTE`; parent: `/root`; fresh nondelegating Agent 2.
- Launch configuration received: `gpt-5.6-sol`, high reasoning. Runtime model introspection is unavailable inside the instance; no substitution was reported.
- Accepted source basis: `533332349a4607eee561d4ef90fb05a62d86519e`.
- Sealed brief SHA-256: `2cea13dfc1ec59889f3cedc7ac06f9023befca85982c95167a69d74d068dfd81`.
- Recommendation SHA-256: `ef335820dec9f920778698f4a533315fcc939aa6138367ccd8110f71f84bc379`.
- Exact file inventory and hashes: `INPUTS.sha256`. Source files were read using `git show 533332349a4607eee561d4ef90fb05a62d86519e:<path>`; mutable source bytes were not used.
- Primary engineering comparison: [Abaqus Connector Elastic Behavior](https://docs.software.vt.edu/abaqusv2025/English/SIMACAEELMRefMap/simaelm-c-connelastbehav.htm) and [Abaqus Connector Behavior](https://docs.software.vt.edu/abaqusv2025/English/SIMACAEELMRefMap/simaelm-c-connectorbehavior.htm). The former defines elastic response in local relative-motion components and treats coupled linear elasticity as symmetric. The latter distinguishes zero-force reference lengths/angles from the initial position and describes series as additional nodes with connectors strung between them, while parallel connectors share nodes.

No project-corpus equation was used.

## Algebra and independent calculations

With `S(r)v=r×v`, the attachment displacement is `d_k=u_k+theta_k×a_k=u_k-S(a_k)theta_k`. Therefore

```text
d_j-d_i-theta_c×r0
= -u_i + [S(a_i)+0.5 S(r0)] theta_i
  +u_j + [-S(a_j)+0.5 S(r0)] theta_j,
```

so the recommendation's signs in `B_t` are correct. For an infinitesimal rigid motion `u_k=t+omega×(x_k-o)`, `theta_i=theta_j=omega`, the attachment-motion difference is `omega×r0`; the midpoint term cancels it for any origin `o`. `B_r` also vanishes.

`connector_refutation_check.py` constructs a non-axis-aligned finite connector with nonzero offsets, a right-handed rotated triad, and a coupled positive-definite `Kc=A^T A`. `CALCULATION_RESULTS.json` records:

- stress-free six-mode maxima: `|q|=8.88e-16`, `|f|=9.62e-15`, `|Pi|=2.60e-30`;
- `B=dq/dd` centered-difference maximum error: `5.66e-11`;
- `f=dPi/dd` centered-difference maximum error: `4.59e-11`;
- `Ke` symmetry error: `3.55e-15`; its six null eigenvalues are within `2.17e-15` of zero and six deformational eigenvalues are positive (`2.685` through `57.858`) in the fixed SI numerical basis;
- force balance: exactly zero; moment-balance norms about three origins: `1.49e-16`, `3.33e-16`, `1.78e-15`;
- rotated/translated installation covariance error: `5.00e-16`;
- endpoint reversal: `B` error zero, energy error zero, nodal-force permutation error `1.11e-16`;
- raw `u_j-u_i` mutation gives rigid-rotation translational-deformation norms `2.50`, `3.35`, `3.61`, so every nonzero-length rigid rotation kills the mutation;
- all six unit generalized components are reachable, with maximum reconstruction error `2.22e-16`.

The PSD test is meaningful only after a declared unit conversion and coordinate scaling. A raw eigenvalue test on user numbers carrying mixed units is not a valid acceptance test.

## Required correction: `q0` is a constitutive reference, not an installed zero-force state

As written, `q=Bd` is zero at the reference geometry and for every superposed infinitesimal rigid motion. If `q0` is nonzero,

```text
f(d_rigid)  = -B^T Kc q0
Pi(d_rigid) = 1/2 q0^T Kc q0.
```

The calculation obtains force norm `0.4687` and energy `0.00536` for its nonzero `q0`. It also verifies the correct objectivity property: adding a rigid mode to an arbitrary deformed state changes `q`, `Pi`, and `f` by at most `1.03e-14`.

Selected correction:

1. Rename the field `q_zero_force` or `q_ref`; define it as the local generalized deformation at which constitutive force vanishes. Do not call a nonzero value the installed zero-force deformation when nodal DOFs and `p_i0,p_j0` use that installed geometry as `d=0`.
2. If the installed reference is declared stress-free, require `Kc q_ref=0`; for positive-definite `Kc`, this forces `q_ref=0`. The rigid-mode oracle may require `q=f=Pi=0` only under this precondition.
3. If nonzero `q_ref` is allowed, classify it explicitly as preextension/precompression/pre-rotation. The primitive must return both `Ke=B^T Kc B` and the initial internal residual `r0_internal=-B^T Kc q_ref`; product assembly must place the corresponding equivalent load on the global right-hand side. Testing stiffness alone cannot validate this case. For singular `Kc`, zero force occurs whenever `q-q_ref` lies in `null(Kc)`, so `q_ref` is not a unique physical state unless the contract says how null components are handled.

This correction agrees with the primary comparison source, which distinguishes a zero-force constitutive reference from the initial connector position and describes nonzero initial force for an installed precompressed or preextended connector.

## Midpoint-frame adequacy and limits

The midpoint rotation is a defensible selected convention for a straight, finite, small-rotation connector. With `r0=L e_x`, its transverse measures are the relative transverse translations minus `L` times the average end rotation. This separates constant-curvature end rotation from shear-like relative motion, is symmetric in the ends, and has a clean reversal law. It is not the only objective first-order interpolation: any weighted end rotation with weights summing to one removes a rigid rotation, while the one-half weights add endpoint symmetry.

The contract must say that this is the `symmetric_midpoint_small_rotation_v1` generalized-motion basis. It is not a universal interpretation of manufacturer axial/lateral/angular test values. Values measured with parallel end planes, one fixed end, gimballed ends, or other restraints require a documented transform into this basis; a general symmetric `Kc` cannot repair an undefined measurement basis. The convention is first-order only. Large relative rotations, evolving axes, follower effects, geometric stiffness, and corotational updates remain outside this tranche.

Require `Q0` to be stored or reconstructed as column axes with `Q0^T Q0=I` and `det(Q0)=+1`. For finite length require its first axis to agree with `r0/|r0|` within a named tolerance. A coincident-end special case cannot derive an axis from `r0`; it needs an explicit triad.

For canonical reversal, select `D=diag(-1,+1,-1)`, so `Q0'=Q0 D` preserves local `y` while reversing `x` and `z`. Then

```text
T = blockdiag(-D,-D)
q' = T q
q_ref' = T q_ref
Kc' = T Kc T^T.
```

The calculation verifies that reversed element forces are exactly the original nodal forces with the endpoint blocks permuted. An implementation may instead canonicalize endpoint order before assembly, but it must test the same transformation and never merely swap node IDs while leaving `Q0`, `Kc`, or `q_ref` unchanged.

## Units and stiffness admissibility

Use work-conjugate ordering `[translation_x,y,z; rotation_x,y,z]` and `[force_x,y,z; moment_x,y,z]`, with canonical solver length and force units and radians. The block dimensions are:

| Block | Dimension |
|---|---|
| `Ktt` | force / length |
| `Ktr` | force / radian |
| `Krt` | moment / length, dimensionally force |
| `Krr` | moment / radian |

Symmetry means work reciprocity after conversion to one canonical basis. Schema entries need per-block dimensions; the existing linear/rotational stiffness pair is insufficient for mixed terms. Validate finite values, symmetry after unit normalization, and PSD after applying a declared diagonal coordinate scale, for example `q_hat=[q_t/Ls;q_r]`. Store the scale used by the numerical tolerance. Reject materially negative eigenvalues; permit semidefinite null modes intentionally. Do not require every matrix entry to be positive: valid PSD matrices can have negative coupling entries.

## Live topology and compatibility trace

At the accepted source basis, the current behavior is unambiguously a full-pipe-span parallel spring:

- `core/product_physics/src/lib.rs:3308-3394` excludes straight spans only for curved-bend macro-elements. Expansion-joint mapped pipes remain in `frame_elements`.
- `core/product_physics/src/lib.rs:3397-3398` adds expansion-joint user-stiffness elements afterward.
- `core/product_physics/src/lib.rs:3538-3631` maps the component node to the opposite endpoint of the selected incident pipe and builds the spring between the whole pipe's endpoints.
- `core/solver/frame_kernel/src/lib.rs:759-785` assembles ordinary frame elements and user-stiffness elements into the same global matrix.

Selected topology semantics:

- `replaces_span`: the connector's explicit end nodes equal the endpoints of one named replaceable analytical span; that span is omitted from frame assembly. Reject zero, multiple, or endpoint-mismatched targets.
- `series_between_end_planes`: the connector joins two explicit end-plane nodes and adjacent pipe spans terminate at those nodes. Do not infer series behavior from a token or condense an entire pipe automatically.
- `parallel_with_elements`: every named parallel analytical element has exactly the same endpoint-node pair after canonical ordering; assemble all named elements and the connector. The present behavior can be labelled `legacy_pipe_span_parallel_v0`, but that label records computation, not physical intent.

Preserve legacy fields and bytes through migration. Do not map the four old scalars into `Kc` because their restraint and reference-point basis is unknown. The selected safety behavior is to keep them editable/reportable under `legacy_pipe_span_parallel_v0` but block finite-span solver activation with a deterministic diagnostic until a user authors the objective v1 payload. This prevents continued use of the known rigid-motion-defective formulation while retaining data. Pressure effective area remains a separate existing load-path input and is not part of this connector adoption.

## Live contract and consumer trace

The recommendation's three-file fence is incomplete:

- `schemas/model.schema.yaml:290-338` defines a canonical component with `component_type`, quantity-only `geometry`, and `mechanics_modifiers`. It is not the live preview component shape.
- `schemas/component.schema.yaml:348-455,470-515` defines library field slots and mechanics-interface enums. It also lacks the objective connector record.
- The live preview shape exists in TypeScript `apps/desktop/src/types.ts:5-64` and Rust `core/product_physics/src/lib.rs:217-333`; there is no matching strict preview JSON schema in the traced source.
- `componentIntent.ts:263-270` and `operation_applier/src/lib.rs:2370-2383` create an expansion joint without `mechanics_interface`, while `product_physics/src/validation.rs:1029-1043` and the solver require `solver_consumption=mechanics_geometry_and_user_flexibility`. This is an existing authoring-to-solver discontinuity.
- Both migration mirrors are live: `apps/desktop/src/services/projectService.ts:33-57,123-210` and `apps/desktop/src-tauri/src/model_document_migration.rs:14-43`. A shape change must add the same explicit `0.2.0 -> 0.3.0` transform to both. A no-op is insufficient if legacy behavior must be labelled; the transform may add only the deterministic legacy label and must not synthesize objective fields.
- `core/product_physics/src/lib.rs:7825-7923` emits four scalar stiffness review rows with hard-coded SI units, and `ReportPanel.tsx:333-370,402-442` consumes them.
- `previewService.ts:341-360` and `core/analysis_runs/records.py:461-495` correctly map those rows to linear or rotational stiffness by metadata component.
- Existing downstream bugs are separate from objective-v1 adoption: `ResultExportPanel.tsx:267-286`, `StressNeutralExportPanel.tsx:492-511`, and `HeadlessRunnerPanel.tsx:298-317` fall through stiffness rows to ratio/dimensionless; `HandoffPanel.tsx:325-333` returns `TBD`; Rust `core/runner/headless/src/result_envelope_binding.rs:109-124` has no stiffness family and explicitly tests it as unmapped. These should be repaired even if objective connector adoption is deferred.

## Smallest coherent implementation and test fence

Proceed in three reviewable stages:

1. **Dormant primitive.** In `core/solver/frame_kernel/src/lib.rs`, add an objective connector type separate from `UserStiffnessElement`. Its constructor validates geometry, triad, units-normalized `Kc`, and `q_ref`; its API returns `B`, `Ke`, and initial residual. Tests reproduce every calculation in `CALCULATION_RESULTS.json`, including stress-free and prestressed rigid-mode rules, unit-scaled PSD, reversal, and the raw-difference mutation. Existing behavior remains unchanged.
2. **Versioned contract and authoring.** Add explicit objective-v1 records to `schemas/model.schema.yaml`, `schemas/component.schema.yaml`, and the actual preview contract (`types.ts` plus Rust serde inputs); update `componentIntent.ts`, the property inspector and viewport forms, model tree/workspace projections, `operation_applier/src/lib.rs`, `product_physics/src/validation.rs`, fixtures, and both migration mirrors. Introduce a distinct solver-consumption token. Legacy migration adds only `legacy_pipe_span_parallel_v0`; it does not invent end planes, offsets, triads, topology, `Kc`, or `q_ref`.
3. **Activation and outputs.** In `core/product_physics/src/lib.rs`, build explicit graph topology, omit only validated replacement spans, assemble connector stiffness and initial residual, and emit basis/topology/reference-state evidence. Update `previewService.ts`, `ReportPanel.tsx`, Results/result-interpretation, result export, stress-neutral export, handoff/headless witnesses, `core/analysis_runs/records.py`, Rust headless binding, and `schemas/results.schema.yaml` so matrix entries and generalized forces/moments retain correct dimensions. Add fixtures for all three topologies and ambiguous legacy refusal.

This is the minimum coherent activation fence because omitting any contract, migration, authoring, assembly, or result leg creates either unreachable inputs, silent reinterpretation, a stiffness-only `q_ref`, topology double counting, or wrong exported dimensions.

## Later validation

This bounded run did not execute Cargo, builds, servers, or port-using tests. Before adoption, run the frame-kernel and product-physics unit suites; operation-applier, schema, migration round-trip, result binding/export, and desktop component tests; the registered evidence sweep; and the required fresh source review over the frozen diff. Then compare a small declared benchmark against an external solver using identical end frames, offsets, midpoint component basis, topology, unit normalization, and constitutive reference. No such solver validation is claimed here.

## Validation status

- Deterministic scratch calculation: `PASS`.
- Source mutation: none.
- Scope: only `instances/CREFUTE/design/**` written.
- Overall recommendation: `CHANGES_REQUIRED`.
