# Imposed displacement with nonlinear supports (stage 2)

**Status.** Stage 2 is built: R-a and R-b (§9), R-d (§10) and R-c (§11). In a model with nonlinear
supports, the pressure-free route carries a stage-1 primitive on a rigid
anchor or restraint into the nonlinear active-set loop and its K-D5
confirmation as a base prescribed value (R-a), and a primitive on a nonlinear
support's own DOF moves that support's ground (R-b): its state is measured on
u_d − g_s, an active or sticking state prescribes the moved ground, and the
support publishes a relative-displacement observation row.
`IMPOSED_DISPLACEMENT_WITH_NONLINEAR_SUPPORTS` is retired.

**The former open items** (Agent 0, under the owner's delegation of
2026-10-27). R-d, a rigid restraint on a mixed nonlinear record moving, is
built (§10). R-c, a linear spring's moving base, is built (§11). Only a
constant-effort record stays refused as a target
(`IMPOSED_DISPLACEMENT_SUPPORT_NOT_RIGID`).

This design extends imposed displacement on the pressure-free route (PF) to
models with nonlinear supports: gap, one-way, lift-off and friction. It is
stage 2 of the owner's ruling, 2026-10-23, verbatim: "Row 2 I approve option
A".

The two stages are described in DEL-04-07 `Work/pressure-contract.md`.
Stage 1 is `Work/pf-imposed-displacement.md` (W-16 slice a, #1615). Agent 0
set the scope on 2026-10-25: R-a and R-b, both in stage 2, kept separable in
design, benchmarks and review. R-b may land as a later slice.

Paths are relative to `projects/chirality-piping/`. PP is
`core/product_physics/src`, NI is `core/solver/nonlinear_integration/src`, NS
is `core/solver/nonlinear_supports/src` and FK is
`core/solver/frame_kernel/src`. Line numbers are at `a7f7be6020`.

## 0. Two separable parts

**R-a: moving rigid restraints.** Stage-1 primitives on rigid anchors and
restraints give nonzero base values g_B. Every nonlinear support's ground stays
at 0, so states are measured on the absolute u_d, as today.

**R-b: moving nonlinear-support ground.** A primitive on a nonlinear support's
own DOF moves its ground by g_s. Its state is measured on δ_s = u_d − g_s. An
active one-way, lift-off or sticking state prescribes g_s. An active gap
prescribes p_s = g_s + σ·c, where σ = +1 for "closes in +", −1 for "closes
in −", and c is the clearance. Stage 1 refuses this target
(`imposed_motion.rs` ~144-149).

**Size.** Functionally, R-a changes the affine base boundary. Its inventory
also includes callers, about 43 test literals, verification text and pricing
pins. R-b changes support-relative laws and publication, and has the larger
contract surface: normalization, admission, exact-gap operands, certificate
comparisons, publication precision, schemas and readers. R-b can land after
R-a without changing it. R-a alone keeps every nonlinear ground fixed, keeps
the R-b target refusal, and adds no relative row.

## 1. The nonlinear loop (NI)

1. **Input.** `NonlinearFrameSolveInput` (~239-254) gains
   `base_prescribed: Vec<(usize, f64)>` (R-a) and
   `ground_motion: Vec<(String, f64)>` (R-b). Both are sparse; an absent entry
   is 0. `validate_input` (~1190-1198) refuses an entry that:
   - lies off `base_restrained_dofs`;
   - names an unknown support;
   - is a duplicate;
   - is not finite;
   - is an R-b gap entry whose closing position c + σ·g_s is not finite and
     exact in binary64 (zero TwoSum residual). This is the same invariant as
     §3.2, enforced at the loop's public boundary so a direct caller cannot
     bypass the product refusal.

   The product binds the whole input by equality (~890), so no new binding
   code is needed. About 43 test literals gain the new fields.
2. **`active_boundary`** (~1884-1938). Base DOFs take g_b, or 0 when absent,
   instead of `0.0` (~1890-1894). `prescribed_displacement_for_state`
   (~1940-1965) takes the support's g_s:
   - Sticking, and Active one-way or lift-off: g_s instead of 0.
   - Active gap: p_s instead of ±c.

   Both loop callers (~625-630, ~669-674) and the qualification (~1026-1031)
   pass the new data.
3. **Trial states.** `build_trial_states` (~2357-2384) builds the trial on
   fl(u_d − g_s) only when a ground offset exists, so the no-motion path is
   unchanged. The NS classifier (~581-676) is unchanged; it reads the trial
   displacement.

   **Signs.** For finite binary64 operands, with gradual underflow, a finite
   fl(a − b) has the exact sign of a − b and is zero only when a = b.
   Unilateral states and slip signs are therefore decided exactly. Overflow is
   refused by name as a non-finite value.

   **Gap comparisons are not exact.** For example, take u = nextDown(0.5),
   g_s = −0.5 and c = 1. The exact u − g_s is 1 − 2^−54 < c, but
   fl(u − g_s) = c, so the rounded classifier closes an exactly open gap.

   **The exact strict-gap proof** decides whenever it qualifies (~734-737),
   because it forms σ(u − g_s) − c exactly. It is available only for gap-only
   qualified straight-frame bodies with supported sources and bounded exact
   work (`structural_adapter.rs` ~2843-2869, ~2992-3045). In mixed, curved or
   exhausted cases, a rounded comparison can change an inspection branch.
   That branch is never certified: R1 keeps every gap case unqualified.

   The qualification's reclassification uses the same builder (~1034-1040,
   ~1076-1081).
4. **Sliding friction.** The slip direction is taken on u_t − g_t
   (~1684-1687, ~1771-1779). `resolve_sliding_friction_states` (~1613-1650)
   reads the trial, so it becomes relative through item 3.

   The derived-normal coupling is unchanged. The base and unit solves
   (~1383-1429) share one boundary, which now carries g. The base reactions
   include the motion. The influence, unit − base, stays the homogeneous unit
   response.
5. **Reported reactions.** These (~1732-1742, ~2070-2074) are unchanged in
   form. u carries g on the constrained set C, so K·u includes K_·C·g. At an
   active R-b support with a parallel spring on a separate record, the
   reaction includes k·(g_s − g_k), with g_k that spring's base motion (0
   unless the case moves it, §11).
