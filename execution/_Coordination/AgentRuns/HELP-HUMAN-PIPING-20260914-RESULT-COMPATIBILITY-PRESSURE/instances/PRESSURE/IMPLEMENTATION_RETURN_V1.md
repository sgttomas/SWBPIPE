# PRESSURE implementation return V1

Status: `IMPLEMENTED_FOCUSED_TESTS_PASS`; source frozen for root-owned refutation and integration.

The released private pressure kernel is implemented in exactly two product files. `lib.rs` contains only an allowed private dormant module declaration. `pressure_exact.rs` contains the validated values, scalar mechanics, local pairs, and 17 co-located tests. No product runtime references the module.

## Frozen source inventory

- `ExactAnnulus` keeps radii and exact fluid/wall areas private behind a checked constructor and crate-private read-only accessors. Wall area uses `PI*(ro-ri)*(ro+ri)` and admits adjacent radii whenever both areas remain positive and representable.
- `IsotropicENu` validates finite `E>0`, `-1<nu<0.5` and derives the only `G` authority; derived zero/infinity rejects.
- `InternalDifferentialPressure` accepts finite positive, zero, negative and subnormal pressure.
- `lame_at_radius` enforces the closed wall interval. It uses a factored area-ratio radial expression to preserve exact boundary tractions at pressure scale, including a valid annulus whose unscaled squared-radius difference rounds to zero. Hoop follows the constant transverse-trace identity. Non-finite derived loads/stresses reject.
- `axial_state` preserves `Nw=EAs(epsilon_z-thermal)+2nuP`, `S=Nw-P` and `sigma_z=Nw/As` with positive tension. It never subtracts cap load from wall force.
- `eigenload_pair` is documented and implemented as the applied external equivalent RHS. It calculates `2nuP-EAs*thermal` directly with stable triple products, preserving representable eigen force when an `epsilon0` intermediate would underflow.
- `cap_pair` uses the exact internal area and returns the pure local mathematical `[-P,+P]` pair without topology inference.

## Frozen test inventory and result

Seventeen focused tests cover exact/reference/adjacent and nonrepresentable annuli; single-authority `G`, midpoint `2160/31`, auxetic and open-bound material bounds; signed/subnormal pressure; Lamé inner/outer/interior/constant-sum behavior; pressure-scale adjacent-radius traction; radius rejection; all five states spanning the four rational/pi cases; `Nw/S/sigma_z` independence; applied eigen and cap signs; zero/nu-zero/thermal/signed/superposition reductions; exponent ±200 geometries; subnormal rounding; stable eigen-force recovery when axial stiffness overflows; and all frozen invalid-input/true-overflow classes.

Final focused locked result: 17 passed, 0 failed, 0 ignored, 149 filtered out. Format check and `git diff --check` pass. Exact commands and output are under `_run_records/`.

## Failed attempts retained

The initial formatting check exited 1 and printed rustfmt-only layout changes. `cargo fmt` applied them and the next check passed. An earlier hash-verification shell loop used the zsh-special variable name `path`, temporarily replacing command lookup inside that loop; the corrected loop used `file_path` and verified all eight release hashes. Neither failure changed product semantics. There were no test failures and no source repair cycle.

## Claim and handoff boundary

This is a tested dormant pure kernel only. It does not qualify runtime pressure, topology, solver assembly/recovery, DTOs, versions/migrations, units at public boundaries, result identities/hashing/persistence, dense/sparse parity, native behavior, lifecycle, release, or professional reliance. Root owns independent oracle-to-product refutation, full-diff review, broader checks, integration and closeout.
