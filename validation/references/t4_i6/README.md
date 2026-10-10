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
