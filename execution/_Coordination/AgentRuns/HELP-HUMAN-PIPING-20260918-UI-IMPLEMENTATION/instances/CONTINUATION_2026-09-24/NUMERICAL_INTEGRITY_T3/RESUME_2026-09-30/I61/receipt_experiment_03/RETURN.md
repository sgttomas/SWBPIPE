# I61 RETURN: experiment 03, facade capture end to end (step 4, stage 1)

**Status: PASS.** The milestone receipts for RF-SKEW-T-CANT-OFF-122-r1e-04 were produced through the actual facade, `run_linear_static_preview_value_with_retained_direct`, with capture installed in the single actual ordinary run. Both modes were run on the unchanged model-0.1.0 request.
- **Readers:** the three accepted readers at NUM pass G0–G8 in both modes. Eligibility is off, and standing is `needs_recompute`.
- **Byte-diff:** both receipts are **byte-identical** to the experiment-02 receipts, which came from the private driver. The only differences are in the provenance sidecar and are explained below.
- **Ordinary bytes:** they are unchanged under capture, by three byte controls.
- **G-l:** the legacy failure is now captured typed. Four negative controls show that the typed values are the ones emitted and that they are cross-checked.
- **Stops:** no stop condition was reached. No contract reading was needed for the milestone path; one is needed for U1 (see R-U1-1).

This is disposable archive evidence. Under decision 7, the test-only permit stub exists only in the archive. It is not the public milestone: U4's profile and M, U1's serializer and U3's maintained installation are still required.

TASK Type 2 under ROOT's standing assignment (NUM `dca3b65b3d`, ruling "Step 4 planned: decisions and dispatch"), with no descendants.
- **Time:** 2026-10-04T03:06:52Z to about 03:25Z.
- **Host:** the memory guard (PID 5387) was running throughout. Default toolchain (no `DEVELOPER_DIR`); `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`; one Cargo job at a time, each under a 1200 s perl alarm.
- **Git:** no Git writes. Git reads used `GIT_OPTIONAL_LOCKS=0` (`git show`, `git log`).
- **Writes:** only inside the fence:
  - WT/scratch/i61_receipt_experiment_03/;
  - WT/targets/i61-receipt/{product_physics,result_export}/;
  - this folder.

## The slice (`_run_records/archive_*.diff`, `emitter_delta_from_experiment_02.diff`)

The archive is a `git archive` of NUM `dca3b65b3d` (the whole P tree). P's `node_modules` is linked, and the prebuilt `apps/desktop/public/` WASM is copied in; it is byte-identical to experiment 02's. The archive edits are all `#[cfg(test)]`:

1. **PP/lib.rs dispatch (NUM :2199–2229):** after the census, a branch runs only when `admission.is_some() && test_permit()` and the model is neither load-state nor exact.
   - It installs `ProductCapture::prepared_probe()` into `run_linear_static_preview_observed`, using the same `SourceRecoveryBudget` the plain path uses. This is the one actual ordinary run.
   - It keeps the `SOURCE_BLOCKS_FINALIZATION_FAILED` check unchanged.
   - It calls the emitter's `facade_w1`.
   - It returns the ordinary envelope, plus a test-only `successor` field.
   
   The permit is a thread-local that is false by default. Every other call takes the unchanged path.
2. **PP/lib.rs, the G-l site (NUM :3700–3760):**
   - a local `legacy_attempted` flag, set only in the branch that calls `source_recovery::solve_ordinary`;
   - in the `Err(failure)` arm, when `product` is `Some`, it records `LegacyCapture{case, attempted, stage, helper_stage, charged, rejected, limit}` from the typed `RecoveryFailure` and its `WorkReport`. `helper_stage` uses a closed match over the eight `AttemptStage` variants.
   
   The arm's diagnostics, debit and control flow are untouched.
3. **PP/retained_product.rs:** `LegacyCapture` and the `ProductCapture.legacy_failure` field.
4. **The emitter (the experiment-02 file, refactored):**
   - `facade_w1` handles the coexistence bypass when the ordinary envelope carries `source_block_recovery`. It records the ordinary bytes, keeps a copy of the ordinary envelope for the fallback and the return, then calls `prepare_case`, `solve_native`, `project_candidate` and `emit_selected`. A failure at any stage returns the unchanged ordinary envelope (W1-unavailable).
   - `emit_selected` is experiment 02's `milestone()` body, minus its own `prepare_observed` call.
   - `legacy_source` and `legacy_source_work` now come from `LegacyCapture`. The experiment-02 message parser survives only as an equality cross-check and is never emitted.
   - A typed capture without an actual attempt, or a disclosure without a capture, panics as `UNMAPPED … contract reading needed` instead of being guessed.
   - The refusal receipt is dropped (T3 / D9b: on a one-case refusal the ordinary result is published).

