# Contribution-preserving precision: independent refutation

2026-09-24 — TASK `/root/correctness_design/precision_refutation`, delegated-harness-native child of HELPS_HUMANS `/root/correctness_design`. No descendants. This is an independent analytical design review and bounded arithmetic prototype, not product implementation, MPFR qualification, Rust execution, or a complete-candidate review.

**Disposition: the proposed adaptive binary multiprecision path is feasible for the named defects and is preferable to fixed double-double as the first general repair, subject to the concrete boundaries below.** Start candidate solves at 128 significand bits, escalate to 256 and 512 within a declared resource limit, and retain high-precision relative kinematics through action/reaction recovery. The parent's full `2p` verification solves require a backend/recovery ceiling of 1024 bits. Do not claim that 128 bits, agreement of two precisions, or MPFR's correctly rounded primitive operations proves accuracy of a complete finite-element answer. The unchanged intended-reference criterion is `1e-9`; none of this return changes it.

## What this independently checked

Read Root AGENTS, full TASK, Piping AGENTS/LOOP_INIT, current CORRECTNESS_ACTIVATION, the prior NUMERICAL_POLICY_REVIEW return/derivation, and the isolated numerical branch's oracle generator, frozen N05/N06/NP-A values, oracle handoff, KERNEL/REVIEW_REPAIR and relevant `frame_kernel/src/structural.rs` seams. Actual origin hashes and execution limits are retained under [_run_records/precision_refutation](_run_records/precision_refutation/ORIGINS_AND_EXECUTION.md).

The isolated oracle package actually inspected uses N05 `k=1e-4,T=1e-8`, N06 `k=1e-12,T=1e-16`, and an ULP sweep. It does **not** contain the briefing's possible `k=1e-10,T=.1` NP-A variant. That variant is retained below as a separately named algebraic stress control, with a realistic small-angle companion. It must not silently replace the frozen oracle identities.

The current structural source retains exact contribution expansions for audit but assembles/factors `Vec<Vec<f64>>`, solves/refines in f64, and returns `StructuralSolution.displacements: Vec<f64>`. Its intended-action check still consumes these rounded global displacements. That is a useful detection layer, not the stronger solve/recovery repair. The source expressly says its intended residual does not establish forward accuracy of a soft mode.

## 1. The distinction that a valid repair must preserve

For `a=687800*pi N*m/rad`, the two-coordinate energy is

    E(theta) = 1/2 k theta0^2 + 1/2 a (theta1-theta0)^2 - T theta1.
    K = [[a+k,-a],[-a,a]], f=[0,T].
    theta0=T/k; delta=theta1-theta0=T/a; reaction=-T.

Positive `a,k` give exact positive definiteness, independently of their ratio. There are three distinct bases:

1. The analytical decimal/material/geometry problem used by the frozen reference.
2. The exact mathematical problem defined by individually supplied binary64 inputs and retained element/support/constraint operations. A binary64 value can be lifted exactly into a wider binary format; this does not recover the user's preceding decimal, geometry, or unit-conversion intent.
3. The already rounded/coalesced binary64 global matrix. Solving this matrix more accurately cannot recreate a contribution it lost.

The N05 one-step assembly makes `k_stored=214748*2^-31=0.00009999983012676239013671875`, causing approximately `1.6987352618e-6` relative root error even with an exact stored-matrix solve. For N06 that step removes `k` altogether. Double-double **initialized from the rounded global matrix** has the same information loss as MPFR initialized from it.

Preserving only individually rounded *global element entries* also is insufficient on arbitrary orientations. They can perturb an element's exact rigid nullspace by `O(u64*a)`, exceeding the intended tiny spring. In the independent two-coordinate oblique analogue, hard and soft energies use orthogonal represented vectors `v=(f64(.6),f64(.8))`, `w=(-v1,v0)`, with `K=a vv^T+k ww^T`. Re-evaluating these factors at 128 bits gives about `2.83e-21` relative displacement/soft-action error. Lifting individually rounded f64 global energy terms and adding them at 128 bits produces a nonpositive computed LDL pivot. This is a numerical analogue, not a deployed spatial-frame test, but it refutes a matrix-entry-only repair.

