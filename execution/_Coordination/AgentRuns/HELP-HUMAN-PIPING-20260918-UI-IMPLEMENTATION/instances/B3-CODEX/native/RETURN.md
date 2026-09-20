# B3-CODEX native return

I am Codex on GPT-5.6 Sol with high reasoning, by my own statement. I acted as
TASK Type 2, delegated no work, used only the assigned worktree, and made no Git
mutation. Manager/ROOT owned frontend integration, production checkpoints and
probe assignments.

## Implementation returned

Native source landed in the B3 candidate:

- Tauri minimum changed from 1024 × 768 to 1280 × 800.
- Native View gained stateful Table/Model/Both, Model/Loads/Results/Review,
  Inspector, Light/Dark/System and Comfortable/Compact commands.
- Native `⌘1`, `⌘2`, `⌘3`, `⌘I` dispatch the same typed command IDs through the
  one existing `runMenuCommand` sink. The webview shell listener stands down for
  those keys in Tauri, preventing duplicate execution.
- `sync_native_shell_state` validates the frontend payload and synchronizes
  menu checked/enabled state, Undo/Redo, Run/Cancel and project title.
- Window title is `project name — SWBPIPE`, with a `SWBPIPE` fallback.
- macOS's automatic pre-event CheckMenuItem toggle is restored to the last
  React-owned value before command dispatch, so selecting an already-active
  item cannot leave it falsely unchecked.
- The accepted later candidate adds predefined AppKit Undo/Redo responders for
  focused text while retaining custom model menu commands and the guarded
  webview model-shortcut route.

No schema, persistence format, operation contract, result designation, solve
basis, generation guard, renderer, picking, selection, resource or halo
semantics changed.

## Files owned by this child

The child's product edits were:

| File | Final 2882 SHA-256 |
|---|---|
| `projects/chirality-piping/apps/desktop/src-tauri/src/lib.rs` | `b886476b0b807b1453dcf8b6a5f388681ea4874ce8aded214452aef3d8c6dd1b` |
| `projects/chirality-piping/apps/desktop/src-tauri/tauri.conf.json` | `b3de0965f205e1ed34f07744939314c9ac40dd00af2ee8c0de336c6e2bfba5d0` |

The manager later added the two predefined responder items in `lib.rs`; the
hash above binds the resulting reviewed production source at
`2882acab94120cfa6c3c115ddb416a368cebd9cd`. `main.rs`, Cargo manifests and
lockfile were not changed by this child. No frontend/e2e file was written by
this child.

Evidence authored under `instances/B3-CODEX/**` is intentionally uncommitted
for manager/ROOT retention. Exact evidence-file hashes should be taken after
manager finalizes this return packet; completed raw logs and historical witness
records were not rewritten.

## Bridge contract

Frontend state sync payload:

`projectName`, `stage`, `view`, `theme`, `density`, `resultsStageEnabled`,
`reviewStageEnabled`, `inspectorOpen`, `canUndo`, `canRedo`, `canRun`,
`canCancel`.

Project name uses `projectSummary?.project_name ?? model?.project.name ?? null`.
Stage/view/run/preference booleans derive from the same session cells as the DOM
controls. Undo/Redo include `!operationBusy`. All native commands return to the
one menu sink; none mutates the model directly.

Busy-project File gating and the unsaved-title marker remain the separately
scheduled B3B/B3A work by explicit direction.

## Checks

- Final child Rust run before later frontend repairs:
  `cargo test --manifest-path projects/chirality-piping/apps/desktop/src-tauri/Cargo.toml --quiet`:
  **101 passed, 0 failed**; main/doc targets passed with zero tests. Raw output:
  `../_run_records/native/RUST_TEST_RAW.log`.
- Final uninstrumented responders build:
  `npm run tauri -- build --debug --bundles app`: passed. Build manifest:
  `../_run_records/native/RESPONDERS_BUILD_MANIFEST.json`, SHA-256
  `590a288a77a20215ed04a00569712d21bad783ad63d9eed8de2b6fdb4f00b4c4`.
