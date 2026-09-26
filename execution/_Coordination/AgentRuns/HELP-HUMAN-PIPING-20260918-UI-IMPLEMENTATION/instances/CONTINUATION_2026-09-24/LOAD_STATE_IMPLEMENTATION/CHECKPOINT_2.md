# Checkpoint 2 — resolved case joined to assembly, recovery and evidence

Session-2 WORKING_ITEMS manager. The checkout is branch
`codex/piping-load-states-20260925` (WIP freeze `3293c319a`, then the bytes
below). Paths are WORKING_ROOT-relative, and machine records are in
`_run_records/session2/`. This is completed execution of checkpoint 2, not
acceptance. No M10, M16 or M29 finding closes until connected
authoring/readers/persistence, the retained-source join and native witnesses
land (checkpoint 3). The candidate still needs its fresh-context independent
review on a frozen commit.

## What now works through the product route

A model document `0.4.0` on the unchanged exact straight-pressure route
resolves one case per load case through `case_state::resolve`. That resolved
case is the only input to stiffness, pressure recovery, eigenload, boundary
values, the source ledger and the published evidence. The concrete wire is in
`CP2_WIRE.md` (frozen, `81a7adba…`) plus `CP2_WIRE_ADDENDUM_1.md`.

- **M16, per-member material.** Each pipe selects its own E/ν from its actual
  material. The options are fixed base with an applicability reference, an
  exact point, or piecewise-linear interpolation with no extrapolation. G is
  derived from the selected pair. `build_model_for_members` and
  `build_pressure_case_with_members` consume the pairs keyed by pipe ID. Two
  members sharing one material ID solve with different E/ν, and no material
  record is synthesized. Actual, selected, installation and datum temperatures
  stay distinct. A known actual/selected mismatch needs an explicit override.
- **M16/M29, thermal and fit.** Explicit definitions resolve to thermal and
  fit strains, composed as `ε* = λ_fit·λ_th − 1`. The definitions are
  unchanged reference, explicit interval strain, constant-α interval, and a
  free-length law (secant constant/table, dilation table, per-datum-length or
  logarithmic coefficient table). ε* enters once as the axial eigenload
  `E_member·A_s·ε*` through the existing `ThermalElementLoad`, which recovery
  removes once. Legacy thermal primitives cannot also be consumed.
- **M10, prescribed boundary.** `support_states[].boundary_motion` on DOFs the
  support restrains rigidly becomes actual prescribed tuples:
  - the structural solve applies K_fc·g_c against the original f;
  - `reduce_system_with_prescribed_displacements` builds the reduced system;
  - the complete u includes g;
  - reactions come from the unreduced `K·u − f`.

  The legacy DEC050/053 observation lanes use a coupled force, so they observe
  the same reduced system. All-prescribed, zero-free-DOF systems solve with
  nonzero reactions. Motion on an unrestrained or spring DOF, a dimension
  mismatch or a duplicate DOF blocks.
- **Physical-source inclusion ledger.** `analysis_state.load_sources` is the
  complete inclusion list for the case's stored primitives, with explicit
  finite nonzero factors. A duplicate ID blocks. Unreferenced primitives are
  excluded and reported. Eigenstrain, prescribed-boundary and pressure-region
  contributions are echoed with their owner and classification.
- **Evidence and identities.** `contract_evidence.load_reference_states[]`
  carries each case's complete resolved record (addendum §3). The producer is
  `openpipestress.result_semantics/0.3.0/load-reference-1` and the profile is
  `resolved_straight_load_state_v1`. `exact_cases.pipe_materials` and the
  pressure-region material evidence come from the same resolved pairs. Raw and
  producer carriers stay at 0.2, and existing meanings and hashes are
  unchanged.
- **Retained-source recovery.** It is not attempted for 0.4.0. Each case
  records `LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED` (info) and
  `source_recovery: not_joined`. No source-case finalization or receipt is
  produced, so no old method eligibility is inherited. The verified join is
  still required (checkpoint 3).
- **Old documents.** A 0.1–0.3 document carrying `reference_configurations`,
  material `expansion_laws` or `analysis_state` blocks
  (`LOAD_STATE_CONTRACT_VERSION_MISMATCH`). Unknown fields or discriminants in
  the new namespace are rejected at the typed boundary.

## Checkpoint-1 review repairs (ROOT dispositions)