**Required construction:** re-evaluate the actual supported element energy/strain factors, transformations, support directions, constraint maps, prescribed-value load shifts, equivalent loads and constitutive operations at the selected precision from a declared immutable source basis. Where possible retain `B_e`, positive constitutive factors and their shared identities so `K_e=B_e^T D_e B_e` and `q_e=B_e u-q_ref,e` describe the same element. Do not invent a `B^T D B` factorization of an arbitrary already rounded matrix and call it restored physics. Supported element families must supply their actual kinematic/energy contract.

Shared products/sums must be shared consistently. A rounded independent geometry normalization, matrix triangle, load rotation or prescribed-displacement shift can reintroduce the defect before factorization. Computing only `a+k` at high precision is not adequate scope for a generic repair. If source coordinates have already collapsed to the same f64 value, higher precision cannot recover their lost separation. New precision addresses computation on represented inputs, not unquantified physical/source uncertainty.

## 2. Exact expected outcomes and physical scope

All scalar cases use the same `a`; signs shown are positive applied tip torque and negative grounded reaction. The analytical expressions, rather than their displayed decimals, are the reference.

| Case | k (N*m/rad) | T (N*m) | theta0 (rad) | delta=T/a (rad), approximately | Required action/reaction |
|---|---:|---:|---:|---:|---|
| Ordinary-scale small-angle companion | 100 | .01 | 1e-4 | 4.627942515030397e-9 | `+.01` transmitted; ground `-.01` |
| N05, unchanged | 1e-4 | 1e-8 | 1e-4 | 4.627942515030397e-15 | `+T` transmitted; ground `-T` |
| N06, unchanged | 1e-12 | 1e-16 | 1e-4 | 4.627942515030397e-23 | `+T` transmitted; ground `-T` |
| Supplemental large-angle algebra control | 1e-10 | .1 | 1e9 | 4.627942515030397e-8 | Algebra only; outside small-rotation model |
| Supplemental small-angle companion | 1e-10 | 1e-14 | 1e-4 | 4.627942515030397e-21 | `+T` transmitted; ground `-T` |

`theta1=theta0+delta` in every case. One f64 ULP near `1e-4` is `2^-66≈1.355252715606881e-20 rad`. N06 delta is only `0.00341481885` of that spacing; the small-angle companion is `0.341481885` ULP. N05 has resolvable global twist but does not have enough relative twist digits for `1e-9` action accuracy when recovered by subtracting independently rounded global rotations.

The two unchanged physical fixtures have small global rotations. They are meaningful analytical small-displacement continuum/numerical references, despite extremely soft springs and tiny applied torques; they do not demonstrate practical measurability or qualification of a real spring device. The `1e9 rad` control must remain labelled algebraic, never realistic pipe validation. No geometric-nonlinear theory is needed to diagnose arithmetic at `1e-4 rad`; its physical adequacy outside that regime is separate.

For the supplementary eight-span scalar chain, with positive span stiffnesses `a_i`, exact equilibrium gives `theta_j=T/k + sum_(i<j) T/a_i`; every element transmits `T` and the ground reaction is `-T`. This is an independent generic connected-chain control, not a full 3-D piping benchmark.

## 3. Executed arithmetic evidence

[probe.py](_run_records/precision_refutation/probe.py) uses no product imports. Its generic dense SPD LDL loop has no case-name or fixture-specific solve branches. It executes two kinds of arithmetic:

- A real two-word high/low prototype using binary64 TwoSum and `math.fma` product residuals, normalized addition/multiplication and three quotient corrections. This is genuine multiword arithmetic, not compensated summation followed by f64 factorization. It is an experimental transcription, not certification or execution of the QD library.
- A `Fraction` oracle emulating correctly rounded binary arithmetic after **every** operation at 53/106/128/192/256/512 significand bits, ties-even, with unbounded exponents. This isolates significand precision; it is **not** an MPFR run and does not test finite exponent limits, allocation, Rust bindings or platform behavior.

