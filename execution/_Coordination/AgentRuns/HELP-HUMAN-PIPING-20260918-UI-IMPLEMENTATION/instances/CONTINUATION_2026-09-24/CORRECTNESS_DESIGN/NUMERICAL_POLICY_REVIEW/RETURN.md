# Independent numerical-policy review

Date: 2026-09-24. Executor: /root/numerical_policy_review, TASK Type 2, delegated-harness-native child of ROOT. This is independent analytical policy refutation, not implementation, executed numerical validation, or complete-candidate code review. No descendants were launched.

**Disposition: ready to author the first bounded operational integrity slice using the qualified policy below. Reject mandatory comparison-inverse certification as the application-wide success gate.** Its sufficient inequality is mathematically valid, but the proposed inexpensive inverse bounds can be exponentially pessimistic on a sparse, uniformly well-conditioned SPD family. Do not postpone the currently missing structural/residual gates while building a formal IEEE certificate. Conversely, a finite solution, a positive computed pivot, or a small residual alone remains insufficient.

The existing proposed reference is sound on work-conjugate scaling, original-equation residuals, separation of stability from sensitivity, full-precision transport, and the analytical classifications of N01–N09/R01–R07. Its requirement that every ordinary successful solve have rigorous p*q*delta certification should be superseded prospectively. Keep that certificate as optional stronger evidence when its actual hypotheses are met. Ordinary output must say that numerical integrity checks passed; it must not claim formally certified positivity/inertia, guaranteed forward digits, or physical accuracy merely from those checks.

ROOT's current activation and subsequent reference-quality steering authorize these ordinary technical choices. This review changes no protected criterion. The physical-answer accuracy problem exposed by N05 remains visible and is not repaired by redefining a tolerance.

## Scope and evidence

The supplied source identity is 0be12dc968d78b747beb5e0d10c32e12cc991d75. The four core source hashes actually read match the numerical designer's recorded source hashes. No Git command or mutation, product write, solver/test/build process, native/browser application, or external solver was used. Read-only shell file operations, current primary-source web retrieval, and elementary scalar evaluation of analytical expressions were used. The read manifest is in [_run_records/BASIS.md](_run_records/BASIS.md); detailed derivations and source qualification are in [_run_records/DERIVATIONS_AND_REFERENCES.md](_run_records/DERIVATIONS_AND_REFERENCES.md).

Inspected seams: frame_kernel boundary reduction and row-pivoted solve; sparse_direct profile assembly, fixed-order LDL, report and solve; product_physics ground-DOF count, selected dense/sparse/fallback solve and publication; nonlinear_integration selected linearized solve/reactions; DEC-050/053 policy records and the D-04 record. Full Root AGENTS, TASK, Piping AGENTS, LOOP_INIT, current CORRECTNESS_ACTIVATION, NUMERICAL_REFERENCE and design RETURN were read. No other full role/workflow/skill body was selected.

## Findings to integrate

| ID | Finding and consequence | Disposition |
|---|---|---|
| NP-01 | The congruence and recovery algebra are correct. For ideal diagonal equilibration the characteristic length and energy references cancel from A; computing every intermediate literally is unnecessary and may increase range risk. | Implement equivalent exponent-aware total diagonal scaling, preserve its physical basis and scale/recovery evidence. No stiffness floor. |
| NP-02 | p*q*delta < 1 is a valid sufficient SPD certificate with true upper bounds and symmetric original A. It is not necessary. The comparison-triangular recurrences can become enormous while the true matrix condition remains bounded independently of size. | Reject its mandatory application-wide use; retain optional certification. See the explicit banded counterexample below and in the derivations. |
| NP-03 | 64*gamma(m) is an engineering screening reserve, not a universal theorem about assembly, rank, factorization, or answer accuracy. A single dimension n is not all those algorithms' operation count. | Use separately named screens with separately recorded arithmetic/count bases and candidate probes. Do not call the reserve certified or expand it to make a failure pass. |
| NP-04 | The original free-equation componentwise residual is correct and scaling invariant. A full rigorous residual enclosure is much cheaper than global inverse certification, but a conventional residual with explicit evaluation allowance is also a legitimate operational screen. | Keep the original-equation gate in every selected path. Distinguish an implemented rigorous enclosure from an estimated/conservative roundoff-model guard in the output. |
| NP-05 | N05 is exactly stable, but one ordinary binary64 assembly addition changes the effective tiny spring by about 1.70e-6 relative. Exact solution of the stored matrix can therefore miss the intended root rotation by about 1.70e-6 relative. | Preserve intended-model and represented-matrix oracles separately. A protected 1e-9 accuracy conflict stays a failure. Report stable model with numerical/physical-answer accuracy unresolved; do not falsely report a mechanism. |
| NP-06 | Six-column rigid rank checks eliminate the reported oblique rigid-motion defect but do not prove absence of internal released/connector modes. Positive support stiffness may count geometrically even if it is numerically lost during assembly. | Retain an assembled structural factor/uncertainty check, model-family scope, and an assembly-sensitive control. Contacts must use the selected active state. |
| NP-07 | Sparse_direct reads one triangle, accepts/counts negative LDL pivots, and product dense/fallback paths lack equivalent integrity gates. Existing report fields are observations, not these missing gates. | Add the typed shared contract before result success in all modes. A sparse integrity rejection cannot be bypassed by finite LU fallback. |
| NP-08 | Full precision is independently ready. Lossless transport is not a claim of accurate physics. | Remove publication quantization while preserving finite guards, historical envelopes, canonicalization compatibility, producer/version identity and declared unit-conversion checks. |

