# M07 connector and pressure reference — independent refutation

Date: 2026-09-24. Status: bounded analytical return to HELPS_HUMANS; no production effect, manufacturer qualification, professional acceptance or release claim. TASK executed through the delegated harness and did not delegate. Parentage, actual supplied instruction/source origins and hashes are in `_run_records/JOINT_REFUTATION_ORIGINS.json`. Local source paths below are relative to the Piping project unless marked Root.

## Disposition

**PASS, within an explicit first-order reference-geometry model:** the symmetric midpoint deformation measure, its stated B, and the energy law with symmetric positive-semidefinite K satisfy all six infinitesimal rigid modes, virtual work, reference-position resultant equilibrium, coordinate rotation, end reversal and work-coordinate rescaling. The connector portion of the 2026-09-13 `ENGINEERING/verification/ANALYTICAL_ORACLES_V1.json` agrees with the independent derivation below. Those retained equations acquire support from the derivation, not from their prior hashes.

**REFUTED if asserted:** finite-rotation objectivity; arbitrary manufacturer's four rates as a diagonal midpoint matrix; centerline axial restraint as an interchangeable representation of offset tie rods; zero pipe force or zero anchor reaction whenever a joint is tied; and adding full bellows pressure thrust on top of globally owned actual pipe caps without cancelling the pipe-bore interface terms.

**QUALIFIED PASS for pressure composition:** full joint generalized pressure force `gp=[p Aeff,0,0,0,0,0]` with RHS `B^T gp` is consistent when the adjacent pipe segments carry their own complete pressure end-pair bookkeeping. In the equal-bore collinear assembly, the equivalent global-cap representation is actual terminal caps `p Ai` plus joint correction `p(Aeff−Ai)`. Face ownership and recovered wall/effective force definitions are necessary parts of that equivalence.

The selected constant-reference law is suitable for the present small-displacement/static undertaking. It supplies neither a geometric tangent nor pressure stiffness, stability/squirm, follower loading, finite-rotation or bellows fatigue qualification. A stiffness measured at operating pressure must identify whether it already contains the applicable pressure-dependent incremental response; no additional pressure stiffening may be inferred.

## 1. Independent kinematics and force derivation

Use global infinitesimal translations/rotations `d=[ui,θi,uj,θj]`. Let `ai,aj` be fixed global attachment offsets, `pi=xi+ai`, `pj=xj+aj`, `r=pj−pi`, and Q a proper orthonormal connector frame. Define `S(v)w=v×w`.

```
qt = Qᵀ[(uj + θj×aj) − (ui + θi×ai) − (θi+θj)×r/2]
qr = Qᵀ(θj−θi)
Bt = Qᵀ[−I, S(ai)+S(r)/2, I, −S(aj)+S(r)/2]
Br = Qᵀ[ 0, −I,             0,  I]
q = Bd; B = [Bt;Br]
g = K(q−qref)
Πelastic = (q−qref)ᵀK(q−qref)/2
fint = Bᵀg; Ke = BᵀKB
```

B follows by differentiating each cross product, with no solver invocation. For symmetric PSD K, Ke is symmetric PSD and `δΠelastic=gᵀδq=fintᵀδd`. No positivity of individual off-diagonal entries is required. B has row rank six: prescribe any relative rotation and then choose the end-j translation to obtain any qt. Thus positive-definite K leaves exactly the six rigid modes; semidefinite K can add genuine constitutive mechanisms. A nonzero null-coordinate qref is not observable in force/energy.

Let `F=Q gt`, `M=Q gr`. The four nodal internal blocks are exactly:

```
Fi = −F                         Fj = F
Mi = −(ai+r/2)×F − M            Mj = (aj−r/2)×F + M
```

This distinguishes generalized connector moment M from the actual endpoint moments. With `xj−xi=r+ai−aj`, any reference origin o gives

```
Fi+Fj = 0
(xi−o)×Fi + Mi + (xj−o)×Fj + Mj = 0.
```

These are reference-position equilibrium identities. Replacing xi by `xi+ui` and xj by `xj+uj` leaves an additional `ui×Fi+uj×Fj`, generally nonzero: exact equilibrium in the deformed geometry is not claimed. In particular, nonzero initial force does not generate its missing geometric stiffness through this energy law.

### Six rigid modes and their finite-rotation limit

