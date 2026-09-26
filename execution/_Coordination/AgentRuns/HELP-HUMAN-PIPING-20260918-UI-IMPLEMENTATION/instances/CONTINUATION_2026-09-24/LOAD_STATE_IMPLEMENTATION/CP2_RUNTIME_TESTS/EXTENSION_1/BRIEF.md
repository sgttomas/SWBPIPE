# TASK extension 1: independent public-route acceptance coverage for review N-4

**Parent of record.** The session-2 WORKING_ITEMS load-state manager (SendMessage id `a3675abb28ada0834`). ROOT resumes the runtime-test TASK `acfae78c40c4525d2` with this brief.

- Report to the manager by SendMessage, and send the final report to ROOT.
- Do not delegate further.

**Paths.** All paths are WORKING_ROOT-relative. `WORKING_ROOT` is `<checkout>/projects/chirality-piping`, and the checkout is the manager's load-state worktree on branch `codex/piping-load-states-20260925`.

**Your role.** You wrote the checkpoint-2 suite (`core/product_physics/tests/load_reference_state_runtime.rs`, 20 tests). You are not the implementer. The implementer's in-module tests do not count toward this acceptance coverage.

## Why

The independent checkpoint-2 review (`LOAD_STATE_IMPLEMENTATION/REVIEW_CHECKPOINT_2/RETURN.md`, finding N-4) found behaviour that only implementer tests, or no test at all, pin. ROOT dispositioned the public-route acceptance coverage to you.

## Exclusive write boundary

- **New file** `core/product_physics/tests/load_reference_state_runtime_extension.rs`. You are its sole writer.
- **Your checkpoint-2 file is read-only.** Leave `load_reference_state_runtime.rs` byte-for-byte unchanged. Its 20 tests stay the protected CP2 acceptance baseline.
- **Helper models.** You may add new files under `core/product_physics/tests/fixtures/load_reference_states/models/`. Do not modify existing files, and do not modify `reference_cases.json`.
- **Evidence.** Write under `LOAD_STATE_IMPLEMENTATION/CP2_RUNTIME_TESTS/EXTENSION_1/`: `RETURN.md` and `_run_records/`. Machine-specific paths go only inside `_run_records/`.

Make no other writes: no `src/**`, no other test, fixture, schema, UI or Git.

## Inputs to read

**Wire.**
- `CP2_WIRE.md`
- `CP2_WIRE_ADDENDUM_1.md`
- `CP2_WIRE_ADDENDUM_2.md`, sha256 `ec66628ef8db1ac70500b80abbcc754b82133fb8d97d4cf0f41ecd4a32ab9133` (new; it records the checkpoint-3 typed-boundary, null and joined-route facts)

Build every request exactly to these.

**Design.** Read it read-only with `git show 9e8a55daecdeb9669131fd3e53c0e0303ee550d6:projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/CORRECTNESS_DESIGN/LOAD_REFERENCE_STATES/{VERIFICATION,INTERFACE,DESIGN}.md`.

**The maintained reference fixture** `core/product_physics/tests/fixtures/load_reference_states/reference_cases.json`.
- Read its expected values at test time. Do not copy constants.
- Its `temperature_unit_identity` case and its `thermal_datum_ratio.verification_two_point` variant currently have no consumer.

**The review record**, for the probe and mutant descriptions it names: `REVIEW_CHECKPOINT_2/RETURN.md` and `REVIEW_CHECKPOINT_2/_run_records/`.
- You may read the reviewer's probe sources to understand a scenario.
- Derive every expectation independently, from the fixture or from your own stated derivation.

## Required coverage

Drive the real product through the public `open_pipe_stress_product_physics::run_linear_static_preview_value_with_mode`. Run every value check in both `SparseInteractive` and `DenseScrutiny`.

1. **Nonzero pressure with per-member E/ν.** Author a closed-region, nonzero-pressure, two-case check. The two cases select different member E/ν, and one case also carries an eigenstrain.
   - Assert the published wall action and pressure evidence (`pressure[].materials`, `exact_cases.pipe_materials`) for each case. Use the case's own resolved pair, not the base material or another case's pair.
   - The kill targets are the reviewer's M06, M19 and M23. Give the wall-action relation from your own derivation, and name the design clause it comes from.

2. **Temperature identity.** Consume the fixture's `temperature_unit_identity`.
   - **Each identity group:** −50 °C ≡ 223.15 K; 242 °C ≡ 467.6 °F ≡ 515.15 K; 20 °C ≡ 527.67 °R.
     - Show that equal authored temperatures select the same material point, through an exact-point selection authored in another unit.
     - Show that they produce identical normalized mechanics.
   - **The non-equal control.** Assert that it is not treated as equal. That means an interpolation fraction strictly greater than 0 with two consumed points, or the fixture's stated refusal. Also assert that an exact-point selection at that temperature requires the override (`LOAD_STATE_MATERIAL_TEMPERATURE_OVERRIDE_REQUIRED`).
   - **Endpoint snapping (M24).** Include −49.999999 °C against a 223.15 K point. This fails under a relative 1e-8 snap.