6. **Strict-gap proof** (`structural_adapter.rs` ~2819-2987).
   - R-a needs no change: the exact system takes the boundary map.
   - R-b passes σ·p_s = c + σ·g_s as the clearance to the gap proof
     (~2955-2961). FK forms the penetration exactly (`exact_boundary.rs`
     ~1897-1899) and requires only finiteness, so FK needs no change when
     c + σ·g_s is exact in binary64 (§3.2).
   - Every downstream use of the contact and decision gap (~2571, ~2726) must
     accept a negative effective clearance. This is to be confirmed in review.
7. **PP plumbing.** `append_nonlinear_support_loop_results` (PP `lib.rs`
   ~6918-6929) receives the case's base motion from the PF map (~5006-5009)
   and the R-b ground map, and fills both input fields (~6963-6979).
   `imposed_motion::case_prescribed` (~204-244) separates R-b entries from
   rigid DOFs. Rigid prescribed values and nonlinear-ground offsets add no
   load term. R-c adds each nonzero spring-base term k·g; the loop force
   equals the complete ledger bit for bit.

## 2. The K-D5 certificate

These are revision-2 changes to DEL-04-01 `kd5-nonlinear-active-set.md`.

- **§1.1 notation.**
  - B carries the values g_B, each 0 unless authored.
  - Each support s has a ground offset g_s, 0 unless authored.
  - A has p_s = g_s for one-way, lift-off and sticking states, and
    p_s = g_s + σc for a gap (gaps are not run under R1).
  - u_C = (g_B, p_A) and δ_s = u_{d_s} − g_s.
  - For μ = 0, dir_s is the sign of δ_t.
- **§1.2.** A linear restraint is prescribed g_b. An Active one-way or
  lift-off support has u_d = g_s. A Sticking support has u_t = g_t.
- **§1.3 intended system.** Its form is unchanged. u_C carries the motion, and
  R_n includes K_int,nC·u_C. K-D5's residual subtracts K·u over the full u.
- **§1.4 Stage A.** The identity R_n(ũ(φ)) = R_n(ũ(φ⁰)) + g·(φ − φ⁰) holds,
  because the difference of the two states satisfies zero prescribed values.
  The A-2 influence system stays at zero prescribed values
  (`confirmation.rs` ~583-590). An influence system that carries g_C is a
  mutant and is tested as one.
- **§1.6 load closure.** "Prescribed values 0" becomes "prescribed values
  u_C = (g_B, p_A)". R-a, R-b and R-d add no ledger term; R-c adds the
  spring-base term defined in §11.1.
- **§2.2 strict conditions.** Every reaction, friction-normal and stick-margin
  condition is unchanged. These are shifted:

| Support, state | Condition |
|---|---|
| One-way or lift-off, sense +, Inactive | lower(u_d) > g_s |
| One-way or lift-off, sense −, Inactive | upper(u_d) < g_s |
| Gap, Inactive (G2 only), closes in + | upper(u_d) < g_s + c |
| Gap, Inactive (G2 only), closes in − | lower(u_d) > g_s − c |
| Sliding, dir = +1 | lower(u_t) > g_t |
| Sliding, dir = −1 | upper(u_t) < g_t |
| Sliding, μ = 0 | as above, with dir = sign δ_t |

  g_s is an exact binary64 threshold, so the comparisons stay exact. The code
  sites are `confirmation.rs` ~685-689 (unilateral), ~666-674 (slip
  established), ~849 (μ = 0 direction) and ~249 (the manifest direction, now
  the sign of fl(u − g_t)). §2.3's claim is restated on δ_s.
- **§2.4 fail closed.** Item 2 keeps its existing bit-for-bit check of the
  loop's u against the state's prescribed map (~143-152, ~360-364), and gains
  a second check under the same reason name. The confirmation builds the
  intended map independently, g_B from `base_prescribed` and p_s from
  `ground_motion` and the final states, and compares it bit for bit with the
  loop's active prescribed values. Today only the qualification checks the
  boundary, and it reuses `active_boundary`, so a fault there would pass both.
  The comparison covers complete canonical maps, including default zeros,
  coverage, duplicates and bits, not only the supplied nonzero entries. It
  closes the boundary-consistency obligation. It does not independently check
  PP's primitive-to-map resolution, which the stage-1 refusals and route-parity
  tests cover. Item 3 is unchanged.

- **K-D5 §9's M04 equivalence statement** (~423) holds only at zero motion. It
  is restated as such. Under R-b, B-1's spring twin makes M04 observable.
- **R1 is unchanged.** A case carrying a gap never runs the confirmation,
  with or without motion (~127-134). It keeps
  `NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED`. No gap state is
  certified.

## 3. Refusals

### 3.1 Unchanged

These refusals stay as they are:
- the rotational gap and rotational friction refusals;
- `NONLINEAR_COMBINATION_REQUIRES_SOLVE`;
- `EXACT_IMPOSED_DISPLACEMENT_PRIMITIVE_UNSUPPORTED`, because EX also refuses
  nonlinear supports;
- retained D1.6 and D1.7;
- every stage-1 code.

`IMPOSED_DISPLACEMENT_WITH_NONLINEAR_SUPPORTS` is retired. If R-a lands
alone, `IMPOSED_DISPLACEMENT_SUPPORT_NOT_RIGID` keeps refusing R-b targets.

### 3.2 New for R-b

The order is: dimensional checks first, then each operand normalized
independently by the same path the solve uses (PP `lib.rs` ~9265, ~9704), then
these checks on the canonical binary64 values:
- `IMPOSED_DISPLACEMENT_NONLINEAR_DOF_MISMATCH` applies when a nonlinear
  record's target DOF is neither its nonlinear DOF, its admitted spring DOF
  nor one of its rigid restraints.
- `IMPOSED_DISPLACEMENT_GAP_CLOSURE_INEXACT`: the normalized c + σ·g_s is not
  finite, or has a nonzero TwoSum residual. A rounded closing position is never
  used silently. Checking authored magnitudes would be wrong: 1 m + 1 in is
  exact as authored numbers, but 1.0 + 0.0254 is inexact in SI binary64.
- `IMPOSED_DISPLACEMENT_COINCIDENT_RESTRAINT`: extended to another nonlinear
  support acting on the same node DOF.
- A spring and nonlinear law may share a record only on disjoint DOFs (§10.1).
  A primitive on the nonlinear DOF moves only that ground, and a primitive on
  the spring DOF moves only the spring's base (R-c, §11).