The maintained retained_memory.rs, source_recovery.rs and retained_receipt.rs are byte-identical to NUM, verified by `cmp`. `admission()` still always refuses. The census reports `profile=Missing allowance=Unselected` for the milestone in both modes; the stub bypassed it, as decision 7 allows in the archive only.

## Results

**Readers** (`reader_results_{py,rs,ts}.txt`): all three are the accepted readers at NUM `dca3b65b3d`, run unchanged.
- Python: `_validate_draft`.
- Rust: `retained_precision::validate`.
- TypeScript: `validateRetainedPrecision`.

| Receipt | Python | Rust | TypeScript |
|---|---|---|---|
| milestone, sparse_interactive | **PASS G0–G8**, `needs_recompute`, eligible=False, invocation-bound, 98 classifications | **PASS**, eligible=false, invocation-bound, 98 | **PASS**, `needs_recompute`, eligible=false, 98 |
| milestone, dense_scrutiny | **PASS G0–G8**, `needs_recompute`, eligible=False, invocation-bound, 99 | **PASS**, eligible=false, invocation-bound, 99 | **PASS**, `needs_recompute`, eligible=false, 99 |

- **Schema:** 0 jsonschema violations against the NUM schema.
- **Eligibility:** off in every reader (Python and Rust `IMPLEMENTATION_COMPLETE = false`; TypeScript `SUMMARY_COVERAGE_COMPLETE = false`).
- **Parity:** Python = Rust = TypeScript on every row, and every reader equals the producer's `CertifiedProductProof::verdicts()` (98/98 sparse, 99/99 dense). All verdicts passed.
- **Classes:**
  - sparse: 69 absolute_verified, 25 relative_verified, 3 input_derived, 1 non_quantity;
  - dense: the same, with 2 non_quantity.

**Byte controls** (`emit_log_excerpt.txt`, `controls_and_mutants.txt`):

| Control | Sparse | Dense |
|---|---|---|
| A: no-permit `…_retained_direct` bytes = `…_with_mode` bytes | equal | equal |
| B: the ordinary run inside the permit path = the plain run | equal (68250 B, `9c7ec1a1…`) | equal (69366 B, `21ca629c…`) |
| B′: the envelope returned on the permit path = the plain run | equal | equal |
| Determinism: rerun, and the direct binary run | identical | identical |

**PP `--lib` in the archive:**
- **Base:** the NUM bytes of lib.rs and retained_product.rs, with the emitter absent: 456 passed, 1 failed, 1 ignored.
- **Experiment 03:** 457 passed, 1 failed, 1 ignored.
- **Outcome-set diff:** the only change is `+ i61_receipt_probe::i61_facade_receipts ... ok`.
- **The one failure** is `s11g_tests::t13_committed_fallback_uz_is_byte_identical` ("SparseInteractive: committed bytes changed"). It fails identically at base and is the known Mac platform failure (HANDOFF_2026-09-30_AUDIT_PAUSE:120). It is not caused by this slice, and it was not touched.

**G-l negative controls** (`I61_MUTANT`, final binary; each must fail, and each does):

| Mutant | Result |
|---|---|
| `gl_charged` | killed: "typed G-l equals the disclosed failure" |
| `gl_helper` | killed: the same assertion |
| `gl_drop` | killed: `UNMAPPED legacy outcome … typed=None` |
| `gl_unattempted` | killed: `UNMAPPED legacy outcome … attempted: false` |

**Memory** (`producer_rss.txt`): the whole test peaked at 20.6 MB maximum RSS (7.9 MB peak footprint). That covers six ordinary runs (two controls and one permit run per mode) and two prepared/native/proof/emission passes. It is consistent with experiment 02's 20.5 MB.

## The byte-diff against experiment 02, explained (`checks_and_class_parity.txt`)

| File | Experiment 02 (private driver) | Experiment 03 (facade) | Difference |
|---|---|---|---|
| milestone_sparse_interactive.json | `bca4e9ca…2e59`, 201916 B | `bca4e9ca…2e59`, 201916 B | **none** |
| milestone_dense_scrutiny.json | `06449153…f310`, 203265 B | `06449153…f310`, 203265 B | **none** |
| `receipt_sha256` and `publication_sha256` | sparse `2c8cee1a…e961`, dense `dbcc7dd0…2c70` | the same | none |
| producer verdicts and the py/rs/ts class files | — | identical | none |
| provenance sidecar | — | 3 entries differ | explained below |
| refusal receipt | emitted (G0–G2 check) | not emitted | deliberate (T3 / D9b) |

