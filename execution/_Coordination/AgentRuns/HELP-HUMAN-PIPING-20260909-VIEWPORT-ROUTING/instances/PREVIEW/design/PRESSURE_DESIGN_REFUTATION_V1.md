# Independent pressure-design refutation V1

Status: `FROZEN`
Verdict: `CHANGES_REQUIRED`
Basis HEAD: `533332349a4607eee561d4ef90fb05a62d86519e`
Reviewed candidate SHA-256: `df940eb27fdbe091cabca9f9044c031367e412a629de831e65c4c298a8baaf15`

This is a pre-effect engineering review. It does not adopt the provisional recommendation, authorize source effect, change dependency state, or claim an industry-solver comparison has passed.

## Decision

The exact-annulus four-case mathematics and the stated tension-positive section-cut signs survive refutation. The proposed runtime contract does not. It permits an internally inconsistent `E/G/nu` material to be called homogeneous isotropic, attaches closure-transfer meaning to element pressure loads without owning pressure-region terminals, and names schema/adapter files that are not the live desktop or headless preview input route. Its legacy migration label would also recast the old approximation as a new closed-transfer model even though legacy documents have neither `nu` nor the topology needed to run that model.

One current P0 correctness defect is independent of that redesign and should proceed first: endpoint stress recovery must consume a section cut, while the published raw endpoint-action rows remain unchanged. The exact repair is frozen separately in `ENDPOINT_SECTION_CUT_REPAIR_BRIEF_V1.md`, with metadata and validation refinements in the two supplements.

## What survived mathematical refutation

For an annulus with inner/outer radii `ri, ro`, wall area `As=pi(ro^2-ri^2)`, internal/external pressure areas `Ai=pi ri^2`, `Ae=pi ro^2`, and signed pressure thrust `Pc=pi Ai-pe Ae`, the proposal's Lamé fields are consistent with the traction boundaries:

```text
A = Pc/As
B = (pi-pe) ri^2 ro^2/(ro^2-ri^2)
sigma_r     = A-B/r^2
sigma_hoop  = A+B/r^2
sigma_r(ri)=-pi; sigma_r(ro)=-pe
```

Since `sigma_r+sigma_hoop=2A`, homogeneous-isotropic generalized Hooke law gives

```text
Nw = E As (epsilon_z-alpha DeltaT) + 2 nu Pc
S  = Nw-Pc
```

where `Nw` is tensile wall force and `S` is effective axial force. The four candidate cases follow:

| Case | `Nw` | `S` | `epsilon_z` |
|---|---:|---:|---:|
| free, closures transfer thrust | `Pc` | `0` | `alpha DeltaT+(1-2nu)Pc/(EAs)` |
| restrained, closures transfer thrust | `2nu Pc-EAs alpha DeltaT` | `-(1-2nu)Pc-EAs alpha DeltaT` | `0` |
| free barrel, closure thrust separately supported | `0` | `-Pc` | `alpha DeltaT-2nu Pc/(EAs)` |
| thermal plus pressure | linear superposition of the preceding terms | same definition | same definition |

The independent script `pressure_refutation_calculations.py` reproduces all five numerical rows in the investigation, the exact inner/outer Lamé values, and the equal-area two-element cancellation. Its recorded output is `PRESSURE_CALCULATION_RESULTS_V1.json`.

The implementation signs need one sharper decomposition than the candidate states. With local axial ordering `[i,j]`, use

```text
epsilon0 = alpha DeltaT - 2 nu Pc/(EAs)
f_eigen  = [-EA epsilon0, +EA epsilon0]
         = [-EA alpha DeltaT+2nu Pc, +EA alpha DeltaT-2nu Pc]
f_cap    = [-Pc,+Pc]                    # only closure transfer to this wall region
q_wall   = Kd-f_eigen                   # never subtract f_cap from wall recovery
```

Then `q_wall=[-Nw,+Nw]`. A free closed pipe solves `Kd=f_eigen+f_cap` and recovers `Nw=Pc`; a free separately closed barrel solves `Kd=f_eigen` and recovers `Nw=0`. Keeping the constitutive eigenload and external cap-transfer load separate prevents the current effective-force-like correction from being relabelled as wall force.