Friction tangential motion needs no refusal. Neither does a moving
derived-normal source, which is R-a on a rigid restraint, carried by R_n.
Rigid restraints cannot overlap a nonlinear DOF
(`SUPPORT_LINEAR_NONLINEAR_DOF_CONFLICT`).

### 3.3 The former open items

- **Linear springs with a moving base (R-c).** The force k(u − g) adds a
  formed load k·g. That needs a new ledger term, a K-D5 closure term in FK and
  the same change on the linear route, so it is not a prescribed value and
  does not fall out of this formulation. Built (§11).
- **The rigid restraints on a mixed nonlinear record (R-d).** Built (§10).

## 4. Publication

- **Support rows.**
  - Linear supports: Stage B's certified rows, which include the motion's
    reactions.
  - Nonlinear slot rows: kd5 §3.1, unchanged. A certified reaction for
    Active or Sticking, φ for Sliding, and +0 for Inactive.
- **Nodal displacements.** Absolute. A moved anchor publishes g exactly.
- **`nonlinear_support_final_displacement`.** Stays absolute in the global
  frame, with its kind and bytes unchanged.
- **New row for R-b.** Each R-b support also publishes a new row,
  `nonlinear_support_final_relative_displacement`. It is an observation row,
  not a corrected or certified quantity:
  - **Value.** fl(U − g_s) in SI, then the declared unit conversion. U is the
    support DOF's published absolute final displacement in SI. It is taken
    from the actual `published_nodal_displacements` vector (PP `lib.rs`
    ~5563-5579, ~14264-14287), including that helper's whole-vector fallback
    to the selected loop values when any corrected component cannot be formed
    finitely. A selected exact-gap projection is used where one exists. A
    blocked or non-converged case publishes no row.
  - **What it does not claim.** It does not prove a strict sign. A strictly
    separated exact state may round to zero. Certified state claims stay with
    the state rows and the K-D5 evidence.
  - **Signatures.** Translations in mm (length), rotations in rad (angle),
    following the existing two families (`semantic_contract_v0_3_preview_physics_2.json`
    ~611, ~851). Each carries support identity, component, location and the
    global sign convention.
  - **When it appears.** Only when g_s is authored, including an authored
    zero, so no existing output moves.
  - **Classification.** `never_combined`, and not eligible for governing
    ratios.
  - **Registration.** In both preview tables (the retained table inherits;
    `semantic_contract_tables.py` ~279-295), with their counts, vocabulary and
    hashes; in the schemas; in result export (`preview_physics_evidence.rs`
    ~575-585); and in the Rust and Python attribution readers.
  - **Standing.** A certificate failure keeps the loop value. A later recovery
    demotion keeps the corrected value, because the bound already existed (PP
    `lib.rs` ~14255-14274). Gap rows stay inspection evidence.

  The K-D5 evidence line gains one clause stating the row's observation
  status.
- **Standing.** `NUMERICAL_INTEGRITY_*` standing follows kd5 §3.1 and §2.4.
  Gap cases keep `RECOVERY_BASIS_UNQUALIFIED`.

## 5. Verification

The independent references go in `validation/references/w16_nonlinear_motion/`.
They are exact-rational, generated without running product code, following
`kd5_nonlinear/reference.py`. Models are axis-aligned and Euler–Bernoulli, with
power-of-two lengths and dyadic motions. Both solver modes are run. The
criterion is the K-D5 ball bound, or 1e-9 relative for gap projections.

**R-a**
- **A-1.** A clamped cantilever with a midspan load P and a tip one-way prop.
  The anchor rises by s. The prop reaction is R = 5P/16 − 3EI·s/L³, so lift-off
  occurs at s* = 5PL³/(48EI). Cases:
  - s = s*/2: Active, R = 5P/32;
  - s = 2s*: Inactive, u_tip = s*;
  - s = s*: fails closed. At this exact boundary the loop's binary64
    classifier alternates between the two weak states and does not converge,
    so the case is withheld before the confirmation
    (`NONLINEAR_SUPPORT_NONCONVERGENCE`, `SOLVER_SYSTEM_BLOCKED`; §9);
  - an anchor-rotation twin.
- **A-2.** The cantilever of A-1, with a downward load P at the tip.
  - The derived-normal source is a rigid tip UY restraint at signed upward
    position s, which gives R_n = P + k_b·s with k_b = 3EI/L³. A downward
    settlement d > 0 is s = −d, so R_n = P − k_b·d.
  - A tip friction support acts on UX under an uncoupled tangential load T.
    Sticking requires |T| < μ|R_n|.
  - Cases: strictly sticking, strictly sliding, and the weak equality. The
    weak case fails closed ("stick capacity not established").
- **A-3.** An invented planar L-frame. Member 1 runs along x from anchor A to
  corner C, and member 2 along y from C to tip T.
  - Friction acts on the tip UX, with its derived normal from a rigid tip UY
    restraint whose ground settles by g_n. The frame couples UX to the source
    reaction, so the influence γ is nonzero.
  - The generator eliminates the reduced system exactly. With
    K = [[a, b], [b, d]] on (n, t) and u_n = g_n:
    α = (a − b²/d)·g_n + (b/d)·f_t − f_n, γ = b/d,
    φ = c_f·α/(1 − c_f·γ), u_t = (f_t + φ − b·g_n)/d, with
    c_f = −dir·μ·β.
  - It checks this identity against a full direct-stiffness solve. An
    independent Euler–Bernoulli L-frame, with both members at L = EA = EI = 1 and the tip rotation free, gives
    K_(n,t) = (1/103)·[[84, 18], [18, 48]] and γ = 3/8. The committed fixture
    pins the lengths, the normalized section and material leaves, the loads,
    the settlement, μ, and an admissible dir and β. Materializing it checks
    that the branch is strictly sliding and that the denominator is nonzero. An
    influence system that carries g_n gives a different γ and violates Coulomb
    equilibrium, which kills that mutant. So does a Stage A that omits motion.

**R-b**
- **B-1.** A tip lift-off support whose ground drops by d.
  - Active: R = P − 3EI·d/L³.
  - Inactive beyond d* = PL³/(3EI), with u = −d* and separation d − d*. A
    classifier on absolute u, or a threshold of 0, fails this case.
  - Spring twin (a separate record): R = P − (k + 3EI/L³)·d. This kills
    K-D5's mutant M04.
  - Rotational twin, using the condensed rotational tip stiffness, not k_b.
