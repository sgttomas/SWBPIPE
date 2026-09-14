# Objective connector contract V1

Status: design candidate under the approved V3 engineering undertaking; no connector activation, finite insertion, manufacturer equivalence or new unit-policy adoption. Primary DEL-03-06 / SOW-010 / OBJ-004; primitive interface DEL-04-02 / SOW-006 / OBJ-003. Source basis8f27fa3d8ec5e128e61fd3ac4076e74d7955f355. Paths relative to WORKING_ROOT.

## 1. Separate namespace, versions and compatibility

A component may store `objective_connector` with `version:"1.0.0"`; common product document0.3.0 is the proposed minimum serialization container. This is independent of `model.pressure_contract`: objective connectors may activate with `legacy_pressure_v1` and legacy E/G materials, with **no E/nu prerequisite**. The component's connector stiffness defines its own constitutive response. Pressure/joint load composition is excluded: first connector activation refuses any pressure primitive/region targeting its replaced or adjacent joint span, rather than borrowing effective area or silently retaining a whole-pipe pressure action.

Old product0.1/0.2 documents retain original fields and original execution until a separately selected connector compatibility consequence is activated. Migration never creates objective fields, frames, reference state, Kc or graph intent from four scalar stiffnesses and `expansion_joint_pipe_ref`. Legacy data remain editable/reportable and unchanged-save bytes remain identical. Old History remains checksum-verifiable independently of any modern quantity adapter. New namespace in an old version blocks contract-version mismatch, never silently ignored.

Selected compatibility recommendation at future **connector activation**: retain legacy finite-span joint data but block solver consumption with `LEGACY_FINITE_CONNECTOR_REAUTHOR_REQUIRED`; explicitly authored objective records enable the new path. No silent matrix mapping. A coincident-node relative-DOF spring remains a mathematically valid special case, but the current full-pipe-span builder cannot construct it; do not infer a safe zero-length legacy interpretation. The retained public compatibility hold is the externally visible refusal versus preservation of old finite-span solves; this document proposes refusal, does not enact it. Pressure-only activation leaves this decision held and preserves its legacy connector behavior.

Known unsupported objective version1.0.0 gives `OBJECTIVE_CONNECTOR_NOT_IMPLEMENTED` until its own consumer/primitive release; unknown versions give `OBJECTIVE_CONNECTOR_VERSION_UNSUPPORTED`. Required-for-solve fields may be missing in editable saved documents; solver missing-data validation is explicit. No UI insertion button or automatic graph splitting is authorized by this contract.

## 2. Authored record and frame rules

```
ObjectiveConnectorV1 {
 version:"1.0.0",
 motion_basis:"symmetric_midpoint_small_rotation_v1",
 end_i:Attachment, end_j:Attachment,
 connector_axes_global:Matrix3,             // columns x,y,z; proper triad Q
 installed_reference_temperature:Quantity<temperature>,
 temperature_applicability:"fixed_installed_parameters_v1",
 reference_state:"stress_free"|"prestressed",
 q_ref:{translation:VectorQuantity<length>,rotation:VectorQuantity<angle>},
 stiffness:ScaledConnectorWorkMatrixV1,
 topology:ConnectorTopologyV1,
 provenance:{source_reference:nonempty string,
   measurement_restraints:nonempty string, basis_transform_reference:nonempty string,
   validity_statement:nonempty string}
}
Attachment {
 node_ref:Id,
 initial_node_axes_global:Matrix3,           // authored Qi/Qj, not implied by node coordinates
 offset_local:VectorQuantity<length>
}
```

All reference positions are current authored node positions, in the model's declared length unit before SI normalization. Nodal rotations are global infinitesimal rotation-vector DOFs. Initial node triads only transform authored offsets to global; they are not evolving orientations or new dynamic node DOFs. `ai=Qi offset_i`, `aj=Qj offset_j`; `pi0=xi+ai`, `pj0=xj+aj`, `r=pj0-pi0`. Separate connector Q expresses constitutive coordinates.

Validate finite positions/offsets/triads; `Q^TQ=I`, `detQ=+1`, and finite-length Q.x aligns with r/|r|. Matrix3 is nested row-major numeric storage **whose columns are axes**; a rotated-record example and tests prevent transposition ambiguity. For coincident attachments explicit Q remains required; no length-derived axis fallback. Use documented machine-precision-scaled representation guards for orthonormality/alignment, with separate position/length scale and dimensionless triad scale. These are arithmetic guards, not physical installation tolerances. Do not insert identity frames or infer manufacturer orientation.

`installed_reference_temperature` identifies the declared geometry/parameter state. V1 does not temperature-interpolate Kc or evolve q_ref; missing reference temperature blocks. Report selected case temperature when available and disclose `CONNECTOR_TEMPERATURE_LAW_NOT_PROVIDED` when it differs from the installed reference. With unavailable case temperature disclose that unavailability. The fixed-parameter law is explicit; software does not evaluate prose validity as a numeric fit criterion or claim temperature suitability. No numeric validity range is invented.

