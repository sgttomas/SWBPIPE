# Nonlinear Supports

This crate is the bounded implementation slice for `DEL-04-04`. It models the
active-set decision boundary for one-way supports, gaps, lift-off/contact, and
friction-limited supports.

The crate consumes trial displacement and reaction facts from a mechanics solve
iteration and classifies nonlinear support states as active, inactive, sticking,
or sliding. Unilateral supports use the `DEC-067` state-switched
complementarity test: an engaged support (including a closed gap) classifies on
the sign of the trial reaction, and a released support classifies on trial
displacement penetration toward the side it can bear against (or the explicit
gap clearance), so re-engagement and lift-off transitions are both observable
and the converged state does not depend on the seeded prior state. It records changed support states, residuals, convergence, and
nonconvergence diagnostics. `ActiveSetReportRecord` exposes report-facing
iteration settings, support states, changed supports, diagnostics,
assumptions, and limitations without requiring downstream reports to parse
diagnostic text. The crate does not assemble or solve the global nonlinear
system, select sparse solvers, define release tolerance policy, perform load
case algebra, run rule-pack checks, or make professional/code-compliance
claims.

## Boundary

- Support behavior is explicit model data; no support/catalog defaults are
  supplied.
- Sign conventions are explicit through activation and gap-direction enums
  rather than hidden assumptions.
- Gap clearance and friction coefficient fields are validated at classification
  time as well as constructor time, so public-struct callers cannot accidentally
  receive zero defaults.
- Friction behavior uses user/model supplied coefficient and normal reaction
  values; no engineering defaults are invented.
- `NonlinearSupportUnitMetadata` binds gap/trial displacement, reaction, and
  friction coefficient values to caller-supplied unit-system and unit
  identifiers using accepted PKG-02 dimensions. It records metadata only and
  does not convert or default units.
- Nonconvergence and nonlinear missing-data errors are surfaced through the
  solver diagnostics contract.
- Report-facing active-set records copy structured diagnostics and explicit
  assumptions/limitations while preserving the existing solver iteration
  record.

## Tests

The unit tests cover one-way engaged/released classification and
penetration re-engagement, gap opening/closing and closed-gap bearing-sign
release, missing gap data,
lift-off behavior, friction sticking/sliding/contact-loss/missing-coefficient
edges, invalid numeric inputs, active-set convergence, and iteration-limit
nonconvergence diagnostics with active-set context, plus explicit
report-record conversion fields.
