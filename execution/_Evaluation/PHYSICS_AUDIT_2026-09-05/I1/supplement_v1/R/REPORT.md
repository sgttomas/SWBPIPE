# Additive native-failure service experiment — I1-R-09

## Basis and execution
Frozen production source2be412ccea62bdc4bd96deb082c46d7a792076ea. Original accepted I1 packet and children/R remain immutable. This derivative supplement executes the exact parent brief; no production source, GUI, user project/database, Git, or network changes. TASK Agent2 no delegation, actual model not exposed, role enforcement instruction+config asserted. Questions and harness were frozen before execution.

One isolated Vitest case passed, covering unchanged and edited models through the actual imported previewService. Runtime window shim activates native detection; only Tauri invoke is mocked to reject with I1_R_INJECTED_NATIVE_FAILURE. Assertions confirm both native job-start and direct mechanics commands were actually attempted. This is an actual service-layer execution, not a simulated copy of the service and not an actual native process failure.

## Observations
Both requested dense_scrutiny. With unchanged bundled model, the native job rejection returns receipt mode browser_fixture_no_backend_job, then direct mechanics rejection returns MECHANICS_SOLVED and830 fixture rows. Returned mechanics exactly equals the ordinary browser baseline; neither envelope nor receipt preserves the injected error. The fixture contains sparse_interactive solver-mode evidence even though the requested mode was dense_scrutiny.

With edited model (one coordinate +.125), the same rejected commands return the same explicit browser receipt but MODEL_INCOMPLETE, zero result rows, and BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL. This protects edited-model input association; do not claim stale solved results are returned for edits.

Immediate consumer inspection App.tsx653-721 checks the receipt and follows direct fallback only without a backend job. startSolveJob records backend_job_seam=browser_fixture_no_backend_job and a browser-fixture message. The fallback route therefore is not universally hidden: the job audit explicitly labels it. However, runPreviewMechanics itself returns only MechanicsResult, so its direct callers do not receive a transport-error/fallback marker. App.tsx723 onward subsequently constructs input-manifest solver fields using the requested mode; the full frontend-manifest path was not executed in this supplement and is a grounded follow-up concern, not a witnessed manifest defect.

## Calibration and recommendation
I1-R-09 UNKNOWN is refined to REPRODUCED_FALLBACK_WITH_DECLARED_JOB_MODE. Confidence high for service behavior; severity medium for provenance/diagnosis. The narrow confirmed observation is swallowed native failure plus different requested/fixture solver modes, with explicit job-mode disclosure and edited-model blocking. It does not prove misleading GUI presentation or a physics-kernel defect.

Preserve the native rejection reason and actual result origin/mode through the service contract. On native failure either report a failed solve or make any example-fixture fallback explicit to all consumers, and avoid binding requested solver settings as if that solver executed. Choosing this product behavior/compatibility belongs to the parent implementation brief. No repair made here.

## Rerun and handoff
Use the recorded command/config and native-failure.test.ts in a new additive packet rather than overwrite this immutable evidence. Existing dependencies suffice; no full build or suite needed. Native injection only; restore globals after test. Recheck unchanged versus edited models, requested versus actual result mode, job disclosure, and error preservation. Add actual UI/manifest verification if fixing that downstream contract. Parent fan-in and whole-baseline independent review remain required; no lifecycle/engineering acceptance implication.
