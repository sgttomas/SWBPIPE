# RV23C-M1: non-budget cache probe gap

The original evidence is `REVIEW/KF3_REVIEW.md` C.1/C.3/C.4 and
`REVIEW/_run_records/kf3_review/confirm_aa83f6796/`:
- `scripts/rv23c_probe_patch.diff.txt`, lines 9–29: copy-only forcing of a
  non-refusal `Arithmetic(WideError::NonFinite)` at one block/pass;
- `scripts/rv23_probe.rs.txt`, line 870, the exact historical probe
  `rv23c_a_cached_failed_shared_build_carries_its_refusals_to_a_later_case`;
- `probes/confirm.out`, lines 6–7: both forced cases preserve
  `[0:Uc:refused:span:backward:72]`; case 1 has built_here=false;
- `probes/cache_probe_under_M1.out`, lines 1–2: case 0 retains that refusal,
  while case 1 loses it. These are historical results, not current observations.

All source-qualified paths/hashes are in INVENTORY.json. The source today
explicitly avoids caching Budget in `obtain_verify` (adaptive.rs 3820–3828).
The standard budget-stop test therefore cannot discriminate the cached field.
Its proposed use for this mutant has been withdrawn from the focused map.

Smallest faithful follow-up proposal: in an isolated, separately granted
current-source diagnostic, adapt only the original non-budget force point in
`uc_bounds`/`bounds_from` for the other block's Form pass; retain the genuine
chain refusal. Run the original two same-stiffness load cases (0.0625 and
0.125) as a group. Assert first build=true and second build=false, both stop
with the forced Arithmetic reason, and both retain the exact same nonempty
refusal list. Under the mutant, credit only loss of case 1's list while case
0 remains correct. Keep the original unforced companion and stage identities.
No permanent hook or test is proposed here; no hook or test was installed.
The old probe primarily prints the relevant data, so current acceptance must
check those explicit semantics rather than merely its exit status.

Historical disposition remains suite survivor with optional forced probe
(RV23C-N1), not a newly retired obligation or proof of current equivalence.
