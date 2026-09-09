# Provisional objective finite-end connector recommendation V1

## Status and evidence basis

This is a bounded engineering recommendation for a later independently reviewed implementation tranche. It is derivative evaluation evidence, not an adopted public contract or validation against an external industry solver.

- Accepted source basis: `533332349a4607eee561d4ef90fb05a62d86519e`.
- `core/solver/frame_kernel/src/lib.rs`: SHA-256 `d657e6507aa5d4ef80124e3f6d10bd5a8af8eae54fe21af787c92814c8b73fde`.
- `core/product_physics/src/lib.rs`: SHA-256 `f1ae3322a752d92e2125274e7c5ccc0f7ca2c26a00f19d8d5cd3d6c4234a3ed5`.
- `schemas/model.schema.yaml`: SHA-256 `a47b00e0f93c915da49657057261b6903ea1eb9f091adaf555cae312bbe01b6c`.
- Primary comparison basis: Abaqus defines connector elasticity in components of relative connector displacement/rotation, supports symmetric coupled stiffness, and resolves these components in declared local directions: <https://docs.software.vt.edu/abaqusv2025/English/SIMACAEELMRefMap/simaelm-c-connelastbehav.htm>. Its connector behavior also carries an explicit reference length/position at which constitutive force and moment are zero: <https://docs.software.vt.edu/abaqusv2025/English/SIMACAECAERefMap/simacae-c-itnconnpropbehavior.htm>.

## Current defect and scope

`frame_kernel::user_stiffness_local_matrix` at `src/lib.rs:1018-1040` applies six uncoupled springs directly to `u_j-u_i` and `theta_j-theta_i`. For a finite connector whose two nodes occupy different reference positions, a small rigid-body rotation has `u_j-u_i = omega x (x_j-x_i)`. The current lateral springs therefore create strain energy and an uncancelled force couple under a rigid motion.

`product_physics::build_expansion_joint_user_stiffness_elements` at `src/lib.rs:3538-3631` maps an expansion-joint component node to the opposite endpoint of an entire referenced pipe. The current payload has four stiffness scalars and one `expansion_joint_pipe_ref`; it does not identify two joint end planes, attachment offsets, installed zero-force state, stiffness component basis, or series/parallel topology.

This is a mechanics defect when the current element is used as a finite-end expansion joint. A zero-length coincident-node spring remains a valid special case.

## Selected small-displacement formulation

Use an energy-derived, symmetric two-frame connector. Declare nodal reference positions `x_i,x_j`; attachment offsets `a_i,a_j` in each node's initial frame, transformed to global `a_i^g,a_j^g`; initial attachment positions `p_i0=x_i+a_i^g`, `p_j0=x_j+a_j^g`; `r0=p_j0-p_i0`; and a right-handed initial triad `Q0` whose first axis follows the declared i-to-j connector axis.

For nodal degrees of freedom `d=[u_i,theta_i,u_j,theta_j]`, the linearized attachment motions are

```text
d_i = u_i + theta_i x a_i^g
d_j = u_j + theta_j x a_j^g
theta_c = (theta_i + theta_j)/2
q_t = Q0^T [(d_j-d_i) - theta_c x r0]
q_r = Q0^T (theta_j-theta_i)
q = [q_t;q_r]
```

With `S(r)v=r x v`, use

```text
B_t = Q0^T[-I, S(a_i^g)+0.5S(r0), I, -S(a_j^g)+0.5S(r0)]
B_r = Q0^T[ 0,                  -I, 0,                       I]
B   = [B_t;B_r]
```

Declare an installed zero-force deformation `q0` and a symmetric positive-semidefinite `6x6` connector stiffness `Kc` in that basis. Then

```text
Pi = 1/2 (q-q0)^T Kc (q-q0)
f  = B^T Kc (q-q0)
Ke = B^T Kc B
```

All six small rigid-body modes give `q=0`, and stiffness symmetry, force/moment balance, and virtual work follow from the energy rather than from added corrective forces. The symmetric midpoint rotation is selected because it treats both connector ends evenly and cancels the linear rigid-rotation field exactly. This is a small-rotation formulation; a finite-rotation corotational update is outside this tranche.

The current four scalars may map to

```text
diag(Kc) = [k_axial, k_lateral_y, k_lateral_z,
            k_torsional, k_angular_y, k_angular_z]
         = [k_axial, k_lateral, k_lateral,
            k_torsional, k_angular, k_angular]
```

only when a new explicit `isotropic_uncoupled_v1` interpretation is authored. Existing values must not be silently reinterpreted because their measurement restraints and reference points are unknown.

## Required input contract

The versioned connector payload must state:

- two attachment node/frame references and their i-to-j order;
- attachment offsets `a_i,a_j` with length units;
- `Q0`, handedness, and local component ordering/signs;
- `q0`, installed reference state, and reference temperature;
- `Kc`, including mixed block units and provenance/measurement restraints;
- validity ranges and approximation class;
- topology: `replaces_span`, `series_between_end_planes`, or an explicit parallel connection;
- pressure effective area and pressure-thrust reference as separate load-path inputs.

The adapter must not infer the remote expansion-joint end from the opposite endpoint of an entire pipe segment.

## Implementation and compatibility fence

Stage the work in this order:

1. Add the energy-derived connector primitive and its tests in `projects/chirality-piping/core/solver/frame_kernel/src/lib.rs` without changing existing `UserStiffnessElement` behavior.
2. Add versioned canonical and preview connector fields in `projects/chirality-piping/schemas/model.schema.yaml`, the physical-to-analytical transform/adapter, and `projects/chirality-piping/core/product_physics/src/lib.rs`.
3. Consume the objective primitive only when the new interpretation and topology are explicit. Keep legacy payloads on a visibly labelled legacy path or block their use as finite expansion-joint elements with a deterministic migration diagnostic.

Do not combine this tranche with pressure-membrane adoption. Effective area remains a separate pressure-load input.

## Required independent refutation

The review matrix must include:

- all six rigid translations/rotations with `q=f=Pi=0`;
- each single generalized connector component;
- one mixed off-diagonal symmetric `Kc` case;
- finite-difference verification of `B=dq/dd`;
- `Ke` symmetry and positive-semidefinite energy checks;
- finite-difference virtual-work verification of `f=dPi/dd`;
- force and moment balance about arbitrary origins;
- endpoint reversal with canonicalized axes/signs;
- translated and rotated installations, nonzero offsets, and nonzero `q0`;
- each topology value and a failure fixture for ambiguous legacy topology;
- a mutation restoring raw `u_j-u_i`, which every nonzero-length rigid-rotation case must kill.

Later external industry-solver comparison should use the same declared end frames, offsets, component basis, and installed reference state.
