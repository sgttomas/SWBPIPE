# Independent affected review — D70 smoke repair

**PASS. No unresolved actionable findings.** The 135-path frozen preparation candidate and the explicitly added successor launch context are suitable for ROOT fan-in and consideration of one untimed smoke requalification. This does not establish smoke success or authorize timed collection.

## Coverage and bindings

Base `fc21b00d0b4233349c7fa8df433d6a82e9f7678e`; candidate `418f22f677ff65677f396abbff754ac3378c9efe`; prior reviewed candidate `3c9142dad05867b113a97cca150a0ec228f6f0d7`.

- Freeze SHA-256: `3a3201e12dc965f9ffb9305e0844ce780aa0148600d25222a4dc410a1846034a`.
- Full binary diff SHA-256: `e17cf74762d06b916bf0c42b62056f737ee82e732f90642ee06f28f886021e38`.
- Full inventory digest: `0d97fdc8259b357835d39f2bae25d6cca1c7630c287dfa13119f974339054c7a` (sorted-key compact JSON files array).
- Prior return SHA-256: `6f208b0dc383e114ac4b90566f4af43464d09b1680c0cb80b556de34f967b690`.
- Successor 34-file method SHA-256: `064195227bf8ef4c050757f280b2b5dc6b1b29e45154e2c5aa1833426410e123`.

All 135 candidate blobs, sizes and working-file hashes match the freeze. Full coverage is 90 unchanged paths carried forward by exact prior-reviewed hash plus all 45 delta paths reviewed now. The delta contains two changed source files, changed phase graph and 42 newly tracked evidence/control files. Original review outputs remain byte-identical. RETURN.json enumerates every path/hash, its coverage basis and five separately reviewed post-freeze context files; those five are not silently added to the frozen candidate inventory.

## Findings and closure

The original smoke defect is confirmed by canonical `controls-smoke-01` failure: actual connected main CANVAS hit evidence has null epochs because smoke deliberately installs no causal observer. The shared timed assertion correctly rejected it; the smoke caller chose the wrong assertion. The original review PASS was static preparation evidence and did not prove this runtime behavior.

The two-file repair closes that source defect. `assertSmokeMainCanvasHitTarget` requires exact connected CANVAS identity, correct test identity, finite positive rectangle, in-bounds point and null causal/armed fields. Each point, box endpoint and orbit check directly consumes a fresh `validateMainCanvasHitTarget` call, which requires one connected main canvas and compares actual `elementFromPoint` identity. Overlay/wrong/disconnected targets fail. No epoch is invented and no observer is installed. Timed guard, timed controller, harness, scorer, extractor, fixture and oracle-freezer bytes remain unchanged; typed oracle/state postconditions remain. The exact observed regression fixture, negative cases and unchanged timed epoch controls are covered by the supplied passing test. TypeScript evidence is passing; no tests were rerun by this reviewer.

One actionable P2 was found in the explicitly added post-freeze `run_smoke_v3.py:5`: replacing the declared oracle-manifest hash with the current file hash would silently bless manifest drift. ROOT accepted the finding and supplied an immutable successor. **Closed in `run_smoke_v4.py` SHA `deeabee87638050e89d9d7f7ea50fc90f331e865a4dbb0ebbbad8cc2c3e7e73f`:** the script preserves the environment's frozen hash and compares it before launch and again in the post-run check; every oracle member is checked for containment, byte count and SHA. V3 and command V3 bytes remain preserved. Use V4 for successor execution; V3 is retained historical evidence, not the approved execution path.

## Governance, evidence and reuse

The path-only preparation repair adds the required `instances/VERIFY` subtree and new camera output path without overwriting the failed initial launch. Launch/process records agree with the declared command/environment records; return codes distinguish failed path initialization, successful two-size camera preparation/freezer, and failed smoke. Zero cohort contribution and the timed hold remain explicit. The phase graph accurately records the smoke defect and bounded repair frontier; old acceptance/launch records remain historical.

Independently hashed all 39 cited canonical files across initial and V2 preparation returns. Camera records show both sizes, real Isometric/Fit Model commands, 200 actionable projection crosschecks and zero measured contribution. Frozen oracle manifest `0b8a0b5239f10547c72907e32160db262288216ed1fb4818551782dc89b05b17` binds 200 point probes and 20 box samples per size, the camera records and source/dependency/visual-token identities. Relevant camera/freezer/helper/product bindings are unchanged by the smoke-only repair, supporting reuse of these preparation outputs without repeating camera runtime. This is applicability review, not an independent re-execution or a new image-quality evaluation.

The archived two-file repair patch exactly equals the actual affected source diff. All 34 successor method members match current bytes and exactly the two authorized members differ from the prior manifest. Structured test evidence records one expected pass, no skipped/unexpected/flaky results or errors; typecheck log is empty consistently with the writer's successful command return. All frozen JSON parses.

The additional command V3 context is hash-bound (`124dbf3324c16d34f8d718ca3bea27d7ea095bfdb9c04117601eba8fb12729c5`), removes already-completed camera/freezer routes, uses the successor method and frozen oracle, and targets fresh `controls-smoke-02`. The V4 launcher accepts only smoke and refuses an existing destination; earlier smoke outputs are preserved. The old V2 scripts and held cohort inputs retain prior method hashes and are historical preparation records, not current launch instructions. ROOT must record the successor release; the launcher itself does not replace that instruction-asserted authorization boundary.

## Limits and handoff

Pending: ROOT release of one both-size smoke requalification using V4; reconciliation of process/test/teardown outcomes; fresh final cohort launch/lease only after required smoke success; single timed attempt, offline observations, report and final evidence/registered checks. No smoke, cohort, metric, native-resource, usability, lifecycle or release acceptance is claimed here. Original D70 targets, failure retention, no timed replacement/retry, settled/resource obligations and independent holds remain unchanged. Any changed reviewed bytes require affected re-review.

TASK `/root/d70_review`, parent `/root`, configured `gpt-6-astra/low` by dispatch; independent model introspection unavailable. Same software-code-review skill and original read-only role/tools apply. Used Git read-only diff/show/rev-parse, cat/rg/nl, and Python standard-library JSON/hash/gzip/structural inspection. No source edits, tests/builds/browser/server/runtime, Git mutations, delegation or sibling communication. Only these two SMOKE_REPAIR review outputs were written. Scope/non-delegation remain instruction/config asserted on an unrestricted host. This derivative review consumes the frozen preparation and accepted D70 basis; ROOT owns fan-in and governed closure. F-PIP-2 / DEC-081 claims fence applies.
