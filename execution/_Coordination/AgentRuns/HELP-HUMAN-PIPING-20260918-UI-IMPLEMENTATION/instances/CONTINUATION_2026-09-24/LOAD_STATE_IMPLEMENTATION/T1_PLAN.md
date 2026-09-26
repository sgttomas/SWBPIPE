# T1 plan — connected remainder for load and reference states (M10, M16, M29)

- **Author:** the session-3 T1 WORKING_ITEMS manager, parent HELP_HUMAN (ROOT).
- **Status:** a prepared plan for ROOT. **Nothing in it has started.** It is an ad hoc plan, not a reusable or accepted workflow, and it grants no scope.
- **Basis:**
  - the work graph's "Current route" row T1;
  - the reviewed design at Git `9e8a55d`, `CORRECTNESS_DESIGN/LOAD_REFERENCE_STATES/` (`DESIGN.md` §7–8 and `INTERFACE.md`, "Authoring and persistence acceptance");
  - `CHECKPOINT_1–4.md` on this branch.
- **Owner direction applied:** solver and validation work lead. UI work stays on this route only where T1 needs it to reach or persist the capability. Layout and visual work goes to `UI-SUCCESSOR`.
- **Paths:** WORKING_ROOT-relative. `LSI` is this records folder.

## 1. Where T1 stands

**Done on the branch and reviewed at checkpoints 1–3.** CHECKPOINT_4 adds the SF-1 and N-1–N-5 repairs, which are under review.

- The 0.4.0 resolved-case product route for M10, M16 and M29:
  - nonzero support motion;
  - per-member E/ν/G;
  - thermal, reference and fit eigenstrain;
  - the source ledger.
- The retained-source join, with the SF-1 fallback.
- The `load-reference-1` semantic table.
- Readers in `core/reporting/result_export` and `core/analysis_runs`.
- Schemas for the results, AnalysisRun and stress-neutral carriers.
- Stress-neutral packaging for `load-reference-1`.
- Independent analytical references (`core/product_physics/tests/fixtures/load_reference_states/reference_cases.json`, 13 cases).

**Not done: the connected remainder.** No user can yet author, persist or reopen a 0.4.0 model in the application:

- desktop `types.ts` has no 0.4.0 fields;
- native migration refuses `schema_version` 0.4.0 ("newer than the legacy migration target");
- the typed operations cannot edit the new records.

Other gaps:

- The headless runner's 0.4.0 path is not specifically tested.
- No consumer accepts the joined `load-reference-source-1` envelope.
- There is no validation-harness adapter or VP-STATIC case for this profile.

No M10, M16 or M29 finding closes until this connected path passes on a merged candidate, including its native witnesses (graph closure rule).

## 2. Work packages

Each package lists its write set. The sets are disjoint so TASKs can run concurrently. I remain the integration owner and the sole writer of `core/product_physics/src/lib.rs`. Most packages need write scope beyond my current boundary; §5 lists the requests.

### WP1 — joined-envelope readers (depends on decision D1)

If ROOT activates `load-reference-source-1` in the T1 PR:

- extend the Rust and Python readers to accept it exclusively with profile `resolved_straight_load_state_source_v1` and policy `LOAD-REFERENCE-SOURCE-1`;
- give it a `contract_evidence` namespace of exactly `{pressure, connector, exact_cases, load_reference_states}`;
- add additive schema branches in the results, AnalysisRun and stress-neutral 0.3 carriers;
- add it to the packager's method sets;
- create shared mutation cases from the ten committed joined raws and from the SF-1 fallback outputs.

Pre-existing tables, bytes and branches stay unchanged.

- **Writers:** one TASK for the readers (`core/reporting/result_export`, `core/analysis_runs`), and one TASK for the schemas and carriers (`schemas/`, `fixtures/results/`, `tests/test_load_reference_*`).
- **Packager edit:** the one-line edit to `core/handoff/stress_neutral/package_v0_3.py` was refused by the host classifier at CP3 and needed explicit owner approval. It needs that approval again (§4, D1).

