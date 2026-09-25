# R1/R2 repaired — revised proposal only

The original recipe proposal remains unchanged. This directory carries the full
revised PROPOSED.patch, the narrow REPAIR.diff against that proposal and pinned
proposed files/hashes in PATCH_FREEZE.json. No maintained source, package, fixture
or application file was applied or replayed; no Node/Cargo/test/build/Git command
ran. App/report/comparison workers are untouched.

R1: all existing destination preimages are now written to separate staged backup
files and fsynced before the first installation rename. New files and a recovery
map are also staged first. Rollback restores those backups by rename, attempts
all remaining restorations after an individual failure, and refuses to overwrite
a destination altered since installation. If any restoration fails, the staging
material is retained and the error reports both the original installation failure
and each rollback failure plus its recovery location. Cleanup no longer destroys
incomplete-recovery material. Cleanup failure itself is reported without deleting
unverified paths or concealing retained material.

R2: the project root and every traversed path component are checked with lstat
and realpath. Symlink/redirected components are rejected, not silently resolved
into an alternate project. Fixed input/generator/recipe/manifest and the fixture
parent are checked before Cargo metadata or reading. Inventory directories and
files use the same checks. The actual fixture parent is checked before mkdtemp,
and staging/destination/source paths are rechecked around writes, installation,
rollback and cleanup. Existing destinations must remain regular files. This is
ordinary single-writer containment, not a claim to defeat hostile concurrent
filesystem path replacement.

The fixed three-output allowlist, both actual producer modes, nonpassing outcome
preservation, unchanged source/input/lock checks, raw stdout bytes and read-only
legacy fixture are retained. The only package recipe change remains the reviewed
Node-script invocation. No new dependency, configuration or numerical method is
introduced.

Same-reviewer backcheck is requested before any application/replay. Later authorized
fault controls should cover second/third installation failure, a failed restoration
with durable remaining backups, and a symlinked fixture parent refusing before
staging/output mutation. These behaviors are implemented but not runtime-tested by
this TASK. Exact original proposal and review origins are under _run_records.

Actual author: native TASK /root/solver_manager/evidence_primitives under
/root/solver_manager; no descendants.
