# B7: one reader, the v3 retained-source join and the legacy removals

This is the governing plan for B7. Two of its legacy items were done ahead of it, in the pre-B7 batch: E13 and the Rust reader's namespace rename (see B7a). The rest is not implemented yet. It lands in this order:

1. **Unit R**, the one-reader consolidation, as its own PR ahead of B7.
2. **The join → B7a → B7b**, as stacked PRs. Each is reviewed; the three merge to main as one batch. Until that batch merges, main keeps the named "retained-source recovery not joined" diagnostic on v3.

This file lives with DEL-05-02 because DEL-05-02 owns the retained readers and the shared corpus (`retained-combinations.md`; `deliverable.yaml` points to the three readers). The retained-combinations contract stays in its own file.

## Decisions applied

The owner, 2026-10-10:

- **Direction:** "nothing needs to be preserved for backwards compatibility. I want the best solver engine and I want clean code … if the right solution is to rewrite something, then do it, don't hold onto legacy code that requires workarounds or isn't consistent with the newer code and methods."
- **One current version per format:** "as you recommended, no-compatibility principle carries."
  - Model documents, analysis-run records and stress-neutral exports each keep only their current version.
  - Older versions are refused by name ("re-author" or "re-export"). No migration code is kept.
  - This reverses the 2026-10-08 decision that kept model formats 0.1.0/0.2.0 as the pressure-free namespace. The end state is one model format that expresses models with or without pressure.
- **One product reader:** "move to one product reader in Rust."

**T3's application of the owner's principle, pending HELP_HUMAN's confirmation** (not the owner's decision): result exports (0.1/0.2 beside 0.3.0) and the mechanics-result 0.1.0 carrier also keep only their current version. The `.opsproj` package's single-version reader is the intended direction for DEC-028's open item, not a commitment.

HELP_HUMAN, 2026-10-10, on the v3 retained-source join: D1, D2 (inside B7, no constant-substitution shim), D3, D4 (a), D5, D6 and D7, stated below where they apply.

**Terms.**
- *Reviewed inputs* are `REVIEWED_INPUTS` in `core/product_physics/src/build_identity.rs` (17 files) and their digests in the registered profile (`retained_memory.rs`).
- *Priced* means a layout or TEXT entry of the registered memory profile.
- *The corpus* is:
  - `fixtures/results/retained_precision_cases.json` (07m/07n: 26 bases, 534 mutations, 78 must-pass);
  - `retained_precision_cases_07o.json` (19 bases, 69 mutations, 19 must-pass).
- *Re-registration* follows **D3**. It is approved on the join head and on the B7 heads if M stays 10.5 GiB (11,274,289,152 bytes) and the 5 % text budget holds. Otherwise it returns to HELP_HUMAN for a ruling. Any change of M or of the budget model is decision 22, which the owner holds.

## Unit R: one product reader

**The arrangement.** One product reader replaces the three readers held at parity:
- **Rust, the only product reader.** `core/reporting/result_export/src/retained_precision.rs` reads retained-precision successors natively, and compiled to wasm for the desktop.
- **TypeScript, deleted.** `apps/desktop/src/features/results/retainedPrecision.ts` goes.
- **Python, a test-only verifier.** `core/analysis_runs/retained_precision.py` becomes an independent verifier that only tests use, never a product path.

As a result, B7's retained-reader table, schema and reader changes are made in two places, not three. Base-route readers are outside this unless R is extended (see "Scope" below).

**Bytes and registration.** R moves no reviewed input, priced layout, table, successor byte or corpus base, so it needs no re-registration. If it needs a Rust-reader change that moves any of them, it joins the batch and re-registers under D3.

**Wasm.**
- **Where the bindings go.** They are added to the desktop's existing wasm engine crate, `core/model_operations/operation_applier` (`npm run build:wasm`). That crate already resolves `open_pipe_stress_result_export` through product_physics.
  - result_export gains no dependency, so PP's `Cargo.lock`, a reviewed input, does not move.
