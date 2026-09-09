# U7 browser-flow maintenance return

Status: PASS; source frozen for root-routed independent review.

B1 updated only `projects/chirality-piping/apps/desktop/e2e/r2-smoke.spec.ts` from sealed SHA-256 `be0650f640986a9282db43803107439f0c08d71a7c37639508e3758fa6c13248` to `da2efafca0594a417cfd3c03aa99c143325048e6419a7d3a1c4ff4434a5b82b6`. Both affected journeys now follow Add -> frozen review -> Apply for viewport node and pipe creation, while legacy forms keep their queued-operation assertions with correct independent queue and applied-receipt counters.

Focused Playwright validation passed 4/4 in `chromium-desktop` and `chromium-compact`. B1's return contains the complete coverage/counter map. The 17-member live binding and both logical diffs are frozen in the authorized U7 and DEL-07-01 evidence scopes; `RAW_DIFF_RESOLVER_V1.json` maps lossless original archives to candidate-clean active pointers and LF-clean record successors. No production behavior, API, fixture, physics, or other test changed.

Next gate: fresh root-routed review of the frozen one-file change and 17-member binding. Native/build/integration remain outside U7 ownership.
