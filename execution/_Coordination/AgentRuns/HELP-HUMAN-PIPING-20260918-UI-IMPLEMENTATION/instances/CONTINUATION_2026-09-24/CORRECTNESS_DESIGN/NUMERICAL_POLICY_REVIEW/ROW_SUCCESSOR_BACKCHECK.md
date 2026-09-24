# Independent backcheck of the prospective product residual successor

Date: 2026-09-24. Executor /root/numerical_policy_review, TASK Type 2, delegated-harness-native child of ROOT. New bounded assignment; no descendants. Active Root/TASK/Piping instructions and owner correctness/reference-quality direction remain the previously read basis.

**Disposition: PASS for this bounded reference/evidence check. The selected M03 original-equation, componentwise residual metric is fit for ROOT's prospective adoption for the historically quantized product-preview continuous residuals, with the limits below.** The captured five free rows satisfy the specified numerical screen and the five independent nonzero element-theory references satisfy the unchanged relative 1e-9 comparisons. The old protected zero-force witness still fails. This review neither changes that historical result nor qualifies the current implementation.

The scope/rounding facts in PRODUCT_RESIDUAL_SCOPE remain valid. Following ROOT's live-authority clarification and this TASK's chat correction, no source was identified that non-delegably reserves this measured successor against the owner's explicit delegation of expert correctness decisions. ROOT may record the prospective method/measurement governance event under that delegation; it must attribute the formula selection to the agents' expert work, not to personal owner approval of the formula. No additional owner prompt is a prerequisite identified by this backcheck.

## Basis and independent method

The inspected packet is the numerical branch's C/SOLVER_MANAGER/NUMERICAL_INTEGRITY/POLICY_ROW_PROBE, where C is the established CONTINUATION_2026-09-24 directory under Piping. ANALYSIS.json SHA-256 is 4cb3b3edb8f9991d86e7d37c4db66f5e47f424f2c271b45981a3ebe9c95dec41, matching the supplied identity. RETURN, ANALYSIS, analyze.py, DELIVERY_HASHES, raw rows/model/result/run log, source-context manifest, original source excerpts and the logging patch were inspected or hash-checked within the bounded inquiry.

I did **not** execute the supplied analyze.py, which writes its own ANALYSIS file. I independently wrote and ran [_run_records/row_successor_backcheck.py](_run_records/row_successor_backcheck.py), using Python standard-library Fraction for exact represented arithmetic and a different pi construction: Machin's identity with rational alternating-series remainder bounds. Its independently generated [_run_records/ROW_SUCCESSOR_PROOF.json](_run_records/ROW_SUCCESSOR_PROOF.json) retains exact fraction residuals/denominators/products, numerical summaries, raw-source hashes and the Python version. This lightweight read-only-input arithmetic is the only execution by this TASK; no Cargo, unit/integration test process, build, native/browser operation, Git command or product edit occurred.

The probe's captured SparseInteractive source predates pending kernel repairs and selected-path product integration. Its manager-owned source-context manifest and logging patch bind that observation only. The patch adds capture/serialization and residual observations; it leaves the old failing assertion and numerical limits in place. The raw run log still exits on the unchanged exact-zero force assertion. Neither the packet nor this backcheck can clear current source-review findings or substitute for actual-candidate validation.

## Exact original-row and count checks

The final recorded state uses 12 global DOFs. Free DOFs are [6,8,9,10,11]; prescribed DOFs are root [0,1,2,3,4,5] plus tip UY [7]. Every prescribed value agrees with the final displacement vector. The selected original force vector includes tip Fx=100 N, Fy=100000 N and effective Fz=97 N after the explicit -3 N sliding force. The final linearized and selected-iteration displacement vectors agree.

Each row was reconstructed from **all 12 captured K/u terms**, with exact binary64 values interpreted as rational numbers. Zero-coefficient terms were included in the reconstruction. The denominator is sum_j |K_ij*u_j|+|f_i|, not a reduced-force-only or rounded-carrier expression.

