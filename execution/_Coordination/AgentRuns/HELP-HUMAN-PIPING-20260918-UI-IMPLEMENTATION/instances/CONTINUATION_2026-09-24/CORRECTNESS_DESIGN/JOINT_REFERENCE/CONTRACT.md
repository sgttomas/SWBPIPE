# M07 — objective linear joint and pressure transfer

Implementation design, 2026-09-24, `/root/correctness_design` (HELPS_HUMANS). Current owner correctness/reference-quality steering governs. No product effect or shared numerical/version-contract amendment occurs here. `P/` means `projects/chirality-piping/`; `E/` means `P/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260913-RESULTS-ENGINEERING-3D/instances/ENGINEERING/`. Actual origins/hashes/parentage are in `_run_records/origins.json`. Read the independent return alongside this contract before freezing executable expectations.

## 1. Selected formulation and limits

Implement an explicitly authored **symmetric-midpoint, small-displacement/small-rotation, elastic connector**. Reuse the corrected kinematic/reference/matrix representation in `E/contracts/CONNECTOR_CONTRACT_V1.md`, with the pressure and tie additions below. Its prior preparation-only/dormant-only limits do not create a new permission question. The still-live six independent endpoint-difference springs are not this formulation: a finite span with lateral stiffness incorrectly resists common rigid rotation, and the product currently adds its ordinary pipe in parallel.

“Objective” here means exact cancellation of **infinitesimal** rigid modes and covariance under a rigid change of reference coordinates. It is not exact invariance under a finite rotation applied as deformation, nor exact angular equilibrium about deformed positions. Kinematics, moments, dead pressure directions and balance use the reference configuration. Prestress/pressure geometric stiffness, follower-load tangent, large rotation, bellows squirm/buckling, yielding, hysteresis, damping, fatigue and flow momentum are outside this first model. Small end rotation alone does not prove pressure geometric stiffness negligible in a very soft joint. Report the actual approximation and input applicability; do not manufacture a universal angle, stroke or pressure cutoff.

The selected elastic matrix is structural elastic response in the named reference configuration, with a declared pressure/temperature calibration basis. First activation accepts an explicitly declared constant elastic law excluding pressure follower/geometric effects, plus the explicit pressure load below. An operating-pressure tangent that already includes those effects cannot be silently substituted for it. Manufacturer test restraints, end/reference locations, units and directions must be known before mapping rates. Four legacy scalar rates do not supply that information. An explicitly authored uncoupled isotropic interpretation may map them to six diagonal entries; migration never makes that interpretation automatically.

The bounded mechanical primitive can operate at zero pressure independently of pipe E/nu. A connected pressurized pipe/joint composition requires the qualified wall/effective-force pressure formulation and explicit domain/face ownership. The earlier straight-only pressure profile does not silently gain joints through this design. Its composition extension, model container and result semantic registry are coordinated by ROOT through the already adopted shared contracts; this record allocates no new public version and changes no solver tolerance.

## 2. Exact kinematics, energy and end actions

Let node positions be `xi,xj`; normalized global offsets `ai,aj` come from the separately authored initial attachment frames. Attachment positions are `pi=xi+ai`, `pj=xj+aj`, `r=pj-pi`. Q is a proper orthogonal matrix whose **columns** are connector axes in global coordinates; for nonzero r its x-axis follows r. Every active joint requires distinct end-node identities. Coincident attachments still require explicit Q and those distinct solver nodes. DOFs are `d=[ui,theta_i,uj,theta_j]`, with translations in m and global infinitesimal rotations in rad. Put `S(v)w=v×w`.

```
vi = ui + theta_i × ai; vj = uj + theta_j × aj
theta_c = (theta_i + theta_j)/2
qt = Q^T[(vj-vi) - theta_c × r]
qr = Q^T(theta_j-theta_i); q=[qt;qr]=B d
Bt = Q^T[-I, S(ai)+S(r)/2, I, -S(aj)+S(r)/2]
Br = Q^T[0,-I,0,I]
```

