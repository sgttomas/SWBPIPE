# Derived-normal priority reconciliation V1

## Verdict

**PASS** for the selected priority amendment. **CHANGES_REQUIRED** for the
unamended working candidate at:

- `nonlinear_supports/src/lib.rs` SHA-256
  `956f64abfab50f16310641e2ffa63eccf37a3cb31ce48332fe7c14be28125d11`;
- `nonlinear_integration/src/lib.rs` SHA-256
  `f4218ccf67e6d458be79fad9ac1b1b87ff836cdfc28a53b64696eae4e6dfc9d4`.

This addendum supersedes only the priority statement in
`F4_RECONCILIATION_V1.md` that treated an inadmissible tangential direction as
governing whenever the tangential and derived-normal checks both failed. The
rest of that review remains a conditional historical input.

## Why the priority is necessary

The PR760 sign-flip fixture has the free tangential equation and restrained
normal reaction

```text
150 u_t = F_x + F_t
R_n = -50 u_t - F_y.
```

For `(F_x,F_y)=(+10,-1)`, extrapolating the assumed positive signed-normal
branch beyond its domain gives

```text
u_t,2 = 97/1350 = +0.07185185185185186
R_n,2 = -70/27 = -2.5925925925925903
F_t,2 = +7/9 = +0.7777777777777771.
```

The assumed branch requires `R_n >= 0`, while the same solve returns `R_n < 0`.
The assisting product `F_t,2 u_t,2 > 0` is therefore produced by a force law
evaluated outside that branch's domain. It is evidence that the trial branch is
invalid; it is not admissible evidence for selecting `Sticking`.

Retrying `Sliding` with the observed negative branch gives

```text
u_t,3 = 103/1650 = +0.06242424242424242
R_n,3 = -70/33 = -2.121212121212121
F_t,3 = -7/11 = -0.6363636363636364
F_t,3 u_t,3 < 0.
```

For `(-10,+1)`, all three signed quantities reverse consistently. This is the
established three-iteration PR760 path.

## Accepted row-local resolution order

For each row currently marked `Sliding`:

1. Resolve contact first. Explicit `N <= 0` is `Inactive`. A derived normal is
   passed to the contact classifier as `abs(R_n,source)`, so exact
   `R_n,source == 0` is `Inactive`; a negative nonzero signed source reaction is
   still positive contact magnitude and is not contact loss.
2. Preserve the first sliding-seed no-force iterate as the existing
   nonconvergent direction warm start, after contact resolution.
3. If a nonzero derived signed-normal branch is inadmissible, retain
   `Sliding`, do not use its provisional force/reaction for the tangential
   state decision, and retry from the observed signed source-reaction branch.
4. Only for a derived branch that is admissible, or for an explicit normal,
   apply the final tangential check. Zero limit requires no applied-force record
   and nonzero motion. Positive limit requires nonzero motion and strict
   opposition from both the applied and reported tangential forces. Failure
   selects `Sticking` before residual and convergence formation.

Skipping step 4 for an inadmissible derived branch means “not evaluated.” An
implementation may leave that row neutral in the aggregate tangential boolean;
it must not claim that the provisional tangential force is physically
admissible.

## Convergence guard

Acceptance still requires all of the following in the same iterate:

```text
active-set state convergence
and no blocking diagnostic
and no deferred sliding force
and every evaluated tangential branch admissible
and every derived signed-normal branch admissible.
```

The derived-normal gate must remain separate. It blocks convergence when the
row stays `Sliding` and its state residual is zero during a branch retry. The
tangential gate must also remain separate because an explicit-normal row, or a
derived row on a valid normal branch, can retain the same base classifier state
while its final applied force assists motion.

Rows resolve independently before the aggregate gates are formed. Thus one
derived row may remain `Sliding` for a normal-branch retry while another row
re-sticks for a valid-normal tangential failure; convergence is blocked if
either aggregate fails.

## Required oracles

- PR760 both load signs and both dense/sparse modes: iteration 2 remains
  `Sliding` and is nonconverged on the derived gate; iteration 3 converges to
  the values above (with signs mirrored), satisfying
  `abs(F_t)=mu*abs(R_n)` and `F_t*u_t<0`.
- A two-iteration cap on the same fixture fails honestly with the existing
  derived-normal nonconvergence diagnostic.
- The explicit-normal sub-limit sliding-seed counterexample has no invalid
  derived branch to mask it; its post-force assisting trial re-sticks.
- Explicit `N<=0` and derived `abs(R_n,source)==0` select `Inactive` before
  either branch retry or tangential checking.
- A contacted zero-limit row applies no friction force and remains sliding for
  nonzero motion under the existing convention.
- A mixed-row fixture proves row-local simultaneous updates and proves that
  neither aggregate convergence gate can be removed.

No numerical tolerance, history state, schema, or additional source file is
needed for this amendment.
