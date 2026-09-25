# Strict gap boundary: independent repair contract

**Recommendation:** retain the current dense/sparse positive structural paths and add source-bound exact boundary scrutiny before gap classification and final qualification. An authored-boundary vector is a valid candidate only after exact full intended-equation verification and a valid uniqueness warrant. Candidate rejection must lead to a resolved strict sign or an explicit unresolved result; small backward error cannot authorize the branch. The existing dense-to-LDL substitution alone is insufficient.

This is independent TASK Type2 design/reference/refutation, returned to SOLVER_MANAGER. No production repair or production execution is claimed. Runtime parentage, actual roots, instructions, source hashes, web qualifications and the lightweight exact reference execution are under `_run_records/`. No new public version, dependency, numerical backend, workflow, engineering acceptance or release is selected here.

## Evidence and unchanged obligations

The actual 24-case parent probe contains 16 exact-gap and eight affine cases. Four dense exact-gap cases, both signs with g1 initially inactive, converge to Inactive/Inactive at an exact authored tie. The current receipt repair addresses evidence provenance, not this decision. The exact gap convention is stronger than abstract complementarity: touching with zero reaction must be Active. OneWay/LiftOff instead use strict signs and do not inherit that tie rule.

Keep the existing changed-support count/floor zero, product cap four, actual first-iteration singular-seed recovery rule, physical admissibility and recovery checks. Preserve original analytical relative1e-9 and the explicitly adopted cross-algorithm affine1e-12 references. Neither the equilibrium successor nor this design changes these criteria. Existing failed probe/test history remains failed evidence until the actual new candidate passes.

## 1. Exact candidate route and its proof

Let C be the prescribed DOFs of the **current actual linearized state**, F the remaining DOFs, A the exact intended K_FF, and h=f_F−K_FC u_C. The immutable intended source includes individual stiffness and load contributions, source/formulation identity, the current spring set, current affine loads if supported, and the actual C/F map. All original free rows are checked, including rows that a proposed contact closure would remove from a candidate reduced system.

Construct a finite candidate v from authored signed gap values and current prescribed values. Other free coordinates may remain from the current solution, or come from a separately recorded candidate computation. Single-gap substitutions and one simultaneous all-free-gap substitution are reasonable deterministic proposals; none is accepted based on distance from u_hat. A simultaneous proposal is necessary for the observed chain: replacing either of its two rounded coordinates alone leaves a nonzero exact residual. Deterministic subset/coupled proposals may be added under an explicit work budget, but failure to enumerate a subset is never proof that it cannot close.

Accept v as a correction of the current linear response only if:

1. Complete source coverage, map compatibility, finite/range checks and the ordinary source-bound positive structural gate still pass.
2. v_C equals the current prescribed u_C exactly, with no duplicate/contradictory constraints.
3. For **every original current F row**, exact intended action sum_j K_ij v_j−f_i is zero. This includes all contribution tails and K_FC u_C. A guard/epsilon/rounded residual equal to zero is insufficient.
4. A separate valid warrant establishes nonsingularity of that intended A. The current positive floating factor and rcond estimator are operational screens, expressly not such a theorem. For the two-dimensional first family below, exact principal minors provide the warrant. A source-owned exact positive-energy/rank certificate or a verified original-matrix enclosure can extend coverage later.
5. Recover the full original reaction/action from the same candidate and source; obtain exact signs needed by every involved contact. Apply the unchanged classifier with the actual prior states. Do not accept arbitrary active reactions simply because prescribed rows were omitted from the free residual test.

Proof: v satisfies A v_F=h. Nonsingularity makes it the unique response of the current state. Therefore replacing u_hat by v corrects the response to that original system; it does not perturb the load, stiffness, gap or contact law. Reclassification uses the ordinary count mechanism. It is not permission to declare a newly changed contact state converged: changes remain changes and the next state still receives its ordinary solve/check within the existing cap.

