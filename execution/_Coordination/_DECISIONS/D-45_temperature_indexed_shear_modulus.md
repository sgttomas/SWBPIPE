# D-45 - Temperature-Indexed Shear Modulus

**Status:** PROPOSAL  
**Date prepared:** 2026-07-15  
**Decision ID:** D-45  
**Prepared by:** agent, as the governed residual gateway from the bounded
`DEC-077` implementation tranche

## 1. Decision Statement And Scope

Should the material/load-case basis model extend temperature indexing to
shear modulus G, and if so, what user-entered data and derivation method may the
software use?

This decision is limited to the existing modulus-basis solve semantics. It
does not authorize a bundled material curve, catalog, code table, default,
professional-reliance claim, or lifecycle/release transition.

## 2. Accepted Basis And Current Behavior

- TP-PMM-P3-MODULUSBASIS-001 recorded temperature-indexed shear modulus as a
  residual and preserved the user-entered base G under every basis.
- The owner ruled D-38 Option O-B, codified as `DEC-077`, for linear
  interpolation of **E and alpha** between adjacent user-entered temperature
  points, with explicit two-point provenance and no extrapolation.
- `DEC-077` does not name G. The live material temperature-point shape has E
  and alpha slots but no temperature-indexed G slot. Extending the ruled method
  to G would therefore change both the data contract and numerical results and
  is not agent-decidable.

## 3. Options

### O-A - Permanently preserve base G under all modulus bases

Record the current behavior as the final policy. Temperature bases affect E
and alpha only; G always uses the material's explicit base value.

- Pro: no new data shape or synthesized value; current behavior remains exact.
- Con: E and G can reflect different temperature bases unless the user manages
  the base value externally.

### O-B - Add user-entered temperature-indexed G points

Add an explicit G slot to user-entered temperature points and apply a declared
method. The default implementation proposal is the already-ruled `DEC-077`
method: linear interpolation only between strictly bracketing adjacent points,
two-source provenance, and blocking without extrapolation.

- Pro: a load-case basis can keep E, alpha, and G temperature-consistent using
  only user-entered values.
- Con: expands the schema, authoring, operation, solver, benchmark, and
  provenance surfaces and synthesizes an additional numerical input.

### O-C - Defer and preserve current base-G behavior

Keep base G as the current behavior and leave the permanent policy open.

- Pro: avoids premature data-model expansion.
- Con: retains an explicit modeling limitation and later decision cost.

## 4. Non-Binding Recommendation

**O-C** until an owner-approved data requirement establishes whether users need
temperature-indexed G independently of E. This avoids silently treating
`DEC-077` as broader than ruled. If O-B is selected, the implementation should
meet the same independent hand-calculation, benchmark, explicit provenance,
edge-blocking, unit-bearing authoring, and mutual-selection evidence bar used
for `DEC-077`.

## 5. Gateway

An owner act is required. Until then, base G remains the explicit current
behavior, D-45 remains `AWAITING_RULING`, and no temperature-indexed G field or
interpolation may be implemented.
