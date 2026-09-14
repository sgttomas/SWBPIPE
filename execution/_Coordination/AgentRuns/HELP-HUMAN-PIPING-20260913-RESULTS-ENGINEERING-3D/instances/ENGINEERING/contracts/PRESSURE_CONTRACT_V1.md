# Straight exact-pressure contract V1

Status: executable design candidate under the approved V3 design undertaking. This document selects normal engineering/software details for independent refutation; it activates no public physics, migration, acceptance threshold or source. Primary DEL-05-03 / SOW-015 / OBJ-003; material interface DEL-03-01 / SOW-017; document interface DEL-02-05 / SOW-050,SOW-041. Source basis `8f27fa3d8ec5e128e61fd3ac4076e74d7955f355`. All project paths are relative to WORKING_ROOT, resolved from the implementing checkout.

## 1. Selected representation and version ownership

Reserve product-preview model-document `0.3.0` for explicit namespaced mechanics contracts. Model `document_kind` remains `openpipestress.product_preview.model`; model semver remains the sole document-version authority. Pressure profile is **model-wide** so one assembled load case cannot mix independent legacy E/G and exact isotropic E/nu stiffness. `model.pressure_contract` is required at 0.3.0:

```
PressureContract =
  {version:"1.0.0", mode:"legacy_pressure_v1"}
| {version:"2.0.0", mode:"exact_straight_pressure_v2"}
```

Every exact-profile `PreviewLoadCase` stores `pressure_regions:PressureRegion[]`, including explicit `[]` for a zero-pressure case. Existing `primitive_loads`, `modulus_basis_ref` and `modulus_basis_temperature` retain their names. No top-level default or scalar pressure invents topology. Fields marked required are solve contracts; desktop authoring retains incomplete raw JSON. Rust missing-data DTO fields remain optional until the validator produces structured blocking findings, then constructs validated exact types; malformed/new namespace payloads are rejected with source-qualified parser diagnostics rather than coerced. Legacy cases prohibit `pressure_regions`; their primitive load shape and algorithm remain unchanged.

Supported direct solve versions become exactly 0.1.0, 0.2.0 and 0.3.0. Old versions with no new contract namespace select legacy computation at the Rust boundary without mutating input; old versions carrying v2/region/objective-connector namespaces block with `PREVIEW_CONTRACT_VERSION_MISMATCH`. Missing/invalid/unknown/newer semver blocks structured solve; desktop open continues its unsupported/read-only diagnostic policy. Do not continue today's any-nonempty-version behavior.

Desktop `model_document_migration.rs` owns the supported version and a named `0.2.0→0.3.0` migration `model-doc-0.2.0-to-0.3.0-explicit-legacy-pressure`. It adds **only** legacy profile and document version in memory; it neither infers nu nor closure/frame/topology. Existing 0.1→0.2 step is unchanged. Stored model/attachment bytes remain untouched until explicit save; save writes normal migration ledger evidence and the changed model hash invalidates Current results through PR785 rules. Original attachments remain Historical and verify through immutable legacy binding/serializer behavior. Old direct headless requests do not call desktop migration: their explicit version dispatch selects legacy mode with equivalent algorithm/provenance, without synthesizing a new document.

Namespaced objective connector record version1.0.0 is independently defined in CONNECTOR_CONTRACT_V1.md and is valid only in the common 0.3.0 container. Its absent namespace means legacy data, not an objective connector. A pressure-only implementation must reject an explicitly requested but unimplemented objective contract rather than ignore it. Connector activation is separate and can run with the legacy pressure profile; this common serialization container is not a pressure mechanics dependency. Before any source effect root decides precise activated feature subset and releases shared version/DTO ownership.

## 2. Exact material types and selection

The existing `Quantity {value:number,unit:string}` is reused. Every solve-consumed exact-profile material requires:

```
{
 id:Id,
 constitutive_basis:"homogeneous_isotropic_E_nu_v1",
 elastic_modulus:Quantity<stress>,
 poisson_ratio:Quantity<dimensionless,unit="1">,
 shear_modulus?:Quantity<stress>,        // redundant, nonauthoritative only
 thermal_expansion_coefficient?:Quantity<thermal_expansion_coefficient>,
 temperature_points?:[{
   id:Id, temperature?:Quantity<temperature>,
   elastic_modulus?:Quantity<stress>, poisson_ratio?:Quantity<dimensionless>,
   shear_modulus?:Quantity<stress>, thermal_expansion_coefficient?:Quantity<...>,
   provenance?:string
 }],
 provenance:nonempty string
}
```

Finite E>0 and -1<nu<0.5 define the isotropic model; reject nonrepresentable/overflowed derived G and derived geometry/load/stress outputs. Normalize units, select/interpolate E and nu on the **same** exact point/bracket, then derive `G=E/[2(1+nu)]` for all axial/bending/torsion stiffness. Retained G is disclosed as nonauthoritative; never consume it even when inconsistent. A redundant G warning does not repair a three-authority solve. No mismatch tolerance is needed with this precedence.

