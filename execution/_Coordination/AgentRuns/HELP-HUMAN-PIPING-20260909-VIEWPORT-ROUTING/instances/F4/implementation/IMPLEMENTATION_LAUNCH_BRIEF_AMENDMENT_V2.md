# F4 implementation launch brief amendment V2

Status: **SEALED; ROOT-ACCEPTED AFTER INDEPENDENT FREFUTE RECONCILIATION**

Recorded: `2026-09-09T07:06:31Z`

This additive amendment changes only the decision priority for a sliding row whose same-iterate derived signed-normal branch is inadmissible. It supplements the normalized `IMPLEMENTATION_LAUNCH_BRIEF_V1.md` at SHA-256 `88722b8597722f0a16e754d285cbafae602aa3f225b4a14e824d5bd98ac5b81d`. Every other objective, two-source-file fence, test requirement, prohibition, and output contract remains binding.

## Binding branch order

For each current `Sliding` friction row, resolve the next state in this order:

1. Apply contact validity first. Explicit `N <= 0` is `Inactive`. A derived current-normal magnitude `abs(R_n) == 0` is also `Inactive`. No sliding force remains in an accepted inactive result.
2. Preserve the one explicitly deferred initial-sliding/no-force warm-start iterate. Its existing gate blocks convergence.
3. If the assumed derived signed-normal branch is inadmissible, retain `Sliding`, do not evaluate tangential direction from the provisional force, and block convergence through the existing derived-normal aggregate gate. Retry with the observed signed source-reaction branch.
4. Only when the derived signed-normal branch is admissible, evaluate tangential admissibility:
   - at computed zero Coulomb limit, retain `Sliding` only for nonzero motion with no applied-force record;
   - at positive limit, retain `Sliding` only with nonzero motion and actual applied and reported tangential forces opposing that motion;
   - otherwise explicitly select `Sticking` before active-set residual, convergence, and diagnostics are formed.

Convergence remains the logical AND of the existing active-set/blocking/deferred gates plus both aggregate checks: all tangential branches admissible and all derived signed-normal branches admissible.

## Required regression evidence

- In `derived_normal_sign_flips_retry_before_convergence_and_fail_honestly_at_cap`, iteration 2 must retain `Sliding` for both load signs because its force was constructed outside the assumed signed-normal domain. For the positive-load case the exact evidence is `u_t = 97/1350`, `F_t = 7/9`, and source `R_n = -70/27` under an assumed positive branch.
- The observed negative-branch retry must converge at iteration 3 with `u_t = 103/1650`, `F_t = -7/11`, and source `R_n = -70/33`; its final force opposes motion and satisfies the current-normal Coulomb construction.
- Explicit-positive-normal sub-limit candidates still re-stick when their final tangential force assists motion.
- Explicit zero/negative contact and derived exact-zero contact remain `Inactive`; zero-limit frictionless release remains a separate positive-contact case.
- Mixed rows update together independent of row order, and any invalid aggregate branch blocks convergence.

The observed conflict and pre-amendment iteration evidence are preserved in `DERIVED_NORMAL_PRIORITY_CONFLICT_V1.md` at SHA-256 `2e05e9ba5468f5ae880ff5dda5a173cc92d80cebf4fdbd1f60d96a2907ebd0dd`.

No public schema, new tolerance, path history, friction history, diagnostic code, source-file scope, or global uniqueness claim is authorized by this amendment.