- **What they expose.** `retained_precision::validate` and `validate_transport_metadata`, JSON in. Out comes the `Validation`, or the error's gate, code and detail.
- **How the desktop loads them.** Through `services/wasmEngine/loadWasmEngine`.

**TypeScript call sites that change** (under `apps/desktop/src/`):
- These move to the wasm binding:
  - `features/results/retainedPrecisionStanding.ts`;
  - `retainedPrecisionDisclosure.ts`;
  - `numericalResultQuality.ts` (`validateRetainedPrecisionTransport`, `RETAINED_METHOD`);
  - `HistoricalRunContext.tsx`;
  - `services/analysisRunCompatibility.ts`;
  - `features/stress-neutral/StressNeutralExportPanel.tsx`.
- `knownSemanticLimitations.ts` and the modules above import `decodeBinary64`, `binary64Bits` and the reader's types. These move to a small module that holds no validation.

**Vitest tests that change.**
- **Deleted:** `features/results/retainedPrecision.test.ts`, the reader-internal tests (`phi512`, `eHat`, `stopFeasible`, `nativeSchedule`, `accountingRules`, …). The Rust tests and the verifier cover these.
- **Re-pointed at the wasm reader:**
  - `retainedPrecisionIntegration.test.tsx`;
  - `retainedPrecisionOutputRefusal.test.tsx`;
  - `outputPolicy.test.ts`;
  - `result-export/retainedPrecisionResultExport.test.tsx`;
  - `stress-neutral/retainedPrecisionStressNeutral.test.tsx`;
  - `stress-neutral/StressNeutralExportPanel.test.tsx`;
  - `services/retainedPrecisionAnalysisRun.test.ts`.

**The Python verifier.**
- **Location.** It moves under `tests/`. A test asserts that no module outside `tests/` imports it.
- **Product callers.** `core/analysis_runs/compatibility.py`'s `_retained_validation` and `_retained_transport` stop importing it.
  - Python product code that meets a retained successor takes the Rust reader's verdict. It gets it through an `openpipestress-runner` subcommand that R adds over the same two functions.
  - It never re-derives a verdict.
- **Independence.** It shares no code with the producer.
  - Today it hashes through `core/serialization/canonical_json/adapter.py` and converts units through `core/units/adapter.py`. Both are subprocess bridges to crates the producer links.
  - R replaces both with Python implementations under `tests/`. These are checked against those crates' committed vectors, not by calling them.

**The shared conformance corpus.** These become the conformance corpus shared by the Rust reader and the Python verifier:
- the 07m/07n/07o corpora;
- `retained_precision_b2_parity_probes.json` and `retained_precision_carrier_cases.json`;
- the successor fixtures `retained_precision_*_successor_*.json`;
- `retained_precision_rv120_b3_inputs.json`.

Both run every entry unmodified. `expected_by_reader` and `expected_unbound_by_reader` lose their `typescript` member.

**What is gated, and how equivalence is shown.** Deleting the TS reader is gated on an equivalence test. It runs on R's head while both readers are still present.
1. **Wasm matches native Rust.** This covers every corpus entry, including its mutation, must-pass, transport (`expected_transport`) and unbound (`expected_unbound`) forms. Through wasm, each gives the same outcome as the native Rust reader: a pass with an identical `Validation`, or the same gate, code and detail.
2. **Wasm matches the TS reader, except where declared.** The exceptions are 47 entries (46 in 07m/07n, 1 in 07o) with 92 expectations, where the TypeScript expectation differs from Rust's: Rust reports finer base codes at G7. There the desktop takes Rust's outcome, and the change is declared.
3. **Every desktop gate code gives the same outcome through wasm.** These are the codes the standing, disclosure, quality, analysis-run, historical-run, result-export and stress-neutral consumers receive, run in the vitest suites above. The only changes allowed are item 2's.

A difference outside item 2 stops the deletion.

**Scope.** R covers the retained reader. The TypeScript and Python readers of the base routes stay three-way after R. Those routes are preview-physics, physics, physics-source, load-reference, source-blocks evidence and standing. Whether R extends to them is open (see "Open: needs a ruling").

