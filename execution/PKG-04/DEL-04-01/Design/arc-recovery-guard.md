# Arc recovery guard (O-13)

Requirement for T4's unit that extends S11-G's recovery guard R-b′ to realized arcs. It lands before T4-U2 merges. HELP_HUMAN moved its ownership from T3 to T4 on 2026-10-10. T3 sets the requirement and the review conditions below.

## R-b′ on the straight route

- **Where.** PP `core/product_physics/src/formation_guard.rs` holds S11-G's design (revision 2.1 with I5's rulings; revision 2.2 changed only the routing gate): `recovery_finding`, `rb_prime_fires`, `moment_scales`, `demote` and `amend_integrity_report`.
  - The per-end bound is straight_pipe `bending_formation_bound`, carried in PP's `RecoveryRecord`s.
  - The load-row guard's finding goes in at both `append_integrity_report` sites. R-b′ is one call after the straight element-recovery loop, which amends whichever integrity diagnostic was emitted (I5 ruling 1).
  - Tests: PP `core/product_physics/src/s11g_tests.rs` (`rb_prime_clauses_at_their_boundaries`, `t11_inplane_rows_are_demoted_by_rb_prime_alone`, `t12_…`, `t13_…`, `t13b_…`, `ruling1_both_guards_fire_and_the_load_row_sentence_stands`, `t20_characterization_rb_prime_residual_c1`) and straight_pipe's `t17_bending_formation_bound_on_a_skew_member`.
- **What it guards.** Some straight member ends publish bending rows formed from `K_e·u`. Those rows can carry formation noise above the 1e-9 criterion while the case's other integrity checks pass. After S11-F, R-b′ is the only check that catches the INPLANE formation rows, so its tests are load-bearing.
- **Predicate.** It is decided exactly, with `CRITERION = RD(1e-9)`. Let `q = hypot(My, Mz)` at the end and `B` be its formation bound. The end fires when all three hold:
  - `B > RD(1e-9)·q`;
  - `q > 2^10·B`;
  - `S*_moment ≥ 2^-988` and `q ≥ 2^-34·S*_moment`, where `S*_moment` is the body's coupled moment scale over the case's published rows, `max(S(moment), L_b·S(force))`. Below 2^-988 the floor product is inexact (DESIGN N-2), and an end with no body or scale has `S*_moment = 0`; in both cases the end does not fire.

  A non-finite or unavailable bound fires.
- **Consequence.** A fired case is demoted from `NUMERICAL_INTEGRITY_CHECKS_PASSED` to `NUMERICAL_INTEGRITY_SENSITIVE` by one sentence appended to its existing integrity diagnostic, naming at most six ends.
  - Nothing is refused, no value changes, and no field or code is added.
  - Any code already `SENSITIVE` or weaker is left byte for byte.
- **Gap.** Curved spans are outside R-b′:
  - `moment_scales` excludes them;
  - `recovery_finding` reads straight records only;
  - the load-row guard's ledger records `CannotBound` for uncertified curved consistent load vectors (SF-2), and that guard then fires.

  Under D-2 (only realized bends carry pressure; `pressure-contract.md`), realized arcs become the normal bend. Their end actions, formed from `K̃_c·u`, would then publish with no recovery guard.

## What the arc guard must establish

