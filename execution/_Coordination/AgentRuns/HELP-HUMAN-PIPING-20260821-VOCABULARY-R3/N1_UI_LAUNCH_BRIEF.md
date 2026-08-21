# N1 P1 UI repair brief

- Instance: `WORKING-ITEMS-VOCAB-R3-N1-UI`; role `WORKING_ITEMS`; package `PKG-07`; deliverables `DEL-07-01` and `DEL-07-02`.
- Basis: run plan v1; pre-repair `2bee267300e571e4e8686f73aba6ad4ba8be4c54`; R2 review P1.
- Objective: for tee/reducer/valve/flange, initialize and clear required pipe-role selectors after kind selection and node change; disable queueing until deliberate selection; prove negative empty-state behavior and exact user-selected roles with a three-plus-incident-span fixture and rigid exact-span case. Preserve accepted bend behavior.
- Allowed writes: the four pre-repair UI/test paths (`componentIntent.ts`, `PropertyInspector.tsx`, `PipeViewport.tsx`, `App.test.tsx`) and instance-local N1 UI return/check records only.
- Required checks: affected inspector/viewport matrix plus new negative/positive cases, bend regression, browser WASM rebuild using current resolver, desktop build, exact-final registered desktop tests, diff check; record pre/post hashes.
- Prohibited: resolver/status/coverage/receipt/Git writes or inferred engineering defaults.
