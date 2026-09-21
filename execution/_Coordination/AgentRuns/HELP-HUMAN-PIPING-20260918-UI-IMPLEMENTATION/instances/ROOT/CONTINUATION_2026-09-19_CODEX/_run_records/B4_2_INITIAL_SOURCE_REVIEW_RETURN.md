**R1 — P2: Value assertions still mutate review-editor state.**

At [ModelTree.test.tsx:64](/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-wt3/projects/chirality-piping/apps/desktop/src/features/model-tree/ModelTree.test.tsx:64), assertions call `reviewEditor(...)`, whose implementation clicks **Keep draft** and double-clicks another cell. This also occurs at lines 69–70, 76–77 and 86, and [ModelTree.table.test.tsx:29](/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-wt3/projects/chirality-piping/apps/desktop/src/features/workspace/table/ModelTree.table.test.tsx:29) and line 143.

These reads change which editor is active before subsequent Tree/filter/Queue/generation steps. They therefore test a different interaction sequence from the original passive value assertions and can read retained editor text instead of the current cell projection. The earlier connected failures already demonstrate this helper’s ability to change what later assertions observe.

Replace assertion-only uses with a nonmutating accessor that reads the requested cell button or its specifically identified active input. Keep editor-opening helpers only for deliberate editing actions. Preserve every expected value, Queue count and payload assertion; rerun the affected tests and obtain a bounded backcheck. This follows the sealed brief’s explicit restriction against helper-driven edits merely to read values.

**Disposition:** Complete frozen-diff review with one actionable verification finding. I would hold fan-in pending R1’s correction/backcheck. No additional product correctness defect was established in this review.

Reviewed candidate: `15e8b7a72ce0ed440f1f46b9b6ca83503af9c02c..a1833393e597acbe81f943accf64a9548035fe6a`. All 15 maintained paths exactly match `SOURCE_FREEZE.json`; all recorded SHA-256 values match Git-pinned bytes. Evidence commit `deb50f5db6e3dc4b89e88aaca34449357647cab9` has an empty desktop diff against `a183`.

Coverage included:

- Five product files: `ModelTree.tsx`, `EngineeringTable.tsx`, `tableState.ts`, `modelTableAdapter.ts`, `styles.css`.
- Six unit-test files: `App.test.tsx`, `App.projectHandlers.test.tsx`, `ModelTree.test.tsx`, `DisplayIntegration.test.tsx`, `EngineeringTable.test.tsx`, `ModelTree.table.test.tsx`.
- Four browser-test files: `b3a-session-status.spec.ts`, `b4-table-editing.spec.ts`, `r2-smoke.spec.ts`, `ui-foundation.spec.ts`.

The product inspection covered captured direct before/unit/generation, trimmed text and exact normalized no-op comparison, blank/TBD handling, rejection and asynchronous ownership, non-string provenance guards, raw review staging, current-model Queue basis, filtered draft retention, Clear/reset callbacks, unit drift, converted canonical readouts, ordered selection, hidden/inert instances, keyboard behavior, sorting, focus recovery and bounded layout. The engine’s unchanged `FieldKind::Text` implementation supports the direct text contract. No controller, engine, schema, shared VirtualList, native or CI-policy change appears in the maintained diff.

The remaining test adaptations preserve their numeric, target/unit, Queue, history, hash, result and multiselection assertions. The Sort-Z locator correction and measured pointer placement preserve the existing alignment, rectangle, boundary and timeout oracles. The new cumulative application assertion correctly checks three prior direct applications plus exactly one review application.

Recorded evidence supports **117 core passes, 25 connected passes with 233 skipped, and 9 final guard passes**. Final TypeScript success is supported by the command/exit record; its log is empty. Browser logs show **14/18**, then **9/10**, then **1/1**. Together they contain passing observations for the 18 selected identities across revisions; they are not a final-source 18-case run. Carry-forward is reasonable as bounded supporting evidence, with the final core hash already present in the repair manifest and the later product change limited to the defensive provenance guard. Intermediate causal explanations remain retained records rather than independently rerun observations.

I verified all **68 artifact-manifest entries**, including seven trace archives, against committed bytes. Earlier failures and collection mistakes remain retained. I executed no tests, builds, UI actions, network/provider requests, Git mutations, resource claims or delegation. Native verification, the clean DEC-025 sweep and actual-head CI remain pending; this review establishes no full B4, usability, engineering or lifecycle acceptance.

Actual origins and consulted identity hashes follow. `R` is `/Users/ryan/.codex/worktrees/b3d208ad-bb6f-4bed-aaab-c567e28cbe23/chirality`; `W` is `/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-wt3`. Both repository roots were resolved with Git; each working root is its `projects/chirality-piping` directory.

