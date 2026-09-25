# Fault harness source clearance

The prepared harness is source-clear for the four specifically granted transaction controls, using `node run.mjs ./EXECUTION_01` from its own fault_controls directory. This is source clearance only; no assertion outcome was known at this checkpoint.

All 13 files match PREPARATION_FREEZE.json. The full harness `run.mjs` is `32e8d8bb0879f77b8a0400429ddf4d592432ddb5082e97ed75ff6a42e10e12e8`; copied and maintained recipe bytes both match the independently reviewed `4ebeca47a8040f8334ea38ea4c36bb5d50078b7ad2912a0981a9e86e3d10d399`.

The harness copies that exact recipe into disposable projects under canonical `/private/tmp`, supplies explicit stub Cargo/rustc executables, removes inherited NODE_OPTIONS for the recipe child, and injects rename failures only through an external Node preload. It changes no production recipe or source. Producer-shaped inputs are clearly marked transaction-only scaffolds with unresolved numerical quality; they do not stand in for real producer replay or numerical proof.

The four scenarios assert second/third installation failure restoration, incomplete rollback with backups and continued restoration, and symlink-parent refusal before Cargo/staging/rename. Expected recipe exit status 1 is distinguished from harness assertion success. Raw logs, event traces, source/input hashes, actual commands and disposable recovery sandboxes are retained. No actionable source finding was identified in this bounded harness.

Reviewer: TASK `/root/solver_manager/fixture_route_review`, same independent reviewer, parent `/root/solver_manager`; read-only source inspection and Python hash checks only. Combined real-replay/documentation/fault-result return follows separately.
