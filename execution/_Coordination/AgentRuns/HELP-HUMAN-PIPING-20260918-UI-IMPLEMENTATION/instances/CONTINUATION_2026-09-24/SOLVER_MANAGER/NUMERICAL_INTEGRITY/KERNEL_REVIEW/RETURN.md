# Frozen kernel and sparse implementation review

**Disposition: not yet suitable for operational fan-in.** Complete review of the six-file frozen kernel/sparse diff found five actionable issues below. The existing top-level dense and sparse wrappers do require positive factors, retain full original equations, and share residual/condition completion. Repair the identified geometry/fidelity/API seams and backcheck the repaired candidate before integration. This is a bounded initial-module review, not an M03 completion assessment.

Executor: TASK Type 2 `/root/solver_manager/kernel_review`, delegated-harness-native child of `/root/solver_manager`. Parent reports actual dispatch gpt-6-astra xhigh. Independent of implementation and numerical-policy authors; same-model review is not model diversity. No descendants, source/test edits, commits, Cargo/build/npm/native/browser runs. Writes are confined to this review evidence directory.

## Candidate and evidence

The base is `63b9a56cbcc40fd2ba7e1a3c2f2555214aa6d8d9`. All six hashes match KERNEL's supplied freeze: common structural `168c78233805254cb88a1d6d6aad5baa93a7f2e912249642dd0989538fb1a27e`, geometry `95b5f2fdf2deaaa78fe5fd887ba836ded7552a7c575f4f2830a5c4ad536afd33`, and sparse structural `09939a029b575bde9d677f95e35113da62b68ab2c535a93850ee217609f12ded`. Six-file source snapshots, hashes and generated-lock observations are in [_run_records/SOURCE_CHECK.json](_run_records/SOURCE_CHECK.json) and [_run_records/reviewed_sources](_run_records/reviewed_sources). The two kernel/sparse locks are ignored generated files with no tracked base; their local package/dependency contents match the manifests. No tracked lock delta belongs to this slice.

Read full new-worktree Root/TASK/Piping/LOOP_INIT instructions and PRIMARY software-code-review skill; the selected implementation/policy/reference bodies were read from PRIMARY, where they exist. Exact roots, hashes, parentage, runtime and enforced assignment limits are in [_run_records/ORIGINS.json](_run_records/ORIGINS.json). No other full role/workflow was loaded. [_run_records/scope_check.json](_run_records/scope_check.json) records the supported scope validator passing the six explicitly owned source paths; it does not certify unrelated concurrent edits.

Parent-run logs actually report repaired frame **51/51** and sparse **24/24**. The original frame **47/48** run and the 1e-300 false Range remain historical failures; they were not overwritten or called passes. The repaired soft residual is independently explained below. [Input evidence hashes](_run_records/EVIDENCE_INPUTS.json) bind the logs, return/freeze and maintained fixtures. KERNEL/RETURN.md's stale “await parent rerun” sentences do not supersede the observed rerun logs.

[_run_records/independent_probes.py](_run_records/independent_probes.py) ran with Python standard library only; [_run_records/independent_probes.json](_run_records/independent_probes.json) retains output. Exact Fraction rank/substitution/inverse calculations are independent oracles. Explicitly named source transcriptions demonstrate the source arithmetic consequences; **they are not execution of Rust or the production solver**. A rerun is `python3 <this-directory>/_run_records/independent_probes.py`. Largest exact solve is 8x8; geometry uses six-column small matrices, and order-80 comparison recurrence is scalar.

## Confirmed findings

Paths below are relative to `projects/chirality-piping/` and line numbers refer to the frozen source snapshots.

### KREV-01 — P1: physical null witness checks coarsened geometry

Location: `core/solver/frame_kernel/src/rigid_body.rs:48,61–65,80–81,195–217`.

The code first rounds coordinate subtraction and normalization, then calls those resulting rows `actual_constraints`. Its exact dot check establishes only exact annihilation of that already modified matrix. Two independent finite-input counterexamples, with all three translations grounded at all three nodes and the qualified welded-frame family:

- Coordinates [(0,0,0),(1e200,0,0),(0,1e-200,0)]: dividing by characteristic length erases the third point's offset. Actual exact rigid rank is 6, normalized rank is 5, and the emitted RX candidate has actual third-node z action 1e-200.
- Coordinates [(1e16,1e16,0),(-1e16,-1e16,0),(-1e16,-9999999999999998,0)]: subtraction rounds the last two relative positions to the same vector. Exact represented-input geometry is non-collinear (determinant -4e16); the emitted candidate has actual constraint action approximately 1.4142. This case needs no underflow.

