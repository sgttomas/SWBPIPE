# B3 compact/reset fidelity backcheck

**PASS for original F1 and the bounded geometry/cancellation paths below; no additional actionable finding from this UI inspection. This is not an overall compact-control pass.** Parent ROOT subsequently reported two independent code findings on this same candidate: invalid/disabled-current dismissal can select the first enabled option, and an active index can retarget when options change. Neither edge case was proved passing here; both require their owning repair/backcheck.

Candidate: `41098f1e18edf58c59b2694119ea76a37ab9a1bc`, frozen wt2 dist. Input manifest and brief hashes matched the launch (`verification.json`), as did all 21 dist-file hashes. Candidate and dist remained unchanged at exit. Independent TASK Type 2, parent ROOT; configured Astra/high per sealed assignment, with no independent serving-model telemetry. No delegation, source/test/fixture/Git write, state injection, native/CUA/foreground access or build.

## F1 disposition

Closed on this candidate by actual pointer/keyboard workflows:

1. Loads/Table → File/New Blank Project opens Model/Both, with Model current, Both pressed, and Results/Review disabled rather than current. `01-blank-from-loads.*`.
2. Model/Model with Node armed and a typed draft → Project page/New blank opens Model/Both, closes the page and clears the armed tool. `02-blank-from-model-armed-page.*`.
3. Built-in invented model → toolbar Run → Results → Save Local Project → Open Local Project (Historical) → New Blank Project opens Model/Both with coherent disabled Results/Review. `04-historical-basis.*`, `05-blank-from-historical.*`.

Dark theme and Compact density were selected through View before the first reset and retained through resets (screenshots and `preferences-after-reset.json`; density additionally recorded as MAIN `data-density=compact` in the subsequent state observations). Within one project, Loads changed from Table to Both, was left and re-entered, and restored Both (`03-within-project-memory.*`). The original F1 return/manifests were not altered.

## Compact controls exercised

- Node coordinate-unit control: pointer opening at 1440×920; popup has one actual browser option, entered `m`. First Escape dismisses it, preserves the inspector, and returns focus to its trigger (`06`/`07`). Space reopens it; selecting the existing `m` is a same-value commit (`08-same-value.json`). Later Escape with the control closed closes the Both inspector and focuses Inspector (`09`). This does not prove native mm choices.
- Support family: actual multi-option property control, initially showing disabled selected `Not provided (preserved)` (`10-family-popup-1440.json/png`). Explicit pointer Anchor changes the draft; Space/ArrowDown/Enter changes it to Guide (`11-family-changed.json/png`). Undo remains disabled; no engineering operation was queued or applied. Home moves active access to the first enabled option, End exposes the last option with scrolling, and typeahead `g` plus Enter chooses Guide (`12`/`13`). Selected and active options have distinct visual states.
- Tab/outside dismissal: after explicitly navigating from Guide to Line stop, Tab closes and commits Line stop, then moves focus to Restrain UX. After navigating again, an outside click closes and commits Vertical support (`13-typeahead-tab.json`, `14-outside.json`). These observations are **not** proof of no-navigation invalid-current dismissal: that path was not attempted. Dynamic option reordering/change was not exercised.
- Support-family popup fit within the viewport and inspector-side bounds at 1440×920 and 1280×800 (`10`, `14`). At 1440 its measured rectangle was x1117/y512.516, 259×280; options remained readable through scroll. Its 1280 bounds and option states are retained separately in `14-family-popup-1280.json`. The popup opened upward as needed near the viewport bottom.
- Meta+2 while Support family was open closed the popup and preserved Vertical support and the same canvas DOM node (`15-view-change-closes.png`; actual action/result in transcript). An open Node-unit popup followed by Loads then Model closed the popup and retained `node:retained` and the same canvas DOM node (`16-stage-change.json/png`).
- At 1200×800 browser fallback, first Escape from Node-unit popup leaves the inspector open, and the next Escape closes it/focuses Inspector (`17`–`19`). This is a browser fallback witness, not a native minimum-size or native popup witness.
- Routing construction-plane control: initially disabled by its containing fieldset with a reason requiring a resolved From node. Genuine pointer attempts were refused and no popup opened (`20-disabled-plane.json`). Selecting the available Pump nozzle From option enabled it. The XY/XZ/YZ popup opened at 1280×800 in Light/Comfortable (`20-route-plane-1280-light.png`). Pointer XY committed; ArrowDown opened the popup, and Escape cancelled while retaining XY (`20-route-plane.json`). Further 1440×920 Light popup capture shows readable labels and visible trigger focus (`21-route-plane-1440-light.png`). No route was applied.

The controls were inspected in their actual portalled/scrolling inspector, using genuine pointer and keyboard actions. DOM reads supplied names, roles, expanded/selected/disabled states and geometry; no DOM or storage state was injected. `compact-control-geometry.json` retains final trigger rectangles. The initial Support-family disabled placeholder supplied invalid/disabled-current appearance only, not the untested no-navigation behavior identified later by the code reviewer.

## Limits and custody

This is an affected-path structure backcheck, not a broad B3 re-review or a complete combinatorial test of all substituted select consumers. C5 HUD, closing-pass colour/contrast/fine spacing, native behavior, native Undo, native mm catalog, changed-option-list behavior, complete accessibility conformance and practitioner/product acceptance remain outside its claims. Parent's two code findings remain unresolved by this return.

Headless Chromium 148.0.7778.96, Node v24.18.0, macOS arm64, fresh ephemeral browser context; browser-memory invented projects only. Existing dist on 5183 through the shared lock. `probe.mjs`, `actions.jsonl`, screenshot/text/JSON observations and `SHA256.json` provide rerun/evidence custody. The runner was reused in the newly authorized directory and corrected to exit after cleanup; the old runner was untouched. Exploratory timeouts came from toggling an already-armed Node off and from the legitimately disabled construction plane; they were resolved through observed UI.

State JSON's broad `open` list includes always-expanded hidden target pickers; it is not a list of open compact popups. Use the specifically named compact control, screenshots, and recorded actions for popup claims. No product page error was emitted.

Only frozen candidate context was newly consulted: `B3_NATIVE_SELECT_DECISION.md` was read with `git show 41098f1e:…`; its exact bytes and origin/hash are retained in `B3_NATIVE_SELECT_DECISION.accepted-input.md` and `context-origin.json`. No mutable work graph was used in this backcheck. For the original review, the supplemental graph was read from ROOT's live checkout, not wt2; its complete bytes were transient in a Python process and were not retained by this reviewer, while excerpts remained in tool output. ROOT separately recovered and verified that original snapshot; this backcheck does not substitute new bytes for it.

Browser and server exited, shared lock released, 5183/5184 clear (`cleanup.json`). ROOT was notified immediately to release the slot for manager lanes. Artifact hashes follow in `SHA256.json`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). No native, performance or acceptance claim.
