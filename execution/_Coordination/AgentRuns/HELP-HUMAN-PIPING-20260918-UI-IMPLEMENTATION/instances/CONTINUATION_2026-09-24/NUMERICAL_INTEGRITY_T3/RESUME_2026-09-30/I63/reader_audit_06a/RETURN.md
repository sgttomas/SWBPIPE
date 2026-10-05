# I63 return: Rust reader audited against the 42-item G5 checklist and aligned to snapshot 06a

I63 is a TASK (Type 2). ROOT (HELP_HUMAN) granted this work directly in the session. The basis is the rulings "Reader audit plan approved: one 42-item G5 checklist and snapshot 06 in two grants" and "Snapshot 06a and the Python checklist audit verified" (NUM `498c197fa0`). It ran under the same fence, command, target and rules as `BRIEFS/I63_I64_COVERAGE_READERS.md`. I63 had no descendants.

- **Run:** first tool call 2026-10-03T21:26:44Z; freeze about 21:40Z, inside the 90-minute box. The memory guard (PID 5387) was running.
- **Limits held:** no Git writes or index operations; no install, new tooling, solver, native, UI or DEC-025 job; one Cargo job at a time, each under a 1,200 s wall.
- **Basis files:** READER at `e7dac8d4d9`.
- **Paths** use the brief's placeholders.
- **Status:** all 06a checks pass. **Not accepted; eligibility still held** (`IMPLEMENTATION_COMPLETE = false`).

## Inputs verified at start

- **Shared files:** all five match SHARED_SNAPSHOT_06A (`1d11fc08fc`), and SHA256SUMS_C2_1 verifies.

  | File | sha256 prefix |
  |---|---|
  | corpus | `58562c88dc` |
  | schema | `f943ebd351` |
  | definition | `3e0779a45a` |
  | preview table | `c74742ce6a` |
  | results yaml | `4585a45fcf` |

- **Counts:** 12 cases, 151 mutations and 16 must-pass entries.
- **Python reference (read only):** `5bb6357a36`, with tests at `514745e245`.

## Changed files (READER, inside the fence)

Paths are under P/core/reporting/result_export.

| File | Before (`3e915c0c1b` / 05c) | After |
|---|---|---|
| src/retained_precision.rs | 93fee7bd095b… | 20db5327e13aabcb63d4803f11b9f2d7e1e98fc6a64d558ebcbf2ce72680f68c (148552 B) |
| tests/retained_precision_contract.rs | 4abc640ca71e… | f8313258e618088f0f3793791bbe8087c37ee75829c3cd3ae0056c86dbec5b12 (65119 B) |
| src/lib.rs | 375b073135… | unchanged |

### Source changes

The gate order is unchanged. Within G5 the order follows C3:304, as ruled:
1. the native schedule, graph and native work (`g5_native`);
2. the ordinary checks (`g5_ordinary`);
3. the C3 association and stages, per attempt;
4. the P9 typed pass over all attempts;
5. the C3 WORK equations, deferred until every attempt's association checks are done.

The changes:
- **`g5_schedule` (N1–N10, N14):** a replay of the native ladder, called in each Run before the existing checks.
  - It checks the slot sequence and the escalation rules (one slot after a candidate failure, two after a failed verification solve).
  - A non-escalating failure, or a failure in the verification pass itself, is terminal.
  - A rejected candidate must hand on its verification while a slot remains, and an exhausted ladder must end in `unresolved/ceiling`.
  - A pre-schedule (idle) Run has no records, no charge, and a reasoned terminal that is not selected.
  - Corrections must be ≤ 3.
- **Native (`g5_native`):**
  - **C1:** a failed build fails its requesting record with the same stop.
  - **N10:** a Run entered with `invocation_before` ≥ Li is idle, with no group and the exact reason `budget(invocation)`.
  - **C5/N11:** groups are call-local, formed at the first equality of the stiffness hash, with sources in first-seen order. The Run's group must match, and a refused group means no attempts and a refused terminal carrying the group's reason.
- **O5 (`g5_ordinary`):** a `source_decline` must belong to an unavailable case with no source or Run, owned by that case and its material basis.
- **P2/P6 (`g5_stages`):**
  - pipeline stages after the first one that did not complete are all `not_entered`;
  - observables and G5a are entered together;
  - a source exists if and only if preparation completed;
  - completion follows the stages: values failed gives `separate_failure`; the certificate entered, maxima failed or aliases entered gives `merged`; a projection that did not complete gives `not_entered`.