3. **`thermal_datum_ratio.verification_two_point`.**
   - Use the VERIFICATION control-4 exact inputs:
     - an engineering-secant two-point table: 12e-6/K at 50 °C and 16e-6/K at 150 °C;
     - datum 20 °C, with no point at the datum;
     - installation 50 °C and operating 150 °C.
   - The expected strain is 43/25009, with the fixture's companion values.
   - This is admissible only because coverage is required over [installation, operating]. Include a variant outside that interval, which must block.

4. **`LOAD_STATE_TEMPERATURE_IDENTITY_UNRESOLVED` through the public route.** Use temperatures whose exact classes are ordered but whose binary64 representatives are not, for example 467.6 °F against 515.1500000000001 K. Assert the targeted blocking code and that no envelope results are published.

5. **Duplicate-class material points (M14).** Author one material with two temperature points at the same exact temperature in different units. Assert the targeted refusal. `LOAD_STATE_MATERIAL_TEMPERATURE_AMBIGUOUS` or `LOAD_STATE_MATERIAL_POINT_AMBIGUOUS` is expected; record the observed code if it differs, and do not weaken the test.

6. **Dilation consulted versus consumed (M17).** Author an `engineering_dilation` table law whose datum is a table point with zero dilation. Assert that `consulted_law_point_indices` contains the datum-zero point and `consumed_law_point_indices` does not. The consumed data must be exactly the points that entered the value. Assert the segment `use` labels too.

7. **Joined-route acceptance.** ROOT decides whether this item stays in scope; include it unless ROOT strikes it. Use `fixtures/product_preview/load_reference_source/eigen_motion.request.json` (sha256 `58ce5b95d185d56964a30962b355c14595cdf831505603c6e5d455425c4ea8e9`), read-only; it is a producer witness, all invented.
   - **Closed form.** The inputs are:
     - E = 200 GPa;
     - OD 0.2 m and wall 0.01 m;
     - L = 2 m;
     - thermal 6e-5 and fit 4e-5, so eps* = (1+6e-5)(1+4e-5) − 1;
     - tip stop UX = 5e-5 m;
     - root UY = 1 mm and root RZ = 1e-4 rad;
     - torque 1e-8 N·m on a 1e-4 N·m/rad torsion spring.

     Derive from these independently:
     - tip UX, UY and RZ;
     - root RX;
     - member axial force N = E·A·(δ/L − eps*);
     - the root and tip `support_reaction_component_v2` Fx, in the support-on-pipe convention;
     - zero root shear and moment.
   - **Publication.** Assert:
     - `producer.semantic_contract_id` is `openpipestress.result_semantics/0.3.0/load-reference-source-1` and `formulation_basis.profile_id` is `resolved_straight_load_state_source_v1`;
     - the receipt policy is `LOAD-REFERENCE-SOURCE-1` and its status is `qualified`;
     - the case's `source_recovery` is `{status: "selected", method: "retained_source_blocks_exact_v1"}`;
     - no NOT_JOINED diagnostic is published.
   - **Companion.** Add an unavailable-join companion by adding a nonzero pressure region to that request. Assert that it stays `load-reference-1`, with `not_joined`, one NOT_JOINED diagnostic and an info `SOURCE_BLOCK_RECOVERY_UNAVAILABLE`.

**Criterion.** Use the protected relative 1e-9 criterion, with the existing zero handling: an absolute floor equal to the case magnitude × 1e-9, stated in the test. Invent no looser tolerance. Never move an expectation toward an observed value.

## Build and resources

```
cd core/product_physics && CARGO_TARGET_DIR=<your own target dir outside the repository> cargo +1.97.1 test --locked --offline -j 1 --test load_reference_state_runtime_extension
```

- Also run your checkpoint-2 binary once (`--test load_reference_state_runtime`) to confirm it still passes unchanged.
- Use `-j 1`, and no other Cargo invocations.
- Disk is limited: delete your target when done.
- No browser, native, UI or npm runs, and no Git operations.

**Coordinate with the manager.** The implementation is mid-checkpoint. If an expectation fails, report the case, the observed value, the expected value and your derivation to the manager.

## Return

1. SendMessage the manager an early message with the planned test names.
2. Then return:
   - the file path(s) and sha256;
   - the test list;
   - commands with raw logs under `_run_records/`;
   - pass/fail with the original failure text;
   - the mutant each test targets;
   - any wire ambiguity.

Write `RETURN.md` in this directory.
