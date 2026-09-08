# RU draft launch brief — U7 product-diff review

**Status:** `DRAFT_NOT_LAUNCHABLE_MISSING_FROZEN_U7_BINDINGS`
**Intended parent:** `/root` (`HELP_HUMAN`, Agent 0)
**Construction:** fresh bounded ephemeral Agent 2 generalist; no active TaskSkill
**Required model:** `gpt-5.6-sol`, high reasoning, fresh context (`fork_turns: none`)
**Delegation:** prohibited

## Pre-dispatch holds

Root must replace and verify every placeholder before sealing or dispatch. This draft is not executable.

- U7 final manifest path and SHA-256: `[HOLD: U7_FINAL_MANIFEST_PATH_AND_SHA256]`
- U7 complete frozen product/test diff path and SHA-256: `[HOLD: U7_COMPLETE_DIFF_PATH_AND_SHA256]`
- U7 manager return path and SHA-256: `[HOLD: U7_MANAGER_RETURN_PATH_AND_SHA256]`
- Exact final hashes for all nine source/test paths: `[HOLD: BIND_FROM_VERIFIED_FINAL_MANIFEST]`
- Confirmation that U7 and all U7 writers stopped and live diff equals the frozen diff: `[HOLD: ROOT_VERIFICATION]`

## Objective and accepted basis

Independently review 100% of the frozen U7 product/test diff for correctness, regression risk, scope and authority compliance, consumed-interface preservation, usable product behavior, and test quality. Report substantive product defects and evidence gaps separately. This is technical review evidence only and grants no source, engineering, dependency, lifecycle, or release acceptance.

Resolve `REPO_ROOT` with `git rev-parse --show-toplevel` and set `WORKING_ROOT={REPO_ROOT}/projects/chirality-piping`. Review only the implementation lane based at `55df51ac3201456e0f181823e3aefefef47a73bb` and the final frozen U7 subject supplied above.

Stable authority and design evidence:

- `../../OWNER_ACT.md`, SHA-256 `3e182eb9cd923f3c74f3e332dc4ae50de8c39de197173bf5b8a1b89daf2c6857`
- `../../amendments/U7/SOURCE_RELEASE_V1.md`, SHA-256 `a81ee0439c11d92360d863bc028bdd72f7f82a524220ad2dda9a348da40c1e46`
- prior accepted U7 implementation brief under DEL-07-01's `PHYSICS_UI_EXECUTION_20260908` run records, SHA-256 `23a2ac4595c4b9ee2e18143b057a392f42c34c57051e5f328b45cbfc2ab1f709`
- prior consumed-interface evidence in the same run-record folder, SHA-256 `93b5be9434a2e34c03002748ecd6c0de4095c7b19d9b99f007b891fb1fd64166`
- prior seven-row factual consumer/interface proof at the execution run's `instances/PS/PS1/RETURN.md`, SHA-256 `93e9fa72f5cf46b1d82e79f114502c3b3a76b1004f0bf3df5fd059aa5729881c`

## Exact review scope

Review the final diff for all and only these nine source/test paths:

- `{WORKING_ROOT}/apps/desktop/src/App.tsx`
- `{WORKING_ROOT}/apps/desktop/src/App.test.tsx`
- `{WORKING_ROOT}/apps/desktop/src/features/viewport/PipeViewport.tsx`
- `{WORKING_ROOT}/apps/desktop/src/features/viewport/routeDraft.ts`
- `{WORKING_ROOT}/apps/desktop/src/features/viewport/routeDraft.test.ts`
- `{WORKING_ROOT}/apps/desktop/src/features/model-tree/PropertyInspector.tsx`
- `{WORKING_ROOT}/apps/desktop/src/features/model-tree/typedInspector.test.tsx`
- `{WORKING_ROOT}/apps/desktop/src/styles.css`
- `{WORKING_ROOT}/apps/desktop/e2e/linear-authoring.spec.ts`

Read, without modifying, the consumed services and interfaces named by the frozen consumed-interface evidence, including operation batch/application, canonical types, persistence, units, section binding, and existing toolkit consumers. Do not inspect unrelated source or any unfinished work.

Use the repository software-code-review method's six steps as guidance only: read the sealed basis and evidence, validate the path fence, trace changed behavior and contracts, assess tests, report actionable findings, and state residual risk without lifecycle acceptance. No TaskSkill contract is loaded or active.

Independently check:

- a pure route builder that reserves IDs across the current model and pending operations, with first-class existing and new endpoint modes;
- exact atomic operation membership/order and one-checkpoint publication for existing-to-existing and existing-to-new straight routes;
- Add freezing the displayed draft/intent/diff and Apply submitting only a passed current validation bound to operation IDs, revision, and model hash; a warning, changed diff, blocked member, missing receipt, stale callback, or hash/revision mismatch must publish no model and require a fresh deliberate action;
- stale, cancel, selection, model-change, open/create, undo/redo, busy, and repeated-activation protection with no duplicate or partial publication;
- node-only save/reopen as explicitly incomplete, full-route persistence, post-commit result clearing, and force-edit undo/redo;
- typed coordinates as authoritative, zero accepted, the disclosed `XZ @ Y=0` pointer plane, no invented coordinates/defaults, and ID collision handling;
- exact units, dimensions, provenance, material/section/rule/checksum truthfulness and preservation of all seven consumer contracts through inline Apply;
- persistent 3D canvas, compact icon-led palette, focus/label/error behavior, and practical Add/Apply use at `1024 x 768`;
- test independence and strength across route-builder, inspector, App, and browser coverage. Browser fallback must not be described as native; the actual native walkthrough is a separate downstream gate outside RU.

Scoped read/search/exec is permitted for review, hashing, targeted JavaScript tests, and browser inspection. Use existing dependencies and task-private output locations. Do not run or trigger Rust, WASM, native builds, the full evidence sweep, or the full repository harness. Preserve failed commands losslessly in RU's own run records. Delete only task-private generated artifacts and record cleanup; never alter product source, tests, author packets, Git state, or sibling records.

## Outputs and verdict

Write only:

- `instances/RU/REVIEW.md`
- `instances/RU/REVIEWED_INVENTORY.sha256`
- `instances/RU/VALIDATION.md`
- `instances/RU/RETURN.md`
- `instances/RU/STATUS.json`
- optional lossless command and generated-artifact records under `instances/RU/_run_records/**`

Report actionable findings with exact file/line, impact, evidence, and remediation direction. Label evidence gaps separately from confirmed defects. End with `PASS` or `CHANGES_REQUIRED`, residual risk, exact reviewed inventory/hashes, commands and results, actual model/parentage, cleanup result, and confirmation that no source was written. Root alone performs fan-in and decides whether later native and integration gates may start.
