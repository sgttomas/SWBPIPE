# CP3_SCHEMAS — RETURN

Type 2 TASK for the load-state WORKING_ITEMS manager (`a3675abb28ada0834`); spawned by ROOT.
Paths are WORKING_ROOT-relative. `LSI` = this TASK's parent folder. Machine paths,
exact commands and raw logs are in `_run_records/` (`commands.txt` is the ledger).

**Result: complete.** The new schema was frozen first and its hash was sent to the manager
before any carrier edit. The three carriers gained additive, closed load-reference-1
branches. The preservation check passes. All 527 new tests pass. The required pre-existing
tests have identical outcomes with and without my changes. Two of those tests already fail
at pristine HEAD, outside my boundary (section 4).

## 1. Inputs verified

- The brief `CP3_SCHEMAS/BRIEF.md` matches `c1eede81b5b40dc62115ff47f367d9ba3aed7ffdd1744538ffd2c742523fa274`.
- All nine frozen inputs matched the brief's sha256 values when read: `CP2_WIRE.md` `81a7adba…`,
  `CP2_WIRE_ADDENDUM_1.md` `c389f5e3…`, the load-reference-1 table `44bc41c0…`, and the six
  request/raw fixtures.
- The typed DTO `core/product_physics/src/case_state/input.rs` was read at
  `602725e42baaa9952d11138fab585c24ae280b7e4a2d634cc456c812528cf14f`. It is manager-owned and in flight.
- Pre-edit carrier hashes were recorded before editing (`_run_records/pre_edit_carrier_sha256.txt`).
  HEAD bytes equal these, and HEAD serves as the preservation baseline:
  - results.v0.3: `a527f083dabc9e64ab7103e16c51103e85b6a67f5cef1427d1e8e908d7f5cc9f`
  - analysis_run.v0.3: `f81f1c8e04ba3ecce5da51d72f303d41f6d710b26ea1a77f9163cfe94cae6abe`
  - stress_neutral_export.v0.3: `af3d229289c72349f2daa5b7826bd7f0be4924d6b854c6358d480c89d1d991dd`
- HEAD moved from `404cd9c6b` to `d92ca3b7c` during the run. The new commit changes only the
  review records under `execution/`, and the carrier bytes are unchanged at both commits.

## 2. Files and sha256

| File | sha256 |
|---|---|
| `schemas/load_reference_state.schema.json` (new; **frozen**, sent to the manager) | `640fd4477ac2c84f3c02268cfccc3958f51ee6508b3a67e5538b52d84899af65` |
| `schemas/results.v0.3.schema.yaml` (additive) | `c303219b89c63f41bdda71b49af1ed15175c106b035ff4f5b50e2eaf2cdc57ed` |
| `schemas/analysis_run.v0.3.schema.json` (additive) | `2045d3651709ec32c53b17a16f504c4818c9f14d7898bdfacf7dbc4068477410` |
| `schemas/stress_neutral_export.v0.3.schema.json` (additive) | `80acc90bcf7170c7f07da2f1569a258fc56c4de455c8e936bf8da476e6478e60` |
| `tests/test_load_reference_schema.py` (new) | `9c02dbba8a909c51b124e07de88bc50114e25eb987cbfe705bc23879f4e6c51d` |

All four schema files were written atomically: a temporary file in `schemas/`, then `os.replace`.
The generator and edit scripts are kept in `_run_records/` (`gen_load_reference_state_schema.py`,
`edit_carriers.py`), so the bytes can be re-derived.

## 3. What was built

### 3.1 `load_reference_state.schema.json`

A draft 2020-12 definitions library with no root assertions. Validate one `$defs` entry.

**Authored namespace**, closed throughout (`additionalProperties: false`), with tagged unions
built from `oneOf` and `const` discriminants:

- Reference: `ReferenceConfiguration`, `GeometryReference`, `MemberReference`, `ReferenceBasis`,
  `FitReference`.
- Expansion laws: `ExpansionLaw` (four `definition`s) with `SecantData`, `DilationData`,
  `CoefficientTableData`, `CoefficientPoint`, `DilationPoint`, plus `MaterialExpansionLaws`.
- Case state: `AnalysisState`, `ElementState`, `MaterialSelection`, `ThermalState`,
  `AnalysisBasisOverride`.
- Supports: `SupportState`, `Participation` (active, inactive, and locked with
  `LockedComponent`/`PositionSource`), `Motion`, `DeviceReference`.
- Sources and history: `LoadSource`, `History`.
- `Quantity` is a closed `{value: number, unit: string}`.

