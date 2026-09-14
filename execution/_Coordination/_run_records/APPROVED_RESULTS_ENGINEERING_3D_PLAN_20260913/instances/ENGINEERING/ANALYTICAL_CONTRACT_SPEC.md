# Proposed implementation-ready analytical interfaces

Status: planning specification; no runtime/schema adoption. Consumes RETURN.md's exact case table and cited local refutations at HEAD `8f27fa3d8ec5e128e61fd3ac4076e74d7955f355`.

## Immediately following private pressure slice

New internal module `core/product_physics/src/pressure_exact.rs` and one module declaration in `lib.rs`; DEL-05-03 owns the implementation. No `Deserialize`/`Serialize`, public result IDs, public DTO, topology inference or runtime invocation.

| Private type/function | Required meaning |
|---|---|
| `ExactAnnulus::from_radii(ri_m,ro_m)` | Finite 0<ri<ro; returns inner/outer radii, bore/wall areas. Reject invalid/overflowed derived values. |
| `IsotropicENu::new(E_pa,nu)` | Finite E>0, -1<nu<0.5; returns authoritative pair and derived G. No G fallback or mismatch threshold. |
| `InternalDifferentialPressure(p_pa)` | Finite signed incremental p, external reference zero; not absolute/external pressure or capacity. |
| `lame_at_radius(annulus,p,r_m)` | Radius lies in closed wall interval; returns named radial/hoop stress. Uses boundary identities in RETURN.md. |
| `axial_state(annulus,material,p,thermal_strain,epsilon_z)` | Returns Nw in N, S=Nw-P in N, axial membrane stress Nw/As in Pa. No legacy pressure-longitudinal addition. |
| `eigenload_pair(annulus,material,p,thermal_strain)` | Stiffness-conjugate local i/j pair from epsilon0; no closure meaning. |
| `cap_pair(annulus,p)` | Mathematical local i/j pair only; no region graph or terminal classification. |

Tests use the exact synthetic cases in RETURN.md, dimensional scaling and sign reversal. Mutations must kill missing/duplicated terms. A fresh independent analytical checker freezes expected equations/numerics without importing this module before source effect. Implementation compares algebraic identities with representational tolerances expressed using machine precision and declared scales; it does not create engineering acceptability thresholds. Independent source review and project checks apply at publication. No new owner approval is required for routine release-gate PASS inside the approved bounded brief.

## Proposed UI backfill fields

These names are design drafts; a proposed-control state must remain distinguishable from supported live controls.

| Entity | Proposed fields consumed by interaction/property design |
|---|---|
| Exact-pressure material basis | `constitutive_basis`, `elastic_modulus`, `poisson_ratio`, temperature-point/bracket provenance; read-only derived `shear_modulus` |
| Pressure region | `id`, `member_pipe_ids`, `pressure_basis=internal_differential_zero_external_v1`, unit-aware `pressure`, `terminals[{node_ref,closure_transfer}]` |
| Pressure execution | Explicit `legacy_pressure_v1` versus proposed exact mode; missing-field/blocking diagnostics; supported-document-version declaration |
| Exact results | Distinct wall/effective axial force; axial membrane stress; inner/outer radial/hoop stress; region/material/frame/surface provenance and dimensions |
| Objective connector | `end_i`, `end_j`, initial frame references, unit-aware `attachment_offset_i/j`, `Q0`, `motion_basis=symmetric_midpoint_small_rotation_v1`, `q_ref`, installed reference temperature/state, units/provenance/measurement restraints for `Kc`, explicit topology target records |
| Capabilities | Editing/persistence allowed for incomplete data; solver activation enabled only for a supported complete contract; proposed versus live status explicit |

Preserve PR785 semantics: gestures propose the same typed operations and shared semantic equivalence as tree/property/agent routes. Preview, controlled apply, undo/redo and hash binding remain atomic; no hidden geometry or mechanics mutation. Finite joint insertion must first define end-plane nodes, replaced spans, load/support/provenance ownership and operation diff. Non-engineering navigation/selection design can start independently.

## Corrected connector analytic API for later slice

Given initial nodal positions, initial node-frame-transformed offsets `ai,aj`, attachment separation `r=pj0-pi0`, triad Q, nodal DOFs `[ui,theta_i,uj,theta_j]`, and `S(v)w=v×w`:

```
Bt=Q^T[-I, S(ai)+S(r)/2, I, -S(aj)+S(r)/2]
Br=Q^T[0,-I,0,I]
B=[Bt;Br]
Ke=B^T Kc B
r_internal_initial=-B^T Kc q_ref
f_internal(d)=Ke d+r_internal_initial
```

Constructor/validator returns normalized geometry, B, Ke and initial residual; it never accepts a topology token as graph construction. Product assembly subtracts the initial internal residual onto RHS. Stress-free rigid modes require Kc q_ref=0; prestressed cases instead test invariance under added rigid displacement. Units-normalized symmetric Kc is checked for PSD after declared length scaling. General manufacturer matrices require a documented transform into this coordinate/boundary-restraint basis.

Acceptance includes six rigid modes, offsets, transformed origins/frames, virtual work, finite-difference B and energy gradient, arbitrary-origin equilibrium, coupled PSD/null-mode cases, initial residual, canonical endpoint reversal and explicit topology/failure fixtures. No connector code is part of this next tranche. Pressure-joint coupling and finite rotations remain separately bounded.
