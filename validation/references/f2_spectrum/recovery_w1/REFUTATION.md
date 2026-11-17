# W1 recovery reference refutation

No numerical candidate was refuted in the examination below. Two descriptive
statements are refuted: the README's precision-comparison count and the case
module's claim about exact decimal literals. These need the reference owner's
disposition. This result neither accepts a baseline nor establishes anything
about product implementation, physical correctness or code compliance.

The examined candidate is PR 371,
https://github.com/sgttomas/SWBPIPE/pull/371, head
`52ede188dda972c3070653a288d9ff3a2dbbdd4c`, read from the frozen author checkout
`/Users/ryan/.t3/worktrees/swbpipe/t3-380956a5-d4bcb564223dbcfc`, package
`validation/references/f2_spectrum/recovery_w1/`. The complete seven-file
inventory, bytes and HEAD were checked before and after examination. Every
package file also matches its committed HEAD blob; no symlinks or extra files
were found. The manifest is complete for this candidate, which has no
`REFUTATION.md` yet.

| File | SHA-256 |
| --- | --- |
| README.md | 0881fabd4e8a8e541f202caa4b3242bad393f0afdb03811786f50ee1b96be84a |
| SHA256SUMS | ebb82f6aff0a38ecf04941ad5d7f0269c8af3476f8af1ef74e1cac50eafd07a6 |
| generators/w1_cases.py | 14feed74c01361480148839e72ad0b0c17258a1b0354a73e6f47c1c9aadec2d9 |
| generators/w1_recovery.py | 97a41aa7542783536432a6d5b8dbfd35bf4a282b13f4115ebba2d534b1853f24 |
| w1_base.json | 519bf05a2c6abe35c795a1092446f5159840a5c170b365b11746bd410f3a729a |
| w1_controls.json | 9d29064942c72aa3e478015b9e5ba52a6a4517f0983a0a48cf3ba5189bda0a69 |
| w1_variants.json | e4e52eae29afebaaa693b69e91f906fa4798322bd0322852182d5323f61e0e1b |

**Findings for disposition.**

1. README.md, line 402, says: "All 147,609 published pairs have the same
   40-digit decimal at 80 and 110 digits". The candidate's control JSON says
   `published_pairs_compared: 84429`. Independent enumeration of the seven
   variant records gives **84,429**, and the full generator check reproduces
   that control JSON byte for byte. The base file contains 29,997 pairs and
   the variant file 54,432. The control file contains another 966 pairs;
   those summary pairs are outside the generator's seven-record convergence
   count. Replace 147,609 with 84,429. No numerical expectation needs changing.
2. generators/w1_cases.py, lines 3–4, says every number is a binary64 value
   "written exactly in decimal (dyadic fractions and integers)." The literal
   `ZETA = 0.05` does not write its binary64 value exactly. The actual damping
   operand, correctly used by the candidate and published in its input table,
   is `3602879701896397/72057594037927936`, which exceeds 1/20 by
   `1/360287970189639680`. Describe literals as denoting binary64 values and
   name the exact input table as the arithmetic basis. The README already
   correctly calls damping "binary64 0.05". This is a representation-wording
   finding, not a request to replace the operand with 1/20.

**Boundary and governing basis.** I loaded the repository standing instructions,
release target and TASK role from the selected Library v3.4.0. Governing reads
included DEL-04-10's scope and needs, DEL-04-09's scope and needs on mass,
modal basis and participation, DEL-05-03's section-basis table, DEL-03-05's
component and reducer requirements, DEL-03-04's branch definition, and relevant
PRD component and modal requirements. The package is a bounded mathematical
reference under DEL-04-10 CLM-001, REQ-002, REQ-004 and REQ-008/VER-006, using
DEL-04-09 REQ-001 and REQ-005. I found no mismatch requiring a change to those
scopes or dependency conditions; their wider product obligations remain wider
than this reference family.

I read no product implementation, product test, product fixture/output,
parent research session, private exchange, CAEPIPE or held material. Reads
under the author root were confined to governing documents and the allowed
reference packages. I made no source edits, commits, pushes or watches. Every
local generator and probe execution used
`/Users/ryan/dev/chirality-t3/tools/t3_slot.sh`, without bypassing its guard.

