# Native keyboard diagnosis — read-only return

Status: source diagnosis complete; exact host event observation unavailable. No repair selected or performed; no acceptance claim.

## Frozen reproduction

Canonical reproduction is `../native/F5_NATIVE_WITNESS.md`: source f5bc01f8beaa1c210b988fe28aea95573781f963, executable SHA256 299ea09e810edcbfc12475c71047d31a45199016d1158a7f5aca7f6c7a51e0d7, PID 92269, macOS Tauri com.chirality.swbpipe 0.1.0. Disposable project project:blank-local-20260920t014507z only. Parent supplied this evidence; diagnosis did not independently repeat UI actions. Native executor retains sole UI ownership.

Neutral-focus CmdZ then ShiftCmdZ removed/restored exactly one node. Focused filter Witness plus x did not change under two CmdZ attempts, and model node/history stayed intact. Coordinate-unit native popup first Escape dismissed popup and collapsed Both inspector. Raw screenshot paths/hashes and failed Web Inspector attempts remain canonical in that witness. Initial native RETURN predates the final build and is not the current result.

## Causal assessment: text undo

Frozen workspaceSession.ts:2306–2317 returns before preventDefault and model command dispatch for input/textarea/contenteditable. Native witness confirms model history is protected. This does not prove WebKit received or executed text undo. ModelTree.tsx:47 onward owns filterText; module-level TreeControls at 307 onward renders a search input with untransformed value and synchronous onChange state update. App.tsx:388 mounts ModelTree without a changing key. No local filter keyboard handler, per-keystroke input remount, or value normalization explains the symptom in this source. Controlled-input handling remains an empirical alternative, not a proven defect.

lib.rs:4053–4063 builds custom Undo Model Edit/Redo Model Edit without accelerators, then predefined cut/copy/paste/select_all; it omits predefined undo/redo. Installed locked muda 0.19.1 maps predefined Undo/Redo to AppKit undo:/redo: and CmdZ/ShiftCmdZ (registry macos/mod.rs:985–986; items/predefined.rs:306–311). Thus the editable guard relinquishes model handling but the menu provides no corresponding responder-chain action. This is the strongest source-supported host hypothesis, moderate confidence, not a verified causal repair. Tauri distinguishes custom callback items from predefined items: https://v2.tauri.app/learn/window-menu/#creating-predefined-menu .

Exact API provenance is retained in ../native/F5_CUA_ACTION_LOG.md and supplied by the manager: click(50), typeText('Witness'), focused AX value Witness, pressKey('super+z') unchanged, pressKey('x') produced Witnessx with focus 50 retained, pressKey('super+z') unchanged. No Backspace tried. The separate pressKey x materially reduces the bulk-text-injection alternative, although a native undo transaction is not directly observed. No claim that CUA typing is at fault is warranted. Neutral model sequence was click(13), pressKey super+z, super+shift+z with HTML content focus and one model action each.

Bounded option: evaluate native predefined text undo/redo while preserving separate custom model actions. Their accelerators can preempt the currently successful neutral-focus webview path, so simply adding menu items is not established as safe. Verify responder routing and neutral model shortcuts in actual Tauri before choosing this repair. Do not add DOM-maintained text history or alter model history based on current evidence. If reliable native per-key editing still cannot undo through responder actions, investigate WebKit/control transaction behavior next.

## Causal assessment: popup Escape

PipeViewport.tsx:2543–2555 renders the actual select with onChange only; no local popup cancellation ownership. workspaceSession.ts:2112–2125 sees unprevented Escape inside an open Both inspector and synchronously toggles it closed, then focuses its opener. At inner widths below 1280, handleNarrowDrawerKeyDown:2192–2199 can close the inspector even earlier in React bubbling. Native dimensions were unavailable; screenshot size does not decide which handler ran. Both paths lack a popup-open distinction.

High confidence: shell cancellation arbitration is insufficient for this actual native control. Earliest source divergence is allowing an Escape used to cancel the select to reach inspector-close handling. Exact host ordering (popup closed before DOM event, event defaults, target, :open state at capture/bubble) is unobserved; native UI output alone cannot distinguish these. A defaultPrevented guard is insufficient in the observed host. The actual popup-open and closed-inspector screenshots establish failure, unlike synthetic DOM key tests.

Bounded option: add a narrowly scoped picker-cancellation ownership guard only after verifying a reliable open-state signal/event sequence in this WebKit. MDN documents select :open but this does not prove installed WebKit support or whether it remains true when Escape is delivered: https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/:open . Cover both narrow drawer and global handlers. Blanket suppression of Escape on all select targets is not an acceptable repair: a closed select must still permit the ordinary inspector-close command. If the host exposes no reliable signal under authorized tooling, report that limit and seek an explicit interaction decision rather than guessing open state from focus alone.

## Focused regressions and dependants

- Actual Tauri: genuine per-key insertion followed by deletion, text CmdZ/ShiftCmdZ, filter plus another controlled text field; text changes correctly while model/hash/history stay unchanged. Record exact API calls. Check empty text history too.
- Actual Tauri: neutral Select/canvas focus model CmdZ/ShiftCmdZ still each dispatch once; native menu pointer model Undo/Redo remains synchronized; text edit then blur does not divert model shortcuts into stale text history.
- Actual Tauri: popup open → first Escape closes popup only with value, focus, routing and inspector preserved; second Escape from now-closed select closes Both inspector and restores opener focus. Closed select Escape directly closes normally. Repeat pointer and keyboard-open popup; committed selection followed by Escape; neighboring text/button Escape; Model view docked inspector and narrow drawer path where applicable.
- Keep existing defaultPrevented and editable guard unit checks. Their current synthetic events prove guard behavior, not native undo or native popup cancellation. Re-run affected shell/page/palette/drawer cancellation checks after arbitration changes.

Affected surfaces are native Edit responder behavior and all editable controls if menu handling changes; Escape arbitration affects inspector-contained native selects, routing portal, narrow drawers, palette/page precedence and focus restoration. No renderer, applier, model/schema change is implicated. Separate page-close focus work remains manager-owned.

## Remaining boundaries

No independent UI reproduction, event trace, current WebKit :open test, or exact native dimensions was available to this child. No new UI/debug surface was created. No browser result substitutes for Tauri. Parent must retain exact API provenance and choose any implementation; this return grants none.
