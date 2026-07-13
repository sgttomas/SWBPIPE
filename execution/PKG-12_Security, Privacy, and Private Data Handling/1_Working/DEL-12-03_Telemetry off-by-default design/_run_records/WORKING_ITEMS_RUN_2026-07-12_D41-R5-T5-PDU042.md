# WORKING_ITEMS Run — D-41 R5 T5 PDU-042

- **Scope:** existing DEL-12-03 telemetry panel only.
- **Interaction:** initial render records no request; `Request telemetry enablement review` is a dedicated affirmative action distinct from terms acceptance, installation, application/project open, and solve.
- **Fail-closed result:** the click records a local request but leaves telemetry disabled because opt-in/consent and allowlist approval remain absent. No product config mutation, payload construction, persistence, endpoint, vendor, queue/job, client, or network initialization occurs.
- **Validation:** telemetry service Vitest 3/3; focused App interaction Vitest 1/1 selected (56 skipped); disposable copy-out production build passed with the existing large-chunk warning; scoped `git diff --check`.
- **Residuals:** actual consent/opt-in UI or CLI, approved allowlist, product config schema/storage, runtime enablement, transport, and PDU-043 consumer interception remain unimplemented and separately gated.
- **Boundaries:** no new GUI scope, runtime telemetry, validation promotion, formal review closure, lifecycle, dependency/DAG/register/decomposition, or `ISSUED` change.
