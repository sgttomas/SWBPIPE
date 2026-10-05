# I61 RETURN: U1 grant 1 with U2, the private retained-precision serializer

**Status: DONE; no stop condition was reached.** The candidate is uncommitted in WT/f2a-serializer (branch `codex/piping-f2a-serializer-20261004`, base NUM `43a6368c21`), for ROOT and RV82.

**What it is:** a private serializer, unreachable in production. It maps a certified one-case prepared candidate, plus its typed ordinary capture, to the successor envelope carrying `retained_precision {body, receipt_sha256}`. When it cannot, it returns a typed `receipt_failure`.

**Readers:** on the unchanged 0.1.0 milestone (RF-SKEW-T-CANT-OFF-122-r1e-04), the three accepted readers at NUM pass G0–G8 in both modes, with eligibility off and standing `needs_recompute`. Class parity is 98/98 and 99/99.

**Protected byte controls 1–5:** all hold.
- Ordinary bytes are unchanged on every route.
- The serializer's bytes differ from experiment 03 only by closed gaps G-a and G-b.

**ROOT's mid-grant corrections are adopted:**
- the checked work custody (RR "Checked work custody in U1", NUM `21fe3e923d`);
- D-4 / D-4b (NUM `fe38ea55bc`).

The milestone bytes were re-verified unchanged after each correction.

**Mutants:** **39 of 39 killed** on the final candidate: 20 for the grant, 9 for the checked-work correction and 10 for D-4.

TASK Type 2 under ROOT's stage-2 confirmation (NUM `43a6368c21`, ruling "Experiment 03 … U1 grant 1 confirmed"), with no descendants.
- **Time:** 2026-10-04T03:23Z to about 04:12Z.
- **Host:** the memory guard (PID 5387) was running throughout. Default toolchain; `--locked --offline`; `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`; one Cargo job at a time, each under a 1200 s perl alarm.
- **Git:** no Git writes; reads only, with `GIT_OPTIONAL_LOCKS=0` (`status`, `diff`, `log`, `archive`).
- **Writes:** the fence below, WT/scratch/i61_u1_serializer_01/, WT/targets/i61-u1/ and this folder.
- **Archives in scratch:** `lane/` is a `git archive` of NUM `43a6368c21`. It is the base for the controls and the TypeScript lane; P's `node_modules` is linked and the prebuilt `public/` WASM copied in. `mut/` is a copy of the candidate core, used for mutants.

## Changed files (WT/f2a-serializer; sha256 of the final bytes)

| sha256 | File | Change |
|---|---|---|
| `ddf019b2d4ddf0c6eeb6ef9c78944fe1969df9a4868d79cca18786d788248c45` | P/core/product_physics/src/retained_wire.rs (new) | the serializer |
| `e731565f04dada67f24d0af4785c0823cdfa446b830fe64ceba19028f97cab74` | P/core/product_physics/src/retained_wire_tests.rs (new) | its tests |
| `2c6bc6a5e60f22a0b1269a6b4122d4dc73c486ea62ea68962669ac6f6220b3db` | P/core/product_physics/src/retained_product.rs | typed ordinary capture (`OrdinarySeed` and its recorders); `certificate` made private with an accessor (U2); a test-only certificate swap |
| `d27f6e95dd5f81b25af01dd51ff939032345a862a2c18bbbfb330f04c71e3a2b` | P/core/product_physics/src/lib.rs | G-l and G-b capture sites only, plus two `mod` declarations |
| `6f403a9ff556db92ead19c8cefc69286d631031e06551af1ad2a2f2a6596675e` | P/core/product_physics/src/retained_product_tests.rs | `.certificate` changed to `.certificate()` (9 sites); the I61_FOREIGN_OWNER comment |
| `36aad1f2f1b3f5be54a730f3510cba5d3fb17ed81ac81afc46bbd8cee78ff984` | P/core/product_physics/src/s11g_tests.rs | one added test (the already-Sensitive finding, kills M19) |
| `1c54371407e991ad8feb7e8c97423f351d4ffad4baa4b0abf34e8a83fa824001` | FK/structural/retained/product_certificate/final_case.rs | `CertifiedProductProof::owner_matches` only (3 lines) |
| `ba4013a19a0e0e8fadfa4c3e16c119f3fa902c3c78345cff79de042133bfd009` | P/core/product_physics/Cargo.toml | `[dev-dependencies]` `open_pipe_stress_result_export` (path) only |
| `4f494db6d8a6eca87e7a16d8561197f20b1951a033bd3a6c424acfff5613475b` | P/core/product_physics/Cargo.lock | +1 package entry and 1 dependency line, made by `cargo metadata --offline` |

