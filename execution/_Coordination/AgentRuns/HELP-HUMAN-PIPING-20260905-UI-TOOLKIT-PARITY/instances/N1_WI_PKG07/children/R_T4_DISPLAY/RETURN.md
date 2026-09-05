# Fresh display module review

RUN_STATUS: SUCCESS (review completed)
Verdict: FINDINGS — not valid for implementation acceptance until remediation and fresh backcheck.
Parent: N1_WI_PKG07. Agent2 ephemeral, no delegation, instruction+config asserted role; inherited model exact identifier unavailable. Reviewed 100% of the three frozen files. No source changes.

## Actionable finding

- **P2 — Expose the existing US volume-per-length catalog conversion.** `apps/desktop/src/features/display-units/targets.ts:20` omits US for `volume_per_length`. Rust `core/units/src/lib.rs:1290` explicitly allows `(Area, VolumePerLength)` and its catalog includes `in^2`; `unit_by_symbol` uses that alias and `convert_for_dimension` validates it. A valid `{value: 1, unit: "m^3/m", dimension_id: "volume_per_length"}` readout under US never reaches Rust, retains SI, and displays the incorrect explanation that no US catalog target exists. Add the accepted `in^2` target, test dispatch/conversion for this dimension, and correct T4 RETURN's catalog-gap claim. This is a supported catalog capability rather than a requested unit-engine extension.

## Reviewed behavior and evidence

All other target symbols and dimension aliases traced against live Rust unit definitions and alias handling. Absolute temperature and temperature interval remain explicit and separate; numeric arithmetic occurs only in the Rust service. Readouts do not receive mutable model/hash/draft handles. Provider preference stays session-local. Source/preference/converter identity, effect cleanup on unmount, and resolved-key checks prevent stale presentation updates. Microtask batching deduplicates outstanding requests without retaining settled quantity data. Missing/duplicate IDs, missing responses, rejected service calls, nonfinite converted numbers, and wrong units fall back. Unknown dimension, string/TBD and nonfinite source values stay entered with an accessible notice. Selector truthfully qualifies conversion availability and editing units.

Read every test: five focused tests cover the described principal component paths; source JSON preservation evidence is appropriately module-level, not proof of whole-product integration. Tests do not cover every malformed-response or unmount branch, but manual trace found no separate actionable defect. Existing focused tests/tsc PASS consumed from T4 RETURN; not rerun during this read-only review. Registered affected checks selected: desktop-build, desktop-test, harness-self-check; full integrated checks remain owed by parent. Scope validator PASS for all three declared source paths. Scope validation was run after initial reads rather than before; no source mutation occurred.

Renderer inventory inspected as a handoff inventory, not accepted as completed integration. Full integrated frozen diff review remains required. No schema, migration, generated artifact or dependency changes in this module. No lifecycle, D58, or decomposition acceptance.

## Frozen SHA-256

- displayUnits.test.tsx: 10ad48d1dcda465299479b4d6580b300063bd6f03c98e96764439b0cc5342a9b
- index.tsx: 147119db58caa69057eecf5cc8a5e2cacba1e934bfa5f407c978071bb94221c9
- targets.ts: 82fc7ad623ba94260e747848b3a5847de94885a1245005dce19183f066c272a0

All matched T4 SOURCE_MANIFEST.json at review.

## Method and handoff

ControlSurface: FILE plus parent dispatch. Scope: PKG07 DEL0702 display presentation. Method: software-code-review v1, profile NONE; BRIEF_SCHEMA.md, TOOL_POLICY.md, QA_CHECKS.md found/read. Used read/search/hash inspection through shell, python3 tools/software_workflow/validate_change_scope.py and python3 tools/software_workflow/select_affected_checks.py. Only writes: own RETURN.md and STATUS.json per explicit launch brief; no fixes or delegation.

Accepted upstream references consumed through T4 frozen return: manager TIER3_INTERFACE_CONTRACT_V1, M_EXISTING001, N2 C_ACCEPTED_SERVICE_WIRE_V1/C_ACCEPTED_SNAPSHOT_V1. This review is a derivative evidence packet, not decomposition truth. Closure: module review complete with one unresolved implementation blocker; parent routes repair, updates frozen manifest, requests backcheck, then performs integrated review/checks. No pointer update authorized or made.
