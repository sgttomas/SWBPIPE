# W2 recovery candidate: source-independent refutation

**Result.** No mechanical reference value examined was refuted. Three statements need repair: the units attached to the connector work coefficients (R1), the mass consequence attributed to the stiffness-only ownership mutant (R2), and the description of the precision-comparison population (R3). The arc-frame coverage limit C1 must remain explicit when relying on this package. This is a completed bounded examination of the identified candidate, not acceptance, issuance, release or a product-comparison verdict. The integrating parent disposes the findings, commissions a fresh reader of repairs where needed, copies this record unchanged, and regenerates the package manifest. Agent 0 owns integration.

## Candidate and preservation

- PR: https://github.com/sgttomas/SWBPIPE/pull/372
- Reviewed head: `18838157b2869bb1371e954a9aefc40a8fec8892`.
- Frozen author root: `/Users/ryan/.t3/worktrees/swbpipe/t3-47d275be-879e5dcb5692c971`.
- Package: `validation/references/f2_spectrum/recovery_w2/`.
- SHA-256 of the candidate `SHA256SUMS`: `a1c501f89419cc3f3c20189d072c7187499254b2ffae5cece84a87e5867c6f0b`.

The author checkout was clean and at that head before and after examination. All eleven manifest entries verified before and after. An independent inventory check found exactly those eleven files plus `SHA256SUMS`, with no additional file. There was no `REFUTATION.md` in the candidate. No source, expected value, package, manifest or other worktree file was edited. No commit, push, PR mutation or watch was performed.

The manifest pins the following reviewed bytes:

| File | SHA-256 |
|---|---|
| DERIVATION.md | `4dd627a01fe7fca3b0b1600a753e4b8c82dc76a2325f813b4884a4d1bf77e62e` |
| README.md | `bbd8abec310635c28f3f5f2e733eab5c665b70ce07abb78c7525ad4ed058c9fd` |
| generators/w2_cases.py | `f9cd0b7d0abefa6176db65e8c562b7ae7324ea422bc2b5fa85625f56d190747f` |
| generators/w2_recovery.py | `c00fac72b30443af140890a9ebf6fb5dc0f7ed94d81c2eb33436736cbc8688f0` |
| w2_controls.json | `714a2910859643c8c05b14e68fcb608ed69e1ccf18f403c8e960682a10cc266f` |
| w2_inputs.json | `db071941f0ece1fdd4003319f2e377614b1e18b0122bc4337b42ba0973a1a63a` |
| w2_v_arc.json | `52c4267aee0c5f980203f6c801f7b3802aed1b722c27b5d59055cf2d38b6f860` |
| w2_v_comb_0.json | `624b220d2f2cb03904a6513915dbe9bb0bf62e8957a84ee3719bd366d8ca91e9` |
| w2_v_comb_r.json | `b999e5b82b19ecf4ff7689ee3892e0da32ee1443ad5c82405a1b1d1cdf8d966b` |
| w2_v_joint_0.json | `ae2720dd8a76424f80b15b125366833be3c4d3955135e73d7b0867f935dc57e8` |
| w2_v_joint_r.json | `b8965b8ddb6e227d2444f2fe1818c63a2b811da015a176ed39662d75b482a48d` |

## Independence and sources actually read

I loaded the repository AGENTS instructions, release target, and `agents/AGENT_TASK.md` from `/Users/ryan/dev/chirality-library-releases/library-v3.4.0`. No child was started. Governing mathematics came from DEL-04-10 CLM-001 and REQ-001 to REQ-004/REQ-008, DEL-04-09's mass, extraction, clustering, normalization and participation requirements, DEL-03-03's realized-arc definition and limitation, and DEL-03-06's objective-connector, replaced-span and rod requirements. I read the four deliverable manifests and relevant PRD §12.3 and §12.9 requirements.

Before reuse I read F1 mass-discrete provenance and its refutation, F2 discrete provenance and its refutation, T4-I12 provenance and its connector contract, T4-U5b provenance and rod contract, and T4-U4a's README frame/derivation contract. I inspected the candidate's complete two generators, README, derivation, manifest and seven JSON outputs. The F1 module reading was their identities, import closure and descriptive contracts, not reuse of their computational routines. T4-I12 reading included its kinematic, scaling and direct-wrench definitions. No private exchange, CAEPIPE or held material, product implementation, product test, product fixture/output, or parent research session was read. PR 367's inherited work was not examined. The authored reference-system specification was not needed. No U4a generator was imported or run.

