# Preliminary native witness — B3 Codex native slice

Date: 2026-09-19 (America/Edmonton). This witness is preliminary because the
manager identified a later routing/focus repair in frontend session code. The
native menu, title and minimum-size source bytes were stable during this run;
the final bundled-binary hash must be rebound after that repair.

## Candidate and build

- Repository HEAD: `a63607e5c6d805f364c969eb6963f768baefb300`.
- Build command: `npm run tauri build -- --debug` from
  `projects/chirality-piping/apps/desktop`.
- The configured pre-build ran both `npm run build:wasm` and `npm run build`.
  Vite transformed 1,744 modules and completed in 1.84 s; the Tauri debug build
  completed and bundled `src-tauri/target/debug/bundle/macos/SWBPIPE.app`.
- Bundle identifier: `com.chirality.swbpipe`; version `0.1.0`.
- Bundled executable SHA-256:
  `11e18decc32badeffc490a8fa7977a57632e5e5897bb1406aed3e65370f33615`.
- Native unit command:
  `cargo test --manifest-path projects/chirality-piping/apps/desktop/src-tauri/Cargo.toml`.
  Result: 101 passed, 0 failed, 0 ignored; main and doc-test targets had 0
  tests and passed.

## Actual native actions and observations

The app was launched and operated through native CUA against the macOS bundle,
not a browser surface. The selected window was identified by its title before
each action.

1. Initial state: title `Invented Utility Loop Preview — SWBPIPE`; Model stage;
   Both view; Results and Review disabled; Inspector closed.
2. Pointer: opened the macOS View menu and chose `Model View`. The toolbar then
   exposed Model as pressed, Both as unpressed, and Inspector as expanded but
   disabled with `The inspector is always docked in Model view`.
3. Keyboard: pressed `⌘3`. Both became pressed and Model became unpressed.
4. Keyboard: pressed `⌘I` once. Inspector became expanded and its actual
   property panel appeared. This witnesses one dispatch: a duplicate dispatch
   would have opened and immediately closed the toggle.
5. Pointer: chose `Loads Stage` from the macOS View menu. Loads became pressed,
   Table became the selected view, and Inspector became disabled with the Table
   view reason. The native menu showed Results and Review disabled.
6. Pointer: chose `Dark`, then `Compact Density`, from the macOS View menu. The
   resulting dark and compact Loads surface is retained below.
7. Pointer: chose File > `New Blank Project`. The session model changed to the
   blank model and the native title changed to `Blank Local Model — SWBPIPE`.
   This route called `createLocalProject` and persisted the new disposable row
   `project:blank-local-20260920t014507z` in the native SQLite store. A
   read-only exact-ID query confirmed that row is `Blank Local Model`. No
   pre-existing project row was opened, overwritten or otherwise mutated.
8. Pointer: attempted a bottom-right resize below the configured minimum. The
   CUA screenshot surface changed from 1203 × 768 to 1229 × 768 rather than
   reporting native logical bounds. Because CUA exposes neither window bounds
   nor webview inner bounds, this action does not establish exact outer/inner
   dimensions. The 1280 × 800 config is held by a Rust test; exact runtime
   inner/outer measurement remains unavailable on this host surface.

The initial path lookup and later bundle-id binding left more than one SWBPIPE
window available. All state-changing witness actions above were performed on
the bound window whose title progressed from the invented preview identity to
the blank-model identity. The extra window was not used for project actions.

## Screenshots

All images are native CUA captures. CUA returned 1203 × 768 for actions 1–7;
the post-resize capture is 1229 × 768.

| File | SHA-256 | Witness |
|---|---|---|
| `01_initial_both.png` | `2d8941e60d3369114b9c2b9afb000e44e85a4c21008d7f06af943c51fbf86279` | initial title and Both view |
| `02_native_menu_model_view.png` | `434129b84213eebf7fb688222ddcf2ab9136806de696acbf75cb8c5a8f1f8b4a` | Model view after native menu click |
| `03_cmd_i_single_dispatch_inspector_open.png` | `0e3c38cb43d9d7dea813787df2c12c95b04e2adc846aac0500cb7d0d79fa41b8` | Inspector open after one native accelerator |
| `04_native_menu_loads_dark.png` | `7d73ff94a9bcef3ce2cc747c390427b64a29a62cc2cf12178af112a89af72c45` | Loads stage and Dark theme |
| `05_native_menu_loads_dark_compact.png` | `2302ab6574d0a2274b2b392d631acad55688f41ec94c90cddcec8f65eb77f77f` | Compact density |
| `06_new_blank_title.png` | `afde2c87856b38f44aa97a0c00aa2bf94b0863edcc2b50bc8383c97f9baa1e86` | new-project title transition |
| `07_min_resize_attempt.png` | `b455a01595aed48ce6630297589b342a5897939bfb0735a592ac730e8f63fea7` | bounded minimum-resize attempt; dimensions limitation above |

## Limits and deferred behavior

- Open-local title transition was not exercised in this preliminary pass. The
  exact disposable row created here is now known and can be targeted safely in
  the final pass without selecting an unrelated existing project.
- Busy-project native File gating and the unsaved-title marker remain the
  separately scheduled B3B/B3A work and are absent here by direction.
- CUA supplies screenshots and accessibility state but no documented native
  window-size or webview client-size query. No undocumented script or second UI
  automation technology was used to manufacture that measurement.
- The manager's later routing/focus repair requires a rebuilt bundle and a
  short native backcheck before the final witness is bound.

This record claims observed behavior only. It makes no usability, conformance,
performance or acceptance claim. PDU-045 and PDU-046 remain holds.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
