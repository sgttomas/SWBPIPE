# I62 review repair 07, phase B1: Python repairs, and snapshot 07 staged

**Basis:**
- ROOT_RULINGS_V1: D1–D16 and "Checkpoint A: rulings on the native facts for snapshot 07" (NUM `ff570d05d8`);
- "Rust and TypeScript phase 1 complete; three readings settled" (NUM `dc73fa402e`);
- BRIEFS/I62_I64_REVIEW_REPAIR_07.md.

**Run window:** 2026-10-03T23:38:38Z (first tool call) to 2026-10-04T00:06Z, about 28 minutes of the 2-hour box. The memory guard (PID 5387) was running.

**Limits held:** no Git writes, no Cargo, no installs, no native job. READER writes are only the two Python files below. Snapshot 07 is staged in WT/scratch, **not installed in READER**.

## Changed READER files

| File | Before (`6b607fd01f`) | Now |
|---|---|---|
| P/core/analysis_runs/retained_precision.py | 55736ea65a… | dddac2fa96adc1e148cab4e305739cf7055873f515051b5151dc4ccf56bdf752 (110318 B) |
| P/tests/test_retained_precision_contract.py | 354821ddf4… | 5fb053fa46801195e3cf2d171230e77f8962e9620e5dcf395f703409981a4c98 (62440 B) |

The schema, corpus (06d `d02701ed6a`) and schema test are unchanged in READER.

## Commands and results

**On 06d in READER** (VENV python, both BIN variables): `pytest -q -rA tests/test_retained_precision_contract.py tests/test_retained_precision_schema.py` gave **266 passed, 1 failed** (`pytest_b1.log`).
- **The one failure** is `test_shared_draft_first_failure_controls[prefix_attached_old_input_unbound]`. It now fails at G5 WORK, not G8.
- **This is the ruled R4 move** (checkpoint A, D8: "If S2's R4 is adopted, the first of them moves from G8 to G5 WORK").
- **It clears with 07:** ROOT's ruling rebases the entry onto F′, where it keeps G8. In staged 07 it observes G8 PREPARATION.
- **No other 06d expectation changes.** All 15 bases, all 18 must-pass entries and the other 177 mutations hold.

**RV78 PROBES.json (31 probes):** 31 of 31 observe the ruled expectation. They are now reader-local tests: `test_rv78_probe_relations[*]`.

**RV79's probe script (27 probes):** every probe agrees with the rulings.
- `T4a_diagnostic_ref_not_naming_case` passes, as D6a now requires. RV79's reading is superseded.
- `PR2_empty_coverage_and_empty_inventory` stays at G5a. The D1 empty-inventory conditional is not met, so G3 does not reject it.

**RV79's surviving mutants**, rerun against the new tests (`mutants_b1.txt`):
- **Killed:** M06, M09, M11, M14, M18 and M22.
- **Still surviving:**
  - M21 (the 1+2^-40 sanity factor) needs a native scale at that margin, and there is no faithful vector.
  - M05, M12 and M17 are the deferred-base survivors (RV79-N6).

**Staged 07, in memory** (repaired Python, the staged schema monkeypatched in for `_schema`): **15/15 bases pass** with their expected classifications, **234/234 mutations** fail at their expected first failure and **19/19 must-pass** entries pass (`PYTHON_OUTCOMES_07_STAGED.json`).
- **The 195 carried 06d entries** keep their 06d outcome under the 06d Python reader with the 07 schema and bases.
- **All 57 new entries (56 mutations and 1 must-pass) were also run through the 06d Python reader.** Twenty-nine of them were accepted or reported at a different gate or code, so they pin the new rules. Example: `prepared_failure_cause_without_own_attempt` passes in the 06d reader. The other 28 keep the 06d reader's code but now pin a relation that had no shared entry. The full comparison is in `stage07_new_vs_06d_reader.json`.

## Decision → status → test