For arbitrary t, infinitesimal ω and origin o, set `uk=t+ω×(xk−o)` and both `θk=ω`. The attachment relative displacement becomes `ω×r`; the midpoint term cancels it, while qr is zero. This covers three translations and three rotations, arbitrary offsets, arbitrary origin and any valid Q. Adding that displacement to a prestressed d preserves q and the existing energy/forces; it does not make a prestressed state have zero force. Stress-free zero-energy checks require `K qref=0`.

The stronger finite-motion claim has a direct counterexample. Take offsets zero, Q=I, `xi=0`, `xj=L ex`, and a true rigid rotation Rz(φ). Encode `ui=0`, `uj=L[(cosφ−1)ex+sinφ ey]`, `θi=θj=φ ez`. The candidate gives

```
qt = [L(cosφ−1), L(sinφ−φ), 0]; qr=0.
```

It is nonzero for finite φ: the leading axial error is `−Lφ²/2`. At L=2 m and φ=0.1 rad, qt is approximately `[-0.00999166944394836,−0.000333166706343702,0] m`. This is an expected nonzero counterexample, not a defect within the stated linearized envelope and not an adopted angle threshold. Even a large common rotation with small relative rotation exceeds the fixed-reference guarantee.

The legacy raw difference `qt=Qᵀ(uj−ui)` instead fails at first order. For L=2 m, ωz=.01 rad, `ujy=.02 m`, and `Kyy=20 N/m`, it creates `gy=.4 N` and `.004 J`; the midpoint law gives zero. Merely reducing the amplitude cannot repair that wrong rigid-mode scaling.

### Exact end-moment and measurement controls

At zero offsets, Q=I, r=L ex, impose pure transverse end displacement `ujy=δ` and zero rotations with diagonal K. Then `Fy=ky δ`, `Miz=Mjz=−L Fy/2`. These equal same-sign moments are required to balance the force couple. They must be recovered and applied at the endpoints.

For an end-j angular test `θjz=φ`, `θi=0`, and both translations fixed,

```
qty = −Lφ/2; qrz=φ
Fjy = −ky Lφ/2
Mjz = (krz + ky L²/4)φ
Miz = (ky L²/4 − krz)φ.
```

Thus the end-j moment/angle measured in that test is not simply krz. For L=2 m, ky=20 N/m, krz=60 N m/rad and φ=.01 rad, `Fjy=−.2 N`, `Miz=−.4 N m`, `Mjz=.8 N m`, and elastic energy `.004 J`. In contrast, the retained pure-qrz oracle also supplies `ujy=Lφ/2=.01 m`, cancelling qty, and correctly obtains generalized moment `.6 N m` and energy `.003 J`.

A useful constitutive cross-check is a symmetric Euler–Bernoulli beam reduced to this basis: `ky=12EI/L³`, `krz=EI/L` gives the familiar fixed-translation end-j coefficient `4EI/L`, not `EI/L`. This is an independent mathematical example of why measurement restraints matter; it does not make a bellows a uniform beam.

