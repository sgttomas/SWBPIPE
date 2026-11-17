# W2 additive oblique arc: independent mathematical refutation

**Result.** No examined mechanical value or frame-order witness was refuted. Independent formation, eigenpairs, signed arc recovery and all 132 claimed paths support the additive V-ARC-OBL discrimination. R1 and R3 of the original review are repaired. Three minor description issues remain: R2's README mass summary, the additive leaf count, and wording that still describes all straights as axis-aligned. The dynamics Type 1 disposes these findings and returns the result through Agent 0 / INTERLOCUTOR; Agent 0 alone integrates and merges. This report establishes no acceptance, issuance, release, reference freeze or product comparison.

## Binding and preservation

- PR: https://github.com/sgttomas/SWBPIPE/pull/372
- Examined clean HEAD: `9a860745714d1de878b53ef0f7cdc09b244360ae`.
- Prepared checkout: `/Users/ryan/.t3/worktrees/swbpipe/t3-0e5517e6-3493f387b270f47e`.
- Bounded repairs: `18838157b2869bb1371e954a9aefc40a8fec8892` → `811ee362dd68078cff18acd11d36bfda3e03d061`; additive delta: that repaired head → examined HEAD.
- Examined package: `validation/references/f2_spectrum/recovery_w2/`.

| Bound artifact | SHA-256 |
|---|---|
| Package SHA256SUMS | `04d921e4ede948be20ea39abc1de32a5bd98a8a128ccafc4981a1acf6a097580` |
| generators/w2_cases_oblique.py | `f027b153ae2a9c0bfc64b372bcce9ecd8b2919e10d406bdd42ad620274ff582d` |
| generators/w2_recovery.py | `003e81f27a32a55460259c5b5e1b88467a80bfbb29873f7cd2848e289aee3c0a` |
| w2_v_arc_oblique.json | `2d6695d4542a4593d4678c6e6289aea5c4f621df9e1dd29dc537dec046cf48e5` |
| w2_control_arc_frame_order.json | `ebe3be2c6967809d5effce4d4d85710e8b667ea84a505ddf7e40ebfc100370ea` |
| Immutable original REFUTATION.md | `2ac8d9932209402da7335aa45ac3a63118f61577669d17d456d9ed9d79b32097` |

The original report remains bound **only** to `18838157b…`. It was not rewritten. All seven old JSON files are byte-identical between `811ee362d…` and the examined HEAD, verified against Git objects. Every current package manifest entry verified. HEAD and clean status were checked before and after; no source, expected value or worktree file was edited. No child, commit, push, PR mutation, watch or merge occurred.

## Independent mathematics and evidence

`independent_derivation.txt` was written before examining numerical output expectations or the recovery driver. The probe imports **no reference or product executable module**. It uses only standard-library Decimal, JSON, pathlib and sys. The initial derivation used the case inputs, governing scopes and U4a frame definition; the candidate derivation was also read as an argument to challenge.

**Geometry and frames.** Let t = (3,4,0)/5, v = (−12,9,20)/25 and n = t × v = (16,−12,15)/25. They are exactly orthonormal and right-handed. The plane-coordinate arc is

`x(ψ) − B = R(sin ψ, 1 − cos ψ, 0)`, `0 ≤ ψ ≤ π/2`, `R = 25/32 m`.

It gives the stated dyadic endpoints, centre `(0.375,1.28125,0.625) m`, tangent straights of lengths 5/4 and 25/16 m, and end-frame rows `(t,v,n)` and `(v,−t,n)`. The signed global-to-end-frame rotations agree with the published frames. No frame is a signed permutation.

**Formation.** Annular geometry gives A, I, J = 2I, m̄ including contents, and j̄ = 7850J, ī = j̄/2 excluding contents. Straight K and M come from integrating linear axial/twist and cubic Hermite transverse polynomials and their derivatives, including Rayleigh rotary inertia, rather than using the author's F1 routines.

For the arc, an end-C wrench `(F,M)` gives section moment `M + (xC − x) × F`, axial projection `tangent·F` and torsion `tangent·sectionMoment`. I analytically integrate the Castigliano compliance of `N²/(EA) + T²/(GJ) + k(M·M − T²)/(EI)`, invert it by pivoted Gaussian elimination, and recover end B by force/moment equilibrium. All integrals reduce to moments of `1,sin,cos` and their products; over the quarter-circle these include `π/2,1,1,π/4,π/4,1/2`. My π uses the AGM, not the author's Machin series.

