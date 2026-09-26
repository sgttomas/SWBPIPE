# Checkpoint 3 — retained-source join, CP2 review repairs, readers and schemas

- **Author:** the session-2 WORKING_ITEMS manager.
- **Checkout:** branch `codex/piping-load-states-20260925`, base `d92ca3b7c` (the CP2 candidate `404cd9c6b` plus the CP2 review record), with the bytes below.
- **Paths:** WORKING_ROOT-relative. Machine records are in `_run_records/session2/`.

This record states completed execution, not acceptance.

- **CP3 independent review: not dispatched when this record was written.** During CP3 the owner directed "Don't start any more tasks. Bring your current work to completion and then pause." Afterwards the owner explicitly authorized one independent CP3 review, which ROOT will dispatch on the frozen commit. Nothing in this checkpoint has yet had a fresh-context non-author review.
- **`load-reference-source-1` is not active.** Its identity and its profile `resolved_straight_load_state_source_v1` are ROOT-reserved. They are carried only inside this candidate and count as active only once the frozen candidate passes that review.
- **No finding closes.** No M10, M16 or M29 finding closes at this checkpoint.

The wire of record is:

- `CP2_WIRE.md` (`81a7adba…`);
- `CP2_WIRE_ADDENDUM_1.md` (`c389f5e3…`);
- the new `CP2_WIRE_ADDENDUM_2.md` (`ec66628e…`);
- the new `CP3_WIRE_ADDENDUM.md` (`f69043b682d027cc7f15e2173b932f016cd76f3a64d76b76d974cd0f0105f7d4`), which defines the expansion-law consumed/consulted labels per definition.

## 1. What is done

### 1.1 Retained-source join for resolved cases

This is the required connected work that the CP2 not-joined guard held open. It is now implemented, and the guard is replaced.

**Source adapter** (`src/source_recovery.rs`). `Input` carries the case's `ResolvedCase`. Source closure admits a 0.4.0 case only when the adapter closes on that resolved state:

- **Ownership.** A resolved case is admitted only with a 0.4.0 model, and a 0.4.0 model never omits one.
- **Member pairs.** Each member's selected E and derived G are the built frame operands, bitwise. The frame area equals the exact wall area.
- **Eigen loads.** Each eigen element load equals `E_member·A_s·ε*` bitwise, one per member with nonzero ε*, in member order. It is folded into the force after the nodal loads, in the product's own operation order, and reproduces the actual force bitwise.
  - It is identified as force terms owned by `load_state_eigenstrain:<len>:<pipe>`. A colliding primitive ID is refused.
  - It is removed once through exact member-end offsets: +a at end-i axial and −a at end-j axial. Sections inherit the offset.
- **Prescribed motions.** Resolved support motions are the prescribed values of their unique rigid owners. Every other rigid DOF is an explicit zero.
- **Identity.** The retained identity binds:
  - the pairs, strains, eigen loads and local axes;
  - the motions;
  - the resolver evidence.
- **Unchanged refusals.** Pressure regions must still be explicitly empty. Every other non-nodal producer is still refused.

**Product route** (`src/lib.rs`).
- A 0.4.0 case attempts retained source under the existing eligibility: capture, no nonlinear supports, no combinations, and an ordinary attempt that is sensitive or rejected.
- A selected case publishes the retained projections.
- A case that is not selected publishes its ordinary response, with a NOT_JOINED record and a diagnostic stating the actual attempt: `not_required_ordinary_checks_passed`, `unavailable` or `not_eligible`.
- The eigen-load construction is shared with receipt replay, as `load_state_eigen_loads`.

**Receipt** (`src/source_receipt.rs`, `composite.rs`, `source.rs`).
- The closed receipt policy enum now has three members: SOURCE-BLOCKS-1, PHYSICS-SOURCE-1 and LOAD-REFERENCE-SOURCE-1.
- The policy is chosen from the captured invocation, never from the envelope label.
- **Re-derivation.** A 0.4.0 case is re-derived from the captured request through the resolver pipeline:
  - `validate_profile`, `validate_document`, normalization and `resolve_case`;
  - a member-pair build;
  - effective-case loads plus eigen loads;
  - the resolved prescribed values.

  The case-wide base and modulus methods are never used.
