# I63 return: Rust reader aligned to snapshot 06c

I63 is a TASK (Type 2). ROOT (HELP_HUMAN) granted this work directly in the session. The basis is the rulings "Snapshot 06b verified; WorkAccounting terminals cannot appear in a receipt; invocation edits added to the shared format" and "Snapshot 06c verified; readers align; product-level accounting causes to be settled" (NUM `133d547b8b`). It ran under the same fence, command, target and rules as `BRIEFS/I63_I64_COVERAGE_READERS.md`. I63 had no descendants.

- **Run:** first tool call 2026-10-03T22:14:13Z; freeze about 22:20Z, inside the 60-minute box. The memory guard (PID 5387) was running.
- **Limits held:** no Git writes or index operations; no install, new tooling, solver, native, UI or DEC-025 job; one Cargo job at a time, each under a 1,200 s wall.
- **Basis files:** READER at `c765e4f5b3`.
- **Paths** use the brief's placeholders.
- **Status:** all 06c checks pass. **Not accepted; eligibility still held** (`IMPLEMENTATION_COMPLETE = false`).

## Inputs verified at start

- **Shared files:** all five match SHARED_SNAPSHOT_06C (`2e82e61861`). SHA256SUMS_C2_2 and SHA256SUMS_C2_3 verify.

  | File | sha256 prefix |
  |---|---|
  | corpus | `d4235f59b6` |
  | schema | `f943ebd351` |
  | definition | `3e0779a45a` |
  | preview table | `c74742ce6a` |
  | results yaml | `4585a45fcf` |

- **Counts:** 15 cases, 173 mutations and 23 must-pass entries.
- **Python reference (read only):** `3200f56020`.

## Changed files (READER, inside the fence)

Paths are under P/core/reporting/result_export.

| File | Before (`5467f46e7b`) | After |
|---|---|---|
| src/retained_precision.rs | 20db5327e13a… | e131a6b9a20f26dec770977ae2ee891a55ab6d8b42eabe631582b6797e8bd381 (151402 B) |
| tests/retained_precision_contract.rs | f8313258e618… | 10818998018c8b5aa7b7700e9d656904f8b5aba3499e07760bf00c24cb3d425f (64759 B) |
| src/lib.rs | 375b073135… | unchanged |

### Source changes

1. **G7.** The provisional remap is gone. A new `base_error` makes the leading `[A-Z][A-Z0-9_]*` token of the unchanged Rust base text the bare code, and carries the full text as `detail` only when it says more. `g7_maximum_off_enclosure` now gives `SOURCE_PREVIEW_PHYSICS_EXTREMA_BOUNDS`, which is Rust's `expected_by_reader`.
2. **P7.** The sourceless all-old binding is removed. Every attempt still binds the old tuple of each attached PreparedMember, through the existing PreparedMember loop. Unattached old entries are attestations, so must-pass `prefix_unattached_old_operand_attested` now passes and `prefix_attached_old_input_unbound` fails at G8.
3. **WorkAccounting.** At the top of `g5_schedule`, any unresolved `work_accounting` terminal fails at G5 ATTEMPT, idle or not. That is the native schedule/terminal class, ahead of C3 association.
4. **N10 idle rules,** in `g5_schedule` given the statement body:
   - an idle Run with no group must be exactly Budget(invocation), with `invocation_before` ≥ Li;
   - an idle Run in a ready group must be refused with `ledger_unavailable`.
   
   The existing exhaustion and refused-group rules are kept.
5. **N5/N8/N9 in the replay,** mirroring Python's `_g5_schedule` at 06c:
   - a terminal stop must end on its exact `terminal()` translation (`terminal_of`, adaptive.rs:4349–4378);
   - an escalating last stop with slots left has no emitted terminal, so it fails (N9, `ceiling_before_last_slot`);
   - leaving the ladder past the last slot must be the Ceiling.
   
   An escalating failed verification solve now advances two slots even when it is the last attempt, as in Python.
6. **Unchanged:** N17 (case-first: Budget(case) needs case_charge > Lc; Budget(invocation) needs case_charge ≤ Lc and after > Li, which settles 06a divergence 3), the strict O2 rule, P5 over every conversion, and the reader-logic hooks.

### Harness and test changes

- **`apply_entry`** implements `invocation_edits` exactly per SHARED_SNAPSHOT_06C `format_change`:
  1. edit a copy of the source;
  2. edit a copy of the invocation;
  3. if there are invocation edits, set `body.invocation.value` = `domain_hash("source_blocks_invocation_v1", edited invocation)`;
  4. rehash per `rehash`;
  5. validate against the edited invocation.
