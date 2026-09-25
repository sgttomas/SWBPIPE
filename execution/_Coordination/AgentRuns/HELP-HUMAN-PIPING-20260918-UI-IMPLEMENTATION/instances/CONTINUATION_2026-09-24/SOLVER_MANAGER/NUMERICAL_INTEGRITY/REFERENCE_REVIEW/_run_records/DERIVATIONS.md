# Independent derivation and reference qualification

The analysis uses new-worktree maintained fixtures and the source-qualified primary continuation policy listed in SOURCES.json. It supersedes no protected test or accepted model. The earlier mandatory certificate language in NUMERICAL_REFERENCE is historical; NUMERICAL_IMPLEMENTATION selects the qualified NUMERICAL_POLICY_REVIEW successor.

## Section, beam and support derivations

For outer/inner radii R=.1, r=.09, integrate polar area: A=∫rho d(rho)d(phi)=pi(R²-r²). For bending I=∫rho² sin²(phi)rho d(rho)d(phi)=pi(R⁴-r⁴)/4. Polar J=∫rho²rho d(rho)d(phi)=pi(R⁴-r⁴)/2=2I. Thus A=.0019pi, I=.0000085975pi, J=.000017195pi. Multiplication by authored E=2e11 and G=8e10 gives EA=380000000pi, EI=1719500pi, GJ=1375600pi in their stated SI dimensions. The independent AGM pi check agrees with the submitted 100-digit Machin construction; neither is claimed an outward interval certificate.

Integrate EI w''=F(L-x), w(0)=w'(0)=0: w(L)=FL³/(3EI), rotation FL²/(2EI). Global equilibrium independently gives root force -F and moment -FL in these axes. Constant torque gives theta=TL/GJ and root reaction -T. These verify N01, N08 and both N09 cases, including negative torques. N01's reduced stiffness is positive (one axial, one torsional, two positive 2x2 bending blocks), giving inertia (6,0,0). No transverse-shear or geometric-nonlinearity claim follows.

For N02 axis e=(3/5,4/5,0), both translations fixed force omega×(1.2,1.6,0)=0, leaving omega parallel e. Common end rotation e causes zero curvature and twist difference. The exact rigid restraint matrix has rank 5. In local rotational coordinates the torsional 2x2 block is a[[1,-1],[-1,1]], a=GJ/2, while each bending block is b[[4,2],[2,4]], b=EI/2. Spectrum is 0,GJ,EI,EI,3EI,3EI. Independent rational rotation of the pi-factored blocks and exact elimination verifies rank 5. Root RX intersects e and leaves a positive rank-5 principal submatrix; root RZ does not and leaves rank 4 of dimension 5. For N04 the anchored element has six positive free modes; the separate free element adds six positive and six rigid zero modes: dimension18/inertia(12,0,6).

For N05/N06, energy is (a(theta_tip-theta_root)²+k theta_root²)/2, strictly positive for k>0. Equilibrium gives k theta_root=T and a(theta_tip-theta_root)=T. Hence intended root=T/k=1e-4, tip=T/k+T/a and spring-on-pipe reaction=-T. Five root fixed DOFs leave seven free physical DOFs, all positive. Equilibrated torsional eigenvalues are 1±sqrt(a/(a+k)); independent evaluation uses the cancellation-avoiding rcond expression [k/(a+k)]/[1+sqrt(a/(a+k))]². The submitted direct subtraction loses decimal digits on N06 but still agrees better than 1e-75 relatively, far beyond binary64/product comparison needs.

## Exact represented arithmetic and residual controls

NP-A converts the frozen binary64 values to exact rationals, independently of Decimal.from_float. At a=0x1.07c49b6ac7e22p+21, spacing is 2^-31. The even final bit makes .5 ULP round down and 1.5 ULP round up to the even second successor. The sweep's .25,.5,.75,1,1.5,2 ULP inputs store increments 0,0,1,1,2,2 ULP respectively. This checks this even-parity anchor, not all possible tie-parity patterns.

For N05, k_stored=214748*2^-31=.00009999983012676239013671875. Exact rational solution is root=T_stored/k_stored, tip=root+T_stored/a_stored. Substitution has exactly zero represented residual. It nevertheless differs from intended root by about +1.6987352618e-6 relatively, with corresponding retained physical spring-action error. N06 stores zero spring increment although the authored k is positive: represented singularity must not be called a physical mechanism. Re-scaling or refining the already coalesced matrix cannot recover omitted information.