## 3. Generalized deformation and reference state

Let `d=[ui,theta_i,uj,theta_j]`, attachment motions `di=ui+theta_i×ai`, `dj=uj+theta_j×aj`, `theta_c=(theta_i+theta_j)/2`:

```
qt=Q^T[(dj-di)-theta_c×r]
qr=Q^T(theta_j-theta_i)
q=[qt;qr]
Bt=Q^T[-I,S(ai)+S(r)/2,I,-S(aj)+S(r)/2]  // S(v)w=v×w
Br=Q^T[0,-I,0,I]; B=[Bt;Br]
```

Midpoint convention is chosen for endpoint symmetry and exact first-order rigid-motion cancellation; weighted alternatives can also be objective but do not preserve this particular declared basis. Four manufacturer test scalars cannot select this basis without documented end restraints/reference-point transforms. General symmetric coupling does not cure unknown measurement meaning.

`q_ref` is the generalized deformation where elastic force vanishes, measured relative to **installed d=0 reference geometry**. It is not a nonzero installed stress-free motion. With symmetric PSD Kc:

```
Pi=1/2(q-q_ref)^T Kc(q-q_ref)
f_internal=B^T Kc(q-q_ref)
Ke=B^T Kc B
r_initial=-B^T Kc q_ref
RHS_prestrain=-r_initial
```

For `reference_state=stress_free`, require Kc q_ref=0; with positive-definite Kc this implies q_ref=0. PSD null coordinates may be nonzero but are explicitly nonunique constitutive reference components; preserve their authored values and report the null basis/scale rather than claim a unique physical stress-free state. For `prestressed`, allow finite q_ref, return r_initial, assemble its equivalent RHS and recover total internal force with it. Testing Ke alone is inadequate. Superposed infinitesimal rigid displacement preserves q, Pi and f in the fixed reference basis, including prestressed states; zero force/energy rigid-mode oracle is used only for stress-free reference.

No evolving-axis, follower load, corotational, geometric stiffness, finite/large relative rotation, plasticity, damping or pressure-thrust contribution enters this V1 primitive.

## 4. Coupled matrix units and admissibility

The current canonical unit catalog has linear N/m and rotational N*m/rad stiffness, but no energy DimensionId or N/rad coupling family. Sharing the spelling N*m with moment does **not** make a work coefficient a physical moment quantity. Therefore author an independently typed parameter record; do not serialize H as canonical QuantityResult, dimension=moment, or generic moment review rows.

```
ScaledConnectorWorkMatrixV1 {
 version:"1.0.0",
 representation:"scaled_work_coefficients_v1",
 translation_scale:{value:positive finite number,unit:"m"|"mm"|"in"|"ft"},
 rotation_scale:{value:1,unit:"rad"},         // fixed exact normalization
 coefficient_unit:"N*m",                    // fixed SI work coefficient serialization
 upper_triangle:[21 finite numbers],        // order(0,0)..(0,5),(1,1)..(5,5)
 coordinate_order:["tx","ty","tz","rx","ry","rz"],
 provenance:{source_reference,measurement_restraints,basis_transform_reference}
}
```

Let Ls be normalized translation_scale in metres and `D=diag(Ls,Ls,Ls,1rad,1rad,1rad)`. Numerically SI rotation coordinates are radians; `qhat=D^-1 q` is dimensionless, H coefficients have work units, and `Kc=D^-T H D^-1`. Then `Pi=1/2(qhat-qhat_ref)^T H(qhat-qhat_ref)`. This recovers block roles Ktt force/length, Ktr force/angle, Krt moment/length, Krr moment/angle with exact work conjugacy; store these roles in parameter evidence, not guessed canonical dimensions.

**Future implementation requires a scoped ConnectorParameterUnitContractV1 extension:** a raw parameter decoder/normalizer for H, coordinate scales and recovered Kc, independently typed from CanonicalQuantity/ResultDimension. It reuses already accepted length/angle converters and fixed-SI numeric work carriage; it does not add energy DimensionId or repurpose moment. Upstream non-SI/manufacturer matrices must be converted by an explicit authored basis-transform process before creating this fixed-SI record, with conversion source/checks preserved. V1 refuses unsupported coefficient_unit symbols rather than guessing conversions. Global energy/coupling unit-catalog promotion is a separate later decision, unnecessary for this fixed-SI primitive but not implicitly resolved here.

Construct full symmetric H from its upper triangle, so storage cannot encode contradictory lower entries. Validate finite H, positive Ls, exact rotation normalization, named order and scaled PSD. Negative off-diagonal terms are allowed. PSD test uses machine-relative factorization/eigenvalue guard on H, with declared reference norm; reject materially negative eigenvalues, report representation-scale negative estimates without projecting H, and preserve authored H bytes. No positivity-of-every-entry requirement. Do not normalize by an arbitrary model metre after accepting authored Ls.

