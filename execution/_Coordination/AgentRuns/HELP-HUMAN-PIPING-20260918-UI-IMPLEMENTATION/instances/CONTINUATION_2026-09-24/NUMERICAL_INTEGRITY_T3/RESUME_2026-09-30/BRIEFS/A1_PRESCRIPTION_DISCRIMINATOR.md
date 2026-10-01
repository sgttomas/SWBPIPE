# ROOT new-test discriminator correction

RV29's frozen expanded-test review identified a genuine coverage defect in
pc17_20_exact_prescription_and_magnitude_term. The existing tail2^-100 is
representable beside1 in128-bit arithmetic, so it does not establish the stated
below-retained-spacing distinction. This is a SHOULD-FIX in the new test, not
permission to edit a historical fixture or claim a production defect.

ROOT independently checks the proposed fixed algebra: at1 the p-bit half-ulp
is2^-p. Tail2^-600 is smaller for every retained candidate p=128,256,512, yet
is a normal representable binary64. Exact (1+2^-600)-1 is2^-600; an intermediate
rounding at any of those candidate precisions loses the tail and yields0.

I22 may replace only that new exact-prescription fixture's tail/expected value
with2^-600 and state the precise candidate-precision coverage. Add a compact
wrong-rounding discriminator if needed using existing private arithmetic types;
no new raw model/case, production helper, old fixture or oracle change. Keep the
magnitude-error subcheck intact. Run the affected new-test filter once under
existing C settings, preserve old evidence, and record the changed test hash.
RV29 confirms the repair on the later frozen candidate. C's time boundary remains.
Its other reported coverage gaps remain open until its complete review and the
finite mutation/source-control evidence are reconciled.
