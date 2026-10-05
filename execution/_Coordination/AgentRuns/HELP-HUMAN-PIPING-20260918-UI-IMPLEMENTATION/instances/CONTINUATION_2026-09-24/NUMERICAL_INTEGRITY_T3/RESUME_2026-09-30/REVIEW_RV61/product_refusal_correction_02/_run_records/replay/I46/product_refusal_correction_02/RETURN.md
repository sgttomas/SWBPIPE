# I46 supporting G5a correction — ready for RV61 backcheck

**Corrected:** the original checker applied G5a's guard to uncoupled resolution E. Items3/4 require coupled resolution ê. The corrected copy derives body extent from the actual captured coordinates, couples the two original resolution operands with separate binary64 operations, and only then applies the guard. The zero rule still uses original E. The original sealed packet remains byte-for-byte unchanged.

TASK Type2 `/root/i46_product_refusal_diagnosis`, ROOT HELP_HUMAN parent, delegated-harness-native, no descendants. Brief pin `2b2d2d86473d4e3d569dba57a06b6478917c1e18`; original pin `330dc2e405de87f451189616d2d5b73aaac86de7`. Receipt **2026-10-03 03:43:00 UTC**; scheduled checkpoint/cutoff/deadline **03:48:00 / 03:53:00 / 03:58:00 UTC**. Early sealed return: **2026-10-03 03:46:43 UTC**. Only this correction directory was written; generated material is under `_run_records`.

## Exact repair and changed field

R7 lines231/350 define

`ê_force = max(E_force, RN64(E_moment/L_b))`

`ê_moment = max(E_moment, RN64(L_b*E_force))`.

Both expressions use the original uncoupled operands. At L_b=0, coupling is omitted. R7 line697 identifies the captured resolution fields as uncoupled; G5a line828 keeps the zero rule on original E, while lines829/834 use RN64(ê*c), c=`3ff0000000001000`. DESIGN line499 supplies the body extent: separately rounded coordinate spans and squares, the prescribed left-associated two-add sum, then RN64 square root. The correction uses an exact rational midpoint-square control to obtain that binary64 root; it does not call a model or solver.

Actual captured endpoints yield L_b=`3ff0000000000000` (1) in both cases. Loaded original resolution bits are force `408e82798cc84f69`, moment `4042f77f9ea1d9bc`. Force is larger, so both coupled values become `408e82798cc84f69` and both guarded values become `408e82798cc86deb`. An independent exact-product/neighbor-midpoint comparison proves those guarded bits, without relying on the Python float product as its oracle.

Exactly one existing result leaf changes:

| Existing supporting field | Original | Corrected |
|---|---|---|
| `/cases/loaded/g5a_arithmetic/resolution_guard_bits/1` | `4042f77f9ea1ecb3` | `408e82798cc86deb` |

The loaded force guard is unchanged. Both zero-case guards remain positive zero. Lower-bound values, sanity/lower-bound/summary booleans and original-E zero-rule results are unchanged: the loaded supporting checks still pass; zero's first positive-zero refusal remains row35, with action rows35–52 violating that rule.

`_run_records/analysis.patch` is the complete corrected-copy diff. `_run_records/g5a_coupling_inputs.json` retains actual extent, uncoupled, coupled and guarded bits with explicit zero-rule provenance. This correction supersedes only the original supporting G5a arithmetic calculation; it does not alter the original report or claim the old calculation was correct.

## Unchanged proof and controls

The corrected analysis reran from a byte-identical copy of the original parsed capture. Recursive comparison covers the entire old/new analysis output, finding only the single leaf above. In particular, all ordinary rows, native-point comparisons, algebraic direct-unit projections, classifications, scales, failure lists and negative-zero records in both cases are identical. These comprise 14 named unaffected case subtrees, including 146 ordinary row records, 104 native point records and 104 algebraic projection records.

The complete `rx_counterexample.json` is **byte-identical** (SHA-256 `e6bf48ef285d8e07f4a16300d3d46de5db6e911d38ce4d23b4324c485a3971ab`). Thus the exact source/K comparison, correct RN64(q_K) proof, source error/allowances and nearby dual-cover incompatibility are unchanged. The original analysis stdout is also byte-identical. No Rx theorem was reopened or newly selected.

Six bounded abstract scalar controls pass:

- L_b=4, E=(3,20) gives ê=(5,20), disproving a hardcoded unit-length shared maximum.
- L_b=4, E=(20,3) gives ê=(20,80), exercising the other coupling direction.
- L_b=3 from spans (1,2,2), E=(1,10) gives ê_force=RN64(10/3), bits `400aaaaaaaaaaaab`, with an exact midpoint proof of that rounding.
- L_b=0 preserves E=(3,20) and performs no division.
- L_b=4, original E_force=0 and E_moment=20 gives coupled ê_force=5, while the original-E rule still rejects a negative-zero force row. A coupled-E zero-rule mutant would wrongly accept it.
- Spans (1, 0x1.6cp-27, 0x1.6cp-27) give extent `3ff0000000000001` under the prescribed separate operation order.

These are scalar arithmetic discriminators, not admitted-model or product executions. The source-bound unpublished resolution/estimate/charge facts remain supplied captures; no native replay or full G5a ownership revalidation is claimed.

## Preservation, reproduction and return

All ten relocated original files were compared against their committed bytes at `330dc2e405de87f451189616d2d5b73aaac86de7`, then rehashed after the new checks; every original hash matches. Root/TASK/Piping/skill and selected contract origins are recorded in `_run_records/BASIS.json`. Generated commands/results, the exact comparison, raw outputs and final hashes are retained under `_run_records`.

From this correction's `_run_records`, `python3 analyze.py` regenerates the corrected analysis and coupling trace; then `python3 control_and_compare.py` checks all controls, original preservation and complete unaffected-output equality. The second command expects the retained original at its current sibling relative location. Never run the original sealed checker in place.

Ready for the same RV61 reviewer's backcheck and ROOT disposition. No product source, output, predicate, truth, protected availability, acceptance or release change is selected. No compiler, model, solver, native producer, network/API, Git/index write, host tooling or delegation occurred.
