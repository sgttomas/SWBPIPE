# TASK — public-API runtime tests for the connected load/reference-state route

Parent of record: the session-2 WORKING_ITEMS load-state manager (SendMessage id
`a3675abb28ada0834`). ROOT spawns this TASK on the manager's request; report to
the manager by SendMessage and send your final report to ROOT. Do not delegate
further. Paths are WORKING_ROOT-relative (`WORKING_ROOT =
<checkout>/projects/chirality-piping`; the checkout is the manager's load-state worktree,
branch `codex/piping-load-states-20260925`).

## Objective

Author an independent integration test suite that drives the real product
through the public `open_pipe_stress_product_physics::run_linear_static_preview_value_with_mode`
in both `PreviewSolverMode::SparseInteractive` and `PreviewSolverMode::DenseScrutiny`.
It checks observed product values against the maintained analytical references
in `core/product_physics/tests/fixtures/load_reference_states/reference_cases.json`,
reading expected values from that file at test time; do not copy constants.
You are not the implementer. The implementation manager is writing the facade
concurrently; your tests are its independent acceptance oracle.

## Exclusive write boundary

- `core/product_physics/tests/load_reference_state_runtime.rs` (new; sole writer).
- Optional new helper fixtures under `core/product_physics/tests/fixtures/load_reference_states/models/`
  (new files only; do not modify `reference_cases.json`).
- Evidence under `LOAD_STATE_IMPLEMENTATION/CP2_RUNTIME_TESTS/` (RETURN.md and
  `_run_records/`). Machine-specific paths only inside `_run_records/`.

No other writes: no `src/**`, no existing test, no fixture, schema, UI or Git.

## Inputs to read

- Root `AGENTS.md`, `projects/chirality-piping/AGENTS.md`, `loop/LOOP_INIT.md`,
  and the full TASK role supplied by ROOT.
- `LOAD_STATE_IMPLEMENTATION/CP2_WIRE.md`: the frozen wire shape. Build every
  0.4.0 request exactly to it.
- Reviewed design, read-only via `git show 9e8a55daecdeb9669131fd3e53c0e0303ee550d6:projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/CORRECTNESS_DESIGN/LOAD_REFERENCE_STATES/VERIFICATION.md`
  (and INTERFACE.md). Do not redo reference searches.
- Existing public-API test style: `core/product_physics/tests/pressure_runtime.rs`,
  `support_reactions_runtime.rs`, and fixture `tests/fixtures/exact_pressure_connected_request.json`.
  These show 0.3.0 exact-profile request shape, result row kinds/IDs and sign
  conventions.

## Required coverage (annular companions; OD 0.20 m / wall 0.01 m)

1. Two-bar prescribed translation (root UX = +0.1 mm, far UX = 0): middle UX,
   member axial forces, and root/far `support_reaction_component_v2` Fx.
   Include explicit discriminators. The observed root Fx must differ from the
   reduced-system-only value `K_cf·u_f − f_c` (which omits `K_cc·g_c`) and from
   the omitted-coupling result (middle UX = 0).
2. All-fixed single bar with root UX prescribed: zero free axial DOF behaviour
   and nonzero reactions. If the product blocks this, record the actual
   diagnostic; do not weaken the expectation.
3. Prescribed rotation, all-fixed (root RZ = 1e-3 rad) and free-tip rigid
   rotation (tip UY = L·θ, tip RZ = θ). Include the length-normalized-angle
   discriminator.
4. Shared material, serial companion. One material ID with two dated points
   (E 200 GPa/ν 0.3 and E 100 GPa/ν 0.25). Each member selects a different
   point; member 2 carries explicit interval strain 0.001. Check the middle UX
   and the forces, and that the material-ID-only wrong value is not produced.
5. Thermal datum ratio: engineering-secant table law, datum 20 °C, installation
   50 °C, operating 150 °C. Check the fixed-end wall N and a free-tip UX
   companion. Also check the constant-alpha interval route.
6. Signed fit: cold, hot and return fixed-end forces, released tip UX, a
   no-fit baseline and a cut-long case; include the fixture's wrong-result
   discriminators. Hot uses its own E via an exact point at its temperature;
   return reproduces cold exactly.
7. Persistent source once: an axial cantilever where +100/−1000/+200 N are
   nodal primitives listed in `load_sources` and a stored +999 N primitive is
   unreferenced. Check tip UX and root Fx for −700 N, not −600 N and not
   −700+999. Duplicate `source_ref` must block (`LOAD_STATE_SOURCE_DUPLICATE`),
   and two distinct equal-valued sources must both apply.
8. Negative contract controls from the fixture's `negative_contract_controls`
   that are reachable at the public boundary. At minimum:
   - rotation with a length unit and translation with an angle unit;
   - unknown coefficient definition (typed-boundary rejection: the function
     returns `Err`);
   - fit length change and fit strain on the same member (the union makes this
     a single choice, so test that `kind` is required);
   - legacy thermal primitive referenced together with a resolved thermal state;
   - a 0.3.0 document carrying `analysis_state` (must block, not be ignored);
   - `boundary_motion` on an unrestrained DOF;
   - a missing element state;
   - `free_length_state` without operating temperature.

   Assert the blocking/typed outcome and the diagnostic code where CP2_WIRE.md
   names one. Otherwise assert that it blocks and record the observed code.

Use the protected relative 1e-9 criterion with the existing zero-reference
handling. Do not invent a looser tolerance. Exact zeros can use an absolute
floor scaled by the case's force/displacement magnitude times 1e-9; state this
in the test. Also assert that `producer.semantic_contract_id` is
`openpipestress.result_semantics/0.3.0/load-reference-1` and
`formulation_basis.profile_id` is `resolved_straight_load_state_v1`. For each
case, assert a `contract_evidence.load_reference_states` record exists with the
case ID, and that no `source_block_recovery` is published.

## Build and resources

Coordinate builds with the manager. Until the manager messages that the facade
compiles, write against CP2_WIRE.md and do only syntax checks (rustfmt).
After that, run only your test binary:

`cd core/product_physics && CARGO_TARGET_DIR=<this TASK's own target dir outside the repository> cargo +1.97.1 test --locked --offline -j 1 --test load_reference_state_runtime`

Use `-j 1`, your own target directory and no other Cargo invocations. No
browser, native or UI runs, and no Git operations. If an expectation fails,
report the observed value, expected value and case to the manager. Never adjust
an expectation toward an observed value, and never change a tolerance.

## Return

SendMessage the manager an early message listing your planned test names and
the request-builder API. Then return: the file path and SHA-256, test list,
commands run with raw logs under `_run_records/`, observed pass/fail with
original failure text, and any wire ambiguity found. Write RETURN.md in this
directory.

_Portable publication of the brief. The as-issued bytes that ROOT verified (sha256 `4cf1ae3d8f2dceffefaefde5ee489cda9bf53423d0bb4b47c49ae514669c9b77`), including machine-local paths, are kept under the manager's `_run_records/session2/issued_briefs/`._
