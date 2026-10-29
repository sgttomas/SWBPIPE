# Section bases and shear model

The owner ruled D-5 and D-6 on 2026-10-10: "D-5 and D-6 as recommended". Both apply to `3.0.0/exact_pressure_v3` documents, the only exact contract since B7b deleted `2.0.0/exact_straight_pressure_v2`. Pressure-free documents keep their current single basis, byte-identical (H-1, DEL-04-07 Design); their beam theory is under D-6. On 2026-10-12 the owner answered O1–O4 and HELP_HUMAN ruled H1, H4 and H5 (both relayed to T4's WORKING_ITEMS); O2–O4, H1, H4 and H5 are applied below where they govern. On PRD non-goal 19 the owner chose "Lift for tie rods only" (O1), recorded by SCA-012 and its PRD amendment.

## The two fields (O2, O3, H1)

Both fields are defined in T3's model format 0.5.0 (H1: "no 0.6.0" for these fields), which B7 defines ([DEL-04-08 Design, b7-plan.md](../../../PKG-04/DEL-04-08/Design/b7-plan.md)). They are required on every exact-contract document from the start, and T4-U6 and T4-U8 then only widen what is admitted (H1). HELP_HUMAN's 2026-10-13 technical ruling selects model 0.6.0 for later U4b/U5b input additions, retiring and re-authoring 0.5.0 then; B7b defines no inactive future fields. The result formats and semantic identities that land with 0.6.0 are in DEL-04-07 Design `pressure-contract.md`. The T4-U4b/U5b epoch made 0.6.0 current; it carries both fields unchanged (DEL-04-07 Design `u4b-u5b-implementation.md` §1.1).

- **Corrosion allowance** (owner, O2: "Required; absent = refused"). `section.corrosion_allowance`, a length Quantity ≥ 0, is required on every member's pipe section when `pressure_contract` is present; zero is allowed. Absence is refused by name as incomplete (`EXACT_PRESSURE_CORROSION_ALLOWANCE_REQUIRED`); the field on a document without a pressure contract is refused (`CORROSION_ALLOWANCE_WITHOUT_PRESSURE_CONTRACT`); a nonzero value is admitted from T4-U6, which removed the interim refusal (`EXACT_PRESSURE_CORROSION_ALLOWANCE_UNSUPPORTED`). Mill tolerance plus corrosion allowance must be less than the wall. The allowance is user data with no default; a missing value is never assumed.
- **Shear model** (owner, O3: "Refuse by name"). The per-document selection is `pressure_contract.shear_model`, `euler_bernoulli` or `timoshenko_annulus_energy_v1`, required when that contract is present. The contract is an object, so B7's adopted field specification places the selection inside it; there is no top-level alias. Absence is refused by name (`EXACT_PRESSURE_SHEAR_MODEL_REQUIRED`); a shear-model field without a pressure contract is refused (`SHEAR_MODEL_WITHOUT_PRESSURE_CONTRACT`); both values are admitted from T4-U8, which removed the interim refusal (`EXACT_PRESSURE_SHEAR_MODEL_UNSUPPORTED`). From T4-U8, authoring writes `timoshenko_annulus_energy_v1` by default, per D-6. Contract identity selectors compare `version` and `mode`, and validate the required shear member separately rather than comparing the whole object to a two-member literal.

## D-5: section bases (T4-U6)

- **Stiffness and pipe metal mass** use the nominal wall. **Contents mass** uses the reduced (pressure-path) bore (H4).
- **Stresses** are published on both the nominal section and the reduced section (nominal wall minus mill tolerance and the corrosion allowance). Each row names its basis. Each basis has its own maximum and headline; the product does not choose a governing basis, the user's rule pack does. Where the bases coincide, both families are still published (H4).
- **Pressure load path:** the end-cap area Ai uses the bore of the reduced wall, r_i = OD/2 − wall + mill_tolerance + corrosion_allowance (both the mill tolerance and internal corrosion enlarge it). The area As in ε_p = (1−2ν)pAi/(E·As) is the stiffness section, so the ledger stays self-consistent.
- MILLTOL and other references whose basis changes are re-frozen deliberately, never loosened.

## D-6: shear model (T4-U8)

- The shear model is the explicit field above, which authoring always writes. Its default is Timoshenko straight pipe with the energy-matched annular shear factor (SHEAR_REFERENCE). Euler–Bernoulli remains a legitimate explicit modelling choice.
- Before T4-U8 the field admitted only `euler_bernoulli`. From T4-U8 it admits both, and the evidence names each straight member's theory (`beam_theory`) and each realized arc's Euler–Bernoulli formulation.
- **Implementation (H5).** The selection is per document. Whether Timoshenko is a new FK element type or a re-priced `s(FrameElement)` is the implementer's choice; either way it is priced positively. Realized arcs stay Euler–Bernoulli, with a disclosure.
- References T4-RV4 marked as shear-affected are re-frozen deliberately.
- **Pressure-free documents** (no `pressure_contract`). Straight members are Euler–Bernoulli unless the document's `model_settings.shear_model` selects `timoshenko_annulus_energy_v1`: the same element and shear-factor definition, with ν = E/(2G) − 1 from the authored moduli. An absent member is Euler–Bernoulli, and authoring never writes it unprompted. Realized arcs stay Euler–Bernoulli, with a disclosure. Design: [pf-timoshenko.md](pf-timoshenko.md).

## Reference preparation (T4-U6, T4-U8)

Carried in the owner's 2026-10-14 resumption steer:

- The references are prepared from the vetted SHEAR reference snapshot at `f2211ece` ([SHEAR_REFERENCE](https://github.com/sgttomas/chirality/tree/f2211ecef6/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/CORRECTNESS_DESIGN/SHEAR_REFERENCE)) and the repaired T4-I8 freeze at `5b07d85c` (repair round 01, [T4-I8](https://github.com/sgttomas/chirality/tree/5b07d85cf3/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/PRESSURE_STRESS_T4/T4-I8)).
- Older references that folded the mill tolerance into the wall value are re-authored with the true nominal wall plus explicit mill-tolerance and corrosion-allowance fields. Their reduced bore then follows D-5 above.

## Retained-source and W1-E publication (O4)

The owner chose "Scope extension after B7b" (O4). From T4-U6/U8 until O4, named declines were the interim: `prepare_sources` declined a Timoshenko document and one with nonzero mill tolerance or corrosion allowance by name, and W1-E declined the same documents at its admission.

**As built (O4, epoch 5).** The declines are retired. Both retained routes admit either shear model on straight members and any admitted mill tolerance and corrosion allowance under the D-5 bases: stiffness, the Timoshenko shear area and κ_E on the nominal wall, and each stress row on its own basis section. The source route declares the frame's own Timoshenko local matrix. W1-E forms a retained Timoshenko member, a κ_E lane enclosure and D-entry gap majorants, and certifies a member's reduced stress rows on the reduced section where the bases differ. The readers rederive both bases and the prepared shear-area leaf (A_s,prep = RN(κ̂·A_prep)) through `section_basis`. Design: DEL-04-08 Design [o4-source-extension.md](../../../PKG-04/DEL-04-08/Design/o4-source-extension.md).

## SP-1 scope after D-5 and D-6

SP-1's twin clause (a straight-only v3 case is bit-equal to its v2 twin) was transitional until v2 retired, and it lapsed when B7b deleted v2, before T4-U6 and T4-U8 merged. Its scope after D-5 and D-6 was the v3 cases with zero mill tolerance, zero corrosion allowance and the Euler–Bernoulli formulation, where every basis and the element formulation coincide with v2; those cases keep their reference files and values. Other v3 cases are checked against their own independent references. Where T4-U6 and T4-U8 both change a reference, re-freeze it once.

The technical design that implements these rulings in T4-U6 and T4-U8 is [section-bases-and-timoshenko.md](section-bases-and-timoshenko.md).

Source for the options: [T4 frozen rulings: owner-held later and D-6 presentation](https://github.com/sgttomas/chirality/blob/f2211ecef6/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/PRESSURE_STRESS_T4/T4_RULINGS.md); plan 01 §4.1 (same tag, `PLAN_01/PLAN.md`).