## The join: `pressure-source-1`

**Preconditions:**
- T4's #1233 and #1234 are merged. v3 and `PressureContract` exist only there.
- T3's SQ2, TEXT-chain and statics tools are runnable on current heads.

**Identity (D1).** The reserved identity is `openpipestress.result_semantics/0.3.0/pressure-source-1`:
- receipt policy `PRESSURE-SOURCE-1`;
- case-evidence domain `pressure_source_case_evidence_v1`;
- one identity for model 0.3.0 and 0.4.0;
- profile `exact_pressure_v3`, shared with `pressure-1`.

It is published whenever any case of a v3 invocation publishes a selected retained-source response.
- **Unchanged:** the method's hash domains, the recovery token `retained_source_blocks_exact_v1` and the work limits.
- **Source family:** exactly v2's:
  - straight members;
  - linear restraints and springs;
  - nodal force and moment loads;
  - explicitly empty `pressure_regions`;
  - 0.4.0 eigen element loads and prescribed motions;
  - at most 256 DOFs.
- **Declined by name**, each with its existing decline:
  - pressure regions of any sign, zero included;
  - components (connectors, realized bends);
  - nonlinear supports;
  - non-nodal loads.

**D2: no shim.** The join goes inside B7 as the first stacked PR, with no constant substitution.
- `physics_source_recovery.schema.json` becomes contract-neutral: the policy and `profile_mode` are per contract.
- A reviewed `semantic_contract_v0_3_pressure_source_1.json` is added.
- Both are reviewed inputs, so the join head re-registers (D3).
- Rows still resolve through `physics_source_contract()`. A resident table for them would add a reader static, which goes to HELP_HUMAN before any code (O-10).

**D5.** On the join, ruling 3(b) (straight-only documents authored as v2) and ruling 3(c) (the not-joined diagnostic) lapse by their own condition. Ruling 3(a)'s SP-1 twin clause widens to retained-source publications, with a closed exclusion list:
- identity, profile and limitations;
- `profile_mode` and `profile_version`;
- the receipt policy;
- the invocation-bound hashes: invocation, publication, receipt, case evidence, `normalized_source_sha256` and `functional_plan_sha256`;
- the three work counters, asserted exactly as Δ = 24 × (len(v3 contract) − len(v2 contract)) per reservation.

**D6.** Until T4-U2's export branch lands, export of `pressure-source-1` is refused by name with `PRESSURE_1_EXPORT_NOT_AVAILABLE`.

**D7** is recorded in DEL-04-07 Design (`pressure-contract.md`).

| Unit | Content |
|---|---|
| J1 producer | Delete `exact_admission::joins_retained_source`, both gate sites, and T4-I31's `RETAINED_SOURCE_NOT_JOINED` diagnostic, flag and emission. Choose the identity, policy and case domain from the captured contract, never the envelope label. Give `pressure_v3_formulation_basis` joined and unjoined static texts. Add v3 twins `fixtures/product_preview/pressure_source/*`. Regenerate T4's two v3 reader corpora and the `pressure_1` table's limitations. |
| J2 corpus | `fixtures/results/pressure_source_reader_corpus.json`: v3 twins of the 12 committed source fixtures × 2 modes, plus mutations. A PP test generates it and checks that it is current. |
| J3 Rust reader | `SourceContract`; `Method::PressureJoined` for 0.4.0 (no projection onto v2); header dispatch (allowlist, downgrade exceptions, profile, row table); cross-read and downgrade refusals by name; the D6 export refusal. |
| J4 Python verifier | The same gates and codes, reading J2 unmodified. |
| J5 TypeScript | The TypeScript base-route readers only: `physicsResultEvidence.ts`, `physicsSourceRecovery.ts`, `sourceBlockRecovery.ts` and `numericalResultQuality.ts`. This changes if R is extended. |
| J6 schema and text | `results.v0.3` gains the `PRESSURE-SOURCE-1` receipt and the v3 physical-evidence definitions. DEL-04-07 Design records ruling 3's lapse. |
| J7 consequences | The D5 twin test over 12 fixtures × 2 modes. The frozen RF references in `validation/references/t3_r1` that lie in the source family, run through v3 recovery at the references' own criterion, with no reference or tolerance touched. T4's oracles (`t4_i6`, `t4_i9`, `t4_i12`) decline by name, with ordinary bytes unchanged. Mutants M1 and MJ1–MJ9, each killed by a named test. Byte identity, statics, the TEXT chain, SQ2 and an independent review. |