**Tracked diff:** 7 files, +263/−13 (`_run_records/candidate_tracked.diff`). The new files are copied verbatim in `_run_records/`.
- **lib.rs:** nine guarded capture writes at the G-l and G-b sites, plus a local `legacy_attempted` flag set in the attempting branch. `amend_integrity_report`'s returned bool is now bound and read. Every write is `if let Some(observer) = product.as_deref_mut()`. No diagnostic, debit, text or control flow changed, and the dispatch is untouched.
- **Module declarations:** the two `mod` lines (`retained_wire`, and `#[cfg(test)] retained_wire_tests`) are the declarations the new files need. They are outside the literal "capture sites" wording, so I note them here.
- **Production reachability:** the only observer installation is `PreparedCase::prepare_observed`, which only tests call. `serialize_selected` has no non-test caller.

## The deliverable (`retained_wire::serialize_selected`)

The experiments' emitter was the reference. Every panic, Debug string and text parse is replaced by typed refusals.
- **G-a:** identity, profile, the `recovery_method` token on the case's rows, and `RETAINED_PRECISION_SELECTED` with fixed product text. The text is `diagnostic:retained-precision:{case}:selected`, `info`, `core/product_physics`, with constant message text.
- **G-b:** `initial`, `w2` and `formation` from the typed seed.
  - Report outcome: the published integrity code, cross-checked against the unchanged `numerical_quality` entry.
  - W2: `not_triggered`, or `published` with its trigger and b.
  - The load-row finding: `diagnostic_ref` set only when `demote` disclosed it, i.e. on a Passed report.
  - `d5_diagnostic_ref`: the integrity diagnostic when K-D5's line is present. See finding F1.
- **G-l and D39:** `legacy_source` comes from the typed `RecoveryFailure` and its WorkReport, captured at the `Err` arm, with a typed attempted flag.
  - The rows are not_eligible, not_required, declined_without_attempt (WorkReport 0/0/0), unavailable, and ExactSelected. ExactSelected means the coexistence bypass: refused as Scope, with no successor.
  - **T1 (a):** a selected case's captured disclosure is removed by its id, `diagnostic_ref` is null, and `work_ref` points to `legacy_source_work[k]`.
- **G-c (decision 2):** `diagnostic_refs` are the diagnostics naming the case, in envelope order, excluding `RETAINED_PRECISION_*` and the T1-omitted disclosure.
- **G-d:** `support_indices` is the support build's own unique rigid owner (`supports` × `support_fixed`). Exactly one owner is required.
- **G-e:** `not_covered` holds the certificate verdict rows that have no class and a quantity recipe. Unpublishable is refused typed.
- **G-j:** indices come from the actual invocation (`run.id`, `run.source`, `run.owner`, `physical_records`, the quality index). Anything other than one case, one call, one run and one source is refused as Scope.
- **A1 (decision 1):** `retained_state_sha256` is the raw SHA256 of the K4RST bytes.
- **Hashes:** `canonical_json_checked_v1_text`. `DEFINITION_SHA256` is pinned and bound by test both to the in-tree definition fixture and to the semantic table's entry. The identity, profile and policies are bound to the table.
- **U2:** `CertifiedProductProof::owner_matches(owner)`, a structural check through the proof anchor. Failure gives `receipt_failure{association, "cases[].selection.owner"}`.
- **Untranslated variants** give a typed refusal (`ReceiptCheck::Untranslated`, wire `encoding`). These go to grant 2:
  - Reason and Outcome reasons; BlockRefusal; BridgeError; check failures; group and build reasons;
  - SectionError arithmetic;
  - `initial.formation_failure` (basis_index), the not_attempted cause;
  - W2 failed;
  - non-quarter stations;
  - a named or temperature material basis;
  - derived E/ν.

### ROOT's checked-work correction and D-4 §3 items 1–6 (for RV82)