Both source transcriptions reach MechanismWitnessed; exact Fraction original-coordinate checks refute the witness. The retained near-collinear 1e-16 companion correctly remains NumericallyUnresolved, and the ordinary non-collinear companion is Restrained. Thus the bug is lost geometry, not merely a conservative rank screen.

Remedy: preserve original represented coordinates and verify physical motion/constraint annihilation with range-safe exact/error-controlled differences and products; any unaccounted centering/normalization loss must remain unresolved. Do not repair by increasing rank tolerance or treating a small original action as exact zero. Also verify the recovered node motion is a common physical rigid motion against the actual constraints. Add both counterexamples and origin/unit variants as maintained controls.

### KREV-02 — P1: contribution fidelity excludes prescribed coupling

Location: `core/solver/frame_kernel/src/structural.rs:308–335` (consumed at 658–664).

The perturbation norm visits only free/free entries. Complete directed contribution differences in K_fc are retained in a list but have no effect on the reported amplification or acceptance, even when prescribed motion makes them a material reduced-load error.

Concrete input: stored K=[[2,-1],[-1,2]], f=[0,0], free=[1], prescribed=[(0,1)], and complete intended contributions summing to [[2,-2],[-2,2]]. Both reduced scalar stiffnesses are positive and well-conditioned. The source gate solves u=[1,0.5], with original stored residual zero, audit=true, perturbation=0, amplification=0 and Passed. The contribution-preserved equation gives u=[1,1]; the published candidate's intended free residual is -1. This is separate from N05's known soft-stabilizer issue.

Remedy: account for contribution error in the actual reduced action/RHS, including delta(K_fc)*u_c, and retain a meaningful fidelity/uncertainty result. A known significant prescribed coupling discrepancy cannot be represented by zero assembly amplification. Add positive and corrupted nonzero-prescribed contribution controls to both backends.

### KREV-03 — P2: diagnostic low-word accumulation silently loses terms

Location: `core/solver/frame_kernel/src/structural.rs:279–288`.

Each two_sum correction is added to one ordinary binary64 low accumulator. That accumulator can itself absorb a term and later cancel to zero without evidence. For one off-diagonal entry, the finite sequence [1e16,1,1e-16,-1,-1e16] ends with high=0 and low=0, although the exact sum of the represented inputs is the nonzero represented 1e-16. Mirroring the sequence in the other triangle does not trigger the positive-diagonal absorption check. A coalesced zero entry therefore reports no contribution rounding or perturbation for that lost action.

This counterexample directly concerns the advertised retained-contribution audit; an estimate need not be a rigorous bound, but silently erased diagnostic input cannot support a reported zero perturbation. It is a synthetic cancellation control, not a claim that every constituent is independently passive or that product geometry generates this sequence.

Remedy: preserve/track low-word accumulation error with a longer expansion, compensated tail plus explicit uncertainty, or an unresolved result when fidelity is lost. Add a symmetric off-diagonal cancellation control; the existing positive-diagonal absorption test cannot cover it.

### KREV-04 — P2: public negative-witness helpers still panic on malformed input

Location: `core/solver/frame_kernel/src/structural.rs:875–888,916–926`.

The public verify_negative_direction checks direction length/finiteness only. PreparedSystem fields are public. Starting from a valid one-DOF prepared system, clear prepared.matrix[0] and call verify_negative_direction(system, prepared, [1]); the length check passes and prepared.matrix[0][0] at line 883 panics. A valid prepared matrix with a shortened original free map or scale exponent vector similarly panics. negative_pair_witness also directly indexes a malformed two-row matrix at line 921.

This is a confirmed source-level reachable Rust bounds panic, not a separately executed Rust panic. The new malformed-factor tests exercise Cholesky and the condition helper but miss these exported helpers.

Remedy: validate original system, square prepared shape, map/exponent dimensions and relevant numeric state at every public negative-witness entrypoint, or make them inaccessible outside a validated internal state. Add catch_unwind/Result regressions for ragged matrices and missing maps/exponents.

### KREV-05 — P2: public completion can bypass the positive-factor contract

Location: `core/solver/frame_kernel/src/structural.rs:640–657,684–709`.

finish_structural accepts any solve closure, any pivot vector (including empty), and a caller string for factorization. It never establishes that positive-factor scrutiny happened or that evidence belongs to this prepared matrix. Prepare N07 K=[[1,2],[2,1]], f=[1,1], free=[0,1], then finish with empty pivots and the exact LU closure ((-b0+2*b1)/3,(2*b0-b1)/3). The named estimator returns rcond=1/3, residual passes and completion returns quality Passed, despite the independently verified energy -2 along [1,-1].

The top-level solve_structural_dense/sparse wrappers correctly reject N07; this finding concerns the exported common backend/fallback seam and its unconditional no-finite-LU-bypass comment. Current external product callers have not adopted this seam, so no claim of a newly exercised product bypass is made.