The author's four pinned F1 modules match the hashes in the candidate README, including after execution: `elements.py` (`c2a7b890…696f0`), `model.py` (`c219d158…46761`), `eigen.py` (`22ae605c…7d1c6`), and `numeric.py` (`e08ff240…dd1e7`). Their prior formation evidence remains F1 evidence; sharing them with F2 is not a second formation derivation. My probes import none of them, none of the W2 generators and none of the T4 generators. Their shared dependencies are the stated governing mathematical model, conventions and public reference-side inputs, not executable formation/eigensolver code.

All mechanics in the probes use exact `Fraction` or 85-digit `Decimal`, with standard-library modules only. Binary64 is used solely for represented-input and publication-rounding audits. My π is the Gauss–Legendre AGM, independently of the author's Machin implementation. Every local probe and the candidate's separate check-mode run used `/Users/ryan/dev/chirality-t3/tools/t3_slot.sh`. The guard was not bypassed. An initial `--help` invocation queued because the wrapper treats its argument as a command; I terminated that invocation and used the documented command form. Raw probe logs, including two failed development executions of my probe and their subsequent repairs, remain in scratch. Those failures were Python type/name errors in my script, not candidate findings.

## Independent derivation and computation

I recorded `independent_derivation.txt` before opening the W2 numerical output JSON. The following routes were then implemented independently and compared with candidates.

**Connector.** With zero connector offsets, set `v = uj − ui − ((θi + θj)/2) × r` and `w = θj − θi`; then `q = (Qᵀv, Qᵀw)`. For local generalized force `(f,m)`, put `F = Qf`, `C = Qm`. Direct virtual work gives the two node-on-element wrenches

```
end i: (−F, −C − (r × F)/2)
end j: ( F,  C − (r × F)/2).
```

This direct wrench construction, not multiplication by the candidate Bᵀ, generated my connector end actions and K columns. Its force resultant vanishes; its total moment vanishes because the node-position contribution `r × F` cancels the two half-arm terms. My independently formed exact Q, B and K_e agree entrywise with the candidate rationals. Eighteen adverse exact rational displacement vectors satisfy virtual work. Three rigid rotations about a translated origin and the rigid-motion identities give zero strain exactly. The assembled system stiffness also annihilates its rigid rotation fields to working precision.

**Rods.** The two offsets are `±(3/20, 0, 9/80) m`, with `n = (0,1,0)`. The attachment definition gives `e = n·(uj − ui) + (a × n)·(θj − θi)`. In connector coordinates the two reductions are `e = qx ∓ (3/16) qrz`. Their combined stiffness in those coordinates adds `2kr` axially and `2kr(3/16)²` to bending z; the cross terms cancel. I recover tension-positive end forces `∓Tn` and moments `∓a × Tn` directly, and compare those with the published end actions. Exact rational virtual-work, rigid-motion and reduction controls pass. Connector g and rod forces remain separate observables.

**Straight members.** My mass and stiffness come from exact integration of axial/torsional linear polynomials and transverse Hermite polynomials on `[0,1]`, including their first and second derivatives. This independently establishes the rotary-inertia and transverse rotation signs, rather than copying the F1 displayed matrices or its frame-free implementation. Sections come from annular geometry; contents enter translations and not j̄ or ī. Zero allowances make EX and PF equal here, with no inference about a nonzero-allowance distinction.

**Arc stiffness.** I reconstruct the 90° geometry as `x(ψ) = (1.5 + R sin ψ, R − R cos ψ, 0)`. With end-j force F and moment M, the section moment is `M + (xj − x) × F`, its torsion is the tangent projection, and axial force is the tangent projection of F. The Castigliano tip-flexibility integral needs only the integrals of `1, sin ψ, cos ψ, sin² ψ, cos² ψ, sin ψ cos ψ` over `[0,π/2]`: `π/2, 1, 1, π/4, π/4, 1/2`. I form this analytic 6×6 flexibility, invert it by pivoted Gaussian elimination and spread it by rigid equilibrium. This is not the author's imported arc formation or its quadrature check.

**Arc inertia and recovery.** For each half I analytically integrate the first moments, displacement outer products and tangent outer products using endpoint `√2/2` and `α = π/4`; no Gauss nodes or quadrature are used. From mass m, first moment S and total rotational inertia J, direct rigid-body integration gives

```
F_lump = m gt + gθ × S
M_lump = S × gt + J gθ.
```

I subtract these wrenches from elastic arc actions, about the correct end node, both for `g = λu` and for the rigid acceleration field. The analytic lump matrices, first moments, centroid offsets, centreline inertia and section rotary inertia match the candidate. Total arc mass and centroid also match the independent whole-arc solution `(m̄Rπ/2, (1.5 + 2R/π, R − 2R/π, 0))`.