- **Checks.** Replay, current-source binding and the physical binding of `exact_cases`, `pressure` and the case's `load_reference_states` record all run against that re-derivation.
- **Evidence namespace.** `contract_evidence` is exactly `{pressure, connector, exact_cases, load_reference_states}`.
- **Per-case proof.** Each case's evidence is hashed under `load_reference_source_case_evidence_v1`.
- **Payload.** The source commitment payload for a selected case adds a `load_reference_state` block. It is hashed into `normalized_source_sha256`; the payload itself is not published, as for every source payload. Eigen force terms have owner `member_eigenstrain`.
- **Pre-0.4 unchanged.** Pre-0.4 payload bytes, receipts and hashes are unchanged.

**Publication.** The mapping follows ROOT's accepted semantics 1–2:

- If no case selects retained source, the envelope is `load-reference-1`, with its table, hash, record shape and status value unchanged.
- If at least one case selects and the receipt finalizes, the envelope is `load-reference-source-1` with its exclusive profile and policy `LOAD-REFERENCE-SOURCE-1`.
- In both, `not_joined` means "this case published the ordinary route".

### 1.2 Witnesses and table

All inputs are invented.

**Witness fixtures.** `fixtures/product_preview/load_reference_source/`, authored by `cp3_author_join_requests.py`:

- **0.4.0 unchanged-state companions** of the physics-source-1 witnesses n05, n06, fields and mixed.
  - Every result row is bit-identical to the 0.3.0 physics-source-1 raw fixture in both modes (`cp3_join_companion_compare.{py,log}`).
  - The only differences are intended: no case-wide modulus-basis row or redundant-G warning; pressure `temperature_basis`; identity digests.
  - Mixed is a joined envelope with one retained-source case and one ordinary pressure case, as in 0.3.0.
- **`eigen_motion`.** A sensitive member with thermal 6e-5 and fit 4e-5 eigenstrain, a moving tip stop, and a rigid root translation and rotation. Its closed-form values are checked by `source_receipt/load_state_tests.rs`.
- **Raw envelopes.** Ten, generated by the producer (`examples/physics_source_connected.rs`) and all qualified.

**Semantic table.** `fixtures/results/semantic_contract_v0_3_load_reference_source_1.json` (`d1628194a7730f427843b00228dd233cf92b8e7d26f3bc31c660a3ea59e28337`).
- It is derived from physics-source-1 (`ba13f2ae…`) by `cp3_make_source_semantic_table.py`.
- Rows and policies are unchanged.
- Every emitted signature and declared source basis is present.

**Regenerated load-reference-1 raw fixtures.**
- The four `fixtures/product_preview/load_reference/*.raw.json` files were regenerated by the producer, never hand-edited, and relayed to READERS before its freeze.
- They differ only in the NOT_JOINED message.
- Substituting the CP2 message back reproduces the frozen hashes byte-exactly (`cp3_regen_diff_check.{py,log}`).
- New hashes are listed in ADDENDUM_2 §6.

### 1.3 CP2 review repairs (ROOT dispositions)