For symmetric PSD work-conjugate K, elastic generalized action `g=K(q-qref)`, strain energy `U=(q-qref)^T K(q-qref)/2`, elastic endpoint action `f_el=B^T g`, stiffness `Ke=B^T K B`. At installed `d=0`, the initial residual is `-B^T K qref`; the assembly RHS receives `+B^T K qref`. Do not omit it. Stress-free installed state requires `K qref=0`; positive-definite K then requires qref=0. Prestressed state may have nonzero force/energy while rigid increments leave q unchanged. These are elastic actions, not pressure-augmented “wall stress.”

For global F=Q*g_translation and M=Q*g_rotation, endpoint blocks have the particularly useful independent form:

```
Fi = -F; Fj = +F
Mi = -(ai+r/2) × F - M
Mj = +(aj-r/2) × F + M
```

Thus `Fi+Fj=0` and `Mi+Mj+(xi-o)×Fi+(xj-o)×Fj=0` for any reference origin o. Virtual work is `f_el·delta_d = g·delta_q`. For `uk=t+omega×(xk-o)` and `theta_i=theta_j=omega`, q=0 exactly. For stress-free reference, energy/actions are zero; with preload, compare invariance of the baseline instead of demanding zero.

Canonical endpoint reversal uses `J=diag(-1,+1,-1)`, `Q'=QJ`, swapped attachments, `T=blockdiag(-J,-J)`, `q'=Tq`, `qref'=Tqref`, `K'=TKT^T`. Endpoint actions must be the old actions with node blocks exchanged. Coordinate-frame rotation is different: rotate all reference vectors/axes/displacements consistently, retaining local q/K and rotating global end actions.

Finite-rotation negative control: for no offsets, r=L ex, actual common finite rotation Rz(phi), use nodal displacement `(R-I)x` and nodal rotation vectors phi ez. The linear formula gives `qt=[L(cos(phi)-1), L(sin(phi)-phi),0]`, qr=0. It is intentionally not zero at finite phi. Do not “pass” finite objectivity by suppressing this output, or splice an evolving Q into constant B without deriving a consistent residual/tangent and changing the formulation identity.

Use the existing independently typed scaled work-matrix parameter record: D=diag(Ls,Ls,Ls,1,1,1), H=D^T K D, qhat=D^-1 q. H coefficients have work units; they are not physical moment result rows. Preserve authored Ls, 21-entry symmetric storage, frame/reference provenance and physical qref. Reparameterizing Ls requires congruent H transformation. PSD/null releases are constitutive input checks under the accepted numerical policy, not a new global positivity tolerance or permission to project negative input stiffness to zero.

## 3. Topology and owning loads

Keep `replaces_span`, `series_between_end_planes` and `parallel_with_elements` meanings from the corrected historical contract. A replacement removes exactly the named matching ordinary span from every assembly and recovery path; no pipe stiffness remains by accident. Series requires actual authored end-plane nodes/adjacent span incidence. Explicit parallel preserves exactly the listed parallel elements. This is graph intent, not geometry inferred from `expansion_joint_pipe_ref` or a label.

Removing a span cannot discard its distributed load, thermal load, mass/gravity, pressure assignment or station-bound support. Require explicit transfer/reauthoring for each owned contribution before solve; preserve original records/history. First mechanical-only slice may block unowned span loads rather than guess their distribution. Endpoint nodal loads retain their own nodes. Joint pressure below is a typed joint load, not a pressure primitive still targeting the removed pipe. Explicit joint mass/CG and static gravity allocation need their own honest load record; do not retain fake pipe self-weight to account for it.

Fresh finite-span legacy joint mechanics blocks with `LEGACY_FINITE_CONNECTOR_REAUTHOR_REQUIRED`. Missing solver-consumption fields cannot silently mean a working bellows superimposed on a pipe. An intentionally geometry-only annotation is allowed only as an explicit non-mechanical mode with visible meaning; incomplete physical joint definitions remain editable/saveable but unsolved. No raw old scalar matrix or topology is automatically remapped. Historical analyses remain immutable and readable under the adopted compatibility routes.

