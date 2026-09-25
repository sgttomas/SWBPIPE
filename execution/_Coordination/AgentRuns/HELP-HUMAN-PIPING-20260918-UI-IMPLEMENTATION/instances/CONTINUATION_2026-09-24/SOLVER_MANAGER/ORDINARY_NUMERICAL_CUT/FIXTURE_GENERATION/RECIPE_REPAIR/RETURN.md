# Maintained generator recipe — proposed patch only

PROPOSED.patch contains exactly two proposed maintained-file changes relative to
REPO_ROOT: replace package.json's legacy redirection recipe with
`node tools/serialization/generate_product_preview_mechanics.mjs`, and add that
built-in-Node script. Both proposed files are under proposed/ for review.
PATCH_FREEZE.json pins original/proposed hashes. No maintained file was edited,
no fixture was regenerated, and no Node/Cargo/build/test/Git command was executed.
The currently running test lease is untouched.

The script locates the project from its own installed location and rejects extra
arguments. It uses offline/locked Cargo metadata to discover actual local package
dependencies, requires those packages inside the bounded core tree, and hashes
all their non-build/cache files plus the example, package recipe, input model,
legacy fixture, schemas and project-local compiler configuration files. It records
the resolved package names/versions/sources and tool versions. Source inventory
paths are project-relative. This is reproducible bounded source/input evidence,
not complete compiler-environment authentication or an engineering proof.

It runs the existing preview_result example with the two exact mode arguments,
capturing stdout as bytes. Both processes must succeed and both documents must
validate producer/schema/model identity, actual numerical/case/status consistency,
finite result rows and exact per-case mode evidence before replacement begins.
A genuine blocked empty producer response remains blocked/nonpassing without
fabricated execution coverage. Unresolved or failed quality is never upgraded.
There is no alternative solver, postprocessing of numbers, header synthesis or
new physics. Actual raw stdout bytes become the fixture bytes unchanged.

The identical source/input/lock/legacy inventory and dependency list are checked
after both commands, then again before commit. The write allowlist contains only:

- invented_mechanics_result_precision_1_sparse.json;
- invented_mechanics_result_precision_1_dense.json;
- precision_fixture_generation.json.

All three are staged on their destination filesystem after successful capture and
validation. The generation record commits last, binding output hashes and actual
commands/statuses/source identities. Ordinary replacement errors restore saved
preimages. No multi-file atomicity across process/power interruption is claimed:
a partial interrupted set is detectable by the generation-record hashes and must
be rejected/replayed. The legacy fixture is never a write destination. Temporary
staging is removed; stderr is forwarded to the caller's log and hashed, while raw
stdout persists exactly in the fixture outputs.

The script inherits the caller's normal Cargo environment/resource policy; it
neither installs dependencies nor changes configuration or cache links. Parent
must review, apply after the current lease, then authorize its controlled replay
and capture runtime logs. Failure controls worth checking then: second-mode command
failure, malformed/mismatched mode evidence, source/input change between captures,
legacy preservation, and replacement failure rollback. No such runtime check is
claimed here. The current precision fixture files remain unchanged pending that
explicit replay.

Exact current maintained-file hashes, original package bytes and actual task
origins are retained under _run_records. Authorship: native TASK
/root/solver_manager/evidence_primitives under /root/solver_manager; no descendants.
