# RV77 mutant results

All runs were on RV77's own `git archive` of `c618675e84` in WT/rv77, with targets
WT/targets/rv77/{frame_kernel,product_physics}, `cargo test --locked --offline --lib
<filter> -- --nocapture`, CARGO_BUILD_JOBS=4, RUST_TEST_THREADS=2, one cargo job at a
time, and the memory guard (PID 5387) running. The driver (WT/scratch/rv77_coverage_producer_01/
rv77_mutant.sh) applies one mutation, runs, restores the two source files from a saved
copy and prints their sha256 prefixes; every run restored to the expected bytes.
Logs: WT/scratch/rv77_coverage_producer_01/mutants/<id>.log.

## I61's six mutants and NONE, re-run from the clean archive

Patches applied verbatim with `patch -p1` from `R/I61/coverage_producer_01/_run_records/mutants/`.
Filter: `i61_` on the patched crate, as I61 ran them.

| Id | Result | Killing tests (RV77 run) | Matches I61's table |
|---|---|---|---|
| NONE | FK 3 passed, PP 4 passed | — | yes |
| M1_adapter_copy_source | KILLED | PP `i61_certificate_prefixes…`, `i61_failure_prefixes…` | yes |
| M2_empty_as_all_false | KILLED | the same two | yes |
| M3_null_allowed_on_ready | KILLED | PP `i61_certificate_prefixes…` | yes |
| M4_drop_one_body | KILLED | all four PP `i61_` tests | yes |
| M5_p512_charge_is_estimate | KILLED | FK `i61_synthetic_rederivation…` (synthetic only) | yes |
| M6_accept_flag_mismatch | KILLED | FK `i61_rederivation_refuses…` | yes |

## RV77's own mutants (diffs in mutants/)

Baseline for these: the candidate plus the two reviewer test includes (tests only).
NONE_B passes every filter below. FK-file mutants ran `fk:i61_ fk:rv77_ pp:i61_ pp:rv77_`;
PP-file mutants ran `pp:i61_ pp:rv77_`. Every survivor of the candidate's tests was
re-run against the broader `fk:product_certificate` (34 tests) and `pp:retained` (48 tests)
suites (34 and 48 tests, of which 3 and 1 are RV77's); no candidate or pre-existing test in them killed any survivor.

| Id | Mutation | Candidate tests (i61_ + broader) | RV77 tests | Status |
|---|---|---|---|---|
| R1_floor_refusal_only_when_L_nonzero | the positive-floor-forces-stop refusal applies only when L ≠ 0 | survive | killed (`rv77_full_payload_domain…`) | survivor of candidate suite |
| R1n_native_floor_inside_L_branch | native `summary_coverage_data` ORs the floor only inside the L ≠ 0 branch | survive | survive | survivor: no actual p512 body with L = 0 exists; the seam's floor-stop refusal would catch it on such a body |
| R2_extent_check_dropped | the bit-equality of recomputed L with `prep.extents[b]` is removed | survive | survive | equivalent on every input at this source (both values come from the same `CasePrep`) |
| R3_has_data_from_stop_outcomes | rederived has_data := any(stop) (a final/nonzero-row proxy) | killed (PP `i61_specimen…`, cancelled ±x) | killed (`rv77_genuine…`) | killed |
| R4_certificate_entry_not_required | non-null no longer needs the certificate stage/check entered | killed (PP `i61_certificate_prefixes…`) | killed | killed |
| R5_lanes_unchecked | the two-lane order/result requirement is removed | survive | survive | equivalent on reachable inputs: a non-empty vector exists only after certify_final, which needs both lanes Ok |
| R6_null_allowed_on_passed_g5a | Null is no longer refused when G5a passed | survive | killed (`rv77_stage_rules…`) | survivor of candidate suite |
| R7_absent_kind_refusal_dropped | no refusal of a stop on an absent kind | killed (FK `i61_synthetic…`) | killed | killed |
| R8_input_derived_refusal_dropped | coverage_facts accepts input-derived force/moment rows | survive | survive | equivalent on reachable inputs (recover::layout); necessity shown by `rv77_input_derived…` (336 breaks) |
| R9_presence_from_all_bodies | coverage_facts presence ignores the body filter | survive | survive | equivalent on the fixtures; RV77 reads every solvable body as having all four kinds (each DOF's stiffness or reaction emits a force/moment row), so not reachable — not exhaustively proven |
| R10_source_presence_not_required | non-null no longer needs `capture.source` | survive | killed (`rv77_stage_rules…`) | survivor of candidate suite |

Survivors of the candidate's own tests: R1, R1n, R2, R5, R6, R8, R9, R10.
Survivors of all tests including RV77's: R1n, R2, R5, R8, R9 (each argued equivalent or
unreachable above).
