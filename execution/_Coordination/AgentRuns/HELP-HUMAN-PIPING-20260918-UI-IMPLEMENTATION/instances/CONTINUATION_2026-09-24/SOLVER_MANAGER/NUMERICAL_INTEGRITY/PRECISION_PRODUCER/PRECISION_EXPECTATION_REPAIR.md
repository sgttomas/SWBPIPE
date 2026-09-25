# Precision expectation repair after initial parent execution

The parent supplied INITIAL_CHECKS/_run_records/product_full_initial.log: 186 passed, four failed. The initial precision_ selection passed 5/5. Those are parent observations, not execution by this child. Original logs, first FROZEN.json and CANDIDATE.diff remain untouched. No Cargo/build/native run occurred in this continuation.

The four assertions compared independently evaluated floating point quantities after the old publisher had mapped each onto a 1e-6 absolute decimal grid. Removing that grid exposes differences; it does not change the expected physical equations. The repair uses the previously selected analytic 1e-9 relative criterion without an added absolute floor. It changes no active-set branch, runtime tolerance, engineering formula, oracle value or protected record.

- Ground spring: independent global equilibrium remains +350 N. Observed 349.99999999999994 N differs by about 1.6e-16 relative. Compare full force against 350 N at the existing analytic criterion, rather than requiring exact binary identity after publication quantization.
- Straight endpoint/station stresses: the vector assertion compares dense and sparse factorization results, not repeated identical execution. The initial values differ by a few 1e-15 MPa. Keep exact vector lengths and compare each same-unit nonzero stress at relative 1e-9; do not turn stress agreement into new physical validation or alter the separate exact-repeat tests. Existing named DEC-053 thresholds remain unchanged and scoped to their recorded observation set.
- Mixed friction: independent authored law remains -mu*N = -0.3*10 = -3 N, opposing motion. Observed -3.000000000000057 N differs by about 1.9e-14 relative. A named assert_coulomb_force_balance_n helper checks sign and the existing relative force-comparison criterion. It is not an active-set or contact-admissibility runtime tolerance.
- Derived friction: separately recovered tangential and normal forces obey the same signed Coulomb relation within numerical error. Initial tangential 0.48952718870570366 N versus 0.01*normal = 0.48952718890942876 N differs by about 4.16e-10 relative. The helper checks this full-precision relation under 1e-9, without fitting a new bound to that observation. The historical six-decimal checks remain historical-only. Normal-force provenance, sliding state and displacement assertions remain. This relation check is not an independent oracle for the coupled normal force or qualification of the complete nonlinear model.

Criterion basis re-read: D-04_tolerance_coverage_thresholds.md standing DEC-026 ruling (analytic seed 1e-9, exact regression/corpus classes retained); sparse_default_promotion_policy.dec053.json (relative parity 1e-9, exact repeat 0, retained bounded observation scope); current NUMERICAL_REFERENCE/NUMERICAL_POLICY_REVIEW and parent repair brief. Unmeasured stress/nonlinear acceptance tiers remain unresolved; no universal tolerance is introduced. Exact repeat-determinism assertions and historical fixture bytes are unchanged.

Removed the two unused mut qualifiers on selected summary quantities that were formerly mutated by the removed publication loops. Scoped rustfmt and git diff --check pass. REPAIR_FROZEN.json and REPAIR_CANDIDATE.diff identify the revised candidate. Parent testing also resolved the previously anticipated sparse_direct-to-frame_kernel dependency edge in product Cargo.lock; that actual current lock is included in the new freeze.

Parent rerun commands, after allocated CPU handoff:

```
cargo test --offline --manifest-path projects/chirality-piping/core/product_physics/Cargo.toml
cargo test --offline --manifest-path projects/chirality-piping/core/product_physics/Cargo.toml precision_
```

The whole-crate command covers all four repaired assertions and unchanged exact-repeat checks. New failures require diagnosis; no pass is claimed before that rerun. Product source ownership returns to the parent at this freeze.