## 4. Pressure: virtual work first, then force bookkeeping

Use uniform internal pressure increment p, exterior increment zero, reference geometry, explicit manufacturer/user effective area Ae (m²), with provenance and definition of the generalized axial extension. Do not derive Ae from nominal bore or an unverified mean bellows diameter. For the selected coaxial joint, the constant generalized pressure load is `gp=[p Ae,0,0,0,0,0]`; assembly RHS contribution is `B^T gp`. It is conjugate to qt_x, not a second elastic force or stress. Pressure work increment is `p Ae delta(qt_x)`. Offsets carry the corresponding nodal moments through B. For the isolated boundary condition this load acts away from the two joint ends.

Connected pipes require a complete **wall/fluid** pressure ledger. For a constant-bore pipe segment with reference bore Ai and length change dell, the axial end/face pressure-work term is `p Ai delta(dell)`. This is not the entire deforming-shell volume work: `delta(Ai L)=Ai delta(L)+L delta(Ai)` also contains radial-wall work, whose response is separately condensed into the qualified pipe law's Poisson term. For adjacent pipe stubs plus a joint, sum the axial face contributions and the declared joint pressure-work term before simplifying, while retaining that separate radial/Poisson constitutive contribution. For equal bore and common p, write `Pi=p Ai`, `Pe=p Ae`, positive tensile bellows elastic resultant Tb and tie resultant Tt:

```
S_pipe = Nw - Pi
S_joint = Tb + Tt - Pe
S_pipe = S_joint at a load-free coaxial interface
Nw = Tb + Tt - p(Ae-Ai)
```

S is the effective combined material/fluid resultant; Nw is actual pipe wall force. Neither equals an anchor reaction without the full boundary free body. Two **equivalent** assembly representations are allowed, with one selected implementation owner: (A) every pipe mathematical cap pair `[-Pi,+Pi]` plus joint pair `[-Pe,+Pe]`; or (B) actual outer cap loads plus the residual internal joint-face pair `[-(Pe-Pi),+(Pe-Pi)]`. In (A), common bore cut terms cancel at welded interfaces, leaving the joint area-difference pair; unequal full Pi/Pe contributions do not vanish. In (B), that cancellation is already performed. **Never combine actual outer caps with another full Pe pair.** Also never call a welded pipe-to-joint interface a separately supported closure merely to make a region validator pass.

Select (A) for the connected assembly implementation because it derives directly from the per-element pressure work and supports an auditable contribution ledger; compare against independent (B) in references. Extend the pressure-domain boundary graph with explicit joint interface incidence and shared-pressure/equal-bore identity. Mathematical interior cap terms are not physical extra caps. Real terminals retain explicit transferring/separately supported closure dispositions. First composition supports coaxial equal-bore, common uniform p, straight adjacent members; different pressures/bores, branching, compensating chambers and flow momentum block pending their own volume/free-body contract. The pure joint pressure model has no invented Poisson coefficient; adjacent pipe Poisson eigenloads remain in their wall constitutive law.

Elastic recovery never subtracts pressure loads from `g=K(q-qref)`. Publish elastic, tie, pressure and effective axial quantities separately. The effective assembly endpoint action is `B^T(g + g_tie - gp)` for the ideal common generalized basis; actual tie attachments use their own map. Adjacent pipe wall force is recovered from its qualified pipe constitutive law, not inferred by adding another pressure stress to S. A pressure balance test must inspect individual material/hardware force paths, not only a zero net global force.

This rule preserves the welded-bend clarification: ordinary static pressure around a continuously welded bend has adjacent wall cuts as well as wetted-wall resultant; no independent unbalanced pA nodal force is added merely because the centerline turns. Joint Ae thrust represents its explicitly compliant/interrupted load path, not permission to add thrust everywhere. Tied hardware, remote closures, steady-flow momentum and follower effects remain distinct physical facts.

## 5. Ties and restraint hardware