### WP2 — connected model types and persistence

**Desktop `apps/desktop/src/types.ts`.** Add lossless types for the 0.4.0 model additions:

- `schema_version` "0.4.0";
- `reference_configurations`;
- `materials[].expansion_laws`;
- `load_cases[].analysis_state`, with every closed variant.

Also add the result evidence `contract_evidence.load_reference_states`. There must be no lossy DTO projection: unknown or absent keys round-trip exactly. Tests: vitest round-trip and projection tests.

**Native migration and persistence (`apps/desktop/src-tauri/src/model_document_migration.rs` and the project persistence path).**

- Accept 0.4.0 exactly as 0.3.0 is accepted today: retained without migration, with no down-migration, no invented coefficient, datum, fit or predecessor, and no whole-model serde error for unsupported legacy records.
- Save the authored record, and keep the raw result hash and resolved evidence with the saved result.
- Reopen must not show a result as Current when its reference basis has changed.
- Tests: Rust unit tests in `src-tauri`. The DEC-025 cargo sweep does not cover `src-tauri`, and this host cannot build it (§6).

**Portable schemas.** Add additive 0.4.0 model-document coverage in `schemas/model.schema.yaml` and `schemas/project_persistence.schema.yaml`, if those files validate model bytes.

**Legacy mapping.** The legacy `imposed_displacement` primitive already blocks in 0.4.0. Mapping it into `support_states[].boundary_motion` needs a user-chosen owning rigid DOF, so there is no automatic mapping (§4, D3).

### WP3 — minimum authoring inputs

**Typed operations (`core/model_operations/operation_applier`).** Following the `pressure_authoring.rs` pattern, add closed operations for:

- a reference configuration and its member references (basis, fit);
- material `expansion_laws`;
- per-case `analysis_state`: element states, support states with `boundary_motion`, and `load_sources` with factors.

These go through the existing Review/Apply, Undo/Redo and atomic-batch seam. Validation mirrors the product's closed wire: unknown fields and explicit null are refused as CP2_WIRE_ADDENDUM_2 §1–2 define. The operations add no defaults and no library values; every quantity is user-entered.

- Tests: Rust tests in the crate.

**Minimal native input (D2).** Plain fields in the existing Load Case Manager and Materials panels, so that a human can enter:

- support motion;
- element thermal and fit strain, or temperatures and law selection;
- material selection.

They submit through the operations above. A read-only "resolved state" block in the Solve result shows the published `load_reference_states` record and the retained-source standing. No layout or visual work.

- Tests: vitest, plus Chromium Playwright on the browser fixture.

### WP4 — headless and CLI adapters (`core/runner/headless`)

Tests of the 0.4.0 path through `openpipestress-runner solve` and the library value route, in both modes:

- the `ControlledExport` wrapper, and the canonical results document built by `result_envelope_binding` for `load-reference-1` (and for `load-reference-source-1` if D1 activates it);
- `QualifiedPreviewEvidence`;
- refusal controls: explicit null, an unknown field, a pre-0.4 document carrying 0.4.0 keys;
- the SF-1 fallback through the runner (the budget-cliff request from `load_state_fallback_tests.rs`).

No product semantics change. If the binding needs a seam, it returns to me.

### WP5 — VP-HARNESS `load-reference-1` adapter

Add `tools/validation/qualification_load_reference.py`, mirroring the closed physics-1 adapter (`qualification_physics.py`). It pins:

- the transport, contract, profile and table sha256 (`44bc41c0…`);
- the reader module hash (`core/analysis_runs/load_reference_evidence.py`);
- the units hash.

It resolves every required selector exactly once, including the `load_reference_states` member, support and contribution records. It keeps the refusal denominator: missing, duplicate, nonfinite and zero-case inputs, and interrupted runs, fail correctly. It reports standing (`checks_passed` against `sensitive`) separately from outcomes.

