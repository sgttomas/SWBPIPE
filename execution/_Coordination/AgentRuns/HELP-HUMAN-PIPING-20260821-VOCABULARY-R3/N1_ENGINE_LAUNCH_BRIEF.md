# N1 P2 engine repair brief

- Instance: `WORKING-ITEMS-VOCAB-R3-N1-ENGINE`; role `WORKING_ITEMS`; package `PKG-16`; deliverable `DEL-16-01`.
- Basis: run plan v1; pre-repair `2bee267300e571e4e8686f73aba6ad4ba8be4c54`; R2 review P2.
- Objective: enforce distinct tee header/branch references in the operation applier. Add a negative Rust regression proving a blocking diagnostic and no `applied_model`. Equality is not valid; no alternative/domain choice is authorized.
- Allowed writes: `projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs` and instance-local N1 engine return/check records only.
- Required checks: focused creation slice, full operation-applier suite, wasm32 release build with wasm feature, formatting/diff checks; record pre/post hashes.
- Prohibited: UI/status/coverage/receipt/Git writes or unrelated lint cleanup.
