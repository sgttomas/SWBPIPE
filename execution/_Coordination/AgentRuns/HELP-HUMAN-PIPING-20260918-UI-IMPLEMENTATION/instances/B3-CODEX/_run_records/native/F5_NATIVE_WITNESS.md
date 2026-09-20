# Native witness on repaired product checkpoint f5bc01f8b

Candidate: `f5bc01f8beaa1c210b988fe28aea95573781f963`  
Executable SHA-256: `299ea09e810edcbfc12475c71047d31a45199016d1158a7f5aca7f6c7a51e0d7`  
Fresh process: PID `92269`, started `Sat Sep 19 20:52:15 2026`  
Bundle: `com.chirality.swbpipe`, version `0.1.0`

The prior native process PID 74074 was terminated and absence checked before
this bundle was launched by exact path. The build succeeded after a first
attempt exposed and the manager repaired an unsupported Testing Library
`exact` option. Both raw outputs are retained beside this record.

## Passed observations

- Initial native title: `Invented Utility Loop Preview — SWBPIPE`.
- Reselecting the already-active native View > Both item left the toolbar's
  Both toggle on. Reopening View succeeded. CUA's native-menu AX tree does not
  expose checkmark state and app-target screenshot is unavailable while a
  macOS menu is open, so the visible menu checkmark itself is not captured.
- Immediately before Open, a read-only SQLite query established that the
  latest row was `project:blank-local-20260920t014507z | Blank Local Model`.
  File > Open Local Project opened that exact row, and the title changed to
  `Blank Local Model — SWBPIPE`.
- A disposable node `node:N-WITNESS` was created through Add, service review
  and Apply. The toolbar exposed one Undo checkpoint.
- From neutral Select focus, one `⌘Z` removed the node and produced Undo
  disabled / Redo enabled. One `⇧⌘Z` restored the node and produced Undo enabled
  / Redo disabled. Neither command ran twice.
- While node routing remained armed, switching Inspector Task then Properties
  left the Inspector and routing surface open.
- Theme and density were explicitly restored through native View to System and
  Comfortable at the end of the witness.

## Concrete native defects

1. **Focused text undo is absent.** With the one model checkpoint present, the
   `Filter model tree` input was focused and typed to `Witness`, then extended
   with a single native `x`. Two `⌘Z` attempts left `Witnessx` unchanged.
   Correctly, neither consumed model history: `node:N-WITNESS` remained, Undo
   stayed enabled and Redo stayed disabled. The editable guard prevents model
   undo, but WebKit/default text undo was not observed through this native CUA
   path.
2. **HTML select Escape also closes the Inspector.** With Both Inspector and
   node routing open, clicking `New node coordinate unit` exposed a real popup
   with selected `m` and options `mm`, `in`, `ft`. The first Escape dismissed
   the popup and immediately collapsed the Inspector, removing both routing and
   property panels while the Node tool remained armed. Popup-only first-Escape
   priority is therefore not met.

## Dimension boundary

The debug build exposed no Inspect Element context menu. `⌘⌥I` produced no
inspector or accessibility change, and the app menu contained only About and
Quit. Per ROOT's bounded method, no further debug-inspector attempt was made.
Consequently, CUA supplies no documented native inner/outer-bounds query. The
1280 × 800 config remains unit-tested, but exact runtime inner and outer
dimensions are unavailable and are not inferred from the rescaled 1203 × 768
captures.

## Screenshots

| File | SHA-256 | Pixels | Observation |
|---|---|---|---|
| `FINAL_01_select_popup_open.png` | `dc1aa66ae3abbb81031ea7563e07cef6bb7259bb8c086b347a77aba4973022d0` | 1203 × 768 | real coordinate-unit popup open |
| `FINAL_02_select_escape_closed_inspector.png` | `42080b2c1f32b084a81835fca8ccb5490da4b65c7ae8cf4569f26bff29ce53e3` | 1203 × 768 | popup gone and Inspector incorrectly closed |
| `FINAL_03_stable_state_for_root.png` | `6b24b725850a23b2ebb37426bc3906a0074f35d15814eb0a8203550973a197b9` | 1203 × 768 | Both/System/Comfortable, one unsaved node checkpoint |

The app was left open in Both view, System theme, Comfortable density,
Inspector closed, on project `project:blank-local-20260920t014507z`, with one
unsaved session node `node:N-WITNESS`, Undo enabled and Redo disabled. This
state is historical because an additional page-focus repair is scheduled before
the final bundle and ROOT own-look.

This witness records behavior only; it makes no acceptance claim.
