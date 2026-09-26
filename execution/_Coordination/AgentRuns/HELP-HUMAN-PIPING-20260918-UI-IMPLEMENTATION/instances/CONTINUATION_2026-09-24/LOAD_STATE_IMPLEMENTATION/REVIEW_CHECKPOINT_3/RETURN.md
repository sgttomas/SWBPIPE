# Independent review: load/reference-state checkpoint 3

**Verdict: FINDINGS.** One should-fix and five notes. Nothing blocking.

I found no way for the retained-source join to accept an operand it did not verify. Several things hold under my own probes and mutants, run on a scratch copy of the frozen bytes:

- the resolver re-derivation;
- bitwise closure of pairs, eigen loads, fold order, offsets and motions;
- captured-replay and current-binding refusal;
- the pressure-region refusal;
- exclusive publication of `load-reference-source-1`.

The CP2 repairs, the readers, the schemas and the stress-neutral edit also hold.

The should-fix is about availability, not wrong numbers. Once any case of a 0.4.0 invocation selects the join, the invocation must finalize that join or it publishes nothing. Checkpoint 2 published those inputs ordinarily. The candidate's records say every unselected case publishes its ordinary response, which is not true in these cases.

## Scope and identity

- **Reviewer.** A Type 2 TASK acting as a fresh-context, non-author reviewer, dispatched by HELP_HUMAN (ROOT) with the owner's approval. I wrote none of the reviewed bytes, delegated nothing and made no Git writes.
- **Candidate.** The detached snapshot `485cc2ed0` on `codex/piping-load-states-20260925`. The review diff is `404cd9c6b..485cc2ed0`, which also contains the CP2 review record `d92ca3b7c`.
- **Basis.** The engineering design at git `9e8a55d`, `CORRECTNESS_DESIGN/LOAD_REFERENCE_STATES/DESIGN.md`, read with `git show`.
- **Governing records read:**
  - `CHECKPOINT_3.md`, `CP2_WIRE_ADDENDUM_2.md` and `CP3_WIRE_ADDENDUM.md`;
  - `_run_records/session2/CP3_ROOT_SELECTIONS.json` and `CHECKS_CHECKPOINT_3.json`;
  - the returns in `CP3_READERS/`, `CP3_SCHEMAS/` and `CP2_RUNTIME_TESTS/EXTENSION_1/`;
  - `REVIEW_CHECKPOINT_2/RETURN.md`.
- **Paths.** Paths are relative to WORKING_ROOT = `projects/chirality-piping/`. `LSI` means `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/LOAD_STATE_IMPLEMENTATION`. Line numbers are at `485cc2ed0`.

**Reviewed product and reader files** (sha256 prefix, numstat +/− over the diff; the full list is in `_run_records/reviewed_numstat.txt`):

| File | sha256 | +/− |
|---|---|---|
| `core/product_physics/src/source_recovery.rs` | `8600cdc1` | 213/5 |
| `core/product_physics/src/lib.rs` (diff) | `7a609823` | 201/84 |
| `core/product_physics/src/source_receipt.rs` | `c47990f8` | 200/13 |
| `core/product_physics/src/source_receipt/composite.rs` | `fa98cd4e` | 115/18 |
| `core/product_physics/src/source_receipt/source.rs` | `7e3bdaa8` | 40/1 |
| `core/product_physics/src/source_receipt/load_state_tests.rs` | `9fa4a47f` | 522/0 |
| `core/product_physics/src/case_state/{input,resolve,mod,tests}.rs` | `75574d2e`, `47d5dcbf`, `17f17f20`, `13372a37` | 87/9, 49/18, 9/1, 295/0 |
| `core/product_physics/tests/load_reference_state_runtime_extension.rs` | `5febacba` | 2147/0 |
| `core/reporting/result_export/src/{load_reference,semantic_contract,derivative}.rs` | `8148c981`, `4e7e1a4b`, `aca9ba57` | 1001/0, 59/8, 3/1 |
| `core/analysis_runs/{load_reference_evidence,compatibility}.py` | `7427bd1d`, `ac779ba2` | 659/0, 26/7 |
| `core/handoff/stress_neutral/package_v0_3.py` | `1ef2d859` | 3/3 |