- **Tests:** focused pytest with seeded faults.
- **Joined profile:** only if D1 activates it, under its own adapter identity.

### WP6 — VP-STATIC cases from the existing ANALYTICAL_REFERENCE

For each family:

1. Author invented production runner inputs, using the OD 0.20 m / wall 0.01 m companions that the references provide.
2. Write the selectors and criteria.
3. Take the expected values from `reference_cases.json` at test time. The criterion is the existing protected `|observed − expected| ≤ 1e-9·|expected|`, with the existing zero-reference handling. No new or relaxed threshold.

| Family | Reference cases |
|---|---|
| Support motion (M10) | `prescribed_translation_two_bar`, `prescribed_translation_all_fixed`, `prescribed_rotation_all_fixed`, `prescribed_rotation_free_tip` |
| Reference temperatures (M16) | `thermal_datum_ratio`, `coefficient_definition`, `constant_alpha_interval`, `shared_material_parallel` (analytical topology only; the practical witness is `shared_material_serial_companion`), `multi_segment_free_length`, `temperature_unit_identity` |
| Cold spring (M29) | `signed_fit_states`, `persistent_source_once` |

The references' `wrong_result_discriminators` become negative assertions. Every case checks all six reactions and the governing locations, not only one scalar.

**Freeze first.** An independent non-author TASK checks the inputs, selectors and criteria against the references before any comparison runs. A mismatch returns a repair to the product owner. It never changes a reference, fixture or tolerance.

- **Write set:** `validation/qualification/fixtures/load_reference/` and the harness tests.

### WP7 — qualification of the mergeable PR

For the actual candidate revision:

- the full product_physics suite and the dependent crates;
- the piping pytest sweep;
- desktop vitest and build (if `apps/desktop` is touched);
- the Chromium e2e;
- hosted CI, including the manual full dual-viewport dispatch of `piping-desktop-e2e.yml` with a full 40-character `target_base` (the DEC-093 surface-4 binding);
- a clean five-surface DEC-025 sweep;
- the D-GOV-45 leak scan;
- fresh-context independent review of the complete frozen diff, with the SF-1 fallback in scope as ROOT asked;
- the native witnesses (§6).

ROOT owns Git, the PR, CI dispatch and merge. Then reassess M10, M16 and M29 closure against the merged candidate under the graph's closure rule.

## 3. Sequence

1. Now: CHECKPOINT_4 independent review, then D1–D4 decided by ROOT or the owner.
2. **Batch A, concurrent with disjoint files:**
   - WP1 (if D1);
   - WP2 types and schemas;
   - WP3 operations;
   - WP4;
   - WP5, the harness adapter itself, before any case binding.
3. **Batch B:**
   - WP2 native persistence and WP3 minimal native input, which need WP2 types and WP3 operations;
   - WP6 freeze, which needs WP5 and the fixtures.
4. WP6 comparisons on the integrated candidate, then an integration checkpoint (CHECKPOINT_5) with its own independent review.
5. The owner's Mac native witnesses on the frozen candidate. ROOT schedules them; they are shared resources.
6. WP7 qualification and the PR.

Each batch ends with an integration commit, targeted checks and a checkpoint record. Expensive checks (the DEC-025 sweep, hosted dual-viewport CI) run once, on the final candidate.

## 4. Decisions needed

- **D1 (ROOT): activate `load-reference-source-1` in the T1 PR?**
  - *Recommendation: yes*, after its readers pass the same independent review. The brief says retained-source recovery for this family is required within the undertaking.
  - Without activation, a sensitive 0.4.0 case that selects the join produces an envelope that no downstream reader accepts. The app would then show no consumable result for exactly the cases the join exists for.
  - Activation also needs the owner's approval for the stress-neutral packager edit, which the host classifier refused at CP3.
