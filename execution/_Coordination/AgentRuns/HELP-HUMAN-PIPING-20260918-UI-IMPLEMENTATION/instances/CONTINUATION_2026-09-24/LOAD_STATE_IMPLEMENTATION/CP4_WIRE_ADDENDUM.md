# CP4 wire addendum: selection that cannot finalize, and reader tightening

This addendum supplements these files, all unchanged:

| File | sha256 |
|---|---|
| `CP2_WIRE.md` | `81a7adbaa212517c518a61c5ab54cd7d9444adaee6b7f0dbec8b7b6efd168996` |
| `CP2_WIRE_ADDENDUM_1.md` | `c389f5e3878c6a32d8c72b80d1374ea6f30c4350c2870ab7ac653972df3dc760` |
| `CP2_WIRE_ADDENDUM_2.md` | `ec66628ef8db1ac70500b80abbcc754b82133fb8d97d4cf0f41ecd4a32ab9133` |
| `CP3_WIRE_ADDENDUM.md` | `f69043b682d027cc7f15e2173b932f016cd76f3a64d76b76d974cd0f0105f7d4` |

It records ROOT's checkpoint-3 dispositions (`REVIEW_CHECKPOINT_3/ROOT_DISPOSITION.md`, sha256 `5d8ffa84206efedfc51b86e4f2ef0aca32d5a1f43b443d5d899fcbb30289594f`) as implemented at checkpoint 4. Paths are WORKING_ROOT-relative. Everything here applies only to 0.4.0 (`resolved_straight_load_state_v1`) invocations. Pre-0.4 bytes and meanings are unchanged.

## 1. Correction: a case that is not selected, and a selection that cannot finalize (SF-1)

Two earlier statements were not true at checkpoint 3:

- `CHECKPOINT_3.md` line 43: "A case that is not selected publishes its ordinary response…"
- `CP2_WIRE_ADDENDUM_2.md` line 102: "Otherwise the attempt fails … and the case publishes its ordinary response."

Both failed in two situations:

- another case of the same invocation selected the join;
- the selected case could not finalize.

In either situation, checkpoint 3 published a blocked `load-reference-1` with no results, or the value route returned `Err("SOURCE_BLOCKS_FINALIZATION_FAILED")`. The independent CP3 review showed both: its budget-cliff probe P9, and its composite probe P3 with a sensitive pressure case.

From checkpoint 4, both statements are true, with these rules.

### 1.1 Replay reservation before selection

A 0.4.0 case's successful retained-source attempt is selected only if its live charge `c` leaves at least `c` of the attempt's limit `L` for captured replay: `c ≤ L − c`.

Captured replay continues the same attempt ledger, and it repeats that attempt's source closure and exact solve. At checkpoint 4 its measured charge is at most the live charge: on the committed `eigen_motion` witness, 3,359,828 against 3,360,029.

If the reservation fails, the attempt is declined as a budget refusal:

- `stage` is "captured replay reservation";
- the error is `Exact(Budget)`;
- `work` is `{charged: c, rejected: c, limit: L}`.

The case then follows the existing unavailable path: the `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` info diagnostic, `retained_source_attempt=unavailable`, and its ordinary response.

The reservation covers the replay only. It does not guarantee the later finalization stages (receipt reservation, current-source binding, derived rows). §1.2 covers those.

### 1.2 Fallback when a selected join cannot finalize

The invocation is republished on the ordinary route if either of these happens:

- a selected case's finalization fails (formerly the blocking `SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED` at `diagnostic:source-recovery:<case>:finalization`);
- the invocation receipt cannot finalize (formerly the blocking `diagnostic:source-recovery:publication`, or the value route's `Err`).

That applies, for example, when a joined envelope contains a case that is neither selected nor ordinary `checks_passed`.

The producer then runs the same captured invocation again, in a fresh ledger with the same limits. Executed work is not refunded; the first run's work is simply not published. In this republication:

- every retained-source attempt still runs, so each case keeps its own attempt facts;
- any successful attempt is declined with `stage` "invocation join withheld";
- every case publishes its ordinary response;
- the envelope is `load-reference-1`, with its unchanged table, hash and profile, and no `source_block_recovery`;
- each `load_reference_states[i].source_recovery` is `{status: "not_joined", code: "LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED"}`, and `solve.recovery_method` is the ordinary method;
- each case whose attempt was declined or failed carries `retained_source_attempt=unavailable` and one `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` info diagnostic. Its message names the recorded cause: "…its selected join could not finalize: case <id>: …" or "…: invocation receipt: …". A case whose ordinary checks passed keeps `not_required_ordinary_checks_passed`.
- Numerical qualification still governs Current eligibility. A sensitive ordinary response stays `sensitive`.

If a case's ordinary attempt was itself rejected, its ordinary route is the existing blocked solve. The fallback does not create a result for it.

### 1.3 What stays the same

- The two published identities and when each appears (ADDENDUM_2 §5.1). `load-reference-source-1` still appears only when at least one case publishes a selected response and the receipt finalizes. It and its profile `resolved_straight_load_state_source_v1` remain ROOT-reserved and inactive.
- The per-case record shapes and diagnostics (ADDENDUM_2 §5.3), and the receipt (§5.4).
- The typed route (no capture), which never attempts retained source.
- Every committed producer raw: all 34 regenerate identically (`_run_records/session3/cp4_regen_compare.log`).
- The pre-0.4 physics-source-1 composite outcome: `Err("SOURCE_BLOCKS_FINALIZATION_FAILED")` when a joined envelope's other case is sensitive and refused (reviewer probe P12). It is a separate open finding assigned to T3. A test pins it as a characterization.

## 2. Reader tightening (N-2, N-3)

Both evidence readers of `load-reference-1` (Rust `core/reporting/result_export/src/load_reference.rs` and Python `core/analysis_runs/load_reference_evidence.py`) apply the following. The shared cases are in `core/reporting/result_export/tests/fixtures/load_reference_mutations.json`.

**Numbers.** Any finite JSON number is a number, including integer literals outside i64/u64. Its value is the binary64 that serde_json `as_f64` gives, and later equalities compare those values. Index fields stay non-negative integers ≤ 2^64 − 1. A literal that overflows binary64 is refused: at the text boundary in Rust, and as `SOURCE_LOAD_REFERENCE_NUMBER_INVALID` in Python.

The canonical carrier profile `openpipestress_jcs_ijson_v1` still refuses integral magnitudes above 2^53 − 1 when a canonical document or AnalysisRun is materialized. A reader's acceptance does not change that.

**Law segments** (`CP3_WIRE_ADDENDUM.md` §1, now enforced). In each entry of `consumed_law_segments` and `consulted_law_segments`:
- `upper_index == lower_index + 1`;
- `interpolation_sample` has `start_k == end_k`, and `integration_interval` has `start_k < end_k`; otherwise the code is `SOURCE_LOAD_REFERENCE_LAW_SEGMENT`;
- an entry equal on all five keys to an earlier entry *of the same list* gives `SOURCE_LOAD_REFERENCE_LAW_SEGMENT_DUPLICATE` (new). The two lists are checked independently.

The `[T_lower, T_upper]` containment of `start_k` and `end_k` is not checked by the readers.
