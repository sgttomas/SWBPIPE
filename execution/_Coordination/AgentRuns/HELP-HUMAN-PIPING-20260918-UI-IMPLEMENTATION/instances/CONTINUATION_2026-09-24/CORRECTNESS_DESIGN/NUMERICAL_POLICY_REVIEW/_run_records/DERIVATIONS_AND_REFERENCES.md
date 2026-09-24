# Numerical-policy derivations and primary-source qualification

Executor /root/numerical_policy_review; 2026-09-24. These are independent analytical checks. Elementary scalar approximations were evaluated in functions.exec JavaScript; no product code, numerical test process, matrix solver or performance measurement ran.

## 1. Scaling and the residual definition

For u=Dq, virtual work is u^T*f=q^T*D^T*f. The stiffness quadratic form is q^T*D^T*K*D*q, so the proposed congruence preserves exact inertia and energy. Translational D entries have length units; rotational D entries are dimensionless (radian treated dimensionlessly). All components of D^T*f have energy units. E_c therefore has units of energy, and H=D*K*D/E_c and g=D*(f_f-K_fc*u_c)/E_c are dimensionless. With A=S*H*S, b=S*g and u_f=D*S*y, substitution recovers the constrained original equation.

For exact unit-diagonal equilibration and positive diagonal:

    S_i = sqrt(E_c)/(D_i*sqrt(K_ii))
    A_ij = K_ij/sqrt(K_ii*K_jj)
    b_i = f_reduced_i/(sqrt(E_c)*sqrt(K_ii))
    (D*S)_i = sqrt(E_c)/sqrt(K_ii).

Thus the chosen physical length scale cancels from the final ideal A, while still supplying meaningful intermediate energy coordinates and rigid-mode geometry. A safe implementation can combine exponent factors rather than form D_i*K_ij*D_j and later undo them. Nearby radix powers preserve the same congruence principle; normal-range radix scaling can be exact. A diagonal scale does not recover an already rounded-away stiffness increment.

For r=K*u-f and d=|K|*|u|+|f|, positive diagonal row/unknown rescalings multiply each corresponding residual/denominator row by the same positive factor, so max |r_i|/d_i is invariant in exact arithmetic. Each ratio compares only one physical row, hence it does not mix force and moment units. This is a componentwise unstructured backward-error measure for the represented free equations; it is not a symmetric-structure-preserving perturbation measure or an element/material-parameter backward error.

With prescribed u_c fixed, including |K_fc*u_c| term magnitudes in d is consistent with perturbing equation coefficients rather than perturbing the prescribed values. A large constrained r is the required reaction, not failed free equilibrium. Using the reduced RHS alone can hide cancellation or omitted K_fc*u_c terms; the full original free row is the intended diagnostic.

For ordinary normal-range arithmetic let g=gamma(m), with m covering each relevant roundoff path and m*u<1. Positive accumulation gives:

    (1-g)*d <= d_hat <= (1+g)*d
    d_lower = d_hat/(1+g) <= d
    |r_hat-r| <= g*d <= g*d_hat/(1-g).

Therefore:

    eta_true <= (|r_hat| + g*d_hat/(1-g))*(1+g)/d_hat.

This algebra is exact; its floating-point evaluation needs final-operation control before being labelled a rigorous enclosure. Computing a gamma value in ordinary round-to-nearest and adding one final next_up is not a substitute for tracing the operations that claim an enclosure. A conventional conservative roundoff-model guard is acceptable for a correspondingly qualified operational claim. Subnormal underflow violates the pure relative-error model unless separately accounted for; exact row rescaling or a specified safe-minimum algorithm is necessary. The operation count for rank, assembly and factorization is not supplied by this residual derivation.

## 2. SPD sufficient certificate: valid theorem, unsuitable universal gate

Let A be the represented symmetric matrix; let C be nonsingular with positive diagonal and E=A-C*C^T. Define F=C^-1*E*C^-T. F is symmetric and

    C^-1*A*C^-T = I+F.

If p>=||C^-1||_infinity, q>=||C^-T||_infinity and delta>=||E||_infinity are genuine upper bounds, then ||F||_infinity <= p*q*delta. For a symmetric F, every real eigenvalue has absolute value no greater than any induced matrix norm. Hence p*q*delta<1 implies all eigenvalues of I+F are positive, and congruence proves A SPD. The strict threshold 1 is correct. A norm-condition estimator cannot be substituted for p or q while retaining the theorem.

This proves a claim about the symmetric matrix included in E. Additional scale/assembly error must be included to extend it to an earlier represented matrix; no finite-precision certificate about A automatically certifies the exact intended model parameters. A nonsymmetric original matrix cannot be called SPD merely because one triangle's projection passes.

