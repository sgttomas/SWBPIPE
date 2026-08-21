# N1 engine return

- RunID: `HELP-HUMAN-PIPING-20260821-VOCABULARY-R2`
- InstanceID: `WORKING-ITEMS-VOCAB-R2-N1-ENGINE`
- Role: `WORKING_ITEMS` (Agent 1)
- Package / selected deliverable: `PKG-16` / `DEL-16-01`
- Accepted basis: plan v1 at base `7d5a3f558dfa2e8e902df25fc9a3e813a9ab7048`; proven bend seam `b988d9d0e4a7048ac28a73bbe53ce045c631dff8`
- Branch observed: `codex/piping-vocabulary-r2-20260821`
- Runtime attribution: inherited GPT-5.6 Codex runtime per the frozen plan; no substitution recorded
- Manager verdict: `ACCEPT_FOR_N1_FANIN_PENDING_REQUIRED_FRESH_READ_ONLY_REVIEW`

## Coverage and outputs

Changed production path:

- `projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs`

The operation-applier resolver now accepts the four row-15 residual kinds in the owner-required order and preserves the same structured-only operation route, aggregate envelope-unit contract, target-id uniqueness check, explicit node reference, diff/apply path, claimed-model-hash gate, and immutable-input behavior already proven for bend.

Per-kind behavior:

1. `tee`
   - Requires explicit `branch_header_pipe_ref` and `branch_branch_pipe_ref`; both pipes must exist and be incident to the explicit component node.
   - Requires positive unit-bearing `branch_run_size`, `branch_header_size`, and `branch_connection_angle`, validated as length, length, and angle.
   - Requires non-empty `branch_connection_type`, `branch_reinforcement_reference`, and `branch_geometry_source_reference`.
   - Persists the trimmed references/text and entered quantity values/units without geometry inference or conversion.
2. `reducer`
   - Uses the accepted rigid-component geometry contract: explicit incident `rigid_pipe_ref`; positive body length, end sizes, and weight; finite COG vector; explicit end, stiffness-behavior, and source references.
   - Validates scalar and vector units as length/length/length/force/length and preserves entered values/units.
3. `valve`
   - Same accepted rigid-component geometry and connectivity contract as reducer; persists kind `valve`.
4. `flange`
   - Same accepted rigid-component geometry and connectivity contract as reducer; persists kind `flange`.

No modifier values, geometry, connectivity, dimensions, units, weights, COG coordinates, references, or provenance are defaulted or inferred by the resolver. The UI payload observed during coordination matches these keys and value shapes. The UI additionally prevents a tee from selecting the same pipe twice; the engine does not add that stricter rule because the accepted `product_physics` branch mapping contract requires each reference to exist and be incident but does not establish a distinct-reference rule.

## Validation evidence

Detailed normalized results are in `N1_ENGINE_CHECKS.json`.

- Focused creation tests: `6 passed`; includes bend regression, tee apply/block behavior, and reducer/valve/flange apply behavior.
- Full operation-applier suite: `84 passed` total (`81` unit, `1` canonical-hash parity, `2` contract-corpus); `0 failed`.
- Wasm engine release compile: passed for `wasm32-unknown-unknown` with feature `wasm`.
- Formatting and diff hygiene: `cargo fmt --check` passed; scoped `git diff --check` passed.
- Production-file SHA-256 after validation: `847bca37ad991b24a2e7dbd6315c9294a0ac305f427b78628a36b3b1e177b51d`.

An exploratory `cargo clippy --all-targets -- -D warnings` was not a clean gate because five findings already exist on unchanged lines: three `question_mark` findings in the node-creation resolver, `too_many_arguments` on the existing test helper, and `approx_constant` on the pre-existing bend `1.5708` test value. None was introduced by this diff, and none was repaired because that would expand beyond the sealed objective. The required tests, format check, diff check, and Wasm compile all passed.

## Write containment and coordination

- Production writes remained inside the operation-applier crate.
- Instance evidence writes are only this return and `N1_ENGINE_CHECKS.json`.
- No UI, deliverable status, vocabulary coverage, receipt, shared handoff, export, or Git state was written by this instance.
- The exact payload contract and validation status were relayed upward for parent-mediated UI coordination.
- Project doctrine requires a fresh read-only review over 100% of this `core/**` diff before publication. That review is intentionally left to the supervising fan-in because this sealed brief did not authorize an additional child-launch record.

## Blockers, residuals, and next owner

- Engine-scope blocker: `NONE`.
- Engine-scope residual kinds: `NONE` — tee, reducer, valve, and flange all have resolver and focused test evidence.
- N1 product-node residual: accept the PKG-07 UI return, run integrated/focused checks, obtain a fresh read-only review over 100% of the frozen N1 diff, and fan in once.
- Row 15 and deliverable-state updates remain owned by `HELP_HUMAN` at fan-in; this return makes no closure or lifecycle transition.
- N2 expansion-joint creation remains excluded and sequenced after accepted-and-committed N1.
- Requested Agent 0 action: accept this engine return for N1 fan-in, include the production diff in fresh review, and preserve the exact no-engine-residual statement if integrated checks pass.
