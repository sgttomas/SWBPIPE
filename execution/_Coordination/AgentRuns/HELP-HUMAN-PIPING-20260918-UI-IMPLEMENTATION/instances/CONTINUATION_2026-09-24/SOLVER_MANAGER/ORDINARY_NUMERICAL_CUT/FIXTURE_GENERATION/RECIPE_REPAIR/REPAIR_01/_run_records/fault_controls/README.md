# Transaction fault controls — prepared, not executed

This harness tests only the reviewed generator's filesystem transaction boundaries.
It does not compile product code, run the real solver, validate a physical model,
qualify Current or replace the manager's real two-mode producer replay.

`recipe.mjs` is an exact copy of the revised reviewed proposal, SHA-256
`4ebeca47a8040f8334ea38ea4c36bb5d50078b7ad2912a0981a9e86e3d10d399`.
The harness verifies those bytes before use and copies them into each disposable
project's normal tools/serialization location. No production fault options or
injection hooks are added. `preload.cjs` externally wraps Node's filesystem rename
only; Cargo and rustc are explicit stubs placed first in the child's PATH.

The model, package/source files and producer-shaped stdout are minimal invented
transaction scaffolds. Stub output says unresolved and carries only enough shape
for the recipe's transaction preconditions. It is not a genuine product result
or registered full-method proof. Raw fixture stdout is copied from prepared input
files, and stub commands are retained. No private source or project data is copied.

## Prepared scenarios

1. Fail installation rename2: require all three output preimages exact, legacy/
   input unchanged, and staging cleaned after complete rollback.
2. Fail installation rename3: require the same complete restoration.
3. Fail installation rename3 and the first restoration: require original and
   rollback errors plus the recovery location, retained recovery map and exact
   dense/record backups, sparse restoration still attempted successfully, and
   the unrecovered dense destination equal to captured stub stdout.
4. Symlink fixtures/product_preview to an outside-project directory within the
   disposable container: require refusal before Cargo/staging/rename, with outside
   inputs/output preimages unchanged.

These are ordinary single-writer controls. They do not claim protection against
hostile concurrent directory replacement or multi-file power-loss atomicity.

## Authorized execution command

From this fault_controls directory, when the manager grants execution:

`node run.mjs ./EXECUTION_01`

No command has been executed by the preparing TASK. The harness uses canonical
`/private/tmp` through realpath for disposable projects, not the macOS `/tmp`
symlink. The optional output-directory argument controls only log placement.
It writes stdout/stderr, case records, actual command/source/input hashes and a
SUMMARY.json. Disposable sandboxes are retained, including incomplete rollback
material, so an independent checker can inspect actual files. Do not confuse the
harness's expected status1 from injected failure with an acceptance failure.
Actual assertion outcomes remain unknown until the manager runs it.

A fresh JSON result is produced per scenario even if a behavioral assertion fails;
the harness exits nonzero when any such assertion fails. Infrastructure/setup
failures may stop earlier and are not a transaction test pass. The manager should
preserve the resulting logs before removing disposable sandboxes.

Authorship: native TASK /root/solver_manager/evidence_primitives under
/root/solver_manager; no descendants. Preparation wrote only this evidence subtree.
