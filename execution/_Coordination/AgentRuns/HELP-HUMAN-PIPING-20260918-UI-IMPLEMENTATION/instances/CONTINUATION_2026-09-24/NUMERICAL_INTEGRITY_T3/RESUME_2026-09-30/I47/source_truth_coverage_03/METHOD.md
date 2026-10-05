# Finite required-cover proof for the four frozen I47 cases

This proof applies only to the captured one-member, axis-aligned, fully anchored cantilevers. It neither supplies a general corner theorem for coupled frames nor implements a numerical method or recognizer.

## Existing warrant and distinct laws

I36 `f2a_preview_truth_01/RETURN.md` §2 includes the exact normalized interpolation and the actual rounded resolver values in the ordinary material cover. Its §3 keeps the admitted K readout and source-annulus readout separately identified. I36 `f2a_ordinary_coefficients_02/RETURN.md` §1, finite realization step6, defines H_X=[min(X_-,X_hat),max(X_+,X_hat)]; §2 explicitly covers resolved E_hat/G_hat with source geometry and allows the independent positive E/G rectangle. These are existing warrants. RV47's preview-truth review states that naming two intervals is insufficient if a required third interpretation is omitted. Origins and hashes are recorded.

The following readouts are therefore retained separately:

1. Exact selected E_s/G_s with source annulus.
2. Actual resolved E_hat/G_hat with source annulus.
3. Independent mixed endpoint choices E_s/G_hat and E_hat/G_s with source annulus, and every interior E/G combination in H_E×H_G.
4. The represented K readout, using actual admitted E_hat/G_hat/A/I/J and the required represented Z cover of actual Z_hat and exact I_K/(D/2).

Readout2 is not K: replacing rounded K geometry by the source annulus changes its mechanical response. Readouts2/3 recompute the coherent source-geometry mechanics with those modulus choices; no K displacement is relabelled as a source response and no K action is silently fed to a source denominator. The source annulus remains the fixed exact normalized D/effective-wall geometry with real pi, independent of how an internal certificate encloses it. This packet does not promote every point of the implementation's conservative geometry/coefficient box to a new physical source law or reconstruct its private interval width.

## Exact material rectangle for these captures

The point case has E_s=E_hat=195000000000 and G_s=G_hat=55000000000, so both intervals are singletons.

For interpolation, the actual normalized temperatures are293,303,313 K. The exact denominator is20. Each numerator is the signed sum of the four separately lifted products Thi·Xlo−T·Xlo+T·Xhi−Tlo·Xhi. The preserved normalized operands give

    E_s = 12451840000000001 / 65536
    G_s = 13107200000000001 / 262144
    E_hat = 190000000000
    G_hat = 50000000000.

All inputs, products, numerator sums, denominator and final quotients are dyadic and exactly representable with at most62 significant bits; the script records every individual rational and bit profile. Their exponents are ordinary finite values. Thus I36's1024-bit directed realization rounds none of these material operations: X_-=X_+=X_s here. Its actual material hulls are exactly

    H_E = [190000000000, 12451840000000001/65536]
    H_G = [50000000000, 13107200000000001/262144].

Both lower endpoints are strictly positive. Independent E and G choices are covered even though only the exact/exact and resolved/resolved choices describe the two endpoints of the original selection ambiguity. The accepted proof deliberately ignores that correlation; the mixed choices are conservative cover choices, not a newly asserted isotropic E/nu relation.

## Why these corners and static rows suffice here

The script checks the actual two nodes(0,0,0),(1,0,0), one member, y-reference(0,1,0), one anchor restraining all six base DOFs, no combinations/components, and exactly three individually authored tip terms Fx,Fy,Mx, each0 or1. Source D and effective t satisfy0<t<D/2. The previously source-bound captured source/ledger and material association are preserved. These hypotheses identify the admitted, uncoupled one-member Euler–Bernoulli specimen; no arbitrary frame, spring, pressure, thermal, distributed-load or nonlinear case is inferred.

For any E∈H_E,G∈H_G, A_s,I_s,J_s,Z_s,c_s are fixed positive annular quantities. The fixed anchor removes rigid motions; positive axial, torsion and bending stiffnesses give a unique free-end response. With loaded terms1 and L=1,

    ux=1/(E A_s), uy=1/(3 E I_s),
    rx=1/(G J_s), rz=1/(2 E I_s), uz=ry=0.

Anchor components are exactly zero. The four nonzero components are positive and monotonically decreasing in their sole modulus. Translation magnitude is sqrt((1/A_s)^2+(1/(3 I_s))^2)/E, hence monotonically decreasing in E and independent of G. Therefore the four material corners enclose every component and translation magnitude over the entire positive rectangle; endpoints suffice despite material independence.

Equilibrium of this single statically determinate member fixes every end action, section-cut station action and anchor reaction independently of E/G. The original captured signs and station fractions are retained. Support force/moment norms consequently do not depend on material. Section stresses N/A_s, My/Z_s, Mz/Z_s and T c_s/J_s, and the circular maximum |N|/A_s+hypot(My,Mz)/Z_s, are material-independent for these fixed loads because their internal actions are. Torsional shear remains separate from the normal-stress maximum. Every such source row therefore has the same readout at every material corner and interior point.

In both zero cases every individual load term is zero, not merely a cancelling net. Uniqueness gives zero motions and actions for every E/G in the rectangle. All stresses, norms and maxima are zero. This numerical truth does not erase the separate negative-zero G5a publication rule.

These arguments exhaust the captured73 mechanical rows: input-derived base motions, free motions, translation magnitudes, support components/magnitudes, end/station actions, section stresses and circular maximum. They do not establish corner sufficiency in a statically indeterminate or coupled multi-member model, where actions and mixed responses may depend non-monotonically on several coefficients.

## Exact comparison and uncertainty rule

The new owned script imports only a byte-identical owned copy of the frozen independent Python oracle. It never imports product arithmetic or executes a producer/solver. The retained exact Fraction annulus, alternating-series pi bracket and integer-square-root enclosure are used independently of the candidate certificate. Each material corner and K/Z cover yields separate SI and exact raw-unit intervals.

For each immutable actual normalized n or raw y and each required predicate allowance A, the new check computes:

- a certified upper bound on the worst error over every corner, continuous material rectangle and represented readout;
- a certified lower bound on an error attained by at least one required corner/readout.

Convexity of |y−q| and the proved endpoint ranges justify endpoint-distance maxima. Positive exact raw-unit conversions preserve containment. PASS requires the certified upper bound≤A; a genuine truth miss requires the certified lower bound>A. An interval crossing the threshold is rejected as unproved. No such uncertainty occurred. Every candidate PASS is checked against the full upper bound, including individual passing predicates on rows whose overall candidate verdict refuses.

Before extending the readouts, the frozen292-row two-readout result is reproduced exactly (after JSON tuple/list normalization). That rechecks every captured normalized value, class, scale, absolute bound, exact/binary64 sharper allowance and decimal SI/raw allowance, plus unchanged G5a/observable facts. The extension changes none of those fields. It compares562 applicable mechanical predicates, certifying all390 candidate PASS predicates across the complete cover and recording every refusal without treating all refusals as automatically conservative.

Private interval-recipe correctness and private interval width remain distinct from this exact truth-cover comparison. No private endpoint exposure, source code or new runtime is involved.
