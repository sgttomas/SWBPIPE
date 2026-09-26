# S0 — independent refutation of the T0R references

A fresh-context TASK checked the references, and HELP_HUMAN (ROOT) records the result here. The checker did not write the references. It derived every value from the case definitions in [DESIGN.md](../DESIGN.md) §9.1, the T0 probe, the design probe, STRESS_REFERENCE and the invented preview fixture. It used only the Python standard library, with no product code and no code from `references.py`. It compared its values with `../_run_records/references.stdout.txt` only after its own derivation was done. Its scripts and outputs are archived unchanged in [_run_records/](_run_records/). `s0_compare.py` loads `s0_derive.py` from the scratch directory the checker used, so to rerun it, point it at this folder.

**Verdict: DISCREPANCIES (definition and coverage, not arithmetic).** The checker compared 102 scalar quantities and all agree at 1e-9; the largest relative difference is 2.8e-16. The references still need these fixes before they are frozen:

1. **REF-CE has no frozen oracle.** It is in DESIGN §9.1 but missing from `references.py` and its output. The checker confirms the design's value: the support acts on the pipe with (0, +375, 0; 0, 0, 0), following DEC-049's constant nodal force along the +axis of the declared DOF. Add it, with its model definition and the balance rule.
2. **REF-B1's endpoint values name no product row.** The endpoint force rows are in the chord frame, with node-on-element signs. Either compare b and c through the tangent-frame endpoint stress rows, or freeze the chord-frame values: at b (0, 0, −1000, 707.1068, 989.9495, 0) and at c (0, 0, −1000, 707.1068, 707.1068, 0) as cut-face actions. The product's end_i rows are those values negated.
   - Freeze signed arc-frame values at every station, not only magnitudes:
     - b: T = +1200, My = −200;
     - quarter_1: T = 1047.3428460382688, My = −567.4593388673469;
     - midspan: T = 765.685424949238, My = −848.5281374238567;
     - quarter_3: T = 397.90752586283253, My = −1000.4162189843048;
     - c: T = 0, My = −1000;
     - every station: Vz = +1000.
   - Pin b-c's `y_reference = (1/√2, −1/√2, 0)`.
3. **The SPRING control cannot occur, because no node carries two devices.** Add a case where two devices share a node on different DOFs, for example a spring on UY and a guide on UX.
4. **The COMB force envelope cannot tell max from sum,** because case T has zero force. Also freeze the moment envelope: max 1000, against 1500 for a sum.

## Coverage gaps (claims with no reference)

- **Moment origin.** Every anchor sits at the global origin. Add R1 translated to O = (5, −3, 2), which must still give (−10, 26, −30; −4, 55, 52); the wrong-origin value is M = (34, 185, 152).
- **Withheld attribution and the "zero-fill a withheld support" mutation.** No case triggers `SUPPORT_ACTION_ATTRIBUTION_WITHHELD` or `CONSTANT_EFFORT_NOT_CONSUMED`.
- **Partial rigid restraints.** Freeze X1's pin (0, +57.96238445873169, −231.84953783492676; 0, 0, 0) and roller (0, −57.96238445873169, −231.84953783492676; 0, 0, 0).
- **Other quantities with no case:**
  - attributable nonlinear support reactions;
  - thermal axial force in the straight maximum;
  - stress-coverage withholding (`STRESS_RECOVERY_LIMITED`, an unavailable maximum, the headline withheld when coverage is incomplete);
  - a recomputed combination `displacement_magnitude`;
  - combined constant-effort applied load;
  - component-wise envelopes;
  - the headline tie winner (M08-L has a four-way tie) and the headline scope and domain expectations;
  - the mm/kN and mixed-modulus variants.
- **Case definitions DESIGN §9.1 leaves unstated:**
  - S1's length (1 m);
  - X1's restraint sets;
  - B1's `y_reference`;
  - the stated scale of every zero-valued expectation. The 1e-9·max(|e|, scale) criterion needs it, and on main a nominal zero reads 6.4e-15 MPa.

## Negative controls

These fail as claimed:
- M14-A; S1 unrotated; both X1 controls; M05-T; COMB magnitude algebra; CE; B1.

These discriminate only in combination:
- **M33 first-case control:** only in the A-first order, so the detection run must include A-first.
- **S1 rotated:** only paired with the unrotated case.
- **Pure torque:** does not detect the abs-sum.
- **M08-L:** detects only through the SIF row's presence (14.14 and 28.28 MPa). Its headline equals main's published value.

## Conventions checked

- **Determinacy:** every case is determinate except SPRING, which is indeterminate to degree 1 and resolved by the Euler–Bernoulli 3EI/L³ tip stiffness, matching the kernel.
- **Support convention:** support-on-pipe, in the global frame, with moments about the attachment node. This matches PP:2108-2170 and `append_signed_support_results`.
- **R1:** its permutation is proper (det +1).
- **X1 at 1e-9:** achievable, because the certified enclosure gap is ≤1e-12 relative.