| Global row | Quantity | Exact represented residual (approx.) | Denominator (approx.) | Exact ratio | Guarded observational ratio | Target |
|---|---|---:|---:|---:|---:|---:|
| 6 UX | N | -4.38416e-15 | 200 N | 2.19208e-17 | 6.66134e-16 | 4.26326e-14 |
| 8 UZ | N | -5.41960e-14 | 776 N | 6.98402e-17 | 1.18347e-15 | 7.10543e-14 |
| 9 RX | N*m | exactly 0 | exactly 0 | 0 by exact-zero rule | 0 | 4.26326e-14 |
| 10 RY | N*m | -2.47691e-15 | 388 N*m | 6.38380e-18 | 1.11022e-15 | 7.10543e-14 |
| 11 RZ | N*m | -1.54127e-14 | 1379.23096 N*m | 1.11748e-17 | 1.11022e-15 | 7.10543e-14 |

The governing observed force row is UZ. Its two nonzero-displacement products are approximately +388 N and -291 N, followed by subtraction of 97 N. Exact rational arithmetic gives -5.419598456585851e-14 N, while the ordinary sequential floating-point replay gives -5.684341886080802e-14 N. The difference is 2.647434294949502e-15 N. Thus the residual is **not solely residual-evaluation roundoff**; the captured binary64 displacement has a small nonzero residual in the exact represented equations.

The full replay has 12 multiplication evaluations, 12 sequential accumulations and one subtraction. Its conservative gamma(26)*d envelope is 2.239985974483722e-12 N for UZ. The independently measured evaluation error is below that bound. Every checked intermediate is finite and normal or zero; no overflow, subnormal or flush-to-zero caveat is being concealed for these rows.

For this capture, discarding zero-coefficient products does not change the floating result: finite zero products and additions of signed zero leave the accumulated nonzero result unchanged. The two axial/torsional rows have two nonzero coefficients; the three bending rows have four. The proposed m=2*k+2 therefore gives m=6 or 10 for an explicitly nonzero-entry evaluator. The corresponding gamma(m)*d bounds also exceed the exact measured evaluation error on every row. This validates the count treatment for this capture; it is not permission to infer every assembly/factor/residual operation count from matrix size.

The observational guard was independently evaluated as

    G_i = (|r_hat_i| + gamma(m_i)*d_i/(1-gamma(m_i)))
          / (d_i/(1+gamma(m_i))) * (1+gamma(4)),

with unit roundoff 2^-53 and the **exact rational denominator** from this probe. The exact-zero RX row uses its independently proved zero denominator and zero residual. All G_i are below 64*gamma(m_i). This is a conservative roundoff-model reference calculation. Because its denominator and final bound arithmetic are rational here, it does not validate the current runtime's floating-point denominator/error-bound implementation or create an IEEE certification claim.

## Work evidence and its governing row

For each free row, multiplying |r_i| <= tau_i*d_i by |u_i| gives the dimensionally correct consequence |u_i*r_i| <= |u_i|*tau_i*d_i. Translation times force and dimensionless rotation times moment both have N*m units. This is a per-row residual-work consequence, not independent total strain energy or a new contact convergence test.

All five exact represented work products and all observed work products lie below their respective derived row bounds. For UZ:

- observed work is 7.995487033189681e-19 N*m;
- exact represented work is 7.623103967555941e-19 N*m;
- its derived row target is 7.755622422193997e-16 N*m.

**Preserve the distinction between observed and exact maxima.** The observed maximum is UZ, but the exact represented maximum across free rows is RZ, approximately 1.155951524282885e-18 N*m; that row's observed residual/work is zero. RZ's own derived target is 7.350019048420660e-15 N*m. The source packet's UZ “exact row work” statement is correct as a row statement; it must not be relabelled the exact global maximum.

Retain row identity when aggregating work evidence. If a carrier publishes only the observed maximum, it must not borrow UZ's bound as a universal bound for all other rows. The primary per-row equilibrium checks already supply the relevant guarantees under the stated model; work is supporting evidence, and u=0 can otherwise hide a bad residual.

## Physical-reference backcheck and application envelope

The independently derived reference uses the captured intended decimal inputs: L=1 m, OD=0.168 m, wall=0.007 m, ID=0.154 m, E=200e9 Pa and gap=0.00005 m. Annular area and second moment are obtained by integration, A=pi*(OD²-ID²)/4 and I=pi*(OD⁴-ID⁴)/64. Machin's pi approximation has an explicit absolute remainder bound below 2.56e-142; it was not copied from the packet's Gauss–Legendre routine or production section calculations.