- **P9 (typed pass):** an unavailable result's error kind must match the first failed stage.
- **P5:** the normal/subnormal kind-versus-bits rule moves from G2 to G5 PRODUCT_ATTEMPT (`conversion_kind_ok`). It applies to projection outcomes and to PreparedMember conversions.
- **P4 (G3):** projection-outcome indices must name hull-projected rows, ascending and unique. A valid subset on a Ready case still fails at G5.
- **P7/G8:** for an attempt without a source, every old operational tuple must bind its node positions and selected E/G to the invocation.
- **G7:** the bare base code becomes the error code, with detail carried separately.
  - `ValidationError` gains `detail: Option<String>`.
  - Metadata failures keep their base code.
  - Rust preview-evidence failures report `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID`, with the Rust base code as detail (see divergence 1).
- **Test-only entry points:** `#[doc(hidden)] pub mod reader_logic` exposes `schedule` and `ordinary` for the reader-logic tests. They run the same functions as `validate` and grant nothing.

### Test changes

- `snapshot_06a_mutation_outcomes`, with the tally and a printed table;
- `schedule_replay_terminal_branches_reader_logic` (N5, N6, N8, N10), mirroring Python's;
- `source_decline_relation_reader_logic` (O5), mirroring Python's;
- `g5_audit_local_controls`: P9, P6 and P2 controls on must-pass bases, plus a G7 code/detail check;
- the earlier counts moved to 151.

## Commands and results

Every run used the brief's command and environment variables plus the disclosed `DEVELOPER_DIR`.

| Run | State | Result |
|---|---|---|
| run1 | baseline on 06a | 12 passed, 3 failed |
| run2 | most changes in | 16 passed, 2 failed (G8 prefix not yet added) |
| run3 | plus the G8 prefix binding | 18 passed |
| run4 (**final, full command**) | plus the local controls | **19 passed, 0 failed** |
| run5 | final bytes, `--nocapture snapshot_0 shared_must_pass` | 5 passed; tables captured |

The baseline failures in run1 were 7 of the 30 new mutations plus I63's own count/tally assertions. The 7 were:
- `failed_build_reason_mismatch`
- `group_sources_out_of_order`
- `conversion_subnormal_kind_normal_bits`
- `conversion_normal_kind_subnormal_bits`
- `g7_maximum_off_enclosure`
- `row_index_foreign_support_norm`
- `prefix_old_inputs_unbound`

The other 23 already matched.

**Against the bar:**
- **Mutations:** all 151 match their expected first gate and code. The 06a tally: G5 ATTEMPT 10, G5 PRODUCT_ATTEMPT 10, G3 3, G8 3, G5 WORK 2, G2 1, G7 1.
- **must_pass:** all 16 validate with the base case's classifications.
- **Cases:** all 12 validate.
- **Earlier tests:** all pass.
- **Tables:** `OUTCOMES_06A.json`.
- **No expected outcome looks wrong.**

## Checklist status (Rust; same IDs as RETURN_C2_1)

"Existing" means present before this audit, from I59 or earlier work. "New" means added here.

