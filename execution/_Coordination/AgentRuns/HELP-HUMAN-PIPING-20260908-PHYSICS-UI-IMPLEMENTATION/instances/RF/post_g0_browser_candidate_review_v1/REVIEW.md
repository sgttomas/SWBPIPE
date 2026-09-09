# RF post-G0 browser candidate review

Verdict: `PASS`

The preserved two-project failure is fully explained by stale test sequencing. Canvas placement supplies node ID, label, unit, and coordinates while deliberately setting provenance to `""`. `buildNodeCreationSubmission` calls `validateNodeDraft`, which rejects blank provenance; `nodeDraftValid` is therefore false and the Add button is disabled. The log records the same disabled element 24 times over ten seconds in each browser project, so the evidence does not indicate a render delay or production regression. Existing App coverage independently requires blank initial provenance and disabled Add, then enables Add only after explicit provenance entry.

RU V2 patch `e7fc8179bd12f00969c8692ef2bc3ee5a6209f3a1f3dfa9014d79ea0d202fbcc` applies in memory to the exact live preimage `3f500eb5e061ab0581123d8cc28d581fa4a17b654afc247d624f60af4dfe6f17` and reproduces candidate postimage `be0650f640986a9282db43803107439f0c08d71a7c37639508e3758fa6c13248` byte-for-byte. It removes nothing and adds exactly three lines:

- a disabled-before-provenance assertion after canvas-derived geometry;
- explicit `invented_synthetic_ui_acceptance_input` entry before the retained enabled assertion;
- `payload.provenance` entry in `fillNodeDraft`.

The candidate strengthens rather than weakens the readiness proof. The explicit canvas witness is nonblank and contains the established `invented` marker. The helper has exactly two active call sites, for `node:R2-100` and `node:R2-110`; their existing rehearsal payloads contain nonblank `invented_a12_rehearsal_user_input`, so no fixture data is invented or overridden. The same prepared canvas draft is later queued for the engine-route receipt, so the first insertion covers both its readiness and eventual consumption. The only other active browser spec using `queue-explicit-node-intent`, `linear-authoring.spec.ts`, already fills provenance before Add. No active browser consumer assumption remains unaddressed.

Bindings: RU V2 binding `4bc27366c74035128642b644fedfdeff19d51964f788b9b2ccbedb80ed334907`; raw patch `e7fc8179bd12f00969c8692ef2bc3ee5a6209f3a1f3dfa9014d79ea0d202fbcc`; candidate postimage `be0650f640986a9282db43803107439f0c08d71a7c37639508e3758fa6c13248`; RU V2 return `7d5cb1606bd73539ab5233998d0b1c168997487540a5b4ff457cc1ae8691d225`; RU V2 output manifest `e4cd61ccdd07d9b33f94788839a4074c2bbed3cd36c3a895c33437db95865fa6`; preserved failure log `e80398806723565a79a1770dcb4d7e4279286daaa6237930c8865ded4f837482`.

This is an unapplied, untested candidate review. A bounded Owner scope amendment is required before applying the one-file test correction. After application, the two named journeys must run in both configured Playwright projects. This review does not certify that any test passed and does not accept the separate six-path applied tranche.