- **B-2.** A gap closing in −, whose ground rises by h, with
  δ0 = PL³/(3EI) < c. It closes if and only if h ≥ c − δ0.
  - Closed: u = h − c and R = P + (3EI/L³)(h − c).
  - Cases: open, closed, and the equality (R = 0, which the classifier admits
    as Active). Also both closing senses and a negative effective clearance.
    Standing is `RECOVERY_BASIS_UNQUALIFIED`, with the exact gap decisions
    checked.
- **B-3.** Friction under ground motion g_t, with g* = μ|P_y|L/(EA).
  - Strictly sticking for |g_t| < g*, with u_x = g_t and R_t = EA·g_t/L.
  - Strictly sliding for |g_t| > g*, with u_x = sign(g_t)·g*. The force
    opposes the relative slip.
  - At |g_t| = g*, the relative slip is zero and the reaction equals the
    capacity. This is a weak sticking boundary, the classifier chooses
    Sticking, and the certificate is not established.
  - Twins with an explicit normal, and with P_x ≠ 0. With P_x ≠ 0 the sticking
    interval is (P_x − μN)/k_x ≤ g_t ≤ (P_x + μN)/k_x, with k_x = EA/L. Both
    signs of g_t are run, to kill an absolute-slip mutant.

**Property.** Apply a compatible rigid translation by one dyadic t to the
translational DOFs, chosen so that every translated input stays exactly
representable, moving every relevant ground (anchors and
nonlinear grounds) with it. States and reactions are then unchanged and u
shifts by t. Every spring base on the translated axis moves with it (R-c,
§11). A spring whose base is held while everything else moves is the B-1
spring twin, not an instance of the property.

**Exact boundaries.** Each boundary case is an exact boundary of the system
formed from the normalized binary64 leaves. A rounded analytical threshold is
not used.

**Further controls.** Negative motions, both support senses, authored zero
motion, cancellation, and an overflow case refused as non-finite.

**Negatives.** Every §3 refusal, including a mixed document refused whole. EX
listed and unlisted primitives. A combination with a motion case. Each §2.4
item-2 mismatch.

**Byte identity.** Every model without motion keeps all committed outputs: the
eight Passed case-modes, the kd5 references and G-1/NGR02. Only declared text
and pins change.

The design is reviewed twice. Two reviewers each review it independently,
neither seeing the other's findings, before any code.

## 6. Text changes

- **DEL-04-07 `pressure-contract.md`.** The stage-2 sentence is replaced in
  this design's PR (§8).
- **K-D5 §0 M1 and §0's "prescribed 0".** Replaced at implementation by the
  revision-2 text of §2. M1 becomes: "The loop prescribes each base-restrained
  DOF at its case value g_b, which is 0 unless a pressure-free case authors a
  support-targeted primitive on a rigid restraint (R-a). A primitive on a
  nonlinear support's own DOF moves that support's ground by g_s (R-b): its
  state is measured on u_d − g_s and an active or sticking state prescribes
  g_s (g_s ± c for a gap). The confirmation checks the published u against the
  state's prescribed map, and that map against the case's motion, bit for bit
  (§2.4)."
- **ScopeOfWork, through the owner's wording grant at implementation.**
  - DEL-04-04 ScopeOfWork ~156-159 currently reads "released supports classify
    on trial displacement penetration (or the explicit gap clearance)". The
    insert ", each measured from the support's ground position, which an
    imposed support displacement moves" is proposed for R-b.
  - DEL-04-03 ScopeOfWork ~294 gains "including in models with nonlinear
    supports (stage 2)".
  - DEL-04-01's ScopeOfWork needs no change. Its ~88 covers "imposed
    conditions supplied by other deliverables".
- **Completeness map.** The FR-SOL-006, §12.5, item-6, W-16 and §22.2 rows.
  These are evidence of implementation, not acceptance.

## 7. Retained-memory pricing (D3)

- **Call graph.** Both NI and NS are in the static call graph
  (`tools/retained_pricing/base/callgraph_g5.py` ~343-352). The loop cores are
  reached only through `append_nonlinear_support_loop_results`, which returns
  early without nonlinear supports (D1.6), so they are priced at multiplicity
  0. NS constructors are also reached through `build_nonlinear_supports` (PP
  `lib.rs` ~7785, ~8311).
- **Validation runs on admitted calls.** Retained admission runs before
  ordinary validation (PP `lib.rs` ~2578). Declined calls enter ordinary
  dispatch, and admitted retained calls also reach the shared validator. New
  validation branches and allocations are therefore priced, even though
  motion-bearing models are outside D1.
- **Static owners.** Both preview tables are positive static owners
  (`b7a_domain_model.py` ~119). The new row's signatures grow them.
- **Pins that move.**
  - `w1_reductions_model.py`: the PP `lib.rs` and `structural_adapter.rs`
    pins.
  - `kd5_text_model.py`: the `solve_prepared_load_case` pin.
  - `data/basis_text_rows.json`: the NI and NS line-position rows,
    regenerated.
  - `o10_rules.py`, if the confirmation gains reason text.
- **New refusal texts.** Each needs a class.
- **Inventory.** The pins above are a starting inventory, not a complete
  registration. Shared validation, the semantic tables and the evidence-line
  text get their actual classes and pins at pricing.
- **Basis.** Stage 2 re-prices on W-16a's registration (DEL-04-08
  `retained-pricing-rules.md` §8.23). There, W16-NS zeroes the support-target
  paths behind D1.7's refusal of every support-targeted primitive, and W16-NI
  prices the per-case node-index table. Stage-2 work inside those guarded
  branches is carried by W16-NS's zeros. Work outside them, such as the shared
  validation above, is priced positively. If stage 2 changes the admission,
  W16-NS's guards and their mutation tests (`tests/test_w16_imposed_motion.py`)
  stop the run.
- **Sequence.** Re-pricing and re-registration, with independent review, come
  in the solver lane's next D3 turn after #1625 and #1630 (W-16b/c, then
  stage 2).

## 8. The pressure-contract stage-2 sentence

In DEL-04-07 `Work/pressure-contract.md`, the stage-2 bullet becomes:

