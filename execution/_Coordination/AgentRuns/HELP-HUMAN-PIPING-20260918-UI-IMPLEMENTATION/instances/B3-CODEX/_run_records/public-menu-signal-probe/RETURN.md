# Public AppKit menu-signal probe return

TASK Type2, GPT-6 Astra/low, delegated-harness-native child of B3-CODEX WORKING_ITEMS under ROOT. No delegation, production writes, Git mutation, private APIs, keyboard monitors, permissions or dependencies added. Unrestricted host; scope enforced by instructions. Read instruction origins/hashes in basis.json. Sealed brief hash matches launch. All code changes confined to isolated f5 probe; C consumer restored using f5 bytes and removed helper bytes retained here. B predefined responders and passive instrumentation preserved.

## Result

Public NSMenu notifications DID distinguish tested same-value commits from cancellations on this host. This establishes a concrete diagnostic signal, not a completed repair or universal reliability.

| CUA history | Native evidence in runtime.log | DOM / visible result |
| --- | --- | --- |
| Ordinary View → Both command | lines4–5 will/didSendAction | Both retained. CUA AX menu invocation did not produce begin/end; positive action control only. |
| Pointer-open m → Escape | lines16–21 begin/end; end type10 key53; no action | Only Escape keyup delivered; Inspector stays open. |
| Space-open m → Escape | lines26–31 begin/end; end type10 key53; no action | Only Escape keyup delivered; Inspector stays open. |
| Space-open m → Return, same value | lines36–39 begin/end then will/did action, same menu0xc8898be40; end type11 key36 | No DOM Return/input/change; m retained. |
| Pointer-open m → selected m | lines42–47 begin/end then will/did action, same menu0xc88988700; end type13 | No DOM input/change; m retained. |
| Pointer-open m → mm | lines50–59 begin/end then will/did action | DOM input/change; mm visible. |
| Space-open mm → outside click | lines64–67 begin/end, end type1, no action | Popup closes; mm retained; click reaches Modeling workspace SECTION. |

Raw log and menu-extract.txt preserve actual sequence and menu identity. Menu objects differ per popup; they are not stable DOM-control identities. Crucially didEndTracking precedes will/didSendAction on commits, so a consumer must not equate end-tracking itself with cancellation. At Escape end, public currentEvent provided type/key53 in both observed cases. The ordinary AX menu positive control had no currentEvent; -1 explicitly means unavailable, not cancellation.

## Causal confidence and limits

High confidence in the observed distinction: unchanged commits missing from DOM do emit public action notifications in this build/environment. No polling/timing heuristic was needed to observe it. Public end-event metadata additionally separated tested Escape and outside-pointer endings. Native log emission is synchronous; DOM logs arrive through async Tauri invoke, so cross-channel file order does not prove a usable delivery ordering to a future frontend handler.

The original failure history (Escape keydown delivered after native closure) did NOT reproduce in this run: tested Escape closures delivered only DOM keyup. Therefore this probe does not prove that a frontend bridge will classify that original keydown before shell dismissal. It also does not prove correct one-event consumption when cancellation emits no keydown, later intentional Escape, multiple controls/windows, other macOS/WebKit versions, other menu routes, or teardown. No native minimum resize or uninstrumented acceptance performed. No guard implemented.

A bounded future repair option is a public native menu lifecycle bridge with explicit menu/session correlation and Escape event identity/consumption, tested against original keydown-close reproduction plus same-value Return/pointer commits, outside cancellation and later Escape. That is an option, not selected design or repair authority. If bridge causality cannot be established, concrete interaction choices remain: retain native select with documented Escape limitation; intentionally make focused select consume Escape and require focus exit before pane dismissal; or replace affected popup with an application-owned accessible selection interaction. Each changes behavior/cost and needs owner choice and relevant keyboard/accessibility coverage. No further mechanism guessed or probed.

## Evidence and return boundary

App-only build `npm run tauri -- build --debug --bundles app` passed, build.log. prepared-hashes.json binds helper/build/lib source and bundle binary; public-menu-probe.patch plus public_menu_probe.m reconstruct added diagnostic code; before.patch and removed-C helpers preserve rejected C state. launch.json binds fresh PID19891/path/time/base/binary; verified-process-before-stop.txt captures OS start and exact command. ACTIONS.md records exact CUA calls and visible results. Raw AX/screenshot observations are in tool transcript, not claimed as standalone files.

ROOT clearance arrived through manager before any launch/CUA action. Final blank project project:blank-local-20260920t041132z, Node armed, Both, Inspector open, unit draft mm, pending changes0, Undo/Redo disabled. No saved/deleted model rows or applied operations. PID19891 path and hash verified, SIGTERM sent, subsequent ps returned no process (exit1). UI slot RELEASED. Probe is diagnostic only; no production acceptance, merge or design selection.
