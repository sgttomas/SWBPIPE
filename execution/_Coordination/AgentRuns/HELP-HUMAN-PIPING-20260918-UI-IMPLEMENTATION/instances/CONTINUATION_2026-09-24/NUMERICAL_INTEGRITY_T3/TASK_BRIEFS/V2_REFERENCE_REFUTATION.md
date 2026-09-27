# V2 — independent refutation of the T3 references

Fresh-context refutation TASK. Read `_COMMON.md` first. You are not R1's author, you have not advised R1, and you have not read either T3 design in depth.

## Purpose

Try to break R1's references before ROOT freezes them. They will judge the implementation, so a wrong expectation, an unjustified zero scale, or a negative control that a defective implementation could pass must be found now. This follows T0R's S0, S0′ and S0″ checks (`DEFAULT_ROUTE_DESIGN/REFERENCE_CHECK/`).

## Inputs (read-only)

- `T3/REFERENCES/**` at the commit the manager gives at spawn: `README.md`, `references.py`, `references.json`, `_run_records/`. Verify `SHA256SUMS` first.
- `T3/TASK_BRIEFS/R1_REFERENCES.md` and `T3/TASK_BRIEFS/R1_ADDENDUM_CANCEL.md` (what R1 was asked for).
- `T3/STAGE0_MAP.md` and `T3/MANAGER_NOTES/S11_MAP.md`, for context only.

## Independence

- **Do not read product source** (`P/core/**`, `P/apps/**`), and do not import production code.
- **Re-derive by a different method from R1's.** Where R1 used closed forms or flexibility, use an exact-rational direct-stiffness assembly written from Euler–Bernoulli theory, and the reverse where R1 used stiffness. Do not reuse R1's helper functions. You may read `references.py` to understand its inputs, but derive the expectations with your own code.
- Use standard-library Python only (`fractions`, `decimal`).

## What to check

1. **Values.** Re-derive every family (RF-CHAIN, RF-SKEW, RF-WEAK, RF-LARGE, RF-INVARIANCE, RF-RANGE, RF-ZERO, RF-FINITE, RF-MECH, RF-CANCEL) at a sample that covers every case type. Re-derive all of RF-CANCEL and RF-MECH, because they are the negative-control-critical families. For RF-LARGE, check the smallest and at least one large size fully, and the rest by stated invariants. Report agreement counts per family.
2. **Zero scales.** Is each zero scale derived and justified? Two checks against loosely chosen scales:
   - Is any scale so large that a clearly wrong answer passes `|obs − exp| ≤ 1e-9·max(|exp|, scale)`?
   - V1-S8: D1's stop rule guarantees 1e-9 only for |q| ≥ about 5.4e-11·S\*, where S\* is the connected body's scale for that kind of quantity. Flag every nonzero expected quantity below that floor, so the implementation cannot pass a weak-coupling case by construction.
3. **RF-CANCEL scales.** For each case, check whether the gross or the net load governs the scale, and whether R1's recommendation is defensible.
4. **Negative controls.** Every listed wrong answer must actually fail the comparison under the stated scale. Also construct at least one plausible defect per family that R1 did not list, and check that some expectation catches it. Report any defect that no reference catches.
5. **RF-FINITE.** Check the intended-input against represented-input analysis. Where the two differ by more than 1e-9, the represented-input basis must be the frozen one.
6. **RF-MECH.** Verify each stated null motion: it satisfies every restraint and gives zero strain energy.
7. **Formulation.** Check the theory statements: Euler–Bernoulli, no shear deformation, small displacement, the section constants, units, and the invariance transformations (rotation matrices orthogonal, unit factors exact).
8. **Reproducibility.** `python3 references.py` regenerates `references.json` byte-for-byte, and the hashes match.

## Write set

`T3/REFERENCE_CHECK/**` only: `RETURN.md`, your derivation scripts (`.py` is fine; this is not a crate), their outputs, and `SHA256SUMS`.

## Return

`T3/REFERENCE_CHECK/RETURN.md`:
- a verdict: REFERENCES CONFIRMED, FINDINGS or REFUTED;
- per-family agreement counts;
- a findings table: id, severity (BLOCKING, SHOULD-FIX or NOTE), case and quantity, evidence, and the required change;
- the undetected defects you constructed;
- what you did not check.

Then send the manager a SendMessage summary.