Each half-arc's mass, first moment S and rotational inertia J are integrated analytically to π/4 about its own end. Direct rigid-body integration gives

`F_lump = m gt + gθ × S`, `M_lump = S × gt + J gθ`.

There is no end-to-end arc mass coupling. Subtracting these wrenches from the elastic actions establishes the same node-on-element sign and moment origin as the candidate. Full K/M and every constituent K/M, including both half-arc spatial blocks, agree within publication rounding.

**Modes and recovery.** I solve the 12-DOF problem afresh using Cholesky of M and Jacobi on `L⁻¹ K L⁻ᵀ`; the candidate uses Cholesky of K and the inverse eigenproblem. Code is independent, though both use the Jacobi algorithm class. All 12 eigenvalues, frequencies, normalized shapes, generalized masses and three-direction participation factors agree. The frequencies in Hz are approximately:

`35.4785, 76.2262, 87.0225, 192.2020, 208.1990, 224.6122, 271.4565, 275.0670, 425.7516, 503.5561, 603.6756, 661.4370`.

Thus 320 Hz extracts eight, 250 Hz makes six periodic, and the extracted set reaches the rigid frequency. The smallest adjacent relative eigenvalue gap is 0.02608, far above δc = 2⁻²⁰; every cluster is a singleton. The smallest extracted separation from the rigid frequency is 0.08583 relative; the nearest pair ratio is 0.01677 away from the 1.1 combination boundary. Every queried ordinate is strictly within its table and away from its knots. The 192.2/208.2/224.6 Hz chain gives a genuine distinction between GROUPING and TEN_PERCENT.

I independently interpolate all three spectra, then form each signed modal displacement and acceleration and the three rigid acceleration fields/static solutions. I compare all 27 sets of 12 signed global arc totals, tangent totals, elastic parts and inertia parts (1,296 component comparisons), plus rigid g and F. The largest scale-relative discrepancy is `4.37e-40`, consistent with rounding of the 40-digit publications. The independent free-DOF residual is `1.90e-80` at 95 digits. These are point-arithmetic observations, not certified bounds.

## Component discrimination

From the fresh signed responses I recompute every rule and direction. For each component I compare correct `C({λ·G_u})` with wrong A `λ·{C(G_u,c)}` and wrong B `|λ|·{C(G_u,c)}`. The size scale is the maximum of the **same end and unit**, so forces and moments are never pooled. The rule is difference/correct ≥ 0.01 and correct/end-unit-maximum ≥ 0.001.

| Path family | Paths checked | Minimum A witnesses/path | Minimum B witnesses/path |
|---|---:|---:|---:|
| Periodic: five modal rules × three directions | 15 | 11 | 12 |
| Correction off: all eight extracted modes periodic | 15 | 12 | 12 |
| Periodic + rigid: both rigid rules | 30 | 11 | 12 |
| Full grid: five modal × three rigid/OFF × three directional | 45 | 11 | 12 |
| Single signed mode, magnitude | 24 | 4 | 2 |
| Single signed rigid response, magnitude | 3 | 9 | 8 |

All 132 paths have actual component witnesses against **both** wrong orders. Every published witness list and maximum B separation agrees, as do the six named strongest witnesses. I also compare every global/tangent arc component in the 75 published periodic, correction-off and method-grid combination blocks (1,800 comparisons), not merely their norms or witness counts.

Every base component distinguishes both wrong orders. The minimum A difference is `0.07613181324` at `BEND1.j.Mz_t`; the minimum B difference is `0.3002871114` at `BEND1.j.Fx_t`. Illustrative independently computed triples `(correct, A, B)` are:

- `j.Mz_t`: `(59.94455759, 55.38086973, 85.88839308) N·m`.
- `j.Fx_t`: `(241.3879395, 183.1949186, 313.8736266) N`.
- `j.Fy_t`: `(113.2370030, −226.3931329, 226.3931329) N`.

The threshold selects discriminators; it changes neither reference responses nor the product's governed analytic verification tolerance. The screening class is a separate comparison rule. This case supplies the component discrimination absent from original C1, for this geometry and these paths only.

