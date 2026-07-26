---
doc_id: R20-D06B-APPID-EXTERNAL-ACTION-RESULT
doc_kind: coordination.external_action_result
status: SUCCESS_RELAYED_BY_HELP_HUMAN
created: 2026-07-25
run_id: HELP-HUMAN-PIPING-20260725-D06B-OB-APPID-R20
actor: HELP_HUMAN
evidence_posture: EXTERNAL_OBSERVATION_RELAY
---

# External-action result — Apple Developer Explicit App ID

## Verbatim HELP_HUMAN observation

<!-- BEGIN EXTERNAL OBSERVATION VERBATIM R20 -->
SUCCESS on 2026-07-25. Safari was already signed in as Ryan Tufts to Apple Developer team `8A7JL35U4S`. Identifiers list initially showed only `Chirality` / `com.chirality.app`; exact target was absent. Registered an Explicit App ID with description `OpenPipeStress Technical Preview` and bundle ID `org.openpipestress.technical-preview`. Confirmation page showed all selectable capabilities off; portal displayed its disabled baseline In-App Purchase row, which was not selected or changed by the operator. Final identifiers list showed the new exact row. No agreement, payment, CAPTCHA, unexpected permission, certificate, profile, key, or other credential action occurred.
<!-- END EXTERNAL OBSERVATION VERBATIM R20 -->

Canonical exact-text reading: 675 UTF-8 bytes between the markers, excluding
marker lines and adjacent delimiter newlines, with no trailing newline.
SHA-256:
`10cf9545436cab028bf08b171c0887dce9484f18236e57908b6b73f8be32c37b`.

## Final read-only portal confirmation supplied by HELP_HUMAN

- Identifiers-list URL:
  `https://developer.apple.com/account/resources/identifiers/list`
- Team: `8A7JL35U4S`
- Name: `OpenPipeStress Technical Preview`
- Identifier: `org.openpipestress.technical-preview`
- Type: Explicit
- Non-secret portal resource ID: `V49VYB9W92`
- Edit-page condition: exact team prefix, description, and bundle ID shown;
  Save disabled because there were no pending changes.
- Selectable capabilities: `0` / off.
- Portal baseline: `In-App Purchase` alone displayed value `1` but was
  disabled, not selected or changed by the operator, and is not represented as
  an added capability.

## Bounded result

Only the exact App ID registration is recorded complete. No agreement,
payment, enrollment, CAPTCHA, unexpected permission, certificate,
provisioning profile, key, other credential action, product/configuration
change, signing, notarization, build, packaging, publication, or release
occurred.

This is external evidence relayed by HELP_HUMAN. HELPS_HUMANS did not directly
witness or independently query the Apple Developer portal.

