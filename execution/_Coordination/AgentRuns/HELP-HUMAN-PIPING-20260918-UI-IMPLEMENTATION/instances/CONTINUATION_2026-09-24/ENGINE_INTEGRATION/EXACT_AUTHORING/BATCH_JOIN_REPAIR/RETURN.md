# Batch join and pressure-reference guard repair

Implementer: actual TASK `/root/physics_resume/exact_authoring`, parent `/root/physics_resume`. No delegation. Parent assigned the two independent review findings and the three bounded source/test paths below. This is implementation and self-check evidence, not independent review. Root/Piping/LOOP_INIT and change-skill basis remains recorded in the parent exact-authoring return. The reviewer BRIEF was read as contextual scope; its instructions were not adopted as a reviewer role.

The original reviewer freeze is unchanged. `BEFORE_HASHES.json`, `AFTER_HASHES.json` and `REPAIR.patch` preserve the first, batch-target stage. `REFERENCE_GUARD_BEFORE_HASHES.json`, `REFERENCE_GUARD_AFTER_HASHES.json` and `REFERENCE_GUARD.patch` preserve the second, deletion-reference stage. `COMPLETE_REPAIR.patch` covers the whole bounded repair. Prior source is recovered only in memory and SHA-verified for these diffs; no source trees were copied or reviewer files rebound. Static counterexamples remain in `PREFLIGHT_FAILURE_STATIC.json` and `REFERENCE_GUARD_FAILURE_STATIC.json`; neither is represented as a pre-repair Cargo run.

Changes:

- `core/model_operations/operation_applier/src/atomic_batch.rs`: adds the implemented `Model` target to the closed preflight allowlist. The common single-operation kernel still validates the profile path, payload, target identity, before-value and hash; no generic model write is introduced.
- `core/model_operations/operation_applier/src/lib.rs`: existing pipe and node deletion dependency enumerators now include pressure-region member pipes and terminal nodes. They inspect those references even when a case has no `primitive_loads` member. Diagnostics identify the case-qualified region. They do not remove or retarget any reference.
- `core/model_operations/operation_applier/tests/exact_authoring.rs`: complete UI-shaped intents and eight tests, retaining the original three controls. Added full four-operation E/nu → two per-case region arrays → profile review/apply; late-invalid final-profile rollback after three successful private simulations; independent stale caller/current-plan and current-caller/stale-plan hashes; pipe/node direct and batch deletion blocks; and successful explicit region removal before deletion. The isolated terminal test intentionally uses incomplete imported data with no other node dependencies so the new terminal guard is actually exercised.

Static validation: rustfmt parsed the changed tests and library without altering the library; scoped `git diff --check` passed. Runtime validation pending the coordinated Cargo lane; no Cargo or npm command has been run for this repair yet.

Prepared command, from the joined checkout:

```sh
CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR=/private/tmp/piping-create-section-inputs-target cargo test --manifest-path projects/chirality-piping/core/model_operations/operation_applier/Cargo.toml --test exact_authoring -- --nocapture
```

Parent must route the final hashes/diff and focused check outcome for independent backcheck. The operation-applier WASM artifact built before this repair is no longer current for these source changes and needs rebuilding before connected browser/native packaging claims.

Final handback: parent `/root/physics_resume` executed the coordinated focused check and preserved the exact command, executor, source hashes and timing in [`focused-01.json`](focused-01.json), with untouched output in [`focused-01.log`](focused-01.log). I inspected those records: exit 0, **8 passed, 0 failed, 0 ignored**. This is observed parent execution, not a test run performed by this TASK. The independent [`AUTHORING_NATIVE/REVIEW.md`](../../STABLE_REVIEW/AUTHORING_NATIVE/REVIEW.md) source-backchecks AN-R1 and AN-R3 (plus the separately owned native AN-R2 correction); its original freeze and final reviewed hashes remain in that review directory.

All three source/test files are released to parent ownership. No further source edits or builds will be performed by this TASK. The focused runtime hold above is resolved; rebuilding current-source WASM and completing the promised actual native author/review/apply, undo/redo and save/reopen witness remain separate parent integration work and are not claimed by this check.
