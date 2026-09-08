# F4-I2 sealed RF-F4-001 repair brief

Status: `SUPERSEDED_BY_PORTABLE_V2`

Runtime: bounded Agent 2, `gpt-5.6-sol`, high reasoning, no delegation or sibling messages. Report only to F4 WORKING_ITEMS. Use only `{WORKING_ROOT}` as the working lane. The exact originally consumed bytes and SHA-256 are retained in the structural serialized-original record; `SEALED_REPAIR_BRIEF_V2.md` is the active portable successor with identical task semantics.

## Frozen basis

- RF review SHA-256: `3e223c16452e5efe799c2da7e0719919eaa0f2a0aecca3f4ae27a7986abfbb2f`.
- RF return SHA-256: `7399552a5d0139993c1b0ddd7d73ba1ffd1d6946d7954a6d0a0448a42d34ecee`.
- RF lossless trace SHA-256: `c109579a630d70171e942d1cac8d767513d083a77816639195785af6cb8eee81`.
- F4 V1 source SHA-256: `da4cc3f5d21f1e1841d375dd2525f3bb2d3f0e5d3e960bd2389d32cef3a8b320`.
- F4 V1 complete diff SHA-256: `8080a8c242931409863771aba963897a72b055c39aee1d61fd567eee4bb3801b`.
- F4 V1 final manifest SHA-256: `43a48aa6fdbb360d5a7e2a94d09a62d25412e8fde6d39e22dffc9bfcd63119f5`.
- Pre-primary-test byte region SHA-256: `311b4d6657c103982c78d8e86d4efa7dc5d450556802dc3a45d5ce6264bf42ae`.
- Post-primary-test byte region SHA-256: `73463334658edb9ecf09dd48b4b733410e89d1fe95219fb8f485ff3ff7484ea8`.

RF found no production defect. Its sole finding is that the committed active-zero test never constructs an assumed-zero affine row, and that the affine exact-zero solve covers only an assumed-positive branch.

## Exact objective and write fence

Edit tests only inside the existing primary `#[cfg(test)] mod tests` block of `projects/chirality-piping/core/solver/nonlinear_integration/src/lib.rs`. You may add or minimally reshape test fixture helpers and tests needed for `RF-F4-001`. Do not alter any byte before that module or in the later computed-finite test module. Write successor evidence only under:

- `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/F4/successors/RF-F4-001/children/I2/**`
- `projects/chirality-piping/execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-04_Nonlinear support active-set solver/_run_records/PHYSICS_UI_IMPLEMENTATION_20260908/successors/RF-F4-001/**`

No production code, algorithm, tolerance, threshold, convergence rule, public type, diagnostic, dependency, criteria, other source, Git action, or frozen V1 artifact is writable.

## Required proof

1. Add a direct helper-level exercise of a genuinely active derived-normal row whose assumed branch is `+0.0` and then `-0.0`, whose coefficient and existing sliding direction are nonzero, whose affine base normal is exactly zero, and whose final current normal becomes nonzero because another simultaneously solved derived row contributes through cross-coupling. Assert independently that:
   - the zero-branch row's solved/applied tangential force is zero;
   - its final current normal has a concrete nonzero expected value and sign;
   - `derived_normal_branches_admissible` is false;
   - with the real trial-state evaluation and existing caller expression, the iteration cannot be accepted as converged even if the active-set state residual itself is converged.
2. Respect the actual production fallback: when prior source reaction is zero, the branch is selected from the base current source reaction. Therefore construct a base response whose source reaction is exactly zero; a synthetic prior zero paired with a nonzero base normal does not prove the zero branch.
3. Mirror the helper-level affine exact-zero case for an assumed `-1` branch ending at exact current normal zero. Use a mechanically mirrored fixture, not only a predicate assertion. Assert direction, prior source reaction sign, zero applied force, exact zero final normal, and branch admissibility. Preserve the existing assumed-positive case or make one parameterized test cover both signs explicitly.
4. Use exact expectations where algebra gives zero and concrete rational/numeric expectations for the nonzero coupled result. The existing `1e-12` fixture comparison scale may be used only for independently derived nonzero floating values; add no runtime tolerance.

The existing two-row fixture with equal base tangential displacements can support the active-zero proof: choose loads so the first base source reaction cancels exactly while the second row solves nonzero, then use a prior sliding record with nonzero direction evidence and source reaction `+0.0`/`-0.0`. Verify the actual values rather than assuming this hint.

## Verification and return

Run the two new focused tests, the existing current-normal/sign/zero/order/cap tests, `cargo fmt --check`, and the offline locked full nonlinear-integration crate in one private Cargo target. Do not repeat the Python suite or broad evidence sweep. Preserve every real failed attempt with diagnosis; do not weaken assertions. Remove the private target.

Return a complete base-to-successor live diff, a V1-to-successor test-only diff, source hash, region hashes proving only the primary test block changed, commands/results, and a concise status. Stop if any production edit or new mechanics/acceptance choice appears necessary.
