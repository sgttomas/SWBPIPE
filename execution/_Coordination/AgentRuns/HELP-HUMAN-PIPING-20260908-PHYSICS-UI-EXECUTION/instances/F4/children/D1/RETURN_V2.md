# F4-D1 portable return V2 — M1-N-008 current-normal diagnosis

RUN_STATUS: SUCCESS
ControlSurface: FILE
TaskProfile: NONE
TaskSkill: software-defect-diagnosis
ScopePath: `{WORKING_ROOT}/execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-04_Nonlinear support active-set solver/_run_records/PHYSICS_UI_EXECUTION_20260908/diagnosis`
Source: `779dedb8670625b36af07b89fc5557470e47c50e` (exact accepted-basis match)
Supersedes: `{ScopePath}/evidence/RETURN_V1_CONTROL_PRESERVED.md`, SHA256 `429b638e827f16779449840e1e200ea32160a04d2ff27970e87fac4d70227de6`
Successor reason: move reusable control-facing paths to portable tokens. The superseded bytes remain preserved. Exact temporary host paths and commands remain only in the structural diagnosis evidence.

The unchanged M1-N-008 replay and temporary two-sign extension each exited `0`. SparseInteractive and DenseScrutiny returned identical reported fields. Both temporary witness copies matched original bytes before execution; the historical witness remained byte-identical afterward; the temporary workspace and both Cargo targets were removed.

| Force X | Seed | Returned `u_x` | Returned `R_f` | Current `N` | `mu*N` | Signed discrepancy |
|---:|---|---:|---:|---:|---:|---:|
| +10 N | Sticking | 0.046666666666666676 m | -3.0 N | 7.666666666666668 N | 2.3000000000000003 N | -0.6999999999999997 N |
| +10 N | Sliding | 0.053333333333333344 m | -2.0000000000000004 N | 7.333333333333335 N | 2.2000000000000006 N | +0.20000000000000018 N |
| -10 N | Sticking | -0.046666666666666676 m | +3.0 N | 12.333333333333332 N | 3.6999999999999997 N | -0.6999999999999997 N |
| -10 N | Sliding | -0.04000000000000001 m | +3.9999999999999996 N | 11.999999999999998 N | 3.5999999999999996 N | +0.3999999999999999 N |

Every case reports converged Sliding in two iterations. The independently frozen simultaneous static oracles are `u=7/135, N=200/27, R_f=-20/9` for +10 N and `u=-7/165, N=400/33, R_f=40/11` for -10 N.

The earliest divergence is iteration-2 force assembly at `core/solver/nonlinear_integration/src/lib.rs:427-433,727-733`: the applied force uses the previous iteration's normal. The current normal is seen only after the solve at `:451-457,1375-1382`, while state-only convergence at `:466-490` accepts the lag. `core/solver/nonlinear_supports/src/lib.rs:524-536` contributes to unchanged Sliding in under-bound cases but does not need modification for a simultaneous current-normal solve.

Primary repair surface: `core/solver/nonlinear_integration/src/lib.rs`. Preserve existing direction, public result/iteration structures, diagnostic codes, DEC-046 state-count policy, and cap failure honesty. Machine-readable values, exact witness/source hashes, raw logs, exact commands, removed host-temp paths, the preserved failed wrapper attempt, causal chain, and rejected alternatives remain in the structural diagnosis evidence at `{ScopePath}/evidence/diagnosis.json` (SHA256 `f7d1fc0b8b6f571ba994d86c3214d69d861e34ae51da0b5f54783db4f579baf2`).

No production, test, history, dependency, lifecycle, or decision file changed in F4-D1.