For lower triangular C, applying absolute values in forward substitution with ||b||_infinity<=1 gives the recurrence z_i=(1+sum_(j<i)|C_ij|*z_j)/|C_ii|. Thus max z_i is a valid inverse-norm upper bound if arithmetic is outward-controlled. The corresponding backward recurrence is valid for C^-T. The problem is pessimism, not the direction of this inequality.

### Explicit uniformly well-conditioned sparse family

Let S be the n-by-n lower shift with ones immediately below the diagonal and let

    C = I + (3/4)S + (3/4)S^2
    A = C*C^T.

C is nonsingular triangular and A is SPD, with half-bandwidth two. Its inverse is the truncated power series sum_(j=0)^(n-1) t_j*S^j, where:

    t_0=1; t_1=-3/4
    t_j=-(3/4)t_(j-1)-(3/4)t_(j-2).

The recurrence roots have modulus r=sqrt(3)/2<1. Write cos(theta)=-sqrt(3)/4 and sin(theta)=sqrt(13)/4. Then

    t_j = r^j*sin((j+1)*theta)/sin(theta).

Consequently both ||C^-1||_1 and ||C^-1||_infinity are bounded by

    M = 1/((1-sqrt(3)/2)*(sqrt(13)/4)) < 8.281

for every n. Also ||C||_1 and ||C||_infinity <= 2.5, so ||C||_2<=2.5, ||C^-1||_2<=M and

    kappa_2(A)=kappa_2(C)^2 <= (2.5*M)^2 < 429.

By contrast the proposed absolute-value recurrence gives:

    z_1=1; z_2=7/4
    z_i=1+(3/4)z_(i-1)+(3/4)z_(i-2).

For w_i=z_i+2, w_i=(3/4)w_(i-1)+(3/4)w_(i-2). Its dominant root is lambda=(3+sqrt(57))/8 > 1.3187. Both inverse bounds have maximum z_n by reversal symmetry, so their product grows like lambda^(2n).

Elementary evaluation of the closed-form recurrence gives:

| n | z_n (approximate) | z_n^2*2^-53 (approximate) |
|---|---:|---:|
| 32 | 1.533842e4 | 2.611990e-8 |
| 64 | 1.073540e8 | 1.279519 |
| 80 | 8.980671e9 | 8.954221e3 |
| 128 | 5.257513e15 | 3.068816e15 |

For any routine using a generic residual/error envelope delta>=2^-53 on this normalized-scale family, the sufficient test fails from the displayed 64 case despite uniformly bounded true condition. This is an analytical counterexample to making this particular sufficient bound necessary for routine acceptance, not a claim that every possible certified algorithm must fail.

Because this unperturbed C and A use dyadic entries, an exact residual evaluator may establish E=0 and escape the generic-bound failure. That special case does not rescue the general policy: the comparison growth persists under a neighborhood of small coefficient perturbations for which actual inverse coefficients still decay. Nor is it eliminated by normal diagonal equilibration. A's diagonal entries lie between 1 and 17/8, so equilibration multiplies C on the left by a uniformly bounded positive diagonal; the true condition bound remains uniform and the comparison recurrence still has exponential growth. Reordering can change a particular factor's behavior, but no theorem says the chosen profile ordering eliminates this family of pessimism.

This is a synthetic numerical-method control, not a claimed authored pipe model. Actual many-span pipe incidence, element contrasts, release modes and orderings still need the focused candidate probes. No pipe-size limit or runtime measurement follows from these scalar numbers.

### Cost implications

The two comparison recurrences themselves cost O(nnz(C)) and O(n) workspace; calling them inherently expensive would be inaccurate. Computing a full triangular inverse would generally be dense and unnecessary for ordinary condition estimation. Explicitly materializing E=A-C*C^T can have substantial product/fill cost; naive dense formation is O(n^3) arithmetic and O(n^2) storage. Sparse formation depends on the factor pattern, so no measured app cost is asserted.

A known algorithm-specific factor-error envelope involving |C|*|C^T| need not be materialized: its row sums can be accumulated by |C|*(|C^T|*1) in O(nnz(C)), once the applicable theorem and arithmetic have been established. This is still an error envelope, not the exact factor residual or an SPD certificate. Condition estimation through a bounded number of triangular solves likewise does not require an explicit inverse. Preserve the original sparse equation representation; the mathematical residual contract does not require adding a dense copy to every sparse solve.

## 3. N05/N06 assembly-sensitive derivation

For N05 the exact intended torsion matrix is

    K = [[a+k, -a], [-a, a]]
    a=687800*pi N*m/rad; k=1e-4 N*m/rad
    f=[0,T], T=1e-8 N*m.