For the observed chain, A=[[200,−100],[−100,100]], h=[0,s*25/2], v=[s/8,s/4]. A has exact a=200>0 and determinant10000>0. All free residuals and both contact reactions are exactly zero. Thus v is the unique response, and the protected law chooses Active/Active for either sign. Both values are exactly representable; no precision projection is needed. A candidate checked only against its newly fully constrained reduction would have no free rows and could falsely appear valid for any load: that vacuous check is specifically forbidden.

## 2. Smallest bounded stronger sign route

Candidate substitution alone cannot qualify strict adjacent cases. A practical initial exact predicate family uses the existing checked expansion arithmetic for **original-source connected free blocks of order zero, one or two**. Block selection is by exact zero off-block coupling and DOF maps, not support names, coordinates, loads, bit patterns or known fixture constants. A bound of two is an explicit initial arithmetic/resource scope, not a statement that larger systems are solved. Candidate generation may be wider, but its uniqueness/sign proof needs an actual supported warrant.

For a two-dimensional block A=[[a,b],[b,c]], form exact expansions:

```
Delta = a*c − b*b
N1 = c*h1 − b*h2
N2 = a*h2 − b*h1
u1 = N1/Delta; u2 = N2/Delta
```

Require exact symmetry, a>0 and Delta>0. Completing the square proves x^T A x = a*(x1+b*x2/a)^2 + (Delta/a)*x2^2>0 for nonzero x. Thus this is an exact SPD/nonsingularity proof for that block, not a reinterpretation of the operational factor report. One-dimensional blocks require exact a>0 and use u=h/a. An empty block uses only prescribed action. Every currently free component needed for the proof must be covered; do not ignore an unsupported coupled component. Unknown original asymmetry or projected-symmetry-only source remains outside this exact symmetric proof.

For gap sense s in {+1,−1}, the released penetration is p=s*u_j−g. With positive denominator Delta, its exact sign is sign(s*Nj−g*Delta). No quotient and no gap-distance epsilon is required. Zero retains the existing Active convention. For a constrained active contact, form the exact reaction from original rows. For one common free block the numerator is

```
R_i = sum(j in F) K_ij*Nj + Delta*(sum(j in C) K_ij*u_j − f_i)
r_i = R_i/Delta
```

Use exact positive-denominator combination when multiple independent blocks contribute to a reaction; if the initial implementation supports only one such block, report that limit. Active gaps remain active iff s*r_i<=0. Never substitute the sign of a rounded reconstructed K*u−f or its residual allowance. For order one the same formulas use a as denominator; for order zero evaluate the prescribed source action directly.

Implementation can multiply expansions term by term using existing checked scalar products and exact summation, and determine zero/sign from a reviewed normalized nonoverlapping representation. Add explicit term/operation/range budgets; exhausting them is unresolved. The current `Expansion::rounded()` alone is not an exact-zero or sign certificate. All new exactness claims depend on verified arithmetic preconditions, including subnormal/overflow/underflow handling and fused multiply-add semantics. Reuse the existing error propagation; never truncate low tails to meet the budget.

This route has no closeness trigger: within its supported gap family, evaluate the exact current-state predicates on every relevant iteration and at completion. A broader fast path may later use rigorously enclosing intervals that exclude zero; the present rcond estimate and `64 gamma(m)` residual screen are not rigorous forward-sign enclosures. An interval containing zero means the sign is undecided, not that the gap is closed. Refinement or higher precision by itself cannot prove an exact tie; exact zero evidence or a mathematically valid certificate is still needed.

If a representable exact candidate is verified, install it before full recovery/report generation. Otherwise retain source-bound exact sign/response evidence separately from the approximate public displacement. A private typed result should identify each support, prior state, exact sign basis, actual source/map/force identity and any response correction. The integration classifier and final qualifier must consume that same evidence; do not encode a true sign by fabricating a nearby displacement/reaction or by using an arbitrary numeric sentinel. Mutated or stale public evidence must fail the existing receipt checks. Approximate publication and exact internal predicate evidence must be accurately distinguished. If that seam cannot carry exact signs, the safe implementation outcome is numerical uncertainty, not a claimed qualified state.

## 3. Source fidelity and first supported product family