| ID | Rust status | Check or evidence |
|---|---|---|
| N1 | checked (existing + replay) | record index = position, ≤4 records, ≤3 attempts, records empty iff attempts empty; reader-logic idle test |
| N2 | checked (existing + replay) | `schedule_fresh_first_p256` |
| N3 | checked (existing + replay) | `skip_after_failed_candidate_reused`, `_wrong_slot`; base K |
| N4 | checked (existing + replay) | `failed_verification_reused_as_candidate`, `_one_slot`; base V |
| N5 | checked (logic) | `schedule_replay_terminal_branches_reader_logic` |
| N6 | checked (existing + new closure) | p512 ladder (05b); reader-logic handoff test |
| N7 | checked (existing) | every selected base |
| N8 | checked (logic, new) | reader-logic positive Ceiling and wrong terminal. Shared base **deferred** |
| N9 | partly: the terminal kind must be unresolved or refused, with a reason | the WorkAccounting fault/prior is attested; shared row is C2-2 item 16 |
| N10 | checked (logic, new): idle shape; exhausted entry is idle `budget(invocation)`; group null implies exhaustion (existing) | reader-logic idle tests. Shared base **deferred** (≥60B of work) |
| N11 | implemented (existing per Run + new per group) | no control; no native-faithful base |
| N12 | checked by the G1 schema; the replay reads the Reason/Stop structure | — |
| N13 | checked (existing): candidate {accepted, rejected, failed}, verification {verified, solved, failed}, VtC completed | bases K and V |
| N14 | checked (existing + replay) | `corrections_above_three`, `physical_residual_basis` |
| N15 | checked (existing) | `certified_bound_unbound_drop_existing_g5` |
| N16 | checked (existing) | `physical_stop_stage_partition`, `*_stage_projection_preserves_total` |
| N17 | partly: fragments, sums, chaining and the selected final guard (existing); budget overshoot rule (existing; see divergence 3) | no shared base (**deferred**, ≥20B/60B) |
| C1 | checked (existing cache rules + new build-reason rule) | `failed_build_reason_mismatch`, `cached_failed_slot_rebuilt`; base S |
| C2 | checked (existing): slots, Σ build stages, build origin, backward reuse | all bases |
| C3 | checked (existing): cache_before/after equal the derived ordered cache | `failed_slot_not_cached` |
| C4 | checked (existing) | `native_work`; call chaining |
| C5 | checked (new) | `group_sources_out_of_order` |
| C6 | checked (existing, G3/G5) | all bases |
| O1 | checked (existing, G3/G5) | all bases |
| O2 | checked (existing; stricter: the ref must also affect the case and be listed) | `formation_d5_dangling_diagnostic` |
| O3 | checked (existing) | `w2_published_without_initial_failure` |
| O4 | checked (existing); `abandoned_at_selection` is excluded by the G1 schema enum | `legacy_source_dangling_diagnostic` |
| O5 | checked (logic, new) | `source_decline_relation_reader_logic`. Shared base **deferred** |
| P1 | checked (existing) | `rebind_source_run_only` |
| P2 | checked (existing + new: monotone pipeline, observables/G5a together, source iff preparation); no `entered` value under the G1 enum | `stage_entered_after_failure`, `certificate_stage_check_disagree`; local P2 control |
| P3 | checked (existing) | `lane_k_failed_with_coverage` |
| P4 | checked per ruling (new G3 hull rule + existing G5 Ready set) | `row_index_*` (4) |
| P5 | checked (moved to G5; see divergence 2) | `conversion_*` (5). Positive subnormal/underflow rows **deferred** |
| P6 | checked (existing + new completion rules) | `maxima_abandoned_separate_failure`; local P6 control |
| P7 | checked (existing G3 prefix shapes + new G8 sourceless binding) | 05a prefix bases; `prefix_old_inputs_unbound` |
| P8 | checked (existing reason/phase table) | bases F and P |
| P9 | checked (existing wrapper + new first-failed-stage rule) | `certificate_check_wrong_wrapper`; local P9 control |
| P10 | checked (existing, 05c) | the 05c coverage set |
| P11 | checked (existing) | `native_stage_disagrees_with_run` |
| W1 | checked (existing) | `product_work_only` |
| W2 | checked (existing): lane corrections ≤ 1, data capacity, projection conversions | lane and conversion counts |
| W3 | checked (existing) | — |
| W4 | not publicly checkable (attested) | — |
| G7 ruling | applied (code plus detail) | `g7_maximum_off_enclosure`; local G7 check. See divergence 1 |

## Divergences from Python, and contract readings, for ROOT

1. **The G7 code family (needs a ruling).** The ruling says to report "the bare base code", but the two base validators use different code families. Python's preview base reports every evidence failure as `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID: <detail>`. The unchanged Rust base reports finer codes, here `SOURCE_PREVIEW_PHYSICS_EXTREMA_BOUNDS`. Its literal bare code would therefore not match the shared expectation.
   - **What I did:** Rust runs the base metadata check first, keeping its code. It then runs the full base validator and reports any later (evidence) failure as `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID`, with the Rust code as `detail`. No base validator changed.
   - **What is not established:** parity is shown only for the one shared G7 mutation. Other Rust evidence codes, such as `ROW_SIGNATURE` or `DANGLING_RESULT_REF`, might sit outside Python's evidence step and carry a different code there.
   - **ROOT should confirm the family mapping,** or define the shared G7 expectation per code family.
2. **P5 scope.** Rust applies the kind-versus-bits rule at G5 to every PreparedMember conversion, including those of non-prepared members, and to every projection outcome. Python checks projection outcomes, plus normal ≥ MIN on prepared members only. So a malformed conversion on a non-prepared member fails G5 in Rust but passes in Python. The C3 P5 text reads as applying to every conversion. The metadata-only transport route no longer checks conversion kinds, because it runs G0–G2 only.
3. **N17 budget overshoot.** Rust's existing rule:
   - checks `case_over` for a case-scope Budget;
   - for invocation scope, requires `!case_over && (inv_over || idle-at-limit)`;
   - applies to Runs with no attempts too.
   
   Python requires `case_over` or `inv_over` only when attempts exist, and doesn't require `!case_over` for invocation scope. There is no shared base (deferred). ROOT should rule whether the native code checks the case limit before the invocation limit.