**Blast radius.**
- **Moves:**
  - two reviewed inputs: the neutral receipt schema and the new table. The join head re-registers (D3);
  - priced TEXT, slightly: the joined limitation sentence and the new constants are added, and the not-joined diagnostic is removed;
  - T4's two v3 reader corpora are regenerated, and T4's "v3 not joined" pins flip. Both are declared.
- **Expected unchanged:** priced memory.
- **Unchanged:** every v2 byte, every successor byte, the 07m/07n/07o outcomes and the source-block corpora.

## B7a and B7b

**The split** is the one T3's legacy inventory proposed; item labels such as A1, E8 or D6 are that inventory's, kept for traceability.
- **B7a** holds everything that does not depend on deleting v2: the H-4 wave, the pressure-free half of the model-format change, the other old format versions and the legacy removals.
- **B7b** consolidates the exact route onto v3. That needs the join.

**Registration.** Each head re-registers under D3 and regenerates what it moves. B7b re-derives the exact-route bytes that B7a has already moved (E15, D6). The release identity is the build identity registered on B7b's head, which the batch lands with. Release remains the owner's act.

### B7a

| Item | Change | Blast radius |
|---|---|---|
| H-4: C1–C3 | Rename `component_user_stiffness_macro_element_count` to the curved macro-element count. Remove the historical joint row kind (`component_user_stiffness_macro_element_review`); do not rename it. Delete the retired thrust count and `expansion_joint_pressure_thrust_load_review`. | Priced `MechanicsEnvelope`/`Summary`; all ten semantic tables (reviewed); all 45 corpus bases; every result fixture; `results.v0.3`; the desktop's summary display. |
| H-4: C4–C6 | Reword `preview_physics.rs` LIMITATIONS[1] without "legacy". Apply T4-U4's arc-frame unification on preview-physics-1 if it is ready. Reword any other LIMITATIONS that name retired inputs. | Preview-physics-1 and preview-physics-retained-1 tables (reviewed); `results.v0.3`; 19 result fixtures; all 45 corpus bases; the 6 mutations whose edit paths index `limitations`; 12 source-block raw fixtures and 2 precision fixtures. |
| Model format, pressure-free half (A1, E8, the namespace predicates) | Define 0.5.0 (below) for documents without a pressure contract, and refuse 0.1.0/0.2.0 by name with a re-author diagnostic. Delete the 0.1.0 → 0.2.0 chain (`apps/desktop/src-tauri/src/model_document_migration.rs`), its TypeScript mirror (`apps/desktop/src/services/projectService.ts`, `apps/desktop/src/features/workspace/projectPersistenceIntegrity.ts`) and PP's version arms (`pressure_runtime.rs`, `source_recovery.rs`). Key the W1 namespace on the contract's absence instead of the version: `retained_memory.rs` `NamespaceBranch::Preview`, the Rust reader's `preview_namespace` and the verifier's `_preview_namespace` (the names were aligned in pre-B7; only the predicate changes here). Geometry operations take the current version, not only 0.1.0. | About 112 model fixtures. All 45 corpus bases (30 at 0.1.0, 15 at 0.2.0), and with them every invocation digest, receipt, publication hash and must-pass entry. The milestone and G10 fixtures (below) and the bundled demo models. The migration fields of `schemas/project_persistence.schema.yaml` and `docs/architecture/persistence_contract.md`'s until-B7 notes. The desktop open path's refusal statuses and save normalization (DEL-07-11 Design). The pricing chain's rule for the Rust reader's `preview_namespace` (re-pointed in pre-B7) if the predicate's TEXT moves. |
| Analysis-run records (E1) | Keep 0.3.0 only. Delete the 0.1, legacy-desktop 0.1 and 0.2 schemas and their dispatcher arms; `core/analysis_runs/legacy.py`; the non-JCS `records.canonical_json` hash; `build_analysis_run_v0_2`; and the desktop's legacy digests (`analysisRunCompatibility.ts`, `HistoricalRunContext.tsx`). | `fixtures/analysis_runs/invented/{legacy_desktop_v0_1,legacy_python_v0_1,analysis_run_v0_2}.json`; the `test_analysis_run_*` suites and `HistoricalRunContext.test.tsx`. Saved runs from older records are refused by name. No reviewed input, priced layout or corpus moves. |
| Stress-neutral exports (E3) | Native 0.3 builders in Python and in the desktop. Today both project from the 0.1 packet (`package.py`, `buildStressNeutralExportPacketV01`). Delete the 0.1 and 0.2 schemas and their dispatcher arms, `package.py`, `package_v0_2.py` and the panel's `semantic_contract_v0_2.json` reference. | 0.3 package bytes must not change. A byte test over the 14 committed `*.stress_neutral.json` fixtures checks this, and any change stops the item. Also `test_stress_neutral_*` and `StressNeutralExportPanel.test.tsx`. |
| Result exports (E2) and the mechanics-result 0.1.0 carrier (B2's format part) | Keep one export version, 0.3.0. Delete `results.v0.1`, `results.v0.2` and the dispatcher's inline copies; `derivative.rs`'s `legacy_checksum` and legacy-enriched branches; and the runner's relabeling of 0.3.0 as 0.2.0. Readers refuse a producer-less 0.1.0 mechanics result, and `semantic_contract_v0_2.json` leaves the reviewed inputs. | One reviewed input fewer. The v0_2 SHA pins (10 files). `invented_demo_result_legacy_0_1.json` and the `result_export_v0_2*` fixtures. Runner and `derivative_contract` tests. The Tauri and `product_preview/service.py` consumers. |
| E11 | Delete `#[allow(dead_code)] mod pressure_exact`'s dead items, the `formation_guard::RecoveryRecord` fields that only tests read, and the unread `unit`/`label` fields in PP `lib.rs`. | `RecoveryRecord` is priced, so its layout is re-priced. No published byte moves. |
| E13 (done in pre-B7) | PP's second combination engine, `append_combination_results`, was deleted in the pre-B7 batch; its test was ported to `append_combined_vector_magnitude`. | None left for B7. |
| E13 leftover | `append_combined_vector_magnitude` (PP `lib.rs`) keeps a `support_vectors` parameter that every caller passes empty, and its reaction arm. Delete both, together with the pricing decision ID_AUDIT[39] that covers them. | No published byte moves; TEXT and the pricing decision list change, so it rides B7a's re-registration. |
| `springs` | `SelectedSourceRecovery.springs` (`source_recovery.rs`) is never read; delete it. | `source_recovery` types are priced, so the layout is re-priced. No published byte moves. |
| E14 | Delete the DEC-050/053 unscaled-LU observation lane and its rows (`legacy_unscaled_DEC050_DEC053`, `sparse_live_path_dense_parity_relative_delta`), or re-base the parity observation on the current solve. | Every envelope's observation rows (122 and 47 occurrences in the corpus); tables; successor bytes; the memory guard's byte estimate; the milestone model's displayed rows (G10, held). |
| E15 | Use one correctly rounded norm (`norm3`) for `source_receipt`'s `scaled_norm` and for `displacement_magnitude`. | Successor and corpus bytes where the last bits differ; every source receipt (all source-block corpora). |
| E16 | Replace `stress_recovery`'s `#[path]` include of `correct_norm.rs` with a small shared crate. | PP `Cargo.lock` (reviewed). No published byte moves. |
| E17 | The G7 magnitude guard compares exactly against a correctly rounded norm. This replaces nested libm `hypot` with a 64-eps tolerance in the Rust reader and the verifier, and in the TypeScript base reader unless R is extended. | Outcomes of near-tolerance tampering may move: the 10 `g7*` mutations. Reader statics and TEXT. |
| D6 | One checked `WorkTotal`. Delete the frame kernel's saturating counter (`legacy_saturated`), its compatibility wrappers (`gap_sign` beside `gap_sign_with_work`, `multi.rs`'s legacy getter) and `SaturationNotExcluded`. | Source-receipt bytes, and `legacy_source_work[].rejected` in every successor and corpus base. Priced `source_recovery` and `source_receipt` types. |
| D3 strings | Desktop notices that narrate history ("until T3", "until T6" in `knownSemanticLimitations.ts` and `numericalResultQuality.ts`) state current behaviour instead. | Desktop text and its tests only. |
| Corpus | Regenerate the 45 bases once, under B7a's producer, as one snapshot that replaces 07m, 07n and 07o. Keep mutation ids where the meaning is unchanged, and re-derive the limitations-path mutations. Regenerate the successor, preview, precision, source-block and demo fixtures. | Census: first failures and must-pass outcomes unchanged, except those declared. |