| Item | Implementation | Test or mutant |
|---|---|---|
| 1. Checked accessors only | `record_work()` reads `checked_lme` ×2, `checked_own_work`, `checked_shared_work`, `checked_stop_rule_work`, `checked_verification_work`, `checked_verification_shared_work`, `checked_case_charge`, `checked_invocation_increment`. `RunWork` and Call amounts go through their accessors, `InvocationMeter::checked_charged()` (and no `charged()`), and build `work`. Stage slots are emitted only if `StageWork::checked_total()` is exact (own, shared and build stages); otherwise Null plus `WorkCounter(fault)`. | `u1_checked_record_work_honours_the_latch`: an injected stage-total overflow latches the record, and all 7 latch-joined views refuse while the legacy fields look plausible. `u1_stage_slots_require_an_exact_status`. `u1_serializer_reads_no_legacy_work_field` (source guard). Mutants C01–C09. |
| 2. Exact or abandon, no panic | `Enc` records every failure and the first one is returned. Trace Counts must also be exact (C1 for a selected successor). No `panic!`, `unwrap` or `expect` remains on non-test paths (`unwrap_or_else` appears once, with a failure closure). | `u1_exact_or_abandon_counts_and_legacy_rejected`; D05 |
| 3. D-4b vocabulary | `ReceiptCheck::wire()` is total and uses C1:68 tokens only: Overflow → `work_counter_range`; Inconsistent or Both → `work_counter_inconsistent`; an exact value above 2^53−1 → `work_counter_range`; saturated `rejected` → `saturation_not_excluded`. Two non-wire private categories map as Scope → `association` and Untranslated → `encoding`. No `work_counter_overflow` or `work_counter_unknown` appears anywhere. | `u1_receipt_check_wire_vocabulary` (also checks each token is in the accepted schema enum); D01–D03 |
| 4. Legacy ledger | G-l typed values. `charged` and `limit` must be ≤ 2^53−1, else `work_counter_range`. `rejected` above 2^53−1 gives `saturation_not_excluded`. | `u1_exact_or_abandon_…`; D04 |
| 5. Conservation | Checked again at the producer before emission: own = W+K, own stages = O, shared stages = S+V, D+Q ≤ O, Σ logical case and increment = run, after = before + increment. Failure gives `work_counter_inconsistent`. | `u1_conservation_checked_before_emission`; D06–D10 |
| 6. No reader change | None made. | — |

## Results

**Readers.** The Python and TypeScript lanes run on the emitted files; Rust runs in-process through the new dev-dependency.

| Mode | Python | Rust | TypeScript | Parity with the certificate's verdicts |
|---|---|---|---|---|
| sparse_interactive | **PASS G0–G8**, needs_recompute, eligible=False, 98 | **PASS**, eligible=false, invocation-bound, 98 | **PASS**, needs_recompute, 98 | 98/98, py = rs = ts |
| dense_scrutiny | **PASS G0–G8**, needs_recompute, eligible=False, 99 | **PASS**, 99 | **PASS**, 99 | 99/99 |

The Python class files are byte-identical to experiment 03's. There are 0 schema violations.

**Protected byte controls:**
1. **Ordinary bytes.**
   - The PP suite (the lib plus 21 integration targets) at base (`lane/`, NUM `43a6368c21`) gave 618 passed, 1 failed, 1 ignored. The final candidate gave 634 passed, 1 failed, 1 ignored. The outcome-set diff is exactly the 16 added tests, all passing (`suite_base_pp.outcomes` against `suite_final_pp.outcomes`).
   - runner/headless: base 85 passed / 2 failed; final candidate identical: 85 passed / 2 failed, the same outcome set.
   - **The only failures** are the known Mac platform tests, identical at base: `t13_committed_fallback_uz_is_byte_identical`, and the two runner_headless load_reference tests (HANDOFF_2026-09-30_AUDIT_PAUSE:120–122).
   - result_export: 149/149 (unchanged crate).
   - FK `--lib`: 480 passed, 1 ignored (581.6 s); the FK change is one added method.
   - **Committed controls:** `u1_ordinary_bytes_unchanged_under_capture` (A and B on the private driver) pins the milestone's ordinary bytes, `9c7ec1a1…` (68250 B) and `21ca629c…` (69366 B). Two more requests (S11-G RF-CANCEL-UDL-W1e8, and T13b's N05 case) keep byte-identical ordinary bytes under capture.
   - **Production build warnings:** base 9, candidate 8. The one removed is "field `stage` is never read" (`RecoveryFailure.stage` is now read). No new warning.
2. **Committed successor bytes.** The pretty file and receipt are pinned in `u1_milestone_successor_both_modes`, in both modes, and are deterministic:
   - sparse: file `ac6986b0…59dc`, receipt `efc1a39b…7494`;
   - dense: file `6cd1d249…c9b5`, receipt `3e26499f…ac4a`.
   
   **The byte-diff against experiment 03** (`_run_records/diff_vs_exp03.txt`) has exactly 3 member differences per mode, plus the 2 dependent hashes (`publication_sha256` and `receipt_sha256`):
   - **G-a:** the selected diagnostic's `id` (`…:case` → `…:case:selected`) and its `message` (the experiment text → fixed product text);
   - **G-b:** `ordinary_attempts[0].formation.d5_diagnostic_ref`, from null to `diagnostic:numerical-integrity:case` (F1).
   
   Every other byte is equal, including G-d (`support_indices` `[0]`), G-e (`not_covered` `[]`), A1, all classes and every work counter. After ROOT's checked-work correction and D-4, the bytes were re-verified unchanged.