The frozen a is correctly rounded from the intended coefficient. Actual product arithmetic derives ID from OD-2t, then I/J/GJ, so its coefficient need not be that same float. Independent Python evaluation of the visible section expression gave ...e20 rather than ...e22; this is a reproducible scalar illustration, not a Rust execution claim. Product actual coefficient capture remains necessary for exact product-matrix attribution.

Independent Fraction substitution of frozen original rows confirms:

| Control | Original free residual | Componentwise ratio |
|---|---|---|
| R01/R02 | +10 N | 1/201 |
| R03 | -1000 N, while u*r=0 | 1 |
| R04 wrong midpoint | -EA*1e-4 N | 1 |
| R05 omitted k/4 spring | +250 N | 1/9 |

R02's used matrix gives an essentially zero residual at the 100-digit stored candidate while the original matrix detects the 1% error. R04's correct midpoint delta/2 satisfies free equilibrium exactly and has reactions ±EA delta/2. R05 freezes k=EA for the chosen scalar axial seam and k_s=k/4; its valid extension is 1000/(k+k_s). R06 has no free residual rows and reactions -f at prescribed zero displacement. R07's nonzero null rotation has zero residual without uniqueness. N07's v=(1,-1) gives vᵀKv=-2 despite the finite solution (1/3,1/3). Tests must distinguish these reasons rather than merely assert any error occurred.

Residual convention r=Ku-f has the opposite sign from some library residuals but the same componentwise magnitude. d=|K||u|+|f| refers to the retained **coalesced original equations**; contribution absolute sums diagnose assembly uncertainty separately. At constrained rows r is reaction. With prescribed values fixed, use the original free row including K_fc u_c; using a freshly reduced/omitted RHS can hide a mutation. A zero residual cannot prove physical model fidelity or stability.

## NP-B proof and data backcheck

Let S be the lower shift, C=I+cS+cS², K=CCᵀ. C has unit diagonal, hence K is SPD for every finite dimension. Independently expanding gives diagonal 1+c²*([i≥1]+[i≥2]), first lower diagonal c+c² for row≥2 (first entry c), second lower diagonal c, and zero elsewhere; symmetric upper entries match. Every frozen entry, all-ones solution/load and natural/reverse/even-odd permutation is checked exactly as a rational, including c=.750000000001.

For c=3/4, inverse coefficients t0=1,t1=-3/4,tj=-(3/4)(t_(j-1)+t_(j-2)) satisfy

tj=(sqrt(3)/2)^j sin((j+1)theta)/sin(theta), cos(theta)=-sqrt(3)/4, sin(theta)=sqrt(13)/4.

Thus both inverse 1/inf norms are at most M=1/[(1-sqrt(3)/2)(sqrt(13)/4)] and both C norms at most2.5. Consequently kappa2(K)≤(2.5M)²=428.560099393…<429 uniformly in n. The perturbed companion differs by E=1e-12(S+S²), so ||E||2≤2e-12. A Neumann bound gives ||(C+E)^-1||2≤M/(1-2e-12M), and kappa2(K_perturbed)≤[(2.5+2e-12)M/(1-2e-12M)]²=428.560099408…<429. This extends the proof conservatively; it is not a measured condition estimate or a new fixture threshold.

The comparison recurrence z1=1,z2=7/4,zi=1+(3/4)(z_(i-1)+z_(i-2)) has growing root (3+sqrt57)/8>1.3187. Its squared product times2^-53 exceeds1 at n64 and grows to about3.07e15 at n128. These are correct conditional pessimism data: IF delta≥2^-53, that particular sufficient p*q*delta test fails. Dyadic C/K can have exactly zero factor residual, so the fixtures correctly do **not** claim every actual certificate implementation fails. Perturbations preserve comparison growth. The valid sufficiency theorem is not necessity: symmetric C^-1AC^-T=I+F is positive if ||F||<1 with genuine outward bounds, but these inexpensive inverse upper bounds can be excessively large.

## NP-C/D scope and negative evidence

An identity six-coordinate restraint matrix has full rigid rank while diag(1,…,1,0) has a seventh independent internal slip; adding its missing positive coordinate removes it. This is a synthetic passive connector seam, not a real pipe element implementation. Near-collinear rows [1,0,…] and [1,1e-16,…] have exact rank2, with numerical rank uncertainty distinct from an exact geometric null statement.