| Finding | Repair | Pinned by |
|---|---|---|
| SF-A: field-free branches and namespace quantities dropped sibling keys | The six unit variants are now empty struct variants. Namespace quantities use a closed `deserialize_with` wire (`ClosedQuantity`). The shared legacy `Quantity` stays open, recorded as a pre-existing exception. `geometry_ref` stays exactly `{kind}` and refuses any authored hash, forged or actual (ROOT: refuse). | DTO test `closed_namespace_refuses_sibling_keys…`; public-route test `public_route_refuses_sibling_fields_on_every_field_free_branch_and_quantity` (the reviewer's p03 cases, plus forged and actual hashes, plus two quantities) |
| N-1: observation lanes unpinned | Dense parity is pinned at ≤ 1e-12 under prescribed root motion, and the lane must be available. | `dense_observation_lanes_observe_the_prescribed_boundary_system`; mutants M02/M03 killed |
| N-2: null or malformed new keys handled inconsistently | `Authored<T>` (absent / null / value) for model `reference_configurations`, material `expansion_laws` and case `analysis_state`. Request-level `materials[].expansion_laws` is detected. Pre-0.4 presence, including null, blocks with `LOAD_STATE_CONTRACT_VERSION_MISMATCH`. A 0.4.0 null blocks with the new `LOAD_STATE_EXPLICIT_NULL_UNSUPPORTED`. A malformed legacy key stays a typed `Err`. | `an_explicit_null_new_key_is_authored_presence_in_every_version` |
| N-3: material records last-wins | Model and request material records use a typed wire (`MaterialRecordWire`: flattened `MaterialInput` plus `expansion_laws`). `LinearStaticPreviewRequest` has a manual `Deserialize`; its public fields are unchanged. | `material_records_keep_typed_duplicate_key_refusal_with_positions` |
| N-4: public-route acceptance gaps | Assigned by ROOT to the non-implementer runtime TASK: `CP2_RUNTIME_TESTS/EXTENSION_1/BRIEF.md` (`f453bcec…`). | See §3 |
| N-5: silent `?` could drop an eigen load | A typed error. The product blocks with `LOAD_STATE_MEMBER_SECTION_MISSING`, and replay refuses. No `expect` is used. | `a_resolved_eigenload_without_its_built_section_blocks_instead_of_disappearing` (constructs the missing-section state) |
| N-6: record accuracy | Recorded in ADDENDUM_2 §1 and in §2 below. CP2 records are not rewritten. | — |

### 1.4 Delegated work, integrated

**SCHEMAS** (`a95af25cf935fa27b`, complete; `CP3_SCHEMAS/RETURN.md`).

New schema: `schemas/load_reference_state.schema.json` (`640fd447…`).

Additive load-reference-1 branches in the three carriers:

| Carrier | sha256 |
|---|---|
| `results.v0.3` | `c303219b…` |
| `analysis_run.v0.3` | `2045d365…` |
| `stress_neutral_export.v0.3` | `80acc90b…` |

Each branch mirrors physics-1 in its file, so the AnalysisRun carries no `contract_evidence`.

Tests: `tests/test_load_reference_schema.py`, 527 passed. The manager reran it after the raw regeneration, and it still passes (`cp3_schema_tests_after_regen.log`). Preservation and 13/13 schema-weakening mutations were checked by the TASK.

**READERS** (`a04de5953be29226b`, partial; `CP3_READERS/RETURN.md` `75f6f0a0…`).

- **Rust `core/reporting/result_export`:**
  - `LOAD_REFERENCE_ID/PROFILE/TABLE_SHA256`, `load_reference_contract()` and `verify_load_reference_table`;
  - a closed `validate_load_reference_evidence`, plus transport metadata checked against the frozen schema;
  - `for_source_metadata` binds load-reference-1 exclusively to `resolved_straight_load_state_v1`;
  - `derivative` copies `contract_evidence` verbatim.
- **Python `core/analysis_runs`:**
  - `compatibility.py` registrations;
  - `load_reference_evidence.py`;
  - AnalysisRun 0.3 records that mirror physics-1.
- **Parity.** A shared case file of 201 IDs gives 18 accepted and 183 rejected in both languages, with no case accepted in one and rejected in the other. 358 of 387 comparisons are exact, 20 match on the inherited leading code, and 9 are declared language-specific: non-finite numbers are rejected at the Rust text boundary, and the precision-1 header order predates the TASK.
- **Existing outputs are byte-identical:** 204 Rust outputs and 29 Python AnalysisRun/stress-neutral digests.
- **Carriers.** Four canonical results 0.3 documents and four AnalysisRun 0.3 records, derived from the regenerated raws and reproducible by its `regenerate_carriers.sh`.
- **Stress-neutral outputs.** The host permission system blocked the TASK's own write. The outputs were then produced as integration work (below).
- **For review:** READERS' stricter bindings derived from producer code (RETURN item 3).
- **Carry-forward:**
  - the `core/runner/headless` load-reference-1 path is not specifically tested;
  - no rustfmt pass was run on its Rust files, because the toolchain lacks the component.

**Manager integration check** (`cp3_integration_carrier_validation.{py,log}`).
- The READERS carriers validate against the SCHEMAS carrier schemas:
  - four results documents against `results.v0.3`;
  - four AnalysisRun records against both the `analysis_run` root and `analysis_run.v0.3`.
- A physics-1 relabel of each document is refused.
- The carriers carry the regenerated NOT_JOINED text.

**Stress-neutral 0.3 outputs.**
- **How the edit was made.** READERS was refused by the host permission classifier ("Modify Shared Resources") when it tried to edit `core/handoff/stress_neutral/package_v0_3.py`. No agent routed around that refusal. After the **owner explicitly approved** the edit, **ROOT applied it itself** in this checkout: `db128dab…` → `1ef2d85985169b8b02d8e3da26ea3597600b67538df9f830536ca867e8a1fd7f`.
- **What the edit changes.** It makes three additive changes: `LOAD_REFERENCE_CONTRACT_ID` is added to the import, to `SUPPORTED_METHODS` and to `PHYSICAL_METHODS`. Nothing else changes. `load-reference-source-1` is deliberately **not** added while it stays reserved-inactive.
- **Outputs.** At ROOT's direction, the manager generated the four packages with `cp3_stress_neutral_outputs.py`. That script mirrors the physics-1 construction in `tests/test_stress_neutral_physics_source.py`, and its AnalysisRun equals the READERS carrier. The packages are:

  | File | sha256 |
  |---|---|
  | `fixtures/results/load_reference_connected_sparse.stress_neutral.json` | `e6a3be75…` |
  | `fixtures/results/load_reference_connected_dense.stress_neutral.json` | `0c67b958…` |
  | `fixtures/results/load_reference_pressure_sparse.stress_neutral.json` | `cf95c609…` |
  | `fixtures/results/load_reference_pressure_dense.stress_neutral.json` | `a44f8116…` |

- **Validation.** Each package validates against the SCHEMAS `stress_neutral_export.v0.3` branch (`80acc90b…`) and against the package validator, both alone and with source and analysis. Each nine-member materialization round-trips, and a physics-1 relabel is refused.
- **Reproducibility.** A `--check` rerun reproduces the bytes (`cp3_stress_neutral_outputs{,_check}.log`). No committed test asserts these packages yet; the script is their reproducer.

### 1.5 Runtime-test extension (review N-4)

**TASK.** `acfae78c40c4525d2`, resumed by ROOT. It is complete; see `CP2_RUNTIME_TESTS/EXTENSION_1/RETURN.md` (`4fe96710…`).

**Tests.** `core/product_physics/tests/load_reference_state_runtime_extension.rs` (`5febacba957bc9aed01c8010ea3496616d05021290c2b15d0e08cbabf4558a58`) has nine public-route tests. Each runs in both modes, and each names the mutant it targets:

1. pressure with per-member pairs (M06/M19/M23);
2. identity groups across units (M22);
3. the non-equal control, with override required (M24);
4. `verification_two_point`, plus out-of-interval blocks (M16);
5. `TEMPERATURE_IDENTITY_UNRESOLVED` (M15);
6. duplicate-class points (M14);
7. dilation consulted versus consumed (M17);
8. joined `eigen_motion`, closed form and publication;
9. its unavailable-join pressure companion.

**Test 7 history.** Test 7 first failed on the dilation segment label. The TASK had assumed a sample-difference method, which the wire had not fixed.
- ROOT selected the implemented, CP1-reviewed method: Δd is the sum of segment increments over [T_install, T], and those segments are consumed `integration_interval`.
- `CP3_WIRE_ADDENDUM.md` now defines the labels per definition, their permitted overlap and degenerate samples.
- The TASK revised only that assertion, to derive from the addendum, and reran on a fresh build: 9/9, with CP2's 20/20 unchanged.
- This is not an expectation moved toward an observed value: the wire now defines the method that the assertion checks.
- The producer did not change.

**Notes routed by the TASK, for the CP3 review.**
- The kill targets are argued, not demonstrated: no mutant was run, because the brief allowed no `src/` writes.
- **Torsion visibility (test 8).** The tip RX includes a torsion term T·L/(G·J) of about 4.6e-15 rad. That is below the 1e-9 check on 1e-4 rad, so the check pins the spring path, not GJ.
- **Not exercised publicly by the TASK:**
  - duplicate-class refusal through an exact-point selection;
  - `LOAD_STATE_EXPLICIT_NULL_UNSUPPORTED`;
  - `LOAD_STATE_MEMBER_SECTION_MISSING`;
  - ADDENDUM_2's closed-quantity and empty-variant rejections.

  The manager's in-module tests pin the last three.

## 2. Corrections to earlier records

- **CHECKPOINT_2 typed-boundary claims were inaccurate.** It said unknown fields were rejected at the typed boundary; unit variants and namespace quantities accepted extra keys (review SF-A). This is now true, as §1.3 records.
- **Not every unimplemented branch parses and then blocks.** CP2_WIRE says they do, but `history: continuation` is a typed `Err` (ADDENDUM_2 §1).
- **Stale "until join is verified" text.** CHECKPOINT_2 and the load-reference-1 formulation limitation say retained source "is not joined for these inputs". For `load-reference-1` envelopes this still reads true: they are only published when no case joined. The NOT_JOINED message text that said "until its join is verified" is replaced.

## 3. Checks

The raw record is `CHECKS_CHECKPOINT_3.json`.

**Final gate** (the final bytes; `cp3_final_gate_all_tests.log`, prehash `cp3_final_gate_prehash.txt`), whole product_physics crate:
- 402 passed, 0 failed, 1 pre-existing ignored;
- lib 311, which is CP2's 299 plus 12 CP3 tests;
- runtime 20/20 unchanged (`d5be0bd8…`);
- runtime extension 9/9;
- every other integration binary unchanged and passing;
- sources hash-stable during the run;
- warnings identical by name to the CP2 gate.

The earlier gate on the same product source was 393 passed (`cp3_gate_all_tests.log`), before the extension file landed. The manager's target was deleted afterwards.

**Integration reruns by the manager:**
- result_export: 64/64 (`cp3_integration_result_export_tests.log`; own target, deleted afterwards).
- `tests/test_load_reference_readers.py`: 212 passed, 1 skipped (`cp3_integration_readers_py.log`).
- `tests/test_load_reference_schema.py`: 527 passed after the regeneration.
- Carrier validation (`cp3_integration_carrier_validation.log`).
- **After ROOT's `package_v0_3.py` edit:** `tests/test_stress_neutral_export_package.py`, `test_stress_neutral_physics_source.py`, `test_stress_neutral_precision.py`, `test_load_reference_readers.py` and `test_load_reference_schema.py` gave 928 passed, 1 skipped and 2 failed (`cp3_python_after_stress_neutral_edit.log`). The two failures are exactly the inherited PR905 cases listed below.

**Dependent crates, run** (not only compiled):

| Crate | Tests |
|---|---|
| `core/runner/headless` | 64 |
| `core/model_operations/operation_applier` | 176 |
| `core/loads/self_weight_wasm` | 14 |

Each ran with its own target, deleted afterwards (`cp3_dependent_*.log`). The counts match the CP2 reviewer's.

**Mutation run** (in place, bytes restored and sha256-verified; `cp3_join_mutations.py`, `cp3_join_mutations.log`, `cp3_join_mutations_rerun_J7_J8.log`).
- All 17 mutants are killed. They cover:
  - the observation lanes M02/M03;
  - offset omitted and offset sign flipped;
  - eigen terms unidentified;
  - prescribed owner zeroed;
  - captured replay skipped;
  - record binding skipped;
  - eigen closure skipped;
  - pair formation unchecked;
  - ownership unchecked;
  - joined semantics not selected;
  - a unit variant restored;
  - quantities open;
  - null treated as absent;
  - request laws ignored;
  - missing section dropped.
- In the first run, J7/J8 were built wrongly: an `&&`/`||` precedence mistake left the check active, so they reported "survived". Both are retained in the log. The corrected mutants are killed.

**ROOT constraint (ii) negative controls** (`load_state_tests.rs`).
- A member pair, an eigenstrain, a prescribed motion and the eigen offset sign are each perturbed after capture in a self-consistent live invocation.
- Captured replay refuses each one ("captured source replay"), and current binding refuses the genuine selection.
- Live mismatches are refused at source closure, with named messages.

**Rustfmt.** All touched modules are rustfmt-clean. `lib.rs`'s 47 pre-existing rustfmt differences are unchanged.

**Inherited, not repaired here:**
- The `numerical_integrity` `--locked` lock issue: fixed upstream in PR905 `5b1ccd356`.
- `test_stress_neutral_physics_source.py::test_method_namespace_removal_substitution_and_relabel_are_rejected[physics|source_blocks]` (SN-CSV-PROFILE-MISMATCH): a PR905 test-sequencing defect that fails at pristine HEAD. ROOT is repairing it in PR905, and this branch inherits the fix on merge.

**Not run:** the piping pytest sweep beyond the named files, desktop, native and browser.

## 4. Unreviewed

Everything in §1, including:

- the join and its receipt;
- the load-reference-source-1 table;
- the witnesses;
- the typed-boundary, null, material-wire and missing-section repairs;
- the READERS and SCHEMAS deliverables;
- ROOT's `package_v0_3.py` edit and the stress-neutral outputs;
- `CP3_WIRE_ADDENDUM.md`;
- the runtime extension.

The CP3 independent review had **not** been dispatched when this record was written. The owner has since authorized it, and ROOT dispatches it on the frozen commit. It must cover the frozen candidate before any of this is relied on or `load-reference-source-1` becomes active.

## 5. Remaining

- **Stress-neutral for load-reference-1: resolved.** See §1.4. `load-reference-source-1` is not accepted by the packager while it is reserved-inactive.
- **Readers for `load-reference-source-1`.** No consumer accepts the joined envelope yet: result_export, analysis_runs and the stress-neutral packager all reject it as unsupported. A sensitive 0.4.0 case now publishes an envelope with no downstream reader until that extension lands, inside this candidate and after review.
- **Held since CP2:**
  - desktop `types.ts`, authoring and operations;
  - native/headless adapters and persistence;
  - legacy `imposed_displacement` mapping;
  - native witnesses.
- **Explicitly open and not claimed:** unchanged from CHECKPOINT_2 (M21, spring base motion and device states, history, bends/joints, coupling, shear, combinations, wider reporting).
- **Record rule.** `CP3_READERS/RETURN.md` lines 178–179 name the interpreter and target by machine path, outside `_run_records/`. The file is TASK-owned, so the manager did not edit it. It is flagged for ROOT and for the CP3 review.
- **Housekeeping.** SCHEMAS' conftest-built helper targets are git-ignored, inside the worktree, and about 78 MB: `core/serialization/canonical_json/target`, `core/units/target`. They are left in place because other lanes use them; ROOT decides.

## 6. Decisions requested

- None blocking.
- The owner has authorized three follow-ons once the current work lands:
  - PR905 to merge;
  - one independent CP3 review on the frozen commit, which ROOT dispatches;
  - the stress-neutral edit, already applied by ROOT.