Use current precedence: nonempty `LinearStaticPreviewRequest.materials` completely replaces `model.materials`; an empty override uses model materials. Validate unique IDs and complete coverage against all used pipes in the selected list. Do not merge partial lists or take nu from another source. Exact-profile validation is applied before base model build and again after per-case temperature resolution, preventing an initial E/G-built matrix from surviving later isotropic selection.

When neither existing modulus selector is present use the material base pair. Exact-id selector requires explicit E/nu at that point; temperature selector requires bracket E/nu and temperatures. Both selectors together block. Preserve the existing strict-interior interpolation domain (at/outside endpoints block); exact-id selection remains available at endpoints. Never extrapolate/fall back to base pair. Alpha is required at the same selected base/point/bracket only when a genuine thermal load requires it; absent alpha otherwise remains absent rather than zero. Interpolate alpha on that bracket when required. Existing legacy E/G/alpha interpolation and missing-G behavior are unchanged.

Legacy material payloads have no fabricated constitutive basis or nu. Nu present in old uninterpreted data is never used by legacy computation. Reauthoring a model from legacy to exact is an explicit structured model edit requiring complete E/nu/regions, with changed hash and Current-result invalidation; it is not automatic migration.

## 3. Pressure region and boundary contract

```
PressureRegion {
 id:Id,                                    // unique across model
 member_pipe_ids:nonempty unique Id[],
 pressure_basis:"internal_differential_zero_external_v1",
 pressure:Quantity<pressure>,              // finite signed increment p
 terminals:[Terminal,Terminal],            // ordered region i→j
 provenance:nonempty string
}
Terminal {
 node_ref:Id,
 closure_transfer:"transfers_to_wall"|"separately_supported_or_compensated",
 provenance:nonempty string
}
```

Region geometry is reference geometry; pressure is uniform on the connected members. Region subgraph must be one acyclic collinear chain: ordered terminals are the two degree-one nodes and internal members meet degree-two nodes; no duplicated member or internal external-pipe branch. Traverse from terminal0 to terminal1 deterministically; individual pipe authored orientation may reverse independently. Members have identical normalized bore radius; wall area/E/alpha may differ. Reject unsupported noncollinear/curved/branched/reducer chains, malformed terminals, incompatible overlaps, duplicate region ownership of a pipe, and ambiguous pressure boundary. Each region terminal explicitly declares where the physical pressure domain ends; other mechanically connected members at a terminal do not become pressure members by inference.

For this first runtime cut all solve-active pipes are circular straight frame pipes, active supports are existing linear restraints/springs, and loads are existing explicit nodal/concentrated/distributed/thermal inputs. Nonlinear supports, active joint/bend macros and equivalent-static generators block only the exact profile as unsupported composition; legacy mode is unaffected. This is a bounded composition fence, not an all-project UI prerequisite. No curvature/joint pressure correction, pressure stiffening, ovalization, buckling or collapse is claimed.

An exact-profile case rejects pressure-dimensional primitive rows and pressure-category primitive rows (`EXACT_PRESSURE_REQUIRES_REGION`); pressure goes only through regions. Wind pressure is an equivalent-static generator input, not a primitive pressure region; that generator remains outside this first composition. This removes competing assembly/stress selectors. Zero pressure still requires explicit region geometry/terminals when a region is authored. Signed negative p is a linear incremental algebra case, not external pressure stability/capacity.

Geometric constraints use normalized SI values and a documented arithmetic representation guard, `64*epsilon_machine*reference_scale` for collinearity/normalized-radius equivalence; that guard addresses floating conversion, not a physical fit/applicability threshold. Preserve its scale and rejection diagnostic in verification; do not turn it into user geometry tolerance or engineering acceptance.

## 4. Constitutive force, applied loads and recovery

Let `ri,ro`, `Ai=πri²`, `As=π(ro²-ri²)`, `P=pAi`, `A=P/As`, `B=p ri²ro²/(ro²-ri²)`. At radius r:

```
σr=A-B/r²; σh=A+B/r²
σr(ri)=-p; σr(ro)=0; σr+σh=2P/As
Nw=EAs(εz-αΔT)+2nuP; S=Nw-P
ε0=αΔT-2nuP/(EAs)
f_eigen=[-EAsε0,+EAsε0]    // local i/j external equivalent RHS pair
f_cap=[-P,+P]              // local i/j mathematical member cap pair
```

Map cap pairs through the region traversal/orientation to global. Equal-bore internal cap actions cancel. Assemble only region terminal transfer that reaches the wall; a separately supported/compensated terminal cancels its mathematical cap action, leaving its externally supported closure load outside the pipe solve and explicitly disclosed. The physical wall still has pressure and Poisson eigenstrain; S remains Nw-P.