Do not treat `tied=true` as either deleting pressure or making every joint DOF rigid. Support two explicitly named first-order ideals: an axial-only collective constraint, and individually located axial pin rods. The collective ideal `qt_x=0` is only an authored equivalent axial load-path assumption; it does not claim the actual angular/lateral freedoms of a manufacturer's tied/hinged/gimbal assembly.

For each actual rod with reference attachment positions bi,bj, length l>0, axis n=(bj-bi)/l and rigid nodal offsets, extension is `e=n·(v_bj-v_bi)=Crod d`. It vanishes under infinitesimal rigid motion. A finite positive supplied axial stiffness gives tension `Tr=krod(e-eref)` and residual `Crod^T Tr`; ideal inextensibility enforces `e=eref` with a recovered constraint multiplier. Use exact constraint elimination/rank checking or the governing approved constraint solver, never an invented huge stiffness. This is an interface to the existing numerical policy, not a new saddle-matrix SPD rule. Constraint and elastic force accounting remain separate.

Rod response is declared bilateral linear or a specifically supported unilateral law. Tension-only slackening, stops/limit-rod gaps, friction and nonlinear nuts/hardware are not secretly represented as bilateral rods. Two ideal rods at equal offsets ±a ey yield `e_plus=qt_x-a qr_z`, `e_minus=qt_x+a qr_z`; together they constrain both axial extension and relative z rotation. Four symmetrically placed rods can constrain both relative bending rotations. Their first-order lateral freedom does not imply finite-motion freedom: real rod-length change has second-order lateral terms. This is why an axial-only ideal cannot stand in for all tied hardware.

Under free closed-end pressure and an ideal collective axial tie, Tb=0, Tt=Pe, S=0 and pipe Nw=Pi. Under anchored end-to-end compatibility, those forces change; see the exact cases below. Never simply force tie load to Pe in every support configuration. A pressure-balanced joint requires actual compensating chamber areas and linkage, not a zero-pressure switch.

### Typed additions for the implementing interface owner

Retain the existing `ObjectiveConnectorV1` attachment/Q/qref/H/reference/topology fields. The following tagged types are the bounded proposed additions; their eventual public registration is ROOT-owned, not activated here:

```
JointPressureModel =
  {kind:"unpressurized"}
| {kind:"constant_effective_area_reference_v1",
   effective_area:Quantity<area>,
   pressure_basis:"internal_increment_zero_external_v1",
   source_reference:NonemptyString}

JointHardware =
  {kind:"untied"}
| {kind:"collective_centerline_axial_constraint_v1",
   prescribed_extension:Quantity<length>, source_reference:NonemptyString}
| {kind:"explicit_axial_rods_v1", rods:NonemptyArray<{
   id:Id, end_i:Attachment, end_j:Attachment,
   law:{kind:"bilateral_linear", stiffness:Quantity<linear_stiffness>,
        reference_extension:Quantity<length>}
      |{kind:"ideal_inextensible", prescribed_extension:Quantity<length>},
   source_reference:NonemptyString}>}

JointPressureParticipation = {
  connector_ref:Id, pressure_domain_ref:Id,
  end_i_interface_ref:Id, end_j_interface_ref:Id
}
```

The case's pressure domain is the single pressure-value authority; participation does not copy another pressure magnitude to sum accidentally. Each interface binds the actual adjacent member, pressure face, node, oriented area and cancellation owner. Each connector/face pressure contribution has exactly one owner per case; duplicate participation or competing load owners reject rather than sum. `unpressurized` mode rejects any nonzero-pressure participation. This needs an explicit pressure-domain composition extension owned with M01; the old pipe-only region/terminal schema cannot express it and must reject it until implemented. A connector is not secretly added to `member_pipe_ids`, and its welded interfaces are not classified as physical closures. Pure unpressurized connectors need no such extension. Ae is finite positive and normalized; first composition requires same pressure and bore on both sides. Zero pressure still preserves authored domain/topology identity.