> **Stage 2:** imposed motion in a model with nonlinear supports, in
> separable parts. (R-a) Rigid anchors and restraints carry stage-1 primitives
> as nonzero base prescribed values in the nonlinear active-set loop and its
> K-D5 confirmation; every nonlinear support's ground stays fixed. (R-b) A
> primitive on a nonlinear support's own DOF moves that support's ground: gap
> closure, lift-off and friction slip are measured on the displacement relative
> to the moved ground, and an active or sticking state prescribes the moved
> ground (plus or minus the clearance for a gap). (R-c) A primitive on a
> linear spring's DOF moves its base; the case gains k·g and the spring
> publishes −k·(u−g). (R-d) A primitive on a mixed nonlinear
> record's rigid restraint prescribes that restraint's base value. Design:
> DEL-04-03 `Work/pf-nonlinear-imposed-motion.md`,
> with dual numerical review and its own benchmarks; R-b may land as a later
> slice.

## 9. Implementation

- **Code.** NI `NonlinearFrameSolveInput::base_prescribed`, checked by
  `validate_input` (on the base set, once, finite) and taken by
  `active_boundary`. NI `confirmation.rs`: `intended_prescribed_map` and
  `prescribed_map_mismatch` (§2.4 item 2). PP
  `append_nonlinear_support_loop_results` receives the case's PF map; PP
  `imposed_motion.rs` no longer refuses motion in a nonlinear model.
- **References.** `validation/references/w16_nonlinear_motion/` (README,
  generator, requests, `references.json`, `SHA256SUMS`). The invented
  material E = 5·2^35 Pa and the section OD 0.206 m, wall 0.011 m make every
  boundary case an exact boundary of the system formed from the normalized
  binary64 leaves. The PP consumer is
  `core/product_physics/src/w16_nonlinear_motion_tests.rs`; NI's are in
  `confirmation_tests.rs`.
- **The A-1 boundary (a reading, not a change).** "Fails closed" is met by
  the loop: at s = s* the classifier alternates between Active and Inactive
  on rounding noise, the loop reaches its iteration cap, and the case is
  withheld before any confirmation. Nothing is certified or published. A-2's
  weak boundary converges and fails closed in the confirmation.
- **The property under R-a.** Only directions on which no nonlinear support
  acts can move with R-a alone: A-2 translated by t = 2^-8 m in Y keeps its
  states and reactions exactly, and its displacements shift by t. The full
  property, moving nonlinear grounds, needs R-b.
- **A-3's mutants.** The generator records φ for an influence system that
  carries g_n and for a Stage A that omits the motion; the published φ lies
  within its bound of the exact value and outside it for both.

**R-b.**

- **Code.** NI `NonlinearFrameSolveInput::ground_motion`, checked by
  `validate_input` (a known support, once, finite; a gap's closing position
  c + σ·g_s finite and exact by TwoSum, `exact_gap_closing_position`);
  `active_boundary` and `prescribed_displacement_for_state` prescribe g_s
  (σ·(c + σ·g_s) for an active gap); `build_trial_states` and the slip
  direction take fl(u − g_s) when a ground entry exists; `scrutinize_gaps`
  passes c + σ·g_s to the exact gap proof. A support without a ground entry
  keeps its fixed-ground values bit for bit (a closing-in-− gap still
  prescribes −c, so a zero clearance keeps −0). NI `confirmation.rs`: the
  intended map carries p_s; §2.2's inactive unilateral and slip conditions
  compare with g_s; the manifest and μ = 0 directions are the sign of
  fl(u − g_t). Friction in state Active prescribes its ground as sticking
  does (the loop never reaches it; the confirmation refuses it).
- **PP.** `imposed_motion.rs`: the R-b checks (§3.2) after the dimensional
  check, on the operands normalized by the solve's own path;
  `case_ground_motion` feeds the loop, and `case_prescribed` no longer sees
  nonlinear targets. R-d rigid targets enter the prescribed map; admitted
  spring targets enter the spring-base map; other non-acting DOFs retain
  `IMPOSED_DISPLACEMENT_NONLINEAR_DOF_MISMATCH`.
- **Publication (a reading).** `nonlinear_support_final_relative_displacement`
  rows are appended after the published displacements are formed, one per
  ground entry. The K-D5 evidence clause is appended only to a case that
  publishes such a row, so no existing output moves.
- **Contract.** `semantic_contract_tables.py apply-w16` adds the two
  signatures (supported-source-080, mm; -081, rad) after the W-7 rows of
  preview-physics-2; apply-u4b carries them to preview-physics-retained-1; the
  table digests are repinned by the tool. The Rust and Python readers and the
  desktop's legacy dimension map list the kind.
