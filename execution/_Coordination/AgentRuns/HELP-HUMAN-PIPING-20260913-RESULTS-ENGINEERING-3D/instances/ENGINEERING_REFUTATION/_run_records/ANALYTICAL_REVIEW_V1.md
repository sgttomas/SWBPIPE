# Independent analytical and interface review

Review basis: composite CANDIDATE_MANIFEST_V2.json, 12 entries fully read and SHA/size matched. The review is a derivative analytical/design check on accepted source8f27 plus the user-approved V3 tranche. It is not authoritative decomposition, implementation, numerical qualification, public activation, engineering acceptance or professional reliance.

## Pressure derivation and independent expectations

Radial equilibrium for a long circular annulus gives d(sigma_r)/dr+(sigma_r-sigma_h)/r=0. The axisymmetric isotropic solution sigma_r=A-B/r², sigma_h=A+B/r² satisfies equilibrium. Imposing sigma_r(ri)=-p and sigma_r(ro)=0 yields A=p ri²/(ro²-ri²)=P/As and B=p ri²ro²/(ro²-ri²). Thus sigma_r+sigma_h=2P/As at every wall radius. The axial constitutive relation epsilon_z-alphaDeltaT=[sigma_z-nu(sigma_r+sigma_h)]/E yields Nw=EAs(epsilon_z-alphaDeltaT)+2nuP. The membrane force is wall force; S=Nw-P is a separate effective force. Neither is the mathematical cap load by definition.

Write epsilon0=alphaDeltaT-2nuP/(EAs). The positive external RHS eigenpair is [-EAs epsilon0,+EAs epsilon0]. Recovering node-on-element wall actions Kd-f_mech_equiv-f_eigen restores [-Nw,+Nw]. Cap loads are external terminal actions; subtracting them during this recovery would erase actual wall tension. Distributed mechanical loads retain their separate consistent/fixed-end correction, and the existing section-cut construction determines station variation.

At ri1,ro2,E120,nu1/4,p3, all force coefficients below multiply pi: Ai=1, As=3, P=3, EA=360, G=48, A=1,B=4. Inner/outer radial stresses are -3/0 Pa; hoop stresses5/2 Pa. Fresh rational checking reproduces:

| Case | epsilon_z | Nw/pi | S/pi | sigma_z Pa | support-on-vessel pair/pi |
|---|---:|---:|---:|---:|---|
| P1 free transferring closures | 1/240 | 3 | 0 | 1 | [0,0] |
| P2 restrained transferring closures | 0 | 3/2 | -3/2 | 1/2 | [3/2,-3/2] |
| P3 free separate closures | -1/240 | 0 | -3 | 0 | [0,0] on pipe; remote closure supports[3,-3] |
| P4 free, alphaDeltaT1/1000 | 31/6000 | 3 | 0 | 1 | [0,0] |
| P4 restrained, alphaDeltaT1/1000 | 0 | 57/50 | -93/50 | 19/50 | [93/50,-93/50] |

Pressure-only eigenpair is[3/2,-3/2]pi; P4 eigenpair[57/50,-57/50]pi. On each transferring vessel end, support+cap=wall endpoint action. On separately supported closures, remote support cancels closure pressure without applying cap load to the wall. A one-transfer/one-separate region has an unbalanced net pipe load until external pipe supports or connected mechanics balance it; free equilibrium must not be fabricated.

For a reference global chain0->1->2, each member's mathematical cap pair is[-P,+P]; interior +P-P=0. Reversing one authored member reverses its local axis and swaps its endpoints, giving the same global endpoint vectors and positive tension section force. Different wall areas/E change epsilon0 and stiffness, not the equal-bore P cancellation. This is a direct vector argument, not proof by absolute force magnitudes. P=0, nu=0, zero thermal strain and signed p/thermal increments reduce the linear relation as stated. New geometry/stability/follower effects are excluded by the candidate.

Mutation sensitivity is explicit: omitting Poisson contraction changes P1 free epsilon from1/240 to1/120; flipping its sign gives1/80; subtracting cap during wall recovery incorrectly gives Nw0 at P1; doubling cap yields Nw2P; counting longitudinal pressure twice yields a false second axial stress contribution; using mean-radius inner area makes Ai=9pi/4 rather than pi. For thermal interpolation, averaged E180 and nu7/24 give G2160/31; independent endpoint-G average69 is different. No production module supplied these expectations.

## Connector derivation, units and force balance

Define S(a)b=a cross b. Attachment motion ui+theta_i cross ai equals ui-S(ai)theta_i. Differentiating Q^T[(dj-di)-((theta_i+theta_j)/2) cross r] gives the stated translational B blocks Q^T[-I,S(ai)+S(r)/2,I,-S(aj)+S(r)/2]; rotation blocks are Q^T[0,-I,0,I]. The independent checker formed B from exact coordinate perturbations of the motion definition and compared it with the explicit block formula.

Rigid translation cancels dj-di. For rigid rotation uk=t+omega cross(xk-origin), di=t+omega cross(pi0-origin), so dj-di=omega cross r and theta_c=omega; q=0. This proof permits distinct attachment frames and arbitrary offsets/origin. Equal rotations also give qr0. Therefore all six rigid modes lie in null(B). With symmetric Kc, Ke=B^T Kc B is symmetric and PSD whenever Kc is PSD; virtual work g dot deltaq equals(B^Tg) dot deltad. Force and arbitrary-origin moment balance are the translation/rotation rigid-mode work identities. Exact rational tests include nonzero equal and unequal offsets, rotated/transformed installations and prestress.