Require an explicit stiffness calibration descriptor identifying constant structural elasticity, installed temperature/geometry and whether the supplied rates include pressure-dependent tangent effects; first activation rejects incompatible tangent interpretations. Keep applicability limits/source conditions as supplied, without invented ratings. Reject duplicate rods, zero rod length, missing offsets/axes, contradictory hardware and unsupported tension-only/limit-rod laws. Absent hardware is not implicitly untied. Proposed diagnostics are `JOINT_PRESSURE_INTERFACE_UNRESOLVED`, `JOINT_PRESSURE_AREA_MISSING`, `JOINT_STIFFNESS_BASIS_UNSUPPORTED`, `JOINT_HARDWARE_NOT_DEFINED`, `JOINT_HARDWARE_LAW_UNSUPPORTED` and `JOINT_REPLACED_SPAN_LOAD_UNOWNED`, each with exact component/member/source IDs. Validators preserve incomplete editing while preventing silent fallback at solve.

## 6. Independent analytical cases to freeze before product changes

All dimensions/rates here are synthetic SI controls, not catalog data or manufacturer validation. Obtain executable expectations from the independent derivation, not the product. Use the historical `E/verification/ANALYTICAL_ORACLES_V1.json` connector section for the existing six-mode, coupled H, offset, reversal and preload cases, retaining historical provenance; independently refute them before reliance. Add these connected controls.

**J1, finite lateral motion and end moments.** Let r=0.3 ex m, Q=I, offsets0, stress-free reference, diagonal K=[200000,80000,120000,600,900,1200] in N/m and N*m/rad. Set uj_y=0.001 m, all other DOFs0. Then q_y=0.001 m, g_y=80 N, U=0.04 J; elastic node actions Fi_y=-80, Fj_y=+80 N and Mi_z=Mj_z=-12 N*m. Leaving out those moments fails angular equilibrium. For common omega_z=0.01 rad with uj_y=0.003 m and both theta_z=0.01, q/U/actions0; raw-difference mutation gives q_y=.003 m, 240 N and .36 J. All three rotations plus translations, skew axes, offsets and preload need analogous controls.

**J2, actual rotational freedom.** Same r and K. Set theta_j_z=0.01 rad, theta_i=0, uj_y=0.0015 m, otherDOFs0. Then qt=0, qr_z=.01, g_mz=12 N*m, U=.06 J. Holding uj_y=0 instead produces qt_y=-.0015 m, g_y=-120 N, g_mz=12 N*m. Different test-end restraints therefore yield different apparent angular rates; they cannot be converted from a scalar label alone.

**J3, pressure assembly.** Use two identical 2 m straight stubs, ri=.025 m, ro=.03 m, E=200 GPa, nu=.3; central joint length .3 m, k=200000 N/m, Ae=.004 m²; p=.2 MPa, no thermal strain, qref0. All non-axial modes are restrained for this scalar reference, independently of the full six-DOF cases. Ai=.000625π m², As=.000275π m², EA=55000000π N, Pi=125π N, Pe=800 N, each stub axial stiffness kp=27500000π N/m. Let q be joint extension, u_left_joint=-q/2 and u_right_joint=+q/2 for symmetric anchored ends.

| Configuration | Exact joint/hardware result | Pipe and support result |
|---|---|---|
| Anchored outer ends, untied | `q=(800-50π)/(200000+13750000π)` m; Tb=200000q N | S=Tb-800 N; Nw=S+125π N; support-on-pipe left=-S, right=S |
| Anchored outer ends, ideal axial tie | q=0; Tb=0; Tt=800-50π N | Nw=75π N; S=-50π N; support left=+50π, right=-50π N |
| Free closed outer ends with one axial datum, untied | q=.004 m; Tb=800 N | Nw=125π N; S=0; each stub extension=1/550000 m; total outer-end separation change=.004+1/275000 m; axial supports0 |
| Free closed ends, ideal axial tie | q=0; Tb=0; Tt=800 N | Same pipe force/strain as preceding row; total separation change=1/275000 m; supports0 |
| Free closed ends, finite axial rod krod=20000000 N/m | q=800/20200000 m; Tb=200000q; Tt=20000000q | Nw=125π N; S=0; same pipe extensions; total separation change=q+1/275000 m |

