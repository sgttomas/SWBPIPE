# V3: independent refutation of the RF-ELOAD references

This is a fresh-context refutation TASK. Read `_COMMON.md` first.

You must be independent: you are not the RF-ELOAD author, you have not advised it, and you have not read the T3 designs in depth. This follows V2's refutation of R1 (`TASK_BRIEFS/V2_REFERENCE_REFUTATION.md`, `REFERENCE_CHECK/`), under ROOT's D-11. The references must land, be refuted and be selected before slice F3 (W1b).

## Purpose

Try to break the RF-ELOAD references before ROOT freezes them. They will judge W1b's element, eigen, support-effort, prescribed and generated loads. Find now:
- any wrong expectation;
- any unjustified scale;
- any wrongly adopted definition;
- any negative control that a defective implementation could pass.

## Inputs (read-only)

- `T3/REFERENCES_ELOAD/**` at the commit the manager gives at spawn (the candidate was first committed at `2633c67fb`; a README-only note on verifying the checksums may follow). The files are `README.md`, `references_eload.py`, `references_eload.json` (sha256 `c20eb4c4…`) and `_run_records/`. Verify `SHA256SUMS` first, run from the `REFERENCES_ELOAD/` root, because its paths are relative to that root.
- `T3/TASK_BRIEFS/R1_ADDENDUM_ELOAD.md`: what the author was asked for.
- `T3/MANAGER_NOTES/RF_ELOAD_DEFINITIONS.md` (`28d96084f`): the implemented-theory definitions the manager gave, which ROOT accepted, including ROOT's self-weight ruling (`ROOT_RULINGS_V1.md`, after `28d96084f`).
- For conventions only: `T3/REFERENCES/**` (the frozen R1) and `T3/REFERENCE_CHECK/**` (V2's method).
- Permitted theory records, as for the author:
  - `P/validation/hand_calcs/mechanics/*.md`;
  - `P/docs/validation_manual/cases/mechanics/*.md`;
  - T1's `LOAD_STATE_IMPLEMENTATION/ANALYTICAL_REFERENCE/**`.

## Independence

- **Do not read product source** (`P/core/**`, `P/apps/**`), and do not import production code.
- **Re-derive by a third route, different from both of the author's routes.** Routes A and B share one author, so their agreement is not independent evidence; your route is the independent one. Re-derive enough values to challenge both. Its route A is Euler–Bernoulli tree integration plus the force method; its route B is exact direct stiffness with consistent Hermite loads and fixed-end-corrected recovery. Use, for example:
  - energy methods or Castigliano, or unit-load virtual work;
  - or a direct-stiffness code written from scratch with a different formulation, such as exact element flexibility inverted, or loads applied by shape-function integration you derive yourself.
  Do not reuse the author's helper functions. You may read `references_eload.py` for its inputs only.
- Use standard-library Python only (`fractions`, `decimal`). Carry π to at least 100 digits, or symbolically.

## What to check

1. **Values.** Re-derive every family at a sample covering every case type. Re-derive fully:
   - every cancellation case (CANCEL-FEM, CANCEL-SEIS, CE-CANCEL);
   - every generated equivalent-static case, both intended and represented;
   - every prescribed-motion case.
   Report agreement counts per family.
2. **Definitions.** Check that each adopted definition matches the manager's answers Q1–Q11 (README §2). Where a hand-calc record states the theory, check the case against it too. The key items:
   - the thrust effective axial force N = EA·ext/L − pA;
   - one effective wall for stiffness, thrust area and mass;
   - wind per unit member length, along a global axis, with no projection;
   - the consistent partial-span integration;
   - ε* for legacy and 0.4.0 thermal;
   - constant effort in every solved case;
   - algebraic combinations.
   Report any definition the manager's answers do not support.
3. **Scales.** Is each zero scale and nonzero scale derived and justified? Specifically:
   - **The author's own rule** (README §4): the translation zero scale drops R1's F·L³/(3EI) term, and adds moment·L²/2EI only when moments are nonzero. **Rule whether it is sound or merely convenient.** Derive what the scale should be from the case's load and geometry, and compare. Is any resulting scale so small that a correct binary64 implementation fails, or so large that a clearly wrong answer passes `|obs − exp| ≤ 1e-9·max(|exp|, scale)`? Check specifically whether it could hide any negative control, listed or constructed.
   - **V1-S8:** D1's stop rule guarantees 1e-9 only for |q| ≥ about 5.4e-11·S\*. Flag every nonzero expectation below that floor.
   - **Cancellation cases:** check whether the gross or the net load governs each quantity. That covers CANCEL-FEM's single net-governed value and 15 mixed values, and whether the gross scale really hides every float-sum and product control there.
   - **Consistency of the net-governed scale.** The shared-node rotation in the FEM cancellation (author's finding 4) is net-governed. Check that this scale is applied consistently everywhere it appears (the JSON, every control evaluation, the README tables), and that a gross-governed scale is never quietly substituted for it.
4. **The represented-input basis.** For CANCEL-SEIS-G1e7 and -G1e8:
   - confirm that decoding the binary64 inputs moves the net by more than 1e-9 of scale (the author reports 1.12e-9 and 1.27e-8), so freezing them on represented inputs is consistent with R1's rule (R1_REFERENCES.md, RF-FINITE);
   - **confirm that the D-14 hazard control (the binary64 left-to-right generation product) still discriminates against the represented basis** (the author reports 9.1× at 1e7 and 90.9× at 1e8);
   - check the rule was applied to every case, not only these.
5. **Negative controls.**
   - Every listed discriminating control must actually fail under the stated scale.
   - Audit each of the 23 that do not discriminate (README §7). For each, check the reason given and **decide: retire, re-scale, or keep with a label** (for example "documents a benign order"). Give the decision per control in a table.
   - Construct at least one plausible defect per family that the author did not list, and check that some expectation catches it. Report any defect nothing catches.
   - **Think in particular about the D-14 hazard.** The author reports that without cancellation, the binary64 generation product is about 1e-7 of the criterion, so it is undetectable at 1e-9. Say whether any reference case could discriminate exact generation from the binary64 product, or whether that property can only be tested at kernel level (by inspecting the source term). Recommend accordingly.
6. **Sterbenz and orders.** Confirm the author's claim that two-term float summation of the nearly cancelling values is exact in CANCEL-SEIS (so NC-FLOAT-SUM cannot discriminate there), and that the three-term orders listed do lose the net.
7. **Formulation.** Check:
   - Euler–Bernoulli with no shear, small displacement;
   - the section constants on the effective wall;
   - units;
   - the rotations used (orthogonal, exact);
   - equilibrium of every case.
8. **Reproducibility.** `python3 references_eload.py` regenerates `references_eload.json` byte for byte, and the hashes match.

## Write set

`T3/REFERENCE_CHECK_ELOAD/**` only: `RETURN.md`, your derivation scripts, their outputs, and `SHA256SUMS`. Make no Git writes.

## Return

Write `T3/REFERENCE_CHECK_ELOAD/RETURN.md`, containing:
- a verdict: REFERENCES CONFIRMED, FINDINGS or REFUTED;
- per-family agreement counts;
- a findings table: ID, severity (BLOCKING, SHOULD-FIX or NOTE), case and quantity, evidence, and the required change;
- the undetected defects you constructed;
- your D-14 recommendation;
- what you did not check.

Then send the manager a SendMessage summary with the verdict and the file hashes.
