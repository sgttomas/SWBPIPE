# Section bases and shear model

The owner ruled D-5 and D-6 on 2026-10-10: "D-5 and D-6 as recommended". Both apply to `3.0.0/exact_pressure_v3` documents. `2.0.0/exact_straight_pressure_v2`, until B7b deletes it, and pressure-free documents keep their current single basis and Euler–Bernoulli model, byte-identical (H-1, DEL-04-07 Design). On 2026-10-12 the owner answered O1–O4 and HELP_HUMAN ruled H1, H4 and H5 (both relayed to T4's WORKING_ITEMS); O2–O4, H1, H4 and H5 are applied below where they govern. On PRD non-goal 19 the owner chose "Lift for tie rods only" (O1); SCA-012 and its PRD amendment are carried in this branch and are not yet on main.

## The two fields (O2, O3, H1)

Both fields are defined in T3's model format 0.5.0 (H1: "no 0.6.0"), which B7 defines ([DEL-05-02 Design, b7-plan.md](../../DEL-05-02/Design/b7-plan.md)). They are required on every exact-contract document from the start, and T4-U6 and T4-U8 then only widen what is admitted (H1).

- **Corrosion allowance** (owner, O2: "Required; absent = refused"). `section.corrosion_allowance`, a length Quantity ≥ 0, is required on every member's pipe section when `pressure_contract` is present; zero is allowed. Absence is refused by name as incomplete (`EXACT_PRESSURE_CORROSION_ALLOWANCE_REQUIRED`); the field on a document without a pressure contract is refused (`CORROSION_ALLOWANCE_WITHOUT_PRESSURE_CONTRACT`); until T4-U6 lands, a nonzero value is refused by name as not yet supported (`EXACT_PRESSURE_CORROSION_ALLOWANCE_UNSUPPORTED`). Mill tolerance plus corrosion allowance must be less than the wall. The allowance is user data with no default; a missing value is never assumed.
- **Shear model** (owner, O3: "Refuse by name"). The per-document selection is `pressure_contract.shear_model`, `euler_bernoulli` or `timoshenko_annulus_energy_v1`, required when that contract is present. The contract is an object, so B7's adopted field specification places the selection inside it; there is no top-level alias. Absence is refused by name (`EXACT_PRESSURE_SHEAR_MODEL_REQUIRED`); a shear-model field without a pressure contract is refused (`SHEAR_MODEL_WITHOUT_PRESSURE_CONTRACT`); until T4-U8 only `euler_bernoulli` is admitted (`EXACT_PRESSURE_SHEAR_MODEL_UNSUPPORTED`). When T4-U8 lands, authoring writes `timoshenko_annulus_energy_v1` by default, per D-6. Contract identity selectors compare `version` and `mode`, and validate the required shear member separately rather than comparing the whole object to a two-member literal.

## D-5: section bases (T4-U6)

- **Stiffness and pipe metal mass** use the nominal wall. **Contents mass** uses the reduced (pressure-path) bore (H4).
- **Stresses** are published on both the nominal section and the reduced section (nominal wall minus mill tolerance and the corrosion allowance). Each row names its basis. Each basis has its own maximum and headline; the product does not choose a governing basis, the user's rule pack does. Where the bases coincide, both families are still published (H4).
- **Pressure load path:** the end-cap area Ai uses the bore of the reduced wall (internal corrosion enlarges it). The area As in ε_p = (1−2ν)pAi/(E·As) is the stiffness section, so the ledger stays self-consistent.
- MILLTOL and other references whose basis changes are re-frozen deliberately, never loosened.

## D-6: shear model (T4-U8)

- The shear model is the explicit field above, which authoring always writes. Its default is Timoshenko straight pipe with the energy-matched annular shear factor (SHEAR_REFERENCE). Euler–Bernoulli remains a legitimate explicit modelling choice.
- Until T4-U8 the field admits only `euler_bernoulli`, and the formulation evidence names it.
- **Implementation (H5).** The selection is per document. Whether Timoshenko is a new FK element type or a re-priced `s(FrameElement)` is the implementer's choice; either way it is priced positively. Realized arcs stay Euler–Bernoulli, with a disclosure.
- References T4-RV4 marked as shear-affected are re-frozen deliberately.

## Retained-source and W1-E publication (O4)

The owner chose "Scope extension after B7b" (O4). Until then, named declines are the interim: `prepare_sources` declines a Timoshenko case and a case with nonzero mill tolerance or corrosion allowance by name, so the case publishes on the ordinary route without retained-source or W1-E publication. Once T4-U8 makes Timoshenko the authoring default, default-authored documents take that decline. A source-method/W1 extension that restores retained-source and W1-E publication for default-authored documents is T3 work after B7b (DEL-04-07 Design, "T3 coexistence and B7").

## SP-1 scope after D-5 and D-6

SP-1's twin clause (a straight-only v3 case is bit-equal to its v2 twin; transitional until v2 retires) applies to v3 cases with zero mill tolerance, zero corrosion allowance and the Euler–Bernoulli formulation, where every basis and the element formulation coincide with v2. Other v3 cases are checked against their own independent references. Where T4-U6 and T4-U8 both change a reference, re-freeze it once.

Source for the options: [T4 frozen rulings: owner-held later and D-6 presentation](https://github.com/sgttomas/chirality/blob/f2211ecef6/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/PRESSURE_STRESS_T4/T4_RULINGS.md); plan 01 §4.1 (same tag, `PLAN_01/PLAN.md`).
