Review complete: **20/20 changed paths reviewed** for `fd195cf4287e84572a12183169478a6f7ddf6a92..87faf4b240d1be8b197843cd869b22408e85be3b`. **Not ready for fan-in:** two additional P2 findings and the known Tab defect remain on this frozen revision.

1. **P2 — Filtering away the focused row removes keyboard access to coordinates.** [EngineeringTable.tsx:178](/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-wt3/projects/chirality-piping/apps/desktop/src/features/workspace/table/EngineeringTable.tsx:178). Focus coordinate A without editing, then filter to exclude A. `focused` remains A, so every visible coordinate receives `tabIndex=-1`. Row-header buttons only select rows and provide no coordinate-navigation handler. Keyboard users cannot enter the remaining coordinates until a pointer click. Deleting the focused row produces the same condition. Reconcile the roving Tab target with visible live cells while preserving external DOM focus and any captured editor. Add filter/deletion re-entry coverage.

2. **P2 — Deferred navigation can overwrite a newer selection that does not move DOM focus.** [EngineeringTable.tsx:104](/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-wt3/projects/chirality-piping/apps/desktop/src/features/workspace/table/EngineeringTable.tsx:104), continuation at line 126. Start a delayed A→B Enter apply, then update selection to C while the editor remains focused. `canFocus()` checks only DOM element equality; completion calls `focusCell(B)`, including `onSelect(B)`, overriding C. Minimal deterministic reproduction: retain the existing test’s `selectedKey=C` rerender at lines 125–135 but omit `outside.focus()`. Bind navigation to selection ownership as well as focus ownership. This is source-confirmed; I did not execute the reproduction.

3. **Known P2 — Last unchanged editor’s Tab returns focus to its coordinate.** [EngineeringTable.tsx:145](/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-wt3/projects/chirality-piping/apps/desktop/src/features/workspace/table/EngineeringTable.tsx:145). Already under manager repair. Both browser profiles fail the unchanged assertion at `e2e/b4-table-editing.spec.ts:35`. Later repair bytes are outside this review.

The shared intent extraction preserves bulk semantics and attribution. All seven legacy-consumer changes only add disclosure-opening setup; their original numeric, target, unit, history, stale-response, geometry and result-standing assertions remain intact. The controller retains its revision/hash guards, exactly-once busy ownership, and ordinary editing during `projectBusy`. Generation-feedback cleanup matches the bounded correction.

I independently compared the freeze02 manifest with committed bytes: exact 20-path set, **zero hash mismatches**. Retained logs show 130 focused tests and 17 selected App tests at freeze01, followed by 14 interaction tests for freeze02. TypeScript success is recorded; its output files are empty. Browser `new-01` records 2 passes and 2 failures; the failing journeys did not reach Save/Open. Six legacy browser cases, native verification, final sweep and combined-candidate checks remain unavailable here. I ran no tests, builds, UI/CUA, ports, network operations or mutations.

Actual role: `/root/b4_1_code_review`, fresh TASK Type2, Codex delegated-harness-native; supplied model/effort `gpt-6-astra/xhigh`, same-model independence. No descendants.

Context origins and verified SHA256s:

| Origin | SHA256 |
|---|---|
| ROOT checkout `AGENTS.md` | `d151dad92a074abebf8e6225c92c4c6e88fd586f2283377b50b5051eff39be7b` |
| ROOT `projects/chirality-piping/AGENTS.md` | `eec1b9accc5dc4485fd0aab423678bed4c4c98396dc88b9089703965eb3d2005` |
| ROOT `agents/AGENT_TASK.md` | `1a13a5b00b3ce01ff8519efe6b46bcbe0cd6a5b7985e24282fa7efa2c57c8fb7` |
| ROOT Piping `loop/LOOP_INIT.md` | `f327d5c6fe4d964a15786813f6de72170bc5e94471753846f461f622281e5f94` |
| ROOT `.agents/skills/software-code-review/SKILL.md` | `06c27b1be5cfbd9e638570918a8f837d8439c8073c40d3ef708e53874f95570a` |
| ROOT continuation `B4_1_INDEPENDENT_REVIEW_BRIEF.md` | `708d04fbaa223d8519ea15361ed146a6b7e24213973a07834909d92721a0aae4` |
| ROOT continuation `B4_1_MANAGER_BRIEF.md` | `33090ae5e7cc98589685578c16a605158c29123b5cbd2b4054b98aea4bd64446` |
| ROOT continuation `B4_1_COMPATIBILITY_DISPOSITION.md` | `f317cddf86edc5faf672629f70c0575d26ff77e2b3b494d7e167687e42865827` |
| wt3 design-run `UX-SPEC/UX_SPEC_V1.md`, bounded §§2.4/3.1/3.2 | `2141c1e844109d4869287048c695e79677f423acae965d5dd0436b7f6aeff682` |
| wt3 `MOCKS/frames/s3_table_light.html`, structural excerpts | `0fc7b447af94923a3e08718726dbfa1dc36f79ab2183cf7a92c6b5044c993706` |
| wt3 `MOCKS/frames/s4_table_light.html`, structural excerpts | `adf21ed176ed61028a297b5eb0dbfcecf48ae4c75378be4267088e0c9477ce7e` |
| wt3 `MOCKS/frames/s4_both_light.html`, structural excerpts | `ea33eed6047565691f05b83e2959bb9db189007832fd8f0e9bc412faa69aa80e` |
| wt3 `MOCKS/frames/s5_table_light.html`, structural excerpts | `91077b8efbac0d016b4169a621624c1d75d023b29913bac45c1dc73ba969217f` |
| wt3 B4-1 `WORKER_BRIEF.md` | `56df2fc1dec4fffac347042287cc90f9b50f548df24fa810d2d456e4fef43578` |
| wt3 B4-1 `_run_records/LAUNCH_INPUTS.json` | `58a86b65a5d125e481ceb0e46a05090cd2befeaaee25f9bd46f43ef0ef0b8d6a` |
| wt3 worker `SOURCE_FREEZE_02.json` | `401dac8dc77feb3148c9cc4646bdb244e2fb29ea5ba5ba5db1d97f6715b63cc6` |
| wt3 worker `WORKER_RETURN_FREEZE02.md` | `1b6cfaf0182852acf32017f1f2cc7187adc6ec45c4a45987a8cd2eedc37c7f79` |
| wt3 worker `CHECKS.md` | `fa39ea99b9140014e5707f22e78e384bb95718b4e1e1ff8c9157d9a6390f4ebe` |
| wt3 worker `OWNERSHIP_DEFECT_DIAGNOSIS.md` | `107966f603a5e29a106b178e5bfdedaa8ab2ed6438691718c20781b5e91edd8a` |

Raw evidence inspected: focused-log tail `d14ec0e451f7ae24d0075629cf6656fee1de74266818d84582c7b85214a5666d`; App-log tail `ce3055b734a8700c9012abee1efd79d476959c9f4626a952fc1d9a01984ecdf7`; freeze02 interaction log `f00da2359ba3642e25c2b4379708f7fd5bffe64d52df436bb49704e946477244`; browser `new-01` output `0b9868c37673faa17e5f48dafe956d9494d3bd95a43d8de7c2383e35db1660cd`, with its RUN/RESULT records.