Required and optional keys follow CP2_WIRE and the DTO. The `description` fields state that
coverage, resolution, unit dimension, finiteness, empty text, an unknown `contract` string and
the parse-then-block branches are producer diagnostics, not shape. So `contract` is a string,
not a const, and ids and text carry no `minLength`.

**Raw producer evidence**, closed:

- `LoadReferenceContractEvidence` is `{pressure, connector: [], exact_cases, load_reference_states}`.
- `LoadReferenceStateRecord` has exactly the §3 keys:
  - `reference_geometry` with a 64-hex hash;
  - `solve`, where the mode and method pairing is a closed `oneOf`;
  - `source_recovery` const `{not_joined, LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED}`.
- It uses these item definitions:
  - `LoadReferenceMember`: all §3 member keys; absent optionals are `null`.
  - `ConsumedMaterialPoint`.
  - `LawSegment`, whose `use` is an enum.
  - `SupportComponent`: DOF and unit are coupled, m for U\* and rad for R\*.
  - `Contribution`: four owner kinds, each with its fixed classification.
  - `ExcludedSource`.
- `LoadReferenceExactCase` is the physics `exact_cases` item with `material_basis` const
  `resolved_per_member_load_reference_state_v1`. Its `pipe_materials` items are
  `LoadReferencePipeMaterial`: the physics material plus `material_selection_kind` (enum) and
  `resolved_eigenstrain`.
- `LoadReferencePressureEvidence` is the physics pressure item except `materials[].temperature_basis`
  (finding F1). The local `PhysicsGeometry` is a byte-equal copy.
- Tests pin each copy equal to results.v0.3 `PhysicsContractEvidence` except at the documented
  pointers.

**Reader compatibility:**

- Every `$ref` is local and one level deep (`#/$defs/Name`).
- The only patterns are `^[0-9a-f]{64}$` and `^[0-9a-f]{16}$`.
- Every object has `properties` and every array has `items`. No `not`, `if` or `prefixItems`
  is used.
- The file can therefore serve as a root for the repository's closed-shape checkers: Python
  `source_blocks._shape`, Rust `shape_in` and TS `shape`.
- The vocabulary is pinned by a test. The Python checker agrees with jsonschema on 27 positive
  and negative cases (`_run_records/shape_checker_crosscheck.log`). I did not run the Rust and TS
  checkers; I checked them only by reading the source.

### 3.2 Carrier branches

Each new branch is a deep copy of that file's own physics-1 branch. The ID, table hash and
profile constants are swapped, and where physics-1 carries it, the `contract_evidence` `$ref` is
swapped too. Each branch is appended as `oneOf[4]`. Only enum arrays and `oneOf`/`anyOf` lists
are extended.

