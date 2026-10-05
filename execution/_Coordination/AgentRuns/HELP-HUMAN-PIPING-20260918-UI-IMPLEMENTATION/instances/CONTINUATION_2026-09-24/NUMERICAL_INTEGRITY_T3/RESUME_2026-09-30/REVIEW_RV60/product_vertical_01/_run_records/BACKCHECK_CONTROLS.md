# Preserved RV60 controls

The frozen candidate is 52842022cc49d0c9dd8d0760e727963d29015842. See CANDIDATE_SOURCE.json for the ten exact maintained hashes. SETUP.json and OVERLAY_INVENTORY.json define the external fixture; its symlinks are runtime-only and are not sealed evidence.

From the NUM checkout, run `_run_records/run.py` with the named job, crate-relative path, and remaining Cargo arguments. It always sets an absolute manifest and external target, --locked --offline, four jobs, two test threads, a 1200-second wall, and records candidate source before/after hashes plus raw output. Names starting with control_ select the external overlay; other names select CODE. Absolute paths are recoverable from each command record. Rebind a repaired candidate explicitly during a later authorized same-reviewer backcheck; do not overwrite these original records.

Reconstruct the overlay by copying frozen core/product_physics and core/solver/frame_kernel, linking remaining unchanged core/solver children and fixtures from the pinned CODE tree, then appending the listed control files byte-for-byte in OVERLAY_INVENTORY.json order. No source production body is replaced. rv60_g5a_access.rs adds only a cfg(test) wrapper so the independent sibling test can invoke the existing private G5a method.

- `control_pp product_physics --lib rv60_ -- --nocapture` exercises the first two new PP tests.
- `control_pp2 product_physics --lib rv60_ -- --nocapture` exercises all four after the second append and access wrapper. Current failed obligations are logged as *_ACCEPTED true; a repair must change these to false or typed refusal, with unchanged numeric output. These are diagnostic observation tests, not assertions expecting permanent acceptance of the defect.
- `control_fk solver/frame_kernel --lib rv60_ -- --nocapture` exercises dual-hull necessity, input-derived exactness, the exact absolute boundary and signed quotient endpoints.
- `baseline_pp product_physics --lib retained_product_tests -- --nocapture` and `baseline_pp_release product_physics --release --lib retained_product_tests -- --nocapture` run the frozen real specimens and existing controls.
- `rv60_exact.py <actual log> <new result.json>` imports only Python standard library. It independently derives source/represented truths from actual captured input bits, verifies all classes/scales/predicates and full G5a, and asserts every failed product row also misses source truth. Use a fresh output path for any later run.

The synthetic +0 snapshot in rv60_g5a_join_mutation_controls is labelled test-only: it changes ordinary W0 negative zero fields solely to isolate metadata validation with passing arithmetic. It does not replace actual W0, establish product availability, or authorize a sign repair for the real ordinary output.
