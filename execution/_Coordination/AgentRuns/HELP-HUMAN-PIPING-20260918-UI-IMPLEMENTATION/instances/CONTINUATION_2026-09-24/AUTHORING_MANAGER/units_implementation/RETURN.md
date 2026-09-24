# M24 TASK return (candidate; verification pending resource lane)

Parent `/root/authoring_manager`; harness-native TASK child; no delegation.
Role and source hashes: [CONTEXT.json](CONTEXT.json).

Scope: core/units; projectService and test; LoadCaseManagerPanel and focused test;
operation_applier primitive unit validation and test. No schema edits, solver edits,
commits, dependency installs, or runtime commands. Shared isolated checkout is a
coordination boundary, not an OS enforcement guarantee.

Implemented canonical new-project/new-thermal `degC`; persisted `C` preserved and
explicitly resolved by requested absolute/interval dimension. Exact catalog
extensions kN, kN*m, kN/m, GPa, bar, N/mm, kN/mm, 1/degF; source links and factors
in units README. Pressure reference semantics unchanged. Primitive creation no
longer trusts unknown units equal to project metadata. Draft readiness calls the
existing async native/WASM conversion service, blocks pending/invalid results,
and ignores superseded responses.

Prepared checks: units crate all tests; operation_applier primitive creation
regression; Vitest projectService + LoadCaseManagerPanel.units; TypeScript.
All Cargo/npm/build/browser/native execution held pending manager lane grant.
Manager coordinates actual solver and native save/reopen proof; browser-memory
save/reopen test cannot establish native persistence.

Frozen for manager verification and independent review. Candidate hashes:
[CANDIDATE_FILES.json](CANDIDATE_FILES.json). `git diff --check` passed.
Component tests mock conversion and catalog loading: they witness readiness,
request semantics, rejection and stale-response handling, not native execution.

Exact proposed commands (manager owns runtime lane):

```sh
cargo test --manifest-path projects/chirality-piping/core/units/Cargo.toml
cargo test --manifest-path projects/chirality-piping/core/model_operations/operation_applier/Cargo.toml primitive_creation_validates_project_tokens_and_preserves_legacy_c
cargo test --manifest-path projects/chirality-piping/core/model_operations/operation_applier/Cargo.toml explicit_create_primitive_load
```

From isolated `projects/chirality-piping/apps/desktop`:

```sh
../../node_modules/.bin/vitest run src/services/projectService.test.ts src/features/load-cases/LoadCaseManagerPanel.units.test.tsx
../../node_modules/.bin/tsc --noEmit
```

No runtime results claimed. Source frozen at parent's request; subsequent repair
requires review findings or test failures from parent.
