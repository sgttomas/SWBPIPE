# S0″ — narrow check of the revision-3 reference changes

HELP_HUMAN (ROOT) records this check. The same independent checker derived the changed values first, using the standard library only and no code from `references.py`, and then compared them with the author's. Its scripts and outputs are archived unchanged in [_run_records/](_run_records/); they refer to the checker's scratch directory.

**Verdict: REFERENCES CONFIRMED.** Of 50 compared values, none disagrees; the largest relative difference is 2.0e-15. The checker confirmed:

- **The revision-3 changes:**
  - COMB-2, split into A1/T (1000 against 1500) and A1/A2 (1000 against 2000);
  - I-L's two labelled axial variants;
  - I-T with Fx = +800 at c (header 13.963 MPa, branch 29.160 MPa; the old 7.400 MPa kept as a control);
  - B2's chord-frame cut face at b for k = 1, 2 and 4, with end_i as its negation;
  - REF-M14-TH's moment zero scale, |N|·ro = 49762.83 N·m.
- **The pinned JSON:** the nonlinear definitions for NL-C3, SPRING-GAP and ATTR match the product's parsers (PP:411-430, 4155-4166, 10868-10894) and its gap law. The gap values are in metres.
- **The ATTR exemption:** its assertions catch the zero-fill defect, and it is the only exemption from the global-balance rule.
- **Unchanged values:** the revision-1 block still re-serializes to `f3c8e2f9…`. The only revision_2 values that changed are the three intended ones. Both earlier archives verify.

Optional wording, not blocking the freeze:

- **ATTR:** "no zero-filled tip rows" could read "no `support_reaction_*` rows for either tip device". That would also cover copying the nodal total to both devices, which the reader rule already catches.
- **B2:** the zero chord components rely on the general zero-scale rule.
