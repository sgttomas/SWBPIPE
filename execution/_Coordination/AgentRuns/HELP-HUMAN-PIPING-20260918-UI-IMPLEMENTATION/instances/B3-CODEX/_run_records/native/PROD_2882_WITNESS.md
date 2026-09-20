# Production native witness — 2882acab9

Source commit: `2882acab94120cfa6c3c115ddb416a368cebd9cd`  
Uninstrumented executable SHA-256:
`1836d91ddbc5dd4809e53b12dde92c87fab587b8e9709e19fc920ec2e780af3a`  
Build manifest: `RESPONDERS_BUILD_MANIFEST.json`, SHA-256
`590a288a77a20215ed04a00569712d21bad783ad63d9eed8de2b6fdb4f00b4c4`  
Bundle: `com.chirality.swbpipe`, version `0.1.0`

This was actual CUA interaction with the uninstrumented Tauri bundle. There was
no product source write during the witness.

## Fresh process identity

No SWBPIPE process existed before the first launch.

- First witness process: PID `29675`, started `Sat Sep 19 22:41:28 2026`.
- PID 29675 was deliberately terminated and absence checked before the Open
  transition.
- ROOT handoff process: PID `30560`, started `Sat Sep 19 22:47:08 2026`.

Both were launched by exact bundle path, not bundle ID. PID 30560 remains open
for ROOT's own look.

## Project and title transitions

The first fresh launch showed `Invented Utility Loop Preview — SWBPIPE`.
File > New Blank Project persisted this child-owned disposable row:

`project:blank-local-20260920t044213z | Blank Local Model`

The title changed to `Blank Local Model — SWBPIPE`. One node was later applied
in session for history checks but was never saved, so the store row remained a
blank model. After PID 29675 stopped, PID 30560 fresh-launched with the invented
preview title. A read-only query immediately before File > Open established
that the exact row above was the latest row. Open loaded it; title and selected
identity became `Blank Local Model — SWBPIPE` and
`project:blank-local-20260920t044213z`. No pre-existing row was opened or
modified.

## Native menus and state

- Reselecting the already-active native View > Both item left the toolbar Both
  toggle on. The menu could be reopened. CUA's native-menu AX tree does not
  expose checkmark state and app-target screenshots are unavailable while a
  macOS menu is open, so the glyph itself is not captured.
- Results and Review remained disabled before a run. Inspector state followed
  Both/closed state.
- The native Edit menu contained disabled custom model Undo/Redo when no model
  history existed and enabled predefined `Undo`/`Redo` responders according to
  the focused text undo manager.

## Text and model Undo/Redo

Before any model history, the real Node Label input received three separate CUA
key actions, `a`, `b`, `c`, and showed `abc`.

- The first `⌘Z` immediately after typing produced no AX/value change.
- The predefined native Edit > Undo pointer item then cleared the field.
- `⇧⌘Z` restored `abc`; `⌘Z` cleared it.
- Another `⌘Z` was an empty-stack no-op.
- `⇧⌘Z` restored `abc`; a second `⇧⌘Z` restored an observed autocorrection
  layer (`abc` → `Abc`); a third was a redo-stack no-op.
- `⌘Z` then traversed `Abc` → `abc` → empty.

Thus the responder history and exhaustion behavior are present, but this record
does **not** claim one-press text Undo closed: the first accelerator immediately
after typing was a no-op.

The disposable model operation created `node:N-PROD-WITNESS` through Add,
service review and Apply. Entity count changed 2 → 3 and one model checkpoint
was visible.

- Before filter text history, neutral-focus `⌘Z` changed 3 → 2 with Undo
  disabled/Redo enabled; `⇧⌘Z` changed 2 → 3 with Undo enabled/Redo disabled.
- Toolbar pointer Undo and Redo each made the same single transitions.
- With model history present, the Filter model tree field was typed to `wit`.
  Its first focused `⌘Z` was again a no-op; the second cleared it; `⇧⌘Z`
  restored `wit`.
- After focus moved to Select, one `⌘Z` removed only the node while preserving
  `wit`; one `⇧⌘Z` restored only the node. Populated text history did not divert
  the neutral model shortcuts.

## Page-close focus

Libraries was opened from its rail item. In native WebKit's AX tree, the page
content itself was omitted even though it was visibly present; the screenshot
records it. Starting from the shell, Tab moved to Rules, then Issues, then the
visible Libraries Close control (the latter had no AX node). Return closed the
page. The resulting full AX tree reported focus on the visible, collapsed
Libraries rail opener. The model, canvas and one unsaved node checkpoint
remained intact.

## Native dimensions

The production debug bundle exposed no Inspect Element context menu, no working
`⌘⌥I` inspector, and no debug item in the app menu; the bounded production
method therefore cannot read inner/outer sizes directly. Screenshot pixels are
not substituted.

The separate actual-Tauri diagnostic probe, on the same 1280 × 800 native
configuration, retained public native API measurements:

- startup inner and outer: 2880 × 1840 physical at scale 2 = 1440 × 920 logical;
- minimum after real corner drag: inner and outer 2560 × 1600 physical at scale
  2 = 1280 × 800 logical;
- a second shrink attempt emitted no further resize;
- startup DOM inner height was 888 CSS px, so 800 logical outer/inner is not
  claimed as 800 CSS content pixels.

That probe is diagnostic evidence, not an assertion that the uninstrumented
PID exposed a hidden size API. The config's 1280 × 800 minimum remains covered
by native source tests.

## Screenshots

| File | SHA-256 | Pixels | Observation |
|---|---|---|---|
| `PROD_01_libraries_page.png` | `d000025dd4dc0a8161bc9c93abf997e262767417e37a5b36adaf81542ee7a180` | 1203 × 768 | visible Libraries page and Close control |
| `PROD_01_model_history_restored.png` | `1c8aeeda820b18f664950dfed0ca7324d7f7db71dc8be0d662056fd9b64f18da` | 1203 × 768 | node restored, Undo available |
| `PROD_02_page_close_focus.png` | `1c8aeeda820b18f664950dfed0ca7324d7f7db71dc8be0d662056fd9b64f18da` | 1203 × 768 | same pixels after keyboard close; AX binds focus to Libraries |
| `PROD_03_root_handoff_clean_open.png` | `b5a9e7889c6ba4c88f66bc73a2d574f236ecfd2684112eb1fa45e1850ca4a037` | 1203 × 768 | clean reopened disposable project for ROOT |

## Handoff state

PID 30560 remains open with:

- project `project:blank-local-20260920t044213z`, blank persisted model;
- title `Blank Local Model — SWBPIPE`;
- Model stage, Both view, Inspector closed;
- no model Undo or Redo history;
- System theme and Comfortable density, verified from the visible Appearance
  disclosure;
- no active tool and no filter.

The select-popup Escape repair is unresolved and was not claimed or requalified
on this candidate. Earlier f5 evidence remains the reproduction: first Escape
closed both popup and Inspector.

This record makes no usability, conformance, performance or acceptance claim.