1. **A per-end formation bound** for curved spans, on published end actions formed from `K̃_c·u`.
   - It covers at least the bending pair, as R-b′ does. Cover axial, shear or torsion rows too if they can carry unguarded formation noise, and say whether they can.
   - It is conservative on the product's actual arithmetic: T4-U1's stable forms, the libm-free φ, the inversion of F and its pivot guard, and the global-frame transform, including the transform loss T4-I19 found on near-coordinate-plane chords (about 1e-11 of the row scale in `T^T K_L T`, removed by U1's global-frame assembly fix, which this bound must reflect as landed).
   - A bound that cannot be established is unavailable, and an unavailable bound fires.
2. **R-b′'s predicate and consequence,** with `S*_moment` extended to cover guarded arcs' rows and bodies, and the scale argument restated. The same demotion, sentence mechanism and no-op rule apply. The guard never refuses, and adds no field or code.
3. **Evidence against an exact reference,** using RV129's 120-digit tools or an exact-rational reference, over T4's arc fixtures and the I9/RV129 probe sets:
   - small angles down to 3e-9 rad on a 0.3 m chord;
   - near π;
   - 1°, 5° and 45°;
   - R/L extremes.

   For each arc end, report the true formation error, the bound and the verdict.
   - Every arc end that is a published Passed breach of the 1e-9 criterion, at or above the predicate's floor and resolved (`q > 2^10·B`), must fire. An arc end that breaches while unresolved or below the floor is reported with its numbers; whether the arc predicate may differ from R-b′'s there is decided by HELP_HUMAN, not by the implementer.
   - No correct arc row of a committed model may be demoted: every committed model with a realized arc (T4's fixtures, including T4-U2's first usable L-line model) is listed in the unit's evidence.
   - False demotions may occur only on synthetic shapes, and each one is listed.
   - If both conditions cannot hold, bring the numbers to HELP_HUMAN before implementation.
4. **The relation to SF-2's curved `CannotBound`:** kept, narrowed or replaced, with W1c's ownership stated. R-1 ([curved-numerical-integrity.md](curved-numerical-integrity.md), "Certified arc loads") already replaces it for certified curved consistent-load terms.
5. **Load-bearing tests with mutation kills** covering:
   - each predicate conjunct;
   - each bound component;
   - the scale extension;
   - the unavailable-bound path;
   - the no-op rule;
   - the load-row guard and the arc guard firing on the same case (the load-row sentence stands, as in I5 ruling 1);
   - a source pin tying the call site to the tested predicate.

   An arc Passed breach at the criterion must be caught by a test that fails when the guard is removed.
6. **No T3 regression:**
   - straight R-b′ and every straight pin are unchanged;
   - the retained route is unaffected, because curved spans are outside D1;
   - T3's TEXT chain (the memory-pricing chain behind `retained_memory.rs`'s GENERATED PROFILE) is re-run on the unit's head, and any new loop reachable from the retained route's D1 admission root needs a T3 rule;
   - I5 ruling 3's receipt residual cannot reach arcs, because captured and retained recovery refuse models with realized curved elements; the unit confirms this still holds;
   - SP-1 holds, with each declared demotion listed alongside its evidence.

## As implemented (T4-O13)

T4-O13 implements T4-I34's design (T4's scratch `t4_i34/DESIGN.md`) with HELP_HUMAN's rulings of 2026-10-10 on R-1, R-2 and O-1. This records what the code does; it is not an acceptance claim.

### The bound

- **Where.** FK `structural/arc_certificate.rs`: `certify_curved_recovery` (per arc, case-independent) and `CertifiedArcRecovery::bound_end_actions` (per end action). PP `arc_recovery_record` forms the record in the recovery loop's realized-arc branch from the same u, free expansions and uniform loads the recovery used.
- **What it bounds.** A ball (m, r) around the exact real f(z, u) = A_s[K(z)(u − Σũ) − Σp(z, w)] for each of the 12 published rows, where z is the held operands (the arc's `CurvedFormation`, the replaced span's nodes and y reference), u the product's binary64 displacements, ũ the held free-expansion vectors (thermal, and T4-U2's represented ε_p) and p the uniform loads' certified balls. It is evaluated through K·u = [−Hf_j; f_j], f_j = K_gΔ, Δ = (u_jt − u_it − θ_i × d; θ_j − θ_i), so rigid motion cancels exactly. The arithmetic is T4-U1b's ball arithmetic. The arc's F is formed by the same prefix as the load certificate, whose bits are unchanged. K_t = F⁻¹ uses L7′, the equilibrated verified inverse: with the exact power-of-two scaling D and F′ = DFD, each entry's radius is DᵢDₖsᵢcₖ/(1 − ρ′) rather than L7's uniform ‖X̃‖∞ρ/(1 − ρ). L7's radius alone exceeded the criterion 6–12 times on the 30 m chord at 2e-9 rad; L7′'s share is 2.5e-5 (FK `c2_row_bounds_enclose_and_stay_tight`).
- **The bound.** B_f(row) = |v − m| + r, rounded upward. It bounds |v − f(z, u)| whatever arithmetic produced v, so the stable F terms, the libm-free φ, the inversion and its pivot guard, the global-frame assembly (as landed by T4-I33) and the chord rotation are all covered by measurement rather than by a rounding model. For the bending pair, B_u = Σ_a|A_s|Σ_c|K(z)|(2⁻⁵³|u_c| + 2⁻¹⁰⁷⁴) is added: it bounds the change in f from any u* whose correct rounding is u, as R-b′'s γ₁₆|K||T||u| does on straight members.
- **Groups.** Every arc end row can carry formation noise, because no arc row has an exact path. Four groups are guarded per end: bending (q = hypot(My, Mz), B = B_f + B_u), torsion (|T|, B_f), axial (|N|, B_f) and shear (hypot(Vy, Vz), B_f).
- **Unavailable.** A failed precondition of the arc's certificate or of any uniform load's certificate makes the record's bound unavailable, and every group of that arc fires.

