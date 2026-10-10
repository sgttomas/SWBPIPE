# Section bases and shear model

The owner ruled D-5 and D-6 on 2026-10-10: "D-5 and D-6 as recommended". Both apply to `3.0.0/exact_pressure_v3` documents. `2.0.0/exact_straight_pressure_v2` and the pressure-free formats keep their current single basis and Euler–Bernoulli model, byte-identical (H-1, DEL-04-07 Design).

## D-5: section bases (T4-U6)

- **Stiffness and mass** use the nominal wall.
- **Stresses** are published on both the nominal section and the reduced section (nominal wall minus mill tolerance and a new corrosion-allowance input). Each row names its basis; the user's rule pack selects which governs.
- **Pressure load path:** the end-cap area Ai uses the bore of the reduced wall (internal corrosion enlarges it). The area As in ε_p = (1−2ν)pAi/(E·As) is the stiffness section, so the ledger stays self-consistent.
- The corrosion allowance is user data with no default; a missing value is reported, not assumed.
- MILLTOL and other references whose basis changes are re-frozen deliberately, never loosened.

## D-6: shear model (T4-U8)

- New v3 documents default to Timoshenko straight pipe with the energy-matched annular shear factor (SHEAR_REFERENCE).
- Euler–Bernoulli remains an explicit, recorded selection. A v3 document without the selection reads as Euler–Bernoulli, so no saved document changes its numbers on reopening.
- References T4-RV4 marked as shear-affected are re-frozen deliberately.

## SP-1 scope after D-5 and D-6

SP-1's twin clause (a straight-only v3 case is bit-equal to its v2 twin) applies to v3 cases with zero mill tolerance, zero corrosion allowance and the Euler–Bernoulli selection, where every basis and the element formulation coincide with v2. Other v3 cases are checked against their own independent references. Where T4-U6 and T4-U8 both change a reference, re-freeze it once.

Source for the options: [T4 frozen rulings: owner-held later and D-6 presentation](https://github.com/sgttomas/chirality/blob/f2211ecef6/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/PRESSURE_STRESS_T4/T4_RULINGS.md); plan 01 §4.1 (same tag, `PLAN_01/PLAN.md`).
