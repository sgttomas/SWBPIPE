# Hydrostatic test condition (outline)

**Row:** RT-A6 (Must; OC-5 (a): "A flagged test condition with test pressure,
test contents density and spring supports held, marked as a test case in
results and reports"). **Basis:** main `8cab1b3e5` (2026-10-30).
**Status:** outline. It fixes the model-document member so that it joins
0.9.0 before S1 (SWBPIPE Agent 0, 2026-10-30). The mechanics design follows
0.9.0 S2 and S4 and RT-A3, and is its own reviewed revision of this file.
Risk class: numerical meaning and persistence.

Paths are relative to the repository root. Standard claim fence applies
(F-PIP-2; claims taxonomy per DEC-081). Nothing here records or implies
acceptance, issuance or release.

## 1. Decisions applied

- **Held supports** (SWBPIPE Agent 0, 2026-10-30, under the engineering-judgment
  delegation): the test condition holds constant-effort supports as well as
  springs, because both are locked at their travel stops for a hydrotest. RT-A3
  D-2 makes a constant-effort force act in every state; a held support's force
  law does not act in the test case.
- **Route:** the test runs on EX, which carries pressure. PF has no pressure
  carrier (RT-A3 / OC-2: EX is extended; PF gets no pressure).

## 2. The member (model document 0.9.0)

`load_cases[].test_condition`, optional, a closed `TestCondition`:

| Member | Type | Required | Meaning |
|---|---|---|---|
| `kind` | enum `["hydrostatic"]` | yes | The case is a hydrostatic test. Results and reports mark it as a test case. |
| `held_supports` | enum `["springs_and_constant_effort", "none"]` | yes | `springs_and_constant_effort`: every spring support (family `spring`, and `variable_spring_hanger` or `spring_hanger` hangers) and every constant-effort support acts in this case as a rigid restraint on the DOF it acts on, and its spring or constant force does not act. `none`: supports act as in every other case. |

- Nothing is defaulted, and an authored `null` is refused (0.9.0 §1.2's rule).
  `held_supports` is entered, never inferred from `kind`; the desktop's
  hydrotest template writes `springs_and_constant_effort`, the decided practice.
- **Test pressure** is the case's pressure, authored in its `pressure_regions`
  or taken from a condition through `condition_use.pressure` (0.9.0 S4).
- **Test contents density** is the case's `condition_use.contents_density`,
  the member 0.9.0 reserves for a per-condition contents consumer (§1.6, OC-7).
  RT-A6 is that consumer, so its refusal (`LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED`)
  lifts with this row.
- **Interim (S1 until RT-A6's build):** a document carrying `test_condition`
  opens, validates and edits, and the applier refuses authoring it. The solve
  refuses the case by name, blocking, as `LOAD_CASE_TEST_CONDITION_UNSUPPORTED`.
  The existing `HYDROTEST_PRESSURE_UNSUPPORTED` refusal of a `hydrotest`
  primitive with pressure dimension is unchanged.

## 3. Questions for the mechanics design

- **Held, exactly.** The restraint's displacement reference (zero displacement
  at the support's node, as a rigid support), its direction for a skewed
  `direction` (0.9.0 §2.1), the spring's installed load and the variable
  spring's preload (RT-S3's open item), and the published reaction of a held
  support (a support row in the test case, marked held).
- **Contents.** How the condition density replaces `section.contents_density`
  in self-weight generation for the test case only, its effect on the
  self-weight staleness check (`inspect_applied_self_weight`), and the EX
  ledger entry (`analysis_state.load_sources`). EX does not provide
  per-case contents weight today (`exactReportLimitations.ts`).
  **Planned shape** (to be designed and reviewed here; agreed in outline with
  the dynamics lane, 2026-10-31, because modal's mass must equal a named
  state's self-weight basis): one resolver,
  `case_contents_density(model, case, pipe)`, giving condition key k's density
  when the case's `condition_use.contents_density` names k, and the section's
  otherwise. Both mass-per-length functions (`stable_mass_per_length`,
  `exact_generation_mass_per_length`) read through it, the self-weight
  dependency projection records the resolved value and its source, and modal
  calls the same resolver and mass function for its named state, so the two
  bases cannot drift. The resolver is a public crate function callable for any
  named case id outside self-weight generation (DEL-04-09's state-case mass
  calls it directly); until it lands, modal refuses a state case that resolves
  condition contents, as the solve does. A condition's `added_weight` is a `weight` primitive,
  not self-weight lineage.
- **PF.** Whether PF admits a pressure-free test case (contents weight and held
  supports only) or refuses `test_condition` by name.
- **Marking.** The result envelope's case record, the report's load-case
  section and the results navigator carry the mark. A test case in a
  combination: allowed and labelled, or withheld.
- **The `hydrotest` primitive category** with pressure dimension: kept as an
  unsolved record, or retired in favour of the test condition.
- **Reference (RT-V1).** An invented simply supported, water-filled span on EX:
  the weight of pipe and contents (wL⁴ deflection, wL/2 reactions), a held
  spring at midspan as a rigid prop (the propped-beam closed form), and the
  test-pressure membrane stresses (Lamé). An independent generator under
  `validation/references/`.

## 4. Order

After 0.9.0 S2 (conditions on PF) and S4 (conditions on EX), and after RT-A3
admits springs and constant-effort supports on EX. Designed with DEL-04-07.