Remedy: bind completion to validated positive-factor evidence associated with the same prepared system (or restrict/seal this unchecked helper and expose a checked backend interface). A caller-supplied label/pivot vector must not constitute structural qualification. Add a direct common-completion N07 control so later fallback integration cannot accidentally reuse LU success.

## Review coverage and findings-free observations

- Complete full stiffness/load and disjoint free/prescribed validation precede both ordinary solve wrappers. Nonzero prescribed elimination is algebraically correct; constrained loads are not forced into the free residual, all-fixed systems return the prescribed vector, and missing partition entries fail.
- Radix congruence is A=T*K_ff*T, b=T*(f_f-K_fc*u_c), u_f=T*y. Nonfinite/range checks conservatively reject unrepresentable operations rather than install a stiffness floor. The admitted numerical envelope remains narrower than all finite/subnormal inputs; this is explicit.
- Both triangles of full original K are checked. A documented projection applies only to the prepared free matrix; residuals still target the unsymmetrized full original equations. The transformation estimate's 12-product/12-add stages match existing K*T then T^T*temporary source order. Constitutive/orientation formation and assembly are rightly outside that helper; caller evidence and operation counts still need integration validation.
- Dense Cholesky and skyline LDL require positive finite screened pivots; local counts match one product/subtraction per dense update and already-computed-work sparse updates. Counts do not cover prior factor error, and reports do not claim certified original inertia. Sparse profile ordering and pivot global-DOF mapping are consistent by source trace.
- Hager-Higham inverse 1-norm iteration plus alternating safeguard is an estimate, not a pivot ratio or bound. An independently inverted 8x8 NP-B matrix has exact reciprocal 1-norm condition approximately 0.01855694; source-estimator transcription returns approximately 0.02135131, demonstrating the expected optimistic-estimate limitation without a false certification claim. Three exact orderings solve to the independently specified all-ones vector. The order-80 comparison bound is approximately 8954.221 while the analytical condition2 bound remains <429; no mandatory comparison-inverse gate was introduced.
- Original residual count is 2*nonzero coalesced row terms+2 and includes prescribed values. Normalized mantissa multiplication and sequential accumulation match the declared count path; retained guard includes evaluation allowance, denominator reliability and final-operation reserve. Independent R01/R02/R03/R04/R05 candidates fail the stated original-equation reason; R04's correct midpoint passes. The repaired soft 2e-300 row retains physical residual -6.63123685e-316 with a normal normalized residual and passes, while the wrong-displacement control remains failing.
- Refinement signs and scale recovery are correct by substitution: A*delta_y=-T*r, u=T*(y+delta_y). It stops at three attempts or non-improvement. A failed residual is not overridden by output finiteness.
- Negative diagonal and pair-direction diagnostics are mapped to full DOFs; generic nonsymmetric solve_dense and legacy sparse factor/report formulas remain untouched. Their historical raw pivot metrics were not renamed or replaced.
- Source test expectations are analytical/simple manufactured controls rather than fitted observed solver values. NP-B's local test is a perturbed manufactured family, not the frozen full reference/permutation suite. The maintained generator imports no production assembly; its observer calls old APIs and remains observation-only.

## Required work that is not a new module defect

The five findings require repair/backcheck. Beyond them, first-module readiness still does not establish all operational gates: execute the maintained N/R/NP suite through the new dense/sparse APIs, mutation hooks including sparse omitted contribution and common completion, positive symmetry-projection/transform bounds, refinement behavior, condition sensitivity and range variants, and ordering/global-map controls. The existing Rust unit passes are useful but do not supply all these witnesses.

Product/nonlinear callers still visibly use legacy solve_dense/solve_symmetric_system_from_entries and finite dense fallbacks. Their adoption, actual selected active-contact state, entity/status mapping, and fallback refusal after integrity rejection are manager-owned integration work outside this reviewed diff. Full-ground geometric rank remains only a necessary screen for qualified families; internal released/connector modes require their own energy/nullspace evidence and the matrix gate.

N05/NP-A represented-K residual success cannot establish intended physical accuracy. N06 lost stabilization cannot be called a physical mechanism. Contribution-preserving solve/recovery or another independently validated stronger method remains explicit required work, as KERNEL's return already states; no warning-only closure is accepted here.

Protected analytical 1e-9, DEC-050 raw observations, DEC-053 predicates, broad pipeline checks, clean DEC-025, actual-candidate CI, native witnesses and professional/release boundaries remain unchanged and outstanding with the parent. No certified SPD, guaranteed forward digits, release readiness, native execution, or model/OS diversity is claimed.

