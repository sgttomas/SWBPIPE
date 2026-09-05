# U1 return V2 — Support command disclosure repair

Status: READY_FOR_MANAGER_VERIFICATION; source frozen in SOURCE_FREEZE_V2.json.
Parent authorization: AMENDMENT_V6.md. Agent2 TASK, no delegation; actual model unknown. All V1 evidence remains unchanged.

R1 found one P2: direct Support toolbar/Insert-menu commands armed the tool without opening the support creation disclosure. App now requests focus on create-support-id through its existing ancestor-Details opener whenever handleArmCreationTool receives support. The original selection, armed state, queued operations and creation handlers remain unchanged.

Changed only App.tsx (one-line presentation routing) and App.test.tsx (six route/selection regression cases). SOURCE_FREEZE_V2.json includes hashes for all eight U1 source files and confirms only these two differ from V1.

Focused verification: _run_records/support-disclosure-v2.log PASS8/8,118skipped. Includes toolbar and Insert-menu route from project/node/pipe selections; intended disclosure initially closed then opened; target receives focus; inspector expands; selection and canvas identity preserved. Existing12 Toolkit-route/canvas-navigation regression checks also pass. Non-fatal existing asynchronous SectionAssignment React act warning recorded in log; no failed assertions.

scope-validation-v2.json PASS; git diff --check PASS. Parent owns final browser directSupport check, combined appropriate final checks, and fresh re-review. Prior central742-test PASS is earlier evidence, not a claimed post-fix full-suite result. No other product changes, no Git or model mutation, no new owner decisions.