**Modal and rigid routes.** First I tested all published eigenpairs against my independently formed K and M. I then solved all 84 finite modes afresh: Cholesky of M, Jacobi on `L⁻¹ K L⁻ᵀ`, and transformation back to physical DOFs. The candidate instead uses Cholesky of K and the inverse eigenproblem. Thus the second solution shares the Jacobi algorithm class but neither its code nor its matrix formulation. My static solves use pivoted Gaussian elimination rather than the candidate's Cholesky solve. Every periodic and rigid recovery was recomputed on this fresh eigenbasis and compared directly with the published outputs.

All five variants have the declared free counts and simple modes. The smallest adjacent relative eigenvalue gap over all five is `0.002965274721939…`, far above `2⁻²⁰`; every scope cluster is a singleton. The extraction lists are 6, 5, 5, 6, 5 modes and periodic lists 4, 4, 4, 4, 3, respectively. The extracted set reaches f_r in every case. Pure torsion is normalized as case (b) and has zero participation in the joint variants.

I independently compute `p = (MR)_f`, generalized mass, Γ, effective and cumulative masses, full-DOF T, missing-mass fields and constituent shares. Static rigid displacement is checked against the resolvent sum over every nonperiodic finite mode, including unextracted modes. Per-node action sums equal zero at free nodes and anchor reactions at restrained nodes. Global anchor forces and moments about the origin balance the full inertia load, including restrained rows. The span D–E has no mass, pipe K or member-result key; connector and rods have no inertia share.

Every LL, GG and PLG spectrum response is evaluated on its authored abscissa. I recompute all five modal rules, both active rigid rules and OFF, and all three directional rules. The CQC expression is independently written in symmetric frequency form. OFF includes every extracted mode. Signed arc totals are transformed before my combinations; C1 qualifies the discrimination this geometry provides.

## Numerical evidence and controls

Normalized discrepancies below use the matrix maximum, the physical quantity's SI unit-block maximum, or the quantity's own magnitude as implemented and labelled in the scratch summaries. These are diagnostic comparison scales, not newly set product tolerances.

| Examination | Coverage | Largest discrepancy |
|---|---:|---:|
| Independent full K and M | 4,320 entries each | `1.20e-40`, `1.82e-40` |
| Constituent K and M | 3,312 and 1,944 entries | `7.40e-41`, `1.82e-40` |
| Fresh eigenvalues and normalized shapes | 84 eigenvalues; 2,592 shape components | `3.31e-40`, `4.54e-40` |
| Fresh-eigenbasis periodic recovery directly against candidate | 81 signed modal responses; 10,014 components | `4.08e-40` |
| Fresh-eigenbasis rigid recovery directly against candidate | 15 signed rigid responses; 1,848 components | `4.70e-40` |
| Fresh-eigenbasis base combined magnitudes directly against candidate | 616 components | `3.76e-40` |
| Every modal rule, periodic and OFF | 9,240 components each | `3.07e-38` using rounded published modes |
| Every 45-entry mutation grid | 27,720 components | `2.79e-38` using rounded published modes |
| Full rigid F and constituent shares | 432 and 540 components | `4.37e-40` |
| Missing-mass disclosure | 105 quantities | `4.06e-40` |
| Pressure/reference forbidden-constant mutants | 38 combined components | `2.07e-40` |
| Three span mutants | 24 frequencies, 9 masses, 108 combined components | `3.14e-40`, `1.07e-40`, `3.94e-40` |

The rounded-published-mode route's larger residuals are explained by using 40-digit modes in cancellation-sensitive recovery; the fresh-eigenbasis route removes that dependency and gives the directly compared figures above. All independently computed nonzero periodic components also round to their candidate binary64 partners.

Reference-case controls include an independently integrated one-element axial witness: `Γ = 3/2`, modal mass `T/3`, effective mass `3T/4`, and support reaction `−3T/4` under unit pseudo-acceleration. On each of the five assembled models, taking all finite modes periodic cancels the free rigid load and leaves total support reaction `−(T − pᵀM_ff⁻¹p)`; taking none periodic gives `−T`. All fifteen direction cases satisfy both controls.

The exact affine increment examination reproduces 144 checks. At fixed geometry, K, B and b, subtracting actions at `d0 + δ` and `d0` cancels q_ref, held displacement and the constant pressure pair exactly. The nonzero held state gives `P = 46875 N`, and its exact connector and face constants match the candidate. This supplies no EX static equilibrium, pressure-region mapping or nonlinear-state qualification.