At installed d0, initial internal residual=-B^T Kc q_ref. Its negative is the prestrain RHS. Stress-free labeling requires Kc q_ref0; nonzero components can exist only in constitutive null modes and do not establish a unique physical free reference. For prestress, adding rigid displacement leaves q, total internal force and energy unchanged but need not make them zero. The coupled example at Ls2 gives g_tx1/4 N,g_rx1 N*m and energy3/40 N*m; the installed residual force blocks are[+1/4,-1/4]N and moment blocks[+1,-1]N*m.

Canonical endpoint reversal Q'=QJ,Jdiag(-1,1,-1) gives Tblockdiag(-J,-J), q'=Tq and Kc'=TKcT^T. The exact energy and global nodal blocks are unchanged up to endpoint permutation only when q_ref and H transform too. Arbitrary proper rigid frame changes rotate reference geometry/triads and nodal vector DOFs together; local q and H remain the same, global forces/moments rotate. The checker verifies both transformations and the raw-difference mutation (false qy.02 m, energy.004 N*m for rigid omegaz.01).

The fixed-SI parameter contract is coherent. qhat=D^-1 q is dimensionless with explicit length/angle reference scales; H represents work coefficients, not physical moments. Kc=D^-T H D^-1 yields work-conjugate generalized forces/moments. At Ls2, H00=4,H03=1,H33=9 is PSD: its active quadratic form is4(x+y/4)²+35y²/4, with four declared null coordinates. Negative off-diagonal couplings can also be PSD; positive entries alone would not prove it. Ls2->1 uses Rdiag(1/2,1/2,1/2,1,1,1), giving H'00=1,H'03=1/2,H'33=9 with identical Kc and force/energy. The candidate's separate raw decoder, unsupported-unit refusal and preserved authored basis-conversion provenance avoid inventing an energy DimensionId. Normalization/null-factorization numerical algorithms remain future implementation checks, not executed here.

## Interface, topology and composition checks

The candidate concretely separates legacy direct versions0.1/0.2, common document0.3 and independently activated pressure/connector namespaces. Known but unimplemented connector requests block; connector-only activation can use legacy E/G without nu. Complete-replacement request.materials precedence is source-backed, including empty override fallback to model.materials. Exact base/point/bracket E/nu authority, derived G before each assembly, strict interior interpolation, no extrapolation, thermal-alpha dependency, raw incomplete edit/save behavior and explicit unknown-version diagnostics are consistent. Changing model hash invalidates Current results; old saved/attachment bytes and serializer/dimension checks remain version-qualified. These are design instructions, not an executed DTO or migration witness.

Pressure first activation restricts geometry, topology, primitive pressure selectors, supports/generators and new-region ownership. Lamé physical row IDs/units/stations and existing nonaxial outputs remain distinct from wall endpoint actions. V2 explicitly blocks combinations for new features, preserves p0 generic summaries for cases with no regions, makes global legacy summary unavailable if any exact regions exist, and prevents old rule IDs from being redirected. One uncovered component scalar-review consumer is ER-01 in RETURN.md; it prevents complete design PASS.

Connector replacement omits the named span and blocks unowned distributed/thermal/self-weight/pressure/semantic bindings; retained endpoint nodal loads/supports have an owner. Series requires explicit through/terminal incidence and rejects direct bridging pipes; parallel lists all same-pair solver elements. Coincident connector nodes require explicit triads and a valid graph without zero-length frame inference. Primitive objectivity does not itself solve graph ownership. The contract and future CutB acceptance express these graph rules; no runtime graph fixture has executed.

The consumer map names46 primary and4 supplemental source files. All50 source hashes match; the108 manager source-origin hashes also match. Full source semantics for all50/108 were not inspected. Targeted source reads independently checked DTO/interpolation, the summary producer, stress component recovery and the uncovered component/report route. Canonical model/material/physical-to-analytical bridges are honestly deferred rather than misidentified as live preview validators. Private pressure brief stays dormant; pressure runtime and connector CutA/CutB retain separate effects and scoped public consequences. Model0.3/result0.3 reservations are proposals, not current versions.

## Coverage limits

Executed: one leased pure stdlib Fraction checker,111 exact rational assertions, exit0; all frozen candidate SHA/byte bindings;50 referenced consumer hashes;108 manager-origin hashes. No floating-point production solver was used as an oracle.

Reasoned without executable production witnesses: general Lamé/axial derivation; vector internal cap cancellation and mixed closure balance; reductions and several mutation counterexamples; frame/offset/rigid virtual-work identities; PSD implications; version/material/DTO/topology/composition/consumer correspondence. The exact checker central differences are rational derivatives of a quadratic energy, not a numerical convergence study.

Unexecuted: malformed/unsupported DTO parser cases, region graph assembly, actual load/support fixture conservation, source mutation testing, production PSD/finiteness algorithms, temperature normalization through real unit libraries, Rust/TS parity, dense/sparse/native/browser/export/reopen/rule/report behavior, schema/serializer migration, DEC025/CI/publication and external solver/professional qualification. These are future implementation acceptance checks. No release or whole-project closure follows this review.
