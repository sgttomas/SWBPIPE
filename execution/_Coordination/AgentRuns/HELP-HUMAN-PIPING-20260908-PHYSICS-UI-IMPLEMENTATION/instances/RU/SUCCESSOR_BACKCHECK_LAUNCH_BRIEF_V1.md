# RU sealed launch brief — RU-F1 through RU-F5 successor backcheck

**Status:** `SEALED_FOR_ROOT_DISPATCH`
**Intended instance:** `/root/ui_code_review`, the same independent bounded ephemeral Agent 2 generalist
**Parent:** `/root` (`HELP_HUMAN`, Agent 0)
**Required model:** `gpt-5.6-sol`, high reasoning
**Delegation:** prohibited; no active TaskSkill

## Sealed U7 R2 bindings

This brief succeeds `SUCCESSOR_BACKCHECK_LAUNCH_BRIEF_DRAFT_V1.md`, SHA-256 `952551e4c84717632642f03dedbff8c5312cdf1fe4f7193a28e5b628797df327`; the draft remains unchanged.

DEL-07-01 current-run records:

- `SUCCESSOR_MANIFEST_V2.json`, SHA-256 `72d1cb5f31d7d81e92f05edd6f0b9e8b74c102cc27c898b7139e99644fb529f1`
- `U7_SUCCESSOR_FULL_BASE.diff`, SHA-256 `8c01ab323ef27836ecb1e777366b9166adfe28f9948bd990b01441fc2302f30e`
- `U7_V1_TO_SUCCESSOR.delta`, SHA-256 `09881c4a563bd919a52bd855fc954c798565be9d8817dabfbf36a32793f83863`
- `RU_FIVE_FINDING_CLOSURE_V1.md`, SHA-256 `5601018513ac2d61cdcc59796e14154e290861254658632022f69d63f8424528`
- `SEVEN_ROW_CONSUMER_PROOF_V2.md`, SHA-256 `324acb1aeeff8db2bba679ce55dd41f262561c5bc7f5dff242535971feeb840b`

Manager records:

- `../U7/MANAGER_RETURN_V2.md`, SHA-256 `e90589df6dea42c7acd22e6562cd9933c035d849097d391268231e43299d46ab`
- `../U7/SUCCESSOR_MANAGER_VALIDATION_V1.md`, SHA-256 `dbed0f9b0e4ff7e586a27daf904cb5f5da6757a672a8441f318e091b0eda4175`
- `../U7/children/I1/REPAIR_RETURN_V2.md`, SHA-256 `0aef9be19b342f376266672df88481f45641d7111287fd6588125ef01fe51af9`

Root independently verified all 26 successor-manifest members, the exact live nine-path full diff, the manager return, and unchanged F4 source. U7 and its child are stopped.

Exact final source hashes copied from the verified successor manifest:

- `apps/desktop/src/App.tsx`: `106453b26388f832b2f0cc5683e9e50bf7c2ec9fc50e0576abfd0a3ae2036155`
- `apps/desktop/src/App.test.tsx`: `5204a848a44b75f918108a70dfae075e5c4cbc6f434403fa10ec3da6ecbd05e8`
- `apps/desktop/src/features/viewport/PipeViewport.tsx`: `395d81f29f2a3e827e111dd5077bee30b62d7970b49ec5233abb12877b23dfdf`
- `apps/desktop/src/features/viewport/routeDraft.ts`: `5f03c2bb8af223dca4ddf4e058c7b80676a957ea247fcea59d423fad2ad8884d`
- `apps/desktop/src/features/viewport/routeDraft.test.ts`: `84bca45138fafd18d09601ae5af8f1a5e683026f954c636dcccfc396d5b07bee`
- `apps/desktop/src/features/model-tree/PropertyInspector.tsx`: `a6b63807e99e36a1194e3d01d1b91ce604e3b0478f6b29d9255971c1fb9c3a78`
- `apps/desktop/src/features/model-tree/typedInspector.test.tsx`: `5f42c1d77a479c9cfb00cd9f98d901b7953c4bc7f900dc094e54e2a99c615712`
- `apps/desktop/src/styles.css`: `cefbc458a1f2189268e467608dc9be0e24d6c132234d28586551cfb7a599ca26`
- `apps/desktop/e2e/linear-authoring.spec.ts`: `338625b6d893a2aa47804327cec57127464f4bc600c30556d4ff1946b6faa305`

## Objective and immutable basis