The forbidden-constant minimum relative separations are independently reproduced:

| Constant mutant | V-JOINT-R | V-COMB-R |
|---|---:|---:|
| Held connector force | `46.35927` | `4.027946` |
| Thrust folded into axial g | `40063813.54` | `400045.25` |
| Held face action with thrust | `225.4980` | `21.02503` |

The zero-reference torsional connector/face components in V-JOINT-R also become nonzero under the relevant mutants; they are not silently lost by the relative-distance metric.

Independent span-mutant solutions give:

| Model | First frequency, Hz | T, kg | Minimum selected relative separation |
|---|---:|---:|---:|
| Reference V-COMB-R | `21.3997254430370` | `220.355770244907` | — |
| Extra span mass | `19.4422286841775` | `245.747945974377` | `0.004128681526` (`F.Fx`) |
| Extra span K | `29.3689952259508` | `220.355770244907` | `0.003918346377` (`F.Fy`) |
| Extra span mass and K | `28.2571349783992` | `245.747945974377` | `0.01157922896` (`A.Fx`) |

The arc-only, joint-only and rod-absent/present controls were each rebuilt, not inferred from the combined model. They preserve the stated failure-localization purpose. The joint variants' unchanged first frequency when rods are added is consistent with that mode producing no rod extension; it is not evidence that rods were omitted.

## Ranked findings

**R1 — Major: H is labelled with physical stiffness units rather than work-coefficient units.** `generators/w2_cases.py` supplies `H_units = [N/m, N/m, N/m, N*m/rad, N*m/rad, N*m/rad]`, copied into `w2_inputs.json`; the README model table and DERIVATION §1.3 also attach these units to H. With normalized coordinates `q̂ = D⁻¹q`, the potential is `q̂ᵀHq̂/2`, so H's coefficients are in `N*m`. Physical K has the stated translation/rotation stiffness dimensions after `K = D⁻¹HD⁻¹`. DEL-03-06 defines work coefficients and requires dimensional checking; T4-I12 explicitly stores `H_upper_triangle_21_N_m` and `coefficient_unit = N*m`.

This is a published dimensional error, even though Ls = 1 m and rotation scale 1 rad make H and physical K numerically coincide here. An exact adverse scale control gives the same physical response at Ls = 2 m only when the three translational diagonal H entries are multiplied by four. Calling those coefficients N/m while also dividing by a length squared would instead give N/m³. Correct the H metadata and explanatory wording; label the physical K diagonal separately. No mechanical expected value needs to change for the present inputs. A separate reader should examine the repaired metadata and regenerated package.

**R2 — Minor: the stiffness-only mutant does not change T.** DERIVATION §7 says, “Each mutant changes T_k, the frequencies and the combined values.” The stiffness-only mutant leaves M and therefore every `T_k = R_kᵀ M R_k` unchanged. Its published T and my independently computed T both remain `220.355770244907… kg`, while the mass-bearing mutants give `245.747945974377… kg`. Clarify the README's corresponding mass summary too. The mutant values and their discrimination are correct.

**R3 — Minor: 86,516 is a scalar-leaf count, not a count of paired numerical values.** The generator's `count_leaves` includes metadata strings, flags, nulls and integers. Independently walking the exact 80/100 comparison population gives 51,284 decimal/binary64 pairs, 31,154 string-zero leaves, 377 other numeric strings, 3,418 other strings, 17 booleans, 264 integers and 2 nulls: total 86,516. The README's statements that all 86,516 published values agree “in the 40-digit decimal and in the binary64” should distinguish the scalar-leaf population from the numerical pairs. The precision comparison itself succeeds and excludes diagnostic `cross_checks` as the generator says.

**C1 — Reliance limit: this arc cannot discriminate every rotation-after-combination defect.** The exact global tangent-frame rows are identity at B and `[(0,1,0), (−1,0,0), (0,0,1)]` at C. These are signed permutations. For any of the stated componentwise magnitude rules, applying the absolute permutation to already combined global magnitudes gives exactly the same local magnitudes as transforming the signed contributions first. My reconstruction gives zero difference on every base arc total in all three arc-bearing variants. Thus the code inspected does transform before combining, but these values cannot kill an implementation that maps the magnitudes afterward by the correct absolute signed permutation. This limits a generalized frame-order qualification; it does not refute these fixed-geometry values. As an analytic contrast outside this fixed geometry, two modal vectors `(1,1)` and `(1,−1)` viewed in a 45° frame give `√2` under correct transformed SRSS and `2` when magnitudes are rotated afterward. Use an already independently established non-permutation reference or an additive rotated-geometry case if the parent needs that discrimination.