**Valid linear orders remain valid.** Independently, rotation of the signed sum agrees with the sum of rotated responses to `1.35e-94`, and rotating elastic-minus-lump agrees with the difference of rotated parts to `1.44e-94` at 95 digits. The control does not reject either. Original signed-permutation V-ARC's inability to reject wrong B also follows algebraically; I reused its original review and the author's regeneration instead of rebuilding that older case.

## Convergence repairs and precision

The preserved author `write.log`, `write2.log`, `check.log` and `check2.log` record failed 80/100 comparisons; `diff2.log` identifies the near-tied mode-2 strongest-witness choice changing between Z and X. `check5_final.log` passes. Their raw bytes are copied under `author_logs/`.

The current omission of relative differences affects only witness metadata after mechanical responses have been formed. My independent all-path audit finds 144 omitted components; their largest size fraction is `7.98e-80` at 95 digits. The smallest eligible fraction is `0.03149`, well above 0.001. No finite small response is being concealed at the threshold. Every path still has witnesses.

The `1e-30` first-path near-tie rule likewise selects a summary entry after recovery and combination. My computation reproduces the chosen entries; mode 2's `j.Fy_t` B separation is `14.73842924` under X, Y and Z, because each direction scales that one mode's vector. The selector is appropriate for this scale tie. It changes no mechanical response or witness-membership threshold. The retained logs do not establish every historical intermediate edit's bytes; the conclusion concerns the examined code and fresh computation.

The candidate's check-mode run passes byte-for-byte regeneration, manifest membership and its own 80/100 equality test. Independently, all **12,072** additive decimal/binary64 pairs are internally consistent; the smallest midpoint margin relative to the published decimal is `5.52538275e-21`. My independent 75/95-digit packets, including K/M, frequencies, signed arc responses and all paths, agree at 40 digits after symmetry-zero normalization. The common packet hash is `c1058196584fe3a978cc552f93f78ee7722bc65d4662a1e295ba8330c9ecbc8e`.

The brief's `2.6e-77` residual is the author's `rigid_vs_modal_expansion = 2.61e-77`, not its largest diagnostic: its maximum listed numerical residual is `m_orthogonality = 1.77e-76`. I do not certify those precise author exponents. The source-equality test remains effective despite the minor leaf-count description error below.

## Ranked remaining findings and bounded repairs

**R2 residual — Minor, description only.** [README.md:323](/Users/ryan/.t3/worktrees/swbpipe/t3-0e5517e6-3493f387b270f47e/validation/references/f2_spectrum/recovery_w2/README.md:323) lists mass-only, stiffness-only and both mutants, then says unqualified “T_k becomes 245.75 kg against 220.36 kg.” The original R2 explicitly asked that summary to be clarified. [DERIVATION.md:348](/Users/ryan/.t3/worktrees/swbpipe/t3-0e5517e6-3493f387b270f47e/validation/references/f2_spectrum/recovery_w2/DERIVATION.md:348) now correctly says the stiffness-only mutant leaves M and T unchanged. Add the same qualification to the README: only the two mass-bearing mutants change T; stiffness-only retains 220.36 kg. No new old-case mechanical computation is claimed.

**R4 — Minor, additive scalar-leaf count.** [README.md:415](/Users/ryan/.t3/worktrees/swbpipe/t3-0e5517e6-3493f387b270f47e/validation/references/f2_spectrum/recovery_w2/README.md:415) and [w2_control_arc_frame_order.json:4810](/Users/ryan/.t3/worktrees/swbpipe/t3-0e5517e6-3493f387b270f47e/validation/references/f2_spectrum/recovery_w2/w2_control_arc_frame_order.json:4810) call 18,327 the scalar-leaf population. Under the package's convention that collapses each decimal/binary64 pair to one leaf, the serialized JSON has **18,333**. [w2_cases_oblique.py:55](/Users/ryan/.t3/worktrees/swbpipe/t3-0e5517e6-3493f387b270f47e/validation/references/f2_spectrum/recovery_w2/generators/w2_cases_oblique.py:55) stores t1/t2 as tuples; [w2_recovery.py:1946](/Users/ryan/.t3/worktrees/swbpipe/t3-0e5517e6-3493f387b270f47e/validation/references/f2_spectrum/recovery_w2/generators/w2_recovery.py:1946) descends into lists but counts each tuple as one scalar before serialization. Each serialized t tuple has four leaves instead of one, explaining all six. Equality still compares the complete tuples, and the 12,072 pair count is correct. Either name this as the pre-serialization container-leaf count or count tuples recursively and regenerate the additive diagnostic. This is not a failed convergence check.