### B7b

| Item | Change | Blast radius |
|---|---|---|
| D4 (a): W1 route E | Rebase `physics-retained-1` from `physics-1` with `{2.0.0, exact_straight_pressure_v2}` onto `pressure-1`. | DEF-E (`retained_precision_prepared_exact_v1.json`) is replaced, and the `physics-retained-1` table changes (both reviewed). The exact successor fixtures, `retained_precision_rv120_b3_inputs.json` and G5's route-E pricing move. No shared-corpus base moves, because none is exact. |
| v2 deletion (A4, A5) | Delete `ExactContract::StraightV2` and the v2-only producer, reader and desktop arms, including the cross-read refusals (`V3_READ_AS_V2`, `V2_READ_AS_V3`). Delete the `physics-1`, `physics-source-1`, `load-reference-1` and `load-reference-source-1` tables. `physics_source_recovery.schema.json` takes v3 constants. `pressure-1` gets its own registered table, priced under O-10. | Reviewed inputs: four tables out, one in, plus the receipt schema. The v2 raw fixtures and derived documents are replaced by their v3 twins. The v2 branches of `results.v0.3` and `stress_neutral_export.v0.3`, and the desktop's pressure-authoring panels, change. |
| Model format, exact half (A6) | 0.5.0 gains the contract-present form and the load/reference-state group, and 0.3.0/0.4.0 are refused by name. Every version-keyed selector keys instead on the contract and the group: `case_state::is_load_state` and `LOAD_STATE_MODEL_VERSION`, `preview_physics.rs`'s 0.4.0 test, `pressure_runtime::is_exact`, and the join's record key, which today is present exactly on 0.4.0. | Every exact and load-reference fixture is re-authored once, to 0.5.0 with v3. The source-block and load-reference corpora, T4's v3 reader corpora and J2 move with them. |

