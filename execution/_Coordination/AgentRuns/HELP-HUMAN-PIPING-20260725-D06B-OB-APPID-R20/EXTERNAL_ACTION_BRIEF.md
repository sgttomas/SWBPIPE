---
doc_id: R20-D06B-APPID-EXTERNAL-ACTION-BRIEF
doc_kind: coordination.external_action_brief
status: EXECUTED_BY_HELP_HUMAN
created: 2026-07-25
run_id: HELP-HUMAN-PIPING-20260725-D06B-OB-APPID-R20
actor: HELP_HUMAN
---

# Bounded external-action brief — one Apple Developer Explicit App ID

This durable brief records the exact bounded scope applied to the separately
owner-authorized Safari action. HELPS_HUMANS did not operate Safari, access the
credentials, or perform the external write.

## Exact action

Using the owner's existing on-machine signed-in credentials/session through
Safari, create one new **Explicit App ID** in the owner's Apple Developer
profile:

| Field | Exact value |
|---|---|
| Description | `OpenPipeStress Technical Preview` |
| Bundle ID | `org.openpipestress.technical-preview` |
| Type | Explicit |
| Capabilities | Add none; do not select or change any capability |

## Stop conditions

Stop without completing the action if the exact identifier already exists; an
identifier or description mismatch appears; the portal requests enrollment,
payment, a new agreement, CAPTCHA, unexpected permission, account recovery, or
credential changes; a capability must be selected; or any act beyond the
single exact registration is required.

Do not expose, copy, log, export, replace, or generate credentials. Do not
create or change certificates, provisioning profiles, or keys. Do not edit
product/configuration/code, documentation/status, build/release state, or Git.
Do not sign, notarize, build, package, publish, or release anything.

## Required result

Return success/failure, date, team identifier if visibly non-secret, whether
the target was initially absent, final exact description/type/bundle ID,
capability state, any disabled portal baseline distinguished from an
operator-selected capability, and whether any stop-condition event occurred.
Do not return secrets.

The HELP_HUMAN-supplied result is preserved separately in
`EXTERNAL_ACTION_RESULT.md`.

