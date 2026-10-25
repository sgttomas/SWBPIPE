# Imposed displacement with nonlinear supports (stage 2)

**Status: a design, not current behaviour.** Until the stage-2 implementation
lands, the pressure-free route refuses any imposed motion in a model with
nonlinear supports by name (`IMPOSED_DISPLACEMENT_WITH_NONLINEAR_SUPPORTS`),
and refuses motion targeted at a nonlinear support
(`IMPOSED_DISPLACEMENT_SUPPORT_NOT_RIGID`). Throughout this document, "is",
"becomes" and "publishes" describe the designed stage 2.

**Open items**, outside this design:
- motion of a linear spring's base, which keeps its refusal (§3.3);
- motion of the rigid restraints on mixed nonlinear records (§3.3).

This design extends imposed displacement on the pressure-free route (PF) to
models with nonlinear supports: gap, one-way, lift-off and friction. It is
stage 2 of the owner's ruling, 2026-10-23, verbatim: "Row 2 I approve option
A".

The two stages are described in DEL-04-07 `Design/pressure-contract.md`.
Stage 1 is `Design/pf-imposed-displacement.md` (W-16 slice a, #1615). Agent 0
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
   reaction includes k·g_s, because that spring's ground stays fixed.
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
   rigid DOFs. Motion is never a force: the loop force still equals the
   ledger bit for bit (NI ~555-572).

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
  u_C = (g_B, p_A)". No ledger term is added.
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
- `IMPOSED_DISPLACEMENT_NONLINEAR_DOF_MISMATCH`: the target DOF is not the
  record's `nonlinear.dof`.
- `IMPOSED_DISPLACEMENT_GAP_CLOSURE_INEXACT`: the normalized c + σ·g_s is not
  finite, or has a nonzero TwoSum residual. A rounded closing position is never
  used silently. Checking authored magnitudes would be wrong: 1 m + 1 in is
  exact as authored numbers, but 1.0 + 0.0254 is inexact in SI binary64.
- `IMPOSED_DISPLACEMENT_COINCIDENT_RESTRAINT`: extended to another nonlinear
  support acting on the same node DOF.
- `IMPOSED_DISPLACEMENT_SUPPORT_NOT_RIGID` is kept for a spring on the same
  nonlinear record. Moving that support's ground would also move the spring's.

Friction tangential motion needs no refusal. Neither does a moving
derived-normal source, which is R-a on a rigid restraint, carried by R_n.
Rigid restraints cannot overlap a nonlinear DOF
(`SUPPORT_LINEAR_NONLINEAR_DOF_CONFLICT`).

### 3.3 Still refused, and open

- **Linear springs with a moving base.** The force k(u − g) adds a formed
  load k·g. That needs a new ledger term, a K-D5 closure term in FK and the
  same change on the linear route, so it is not a prescribed value and does
  not fall out of this formulation. `IMPOSED_DISPLACEMENT_SUPPORT_NOT_RIGID`
  stays. It is open for a later unit.
- **The rigid restraints on a mixed nonlinear record** (PP `lib.rs`
  ~8316-8321). They keep `IMPOSED_DISPLACEMENT_SUPPORT_NOT_RIGID` under R-a.
  This is also open.

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
  - s = s*: fails closed;
  - an anchor-rotation twin.
- **A-2.** The cantilever of A-1, with a downward load P at the tip.
  - The derived-normal source is a rigid tip UY restraint at signed upward
    position s, which gives R_n = P + k_b·s with k_b = 3EI/L³. A downward
    settlement d > 0 is s = −d, so R_n = P − k_b·d.
  - A tip friction support acts on UX under an uncoupled tangential load T.
    Sticking requires |T| < μ|R_n|.
  - Cases: strictly sticking, strictly sliding, and the weak equality. The
    weak case fails closed.
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
shifts by t. Models with a fixed-ground parallel spring are excluded, because
that spring's ground does not move under this scope.

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
- **Sequence.** Re-pricing and re-registration, with independent review,
  follow #1615's turn.

## 8. The pressure-contract stage-2 sentence

In DEL-04-07 `Design/pressure-contract.md`, the stage-2 bullet becomes:

> **Stage 2:** imposed motion in a model with nonlinear supports, in two
> separable parts. (R-a) Rigid anchors and restraints carry stage-1 primitives
> as nonzero base prescribed values in the nonlinear active-set loop and its
> K-D5 confirmation; every nonlinear support's ground stays fixed. (R-b) A
> primitive on a nonlinear support's own DOF moves that support's ground: gap
> closure, lift-off and friction slip are measured on the displacement relative
> to the moved ground, and an active or sticking state prescribes the moved
> ground (plus or minus the clearance for a gap). Linear springs with a moving
> base stay refused. Design: DEL-04-03 `Design/pf-nonlinear-imposed-motion.md`,
> with dual numerical review and its own benchmarks; R-b may land as a later
> slice.
