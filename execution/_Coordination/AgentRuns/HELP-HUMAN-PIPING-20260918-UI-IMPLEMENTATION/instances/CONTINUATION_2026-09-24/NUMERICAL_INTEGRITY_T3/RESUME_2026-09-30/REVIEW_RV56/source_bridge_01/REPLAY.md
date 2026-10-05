# Bounded replay and repair backcheck

The frozen review subject is 4afbac6e613203f93a499b780082b28be37f6c2c. Source
hashes and candidate-owned commands are in _run_records/scope_and_freeze.json
and candidate_bridge.*. All actual Cargo commands used four build jobs, two
test threads, isolated manifest targets, and a 1200-second process-group wall.
ROOT explicitly granted the lane. Do not run after lane release without a new
grant. No dependencies were installed or source/Cargo locks edited.

Private reviewer controls are in _run_records/rv56_controls.rs and independent
vectors in _run_records/review_vectors.rs. setup_import.py creates a sparse
read-only symlink overlay from an explicit frame_kernel checkout and appends
those controls to a copied test module. It copies adaptive.rs unchanged for
private field access. Pass a new destination inside the authorized review packet.
Never reuse an old adaptive copy to backcheck a repaired candidate.

For the exact frozen F1 reproduction, build the harness against the frozen FK
source and run cargo test --locked --offline --manifest-path H/Cargo.toml --lib
rv56_relative_ceiling_refusal -- --nocapture --test-threads=2 under the recorded
job/target/wall limits. Expected original result: one failure, Ok(0) vs Ok(5).
For the additional source control, use filter rv56_rotated; expected 55/55
source row containment. The direct maintained candidate uses filter source_bridge.
Actual argv/PID/times/exits and raw stdout/stderr for all runs are preserved.

For repair, ROOT should freeze the new candidate, inspect the bounded diff,
create a fresh overlay from that exact source, and run the unchanged reviewer
F1 control plus the author's early-zero/post-helper-work controls. Reassess
legacy publication-radius compatibility and source bridge tests. Review any new
helper's numeric order and simultaneous numeric/accounting error handling. A
passing rerun alone does not substitute for that trace. No repair was made here.

The independent_oracle.py script imports neither author generator nor evaluator.
It verifies the original candidate_bridge.stdout with an independent Machin pi
proof and closed mechanics, records independent_oracle.json, and creates the
rotated axial/spring vector file. Its saved result confirms all 104 native
source and K references and all 104 exact-source predicates, while only 59/104
conservative enclosures prove those predicates. It does not prove raw PP units
or any product coverage/admission. The maintained generator's --diagnose replay
is separately preserved as candidate_oracle.json.

The first absolute-module-path import attempt failed before any test ran because
Rust resolved nested modules differently. Its files and command/stderr remain in
imported_path_attempt and imported_prefix.*. The corrected sparse-overlay setup
completed at 01:03:45Z; the discriminating failure completed at 01:03:56Z, within
four minutes of receipt. A documentation-write tool call later had a JavaScript
syntax error before shell execution; the corrected call wrote only this packet.
Runtime symlink overlay and build targets are ignored by this packet's .gitignore;
replay scripts, explicit source revision/hashes, controls, vectors, raw failures,
and the failed import construction manifest retain the necessary durable basis.