- **D2 (HELPS_HUMANS, then the owner): the minimum authoring surface.**
  - *Proposal:* the closed operations (WP3), plain native input fields for support motion, element thermal/fit state and material selection, and a read-only resolved-state block. Tables, layout and visual design stay in `UI-SUCCESSOR`.
  - *Alternative:* operations and headless/agent authoring only, with a read-only native display. This is cheaper, but it cannot produce the T1 row's "native witnesses of the actual authoring … path" for a human.
- **D3 (owner): pre-0.4 models and legacy `imposed_displacement`.**
  - Is an explicit, user-invoked "upgrade to 0.4.0" operation in T1 scope?
  - It could invent nothing: every case would need user-entered state, and each imposed primitive would need a user-chosen owning rigid DOF.
  - Otherwise, T1 limits 0.4.0 to new models and records upgrade as open.
  - Before deciding, I will have a TASK establish what pre-0.4 solves do with `imposed_displacement` today. If they silently drop it, changing that alters a pre-0.4 meaning, which needs the owner.
- **D4 (ROOT): write scope.** See §5.

No decision here reverses a protected criterion, reference or accepted ruling.

## 5. Scope requests to ROOT

These are beyond my current boundary. Each is needed only by the WP named:

| Path | Needed by |
|---|---|
| `core/handoff/stress_neutral/package_v0_3.py` (owner approval) | WP1 |
| `apps/desktop/src/**` (types, the two panels, result interpretation, their tests) | WP2, WP3 |
| `apps/desktop/src-tauri/src/model_document_migration.rs`, persistence and IPC files, and tests | WP2 |
| `core/model_operations/operation_applier/**` | WP3 |
| `core/runner/headless/**` | WP4 |
| `tools/validation/qualification_load_reference.py` and its tests | WP5 |
| `validation/qualification/fixtures/load_reference/**` | WP6 |

Records, graph, Git, CI and native scheduling stay with ROOT.

## 6. Evidence limits on this host

- **Here (Linux).**
  - Available: all Rust crates under `core/` and `validation/benchmarks`, pytest, desktop vitest, the desktop production build, and Chromium Playwright on the browser fixture.
  - These give headless evidence for the product route, the operations, the headless runner, the harness and VP-STATIC comparisons, and browser evidence for the TypeScript input and projection code.
- **Not here.** This Linux host cannot build or run the macOS desktop application. Browser evidence does not substitute for native evidence.
- **Correction (checkpoint 7).** ROOT installed the WebKitGTK development libraries, so `apps/desktop/src-tauri` does compile and its suite runs here (114/114 on the candidate). The DEC-025 cargo sweep still does not discover `src-tauri`, so it is run separately. This Linux run does not replace native witness 1 on the owner's Mac.
- **Needed on the owner's Mac**, on the frozen candidate executable:
  1. `src-tauri` unit tests for 0.4.0 migration and persistence;
  2. open a headlessly authored 0.4.0 model, with inputs shown and retained byte-exactly;
  3. author support motion, element thermal/fit state and material selection through the native inputs; Review/Apply; Undo/Redo restores the document bytes;
  4. solve in both modes, with honest standing: a sensitive result is not promoted, and the joined or ordinary route is labelled;
  5. save, close and reopen, with the document and result standing preserved; a changed reference makes the result not Current;
  6. a pre-0.4 and a 0.3.0 model open unchanged, with no invented state;
  7. refusals (explicit null, unknown field) give targeted diagnostics while the model stays editable;
  8. the SF-1 budget-cliff model shows the ordinary route, not a blocked result.

Until these witnesses run on the actual candidate, they remain outstanding. The T1 PR may still merge as source integration if ROOT and the owner accept that, but M10, M16 and M29 do not close.

## 7. Not in T1

These stay open and are not claimed:

- M21 (hydrostatic head, contents and hydrotest: T2);
- spring base motion, device preload and lock states (T5);
- nonlinear and installation history;
- bends and joints;
- combinations (T6);
- the inherited physics-source-1 composite `SOURCE_BLOCKS_FINALIZATION_FAILED` finding (T3).