I also reviewed the four carrier and new schemas, the two new semantic tables, the 14 raw fixtures, the 12 results carriers, and both new Python test files.

## Findings

| # | Severity | Location | Concrete failure scenario | Repair |
|---|---|---|---|---|
| SF-1 | should-fix | `core/product_physics/src/lib.rs:2290–2294`: selection reserves no finalization work.<br>`lib.rs:3338–3345`: a per-case finalization failure is a blocking diagnostic.<br>`lib.rs:1893–1901`, `:1389–1393`: an invocation-level failure gives `Err`.<br>`core/product_physics/src/source_receipt/composite.rs:999`: every case must qualify.<br>Records: `LSI/CHECKPOINT_3.md:43`, `LSI/CP2_WIRE_ADDENDUM_2.md:102`. | **When a case selects the join but the invocation cannot finalize it, the whole invocation loses its results.**<br>(a) **Budget cliff.** Take the committed `eigen_motion` witness and add one declared 1e-6 N tip force.<br>• The join is selected, with 4.06M of 8M work charged.<br>• Captured replay then exhausts the same budget: `Exact(Budget)` at 7.99M.<br>• The case gets a blocking `SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED`, and the envelope is a blocked `load-reference-1` with `MODEL_INCOMPLETE` and no results, in both modes. Adding 2 or 3 forces gives the same result (probe P9).<br>• The cost is driven by the support-state motions. The committed witness selects at 3.36M, and its finalized case charges about 7.34M of the 8M per-case limit (`invocation_work` 7.62M including publication), which leaves about 8% headroom. Without its motions the witness selects at 1.40M (P10).<br>(b) **Composite rule.** Take `eigen_motion`, plus a second sensitive case with a 1e-5 Pa pressure region.<br>• The value route returns `Err("SOURCE_BLOCKS_FINALIZATION_FAILED")` and publishes no envelope (P3).<br>• Pre-0.4 `physics-source-1` behaves the same (P12), so the rule is inherited.<br>• ROOT's accepted semantics define only two outcomes: "no case selects", and "selects and the receipt finalizes".<br>At `404cd9c6b`, both inputs published their ordinary sensitive responses, because no join was attempted. CHECKPOINT_3 says "a case that is not selected publishes its ordinary response", and ADDENDUM_2 §5.2 says a failed attempt "publishes its ordinary response". Neither is true when another case selects, or when the selected case cannot finalize. | Have ROOT select the policy for "selected but not finalizable". Then implement it and pin it with public-route tests for (a) and (b). Two options:<br>• **Fall back.** For 0.4.0, publish `load-reference-1` with every case on its ordinary route: `not_joined` / `retained_source_attempt=unavailable`, plus an info diagnostic naming the finalization cause. This applies only when every ordinary attempt returned a solve. At minimum, select only when the live charge leaves room for the replay (or give the replay its own budget), so that (a) becomes `unavailable` rather than blocked.<br>• **Fail by design.** Record that these invocations fail, and correct CHECKPOINT_3 §1.1 and ADDENDUM_2 §5.2 accordingly.<br>Keep pre-0.4 bytes unchanged either way. |
| N-1 | note | `core/product_physics/src/source_receipt/load_state_tests.rs`<br>`core/product_physics/src/source_recovery.rs:1213` (collision), `:1252` (evidence identity), `:735–748` (motion ownership)<br>`core/product_physics/src/source_receipt/composite.rs:1051` (ordinary-case record) | **Parts of the join's closure are not pinned by any committed test.** My mutants on the frozen bytes:<br>• K5 (identity omits the resolver evidence), K7 (motion-ownership loop removed) and K10 (identity omits the thermal/fit split) survive every committed suite. Only my post-capture perturbations kill them (P5: evidence-only, explicit zero motion, strain split with the same total).<br>• K8 (eigen-ID collision check removed) and K9 (ordinary-case record binding in a joined envelope removed) survive everything, including my probes. They are defence in depth, but a K8 regression would label a user load as `member_eigenstrain` in the source commitment.<br>• No committed test covers any of these, although probes P1, P2, P4, P6, P7 and P11 cover each of them and pass:<br>  – a selected case whose pair differs from the base material;<br>  – two selected cases with different state;<br>  – a joined envelope that contains an ordinary case (the `mixed` witness is fixture-only);<br>  – factor ≠ 1 with an excluded source;<br>  – a parallel spring on a moving rigid DOF;<br>  – the collision refusal;<br>  – a zero-valued pressure region.<br>• The "offset_sign" negative control flips the eigenstrain sign. It is not an offset-only perturbation; the code-level offset flip is killed by the closed-form test. | Add public-route tests equivalent to P1, P2, P4, P6, P7 and P11, plus post-capture controls equivalent to P5. |
| N-2 | note | `core/analysis_runs/load_reference_evidence.py:96–99` (`_is_number` bounds integers by `U64_MAX`) | **Parity break.** Take `pressure-dense_scrutiny.raw.json` and author one member's operating, selection and point temperatures as the integer literal `100000000000000000000`. Python rejects it with `SOURCE_LOAD_REFERENCE_NUMBER_INVALID`, at dispatch and in the validator. Rust (`for_source`, `validate_load_reference_evidence`) accepts it. The same value written as `1e+20` is accepted by both. So the claim that no case is "accepted in one and rejected in the other" does not hold in general. The producer never writes such literals, so the exposure is transported or hand-authored documents. | Treat any finite JSON integer as a number, as physics-1 `_number` and serde_json's f64 fallback do, keeping `U64_MAX` only for indices. Add shared mutation cases for both signs. |
| N-3 | note | `core/reporting/result_export/src/load_reference.rs:880`<br>`core/analysis_runs/load_reference_evidence.py:573` | **Both readers are looser than `CP3_WIRE_ADDENDUM.md` §1 for law segments.** `{use: integration_interval, lower_index: 0, upper_index: 3, start_k: 300, end_k: 300}` is accepted, but the addendum fixes `upper_index = lower_index + 1` and `start_k < end_k` for an interval. Duplicate entries are also accepted. The addendum post-dates the READERS freeze. | Tighten both readers identically, with shared mutation cases, or record the laxity. |
| N-4 | note | `LSI/CHECKPOINT_3.md:122`, `:293`, `:236–237`<br>`LSI/_run_records/session2/CHECKS_CHECKPOINT_3.json:164`<br>`LSI/_run_records/session2/cp3_mutation_prehash.txt` | **Some record statements are inaccurate.**<br>• CHECKPOINT_3 cites `CP3_READERS/RETURN.md` as `75f6f0a0…`, but the frozen file is `57f12bcb…`.<br>• The "Record rule" item says lines 178–179 still carry machine paths. They no longer do: they now point to `RR/environment.json`. No record says who edited this TASK-owned return, or when.<br>• CHECKS cites `CP3_ROOT_SELECTIONS.json` as `e6b20097…`, but the file is `767f0e99…`.<br>• The 17-mutant run used pre-final bytes: 4 of its 6 prehashed files differ from the candidate. This is not disclosed. I reran all 17 on the frozen bytes, and all 17 are killed (below). | Correct these in the next checkpoint record or addendum. Do not rewrite the history. |
| N-5 | note | `core/reporting/result_export/src/load_reference.rs`, `src/semantic_contract.rs`, `tests/load_reference_contract.rs` | **The new result_export code is not rustfmt-clean:** `rustfmt --check` reports 15, 4 new and 18 diff hunks. CHECKPOINT_3 says no pass was possible because the toolchain lacks the component. But the stable rustfmt 1.8.0 that the manager used for product_physics is on this machine. | Run rustfmt on the new result_export files, or record the exception. |

