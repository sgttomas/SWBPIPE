# F1 V3 held candidate

Authorized four test cases received independently derived fixture count/value sync. See DERIVATION.md and APP_DIFF.patch for every change. No actions, IDs, units, tolerances, filters, timing or behavior changed. Original V1/V2 remain immutable.

Latest targeted_4: 3 PASS, 1 FAIL,122 filtered out. The remaining failure is a selection behavior issue: App.test.tsx:11333 clicks P-120 bending-z:end-j comparison row. ComparisonPanel.tsx:6/17 caps visible deltas at4. Independent existing matching/sorting rule applied to preserved old/new fixtures ranks this row old2 (absolute delta207.350325), new5 (165.986377). It is absent from the DOM. Root notified; no unauthorized test action/ID or UI production adaptation made. Full suite not run because targeted prerequisite is not green.

Earlier targeted_2 repeated the pre-follow-on-edit failure because a preparation assertion aborted before writing its patch; retained as historical execution evidence, not new source verdict. targeted_3 progressed to comparison summary; targeted_4 progressed to selection.

Root should decide bounded selection adaptation or separately scope UI change. Product source and generated fixture remain unchanged. This candidate is not accepted, release-ready, engineering acceptance or a new physics decision.