Both start from exactly decoded f64 `a,k,T` and compare against exact rational analytical chain solutions on that same input basis. This separates arithmetic error from f64 input conversion. The independent Machin evaluation of pi supplies the displayed decimal reference and nearest f64 `a`. The raw reproducible output is [probe.stdout.json](_run_records/precision_refutation/probe.stdout.json).

| Case | Actual two-word DD max displacement/action relative errors | Binary128 emulation max displacement/action relative errors | Action error if high-precision u is first published as f64 |
|---|---|---|---|
| N05 | `4.36e-22 / 2.99e-22` | `4.44e-29 / 3.06e-29` | `3.38e-7` |
| N06 | `4.63e-19 / 6.97e-15` | `5.59e-21 / 5.88e-21` | `1` (zero twist/torque) |
| Small-angle companion | `4.63e-17 / 7.78e-18` | `8.59e-23 / 7.58e-23` | `1.93` for binary128 result's final rounding |
| Eight-span N06 analogue | `4.96e-18 / 7.03e-15` | `1.03e-21 / 3.88e-21` | `1` |

The small-angle companion's independently rounded endpoints can differ by one f64 ULP even though the true twist is below half an ULP: their position relative to a rounding boundary matters. Sub-ULP separation does not always imply identical published values; it does imply that their difference is not an adequate internal recovery representation. The exact decimal `theta0=1e-4` basis and the exact-f64-input `T/k` basis can land differently at that boundary.

Negative controls were retained:

- Coalescing to f64 first and then using binary128 retains N05's `1.698735e-6` displacement error and makes N06's computed pivot nonpositive.
- The eight-span rounded-matrix path returns a positive computed factor with about `0.996` displacement relative error and near-unit action error. Positive factorization alone does not establish model fidelity.
- Rounding a correct internal displacement solution before action recovery yields the errors in the table, despite individually accurate published global rotations.
- The additional small-angle precision stress case `k=1e-28,T=1e-32` keeps `theta0≈1e-4`. Binary128 errors are `4.46e-6` for displacement and `1.39e-5` for action; binary256 errors are about `2.23e-43 / 1.94e-43`. The two-word DD prototype gives an accurate-looking root but loses the entire relative twist/action. Thus neither a fixed 106-bit path nor a fixed 128-bit path is a general guarantee, even with small global rotations. This stress case is not added to or substituted for an existing protected fixture.

These are bounded arithmetic observations. No product test, Rust/Cargo/build, UI/native endpoint, source integration, or measured application performance is claimed.

The ordinary-scale `k=100,T=.01` companion contrasts the difficult cases without changing rotation magnitude. The binary53 emulation has displacement/action errors `2.27e-13 / 1.24e-12`, already inside `1e-9`; there is no numerical reason in this bounded case to require escalation. Its purpose is ordinary-path continuity, not actual hardware qualification.

## 4. Precision, conditioning and range: what follows and what does not

For the two-coordinate torsion system, ideal diagonal congruence has `kappa_2≈4a/k`: approximately `8.64315e10` for N05, `8.64315e18` for N06 and `8.64315e16` for the companion. Scaling does not remove this soft-mode condition. `u_p=2^-p`; a conventional first-order estimate `C*m*kappa*u_p` must include the actual algorithm/operation factor. The rough N06 condition consumes about 63 bits; nine relative decimal digits cost another 30 bits. DD's nominal 106 bits therefore leaves only about 13 bits for that simplified estimate, versus about 35 at 128 bits. These are precision-planning estimates, not error certificates.

Action accuracy has an additional output sensitivity: `q=b^T u` has an absolute error bounded by `|b|^T |delta u|` plus its own evaluation/input errors. Dividing by a tiny `|q|` can magnify global displacement errors greatly; a normwise displacement check alone does not bound the relative action. A worst-case normwise factor bound followed by subtraction can even introduce an additional condition factor. The observed correlated solve/recovery behavior of these fixtures must not be promoted to a universal `kappa*u` action theorem.