The exact arithmetic proves the declared numerical source, not an unrecorded ideal engineering model. Current AssemblyEvidence records rounded element-global coefficients and explicit scalar springs; StructuralSystem records only one already aggregated force vector. This is enough to state an exact represented-contribution claim, but not automatically an exact primitive/derived-force claim. The actual axial fixture is stronger: its axis-aligned geometry, unit section/length and E=100 yield the stated dyadic coefficients and direct nodal load exactly. That makes the reference relevant to the actual equations rather than an idealized substitute.

First supported product scope should be explicitly source-qualified pure Gap supports on passive straight-frame/positive-spring systems with complete direct nodal loads and exact prescribed maps, for which the intended free block and needed reactions meet the exact arithmetic scope above. If generalized source fidelity is not yet available, state the narrower represented-contribution scope honestly. Do not label arbitrary rounded-only curved/user/connector matrices as primitive-exact. Missing force contributions, lost K_FC terms, absorbed stabilizing terms, inconsistent transformations or unrepresented geometric source invalidate the stronger claim. Existing source-fidelity rejection remains a rejection; this seam cannot close NP-A/N05/N06 recovery work by itself.

Mixed friction/derived-normal/affine-load cases need source-preserved normal/tangential load equations and their own simultaneous branch proof. Do not apply the symmetric gap determinant argument to the nonsymmetric Coulomb coupling. OneWay and LiftOff have different strict zero conventions; preserve them and exclude them from the gap tie correction unless deliberately supported and tested. Curved/connector/user producers need their actual source and passivity/constraint contract. Unsupported arithmetic, source family, sign range or coupling produces a specific unresolved outcome where strict gap qualification is requested; no silent fallback to the same rounded predicate qualifies that unsupported case. This does not claim the separate already adopted affine policy is superseded.

## 4. Iteration, trial and resource accounting

Checking a proposed vector by exact action is a `boundary candidate verification`, not a structural factorization or active-set iteration. Record how many proposals and exact operations were attempted. Correcting the same current linear response and then classifying leaves `changed_supports` and the ordinary iteration counter truthful.

An extra solve with new prescribed contacts is instead a `boundary analysis trial`: record the attempted state, source, factorization, result and resource use. It cannot masquerade as the existing first-iteration singular recovery or silently bypass the cap. Recommended minimal implementation avoids such extra state solves: verify vectors and current-state signs, then let the existing state-change loop perform the next ordinary solve. Keep the original singular-seed recovery eligibility and one-trial maximum unchanged. It may still be necessary when a released seed has an actual rigid mode; the exact SPD proof does not fabricate a stabilizing contact.

At cap one a changed state is still nonconverged. Returning the right guessed state without its required final-state evidence is not a pass. Under exact arithmetic the 27 neighboring input combinations below converge within three iterations for all four seeds, so they fit cap four without changing it. This is a reference fact, not observed candidate execution or a general convergence theorem.

## 5. Independently proved controls

`_run_records/reference_proof.py` uses Python Fraction only; it imports no production package and never reads production outputs to generate expectations. It derives constrained equations, verifies exact equilibrium and admissibility, enumerates all four final state candidates and simulates the unchanged state-switched signs. `_run_records/REFERENCE_PROOF.json` retains exact fractions for 27 combinations: predecessor/equal/successor load magnitude and each gap magnitude, both physical signs and four initial seeds. The two-mode execution requirement is 432 cases, not432 executed production tests. All 27 combinations have exactly one convention-admissible state and all reference traces converge within three iterations.

Let F0=25/2, g1=1/8, g2=1/4, and normalize x=s*u, q=s*r. The physical negative-sense cases are exact reflections. Adjacent load increment is 2^-49. Gap predecessor steps are 2^-56 and2^-55; successor steps are2^-55 and2^-54. This asymmetric spacing at powers of two is retained.