- Production source commit:
  `2882acab94120cfa6c3c115ddb416a368cebd9cd`.
- Production executable:
  `1836d91ddbc5dd4809e53b12dde92c87fab587b8e9709e19fc920ec2e780af3a`.
- `git diff --check` passed for the native source changes at their checkpoints.
- Browser 1024 × 768 robustness does not derive from Tauri config. The source
  and dist specs retain explicit 1024 cases; Playwright compact remains 1280 ×
  800. This child changed no e2e case, skip, timeout, tolerance or oracle.

## Actual Tauri witness

Canonical production record:
`../_run_records/native/PROD_2882_WITNESS.md`; exact calls:
`../_run_records/native/PROD_2882_CUA_ACTIONS.md`.

The uninstrumented production bundle was fresh-launched by exact path. The
final handoff process is PID `30560`, start `Sat Sep 19 22:47:08 2026`, and is
left open for ROOT's own look.

Observed passes:

- initial title `Invented Utility Loop Preview — SWBPIPE`;
- New Blank persisted child-owned project
  `project:blank-local-20260920t044213z`, title `Blank Local Model — SWBPIPE`;
- after restart, read-only latest-row verification and native File > Open loaded
  that exact project and title;
- same-value native Both reselect retained Both;
- native stage/view/menu enablement reflected session state;
- one neutral-focus model `⌘Z` and one `⇧⌘Z` each changed exactly one checkpoint,
  both before and after populated text history;
- pointer model Undo and Redo each changed exactly one checkpoint;
- text responders traversed insertion/autocorrection Undo/Redo layers and empty
  stack no-ops without consuming model history;
- Libraries Close reached by Tab/Return restored visible AX focus to Libraries;
- System theme and Comfortable density were visibly verified and restored.

Text accelerator limitation: immediately after genuine per-key `abc` in Node
Label, the first `⌘Z` made no value change. Native Edit > predefined Undo then
cleared it; subsequent `⇧⌘Z`/`⌘Z` and exhaustion behaved correctly. With model
history and Filter=`wit`, the first focused `⌘Z` was again a no-op, the second
cleared it, and `⇧⌘Z` restored it. This return does not claim one-press focused
text Undo is closed.

Select-popup Escape remains unresolved by ROOT's production disposition and is
not claimed repaired. The earlier f5 native witness reproduced first Escape
closing both the real HTML select popup and Inspector; the 2882 candidate adds
no select bridge.

## Native dimensions

Production exposed no supported debug inspector path, so exact production
inner/outer API values remain unavailable rather than inferred from screenshots.
The actual-Tauri diagnostic probe on the same configuration measured:

- startup inner/outer 2880 × 1840 physical, scale 2 = 1440 × 920 logical;
- real-drag minimum inner/outer 2560 × 1600 physical, scale 2 = 1280 × 800
  logical;
- no further resize after another shrink attempt;
- DOM inner height 888 CSS px at startup.

This separates native window dimensions from CSS content size. It is diagnostic
evidence, not a hidden production measurement.

## Native controls and contrast

Touched controls are native View items plus existing Inspector, Undo, Redo, Run
and Cancel state. Native labels provide accessible names; pointer operation is
the macOS menu; specified accelerators route to existing controls; unavailable
items are disabled. Target drawing, focus and contrast are OS-rendered. No
color/contrast token changed. The closing-pass rail-caption and Inspector-label
contrast witnesses remain with the shell lane.

## Current handoff state

PID 30560 is open with blank persisted project
`project:blank-local-20260920t044213z`, no session edits/history, Model stage,
Both view, Inspector closed, System theme, Comfortable density, no active tool
and no filter. Screenshot:
`../_run_records/native/PROD_03_root_handoff_clean_open.png`.

This return claims observed behavior only. It makes no usability, conformance,
performance or owner-acceptance claim. PDU-045 and PDU-046 remain holds.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