Independently review the complete 1,102-line, six-path U7 R2 successor delta and affected surrounding state, operation-receipt, persistence, and consumer contracts. Close each RU-F1 through RU-F5 finding and every named warning-path evidence gap with affirmative repaired-behavior evidence. Source and repository tests are read-only.

Resolve `REPO_ROOT` with `git rev-parse --show-toplevel` and set `WORKING_ROOT={REPO_ROOT}/projects/chirality-piping`. Bind:

- original RU `REVIEW.md`, SHA-256 `51a2123ed8ada427eaa34bac2aaccd9e92372499fc738b8663bc2ef7b8b9bbc0`
- original RU `RETURN.md`, SHA-256 `481edae2ba1fe5a0227cffab231daca0d59086b769f3c55ebdd7bc784a69b69f`
- original DEL-07-01 `FROZEN_NINE_PATH_MANIFEST_V1.json`, SHA-256 `9f71f16fe91112a42ed01fc69aee040665c4c1663d3e9d5a02d8df00747f4446`
- original DEL-07-01 `U7_FINAL_NINE_PATH.diff`, SHA-256 `df93c21cb945f6ed5d3e42ecbfd8f4f25ef462ba5c453d76757b36d2b2fa2056`
- root repair disposition `../../dispositions/RF_PASS_RU_REPAIR_RELEASE_V1.json`, SHA-256 `2f215d2d0e1695a81cdc6ce9b0ccabc5a011a5973fa97790209539272bbf86e8`

Original U7/RU records remain frozen. The successor must remain inside the same nine-path source/test fence and preserve the accepted structural equality and seven-row consumer behavior.

The original U7 source hashes are historical after the authorized repair. Reconstruct V1 from source base `55df51ac3201456e0f181823e3aefefef47a73bb` plus the frozen original diff when comparison is needed; do not require historical hashes to match current live files.

## Required backcheck

Independently verify with source trace, complete-delta review, and affirmative adversarial probes:

- RU-F1: every control or invalidation path available during delayed Apply either remains disabled or invalidates App publication before completion; cancel/stale callbacks publish no model or checkpoint;
- RU-F2: single and batch Apply require complete, well-formed acceptance evidence and exact frozen operation, step, target, model-basis, and hash bindings; missing, malformed, warning, changed diff, blocked member, or mismatched receipt fields fail closed and require a fresh Add. Verify the existing valid hash `payload_ref` contract is enforced rather than inventing a new equality criterion;
- RU-F3: successful App publication preserves requested continuation from the committed endpoint, while external replacement, open/create, undo/redo, and Cancel still clear it;
- RU-F4: reservation projection requires valid runtime kind, target object type, and string reference, preserves malformed members for authoritative fail-closed validation, and rejects new-node/new-pipe ID collision inside one submission;
- RU-F5: required node, endpoint, and pipe provenance starts empty and Add remains unavailable until the user explicitly enters it;
- object-key-insensitive but array-order-sensitive structural equality remains exact, and all seven consumer fields/contracts remain truthful;
- the successor changes only approved production/test paths, tests demonstrate the repaired outcomes, and no test that merely asserts an old defective outcome is counted as PASS;
- any evidence reuse after the runtime-tested cut is justified only by TypeScript assertions that erase from emitted runtime behavior; verify the exact delta before accepting that reasoning;
- the final-cut browser UI at `1024 x 768` retains the persistent 3D canvas, compact palette, reachable Add/Apply controls, truthful labels/errors, and no required horizontal clipping.

Run focused private JavaScript and browser probes plus the affected frozen tests. Existing prebuilt WASM may be exercised through browser tests only when recorded accurately; do not build Rust, WASM, or native artifacts. Do not run the full sweep or repository harness. The actual isolated native witness remains a separate downstream gate.

Use read/search and scoped JS/browser execution only. Write no source, tests, author packets, original RU records, governance, or Git state. Write only under `instances/RU/successors/RU-F1-F5-BACKCHECK/**`, including concise `REVIEW.md`, `REVIEWED_INVENTORY.sha256`, `VALIDATION.md`, `RETURN.md`, `STATUS.json`, final private browser evidence where needed, and lossless `_run_records/**` for exact failed commands and retries. Clean only task-owned generated artifacts with exact path guards.

Return `PASS` or `CHANGES_REQUIRED` with exact successor bindings, finding-by-finding closure, production defects separated from evidence gaps, command results, final browser provenance, residual risk, actual model/parentage, and confirmation that no source was written. No waiver, new criterion, source acceptance, engineering decision, dependency/lifecycle change, or native claim is permitted; root alone performs fan-in.