Before dependency reuse, I read the F1 mass-discrete README and refutation and
the F2 discrete provenance/refutation material. The candidate imports four
unchanged F1 formation/numerical modules. Their pins and the two JSON control
pins were verified before helper reuse and again after the probes:

| Dependency | SHA-256 |
| --- | --- |
| F1 elements.py | c2a7b890c90d8c2d016fc72036aad36165173cb9f146158bea7629364bd696f0 |
| F1 model.py | c219d158aa2aa260153dff303514508a4c41e4983b2b1c5236a2beb79c046761 |
| F1 numeric.py | e08ff24066f636f0d6c76e38ae95de2ef42e9e1c0f651714910034575d5dd1e7 |
| F1 eigen.py | 22ae605c36ae47ad7e80369ecbee422646bef67acb1d46294a744228589d17c6 |
| F2 c3_cantilever_3d.json | 54860437fd3df398d732595bd152716872e8ec0d05e94265064adbac01a1b1aa |
| F2 c9_pipe_diagonal.json | c669ceea4182b2c20b1cacf5d587b5d3151cde55a9fb33f2c112179a48c3e0ba |

Static inspection found only standard-library imports, the candidate's case
module and those four F1 modules. The imported F1 modules have no file-reading
operations; the driver reads its package and pinned reference controls. F1 is
put first on the import path and import origins are asserted. Candidate replay
and helper experiments share F1's prior formation evidence; they are not new
independent formation proof. My primary recomputation below imports none of
those generators. Importing candidate helpers for the later adverse probes
and running its check mode are separate experiments, not the grounds for the
primary numerical verdict.

**Independent mathematics and computation.** Before numerical comparison I
wrote the recovery derivation in scratch. For local displacement fields
(u,v,w), linear axial displacement and twist and cubic Hermite transverse
fields give right-handed section rotations theta_y = -w' and theta_z = v'.
I integrated their polynomial products directly:

    2U = integral [EA u'^2 + EI(v''^2+w''^2) + GJ theta_x'^2] dx
    2T = integral [mbar(udot^2+vdot^2+wdot^2)
                  + jbar theta_xdot^2
                  + (jbar/2)(theta_ydot^2+theta_zdot^2)] dx

This yields local energy matrices, rotated by my Gram–Schmidt frames and
assembled by node. It differs from F1's frame-free projector formation. Metal,
contents and insulation translational fields and the two material rotary
fields were integrated separately. A body uses its actual CoG velocity
B q = u + theta cross r, so M_body = (W/a) B^T B. Only the eligible body's
mapped distributed mass is removed; its pipe stiffness remains separately
scaled. Contents enter translations but not section rotary inertia.

Virtual work against each nodal basis function gives the signed member wrench
K_e u_e - M_e g_e, with g = omega^2 u for a periodic contribution. Assembly
then leaves (K u - M g)_c as the force the support applies to the piping.
A body's inertia belongs to its node, not to the mapped member's recovered
wrench. Testing with global rigid translations and rotations annihilates
elastic work and gives force and moment balance, including x cross F.

For rigid recovery I formed every full mass row before extracting free rows:
g = a_r(R - sum_periodic Gamma phi), followed by a partial-pivot LU solve of
K_ff u_f = (M g)_f. A separate complete nonperiodic modal expansion,
a_r sum_nonperiodic Gamma phi/lambda, checks that solve. Also,
R^T M g = a_r(T - sum_periodic m_eff), and complete reachable mass is
p^T M_ff^-1 p. These give reaction and support-moving mass identities without
using the author's balance routines.

My eigen route is M-Cholesky, Householder tridiagonalization, scalar Sturm
bisection and inverse iteration, with normalization and participation formed
independently. It differs from the candidate's K-Cholesky/Jacobi route and its
pencil-LDL bisection control. All 33 finite modes of each variant were solved
at 100-digit Decimal precision. Pi came from the Gauss–Legendre AGM. Fraction
checks covered input representation and rational frames. Logarithmic spectral
interpolation used an independent ratio-based expression on the table's own
abscissa. CQC used the symmetric omega expression rather than the driver's
frequency-ratio expression. No product or external numerical package supplied
any value.

**Numerical coverage.** Counts below are scalar comparison visits, not claims
of separate experiments. Every comparison passed. For published-value rows,
the discrepancy is |mine-published| / max(|mine|, |published|, 1e-30); the floor
allows exact/flushed zeros. Every nonzero value encountered in the primary
published-value comparisons also rounded to exactly the published 40-digit
number: 74,108 such visits. This includes repeated summary occurrences and
is not the manifest's payload count.