The counterexample is A=C*C^T, C=I+(3/4)S+(3/4)S^2, where S is the lower shift. A has half-bandwidth two and kappa_2(A) < 429 for every finite order. Yet the proposed comparison inverse bounds obey z_i=1+(3/4)z_(i-1)+(3/4)z_(i-2) and grow like ((3+sqrt(57))/8)^i. At order 64 the analytical p*q*u value is about 1.28; at order 80 it is about 8954. Thus any generic residual envelope delta >= u fails that certificate at those sizes despite uniformly bounded conditioning. These are evaluations of the derived bound, not measured solver failures or a claimed pipe performance limit. Exact-zero special treatment can escape the dyadic example; it does not remove the exponential overestimation mechanism. Small perturbations preserve both bounded true conditioning and exponential comparison growth.

## Concrete first operational policy: M03-INTEGRITY-v1

This is a bounded implementation policy recommended for ROOT/design-manager integration and candidate validation, not a new owner acceptance or change to protected numeric limits. Its engineering screens may reject an uncertain state; none may label uncertainty a proven physical mechanism.

Use binary64 round-to-nearest with unit roundoff u=2^-53 and gamma(m)=m*u/(1-m*u), m*u<1. In the settings below EPSILON means Rust f64::EPSILON=2^-52, not unit roundoff. Range errors require explicit handling; the pure relative-roundoff model does not cover silent underflow or overflow.

### 1. Declare the supported structural contract

Use the gate on passive small-displacement symmetric elastic structural subproblems after exact constraint elimination. Keep the generic dense algebra API for nonsymmetric auxiliary/friction systems; do not attach an SPD requirement to all callers.

Validate finite inputs, strictly positive intended material/section/stiffness quantities where required, the actual constraint map, and assembly provenance. For welded, unreleased objective straight-frame components, positive strain energy plus full rigid-restraint rank proves exact-model uniqueness: each element's zero-energy state is rigid, shared six-DOF nodes give one common rigid motion, and the supports remove it. That proof does not extend automatically to releases, nonobjective user matrices, kinematic connectors, geometric stiffness, follower loads or nonlinear material tangents. For other included passive families, document their energy/nullspace contract and test an internal mechanism. Unsupported family evidence is explicit; it is not a whole-programme prerequisite for implementing the straight-frame gate.

### 2. Preserve equations and scale them without changing the model

Retain an immutable canonical original equation representation, loads, imposed values, free/global maps, model/case basis and selected support state before ordering/factorization. The representation may be sparse; a second dense copy is not required by the mathematics.

Apply the proposed positive diagonal congruence and inverse mapping, preferably radix-power scaling where appropriate. Audit both triangles before accepting a symmetric representation. A symmetry screen uses assembly/transformation operation evidence, not a universal N or N*m cutoff. Do not silently average material skew. Retain the unsymmetrized original equations for residuals when a documented roundoff-sized projection is used.

Track assembly cancellation and loss of contributions that matter to a soft mode. The positive sum of absolute contributions is useful uncertainty evidence; it is not interchangeable with the absolute value of a coalesced matrix entry in the componentwise residual denominator. Computing residual and denominator from an omitted-contribution matrix would preserve the original bug.

### 3. Separate geometric mechanism evidence from numerical rank uncertainty

Construct the proposed per-component six-column rigid-restraint matrix from actual support directions, relative coordinates and active constraints. Positive grounded springs count as restraints in exact geometry irrespective of their stiffness magnitude. Inter-node links do not automatically count as ground. Use the actual multi-point/offset constraint map.