- **`expected_for`** uses `expected_by_reader.rust` when present.
- The shared, observation and must-pass tests use both helpers.
- The per-snapshot tables are one `slice_outcomes` helper: 30..77, 77..104, 104..121, 121..150, 150..163 and 163..173. They assert 173 mutations, and must-pass asserts 23.
- **New `work_accounting_mutations_fail_at_the_terminal_rule`:** for each of the three WorkAccounting mutations, the schedule replay alone rejects the WorkAccounting Run.
- The idle reader-logic test now also rejects an idle WorkAccounting Run.
- The G7 local check expects the Rust base code.

## Commands and results

Every run used the brief's command and environment variables plus the disclosed `DEVELOPER_DIR`.

| Run | State | Result |
|---|---|---|
| run1 | all changes | 21 passed, 0 failed |
| run2 (probe, `shared_` tests only) | the previous (06a) reader source under the new harness; source restored and hash-verified at `e131a6b9a2` | failed on exactly these, the ones this grant changes: `prefix_unattached_old_operand_attested` (must-pass rejected at G8), `g7_maximum_off_enclosure` (remapped code), `ceiling_before_last_slot`, `terminal_stop_wrong_translation`, `terminal_refusal_wrong_kind`, `idle_ready_group_not_ledger_refusal` (each reached a later PRODUCT_ATTEMPT). The three WorkAccounting mutations already gave ATTEMPT there through other checks; the new test pins them to the terminal rule |
| run3 (**final, full command**) | final bytes | **22 passed, 0 failed** |
| run4 | final bytes, `--nocapture snapshot_0 shared_must_pass` | 7 passed; tables captured |

**Against the bar:**
- **Mutations:** all 173 match this reader's expected first gate and code, using the per-reader expectation for G7.
  - 06b slice: G5 ATTEMPT 8, G8 3, G1 1, G5 PRODUCT_ATTEMPT 1.
  - 06c slice: G8 7, G5 ATTEMPT 3.
- **must_pass:** all 23 validate with the base case's classifications.
- **Cases:** all 15 validate.
- **Earlier tests:** all pass.
- **Tables:** `OUTCOMES_06C.json`.
- **No expected outcome looks wrong.**

## Checklist status (Rust, 06c)

Rows not listed are unchanged from `reader_audit_06a/RETURN.md`.

| ID | Rust status | Check or evidence |
|---|---|---|
| N5 | checked: exact `terminal()` translation only | `terminal_stop_wrong_translation`, `terminal_refusal_wrong_kind`, `escalating_end_not_a_terminal_translation`; reader-logic test |
| N8 | checked (logic): past the last slot, only the Ceiling | reader-logic Ceiling test; `work_accounting_at_last_slot`. Shared positive base **deferred** |
| N9 | checked: no WorkAccounting terminal anywhere; an escalating end with slots left has no emitted terminal | `ceiling_before_last_slot`, `escalating_end_not_a_terminal_translation`, `work_accounting_after_escalating_stop`, `work_accounting_at_last_slot`; `work_accounting_mutations_fail_at_the_terminal_rule` |
| N10 | checked: an idle group-null Run is Budget(invocation) with `invocation_before` ≥ Li; idle WorkAccounting is rejected; idle in a ready group is refused `ledger_unavailable` | `idle_budget_below_invocation_limit`, `idle_ready_group_not_ledger_refusal`, `idle_work_accounting_run`; reader-logic test. Exhausted shared base **deferred** |
| N12 | checked by the G1 schema (`prior` off the wire); the replay reads the Reason structure | `work_accounting_prior_not_on_wire` |
| N17 | checked in code (case-first rule); matches Python | no shared base (**deferred**) |
| C5 | checked | `group_sources_out_of_order`, `distinct_stiffness_merged_group`; base `two_case_two_groups_synthetic` |
| O2 | checked (strict, as ruled) | `formation_d5_dangling_diagnostic`, `report_reference_unrelated_diagnostic` |
| P5 | checked (every Conversion) | `conversion_*`, `refused_member_conversion_kind_bits` |
| P6 | checked | 06a, plus must-pass `values_failed_separate_completion`, `aliases_abandoned`, `bind_rows_abandoned` |
| P7 | checked (attached entries, every attempt) | `prefix_attached_old_input_unbound`; must-pass `prefix_unattached_old_operand_attested` |
| P9 | checked | 06a, plus must-pass `observables_failed_after_certificate`, `g5a_failed_after_certificate` |
| G7 | per-language base code (`SOURCE_PREVIEW_PHYSICS_EXTREMA_BOUNDS`) | `g7_maximum_off_enclosure` (`expected_by_reader.rust`); local G7 check |
| G8 material/units | checked, including the invocation-level refusals through `invocation_edits` | `named_point_value_mismatch`, `interpolation_target_mismatch`, `mm_unnormalized_coordinate`, `interpolation_missing_alpha`, `interpolation_duplicate_temperature`, `interpolation_target_{below_range,at_lower_point,at_upper_point,above_range}`; bases I and M |

