# F4-I4 browser repair return

Status: `BLOCKED` pending manager validation and a versioned successor scope. The reviewed one-file patch was applied exactly, but both required journeys failed in both configured browser projects. No retry, additional repair, or acceptance claim was made.

## Exact application

- Source/test path: `projects/chirality-piping/apps/desktop/e2e/r2-smoke.spec.ts` only.
- Preimage SHA-256: `3f500eb5e061ab0581123d8cc28d581fa4a17b654afc247d624f60af4dfe6f17`.
- Post-image SHA-256: `be0650f640986a9282db43803107439f0c08d71a7c37639508e3758fa6c13248`; 101,982 bytes.
- Archive-decoded patch SHA-256: `e7fc8179bd12f00969c8692ef2bc3ee5a6209f3a1f3dfa9014d79ea0d202fbcc`.
- Patch delta: three additions, zero deletions. Reverse reconstruction from the live post-image reproduced the exact sealed preimage.
- Application proof SHA-256: `cd0468986a213faabbedd003a775269fec8ea184ad88b4dde104056aa00c463a`.
- Lossless patch-application log SHA-256: `60481125148e90ff5302da025e9672a16aefd7bd112b3df9b79a90b355b38706`.
- No source, test, or fixture path outside the one-file fence was changed by this child.

## Required browser validation

One no-retry Playwright invocation selected exactly the two named journeys in both configured projects. Result: 0 passed, 4 failed.

- `R2 desktop preview smoke covers solve, results, report, and viewport overlay` failed in `chromium-desktop` and `chromium-compact` at line 785. `operation-apply-summary` expected `1 queued; 0 applied` and received `0 queued; 0 applied this session` after 10,000 ms.
- Before that later line-785 failure, both main-journey projects reached and passed the changed V4 numeric assertion.
- `R2 from-blank GUI journey authors the A12 rehearsal script` failed in `chromium-desktop` and `chromium-compact` at line 907. `viewport-intent-unit-validation-create_node` was not found after 10,000 ms; the expected text was `unit_validation=length=model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview`.

The lossless browser log SHA-256 is `1026851786303ecef405be5fdce91699a13ea3f69a32f7ba2d7b631509275ca9`. Four error contexts are encoded in `PLAYWRIGHT_FAILURE_CONTEXTS.base64.json`, SHA-256 `2b2c26161c7807b01bf27ad3e4c38265cc19679f1ed8ad4baa1c19a4f17b16d3`. Four traces are preserved and individually bound by `PLAYWRIGHT_FAILURE_ARTIFACTS.json`, SHA-256 `1a2be06b9cbbafe6a70d22f005fd8b8adac8e610eb8acdf31ad9518473809135`.

## Frozen binding and evidence

- Updated 17-member binding SHA-256: `94b19000230b24001d98101916c55e9e891634cf1d90e927bf6309881f05530b`.
- Updated aggregate serialization SHA-256: `847866e6be422966e285968bd280b1bfe5415d084419f263b64936c977a93801`.
- Exactly one binding member changed from application V1; the other 16 members match their prior hashes and byte counts.
- Authority binding SHA-256: `355a49046b722cc017faa12762c65511b40a6e4c80ce17fb46667b2fa8043157`, including owner direction `adfa58c0…4326`, dispatch amendment `331cad21…24b4`, work graph V4 `b907dcaa…f492`, and I4 direction-binding amendment `3ec0122a…d0c`.
- Validation summary SHA-256: `d6f3e04b49dd4afa29342dc63090c1d08ff0212a26840650c5a54ea8b4407e7b`.
- Evidence manifest SHA-256: `f2ac07eb5e641fc67ebcdb60f6bce5ca9b0ab8fe4bb94627039021dfd88a087a`; aggregate member serialization SHA-256 `53947ccb33e7d6779a2348e2dd575a9de060eafff9b846f201f328d2c76c0578`.
- Temporary browser output was removed after the failure contexts and traces were preserved.

Residual risk and blocker: the two required browser journeys remain failing in both viewport projects, so the application cannot advance to acceptance. Any repair requires a versioned successor scope and fresh review. Native build, full DEC-025, practitioner suite, and previously passed non-browser checks were excluded and not run.

Exclusive one-file browser-repair ownership is released with this terminal return.
