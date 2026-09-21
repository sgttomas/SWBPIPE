# P2 eligibility/sort separation

Production consumer inventory is limited to `MaterialTable` in ModelTree.tsx and the table-local projection helper. No EngineeringTable, tableState, style, service, VirtualList, engine/controller or Node rendering code changes.

The hook returns separate `sortValues` and `eligibleCells`. ModelTree uses only `sortValues` for physical ordering, and only `eligibleCells` plus existing base readonly checks for direct availability. It does not buffer or replay keys.

## Admission

An eligibility proof is admitted only by an owned conversion response with a well-formed complete identity envelope, that cell's status converted, finite result, and exact canonical target unit. Its source must also be a conversion candidate without readonly/unavailable metadata. No unit is inferred or newly admitted. Existing direct source-number/type/actual-unit guards remain.

Proof scope binds exact generation and family/policy. Per-cell basis binds typed row identity, column identity, dimension, raw displayed value, actual unit, canonical type/value/draft basis, convertible state, readonly/unavailable state and canonical target. ModelTree explicitly distinguishes signed zero in the canonical basis. A numeric source and a malformed numeric-looking string cannot share eligibility.

## Pending and revocation

Render derives current eligible keys by exact comparison with the last committed proof state; it never mutates a cache/ref. Unchanged verified cells retain eligibility during another cell's refresh. Changed cells are immediately excluded from returned eligibility without waiting for effects.

At commit, pruning retains only matching proofs for current cells/current scope. Changed/removed cells and changed generation/policy are evicted. Returning later to an old basis does not revive an evicted proof. State holds at most one current-cell proof map, not a history. Unmount releases the hook's state. Previous sort-result data is discarded when refreshing, including across project replacement.

A settled owned unavailable cell response revokes that cell's proof; malformed/rejected whole responses revoke all proofs. That is intentionally distinct from pending work. Stale completion effects are cancelled and cannot admit, revoke or reorder current cells.

## Sorting

The full current plan owns sort keys independently. A new plan immediately returns no prior physical keys, including changed-then-restored byte sequences. Only a current owned response publishes keys; existing whole-displayed-column completeness and input-order/unavailable behavior remain in EngineeringTable. Per-cell eligibility is never a fallback source of sort values.

## Regression and reversal

Actual fail-before: real Elastic Apply+Tab then keyboard8 into unchanged Shear stayed on readonly button with no editor. Separate unchanged-open-editor unit Apply called no route. After repair, both browser profiles enter8 then87 without focus/fill rescue, preserve pending-sort input order, cancel Shear and undo exactly one Elastic checkpoint. Own-cell value/type/unit/generation cases stay blocked or cancel their old editor; hook tests add dimension/typed identity/field/family/policy/availability/removal revocation, stale completions, failed/malformed settled results and non-revival after restoration.

Reversal is the two production-file delta relative to daf8d966. It would restore the documented lost-first-key/readonly-editor defect; no canonical model/history migration or new unit support is involved.