| Item | Repair |
|---|---|
| SF1 | Three-segment datum-length, logarithmic and dilation tests with interior install/operate temperatures, reversal, split composition and labelled segment coverage. The targets come from the manager's own exact derivation (`sf1_multisegment_derivation.py`), which matches the reviewer's. The last-segment-only mutants for the integral and the dilation difference are now killed. |
| SF2 (departure from VERIFICATION control 4, now removed) | A secant table needs coverage and positivity only over [min(T_install,T), max(T_install,T)], because λ(T_m)=1 by definition. The test uses control 4's exact two-point table. Policy-pin tests refuse uncovered evaluation temperatures and accept positivity outside the evaluated interval. The restored-datum-coverage mutant is killed. |
| SF3 | New `case_state/temperature.rs`. Each consumed absolute temperature gets exact rational kelvin from its shortest round-trip decimal and the exact unit definition (K; °C+273.15; (°F+459.67)·5/9; °R·5/9). Exactly equal temperatures share one binary64 representative, which feeds both selector and kernel. A set whose representatives would lose the exact order is refused (`LOAD_STATE_TEMPERATURE_IDENTITY_UNRESOLVED`); there is no tolerance snapping. The reviewer's regressions (−50 °C/223.15 K, 242 °C/467.6 °F, 467.6 °F/515.15 K) pass through the public route. Disabling canonicalization is killed. |
| N1 | Kernel output splits `consumed_*` (entered the value) from `consulted_*` (coverage or positivity only). Segments are labelled `interpolation_sample` or `integration_interval`, and the evidence publishes both sets. |
| N2 | Vacuous assertions were removed or renamed; corrections are below. |
| N3 | Sort-order control added (authored order 120/20/70 °C, request 95 °C). |
| N4/N5 | The module-wide `dead_code` allow was removed and the module doc now matches the integrated role. |
| N6 | Delegated: fixture README plus additive cases. Done; see Delegation below. |
| N7 | °F coefficient and absolute-temperature adapters are covered by runtime tests 6 and 8. Unknown definitions and malformed units are rejected. There is one actual operating temperature: the thermal kernel receives exactly the selector's resolved actual T. |

Mutation evidence is in `_run_records/session2/cp2_repair_mutations.{py,log}`.
All four mutants are killed, and the bytes were restored and hash-verified.

## Corrections to CHECKPOINT_1.md

These are recorded here rather than by rewriting checkpoint 1.

- "K/degC-normalized equivalence" was overstated. That test compared
  bit-identical kelvin inputs. Unit equivalence belongs to the adapter and is
  now tested there (SF3).
- The fit test's "doubled", "wrong" and "return without accumulation" checks
  could not fail once its main assertion passed; the pure kernel holds no
  state. No-accumulation is a solver-level property, now checked by runtime
  test 8 (return equals cold bit for bit across independent cases).
- The secant datum-coverage rule was called "conservative" but was not
  recorded as a departure from VERIFICATION control 4. It was a departure and
  has been removed (SF2).
- "No defect was found" in the read review did not cover missing tests for
  multi-segment integration (SF1) or the cross-unit temperature identity gap
  (SF3). Both are now repaired.

## Delegation

TASKs were spawned by ROOT on this manager's request, reported to this
manager by SendMessage, and did not delegate further (`MECHANISM.json`).

- **Runtime tests** (`acfae78c40c4525d2`): 20 public-API tests
  (`tests/load_reference_state_runtime.rs`, sha256 `d5be0bd8…`).
  - Written by a non-implementer against the frozen wire and the fixture.
    Every test runs in both solver modes.
  - Includes ROOT's two added negative controls: double source inclusion, and
    a prescribed-DOF reaction computed from the reduced rather than the
    unreduced system.
  - Also covers the SF2/SF3 behaviour variants.
  - 20/20 on the TASK's pinned run and on the manager's freeze-gate run. The
    TASK found no failures, and no expectations were changed.