Use a well-tested rank-revealing QR/SVD. The initial proposed band tau_B=64*gamma(max(rows(B),6))*sigma_max is acceptable as a **numerical rank screen**, with the method, singular values and dimensions retained. It is not an exact nullity theorem. A deficient/ambiguous numerical rank blocks ordinary solved publication pending either a witnessed rigid mode or stronger scrutiny. Report a mechanism only with a physical null-motion witness checked against the actual constraints/element family. Normalize rows and compute relative geometry safely; huge absolute origins can already have destroyed relative coordinate information before centering.

### 4. Require a positive structural factor and screen numerical resolution

Use symmetric positive Cholesky in dense structural scrutiny or the existing skyline LDL with every structural D strictly positive and finite. Positive-factor success is evidence for the computed factor. Do not count row-pivoted LU diagonal signs as inertia. Do not introduce raw global 1e-10 pivots or preserve the current dimensional 1e-12 as the structural rank rule.

Record each LDL Schur pivot's cancellation scale, such as c_i=|a_ii|+sum_k |L_ik*(D_k*L_ik)|, and the actual ordered summation length/operations. The initial local screen d_i <= 64*gamma(m_i)*c_i is a reasonable flag for doubtful resolution. For s_i diagonal updates using already computed work_k=D_k*L_ik, m_i=2*s_i+2 is a conservative local multiply/subtract count; if both products are recomputed in each term, use at least 3*s_i+2. Earlier factor errors are not covered by either local expression. Therefore call it a cancellation screen, not an error bound for the exact pivot or a proof of original inertia. An ambiguous or nonpositive factor blocks ordinary success and routes to numerical-unresolved or a separately established negative-energy/mechanism diagnosis.

Add a named DPOCON/LACN2-style norm-condition estimate using scaled triangular solves; retain the scaled norm and estimator status. A pivot ratio is not a replacement. For the first slice:

- Estimated rcond at or below binary64 EPSILON, zero, nonfinite, or unavailable because the estimation solve lost range gives numerical-resolution-unresolved. This selected one-ULP boundary follows the conventional working-precision scale; it is not asserted to be the exact DLAMCH constant on every LAPACK build. A stronger method may resolve it.
- Estimated rcond below sqrt(EPSILON) gives a sensitivity advisory. Above the working-precision boundary it is not an automatic physical-mechanism or failure classification.
- Keep any assembly/factor perturbation estimate alongside rcond in the same scaled norm. If rho_est=kappa_est*perturbation_relative_est is at least 1, or a stabilizing contribution has been lost, classify accuracy/resolution as unresolved even if rcond alone clears its boundary. Below that boundary retain the estimated amplification, sensitivity and any separate required quantity-accuracy check; do not turn a small estimated rho into guaranteed digits. A condition estimate is not a rigorous upper bound and does not certify a sign.
- A negative-energy verdict requires the proposed mapped direction with negative quadratic energy beyond its evaluation uncertainty, or exact authored-energy evidence. A factor failure without that witness remains numerical-unresolved.

These are operational screens to validate on the named candidate probes, not a proof that every accepted binary64 matrix has certified positive inertia. The first slice may legitimately publish “numerical integrity checks passed” for a checked passive model with positive factor, clear uncertainty screens and acceptable residual. Reserve any stronger certified status for a genuinely implemented bound. This prevents mandatory formal verification from blocking ordinary repair.

### 5. Gate every selected result on the original free equations

Use r=K_original*u-f_original and d_i=sum_j |K_original[i,j]|*|u_j|+|f_original[i]|, including prescribed displacements. Gate free rows only; constrained residuals are reactions. Retain force and moment residuals separately with DOF/entity mapping. Check imposed compatibility independently.

Keep the initial target 64*gamma(m_i) **per row**, with an actual evaluation basis. A safe elementary implementation basis for k_i coalesced terms with ordinary products and sequential addition/subtraction is m_i=2*k_i+2, subject to normal arithmetic/range assumptions. A tighter gamma(k_i+1) augmented-dot derivation is also valid when implemented and documented correctly. Pairwise, compensated or FMA implementations need their own verified derivation; the operation count is not inferred from n. Testing every row against its own target avoids letting one dense row inflate every other row's allowance.

Compute an evaluation allowance and denominator reliability along with the observed ratio. The proposed upper enclosure using |r_hat|+e_i and d_lower is mathematically correct. In the no-underflow/overflow roundoff model, a directly derivable choice is d_lower=d_hat/(1+gamma(m_i)) and e_i=gamma(m_i)*d_hat/(1-gamma(m_i)); final arithmetic must also be accounted for before calling the result a rigorous upper bound. Without that last enclosure implementation, report a roundoff-model guard/estimate and retain a conservative final-operation margin; do not use the word certified. This distinction does not justify an unchecked residual test.