The parent's quantity-adjoint alternative also checks algebraically: if `K^T z=c` and residual is `r=K*u-f`, then `c^T(u-u_exact)=z^T r` on the exact fixed-source system. For torsion `c=[-a,a]`, `z=[0,1]` exactly. This explains why an accurately evaluated tip residual can tightly assess this particular torque even when global rotations are sensitive. It requires the intended `K` and intended output map; changed constitutive/geometry/load terms and numerical adjoint error remain separate. An approximate adjoint estimate is useful evidence and does not reintroduce mandatory formal inverse certification.

Use p-aware arithmetic uncertainty and factor/condition screens on the high-precision candidate. The f64 `rcond<=EPSILON` gate would deliberately reject N06 and therefore cannot remain the acceptance gate for a stronger internal solve. Its replacement must identify actual precision and its uncertainty rationale; it is not permission to change the protected `1e-9` intended-output comparisons. Preserve the binary64 matrix/recovery discrepancy as evidence where useful, without forcing an intentionally rounded publication view to satisfy equations it cannot represent accurately.

An adaptive attempt should rebuild from the same immutable source inputs at the new precision; increasing the precision of already rounded intermediate values merely appends zeros. Refinement must compute the original intended contribution/source action at higher precision and correct with a factor of the same preserved problem. Low-precision factors may be used only where a stated convergence test supports them. A singular rounded f64 preconditioner cannot be assumed to work for N06. Direct higher-precision factorization is the simpler reliable starting path here.

At each precision retain intended-equation residuals, imposed compatibility, positive-factor evidence, scale/condition estimates, refinement history and the actual relative kinematics/actions. Cross-precision agreement of displacement **and actions/reactions** is useful operational evidence, but can agree on the wrong source model or a consistently omitted term. It is not a rigorous enclosure. Retain the unchanged independent fixture comparisons and mutation controls. If 512 bits or the memory/time bound is exhausted, return numerical-unresolved with the attempted precision/range and affected output; never a fabricated mechanism or solved status.

The emulator has unbounded exponents. A real implementation must choose/check MPFR exponent limits and exception flags, guard conversion to f64, and account for intermediate products, extreme geometry, constraint coefficients and factor growth. DD keeps essentially the binary64 component exponent range and needs explicit overflow/underflow-safe product residuals and scaling. Neither method permits silently flushing a nonzero stiffness/action to zero. A published required nonzero action below f64 range needs an explicit representability outcome or an additional high-precision representation; the model being solved internally does not make such a publication loss harmless. No fixed precision cap solves all possible finite input problems.

## 5. Relative coordinates versus multiprecision

The independent two-coordinate change `theta0=q0`, `theta1=q0+q1` gives energy `1/2 k q0^2+1/2 a q1^2-T(q0+q1)`. It is exactly diagonal, with `q0=T/k`, `q1=T/a`, and separate f64 storage/recovery already gives approximately `1e-16` action errors in these bounded controls. The analogous scalar tree stores a root value and edge differences. This is a strong independence oracle and explains the defect; it is not authorization for a special-case production branch.

For a general frame graph, an invertible basis preserving all rigid motions, bending/axial/torsional strains, cycles, releases, offsets, supports and prescribed constraints is a substantial additional formulation. A spanning tree requires cycle compatibility; simply storing every edge difference makes redundant coordinates and constraints. Mode/energy coordinates require a checked transformation, rank and mapping back to global loads/actions. Their construction can itself be ill-conditioned and their sparsity/fill cost can differ markedly. No generic basis of that scope was designed or validated by this return. It should remain an optional formulation improvement rather than a prerequisite for the immediate contribution-preserving path.

DD is a credible bounded fast path: 16-byte two-word values and ordinary hardware operations, but it requires reviewed normalization, division, product residual, range and compiler assumptions throughout assembly, factorization and recovery. A pair of summary high/low audit fields is not DD arithmetic. Adaptive multiprecision has clearer per-operation semantics and an explicit escalation route, at allocation/memory/operation cost. Dense factorization still has cubic arithmetic work/quadratic storage; a sparse/profile implementation retains its fill-dependent work but each scalar becomes more expensive. No speed ratio or practical model-size ceiling was measured. The implementation owner must qualify a library/binding and target platforms, and measure representative sparse models before making cost claims.