| Decision | Status | Pinned by |
|---|---|---|
| D1 run ids / execution-order bijection at G3 | implemented (moved from G5) | probes R1a, R1b; 07 `execution_order_swapped`, `run_id_not_execution_position` |
| D1 sourced complete old = source member map, at G3 | implemented (moved from G5 PA) | R3, R3b; 07 `complete_old_short_of_source`, `complete_old_longer_than_source` |
| D1 unsourced complete = every CaseSource's count (G3); else the invocation's pipe count (G8) | implemented. The non-empty clause is withdrawn. | R2b, RV79 R3b; the G8 branch has no base (every base has a CaseSource) |
| D1 member ids `0..len−1`; `captured_prefix` member part at G3, rest at G5 PA | already compliant | R2, R2b |
| D2 G0 union, plus settled readings 1–2: thresholds, canonicalization, `receipt_version` exactly 1 (typed int), absent receipt or body, typed malformed producer (RV79-N2) | implemented | `test_g0_union_d2`; 07 `g0_*` (6) |
| D3 class order: ordinary, rcond and selected-case checks in class 2 (`_g5_ordinary`) | implemented | probes T4d; RV79 D1–D3; 07 `ordinary_dangling_plus_adapter_fault`, `rcond_label_plus_adapter_fault` |
| D3 class-1 convention: native WORK collected and raised at the end of class 1 | implemented (`_g5_native` wraps `_g5_native_checks`) | `test_class1_attempt_defect_wins_over_native_work_d3`; 07 `native_attempt_defect_after_native_work_defect` |
| Settled reading 3: computation faults inside WORK equations | already compliant. Python's integer sums never raise. A sum beyond the safe range, or a negative fragment difference, is the WORK predicate failing, deferred with its class. Dangling build refs are deferred WORK, and the dependent build is skipped. | same; `dangling_build_ref` |
| D4a sourced attempt ⇒ `source.preparation.attempt_ref` = attempt | implemented | `test_association_d4`; 07 `source_preparation_null`, `unavailable_source_backref_foreign` |
| D4b attempt basis = ordinary attempt's basis | implemented | R5; 07 `attempt_basis_not_ordinary` |
| D4c prepared_product_failure case owns that attempt (case pass at the start of class 2) | implemented | 07 `prepared_failure_cause_without_own_attempt` (06d reader: pass), `prepared_failure_cause_foreign_attempt` |
| D4d preparation ⇒ no Run and preparation failed; native ⇒ nonselected Run, same run_ref | implemented | R6a; RV79 R6c; 07 `native_error_with_selected_run`, `preparation_error_with_selected_run`. Native with a *nonselected* Run has no base (deferred). |
| D4e `run_ref` null ⇔ no Run | implemented | 07 `run_ref_null_with_case_run` |
| D5a candidate record: no verification, no v-build, `verification_lme` 0 | implemented | T3; 07 `candidate_record_with_verification` |
| D5b escalating verification failure shows no pass work | implemented | T1; 07 `escalating_failed_verification_pass_entered` |
| D5c `verification_failed` only with the failed phase | implemented | 07 `rejected_verification_failed_with_completed_verification` |
| D5d stop-rule quantity resolves in the layout (same body and kind) | implemented | T2; 07 `stop_rule_quantity_other_body` |
| D5e group call exists; sources unique and in the call | implemented | R8; 07 `group_call_out_of_range` |
| D6a untyped refs unique and resolve (no names-the-case) | implemented | T4a, T4b; 07 `ordinary_diagnostic_ref_duplicate`, `_dangling` |
| D6b selected quality ∈ {sensitive, unresolved, failed} | implemented | T4e; `test_ordinary_pass_d6`; 07 `selected_case_ordinary_checks_passed` |
| D6c W2 trigger = initial kind and error; published exponent ≠ 0 | implemented | `test_ordinary_pass_d6` (reader logic; no base has W2) |
| D6d `legacy_source.work_ref` resolves, same case (ATTEMPT) | implemented | `test_ordinary_pass_d6` (reader logic; no base) |
| D7 retained diagnostics name exactly one requested case | implemented | `test_g4_…_d7`; 07 `retained_diagnostic_names_unrequested_case` |
| D8 R1′ | unchanged | 06d R1 pins |
| D8 R2′ (lost or OperationalError accounting, by location) | implemented | Q4; 07 `old_operational_accounting_not_lost` |
| D8 R3′ (three spellings, owner scopes: member, lane, values completion, proof) | implemented | Q1, Q2; `test_accounting_rules_d8_and_r3_both` (`both`, lane-scope binding); 07 `nested_stop_…`, `view_work_…` |
| D8 R4 (SectionError accounting needs a non-exact member PrepWork status) | implemented | B_section; 07 `section_accounting_exact_status`; rebase of `prefix_attached_old_input_unbound` |
| D8 kernel scope: `work_accounting` stop or reason in a Run, build or group refusal ⇒ G5 ATTEMPT class 1 | implemented (start of class 1) | 06c WorkAccounting pins |
| D10 G5b arithmetic fault ⇒ G5b SCALE; counters must be JSON integers | implemented (phase-labelled fallback; `type is int`) | `test_g5b_…`, `test_g2_…`; 07 `g5b_zero_section_area` |
| D13 pins | implemented | theta +0 on a no-data body; the Ceiling after a p128 verification-solve failure; R3 `both`; the 2^-988 switch on both sides; the strict bracket (equal-E probes) |
| D16 dangling references take their check's code | implemented | class 1: `dangling_candidate_record` (ATTEMPT), `dangling_build_ref` (WORK). Class 2: `ordinary_diagnostic_ref_dangling` (ATTEMPT), `dangling_ordinary_attempt_ref` (PRODUCT_ATTEMPT). The strict-bracket `next` is now an explicit G8 check. |
| Fail-closed fallback | retained only as a fallback. It also catches `AttributeError`. | — |

