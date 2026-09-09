# F4 derived-normal branch priority conflict V1

Status: **OBSERVED; PRIORITY AMENDMENT UNDER INDEPENDENT RECONCILIATION**

Reproducer: `projects/chirality-piping/core/solver/nonlinear_integration/src/lib.rs`, test `derived_normal_sign_flips_retry_before_convergence_and_fail_honestly_at_cap` (current source line 2722), case `force_x=+10`, `force_y=-1`, `seed=Sticking`, `mode=SparseInteractive`.

At iteration 2, the existing PR760 fixture produced:

- tangential displacement `u_t = +0.07185185185185186`;
- applied sliding force `F_t = +0.7777777777777771`;
- reported tangential support reaction `R_t = +0.7777777777777771`;
- observed signed source reaction for the derived normal `R_n,source = -2.5925925925925903`;
- assumed derived-normal branch sign `+1`.

Thus the assumed signed-normal branch is disproved, and the force produced under that branch assists the final motion (`F_t*u_t > 0`, `R_t*u_t > 0`). Applying the previously frozen “direction governs when both checks fail” priority re-sticks at iteration 2; the fixture then alternates between `Sliding` and `Sticking` and does not converge by the four-iteration cap, instead of preserving the established three-iteration retry.

The derived normal’s contact magnitude and its signed branch evidence are distinct. `friction_normal_for_support` supplies `abs(R_n,source)` to the contact/Coulomb classifier, so this observation is not contact loss and is not the explicit `N <= 0 -> Inactive` case. Zero-limit behavior also remains separate: it requires a computed Coulomb limit of zero and no applied-force record. Here the contact magnitude is positive and the nonzero applied force was constructed from a signed-normal branch that the same iterate disproved.

Candidate priority rule for independent review: while a derived signed-normal branch is inadmissible, its force is provisional and cannot decide the next tangential state. Retain `Sliding`, block convergence through the existing derived-normal branch gate, retry with the observed source-reaction sign, and apply the tangential direction check only when the same-iterate derived-normal branch is admissible. No result is accepted until both branch and final force/motion evidence are admissible.

No priority change was applied when this record was written. Temporary debugging output was removed, both solver files were rustfmt-clean and coherent, and no Cargo process was running.