| Carrier | Added |
|---|---|
| `results.v0.3` | - LR ID in the producer and `semantic_contract_ref` enums.<br>- LR profile in the `formulation_basis` enum.<br>- `LoadReferenceContractEvidence` appended to the `contract_evidence` `anyOf`.<br>- Branch: requires `contract_evidence` → `load_reference_state.schema.json#/$defs/LoadReferenceContractEvidence`; `result_sets` items are `ResultSet`; forbids `source_block_recovery`.<br>- This file carries no table hash for any method, so none was added. |
| `analysis_run.v0.3` | - `SemanticContract` gains the LR ID and hash in its enums, plus an `{id, sha256}` pair in its `oneOf`.<br>- `AnalysisRun` branch: LR ID and hash on `reproducibility` and every `result_refs[]`.<br>- Forbids both `source_block_recovery` and `contract_evidence`, exactly like physics-1 (the manager's clarification of the brief).<br>- No profile exists in this file. |
| `stress_neutral_export.v0.3` | - LR ID in the producer, `semantic_contract_ref` and `semantic_contract.id` enums; LR profile in the `formulation_basis` enum.<br>- Branch: `semantic_contract` `{LR ID, 44bc41c0…}`; profile const; `contract_evidence` → LR evidence.<br>- Keeps the physics-1 utf8 CSV and manifest canonicalization.<br>- Requires `contract_evidence` and `source_annotations`; forbids `source_block_recovery`. |

## 4. Checks: commands, logs and results

The interpreter was the manager-supplied venv: Python 3.11.15, jsonschema 4.26.0, referencing 0.37.0,
pytest 9.1.1 (`_run_records/environment.txt`). `<py>` below is that interpreter, run from WORKING_ROOT.

| Check | Command (see `commands.txt`) | Log | Result |
|---|---|---|---|
| Strict JSON parse and draft 2020-12 metaschema for all four schemas | in the new tests (duplicate-key and NaN/Infinity rejecting parse) | `new_tests_worktree.log` | **pass** |
| Preservation | `<py> _run_records/preservation_check.py` | `preservation_check.log` | **PASS**. Baseline pointers compared: 2192, 614 and 2134, with 0 failures. Extended lists: 5, 4 and 5, each only appended. |
| New tests (worktree) | `<py> -m pytest -q tests/test_load_reference_schema.py` | `new_tests_worktree.log` | **527 passed** (exit 0; the log header records the input hashes used) |
| New tests (isolated HEAD + candidate) | same, in a git-archive scratch tree | `new_tests_isolated_head_plus_candidate.log` | **527 passed** (exit 0) |
| Test-the-tests | `<py> _run_records/mutation_check.py` (scratch tree) | `mutation_check.log` | **PASS**: all 13 deliberate schema weakenings caught (each makes its targeted tests fail; original bytes restored) |
| Required pre-existing tests (worktree, after edit) | `<py> -m pytest -q tests/test_results_schema.py tests/test_analysis_run_schema.py tests/test_stress_neutral_*.py tests/test_source_block_schema_contract.py` | `existing_tests_post_edit.log` | 293 passed, **2 failed**, 11 skipped |
| Same set, pristine HEAD (isolated archive, none of my changes) | same | `existing_tests_isolated_base.log` | 293 passed, **2 failed**, 11 skipped |
| Same set, HEAD + only my four schema files (isolated archive) | same | `existing_tests_isolated_cand.log` | 293 passed, **2 failed**, 11 skipped |

**The two failures are pre-existing and outside my boundary.** They are
`tests/test_stress_neutral_physics_source.py::test_method_namespace_removal_substitution_and_relabel_are_rejected[physics]`
and `[source_blocks]`.

- Both raise `ValueError: SN-CSV-PROFILE-MISMATCH` from `rehash_packet` →
  `core/handoff/stress_neutral/package_v0_3.py::_csv_policy`. This happens at test line 183, after
  the test relabels the producer to an unknown method and before any schema is read.
- The failure reproduces identically at pristine HEAD, in a `git archive` tree with none of my
  edits and none of the other lanes' in-flight changes.
- The same test IDs and the same error appear in all three runs.
- I did not modify or weaken the test. The 11 skips are the pre-existing
  `HEADLESS_SOURCE_BLOCK_OUTPUT_DIR` artifact gates.

**New-test coverage (527 cases):**

- Positive:
  - both frozen requests: each `reference_configurations[]`, each material's `expansion_laws`,
    each case's `analysis_state`;
  - all four raw `contract_evidence` objects;
  - every reviewed authored variant;
  - null and absence for the optional authored fields.
- Negative:
  - an unknown key at every object level: 71 authored shapes, and 33 raw shapes across both
    envelope families plus a populated-variant instance;
  - a missing required key: 183 authored keys, where the wire-optional keys are instead proven
    optional, and 124 raw keys;
  - unknown and null discriminants for `kind`, `definition`, `meaning`, `interpolation`,
    `extrapolation`, `coefficient_meaning` and history;
  - a fit with both `length_change` and `strain`, under all three kinds;
  - 24 raw enum and constant mutations.
- Carriers:
  - the LR branch accepts verbatim raw metadata in all three carriers;
  - physics-1 carriers still validate;
  - rejected: wrong ID, hash or profile in each position; a mixed row contract;
    `source_block_recovery` present (an actual valid source-blocks receipt); physics evidence
    under LR; missing evidence or `source_annotations`; a physics-1 carrier carrying
    `load_reference_states`; LR relabelled as physics-1.
- Carrier instances:
  - The AnalysisRun and stress-neutral instances are transforms of the actual physics-1 carriers
    that the existing Python route builds.
  - No results.v0.3 physics-1 carrier is checked in (only the Rust headless lane produces one),
    so results uses a minimal hand-built scaffold around verbatim raw
    `producer/formulation_basis/numerical_quality/contract_evidence`.
  - These are schema-level instances only, with no hash or cross-binding claim. The manager
    validates the READERS TASK's actual carriers at integration.
- The tests never pin raw-fixture hashes or diagnostic text, so regenerated envelopes are read
  as they are.

## 5. Wire/DTO ambiguities and findings

- **F1. Pressure cannot reuse the physics item by `$ref`.**
  - The producer publishes `pressure[].materials[].temperature_basis` as the string
    `"resolved_member_state"` (`pressure_runtime.rs:640`). The physics shape requires one of three
    objects there.
  - Neither CP2_WIRE nor the addendum states this.
  - The local copy differs only at that pointer. The equality is pinned by a test, and another
    test shows that the actual raw pressure item fails the physics shape.
- **F2. Addendum §3 omits two producer keys in `contributions[]`.**
  - `support_state` entries carry `value`, the prescribed value (`resolve.rs:1028`).
  - `pressure_region` entries carry `factor: null` (`resolve.rs:1050`).
  - The schema admits exactly the producer's key sets. The addendum should list both keys.
- **F3. The typed boundary is looser than the wire's "all new objects are closed".**
  - Under serde 1.0.228, internally tagged unit variants ignore extra keys
    (`InternallyTaggedUnitVisitor`). This affects `authored_model_geometry`,
    `direct_strain_reference`, fit `none`, `active_model_device`, `inactive` and
    `independent_equilibrium`.
  - `Quantity` (`lib.rs:301`) has no `deny_unknown_fields`.
  - I derived this from the serde source. The independent review (`REVIEW_CHECKPOINT_2`, SF-A,
    probe p03) confirms it empirically.
  - The schema is closed as the wire states: it rejects, for example, `{"kind":"none","strain":…}`
    and quantity extras.
  - If the repair records `Quantity` as an explicit open exception instead of closing it, drop
    `additionalProperties` from `$defs/Quantity`. That is one line in the generator.
- **F4. Null is accepted as absence for the optional authored fields.**
  - The fields are `label`, `operating_temperature`, `analysis_basis_override`, `mass_state_ref`,
    `boundary_motion`, `base_motion` and `device_reference`. This matches serde
    `Option` + `default`.
  - A present `expansion_laws: null` is rejected, which matches the DTO's typed `Err`.
  - This ties to REVIEW_CHECKPOINT_2 N-2. If the manager decides a present null must block or
    be refused, the change is a small edit to the `nullable(...)` wrappers.
- **F5. `contract_evidence` in analysis_run.**
  - The brief line "contract_evidence → LoadReferenceContractEvidence" conflicted with the
    physics-1 pattern: physics-1 forbids `contract_evidence`, and its property is a closed `$ref`
    to `physical_evidence`.
  - The manager ruled that the physics-1 mirroring governs. The branch forbids the key, and the
    change stays additive.
- **F6. results.v0.3 carries no semantic-table hash for any method.** The brief's "table sha256"
  therefore applies only where physics-1 expresses one: analysis_run and stress-neutral.
- **F7. Evidence cardinalities are not encoded.**
  - Examples: 0 to 2 consumed material points; the two `consumed_input_refs`; the pairing of
    `fit_kind` with `fit_input`; case, member and pipe bijections; unique source IDs; nonzero
    factors.
  - These are cross-field rules for the reader validators, not shape.
- **F8. Variant coverage from producer output is narrow.** The frozen requests exercise only
  `exact_point` + `free_length_state` with a constant secant law, fits `none` and
  `natural_length_change`, and `active_model_device`. The other reviewed variants are covered by
  hand-built instances, not actual producer output.

## 6. Boundary and environment notes

- **Implicit Cargo.**
  - My first pytest run triggered `tests/conftest.py::pytest_sessionstart`, which runs
    `cargo build --release` for the checked-json and units helper binaries.
  - The builds went into the git-ignored `core/serialization/canonical_json/target` and
    `core/units/target`, created at 18:38Z, about 78 MB.
  - I never invoked Cargo directly. Every later run points `OPENPIPESTRESS_CHECKED_JSON_BIN` and
    `OPENPIPESTRESS_UNITS_BIN` at those binaries.
  - The target directories are left in place at the manager's direction.
- **Bytecode.** The first run may also have written git-ignored `__pycache__` bytecode. Later runs
  set `PYTHONDONTWRITEBYTECODE=1`.
- **Scratch trees.** The isolated trees live in the session scratch directory, outside the
  repository.
- **Writes and Git.** I made no Git write, and no write outside the boundary other than the build
  output above. No producer, reader, fixture, table or library content was changed.
- **Concurrent lanes.** `core/analysis_runs/compatibility.py`, `result_export` and
  `product_physics` were modified by other lanes during the run. The isolated runs exclude those
  changes.

## 7. For the caller to carry forward

- Validate the READERS TASK's actual load-reference-1 carriers against the three branches at
  integration.
- Decide F3 (close the DTO unit variants and `Quantity`, or declare an exception) and F4 (null
  policy).
- Amend addendum §3 for F1 and F2.
- Re-run `tests/test_load_reference_schema.py` after the raw envelopes are regenerated.
- The `test_stress_neutral_physics_source` failures need an owner outside this TASK.