These are unchanged and still match: N1–N4, N6, N7, N11 (implemented; no base), N13–N16, C1–C4, C6, O1, O3–O5, P1–P4, P8, P10, P11, W1–W3, and W4 (not publicly checkable).

## Host

The Xcode licence is still unaccepted, so every cargo run set `DEVELOPER_DIR=/Library/Developer/CommandLineTools` for the process only. No system setting changed.

## Open

- **Remaining divergences from Python: none known on 06c.** The 06a divergences are settled: G7 per language, P5 all conversions, O2 strict, and the N17 case-first rule.
- **For ROOT and I62:** the product-level `work_accounting` question (whether must-pass entries such as `cert_failed_before_summary` are emittable), and the schema/contract tension over the WorkAccounting shape. I63 implements neither.
- **Not claimed:** acceptance, eligibility, three-reader parity or independent review. I63 ran no Python or TypeScript.

## Files read (sha256)

These are in addition to the earlier I63 RETURNs.

| sha256 | File |
|---|---|
| d7cc260cff250c648c2e53437a63ce160b2519b4979d43b1bfb63ca14667dd39 | R/I62/coverage_shared_python_01/RETURN_C2_2.md |
| 0d78d88559dfb8086a8e185b6e5102c109ed6f6889e2d190f25a140387c48026 | R/I62/coverage_shared_python_01/RETURN_C2_3.md |
| 1ddd8146ed3724bb9cb94403b52955a3a7d8b9dfd884156371fd300e9b3c1932 | R/I62/coverage_shared_python_01/SHARED_SNAPSHOT_06B.json (format_change, counts) |
| 2e82e61861b983ee32a2abf20c12543a792fd2522629cc49ad753d682eab6d52 | R/I62/coverage_shared_python_01/SHARED_SNAPSHOT_06C.json (format_change, counts, files) |
| 9e00ed1f2ae3c5c49dc2528edd336d6c066028e755f308b28f587a6d70f7c596 | R/I62/coverage_shared_python_01/SHA256SUMS_C2_2 |
| 13c25ef5018c56fda7ba46bcf2e7484bcb40e320aab2b6a2df6d607624e15bfc | R/I62/coverage_shared_python_01/SHA256SUMS_C2_3 |
| e78c519779085714a589a2855d3bc999ff90b31a3e0b608bd950854b7d4fad4c | T3/ROOT_RULINGS_V1.md (the 06b and 06c sections) |
| 3200f560201bff2c3aed7fb5730dd8925610abd3e0df15efd0b22cebd163242e | READER/P/core/analysis_runs/retained_precision.py (the `_stop_of`, `_terminal_of` and `_g5_schedule` sections; read only) |

## Bulk (WT/scratch/i63_reader_align_06c/)

| sha256 | bytes | file |
|---|---|---|
| b87eac1b68685f44cc7cac3d8b5408fc420ac55db18ce9671aeb1adee4849d64 | 11647 | I63_06C_DELTA_src.diff |
| 66cfee0d9e4b62b9f5ab1ff0a2d85006ed196b86f6bc2d74e7257a8cd350070c | 20421 | I63_06C_DELTA_test.diff |
| bfd4eb43043516d3b31a888fdd53ba67c6260ffdcd7f69493f7213818f952ce0 | 2023 | run1.log |
| f5b2f458621850ef50f31c97892a7b46010868f880cd8ee74de5d2187c768aa6 | 3877 | run2_probe_prev_reader.log |
| 71ab24c92d1c9d6a5b404c3d2055a722c37361bc8781fa398ca259d9ed765c78 | 2087 | run3.log (final) |
| 6666094ca231e9aa311d435ddb78d5b1b17e691994b38dedf5445e64d92433ee | 37839 | run4_outcomes.log |
| 20db5327e13aabcb63d4803f11b9f2d7e1e98fc6a64d558ebcbf2ce72680f68c | 148552 | before/retained_precision.rs (READER HEAD) |
| f8313258e618088f0f3793791bbe8087c37ee75829c3cd3ae0056c86dbec5b12 | 65119 | before/retained_precision_contract.rs (READER HEAD) |
| 375b07313518a0cdd09e317b83b9ca7536acef5c80fd4cd0a94920fd8bf2486d | 66051 | before/lib.rs |
| e131a6b9a20f26dec770977ae2ee891a55ab6d8b42eabe631582b6797e8bd381 | 151402 | final/retained_precision.rs |
| 10818998018c8b5aa7b7700e9d656904f8b5aba3499e07760bf00c24cb3d425f | 64759 | final/retained_precision_contract.rs |
