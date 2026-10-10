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

Sources:
- S11-G design: [S11G_GUARD.md revision 2.1](https://github.com/sgttomas/chirality/blob/e85d383b64/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/DESIGN_NUMERICS/S11G_GUARD.md).
- R-b′ selection and load-bearing ruling: [T3 rulings](https://github.com/sgttomas/chirality/blob/e85d383b64/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/ROOT_RULINGS_V1.md), sections "S11-G note revision 2: rulings …", "S11-G revision 2 after V1's delta check" and "S11-G implementation: I5 rulings".
- Curved requirements: [curved-numerical-integrity.md](curved-numerical-integrity.md).
