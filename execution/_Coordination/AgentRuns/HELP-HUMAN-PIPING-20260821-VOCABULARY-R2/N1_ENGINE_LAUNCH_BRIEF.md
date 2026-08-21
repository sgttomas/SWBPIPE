# N1 engine launch brief

- Instance: `WORKING-ITEMS-VOCAB-R2-N1-ENGINE`; role `WORKING_ITEMS`; package `PKG-16`; selected deliverable `DEL-16-01`.
- Basis: run plan v1 and branch/base recorded there; accepted bend implementation commit `b988d9d0e4a7048ac28a73bbe53ce045c631dff8`.
- Objective: replicate resolver coverage in `core/model_operations/operation_applier/src/lib.rs` for tee, reducer, valve, and flange, in that order, with focused Rust tests and explicit geometry/connectivity inputs.
- Allowed writes: the operation-applier crate and instance-local N1 engine return/check records only. Do not write UI, deliverable status, coverage, receipt, shared handoff, export, or Git state.
- Return: changed paths, per-kind behavior, tests/commands/results, blockers, exact residuals, and validated package-level verdict.
- Escalate: any new normative/design choice, overlap, or scope expansion.