- **Fixture extension** (`a3bb458a695b524aa`, complete; `ANALYTICAL_REFERENCE/CP2_EXTENSION/RETURN.md`):
  - Additions to `reference_cases.json` (`5478bba8…` → `4d7b7777…`):
    `multi_segment_free_length` (coefficient and dilation variants, forward,
    reverse and split, with first- and last-segment-only discriminators),
    `thermal_datum_ratio.verification_two_point` (control 4 exact, with the
    ROOT admissibility policy) and `temperature_unit_identity`.
  - The change is additive: all 2656 pre-existing paths are preserved, with
    1360 lines added and 0 deleted. Its own checker ran 1002 checks, with 10/10
    mutations detected.
  - The fixture README is new (`fe8149c0…`), closing review N6.
  - Its values were derived independently of the kernels and of the manager's
    derivation, and they agree with both.
  - Carry-forward:
    - degR acceptance is asserted only at the identity-module level
      (20 °C ≡ 527.67 °R), not through the public route;
    - the multi-segment case now has a product-route mechanics check (above).

## Checks

The raw record is `_run_records/session2/CHECKS_CHECKPOINT_2.json`.

- **Baseline** at `be1b9294a`: 348 passed, 1 pre-existing ignored.
- **Freeze gate** (the bytes of `3293c319a`), whole crate:
  - lib 297 (268 pre-existing non-case_state tests unchanged, plus 29
    case_state);
  - runtime 20;
  - other integration 62 unchanged;
  - sources hash-stable during the run.
- **Added after freeze:**
  - `pressure_recovery_uses_each_cases_resolved_pair_and_eigenstrain_once`:
    two cases in one model, closed-region pressure plus per-case E/ν and
    eigenstrain, analytical N_wall = 2ν·p·A_i − E·A_s·ε*, both modes.
  - `product_route_reproduces_single_and_multi_segment_free_length_references`:
    the product free-tip route reproduces the new multi-segment references
    (datum, log and dilation, forward and reverse) and the single-segment
    control-5 references.
  - Both pass.
- **Gate 2** (current bytes on the extended fixture `4d7b7777`), whole crate:
  - lib 299 (268 pre-existing non-case_state tests unchanged, plus 31
    case_state), 1 pre-existing ignored;
  - runtime 20 (still passes with the new fixture variants present);
  - other integration 62 unchanged;
  - sources hash-stable during the run.

  The manager's independent fixture checker gives 200 checks, 0 failures.
- **Dependent crates** compile with tests (`--no-run`): core/runner/headless,
  core/model_operations/operation_applier, core/loads/self_weight_wasm and
  validation/benchmarks/physics_audit_regression.
  - The first attempts of the last two failed with ENOSPC (environmental; raw
    text retained). They passed on rerun.
  - validation/benchmarks/numerical_integrity cannot build with `--locked`.
    This is inherited: a stale leaf lock after product_physics gained the
    canonical_json/sha2 edges. It is fixed upstream in PR905 `5b1ccd356`
    (ROOT) and is not repaired here. This branch changes no manifest or lock.
- **Implementer failures** along the way are kept with their original text in
  CHECKS_CHECKPOINT_2.json:
  - an import error;
  - smoke inputs missing provenance;
  - a real defect: legacy `pipe_materials` evidence dereferenced a base G that
    0.4.0 never resolves;
  - a test ordering bug.
- **Not run:** piping pytest, the evidence sweep, desktop, native and
  browser. ROOT schedules these for the mergeable candidate.

## Review status

The frozen checkpoint-2 candidate needs a fresh-context non-author review
covering `src/lib.rs`, `src/pressure_runtime.rs`, `src/case_state/*` (with a
recheck of the SF1–SF3/N1–N5 repairs) and both TASK deliverables. ROOT
arranges it. The runtime TASK's independent acceptance run should be repeated
on the frozen commit.

## Remaining (checkpoint 3 and beyond)

Required for this capability:

- the verified retained-source/resolved-input join, replacing the not-joined
  guard;
- the `load-reference-1` result-semantics table and hash;
- readers in result_export, analysis_runs and the desktop result
  interpretation;
- model/results schemas for 0.4.0;
- desktop `types.ts`, authoring and operations (review/apply, undo/redo);
- native/headless adapters and persistence (save/reopen, stale flags);
- mapping the legacy `imposed_displacement` support-target primitive into
  `support_states`;
- native witnesses.

Explicitly open and not claimed:

- hydrostatic head, contents weight and per-case mass/support selection (M21);
- spring base motion and device preload/lock states;
- nonlinear or installation history;
- bends and joints; sparse or general coupling; shear;
- combinations and wider reporting.

## Decisions requested

None blocking.
