# RU draft launch brief — RU-F1 through RU-F5 successor backcheck

**Status:** `DRAFT_NOT_LAUNCHABLE_MISSING_U7_R2_BINDINGS`
**Intended instance:** `/root/ui_code_review`, the same independent bounded ephemeral Agent 2 generalist
**Parent:** `/root` (`HELP_HUMAN`, Agent 0)
**Required model:** `gpt-5.6-sol`, high reasoning
**Delegation:** prohibited; no active TaskSkill

## Pre-dispatch holds

Root must replace and verify every placeholder before sealing or dispatch.

- R2 final nine source hashes: `[HOLD: U7_R2_FINAL_SOURCE_HASHES]`
- R2 successor manifest path and SHA-256: `[HOLD: U7_R2_SUCCESSOR_MANIFEST]`
- Complete R2 base-to-successor and R1-to-R2 diff paths and SHA-256: `[HOLD: U7_R2_DIFF_BINDINGS]`
- U7 manager successor return path and SHA-256: `[HOLD: U7_R2_MANAGER_RETURN]`
- RU-F1 through RU-F5 closure map path and SHA-256: `[HOLD: U7_R2_CLOSURE_MAP]`
- Root proof that all U7 writers stopped, every successor member rehashes, and the live nine-path diff equals the frozen successor: `[HOLD: ROOT_R2_VERIFICATION]`

## Objective and immutable basis

Independently review 100% of the U7 R2 successor delta and affected surrounding state, operation-receipt, persistence, and consumer contracts. Close each RU-F1 through RU-F5 finding and every named warning-path evidence gap with affirmative repaired-behavior evidence. Source and repository tests are read-only.

Resolve `REPO_ROOT` with `git rev-parse --show-toplevel` and set `WORKING_ROOT={REPO_ROOT}/projects/chirality-piping`. Bind:

- original RU `REVIEW.md`, SHA-256 `51a2123ed8ada427eaa34bac2aaccd9e92372499fc738b8663bc2ef7b8b9bbc0`
- original RU `RETURN.md`, SHA-256 `481edae2ba1fe5a0227cffab231daca0d59086b769f3c55ebdd7bc784a69b69f`
- original DEL-07-01 `FROZEN_NINE_PATH_MANIFEST_V1.json`, SHA-256 `9f71f16fe91112a42ed01fc69aee040665c4c1663d3e9d5a02d8df00747f4446`
- original DEL-07-01 `U7_FINAL_NINE_PATH.diff`, SHA-256 `df93c21cb945f6ed5d3e42ecbfd8f4f25ef462ba5c453d76757b36d2b2fa2056`
- root repair disposition `../../dispositions/RF_PASS_RU_REPAIR_RELEASE_V1.json`, SHA-256 `2f215d2d0e1695a81cdc6ce9b0ccabc5a011a5973fa97790209539272bbf86e8`

Original U7/RU records remain frozen. The successor must remain inside the same nine-path source/test fence and preserve the accepted structural equality and seven-row consumer behavior.

## Required backcheck

Independently verify with source trace, complete-delta review, and affirmative adversarial probes:

- RU-F1: every control or invalidation path available during delayed Apply either remains disabled or invalidates App publication before completion; cancel/stale callbacks publish no model or checkpoint;
- RU-F2: single and batch Apply require complete, well-formed acceptance evidence and exact frozen operation, step, target, model-basis, and hash bindings; missing, malformed, warning, changed diff, blocked member, or mismatched receipt fields fail closed and require a fresh Add;
- RU-F3: successful App publication preserves requested continuation from the committed endpoint, while external replacement, open/create, undo/redo, and Cancel still clear it;
- RU-F4: reservation projection requires valid runtime kind, target object type, and string reference, preserves malformed members for authoritative fail-closed validation, and rejects new-node/new-pipe ID collision inside one submission;
- RU-F5: required node, endpoint, and pipe provenance starts empty and Add remains unavailable until the user explicitly enters it;
- object-key-insensitive but array-order-sensitive structural equality remains exact, and all seven consumer fields/contracts remain truthful;
- the successor changes only approved production/test paths, tests demonstrate the repaired outcomes, and no test that merely asserts an old defective outcome is counted as PASS;
- the final-cut browser UI at `1024 x 768` retains the persistent 3D canvas, compact palette, reachable Add/Apply controls, truthful labels/errors, and no required horizontal clipping.

Run focused private JavaScript and browser probes plus the affected frozen tests. Existing prebuilt WASM may be exercised through browser tests only when recorded accurately; do not build Rust, WASM, or native artifacts. Do not run the full sweep or repository harness. The actual isolated native witness remains a separate downstream gate.

Use read/search and scoped JS/browser execution only. Write no source, tests, author packets, original RU records, governance, or Git state. Write only under `instances/RU/successors/RU-F1-F5-BACKCHECK/**`, including concise `REVIEW.md`, `REVIEWED_INVENTORY.sha256`, `VALIDATION.md`, `RETURN.md`, `STATUS.json`, final private browser evidence where needed, and lossless `_run_records/**` for exact failed commands and retries. Clean only task-owned generated artifacts with exact path guards.

Return `PASS` or `CHANGES_REQUIRED` with exact successor bindings, finding-by-finding closure, production defects separated from evidence gaps, command results, final browser provenance, residual risk, actual model/parentage, and confirmation that no source was written. No waiver, new criterion, source acceptance, engineering decision, dependency/lifecycle change, or native claim is permitted; root alone performs fan-in.
