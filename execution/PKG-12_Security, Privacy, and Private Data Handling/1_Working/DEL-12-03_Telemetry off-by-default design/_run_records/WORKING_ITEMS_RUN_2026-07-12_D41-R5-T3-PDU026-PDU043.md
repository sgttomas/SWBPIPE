# WORKING_ITEMS Run — D-41 R5 T3 PDU-026/PDU-043

- **Authority:** DEC-074 O7-before-E5; PDU-026 selected private-by-default telemetry seam.
- **Implemented grain:** desktop telemetry-panel event attempts route through a pure fail-closed pre-payload service; no endpoint, vendor, transport, queue, upload, persistence, or client exists.
- **Negative evidence:** absent/default config, capability request without affirmative opt-in/allowlist, payload-shaped attempt, telemetry-persistence key, and forbidden report field all remain dropped/rejected with payload/network initialization false.
- **PDU-043 hold:** plugin/adapter/import-export/report/private-library runtime interception remains documented absence and is recorded in `_STATUS.md ## Remaining`.
- **Validation:** focused telemetry service Vitest 3/3; focused App workspace packet Vitest 1/1 selected (56 skipped); cache-disabled focused Python security tests 18/18; disposable copy-out desktop production build passed with the existing large-chunk warning.
- **Boundary:** verified-not-security-validated; no whole-product security/privacy sufficiency, review closure, lifecycle, dependency/DAG/register/decomposition, ISSUED, release, or professional claim.
