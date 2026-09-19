# Retained return — slice B2G (B-SHELL manager), 2026-09-19

Retained by ROOT. What follows is the agent's final message as the host stored it, with the machine's repository path replaced by `{REPO_ROOT}`; nothing else is changed. The message exactly as the host stored it has SHA-256 `2bff1130638030b34241fb30fd581b2cf3bcd3a5d4093fed99c7f661ef9b4610` (99 lines), by this script.

---

# Slice B2G return: a failed or superseded save or create keeps the open project's integrity record (addendum 4)

Slice B2G is repaired test-first and frozen for review at `0fc5b065e4822fea37ee8cf748f83e45eb3cf9df`. B3 stage 1 starts on top of it after ROOT's reply.

This is B-SHELL (WORKING_ITEMS, Type 1), lane `codex/swbpipe-b-shell-20260918`. By my own statement I ran as Claude Fable 5.1 (`claude-fable-5-1`). I did the slice myself, with no child, and dispatched no reviewer.

## Candidate and range

- **Frozen candidate:** `0fc5b065e`, a records-only commit. The product tree is frozen at `74707b134efcd899898a088c6fd3f5a913ca6c96`.
- **Diff range for B2G alone:** `dfa7c47d5..0fc5b065e`.
- **Diff range against `origin/main` `64f86e17f`:** `64f86e17f..0fc5b065e`. It also holds B2F, which ROOT is merging from its own branch.
- **Product files:**
  - `{DESKTOP}/src/features/workspace/workspaceSession.ts`, +12 −4.
  - `{DESKTOP}/src/App.projectHandlers.test.tsx`, +114, all additions.
- **Commits:**
  - `9b7687da9`: the four tests. Three fail on that tree by design.
  - `74707b134`: the repair.
  - `0fc5b065e`: records. It also carries the re-sealed B3 brief.

## Checks

| Check | Command | Result |
|---|---|---|
| Addendum 4 | `shasum -a 256` | `08b51c66…`, matches |
| Fail first | `npx vitest run src/App.projectHandlers.test.tsx -t "B2G"`, on `9b7687da9`'s source | 3 failed, 1 passed: the three repair tests expect `integrity=mismatch_review_required` and receive `open_verification_not_run_this_session`; the fourth pins today's success path and passes |
| After the repair | `npx vitest run src/App.projectHandlers.test.tsx` | 11 passed |
| Build, with type check | `npm run build:desktop`, from `{WORKING_ROOT}` | exit 0 |
| Unit suite | `npm run test:desktop`, from `{WORKING_ROOT}` | 76 files, 1,201 tests passed (1,197 plus the 4) |
| Playwright, source lane | `PLAYWRIGHT_WORKERS=1 sh {RUN}/tools/with_e2e_lock.sh npm run test:e2e`, from `{DESKTOP}`, once, on `74707b134` | 374 passed, 20 skipped, exit 0 |
| Playwright, dist lane | the same with `npm run test:e2e:dist` | 53 passed, exit 0 |
| Validators | `validate_path_anchors.py`; `validate_claims_language.py` | PASS; VALID, 335 files |

- **Full suite timing:** the full unit suite ran before a comment-only rewording of the two added code comments. After that rewording, the build ran again with exit 0, and `src/App.projectHandlers.test.tsx` with `src/App.test.tsx` gave 229 passed. Both Playwright lanes ran on the final commit.
- **Not re-run in this slice:** the harness self-check and the practitioner-harness pytest. The slice touches no registered surface beyond my instance folder.
- **Tests and frozen values:** no existing test was changed. No tolerance, oracle or frozen value was touched.
- **State at freeze:** no listener is on 5174 or 5175, no lock is held, and the worktree is clean.

## Tests that carry the control layer

All are in the `describe` block "a failed save or create leaves the open project's integrity record (B2G)". Each asserts both the model line and the envelope line of the validation panel.

- "keeps both recorded mismatches when a save fails".
- "keeps both recorded mismatches when a create fails".
- "keeps both recorded mismatches when a pending save is superseded by an open that finds nothing".
- "still clears the open-time verification when a save lands, as before". This one pins that success behaves exactly as today.

## The repair

- In `handleCreateProject` and `handleSaveProject`, the statements `setModelHashIntegrity(null)` and `setProjectEnvelopeHashIntegrity(null)` moved. They were before the first `await`. They now sit directly after the `stillCurrent` check that follows the `createLocalProject` or `saveLocalProject` call.
- From that point on, both handlers read as before: the cells are nulled and not re-derived.
- **Placement choice:** I placed the nulls at the point the persistence has landed, not at the end of the success path. The PROJECT-PERSISTENCE-RESPONSE-INTEGRITY refusal returns after the bytes were written. Placed here, that refusal still nulls the cells, as it does today.
- **Untouched:** `runMenuCommand`, every guard's condition, both gate classes, `clearComputedModelState`, the blank and open handlers and every existing test.

## Controls

- **Controls added:** none.
- **Contrast not yet checked:** none.
- **Appearance left for the closing pass:** none.

## Semantic changes, named

1. **A save or create that fails.**
   - Today it nulls the open project's two integrity cells. A recorded `mismatch_review_required` then reads `open_verification_not_run_this_session`.
   - Now the cells stay as they were.
   - Held by the first two tests.
2. **A save or create that is superseded before its response is accepted.**
   - Today the same nulling happens.
   - Now the cells stay, unless the superseding request sets them itself.
   - Held by the third test.
3. **While a save or create is pending.**
   - Today the integrity status disappears at the click.
   - Now it stays until the persistence lands.
   - No dedicated test covers the window; both ends are tested.

## What I bring back

1. **Should a successful save re-derive the integrity record instead of nulling it?** The reviewer's design question is not answered here. Today and now, after a save lands the panel reads `open_verification_not_run_this_session`.
2. **An edge of change 2.**
   - A superseded save may in fact have written its bytes, because its response is dropped, not its effect.
   - If the superseding open then finds nothing, the panel keeps the open-time record although the stored bytes changed.
   - Before B2G the panel read "not run" in that case.
   - It is reachable only through the native menu during a busy request, which is item 1 already with the owner.
   - Proposal: leave it, and close it with the native-menu question.

## B3, ready

- `{RUN}/instances/B-SHELL/briefs/B3-SHELL.md` is re-sealed after addendum 4 as `079876c393d2a93ae7969ee2c6294f5225e3eccb95c3d487b8d96e0d9af29cec`. The index row is struck and restated.
- **Changes in the re-seal:**
  - The Agent toggle and strip show "Agent: not available yet", reachable by pointer and by keyboard focus.
  - The benchmark-hooks table moves into stage 1's return, so ROOT has it before stage 2 lands. It stops there if a recorded first-profile value cannot stay true.
  - The five project buttons keep their accessible names and stay findable by role and name (the review's observation 2).
  - The `status-pill-*` inventory gives old text, new text and the rule.
  - Addendum 4 is added to the reading list.

**Next slice:** B3, stage 1 (the shell's control layer, nothing visible), by one Fable child in the foreground, on top of `0fc5b065e`, after ROOT's reply.

I did not push, open a pull request, rewrite history or touch another worktree's files. I claim no usability, conformance or performance acceptance. PDU-045 and PDU-046 remain holds.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
