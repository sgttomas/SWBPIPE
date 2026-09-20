# Correlated popup diagnostic prototype — stopped failure

TASK Type 2, assigned GPT-6 Astra/low, delegated-harness-native child of B3-CODEX WORKING_ITEMS under ROOT HELP_HUMAN. No delegation. Scope was isolated f5 probe source/test/build plus own evidence, instruction-enforced on unrestricted host. No production/Git writes. basis.json records actual origins/hashes; before.patch and before-public_menu_probe.m preserve starting diagnostic state. Prototype archive, prepared-hashes.json, and launch.json bind tested source/binary/PID. This is diagnostic evidence, not final uninstrumented native acceptance.

## Result and causal confidence

Live Space-open correlation IS possible on this host with public native state: DOM Space keyup observed :open=true, native main-thread snapshot returned generation2/windowEpoch1/tracking1, and frontend response still saw the same select open (bind valid=true). This establishes a usable live observation in that tested history; it does not establish all popup opening routes.

The first same-value Return→Escape test FAILED and triggered the sealed stopping rule. Return produced gen2 end key36 then will/did action, with no DOM commit event. The later ambiguous Escape snapshot correctly retained gen2/action1, but windowEpoch had become2. The prototype rejected that mismatch (current=true, correlated=false, STOP), leaving Both Inspector open when it should close. The process-global NSWindowDidResignKey observer is too broad to represent owning-webview focus: its counter changed during this popup lifecycle even though the final focused DOM control remained the same select. High confidence in this implementation defect; exact window identity behind the notification was not logged, so attribution specifically to the popup window rather than another app-owned window remains inference. No repair attempted after this gate.

Pointer-open produced native begin, but no DOM event observing :open and therefore no native ownership binding. First Escape delivered only keyup and preserved Inspector. That is visual behavior only; the original pointer/post-close-keydown failure was not reproduced or repaired. Original Space/post-close-keydown cancellation failure also did not occur before stop. Async raw DOM/native line arrival is not treated as handler ordering. Native snapshot/action sequence is proof only for tested history.

## Verification matrix

| Check | Result |
| --- | --- |
| 13 unit/state-machine cases | PASS modeled delivery only; delayed/reordered replies, duplicate consumers, stale input/focus/view/reopen/dispose, native generation/window mismatch, keyup-only clearing, child consumption |
| Frontend build and app-only bundle | PASS; exact commands/logs in METHOD.md and build.log |
| Pointer-open first Escape | Visually preserves Inspector; only keyup; no live binding |
| Space-open live binding | PASS for gen2, same select still open |
| Same-value Return then intentional Escape | FAIL: window epoch mismatch suppresses closure |
| First/second Space Escape; second pointer Escape | NOT EXECUTED after STOP gate |
| Same-value pointer, changed commit, outside focus/window exit | NOT EXECUTED after STOP gate |
| Reopen, quick consecutive native input, native narrow consumer | NOT EXECUTED after STOP gate; modeled only |
| Model docking | Both tables remained docked throughout observed actions; broader Model scenario not exercised |

## Feasibility and concrete tradeoff

The public menu generation/action signal and a live main-thread snapshot were demonstrated. A production lifecycle bridge is NOT supported by this return: the implemented window correlation failed, pointer ownership remains absent, and the original cancellation keydown was not reproduced. The command uses main-thread dispatch followed by blocking channel receive inside an async command; that diagnostic scheduling choice also needs production review. Process-global menu IDs are not DOM-control IDs. No new speculative layer was added.

ROOT can choose an explicit interaction tradeoff: retain native selects with the bounded Escape limitation; intentionally have focused select consume Escape until focus leaves, changing the second-Escape contract; or replace affected selects with an app-owned accessible selection interaction, requiring keyboard/accessibility work. Preserving the current first/second-Escape contract across all native routes remains unproven. This task recommends no production adoption of the tested prototype.

## Process and UI return

PID19891 verified absent before launch. New owned PID24955 launched through explicit disposable bundle binary, source/binary hash retained. At stop, project:blank-local-20260920t042901z, Both, Node armed, Inspector expanded, m draft unit, popup/devtools closed, pending changes0, Undo/Redo disabled. No field/model operation was applied, saved, or deleted. Exact PID/path/start/binary verified, SIGTERM sent, ps returned exit1/no process. Native UI slot RELEASED.