- **References.** B-1 (with its spring and rotational twins), B-2 (both
  senses, the equality, a cancelling and a negative effective clearance),
  B-3 (both signs, an explicit normal, P_x ≠ 0) and the property on B-1, B-2
  and B-3 (every anchor and nonlinear ground on the axis moved by
  t = 2^-8 m). Unlike A-1's, B-1's exact boundary converges, on either weak
  state by solver mode, and fails closed in the confirmation ("separation not
  established" or "contact force sign not established"). B-3's boundary is the
  weak sticking boundary ("stick capacity not established"). Every B-2 case
  takes the exact strict-gap decisions and stays
  `NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED`; the equality is Active.

## 10. R-d: the rigid restraints of a mixed nonlinear record

Agent 0 adopted R-d with R-c on 2026-10-27, under the owner's delegation. R-c,
a linear spring's moving base, is built (§11). Both are pressure-free
only. The exact route keeps refusing every support-targeted primitive, and its
`boundary_motion` law stays rigid-only.

### 10.1 One action per target DOF

A support record acts on each DOF in at most one way, decided per DOF:

- **nonlinear:** the record's `nonlinear.dof`. A primitive moves that
  support's ground g_s (R-b).
- **spring:** the stiffness axis of a `spring` or `variable_spring_hanger`
  record, chosen as PP `build_model` chooses it (the stiffness DOF, else the
  first restraint). A primitive moves the spring's base g_k (R-c, §11).
- **rigid:** every other DOF in `restraints` of a record that is neither a
  spring family nor a constant-effort device. This includes a mixed nonlinear
  record. A primitive prescribes the base value g_b (stage 1, R-a, R-d).
- Any other DOF is not an action of the record.

A primitive moves the base of the one action it names and never another DOF
of the record.

Constant-effort records remain refused as motion targets, including records
carrying a nonlinear field. Validation and all three case resolvers classify
the target DOF: only rigid targets enter `case_prescribed`, only nonlinear
targets enter `case_ground_motion`, and only spring targets enter the
spring-base map.

A spring and nonlinear law may share a record only on disjoint DOFs, using a
supported spring family. Same-DOF spring/nonlinear action requires separate
records. Existing stiffness-family, spring-axis and linear/nonlinear-overlap
refusals remain unchanged. A record carrying spring stiffness cannot also
carry additional rigid restraints.

`build_model` assembles a nonlinear record's rigid restraints according to its
family token. Under family `nonlinear` they form a guide, which holds
translations only. A rotational restraint on such a record is therefore
refused (`SUPPORT_INPUT_INVALID`), and this design does not change that.

### 10.2 Mechanics and admission

**Mechanics.** A mixed nonlinear record has `restraints` on DOFs other than
its nonlinear DOF. PP `build_model` admits those restraints as a rigid linear
support, so they are base-restrained DOFs of the loop. A primitive on one of
them is an R-a base value g_b. It reaches the loop's `base_prescribed`, the
K-D5 confirmation's intended map (§2.4) and the published displacement, bit
for bit, and nothing else changes.

Such a record still cannot be a derived friction normal source
(`NONLINEAR_FRICTION_NORMAL_SOURCE_INVALID`). Its restraints still cannot
overlap its nonlinear DOF (`SUPPORT_LINEAR_NONLINEAR_DOF_CONFLICT`).

It used to be refused only because stage 1's record-level rigid test counted
the whole record as non-rigid. That test also kept its restraints out of the
coincident-restraint count, so a coincidence there fell to the generic
`SUPPORT_INPUT_INVALID`.

**Admission (pressure-free).**

| Code | R-d |
|---|---|
| `IMPOSED_DISPLACEMENT_SUPPORT_NOT_RIGID` | Retired for a mixed record's rigid restraints (R-d) and for a spring or hanger axis (R-c, §11). It still covers a constant-effort record (a nonlinear field included) on any DOF. |
| `IMPOSED_DISPLACEMENT_NONLINEAR_DOF_MISMATCH` | as §3.2 |
| `IMPOSED_DISPLACEMENT_COINCIDENT_RESTRAINT` | A rigid target counts every rigid restraint on its node DOF, a mixed record's included, and is named ahead of the generic repeated-DOF refusal. A nonlinear target keeps R-b's check. |
| `IMPOSED_DISPLACEMENT_DOF_DUPLICATE` | Duplicates are checked per base: (node, DOF) for rigid targets and (support, DOF) for spring and nonlinear targets. |
| every other stage-1 and R-b code; `EXACT_IMPOSED_DISPLACEMENT_PRIMITIVE_UNSUPPORTED`; retained D1.7 | unchanged |

### 10.3 Verification

The independent references extend `validation/references/w16_nonlinear_motion/`.
The generator, which runs no product code, now reads a mixed record's rigid
restraints as rigid DOFs, and routes a primitive by its target DOF.

- **D-1.** A cantilever runs along +X: 4 m, with a midspan node. Its tip mixed
  record (family `nonlinear`) has a rotational lift-off on RY, in contact for
  a positive moment, and rigid UY and UZ restraints. The tip's global UZ base
  displacement is −s. The load is −P on UZ at midspan.
  - The condensed tip coupling K[RY,UZ] is +6EI_y/L², so the active support
    moment is R = PL/8 − 6EI_y·s/L² and s* = PL³/(48EI_y).
  - At s*/2, R = PL/16.
  - At 2s*, the support is Inactive, R = 0 and RY = 3(s − s*)/(2L) =
    3s*/(2L).
  - At s*, R = 0 is an exact weak boundary and must not receive a strict
    certificate.
  - A record whose rigid UZ stayed fixed would carry PL/8, so the motion is
    observable.
  - Deviation (accepted by Agent 0, 2026-10-27): rigid restraints on a
    nonlinear record assemble as a guide, so D-1 uses a rotational lift-off
    with a moving UZ base instead of a one-way UZ prop with a rotating rigid
    RY.
- **D-3.** D-1's record without UY. Its own ground rotates by global −g (R-b)
  and its rigid UZ moves by −s (R-d).
  - The active moment is R = PL/8 − 6EI_y·s/L² − 4EI_y·g/L.
  - Cases: g alone, s alone, and both, which gives R = PL/16 at s = s*/4 and
    g = 3s*/32. Then s = s* with g: Inactive, R = 0, RY = 0 and separation g.
  - Each motion present is observable.
- **The property.** D-1 translated rigidly in Y, moving the anchor's UY and
  the mixed record's UY by t = 2^-8 m.
- **Parity.** D-1's UY and UZ moved to a separate guide record carrying the
  same primitive give every displacement and the support moment bit for bit.
- **Negatives:**
  - a non-acting DOF of the mixed record (`NONLINEAR_DOF_MISMATCH`);
  - two primitives on one rigid restraint, with equal values (`DOF_DUPLICATE`);
  - the mixed record's UZ coincident with a second rigid UZ, targeting either
    record (`COINCIDENT_RESTRAINT`, not the generic refusal);
  - a constant-effort record carrying a nonlinear field (`SUPPORT_NOT_RIGID`);
  - At the R-d layer, a spring-family nonlinear record's spring axis was
    refused with `SUPPORT_NOT_RIGID`; R-c retires this refusal (§11).
  - a restraint overlapping the nonlinear DOF
    (`SUPPORT_LINEAR_NONLINEAR_DOF_CONFLICT`, unchanged).
- **Byte identity.**
  - Previously admitted cases containing no newly admitted primitive retain
    their numerical output bytes. Declared refusal-text changes, newly named
    coincident-restraint diagnostics and pricing pins are excluded.
  - An authored zero on a mixed record's rigid restraint gives every row and
    diagnostic of the no-motion case.
  - The references for earlier requests, their 74 cross-checks among them,
    are unchanged.

Observed on the branch at `18db0ca3c5`, both solver modes:
- D-1 and D-3: every certified group within B; the nonlinear slot,
  mixed-record component and displacement errors are 0, apart from one
  displacement at 2.9e-47 relative in D-3 Inactive (an exact-zero component).
- D-1's boundary: the loop converges on either weak state and the
  confirmation fails closed ("separation not established" or "contact force
  sign not established").
- 96 cross-checks agree.

### 10.4 Retained-memory pricing (D3)

No semantic-table, schema or reader content change is planned. D3 still
reprices and re-registers the actual implementing source. Review every
changed source, owner and lifetime, including the case resolvers and
recovery-map plumbing; refresh all affected source/function/site bindings
and regenerate displaced basis-text rows. The inventory below is a starting
list, not a complete registration.

- **New work behind W16-NS's zeros** (a support-targeted primitive, which
  D1.7 refuses): the per-DOF classifier (`acting`, `target_acting`) in
  validation and in both case resolvers, and the shared rigid-base checks.
- **Changed source:** PP `imposed_motion.rs` only. PP `lib.rs`, NI and FK are
  unchanged by R-d.
- **Pins that bind `imposed_motion.rs`:**
  - `w16_owners_model.py` `PIN`;
  - the W16 sites in `o10_rules.py`;
  - `linemap_resolutions.json`;
  - `sq2_decisions_E.py`;
  - `ordinary_caps.py`.
  On the branch the W16 pricing guard reports the change ("imposed_motion.rs
  changed; re-review the owners it prices"). That is left for D3.

## 11. R-c: a linear spring with a moving base

### 11.1 Mechanics

A spring of stiffness k between DOF d and a base displaced by g acts on the
pipe with −k(u_d − g). The global system gains the load k·g at d:

  K·u = f + k·g·e_d,

with K unchanged; k stays on K's diagonal.

**Where the term goes.**
- **Linear route.** The term joins the case force before the partitioned
  reduction, so a spring at a free DOF gives
  K_ff·u_f = f_f + k·g·e_f − K_fc·g_c. A spring at a rigidly prescribed DOF
  changes only that DOF's reaction. Superposition holds, so linear
  combinations take spring-motion cases under their existing gates.
- **With nonlinear supports.** The term reaches the loop through the same
  force. The loop's force must equal the ledger bit for bit (NI ~555-572), so
  the loop, its qualification and its strict-gap proof need no change.
- **A spring sharing a DOF with a nonlinear support** keeps its own base. That
  spring must be on a separate record (§10.1). An active support's reaction
  then includes k·(g_s − g_k).

R-c covers every linear spring primitive: `spring` and
`variable_spring_hanger` records, translational and rotational. Variable
spring hangers use the same incremental spring law. Installed, cold and hot
loads, travel ranges and movement limits remain authored review data; base
motion neither recalculates them nor establishes preload, available travel or
suitability.

**Current limitation (§8.30 correction 3).** A variable spring hanger's
installed, cold and hot loads are not applied as preload. Results treat the
hanger as a linear spring from zero load. Each non-zero entered load gets a
warning, `SPRING_HANGER_LOAD_NOT_APPLIED_AS_PRELOAD` (PP `validation.rs`), per
support and field. The Diagnostics panel, the results standing line and the
report carry it. RT-S3 and RT-A11 (hanger sizing) lift this limitation.

**Decided: a variable spring carries its cold load** (Agent 0, 2026-10-31,
D-3, under the engineering-judgment and scope delegations of
`loop/LOOP_INIT.md`).
- **The decision.** A variable spring hanger is modelled as its stiffness plus
  its `cold_load`. The cold load is applied as a constant force along the
  stiffness DOF in every load case, so the spring carries F_cold − k·u.
- **The reason.** That is the normative model of a set spring. Without it, a
  hanger sized and applied under DEL-04-11 cannot reproduce its own table in
  the model's cases.
- **How it is built.** The force enters through the constant-effort seam and
  its combination gate. `SPRING_HANGER_LOAD_NOT_APPLIED_AS_PRELOAD` is retired,
  and a warning is added where `installed_load` and `cold_load` differ.
- **Effect.** Documents that carry `cold_load` solve differently afterwards.
- **Until then.** The limitation above stands until DEL-04-11's increment 6
  builds this (DEL-04-11 `Work/hanger-sizing.md` §11, ScopeOfWork REQ-014).
  This deliverable's commitment text changes with that increment.

**The ledger term.** One formed term is added per moved spring, at d, with
value fl(k·g):
- Formation `RoundedProduct{k: 1, a: k, b: g}`;
- summand `NodalProduct{global_dof: d, magnitude: k, factor: g}`, under the
  identity (`spring_base_motion`, support id, 0).

K-D5 re-forms it as the product of two binary64 operands at p, as it does
the source-factor product, so FK gains no new load formula. `NodalProduct`'s
documentation widens to "a product of two binary64 operands at one nodal DOF".

**Zero.** When g compares equal to zero, of either sign, omit the ledger term
and use the existing fixed-base arithmetic in ordinary, certified and W2
recovery. Do not subtract the authored zero. Pin both zero signs with bit
comparisons.

**K-D5.**
- On the linear route the closure covers the term like any formed term.
- The certified row adds the exact product +k·g to the accumulator containing
  −k·u, −k·w and −k·w₂; its evaluation scale includes |k·g|. No binary64 fold
  of k·g or u−g is used. If implemented through `evaluate_row`, supply −k·g as
  its load argument, because that helper subtracts the argument. FK
  `certify_spring_reaction` takes g.
- The nonlinear confirmation evaluates Stage A and Stage B on the ledger
  force with the term, so R_n, contact reactions and slot rows include it with
  no other change.

**The force-scaled route (W2, K2b).** FK `force_scaled_spring_action` forms
the exact sum −(k·2^b)u + (k·2^b)g, rounds that scaled action once, then uses
the existing checked unscale for publication. When g compares equal to zero,
the fixed-base arithmetic is retained. A nonzero spring-base term reaching
product W2 admission is refused as `non_nodal_load_term`, unless an earlier
gate has already refused the case. This preserves the existing F1b subset.
Moving-base recovery is covered at FK level; no product W2 moving-base solve
is claimed.

### 11.2 Publication

- The spring's support rows (`support_reaction_component_v2` and its
  magnitudes) carry −k·(u − g). They are certified where the member bound
  exists. On the ordinary route they are the binary64 −k·(u − g), and the
  fixed-base expression is kept when g is zero.
- When a spring shares a DOF with a rigid restraint, the existing
  attribution rule withholds both records' signed rows. Displacements and
  certified groups are still compared.
- No row kind is added, and the sign convention text is unchanged. The
  spring's relative displacement is −row/k and is not published.
- R-b's `nonlinear_support_final_relative_displacement` stays a
  nonlinear-support row, because its readers bind it to nonlinear support
  identities.
- The semantic tables, schemas and readers are unchanged.
- **Units.** g is normalized like any motion primitive (m or rad). k·g is in
  N for a translational spring and N·m for a rotational one.

### 11.3 Admission

`IMPOSED_DISPLACEMENT_SUPPORT_NOT_RIGID` is retired for a spring or hanger
axis, including a spring-family nonlinear record's spring DOF. Duplicates on a
spring are keyed (support, DOF), so two springs on one node DOF may each move.
No coincident-restraint refusal applies between parallel springs: each keeps
its own base. Every other code is unchanged (§10.2).

### 11.4 Verification

The generator gains the spring base load k·g and a spring's support row
−k(u − g).

- **C-1, the axial spring.**
  - A bar of length L runs from an anchor. A tip spring on UX of stiffness
    k_s has a base that moves by g, and k_m = EA/L.
  - Then u = k_s·g/(k_s + k_m), the spring row is k_s·k_m·g/(k_s + k_m), and
    the anchor reaction is −k_m·u. With k_s = k_m, u = g/2.
  - A torsional twin (RX, k_m = GJ/L) and a variable-spring-hanger twin are
    run. The hanger twin's rows equal the spring twin's bit for bit. The RX
    twin covers rotational spring motion.
- **C-2, a loaded cantilever.** A tip load −P and a tip spring on UZ with
  k_s = k_b = 3EI/L³, whose global UZ base displacement is g=−d, give
  u = −(P + k_s·d)/(k_s + k_b). Both signs of d are run.
- **C-3, parallel springs.** Two springs k₁, k₂ on one DOF with bases g₁, g₂
  equal one spring k₁ + k₂ with base (k₁g₁ + k₂g₂)/(k₁ + k₂). The operands are
  chosen so that k₁+k₂, both products, their sum and the equivalent base are
  all exactly representable.
- **C-4, force parity.** A spring-base case is run against the same model with
  the nodal force k·g (exact in binary64) instead. Both give the same
  displacement and member rows, bit for bit in both modes. The exact spring
  actions differ by k·g. Compare published spring actions against their exact
  references and certified bounds; assert an exact published difference only
  for fixtures whose individual actions are exactly representable.
  Certificate radii and evidence need not match the unfactored nodal-force
  twin.
- **C-5, with nonlinear supports.** B-1's spring twin with the spring base
  moving with the lift-off ground (both by −d) gives R = P − k_b·d. The
  property's B-1 instance then moves the spring base too.
- **D-2.**
  - D-3's record carries no UY. Add a separate UY spring at that tip, with
    k = 3EI_z/L³ and base motion g; then u_y = g/2.
  - The twin uses one spring-family record with nonlinear RY and spring UY,
    and keeps the rigid UZ restraint on another record.
- In both modes, test unequal spring/nonlinear grounds, a moved spring
  sharing a rigid prescribed DOF, and a moved spring affecting a derived
  friction-normal reaction. Test disjoint same-record motions independently;
  retain overlap and constant-effort-with-nonlinear-field refusals. Include an
  inexact k·g product and closure mutants for missing or wrongly bound
  operands.
- **W2.** FK unit tests cover `force_scaled_spring_action` with a base and
  `certify_spring_reaction`'s exact addend, with both zero signs pinned.
  PP `f1b_a_spring_base_term_is_not_admitted` pins the W2 admission refusal
  (`non_nodal_load_term`) of a spring-base term.
- **Negatives:**
  - a spring record's non-axis DOF (`DOF_UNRESTRAINED`);
  - two primitives on one spring DOF (`DOF_DUPLICATE`);
  - the exact route's refusal, listed and unlisted;
  - retained D1.7;
  - a spring-motion combination: withheld in a nonlinear model, admitted and
    superposed in a linear one.
- **Byte identity.** Previously admitted cases containing no newly admitted
  primitive retain their numerical output bytes. Declared refusal-text
  changes, newly named coincident-restraint diagnostics and pricing pins are
  excluded.

### 11.5 Retained-memory pricing (D3)

No semantic-table, schema or reader content change is planned. D3 still
reprices and re-registers the actual implementing source. Review every
changed source, owner and lifetime, including the case resolvers and
recovery-map plumbing; refresh all affected source/function/site bindings
and regenerate displaced basis-text rows. The inventory below is a starting
list, not a complete registration.

- **New work behind W16-NS's zeros:** spring-target classification, matching
  and routing; one formed ledger term and its attached primitive per spring
  whose base motion compares unequal to zero.
- **New work outside the zeros:** for s prepared springs, each pressure-free
  case allocates a ground `Vec<f64>` and a moved `Vec<bool>`, each of length
  s, even without a spring target. The moved vector ends at resolver return;
  ground remains through recovery. Load-state cases allocate the length-s
  zero-ground vector directly. Each case also scans the prepared springs in
  `spring_base_motion`; zero grounds emit no terms.
- **FK:**
  - `certify_spring_reaction`: one more parameter and the exact addend;
  - its caller in PP `certified_publication.rs`;
  - `force_scaled_spring_action`: one more parameter;
  - its K2b caller in PP `lib.rs`;
  - the `NodalProduct` doc comment.
- **Pins that move:**
  - `kd5_text_model.PRODUCTION_PINS`, and the function hashes in
    `kd5_text_model.PINS` (`lib.rs` `solve_prepared_load_case`;
    `certified_publication.rs` `support_record` itself is unchanged);
  - `B7A_PRIMITIVE_LIB_SOURCE_PIN`, and `b7a_domain_model.py` `SOURCE_FILES`;
  - `kd5_first_error_model`'s CPP file and prefix, and its solve-body pins;
  - W1's certified-publication, certified-recovery and formation-check source
    pins (`w1_reductions_model.py`);
  - `w16_owners_model.py` `PIN`, and the W16 sites in `o10_rules.py`;
  - `data/basis_text_rows.json`, for the FK rows that move.
  - `batch5_model.PINS` for the changed case-solve body; carried
    `b7a_formation_model.FIRST_RETURN_*`, `b7a_returning_errors.SITES` and
    `b7a_owner_choices.PREIMAGES` helper-site bindings displaced by these
    edits. Preserve unchanged function hashes.
