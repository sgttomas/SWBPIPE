# Engineering next-tranche recommendation — frozen planning return

HELPS_HUMANS Type 1 → HELP_HUMAN `/root`; no delegation. Basis: `8f27fa3d8ec5e128e61fd3ac4076e74d7955f355`. Paths below are relative to `projects/chirality-piping` unless marked root. Model/effort are parent-configured `gpt-5.6-sol/high`, not independently runtime-attested.

Recommend **result-compatibility implementation plus pressure/connector contract and analytical-specification design, concurrent with professional 3D design**. Implement no public pressure-v2 or objective connector activation in this tranche. Freeze a private annulus-kernel implementation brief for the immediately following physics slice. ROOT reports RESULTS' concrete rotation/work compatibility defects; its frozen return owns that evidence. Engineering does not independently reproduce them here.

## Current truth and authority

The committed standing plan was loaded with `git show HEAD:projects/chirality-piping/loop/WORKPLAN_2026-07-18b_piping_loop.md`; its expected SHA-256 and the sealed engineering brief match. Receipt 139 discovery is parent-attributed, not independently checked. `CONTEXT_INVENTORY.json` records actual origins/hashes. Ten requested `Specification.md` paths are absent in this checkout; remaining context and register scope were read, without inventing specifications or repairing files.

`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-03_Fundamental stress recovery module/_STATUS.md` keeps pressure production/public adoption open. Its frozen `PHYSICS_UI_EXECUTION_20260908/INVESTIGATION_REPORT.md` and `SIGN_CONVENTION_CLARIFICATION_V2.md` establish candidate mathematical references with corrected node-on-element terminology. `POST_FAN_IN_REFINEMENTS.md` accepts their survival under independent refutation, **not runtime policy adoption**. D-66 ruling/addendum explicitly retain pressure meaning, public migration, acceptance thresholds, connector selection and formal dependency holds. Current chat authorizes this planning work; it creates no register ruling.

Live source removes two obsolete repair tasks: `core/product_physics/src/lib.rs` now has endpoint-section-cut/arc tests and `pressure_for_pipe` uses the same `genuine_pressure_element_target` as thrust assembly (`:6751`, `:9412`). Do not restart endpoint/category repairs. `MaterialInput` still authors E/G/alpha without nu (`:554` onward); primitive pressure still targets an element (`:535`). Bounded inspection of product-physics source and `core/loads/stress_recovery/src/lib.rs` found no exact Lamé/annulus pressure kernel or pressure-v2 mode. Current thin-wall benchmark is `validation/hand_calcs/stress/pressure_membrane.md`.

Connector defects remain current: `build_model` omits only curved-bend chord spans (`lib.rs:3406–3501`); joint stiffness is added alongside ordinary pipe stiffness. `build_expansion_joint_user_stiffness_elements` (`:3640`) selects the opposite endpoint of an entire incident pipe. `frame_kernel::user_stiffness_local_matrix` (`core/solver/frame_kernel/src/lib.rs:1018`) applies six raw displacement/rotation-difference springs. A finite rigid rotation therefore excites transverse springs. This is not finite insertion or a declared end-plane contract.

## Pressure contract to freeze now

Select exact circular-annulus, long-straight, homogeneous-isotropic small-strain mathematics as the recommended next physical model; keep the selection proposed until its retained owner hold is discharged. First runtime scope: connected collinear straight members, equal bore area, uniform signed internal differential pressure relative to zero external reference. Explicitly exclude branches, reducers, curved/joint pressure coupling and absolute/external-pressure claims.

Define `Ai=πri²`, `As=π(ro²-ri²)`, `P=pAi`, `A=P/As`, `B=p ri²ro²/(ro²-ri²)`. With tension positive:

```
σr=A-B/r²; σh=A+B/r²
Nw=EAs(εz-αΔT)+2νP; S=Nw-P
ε0=αΔT-2νP/(EAs)
f_eigen=[-EAs ε0,+EAs ε0]
f_cap=[-P,+P]; q_wall=Kd-f_eigen=[-Nw,+Nw]
```

Never subtract cap transfer from wall recovery. Publish `Nw/As` once as axial membrane stress, separate from bending; do not add legacy longitudinal pressure stress. Exact radial/hoop outputs name inner/outer surfaces. Preserve generic legacy result identity and existing raw action meanings.

