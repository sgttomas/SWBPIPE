# NATIVE launch brief amendment — synthetic fixture provenance label

**Status:** `SEALED_FIXTURE_INPUT_CHANGE_NEW_ATTEMPT_HELD`
**Amends:** `LAUNCH_BRIEF_V1.md`, SHA-256 `7d3aaeeade894dd662d3ca793984b1d25a34754a6ec329d52294d3367a510bb2`
**Preserves:** `LAUNCH_BRIEF_AMENDMENT_V1.md`, SHA-256 `afc30a71ec2d39eca7b5868f4e706fa761c622bf5973f2b0794d5fa5e0c1b61b`, and `LAUNCH_BRIEF_AMENDMENT_V2.md`, SHA-256 `6d68a99e9f07be570854df84e30f5031ef2b5ffdaf4a96fd2566a9c79a3fafa4`

For all eight synthetic entity provenance inputs only, replace the planned value `synthetic_ui_acceptance_input` with the truthful value `invented_synthetic_ui_acceptance_input`. The revised fixture label satisfies the existing backend invented-or-cleared provenance classifier. Entity IDs, quantities, units, steps, the dedicated store, and the load-case Queue workaround remain unchanged.

Root's completed U7 full-path triage found that active node Add/Apply calls `buildNodeCreationSubmission`, which uses `draft.provenance.trim()`, and that the backend resolver preserves the submitted value. The legacy `buildExplicitNodeIntent` hardcode has zero callers. The confirmed native blocker was the fixture label; the earlier source-defect inference is withdrawn. No source defect, source change, R4 repair, review reopening, backend-policy change, API decision, engineering threshold, or acceptance change is authorized.

The failed native attempt remains immutable evidence. The exact bundle, source bindings, and frozen source-review PASS records remain valid. A new GUI attempt is held pending the failed-attempt packet, quit and cleanup evidence, and final verification that the same bundle remains bound and the exact dedicated store is absent. This amendment does not release a launch or claim native acceptance.
