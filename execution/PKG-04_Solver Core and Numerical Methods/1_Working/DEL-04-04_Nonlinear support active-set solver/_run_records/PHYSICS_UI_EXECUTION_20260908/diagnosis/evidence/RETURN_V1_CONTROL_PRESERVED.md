# F4-D1 return — M1-N-008 current-normal diagnosis

RUN_STATUS: SUCCESS
ControlSurface: FILE
TaskProfile: NONE
TaskSkill: software-defect-diagnosis
ScopePath: `/Users/ryan/.codex/worktrees/8728/chirality-physics-ui-execution-20260908/projects/chirality-piping/execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-04_Nonlinear support active-set solver/_run_records/PHYSICS_UI_EXECUTION_20260908/diagnosis`
Source: `779dedb8670625b36af07b89fc5557470e47c50e` (exact accepted-basis match)

## Reproduction result

The unchanged `supplement_v2` witness has no drift. Pre-copy and post-run SHA256 values are:

| File | Bytes | SHA256 |
|---|---:|---|
| `harness/Cargo.lock` | 1538 | `ac1abea2355eafb839ee15c8dbde2cb500710b66217106c9b036b59d54461cd4` |
| `harness/Cargo.toml` | 377 | `326677deba0692d4eb2bebbb4c4bd009c805fc0a742197798a072afaf4924558` |
| `harness/src/main.rs` | 3139 | `5b4671282466232902d42daeb6ec12391654381c067c9b9276cc6dbd6253ed2d` |

Both temporary copies matched those bytes before execution. The unchanged replay and the temporary two-sign extension each exited `0`; SparseInteractive and DenseScrutiny returned identical reported fields. The temporary root `/tmp/chirality-f4-d1.EflygK` and both unique Cargo targets were removed after evidence capture.

| Force X | Seed | Modes | Returned `u_x` | Returned `R_f` | Current `N` | `mu*N` | `R_f - opposing(mu*N)` |
|---:|---|---|---:|---:|---:|---:|---:|
| +10 N | Sticking | both | 0.046666666666666676 m | -3.0 N | 7.666666666666668 N | 2.3000000000000003 N | -0.6999999999999997 N |
| +10 N | Sliding | both | 0.053333333333333344 m | -2.0000000000000004 N | 7.333333333333335 N | 2.2000000000000006 N | +0.20000000000000018 N |
| -10 N | Sticking | both | -0.046666666666666676 m | +3.0 N | 12.333333333333332 N | 3.6999999999999997 N | -0.6999999999999997 N |
| -10 N | Sliding | both | -0.04000000000000001 m | +3.9999999999999996 N | 11.999999999999998 N | 3.5999999999999996 N | +0.3999999999999999 N |

Every case reports `converged: true`, final state `Sliding`, and two iterations. The independent simultaneous static oracles are `u=7/135`, `N=200/27`, `R_f=-20/9` for +10 N and `u=-7/165`, `N=400/33`, `R_f=40/11` for -10 N.

## Causal diagnosis

Confidence: high. The earliest divergence occurs during iteration-2 force assembly in `core/solver/nonlinear_integration/src/lib.rs:427-433` and `:727-733`. `applied_sliding_friction_forces` receives `iterations.last()` and derives the applied magnitude from the previous iteration's reaction vector before solving the current iterate. In this coupled fixture, the applied tangential force changes `u_x`, which changes the derived normal reaction.

After the solve, `build_trial_states` uses the current reaction at `:451-457` and `:1375-1382`, but convergence at `:466-490` tests only active-set state change. It does not require the applied sliding force to equal the force implied by the newly solved normal. `core/solver/nonlinear_supports/src/lib.rs:524-536` additionally preserves Sliding for a previously Sliding support with nonzero displacement when the raw current classifier would return Sticking. This contributes to zero state-change residual in the under-bound cases; the over-bound cases also stop because Sliding remains Sliding.

The defect is independent of sparse/dense mode and tangential force direction. Witness drift, wrong source SHA, mode-specific numerics, direction reversal, and tolerance-scale noise are rejected by hashes, exact source match, exact mode parity, opposing force signs, and 0.2–0.7 N algebraic discrepancies.

## Repair surface and verification

The likely minimal source surface is `core/solver/nonlinear_integration/src/lib.rs`, specifically the sliding-force update and convergence acceptance. A repair must prevent unchanged support labels from terminating while the applied force is inconsistent with the returned current normal, or assemble the fixed sliding branch's derived-normal Coulomb relation simultaneously. `nonlinear_supports/src/lib.rs` is needed only if the selected repair changes anti-chatter semantics. An iterative repair still needs an authorized force/current-normal closure policy because this witness supplies zero state-change tolerance; this diagnostic does not choose one.

Regression coverage should preserve the unchanged +10 N witness and add the mirrored -10 N fixture, both seeds and modes. Assert seed/mode identity, signed `R_f = +/-mu*N_current`, the rational static oracles, full iteration evidence, and max-iteration failure honesty. Add a mutation check that restores previous-reaction scaling or premature state-only convergence, plus zero-mu, zero-normal, threshold-touch, changing-normal, and repeated-solve cases.

## Evidence

Machine-readable diagnosis and raw logs are under the diagnosis `evidence/` directory. `diagnosis.json` binds all cases, commands, source and witness hashes, temporary paths/targets, raw byte counts/hashes, causal chain, rejected alternatives, repair options, and unknowns. Raw logs are unmodified bytes with observed UTF-8 content:

- `replay.stdout.raw`: 485000 bytes, SHA256 `a52a0e015c46f36b871827c955d0102b2988749ec739ad1c9e4f36f591cbc47b`
- `replay.stderr.raw`: 153 bytes, SHA256 `7612d816b0427e51e1bc3222228bdcae0906156a6f5b152954c070ac8d404d60`
- `extension.stdout.raw`: 539242 bytes, SHA256 `11a3026cd2f574dca6eb0e6e9e2c42d71971d4a66d99e70a6460de4eca4e52e2`
- `extension.stderr.raw`: 1510 bytes, SHA256 `fcd8b9e2ae23815654cc2629bd42bace94abb9738384e21b02264e4cc7123569`

One initial command wrapper exited `1` after Cargo completed because it assigned zsh's read-only `status` variable. Its stdout, Cargo stderr, and exact wrapper stderr are retained as `replay_attempt1.*`; the accepted rerun used the same unchanged witness and produced byte-identical stdout with captured exit `0`.

## Run contract

ToolsUsed: `python3 tools/software_workflow/discover_repository.py`, `python3 tools/software_workflow/select_affected_checks.py`, Rust/Cargo and deterministic local shell probes authorized by the sealed brief.
ToolPolicyCompliance: PASS
WriteAuthorization: ALLOWED_WRITE_TARGETS
MISSING: none
NEEDS_HUMAN_RULING: repair algorithm and any required nonzero force/current-normal convergence policy remain outside this diagnostic authority.
DEPENDENCY_NOTES: evidence-stage only; no dependency disposition assumed or mutated.
AppliedChanges: D1 `RETURN.md` and diagnosis/run-record evidence only; no source, product, test, historical, decision, dependency, lifecycle, or pointer edit.