## Current model format

**Evidence.** It comes from `apps/desktop/src-tauri/src/model_document_migration.rs`, `core/product_physics/src/pressure_runtime.rs`, `core/product_physics/src/case_state/` and the schemas. `schemas/model.schema.yaml` has one shape for every version; the meaning of each version lives in code.

| Version | What it expresses |
|---|---|
| 0.1.0 / 0.2.0 | No pressure contract (a declared one is refused) and no `pressure_regions`. It solves on the preview route, with combinations (0.2.0 adds their shape fields), nonlinear supports, components and `equivalent_static`. 0.1.0 reaches 0.2.0 by a no-op migration. |
| 0.3.0 | The exact pressure contract is required (`PRESSURE_CONTRACT_REQUIRED`), and every case carries `pressure_regions`, with `[]` when it is unpressurized. It solves on the exact route only. That route refuses combinations, nonlinear supports, `equivalent_static` and, until T4 extends it, components. |
| 0.4.0 | 0.3.0 plus the load/reference-state group: `reference_configurations`, material `expansion_laws`, and an explicit `analysis_state` on every case. All of these are required, and so is the exact contract. The version alone selects the load-reference identities. |

**Choice: neither 0.3.0 nor 0.4.0; B7 defines 0.5.0.**

Neither existing version expresses models with and without pressure as one format.
- **Both require the exact contract.** Every document therefore goes to the exact route, and a pressure-free model that needs the preview route cannot be written in either.
- **The milestone model is such a model.** It is pressure-free on the preview route (W1 route L). Writing it as 0.3.0 would move it onto route E.
- **Widening either in place is ruled out.** It would change what an existing version can contain, against DEC-033's rule that such a change bumps the minor version. Behaviour would also stay keyed on a version that means two things.

