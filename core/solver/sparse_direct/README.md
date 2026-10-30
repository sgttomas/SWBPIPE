# Sparse Skyline Direct Solver

This crate is the first bounded implementation slice of the human ruling `DEC-023` (decision D-03, Option C; recorded in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12). It provides the project's in-repo sparse solver strategy: bandwidth/profile-reducing DOF ordering, skyline (profile) storage of the symmetric stiffness matrix, and an in-repo LDLᵀ profile factorization and solve. It supports `DEL-04-01` (sparse solve interface) within PKG-04.

## Scope

- Deterministic reverse Cuthill–McKee (RCM) ordering over a node adjacency structure or a dense matrix nonzero pattern.
- Skyline (profile/envelope) storage of a symmetric matrix: lower triangle by rows, each row storing the contiguous span from its first nonzero column to the diagonal.
- In-repo LDLᵀ profile factorization with a fixed operation order and no pivot reordering.
- Forward/diagonal/backward substitution solve in the ordered numbering, with permutation back to the caller's numbering.
- A high-level `solve_symmetric_system` seam that orders, stores, factors, and solves a dense symmetric system and returns ordering, profile, and factorization observations for benchmark evidence (FR-008).
- Factorization reporting: singular pivot location (ordered and original indices), accepted-pivot extrema, a pivot-ratio conditioning proxy, and nonpositive-pivot count and first location.

## Ordering Algorithm Choice and Determinism

The ordering is **reverse Cuthill–McKee**, chosen because:

- it is the classic bandwidth/profile-reducing ordering for the banded/branched frame topologies this solver targets (the structure named by DEC-023's "bandwidth/profile-ordered DOF numbering");
- it is a pure breadth-first traversal that admits a fully deterministic formulation with no randomized or data-dependent-hash tie-breaking.

Determinism properties (all enforced in code, none statistical):

- The result is a pure function of the undirected graph structure: input adjacency lists are symmetrized, sorted, and deduplicated before use, so caller list order cannot change the result.
- Connected components are processed in order of their lowest original node index.
- Each component starts from a pseudo-peripheral node found by the standard level-structure iteration, with every tie broken by ascending `(degree, original index)`.
- Breadth-first neighbor visits are ordered by ascending `(degree, original index)`.
- The concatenated Cuthill–McKee order is reversed once at the end.
- No randomization, no threading, no platform-dependent iteration order anywhere.

## Numerical Determinism Posture

The factorization and solve are single-threaded straight-line `f64` arithmetic with a fixed operation order (rows ascending; within a row, columns ascending; inner summations ascending; backward substitution rows descending with ascending columns). No reassociation, blocking, SIMD dispatch, or parallel reduction is introduced, preserving the same byte-reproducibility argument as the existing dense path (`core/solver/frame_kernel`). Repeat-run bitwise identity is asserted in the unit tests.

## Pivot Guard and Conditioning Proxy

- `SPARSE_SOLVE_ZERO_PIVOT_GUARD = 1.0e-12` mirrors the in-repo dense path's `DENSE_SOLVE_ZERO_PIVOT_GUARD` precedent. It is an internal verification guard, **not** the project solver tolerance policy (tolerance policy is governed under D-04; see `DEC-026`).
- A pivot at or below the guard fails as `SingularPivot` with both the profile-ordered index and the original (pre-ordering) index, so the failure can be traced to a reduced DOF.
- Accepted negative pivots are never silently absorbed: the factorization report carries `nonpositive_pivot_count` and the first location. A symmetric stiffness system is expected positive definite; nonpositive pivots indicate the reduced system is not.
- `pivot_condition_ratio_estimate` is `max |d| / min |d|` over accepted LDLᵀ pivots. This is a **documented proxy** for conditioning, not a true condition-number estimate. No classification thresholds are encoded here; thresholds remain governed by D-04.

## Symmetric-Input Contract

The solver consumes only one triangle of the input: for any index pair the entry read is the original lower-triangle entry `dense[max][min]`. Symmetry of the input is the caller's contract. This convention keeps construction deterministic even when transform round-off leaves last-ulp asymmetry in an assembled dense stiffness matrix.

## Dependency Posture

Zero external dependencies (`[dependencies]` is empty; Rust standard library only), matching the dependency posture of the sibling solver crates and the DEC-023 requirement. The frame kernel appears only as a dev-dependency for parity tests.

## Non-Compliance Boundary

This crate computes open numerical-mechanics quantities only. It does not encode compliance rules, code-specific values, protected standards content, material allowables, private project data, or engineering approval/certification claims. All test models are invented/synthetic.

## Verification

The unit tests cover deterministic RCM ordering (path, star, disconnected, scrambled numbering, tie-breaks, input symmetrization), profile storage round-trips, exact hand-checked LDLᵀ factorization and solve, singular-pivot location reporting in both numberings, nonpositive-pivot counting and location, zero-dimension and invalid-input handling, repeat-run bitwise determinism, and sparse-vs-dense parity on invented frame models (small chain, small grid frame, and a larger generated banded chain) against the `core/solver/frame_kernel` dense solve. Parity comparisons cite the DEC-026 analytic-class relative seed (1.0e-9) rather than inventing a tolerance.

## Integration Status

The live product solve path still uses the dense verification interface; binding it to this solver is a separate bounded tranche (see the `SparseSolverTbd` diagnostic in `core/solver/diagnostics`, whose message records this adoption-pending state).