**Why the receipt bytes are identical.** The private driver's `PreparedCase::prepare_observed` and the facade's permit path do the same computation:
- one ordinary run, with `prepared_probe()` installed and `SourceRecoveryBudget::default()`. This is the 0.1.0 non-exact model, so the dispatch does not raise the budget;
- then the same preparation, native solve, proof and projection over the same captured facts.

The facade adds only the census, which is read-only, and the finalization check, which is not triggered. The actual request Value is the same: the emitter reads it from `CapturedInvocation::borrowed_raw()` instead of the fixture text it parsed in experiment 02. So `invocation.sha256`, the hashes and every overlay row agree.

PLAN §4's premise was "the same certified receipt, apart from what genuinely differs". Nothing in the receipt genuinely differs, because the private driver's ordinary run was always the one actual run, with capture installed. It is now simply the facade's own run.

**The typed G-l values equal the values experiment 02 parsed from the message:** `{stage:"source closure", helper_stage:"source_closure", charged:46628, rejected:0, limit:4000000}`. So moving from parse to typed capture changes no byte. The cross-check enforces this equality, and the mutants show that it is live.

**The provenance differences, which are sidecar only and never hashed:**
- `+ ROUTE`: names the facade route;
- `ordinary_attempts[].legacy_source` and `body.legacy_source_work[]`: changed from "PRODUCER GAP: read back from the diagnostic message" to "PRODUCER (G-l): typed LegacyCapture … message text only cross-checked".

## Design inputs for U3 (no contract reading; for ROOT's U3 brief)

1. **The output type.** `MechanicsEnvelope` has no `retained_precision` member, so the archive carried the successor as a test-only Value beside the ordinary envelope. U3 needs a successor-bearing output in RetainedPreviewOutput, for example a private successor carrier that is consumed at the transfer. Today's `envelope()` and `into_parts()` cannot express it.
2. **The fallback copy.** `prepare_case` consumes the ordinary envelope. A W1-unavailable fallback with preserved ordinary bytes needs a retained copy: in the archive, a clone, about 68–69 KB serialized here. The I51 frozen-candidate split should own it, and U4's M must count it.
3. **The domain gate and SF-1.** The branch excludes load-state models, whose SF-1 republication lives in `run_linear_static_preview_captured`, and exact pressure, whose budget is raised. Decision 6's first domain excludes both. If a later domain admits load-state, U3 must order capture against the SF-1 rerun.
4. **The permit position.** The census runs before installation. The stub was consulted after the census and before the single ordinary run, so the no-permit path had no extra run and no extra clone (control A).

## U1 grant-1 brief proposal (stage 2; for ROOT to issue after confirming stage 1)

**Purpose:** U1(a) plus U2 from PLAN §2. Grant 1 covers:
- the typed capture (G-l and G-b);
- the projection and emission of a selected one-case attempt;
- RV77-N4's structural owner binding.

U1(b) is a separate later grant: the closed error translations (G-i), the D38 unavailable-path representation, and the full mutant set.

**Basis:**
- NUM at ROOT's current revision (`dca3b65b3d` or later);
- decisions 1–9;
- D38;
- C1, C2 and C3 with seams 06–08;
- the T1 (a) ruling;
- this experiment, whose emitter (`_run_records/emitter_i61_receipt_probe.rs`) is the reference implementation;
- the in-tree fixtures (definition, semantic table, schema), which are read-only.

**One reading is needed before or at the start of the grant: R-U1-1, the legacy_source disposition table.** C2:160 names four dispositions but does not map them to producer sites. The experiment exercised only `unavailable`, and refuses everything else. Proposed, by the producer's own branches at PP/lib.rs :3700–3760:

| Producer branch | Disposition |
|---|---|
| `!source_eligible` | `not_eligible` |
| `source_eligible && !needs_source_recovery` | `not_required` |
| `formation_decline_without_attempt()` (stage "formation guard") or `range_formation_decline_without_attempt()` (stage "range formation") | `declined_without_attempt` |
| Err after an actual `solve_ordinary` attempt | `unavailable` |
| `Ok(recovery)` | exact Selected; the coexistence bypass, so no successor |

`diagnostic_ref` and `work_ref` follow T1 (a) on the selected case. Experiment 02's "no disclosure, so `not_eligible`" conflated the first two rows. Grant 1 must capture `source_eligible` and `needs_source_recovery` typed (part of G-b).