For equal uniform stub thermal strain eT and joint qref, the untied anchored reference becomes `(k+kp/2)q=Pe-(1-2nu)Pi-EA eT+k qref`. Thermal elongation of anchored stubs therefore compresses the joint. A separately specified joint thermal reference law is required before assigning eT to the bellows; the removed pipe's alpha cannot supply it. Check p=0, qref=0, nu=0, signs, fixed prescribed joint movement, soft/stiff limits, arbitrary whole-model coordinate rotation and unit conversions. A movement outside a declared source travel/pressure/temperature range is diagnosed, not clipped.

**J4, double-count mutation.** Assemble global caps plus full Pe instead of Pe-Pi. For anchored ideal tie, q, Nw and anchor reactions can still match the correct row, but Tt is wrong: `800+75π` N instead of `800-50π` N. Thus global reaction balance alone cannot detect this defect. In the free tied case Tt incorrectly becomes Pe+Pi; in free untied q incorrectly becomes `(Pe+Pi)/k`. The negative control must inspect hardware action and pressure work/source ownership.

Further controls: remove exactly the replaced pipe stiffness and all its solver contributions; stiff retained-span mutation must alter axial/lateral response; negative/indefinite H, unknown basis and duplicate replacement ownership block; zero-length explicit Q works while missing Q blocks; unsupported ties/temperature law/pressure interface cannot fall back to old mechanics. Direct assembly and independent cap-cancelled free-body formulations must agree.

## 7. Implementation slices and completion

**J-A:** reference freeze and pure connector primitive. Own `core/solver/frame_kernel` (or an explicitly factored solver connector module) with independently tested B/energy/end-action APIs and normalization, preserving historical `UserStiffnessElement` for historical witnesses only. No production substitution before reference qualification. Use accepted numerical input/stability machinery; do not adopt the superseded mandatory comparison-inverse SPD proposal.

**J-B:** live mechanical topology and authoring. One product owner coordinates `core/product_physics/src/lib.rs`/`validation.rs`: `build_expansion_joint_user_stiffness_elements`, `build_model`, frame/sparse assembly, local recovery and diagnostics; `apps/desktop/src/types.ts`, component intent/typed operation validation, end-plane/topology/frame/reference controls and model migration. Backend-only explicit imported data may precede insertion UX, but M07 remains open until the authorized live path faithfully authors/applies/saves/solves the new mechanics. Shared DTO/model versions remain ROOT-owned. Preserve old data; new solves cannot silently use the known finite-span bug.

**J-C:** pressure-domain join and ties. Replace `expansion_joint_pressure_thrust_inputs_by_pipe`/`build_pressure_thrust_loads` implicit area substitution with explicit joint/domain load records and ownership ledger. Tie rod maps/constraint actions join assembly and recovery. Qualify mechanical-only, isolated pressure, connected stubs, tied and untied cases before enabling each supported composition. Native sync/background/headless routes share validation; malformed/missing modes and composition errors block deterministically.

Publish named local generalized translations/rotations and elastic forces/moments; global endpoint elastic actions; individual tie actions/constraint multipliers; pressure area/load/source contributions; effective axial action where its definition applies. Include topology/replaced-span, frame, reference temperature/qref, matrix scale and provenance. Work matrices/energy stay typed parameter/evidence records, not moment stress/code rows. Bind new semantics through ROOT's adopted raw/derivative/analysis/stress-neutral families and exact active semantic registry; no ad hoc legacy reinterpretation.

Completion evidence must bind candidate/model/case IDs through author→review/apply→solve→result inspection→undo/redo/invalidation→save/reopen→report/export. Verify actual native-host results and core/headless parity; browser frozen mechanics fixtures prove neither. Retain source-faithful inputs, actual observations, independent expected values, all failed/negative controls and actual-candidate review/checks. No execution of these checks is claimed by this design. Unsupported finite-rotation/hardware/pressure compositions remain explicit M07 successor obligations, not closed by a warning or a bibliography.
