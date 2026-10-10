# Curved-element numerical integrity

Current requirements for the T4 curved-element replacement. The section "T4-U1 as implemented" records how T4-U1 meets them; it is not an acceptance claim.

## Mutation equivalence and conditioning

M31b0 equivalence remains **pending**, not accepted. Before seeking the narrowing, correct the derivation's numerical paragraph (N-1: inverted worst-case wording; the 2⁻¹²⁷/φ bound omits a constant and a y-projection term; the evidence uses decimal precision, not p = 128). N-2 requires K-D5 inputs to become (R, y), without a binary64 centre, derived radius or φ. Present the corrected derivation, RV131's independent check and T3's view for a ruling. Do not retire M31b0's existing kill before that ruling.

K1 must exercise the stable T4-U1 form at φ = 1e-8, extending the earlier 1e-4 acceptance floor. If the stable form moves to T4-U1c, K1 moves with it. K2 remains a supplemental mutation check; it **does not satisfy** the conditioning-driven curved-demotion requirement. Construct a curved true positive (RV131's candidate is F122 realized as a bend, R = 10 m, φ ≈ 0.30 rad). If no such positive can be constructed, seek a narrowing rather than silently substituting K2. Build K2 from the frozen reference or K-D5's p = 128 re-formation and verify the claim that it also kills M31a.

For CSKEW_8_5, agree the rule with T3 before running: no Passed breach in either mode; demote exactly when actual > 0.5, with a ±0.05 guard. Substitute CSKEW_9 or CSKEW_10 instead of relaxing the condition. The kernel's curved-positive controls must reflect this requirement. Regenerate the old models from their bow vectors.

## Stable small-angle evaluation

Use the stable form **1 − cos φ = 2s² at p = 128**, rather than raising precision, in K-D5. Include a kernel check at L = 30 m: long nearly straight bends (φ ≤ 2e-9) otherwise produce false demotions. The product element uses stable binary64 series/half-angle evaluation, retains the existing 1e-9 rad floor and introduces no minimum-angle refusal. Its switch must be continuous and bitwise deterministic. Keep Sensitive demotion as the fallback.

Acceptance also requires guard and K-D5 margin from φ = 1e-4 rad to π − ε at both ordinary and UTM coordinates, and I9's probe rerun on the product. K1's φ = 1e-8 requirement above extends this acceptance floor; it does not replace the full-range check.

If the stable form needs more than about two days, or a T3 re-agreement beyond T4-U1's own, land T4-U1 as planned with (c) disclosed and follow with T4-U1c before T4-U2 merges. The conditioning-driven true-positive demotion requirement remains in force.

O-3 accepts Lemma 3 with no `CRITERION` change. O-4 makes this `atan_positive`'s first product caller, with N-2's test and A-6's near-π treatment. These are the RV129 O-3/O-4 dispositions, distinct from the M31b0 N-1/N-2 conditions above.

## T4-U1 as implemented

- **Element.** CB forms the bend from (x_i, x_j, R, y_reference) only: no absolute centre; H from A·d; c from 4R² − |d|² formed from exact products. φ comes from K3a's `atan_positive` at p = 128 rounded once, so the formation path calls no platform libm and its bits are the same on every platform. The cancelling flexibility integrals use exact-rational series below the switch s < 1/2 (a libm-free comparison) and half-angle closed forms above; the jump at the switch is ≤ 5e-15 relative. Before the guarded tip inversion F is scaled by one power of two, so short arcs form down to the 1e-9 rad floor without changing any element that formed before. PP drops the `bend_plane_orientation` requirement (D-B) except on `2.0.0/exact_straight_pressure_v2` documents, whose diagnostics stay byte-identical (SP-1).
- **K-D5.** `CurvedFormation` carries (R, y_reference); φ and the chord are formed at p = 128 from (d, R, y) with 1 − cos φ = 2s² (N-2 holds by construction). The L = 30 m kernel test kills a reversion to the cancelling form.
- **Kills.** M31b is killed by K1/K1F (SA and PP at UTM) and K2 (FK, frozen binary64 system matrix, fl(cos φ_b) = 1 pinned); M31a by K2 with `shared` from the scattered matrix. M31b0's two existing kill tests are kept but ignored, pending the ruling above.
- **Conditioning-driven curved positive.** Selected by the pre-registered rule (first candidate in T4-I6 round 01's rank order that publishes Passed ordinarily and is demoted in both modes with actual > 0.55 and EF/actual within 1e-3; the others stay D5C-1 controls): `C122-R10-Y100` (F122's 3 m member realized at R = 10 m, y = (1,0,0)), actual 1.859 dense / 1.147 sparse. It replaces CSKEW_8_5 in NI's parity and ledger-split tests.
- **CSKEW_8_5** runs as an undemoted D5C-1 control (actual 0.272 dense / 0.219 sparse); no substitution was needed.
- **Small direction changes.** The stable form meets the guard and K-D5 margin from 1e-4 rad when the arc is not much shorter than its neighbours. A standard-radius bend of a few degrees between metre-scale straights is a short, stiff element beside soft ones: the global solve is genuinely ill-conditioned (≈0.67·(Rφ/L)³), the ordinary report is Sensitive below roughly Rφ/L ≈ 6e-3 and the pivot screen refuses below roughly 5e-5. A straight chord element of the same length behaves identically. Whether to accept these limits or change the formulation (series-composing short bends with a neighbour) is open for HELP_HUMAN.

## Certified arc loads

R-1 replaces CannotBound for certified curved consistent-load terms with the proven midpoint–radius enclosure; the guard still demotes when the certified defect including radius exceeds the criterion. After RV129, R-1 remains in force **conditional on revision 01 incorporating A-1–A-6 and RV129 confirming it**. Failure to close the directed-rounding omission makes R-1 lapse.

The amendments require directed rounding of binary64 radii (lower-bound denominators, upward constants/sums and subnormal allowance), tests for all nine failed-precondition classes, and the unchanged priced layout: `Exact { scale: 1.0, scaled_intended: split(m) }` with operand bound r plus γ₅ for generated loads. No new FK type or PINNED_RECORD change. Re-derive T15d/MU2 after the small-bend change and use a sharpened L5 bound or angle reflection near π. Failed preconditions remain CannotBound.

R-2 requires a natural refusal for T15c/M16 first. Probe k ∈ {6e35, 1e36, 1e37, 1e38}. A test seam is a last resort needing a narrowing; retaining only the natural guard-level test also needs the stated narrowing if the end-to-end claim is lost. T3's arc R-b′ analogue remains needed before T4-U2.

Sources: [T4 frozen log, final RV131 and RV129 entries](https://github.com/sgttomas/chirality/blob/f2211ecef6/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/PRESSURE_STRESS_T4/WORKING_ITEMS_LOG.md); [T4 rulings, R-1 after RV129 and small bends](https://github.com/sgttomas/chirality/blob/f2211ecef6/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/PRESSURE_STRESS_T4/T4_RULINGS.md).