The fixed-root prismatic Euler–Bernoulli/axial relations are:

    ux = 100*L/(E*A)
    uz = 97*L^3/(3*E*I)
    ry = -97*L^2/(2*E*I)
    uy = gap
    rz = 3*gap/(2*L).

The sign of RY follows positive transverse z deflection under the stated beam convention. The y reaction check is root Fy=-3*EI*gap/L³ and tip gap reaction=3*EI*gap/L³-100000, approximately -344.8077400795 N and -99655.1922599205 N. They balance the 100000 N applied force analytically. The active gap resists the positive load; positive UZ and -3 N friction have opposing signs; the one-way support is inactive with positive UX. These observations are consistent with the particular recorded support state and explicit normal input, not a general independent qualification of every support law.

The largest relative error among the five captured nonzero displacements/rotations and these independent references is 4.3273e-16, below the unchanged 1e-9 criterion. Max selected rotation is approximately 7.5e-5 rad. L/OD is approximately 5.95238: this reference verifies the selected small-displacement EB element theory, not negligible shear in a real short pipe. A Timoshenko/pipe-section or other richer physical model has its own formulation and applicability requirements; do not claim this fixture validates real-pipe deflection accuracy.

No new primary-source search was necessary for this arithmetic follow-up: the applicable LAPACK/backward-error and modern refinement/active-set sources were already inspected and recorded in this review directory. This packet does not change their applicability limits.

## What the negative controls prove

1. **First-iteration state substituted into the final original equations:** independently fails UX and UZ row screens and the prescribed UY boundary. Initial UY is 0.014500834577689546 m instead of final 0.00005 m; the initial count residual is 3, with nonconvergence. This is a valid rejection of a stale state/solution being used as the final state.
2. **Omitted tiny load:** K=1, candidate u=0 and true f=1e-12 N gives r=-1e-12 N, d=1e-12 N and eta=1, well above 64*gamma(4). The historical six-decimal carrier and residual-work product both become zero. This proves the new row measure detects this error that either of those observations alone can hide.

The first control changes equilibrium, boundary and state facts together. It is **not an isolated complementarity/Coulomb mutation** and cannot demonstrate that removing a contact-law check would be detected. A future whole nonlinear acceptance claim still requires a state that satisfies its own linear equations but violates the relevant support/admissibility condition, rejected by that separate check. This limitation does not block adopting the continuous-equilibrium metric: the successor explicitly preserves and complements the existing nonlinear checks.

The recorded final iteration is 2, changed-support residual 0, empty changed-support list and cap 4, with gap Active, friction Sliding and one-way Inactive. Preserve the exact discrete zero criterion, cap, contact admissibility, analytical comparison limits and all unaffected protected policies. A linear residual pass alone must never set complete nonlinear success.

## Prospective adoption and remaining implementation gates

ROOT has sufficient bounded independent evidence to record the chosen method/measurement successor under the owner's delegated correctness authority, explicitly linked to the historical preview zero policy and the quantization defect. The governance event should distinguish:

- old zero-force criterion: failed and preserved as historical evidence;
- new prospective raw-equilibrium criterion: selected on original-equation/componentwise error grounds, with named operation/range assumptions and independently checked positive/negative examples;
- current runtime implementation: still awaiting repaired candidate integration, complete independent review and required checks.

No epsilon was picked merely above the observed 5.684e-14 N. No factor, contact parameter, count limit, iteration cap, benchmark accuracy tolerance or old historical value was changed by this check. The successor is a new explicit contract, not a claim that positive raw residual met the old zero limit.

Before runtime acceptance, validate the actual evaluator's denominator/error arithmetic, zero/subnormal/nonfinite behavior, original assembly/state identity and prescribed maps; exercise both dense and sparse/fallback selected paths; apply the shared positive-factor/structural/sensitivity and model-fidelity gates; retain separate nonlinear/contact checks and their controls; and run actual-candidate source review, protected suites and required product/native consumer witnesses. This SparseInteractive observational capture does not supply those results.

New origins, proof hashes and exact execution command are recorded in [_run_records/ROW_SUCCESSOR_BASIS.json](_run_records/ROW_SUCCESSOR_BASIS.json). No previously frozen governance/checkpoint file was edited.