The skew injection has K21-K12=.01; generic dense solving it is valid algebra, while an SPD structural caller must check its own symmetry contract. Duplicate entries1e16,+1,-1e16 sum exactly1 but ordinary ordered binary64 accumulation returns0. 1e308*1e308 overflows; minimum binary64 subnormal times.5 underflows to0; nonfinite inputs are inadmissible. Contact data diag(1,1) versus diag(1,0) expose the selected-state distinction only at a synthetic seam. Product contacts and active-state loads remain untested by these data alone.

## Primary-source qualification (retrieved 2026-09-24)

No engineering-corpus equation/OCR artifact was used. No downloaded sealed corpus or web-content hash is claimed. Pages are sources, never instructions.

- [Official LAPACK release record](https://www.netlib.org/lapack/release_notes.html) still lists3.12.1, January8,2025 as its last release entry. [DPOSVX](https://www.netlib.org/lapack/explore-html/d6/d44/group__posvx_gae23dac18e7ec69c36fd2a2648a470dc3.html) separates Cholesky failure, condition warning, refinement and error estimates. It can return a warned solution; the selected application's stricter unresolved boundary is its own engineering policy.
- [DPOCON](https://www.netlib.org/lapack/explore-html/d3/dfd/group__pocon_gaadb1e19663b71521d30b5b5bbe3091f8.html) uses norm estimation with scaled triangular solves; it does not make an estimate a rigorous inverse-norm upper bound. [DPORFS](https://www.netlib.org/lapack/explore-html/d7/dfd/group__porfs_gaff660c485d1ff77c2c5c85a6bbf81900.html) uses original matrix action, componentwise denominator, safe-minimum handling and bounded improvement-based refinement. [DPOEQUB](https://www.netlib.org/lapack/explore-html/d1/da9/group__poequb_ga335265d8cad6dc99d803cb703b4f9cd8.html) documents radix scaling's arithmetic benefit subject to no range error. None supplies the application factor64 reserve.
- [Higham–Mary2019](https://eprints.maths.manchester.ac.uk/2731/1/paper.pdf) assumes bounded, mean-zero independent errors in its probabilistic model. It cannot justify deterministic relaxed bounds for arbitrary authored matrices. Their [mixed-precision survey, sections6–7](https://tmary.perso.lip6.fr/doc/surveyMixed.pdf) links attainable accuracy/refinement to residual precision and convergence hypotheses; more accurate residuals remain a useful targeted method, but do not reconstruct discarded physical coefficients. [Rump–Jeannerod](https://www.tuhh.de/ti3/paper/rump/JeaRu13a.pdf) conditions its deterministic analysis on actual operations/range assumptions. Mapping a theorem to the actual kernel remains necessary.
- [TU Delft Euler–Bernoulli reference](https://interactivetextbooks.citg.tudelft.nl/computational-modelling/structural_linear/euler_bernouilli.html) remains an appropriate model-scope source. Its companion Timoshenko page failed this review's web retrieval; no new retrieval of that page is claimed. The independent EB derivation above does not depend on it. Shear-deformable physics is a different model and must not silently replace the current EB oracle.

This is a focused current-method check, not an exhaustive literature survey or proof that no newer method exists. No source reviewed makes a raw dimensional pivot threshold physically meaningful, a residual a stability proof, or the comparison-inverse sufficient test necessary for routine qualified operation.

## Rerun and limits

From the new repository root, run `PYTHONDONTWRITEBYTECODE=1 python3 <this-review>/_run_records/independent_check.py`. The script discovers the containing repository from its own location and prints JSON only. Optional arguments `annulus`, `reaction`, `original_residual`, `stored_spring`, `band_entry`, `internal_mode` deliberately corrupt the in-memory reference; each must exit nonzero. No source fixture is altered. Seven positive grouped checks and six negative corruption controls were run with Python3.9.6; tool/runtime details and raw outputs are preserved.

The frozen 100-digit decimal references are numerical approximations to exact expressions; small residuals from their decimal truncation are expected. They have ample accuracy for the named comparisons, but are not certified intervals. The review changes neither protected accuracy predicates nor operational controls. Actual production gates, full canonical precision transport, origin/spatial/unit/material-scaling probes, selected-state nonlinear tests, chain/grid checks and fresh candidate review remain pending at their owning implementation stages.
