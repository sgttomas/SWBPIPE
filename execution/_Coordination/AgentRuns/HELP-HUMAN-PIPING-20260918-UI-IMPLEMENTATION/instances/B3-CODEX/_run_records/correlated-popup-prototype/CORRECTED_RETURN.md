# Corrected correlated popup prototype — partial native proof

TASK Type2, assigned GPT-6 Astra/low, delegated-harness-native child of B3-CODEX WORKING_ITEMS under ROOT. No delegation or production/Git writes. Same instruction-enforced isolated scope on unrestricted host. Initial failure RETURN.md and seal.json preserved unchanged. ROOT addendum1 hash1180e1595b4d6f7a857571171865c46031864dd1e1c342a03d63aa7de50d595f authorized exactly the owning-window epoch correction. corrected-basis/hashes/patch/launch bind corrected bytes.

## Outcome

The narrow correction fixed the reproduced initial implementation defect: Tauri's public ns_window() supplies the known owning webview NSWindow; the observer increments focus epoch only for that window and logs other app windows separately. Spaced Space→same-value Return→Escape now closes Inspector and restores opener focus. Native gen1/epoch0/action1 matched its live binding (corrected-runtime.log31–47), while the popup-window resign was explicitly owningWebview=0. This is actual native proof of same-value commit classification in that history.

Live native snapshot binding works for the tested Space openings and one pointer opening. Correction to preliminary progress wording: pointer gen3 DID produce click :open=true and bind valid=true (lines107–111); initial pointer route and corrected same-value pointer gen4 did not. Pointer ownership is therefore intermittent across observed routes, not universally absent. No tested first popup Escape delivered the original problematic DOM Escape keydown after native closure. All observed first cancellations delivered keyup only. Visual first/second-Escape success therefore does not prove guard interception of the original defect.

No general production repair is established. The current bridge cannot prove ownership for every pointer route, never exercised its actual native cancel=true deferred decision, and has incomplete host-focus-return/native narrow-consumer coverage. No further layer or production adoption was attempted.

## Evidence matrix

| Scenario | Observed result and limit |
| --- | --- |
| 13 state-machine tests | PASS before native-only correction; exact frontend/test hashes unchanged. Models delayed/reordered replies, duplicate consumers, stale input/focus/view/reopen/dispose, keyup-only clearing and child consumption. Not native proof. |
| Corrected app-only build | PASS npm run tauri -- build --debug --bundles app; corrected-build.log. No DMG/Finder. |
| Space same-value Return→Escape first retest | PASS actual correlated action snapshot; Inspector closes/focus restores. |
| Space first/second Escape | PASS visible contract, gen2 binding; first only keyup. Original keydown guard path unproven. |
| Pointer first/second Escape | PASS visible contract, gen3 click :open live binding; first only keyup. |
| Same-value pointer→Escape | PASS visible contract, gen4 native action but no binding; not bridge proof. |
| Space open→changed pointer m→mm→Escape | PASS; gen5 action and DOM input/change clear ownership. Draft change only, no applied operation. |
| Space open→outside pointer→Escape | PASS correlated gen6 no-action closed snapshot, Inspector closes. Outside pointer did not move DOM focus. |
| Repeated inspector reopen and popup reopen | PASS as preparations across generations1–8; invalidation after actual host focus return not exercised. |
| Quick consecutive Space/Return/Escape | Host still handled Escape in native menu: end currentEvent Escape AND action notifications, DOM only Escape keyup. Inspector retained; next separate Escape closes. No guard classification ran, no timing claim beyond raw native log. |
| Genuine owning-window focus exit | PASS signal scope: minimize logs owningWebview=1/window115645/epoch1; popup windows had owningWebview=0/epoch0. |
| Host focus return/reopen after exit | NOT VERIFIED: CUA AX Raise/repeated minimize did not restore, Dock binding timed out. Tool-route boundary only. |
| Narrow/global duplicate consumer native witness | NOT VERIFIED; modeled unit test only. No forced below-minimum window geometry. |
| Model docking | Both tables stayed docked throughout observed actions; dedicated Model-view native scenario not exercised. |

## Causality, delivery and feasibility

Binding requires native tracking1 plus the same genuinely :open DOM select when the reply arrives. Interleaved or reordered snapshots that arrive after closure or a new frontend generation are rejected, not retroactively bound. Escape queries native state through main-thread dispatch and requires the bound native generation/window epoch plus input/focus/view validity. One KeyboardEvent claim is shared across React narrow and window-global consumers. No native asynchronous push arrival decides classification.

Actual proof is narrower than the model: gen1 committed and gen6 outside-cancelled snapshots correctly authorized later closed-select Escape. The original native pre-keydown-close cancellation was never reproduced, so cancellation-side causal timing and complete pointer correlation remain unknown. Rapid gen7 shows end currentEvent Escape can coexist with action; action state must take precedence. Raw DOM logs cross an async invoke and their file order does not prove cross-channel handler order.

The command dispatches to main then blocks on channel receipt inside an async command. That is retained diagnostic architecture and needs production scheduling review; these tests do not establish a universal main-loop barrier contract. Native menu generations are process-global, not DOM IDs. The concrete observer-scope defect is fixed; reliable full-contract bridge feasibility remains unproven.

Minimal owner tradeoffs if this bounded evidence is insufficient: keep native select with the unresolved Escape limitation, intentionally consume focused-select Escape until focus leaves (changes second-Escape behavior), or own the affected accessible popup interaction in the app (adds keyboard/accessibility implementation and testing). Do not adopt this prototype as production repair on this return.

## Process and custody

Initial PID24955 absence verified before corrected launch; corrected PID26433 bound to explicit isolated app binary/source. Only project:blank-local-20260920t043300z used; no private model/field entry, save/delete or applied operation. Final draft mm, Both, Node armed, Inspector expanded, pending0, Undo/Redo disabled, popup/devtools closed, main window minimized. PID/path/start/binary reverified, SIGTERM sent, ps exit1 confirms absent. Exclusive native/CUA slot RELEASED. Original seal verified unchanged; corrected-seal.json hashes new evidence.