| Consulted instruction/basis in R | SHA-256 |
|---|---|
| `AGENTS.md` | `d151dad92a074abebf8e6225c92c4c6e88fd586f2283377b50b5051eff39be7b` |
| `agents/AGENT_TASK.md` | `1a13a5b00b3ce01ff8519efe6b46bcbe0cd6a5b7985e24282fa7efa2c57c8fb7` |
| Piping `AGENTS.md` | `eec1b9accc5dc4485fd0aab423678bed4c4c98396dc88b9089703965eb3d2005` |
| Piping `loop/LOOP_INIT.md` | `f327d5c6fe4d964a15786813f6de72170bc5e94471753846f461f622281e5f94` |
| `.agents/skills/software-code-review/SKILL.md` | `06c27b1be5cfbd9e638570918a8f837d8439c8073c40d3ef708e53874f95570a` |
| `B4_2_INDEPENDENT_SOURCE_REVIEW_BRIEF.md` | `fc0ab7a77acc2f5ed063308d4c2ef94658099a7dba93a304f0e88a322493a937` |
| `B4_2_MANAGER_BRIEF.md` | `9d534b90967995ddf67dd81bef9afc6ff7e7a9bac0d711afa91272d7cf0c72f0` |
| `B4_2_SEMANTIC_CLARIFICATION.md` | `7fd4ba6f56c1941823f90fbb488159efd2f624d33f53afb2d091d5e8d37a833a` |
| `B4_2_REVIEW_UNIT_CLARIFICATION.md` | `e0628e173b3dfecc34f3f692823e7dbafaa06d2b5a4296fb4eaa63c6874fca9c` |
| `B4_1_COMPATIBILITY_DISPOSITION.md` | `f317cddf86edc5faf672629f70c0575d26ff77e2b3b494d7e167687e42865827` |
| `B4_1_VERTICAL_SCROLL_REPAIR_DISPOSITION.md` | `b125b5b7b5fefa7011e504c891f440edcd22801f361ccd8778d770de28a072b6` |
| `B4_REMAINDER_PREPARATION_RETURN.md` | `065aeb1bc0bcb74752612fbc7f27913aa7f7ae104a75071867822dc2ddbde4ef` |
| `B4_REMAINDER_CLARIFICATION_RETURN.md` | `765e619dc83014bd8d451188c787d8152b01ed116e873a76e003492d5a89e8e2` |
| Owner control-layer-first direction | `bc41f82680cfcb69a193ee3820cb0861f36fbe872fae970df1be1729f9a23268` |
| Owner eight-UX-items direction | `5b86c2783c5d460adf63b69dc3a49db7df672795f0feeb59079a2f93a82c0f4e` |
| `UX_SPEC_V1.md`, applicable sections | `2141c1e844109d4869287048c695e79677f423acae965d5dd0436b7f6aeff682` |

Decision-bearing source/evidence identities in W:

| Record | SHA-256 |
|---|---|
| Engine `operation_applier/src/lib.rs` | `22062ea42fd135a9dbefa0f7e09cf3a2feccc5193ff6028818fd1c6221a3f355` |
| `SOURCE_FREEZE.json` — complete maintained hash set | `d6122068807c2a984fab19889627c06e3a7fb3ad2c08200761b7e5566464d6f2` |
| `TASK_BRIEF.md` | `965a3dfa57ac60c4ee608bb81018a36a838717d6961d7a1d4ecc2148ec9fb51c` |
| `ACTUAL_LAUNCH.json` | `0618651a462a486914d190681974ac7302ef698c466678afab4b986bc1907a85` |
| `LAUNCH_INPUTS.json` | `bda93ff14eb3771e261be5028ee6d975103c5d3ff539f08588528fe79c892ce0` |
| `MANAGER_SOURCE_HANDOFF.md` | `81757d885f0e88a0ed7b9645230b79e1a10dbdae9c95b68bb9232141b7728b38` |
| Worker `RETURN.md` | `9c6723c79318b97b02a18b36fb2b77fe6cd6e8716b0eda8ae403ccbe784f30e4` |
| `CHECK_NOTES.md` | `ab7e8085c21e9e07198479dcc4a3107ea465fa3cd6852a3c10b965de8a3fcd3c` |
| `CHECK_COMMANDS.json` | `26f85e938ea372e9aa5b22838c0b8f90653d4147eb0f4930637519e0a1f667d1` |
| `ARTIFACT_HASHES.json` — all 68 payloads verified | `a0473f26c723e4d9df097343d6a6373d59519353443d4262837d5fbcb3547afe` |
| `browser-selected-source.json` | `3e4808f95b5c2e739eb2acfa3a341a861ab34e1740c49efc7210914db7901efe` |
| `browser-repair-source.json` | `101285ec19b788450c71a00d1bcd2465cf917b826f755cd5d1b4f005893ca4b7` |

Review attribution: fresh TASK `/root/b4_2_code_review`, gpt-6-astra/xhigh, Codex harness-native child of ROOT HELP_HUMAN; no descendants. Independence is from the author and implementation context, not model diversity.
