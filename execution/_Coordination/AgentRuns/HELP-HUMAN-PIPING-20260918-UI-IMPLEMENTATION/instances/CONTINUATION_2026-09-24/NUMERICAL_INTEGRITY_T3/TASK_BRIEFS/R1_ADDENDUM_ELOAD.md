# R1 addendum: RF-ELOAD (element, eigen, support-effort, prescribed and generated loads)

This is a reference author TASK. Read `_COMMON.md` and [R1_REFERENCES.md](R1_REFERENCES.md) first. The independence rules, formulation, criterion, "what not to do" and return format of R1 apply unchanged, except where this brief differs.

- **Commissioned by:** ROOT, D-11 and D-14, 2026-09-26. It must land, be refuted and be selected before slice F3 (W1b) is implemented.
- **Author:** a fresh, product-code-blind instance. Refutation follows in a separate V2-style TASK.

## Purpose

W1b extends the retained-precision method to every load family beyond nodal loads. This addendum supplies the independent hand-derived references and negative controls that will judge it. By ROOT's D-14 ruling, it also covers **generated equivalent-static loads**. Their magnitudes are computed exactly from the user's inputs as the intended source, the same principle as re-forming stiffness from its inputs. The rounded binary64 product is not treated as the input.

## Independence and permitted reading

- **Do not read product source** (`P/core/**`, `P/apps/**`) or product test expectations. Do not import or call production code.
- In addition to R1's permitted list, you may read the following records, which state the implemented theory in open mechanics. They are not product code.
  - `P/validation/hand_calcs/mechanics/*.md`, especially:
    - `straight_pipe_weight_recovery.md`
    - `primitive_load_preparation.md`
    - `tp_phys_004`, `tp_phys_005`, `tp_phys_006`, `tp_phys_007`, `tp_phys_008`, `tp_phys_009`
    - `fixed_fixed_thermal_axial.md`
    - `constant_effort_support_applied_load.md`
    - `imposed_displacement_spring.md`
    - `tp_pmm_p3_occloadgen_equivalent_static.md` and `tp_pmm_p3_subspan_wind_exposure.md`
  - `P/docs/validation_manual/cases/mechanics/*.md`.
  - T1's analytical reference package, for the 0.4.0 resolved prescribed-motion and thermal-eigen conventions: `T3/../LOAD_STATE_IMPLEMENTATION/ANALYTICAL_REFERENCE/**`.
  - `T3/REFERENCES/**`, the frozen R1 package, for conventions and case-id style. **Do not edit it.**
- These records say *what* the implemented theory is. Derive every value yourself. Where they are silent, ambiguous or disagree about the theory, **ask the manager** before choosing. The manager answers from the accepted records and the source, and records the answer. Record each definition you adopt, and its basis, in your README.

## Theory, stated by the manager

- **Straight members.** Uniform and partial-span uniform loads (in local or global direction) enter as consistent Euler–Bernoulli equivalent nodal loads, including fixed-end moments. Member end actions are recovered with the fixed-end correction. For a uniform load this reproduces the exact Euler–Bernoulli solution at the nodes and member ends, so write the exact theory.
- **The 50/50 lumping** named in some records is a primitive-load preparation cross-check, not the solve. Use it only as a negative control.
- **Curved bends** are excluded (W1c, with T4).

## Required families (family id RF-ELOAD)

For every case, supply what R1 requires:
- exact expressions or derivation;
- frozen values;
- the reference's own accuracy;
- a derived scale for each quantity, and a zero scale for each expected zero;
- negative controls.

Use invented properties and the N-series section unless a case needs otherwise. State every input, including the densities, g-factors, pressures, shape factors, insulation thickness, mill tolerance and gravity acceleration. These are user-entered inputs; none is material or catalogue data.

1. **Uniform element loads.**
   - Full span and partial span.
   - Local and global direction, on axis-aligned and rational-skew members (Pythagorean-quadruple rotations).
   - A statically indeterminate case, such as a propped cantilever or a two-span continuous beam.
   - Weight entered as a user-given intensity.
2. **Generated self-weight,** if the records define it as generated from user-entered density and section. Write it by the same exact-from-inputs rule as item 6.
3. **Thermal eigen axial load:** legacy and 0.4.0 resolved.
   - Fixed-fixed, and one end on a soft axial spring.
   - A two-member case in which the restraint comes from the structure.
4. **Pressure thrust on straight members,** as the records define it (end-cap thrust and its fixed-end correction).
   - Include a case with a free end and a case with both ends restrained.
5. **Constant-effort support forces:**
   - alone;
   - with nodal loads;
   - with the effort nearly cancelling another load at the same node.
6. **Generated equivalent-static loads (D-14).**
   - **Seismic.** Intensity is g-factor × g × mass per length. Mass per length is metal plus contents plus insulation, taken over the effective wall defined in the records (including mill tolerance), per global axis.
   - **Wind.** Intensity is pressure × shape factor × exposed diameter, where exposed diameter is outside diameter plus twice the insulation thickness. Apply it on marked spans only, including a sub-span exposure case.
   - Compute each intensity **exactly in rationals from the stated inputs**, with π carried as in R1. Then compute the response.
   - **Represented-input variant.** Also give the response when each binary64 input is decoded exactly but the intensity is the exact product. Report the relative difference between intended and represented inputs per quantity, as RF-FINITE does, and freeze the basis R1's rule selects.
7. **Nonzero prescribed support motion** (0.4.0 resolved):
   - a single support;
   - two supports with differential settlement;
   - combined with element loads.
8. **Cancellation across element loads (S11-F).**
   - Opposing uniform loads on adjacent spans whose fixed-end moments nearly cancel at the shared node, plus a small nodal moment there.
   - A generated seismic load cancelled by an authored uniform load of opposite sign, leaving a small net intensity.
   - Gross/net ratios 1e5, 1e7 and 1e8, with the recommended scale governed by the net load, as in RF-CANCEL.
9. **One combination** of an element-load case with a nodal-load case (sum and difference), with exactly combined expectations.

## Negative controls (derive whichever apply)

- 50/50 lumping instead of consistent loads (missing fixed-end moments).
- The fixed-end correction omitted in member recovery.
- The partial-span load integrated over the full span.
- A wrong local or global direction, or a wrong frame transform.
- The thermal or thrust sign reversed, or the eigen term omitted.
- The prescribed-motion term omitted.
- Wind applied to unmarked spans.
- Insulation omitted from the exposed diameter or from the mass.
- Nominal wall instead of the mill-tolerance effective wall.
- **The generated intensity as the binary64 left-to-right product** of the binary64 inputs (compute with Python floats in the order the records state), which is the D-14 hazard.
- The cancellation net obtained by summing binary64 contributions left to right.
- The small contribution dropped.

## Write set

`T3/REFERENCES_ELOAD/**` only. It is a separate package, and the frozen `T3/REFERENCES/**` is never touched.
- `references_eload.py`: standard library only, deterministic.
- `references_eload.json`
- `README.md`: the return, including families, derivations, adopted definitions with their basis, the independence statement, limits, and what you read and ran.
- `_run_records/`: stdout, `SHA256SUMS` and the Python version.

## Return

Write the README, then send the manager a SendMessage summary with the case count, value count, control count, file hashes and every definition question you asked. The references are frozen only when ROOT selects them after the V2-style refutation.