## Precision, dependencies and proof limits

All 113 represented-input audit entries agree in decimal target, exact binary64 rational and hex, with the declared ζ exception. The rational connector frame is exactly orthonormal. Authoring its coefficients as binary64 0.8 and 0.6 is a distinct model: its exact `QᵀQ − I` diagonal defect is `3602879701896397 / 81129638414606681695789005144064`, approximately `4.4408920985006264e-17`. No such rounded frame entered my mechanical computation.

All 51,284 published decimal/binary64 pairs are internally consistent. The independently recomputed smallest relative midpoint distance is `1.47146444323152168676e-21`, confirming the rounded `1.47e-21` claim. The candidate's default check mode regenerated every output byte for byte and confirmed manifest membership. This also reruns the author's 80/100 convergence comparison. My own computation used 85 digits and an independent π, formation and eigenproblem route; it was not a second 80/100 convergence experiment. Neither this comparison nor the candidate's point-arithmetic residuals constitute outward-rounded mathematical error bounds or a proof of correctly rounded exact solutions. The stated very small eigen/equilibrium/quadrature/frame residuals remain the author's internal diagnostics; I did not recertify their exact exponents. The new analytic half-arc route independently challenges the formation/recovery mathematics without rerunning its 48-point integral as proof.

The fresh eigenbasis route directly rechecks every signed periodic and rigid component and the base combined magnitudes. The full 45-rule grids were independently recomputed from the validated, rounded published modal basis; I did not repeat every grid on the fresh basis. There is no new proof here for F1's entire model domain or F2's repeated-mode/cluster cases. There are no exact repeats or multiple-mode clusters in W2. The all/none-periodic controls are algebraic controls on the same assembled systems, not additional published/admitted spectrum requests.

The retained domain is small-rotation linear Euler–Bernoulli straight and thin curved-beam mechanics, one common bend k, these invented unequal face pipes, the stated exact frame, zero connector offsets, explicit nonzero rod offsets, bilateral massless rods with zero pretension, and fixed affine-state subtraction. It does not cover spring/snubber/nonlinear supports, Timoshenko straight mass, directional k, finite rotations, nonzero connector attachment offsets, rod pretension or pressure stiffness. EX/PF section discrimination belongs elsewhere. No static EX equilibrium, nonlinear qualification, product tolerance, product implementation correctness or CAEPIPE comparison is established. LIM-021 remains the thin-bend applicability premise; matching its mathematical model does not establish its physical adequacy.

The governing declarations need no scope/dependency amendment as a consequence of the mechanical results examined: the mass, inertia recovery, modal participation and replaced-span ownership match the cited DEL-04-09/DEL-04-10 and component contracts. R1 is a candidate representation repair under DEL-03-06's existing dimensional requirement. Nothing in this review changes a commitment, criterion, hold or reserved owner act.

## Review evidence for the parent

Scratch: `/tmp/swbpipe-w2-refuter.JT3bsg/`. Keep it while disposition or the next review consumes it. The authored report is `REFUTATION.md`; raw evidence and reviewer code are outside all worktrees.

- `independent_derivation.txt`: derivation recorded before numerical-output examination; SHA-256 `fb18abbe8010c3c6d875c27f261e0f1c904b8459137baeb913827b415105f950`.
- `probe.py`: independent exact/Decimal assembly, published-mode and combination checks; SHA-256 `e34ab04af1b4fa2a4b9406074483d751322ada46d94207674c97a34da26f407b`.
- `extra.py`: fresh M-congruence eigensolutions, direct candidate comparisons, controls and mutants; SHA-256 `5e690432d8ac7d9399ee15a9a04443d635fe3cda5b097b6ecc063d29cd4c2ef8`.
- `complete.log` and `extra_summary.json`: final complete probe output, no failures.
- `audit.py`, `audit.log`: independent inventory, leaf-count, import/pin and modal-gap inspection.
- `author_regeneration.log`: separate check-mode result, “check passed: every output regenerates byte for byte; SHA256SUMS and file membership agree”. This is reproducibility evidence, not my refutation method.
- `identity_before.log`, `inspect.log`, `identity_after.log`: candidate head, package hashes, clean status and preservation checks.

The parent can reproduce my final computation with `t3_slot.sh python3 -I -B /tmp/swbpipe-w2-refuter.JT3bsg/extra.py`, which loads only my own scratch `probe.py` plus allowlisted reference JSON. The failed `probe.log` and `probe_rerun.log` remain raw alongside the passing `probe_corrected.log` and final `complete.log`. No package values were altered to obtain agreement.