## What I verified, and how

Commands and raw logs are in `LSI/REVIEW_CHECKPOINT_3/_run_records/`, indexed by `RUNS.json`.

### 1. Snapshot and suites

- **Snapshot.** The prehash (`prehash_snapshot.txt`, 828 files) was taken before any run, and the posthash is equal. `git status` shows only this directory as new.
- **product_physics.** `cargo +1.97.1 test --locked --offline -j 1` gave:
  - 402 passed, 1 pre-existing ignored;
  - lib 311, runtime 20, extension 9, other binaries unchanged;
  - warnings identical by name to `cp2_gate2_all_tests.log`;
  - the log is `snapshot_product_physics_tests.log`.
  - The manager's final-gate prehash (74 files) matches the frozen bytes.
- **result_export.** 64/64 (`snapshot_result_export_tests.log`).
- **Python.** The four named files gave 867 passed and 1 skipped: 213 + 527 + 80 + 48 collected. The skip is the optional parity recorder. The log is `python_named_tests.log`. Helper binaries were built outside the snapshot, and bytecode and cache writes were disabled.

### 2. Retained-source join (brief item 1), by code reading plus probes P1–P12 (`review3_probes.rs.txt`, `probes_final.log`)

**Same resolver.**
- `captured_load_state_case` runs `validate_profile`, `validate_document`, `resolve_shared_sections`, `normalize_model_units` and `resolve_case` as the product does.
- It then runs `build_model_for_members(Some(pairs))`.
- The product-only validators take `&model`, so the state before resolution is identical.
- Mutant K1 (replay builds with base materials) is killed by the committed tests.

