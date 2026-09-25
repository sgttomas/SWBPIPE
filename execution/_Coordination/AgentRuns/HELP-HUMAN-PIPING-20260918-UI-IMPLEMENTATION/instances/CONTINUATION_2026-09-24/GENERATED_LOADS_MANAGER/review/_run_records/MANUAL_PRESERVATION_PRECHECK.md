# Manual-preservation source precheck

Independent follow-up TASK from `/root/authoring_manager`; no implementation edits or product executions. The earlier seven-mass reference return is unchanged. This checks the current pre-freeze source against INTEGRATION_CONTRACT and STABLE_MASS_SUCCESSOR. Source snapshots and hashes are in [manual_precheck/MANIFEST.json](manual_precheck/MANIFEST.json). Locations below refer to those snapshots, before pending stable-helper integration.

## Confirmed findings

### P2 — A valid retarget cannot be detached on apply

`core/model_operations/operation_applier/src/rich_authoring.rs:1018–1020` compares historical `prior.pipe_id` with the current primitive's `/target/pipe` before selecting the new record's method. Take a valid generated primitive originally on pipe A, keep its ID and historical generation provenance, edit only its target to an existing pipe B, then request explicit `manual_overrides:"preserve"`. With A still present and valid, inspection sees the payload change and the facade produces a provenance-only detach draft, but apply rejects it as mismatched generated pipe identity. This prevents the contract's explicit preservation of an already modified physical field.

Move the original-pipe/current-payload binding guard into the managed GENERATED_V2-refresh branch. For MANUAL_WEIGHT, independently validate historical generation lineage, enforce exact old/current physical-payload preservation in the proposed operation, and validate the current primitive's target and compatible quantity. Do not require its current target to equal the historical generation source. Keep existing count/order/ID, stale-before, source-hash, unchanged-record, and atomic-write guards. This is a restriction on *replacement* of the current manual payload, not an instruction to restore its original target.

### P2 — Current-source generation runs before a known modified-payload decision

`core/product_physics/src/self_weight.rs:749–760` calls `generate_operations` with the historical pipe ID before computing `payload_changed` or legacy uncertainty. Generation requires exactly one current pipe at lines 222–225. Starting from the valid retarget above, remove original pipe A while leaving current target B valid and all ordinary model references consistent. Inspection fails original-pipe resolution, leaves state Invalid, and `self_weight_wasm/src/lib.rs:150–159` blocks the entire preservation plan. The original historical source is already available in the retained provenance; it does not need to exist in today's model for a fixed manually owned load on B.

Validate the self-contained historical record first, then compare current payload with the historical original and determine exact legacy uncertainty. If Modified, return that classification without regenerating from current historical-source entities; any invalid current manual primitive must still be rejected by ordinary primitive/application validation. Current-source regeneration and dependency comparison belong to managed records whose original payload is intact and which can actually be refreshed. An unmodified primitive still targeting a deleted pipe remains invalid; this repair does not allow dangling current targets or malformed historical lineage.

The contract sentence that invalid/unresolvable source blocks refresh must be applied to the source required for managed recomputation. It cannot also demand a deleted historical generation source for an otherwise valid explicitly preserved current manual primitive while promising preservation of retargeted physical fields. State this branch distinction in the contract repair to keep the behavior reviewable.

### P2 — Retained mill deduction is validated with the wrong dimension

`core/model_operations/operation_applier/src/rich_authoring.rs:1086` assigns `Dimension::Dimensionless` to `mass_inputs.mill_tolerance`. The generation/retained-basis normalization in `self_weight.rs:310–312` and `:568–570` uses `Dimension::Length`, consistent with the approved absolute wall deduction. A valid retained value such as `{value:0.001,unit:"m"}` reaches `quantity`/`unit_by_symbol` at `rich_authoring.rs:52–66` and is rejected in the dimensionless catalog. Both explicit managed refresh and manual detach invoke this validator before their branch completes.

Use `Dimension::Length` for that retained mass input. This is a direct unit-validation correction; no tolerance, physical mill interpretation, or M30 capability changes are needed. The currently read tests use null mill deduction and therefore do not cover this trigger.

## Non-findings and precise boundaries

Changing a valid global direction from `global_y` to `global_z` is not blocked by a historical gravity-axis equality guard in the currently read applier. At line 1027 the current direction only needs to belong to the global-axis enum. With an otherwise valid original source, inspection classifies it Modified, the facade changes only provenance, and the applier's exact physical comparison permits preserving the new direction. This is source tracing, not a runtime pass. Include that case as a regression witness because the narrower retarget restriction could otherwise be generalized incorrectly.

The applier's unconditional category/dimension checks at lines 1023–1028 also limit the detach route to distributed force. The requested concrete retarget and direction cases remain distributed-force loads and do not require expanding this category boundary. If the contract's “every physical field” is intended to include an already valid conversion to another supported primitive category, the manual branch must use the ordinary category/target/dimension rules instead of generated-only constraints. Do not silently claim that broader behavior from the present implementation. No new load category is proposed in this precheck.

The current facade parses the original provenance string into a JSON object at `self_weight_wasm/src/lib.rs:140–146`, then stores that object under `original_generation_provenance`; the applier requires semantic object equality at `rich_authoring.rs:1046`, and the inspector's manual branch reads an object at `self_weight.rs:737`. That does preserve the parsed historical record, but it cannot preserve arbitrary original JSON whitespace/key order/numeric spelling verbatim. For the newly required v1 method-refresh `legacy_generation_provenance` string, carry the original string directly and parse a separate copy for checking. If raw preservation is also required for manual detach, update all three manual-record seams consistently; do not stringify the parsed object and call it the original bytes. The stable v1→v2 refresh implementation is still pending, so this is an integration requirement, not a finding against a completed migration.

## Required correction backchecks

The manager should exercise: valid direction change; valid retarget while both pipes exist; valid retarget after deletion of the original pipe; a nonzero length-valued mill deduction; rejection of a dangling *current* target, malformed historical record, stale source hash, or simultaneous physical mutation in a detach operation. Preserve every physical field and unrelated record through apply/save/reopen; solving the detached record must use its current target/direction and emit the manual warning. Run the same meaningful cases on final stable-v2 and actual retained v1 where exact provenance permits classification. No test was run or added by this reviewer.

Conclusion: the current pre-freeze manual-preservation source has three actionable blockers. Full candidate review and correction backcheck remain outstanding. The independently verified mass-reference packet remains suitable for the stable-method work.