**Model document 0.5.0** is the only current version:
This is the design choice DEL-02-05 CLM-003 and DEL-04-07 Design point to.

- **`pressure_contract` is optional.**
  - Absent: the document is pressure-free and solves on the preview route. `pressure_regions` and every pressure primitive are refused by name.
  - Present: it names the current exact contract (v3 after B7b), and every case carries `pressure_regions`, as 0.3.0 requires today.
- **The load/reference-state group is optional as a whole.** When present, it carries 0.4.0's rules (an explicit `analysis_state` on every case) and requires the exact contract, as today.
- **Behaviour keys on presence.** It keys on whether the contract and the group are present, never on the version string.
- **Every other version is refused by name** with a re-author diagnostic. Nothing is migrated.

**Re-authoring the fixtures.**
- **B7a** re-authors every committed 0.1.0/0.2.0 model to 0.5.0 and re-checks its pins. This includes:
  - `fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json`;
  - the G10 desktop-shaped milestone fixture `fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.desktop_model.json`;
  - the demo models `invented_demo_model.json` and `invented_preview_model.json`.

  The pins re-checked include `core/product_physics/tests/g10_desktop_milestone_model.rs`, `apps/desktop/src/services/g10DesktopMilestoneModel.test.ts`, the milestone successor fixtures and corpus base, and both `retained_precision_admission.rs` suites.
- **B7b** re-authors the 0.3.0/0.4.0 fixtures.

## Held by the owner

These are not decided here.
- **G10 interpretation.** B7a changes the milestone model's document version, its ordinary-route bytes and its displayed rows (C4, E14, the model format). Whether G10 is witnessed before B7a, or re-issued against the B7 head, is the owner's choice.
- **B8 (activation).** R and B7a change the desktop's retained standing text and notices. Nothing here activates anything.
- **Decision 22.** Any change of M or of the budget model.
- **R-b′ for all members (conditional).**
  - **The finding:** T4's O-13 work found that R-b′'s resolution conjunct (`q > 2^10·B`) masks gross arc errors at k ≥ 1e8. It adopted an extra arc firing condition, `B_f > RD(1e-9)·q` (DEL-04-01 Design, `arc-recovery-guard.md`).
  - **The open question:** a reviewer is checking whether straight members are masked the same way.
  - **If they are,** B7a makes R-b′ one predicate for all members and declares its blast radius:
    - straight ends that newly fire move cases from `NUMERICAL_INTEGRITY_CHECKS_PASSED` to `NUMERICAL_INTEGRITY_SENSITIVE`;
    - the s11g pins;
    - corpus bases whose integrity code moves;
    - successor bytes;
    - TEXT.
  - **If they are not,** nothing here changes.

## Open: needs a ruling

- **Extending R to the base-route readers.** The owner's words, "move to one product reader in Rust", read as covering every reader. Today's scope covers the retained reader only.
  - If R extends, the TypeScript base-route readers are deleted the same way, and the Python ones become test-only. J5 then disappears, and B7's base-reader changes are made in two places instead of three.
- **Inventory items with no scoping ruling:**
  - B1 (precision-1);
  - B3 (retired row kinds);
  - B4 and D1 (the source-blocks route and the W1 `legacy_source` member);
  - B5 (stale reservations);
  - B6 (downgrade refusals);
  - D8 (snapshot pin tests).

  Each moves tables or the corpus. Any that does not ride B7a needs its own re-registration and corpus regeneration later.