Changing authoring Ls to Ls' without changing physical Kc requires `R=D' D^-1`, `H'=R^T H R` and `qhat_ref'=R^-1 qhat_ref`; canonical physical q_ref is unchanged. Verify virtual work/energy/forces under this coordinate rescaling. This is a parameterization change, not a physical stiffness edit.

## 5. Graph construction contract

```
ConnectorTopologyV1 =
 {type:"replaces_span",span_ref:Id}
|{type:"series_between_end_planes",end_i_role:"terminal"|"through",end_j_role:"terminal"|"through",
  adjacent_i_pipe_refs:Id[],adjacent_j_pipe_refs:Id[]}
|{type:"parallel_with_elements",element_refs:nonempty unique Id[]}
```

Both attachment node IDs are explicit. Replacement requires exactly one named analytical straight-pipe span with matching unordered endpoint pair; omit it from frame assembly and preserve its authored record as replaced-source provenance. Reject multiple replacement ownership, unknown/curved/endpoint-mismatched targets or two connectors replacing one span. Loads/supports on a replaced span do not silently move: first activation blocks distributed/thermal/self-weight/pressure loads and span-anchored semantic bindings requiring a transfer owner; explicit nodal loads/supports on retained endpoint nodes may remain. Removing a span must never drop an unapportioned load or leave a hidden dangling constraint.

Series joins two explicitly authored end-plane nodes; adjacent_i/j referenced pipes terminate at the corresponding nodes, with declared roles. Lists may be empty only when the corresponding end role is explicitly `terminal`; otherwise require at least one incident pipe on each nonterminal end and reject any direct pipe span still bridging the connector pair. No whole-pipe condensing or split-by-token inference. The graph must be authored before activation; finite insert-in-run additionally needs an atomic operation batch that owns newly created end nodes, adjacent pipe edits, replaced loads/supports and provenance. That UI/operation undertaking is not launched here.

Parallel requires every named parallel element to share exactly the attachment node pair (after canonical ordering); retain and assemble them plus connector. Reject omitted same-pair solver elements and unrelated references, so parallel superposition is explicit/exhaustive. Legacy full-pipe parallel behavior may be labelled `legacy_pipe_span_parallel_v0` in evidence; that label is computational history, not physical intent.

For replacement/parallel coincident-end connectors, ordinary zero-length frame spans remain invalid and cannot be inferred. Zero-length connector primitive is supported by explicit triads, but runtime graph role must be valid (typically a series link between distinct coincident nodes).

## 6. Reversal, outputs and consumers

Select canonical reversal Q'=QJ with `J=diag(-1,+1,-1)`; swap end attachments and initial node frames/offsets. `T=blockdiag(-J,-J)` gives `q'=Tq`, `q_ref'=Tq_ref`, `Kc'=T Kc T^T`. At unchanged scalar coordinate scales `H'=T H T^T`. General stiffness rescaling uses the rule above. Reversed force blocks equal original nodal blocks permuted; merely swapping IDs without transforming basis/reference/matrix is invalid.

Primitive returns normalized attachment geometry/Q, B, Ke, initial residual and total q/generalized force/energy for verification. Public runtime new mechanics envelope0.2.0 carries typed `contract_evidence.connector` with record version, end-node/frame/offset references, installed/reference/prestress state, normalization scales, authored H/source hashes, derived-block roles and assembled topology/replaced span IDs. H and energy remain typed parameter/verification disclosures, never physical moment rows or ratios. Canonical result0.3.0 may disclose this typed evidence losslessly by reference, pending its scoped disclosure contract; no future matrix export to v0.2.0 quantity tables is promised.

New physical kinds `connector_generalized_translation_v1` (m,length), `connector_generalized_rotation_v1` (rad,angle), `connector_generalized_force_v1` (N,force) and `connector_generalized_moment_v1` (N*m,moment) require explicit xyz component/basis/local-coordinate metadata and connector basis_ref. Node-on-element forces/moments use distinct `connector_endpoint_force_v1`/`connector_endpoint_moment_v1` with endpoint location/global axes, including offsets and initial residual. New IDs begin `result:connector-objective:<case>:<connector>:<location>:<component>`; legacy review IDs remain authored evidence. Result semantics table, TS/Rust binding, persistence dimension declaration and all report/export consumers must agree before activation, without editing immutable legacy dimensions/hash paths.

## 7. Verification and effect fence

ANALYTICAL_ORACLES_V1.json freezes stress-free/prestressed rigid modes, six components, coupled H energy/force, q_ref, scale/unit conjugacy, B/virtual-work derivatives, nonzero offsets, translated/rotated installation, arbitrary-origin equilibrium, reversal, PSD/null modes, all graphs and mutation kills. Fresh independent root-direct analytical refutation precedes implementation; no reviewer or implementation child executed in this manager. Public legacy refusal and activation remain specific retained consequences, not blanket engineering approvals. Pure dormant primitive requires its own bounded source brief, independently reviewed without changing UserStiffnessElement behavior. No pressure-v2 prerequisite, all-backend UI gate, industry-solver/manufacturer claim or external-solver prerequisite. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