4. **O2 strictness.** Rust requires every diagnostic reference to name a diagnostic that affects the case and appears in `diagnostic_refs`. Python requires only that it resolve.
5. **Same as Python:** the order within G5 and the dual-defect order. The coverage-plus-WORK and native-WORK pins pass.

## Host

The Xcode licence is still unaccepted, so every cargo run set `DEVELOPER_DIR=/Library/Developer/CommandLineTools` for the process only. No system setting changed.

## Open

- **Deferred, as for Python:** the shared bases for N8, N10, O5, the N17 overshoot, positive subnormal rows, L = 0, the source-construction base, and the C2-2 items (4, 5, 11, 14, 15, 16).
- **Not claimed:** acceptance, eligibility, three-reader parity or independent review. I63 ran no Python or TypeScript.

## Files read (sha256)

These are in addition to the earlier I63 RETURNs.

| sha256 | File |
|---|---|
| ee3cc5918cec2a789116702fd4bdbce78dd0a243dde388b21b2b6828954a7790 | R/I62/coverage_shared_python_01/READER_AUDIT_PLAN.md |
| 06419414e214e0675110b9ffd2d4370130991d83bf79a77d9b3e9a346ca15935 | R/I62/coverage_shared_python_01/RETURN_C2_1.md |
| 1d11fc08fc17d5bbc088e70a53e0fbf0702a9ff02f8e6c644d969efb520208d2 | R/I62/coverage_shared_python_01/SHARED_SNAPSHOT_06A.json (keys, files, counts, new-mutation table) |
| 81e679dadb3b7e06e9d0d9f73f32fca7604557dc92f6feb1f83fee66423fcd40 | R/I62/coverage_shared_python_01/SHA256SUMS_C2_1 |
| c0f57b5f8611e4d22d90a260fddb506a8b8bd20a473dcdc772aa7670b54630ab | T3/ROOT_RULINGS_V1.md (the audit-plan, seam and 06a sections) |
| 5bb6357a3633d2693313ffc80396e4e4015a830800ba2320bc5456b8b28e1d14 | READER/P/core/analysis_runs/retained_precision.py (the G5 schedule, cache, native, stages, typed, ordinary, products, G7 and G8 prefix sections; read only) |
| 514745e24569ee28585bc0047370c900786d4bd79cacc3a1546cf3071fb7f9d3 | READER/P/tests/test_retained_precision_contract.py (reader-logic tests; read only) |
| (READER, unchanged) | P/core/reporting/result_export/src/preview_physics_evidence.rs and semantic_contract.rs (code families only) |

## Bulk (WT/scratch/i63_reader_audit_06a/)

| sha256 | bytes | file |
|---|---|---|
| 70d4ba3fcf02a77c44a240f19958ffc06975fab8c3ee52d9b07a0a8f35c302b9 | 19189 | I63_06A_DELTA_src.diff |
| 8028888816684939bc214cc9e04643cfc3733238b69de7363d604e7c6f7eadb2 | 12149 | I63_06A_DELTA_test.diff |
| 76ae6d016a591c5ed1420d25cc2c62a831b90d21aa75935eac38dd3cd47ced9a | 16046 | run1_baseline.log |
| 03a818ad2b4f6437883ba217911a91bee8018aefd131c76dcb023cc3f8735a92 | 10310 | run2.log |
| 9943784430f51a5a354f2837aa4704066c651d4ff197b01d99b00011fa4910fd | 1901 | run3.log |
| 845d52225261c625f6b85077d8042e30872c13af5c27cc93b36251445bc5080b | 1937 | run4.log (final) |
| 7fa2f527e9958cbdc662f90a275c6de74db6fa302fe5d10b4547e2df6ff83054 | 31619 | run5_outcomes.log |
| 93fee7bd095b18ca1b67074b71d80e56483a0f9358ee19ea711c86dcde0592dd | 137032 | before/retained_precision.rs |
| 4abc640ca71ee17388da85215ca98f22f6b40592b783a7192e9e57faefbdc7b3 | 54664 | before/retained_precision_contract.rs |
| 375b07313518a0cdd09e317b83b9ca7536acef5c80fd4cd0a94920fd8bf2486d | 66051 | before/lib.rs |
| 20db5327e13aabcb63d4803f11b9f2d7e1e98fc6a64d558ebcbf2ce72680f68c | 148552 | final/retained_precision.rs |
| f8313258e618088f0f3793791bbe8087c37ee75829c3cd3ae0056c86dbec5b12 | 65119 | final/retained_precision_contract.rs |

The run*.exit and run*_start.txt files hold the exit codes and UTC bounds.