Exactly zero denominator with exactly zero equation action contributes zero. Nonfinite/range-ambiguous rows do not pass. Prefer exact power-of-two local scaling or a specified safe-minimum algorithm over a fixed physical-unit floor. A wholly constrained system has no free residual gate but still needs exact imposed values and correct load/reaction publication.

Attempt the proposed maximum of three correction solves when a residual fails, using the original residual and unchanged physical problem; stop when it passes or stops improving. Three is a resource choice, not a convergence theorem. Retain the final failed status. Higher-precision residual/refinement is an appropriate targeted successor for an ill-conditioned case, but cannot recover information already discarded from the original assembled matrix.

### 6. Keep result status and accuracy claims separate

Use orthogonal fields (or an equivalent explicit product status) for:

| Field | Required distinction |
|---|---|
| Structural classification | Physical mechanism witnessed; negative energy witnessed; passive stable-model basis; or structural/numerical uncertainty. |
| Numerical solve quality | Checks passed; sensitive; backward-error failed; arithmetic/range unresolved. |
| Model-to-matrix fidelity | Represented equations retained; detected assembly-loss/uncertainty; verified against intended independent fixture where available. |
| Accuracy evidence | Quantity comparisons/forward-error estimates and their basis; no claim of guaranteed engineering accuracy from residual alone. |

A small residual and sensitivity warning can coexist with an ordinary approximate numerical solution when no required accuracy criterion has failed. A known failed protected accuracy comparison cannot be relabelled as success. For N05 specifically, ordinary assembled double precision may produce “stable model; intended-answer accuracy unresolved” and withhold normal MECHANICS_SOLVED success under the relevant required accuracy witness. That is an honest identified repair requirement, not permission to omit it permanently. N06 must remain unresolved unless a reformulation/higher-precision assembly preserves and solves the stabilizing contribution.

The dense fallback path must re-enter these same checks. Sparse storage/range failure may justify fallback; evidence of a physical mechanism/negative energy or a failed original-equation gate is not cured by finite LU output. Nonlinear final selection must re-run the checks on the actual selected active state and its actual same-state load; complementarity/friction/state convergence remain independent.

## Reference dispositions and candidate probes

N01–N04 and N07–N09 formulas, signs, dimensions, inertia and null directions were independently backchecked by derivation. N05/N06 exact-model classifications and formulas also check; add the assembly qualification below. R01–R07 substitutions and constrained/free distinctions check. The EB bending formulas apply to that implemented theory. N01 is not a validation of negligible shear deformation; the slender N09 and pure torsion controls are relevant without asserting a universal pipe slenderness cutoff.

The following probes are required before merging the operational gate; their implementation and actual results belong to the solver manager, not this return:

1. Freeze independent analytical N01–N09/R01–R07 references before production changes; exercise dense, sparse and fallback. Show rejection of R01–R05 for the intended original-equation reason and R07 despite zero residual; retain R04's stated M10 product dependency.
2. Add NP-A: the isolated N05 two-DOF torsion matrix, with one-step assembly a+k, intended exact a,k oracle, and a separate high-precision solution of the exact stored binary64 matrix. Record both forward differences, represented residual, element/spring action equilibrium and spring reaction. Repeat N06 and a small stiffness sweep around one ULP of a. Do not derive the oracle from the production assembler.
3. Add NP-B: the explicit banded C family above, including a small non-dyadic perturbation and several orderings. It must not be classified as a mechanism or negative energy because a comparison inverse bound explodes. Record the optional certificate failure separately from ordinary positive-factor/residual/condition quality.
4. Add NP-C: an internal released/connector mechanism whose global rigid-restraint rank is full, plus a fully stabilizing companion. A support-rank-only mutation must be killed. Include near-collinear support geometry; small positive stiffness versus missing stiffness; origin shifts; unit/orthogonal rotations; uniform material scaling.
5. Add NP-D: intentional upper/lower skew, duplicate-entry loss/cancellation, sparse-only omitted spring, lost tiny stabilizer, nonzero prescribed boundary, loaded constrained DOF, nonfinite and subnormal/range cases. Test the state where a contact lifts off and exposes a mechanism.
6. Run the existing DEC-053 nine chain/grid observations (8/24/48/32-span chains and named grids) under their unchanged recorded predicates, plus focused realistic many-span orientation/support/stiffness-contrast cases. Measure original matrix/profile/factor storage, actual count bases, residual and condition evidence, false-unresolved cases, and time/memory observations. No size, speed or memory threshold is supplied or claimed here.
7. Freeze the candidate policy names, constants, count definitions and status semantics; obtain fresh independent complete-diff review, registered checks and actual native headless/bridge/persistence/export evidence required by the loop. This design review cannot serve as that code review.