**Closure.** `close_load_state` checks the following bitwise:
- pair E, ν and G against the owned pair and against the frame E and G;
- frame area = section area = exact wall area;
- every eigen load = `E·A·ε*`, in member order, with no extras.

The fold mirrors `add_thermal_equivalent_loads` after the nodal fold. The full folded force and stiffness are compared bitwise with the actual aggregates. The offsets are `+a` at end i, row 0, and `−a` at end j, row 6, and sections inherit them through `scaled_row`. Support actions subtract every identified force term. The motions must match the unique rigid owner's expected value, and the partition must be complete.

Closed-form checks:

| Probe | Result |
|---|---|
| P1 (pair E 150 GPa ≠ base) | N matches E·A(δ/L − ε*) to 5e-13 |
| P11 (joined against the ordinary route) | every shared non-torsion row agrees to ≤ 1.1e-12 relative, both modes, for `eigen_motion`, the exact-point pair, factor 2.5 with an excluded 7 kN source (root RX = 2.5·T/k, excluded force absent), and a parallel spring on the moving root UY (spring −k·g = −1 N, anchor +1 N) |
| P2 (two selected cases with different pairs and motions) | both qualify; tip UY 1.2 / 2.2 mm |

**Refusal after capture.**
- The four implementer controls pass.
- P5 adds five more, each refused by captured replay: a strain split with the same total, evidence only, an explicit zero motion, an effective primitive magnitude and ν only.
- Current binding is covered by the implementer test, and the J-series mutants confirm it.

**Old route kept apart.**
- `check_input_with_physical` dispatches 0.4.0 to `check_load_state_input`.
- `finalize` (SOURCE-BLOCKS-1) refuses 0.4.0.
- The policy is selected from the captured model.
- Pre-0.4 payloads and proofs are unchanged when `load_state` is `None`. The pre-0.4 `physics_source_runtime` passes, and so does its companion comparison.

**`load-reference-source-1` only when qualified.**
- The label is set only with `source_selected`.
- The value route returns `Err` if the receipt is missing.
- `validate_publication` requires every case to be qualified.
- A per-case finalization failure gives a blocked `load-reference-1` (see SF-1).

**Pressure refused.**
- A non-empty region is refused explicitly (`source_recovery.rs:460`), including a zero-valued one (P4 gives `unavailable`).
- K6, which removes that check, is killed by an existing test and by P4.