Four scalar rates can define a selected uncoupled, transversely isotropic six-coordinate law only after their test motions, restraints, angle convention, reference points, temperature/pressure state and basis transformations are known. In general a symmetric six-by-six matrix has 21 independent coefficients. A full matrix does not fix unknown test meaning either. This agrees with the vendor separation of motion definitions from elastic coefficients in [Abaqus Connector Elastic Behavior](https://docs.software.vt.edu/abaqusv2025/English/SIMACAEELMRefMap/simaelm-c-connelastbehav.htm). [Senior Flexonics Application Engineering](https://flexonics.com/application-engineering/) states rates in force/displacement or moment/angle terms, but supplies no authorization to identify every such catalog rate with this particular midpoint coordinate.

### Frame, reversal, normalization and reference controls

A passive proper global reorientation R, with translations of the reference origin, transforms `x,a,u,θ,Q` consistently; q, energy and local g are unchanged, while global nodal forces/moments transform by R. This exact coordinate covariance must be distinguished from the finite rigid displacement counterexample above.

For end reversal with `J=diag(−1,+1,−1)`, swap i/j and use `Q′=QJ`. Then `T=blockdiag(−J,−J)`, `q′=Tq`, `qref′=Tqref`, `K′=TKTᵀ`. The nodal force blocks are precisely permuted. Swapping node IDs while retaining anisotropic/coupled K or qref untransformed changes the physical law.

For `D=diag(Ls,Ls,Ls,1,1,1)`, `qhat=D⁻¹q`, `H=DᵀKD`, energy is `qhatᵀH qhat/2` in work units. Changing Ls while preserving physical K requires `H′=D′ᵀKD′`; physical qref stays unchanged. For the retained coupled block `H00=4,H03=1,H33=9`, Ls=2 m and `qtx=.2 m,qrx=.1 rad`, direct evaluation gives `.075 J`, `.25 N`, `1 N m`. Ls=1 m gives `H′00=1,H′03=.5,H′33=9` and identical results. At d=0 with that physical qref, g reverses sign, giving initial nodal force `[+.25,−.25] N` and x-moments `[+1,−1] N m`. Omitting this initial residual would fail the authored reference law.

## 2. Pressure work, face ownership and wall force

This derivation covers a static, collinear equal-bore chain with uniform internal pressure p, zero external pressure, no flow momentum and constant effective area. It uses authored Aeff as the first-order pressure-volume derivative of the joint. It does not derive Aeff from a proprietary bellows rule. Pressure loss, unequal bores, reducers, offset pressure-face centres or changing orientations need their own face-resolved work/geometry; one scalar difference-area correction cannot silently generalize to them.

Let the chain be `A — pipe L — B — joint — C — pipe R — D`. A and D are real closed end caps. The welded B and C interfaces are not additional physical blind caps. Let `Pi=p Ai`, `Pe=p Aeff`, and positive axial joint opening `q=uC−uB`. Equal pressure-face centres and the axial connector direction coincide in this fixture.

The axial end/face pressure work at the reference cross-sections is

```
δWp = Pi(δuB−δuA) + Pe(δuC−δuB) + Pi(δuD−δuC)
    = Pi(δuD−δuA) + (Pe−Pi)(δuC−δuB).
```

This is an axial load ledger, not a claim to include all pressure work on a deforming pipe shell. For `V=Ai L`, full volume variation also contains `L δAi`; the pipe's radial-pressure response is separately condensed into the `2νPi` wall-force term below. The authored `Pe δq` joint term follows its declared effective-area/generalized-motion definition and does not establish a general deforming-shell model.

The first line is the element-owned representation: left pipe end pair `[-Pi,+Pi]`, joint `[-Pe,+Pe]`, right pipe `[-Pi,+Pi]`. Assembly cancels only equal opposing cut contributions. The second is the globally owned representation: actual A/D caps `[-Pi,+Pi]` and B/C correction `[-(Pe−Pi),+(Pe−Pi)]`. Either representation gives the assembled pressure vector

```
[A,B,C,D] = [−Pi, Pi−Pe, Pe−Pi, Pi].
```

Do not combine their pieces twice. In particular, globally owned caps plus `[-Pe,+Pe]` at B/C add fictitious opening work `Pi δq`. Conversely, deleting adjacent pipe pairs without assigning the actual caps/face correction loses legitimate work. The ownership record must identify each region, face, area, orientation, load assembler and transferred or cancelled contribution.

The ordinary pipe's axial wall law, independently obtained by integrating the isotropic axial constitutive equation with the annulus transverse-stress trace, is

```
Nw = EA(ε−εT) + 2νPi.
```

Here A is metal wall area; Nw is actual tension-positive wall force. For a constant-radius isotropic annulus, `σr+σθ=2Pi/A`, so this relation follows from `E ε=σz−ν(σr+σθ)+E εT`. No unreviewed OCR equation is used. Define the separate effective force `Spipe=Nw−Pi`. For joint elastic tensile force Tb and explicit tie force Tt, define `Sjoint=Tb+Tt−Pe`. Welded-interface equilibrium then gives

```
Spipe = Sjoint = S
Nw = Tb + Tt − (Pe−Pi).
```

Nw, Tb, Tt, Pe and S are different quantities. Recover wall stress as Nw/A, once. A connector net/effective residual cannot be published as pipe wall stress or as total bellows material stress. The joint's gp is an external pressure load and must not be folded into the elastic g without distinguishing the result type.

With Qx parallel to r, `Bᵀgp` gives the full `[-Pe,+Pe]` attachment pressure pair and its offset nodal moments; the midpoint r-cross-force terms vanish. If used, this is the first-line element-owned representation above. A fixed gp is a dead generalized load in the selected reference geometry: it does not provide follower-load derivatives or pressure-dependent stiffness.

[Abaqus pressure loading documentation](https://docs.software.vt.edu/abaqusv2025/English/SIMACAEPRCRefMap/simaprc-c-loaddistributed.htm) explicitly includes element end loading, cancellation at adjacent equal sections, and compensation when an end condition differs. It supports the bookkeeping distinction, not a claim that Chirality matches an external solver.

The current [Witzenmann expansion-joint manual](https://media.witzenmann.com/mediapool/documents/brochures/expansion-joint-manual.pdf), printed pp. 574–579, separately discusses the bellows/pipe cross-section difference transmitted through the pipe and restrained-joint load transfer. Its pipe-connection examples preserve longitudinal pressure force for restrained connections. These are conceptual corroboration of the independent free bodies, not a universal zero-reaction boundary condition. Printed pp. 572–573 also distinguish small elastic spring rate from working rates affected by pressure and operating movement; the selected constant K needs its own parameter-state provenance.

## 3. Exact realistic assembly controls

Use two straight stubs each 2 m long, ri=.025 m, ro=.03 m, E=200 GPa, ν=.3. Put a .3 m long joint between B/C, axial elastic stiffness `kb=200000 N/m`, `Aeff=.004 m²`, `p=200000 Pa`, qref=0 and εT=0. Lateral guides suppress transverse rigid modes but exert no axial load. Values are authored analytical test parameters, not manufacturer's product data.

```
Ai = .000625π m²          A = .000275π m²
EA = 55000000π N          kp=EA/2 = 27500000π N/m
Pi = 125π N               Pe=800 N
h = 2νPi = 75π N          ΔP=Pe−Pi=800−125π N
```

### A. Untied, both caps axially anchored

Set `uA=uD=0`. Symmetry gives `uB=−q/2`, `uC=q/2`, `Nw=h−kp q/2`, `Tb=kb q`, and `Nw=Tb−ΔP`. Therefore

```
q = (h+ΔP)/(kb+kp/2)
  = (800−50π)/(200000+13750000π) m
  ≈ .0000148148919008057 m
Tb ≈ 2.96297838016114 N
Nw = Tb−800+125π ≈ −404.337939921115 N
S = Tb−800 ≈ −797.037021619839 N
support-on-model RA=−S, RD=S.
```

The pipe wall is slightly compressed, and the anchors react approximately the bellows thrust. Those signs follow the actual cap/face free body. It would be wrong to expect Nw=Pe or to remove wall force because a pressure load is present.

With a common pipe thermal strain εT, replace h by `h−EA εT`; hence `q=(h−EA εT+ΔP)/(kb+kp/2)`. Positive heating pushes the joint ends together in this doubly anchored assembly, reducing q. This sign control must accompany pressure/thermal superposition.

### B. Ideal axial-only tie, both caps axially anchored

Impose exactly q=0, using a legitimate constraint, not an invented giant stiffness. The constraint tensile force λ is the tie force:

```
Tb=0; Tt=λ=h+ΔP=800−50π ≈642.920367320510 N
Nw=h=75π ≈235.619449019234 N
S=−50π ≈−157.079632679490 N
RA=+50π; RD=−50π N.
```

The tie is not necessarily carrying Pe in a multiply restrained assembly: the stubs and anchors also share the load. Nonzero reactions arise from restraining the ordinary pressure-induced pipe extension. For εT≠0, `Nw=h−EA εT`, `Tt=h−EA εT+ΔP`, with q still zero. A bilateral ideal tie can carry signed λ; tension-only rods with slack/contact require the corresponding separate constitutive model.

### C. Untied, closed chain free to extend with one axial datum

Keep A as an axial datum and release the axial restraint at D; still retain the real end caps and transverse guidance. There is no net external axial force, so the datum reaction is zero. Free D gives `Nw=Pi`, `S=0`, and continuity gives `Tb=Pe`:

```
q=Pe/kb=.004 m
Tb=800 N; Nw=125π N; S=0
each pipe extension = 2(Pi−h)/(EA) = 1/550000 m
uD−uA = .004 + 1/275000 m ≈ .00400363636363636 m.
```

The realistic finite pipe stiffness and Poisson contribution are retained. A connector-only fixture would not demonstrate these wall/cap effects.

### D. Ideal axial-only tie, same free closed chain

Now impose q=0 while retaining free D. Then `Tb=0`, `Tt=800 N`, `Nw=125π N`, `S=0`, zero axial datum reaction, and total extension `1/275000 m`. This is the case in which full Pe is carried by the ideal tie. It must not be generalized to case B.

For a finite positive axial tie stiffness `kt=20000000 N/m` in the same free chain, `q=800/(kb+kt)≈.0000396039603960396 m`, `Tb≈7.92079207920792 N`, `Tt≈792.079207920792 N`, while Nw and S remain `125π` and zero. The load division tests the actual tie constitutive path.

### Required negative controls

1. **Double cap count:** global A/D caps plus full Pe at B/C produce `qwrong=(h+Pe)/(kb+kp/2)≈.0000238639044078429 m` in A and `qwrong=(Pe+Pi)/kb≈.00596349540849362 m` in C. In exact-tied B, q, Nw and anchor reactions remain unchanged, but `Ttwrong=Pe+h≈1035.61944901923 N`, exceeding the correct tie force by exactly Pi. Therefore displacements and global equilibrium alone are insufficient. D gives `Ttwrong=Pe+Pi`.
2. **Area-equality control:** set Aeff=Ai. The B/C correction must vanish exactly. The globally owned pressure vector then contains only the real terminal caps. A nonzero interior full-pressure pair would expose duplicate bore thrust.
3. **No-pressure control:** p=0 and εT=0 gives q=0 and all forces/reactions zero in A–D. Positive thermal strain in A must close the joint.
4. **Wall/effective confusion:** in free cases C/D, Nw=Pi while S=0. Subtracting cap force during wall recovery incorrectly reports zero axial wall stress.
5. **Topology retention:** replacing a pipe span must omit that span's stiffness and transfer/cancel its pressure faces deliberately. Keeping it in parallel changes the stiffness and pressure work; it is not an innocent discretization choice.

## 4. Exact ties versus explicit rods

The axial-only condition qx=0 is an expressly selected idealization. A parallel pin-ended rod with fixed initial unit direction n and attachment offsets bi,bj has first-order extension

```
e = n·[(uj+θj×bj)−(ui+θi×bi)].
```

For a rod's own collinear end-to-end r, subtracting mean rigid rotation along n has zero projection, so this extension has the correct infinitesimal rigid modes. Positive finite k gives `T=k(e−eref)` with endpoint forces and offset moments from virtual work. An ideal inextensible rod imposes e=eref as an exact constraint; its force is recovered as the constraint reaction. Do not replace a rigid constraint by an arbitrary penalty value.

For symmetric rods at offsets `±a ey`, initially along x, their extensions are `e+=qx−a qrz`, `e−=qx+a qrz` in the centred collinear fixture. Two ideal rods impose both qx=0 and qrz=0. Two equal elastic rods contribute `Kxx=2kt`, `Krzrz=2kt a²`, with cancelling cross terms. A further symmetric z pair also restrains relative ry. One offset rod instead couples axial and angular motion. These are mechanically different from a centerline axial-only constraint. Finite lateral movement, slack, nuts/stops, tension-only operation, pin articulation and pretension are additional authored properties; the word “tied” establishes none of them by itself.

## 5. Contemporary reference comparison and adoption limit

[Märtins et al., published 2024-06-05, volume 64 (2025), DOI 10.1007/s11044-024-09998-w](https://link.springer.com/article/10.1007/s11044-024-09998-w), develops a geometrically exact director-based coupling in a total-Lagrangian framework, with objective deformation measures and consistent nonlinear equations. Its sections 4.2 and 5 compare linearized response for small rotations and discuss limits of a linear elastic material model even with geometrically exact kinematics. This is a relevant modern alternative when large rotations or stability are required. It is not a drop-in validation of the midpoint law or of manufacturer bellows data, and its example angles do not create a universal acceptance threshold.

[Abaqus CARTESIAN](https://docs.software.vt.edu/abaqusv2025/English/SIMACAEELMRefMap/simaelm-c-connectiontypedesc-cartesian.htm) measures translation in the rotating first-end frame. Its small-rotation translation measure uses that end's rotation, rather than the mean rotation. Hence even an objective established connector may have a different constitutive basis and coupling. [Abaqus ROTATION](https://docs.software.vt.edu/abaqusv2025/English/SIMACAEELMRefMap/simaelm-c-connectiontypedesc-rotation.htm) uses a relative finite-rotation parameterization and explains limitations of that coordinate choice for constitutive behavior. These official current documentation pages reinforce the need to choose both kinematics and constitutive coordinates explicitly.

Retain the constant-reference midpoint law for the bounded first-order solver, clearly name that envelope, preserve measured/basis-transformed parameter provenance, and withhold finite-objectivity or stability claims. Select a geometrically exact/corotational system formulation if those capabilities become required; updating axes alone without consistent residual/tangent and rotation treatment would not establish them.

## 6. Verification performed and limits

The TASK independently derived the equations before any production source inspection; no production connector code was read or imported. It read the historical contract and connector oracle as candidate statements to refute. No product/tests/build/native/browser/endpoint/Git writes occurred. Public primary references were consulted on 2026-09-24; no licensed or OCR-extracted engineering formula was used as authority.

A Python standard-library rational calculation independently formed all 12 columns of B by evaluating the kinematics on coordinate basis vectors. With `xi=(1,2,3) m`, `ai=(1/5,2/5,−1/5) m`, `aj=(−1/10,3/10,1/2) m`, `r=(2,0,0) m`, `xj=xi+ai+r−aj`, Q=I and arbitrary origin `(7,−3,5) m`, all six rigid modes gave q=0 exactly. For `g=(2,−3,5,7,−11,13)` in the appropriate force/moment units, the internal blocks were

```
Fi=(−2,3,−5); Mi=(−42/5,87/5,−43/5)
Fj=(2,−3,5); Mj=(10,−9/2,157/10).
```

Force and arbitrary-origin moment sums were exactly zero. For `d_k=(k−4)/17`, k=0…11, with translation/rotation units understood, `q=(73/170,−26/85,29/34,6/17,6/17,6/17)` and both virtual-work expressions equalled `1567/170` in work units. The same independent command evaluated the exact case A–D expressions and finite-rotation counterexample; numerical values are reported above. All assertions completed and the final output returned. The original tool wrapper emitted only the command's output field, so raw subprocess exit metadata was not preserved; zero exit was inferred from normal completion, not separately observed in retained output. This corrects the earlier stronger exit-status wording. These are analytical arithmetic checks, not implementation tests, a manufactured-data claim, another solver comparison or an application solve. Future live-path checks must compare the actual selected candidate against these independently fixed expectations and preserve the negative controls.

The exact executed shell command is preserved in [`_run_records/ARITHMETIC_COMMAND.sh`](_run_records/ARITHMETIC_COMMAND.sh), with the returned output copied verbatim into [`_run_records/ARITHMETIC_STDOUT.txt`](_run_records/ARITHMETIC_STDOUT.txt). [`_run_records/ARITHMETIC_EXECUTION.json`](_run_records/ARITHMETIC_EXECUTION.json) records the invocation, hashes and exit-evidence limitation. These files recover the prior execution; the arithmetic was not rerun during preservation.

After this derivation, the TASK read the parent's sibling `CONTRACT.md`, specifically §§2–6. Its endpoint moment signs, finite-rotation counterexample, selected per-element pressure representation, J1/J2 numerical expectations, every J3 pressure/tie case and thermal/qref sign, J4 mutation, and offset-rod constraints agree. There is no blocking mathematical discrepancy in those sections. The sole prose note is repaired: the current wording explicitly cancels the common bore-cut contributions at a welded interface while preserving the difference-area residual of unequal full pipe/joint pairs.

The later §5 typed interface addition also passes this bounded mechanics check. Pressure, collective constraints and individual rods retain distinct authored meanings; the pressure domain supplies one magnitude, and every connector/face pressure contribution has one owner per case. Duplicate ownership and nonzero-pressure participation by an unpressurized connector reject. The text requires an explicit joint/interface graph extension, rejects using the old pipe-only region schema for this purpose, and creates no shared version or numerical-policy amendment. The final §2 clarification requiring distinct solver-node identities for every active joint is consistent with the independently parameterized two-node B and prevents global-DOF aliasing. The final §4 qualification of axial end/face work versus complete deforming-shell pressure work also passes: it retains radial/Poisson response separately and changes no equation or analytical case. Reverting that paragraph, removing the typed-interface subsection and reverting the node-identity sentence recovers the previously recorded contract hash exactly, confirming the bounded extent of subsequent edits. The final reviewed content identity and comparison are in the origins manifest; later substantive changes require affected backchecking.
