# RF sealed launch brief — RF-F4-001 successor backcheck

**Status:** `SEALED_FOR_ROOT_DISPATCH`
**Intended parent:** `/root` (`HELP_HUMAN`, Agent 0)
**Construction:** same independent RF bounded ephemeral Agent 2 generalist; no active TaskSkill
**Required model:** `gpt-5.6-sol`, high reasoning
**Delegation:** prohibited

## Sealed successor bindings

This brief succeeds `SUCCESSOR_BACKCHECK_LAUNCH_BRIEF_DRAFT_V1.md`, SHA-256 `832768769ef183774d314c24de75cfe02438025f99b5086982de34da61e64f2b`; the draft remains unchanged.

- `../F4/successors/RF-F4-001/SUCCESSOR_MANIFEST.json`, SHA-256 `c532b4ac8815545bca286694be35239fcd117e37fb00cb371b6a535d926ae623`
- `../F4/successors/RF-F4-001/RETURN.md`, SHA-256 `aa4442494138fa9ae93b5ba9f801a81b8f1f26fc809abb5f5aef3ab58d0d3605`
- `{WORKING_ROOT}/core/solver/nonlinear_integration/src/lib.rs`, SHA-256 `6e163a47db60288179f844d473992d28a53599ad2204149093186c760723ad46`
- DEL-04-04 current-run successor `MANAGER_VALIDATION.md`, SHA-256 `77fd71b89ae97cc4b14f43f04737f14cf248e9f673742b19bfdc33a28a1ec2b9`
- DEL-04-04 current-run successor `CANDIDATE_DIFF_BASE_TO_SUCCESSOR.patch`, SHA-256 `ff74ab001f108f36a6e0f4105b4fe40a7a7d784045e4fc0827518074a62d97cc`
- DEL-04-04 current-run successor `CANDIDATE_DIFF_V1_TO_SUCCESSOR.patch`, SHA-256 `9bdf1403176e697534c4a373250d4f81930336065b13d7ee79a618e21aa1d09e`

Root independently verified all 13 successor-manifest records, the manifest hash, live source equality, both complete patch bindings, and the manager return. F4 is stopped. Root grants RF the exclusive Rust slot upon dispatch; U7 performs no Rust work and CHANGE is stopped.

## Objective and immutable basis

Independently backcheck the complete RF-F4-001 successor. Verify that the successor delta is test-only, production behavior is byte-identical to the already reviewed F4 production implementation, and the repaired tests genuinely close the missing branch evidence. Tests and product source are read-only.

Resolve `REPO_ROOT` with `git rev-parse --show-toplevel` and set `WORKING_ROOT={REPO_ROOT}/projects/chirality-piping`. Bind:

- original RF review `REVIEW.md`, SHA-256 `3e223c16452e5efe799c2da7e0719919eaa0f2a0aecca3f4ae27a7986abfbb2f`
- original RF `RETURN.md`, SHA-256 `7399552a5d0139993c1b0ddd7d73ba1ffd1d6946d7954a6d0a0448a42d34ecee`
- corrected portable RF `VALIDATION.md`, SHA-256 `7a85d4ef11f7d8d2d84f8b0c56ccb3706f82577a337d2eeec9ef8d87376949e2`
- RF portability correction manifest `VALIDATION_PORTABILITY_CORRECTION.json`, SHA-256 `faeb483f9dfa02e2ef035f5991c86bb0cff05d1c9ba6e5bf57854ff95b29c247`
- prior reviewed F4 production source SHA-256 `da4cc3f5d21f1e1841d375dd2525f3bb2d3f0e5d3e960bd2389d32cef3a8b320` and complete diff SHA-256 `8080a8c242931409863771aba963897a72b055c39aee1d61fd567eee4bb3801b`

Reuse the prior production review only if independent comparison proves that every successor change lies inside test code and all production bytes are identical to the previously reviewed implementation. Otherwise return `CHANGES_REQUIRED` without carrying the prior production assessment forward.

## Required backcheck

Review 100% of the successor delta and independently verify:

- direct active-zero fixtures create synthetic prior sliding iterations with derived source reactions of both `+0.0` and `-0.0`, then exercise base-sign fallback, affine force construction, branch admissibility, and the caller convergence gate with independently derived expected values;
- each active zero assumption with nonzero current normal is rejected and cannot produce false convergence;
- a negative assumed branch whose final current normal is exactly zero is accepted in both sparse and dense solve modes;
- assertions prove the intended branch path and caller behavior rather than only testing a predicate truth table;
- existing rational/current-normal, coupled/order, cap, finite/singular, explicit-normal, signed-zero, seed, and solve-mode coverage remains intact and is not weakened.

After root releases the Rust slot, use an exclusive task-private Cargo target outside the repository. Run the repaired focused tests, the unchanged rational witness, the full locked/offline `nonlinear_integration` crate tests, and formatting check. Record exact commands and results, preserve failures losslessly, and clean only the task-private target with narrow identity guards. Do not run the full evidence sweep, repository harness, native/WASM builds, or U7 tests.

## Permissions and outputs

Read/search/exec is allowed for hashing, diff analysis, and scoped Rust verification. Do not modify source, tests, F4 author packets, original RF records, U7 files, governance, or Git state. Write only under `instances/RF/successors/RF-F4-001-BACKCHECK/**`, including concise `REVIEW.md`, `REVIEWED_INVENTORY.sha256`, `VALIDATION.md`, `RETURN.md`, `STATUS.json`, and optional lossless command records.

Return `PASS` or `CHANGES_REQUIRED` with exact successor bindings, production-identity proof, independent expected values, command outcomes, residual risk, actual model/parentage, and confirmation that no source was written. No waiver, new criterion, engineering adoption, lifecycle acceptance, or integration release is permitted; root alone performs fan-in.
