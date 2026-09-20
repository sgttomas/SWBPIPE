# UI foundation benchmark protocol history

`fixture-manifest-v1-superseded-before-timed-run.json` is the first frozen
fixture/protocol manifest, SHA-256
`285b918088cae236e38d8e9af31945782e14aed26e68440a612aef336ba939c1`.
No timed run used it. It was superseded before baseline execution because its
camera block listed nominal poses without distinguishing them from camera state
that the baseline can actually expose.

The maintained manifest separates the real baseline recipe (visible Iso preset
plus the real canvas orbit gesture) from candidate nominal poses. Baseline
camera state remains unobserved because the product has no read-only camera
state seam. A requestAnimationFrame callback establishes only a paint
opportunity; it does not by itself prove compositor presentation.

`fixture-manifest-v2-superseded-after-baseline-load-filter-orbit.json` is the
second frozen manifest, SHA-256
`df35d5291c0cd39f6cc137c7af7e20758811410e8cf35f7308077aa7e62bc5b6`.
BASELINE_V1 through BASELINE_V3 used it for load, filter, and orbit evidence.
Its viewport-label point-selection proxy failed for one requested sample
because another label covered the requested label's click point. The other
199 point rows were not attempted. No label activation is treated as a canvas
point-pick sample. V2 was superseded prospectively by the decision to use the
real Labels command OFF for fixed canvas probes in both baseline and candidate,
with Labels restored ON for label-budget, visual, and orbit conditions.
The exact V2 sample bytes are retained as
`ui-foundation-1000.interactions-v1-label-proxy.json` (SHA-256
`0a68da5acf202c943ad6441e8731429ed1c156fb0f7cee02493d2f9b6ede3bed`)
and `ui-foundation-10000.interactions-v1-label-proxy.json` (SHA-256
`97ce2688aa79492402664ec091f2da43cb8f696fefc3c64d6b5d7b59690ac3a5`).

`fixture-manifest-v3-superseded-before-canvas-point-timing.json` preserves the
next maintained protocol at SHA-256
`8ca81f73de7267c66b1d2c07778e6e08f71778a151e008edb0c4e248ad234575`.
It froze Labels OFF, the independent V3 oracle, and the baseline frustum
limitations. No timed canvas-point run used it. It was superseded
prospectively to require a real project-tree exclusive-selection reset before
every timed point gesture, outside the point timer, and a new action- and
generation-associated render submission after the gesture. The probe order,
anchor coordinates, camera recipes, point target, and acceptance thresholds did
not change.

`fixture-manifest-v4-superseded-before-candidate-camera-preflight.json`
preserves SHA-256
`d6dcc757d1024da020e4333e1f7f624243920821c04946c418384875573f72bc`.
No timed run used it. It was superseded before candidate preflight to state
that the listed nominal workload poses are reference drafts. They do not
authorize benchmark-only product controls or hidden camera mutation. The
candidate uses only real Fit and standard professional presets, freezes the
actual readback and derived independent oracle in a separate untimed preflight,
and validates that recipe in every measured session. Fixture, visibility,
extents, probes, order, action counts, and targets remain unchanged.

`fixture-manifest-v5-superseded-before-resource-route-calibration.json`
preserves SHA-256
`d086af9e6301ba5d2f55d6363576c46296713a18e19ced3dbc8666d9892309aa`.
No timed run used it. It was superseded to correct one resource-protocol
interpretation: real browser-session Create, Save, and Open snapshot routes
already exist. Baseline generation/resource observability is absent and the
twenty-cycle replacement witness was not measured; the control itself is not
absent. Candidate replacement uses ordinary Open. Browser E2E does not inject
a React unmount bridge; component cleanup tests and native quit/reopen remain
separate evidence classes.

`fixture-manifest-v6-superseded-before-box-policy-binding.json` preserves
SHA-256
`3fe26201623736b7810ce9885a43f1ea0edb0400b0f65b4e104b842e2b5254df`.
No timed run used it. It was superseded to bind the frozen independent
box-selection policy. The existing twenty rectangles, directions, filters,
fixture geometry, cameras, action counts, and acceptance target did not change.

`fixture-manifest-v7-superseded-before-final-control-binding.json` preserves
SHA-256
`962b7970428a08eab1ff25583d85e548147220b9118d931b16d32385f95bb926`.
No timed run used it. It was superseded when the UI manager froze the exact
prospective candidate control, typed-tree-row, feedback-region, and
source-only resource-reader bindings. The fixtures, workloads, action counts,
camera rules, oracle policies, and acceptance targets did not change.
