# EXECUTE_RETURN — T4 DEL-05-01 Sub-Span (Partial-Extent) Wind Exposure

**Run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14` / W2 / T4
**Role:** T4 executor (governed Agent 2, serialized, non-delegating)
**Brief:** `execution/_Coordination/CANDIDATE_BRIEF_2026-07-20_T4_PKG05_SUBSPAN_WIND.md`
(`CB-2026-07-20-T4-PKG05-SUBSPAN-WIND-001`, EFFECTIVE (v2); both verifier
returns read in full before execution — v1 BLOCK at
`instances/W2/T4/VERIFY_BRIEF.md`, v2 COMMIT-SAFE at
`instances/W2/T4/VERIFY_BRIEF_V2.md`)
**Date:** 2026-07-20
**Base commit:** `581a15b1c718fd444870f13e75fc7cd974518670` on
`claude/piping-r14-pkg05-loads` (freeze-check §4.1 re-verified before any
durable write: `ElementExposedDiameter` with exactly
`element_index`/`exposed_diameter`, whole-element-only
`generate_wind_equivalent_static_loads` with id
`{case_ref}:generated:wind:{axis}:element-{i}`, `prepare_lumped_nodal_loads`
half-total lumping, `WindGenerationInput` with exactly the four fields,
`add_uniform_element_loads` straight `w·L/2` / bend arc-consistent branches,
block-level-only `validation.rs` checks, canonical
`WindEquivalentStaticInput` four-key required set with
`additionalProperties: false`, live `$defs/ElementLoadSpan` fraction
vocabulary, the `tests/test_model_schema.py` wind assertion block at lines
450–458, the sole DEL-05-01 Remaining row byte-matching the brief, no
`equivalent_static` string anywhere under `validation/witness/`
(grep exit 1), and the benchmark count assertion at 23)
**Checkout state at start:** clean apart from the lawful R14 untracked state
(the candidate brief and `instances/W2/**`). No commit, push, branch, pull,
fetch, PR, or merge was performed by this executor.

## OVERALL STATUS: PASS

## Per-Predicate Table (brief §3.1–§3.11)

| Predicate | Status | Evidence |
|---|---|---|
| §3.1 Extent vocabulary exists once, validated | PASS | `core/loads/primitive_loads` gains public `LoadExtent { start_fraction, end_fraction }` (Copy) with validating constructor `LoadExtent::new` (both fractions finite, `0 <= start < end <= 1`; violations return the new Display-only `PrimitiveLoadError::InvalidExtentBounds` — an error variant, not a finding code) plus `has_valid_fractions`/`covered_fraction`/`centroid_fraction` helpers. `ElementExposedDiameter` gains `exposed_extent: Option<LoadExtent>`; `PrimitiveLoad` and `ElementUniformLoadContribution` gain `extent: Option<LoadExtent>`; every existing helper constructor sets `None`; new helper `partial_uniform_element_load` is the only extent-bearing constructor. No `LoadTarget` variant added; no existing helper signature changed. |
| §3.2 Deterministic partial-extent generation | PASS | `generate_wind_equivalent_static_loads`: `None` entries produce byte-identical loads and ids (the emission arm is the unchanged `uniform_element_load` call with the unchanged id format); `Some(extent)` entries produce one load per entry carrying the extent with id `{case_ref}:generated:wind:{axis}:element-{i}:extent-{start}-{end}` (shortest-round-trip f64 Display: stable across reruns, distinct for distinct extents — unit-tested including a rerun-equality assertion). Invalid extents (non-finite, out of `[0,1]`, `start >= end`) are blocking `InvalidGenerationInput` findings — unit-tested for five invalid shapes. NO new `FindingCode` variant; `core/solver/diagnostics` untouched (read-only regression 24 tests pass, mapping still exhaustive). |
| §3.3 Lever-rule static equivalence at the existing tier | PASS | Both seams implement exactly `W = w·(b−a)·L`, `c = (a+b)/2`, `R_i = W·(1−c)`, `R_j = W·c`: `prepare_lumped_nodal_loads` (extent match arm; `None` arm literally `magnitude·span·0.5` — byte-identical) and the `add_uniform_element_loads` straight-span branch (`None` arm literally `magnitude·length/2.0`). The `(0,1)` reduction is exact including in floating point (multiplying by 1.0 and 0.5 is exact) — asserted with `==` against the whole-span output in both the primitive_loads unit test and the benchmark test. No fixed-end moment anywhere. |
| §3.4 Curved-bend macro spans fail closed | PASS | In `add_uniform_element_loads`, an extent-bearing load on a macro-realized span pushes blocking `LOAD_INPUT_INVALID` (`diagnostic:curved-bend:{case}:{load}:partial-extent-load`; message directs whole-span marking or separate pipes) and `continue`s — no chord approximation, no partial-arc invention, no drop. Whole-span wind on macro spans keeps the arc-consistent path byte-unchanged. `curved_bend_uniform_intensity_by_pipe` skips extent-bearing loads, so extents never enter bend intensity maps. Integration test `subspan_wind_on_curved_bend_macro_span_is_blocking` passes (MODEL_INCOMPLETE, empty results). |
| §3.5 Additive fail-closed preview input | PASS | `WindGenerationInput` gains serde-default `exposed_spans: Vec<WindExposedSpanInput>` (`pipe_ref` + dimensionless `start_fraction`/`end_fraction` quantities, DEC-018-normalized exactly like `shape_factor` via `normalize_dimensionless_quantity`; non-dimensionless units produce the existing normalization blocking diagnostics). Marking rules all block via the existing codes: at-least-one-marking-form (`exposed_pipe_refs` OR `exposed_spans`; the one no-marking message now names both forms — same id/code/severity, and the pre-existing `contains("exposed_pipe_refs")` test assertion still passes unchanged); per-span missing fields (MISSING); unknown refs (INVALID); same pipe in both forms (INVALID, "exactly one form"); overlapping extents per pipe (INVALID; interiors-intersect rule, endpoint-touching disjoint extents allowed and tested solving); invalid fractions (INVALID via `LoadExtent::new`). Exposed diameter uses the identical section basis (OD + 2·insulation). Seismic generation untouched. |
| §3.6 Canonical schema + tests | PASS | `WindEquivalentStaticInput`: required relaxed to `pressure`/`shape_factor`/`direction` + `anyOf` over `exposed_element_refs`/`exposed_spans`; additive `exposed_spans` (minItems 1) with inline items reusing `$defs/ElementReference` + `$defs/ElementLoadSpan`; `additionalProperties: false` preserved; no other `$defs` change; every currently-valid document remains valid (all carry `exposed_element_refs`, satisfying that `anyOf` arm — covered by the unchanged passing `test_equivalent_static_generation_inputs_survive_transform_and_validate`). In `tests/test_model_schema.py`, exactly ONE existing assertion (the wind required-set subset assertion, base lines 452–457) was REPLACED by the (i)–(iv) set: (i) three-key subset, (ii) exact `anyOf` shape equality, (iii) `exposed_element_refs` `minItems == 1` retained + `exposed_spans` `minItems == 1` + `ElementLoadSpan`-based item shape, (iv) `additionalProperties` is False. Every other assertion in both Python files is untouched; `tests/test_physical_to_analytical_transform.py` gains two additive tests (spans-only round-trip + rejection cases). |
| §3.7 Derivation witnessed | PASS (written FIRST, before any source edit) | NEW `validation/hand_calcs/mechanics/tp_pmm_p3_subspan_wind_exposure.md`: partial-extent intensity `w = p·G·D_exposed` over `[a,b]`; resultant `W = w·(b−a)·L` and centroid `c = (a+b)/2`; lever-rule derivation with the two-equation uniqueness argument and both moment cross-checks; exact `(0,1)` reduction (analytic and floating-point); disjoint-extent superposition by linearity; curved-bend fail-closed boundary (no partial-arc derivation invented); closed-form fixture values (w = 84 N/m; extent A [0.2,0.7] → 126 N @ 0.45 → 69.3/56.7 N; extent B [0.8,1.0] → 50.4 N @ 0.9 → 5.04/45.36 N; superposed 74.34/102.06 N; whole-span reduction 126 N). Elementary statics and invented geometry only; every closed form completed — the §4.2 STOP condition was not triggered. |
| §3.8 Benchmark evidence | PASS (default fixture path; §3.8 fallback not needed) | New fixture `MECH-TP-PMM-P3-SUBSPAN-WIND-EXPOSURE` (family `EquivalentStaticGeneration` reused; provenance names the new witness) with eight expected values computed from the witness closed forms; inventory registration; count assertion 23→24; two new tests exercising generator + `prepare_equivalent_static_loads` + extent-aware lumping against the closed forms at the 1.0e-9 internal epsilon (recorded DEC-026 analytic-class tier reused; `tolerance_policy: None` everywhere; no policy JSON; no new tolerance constant). ONE additive inventory line in EACH mirror README (`git diff --numstat` = `1 0` for both); readiness mirror test passes (fixture id in both READMEs, witness filename in the hand-calc README). |
| §3.9 Existing behavior preserved | PASS | Whole-span generation/ids/diagnostics byte-identical by construction (`None` arms are the unchanged code); seismic untouched; all pre-existing tests in all touched and regression crates pass unchanged (the only pre-existing-test-affecting text is the no-marking message extension, which the pre-existing assertion still matches); existing occloadgen fixture and every other fixture pass with recorded values untouched (suite diff is additive + count bump + the two forced `exposed_extent: None` literal updates). Integration tests: (a) partial case vs hand-computed lever-rule nodal forces at 1e-9 relative — PASS; (b) mixed whole+partial on distinct pipes superposes — PASS; (c) every §3.5 blocking rule blocks — PASS; (d) whole-span-only preservation — the unchanged `wind_equivalent_static_generation_matches_authored_distributed_load` passes. del1005 five-case byte-identity holds at the implementation head (0/0/0/1/1; SHAs below). `CliOutput` and every serialized runner surface untouched (headless contract guard PASS; runner crate untouched). |
| §3.10 Bounded state update | PASS (on success only) | DEL-05-01 `_STATUS.md`: exactly the sole Remaining row removed (the W1 T3 line-removal strike precedent; the `## Remaining` header retained, matching the repo-wide empty-section convention); one new History entry dated 2026-07-20 citing the brief and recording the GUI-emit follow-on truthfully; `Last Updated` 2026-07-20. One new `MEMORY.md` entry (sub-span design, curved-bend fail-closed boundary, GUI-emit follow-on reported to HELP_HUMAN). NEW `_run_records/WORKING_ITEMS_RUN_2026-07-20_R14_W2_T4_SUBSPAN_WIND.md` modeled on the TP-PMM-P3-OCCLOADGEN-001 record. No other deliverable state touched. |
| §3.11 Checks | PASS | Full §6 sequence executed in order; tally below. |

## Check Tally (brief §6, in order; all cargo offline: `CARGO_NET_OFFLINE=true` + `--offline`)

| # | Command (from `WORKING_ROOT` unless noted) | Outcome |
|---|---|---|
| 1 | `cargo fmt --manifest-path core/loads/primitive_loads/Cargo.toml --check` | PASS |
| 2 | `cargo test --offline --manifest-path core/loads/primitive_loads/Cargo.toml` | PASS — 49 passed, 0 failed (45 + 4 new) |
| 3 | `cargo fmt --manifest-path core/product_physics/Cargo.toml --check` | PASS |
| 4 | `cargo test --offline --manifest-path core/product_physics/Cargo.toml` | PASS — 94 passed, 0 failed (86 + 8 new) |
| 5 | `cargo fmt --manifest-path validation/benchmarks/mechanics/Cargo.toml --check` | PASS |
| 6 | `cargo test --offline --manifest-path validation/benchmarks/mechanics/Cargo.toml` | PASS — 38 passed, 0 failed (36 + 2 new; readiness mirror test green) |
| 7 | Two-mirror inventory check | PASS — both READMEs list `MECH-TP-PMM-P3-SUBSPAN-WIND-EXPOSURE` (hand-calc README also names `tp_pmm_p3_subspan_wind_exposure.md`); `git diff --numstat` = `1 0` for each mirror (exactly one additive line, zero removals) |
| 8 | `cargo test --offline --manifest-path core/solver/diagnostics/Cargo.toml` | PASS — 24 passed, 0 failed (read-only; no new finding code; mapping exhaustive) |
| 9 | `cargo test --offline --manifest-path core/runner/headless/Cargo.toml` | PASS — 23 (lib) + 1 + 15 (bin) passed, 0 failed (read-only) |
| 10 | `python3 -m pytest -q tests/test_model_schema.py tests/test_physical_to_analytical_transform.py` | PASS — 22 passed (19 pre-existing test functions + 3 new across the two files; the one authorized replacement sits inside the pre-existing schema test function) |
| 11 | `python3 tests/test_headless_runner_contract.py` | PASS (read-only contract guard; `CliOutput` untouched) |
| 12 | del1005 five-case byte-identity | PASS — runner built offline at the implementation head; all five stdouts `cmp` byte-identical to the committed `validation/witness/generated/del1005_payload_binding_*.json`; exits 0/0/0/1/1 |
| 13 | `python3 tools/validation/validate_claims_language.py --repo-root .` (REPO_ROOT) | PASS — `VALID claims-language surfaces: 262 files scanned; DEC-081 registry taxonomy satisfied` (re-run after all durable writes including this file) |
| 14 | `python3 tools/validation/validate_path_anchors.py . --text` (REPO_ROOT) | PASS — no literal home-dir absolute paths (669 → 671 live surfaces across the runs; re-run after all durable writes) |
| 15 | `git diff --check` (REPO_ROOT) | PASS (re-run after all durable writes) |
| 16 | JSON parse of new/changed `.json` | PASS — the only new/changed `.json` is `instances/W2/T4/CHANGE_SCOPE_CONTAINMENT.json` (tool-emitted, re-parsed); no project `.json` changed |
| 17 | `python3 tools/software_workflow/validate_change_scope.py "$REPO_ROOT" --base 581a15b1c718fd444870f13e75fc7cd974518670 --allowed <each §5 fence path> --path <each of the 14 tranche writes>` (REPO_ROOT) | PASS — status PASS, 0 violations over the 14 enumerated writes; JSON stdout persisted to `instances/W2/T4/CHANGE_SCOPE_CONTAINMENT.json`. The `--path` enumeration follows the W1 T3 precedent (its return records the same form): without it the tool sweeps every untracked file, and the pre-existing parent-owned `instances/W2/T5/CONDITION_VERIFICATION.md` (lawful R14 state, not a write of this run) sits outside the T4 fence; the tracked `git diff --name-only` set against the base commit was separately verified to be exactly the 10 tracked paths of the 14-write list |

Wave-level registered checks (`piping-pytest`, `evidence-sweep`,
`harness-pytest`, `harness-self-check`) and the single DEC-025 sweep are
W2-closeout acts of the manager, not this tranche (brief §5 item 10 / §6
closing paragraph, mirroring the recorded W1 refinement).

## Changed Paths (14 durable writes, all inside the §5 fence)

1. `core/loads/primitive_loads/src/lib.rs` (modified; no Cargo.toml/Cargo.lock change was required)
2. `core/product_physics/src/lib.rs` (modified; `validation.rs` NOT touched — the §4.2 strict-necessity condition was not met, as the brief expected; no Cargo.toml/Cargo.lock change was required)
3. `schemas/model.schema.yaml` (modified — §3.6 extension only)
4. `tests/test_model_schema.py` (modified — the single authorized replacement + additive assertions)
5. `tests/test_physical_to_analytical_transform.py` (modified — additive only)
6. `validation/hand_calcs/mechanics/tp_pmm_p3_subspan_wind_exposure.md` (NEW witness, written before implementation)
7. `validation/benchmarks/mechanics/src/lib.rs` (modified — additive fixture + registration + count bump 23→24 + the two forced `ElementExposedDiameter` literal updates; no Cargo.lock change was required)
8. `validation/benchmarks/mechanics/README.md` (modified — exactly one additive inventory line)
9. `validation/hand_calcs/mechanics/README.md` (modified — exactly one additive inventory line)
10. `execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-01_Primitive load case engine/_STATUS.md` (modified)
11. `…/DEL-05-01_Primitive load case engine/MEMORY.md` (modified — one new entry)
12. `…/DEL-05-01_Primitive load case engine/_run_records/WORKING_ITEMS_RUN_2026-07-20_R14_W2_T4_SUBSPAN_WIND.md` (NEW)
13. `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W2/T4/EXECUTE_RETURN.md` (NEW — this file)
14. `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W2/T4/CHANGE_SCOPE_CONTAINMENT.json` (NEW)

Ephemeral writes: cargo target dirs and del1005 stdout captures in the
task-local scratch area outside durable project paths. The other untracked
working-tree entries (the candidate brief and the pre-existing
`instances/W2/**` artifacts) are parent-owned lawful R14 state, not writes
of this executor.

## del1005 Evidence

Runner built offline at the implementation head
(`cargo build --offline --manifest-path core/runner/headless/Cargo.toml
--bin openpipestress-runner`); each stdout `cmp` byte-identical to its
committed witness; stdout SHA-256 (equal to the committed-witness SHAs
recorded in the W1 T1/T3 returns):

| Case | Verb | Exit | SHA-256 |
|---|---|---|---|
| `del1005_payload_binding_benchmark_single_case` | run-benchmark | 0 | `813a702b74be74a88755626d5b4530716d4fd5e27a1b988e48f3da3be3306728` |
| `del1005_payload_binding_benchmark_multi_case` | run-benchmark | 0 | `8feb3d25e50e78dcd7fcc85e2253021610faa971a37837eefb63df5cea456d68` |
| `del1005_payload_binding_regression_full_suite` | run-regression | 0 | `2f89adce9e4d6250280cee347822a567f4405eafbb8bc666483c6ce4cbd87593` |
| `del1005_payload_binding_benchmark_payload_missing` | run-benchmark | 1 | `9596c052c76a178e13bcf29faa5841848df6d9453983b184ebff3fd5fc2449a4` |
| `del1005_payload_binding_regression_payload_missing` | run-regression | 1 | `61cba4f28bcf109510489125b1de11e44796a25285ca64bf9c0714e870d9f518` |

No file under `validation/witness/**` is in the diff.

## Follow-Ons Reported for HELP_HUMAN Routing

- **GUI/operation-applier emission of sub-span wind marking** (DEL-07-02
  TP-PMM-GUIEMIT-001 surface; the existing whole-span field path is
  `equivalent_static.wind.exposed_pipe_refs`): the model-side input
  surfaces (preview `wind.exposed_spans`; canonical
  `WindEquivalentStaticInput.exposed_spans`) now exist, but no GUI,
  operation-applier, or editor emission was performed here. A separate
  lawful selection is required to expose sub-span marking to users
  through the operation surface.
- No other repair need outside the §5 fence was discovered; no
  validation-manual (docs) consequence was identified (the touched
  surfaces carry their own doc comments; `docs/**` untouched and no stale
  `docs/**` statement about wind marking was encountered in the surfaces
  read during this tranche — a full docs sweep was not in scope).

## Truthful Notes on Ambiguities and Recorded Choices

- **No-marking diagnostic message**: the one case with wind present but
  no marked span at all now reports "requires user-entered … 
  exposed_pipe_refs or exposed_spans" (previously "… exposed_pipe_refs").
  Same diagnostic id, code, and severity; the pre-existing product-physics
  assertion (`message.contains("exposed_pipe_refs")`) passes unchanged.
  Recorded as the single diagnostic-text consequence of the additive
  marking form, required for a truthful message; behavior for every
  marked (valid or invalid) input is otherwise unchanged.
- **Overlap semantics**: "disjoint" is implemented as
  non-intersecting interiors — extents sharing exactly one endpoint are
  allowed (no double-counted exposure; tested). Overlap blocks.
- **Extent id suffix formatting** uses Rust's shortest-round-trip f64
  Display (e.g. `:extent-0.8-1`), which is deterministic for identical
  inputs and injective for distinct f64 extents.
- **`PrimitiveLoadError::InvalidExtentBounds`**: a new constructor-error
  variant (Display-only outside the crate; the sole external consumer
  `diagnostic_from_primitive_load_error` formats via `to_string()`), not
  a finding/diagnostic code; `FindingCode` and `core/solver/diagnostics`
  are unchanged, per the no-new-FindingCode predicate.
- **Line-range note from the v2 verifier** (§2.1 precision note): the
  replacement was applied in the minimal reading — the subset assertion
  replaced; the binding, `additionalProperties`, and
  `exposed_element_refs` `minItems` assertions retained verbatim and
  extended additively — which the verifier confirmed lands
  equal-or-stronger under either reading.
- **claims-language scan count** remains 262 files (the validator scans
  its registry surface set; the new witness/run-record files are outside
  that registry). Path-anchor surfaces grew 668/669 → 671 with the new
  files, all PASS.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
