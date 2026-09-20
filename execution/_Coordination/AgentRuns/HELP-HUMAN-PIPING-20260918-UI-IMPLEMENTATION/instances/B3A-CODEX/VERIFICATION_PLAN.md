# B3A verification plan — working

One authoring TASK owns coupled product changes; manager integrates and verifies.
ROOT supplies fresh independent review and one clean-candidate sweep. No duplicate
full browser lanes during implementation. Reserve test/native slots through ROOT;
I1 has initial focused browser5174 slot. Pinned Playwright Chromium, one worker/run
lock, exact source/binary identities. No performance qualification or private models.

State regression matrix:
- untouched loaded-source preview baseline, without claiming saved-file standing;
- real operation changes canonical model; selection/theme/view/display-unit no-dirty;
- save current→clean, edit again→dirty, Undo to verified saved hash→clean, Redo→dirty;
- failed write retains baseline, invalid response cannot clear, project switch rejects
  previous generation's completion; same project id reopened is still new generation;
- write lands after a newer same-project edit: baseline updates to actual persisted
  snapshot, current model/history/result unchanged, Undo to it clean;
- initial/Open baseline hash pending then edit; out-of-order hash completion; no stale
  false clean or stale project title; supported normalization and Historical handling;
- existing integrity failure/supersession tests unchanged (B3B deferred).

Status matrix: MECHANICS_SOLVED/MODEL_INCOMPLETE unchanged; unknown/blocked recorded
mechanics gets only display Solver · Not solved with exact raw source accessible;
failed no-result uses explicitly labelled Solve job state; ordinary cancellation,
never-run complete model and Historical retain no-current-chip policy. Unknown input
is declared invented test data, not a claim the current solver generated it.

Connected browser: disposable New Blank→domain edit→Save→new edit→Undo/Redo→reopen,
heading marker and original operation/history/current-historical state; both configured
sizes, keyboard/pointer as relevant. This is a selected journey, not a full sweep.

Actual native: rebuilt uninstrumented source, fresh verified PID/hash. Exact disposable
New Blank row persisted; author/apply one node through normal route, verify Edited
heading and native title; Save, another edit, Undo/Redo and exact reopen verify mark
and contents. Theme/view/selection changes retain clean/dirty state appropriately.
No save/delete of user projects, no destructive cleanup. Native failure injection is
not required or manufactured; controlled async/race tests cover those states. Record
actual inability and retain prior native evidence if source changes require rebuilding.
Restore testing preferences if changed, release UI and stop owned app for sweep.

Touched controls for deferred closing appearance pass: project heading Edited mark,
native title and Solver fallback chip/popover/Analyze raw-source disclosure. No colour
or fine-spacing tuning, no new authority/engineering status or persisted field.