| Change from exact base | Required final state (g1,g2) | Normalized nonzero reaction |
|---|---|---|
| none | Active, Active | neither; both zero |
| load predecessor | Inactive, Inactive | neither |
| load successor | Active, Active | q2=−2^-49 |
| g1 predecessor only | Active, Inactive | q1=−25/2^54 |
| g1 successor only | Inactive, Active | neither |
| g2 predecessor only | Inactive, Active | q2=−25/2^54 |
| g2 successor only | Active, Inactive | neither |
| both gap predecessors | Active, Active | q2=−25/2^54 |
| both gap successors | Inactive, Inactive | neither |

For load predecessor, the all-active trial has a positive q2 and must release; it must not turn that pulling reaction into zero. For single-gap perturbations, exact authored-all-boundary residuals in the packet prove why forcing all active fails. For each physical sense, every seed and both actual backends, require exact discrete outcomes, unchanged reference comparisons, repeat determinism, ordinary iteration accounting and actual final receipt/source validity. Preserve the separate eight affine cases with unchanged1e-12 comparisons and exact same-mode seed/repeat behavior.

Additional proof/control obligations:

- Current two-coordinate dense candidate: substituting either gap alone yields exact nonzero free action; simultaneous substitution yields zero. This distinguishes a coupled method from a one-coordinate patch.
- Nonzero prescribed root u0=1/16 gives exact target [3/16,5/16]; full original free residuals vanish. Omitting K_FC u_C produces first-row error100/16, so a reduced-only candidate check must reject that omission.
- A source load tail2^-60 added to25/2 is absorbed by binary64 aggregation but changes the authored-boundary reaction by−2^-60. The tail must remain distinguishable; passing the rounded force version cannot prove intended-source equality.
- Source load/stiffness/spring/order/boundary mutations, missing free rows, duplicate gap DOFs and incomplete sign receipt coverage must reject. All-fixed candidates must still check required active reactions; empty free rows alone prove no admissibility.
- Reaction signs must include exact zero, positive/pulling and negative/bearing, including adjacent perturbations. Reference traces exercise release from incorrect initial active states.
- Exact primitive/source uncertainty, unsafe expansions/range, exhausted budget, unsupported blocks larger than the implementation limit and singular/indefinite controls must return the proper failure/uncertainty, not a fabricated exact-zero/physical-mechanism result.
- Node/contribution/support permutations and work-conjugate radix rescaling must preserve the relevant exact proof when their transforms are represented exactly. General oblique transformations need their own source-qualified coverage; do not imply it from these axial references.

## 6. Refutation and external qualification

Switching Cholesky to floating LDL may happen to return exact dyadic values for this chain. It does not establish exact arithmetic for arbitrary divisions, Schur complements or recovered actions; positive pivots are still operational floating evidence. Any finite floating method can return a nearby value on the wrong side of a discontinuous threshold. Even an exact determinant-positive proof alone proves unique response, not that the rounded response has the correct gap sign. Therefore an LDL-only repair must not close the generic strict-sign finding without the boundary evidence above and its negative controls. PSD is not enough for uniqueness; singular PSD admits multiple responses, and zero residual cannot distinguish them.

The local formulas and contact references above are independently derived algebra. External facts were checked against primary sources on2026-09-24. [Shewchuk’s primary robust-predicate research page](https://www.cs.cmu.edu/~quake/robust.html) establishes the relevant general method: determinant signs can fail under floating rounding, and exact/adaptive expansions can resolve them subject to arithmetic assumptions. It does not validate this Rust implementation or prescribe a piping law. [LAPACK’s error definitions](https://www.netlib.org/lapack/lug/node75.html) distinguish answer error and condition estimates; an estimated rcond is not a certified componentwise contact-sign enclosure. [Rust’s f64 documentation](https://doc.rust-lang.org/std/primitive.f64.html#method.mul_add) identifies binary64 and fused multiply-add semantics. These support arithmetic choices only; no external engineering equation extraction or new physical theory was used.

**Return to manager:** coordinate the source-owner adapter and frame-kernel exact-action/sign seam, then require independent complete-candidate review and actual dense/sparse/contact/affine execution. The present blocker remains open pending implementation and observed evidence. This packet supplies a proof contract, a bounded feasible route and exact controls; it does not claim implementation, general multiprecision adoption, NP-A closure or product qualification.
