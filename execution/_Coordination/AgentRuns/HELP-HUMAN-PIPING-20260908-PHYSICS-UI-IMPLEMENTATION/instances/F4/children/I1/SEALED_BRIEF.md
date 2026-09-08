# F4-I1 sealed implementation brief

Role: bounded ephemeral Agent 2; no delegation
RequestedModel: `gpt-5.6-sol`
RequestedReasoning: `high`
ScopePath: `{WORKING_ROOT}/execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-04_Nonlinear support active-set solver/_run_records/PHYSICS_UI_IMPLEMENTATION_20260908/implementation`
PackageID: `PKG-04`
DeliverableIDs: `DEL-04-04`
AcceptedBasis: source `55df51ac3201456e0f181823e3aefefef47a73bb`; adopted Owner candidate `6866bbf52ef861488277b6a659c7ac0a4e63f42865debb971355db5e553ea142`; F4 source release `SOURCE_RELEASE_V1.md` SHA256 `38cdd1de98f0d4b9405a5f92fb5488b305af3779b3ce1ad603d46d79f9222933`; reviewed mechanics `IMPLEMENTATION_BRIEF_V2.md` SHA256 `63c0361979c20474cddaada74076705a495a60da1c2d5cb0548a10ac6fd7974c`
ApplyEdits: true
PROFILE_PATH: `{WORKING_ROOT}/software-workflow.json`

## Objective

Implement the exact simultaneous affine current-normal Coulomb correction in `{WORKING_ROOT}/core/solver/nonlinear_integration/src/lib.rs`. Preserve the existing `sliding_direction`, anti-chatter/history behavior, DEC-046 state-count convergence controls and outer cap, public structs/result fields, diagnostic vocabulary, sparse/dense final-solve evidence, canonical SI, and failure honesty.

Use base plus unit-load reaction influence for all active derived-normal sliding rows, solve `(I + C H)q = -C r0` simultaneously, and validate the exact reviewed branch domains: `s=+1` accepts `r>=0`, `s=-1` accepts `r<=0`, and active `s=0` accepts only `r==0`; zero coefficient or zero existing direction is branch-irrelevant. Any inadmissible row defers convergence to the existing outer retry/cap. Add no tolerance, residual family, inner retry budget, or public field.

## Allowed writes and tools

- `{WORKING_ROOT}/core/solver/nonlinear_integration/src/lib.rs`, including its in-file tests
- `{RUN_ROOT}/instances/F4/children/I1/**`
- `{ScopePath}/**`
- targeted reads, `apply_patch`, Rust/Cargo with one private `CARGO_TARGET_DIR`, and the profile selection/scope tools; no network

You exclusively own the nonlinear-integration file until terminal return. Do not edit product physics or any other source, historical witness, dependency, decision, lifecycle, requirement, status, memory, pointer, Git, or sibling artifact. Do not delegate or message siblings; report only to F4 manager.

## Required tests/evidence

- Preserve pre-edit hashes: nonlinear integration `fb02a52273637f844c63ac2e1545bc7fbe8ab79b58d7f5e259829e46be4b1439`; historical M1-N-008 Cargo.lock `ac1abea2...`, Cargo.toml `326677de...`, main.rs `5b467128...`.
- Add exact positive and negative coupled-normal fixtures, both Sticking/Sliding seeds and SparseInteractive/DenseScrutiny modes, matching `u=7/135, N=200/27, Rf=-20/9` and `u=-7/165, N=400/33, Rf=40/11` at the existing fixture comparison scale.
- Cover zero coefficient, zero current normal, nonzero assumed branch ending at zero without retry, active assumed-zero ending nonzero with retry/no false convergence, both nonzero sign flips, cap failure while branch invalid, and at least two coupled derived-normal sliding rows in both input orders with identical physical outputs.
- Retain relevant existing explicit-normal, seed-cap, general cap, TBD-policy, sparse/dense, derived-normal, invalid-input tests. Run `cargo fmt --manifest-path .../nonlinear_integration/Cargo.toml --check` and focused/full crate tests as appropriate using the private target. Preserve any failure exactly and diagnose it; do not weaken tests.
- Return exact changed lines/symbols, before/after source hashes, commands/exits/counts, private-target cleanup/release, write-scope validation, residual risks, and a complete diff artifact under `{ScopePath}`. Mark source as candidate pending fresh independent review; no acceptance claim.

Stop and return immediately for singular coupling not already represented by existing error behavior, unresolved branch cycle at the existing cap, required source-fence expansion, new threshold/reference/history/direction choice, or public-contract change.
