# F4 post-sweep G0 return

Status: `DIAGNOSIS_COMPLETE_CANDIDATE_HELD_PENDING_NEW_OWNER_ACT_AND_REVIEW`

The clean DEC-025 G0 failure is a stale product-physics test expectation caused by the accepted same-iterate current-normal repair. For the fixture coefficient `mu=0.01` and the separately asserted published current normal `48.952652 N`, the existing publication boundary requires `round6(mu*N(current)) = 0.489527 N`. The stale `0.490101 N` expectation corresponds to the recorded prior normal `49.010116 N`.

Only the active UZ friction-reaction literal in `valid_invented_model_exposes_nonlinear_support_loop_evidence` requires correction. The candidate changes that test only, preserves its other assertions, and adds an explicit public-boundary relation assertion. It has passed `git apply --check`; it has not been applied. No source, test, build, Git, lifecycle, or accepted-evidence state was changed during this diagnosis.

## Frozen outputs

| Output | SHA-256 |
| --- | --- |
| `{RUN_ROOT}/instances/F4/post_sweep_g0/SEALED_DIAGNOSIS_SCOPE_V1.md` | `3c45244661582ba5976cd46280b53420a7cb2040ce7fe04d8339f1a2953b9a5c` |
| `{DEL0404_RUN}/post_sweep_g0/DIAGNOSIS_V1.md` | `bd3a0ee21221eafd966bc5654d912f3f5c018e6608f3202d3269fbe700748a82` |
| `{DEL0404_RUN}/post_sweep_g0/PROPOSED_PRODUCT_TEST_ONLY.patch` | `30c4b8203a43d7b9e2979730dd5b80fa0994a86346ad43e34859a0aabf5ba9f5` |
| `{RUN_ROOT}/instances/F4/post_sweep_g0/REVIEW_BRIEF_V1.md` | `2fd51a8973bcfec8b1f6d21cc75e524f4038482d4b6ab6923bad3d0a6a39bdef` |

`{RUN_ROOT}` is `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION`. `{DEL0404_RUN}` is the DEL-04-04 `_run_records/PHYSICS_UI_IMPLEMENTATION_20260908` directory.

## Required next act

Root must obtain a new bounded Owner act before applying the product-physics test-only patch, then dispatch the sealed independent read-only review and execute the focused/full acceptance gates recorded in the diagnosis. Product and nonlinear production behavior remain unchanged by this packet.
