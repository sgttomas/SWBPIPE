# Nonlinear Integration

`open_pipe_stress_nonlinear_integration` is the Phase D / D6 integration owner
created under `DEC-044`. It wraps the existing linear frame assembly/solve path
and the `open_pipe_stress_nonlinear_supports` active-set classifier without
moving either boundary.

Current slice:

- assembles the frame stiffness matrix each iteration, including explicit
  user-stiffness macro-elements and explicit curved-bend macro-element
  stiffness slots (`DEC-070`) supplied by the caller;
- applies base restraints plus active nonlinear restraints;
- applies the `DEC-067` bounded `+/- mu*N` tangential force at friction
  supports classified sliding, opposing the observed motion with the current
  iterate's normal-reaction evidence (a sliding state seeded before any solved
  iterate defers convergence one iteration so the bound is applied first; no
  path/load-step friction history model is introduced);
- solves the reduced system through the public solver mode policy: sparse
  interactive by default, dense scrutiny when explicitly selected;
- reconstructs full displacement and reaction vectors;
- feeds trial support facts into `evaluate_active_set_iteration`;
- stops when the active set converges or the explicit iteration cap is reached;
- guarantees every non-converged exit carries a visible `NonConvergence`
  failure diagnostic, including the zero-residual capped exit where a
  sliding-seeded first iterate defers convergence at `max_iterations == 1`
  (recorded `TP-PMM-P2-FRICTION-001` corner, closed by
  `TP-PMM-P2-NONCONVDIAG-001`).

The current residual is the active-set state-change count exposed by the
classifier. `DEC-046` supplies accepted active-set count controls for the
product-preview policy, and `DEC-053` promotes sparse interactive as the default
preview/render path with dense scrutiny retained for deliberate review. Callers
must still supply explicit convergence controls; this crate does not provide
protected standards content, catalog values, rule checks, professional approval,
certification, sealing, authentication, or code-compliance claims.
