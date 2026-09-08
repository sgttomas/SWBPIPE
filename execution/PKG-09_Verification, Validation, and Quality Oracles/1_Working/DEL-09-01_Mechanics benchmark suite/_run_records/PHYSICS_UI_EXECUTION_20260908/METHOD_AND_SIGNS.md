# Independent mixed-reference method and sign conventions

Status: FROZEN_BEFORE_PRODUCTION_COMPARISON. The method consumes `INPUT_FREEZE.json`, not production output. The executable implementation is `independent_mixed_reference.py`; `EXPECTED_RESULTS_FROZEN.json` is its first frozen result.

## Scope and units

The calculation is a linear, small-displacement, 3D Euler–Bernoulli frame with a separately enumerated unilateral/contact and Coulomb-friction state. Each node uses global `[UX, UY, UZ, RX, RY, RZ]`; translation is in metres, rotation in radians, force in newtons, and moment in newton-metres. Moduli are in pascals. This is an explicit fixture-local SI basis. It does not establish the held DEL-02-02 canonical conversion basis or the human disposition `PKG09-0901-PKG02-001`.

The live source fixture supplies geometry, material, cross section, loads, supports, spring stiffness and friction coefficient. Production solver output, DEL-04-01/04-02 generated results, and bundled expected constants do not supply an oracle quantity.

## Frame and load calculation

For each member, local x is the normalized i-to-j chord. The authored `y_reference` is projected normal to x and normalized as local y; local z is `x cross y`. The resulting row matrix `R` maps global vectors to local vectors, and block-diagonal `T=diag(R,R,R,R)` maps the 12 global nodal DOFs to local DOFs.

The annulus properties are derived directly from the authored diameter `D` and wall `t`, with `d=D-2t`, `A=pi(D^2-d^2)/4`, `Iy=Iz=pi(D^4-d^4)/64`, and `J=Iy+Iz`. The conventional 12-by-12 prismatic 3D frame stiffness contains axial `EA/L`, torsional `GJ/L`, and the two bending blocks `12EI/L^3`, `6EI/L^2`, `4EI/L`, and `2EI/L`. Global assembly uses `K_e=T^T k_e T`.

For a constant local line load `[qx,qy,qz]`, consistent nodal translations are `qL/2` at each end. End moments are `+qy L^2/12` and `-qy L^2/12` about local z, and `-qz L^2/12` and `+qz L^2/12` about local y. Uniform thermal strain contributes local axial work vector `[-EA alpha DeltaT, +EA alpha DeltaT]`. This retains the authored distributed load as a member load; it is not replaced by a historical `qL/2` nodal-only approximation.

The SH-140 ground spring adds `k` to N-140 UZ. Its physical action on the structure is `-k u`. The physical-frame case includes the four pipe members and omits C-150 pending the separate D02 connector-formulation decision. The literal-adapter diagnostic adds the exact authored relative local C-150 stiffness in parallel with P-130; it is reported separately because independent moment balance exposes the known nonobjective finite-end behavior.

## Contact and friction

NL-140 has `active_when=negative_reaction`. Define `g=-u_y` and `lambda=-R_y`; admissibility is `g>=0`, `lambda>=0`, and `lambda*g=0`. Both active and inactive branches are solved, and only the mechanically admissible branch is retained. This starts from the frictionless actual control.

For NL-130-FRIC, the current normal magnitude is `N=abs(R_S-130,UY)` from the same returned branch. A zero-reference stick candidate constrains N-130 UZ to zero and is admissible only if `abs(R_t)<=mu N`. Each slip-sign candidate leaves UZ free and solves the simultaneous relation `R_t=-mu N sign(u_t)`. Because the frame is linear within a fixed contact state, `R_N(R_t)=a+bR_t`; each sign branch is solved algebraically and then checked for returned displacement sign, opposition, and equality `abs(R_t)=mu N`.

The fixture's `initial_state=sliding` is not accepted as a physical history by itself. An incremental-history interpretation remains open because no ordered load increments, committed slip/reference displacement, rollback rule, or prior converged load-step state are supplied. The zero-reference static result is therefore a bounded branch, not a cyclic-friction claim.

## Signed resultants and checks

Each member local end vector is `p=k_e u_e-f_consistent-f_thermal`, ordered `[Fx,Fy,Fz,Mx,My,Mz]` at i then j. A station vector is the action on the positive-x cut face of the left segment. At station `x`,

- `F_cut=-(p_i_force+q x)`;
- `M_cut=-p_i_moment+x(e_x cross p_i_force)+(x^2/2)(e_x cross q)`.

The script reports full signed global nodal displacements, support actions, local end vectors, and station vectors at 0, 0.25L, 0.5L, 0.75L and L. It checks every element's force and moment balance separately. It also sums external nodal/member loads, rigid/contact/friction actions and the ground-spring action for whole-frame force and moment balance about global origin.

No engineering pass/fail tolerance is introduced. Reported floating-point residuals are raw numerical errors. Applicability is limited to the declared linear frame, pressure-free load case and explicit zero-reference static friction branch.