| Independently computed population | Visits | Largest discrepancy |
| --- | ---: | ---: |
| All finite frequencies, seven variants | 231 | 3.94e-40 |
| Published eigenvalue/omega/period/generalized-mass and bounding-box quantities | 392 | 3.97e-40 |
| Published shapes, 11 per variant | 3,696 | 4.81e-40 |
| Participation, effective mass, fraction and cumulative fraction | 924 | 4.74e-40 |
| Total and constituent translational masses | 147 | 2.64e-40 |
| Missing-mass fractions, every variant and direction | 147 | 3.54e-40 |
| Spectral and scaled ordinates, rigid accelerations | 441 | 3.01e-40 |
| Signed periodic/extracted modal global recovery | 30,870 | 4.96e-40 |
| Signed rigid global recovery | 3,087 | 4.48e-40 |
| Signed member-axis recovery, where published | 4,284 | 4.80e-40 |
| BASE modal inertia shares, resultants and node balances | 8,010 | 4.77e-40 |
| Rigid full-DOF/constituent/member/body/restrained shares and node balances, all variants | 14,301 | 4.91e-40 |
| Base-request combinations, global/member axes and per direction | 6,468 | 4.84e-40 |
| BASE 45-entry method grid, global and member axes | 10,395 | 4.90e-40 |
| CQC coefficients | 700 | 4.77e-40 |
| Variant-summary frequencies, values and ratios | 945 | 4.82e-40 |
| C3-BASE independent control recovery/combination | 1,848 | 4.80e-40 |
| C9-Y independent control, including member axes and spring | 1,771 | 4.46e-40 |

At the independent 100-digit working precision, 231 signed responses
(7 variants times 3 directions times 11 responses: 10 extracted modes and one
rigid response) had maximum scaled free/nodal residuals below 2.90e-95, global
force-and-moment residual below 8.87e-96 and element-resultant residual below
2.85e-97. The 21 direct rigid solves versus complete nonperiodic modal
expansions agreed within 2.09e-94 relative. Every total reaction along its
excitation matched -a_r(T - sum_periodic m_eff) within 1e-65 relative. Each
published rounded shape also satisfies the independently formed pencil: the
largest scaled residual was 2.92e-41. These are measured arithmetic residuals,
not rigorous error bounds.

The generator's check mode passed: all three JSON files and SHA256SUMS
regenerate byte for byte, including its 80/110-digit convergence run. All
85,395 decimal/binary64 pairs across the three JSON files have binary64
partners equal to rounding their published decimals. The exact input table
contains the complete 63 distinct numeric operands used by the seven models,
spectra and base request, plus the stated derived G. Every rational/hex/
round-trip representation agrees. E = 1.9e11 and nu = 1/4 give G = 7.6e10
exactly. The modulus is a fixed state input; no state or pressure-dependent
stiffness was introduced.

**Variant-specific attempts.** Every variant was solved and recovered
independently; no variant was cleared from BASE's balance alone.

- MILL and CORR have independently identical EX K and M. Their S1 pressure
  bore depends on the same sum of deductions, while nominal metal, section
  rotary inertia and stiffness do not move. All 48 section-contrast entries
  were checked: PF corrosion does nothing, while PF mill changes its metal,
  contents and stiffness section. No PF dynamic analysis is implied.
- BODY-INELIGIBLE retains BV's pipe mass, removes V1's body mass and retains
  its stiffness scale of 4. Its K equals BASE's K. The warning and missing-CoG
  condition are stated. The whole-model mass falls from 1801.47917318111 kg
  to 1265.71904061452 kg, and combined A1.Fz changes from 2438.76019114674 N
  to 1636.08939132714 N.
- BODY-COG0 retains V1's translational mass while removing its offset coupling.
  Its K and total mass equal BASE's. In the Z rigid response, V1's load at
  restrained N6.Fz changes from 1543.29836150797 N to 1579.53873343636 N;
  R1.Fz changes from -2270.49783852282 N to -2218.43005435076 N. Restrained
  body inertia is therefore exercised directly.