Versioned inputs require material `constitutive_basis=homogeneous_isotropic_E_nu_v1`, E>0, -1<nu<0.5, paired temperature selection, and derived `G=E/[2(1+nu)]`. Interpolate E/nu on the same bracket, then derive G. Retain redundant G only as nonauthoritative provenance, never as torsional stiffness in this mode. This avoids introducing a material-mismatch acceptance threshold.

Pressure-region inputs identify connected members, pressure basis/magnitude/unit, physical terminal nodes and each terminal's `transfers_to_wall` or `separately_supported_or_compensated` load path. Assemble member cap pairs, cancel equal-area internal actions, compensate only explicitly bypassed terminal transfer. Eigenloads remain independent. Block disconnected/branched regions, unequal bore without a transition-action owner, incompatible overlaps, and missing solve inputs; allow incomplete editing/saving. A vented physical terminal cannot remain a uniform-pressure barrel by relabelling it.

Recommend a new supported product-document version and explicit `legacy_pressure_v1`: old desktop migration and direct old-version headless payloads select the actual old algorithm without fabricated nu/topology. New version spelling, supported-version table, coexistence and public row vocabulary remain concrete retained compatibility choices, to be presented together rather than individually re-escalated.

## Independent analytical specification

Freeze expectations before implementation; a later fresh analytical refuter imports no production module. Invented exact oracle: ri=1 m, ro=2 m, L=1 m, E=120 Pa, nu=1/4, p=3 Pa; `Ai=π`, `As=3π`, `P=3π N`, G=48 Pa. Lamé inner `(σr,σh)=(-3,5) Pa`, outer `(0,2) Pa`.

| Required case | Nw | S | εz | Additional acceptance |
|---|---:|---:|---:|---|
| Free closures transfer, ΔT=0 | 3π N | 0 | 1/240 | Zero axial pipe reaction |
| Axially restrained closures transfer | 3π/2 N | -3π/2 N | 0 | Support-on-vessel pair `[+3π/2,-3π/2]` N |
| Free barrel, separately supported closures | 0 | -3π N | -1/240 | Pipe reaction zero; remote closures each carry 3π N |
| Thermal + pressure, αΔT=1/1000 | Free:3π; restrained:57π/50 N | Free:0; restrained:-93π/50 N | Free:31/6000; restrained:0 | Restrained inward support magnitude 93π/50 N |

Add p=0, nu=0, ΔT=0, signed-load reversal, member reversal, two-member internal cancellation and rejected area-change/overlap fixtures. Verify dimensions, both terminal force vectors, conservation and endpoint/station section-stress parity. Mutations omit/double Poisson, eigenload, cap, or longitudinal stress terms. Arithmetic test tolerances are implementation precision controls; existing PKG-09 process owns any new engineering acceptance threshold.

## Connector contract to freeze independently

Use the independently refuted `symmetric_midpoint_small_rotation_v1` basis from `PROVISIONAL_OBJECTIVE_CONNECTOR_RECOMMENDATION_V1.md`, corrected by `instances/CREFUTE/design/CONNECTOR_DESIGN_REFUTATION_RETURN_V1.md` in the prior VIEWPORT-ROUTING run. Author two end nodes/initial frames, offsets, attachment positions, right-handed Q0 and installed reference geometry. Finite Q0.x follows the attachment separation; coincident ends require an explicit triad. Define `q=Bd`, constitutive zero-force `q_ref`, `Π=½(q-q_ref)ᵀKc(q-q_ref)`, `Ke=BᵀKcB`, initial residual `-BᵀKc q_ref`. Nonzero q_ref is prestress, not an installed stress-free state.

Require work-conjugate `[translation;rotation]`/`[force;moment]`, per-block stiffness units, provenance/measurement restraints, symmetric units-normalized Kc and PSD after declared coordinate scaling. Four legacy scalars cannot silently become a manufacturer-compatible matrix. Recommend preserving/editing/reporting legacy values while refusing finite-span activation until explicit v1 authoring; that externally visible compatibility change remains a retained decision.

Topology must name graphs: `replaces_span` validates one exact matching span and omits it; `series_between_end_planes` supplies two explicit end-plane nodes with adjacent pipes terminating there; `parallel_with_elements` names elements sharing the same endpoint pair. Pressure effective area/load path remains separate. Finite insert-in-run depends on this topology/reference contract and atomic structured-operation rules; it is not an independent geometry implementation.