## 6. Internal recovery and publication contract

Retain an immutable solution object bound to the source/active-state/precision and factor evidence. Preserve high-precision global unknowns long enough to compute each element's `q_e=B_e u-q_ref,e`, spring deformation, fixed-end/equivalent-load action and reaction. Store the relative kinematics and resulting actions with their entity/frame/unit/provenance mapping. The generic finite-element operation is recovery from the element kinematics and constitutive relation; do not fill `T` or `T/k` into a case by recognizing its fixture name.

Only after internal recovery and checks should the user-facing global displacements and forces be rounded to finite f64 publication fields. A UI/export may legitimately show equal N06 nodal rotations while showing a nonzero accurately recovered torque. Recomputing a later result, reaction, stress, combination, nonlinear state, or saved-result view from those rounded global values would reintroduce the bug. Every such consumer must use preserved actions/relative state or rerun recovery from the immutable high-precision solution. Publication fields are views, not the authoritative computational state. This scope is necessary for the claimed repair, not optional presentation polish.

## 7. Primary technical reference check

Accessed 2026-09-24; URLs were read, not installed or executed. No unreviewed OCR corpus was used.

- [GNU MPFR 4.2.2 manual](https://www.mpfr.org/mpfr-current/mpfr.html), precision/rounding/exception sections: per-variable binary precision, correctly rounded operations, explicit rounding modes and a configurable wider exponent range provide a suitable semantic basis. The manual explicitly distinguishes arithmetic precision from tracked accuracy; MPFR itself does not prove whole-computation error. Its `mpfr_can_round` requires an independently known error bound. This is support for a library design choice, not deployment qualification.
- [Hida, Li and Bailey, Library for Double-Double and Quad-Double Arithmetic](https://www.davidhbailey.com/dhbpapers/qd.pdf), introduction and error-free building blocks: multiword representation and TwoSum/TwoProd/FMA residual arithmetic explain why genuine DD differs from one-word accumulation or stored-matrix promotion. The source documents assumptions and algorithm qualifications. Its historical timings are not app performance evidence, and its algorithms are not a universal finite-element error certificate.
- [Carson and Higham, Accelerating the Solution of Linear Systems by Iterative Refinement in Three Precisions](https://eprints.maths.manchester.ac.uk/2629/1/cahi18.pdf), 2018: factor/correction, working and residual precisions have distinct roles; convergence/attainable errors require conditions. It supports adaptive/refined solves on retained equations. It does not justify recovering erased assembly contributions, blindly reusing singular factors, or claiming every mixed-precision loop converges.

These sources justify the numerical mechanisms, not new beam physics. No newer theory invalidates the small-rotation torsion equilibrium used here. The selected immediate alternative is adaptive arithmetic on the existing justified source mechanics; a universal relative-coordinate frame formulation remains unproved in this bounded work.

## Return boundary

No blocking mathematical counterexample was found to the bounded `128 -> 256 -> 512` source-preserving design with internal recovery and truthful unresolved outcomes. The retained counterexamples **do** block narrower claims: fixed precision sufficiency, matrix-entry-only promotion, forward accuracy from residual alone, and recovery from published f64 global displacements. Parent must integrate the source-energy, recovery-consumer and library/range requirements into its design. Implementation, independent candidate review and live-path evidence remain with their named owners.

The current parent [CONTRACT.md](CONTRACT.md) was read in full and its final two clarifications backchecked: candidate `512` plus `2p` verification explicitly means a `1024`-bit backend ceiling, and f64 values are rounded from the checked retained numerical result rather than asserted correctly rounded from the unknown exact solution. Its adjoint relation, opaque source-bound state, prospective precision-specific screens, persistence/recovery separation and unchanged protected comparisons are consistent with this refutation. The reviewed contract hash and final bounded checks are in this task's run records; no public version was allocated.
