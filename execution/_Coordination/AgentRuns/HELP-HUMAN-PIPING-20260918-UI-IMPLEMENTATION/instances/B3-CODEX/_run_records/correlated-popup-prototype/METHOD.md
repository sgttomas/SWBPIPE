# Diagnostic correlated popup prototype

TASK Type 2, assigned Astra/low; no delegation or production/Git writes.

Commands ran with explicit isolated desktop workdir: npm run build; npm test -- src/features/workspace/correlatedPopup.test.ts; npm run tauri -- build --debug --bundles app. Source archive and hashes bind exact bytes. All 13 tests pass. These model delivered state, not native ordering.

Native notifications synchronously update monotonically increasing menu generation, tracking, action, end control key, and key-window epoch. The Tauri snapshot command dispatches a read through run_on_main_thread and replies with that state. No native push event is used by the decision.

On an actual DOM :open observation, a snapshot request starts. Its response must still have native tracking=1, and the same DOM select must still be :open in the same frontend ownership generation. Only then is its native generation bound. A response arriving after native closure is rejected, even if its menu generation looks plausible. This deliberately tests whether scheduling supplies sufficient public correlation.

An ambiguous Escape shares one claim on the native KeyboardEvent between narrow React and global window handlers. Its snapshot must match bound native menu generation and window epoch. Native action indicates same/changed commit; native endKey=53 or still tracking means popup cancellation. Outside non-action closure permits the later intentional Escape. Reply must match captured input, focus, and view generations; keyup-only cancellation clears ownership; quick later input invalidates pending work. Reopen and focus/input/change clear old ownership. A failed correlation logs STOP and does not close a pane.

Critical limits to test: run_on_main_thread is a queue barrier, not yet proof of action classification timing on this host. A live tracking response may be impossible during native menu tracking. Original pre-keydown-close failure is not yet reproduced. Do not regard this diagnostic prototype as an accepted repair.
