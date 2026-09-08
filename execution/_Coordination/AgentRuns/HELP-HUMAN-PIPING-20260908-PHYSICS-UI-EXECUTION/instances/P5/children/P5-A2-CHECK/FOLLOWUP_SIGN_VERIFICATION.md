# P5-A2-CHECK sign-label follow-up

Verdict: vector signs correct; action labels required correction.

For local `x:i->j` and tensile `N>0`, `Kd=[-N,+N]` is the node-on-element, stiffness-conjugate end-force vector and the external nodal load required to sustain the deformation in `Kd=f_ext`. The opposite force exerted by the element on its nodes is `[+N,-N]`.

Current-source verification:

- `core/solver/straight_pipe/src/lib.rs:482` computes `local_forces=K_local d`; the tensile-extension test at line 1653 verifies `[-N,+N]`.
- `core/product_physics/src/lib.rs:6868` calls the analogous `Kd-f_equiv` quantity the “true node-on-element end forces.”
- `add_pressure_thrust_loads` at product-physics line 6714 adds `[-P,+P]` to the global RHS, the applied external pressure-equivalent nodal load.
- Straight recovery at product-physics lines 1561–1568 forms `Kd-f_equiv`; pressure correction therefore produces `[-S,+S]`.
- Fixed/fixed pressure has raw `Kd=0` and corrected/reported node-on-element endpoints `[+P,-P]`.

The existing section-cut transforms remain correct. The child recommended the four terminology corrections now recorded in `SIGN_CONVENTION_CLARIFICATION_V2.md`. No writes, delegation, broader research, equation change, or numerical change occurred.
