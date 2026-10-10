# T4-I6 frozen references for T4-U1 (the objective curved element)

Consumed by `core/product_physics/tests/t4_u1_reference.rs` (P1–P5 and
RV2 S-4's diagonal-scaled criterion against `u1_reference_cases.json`) and,
through `t4_i14_l30_matrix.py`, by FK's
`kd5_long_nearly_straight_bend_at_the_floor_does_not_demote`.

## Provenance

- **Source:** T4-I6 (T4's reference TASK), frozen before any T4-U1 code was
  read. Archive tag `archive/piping-t4-2026-10-09`, commit `f2211ecef6`,
  path `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/PRESSURE_STRESS_T4/T4-I6/`.
  The definition is that record's `U1_REFERENCE.md` B1; the set is B2, the
  properties B3 and the large-coordinate controls B5.4.
- **Bytes unchanged:** `u1_reference_cases.json`, `_run_records/curved_ref.py`
  and `_run_records/freeze_u1.py` are copied byte for byte. `SHA256SUMS` holds
  their three lines verbatim from the archive's `T4-I6/SHA256SUMS`, whose own
  SHA-256 is `146f734afd64a00b1cba52bfdff9352e73cb73d5ddbddedaa161fc96305a9193`.
  Check with `shasum -a 256 -c SHA256SUMS` in this directory.
- **Regeneration:** `freeze_u1.py` (standard library only, 110-digit decimal
  arithmetic) wrote the JSON with `curved_ref.py`; see the archive's
  `T4-I6/_run_records/RUN.txt`.

## Added by T4-I14 (T4-U1 phase A)

`t4_i14_l30_matrix.py` evaluates the frozen generator `curved_ref.py` for
RV131's `rv_o4_axial` configuration (a 30 m chord bent by 1e-9 and 2e-9 rad)
and prints each 12x12 entry rounded once to binary64;
`t4_i14_l30_matrix.stdout.txt` is its output, embedded in the FK test.
Regenerate with `python3 -I t4_i14_l30_matrix.py _run_records` here.
SHA-256: script `bee26d7d8bed2fa19464bf61a8dd292517ced462dd3e70e54bb07388449af11c`, output `0f2828beaefb5709b11fad5e4f3d8696e2b063316baed786f8fa4cb4cdfce10c`.

## Round 01 (`round_01/`, added by T4-I15, T4-U1 phase B)

T4-I6 repair round 01, produced 2026-10-10 in scratch; SHA256SUMS digest
`4a53e72144eba25ff56736f3a155890b680eb292e9050213e284c28b159a9919`;
independently confirmed by T4-RV10 (values: PASS WITH FINDINGS; the M31b0
derivation: HOLDS WITH CORRECTIONS; that review's record is outside the
repository). HELP_HUMAN ruled M31b0 equivalent on 2026-10-10 with T4-RV10's
C-1 to C-4 applied (DEL-04-01 Design): CB's relative plane tolerance bounds
|y|/|y_perp| below 1e12, so the residual domain that `_run_records/i7_m31b0.py`'s
comments describe (|y|/|y_perp| above about 2e19) is unreachable; the script is
kept byte for byte. `u1_reference_cases_r01.json`
and the scripts that wrote it (`_run_records/r01_lib.py`, `i1_curved122.py`
… `i8_acceptance.py`, `freeze_r01.py`) are copied byte for byte;
`round_01/SHA256SUMS` holds their lines verbatim from round 01's
`SHA256SUMS`. Check with `shasum -a 256 -c SHA256SUMS` in `round_01/`. The
scripts import round 00's `curved_ref.py` from a sibling `rerun00/`
directory; it is `_run_records/curved_ref.py` here (same SHA-256,
`1f524bcc…`). Consumed by: the curved true-positive candidates, CSKEW_9/10,
K1/K1F and the acceptance-range samples (through `kd5_models/`), FK's K2
test (`k2_kernel_kill`, embedded literals), PP's acceptance-range guard
test (`acceptance_range_samples`) and PP's P5 test, whose seeded generator is
round 01 §8's (RV2 N-8).

## `kd5_models/` (added by T4-I15, T4-U1 phase B)

`kd5_models.py` regenerates NI's `structural_adapter/kd5_models.rs`. It is
T3's K-D5 generator (T3 `IMPLEMENTATION/KD5/_run_records/repair/models/
kd5_models.py.txt`, SHA-256 `f445b579…`) revised for T4-U1: each realized
bend is (R, y_reference, k), y taken as an explicit recorded input from round
00's `t3_models[*].regenerated_bend_inputs` bow vectors (O6), and the
intended curved element is D1's objective Decimal re-formation given the
exact centre of that arc (120 digits). It imports T3's cited V1
`probe_d5_check.py` (SHA-256 `d13cf7c8…`) and D1 `curved_ef.py`
(`1c862cea…`) unchanged from a directory given on the command line:
`python3 -I -B kd5_models.py <v1_dir> . <out.rs> <out.json>` from this
directory's parent. Every re-derived u_int is compared with T4-I6's frozen
value (an independent element, `curved_ref.py` at 110 digits):
`kd5_models.stdout.txt` and `kd5_models_comparison.json` record the
comparison, worst 5.0e-11 of the criterion (the frozen values' 20-digit
storage); rows differing after rounding to binary64 are exact zeros of the
model (both sides below 1e-80). SHA-256: script `92097bca…`, generated
`kd5_models.rs` `f3e8ae03…`.

## Short arcs (added by T4-I15, T4-U1 phase B)

`t4_i15_short_arc_matrices.py` evaluates the frozen generator
`curved_ref.py` (110 digits) for R = 0.3 m arcs of 0.06°, 0.1°, 0.5° and 5°,
in-plane and skew, d on the 2^-30 m grid, and prints each 12x12 entry
rounded once to binary64; `t4_i15_short_arc_matrices.stdout.txt` is its
output, embedded in CB's `short_arc_tests.rs`. Regenerate with
`python3 -I t4_i15_short_arc_matrices.py _run_records` here. SHA-256:
script `52b811325257b76d4e97da19550d5ea9432226893f0af215716c948ae2afa0a3`, output `85ff2e972791c365cc201359752fefec7ee27dd0935ac26239cb920424412116`.
