# Async primitive readiness diagnosis

Candidate: `b9dda2addcccb2adb45f618140425aec533314c0`.

Frozen symptom: retained sweep `validation/evidence/sweeps/SWEEP_20260924T162044Z_b9dda2addccc.json` reports nine App failures. Four primitive queue/apply tests read `Validating unit dimension…` instead of their exact operation IDs; support deletion clicks Queue while disabled and cannot find its apply row; four case/target selection checks assert enabled before validation completes. Expected outcomes remain exact operation identity, magnitude/unit/target, queue/apply model state, support-reference deletion rejection, and honest deliberate selections without direct model mutation.

Read-only causal trace: `LoadCaseManagerPanel.tsx` derives a unit/dimension request identity (166–186), asynchronously validates through `convertDisplayQuantities`, and permits an intent only after the current request succeeds. Initial mount or category change therefore renders a pending diagnostic and disabled queue. The conversion service awaits the WASM engine in browser mode. Queue disabled/onClick guards are at 653–669. Existing synchronous tests cross that asynchronous boundary without waiting. This is the leading test-readiness diagnosis, pending narrow reproduction; no product repair conclusion is claimed yet.

Narrow reproduction requested from the manager: direct Vitest `src/App.test.tsx -t 'queues and applies an? .* primitive load through the manager panel|blocks support deletion while an imposed-displacement load still references it|primitive case selection display' --maxWorkers=2`. This includes pressure/thermal neighboring controls. Test execution is manager-owned and held pending CPU-lane handoff. No source edits precede reproduction.

Bounded planned repair: use ordinary `waitFor` for exact operation previews or enabled queue after required inputs, preserve all original semantic assertions, and prove pending validation cannot queue. Existing `LoadCaseManagerPanel.units.test.tsx` supplies controlled pending/unavailable/stale-reply coverage and should run unchanged alongside the repaired tests. No timeout expansion or weakened oracle is planned. Full affected App suite and fresh independent review remain manager integration checks.

## Reproduced and repaired

Manager-owned baseline `../_run_records/baseline.log` and `baseline.result.json` reproduced exactly **9 failed, 2 passed, 207 skipped** with unchanged App source before/after (`d44d00f1946679e50b7deedd292870ed18aaa74405931183df463c6400250a14`). The two passing controls were pressure and thermal queue/apply tests. Baseline exit was 1. This confirms the first divergence is test observation/click before asynchronous readiness, not an altered intent payload. The product's pending guard correctly prevents queuing.

Repair changes only seven assertion sites in `apps/desktop/src/App.test.tsx`: four exact operation-ID preview assertions now use normal `waitFor`; support deletion waits for its primitive Queue control to be enabled; two selection assertion sites (one parameterized across three targets) await enabled Queue after deliberate selection. No timeout override, product changes, or semantic assertion removals. Magnitude edits do not change the unit/dimension request identity, so the existing subsequent synchronous Queue clicks remain meaningful after preview readiness. Existing empty case/target disabled checks, exact payloads, apply outcomes, model counts and support-deletion rejection are intact.

`git diff --check` passed. Frozen repaired App SHA-256: `686687e5b124c393254abd08699183203fbcd417b361368334d46dd478b33028`.

Manager-owned repaired narrow run, unchanged four controlled unit-readiness tests, full affected App suite and fresh independent review remain outstanding at this writing. Existing deferred-service tests provide deterministic proof of pending/unavailable/stale/A→B→A blocking without adding timing-dependent App assumptions.

## Manager validation return

The manager executed and retained `../_run_records/focused_repair.log` / `focused_repair.result.json`: **15/15 passed** (11 selected App cases plus all four unchanged deferred unit-readiness controls; 207 other App tests skipped). `../_run_records/tsc.log` / `tsc.result.json` records passing TypeScript checking. This child did not execute these checks. The repaired source stayed frozen. These outcomes support the test-readiness diagnosis and preserve deterministic pending/unavailable/stale-response blocking proof. Full affected App suite and independent review completion remain integration-owner checks; no full-suite or release claim is made here.
