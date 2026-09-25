# Successor review: source repair backcheck

**Disposition: IREV-S01 is repaired in source; the narrow affine test disposition is correctly implemented. IREV-S02 remains an unresolved blocking contact-state regression. The candidate is not ready for qualification or merge.** Runtime reruns of the new receipt/affine candidate remain required.

This return completes the initial full successor review plus all subsequent frozen repair deltas. [INITIAL_REVIEW.md](INITIAL_REVIEW.md) preserves the original findings, mathematical trace, source-only mutation/coupled-gap probes and actual parent observation results. Earlier successful pre-successor review does not supersede this new assessment.

Final reviewed hashes:

| File, relative to projects/chirality-piping | SHA-256 |
|---|---|
| core/product_physics/src/lib.rs | 10a8d2195e4cd3674261251fb6e97106ab16ccabd46df60d346e7f6cd42bb645 |
| core/solver/nonlinear_integration/src/lib.rs | e21c812accb1ab2eb4cd2c9e4509cd25ba38982b3e649aafaeaa64ea9cb3f009 |
| core/solver/nonlinear_integration/src/product_equilibrium.rs | d18ed7e007e22390105670c89a647c3ab06441b4d27ed685b01e9bba0a8bc53e |
| core/solver/nonlinear_integration/src/structural_adapter.rs | 594377d0e6f5057f2e98961ce45baac3f8e8720c300856f7fdc17a30aad6baac |

## IREV-S01 source backcheck

The private ProductCompletion receipt is created only at the return boundary of the actual gated solve. It owns the validated typed input, ordered springs, selected mode, actual assembled full K, same-state final RHS, free/prescribed map and an immutable public-output snapshot. The nested snapshot contains no receipt; cloning the returned result does not make public mutations alter its trusted snapshot. No public receipt constructor or mutator exists.

The public qualifier now takes explicit springs and rejects input/source changes, spring changes or assembly-order changes. Every public result field, iteration and report must match the private output snapshot. It additionally checks known structural/equilibrium methods, selected mode, exact ordered free-DOF coverage in both residual reports and work rows, prescribed values and actual affine-load reconstruction. Equality is grounded in an independently inaccessible receipt, rather than two caller-editable records. This introduces no second factorization.

The new both-mode controls reject emptied rows/work, unknown methods, wrong row coverage, changed original force/modulus/boundary/spring magnitude or order, changed applied sliding loads and altered public displacement. They retain a genuine positive with no free DOFs, avoiding the incorrect rule that every empty row list is invalid. The original qualifying value is checked again after mutating clones. The isolated wrong-contact-law control separately calls the private physical-conditions seam as well as public qualification, so provenance rejection is not substituted for contact-law evidence. These controls require the parent's Rust execution; this is a source backcheck, not a passing rerun claim.

## Affine disposition backcheck

ROOT's explicit narrow disposition was relayed by the parent: exact repeats and seed equivalence per algorithm; exact discrete outcomes across modes; existing 1e-12 reference comparisons for continuous quantities across different algorithms. No gap-boundary, fixture, contact tolerance, 1e-9 analytical or other policy change was authorized.

The revised assertion group keeps all existing independent displacement, normal, friction and Coulomb comparisons at 1e-12. It adds exact repeated quantities/states/convergence in each mode, exact sparse-seed and dense-seed tuples, and compares the continuous cross-mode tuples under 1e-12. Existing assertions still require Sliding, convergence and two iterations in every original mode/seed case. The coupled exact-gap test is unchanged.

The eight parent-observed affine tuples independently match the rational references 7/135, 200/27, -20/9 and -7/165, 400/33, 40/11, with maximum absolute error 1.453382868600205e-15. Same-mode seed tuples are exact and every state is Sliding. AFFINE_REVIEW.json retains exact Fraction errors. A bounded check of DEL04-04 ScopeOfWork, OPS-K-SOLVER-1 and the original RF review found no permanent universal cross-algorithm byte criterion; historical full-payload equality remains recorded historical evidence. AFFINE_BASIS.json records those origins and the actual direction. This does not authorize weakening exact same-mode repeats or any physical-boundary convention.

## Open blocker and evidence limits

IREV-S02 is untouched: the actual parent probe reports four dense sign/seed combinations converging to Inactive/Inactive at exactly closed coupled gaps where the protected convention requires Active/Active; sparse and other seeds produce Active/Active. The exact oracle and source mechanism are detailed in INITIAL_REVIEW.md. A tiny backward error and a source-bound completion receipt do not establish a correct discontinuous contact predicate. Keep this blocker and its original failed tests. A generic numerical predicate/response repair must preserve exact closure, distinguish adjacent above/below inputs and validate reaction admissibility; no epsilon, snapping, fixture exception or test relaxation is justified by this review.

Original SUCCESSOR_CHECKS compiles and focused successor/mixed/integrity passes are retained, as is full nonlinear46/48 and the 24-case observation run. Their identities predate this receipt repair. Product full and new candidate runtime reruns are unexecuted at this checkpoint. Parent owns them and remaining kernel, NP-A, consumer/native, DEC025 and actual-head gates. NP-A intended-answer preservation/recovery remains required; no evidence here closes engineering or lifecycle acceptance.

Review remained independent TASK Type2 /root/solver_manager/integration_review under parent /root/solver_manager, using software-code-review. No Rust source/test edit, Cargo/build/native/browser run, Git mutation or descendant was performed. Actual read/write roots, instruction and adopted-policy origins, exact original/repaired sources, deltas, probe outputs and hashes are preserved under _run_records. No OS isolation or model diversity is claimed.