### Predicate and consequence

- **Predicate (R-2 adopted for arcs).** A group fires when R-b′'s `rb_prime_fires(q, B, S*)` fires, or when B_f > RD(1e-9)·q exactly and q ≥ 2⁻³⁴·S* (with S* ≥ 2⁻⁹⁸⁸). The second clause is HELP_HUMAN's R-2, a deliberate change to R-b′'s predicate for arcs: the formation part fires above the floor with no resolution conjunct, so a gross formation error at large k or on a sub-millimetre arc is no longer hidden in the unresolved band. The straight predicate is unchanged. A non-finite bound fires.
- **Scales.** S* is the body's coupled moment scale mo = max(S(moment), L_b·S(force)) for bending and torsion, and its coupled force scale fo = max(S(force), S(moment)/L_b) for axial and shear (DESIGN §4.1.6.1 items 5 and 6). Entity rule 2b is lifted for guarded arcs: only a realized arc without a record is excluded, which on the ordinary route is none. Including rows can only raise S*, so a straight end can only lose a fire, and only below 2⁻³⁴ of the larger body scale. T4-I34 counted 0 of 1,052 straight decisions changed on its evidence; every straight pin passes unchanged.
- **Consequence.** As R-b′: one finding, applied by `amend_integrity_report` under the no-op rule. A Passed case becomes Sensitive with the sentence "S11-G recovery guard (R-b', arcs): load case … is Sensitive because n realized-arc end group(s) published from the formed K_c*u carry formation noise above the 1e-9 criterion; every value is kept for inspection. Ends: [member.end.group: q, B, B_f, S*; …]", at most six named. Nothing is refused, no value changes, and no field or code is added. The recovery-side sentences are merged in one finding: the straight sentence (byte for byte when alone), then the arc sentence, then the short-bend sentence.
- **Records.** `formation_guard::ArcRecoveryRecord` is new and unpriced. The priced `RecoveryRecord`, `MemberRecord` and `Preview*` layouts are unchanged.

### Short bends (R-1, option (c))

HELP_HUMAN's principle: no published Passed may breach its criterion. The torsion and shear rows of a short arc carry an error B_f cannot see. It does not come from formation; it is the solve's rounding of u, amplified by the recovery's cancellation.