**The write fence:**
- **PP/retained_product.rs:** the typed capture fields (the G-l `LegacyCapture` equivalent; the G-b ordinary `initial`, `w2` and `formation` members and the eligibility flags), plus making `certificate` private with accessors (U2).
- **PP/lib.rs, only at the ordinary capture sites:**
  - the G-l arm and its attempted flag (:3700–3760);
  - the G-b report, W2 and formation sites.
  
  Each write is guarded by `if let Some(observer) = product.as_deref_mut()`. Diagnostics, debits, text and control flow are unchanged. The dispatch is **not** in the fence; it belongs to U3.
- **PP/retained_receipt.rs:** the C3-seam projection.
- **A new PP/retained_wire.rs:** the JSON projection and the hashes through `canonical_json_checked_v1_text`. It is proposed as a separate file for review size.
- **FK/product_certificate/final_case.rs:** only `CertifiedProductProof::owner_matches(&RetainedSolve)` over `ProofAnchor::matches_owner` (U2).
- **Tests:** PP/retained_product_tests.rs and a new PP/retained_wire_tests.rs.
- **Optional, needs ROOT's choice:** PP/Cargo.toml and Cargo.lock, for decision 5's `result_export` path dependency, added in grant 1 as a dev-dependency so that the committed receipt tests run the accepted Rust reader in-process. Otherwise it waits for U3's precommit validation.
- **Targets and records:** WT/targets/i61-u1/ and NUM/R/I61/u1_serializer_01/.

Everything else is read-only, including:
- retained_memory.rs (no permit and no test permit: decision 7);
- the readers, the schema and the fixtures.

**The deliverable:** a private, production-unreachable serializer.
- **Input:** a completed one-case `PrivatePreparedCandidate`, plus its typed ordinary capture.
- **Output:** either `(successor envelope Value, retained_precision{body, receipt_sha256})` or a typed `receipt_failure`.
- **Coverage:**
  - G-a, with fixed product text for `RETAINED_PRECISION_SELECTED`;
  - G-b;
  - G-l;
  - T1 (a);
  - G-c, as decision 2 (A2/D6a);
  - G-d, typed `support_indices`;
  - G-e, `not_covered` from the certificate verdicts;
  - G-j, for one case;
  - A1, as decision 1 (raw SHA256 of K4RST);
  - the definition and table hashes from the in-tree fixtures;
  - U2: `receipt_failure{association}` unless the proof anchor matches the selected owner, with the `I61_FOREIGN_OWNER` assertion deliberately flipped.
- **Untranslated variants:** any variant that grant 1 does not translate returns a typed `receipt_failure` naming it. It never emits Debug text and never panics in a non-test build.

**Protected byte controls:** any failure is a stop.
1. **The ordinary bytes of every existing route stay unchanged.**
   - PP `--lib` and the PP, runner/headless and result_export integration suites have the same outcome set as base. The expected Mac platform failures are exactly `t13_committed_fallback_uz_is_byte_identical` and the two runner_headless load_reference tests (HANDOFF_2026-09-30_AUDIT_PAUSE:120–122).
   - The experiment's controls A, B and B′ become committed tests on the private driver. The milestone's ordinary bytes stay at `9c7ec1a1…` (sparse) and `21ca629c…` (dense) with capture installed and with it absent.
2. **The serializer output is byte-identical to this experiment's receipts** for the milestone in both modes: `bca4e9ca…2e59` and `06449153…f310`, and receipt hashes `2c8cee1a…` and `dbcc7dd0…`. Any byte difference must trace to a closed gap: G-d, G-e, A1 or G-a's fixed text. Each such difference is listed with its cause and revalidated.
3. **The three accepted readers at NUM pass G0–G8** on the serializer's output, with eligibility off and class parity 98/98 and 99/99. Python and TypeScript run in a test lane outside PP; Rust runs in-process if the dev-dependency is granted.
4. **Nothing is weakened:**
   - no reader, schema, fixture or existing check is changed or weakened;
   - maintained retained_memory.rs is untouched;
   - the dispatch is untouched.
5. **Mutants are killed:** the T1 (a) omission, each G-l field, the helper_stage mapping, the D6a filter, the R-U1-1 table rows, and U2's owner binding.

**Cost:** about 6–8 agent-hours for grant 1, and about 3 hours for an independent review of U1 and U2 together.

**Stops:** the current set:
- a contract reading is needed beyond R-U1-1;
- an ordinary byte changes;
- a check would be removed or weakened;
- any write would fall outside the fence.

No Git writes.
