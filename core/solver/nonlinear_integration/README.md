# Nonlinear Integration

`open_pipe_stress_nonlinear_integration` is the Phase D / D6 integration owner
created under `DEC-044`. It wraps the existing linear frame assembly/solve path
and the `open_pipe_stress_nonlinear_supports` active-set classifier without
moving either boundary.

Current slice:

- assembles the frame stiffness matrix each iteration;
- applies base restraints plus active nonlinear restraints;
- solves the reduced system through the public solver mode policy: sparse
  interactive by default, dense scrutiny when explicitly selected;
- reconstructs full displacement and reaction vectors;
- feeds trial support facts into `evaluate_active_set_iteration`;
- stops when the active set converges or the explicit iteration cap is reached.

The current residual is the active-set state-change count exposed by the
classifier. `DEC-046` supplies accepted active-set count controls for the
product-preview policy, and `DEC-053` promotes sparse interactive as the default
preview/render path with dense scrutiny retained for deliberate review. Callers
must still supply explicit convergence controls; this crate does not provide
protected standards content, catalog values, rule checks, professional approval,
certification, sealing, authentication, or code-compliance claims.
