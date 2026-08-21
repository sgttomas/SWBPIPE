# N1 P2 engine repair return

- RunID: `HELP-HUMAN-PIPING-20260821-VOCABULARY-R3`
- InstanceID: `WORKING-ITEMS-VOCAB-R3-N1-ENGINE`
- Role: `WORKING_ITEMS` (Agent 1)
- Package / deliverable: `PKG-16` / `DEL-16-01`
- Objective: repair only R2 integrated-review finding P2
- Accepted main basis: `66efaf6b95605ef69f3e405b505f48506d3cbada`
- Pre-repair lineage commit: `2bee267300e571e4e8686f73aba6ad4ba8be4c54`
- Branch observed: `codex/piping-vocabulary-r3-20260821`
- Runtime attribution: inherited GPT-5.6 Codex runtime per plan v1; no substitution recorded
- Terminal manager verdict: `ACCEPT_P2_REPAIR_FOR_N1_INTEGRATED_REVIEW`

## Preserved failure history and lineage

Round 2's fresh integrated review, `HELP-HUMAN-PIPING-20260821-VOCABULARY-R2/N1_INTEGRATED_REVIEW.md`, returned `FAIL — two actionable findings`. Its P2 finding established that the UI rejected equal tee header/branch references while the authoritative operation applier accepted and persisted them. The failed review and its hash inventory remain unchanged.

The original operation-applier content SHA-256 frozen by that review was:

`847bca37ad991b24a2e7dbd6315c9294a0ac305f427b78628a36b3b1e177b51d`

That exact hash was re-observed before this repair. The pre-repair lineage is committed at `2bee267300e571e4e8686f73aba6ad4ba8be4c54`; this instance did not modify Git state.

## Repair

Changed production path:

- `projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs`

The tee resolver now compares the trimmed `branch_header_pipe_ref` and `branch_branch_pipe_ref` after required-field validation and before pipe lookup. Equal values:

- set reference validation to `blocked`;
- emit blocking diagnostic `OP-COMPONENT-CONNECTIVITY-INVALID`;
- direct the caller to select different incident spans for the header and branch roles;
- return no canonical component and therefore no `applied_model`.

The regression `tee_creation_blocks_equal_header_and_branch_pipe_refs_without_applied_model` proves the diagnostic code, blocked reference-validation state, blocked application state, and absent applied model. No bend, rigid-component, UI, status, coverage, receipt, shared-handoff, or Git behavior was changed.

Post-repair production content SHA-256:

`f6ecdc476482d8798591689770c4a87a318ec4765e0362b886ae46922b58be7f`

The P2-only working diff against pre-repair lineage is `+35/-0`; its binary diff SHA-256 is `4aa5147ba9e947a0a324a4b8481262a38531458fa8e2aef4ae4161453a26512d`.

## Validation

Detailed commands and normalized results are in `N1_ENGINE_CHECKS.json`.

- Focused creation slice: `7 passed`, `0 failed`; the new equal-ref regression passed.
- Full operation-applier suite: `85 passed`, `0 failed` (`82` unit, `1` canonical-hash parity, `2` contract-corpus).
- Wasm32 release build with `wasm` feature: passed.
- Rust formatting check: passed.
- Scoped Git diff whitespace check: passed.

## Containment, blockers, and residuals

- Allowed-write containment: passed. Production writes are confined to the operation-applier file; evidence writes are confined to these two instance-local records.
- P2 blocker: `NONE`.
- P2 residual: `NONE`.
- N1 residual: sibling P1 UI repair must complete, the five-file lineage must fan in, affected UI/browser/build checks must pass, and a fresh read-only reviewer must inspect 100% of the complete post-repair diff with refreshed hashes.
- This return does not accept N1 as a whole, close row 15, transition a deliverable, or release N2.
- Requested Agent 0 action: accept this bounded P2 repair into N1 fan-in and include the post-repair operation-applier bytes in the fresh integrated review.
