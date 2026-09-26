# File ownership and integration sequence

All paths below are WORKING_ROOT-relative. This is a working allocation, not a claim that every writer has started.

- Parent `/root/physics_resume`: sole `core/product_physics/src/lib.rs` facade and DTO writer, `case_state/mod.rs`, source contribution ledger, nonzero boundary integration, pressure adapter join, normalization/validation and public contract integration. Parent also owns source-recovery/receipt integration until separately transferred, preserving ordinary/source method boundaries.
- TASK `/root/physics_resume/composite_receipt`: sole `core/product_physics/src/case_state/thermal.rs` for private normalized thermal/free-length/fit math and its in-module tests; no facade/material/schema edits.
- TASK `/root/physics_resume/exact_authoring`: sole `core/product_physics/src/case_state/material.rs` for private per-element material selection and its in-module tests; no facade/thermal/schema edits.
- TASK `/root/physics_resume/membrane_backcheck`: independent analytical reference translation only in maintained `core/product_physics/tests/fixtures/load_reference_states/` plus assigned reference evidence; no production kernel/selector writes.

Parent first joins a common resolved case to actual existing assembly/recovery, while analytical controls and private modules progress independently. Then join selected source-recovery inputs/receipts and connected public authoring/readers/native persistence semantics. Batch related work by engineering dependencies; this is not a rigid capability/interface/test phase gate. Relevant frozen source receives independent review from a non-author, reusing completed design/kernel gates within their actual scope. ROOT schedules native/whole-candidate checks and graph/Git work.

No writer owns engine or primary checkout files. Public UI/readers/schema/native ownership will be assigned explicitly when the concrete consumed record is ready; no unannounced concurrent facade writes.
