# F4 initial-phase return

RUN_STATUS: `INITIAL_PHASE_COMPLETE_SOURCE_HELD`
Role: WORKING_ITEMS Agent 1 for PKG-04 / DEL-04-04
Source: `779dedb8670625b36af07b89fc5557470e47c50e`
Configured execution: `gpt-5.6-sol`, high reasoning; actual model identity is not exposed to this instance

The unchanged M1-N-008 witness was hash-verified and replayed from an isolated copy against the bound current source. Both solve modes reproduce the same defect for both initial states: the returned converged Sliding state uses a friction force scaled by the previous normal reaction. The positive cases miss the returned current bound by `+0.7 N` and `-0.2 N`; the mirrored negative cases miss it by `-0.7 N` and `+0.4 N`. The independently frozen simultaneous targets are:

- positive: `u=7/135`, `N=200/27`, `R_f=-20/9`;
- negative: `u=-7/165`, `N=400/33`, `R_f=40/11`.

The causal site is `core/solver/nonlinear_integration/src/lib.rs:427-433,727-733`: force assembly reads `iterations.last().reactions` before the current solve. Current reactions reach trial-state construction only afterward, and convergence at `:466-490` accepts an unchanged active-set label. Sparse and dense modes are identical, and the discrepancy is independent of tangential sign.

Six focused current-head checks passed for the exact E006-E009 consumed interfaces and existing diagnostics/convergence envelope. E006-E008 have current factual producer/consumer evidence but remain subject to the owning dependency disposition. E009 retains the PDU-035 formal human hold. No dependency row, accepted DAG, pointer, lifecycle, decision, or source was changed.

The frozen implementation route is a simultaneous affine current-normal solve within each outer active-set iteration. It preserves `sliding_direction`, public result/iteration types, sparse/dense final-solve evidence, DEC-046's state-count policy, and the existing NonConvergence failure at the cap. It uses base and unit-load reaction influence, solves `(I + C H)q = -C r0`, and adds only a discrete normal-sign branch gate. A predictor/corrector loop is held because it needs a new quantitative force residual and tolerance.

F4-D1 completed successfully; all temporary copies and Cargo targets were removed. The compile slot is released. Active F4 controls contain no machine-absolute paths; the original child return bytes are retained only as structural run evidence, with the active portable successor bound in the manifest.

Resume only after the Owner dependency act covering E009, a root-issued versioned amendment naming exact source/test paths, and a root-assigned isolated compile slot. The later candidate still requires focused post-change tests, fresh independent 100% frozen-diff review, and root-coordinated DEC-025 closeout.

Manifest: `INITIAL_PHASE_MANIFEST.json`