3. The readers pass, as above.
4. **Nothing weakened:**
   - no reader, schema, fixture or existing check is changed;
   - retained_memory.rs and the dispatch are untouched;
   - the I61_FOREIGN_OWNER seam assertion is unchanged (see U2).
5. **Mutants** (`_run_records/mutants_summary.txt`; disposable `mut/` tree, `--lib -- u1_ u2_`): **39/39 killed, with no compile-error kills.** Each kill names its killing tests.
   - **M01–M20, the grant:**
     - T1 (a): omission, and a kept ref;
     - G-l: charged, rejected, limit, stage and the attempted flag;
     - the helper_stage map;
     - D6a: both filters;
     - D39: eligible/required, the decline label and ExactSelected;
     - U2: the binding;
     - G-b: d5, the outcome and the finding disclosure;
     - G-e, G-d and G-a: the method token.
   - **C01–C09, the checked-work correction:** each legacy-field substitution (shared, stop-rule, verification, verification-shared, own without the latch, case charge, invocation increment), the stage-status gate and the legacy meter.
   - **D01–D10, D-4:** the Overflow, Both and saturation tokens; the cause for `rejected`; strict Counts; and the five conservation checks.
   
   **M19** (finding disclosure) needed an already-Sensitive load-row case, which the milestone is not. `s11g_tests::u1_already_sensitive_finding_is_captured_undisclosed` was added and kills it.

**U2.** `u2_foreign_owner_refused` takes two candidates of the same request (identical public facts) and swaps their certified proofs; the serializer then refuses both with `association` at `cases[].selection.owner`. This is the deliberate flip of the custody limit for the certified path. The C3 seam's `I61_FOREIGN_OWNER` assertion concerns a proof *failure* projected against a foreign owner. It stays as written, with its comment updated, because grant 1 serializes no refusal. PLAN U2's "the same on the failure work if a refusal is serialized" goes to grant 2. `PreparedCandidateRefusal.certificate` stays `pub` for the same reason.

## Findings for ROOT

- **F1. `d5_diagnostic_ref` (G-b) corrects an experiment-era emission.** The milestone's integrity diagnostic carries K-D5's `formation_check: reason=estimate` line in both modes. The experiments emitted null because they checked diagnostic codes, and no D5 code exists. The serializer emits the integrity diagnostic's id, which is the "existing diagnostic evidence" of C2:160, since K-D5's record is "one evidence line of the integrity diagnostic" (PP lib.rs `formation_check_evidence_line`, F1a D5C-3). All three readers accept it (it resolves, names the case and is listed). I read this as a direct application of C2:160 with F1a, not a new reading. If ROOT reads it otherwise, the change is one predicate at the report capture site, and mutant M15 pins it.
- **F2. D39 row 3 with T1 (a).** A selected case's declined-without-attempt disclosure is treated exactly as `unavailable` under T1 (a): omitted, with `work_ref` pointing to its zero WorkReport entry, following D39's own "(WorkReport 0/0/0)". The milestone does not exercise this branch. It is exercised by a captured run (RF-CANCEL-UDL-W1e8, `u1_load_row_case_capture`) and the mapping tests. That case cannot be serialized, because the prepared driver refuses uniform loads.
- **F3. R-b' (`amend_integrity_report`) demotion has no wire member.** A case demoted after the report by the recovery guard is captured (`recovery_demoted`) and refused as Untranslated; it is never emitted with a disguised outcome. This needs C2 or ROOT for grant 2 or U3.
- **F4. Memory (U4).** `OrdinarySeed` (ids, the finding's sentence and fired list, and cloned failure payloads) is allocated only while an observer is installed. It is not in the adapter capacity arrays, which would change receipt bytes, so U4's M must count it, together with the serializer's JSON working set.
- **F5. The D6a list and the method token are pinned only by the committed bytes, not by the readers.** The readers allow any resolvable refs, so mutants M09, M10 and M20 are killed by control 2 alone.
- **F6. Conservation and stage gates are defensive at the producer.** Native records are exact in every observed run. The fault and inconsistency branches are exercised by injecting into cloned records: `stages` slots, `shared_work` and `stop_rule_work` are public fields. The invocation meter cannot be faulted from PP (its fields are private), so C09 is killed by the source guard.

## Not done (grant 2 or elsewhere, as ruled)

- G-i's closed translations, D38's unavailable representation, refusal and failure-work owner binding, and multi-case work (wider F2a).
- Facade installation, precommit validation through a runtime result_export dependency, and the frozen-candidate split (U3).
