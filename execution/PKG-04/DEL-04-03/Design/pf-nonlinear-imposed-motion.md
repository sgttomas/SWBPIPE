# Imposed displacement with nonlinear supports (stage 2)

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

**Size.** R-a touches four places: the loop input, `active_boundary`, the PP
plumbing and one confirmation check. R-b adds:
- a ground map;
- the ground offset in about six comparisons;
- three certificate thresholds;
- an exact gap-clearance check;
- admission checks;
- a published relative-displacement row.

R-b can land after R-a without changing it.

## 1. The nonlinear loop (NI)

1. **Input.** `NonlinearFrameSolveInput` (~239-254) gains
   `base_prescribed: Vec<(usize, f64)>` (R-a) and
   `ground_motion: Vec<(String, f64)>` (R-b). Both are sparse; an absent entry
   is 0. `validate_input` (~1190-1198) refuses an entry that:
   - lies off `base_restrained_dofs`;
   - names an unknown support;
   - is a duplicate;
   - is not finite.

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

   Rounding is safe. fl(a − b) has the exact sign of a − b and is zero only
   when a = b, so unilateral states and slip signs are decided exactly. The
   gap test fl(u − g_s) ≥ c is monotone, and differs from the exact test only
   within half an ulp below c. The exact strict-gap proof decides whenever it
   qualifies (~734-737). The qualification's reclassification uses the same
   builder (~1034-1040, ~1076-1081).
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
  Item 3 is unchanged.
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

These run before generic validation:
- `IMPOSED_DISPLACEMENT_NONLINEAR_DOF_MISMATCH`: the target DOF is not the
  record's `nonlinear.dof`.
- `IMPOSED_DISPLACEMENT_GAP_CLOSURE_INEXACT`: c + σ·g_s is not exact in
  binary64, as checked by TwoSum. A rounded closing position is never used
  silently.
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
  `nonlinear_support_final_relative_displacement`, with the value
  fl(u_d − g_s). The row appears only when g_s is authored, so no existing
  output moves. Its signature is registered:
  - in the preview-physics-2 table, with `combination_algebra`
    `never_combined`;
  - in its schema;
  - in result export (`preview_physics_evidence.rs` ~575-585);
  - in the Python mirror.

  The K-D5 evidence line gains one clause for it.
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
- **A-2.** A friction normal source settles by s, so R_n = P + 3EI·s/L³. Cases
  sit on either side of the stick/slide flip and at the flip.
- **A-3.** A strongly coupled derived normal with a settling source. This
  kills an influence system that carries g, and a Stage A that omits motion.

**R-b**
- **B-1.** A tip lift-off support whose ground drops by d.
  - Active: R = P − 3EI·d/L³.
  - Inactive beyond d* = PL³/(3EI), with u = −d* and separation d − d*. A
    classifier on absolute u, or a threshold of 0, fails this case.
  - Spring twin (a separate record): R = P − (k + 3EI/L³)·d. This kills
    K-D5's mutant M04.
  - Rotational twin.
- **B-2.** A gap closing in −, whose ground rises by h, with
  δ0 = PL³/(3EI) < c. It closes if and only if h ≥ c − δ0.
  - Closed: u = h − c and R = P + (3EI/L³)(h − c).
  - Cases: open and closed. Standing is `RECOVERY_BASIS_UNQUALIFIED`, with
    the exact gap decisions checked.
- **B-3.** Friction under ground motion g_t, with g* = μ|P_y|L/(EA).
  - Stick if and only if |g_t| < g*, with R_t = EA·g_t/L.
  - Otherwise slide, with u_x = sign(g_t)·g*.
  - Cases: g*/2, 2g* and g*. Twins with an explicit normal and with
    P_x ≠ 0.

**Property.** Translating every ground by one dyadic t leaves states and
reactions unchanged and shifts u by t.

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
  (`tools/retained_pricing/base/callgraph_g5.py` ~343-352). They are reached
  only through `append_nonlinear_support_loop_results`, which returns early
  without nonlinear supports (D1.6). Their loop cores are priced at
  multiplicity 0.
- **Pins that move.**
  - `w1_reductions_model.py`: the PP `lib.rs` and `structural_adapter.rs`
    pins.
  - `kd5_text_model.py`: the `solve_prepared_load_case` pin.
  - `data/basis_text_rows.json`: the NI and NS line-position rows,
    regenerated.
  - `o10_rules.py`, if the confirmation gains reason text.
- **New refusal texts.** Each needs a class.
- **To confirm at pricing:** whether `validate` runs on retained invocations
  before D1.7 declines.
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