Assemble thermal/Poisson eigenloads separately from terminal cap transfer and existing mechanical load-vector terms. With mechanical fixed-end equivalent load `f_mech_equiv`, recover wall endpoint actions as `q_wall=Kd-f_mech_equiv-f_eigen`, **never** subtract f_cap. Tension-positive section Nw is `-q_wall[i,x]` at end i and `+q_wall[j,x]` at end j; existing straight section-equilibrium recovery supplies distributed-load variation at stations. Do not restart the already-landed endpoint-cut repair. Applied cap/eigenload evidence must remain separate from recovered wall resultants.

Axial membrane stress is Nw/As exactly once, independent of bending. Lamé radial/hoop stresses vary across the wall. Legacy mean-radius pressure-longitudinal output is absent from the exact profile, so closure stress cannot be double counted. Existing legacy load assembly/displacements/effective-force-like generic rows/thin-wall output remain legacy-mode witnesses; exact mode never silently reuses their axial IDs for wall meaning.

## 5. Source and public result contract

Exact-profile raw mechanics envelope reserves version0.2.0; legacy stays0.1.0. Add `contract_evidence` only to new envelopes:

```
{pressure:[{region_id,load_case_id,profile_version,member_pipe_ids,terminals,
  pressure_basis,p_pa,geometry:[{pipe_id,ri_m,ro_m,Ai_m2,As_m2}],
  materials:[{material_id,E_pa,nu,G_pa,constitutive_basis,temperature_basis}],
  approximation:"long_straight_annulus_small_strain_v2"}], connector:[]}
```

Evidence binds the actual normalized selected inputs; it contains no approval/fitness assertion. Each exact pressure row uses `basis_ref={ref_type:"pressure_region",ref_id:region.id}`. Source envelope/row hashes include evidence under existing qualified serializer conventions; never substitute request digest for model hash. The canonical derivative reserves result contract0.3.0 (after this tranche's result0.2.0 repair); adapter upgrades are separate from saved analysis-run schema migration. Add-only dimension mappings for new kinds must not alter legacy declaration/hash behavior.

| New kind | component / location | unit, dimension, family |
|---|---|---|
| `pipe_wall_endpoint_action_v2` | `wall_axial_end_action`, end_i/end_j | N, force, force; node-on-element vector sign |
| `pipe_wall_axial_force_v2` | `wall_axial_force`, end_i/end_j/quarter_1/midspan/quarter_3 | N, force, force; tension-positive section |
| `pipe_effective_axial_force_v2` | `effective_axial_force`, same stations | N, force, force; Nw-P |
| `pipe_axial_membrane_stress_v2` | `axial_membrane_stress`, same stations | Pa, stress, stress |
| `pipe_lame_radial_stress_v2` | `lame_inner_radial_stress` or `lame_outer_radial_stress`, same stations | Pa, stress, stress |
| `pipe_lame_hoop_stress_v2` | `lame_inner_hoop_stress` or `lame_outer_hoop_stress`, same stations | Pa, stress, stress |

IDs are new `result:pressure-exact:<case-suffix>:<pipe-suffix>:<location-token>:<component-token>` with collision-checked stable suffixes, never generic axial/legacy pressure IDs. Existing nonaxial physical force/bending/torsion result IDs may remain, with profile-qualified provenance. On pressure-region members the exact profile suppresses old generic axial endpoint/station force and axial-normal-stress/pressure-hoop/pressure-longitudinal rows, replacing each exhaustively with the new contract. Unpressurized members retain generic physical result IDs with p=0 wall recovery and exact-profile material provenance, without fabricating a pressure region; derivative row accounting records this mode-qualified replacement. Canonical coordinate systems use `element_local` for wall force/action and `pipe_section` for stresses. Basis uses existing `recovered_from_local_element_stiffness` for force/action, `recovered_from_open_mechanics_stress_components` for stresses; exact region/material/surface/frame details go into bound contract evidence and truthful sign_convention. No invented metadata enum strings.

First runtime implementation requires every direct DTO/normalizer/validator, desktop migration/type/property reader, headless invocation/result binder, new-row dimension/persistence consumer, derivative result schema/exporter and report/handoff unit mapper in CONSUMER_MAP_V1.json. Canonical `model.schema.yaml`, material/component libraries and physical-to-analytical adapter are **later bridge work** unless explicitly activated end to end; their generic property maps do not validate this live preview DTO.

## 6. Effect gates and completion

ANALYTICAL_ORACLES_V1.json specifies four exact reference cases, signs/conservation/reductions and mutations. Fresh independent analytical PASS/refutation is a root release gate inside the approved design undertaking, not a new routine user approval. Public model/migration/acceptance consequences retained by D66 remain explicit named holds before activation; this candidate does not adopt them. Pure dormant kernel implementation may follow accepted analytic specifications in its separately activated bounded source brief without implying runtime adoption. Consumer/test inventory and current source hashes are revalidated before that effect. No external-solver comparison prerequisite or new physical allowance is introduced. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