- CONCENTRIC has the same chord length and constituent totals as BASE, but
  different chord geometry and frames. V1's CoG offset stays unchanged.
  Its combined A1.Fz is 2424.43021951410 N. Geometric eccentricity and body
  offset were not substituted for one another.
- REDUCER-BODY replaces RD's distributed mass with RC1's 1200/a point mass
  and offset, leaving RD stiffness unchanged. Both body nodes and shares are
  exercised. Its total mass is 1899.16809524614 kg and combined A1.Fz is
  2512.43657861758 N.

Every member chord, not just its length, was checked in exact rationals.
Published axes are exactly orthonormal and right handed. In particular RD
has x=(12,4,-3)/13, y=(0,3,4)/5 and z=(25,-48,36)/65. Member forces and
moments are rotated with the same proper frame before nonlinear magnitude
combination. For RD.i force, the correct combined member components are
approximately (955.234, 1255.666, 779.680) N. Rotating already-combined global
magnitudes instead gives (939.139, 1454.685, 314.607) N. The candidate matches
the former, so this is a discriminating frame test.

**Adverse probes.** After completing the independent recomputation, I compared
candidate helpers with independent routes on additional cases:

- Arbitrary positive and negative, non-eigen displacement/inertia fields in
  every variant: all global/member recovered components agree below 4.56e-98
  with the polynomial-energy route. This challenges signed inertia and
  ownership without relying on eigensolver equilibrium.
- An all-rigid load a_r M R in X/Y/Z in every variant: direct recovery gives
  total reaction -a_r T. Removing the complete finite modal projection leaves
  negligible free load but nonzero restrained load. In BASE with a_r=3.25,
  the latter includes N6.Fz=536.001868999303 N under Z. Completing the modal
  set does not erase mass moving with the supports.
- An exact 10-percent boundary and a three-frequency chain (100,109,118)
  with signed values (1,-2,3): GROUPING gives sqrt(18), TEN_PERCENT gives
  sqrt(30), and absolute sum is 6. A near-equal pair (1,1+1e-20) with opposite
  signs gives CQC about 1.41510e-19, versus SRSS sqrt(2). Symmetric independent
  CQC and the candidate agree at 130 digits within the probe threshold.
- Body CoG r and -r: point-velocity energy equals q^T M_body q. Translation
  of the global origin changes moments by the independently computed
  translation cross total force. Contents remain outside section rotary mass.
- Flush/guard examples: a relative 1e-51 value flushes, 1e-40 is refused,
  and 1e-34 is retained. Existing published zero decisions are consistent
  with the independent recomputation. This is not exhaustive testing of all
  guard boundaries.

**Limits and next action.** These are numerical falsification attempts on the
stated discrete Euler–Bernoulli family and the two controls. They do not prove
all mathematical cases or correctly rounded outputs by interval enclosure.
I did not certify the author's precise 80-digit residual exponents, the exact
4.5e-43 route-A/route-B diagnostic, every refusal path, or every near-zero
rounding boundary. Those remain reproducible internal diagnostics; the
independent values and residuals above are the separate challenge to them.

No arc, connector, tie rod, snubber/skew restraint, nonlinear state,
Timoshenko, independent support motion, pressure stiffening/elongation or
product mapping was established. W1's extracted clusters are single modes;
this examination supplies no new repeated-cluster proof or new general proof
of F1's omitted formation domains. Reference-side F1 reuse retains its stated
provenance; the C3/C9 replay is not another independent formation derivation.
The screening tolerance and its original no-access history were not re-set or
re-audited here. No product comparison tolerance follows from this record.

The reference owner/parent acts next: disposition the two wording findings,
repair additively without changing expectations to fit any implementation,
copy this report unchanged if retained, and regenerate the package manifest.
Any changed numerical premise or value requires a fresh refutation; a later
reader reviews any repair as required. Agent 0 owns integration. This report
binds only the candidate head and hashes above.

Raw logs, independently authored derivation and probes are retained for the
parent in `/tmp/pr371-refuter.qhQDSQ/`: `derivation.md`, `mathlib.py`,
`audit.py`, `audit.log`, `audit-results.json`, `control_compare.py`,
`control_compare.log`, `control-results.json`, `extra_probe.py`,
`extra_probe.log`, `generator-check.log`, `identity-before.json`,
`identity-after.json` and `final_identity.log`. Earlier failed scratch-probe
attempt logs are retained separately. No candidate file was repaired by this
reviewer.