**Collision refused.** A primitive load named `load_state_eigenstrain:6:member` gives `unavailable` (P6).

### 3. Mutants on the frozen bytes

Each mutant ran on a scratch copy, with bytes restored and sha256-verified each time. Counts are for the first failing test binary.
- Reviewer mutants K1–K12: `review3_mutations.{py,log}`.
- The implementer's 17 mutants, with anchors verbatim: `rerun_implementer_mutants.{py,log}`.
- The CP2 reviewer's N-4 targets against the extension: `rerun_cp2_n4_mutants.{py,log}`.

| Result | Mutants |
|---|---|
| Killed by committed tests | K1 replay with base materials; K2 replay motions zero; K3 replay omits eigen loads; K4 live eigen fold sign; K6 pressure check removed; all 17 implementer mutants (M02/M03, J1–J10, SF-A ×2, N-2 ×2, N-5) |
| Killed only by my probes | K5 identity without resolver evidence; K7 motion-ownership loop removed; K10 identity without the strain split |
| Survive (N-1) | K8 collision check; K9 ordinary-case record binding; K11 identity without local axis and K12 frame-area check (both equivalent in practice: operands are bound elsewhere) |
| EXTENSION_1 kill targets, now demonstrated (the TASK had only argued them) | The extension binary alone kills all 10 CP2 mutants, each by its named test. That includes M14, M17 and M24, which survived every suite at CP2. The mutants are M04 (re-anchored), M06, M14, M15, M16, M17, M19 (re-anchored), M22, M23 and M24. |

### 4. CP2 repairs (brief item 2), against REVIEW_CHECKPOINT_2's scenarios

- **SF-A.**
  - Every p03 case is now a typed `unknown field` refusal through the public route: fit `none` + −2 mm, direct basis + temperature, independent history + predecessor, active device + components, forged and actual geometry hash, and an extra key on a quantity.
  - `inactive` is covered by the DTO test.
  - Every namespace `Quantity` field uses `closed_quantity`; I checked each by grep. The remaining unit variants are plain string enums.
  - The shared pre-0.4 `Quantity` stays open, as recorded.
- **N-1.** Dense parity ≤ 1e-12 with the lane required. M02/M03 are killed on the frozen bytes.
- **N-2.** `Authored<T>` distinguishes absent, null and value, and a null counts as carrying the key.
  - 0.3.0 null or present keys, including request-level `expansion_laws`, give `VERSION_MISMATCH`.
  - A 0.4.0 null gives `EXPLICIT_NULL_UNSUPPORTED`.
  - A malformed legacy key gives a typed `Err`.
  - Both N-2 mutants are killed.
- **N-3.** `MaterialRecordWire` flattens `MaterialInput`, which has no `deny_unknown_fields`, so flatten is admissible. A duplicate `elastic_modulus` is a `duplicate field` error with a position.
- **N-5.** `load_state_eigen_loads` returns `Err(pipe)`. The product blocks with `LOAD_STATE_MEMBER_SECTION_MISSING`, replay refuses, and no `expect` is used. The N-5 mutant is killed.

### 5. Readers and schemas (brief item 3)

- **Parity.** Both suites assert the shared case file, 201 IDs.
- **Real producer outputs**, run through both readers (`review3_reader_probe.rs.txt` and the Python probe in `probes_final.log`):
  - the 4 fixtures and a regenerated unavailable-join envelope, which carries `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` and `retained_source_attempt=unavailable`: accepted by both;
  - joined `eigen_motion` and `mixed`: refused by both, with `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` and `…FOREIGN_METHOD_EVIDENCE`;
  - the exception is N-2.
