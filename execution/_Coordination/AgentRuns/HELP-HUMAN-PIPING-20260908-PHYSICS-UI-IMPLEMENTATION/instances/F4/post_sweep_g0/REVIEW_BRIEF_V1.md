# RF post-sweep G0 candidate review brief

Status: `SEALED_FOR_ROOT_APPROVAL_AND_DISPATCH`

## Identity and execution boundary

- Target: `/root/friction_code_review`
- Role: independent read-only Agent 2 reviewer
- Model and effort: `gpt-5.6-sol`, `high`
- Delegation: prohibited
- Reporting: return only through `/root`; no direct sibling messages
- Source, tests, builds, Git state, lifecycle state, and accepted evidence are read-only.
- The only allowed reviewer outputs are under `{RUN_ROOT}/instances/RF/post_sweep_g0/**`.

## Exact inputs

| Input | SHA-256 |
| --- | --- |
| `{RUN_ROOT}/instances/F4/post_sweep_g0/SEALED_DIAGNOSIS_SCOPE_V1.md` | `3c45244661582ba5976cd46280b53420a7cb2040ce7fe04d8339f1a2953b9a5c` |
| `{DEL0404_RUN}/post_sweep_g0/DIAGNOSIS_V1.md` | `bd3a0ee21221eafd966bc5654d912f3f5c018e6608f3202d3269fbe700748a82` |
| `{DEL0404_RUN}/post_sweep_g0/PROPOSED_PRODUCT_TEST_ONLY.patch` | `30c4b8203a43d7b9e2979730dd5b80fa0994a86346ad43e34859a0aabf5ba9f5` |
| `projects/chirality-piping/core/product_physics/src/lib.rs` | `e757b8a51e2c4ae68ac4d6c37620663bf4d698ff03b8d6349b40b484bb591903` |
| `projects/chirality-piping/core/solver/nonlinear_integration/src/lib.rs` | `6e163a47db60288179f844d473992d28a53599ad2204149093186c760723ad46` |
| `projects/chirality-piping/fixtures/product_preview/invented_preview_model.json` | `986c055944776ca8d0d849d552678e1f2bfb6c4559bbb9ff8a1928157a4f871c` |
| `execution/_Change/PHYSICS_UI_IMPLEMENTATION_20260908/_run_records/DEC025_G0_FAILURE_V1.json` | `b350be906fe94575a28d35e4f498b0a931c74da36831946c9490133817335ffc` |
| `validation/evidence/sweeps/SWEEP_20260908T202715Z_7b73460c5e2d.json` | `0d6f31872b51d3c1021d574d42d127aabaf0b400fa06690720672dfb5625268e` |

`{RUN_ROOT}` is `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION`. `{DEL0404_RUN}` is the DEL-04-04 `_run_records/PHYSICS_UI_IMPLEMENTATION_20260908` directory.

## Review objective

Independently determine whether the clean G0 failure is a stale test oracle caused by the accepted current-normal coupling or a production regression. Do not accept the observed result as the derivation. Verify from the fixture coefficient, the separately asserted current normal, and the product publication rule that `round6(0.01 * 48.952652) = 0.489527`; determine whether `0.490101` instead encodes the superseded prior normal. Confirm the proposed patch changes only the affected test block, leaves production behavior untouched, preserves all valid loop/state/sign/normal/metadata/diagnostic assertions, and adds a relation assertion at the public rounding boundary without introducing a threshold.

Inventory any other active assertion that requires amendment and distinguish immutable historical evidence. Assess whether the proposed post-authorization checks are sufficient: affected test covering both modes, formatting, full offline locked product-physics crate, exact test-only diff/source freeze, fresh independent review, CHANGE integration, and a clean five-surface DEC-025 rerun.

## Required return

Write a concise `REVIEW.md`, `RETURN.md`, and manifest under `{RUN_ROOT}/instances/RF/post_sweep_g0/**`. Give `PASS` or `CHANGES_REQUIRED`, the independent numeric derivation, exact patch/scope assessment, production-byte assessment, assertion inventory, acceptance-gate assessment, and any remaining Owner dependency. Do not approve source execution, claim lifecycle closure, change criteria, or make release decisions.

A `PASS` requires all of the following:

1. `0.489527` follows from `mu=0.01`, current normal `48.952652`, and the existing six-decimal publication boundary rather than from matching the failed observed value.
2. `0.490101` is shown to correspond to the previous-normal behavior.
3. The candidate is exactly test-only and production bytes remain unchanged.
4. No additional active expectation needs correction, or each additional one is precisely identified.
5. A new bounded Owner act remains required before applying the product-physics test patch.
