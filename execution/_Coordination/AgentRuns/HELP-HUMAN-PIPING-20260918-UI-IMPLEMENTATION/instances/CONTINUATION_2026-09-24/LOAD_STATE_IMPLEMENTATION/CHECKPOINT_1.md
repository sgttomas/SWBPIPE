# Checkpoint 1 — private case-state kernels compiled, tested and reference fixture rechecked

Session-2 WORKING_ITEMS manager, dispatched by ROOT into the load-state checkout
(`codex/piping-load-states-20260925` at `f2112be`, base `c278f64`). Paths are
WORKING_ROOT-relative. Machine-specific records are under
`_run_records/session2/` (STARTUP, MECHANISM, CHECKS_CHECKPOINT_1 and raw logs).
This is completed execution of checkpoint 1, not acceptance, and it closes no
M10/M16/M29 finding.

## Result

- `core/product_physics/src/case_state/mod.rs` (new, manager-owned) declares the
  private `material` and `thermal` kernels. `lib.rs` gains a four-line
  `#[allow(dead_code)] mod case_state;` declaration until facade integration.
- `case_state/material.rs` (session-1 TASK bytes, unchanged) compiled on first
  build and its ten prepared tests pass.
- `case_state/thermal.rs`: the interrupted session-1 kernel (lines 1-689,
  unchanged) now has eight manager-authored in-module tests covering the
  43/25009 datum control and its two wrong controls, K/degC-normalized
  equivalence, constant-alpha interval without absolute temperature, datum-length
  integral 0.0015 versus logarithmic exp(0.0015)-1 versus wrong 0.002, reversal
  as ratio inverse (-3/2003), split composition, exact endpoints, dilation-table
  datum rule, signed fit cold +40000 N / hot -89976 N / return without
  accumulation / cut-long / doubled / sign / strain-form alternative, small-interval
  direct evaluation (1e-12 relative), and explicit coverage, duplicate, datum,
  nonfinite, negative-kelvin, interior stationary nonpositive-stretch (datum
  integral and secant), dilation table-point and fit/length range refusals.
  Numerical assertions keep the protected relative 1e-9 criterion.
- `tests/fixtures/load_reference_states/reference_cases.json` (session-1 TASK
  bytes, unchanged) was recomputed by an independent standard-library script
  written without reading or importing its generator: 188 exact-rational and
  binary64-projection checks pass; five single-value mutations are all detected.

| File | SHA-256 | Author |
|---|---|---|
| `core/product_physics/src/case_state/mod.rs` | `0e52838884b2cb4ccc0df98c4a5c1df829110cb29697588e0c8294fa89b57198` | manager |
| `core/product_physics/src/case_state/material.rs` | `1ae8ac998fce43bcdb2f2b1fdb8b0f03569fbe688fa28f420d5f578d37c239bd` | session-1 TASK, unchanged |
| `core/product_physics/src/case_state/thermal.rs` | `4dd284d16f346ab82c2e7e1e207a00f30b6348c398804cae27a89f8efa191b1b` | kernel session-1 TASK (unchanged); tests manager |
| `core/product_physics/src/lib.rs` | `5a4d70153cc53d3d6ee97770f87990df6e2b39314d28496db69d8f33598bc28e` | +4 lines manager |
| `core/product_physics/tests/fixtures/load_reference_states/reference_cases.json` | `5478bba846bcc515187f38366eeda3ac9fb87b339039a5cf3ec2d56dced4b891` | session-1 TASK, unchanged |

Evidence files (session-2 `_run_records`) carry their own hashes in the return.

## Checks

All Cargo runs used `cargo +1.97.1 … --locked --offline -j 2` with a per-manifest
target outside the repository, after one `cargo fetch --locked`.

- `test --lib --no-run`: pass (first attempt failed before fetch; failure text retained).
- `test --lib case_state`: 18 passed. The first run of the new thermal tests had
  one failure caused by a wrong test expectation (decimal -0.85 instead of the
  correctly rounded exact binary64 sum); the kernel was not changed. Original
  failure text is retained in `CHECKS_CHECKPOINT_1.json`.
- `test --lib` (whole product_physics lib): 286 passed, 1 pre-existing ignored.
- rustfmt on the new files only; kernel bytes unchanged.
- Integration tests, pytest, evidence sweep, desktop and native were not run;
  nothing they exercise has changed yet.

## Review status

