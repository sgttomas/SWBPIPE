# RU sealed launch brief — R3 residual backcheck

**Status:** `SEALED_FOR_ROOT_DISPATCH`
**Instance:** `/root/ui_code_review`, the same independent bounded ephemeral Agent 2 generalist
**Parent:** `/root` (`HELP_HUMAN`, Agent 0)
**Model:** `gpt-5.6-sol`, high reasoning
**Delegation:** prohibited; no active TaskSkill

## Sealed R3 subject

This brief succeeds `SUCCESSOR_BACKCHECK_LAUNCH_BRIEF_V1.md`, SHA-256 `951051bf9a4d78b21795d05db82dc64a014fdfd35673180ab0d5967a8f9f55ab`. The predecessor and all reviewed snapshots remain unchanged.

DEL-07-01 current-run records:

- `R3_SUCCESSOR_MANIFEST_V3.json`, SHA-256 `bd5cdbdb12a71179df68305ef375071743607e91bea0f9d3fac074240247ca30`
- `U7_R3_FULL_BASE.diff`, SHA-256 `6e013cf88417d02c1f1f75d48fac124ba635f8e39b974c6d2cda2477b01b6b10`
- `U7_R2_TO_R3.delta`, SHA-256 `a0ae4a385ed9b76d8fa10974df218525b555732e333d4c8964ac76714b3a21fb`
- `RU_RESIDUAL_CLOSURE_V2.md`, SHA-256 `21184102fe38ab0b4dabbe5c7fbbeb66cad5e6401b26557590017797b896b415`
- `SEVEN_ROW_CONSUMER_PROOF_V3.md`, SHA-256 `d63d773439889fae06cb356ed91d8246d62d120ddd8eeade72813df9310739c0`

Manager and review basis:

- `../U7/MANAGER_RETURN_V3.md`, SHA-256 `b447bbf0660ea25b6a37942dab871215280becbd945feb11f5b6547b92757a88`
- `../U7/R3_MANAGER_VALIDATION_V1.md`, SHA-256 `be54303f1cb9018ad198d200758448b88ac64fdb2b0109ec40df44e5e26de8cc`
- prior R2 backcheck `successors/RU-F1-F5-BACKCHECK/REVIEW.md`, SHA-256 `54dd95c5f4410974060430221c16bdc3bd83bbb39964ea0622fc64fdd03f8f18`
- prior R2 backcheck `successors/RU-F1-F5-BACKCHECK/RETURN.md`, SHA-256 `044f422f3ac5fcb9cfe92f7bcc2e78c79b2ebe6ff93a9c96d836a3a0d1eaf2b4`
- root R3 repair disposition `../../dispositions/RU_R3_REPAIR_RELEASE_V1.json`, SHA-256 `c300850a2b85e7e861df23bd6a732e704472aec10975bfb921dfa28f4bace1ff`

Root read the R3 manifest, return, validation, closure, and seven-row proof and independently verified all 27 source/evidence members, the manager return, and exact live tracked/untracked nine-path diff. All source writers are stopped; frozen F4 source SHA-256 `6e163a47db60288179f844d473992d28a53599ad2204149093186c760723ad46` is unchanged.

Exact final U7 source hashes:

- `apps/desktop/src/App.tsx`: `a9217c959bfc2979e21cde5d5baf865f75258f03b902ecdd2c0fd545e5bfbde9`
- `apps/desktop/src/App.test.tsx`: `629037c503c6cf9b32ef3cc6b59e3c04234ec2c8ad61879e6ba73ef43cd3a1be`
- `apps/desktop/src/features/viewport/PipeViewport.tsx`: `ac3789a6b4e8c452fe398d062183b25d884f55f5763af877ac6f1b3d4cb9f2ad`
- `apps/desktop/src/features/viewport/routeDraft.ts`: `cb258db1ab0750b891a023c0723cfbfdb55a898bb4930d4aaf4137cc6992c7a4`
- `apps/desktop/src/features/viewport/routeDraft.test.ts`: `9de95189460a18bc67e231308c641978f38ee1d971f9e6c6343e0b5969d94633`
- `apps/desktop/src/features/model-tree/PropertyInspector.tsx`: `a6b63807e99e36a1194e3d01d1b91ce604e3b0478f6b29d9255971c1fb9c3a78`
- `apps/desktop/src/features/model-tree/typedInspector.test.tsx`: `5f42c1d77a479c9cfb00cd9f98d901b7953c4bc7f900dc094e54e2a99c615712`
- `apps/desktop/src/styles.css`: `cefbc458a1f2189268e467608dc9be0e24d6c132234d28586551cfb7a599ca26`
- `apps/desktop/e2e/linear-authoring.spec.ts`: `338625b6d893a2aa47804327cec57127464f4bc600c30556d4ff1946b6faa305`

## Objective and checks

Review the complete 478-line, five-path R2-to-R3 delta, affected surrounding receipt/state contracts, and exact live reconstruction. Independently close only residual `RU-BC-F1` and `RU-BC-F2`; confirm prior `RU-F1`, `RU-F4`, and `RU-F5` closures remain intact.

- For `RU-BC-F1`, trace the actual producer contract and prove that single and batch Apply validate the existing hash `payload_ref`, model basis, operation/step/target identities, warnings, diffs, blocked members, receipt completeness, and malformed/mismatched responses without inventing a new equality criterion.
- For `RU-BC-F2`, prove continuation survives only the route's own accepted commit and is cleared by external replacement, open/create, undo/redo, and Cancel without stale or partial publication.
- Run focused private adversarial probes that assert repaired fail-closed and continuation outcomes. Do not count tests that assert a defective outcome as PASS.
- Verify structural equality, malformed-member preservation, provenance entry, reservation collision checks, seven-row consumers, exact nine-path containment, and final `1024 x 768` browser behavior remain valid.

Use read/search and necessary focused JavaScript/browser checks. Existing prebuilt WASM may run through browser tests when disclosed accurately. Do not build Rust, WASM, or native artifacts; do not run the full suite or sweep; do not write source, tests, Git, manager records, or earlier RU records.

Write only under `instances/RU/successors/RU-R3-BACKCHECK/**`: concise `REVIEW.md`, `REVIEWED_INVENTORY.sha256`, `VALIDATION.md`, `RETURN.md`, `STATUS.json`, and lossless `_run_records/**` for failed commands or task-owned browser evidence. Clean task-owned artifacts with exact path guards.

Return `PASS` or `CHANGES_REQUIRED` with exact bindings, finding-by-finding closure, production defects separated from evidence gaps, commands and results, cleanup, residual risk, actual model/parentage, and confirmation that no source was written. Native, RI_PRECHECK, RI_FINAL, integration, DEC-025, and publication remain held pending same-RU PASS and root release.