**R5 — Minor, original-case wording applied too broadly.** [README.md:280](/Users/ryan/.t3/worktrees/swbpipe/t3-0e5517e6-3493f387b270f47e/validation/references/f2_spectrum/recovery_w2/README.md:280) says “Straight members are axis-aligned,” and [DERIVATION.md:30](/Users/ryan/.t3/worktrees/swbpipe/t3-0e5517e6-3493f387b270f47e/validation/references/f2_spectrum/recovery_w2/DERIVATION.md:30) says “Every straight here” has an axis-aligned axis. The additive P1/P2 have the oblique rational directions t/v. Qualify the statements to the five original variants and state that the additive straights also have rational axes and lengths, preserving exact K formation. The additive limitations at README.md:426 already correctly say those straights publish global rows only. No numerical or member-axis expectation needs changing.

**Original R1/R3.** R1's H units and physical-K distinction are dimensionally correct: for q̂ = D⁻¹q, potential `q̂ᵀHq̂/2` gives H work units; K = D⁻¹HD⁻¹ restores physical stiffness dimensions. The bounded old diff changes units/description, not coefficients or mechanical values. R3 now correctly distinguishes 51,284 numerical pairs from 86,516 mixed leaves; my walk confirms both old counts. Neither old finding is reopened by the oblique mechanics.

## Boundaries, declarations and next actor

Read: repository AGENTS and release target; TASK from selected Library v3.4.0; relevant PRD §§12.1–12.3/12.9; DEL-04-10 scope/needs, DEL-04-09 mass/modal/needs, DEL-03-03 definition/needs and DEL-03-06 work-coefficient requirements; permitted W2 files, original refutation and bounded diffs; F1 mass-discrete/F2 discrete provenance and refutation records; U4a README frame contract and T4-I12 provenance. I inspected the F1 four-module import closure and confirmed all four current SHA-256 pins against W2's declared values (also matching the original report). The old Design path linked by U4a's README is absent in this checkout; the explicit frozen U4a frame definition and current governing scopes supplied the contract used here. No U4a executable was imported or run.

Not read: product code, tests, fixtures/output, schemas, repository tools, unrelated S2 source, private exchange, CAEPIPE, parent/other product-research sessions, PR discussion or product/reference-system mapping. The external slot wrapper was read and used as explicitly assigned. All Python executions used `/Users/ryan/dev/chirality-t3/tools/t3_slot.sh` with `/Users/ryan/.local/share/mise/installs/python/3.13/bin/python3 -I -B`. No guard was bypassed.

Scope/needs implication: the new case is consistent with DEL-04-09's half-arc mass and full-mass participation and DEL-04-10's signed inertia recovery, missing-mass and componentwise combination commitments. The DEL-03-03 and DEL-03-06 definitions need no changed commitment or dependency condition from this bounded result. Existing declarations remain unchanged; the parent owns final reconciliation and the excluded product-frame mapping. LIM-021 and all reserved acts remain in force.

Limits: this is one small-rotation Euler–Bernoulli / common-k thin-arc geometry, without a connector, rods, compliant supports, repeated modes or multi-mode clusters. It does not qualify stations, every orientation, physical applicability, other U4a mutants, oblique-straight member-axis recovery or product correctness. Old mechanical cases rely on their original head-bound report and verified byte preservation. No broad product checks or hosted PR check status were examined.

Evidence is under `/tmp/w2_refuter_arc_oblique_9a860745/`: `independent_derivation.txt`, independent `probe.py`, `audit.py`, `precision_audit.py`, complete independent packets, raw `logs/`, and verbatim `author_logs/`. `SHA256SUMS` binds the report, reviewer code and logs; the report's detached digest is `REFUTATION_ARC_OBLIQUE.md.sha256`. Keep this scratch while disposition consumes it. The dynamics Type 1 acts next on R2 residual/R4/R5 and supplies this exact report to Agent 0 / INTERLOCUTOR before integration.