Refutation: six rigid modes have zero q/force/energy only for stress-free reference (`Kc q_ref=0`); prestressed modes preserve energy/residual under added rigid motion. Check B finite differences, virtual work, six generalized components, coupled PSD matrix, initial-residual assembly, translated/rotated installations, offsets and equilibrium about arbitrary origins. Canonical endpoint reversal uses `D=diag(-1,1,-1)`, `T=blockdiag(-D,-D)`, transforms q_ref/Kc, and permutes nodal force blocks. Kill raw-difference mutation. No large-rotation or unspecified manufacturer coupling claim.

## Execution sequence and ownership

| Stage | Exact output / owner | Source ownership and true predecessor |
|---|---|---|
| Next tranche | RESULTS repair; parent-selected bounded integration owner | RESULTS' frozen fence; no pressure numerical changes. Engineering supplies proposed dimension vocabulary only. |
| Concurrent design | Pressure contract/oracles: DEL-05-03 with DEL-03-01/DEL-02-05 input/version owners; connector: DEL-03-06 with DEL-04-02 primitive owner | Evidence/design files only; independent refutation precedes effect. Root must scope any cross-package integration explicitly. |
| Concurrent 3D brief | DEL-07-09 vocabulary/palette; DEL-07-01 interactions; DEL-07-02 properties; DEL-16-01/02/03 atomic operation semantics | UI consumes proposed fields/capability states, not live pressure/joint controls. Navigation/selection/routing design needs no annulus kernel. |
| Immediately following physics slice | Private `core/product_physics/src/pressure_exact.rs`, co-located tests, one `lib.rs` module declaration; DEL-05-03 | Freeze pure geometry/material/constitutive/stress/load-pair types now. No serde, public rows, region graph builder or legacy dispatch. Kernel follows accepted analytical spec/refutation. |
| Later runtime cut | Pressure-v2 DTO/dispatch/assembly/recovery + all consumers | `lib.rs`, desktop types/Tauri migration, direct headless request tests, results schema, previewService, Rust result binding, analysis_runs persistence; canonical model/transform/adapter only if bridge activated. Connector activation separate. |

Defer the private kernel now: existing exact mathematics already survived independent calculation; unresolved cost is live DTO/version/topology semantics. A dormant kernel would add source review/sweep and shared-file serialization while resolving neither. The frozen API/oracles make the following physics slice executable without expanding this tranche.

Serialize every writer of `product_physics/src/lib.rs`, `frame_kernel/src/lib.rs`, results vocabulary or DTO/type files; one integrating owner per shared file. Independent analytical/design readers run concurrently. Pressure and connector contracts do not depend on each other, except future joint-pressure composition. Routine frame/sign/formula judgments and an analytic-gate PASS within an approved bounded plan are Agent0 release checks under DEC-087, not fresh owner approvals. D-66 model/public compatibility/threshold choices and formal DAG dispositions remain actual holds.

DAG-010 keeps DEL-05-03 upstream rows E0454–E0458 SATISFIED; DEL-03-06 E0418–E0420 and DEL-07-01 E0478–E0485 remain TBD. Design can expose current interfaces; implementation consumes only specifically released interfaces, never infers formal satisfaction. Existing absolute dense pivot guard and prior NS-03/04 scaling evidence justify a separately bounded numerics repair; no current nominal straight-pressure blocker or urgency requiring it before this contract/UI design was demonstrated. Do not create an all-backend-complete UI gate.

Closure: planning only, immutable derivative package on the stated source snapshot; no physical/public policy adopted, no implementation dispatched, no tests/builds/native/browser evidence produced. Rerun live currency checks before future effect; attach independent analytical/refutation returns and record remaining holds at that boundary. CAEPIPE/harness/bridge promotion, release and instruction changes remain deferred. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

Primary corroboration: [MIT L.4](https://ocw.mit.edu/courses/22-314j-structural-mechanics-in-nuclear-power-technology-fall-2006/137e6469e37e9d347b7b3b69292da2f3_l4_2.pdf) supports isotropic Hooke relations; [Abaqus elastic behavior](https://docs.software.vt.edu/abaqusv2025/English/SIMACAEELMRefMap/simaelm-c-connelastbehav.htm) supports local coupled elasticity; [connector behavior](https://docs.software.vt.edu/abaqusv2025/English/SIMACAEELMRefMap/simaelm-c-connectorbehavior.htm) distinguishes constitutive zero-force and initial references and explicit series/parallel graphs. Project equations above are the local frozen derivation, not extracted corpus/OCR authority.
