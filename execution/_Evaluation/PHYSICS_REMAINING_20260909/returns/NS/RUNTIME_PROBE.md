# NS-01 Current-SHA Runtime Probe

## Purpose and boundary

This is the single focused executable probe authorized after the source review. It invokes the current `open_pipe_stress_nonlinear_integration` crate at source basis `533332349a4607eee561d4ef90fb05a62d86519e` through the evaluation-local runner in `runtime_probe/`. Source crates were read-only. Cargo artifacts were directed to `returns/NS/_scratch/target` and removed after the run.

The model is a two-node axial member with `EA/L = 100 N/m`, `F_x = +1 N`, `mu = 0.3`, fixed normal reaction `N = 10 N`, and Coulomb limit `mu*N = 3 N`. All DOFs other than node 1 `Ux` are restrained. The only changed input between runs is the initial friction state.

## Exact command

Working directory:

```text
/Users/ryan/.codex/worktrees/8728/chirality-viewport-routing-20260909
```

Command:

```sh
CARGO_TARGET_DIR=projects/chirality-piping/execution/_Evaluation/PHYSICS_REMAINING_20260909/returns/NS/_scratch/target cargo run --manifest-path projects/chirality-piping/execution/_Evaluation/PHYSICS_REMAINING_20260909/returns/NS/runtime_probe/Cargo.toml --quiet
```

Exit code: `0`
Elapsed wall time reported by the command tool: `1.831292209 s`

## Exact stdout

```text
NS-01 current-SHA runtime probe: k=100 F=+1 mu=0.3 N=10 muN=3
seed=sticking converged=true iterations=1 final_state=sticking displacement_ux=0.00000000000000000 reported_friction_reaction=-1.00000000000000000 applied_sliding_friction=none friction_times_displacement=-0.00000000000000000
  iteration=1 state=sticking displacement_ux=0.00000000000000000 reported_reaction=-1.00000000000000000 applied_sliding_friction=none
seed=sliding converged=true iterations=2 final_state=sliding displacement_ux=-0.02000000000000000 reported_friction_reaction=-3.00000000000000000 applied_sliding_friction=-3.00000000000000000 friction_times_displacement=0.06000000000000000
  iteration=1 state=sliding displacement_ux=0.01000000000000000 reported_reaction=0.00000000000000000 applied_sliding_friction=none
  iteration=2 state=sliding displacement_ux=-0.02000000000000000 reported_reaction=-3.00000000000000000 applied_sliding_friction=-3.00000000000000000
```

There was no stderr output because `--quiet` suppressed routine Cargo output and the run succeeded.

## Result

The current-SHA executable reproduces NS-01. The sticking seed returns the sub-limit sticking equilibrium `u=0`, `T=-1 N`. The sliding seed returns `u=-0.02 m`, `T=-3 N`, with `T*u=+0.06 J`; the applied friction force has the same sign as the final displacement. Both runs report convergence, but their displacement and state differ solely because of the numerical seed.

This probe establishes the direct nonlinear-integration defect for this exact case. It does not exercise the product adapter, provide an industry-solver comparison, or validate a future repair.
