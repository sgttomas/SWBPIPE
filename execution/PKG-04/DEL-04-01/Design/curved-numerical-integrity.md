# Curved-element numerical integrity

Current requirements for the T4 curved-element replacement; these do not claim that T4 code has landed.

## Mutation equivalence and conditioning

M31b0 equivalence remains **pending**, not accepted. Before seeking the narrowing, correct the derivation's numerical paragraph (N-1: inverted worst-case wording; the 2⁻¹²⁷/φ bound omits a constant and a y-projection term; the evidence uses decimal precision, not p = 128). N-2 requires K-D5 inputs to become (R, y), without a binary64 centre, derived radius or φ. Present the corrected derivation, RV131's independent check and T3's view for a ruling. Do not retire M31b0's existing kill before that ruling.

K1 must exercise the stable T4-U1 form at φ = 1e-8, extending the earlier 1e-4 acceptance floor. If the stable form moves to T4-U1c, K1 moves with it. K2 remains a supplemental mutation check; it **does not satisfy** the conditioning-driven curved-demotion requirement. Construct a curved true positive (RV131's candidate is F122 realized as a bend, R = 10 m, φ ≈ 0.30 rad). If no such positive can be constructed, seek a narrowing rather than silently substituting K2. Build K2 from the frozen reference or K-D5's p = 128 re-formation and verify the claim that it also kills M31a.

For CSKEW_8_5, agree the rule with T3 before running: no Passed breach in either mode; demote exactly when actual > 0.5, with a ±0.05 guard. Substitute CSKEW_9 or CSKEW_10 instead of relaxing the condition. The kernel's curved-positive controls must reflect this requirement. Regenerate the old models from their bow vectors.

## Stable small-angle evaluation

Use the stable form **1 − cos φ = 2s² at p = 128**, rather than raising precision, in K-D5. Include a kernel check at L = 30 m: long nearly straight bends (φ ≤ 2e-9) otherwise produce false demotions. The product element uses stable binary64 series/half-angle evaluation, retains the existing 1e-9 rad floor and introduces no minimum-angle refusal. Its switch must be continuous and bitwise deterministic. Keep Sensitive demotion as the fallback.

## Certified arc loads

R-1 replaces CannotBound for certified curved consistent-load terms with the proven midpoint–radius enclosure; the guard still demotes when the certified defect including radius exceeds the criterion. After RV129, R-1 remains in force **conditional on revision 01 incorporating A-1–A-6 and RV129 confirming it**. Failure to close the directed-rounding omission makes R-1 lapse.

The amendments require directed rounding of binary64 radii (lower-bound denominators, upward constants/sums and subnormal allowance), tests for all nine failed-precondition classes, and the unchanged priced layout: `Exact { scale: 1.0, scaled_intended: split(m) }` with operand bound r plus γ₅ for generated loads. No new FK type or PINNED_RECORD change. Re-derive T15d/MU2 after the small-bend change and use a sharpened L5 bound or angle reflection near π. Failed preconditions remain CannotBound.

R-2 requires a natural refusal for T15c/M16 first. Probe k ∈ {6e35, 1e36, 1e37, 1e38}. A test seam is a last resort needing a narrowing; retaining only the natural guard-level test also needs the stated narrowing if the end-to-end claim is lost. T3's arc R-b′ analogue remains needed before T4-U2.

Sources: [T4 frozen log, final RV131 and RV129 entries](https://github.com/sgttomas/chirality/blob/f2211ecef6/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/PRESSURE_STRESS_T4/WORKING_ITEMS_LOG.md); [T4 rulings, R-1 after RV129 and small bends](https://github.com/sgttomas/chirality/blob/f2211ecef6/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/PRESSURE_STRESS_T4/T4_RULINGS.md).
