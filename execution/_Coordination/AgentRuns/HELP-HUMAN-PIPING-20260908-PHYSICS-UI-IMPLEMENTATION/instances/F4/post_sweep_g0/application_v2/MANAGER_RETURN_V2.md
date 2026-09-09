# F4 browser provenance repair manager return V2

Status: `BLOCKED_VALIDATED`

The reviewed one-file patch was applied exactly once. `apps/desktop/e2e/r2-smoke.spec.ts` now has SHA-256 `be0650f640986a9282db43803107439f0c08d71a7c37639508e3758fa6c13248`; decoded patch SHA-256 is `e7fc8179bd12f00969c8692ef2bc3ee5a6209f3a1f3dfa9014d79ea0d202fbcc`; the delta is three additions and zero deletions. Reverse application reconstructs preimage `3f500eb5e061ab0581123d8cc28d581fa4a17b654afc247d624f60af4dfe6f17`.

The single no-retry Playwright invocation selected both required journeys in both configured projects. All four executions failed after clearing the original line-552 provenance gate:

- both main journeys reached and passed the changed V4 numeric assertion, then failed at line 785 because the test expected `1 queued; 0 applied` while the current direct-review flow reported `0 queued; 0 applied this session`;
- both from-blank journeys failed at line 907 because `viewport-intent-unit-validation-create_node` was absent after the current direct-review queue action.

These failures are preserved without retry or weakening. Child RETURN is `c415ceb917faaedb980086f457eb23afeda42069449d9289cd46b594b2a70890`; child STATUS is `d5876f92d83434a3d4004fba024f2c106722ba49aa1242cc0733b81844495e8a`; lossless browser log is `1026851786303ecef405be5fdce91699a13ea3f69a32f7ba2d7b631509275ca9`; encoded contexts are `2b2c26161c7807b01bf27ad3e4c38265cc19679f1ed8ad4baa1c19a4f17b16d3`; trace inventory is `1a2be06b9cbbafe6a70d22f005fd8b8adac8e610eb8acdf31ad9518473809135`.

The updated 17-member binding is `94b19000230b24001d98101916c55e9e891634cf1d90e927bf6309881f05530b`, aggregate `847866e6be422966e285968bd280b1bfe5415d084419f263b64936c977a93801`. Exactly the one authorized test member changed; the other 16 source/test/fixture/generator members remain byte-identical to application V1. Manager validation is `60a9469fe07070c25451549b9e53dfdabef061a9549486f5bdbce47c26479c24` and confirms every evidence member and encoded payload, reverse patch applicability, scoped diff whitespace, and zero F4 path-anchor or candidate-whitespace findings.

Root has accepted the actual resource-release checkpoint: no browser or build process remains, and F4 has transferred exclusive `r2-smoke.spec.ts`, browser, and build ownership for the broader stale-flow repair to U7. F4 will make no further source or test change. Acceptance, native work, full DEC-025, practitioner checks, Git acts, and publication remain held.