Its determinant is a*k>0. From the two equilibrium rows, k*theta_root=T and a*(theta_tip-theta_root)=T. The exact intended root rotation is 1e-4 rad and the grounded spring reaction is -T. Ideal diagonal equilibration has eigenvalues 1 +/- sqrt(a/(a+k)); its rcond_2 is approximately k/(4a)=1.1569856e-11. A blanket relative 1e-10 stiffness/pivot cutoff would incorrectly call this stable model a mechanism.

Now hold the assembled binary64 a fixed. It lies in [2^21,2^22), so its spacing is exactly 2^-31. The one-step increment fl(a+k)-a is the nearest integer multiple of that spacing:

    k_stored = 214748*2^-31
             = 0.00009999983012676239013671875.

The tiny difference between decimal 1e-4 and its correctly rounded binary64 value does not move it across the nearest-integer boundary. Thus:

    (k_stored-k)/k = -1.6987323760986328125e-6
    (T/k_stored)/(T/k)-1 = approximately +1.6987352618e-6.

The physical root-rotation discrepancy is about 1.70e-10 rad on a 1e-4 rad answer, even for an exact solve of the stored matrix. Which fixture predicate this violates depends on its frozen same-dimension absolute/relative criteria; it certainly is not a 1e-9-relative-accurate intended answer. Do not enlarge Q_ref or a protected tolerance to conceal it.

The original represented-equation residual can be arbitrarily small for that exact stored-matrix solution. An independent retained-contribution/spring-action check instead uses k*theta_root, whose discrepancy from T is approximately 1.70e-6 relative. This is a concrete demonstration that the represented-matrix backward error and original physical assembly error are different quantities. A more accurate residual of the already coalesced matrix, including mixed-precision refinement, does not restore the lost increment.

For N06 k=1e-12 is below half one ULP of a; the one-step addition loses the stabilizing increment altogether. Geometry still sees the positive authored spring and therefore removes the exact-model rigid motion. The represented torsion matrix can nonetheless be singular. This case must route to numerical/assembly unresolved, not physical-mechanism or success. A contribution-preserving soft-mode basis, compensated/multiword accumulation that remains available during factorization, or higher-precision assembly/solve must be independently evaluated if the supported envelope must solve it.

## 4. Independent analytical case backcheck

The circular section follows:

    A=pi*(0.20^2-0.18^2)/4=0.0019*pi m^2
    I=pi*(0.20^4-0.18^4)/64=0.0000085975*pi m^4
    J=2I=0.000017195*pi m^4.

Therefore EA=380000000*pi N, EI=1719500*pi N*m^2 and GJ=1375600*pi N*m^2. Work-conjugate force/rotation signs were checked against force/moment equilibrium and positive constitutive energy.

- N01: u_y=F*L^3/(3EI), theta_z=F*L^2/(2EI), giving the stated expressions at F=1000,L=2; reactions are -F and -F*L.
- N02: with translations fixed, equal end rotations along the element axis have no torsional strain and leave positions fixed. The stated rotational eigenvalues follow one [a,-a;-a,a] torsion block and two [4b,2b;2b,4b] bending blocks. Exactly one zero mode remains.
- N03: global RX has nonzero projection 3/5 on that null axis and removes it; global RZ has zero projection and does not. Counts and exact reduced inertias are correct.
- N04: separate free element contributes six positive deformation modes and six rigid zero modes, giving combined (12,0,6) with N01.
- N05/N06: exact physical SPD as above; assembly/working-precision qualification is additional.
- N07: eigenvalues 3,-1; v=(1,-1) gives v^T*K*v=-2 while u=(1/3,1/3) has zero residual for f=(1,1).
- N08: theta_x=T*L/GJ has the stated magnitude and sign, with root reaction -T; 1 N*m gives about 4.63e-7 rad.
- N09: F=100,L=10 gives stated EB displacement/rotation/reactions; T=0.1,L=10 gives 1/(1375600*pi) rad, about 2.31e-7 rad.
- R01/R02: 1% wrong axial extension gives 10 N residual and denominator 2010 N, hence eta=1/201. The wrong-model residual can be zero.
- R03: original applied 1000 N with u=0 gives eta=1 and zero work residual.
- R04: free row 2*k*u_mid-k*delta=0 gives u_mid=delta/2 and root/end reactions +/-k*delta/2; omitting the prescribed shift gives eta=1.
- R05: omitted spring action is k_s*u at its loaded affected DOF; freezing an actual independent spring fixture remains needed.
- R06: no free rows means vacuous free residual, while reactions are minus applied loads and imposed compatibility still applies.
- R07: zero-load null rotation has zero residual and cannot establish uniqueness.