- **Physics-1 reuse.** S14 runs the unchanged physics-1 validator on a projection that removes only fields already bound. I found no physics-1 check that assumes one common E.
- **Changed physics-1 error code.** It applies only to inputs that were already refused. No consumer or test outside the new files references `load_reference_states`.
- **Schema additions are append-only.** The pointer diff (`review3_static_checks.log`) shows only list appends: 5, 4 and 5 of them. Every `ResultEnvelope`, `AnalysisRun` and stress-neutral branch pins its producer ID and profile, and `producer` and `formulation_basis` are required. Old documents therefore match the same branch as before.
- **Tables.**
  - The precision-1, physics-1, source-blocks-1 and physics-source-1 tables are byte-unchanged.
  - The load-reference-1 table is `44bc41c0…`, as pinned in both readers.
  - `load-reference-source-1` (`d1628194…`) differs from physics-source-1 only in its ID, profile, policies, limitations and inherited contracts. Its rows are identical.

### 6. Stress-neutral edit (brief item 4)

- **The edit is minimal.** The diff is exactly the import and the two set members. `load-reference-source-1` is absent, and `RECEIPT_METHODS` is unchanged.
- **The packages are valid.** Rerunning `cp3_stress_neutral_outputs.py --check` reproduces all four packages byte-exactly (`e6a3be75`, `0c67b958`, `cf95c609`, `a44f8116`). It covers schema validation, the validator (alone and with source and analysis), the nine-member round trip and physics-1 relabel refusal. The log is `stress_neutral_outputs_check_rerun.log`.

### 7. Wire and records (brief item 5)

- **Producer output.** All 14 raw envelopes (4 `load_reference`, 10 `load_reference_source`) regenerate byte-identically from the frozen producer (`regen_compare.log`).
- **Regeneration claims.** `cp3_regen_diff_check.py` reproduces the four frozen hashes, and `cp3_join_companion_compare.py` gives ALL_MATCH.
- **ADDENDUM_2 §5.**
  - §5.3: the NOT_JOINED text and the record variants are as implemented.
  - §5.4: the receipt `members`, `eigen_loads` and `prescribed_motions` keys, the `member_eigenstrain` owner, the evidence namespace and the per-case domain are as implemented.
  - The exceptions are covered by SF-1.
- **CP3_WIRE_ADDENDUM.** It matches `thermal.rs`:
  - `Consumption::segment` records both endpoints;
  - `dilation_difference` records the consumed `integration_interval` pieces and returns early when T_i = T;
  - `check_path(Dilation)` records only samples;
  - secant stationary segments are consulted intervals;
  - the integral laws record a sample at each piece end.

  The dilation selection is coherent. Δd as a sum of segment increments equals d(T) − d(T_i) exactly in real arithmetic. Labelling it `integration_interval` is consistent with the §1 definition, and test 7 asserts the addendum's sets exactly.
- **CHECKPOINT_3 claims.** These match the logs: counts, hashes (except N-4), dependent-crate counts (64/176/14 in the manager's logs) and the J7/J8 history.

### 8. Tests (brief item 6)

- **Implementer tests.** The new tests use closed forms or fixture-derived expectations at the protected 1e-9, with no loosened criterion.
- **Extension test 8.** It pins the spring path for tip RX, not GJ, as the TASK disclosed.
- **Implementer rustfmt claim.** The touched product_physics modules are rustfmt-clean with stable rustfmt 1.8.0.

## Limits

- **Not run:**
  - the dependent crates (headless, operation_applier, self_weight_wasm), because they were outside the permitted commands; the manager's logs were read instead;
  - `tests/test_stress_neutral_physics_source.py` and the wider pytest sweep;
  - the DEC-025 sweep;
  - the desktop, native and browser lanes.
- **Not reviewed:** the CP1/CP2-reviewed kernels outside the diff, except where the join consumes them.
- **Writes and cleanup.**
  - Probes and mutants used scratch copies only, outside the snapshot.
  - Reviewed bytes were never edited, and the snapshot changed only inside `REVIEW_CHECKPOINT_3/`.
  - Scratch copies and cargo targets were deleted afterwards (`RUNS.json`).
- **Status.** This review closes no M10, M16 or M29 finding. It is not acceptance, and it does not by itself activate `load-reference-source-1`.