**D16, classes 3 and 4:** Python's typed checks (`_g5_typed`) and C3 work equations read no reference field, so they have no dangling-reference input to pin.
- The only reference near class 3, `native{run_ref}`, is checked by D4d in class 2.
- R3′/R4 owners are positions (lane index, last member), not references.

## Snapshot 07, staged (WT/scratch/i62_review_repair_07/stage/)

| Staged file | sha256 | Bytes |
|---|---|---|
| fixtures/results/retained_precision_cases.json | cb148a0f7d30dac7496d7ad8fc9000fd6607ea056a79557ac4026c89d4534381 | 4745454 |
| schemas/retained_precision_mp_v2.schema.json | 07951edacfedd410c153929ee75bb5bada15dbd222369ec63240c678b233b61c | 177115 |

**Counts:** 15 cases, 234 mutations (178 + 56 new), 19 must-pass entries (18 + 1).

**Schema (D9):**
- Refusal has five variants: `work_accounting` and `count_range` are removed.
- The unavailable Case is split into two closed branches:
  - `source_ref: null`, with optional `source_decline`;
  - `source_ref: U`, with no `source_decline`.

  `source_ref` is required in both.
- The top-level `$comment` records that the integer and bit constraints are G2, that G5 enforces the emitted domain (RV78-N8), and D9b.

**Bases:**
- **P′ case 1** gains `source_ref: null` (rehashed in full).
- **`prefix_attached_old_input_unbound` is rebased** onto F′ with the single edit `product_attempts[1].operational.old[0].inputs[9]="3ff0000000000000"`. It observes G8 PREPARATION at the same `_g8` check.

**New entries (56 mutations, 1 must-pass), each carrying its ruled gate and code:**
- **From RV78 PROBES.json (25):** the S1/B1 relations; R4, R5 and R6a; R8; T1–T3; T4a, T4b, T4d and T4e; R2b and R3b; the D8 pins Q1, Q2, Q4 and B_section; the four equal-E strict-bracket variants.
- **RV79, D4 and S4 pins (14):** D4a, D4c (two), D4d and D4e; P8 on F′ and on P′; N13 (two); O1, N1, N7, W2 and W3.
- **Settled-reading and D9 pins (11):**
  - D7: one;
  - G0: six (the two thresholds, `receipt_version`, canonicalization, and the absent receipt and absent body);
  - D9 G1: four.
- **D3, D16 and D10 pins (6):** two class-order pins, three dangling-reference pins (class 1 ATTEMPT and WORK; class 2 PRODUCT_ATTEMPT) and `g5b_zero_section_area`.
- **New must-pass:** `interpolation_equal_e_bracketed_control`.
- No new entry needs `expected_by_reader`. G7 is unchanged.

**Harness format** (shared semantics for all readers):
- Only `rehash:"all"` is admitted. The Python harness asserts it.
- When an entry removes `retained_precision` or its body (the G0 pins), there is nothing to rehash, and the harness skips the rehash.

**Not built (deferred):**
- T4c, which needs post-rehash edits that the format does not admit. Its G1 integrity relation is pinned elsewhere.
- N11 (a refused group), D4d native with a nonselected Run, D6c/D6d shared pins, and the D1 G8 no-CaseSource branch: no faithful base exists.
- The unsourced attached-member variant of P7 (checkpoint A).
- M21, M05, M12 and M17: no vector, or a deferred base.

## Remaining known differences from Rust and TypeScript

These were read from I63 RETURN (`dd684fcbeb`) and I64 RETURN_FOLLOWUP. I did not run either reader here.

1. **D8 and D9 are not yet in Rust or TypeScript.** Both readers take them in phase 2 with 07. Until then, the 07 pins for R2′, R3′, R4, the kernel scope, D9a and D9b separate Python from them.
2. **`g5b_zero_section_area`.** Python reports G5b SCALE through its phase-labelled fallback (D10). Rust and TypeScript divide in binary64. RV79 expects them to fail G5b, but this is unconfirmed until phase 2 runs 07.
3. **The fail-closed fallback code** differs: Python's G5 fallback is PRODUCT_ATTEMPT, and TypeScript's native fallback is ATTEMPT. With every reference explicit, none of the three readers knows an input that reaches it.
4. **Order inside class 2.** Python runs the ordinary pass, then the D4c case pass, then the per-attempt association. Rust also runs D4c in a case pass at the start of class 2. A dual ordinary + association defect is not pinned, and C3:304 does not order them.
5. **Kernel-scope placement.** Python checks it at the start of class 1, before the call loop. All codes are ATTEMPT, so first codes cannot differ.
6. **D10 integral floats (G2)** is Python-only by ruling. No shared pin.

No other known differences.
