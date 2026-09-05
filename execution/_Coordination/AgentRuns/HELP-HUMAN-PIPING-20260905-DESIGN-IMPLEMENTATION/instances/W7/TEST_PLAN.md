# W7 verification plan

Derivative plan bound to PLAN_V1 and U1 v3/U2 v2, owner window target1024x768. No independent usability or numerical acceptance claimed.

| Risk | Evidence | Authority mapping |
|---|---|---|
| Model displaced by tool catalogue/options/drawers | Before/after screenshots and actual canvas/window/ancestor rectangles at1024x768, eight report states; check visible controls and no page overflow |07-01 REQ01/09 AC001;07-06 AC001|
| Text label treated as quantity | Focused text/ref/numeric field regression; existing dimensional validation tests |07-02 RQ002/003/004 AC001|
| Commands lost while shortening UI | Catalogue search/route tests; App journey and deadcontrols tests; eightstates browser|07-01 REQ04;07-02 RQ006|
| Changed queue/validation/acceptance/undo | Existing App atomic/stale/undo tests and realWasm batch e2e afterfreeze |07-08 REQ003/004 AC001|
| Hidden warning/focus trap | Keyboard Escape, focus return, inline error screenshot; command availability detail reachable |07-04 AC001;07-06 AC001|
| Browser/native evidence conflation | Browser screenshot provenance explicit; native build/walkthrough parent owned afterfreeze |07-08 REQ002/010|

Registered checks: desktop-test,desktop-build. Parent owns harness-self-check,harness-pytest plus complete DEC025 only after committed clean source freeze. Browser screenshot script and focused Playwright are authorized runtime verification under parent explicit browser/native evidence request; host escalation required when sandbox denies. No geometry/numerical behavior is changed or accepted.

Eight states: blank; routing existing endpoint tool; selected pipe; support editor; load editor; bundled fixture results (browser mechanics limitation labeled); field error/repair; exact batch review. Values are public invented fixture or explicitly invented rehearsal values. Browsermemory save/reopen can be exercised but never called durableSQLite evidence.