These are derivations, not executed oracle values or a statement that every product family implements EB/objective small-strain mechanics. Timoshenko shear deformation, geometric stiffness/buckling and follower loads remain different model scopes. Pure torsion and the geometric null controls do not depend on choosing a transverse shear factor.

## 5. Current primary sources, relevance and alternatives

Access date for all entries: 2026-09-24. Web pages/PDFs were inspected through web tools; none was installed, executed or treated as an instruction. No sealed downloaded corpus or content hash is claimed.

| Source inspected | Relevant observation and selected use | Limits |
|---|---|---|
| [LAPACK release notes](https://www.netlib.org/lapack/release_notes.html) | Official latest entry 3.12.1, 2025-01-08. | This is the retrieved release page's statement, not an assumption that all netlib /double/ URLs are current. |
| [LAPACK 3.12.1 DPOSVX](https://www.netlib.org/lapack/explore-html/d6/d44/group__posvx_gae23dac18e7ec69c36fd2a2648a470dc3.html), Description and RCOND/FERR/INFO | Equilibration, positive factorization, condition estimation, refinement and distinct error measures remain current conventional practice. Working-precision condition warnings are distinct from factorization failure. | The app's deliberate unresolved publication at an extreme-condition boundary is an engineering choice; LAPACK may still compute a warned solution. No rigorous-inertia certificate is implied. |
| [DPOCON](https://www.netlib.org/lapack/explore-html/d3/dfd/group__pocon_gaadb1e19663b71521d30b5b5bbe3091f8.html), purpose and source | Estimates inverse 1-norm through DLACN2 and scaled triangular solves. | Estimation is not an upper bound; this is an algorithm reference, not proof that the in-repo implementation is correct. |
| [DPORFS](https://www.netlib.org/lapack/explore-html/d7/dfd/group__porfs_gaff660c485d1ff77c2c5c85a6bbf81900.html), residual/SAFE1/SAFE2/stopping source | Uses original matrix action and componentwise ratio, safe-minimum guards, residual decrease and bounded iteration. | Standard BERR is not presented here as an outward-certified enclosure. App force/moment units and original-assembly fidelity remain separate. |
| [DPOEQUB](https://www.netlib.org/lapack/explore-html/d1/da9/group__poequb_ga335265d8cad6dc99d803cb703b4f9cd8.html), purpose | Radix-power factors avoid added scaling roundoff absent under/overflow. | Does not repair singularity or lost assembly information. |
| [Higham and Mary, 2019, A New Approach to Probabilistic Rounding Error Analysis](https://eprints.maths.manchester.ac.uk/2731/1/paper.pdf), §2 assumptions and §3.4 | Develops smaller probabilistic bounds, including Cholesky, under explicit rounding assumptions. Useful to explain deterministic worst-case pessimism. | Do not replace deterministic guards with sqrt(n) rules or claim every authored matrix satisfies the probability model. |
| [Higham and Mary, Mixed precision algorithms in numerical linear algebra, 2022 author preprint](https://tmary.perso.lip6.fr/doc/surveyMixed.pdf), §§6–7 and sparse discussion | Refinement's attainable accuracy depends on residual precision and convergence conditions; more accurate residuals can improve difficult solves. Select as a targeted successor when double precision is insufficient. | Author PDF is version 2022-02-18; [journal metadata](https://www.cambridge.org/core/journals/acta-numerica/article/mixed-precision-algorithms-in-numerical-linear-algebra/43CA701BA29251B5790C653E66F46197) identifies publication 2022-06-09. No app runtime speedup or universal convergence is inferred. |
| [Rump and Jeannerod, Improved backward error bounds for LU and Cholesky factorizations](https://www.tuhh.de/ti3/paper/rump/JeaRu13a.pdf), floating-point assumptions and §4 | Improved deterministic analysis depends on the actual factor/triangular algorithm and absence of unaccounted range errors. Supports using an applicable algorithmic bound rather than an unexplained gamma(n) multiplier. | Not a theorem for the app's exact current skyline loop without an operation/arithmetic mapping. |

An initially attempted guessed DPOSVX Doxygen URL was unavailable. Legacy /lapack/double/dposvx.f identifies an older 3.3.1 version; it was not the current-version basis. Current Doxygen pages were subsequently followed from the official routine index. Some /lapack/lapack_routine/*.f web retrieval attempts also failed; the current Doxygen source was accessible and used. These retrieval failures are not missing software validation or successful source execution.

The contemporary numerical alternatives improve attainable accuracy, bounds or cost under their own assumptions. None makes a scalar raw pivot threshold physically meaningful, turns residual into stability, or makes a mandatory highly conservative inverse envelope necessary for an ordinary qualified FE solve. The selected first policy is conventional and deliberately makes only the claim supported by its checks.