The primary engineering basis is consistent with this result: the [MIT 22.314 L.4 notes](https://ocw.mit.edu/courses/22-314j-structural-mechanics-in-nuclear-power-technology-fall-2006/137e6469e37e9d347b7b3b69292da2f3_l4_2.pdf) give generalized isotropic Hooke law, `G=E/[2(1+nu)]`, the thick-cylinder solution, and closed-end axial equilibrium. The official [Abaqus pipe-pressure documentation](https://docs.software.vt.edu/abaqusv2025/English/SIMACAEPRCRefMap/simaprc-c-loaddistributed.htm) states that pipe pressure includes closed-end loads, adjacent pipe-element end loads cancel, and an open end is represented by compensation; the [Abaqus section-output documentation](https://docs.software.vt.edu/abaqusv2025/English/SIMACAEOUTRefMap/simaout-c-std-elementsectionvariables.htm) distinguishes ordinary section force from effective axial force. These sources were checked directly on 2026-09-09.

## Actionable findings

### P0 — Endpoint stress uses raw endpoint action signs and, for bends, the wrong frame

`core/product_physics/src/lib.rs:1570-1573` publishes `corrected_local_forces` as endpoint action rows, then `:1664-1673` sends those same components directly to endpoint stress recovery while station stress consumes section resultants. `straight_section_resultants` at `:6325-6353` establishes the product's j-side section-cut convention by negating the i-side action. For a uniform tensile state with raw actions `[-Nw,+Nw]`, current endpoint axial stresses are `[-Nw/As,+Nw/As]` while every station is `+Nw/As`. Torsion and both bending components have the same action-versus-cut defect.

For straight members, recover endpoint stresses from `straight_section_resultants(...,0)` and `(...,1)`. For curved members, do not merely flip six chord-frame components. Rotate the recovered j-end action to global as the existing curved station path does, then call `arc_section_resultants_with_radial_pressure` at fractions `0` and `1`; this supplies the actual endpoint tangent, inward radial, and bend-normal section frame and includes distributed-load equilibrium. `core/solver/curved_bend/src/lib.rs:345-358,437-448,557-589` defines those exact endpoint invariants.

Raw endpoint-action result IDs, values, kinds, and chord-frame metadata remain unchanged. Only stress recovery consumes the section-cut representation. This repair is pressure-model independent and should not wait for the new Poisson/topology contract.

The existing curved station producer also emits `coordinate_system=arc_section_frame` and `basis=arc_section_equilibrium_from_assembled_end_forces`, but `schemas/results.schema.yaml:600-637` admits neither value and `core/runner/headless/src/result_envelope_binding.rs:216-223` copies them verbatim. In the same producer file, align endpoint and existing station metadata to canonical `element_local` / `recovered_from_local_element_stiffness` and retain the exact arc frame/equilibrium description in `sign_convention`. This preserves numerical leaves and permits a truthful full affected-envelope schema validation. Adding schema enums solely for those descriptions is unnecessary.

### P1 — A diagnostic-only `E/G/nu` residual is not a homogeneous-isotropic contract

The proposed model calls the material homogeneous isotropic but keeps three independently authored elastic constants, using current `E` for axial/bending stiffness, current `G` for torsion, and new `nu` for pressure coupling. Those values are not independent: isotropy requires `G=E/[2(1+nu)]`. Publishing a residual while solving with an inconsistent triple produces an ad hoc decoupled material, not the selected model.

Select two authoritative constants. For the proposed new path, use explicit `(E,nu)` as the pair, derive `G` for stiffness, and identify this precedence in input and result provenance. Validate finite `E>0` and `-1<nu<0.5`. If a redundant user `G` is retained, either reject a material that disagrees beyond a documented representation tolerance or mark it non-authoritative and never use it in that solve. A warning while still consuming the mismatched `G` is insufficient.

Temperature selection must apply that same rule. Current code linearly interpolates `E`, `G`, and `alpha` independently (`core/product_physics/src/lib.rs:5771-5776,5992-6012`). Adding an independently interpolated `nu` does not preserve isotropy. The deterministic counterexample starts with two individually compatible points and produces a midpoint `G` residual of `-0.241758%`. For the new path, interpolate the authoritative `E` and `nu` over the same bracket, then derive `G`; keep the current independent `E/G` interpolation only in legacy mode.

### P1 — Legacy compatibility needs a real semantic mode, not inferred new topology

The proposed migration name `closed_transferred_legacy_inference` is misleading. Old models lack `nu`; the old solver omits Poisson pressure strain, subtracts both thermal and cap-equivalent axial pairs during straight recovery, suppresses the thin-wall longitudinal pressure component when thrust is active, and accepts any nonempty `schema_version`. That is a distinct algorithm, not the new closed-transfer case with missing data.

Bump the authored product-document version and define an explicit `legacy_pressure_v1` execution mode. Documents at the old supported version migrate by recording that legacy mode; do not write a physical closure assertion. Missing `nu` or region topology remains valid only in legacy mode. Newly authored exact-pressure records require the new constitutive and region fields and block when absent. Preserve the legacy load assembly, displacements, thin-wall pressure output, and generic-row identity; the independently authorized endpoint section-cut correction still fixes the erroneous endpoint stress representation.

The candidate fence omits the actual version authority. `apps/desktop/src-tauri/src/model_document_migration.rs:17-46` owns supported version `0.2.0` and the migration chain. `PreviewModel` itself only checks that the version string is nonempty (`core/product_physics/src/lib.rs:849-856`). Direct headless solve also deserializes `LinearStaticPreviewRequest` without running the desktop migration (`core/runner/headless/src/bin/openpipestress-runner.rs:732-740`). The new mode dispatch therefore needs both a document migration and an explicit product-physics rule for older direct/headless payloads.

### P1 — Pressure magnitude and wall-transfer topology are not yet owned at the right level

The current primitive record has one generic signed pressure scalar and an element target. It cannot distinguish internal from external absolute pressure, differential/gauge basis, or the physical terminals that transfer closure thrust. The first exact tranche should make a smaller explicit claim: signed internal pressure difference relative to a zero external reference, with `pe=0`. It yields incremental Lamé stresses with `sigma_r(ri)=-p` and `sigma_r(ro)=0`; general external pressure and absolute hydrostatic stress remain deferred. Do not silently assume this basis from the old scalar.

Closure transfer belongs to a connected pressure region and each physical terminal, not independently to each element. For the first straight-only region:

- require an explicit region ID, its connected straight-pipe members, uniform signed differential pressure, and two terminal node records;
- declare each terminal as `transfers_to_wall` or `separately_supported_or_compensated`;
- assemble `[-Pc,+Pc]` on each region member so equal-area internal actions cancel;
- cancel only the terminal action whose declared closure bypasses the pipe wall;
- always assemble the Poisson/thermal eigenload independently of the terminal choice;
- restrict the first tranche to equal bore area through the region, or require an explicit reducer/transition pressure-action owner. A residual `Pc_left-Pc_right` at an area change is physical and must not be discarded as failed cancellation.

This region contract also prevents incompatible topology fields from being summed when multiple pressure records target one element. Curved radial pressure wall loading is a separate term and must never be disabled merely because terminal cap thrust bypasses the barrel; curved extension remains outside the selected straight tranche.

There is also a current producer inconsistency to close: pressure-thrust assembly filters both `category=pressure` and `dimension=pressure` (`core/product_physics/src/lib.rs:6618-6647`), while stress recovery's `pressure_for_pipe` sums every matching `dimension=pressure` regardless of category (`:9311-9337`). One validated pressure-region selection must feed assembly, constitutive recovery, and pressure stress output.

### P1 — The proposed source fence does not reach the live inputs or every typed-row consumer

`schemas/model.schema.yaml` describes canonical model materials as a generic property map; it does not describe the `openpipestress.product_preview.model` DTO with `pipe_segments` and flat material fields. The physical-to-analytical transform copies canonical material properties generically, while `_solver_boundary_adapter.py` accepts only `E/G` and only force/moment/distributed-force load union members. More decisively, neither is in the live preview invocation:

- desktop Tauri deserializes the product preview model directly into the Rust `PreviewModel` (`apps/desktop/src-tauri/src/lib.rs:1449-1461`);
- headless deserializes `LinearStaticPreviewRequest` directly (`openpipestress-runner.rs:732-740`);
- the TypeScript preview material shape has no `nu` (`apps/desktop/src/types.ts:120-134`).

New mechanics result kinds also require more than `results.schema.yaml`. Headless export has an explicit `(kind,unit)` mapping and discloses rather than exports unmapped rows (`core/runner/headless/src/result_envelope_binding.rs:55-125,393-425`). Desktop analysis-run preparation and persisted analysis-run records each have closed source-kind dimension tables (`apps/desktop/src/services/previewService.ts:341-423`; `core/analysis_runs/records.py:461-528`). Omitting these consumers would make new wall/effective-force rows fail, disappear from the exported value set with diagnostics, or be impossible to persist.

Do not edit the canonical transform and adapter in the first runtime tranche unless the physical-to-analytical bridge is actually activated and its canonical `LoadRecord` union is extended. Carrying unused fields through a disconnected adapter is not runtime validation.

### P2 — New stress rows need exact component meaning and no legacy double count

For the selected long, straight, homogeneous model, report `Nw/As` as `axial_membrane_stress`, with bending kept as a separate contribution. Report Lamé radial and hoop stress at named inner and outer surfaces. `wall_average_axial_stress` is acceptable only if its metadata states that the ideal solution is uniform away from end discontinuities. Do not add the existing mean-radius `pressure_longitudinal` term to a stress already recovered from `Nw`; that would count closure pressure twice in the free-closed case.

Keep current thin-wall pressure rows only in `legacy_pressure_v1`. New exact rows need additive kinds/components and provenance that names signed differential pressure, exact `ri/ro/Ai/As`, material pair/temperature basis, pressure-region ID, and terminal-transfer basis.

## Smallest coherent implementation sequence

### Cut 0 — Immediate generic endpoint section-cut repair

This is the smallest production correction and may proceed while the broader pressure design remains held.

1. Edit only `core/product_physics/src/lib.rs` and its co-located tests, plus regenerate only fixtures proven to contain affected endpoint stress or curved metadata rows.
2. Recover straight endpoint stress from section resultants at fractions `0/1`.
3. Recover curved endpoint stress through the existing arc equilibrium helper at `0/1`, using actual arc endpoint frames and existing distributed/radial pressure inputs.
4. Preserve raw endpoint-action rows exactly.
5. Align existing curved station metadata to the schema's canonical enums and retain exact frame detail in `sign_convention`.
6. Test reversal for pure axial, torsion, bending-y, and bending-z; test combined pressure plus temperature; require endpoint/station parity and full affected-envelope schema validity.

The three frozen endpoint briefs in this directory are the detailed implementation contract.

### Cut 1 — Pure exact-pressure kernel, no public runtime claim

Freeze pure functions/types for annulus geometry, Lamé stress, `Nw/S`, `(E,nu)->G`, eigenload, cap-transfer load, and terminal assembly. Keep them private to `core/product_physics` (or one new internal module) and add focused tests using `pressure_refutation_calculations.py` as an independent oracle. If exact stress computation is placed in `core/loads/stress_recovery`, give it a new explicitly exact-annulus input/result type and leave the legacy mean-radius helper untouched.

This cut changes no serialized input, schema, result kind, fixture, or legacy execution. It proves signs and reductions before contract work.

### Cut 2 — Explicit product-preview pressure-v2 contract

After the region/material contract is frozen, the smallest honest runtime fence includes:

1. `core/product_physics/src/lib.rs` plus focused tests for DTO validation, assembly, recovery, mode dispatch, and typed production;
2. `core/loads/stress_recovery/src/lib.rs` only if the exact stress kernel lives there;
3. `apps/desktop/src-tauri/src/model_document_migration.rs` and its tests for the version bump and `legacy_pressure_v1` migration;
4. `apps/desktop/src/types.ts` for the actual product-preview material/pressure shape;
5. `apps/desktop/src-tauri/src/lib.rs` and headless runner tests that prove direct payload behavior;
6. `core/runner/headless/src/result_envelope_binding.rs`, `apps/desktop/src/services/previewService.ts`, and `core/analysis_runs/records.py` for every new result kind;
7. `schemas/results.schema.yaml` for additive component vocabulary while reusing canonical metadata enums;
8. only the focused input/output fixtures whose numerical or metadata leaves change.

`schemas/model.schema.yaml`, the physical-to-analytical transform, and `_solver_boundary_adapter.py` are a later bridge cut unless runtime invocation is added. If that bridge is activated, add a canonical pressure-region/load union and validate it end to end rather than copying orphan fields.

## Frozen validation requirements

The implementation review must require:

- dense/sparse parity for all four cases, mixed signs, `p=0`, `nu=0`, `DeltaT=0`, and signed negative differential pressure;
- `q_wall=Kd-f_eigen`, never `Kd-f_eigen-f_cap`, with mutations that omit or double each term;
- exact `Ai`, `As`, Lamé traction boundaries, `Nw/As`, and separate `S=Nw-Pc`;
- free and restrained support-vector equilibrium, plus member-orientation reversal;
- two equal-area collinear segments with zero internal cap-transfer action; an area-change case that blocks without an explicit transition owner;
- missing new `nu`, missing constitutive precedence, inconsistent redundant `G`, missing region terminals, disconnected region members, and incompatible duplicate pressure records all block only the v2 path;
- old documents and direct old-version headless payloads run `legacy_pressure_v1` without a `nu` default or a fabricated closure assertion;
- raw endpoint-action leaves remain unchanged by Cut 0, endpoint/station section stress agrees, and complete affected headless envelopes validate against `results.schema.yaml`;
- every new typed row survives headless export, desktop dimension declaration, and analysis-run persistence with exact provenance.

Future external validation should use two Abaqus references: an axisymmetric or solid thick-cylinder model for Lamé radial/hoop/axial stress and free/restrained strain, and a pipe-element chain for closed-end load cancellation, open-end compensation, reactions, section force, and effective axial force. Compare all four frozen cases, reversal, a two-element region, and one pressure-area transition with explicitly modeled transition thrust. Record element formulation, mesh convergence, sign/frame translation, tolerances chosen before comparison, and solver version. No such comparison has yet been performed.

## Residual limits

Even after these changes, the exact kernel remains a long-straight-cylinder membrane model away from end discontinuities. General external/absolute pressure, pressure gradients, bends, reducers, branches, expansion-joint effective area, ovalization, pressure stiffening, large displacement, plasticity, and design-code allowables remain outside this cut. Negative differential pressure is algebraically valid as an incremental linear load but does not establish buckling or collapse capacity.