- **Mechanism.** A bend of arc length Rφ between members of length ≥ L is a short stiff element: its ends move together almost rigidly by about |u|, while its end actions come from a relative motion smaller by the stiffness ratio. The recovery's cancellation C = Σ|A_s||K||u|/|f| therefore grows as (Rφ/L)⁻³, the exponent of the solve's conditioning 0.67·(Rφ/L)³ ([curved-numerical-integrity.md](curved-numerical-integrity.md)). A relative error of u enters the published rows as E/q = θ·2⁻⁵³·C. The representation of u contributes θ ≤ 1 exactly. The solve's backward error on the arc's stiff rows adds a small multiple.
- **Constants** (implementing head, every L-line case-mode from 90° to 1e-3 rad, w, w+T and UTM, exact 120-digit model solves; T4-I37's evidence). The shear row's a = C·(Rφ/L)³ is constant from Rφ/L = 7.6e-5 to 2e-2 (10.7 to 13.7), which confirms the cubic law; T4-I34 measured C = 4.5e7 at 5°. Taking C_shear as the amplification, the effective factor θ = E_pub/(2⁻⁵³·C_shear·q) over every group of every end, Passed or Sensitive, is at most 1.21.
- **Limit.** E_pub reaches RD(1e-9)·q at ρ_b = (θ·a·2⁻⁵³/RD(1e-9))^(1/3) = 1.23e-2. The Passed limit is ρ_b·2^(1/3), a margin of 2 on the error, rounded up to **1.6e-2**: `formation_guard::SHORT_BEND_PASSED_RATIO`. T4-I34's empirical estimate was 1.1e-2.
- **Rule.** A case that publishes a realized bend whose ratio Rφ/L (L the shorter adjacent member, as `CURVED_BEND_SHORT_RELATIVE_TO_NEIGHBOURS` computes it) is below the limit is demoted to Sensitive by the sentence "Short-bend rule: load case … is Sensitive because n realized bend(s) are short against their neighbours (arc length / shorter adjacent member length below 1.6e-2): …; Bends: [component (pipe): ratio]". The mechanism and no-op rule are the arc guard's. The pivot-screen refusal below 1e-4 is unchanged.
- **What it demotes.** On T4's fixtures, the L line at 5° and 8° and the UTM L line at 4°, 14 case-modes. The 5° and UTM 4° case-modes carry torsion and shear breaches of up to 4.9×. The 8° case-modes are correct (worst 0.79×) and are demoted by the rule's disclosed geometric limit. The L line at 15° (ratio 2.0e-2, worst 0.20×) stays Passed.
- **Follow-up (d), a separate unit.** It would lift this limit toward the refusal boundary. It needs a solve-side bound on each arc's relative end motion Δ, not available to recovery today. Two routes: K-D5's p = 128 re-formation and residual, extended to report a member-relative forward-error bound on Δ through the arc's tip stiffness; or W1c's retained precision for curved cases. The arc's rows would then be judged against the arc's own q instead of a geometric screen.

### Relation to SF-2's curved CannotBound

SF-2's `CannotBound` is kept, already narrowed by T4-U1b: it remains only for an arc load whose certificate fails. The arc guard does not replace the load-row guard, which judges load rows before the solve against the load-row scale. When a loaded arc's load certificate fails, both fire: the load row demotes at `append_integrity_report`, the arc bound is unavailable, and its amendment is a no-op, so the load-row sentence stands (`p4_both_guards_fire_and_the_load_row_sentence_stands`).

T4-U2's bend-pressure terms need no recovery-side ball of their own (O-1). Recovery subtracts only K_b·u_free(ε_p), whose held free expansion enters the ball exactly as the thermal one does. −c_b, the tangent caps and the kink remainder are nodal source loads that reach the arc only through u. All 79 valued T4-I7 pressurized cases stay Passed in both modes, with B/(RD(1e-9)·q) at most 8.2e-3.

W1c (curved bends in W1, with T4) remains T3's retained-precision phase. When it selects a curved case, its arc rows become retained projections and this finding, formed after routing, does not apply to them. W1c would also replace the short-bend rule and SF-2's fallback for the cases it selects.

### Retained route

Curved spans stay outside D1, checked on the implementing head. `source_recovery` refuses the component, curved, user-matrix or release source family, `retained_product` refuses any realized curved element, and F1b's W2 refuses the curved family at b ≠ 0. T4-U2's v3 admission of realized bends does not reach the retained source. Arc records therefore exist only on the ordinary linear and nonlinear routes. I5 ruling 3's receipt residual cannot reach arcs.

### Evidence and cost

T4-I37's evidence is in T4's scratch (`t4_i37/`). The product's own bounds and verdicts were recorded by a scratch hook and compared with T4-I34's 120-digit references: f(z, u) for formation, and exact model solves for published errors where the model has no pressure, imposed motion or nonlinear support.

- **T4's fixtures (400 case-modes, both modes).** Covered: the L line from 90° to 1e-3 rad (w, w+T, UTM), 0.3 m arcs to 1e-4 rad, k = 2 and 10, the I7 L and U lines (incl. skew and UTM), the 79 pressurized I7 documents, the demo with its elbow realized, the ARC-INPLANE analogues and large k.
  - Every bound encloses.
  - No final Passed case publishes a breach above the floor.
  - The arc guard demotes 20 case-modes, every one a breach of 7.6e2× to 5.2e3× (ARC-INPLANE and ARC-ONLY at G1e80). The short-bend rule demotes 14.
  - No correct case is demoted except by the short-bend limit (the 8° L lines).
- **Committed PP suites under the hook.** Covered: the lib and the curved, pressure, connector and admission suites.
  - The only demotions are this unit's own tests.
  - Every T4-I7 pressure reference stays Passed.
  - The residual is R-b′'s accepted structural zero. Two stress-free arcs publish rounding-size rows against an exact zero, with B_f ≤ 8e-6 of the criterion, and stay Passed (`p8_a_stress_free_arc_stays_passed`).
- **SP-1.** T4-I19's probe (182 documents, straight-only and bend-free; PP's ordinary, captured, typed and retained entries and HR's runner; both modes) gives 364 of 364 rows byte-identical between this unit's head and its pre-O-13 base. Against main, the differences are those the base already had (T4's declared legacy-joint refusals); none is this unit's.
- **Cost (O-2; release, shared host).**
  - Per arc and case: `certify_curved_recovery` about 80 µs, about 80 µs per uniform-load certificate, and `bound_end_actions` about 5 µs.
  - Per invocation: the I7 U line rose from 4.0 to 4.6 ms and the L line from 1.3 to 1.45 ms.
  - The per-arc certificate is not cached across cases. Caching would save about 80 µs per arc and additional case, and would need state carried through the case loop.

### Tests

- **FK** `arc_recovery_tests.rs` against `validation/references/t4_o13/`:
  - C1: stiffness enclosure and |K(z)| tightness;
  - C2–C4: row enclosure, tightness and the L7′ radius share;
  - C5: B_u;
  - free expansion and certified loads in the ball;
  - C6: unavailable;
  - L7′ against L7.
- **PP** `s11g_tests/arc_recovery.rs`:
  - G1–G6: conjuncts incl. R-2, scale assignment, unavailable, sentence, merge, scales;
  - P1: load-bearing ARC-INPLANE-M-G1e80 catch against the exact f*;
  - P2: realistic arcs not demoted, B/c < 0.1;
  - P4: both guards;
  - P5: no-op;
  - P6: arc-only body (2b lift);
  - P7: source pin;
  - P8: stress-free arc;
  - the short-bend rule at its limit and on R-1's cases.

## Independent review

A fresh numerical reviewer, who did not design or write the unit, reviews it against this requirement, and copies the verdict line to T3's WORKING_ITEMS.

- **Hardest attacks:**
  - an admissible arc model whose published end action is a Passed breach against an exact reference while the guard stays silent: small φ near the pivot guard, near π, chords near a coordinate plane, extreme R/L, stiff and soft sections, large k;
  - a demoted committed or realistic arc row that is actually correct.

  Either one is BLOCKING.
- **Also checked:**
  - the predicate and scale match R-b′;
  - the guard fails closed;
  - every mutant is killed by a named test;
  - item 6's no-regression conditions hold.

## Superseded in part by the K-D5 member-relative bound unit

HELP_HUMAN ruled on 2026-10-11 for option (d) ([kd5-member-relative-bound.md](kd5-member-relative-bound.md)). The ruling supersedes "my short-bend thresholds (ruling 5, plus the withdrawn (c) limit)", to be named "in the DEL-04-01 Design as superseded by this unit". When that unit lands:

- **Predicate.** The arc predicate is replaced by the unified member predicate on certified rows. This covers R-b′ on B_f + B_u and R-2's B_f clause.
- **Arc rows.** They are published as recoveries at the corrected displacement u + w + w₂, with K-D5's arc matrix. They no longer come from the formed K_c·u.
- **Short-bend rule.** The rule and the withdrawn (c) limit, its Passed ratio of 1.6e-2, are removed with the load closure.

**Batch-4 integration (K-D5 with U2/O-13).** On the integrated branch the predicate and arc-row items hold: `arc_group_fires`, its B_u and O-13's `ArcRecoveryRecord` are gone, arcs are judged by `member_group_decision` on their certified rows, and O-13's coupled `ActionScales` are the S* of every group. O-13's protective tests are kept against the unified predicate (`s11g_tests/arc_recovery.rs`). The short-bend rule (`short_bend_finding`, `ShortBend`, `SHORT_BEND_PASSED_RATIO`) and its finding merge are removed: M3 on the integrated producer verified the closed recovery in both modes ([kd5-member-relative-bound.md](kd5-member-relative-bound.md) §4.3), and its cases are pinned to the unified outcome (`r1_short_bend_cases_take_the_unified_outcome`). `bend_neighbour_ratios` and `CURVED_BEND_SHORT_RELATIVE_TO_NEIGHBOURS` stay for the pivot-screen refusal.

The requirement above remains the record of O-13's unit.

Sources:
- S11-G design: [S11G_GUARD.md revision 2.1](https://github.com/sgttomas/chirality/blob/e85d383b64/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/DESIGN_NUMERICS/S11G_GUARD.md).
- R-b′ selection and load-bearing ruling: [T3 rulings](https://github.com/sgttomas/chirality/blob/e85d383b64/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/ROOT_RULINGS_V1.md), sections "S11-G note revision 2: rulings …", "S11-G revision 2 after V1's delta check" and "S11-G implementation: I5 rulings".
- Curved requirements: [curved-numerical-integrity.md](curved-numerical-integrity.md).
