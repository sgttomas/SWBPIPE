# WORKING_ITEMS Run — R7 packaged edited-load proof

| Field | Value |
|---|---|
| RunID | `HELP-HUMAN-PIPING-20260820-R7-ENGINEERING` |
| InstanceID | `WI-PKG07-DEL0706` |
| Package / deliverable | `PKG-07` / `DEL-07-06` |
| Basis | branch `codex/piping-r7-engineering-20260820`; base `8eea5d06d3f98dd91b56b53a2c7caec2f7ed5919`; Receipt-117; DAG-009; R5 |
| Objective | Run a packaged-Tauri smoke over a saved project with edited load data; repair an in-scope defect if exposed |
| Outcome | `PRODUCT_PROOF_PASS / PACKAGED_GUI_BLOCKED_HOST_AUTOMATION_UNAVAILABLE` |

## Product behavior delivered

The packaged executable now provides the deterministic developer self-test
`--self-test-saved-edited-load`. It executes without the GUI and without a
repository/current-working-directory dependency:

1. embeds the invented public preview fixture;
2. applies the structured `load:L-100/load:L-100-Y` edit `350 N -> 425 N`;
3. writes the edited document to a unique isolated file-backed SQLite store;
4. closes the writer connection and reopens through a second connection;
5. verifies the restored magnitude and unit are exactly `425 N`;
6. solves the restored project to `MECHANICS_SOLVED`, requires non-empty
   result rows, and requires the edited displacement to differ from baseline;
7. emits JSON evidence and removes the temporary store.

This is a real actual-bundle executable surface, not a test-harness-only call.
It materially closes the saved/edit/persist/restore/solve engineering gap, but
it is not substituted for packaged GUI-control observation.

## Host and package evidence

- Initial package build before the product change: PASS; executable SHA-256
  `7dd5d3367433ff9640e56b764eb8d5f822c2f20d0826c27eda69b5c609647d3c`.
- `open` launched the packaged `.app`; macOS listed
  `org.openpipestress.technical-preview` as running.
- Computer Use state inspection attempt 1 hung and returned no accessibility
  tree or screenshot; attempt 2 did the same and was stopped.
- Exact read-only fallback:
  `osascript -e 'tell application "System Events"' ... UI elements of front window ...`
  returned no UI state and was terminated after 20 seconds. This is recorded
  as `BLOCKED_HOST_AUTOMATION_UNAVAILABLE`, not a product failure or pass.
- Fresh post-change package build: PASS. Bundle executable SHA-256
  `28e2effdc2203437f0fb7ef02339f78a2f6e1ad1ccf775bb0053edba858669ca`.
- Actual bundle command:
  `OpenPipeStress Technical Preview.app/Contents/MacOS/openpipestress-desktop --self-test-saved-edited-load`.
- Actual bundle result: PASS; structured output recorded `before=350`,
  `after=425`, `unit=N`, `application_status=applied_to_session_model`,
  `professional_approval=false`, file-backed SQLite schema `11`,
  `persisted_with_ledger_record`, writer connection closed before restore,
  restored `425 N`, `MECHANICS_SOLVED`, model ref
  `project:packaged-edited-load-smoke`, `830` result rows, and changed result
  relative to baseline.
- Integrated review attempt 1 returned P2: the self-test recorded but did not
  gate restored/solved identity. Remediation now rejects a restored SQLite row
  ID, restored model project ID, or solved `model_ref` that differs from
  `project:packaged-edited-load-smoke`; the focused test asserts the solve
  model ref. Focused/full/package checks and the actual bundle invocation were
  rerun green after remediation. Fresh review attempt 2 consumes FROZEN_NODE_DIFF_V2.
- Fresh review attempt 2 returned PASS with no actionable findings after
  verifying all 18 frozen hashes, 100% of the complete remediated node snapshot,
  bundle SHA-256, containment, registered checks, P2 guards, and claim discipline.

## Checks

| Check | Result |
|---|---|
| Focused Vitest `projectService.test.ts` + `App.test.tsx` | PASS, 68/68 |
| Existing Rust `saved_edited_load_model_round_trips_and_solves_from_restored_payload` | PASS, 1/1 before and after change |
| New Rust `packaged_binary_self_test_persists_restores_and_solves_edited_load` | PASS, 1/1 |
| `cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml --check` | PASS after formatting |
| Registered `desktop-test` (`npm run test:desktop`) | PASS, 29 files / 526 tests |
| Registered `desktop-build` (`npm run build:desktop`) | PASS; existing Vite chunk-size warning only |
| Fresh Tauri `.app` build and actual bundle self-test | PASS |
| Registered `harness-self-check` | PASS execution; pre-existing repository findings reported |
| Registered harness pytest | PASS, 350 tests |

The first direct harness attempt used the Xcode Python 3.9 interpreter and
failed operationally because PyYAML was absent. The accepted rerun used the
available Python 3.13 runtime through `uv` with the pinned
`requirements-dev.txt`; both harness checks passed. This was a runtime repair,
not product remediation.

## Scope, review, and residuals

- Product writes are limited to `apps/desktop/src-tauri/src/lib.rs` and
  `apps/desktop/src-tauri/src/main.rs` plus this node's authorized evidence
  and deliverable-local status/memory surfaces.
- No path under `apps/desktop/src/**` changed, but Agent 0 strengthened the
  closeout gate to require a fresh read-only Agent 2 `software-code-review`
  over 100% of the complete frozen node diff, including the Rust and integrated
  evidence changes. Its persisted PASS is mandatory for publishable return.
- The `_STATUS.md` packaged-Tauri GUI smoke bullet remains unchanged. The two
  PDU-045/PDU-046 usability/measurable-target holds remain unchanged.
- Rerun requirement: on a macOS host with working Accessibility control, drive
  the packaged GUI edit/save/quit/relaunch/reopen/solve journey and only then
  consider removing the exact GUI residual.
- No lifecycle, target-stage, release, issuance, professional-reliance,
  accessibility-conformance, certification, sealing, authentication, or
  code-compliance effect is claimed.