- Host delegation: no in-process subagent tool is available here, so no TASK
  was dispatched and all work was sequential (`MECHANISM.json`).
- Non-author checks by this manager: the fixture recomputation above, and a read
  review of `material.rs` and the thermal kernel body. No defect was found.
  Recorded observations for integration: the selector requires the normalized
  Poisson unit `1` (call it after model normalization, which maps `none`);
  secant tables must cover the datum temperature, which is conservative and
  matches the fixture's datum-coverage control; `consumed_point_indices` include
  points consulted by the path-positivity check, so evidence must label them as
  consulted; the 4096-point table bound is a resource limit, not an engineering
  one.
- Required: a fresh-context non-author reviewer for the manager-authored bytes
  (`mod.rs`, thermal tests, lib.rs declaration). A TASK-level recheck of the
  fixture and kernels is also needed if ROOT wants review independent of this
  manager. The reviewer should also check that the thermal tests are
  meaningful, not only that they pass.

## Remaining work and next checkpoint

Checkpoint 2 joins one resolved case to the actual assembly, recovery and
evidence path in `product_physics`, as the reviewed design requires. The
intended engineering route below is ordinary integration choice; ROOT may object
before it lands:

1. Model `0.4.0` reuses the exact straight annulus route unchanged
   (`pressure_contract` 2.0.0/`exact_straight_pressure_v2`, E/nu-derived G,
   signed v2 rows). Every load case must carry `analysis_state.contract =
   openpipestress.load_reference_state/1.0.0`. Formulation profile is
   `resolved_straight_load_state_v1`; producer semantics are
   `openpipestress.result_semantics/0.3.0/load-reference-1`; evidence is added
   under `contract_evidence.load_reference_states` next to the existing
   `pressure`/`exact_cases`. Documents 0.1-0.3 that carry any new field block;
   they are never silently ignored. Current serde would drop an unknown
   `analysis_state` in old documents.
2. Closed deny-unknown DTOs for `analysis_state`, model `reference_configurations`
   and material `expansion_laws`, mapped to the private kernels after unit
   normalization. There is exactly one resolved record per active element; no
   array-order precedence.
3. Per-member E/nu/G: `build_model` receives per-pipe resolved constitutive
   values keyed by pipe ID, with no synthesized material records. The exact
   pressure pipe state, stiffness, recovery and `exact_cases.pipe_materials`
   evidence all read the same resolved pair.
4. M10: `support_states[].boundary_motion` on declared rigid DOFs compiles to
   `reduce_system_with_prescribed_displacements`. Complete `u` is
   reconstructed, and reactions come from the unreduced `K u - f` (the existing
   formula is valid once `u` holds the prescribed values). Motion on
   non-restrained DOFs, dimension mismatch, duplicate claims and legacy
   `imposed_displacement` primitives block.
5. M16/M29: the resolved total eigenstrain becomes the existing
   `ThermalElementLoad` (axial E·A·ε*), assembled once and removed once in
   recovery. Legacy thermal primitives block in the new profile.
6. Source ledger: `load_sources` is the complete inclusion list for ordinary
   primitives. Duplicate IDs block, unreferenced stored primitives are excluded
   and reported, and eigen/boundary contributions are echoed with their owner
   and classification.
7. Retained-source recovery is not attempted for `0.4.0` cases in checkpoint 2.
   Each such case gets an explicit `LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED`
   record, and no old eligibility is inherited. The verified join is
   checkpoint-3 work and remains required.
8. Integration tests drive the public `run_linear_static_preview_value_with_mode`
   in both solver modes. They cover VERIFICATION controls 1-4 and 6 (annular
   companions, serial shared-material companion) and 9's source ledger, and
   compare observed product values with `reference_cases.json`. Existing tests
   and hashes stay unchanged.

Checkpoint 3 then covers the retained-source join, the result-semantics table
and hash for `load-reference-1`, the result_export/analysis_runs/desktop
readers, schemas, authoring/operations, persistence and native witnesses.
Hydrostatic head/contents weight, per-case mass/support selection, nonlinear or
installation history, bends/joints, sparse/general coupling, shear and wider
reporting stay open and are not claimed.

## Decisions requested

None blocking. ROOT is asked to (a) commit/push these bytes, (b) arrange the
non-author reviewer above, and (c) note that without a delegation tool this
manager will continue sequentially unless ROOT supplies worker sessions that
share this checkout.
