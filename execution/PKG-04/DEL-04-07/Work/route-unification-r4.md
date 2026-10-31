# R-4: route unification. The exact pressure route admits every family the pressure-free route has

> **Status: adopted plan, carried by RT-A3 in the order tee, reducers in regions, constant-effort, nonlinear supports, then S6 connectors, S3, S7, P and R-3 (Agent 0, 2026-10-30; S7 added the same day, see below). Built: stage 2 (#1671); the §8.30 corrections; S1's tee and junction part (RT-A3 increment 1), with junctions recognised implicitly at shared terminal nodes and no `junction` token (D-1, recorded in [pressure-contract.md](pressure-contract.md) "Tees and pressure junctions"). S1's reducer part and S2–S8/P are unbuilt. Superseded by any later Design text.**
> Committed from the solver lane's working note so that the adopted decisions are
> discoverable on main. The text is unchanged except that local absolute paths are
> shortened to `scratch/…`; those name working files that are not retained.

**Known stale statements (main at `3be888a820`):**
- The “open” stage-2 status in Bases read and §6.4 is historical: R-a/R-b/R-d and R-c are already on main through #1671 (`ab30066ca9`); the stage-2 merge prerequisite is satisfied.

Design note for the SWBPIPE solver lane (TASK, research and design only). No code was
changed, nothing was built or run, and no measurement was taken. Nothing here records or
implies acceptance, issuance or lifecycle standing (F-PIP-3). "Certified" is used only in
K-D5's technical sense (DEL-04-01 `kd5-nonlinear-active-set.md` R5); product text says
"established" or "confirmed".

**Bases read.**
- **M:** `origin/main` at `c86477557b`.
- **S2:** `origin/claude/piping-w16s2c-spring-base-20261027` at `e0d322cf3a` (stage 2, #1654
  and #1671, open).
- **OPN:** `scratch/solver-operating/design_note.md` (revision 2,
  with Agent 0's decisions of 2026-10-27 in its §0).
- **K6:** `core/solver/performance_harness/observations/k6/k6_packet.json` on M (release build,
  Apple M5 Max, 128 GiB, source `1f354c20bc`).

Paths are relative to `projects/chirality-piping/`. Line numbers are M unless marked S2.

| Short | Path |
|---|---|
| PP | `core/product_physics/src` |
| NI | `core/solver/nonlinear_integration/src` |
| NS | `core/solver/nonlinear_supports/src` |
| FK | `core/solver/frame_kernel/src` |

Arithmetic for §5 and the reference values in §3 and §4 is in
`scratch/solver-r4/estimates.py` (exact rationals, no product code).

---

## 0a. Scope decision: S7 in RT-A3 (Agent 0, 2026-10-30)

Agent 0 added S7, a nonlinear support as a friction normal source (§3.8), to RT-A3's scope. The decision was made under the owner's 2026-10-27 delegation, on the RT-A3 manager's recommendation. A lift-off friction shoe is the ordinary resting support in real models, so exit criterion 1's reference system (DEL-09-01 `reference-system.md`, FR1 at RS-120 with its one-way FR1V) must exercise it.

- **Order.** Increments 1–4, then S6, S3, S7, P and R-3. S7 moves ahead of P only if the M-1a/M-2 measurements show that P can wait.
- **Interim.** Until S7 lands, the reference fixture gives FR1 a linear rest as its normal source (option (a)). That substitution is recorded in DEL-09-01's pending-family ledger against S7.
- **Verification.** S7 changes numerical meaning on both routes, so it takes its own confirmation references (F-2a, F-2b and F-3 in §3.8 at least) and the independent xhigh review.

## 00. Codex round-1 corrections (2026-10-28; these supersede the sections they name)

Codex reviewed this note independently, read-only, against main at `7e734910549e` (the affected sources are unchanged from `c86477557b`). It agreed with every reference value: T-1, T-2, T-4, G-1, F-1, F-2b, H-1, H-1b and H-2. It also confirmed the following:
- **Junction resultant.** −Σ p·A_i,k·s_k is correct, with s_k pointing from the node into each incident member. It uses the reduced pressure bore once, independent of the publication basis. Arc ends use tangents, not chords.
- **Fixed pressure loads.** Under the small-displacement formulation they stay correct through support-state changes. The constitutive law is EA ε = N_w − 2νpA_i, and closed-pipe free elongation must not be added a second time.
- **Code claims.** The ledger, confirmation, hydrotest-validation and dense-loop claims all hold.

The corrections below replace the text they name.

**C-1, §3.1: a live defect on main (decided by Agent 0).** Replace the shared-terminal finding with:
> Main admits component-free pressure junctions represented by shared region terminals. An incorrectly compensated junction terminal excludes its fitting thrust without a junction diagnostic, although the evidence records remote-closure exclusion. Add an immediate named refusal for unsupported shared-terminal junctions, then implement explicit junction admission in S1.

The minimal request is `scratch/solver-r4-review-20261027/shared-terminal.request.json`. In it, R_T = −20,250π ŷ N ≈ −63.6 kN, against 0 for the ordinary closed tee. Agent 0 adopted the named refusal as a correction: it ships with the PF nonlinear memory guard in one PR at §8.30, after §8.29.

**C-2, §4.4: hydrotest "springs held".** Replace the spring bullet with:
> LockedEquivalentSupport replaces the device law on each locked DOF: prescribe the entered absolute position and suppress that device's stiffness and preload there. Recover the complete locked-support action from the resulting unreduced equations. Retaining a parallel spring requires separately reporting spring and locking-device actions.

The superseded text kept the spring's stiffness, which misreports the action by k·u_C: 2,000 N at k = 2 MN/m and u_C = 1 mm. Suppressing constant effort is correct under this replacement convention.

**C-3, §3.1 T-3.** Replace with:
> For s₁ = −x̂ and s₂ = +x̂, the bore-step fitting load is p(A_i,1 − A_i,2)x̂. Support reaction depends on the complete boundaries. With free capped far ends, as in T-1, the junction anchor reaction is zero.

**C-4, §3.1 and §6: inventory coverage.** Replace the relevant scope text with:
> S1 replaces the blanket reducer-region refusal with explicit admission conditions for the proposed concentric mapped-member representation. S6 includes annotation-only joints, geometry-only bends outside pressure regions, and other/TBD metadata records under their PF mechanics and disclosures. Pressure-carrying eccentric reducers require an additional thrust-and-moment slice; until then, reducer pressure parity is incomplete.

**C-5, §3.9.** Replace "reached only by a zero-clearance pair" with:
> Repeated active boundary DOFs can arise from incompatible initial or trial states at any clearance. S8 must define deterministic handling of opposing-stop trials, preserve genuine conflict refusals, and verify seed permutations and transitions. Final confirmation establishes at most one engaged stop and the other stop's separation.

**C-6, §5.2–5.5: performance.**
- The leading audit storage is about 88n² + 16n_f² bytes. That is 14.976 GB (13.947 GiB) at 2,000 nodes. It is not a peak-memory bound.
- The K6 1,984-member grid has 1,024 nodes, so its 0.198 s is not a 2,000-node measurement.

Replace the verdict and measurement text with:
> Both routes share an unguarded quadratic nonlinear allocation. At 2,000 nodes its leading buffers approach 14.976 GB, making 16 GiB operation doubtful without measured headroom. The 3–7 s forecast after P remains a hypothesis. Measure a currently admitted EX model now; measure the full rack after its admission stages; and measure actual EX nonlinear cases after S5/P, including derived-normal friction. Record end-to-end time, peak RSS, iterations and output size. Separating publication and serialization requires instrumentation.

M-1 is therefore redefined in three steps:
- **M-1a:** a currently admitted EX model, now.
- **M-1b:** the full rack, after S1/S4.
- **M-1c:** EX nonlinear, after S5/P.

The interim memory guard (adopted by Agent 0 as a correction, §8.30) applies in both modes before the first dense allocation. It uses checked size arithmetic and a stated provisional ceiling, and does not call 104n² a complete memory bound.

**C-7, §1 inventory (non-blocking).** Add:
> Equivalent-static admission retains named restrictions for partial-span Timoshenko wind and wind on connector-replaced spans. Legacy expansion joints remain refused on both routes.

The partial-span Timoshenko restriction is retired by #1678 when that merges.

**C-8, §4.6 H-a (non-blocking).**
- Replace "no new format" with:
  > H-a can reuse the source-reference record shape, but needs a governed generated-source identity and inclusion contract: generation order, factors, omission, collision checks and evidence. Existing mass selection remains unchanged.
- Replace the blanket coexistence claim with:
  > Locked-support hydrotest and generated seismic currently lack a shared-document path. Hydrotest authored through pressure regions and weight loads without locking can already coexist with seismic on plain EX.

**C-9, §6.3 reserved choices (non-blocking).** Replace with:
> Bourdon eigen-rotation and pressure-dependent flexibility/SIF are separate scope decisions. The latter changes stiffness or recovery inputs and needs formation verification, not merely another RHS term. Performance targets require owner adoption before becoming exit criteria. Hydrotest may use explicit water-weight loads without deciding OC-7.

**C-10, §6: order.** S1–S4 before S5, and P before R-3, are both sound. Replace S7's dependency sentence with:
> S8 and gap-derived normals depend on R-3. One-way-derived normals and k > 1 depend on their own confirmation work and P's scale repair.

**C-11, §2.2: Timoshenko confirmation.** Confirmation also requires complete member identities, and each pressure primitive's frame must match its member's stiffness frame, including beam theory and shear-area bits (FK `structural/formation_check.rs:1275–1297`). "Never run" for EX nonlinear publication is stronger than the code establishes: it is excluded by admission.

## 0. Result in one page

- **The gap is smaller in mechanics than in plumbing.** Pressure on EX is a set of
  case-constant right-hand-side terms: the Poisson pair, terminal caps and bend or kink
  remainders, pushed into the case ledger as source groups (PP `pressure_runtime.rs:906-994`,
  `lib.rs:4949-4966`). The nonlinear loop takes the ledger's `AssembledForce` unchanged
  (PP `lib.rs:7227-7254`). So pressure enters the active-set loop exactly as thermal
  eigenloads do. Nothing in the loop is displacement-dependent: Bourdon opening and pressure
  stiffening are excluded (D-3; PP `exact_admission.rs:106`). The work is admission, the EX
  publication paths, the pressure readers, certificate plumbing and pricing.
- **Tees and reducers share one missing piece: a pressure junction.** A region is one chain
  with two terminals (PP `pressure_runtime.rs:1547-1576`). One closure kind, an open
  junction where regions of one pressure meet, gives the exact resultant −Σ p·A_k·s_k at a
  tee, a reducer or a bore change. It reuses the existing terminal-cap terms and their K-D5
  manifests.
- **Stages** (§3): S1 tees and reducers (M), S2 constant-effort supports (S), S3 imposed
  displacement primitives on plain EX documents (S), S4 hydrotest, linear first (M), S5
  nonlinear supports on EX (L), S6 nonlinear supports with connectors and on load-state
  documents (M), S7 friction with a one-way normal source and k > 1 (M–L), S8 two-sided stops
  (M, with R-3). P, a nonlinear-loop scale repair shared by both routes, is L. **Total XL.**
- **Performance verdict** (§5). The linear EX kernel is fast at 2,000 nodes (K6: sparse
  checked entry about 0.1–0.2 s). End-to-end time is unknown. Certified publication and output
  volume (about 244,000 rows per case) probably dominate it.
  - **The nonlinear loop cannot run a 2,000-node model on the 16 GiB machine on either
    route.** It assembles a dense global stiffness and uses the dense preparation in both
    solver modes (NI `lib.rs:583-600`, `structural_adapter.rs:413-437`; FK
    `structural.rs:932-954`, `:1487-1517`). That is about 104 B·(6N)², roughly 15 GB at
    N = 2,000, and no resource guard applies in sparse mode.
  - This is a shared NI limitation, so it is **not a reason to add pressure to PF**. EX
    plausibly meets the targets once stage P lands. One measurement would settle it (§5.4);
    it was not run.
- **First build slice:** S1 (tees and reducers by the junction closure). It does not touch
  the nonlinear code that slices 1a and 1b, §4 and stage 2 occupy, and almost every real
  pressurized system needs it.

---

## 1. Inventory, taken from the code

### 1.1 How EX refuses

- `ExactFamily` lists 15 families (PP `exact_admission.rs:143-168`). `admission` admits
  valve, flange, reducer, rigid/specialty, objective connector, realized bend,
  equivalent-static and combinations (`:278-304`). Every other family returns
  `EXACT_PRESSURE_FAMILY_NOT_ADMITTED` (`:27`, `:305-308`) with its static text (`:214-261`).
- `validate_profile` emits the refusals:
  - per component (`pressure_runtime.rs:297-307`);
  - per support, for nonlinear and constant-effort records (`:314-322`, through
    `support_family`, `exact_admission.rs:373-381`);
  - per case (`:372-391`).
- The validation that PP shares across routes (`lib.rs:2888-2896`) adds the hydrotest and
  rotational refusals on both routes (§1.4).

### 1.2 What EX refuses and PF admits

| # | Family, feature or input | EX refusal (code, line) | How PF handles it |
|---|---|---|---|
| 1 | **Tee / branch** (`branch`, `tee`, `branch_connection`) | `EXACT_PRESSURE_FAMILY_NOT_ADMITTED`, `exact_admission.rs:181`, `:231-233`, emitted at `pressure_runtime.rs:302-306` | Solved as a rigid three-pipe junction. The component carries user SIFs: PF publishes `component_equal_factor_intensified_bending_stress_v1` rows at the header and branch member ends (`preview_physics.rs:15`, `:64`, `:323-352`). Branch flexibility is not assembled (`lib.rs:13581-13589`, `flexibility_in_assembled_stiffness: false`). Topology findings are warnings (`validation.rs:1308-1377`, `:1661-1716`). |
| 2 | **Reducer inside a pressure region** | `EXACT_PRESSURE_REDUCER_IN_REGION`, `exact_admission.rs:31-33`, `:349-369`, emitted at `pressure_runtime.rs:311-313` | PF has no pressure. The reducer is its mapped straight member with the DEC-045 scale. A reducer outside every region is admitted on EX too. |
| 3 | **One-way and lift-off supports**, translational and rotational | `EXACT_PRESSURE_FAMILY_NOT_ADMITTED` (`NonlinearSupport`), `exact_admission.rs:249-251`, `:374-375`; emitted at `pressure_runtime.rs:314-322` | Active-set loop (NI), with K-D5 confirmation of a qualified non-gap state (PP `lib.rs:7182-7308`). Rotational one-way and lift-off are covered in the moment family (`kd5-nonlinear-active-set.md:9`, `:51`). |
| 4 | **Gap supports** (translational) | as #3 | Solved in the loop. Never confirmed (R1): gap cases publish `NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED` (PP `lib.rs:5704-5731`; the confirmation is skipped at `:7265-7271`). |
| 5 | **Friction**, explicit normal or a normal derived from a linear base restraint | as #3 | Static, history-free Coulomb (DEC-067). Sliding force −dir·μ·\|N\|. The derived normal is coupled by dense coupling solves (NI `lib.rs:1377-1440`). Confirmation covers k ≤ 1 derived sliding supports (`kd5-nonlinear-active-set.md:11`). |
| 6 | **Constant-effort support** | `EXACT_PRESSURE_FAMILY_NOT_ADMITTED` (`ConstantEffortSupport`), `exact_admission.rs:252-254`, `:376-377` | DEC-049: a constant nodal force on its one translational DOF in every case (PP `lib.rs:14128-14143`), with rows `constant_effort_support_applied_load` (`:14145-14199`). Mechanics combinations are gated by the unit factor-sum rule (DEL-04-07 `pressure-contract.md:235-271`). |
| 7 | **Support-targeted `imposed_displacement` primitive** (anchor or nozzle movement) | `EXACT_IMPOSED_DISPLACEMENT_PRIMITIVE_UNSUPPORTED`, PP `imposed_motion.rs:13-14`, `:26`. EX's own form is `analysis_state.support_states[].boundary_motion`, which exists only on load/reference-state documents. | Stage 1: one prescribed value on a rigid support DOF, through the partitioned reduction (`imposed_motion.rs:1-14`; `pressure-contract.md:308-317`). Stage 2 (S2) adds motion in nonlinear models: R-a, R-b and R-c (`pf-nonlinear-imposed-motion.md`; S2 NI `lib.rs:2005-2046`). |
| 8 | **Annotation-only expansion joint** (`not_solver_consumed`) | `EXACT_PRESSURE_FAMILY_NOT_ADMITTED`, `exact_admission.rs:240-242` | Analysed as pipe (the refusal text says so, `:241`). |
| 9 | **Geometry-only bend** (chord) | `EXACT_PRESSURE_FAMILY_NOT_ADMITTED`, `exact_admission.rs:219-221` ("realize this bend") | Straight chord with metadata (D-2; `pressure-contract.md:12`). |
| 10 | **`other`, `TBD` or unrecognized component kinds** | `EXACT_PRESSURE_FAMILY_NOT_ADMITTED`, `exact_admission.rs:246-248`, `:338` | Not traced in detail. PF carries them as records without assembled mechanics. |
| 11 | **`combinations[].difference_basis`** (0.9.0, designed) | refused by name on EX (OPN §0 "Shape"; DEL-02-01 `model-document-0.9.0.md:37-40`) | 1b: G-1 and G-2 gates on PF (OPN §1.2) |
| 12 | **Mechanics combinations in nonlinear or constant-effort models** | cannot arise on EX (`exact-combinations.md:243-245`) | Withheld by `NONLINEAR_COMBINATION_REQUIRES_SOLVE` and `CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE`. Subtractions and range envelopes form from published rows (OPN §1.1). |

On EX, nonlinear supports are refused before any NI call. NI then has refusals of its own,
which apply on both routes and which R-4 inherits:
- objective connectors are refused by the loop (NI `lib.rs:1172-1176`, `:1112`);
- a nonlinear support cannot be a friction normal source (PP `lib.rs:7955-7967`; S2
  `lib.rs:8011-8022`);
- an active boundary DOF may not repeat (NI `lib.rs:1924-1930`; S2 `:1989-1995`);
- the confirmation refuses a DOF that a nonlinear support shares with another support (NI
  `confirmation.rs:183-189`; S2 `:191-197`).

### 1.3 Load/reference-state slots that exist in the format but are refused

These matter for hydrotest (§4) and for nonlinear supports on load-state documents (S6). The
slots exist in `case_state/input.rs`: `SupportStateInput` (`:337-346`), `ParticipationInput`
`{ActiveModelDevice, Inactive, LockedEquivalentSupport{components}}` (`:350-356`),
`LockedComponentInput`/`PositionSourceInput {Entered, PredecessorValue}` (`:358-378`) and
`DeviceReferenceInput {ForceAtReference, UnloadedReference}` (`:395-408`).

| Slot | Refusal (`case_state/resolve.rs`) |
|---|---|
| `participation: inactive` or `locked_equivalent_support` | `LOAD_STATE_SUPPORT_PARTICIPATION_UNSUPPORTED`, `:296-302` |
| `base_motion` (spring base), `device_reference` (preload or reference) | `LOAD_STATE_SUPPORT_REFERENCE_UNSUPPORTED`, `:303-306` |
| `boundary_motion` on a non-rigid DOF (spring, nonlinear, constant-effort) | `LOAD_STATE_BOUNDARY_MOTION_UNRESTRAINED`, `:1055-1064` |
| `element_states[].mass_state_ref` | `LOAD_STATE_MASS_STATE_UNSUPPORTED`, `:262-265` |
| `equivalent_static` on any load-state document | `EXACT_EQUIVALENT_STATIC_LOAD_STATE_UNSUPPORTED`, `exact_admission.rs:128-134`, emitted at `pressure_runtime.rs:377-384` |

The v3 limitation text says the same: "Spring base motion, device preload/reference, inactive
or locked supports are not provided" and "History is independent equilibrium only; no
installation, contact, friction or predecessor history" (`exact_admission.rs:117`, `:119`).

### 1.4 Refused on both routes (parity-neutral, but relevant)

| Item | Where |
|---|---|
| Hydrotest pressure primitive (`category: hydrotest`, `dimension: pressure`) | `HYDROTEST_PRESSURE_UNSUPPORTED`, PP `validation.rs:61-70`. This is the shared validation (`validation.rs:12-28`, called at `lib.rs:2895` on both routes), so **it is not PF-only**. On EX the same primitive is also `EXACT_PRESSURE_REQUIRES_REGION` (`pressure_runtime.rs:386-391`). |
| Rotational gap; rotational friction | `NONLINEAR_ROTATIONAL_GAP_UNSUPPORTED`, `validation.rs:114-122`; `NONLINEAR_ROTATIONAL_FRICTION_UNSUPPORTED`, `:126-134` (shared) |
| Rigid restraint or same-record spring on a nonlinear DOF | `SUPPORT_LINEAR_NONLINEAR_DOF_CONFLICT`, `validation.rs:135-172` (shared) |
| Spring hanger installed, cold and hot loads | Review-only rows on both routes (`lib.rs:13799-13843`; `mechanics_consumption` defaults to `review_only`, `:13774-13778`). The spring force is −k·u from the zero reference; no preset load is applied. **Finding:** programs that preset a hanger's installed load apply it as a force, so a spring-supported sustained case can differ from such a model. This is outside R-4 (both routes lack it). The EX slot `device_reference.force_at_reference` is the natural carrier (§1.3). |
| Branch flexibility factor | not assembled on PF (`lib.rs:13586`); EX likewise once tees are admitted |
| Mitre (straight–straight kink in a region) | `PRESSURE_REGION_NONCOLLINEAR` / `PRESSURE_REGION_MITRE_UNSUPPORTED` (`pressure_runtime.rs:41`, `:1690-1695`, `:1747`); PF has no pressure |

### 1.5 What EX admits and PF does not (the direction of unification)

| EX-only capability | Evidence |
|---|---|
| Pressure: signed internal differential, case-scoped regions, terminal closure transfer, Poisson eigenload, bend and kink remainders | `pressure_runtime.rs:63-83`, `:877-994`; PF refuses `pressure_regions` and pressure primitives (`:355-369`) |
| Lamé hoop and radial rows, wall force N_w = N_el + pA_i, toroidal arc hoop | `pressure-contract.md:56`; PP `u4b_rows.rs:11-21` |
| Timoshenko (`timoshenko_annulus_energy_v1`) | `pressure_runtime.rs:42-53`, `:229-236`; PF refuses it in the 0.8.0 interim (`validation.rs:43-55`) |
| D-5 section bases (nominal and reduced, corrosion allowance) | `exact_admission.rs:94-96`; `CORROSION_ALLOWANCE_WITHOUT_PRESSURE_CONTRACT` on PF (`pressure_runtime.rs:259-261`) |
| Objective connectors and tie rods (expansion joints) | `exact_admission.rs:290-292`; PF refuses them (`pressure_runtime.rs:268-286`) |
| Load/reference-state group: per-member E/ν and temperatures, fit eigenstrain, `boundary_motion` | `exact_admission.rs:113-120`; `case_state/` |
| Retained-source publication (`pressure-source-1`) | `exact_admission.rs:20-22`, `:98-101` |

Unification therefore means extending EX, as the owner's OC-2(a) ruling directs. No EX-only
capability is to be copied to PF.

---

## 2. Common numerical basis for every stage

### 2.1 How pressure enters the loop

- **Assembly.** `ExactPressureCase` carries per-group operands (`pressure_runtime.rs:86-102`).
  The case ledger pushes each source group as a formed term and attaches its
  expected-summand manifest (`lib.rs:4949-4966`). The group bound is γ₂₀·|v|
  (`lib.rs:4969-4985`). The case's `AssembledForce` then holds:
  - mechanical loads;
  - thermal or fit eigenloads;
  - the pressure Poisson pair (coefficient ±2ν, `pressure_runtime.rs:949-962`);
  - terminal caps at `transfers_to_wall` terminals (`:977-993`);
  - bend cap removal on arcs (`:963-976`);
  - kink remainders at bend-adjacent junctions (`:906-937`);
  - the K_b·u_free(ε_p) push on each arc (`pressure_runtime.rs:3-12`).
- **The loop.** `append_nonlinear_support_loop_results` passes `force.values()` and checks it
  bit for bit against the ledger (PP `lib.rs:7232-7254`; NI `lib.rs:545-572`). Every
  pressure term is therefore a fixed load inside the active-set loop, like a thermal
  eigenload.
  - No term depends on u, since the formulation is small-displacement with no follower
    pressure.
  - Bourdon opening of bends and pressure stiffening of k and SIF are excluded and disclosed
    (D-3, `exact_admission.rs:106`; `pressure-contract.md:13`, "T4-U9 requires a later owner
    choice").
  - If T4-U9 later adds Bourdon, it is also an eigen-rotation load K_b·u_free(θ_B) on each
    arc, so it stays a fixed RHS term and does not change the loop.
- **Unilateral supports and pressure elongation.** States are classified on the total u.
  Free closed-pipe elongation is ε_p = (1−2ν)·p·A_i/(E·A_s) (Lamé: σ_r + σ_θ = 2pA_i/A_s,
  constant through the wall). So:
  - a gap closes when u_d crosses ±g;
  - a one-way support lifts off when its reaction changes sign;
  - a friction support's normal changes with the pressure-thrust reaction.

  A negative (vacuum) differential is admitted (`exact_admission.rs:105`) and can open stops
  that positive pressure closes. Collapse is not assessed.
- **Support actions.** A reaction is (K·u − f)_d with f including any cap at that node. For
  example, a stop at a capped terminal carries the cap thrust. This is the existing
  support-on-pipe convention.
- **Wall recovery.** N_w = N_el + pA_i and the Lamé rows are recovered from the final u
  exactly as in a linear case (`pressure-contract.md:20`, `:56`). A nonlinear support changes
  u only.
  - **Caution.** The EX publication path has never run with a nonlinear support. The code
    says so (PP `lib.rs:5758-5760`: "pressure-2's ordinary binary64 publication, which
    cannot naturally admit nonlinear supports yet").

### 2.2 How K-D5 confirmation and closure extend

- **The pressure manifest already reaches the confirmation.** `confirm_converged_state`
  receives the case's `AssembledForce` (NI `confirmation.rs:78-88`). The route appends its
  friction terms after the ledger's own terms, so every original term keeps its index and
  manifest (`kd5-nonlinear-active-set.md:136-146`). The pressure summands are those attached
  at `lib.rs:4964-4965`. So confirmation with pressure is the existing closure applied to a
  ledger that also holds `pressure_operand` summands. No new closure class is needed for
  straight members, arcs (curved sources are passed, `lib.rs:7272-7285`) or connector joints
  (once S6 lifts the connector refusal).
- **Timoshenko.** The confirmation already takes per-member identities "T4-U8 (DEL-05-03
  Design §4 item 2)" (`confirmation.rs:84-86`). FK's formation check re-forms Timoshenko
  leaves (`formation_check.rs:1238`, `:1255`). Each nonlinear stage needs a Timoshenko
  confirmation case, because PF never exercised it (PF refuses Timoshenko).
- **What needs new M3-style controls.** Each stage adds mutation controls on the integrated
  producer:
  - drop one of many pressure summands from a confirmed nonlinear case and the closure must
    fail ("formation check: … load closure: …");
  - misplace a junction cap (S1);
  - swap a source DOF (S7).

  These follow `pressure-contract.md:20`'s drop-one-of-many rule.
- **Active-set certificate.** It is unchanged for one-way, lift-off and explicit or
  base-derived friction (`kd5-nonlinear-active-set.md:13`). Gaps stay unconfirmed until R-3
  (G2), and weakly active supports fail closed (R3).
- **Member bound.** A confirmed state's Stage-B bound is used where the linear bound would
  be (PP `lib.rs:5748-5757`). The pressure wall bound ("the separate wall bound",
  `pressure-contract.md:20`) must be taken from that member bound in the certified
  pressure-2 publication. **This needs an explicit integration test.**

### 2.3 The exact load-state route

On load/reference-state documents each case has its own resolved E/ν per member and its own
stiffness (`exact_admission.rs:115`). The loop is per case, so nothing changes there. It
needs:
- **Boundary motion of rigid restraints** as the loop's base prescribed map. This is stage
  2's R-a (S2; `pf-nonlinear-imposed-motion.md`), fed from `boundary_motion`
  (`resolve.rs:1040-1085`) instead of the PF primitive.
- **Motion on a nonlinear support's own DOF** (R-b) and **spring base motion** (R-c). These
  lift `LOAD_STATE_BOUNDARY_MOTION_UNRESTRAINED` (`resolve.rs:1062-1064`) and the
  `base_motion` refusal (`:303-306`) for those support kinds, with stage 2's semantics.
- **History stays independent.** Each case is solved from the model-level seed (OPN §1.1;
  R-2). The v3 sentence "no … contact, friction … history is represented"
  (`exact_admission.rs:119`) becomes: "contact and friction are static and history-free
  (DEC-067); each case is solved independently from the authored seed".

### 2.4 Publication shared by the nonlinear stages

- **Rows.** Reuse PF's nonlinear kinds and the readers' knowledge of them:
  - state codes and the final displacement, reaction and relative-displacement rows;
  - iteration, residual and converged rows;
  - explicit and derived friction-normal rows;
  - `NONLINEAR_SUPPORT_*` diagnostics.

  The kinds are listed at OPN §1.1. Today only the preview reader knows them (RE
  `preview_physics_evidence.rs`; `tests/verifiers/preview_physics_evidence.py`); the
  pressure reader (RE `physics_evidence.rs`, `tests/verifiers/pressure_2.py`) does not.
  The pressure-2 result schema (`schemas/results_pressure_2.v0.3.schema.json`), its reader
  corpora (`fixtures/results/pressure_v3_*_reader_corpus.json`) and the semantic table take
  the same kinds.
- **Limitation text.** The v3 formulation basis changes in three places: the "Admitted
  families" sentence (`exact_admission.rs:94-96`), the support-attribution sentence (`:109`)
  and the load-state sentences (`:117`, `:119`). These are reviewed semantic inputs, so they
  need re-registration. Their TEXT is priced (O-10, `pressure-contract.md:35-36`).
- **Pricing.** Every stage changes the priced static chain (the v3 text, `pressure_runtime`
  and `solve_prepared_load_case`). Nonlinear sites price at multiplicity 0 under D1.6
  (`retained-pricing-rules.md:75`), but text sites are charged positively, as the nonlinear
  K-D5 unit was (`kd5-nonlinear-active-set.md:442`). **Each stage queues in the serial
  pricing order** (OPN §0, Order).
- **Retained sub-route.** Nonlinear cases never reach W1 or retained-source recovery (four
  refusals, `kd5-nonlinear-active-set.md:282`; `retained_product.rs:1944`,
  `source_recovery.rs:863`). R-4 keeps them, and D1 is unchanged.

---

## 3. Stages

The order is S1 → S2 → S3 → S4 (linear) → S5 → P → S6 → S7 → S8. It is justified in §6.
Interactions with OPN's slices and with R-3 are given per stage and collected in §3.10.

### 3.1 S1: tees and reducers by a pressure junction (M)

**Problem.** A region must be one chain with its two terminals as its only degree-one nodes
(`pressure_runtime.rs:1547-1552`). An internal node may not touch an outside pipe
(`PRESSURE_REGION_INTERNAL_BRANCH`, `:1554-1563`). So a pressurized tee can be neither one
region nor a branch point inside one.

**Finding.** The checks I read do not stop three regions from each ending at the tee node
with `transfers_to_wall` there:
- a terminal node only needs region degree 1 (`:1550`);
- only non-terminal nodes are tested for outside pipes (`:1554-1555`).

That representation gives the right resultant (below). But it calls an open junction a
"closure" in evidence (`closure_pressure_load_global_n`, `:899-903`). It also gives a wrong
answer if a user marks a junction `separately_supported_or_compensated`, because the thrust
is silently dropped. I found no test that exercises it. **S1 must formalize it and refuse
the misuse.**

**Numerical basis.**
- **The junction resultant.** Take the closed surface formed by the fitting's wetted
  interior and its openings, where s_k is the unit vector from the node into member k. Zero
  net pressure force on a closed surface gives the fluid force on the fitting body as
  F = −Σ_k p·A_i,k·s_k. That is exactly the sum of the k regions' terminal caps at that
  node: each `transfers_to_wall` cap is p·A_i,k along the outward direction −s_k
  (`pressure_runtime.rs:977-993`). It covers:
  - an equal tee: −pA_b·s_b, the branch thrust that the branch wall tension balances;
  - a reducing tee;
  - a concentric reducer or bore step: p(A_1 − A_2) along the axis;
  - a cross.

  The kink remainder pA_i(t_in − t_out) (`:906-910`) is the two-member special case.
- **New closure token: `closure_transfer: "junction"`.**
  - **Ledger.** It is treated in the ledger exactly as `transfers_to_wall`, so the same
    `terminal_cap` summands enter and S11-G's γ₂₀ bound applies unchanged.
  - **Evidence.** It is labelled as a junction, not a closure.
  - **Validation**, each failure refused by name:
    - every pipe incident to the node belongs to a region of the case
      (`PRESSURE_JUNCTION_MEMBER_UNREGIONED`);
    - every region with a member at the node has a `junction` terminal there;
    - all those regions have bit-identical pressure quantities
      (`PRESSURE_JUNCTION_PRESSURE_DIFFERS`), because one fluid node has one pressure;
    - the node has degree ≥ 2;
    - a two-member junction is collinear within α_tan. A larger angle stays
      `PRESSURE_REGION_MITRE_UNSUPPORTED`, so T4-U7 is not admitted by the back door;
    - no connector face is at the node;
    - `transfers_to_wall` or `separately_supported_or_compensated` at a node shared by two or
      more pressurized regions is refused (`PRESSURE_TERMINAL_AT_JUNCTION`).
- **Reducers.** Once a bore change is a junction, `REDUCER_IN_REGION`
  (`exact_admission.rs:349-369`) becomes:
  - a concentric reducer's mapped member is its own region, joined by junctions at the
    bore-change node;
  - `REDUCER_IN_REGION` still refuses a reducer placed inside one region (the equal-bore
    check never reads it, `:342-348`);
  - an eccentric reducer (DEL-03-05 `eccentric-reducer-offset.md`) stays refused in
    regions, because its offset adds a moment p·A·e that this design does not form.
- **Tee component on EX.** The branch family is admitted with:
  - its geometry and SIF validation, unchanged and shared (`validation.rs:1308-1377`);
  - PF's intensified rows, `component_equal_factor_intensified_bending_stress_v1`, at the
    header and branch member ends (`preview_physics.rs:15`, `:323-352`), on both D-5 bases
    (Z per basis, each row naming its basis).

  These are derived from published member end rows, so a reader recomputes them. Branch
  flexibility stays unassembled, as on PF. The tee's own SIF semantics are those of PF; no
  code stress is claimed (`preview_physics.rs:64`).
- **Formation checks.** Tee members are ordinary straight (or Timoshenko) members at a
  three-member node. FK's formation check is per element (`formation_check.rs:703-727`), so
  nothing new is formed. An assembled branch flexibility would be a new element class with its
  own formation check, and it is not in R-4.
- **K-D5 and M3.** The closure manifests are the existing `terminal_cap` summands. The new
  M3 controls are:
  - drop one junction cap of three, and closure fails;
  - move a cap to the wrong node;
  - flip one s_k.

**Publication and readers.** A `junction_evidence` record in `pressure_rhs_assembly` gives,
for each junction node, its regions, bores, pressure and resultant. It follows the existing
kink evidence (`pressure_runtime.rs:933-936`). The terminal record gains the `junction`
token. The pressure-2 reader and the Python verifier re-derive the resultant from the
members' bores. The model schema needs the new `closure_transfer` token.

**Timing:** the 0.9.0 format step (DEL-02-01 `model-document-0.9.0.md` S1) is still design
only. Folding the token into it avoids a 0.10.0 step. That is the only timing-sensitive item
in this note.

**Independent references** (exact rationals, π symbolic, Euler–Bernoulli). Common data:
E = 200 GPa, ν = 0.3, p = 10 MPa. Header r_o = 0.1 m, t = 0.01 m; branch r_o = 0.05 m,
t = 0.005 m (both have A_i/A_s = 81/19). Tee node T at the origin; A = (−4,0,0),
B = (4,0,0), C = (0,3,0).
- **T-1, anchored tee, three capped legs, `junction` at T.**
  - The anchor reaction at T is **exactly 0** in all six components.
  - Free-end displacements are free elongations: A and B ∓81/237500 m in x
    (0.341053 mm); C +243/950000 m in y (0.255789 mm). All other components are 0.
  - Wall force pA_i,k is 81,000π, 81,000π and 20,250π N. The axial wall stress is
    810,000,000/19 Pa in every leg.
  - Dropping the junction terms would leave an anchor reaction of 20,250π N ≈ 63,617 N.
    That is the control a mutant must fail.
- **T-2, as T-1 with C `separately_supported_or_compensated`.**
  - The anchor reaction at T is +20,250π N along +y.
  - C moves −729/1,900,000 m (Poisson shortening only, σ_a = 0 in the branch).
  - A and B are as in T-1.
- **T-3, collinear bore step.** Two regions meet at an anchored node, A_i,1 > A_i,2. The
  anchor reaction is −p(A_i,1 − A_i,2) along the step.
- **T-4, free junction, all far ends anchored.**
  - The net load at T is −(1−2ν)pA_i,b·ŷ.
  - v_T = −(1−2ν)p·r_bi² / (E·[6(r_o⁴ − r_i⁴)/a³ + (r_bo² − r_bi²)/L_b])
    = **−1944/7,754,755 m** (−0.250685 mm), with a = 4 and L_b = 3.
  - θ_z = 0 by symmetry, and the header axial displacement at T is 0.

**Size: M.** Region validation, ledger token, evidence, readers, schema token, intensified
rows on EX, pricing, references and desktop region authoring (cross-lane: a "split region at
junction" helper).

**Risks:**
- (a) It changes today's silent behaviour at shared terminal nodes. This is a correction;
  declare the blast radius.
- (b) Authoring burden. Agent 0 chooses between junction terminals and tree regions (§6).
- (c) Reader churn on pressure-2.

**Interactions:**
- None with 1a, 1b, §4 or R-3. It does not touch NI or the nonlinear PP sections.
- The O−S certificate (§4) sees one more pressure ledger term; nothing else changes.

### 3.2 S2: constant-effort supports on EX (S)

**Basis.**
- **Force.** DEC-049's constant nodal force is a plain input term in the ledger
  (`lib.rs:14132-14143`). K-D5 closes input terms with radius 0 and needs no manifest, so
  pressure does not interact with it.
- **Combinations.** EX mechanics combinations already run the unit factor-sum gate for
  connector and tie-rod references (`exact-combinations.md:238-241`). Adding consuming
  constant-effort supports to that gate reuses
  `load_case_algebra::factor_sum_within_unit_tolerance` (`pressure-contract.md:266-271`).
- **Load-state documents.** A case's loads there are "exactly the case's declared source
  ledger" (`exact_admission.rs:118`). Proposal: treat the constant force as a **device
  action of an `active_model_device` support**, not an ordinary primitive.
  - It is present in a case where the support participates actively.
  - It is absent where the support is `locked_equivalent_support`. This is hydrotest's need
    (§4) and arrives with it.
  - The limitation sentence states the rule.

  The alternative is to refuse constant-effort supports on load-state documents by name.
  Agent 0 decides (§6).

**Publication.** The `constant_effort_support_applied_load` rows (`lib.rs:14182-14199`) and
the consumption warnings (`:14105-14126`) join the pressure reader.

**References.**
- **CE-1.** A vertical cantilever of the S1 header section, capped and pressurized, with a
  constant-effort support on UZ at the tip carrying a weight W. The pressure axial result is
  unchanged. The tip deflection is (F_ce − W)·L³/(3EI) and the reaction rows superpose.
- **CE-2.** The same model in combinations 0.3 + 0.7 (admitted) and 2·case (withheld).

**Size: S.** Risks: the load-state semantics choice.

**Interactions.** §4: the constant force is an exact input and cancels in O−S when both cases
carry it.

### 3.3 S3: support-targeted imposed displacement on plain EX documents (S)

**Basis.**
- **Today.** EX refuses the primitive and keeps `boundary_motion`, which exists only on
  load-state documents (`imposed_motion.rs:13-14`). A plain EX document therefore has no
  imposed displacement at all, while a pressurized import's anchor and nozzle movements arrive
  as primitives (OPN source item: the importer maps them to a thermal case).
- **The PF primitive is already built on EX's mechanism.** It maps to "the partitioned
  reduction the exact route already uses for `boundary_motion`" (`imposed_motion.rs:5-11`).
  So on plain EX documents: admit it, with the same rigid-support rules (`:41-47`) and the
  same refusals.
- **Load-state documents.** They keep `boundary_motion` and refuse the primitive, so one
  document never has two motion forms.
- **Pressure.** Pressure is independent of it (RHS versus prescribed values).
- **K-D5.** Prescribed values are input u_C, as for `boundary_motion`.

**Publication.** The existing PF evidence of the motion moves to the pressure reader.

**References.**
- PF's W-9 L-frame settlement and nozzle movement (`tests/w16_imposed_motion.rs`,
  `validation/references/w16_imposed_motion/`), run on EX with an explicitly empty region.
- One pressurized twin, where the pressure result superposes exactly with the motion.

**Size: S.** Risks: none beyond pricing.

**Interactions.**
- With nonlinear supports it needs stage 2's R-a on EX; that comes with S6.
- Slice 1b's G-1 compares primitives by content, so motion primitives participate.

### 3.4 S4: hydrotest, linear supports first (M)

This is described in §4. Its order is here: it needs S2 so that constant-effort supports
can be locked, and it can precede S5 for models whose supports are linear.

### 3.5 S5: nonlinear supports on EX (L), the core

**Scope.** One-way, lift-off and gap supports, and friction with an explicit normal or a
normal derived from a linear base restraint. It covers plain v3 documents, straight,
Timoshenko, realized-bend and rigid-family members, and pressure regions including S1
junctions. Connectors and load-state documents are S6.

**Basis.**
- **Admission.** Remove `NonlinearSupport` from the refusal set
  (`exact_admission.rs:249-251`).
- **The loop and the confirmation** are PF's, called from the shared
  `solve_prepared_load_case` (`lib.rs:5634-5677`). The `force` now carries the pressure
  groups (§2.1, §2.2).
- **Gaps.** Gap cases take R1's existing unqualified branches (`lib.rs:5704-5731`) and are
  published with standing (OPN R-5), until R-3.
- **Combinations.** EX's combination module gains PF's `NONLINEAR_COMBINATION_REQUIRES_SOLVE`
  gate for mechanics combinations (`exact-combinations.md:243-245` states it cannot arise
  today). Subtractions and range envelopes form from published rows with operand standing in
  W-16b's shape, which slice 1a also adopts on PF (OPN §1.2).
- **Slice 1b on EX.** `difference_basis`'s EX refusal lifts here.
  - **G-1** must compare pressure regions by content: members, terminals with their closure
    tokens, and pressure value and unit bits. Subtrahend regions must be contained in the
    minuend's.
  - **G-2** binds the resolved stiffness, which already exists in EX's mechanics gate 2
    (`exact-combinations.md:228-237`).
  - **On load-state documents.** Gate 2 compares the property-selection temperature
    (`exact-combinations.md:227-236`). Where a user selects E at each case's operating
    temperature, O and S differ in stiffness, and G-2 withholds the declared O−S; the plain
    subtraction stays. This is a meaning question for Agent 0 under OPN §5 (§6).
- **Retained.** The four W1 refusals are kept (§2.4).

**Publication.** §2.4, plus the gap standing sentence on EX. The support-attribution sentence
changes to "linear restraints, springs and nonlinear supports". Ambiguous coincident
attribution stays refused, and same-DOF overlap is already a blocking validation
(`validation.rs:135-172`).

**Independent references** (S1 header section, E = 200 GPa, ν = 0.3, p = 10 MPa unless
stated).
- **G-1, gap closing under pressure elongation.** Straight pipe, L = 10 m, anchored at x = 0.
  The terminal at 0 is `separately_supported_or_compensated` and the cap is at x = L
  (`transfers_to_wall`). An axial gap closes in +x at x = L with g = 0.5 mm.
  - ε_p = 81/950,000; free elongation 81/95,000 m (0.852632 mm), so the gap closes.
  - Overlap 67/190,000 m; contact force **13,400π N** (42,097.34 N).
  - Wall force N_w = pA_i − F_c = **67,600π N**; axial wall stress **676,000,000/19 Pa**
    (35.578947 MPa).
- **G-2.** As G-1 with g = 1 mm: the gap stays open, the contact is +0 and the end moves
  81/95,000 m.
- **G-3 (vacuum).** As G-1 with p = −1 MPa and a −x gap of 0.05 mm. The free contraction is
  −81/950,000 m (0.0853 mm), so the −x gap closes with overlap 0.0353 mm.
- **F-1, friction under a pressure-thrust normal.** Cantilever L = 3 m, anchored at node 0
  (terminal `separately_supported_or_compensated`), cap at node 1. A linear UX restraint at
  node 1 is the friction normal source. Friction is on UZ at node 1 with μ = 0.3, and F_z =
  50 kN is applied at node 1.
  - N = (1−2ν)pA_i = **32,400π N**; μN = 9,720π N (30,536.28 N).
  - The case slides: u_z = **27·(50,000 − 9,720π)/(3·1,719,500π) m** (32.4277 mm), and the
    friction force is −9,720π N.
  - The anchor carries the axial wall tension 2νpA_i = 48,600π N
    (σ_a = 486,000,000/19 Pa).
  - With F_z = 25 kN it sticks: u_z = 0 and R_t = −25,000 N.
  - This is a k = 1 Stage-A case (`kd5-nonlinear-active-set.md:107-123`).
- **O-1, one-way lift-off under pressure.** A horizontal pipe on a one-way vertical support
  near a capped end, with an axial elbow-free configuration chosen so that pressure thrust
  through a realized bend lifts the support. This reference is to be built from the S1
  sections with an exact cantilever form. It is listed because the bend's K_b·u_free term
  enters.
- **K-1, controls.** Every PF K-D5 nonlinear control (`kd5_nonlinear_tests.rs`, the
  `references/kd5_nonlinear` set) is run on EX twins with explicitly empty regions, and
  must give bit-identical loop results and the same standing.

**Size: L.**

**Risks:**
- (a) The EX publication path is new to nonlinear states (§2.1 caution).
- (b) Pressure-reader and corpus churn.
- (c) Pricing.
- (d) Timoshenko confirmation, new because PF refuses Timoshenko.
- (e) Model size: §5 shows that models above about 1,300–1,850 nodes fail on memory without
  stage P.

**Interactions:** after slice 1a (the operand record shape) and 1b (the 0.9.0 member), and
after stage 2 merges (shared lines in `append_nonlinear_support_loop_results`).

### 3.6 P: the nonlinear loop's scale (L, shared by both routes)

**Basis.**
- **Where the dense work is.** The loop assembles `assemble_global_stiffness_with_connectors`,
  a dense `Vec<Vec<f64>>` (NI `lib.rs:583-600`; FK `lib.rs:1605-1620`). Every iteration
  builds a dense reduced copy (NI `lib.rs:1984-1989`; FK `lib.rs:715-719`). It then solves
  through `prepare_structural_binary64`, which is FK's dense `prepare_bound`, in **both**
  modes (NI `structural_adapter.rs:413-437`; FK `structural.rs:1419-1423`, `:1487-1517`).
  The audit adds two dense n×n `Expansion` arrays (FK `structural.rs:932-954`,
  `:1045-1056`). Derived-friction coupling solves are hard-coded `DenseScrutiny` (NI
  `lib.rs:1382-1389`, `:1412-1422`).
- **Guard.** The linear route's F1b guard applies only in dense mode and only before the
  cases (PP `lib.rs:3057-3068`).
- **The repair is the linear route's own F1b pattern.**
  - `SparseAssemblyEvidence` (NI `structural_adapter.rs`, already used by PP's linear route,
    `lib.rs:7615`) assembles and prepares the loop.
  - The loop factors once per iteration and reuses that factor for the base and unit-load
    coupling right-hand sides, instead of k + 1 separate dense factorizations.
  - The confirmation uses the sparse formation-checked path in sparse mode.
- **Numerical meaning.**
  - Sparse preparation must be bit-identical to dense preparation on the reduced system, as
    K6 pins the staged linear entries "bit for bit" (performance harness README, K6).
  - Moving the coupling solves to the requested mode changes φ⁰ bits in sparse mode
    (`kd5-nonlinear-active-set.md:55`). Published friction rows are the certified Stage-A/B
    values, so the move lies inside the published bound. It is still a declared move.
- **Interim truthfulness fix (S, worth doing now).** Until P lands, add a named nonlinear
  resource guard in sparse mode, priced like F1b's: refuse before any n² allocation when
  104·(6N)² exceeds the ceiling. Today a large PF nonlinear model in sparse mode would try to
  allocate about 15 GB with no named refusal.

**References.**
- Parity of loop state, displacements and reactions between dense and sparse on every
  committed nonlinear output.
- A K6-style observation of one loop iteration at 1,000, 2,000 and 10,000 members.

**Size: L.** Risks:
- the coupling-solve move;
- confirmation parity;
- pricing (NI is on the static chain).

**Interactions.**
- It touches NI's loop, which stage 2 (R-a/R-b) also changes, so it goes after stage 2.
- It is independent of 1a, 1b and §4.
- R-3's G2 runs inside the same loop, so ordering P before R-3 avoids re-pricing G2's sites
  twice.

### 3.7 S6: nonlinear supports with connectors, and on load-state documents (M)

**Connectors.**
- **What changes.** Lift NI's connector refusal (`lib.rs:1172-1176`). The loop's assembly
  already includes connectors and tie rods (FK `lib.rs:1600-1611`), and the confirmation
  passes `input.connectors` (`confirmation.rs:285`).
- **The joint pressure term** p(A_e − A_i) is a ledger term (DEL-04-07
  `joint-pressure-thrust.md`). The strict-gap path reports
  "mixed/curved/connector/affine" as unqualified (NI `lib.rs:4785`, test), and that stays
  the gap branch.
- **Combinations.** The unit factor-sum gate already covers tie-rod references.
- **Reference C-1.** A pressurized untied joint between two anchors, with a one-way guide on
  one side. The joint thrust p·A_e closes or opens the guide in exact closed form.

**Load-state documents.** §2.3: R-a, R-b and R-c fed from `boundary_motion` and `base_motion`.
**Reference LS-1:** stage 2's PF references, run on load-state twins.

**Size: M.** Risks:
- the connector certificate path in confirmation;
- duplicating stage 2's design onto `boundary_motion`.

**Interactions:** after stage 2 (S2 #1654, #1671) merges.

### 3.8 S7: friction with a one-way normal source, and k > 1 (M–L, both routes)

**Basis.**
- **The refusal to lift.** PP refuses a nonlinear normal source (`lib.rs:7955-7967`). Admit
  a translational one-way or lift-off support as the source. Its reaction is the normal when
  it is Active, and the normal is 0 when it is Inactive, so the friction support is then
  Inactive with no term.
- **The loop.** The source DOF lies in the active prescribed set, so R_n = K_int,n·u − f_int,n
  is the same formula as for a base restraint (`kd5-nonlinear-active-set.md:96-102`).
- **The certificate.**
  - Today a source outside the base fails closed by name (`:33`). That changes to: the
    source's own active-set certificate (strict reaction sign) comes first; then β = sign(N⁰)
    must agree with it.
  - If the source is Inactive, its separation must be certified strictly positive.
  - A closed gap as the source waits for R-3. Rotational sources stay refused.
- **k > 1.** Many sliding resting shoes are the realistic case. Today the case fails closed
  (`:34`). The open item names Rump's verified linear system for the k×k coupling
  (`:457`), and P's factor reuse makes the k + 1 solves affordable.

**References.**
- **F-2a.** F-1 with the UX restraint replaced by a one-way stop resisting +x. At p = +10 MPa
  the stop is active and the result is identical to F-1.
- **F-2b.** The same with p = −1 MPa (vacuum).
  - The stop lifts off and N = 0.
  - u_x(L) = −243/9,500,000 m.
  - With F_z = 20 kN, u_z = 27·20,000/(3·1,719,500π) m (33.321 mm), entirely in the
    cantilever.
- **F-3 (k = 2).** Two shoes on one span with closed-form two-DOF coupling.

**Size: M–L.** Risks:
- Rump's enclosure;
- non-uniqueness with friction, which needs the R-2 disclosure;
- loop convergence with coupled normals.

**Interactions.** OPN folded this into R-4. It is shared NI and PP code, so it lands for PF
too.

### 3.9 S8: two-sided stops on one DOF (M, with R-3)

**Basis.**
- **Why a pair works.** Two gap records on one DOF with clearances g− and g+ (g− + g+ > 0)
  cannot both be closed at one u. An engaged + stop holds u = +g+ while the − stop needs
  u ≤ −g−.
- **What changes.** The loop's repeated-boundary error (NI `lib.rs:1924-1930`) is reached
  only by a zero-clearance pair; refuse that by name and point to a rigid restraint. The
  confirmation's shared-DOF refusal (`confirmation.rs:183-189`) becomes: two gap supports may
  share a DOF when at most one is closed, and the open one's separation is certified
  strictly.
- **Format.** No format change: the MBF `LS` with ±DV maps to the pair. A single
  `limit_stop` record is the alternative, which needs 0.10.0.
- **Certification.** Gaps are unconfirmed until R-3, so S8 is useful only with or after G2.

**References.**
- **TS-1.** G-1 geometry with a two-sided axial stop: g+ = 0.5 mm closes at p = +10 MPa (as
  G-1); g− = 0.05 mm closes at p = −1 MPa (as G-3); g− = 0.1 mm stays open at −1 MPa.

**Size: M.** Risks: R-3's certificate shape.

### 3.10 Interactions with slices 1a and 1b, §4 and R-3

| Item | Interaction with R-4 |
|---|---|
| **1a** (PF operand standing in W-16b's EX shape) | EX already publishes operand `numerical_quality` (`exact-combinations.md:254-262`). S5 adds nonlinear standings (confirmed, not established, gap-unqualified) to the same record, so 1a fixes the shape S5 fills. **S5 builds after 1a.** S1–S4 do not depend on it. |
| **1b** (`difference_basis`, G-1, G-2) | EX refuses the member until S5. Then G-1 must cover pressure regions by content and G-2 binds resolved stiffness. On load-state documents G-2 withholds most declared O−S (§3.5); a decision is needed. |
| **§4** (per-group combination certificate, B_D = RU(B_O + B_S + exact subtraction error)) | It is route-agnostic and first serves W-16b's linear EX combinations. Two points are specific to pressure. (i) Hoop and radial rows depend only on p and the bore, so with equal pressures O−S is exactly 0 and floor-excluded ("not assessed", OPN §4.2 item 6). (ii) **Wall force N_w carries pA_i in both operands.** Its B_D carries both full-magnitude bounds against a small difference, so the group can fail on cancellation. **Proposal:** when the two operands' pressure inputs are bit-identical (G-1 already compares them), certify the derived axial group on N_el, where D(N_w) = D(N_el) exactly, and state it. This also needs the separate wall bound published per group (`pressure-contract.md:20`). Nonlinear EX operands bring their Stage-B bounds once S5 lands. Gap operands need R-3. |
| **R-3** (G2 gap certification) | It is NI-shared, so it lands for both routes at once. Pressure terms are ordinary ledger terms for G2. Gaps on EX are admitted in S5 with R1's unqualified standing (R-5: publish with standing). S8 and the closed-gap normal source in S7 wait for R-3. Building P before R-3 avoids pricing G2's loop sites twice. |
| **"Friction with a one-way normal source" and "two-sided stops"** | S7 and S8 |

---

## 4. Hydrostatic test on EX (analysis side; W-17 owns authoring)

### 4.1 Today

- Hydrotest pressure as a primitive is refused on **both** routes by shared validation
  (`validation.rs:61-70`) and, on EX, by `EXACT_PRESSURE_REQUIRES_REGION`
  (`pressure_runtime.rs:386-391`). The brief's "PF blocks hydrotest pressure at
  validation.rs:62-66" is correct as far as it goes, but the block is not PF-specific.
- `hydrotest` remains a primitive load category (PP `lib.rs:15049`), so a hydrotest-category
  weight load solves as an ordinary load.
- Nothing models "springs held".

### 4.2 Principle

The solver never infers mechanics from a case label: "case labels never imply temperatures,
fit or support state" (`resolve.rs:216-217`), and `load_cases[].kind` is not read by the
solve (OPN §1.1). A flagged test condition is therefore an **authoring template** that writes
explicit records (W-17), and the solver admits those records. `kind: "hydrotest"` stays
descriptive. Rule packs map it through their load-case mappings (W-17 rule-pack format 0.5.0,
`COMPLETENESS_MAP.md:527`).

### 4.3 What the solver needs from the condition set and the case

| Need | Carrier | Solver change |
|---|---|---|
| Test pressure | `pressure_regions[].pressure`, or `condition_use.pressure: "HYDRO"` once 0.9.0 S4 lands (`model-document-0.9.0.md:128-142`, `:364-375`) | None. Regions are case-scoped already. The primitive refusal's message should name the region form. |
| Test contents (water weight) | Weight primitives in the test case: the self-weight operation at the test condition's `contents_density`, written as line loads | None for weight. The weight is part of the case's ledger; on load-state documents it is entered through `load_sources`, as S4 already requires for generated primitives (`:371-372`). Per-case solver mass (`mass_state_ref`, refused at `resolve.rs:262-265`) is **not** needed, because hydrotest is static and has no seismic mass. The condition's `contents_density` consumer (OC-7, reserved, `model-document-0.9.0.md:135`, `:377-382`) is the self-weight generator, which is W-17's. |
| Springs held | `analysis_state.support_states[]` with `participation: {kind: locked_equivalent_support, components: [{dof, position_source: {kind: entered, value}}]}` (`input.rs:350-363`) | **Admit `locked_equivalent_support`** with `entered` positions for springs, variable-spring hangers and (after S2) constant-effort supports. Lift `LOAD_STATE_SUPPORT_PARTICIPATION_UNSUPPORTED` for that kind only (`resolve.rs:296-302`). |
| Ambient temperature | `element_states[].operating_temperature` at the installation temperature, or no thermal source | None |
| Nonlinear supports (resting shoes, gap guides) | as authored | Need S5. Water weight usually closes resting supports. |

### 4.4 How "springs held" is modelled

**Rule.** A locked spring's DOF becomes a prescribed DOF at the entered absolute position
(relative to the reference configuration). It goes through the same partitioned reduction as
`boundary_motion` (`resolve.rs:1040-1085`; `imposed_motion.rs:7-11`), and its reaction comes
from the unreduced equations.

- **Springs.** The spring stiffness still sits on the diagonal. At a prescribed DOF it only
  adds k·u_C to that DOF's reaction, which is 0 when u_C = 0. The whole reaction is
  attributed to the locked support.
- **Constant-effort supports.** A locked constant-effort support applies no constant force in
  that case (S2's device-action rule) and becomes the same prescribed DOF.
- **The convention.** Locking at u = 0 means locking at the as-modelled installed geometry.
  This is how travel stops pinned at the cold setting are normally represented.
  `position_source: predecessor_value` (locked where the sustained case left the DOF) stays
  refused, because it needs case history ordering (`exact_admission.rs:119`). Agent 0 chooses
  under its engineering-judgment delegation (§6).
- **What is not locked.** Rigid and nonlinear supports keep their own behaviour. `inactive`
  stays refused unless a later need appears.
- **K-D5.** Prescribed values are inputs u_C (`kd5-nonlinear-active-set.md:144`). No new
  closure class is needed.
- **Evidence.** The case's `load_state_evidence` lists each locked support, its DOF and its
  entered position. The support rows say "locked" in their basis.

### 4.5 Machinery it reuses

- case-scoped pressure regions and S1 junctions;
- dual-basis Lamé and axial rows (the hoop at test pressure is the governing quantity a user
  checks);
- the load-state group and its partitioned prescribed reduction;
- K-D5 with prescribed inputs;
- W-16b combinations, which are seldom used for hydrotest;
- the shared support-attribution rule.

### 4.6 One coupling to decide

Lockable supports exist only on load-state documents. A load-state document refuses
`equivalent_static` (`exact_admission.rs:128-134`). As built, **one document cannot hold both
a hydrotest case and a seismic case**, and v1.0 names both. There are two ways out:
- **(H-a)** Give generated equivalent-static loads a ledger class in `load_sources`, the
  mechanism 0.9.0 S4 already designs for generated primitives, and lift the refusal.
- **(H-b)** Add a per-case support-participation slot to plain EX documents, which needs a
  0.10.0 step.

H-a is recommended: no new format, and it removes an asymmetry (§6).

### 4.7 References

- **H-1, locked spring.** Two equal spans a = 6 m (S1 header section) with a locked spring at
  the middle support. The structure is a continuous beam, so the reactions are 3/8·wa,
  10/8·wa and 3/8·wa.
  - The test weight is w = g·(ρ_s·A_s + ρ_w·A_i) = 902,800,199/4,000,000·π N/m
    (709.058 N/m; ρ_s = 7,850, ρ_w = 1,000, g = 9.80665).
  - The middle reaction is **2,708,400,597/1,600,000·π N** (5,317.932 N).
- **H-1b, contrast.** The same with the spring active (k = 2·10⁶ N/m):
  R_mid = δ₀/(f + 1/k) = 4,946.79 N, with δ₀ = 5w(2a)⁴/(384EI) and f = (2a)³/(48EI).
- **H-2, test pressure.** Lamé hoop at the bore at p_t = 15 MPa:
  p(r_o² + r_i²)/(r_o² − r_i²) = **2,715,000,000/19 Pa** (142.895 MPa) on the nominal
  section. The reduced section uses its own bore and wall.
- **H-3, constant-effort.** A locked constant-effort hanger: no constant force, a prescribed
  DOF, and the reaction equal to the rigid-support reaction.

### 4.8 Size and stage

**Size: M.** It covers:
- the participation kind;
- constant-effort device-action rules;
- evidence and readers;
- references;
- H-a, about S–M on its own.

**Stage S4.** It comes after S1 (most test systems have tees) and S2. It is independent of the
nonlinear stages for linear-support models, and models with resting supports wait for S5.

---

## 5. Performance and memory of EX on large models

Target (proposed for v1.0, not in the PRD): a 2,000-node model, linear ≤ 10 s, nonlinear
≤ 60 s, on the 16 GiB base machine. The PRD's resource clause covers only retained-precision
invocations (M = 12 GiB, `PRD.md:796`). The retained domain D1 is capped at n = m = g = 32 and
excludes components, pressure and nonlinear supports (`retained-pricing-rules.md:65-77`). It
therefore **does not constrain a 2,000-node ordinary EX solve**: such a model is declined from
D1 and publishes on the ordinary route.

### 5.1 Known

| Fact | Source |
|---|---|
| Linear route, sparse mode: sparse assembly per basis, no dense matrix held across cases | PP `lib.rs:3043-3056` |
| Dense mode is refused above 8,192 DOFs (1,365 nodes) by a 96 B/entry estimate against 6 GiB | PP `lib.rs:4321-4332`, `:3057-3068`; `tests/f1b_guard_ceiling.rs:1-14` |
| K6, sparse, checked linear entry (kernel only): CHAIN 1,000 members 0.078 s, 10,000 members 0.371 s; TREE 10,000 members 0.377 s; GRID 1,984 members 0.198 s, 8,064 members 1.10 s. Peak RSS ≤ 0.75 GB at 10,000 members. | K6 `runs[].stages.entry_checked` |
| K6, dense: CHAIN 1,000 members, entry 94.5 s (factor 79.6 s, prepare 1.18 s), heap 3.48 GB (≈ 96.5 B/entry), RSS 5.0 GB; CHAIN 1,364 members, entry 181 s, RSS 7.09 GB; TREE 1,000 members timed out at 1,800 s | K6 |
| The nonlinear loop assembles dense K, builds a dense reduced copy, and runs the dense `prepare_bound` with dense contribution audits in **both** modes. Coupling solves are forced to `DenseScrutiny`. | §3.6 citations |
| Nonlinear confirmation costs 1.2–1.7 ms per case on the demo; end-to-end demo 115 ms, against 32 ms before | `kd5-nonlinear-active-set.md:444-453` |
| Linear certified publication: realistic set median 7.49 ms per case-mode; D1-cap models (32 members) 55–66 ms; about 756 µs straight recovery per self-weight member | `kd5-member-relative-bound.md:524-531` |
| EX rows per case: P_final = 7n + 114m + 8g + 3 | `retained-pricing-rules.md:1545` |
| A committed EX result averages about 650 B per serialized row (965 B per row including evidence) | `fixtures/persistence/recorded_model_0_7_0/pressure_connected_mechanics_sparse.json`, copied to `scratch/solver-r4/sample_ex.json` and measured by `estimates.py` (321 rows, 309,765 B) |

### 5.2 Estimated (arithmetic in `estimates.py`)

N = 2,000 nodes, n = 12,000 DOFs, n² = 1.44·10⁸.

**Linear EX, per case:**
- Kernel ≈ 0.125 s (K6 CHAIN log-log slope 0.677 between 1,000 and 10,000 members); a grid
  is about 0.2 s.
- Certified publication ≈ 1.5–4.1 s. This range comes from 0.756 ms per member and from
  66 ms per 32 members.
- Rows ≈ 244,000 per case (m = 2,000, g = 250), or 159–235 MB serialized per case.
- **With 8 cases:** about 13–34 s of compute and 1.3–1.9 GB of JSON.

So **linear ≤ 10 s is not assured** for a whole multi-case run. It is plausible per case, or
for 2–3 cases. Publication and output volume dominate, not the solve.

**Nonlinear, either route, as built:**
- Memory per iteration ≈ 104 B·n² = **15.0 GB**: F1b's 96 B (K view, symmetry views, prepared
  matrix and two `Expansion` arrays) plus the reduced copy. That exceeds M = 12 GiB and
  practically fills the 16 GiB machine.
- Memory ceilings at 104 B per entry:
  - 6 GiB: N ≈ 1,311;
  - 12 GiB: N ≈ 1,855;
  - 16 GiB physical: N ≈ 2,142, with no headroom.
- Time per iteration: the dense prepare alone is ≈ 4.7 s (1.184 s × 4).
- A dense factorization ≈ 635 s (79.55 s × 8). Every derived-normal sliding support forces
  one per coupling solve per iteration, so derived-normal friction at 2,000 nodes is
  **hours**.

**Nonlinear after P:**
- Per iteration ≈ the sparse kernel (0.1–0.2 s) plus O(nnz) work.
- With 5–15 iterations, one confirmation and the certified publication (1.5–4 s), a case takes
  about 3–7 s; **8 cases take about 25–55 s**.
- So ≤ 60 s is plausible but tight, and output volume is as for linear.

### 5.3 Unknown

- End-to-end PP time and peak RSS for any EX model above D1 size. No record exists; K6 is
  kernel-only.
- Active-set iteration counts on realistic racks with hundreds of nonlinear supports.
- The in-memory size of `ResultItem` rows, whose `ResultMetadata` strings are owned
  (PP `lib.rs:2385-2402`), and the aggregation's clones when combinations exist
  (`retained-pricing-rules.md:323`).
- The desktop's cost to parse and hold 1–2 GB of results.
- Base-machine CPU speed relative to K6's M5 Max.

### 5.4 Measurement that would settle it (not run; the manager asks Agent 0 first)

- **M-1.** Release `openpipestress-runner solve` in sparse mode on a generated, invented
  2,000-node EX rack.
  - The model has branches by S1 junctions (or, before S1, by three regions per tee), about
    250 linear supports, pressure regions, and 8 cases (sustained, operating ×2, hydrotest
    with locked springs, seismic ±X/±Z).
  - Record wall time and peak RSS under `t3_exclusive.sh`, split into solve, certified
    publication and serialization.
  - It needs one new generated fixture and no product change.
- **M-2.** The PF twin of M-1 with 200 one-way and 50 explicit-normal friction supports, run
  under a hard memory cap (expected: about 15 GB, refused or killed). This confirms §5.2's
  nonlinear wall before P is scheduled.
- **M-3.** A K6 extension with a nonlinear-loop stage, after P.

### 5.5 Verdict

**Linear.** EX plausibly meets the 2,000-node linear target per case. A multi-case run may
exceed 10 s because of publication volume, which is common to both routes.

**Nonlinear.** EX **cannot** meet the nonlinear target today, and neither can PF. The
obstacle is NI's dense loop, which both routes share.

**The PF question.** Adding pressure to PF would not help. The only reason to revisit it
would be an EX-specific cost, and the K-D5 confirmation (1.2–1.7 ms per case) and the
pressure terms are not one. **EX plausibly meets both targets after stage P**, subject to
M-1.

---

## 6. Recommendation

### 6.1 Order and first slice

1. **S1 tees and reducers (M). First build slice.** It touches `pressure_runtime`,
   `exact_admission`, the EX intensified rows and the pressure readers, not NI or the
   nonlinear PP sections that 1a, 1b, §4 and stage 2 occupy. Real pressurized systems cannot
   be built without it. Fold the `junction` token into 0.9.0 S1 if that is still open.
2. **S2 constant-effort supports (S)** and **S3 imposed displacement on plain EX (S).** Both
   are small and complete import parity for linear models.
3. **S4 hydrotest, linear (M)**, with H-a (equivalent-static on load-state documents) so that
   seismic and hydrotest share a document.
4. **The interim nonlinear resource guard (S, both routes)**, any time. It corrects a
   failure that has no named refusal today.
5. After **1a, 1b and stage 2 merge**: **S5 nonlinear supports on EX (L).**
6. **P, the nonlinear scale repair (L)**, right after S5, or before it if Agent 0 wants one
   pricing pass over the loop. **§4's certificate** is built in its own slot, independent of
   R-4, ahead of R-3 (OPN §0).
7. **S6 connectors and load-state nonlinear (M).**
8. **R-3 (G2)**, then **S7 one-way normal and k > 1 (M–L)** and **S8 two-sided stops (M).**

This is ordered by value per size, with dependencies respected:
- S1–S4 deliver sustained, hydrotest and seismic results for pressurized linear-support
  systems without waiting on the nonlinear queue;
- S5 is the largest single value, operating cases with resting shoes;
- P gates the size target;
- S7 and S8 depend on R-3.

### 6.2 Total size

**XL**: about 9–10 build slices, each with targeted regression checks, independent scrutiny
(numerical meaning) and a pricing pass.

### 6.3 Choices

**For Agent 0 (delegated):**
- **D-1. Junction representation:** junction terminals (recommended; reuses chain regions and
  manifests) or tree regions (one region per line system; larger traversal change).
- **D-2. Constant-effort supports on load-state documents:** a device action of an active
  support (recommended; enables hydrotest locking) or a refusal by name.
- **D-3. Hydrotest lock position:** `entered` 0, locked at the installed geometry
  (recommended for v1.0), or `predecessor_value`, which needs case ordering.
- **D-4. Seismic with hydrotest:** H-a (recommended) or H-b (0.10.0).
- **D-5. G-2 on EX load-state documents**, where per-member E differs by case.
  - Withhold declared O−S and keep the plain subtraction (consistent with 1b as decided).
  - Or let the declared basis compare the stiffness-defining inputs the user chose.
  - Either way, it is OPN §5's meaning question.
- **D-6. §4 on pressurized operands:** certify derived axial groups on N_el when pressure
  inputs are bit-identical (recommended), or accept that wall-force groups are seldom
  established.
- **D-7. Non-nonlinear R-4 stages ahead of 1a and 1b**, departing from "R-4 staged after 1a
  and 1b" (OPN §0) for S1–S4 only.

**Reserved for the owner, with consequences:**
- **O-1. Bourdon opening and pressure stiffening (T4-U9)**, an owner choice by
  `pressure-contract.md:13`.
  - If excluded for v1.0, pressurized bends' operating results omit bend opening. The
    exclusion is disclosed on every bend result and in the v3 limitations.
  - If included, it is one further fixed load term per arc and one more S5-style stage
    (M–L, with its own references).
- **O-2. The performance target's model-size clause.** Until P lands and M-1 confirms it,
  nonlinear models above about 1,300 nodes (6 GiB) to 1,850 nodes (12 GiB) cannot be
  solved on either route.
  - Accepting a lower nonlinear size for v1.0 is a risk acceptance and a change to the exit
    criteria, so it is the owner's.
  - Otherwise P is on v1.0's critical path.
- **O-3. Native-smoke `expected.json`.** S5 and P may change demo outputs if the demo moves to
  EX, and those values are owner-reserved.

### 6.4 Blocks and open dependencies

- Stage 2 (#1654, #1671) is open. S5, S6 and P touch the same NI and PP lines.
- The serial pricing queue: OPN §0 Order, §8.27–§8.30.
- 0.9.0 S1 (DEL-02-01) is still design only. The S1 token rides on it, otherwise 0.10.0.
- Desktop region authoring for junctions is cross-lane (W-17 and DEL-07).
- M-1 and M-2 need Agent 0's approval as heavy runs.
- Nothing blocks writing S1's build brief now.