NP-A arithmetic: a=687800*pi lies in [2^21,2^22), so one binary64 addition has spacing 2^-31. The stored increment is round(1e-4/2^-31)*2^-31 = 214748*2^-31 = 0.00009999983012676239013671875. It differs from intended k by -1.6987323761e-6 relative. The exact stored-matrix root rotation T/k_stored differs from T/k by +1.6987352618e-6 relative. This is a predictable representation effect, independent of Gaussian/Cholesky solver quality. Scaling after that addition and exact refinement of the stored matrix cannot repair it. A contribution-preserving soft-mode/relative-coordinate reformulation or higher-precision assembly/action is a concrete repair option to evaluate; injecting stiffness is not.

Protected criteria remain distinct: DEC-050's 1e16 raw pivot proxy is only its named generated-grid observation metric; DEC-053's 1e-9 parity, 1e-6 absolute sparse residual, exact repeats and zero nonpositive pivots retain their named basis. The existing analytical 1e-9 policy and conversion witnesses are not replaced with new numerical-integrity screens. Do not relabel old raw observations as scaled estimates. If the changed implementation exposes a protected conflict, preserve the measured conflict and route it through its existing owning decision.

## Current-source qualification

As retrieved on 2026-09-24, the official [LAPACK release record](https://www.netlib.org/lapack/release_notes.html) lists 3.12.1 (2025-01-08). Its current [DPOSVX](https://www.netlib.org/lapack/explore-html/d6/d44/group__posvx_gae23dac18e7ec69c36fd2a2648a470dc3.html) still combines symmetric equilibration, Cholesky, condition estimation and refinement/error estimates. This supports the conventional first-gate direction; it does not automatically validate this in-repo solver.

Current [DPOCON](https://www.netlib.org/lapack/explore-html/d3/dfd/group__pocon_gaadb1e19663b71521d30b5b5bbe3091f8.html), [DPORFS](https://www.netlib.org/lapack/explore-html/d7/dfd/group__porfs_gaff660c485d1ff77c2c5c85a6bbf81900.html) and [DPOEQUB](https://www.netlib.org/lapack/explore-html/d1/da9/group__poequb_ga335265d8cad6dc99d803cb703b4f9cd8.html) confirm the separation of norm estimates, componentwise backward error, safe-minimum handling, refinement and radix scaling. The proposal's factor 64 and rigorous inverse requirement do not come from these routines.

Newer theory was checked rather than treating an old citation as sufficient. [Higham–Mary 2019](https://eprints.maths.manchester.ac.uk/2731/1/paper.pdf) gives probabilistic error bounds under explicit rounding assumptions, including Cholesky; those probabilistic bounds are not a deterministic certificate for every authored model. Their [2022 Acta Numerica survey, author preprint](https://tmary.perso.lip6.fr/doc/surveyMixed.pdf), particularly §§6–7, treats residual precision, convergence and sparse mixed-precision refinement. It supports targeted higher-precision residual/refinement investigation where needed, without establishing performance on this app or restoring lost assembly data. [Rump–Jeannerod's improved deterministic analysis](https://www.tuhh.de/ti3/paper/rump/JeaRu13a.pdf) also confirms that bound constants depend on the actual algorithm/arithmetic. No newer source justifies replacing the applicable small-displacement reference formulas or importing a universal 1e-10 stiffness cutoff.

## Remaining gates and return

Ready now: author M34 full-precision work, shared M03 evidence/status types, canonical original-equation residual checking, per-component geometric analysis and positive structural factor gating with the explicit qualified policy above. The ordinary operating policy no longer waits on a universal formal SPD certificate. Candidate validation of these screens and correct status propagation remains necessary before merge.

Still required: NP-A assembly-sensitive accuracy resolution; actual implementation count/range audit; named negative controls and representative existing chain/grid observations; independent complete-candidate review; loop checks and actual native consumer witnesses. A successful first slice is bounded numerical integrity, not completion of every mechanics finding, certification, product release or practitioner acceptance. ROOT integrates this return with the programme graph and the design manager's prospective numerical policy; solver_manager owns implementation and recorded evidence.
