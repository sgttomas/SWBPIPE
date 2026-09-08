# RU sealed launch brief — U7 product-diff review

**Status:** `SEALED_FOR_ROOT_DISPATCH`
**Intended parent:** `/root` (`HELP_HUMAN`, Agent 0)
**Construction:** fresh bounded ephemeral Agent 2 generalist; no active TaskSkill
**Required model:** `gpt-5.6-sol`, high reasoning, fresh context (`fork_turns: none`)
**Delegation:** prohibited

## Sealed U7 bindings

This brief succeeds `LAUNCH_BRIEF_DRAFT_V1.md`, SHA-256 `aecb161214dc1e5deba7ef6a0f2eee47ba677d61a936bec8f343a441d9623cea`; the draft remains unchanged.

- DEL-07-01 current-run `FROZEN_NINE_PATH_MANIFEST_V1.json`, SHA-256 `9f71f16fe91112a42ed01fc69aee040665c4c1663d3e9d5a02d8df00747f4446`
- DEL-07-01 current-run `U7_FINAL_NINE_PATH.diff`, SHA-256 `df93c21cb945f6ed5d3e42ecbfd8f4f25ef462ba5c453d76757b36d2b2fa2056`
- DEL-07-01 current-run `SEVEN_ROW_CONSUMER_PROOF_V1.md`, SHA-256 `d7ac33162d291c6a22c17762e5895b5f96c9fd22d5d39020f9c40f2b7e3b4da2`
- `../U7/MANAGER_RETURN_V1.md`, SHA-256 `e5b4e7ab6ad169a456b8ad0b0c2d02a233931f351770d74c7bd19254843c9471`
- `../U7/children/I1/REPAIR_RETURN_V1.md`, SHA-256 `0307c1d9dff3e044176a0a1ece582aa85e08d9e38905cbc46a22bf79c3e0f962`

Root independently verified all 19 manifest members and the exact nine-path diff: tracked chunks equal the Git diff and new-file chunks reconstruct the live bytes. Root read the complete manager return and seven-row proof. U7 and all U7 writers are stopped.

Exact final source hashes copied from the verified manifest:

- `apps/desktop/src/App.tsx`: `58a2c502ac62cd94744ae24d61123da751d53b25be8d8604174a2112d55e1ee7`
- `apps/desktop/src/App.test.tsx`: `3f7471c594d0551724b7ba7bfeaea2508c7abe23d4f88c678c2d25a2fbeab5b7`
- `apps/desktop/src/features/viewport/PipeViewport.tsx`: `e28eedb99d0a6ec1f59ac1e80c119608d92c107ad3b3371619661c8eaa831db2`
- `apps/desktop/src/features/viewport/routeDraft.ts`: `9ee118453f0c0a9ceda55403b40412fa5bd9582f652b4d325729bd2753520dda`
- `apps/desktop/src/features/viewport/routeDraft.test.ts`: `9be8876089630acd2db7bb723252f5c5548a6b11c3c176254ef0c198f215d85f`
- `apps/desktop/src/features/model-tree/PropertyInspector.tsx`: `a6b63807e99e36a1194e3d01d1b91ce604e3b0478f6b29d9255971c1fb9c3a78`
- `apps/desktop/src/features/model-tree/typedInspector.test.tsx`: `5f42c1d77a479c9cfb00cd9f98d901b7953c4bc7f900dc094e54e2a99c615712`
- `apps/desktop/src/styles.css`: `f7a4d3fea4781e4a7cc5d6b6072360a5c95d60cbc1748a17bab898c6380b1820`
- `apps/desktop/e2e/linear-authoring.spec.ts`: `338625b6d893a2aa47804327cec57127464f4bc600c30556d4ff1946b6faa305`

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
- structural operation equality must ignore object-key order while preserving array order and exact values; independently test reordered object keys and a substantive operation mutation;
- restored malformed batch members must be checked for runtime kind, target, and reference before reservation projection; malformed members may be ignored only for ID reservation and must remain stored/requeued evidence that reaches authoritative validation and fails closed without dereference, filtering, rewriting, reservation, or acceptance;
- exact units, dimensions, provenance, material/section/rule/checksum truthfulness and preservation of all seven consumer contracts through inline Apply;
- persistent 3D canvas, compact icon-led palette, focus/label/error behavior, and practical Add/Apply use at `1024 x 768`;
- test independence and strength across route-builder, inspector, App, and browser coverage, including the final structural-equality and malformed-member regressions;
- honest provenance: U7's direct manager patch and later sealed I1/R1 remediation remain disclosed and independently checked. The retained `1024 x 768` screenshot is pre-remediation browser evidence only; it is neither final-cut nor native evidence. The actual native walkthrough is a separate downstream gate outside RU.

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
