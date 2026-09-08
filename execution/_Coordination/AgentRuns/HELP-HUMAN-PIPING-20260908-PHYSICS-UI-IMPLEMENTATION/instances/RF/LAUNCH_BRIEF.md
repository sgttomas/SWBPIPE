# RF sealed launch brief — F4 product-diff review

**Status:** `SEALED_FOR_ROOT_DISPATCH`
**Parent:** `/root` (`HELP_HUMAN`, Agent 0)
**Role:** fresh bounded Agent 2 using `TASK + software-code-review`
**Model:** `gpt-5.6-sol`, high reasoning, fresh context (`fork_turns: none`)
**Delegation:** prohibited

## Objective and basis

Independently review 100% of the frozen F4 product/test diff for correctness, regression risk, scope and authority compliance, public-contract preservation, and test quality. This is technical review evidence only; it grants no lifecycle, engineering, dependency, or release acceptance.

Resolve `REPO_ROOT` with `git rev-parse --show-toplevel` and set `WORKING_ROOT={REPO_ROOT}/projects/chirality-piping`. Work only in the implementation lane based at `55df51ac3201456e0f181823e3aefefef47a73bb`.

Authority and exact subject:

- `../../OWNER_ACT.md`, SHA-256 `3e182eb9cd923f3c74f3e332dc4ae50de8c39de197173bf5b8a1b89daf2c6857`
- `../../amendments/F4/SOURCE_RELEASE_V1.md`, SHA-256 `38cdd1de98f0d4b9405a5f92fb5488b305af3779b3ce1ad603d46d79f9222933`
- prior run `instances/F4/IMPLEMENTATION_BRIEF_V2.md`, SHA-256 `63c0361979c20474cddaada74076705a495a60da1c2d5cb0548a10ac6fd7974c`
- `../F4/FINAL_MANIFEST.json`, SHA-256 `43a48aa6fdbb360d5a7e2a94d09a62d25412e8fde6d39e22dffc9bfcd63119f5`; root independently verified all 14 members and live-diff equality
- sole changed source: `{WORKING_ROOT}/core/solver/nonlinear_integration/src/lib.rs`, SHA-256 `da4cc3f5d21f1e1841d375dd2525f3bb2d3f0e5d3e960bd2389d32cef3a8b320`
- complete matching diff: the `CANDIDATE_DIFF.patch` member of the subject manifest, SHA-256 `8080a8c242931409863771aba963897a72b055c39aee1d61fd567eee4bb3801b`

F4 and its child are stopped and the Rust slot is free. U7 is still writing only its disjoint App fence; do not inspect, hash, test, or interpret unfinished U7 changes.

## Review method and required analysis

Read the repository `skills/software-code-review/SKILL.md`, the authority records above, every subject-manifest member, the complete base-to-live F4 diff, and the relevant stable callers/interfaces in `frame_kernel`, `nonlinear_supports`, `diagnostics`, and `product_physics`. Validate the changed-path fence before judging behavior.

Independently trace and check:

- the affine current-normal derivation, matrix signs, dimensions, and use of the final reported-reaction convention;
- base subtraction when prescribed displacements or explicit-normal friction loads create an affine reaction offset;
- positive, negative, and exact-zero normal branches, including closed-halfspace admissibility and signed-zero handling;
- unchanged first-iterate direction deferral and unchanged public sliding-direction rule;
- simultaneous coupled rows, permutation/order independence, and absence of a sequential order-dependent update;
- honest existing-cap nonconvergence, finite-value checks, and singular-system failure propagation;
- preservation of public structures, canonical SI, history, diagnostics vocabulary, result mapping, conversion constants, convergence controls, thresholds, and solve-mode evidence;
- test quality: independently grounded rational oracles, both signs/seeds/modes, exact-zero and sign-flip cases, explicit-normal cases, cap failure, singular/non-finite behavior, and coupled rows. Confirm the order-independence assertion correction changes comparison rounding only and does not weaken the physical oracle or hide order sensitivity.

Reproduce the focused F4 tests and unchanged rational witness as needed. Use an exclusive task-private Rust target outside the repository, record the exact commands and results, and remove it after review. Do not run the full evidence sweep, a full repository harness, native/WASM builds, or U7 tests. Preserve any failed command output losslessly in RF's own run records; do not erase or reinterpret failures as passes.

## Permissions and outputs

Tools: read/search and command execution for review, hashing, focused tests, and private temporary artifacts. Source, tests, F4 author packets, U7 files, governance, DAG/dependency state, and Git state are read-only. Write only:

- `instances/RF/REVIEW.md`
- `instances/RF/REVIEWED_INVENTORY.sha256`
- `instances/RF/VALIDATION.md`
- `instances/RF/RETURN.md`
- `instances/RF/STATUS.json`
- optional lossless command records under `instances/RF/_run_records/**`

Report only actionable findings with exact file and line, impact, evidence, and remediation direction. End with an unambiguous `PASS` or `CHANGES_REQUIRED`, residual risk, exact reviewed inventory/hashes, command outcomes, actual model/parentage, and confirmation that no source was written. RF does not accept F4 or clear the integration gate; root performs fan-in after reading the return.
