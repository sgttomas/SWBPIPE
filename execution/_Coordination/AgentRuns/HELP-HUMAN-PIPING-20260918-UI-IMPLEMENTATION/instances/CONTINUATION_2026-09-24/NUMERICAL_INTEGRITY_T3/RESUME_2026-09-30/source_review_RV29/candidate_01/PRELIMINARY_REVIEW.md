# RV29 candidate 01 preliminary source review

**NOT READY FOR FAN-IN.** The frozen candidate has three confirmed compile
blockers. ROOT has already authorized their bounded repair. No new arithmetic,
helper-accounting, gate-order or publication-lifetime defect was confirmed in
this preliminary static pass. This is not final independent implementation
review, numerical acceptance, or proof that the new tests/mutants pass.

Candidate: `3cf296e36645d97e4c657c8ad1a6322bc4163f16`. The four maintained files
are FK/src/structural.rs, FK/src/structural/retained/adaptive.rs,
FK/tests/retained_k4/publication_tests.rs, and
P/core/solver/performance_harness/tests/k6b_export.rs. `CORE.diff` preserves their
complete diff from the original accepted primitive revision d01ad98a754698631f927709d08284c272de85e8.
`REVIEW_BASIS.json` hashes the actual frozen blobs. No moving source files were
used to judge this candidate. Supporting records elsewhere in its ancestry are
not recertified by this software review.

## Confirmed findings

**RV29-C1 — BLOCKING for this frozen candidate: new tests do not compile.**
Locations: publication_tests.rs:9 and :82. `Wide::from_f64` and `Wide::ZERO`
have implementations for both the retained fixed width and K3a width2, so the
unqualified associated items are ambiguous. The independently inspected I22
implementation_b/FK_01.stderr.portable reports E0034 at both locations before
any test executes. Apply explicit `Wide::<4>` at these two sites. This is the
repair ROOT already authorized; do not change the fixture arithmetic. Backcheck
the exact repair and the successful compile on its final source identity.

**RV29-C2 — BLOCKING for this frozen candidate: new AttemptReason makes the
protected diagnostic matcher non-exhaustive.** Location:
FK/tests/retained_k4/method_tests.rs:101–116. The new PublicationEnclosure variant
is absent from the exhaustive match, producing E0004. ROOT's additive grant
at COORD `01711fb955`, R/BRIEFS/A1_CHECKPOINT_B_COMPILE_ADDENDUM.md authorizes
only a distinct diagnostic arm naming the layout index and predicate. Preserve
all existing arms, expectations, fixtures and assertions; do not introduce a
catch-all. This fifth-path repair is pending review, not part of candidate 01.

Both findings are compile compatibility defects, not evidence of a numerical
failure. RV29 did not execute the compiler. The inspected sealed author compiler
log and frozen source establish the issue; subsequent author working files and
repair outcomes have not been reviewed or accepted here.

## Independent absolute-work result

`LEDGER_SHA256SUMS` was written before reading author tests or new helper counters.
Its manifest hash is `8fe24165f9905950280a78c672bada583d819d3946b95abcaa28d574ff38209b`.
It remains intact after the source review. Exact integer/Fraction assertions and
a byte-identical scratch rerun passed. No new helper counter observation supplied
an expectation. The original price primitives are unchanged in this candidate.

- Successful H=1+h, b=2 algebra path: **18,103 LME**, including H formation,
  exact final predicate and RU radius conversion. The b=1 numeric-rejected path
  costs **159 LME**, stopping before radius conversion.
- Small-A1 row_bound(h,h)=3h: **17,563 LME**, including the three-term sum,
  clones, both exact correction comparisons and conversion context.
- Explicit helper Span and Exponent controls retain **4** and **42 LME** on
  their specified routes. These differ from earlier primitive-only controls
  because the frozen helper's sign checks/formation are now included.
- The exact checkpoint prefixes and external one-below budget choices are
  frozen in LEDGER.json. There is no distinct budget that first stops at a
  zero-cost final check after the same cumulative total was already checked;
  tests must report the earlier actual checkpoint. No new checkpoint or price
  is proposed.

The code matches the accepted local accounting model statically: H and predicate
sums are collected on success/refusal; clone snapshots exclude inherited counters;
reaches collects fresh scratch; context operations are charged before their result;
round_up merges on every local exit. `checked` preserves arithmetic Err over a
later budget check. The candidate merges certificate total, context work and sum
work once into stop_rule, case and invocation before branching on the result.
Only runtime exact-ledger equality and executed PM16 omission/duplication controls
can close this obligation. The existing >0 counter test does not do so.

## Numerical and control trace

The implementation uses final binary64 x, matching verification v, and the full
R7 error terms. Translation/rotation use W_plus; DisplacementMagnitude additionally
adds |v|*2^(1-P). Every force/moment family uses 69*2^-P E_q + W + 2^-P W + C.
P is report.precision, already2p. The source map covers end/station actions,
global/directional springs, reactions and support force/moment magnitudes because
the canonical recover layout classifies them as Force/Moment. The inspected
verify.rs:1138–1183 supplies the corresponding W_plus and charge entries; missing
or signed-negative required fields refuse rather than becoming zero.

H is exact |x-v| plus those nonnegative terms. The absolute predicate forms exact
b-H. The relative predicates require exact |x|-10^9H, the exact sharper dyadic
allowance minus H, and the separately rounded binary64 sharper allowance minus H.
The a0/a1/u0/u1/a2 operation order and constants agree with the selected addendum.
Nonfinite allowances and negative zero are not accepted as usable radii. No
nearest-rounded H or relaxed allowance is substituted into admission.

Existing R7 comparison executes before certificate admission. The new gate does
not replace its mathematical prerequisites. PublicationEnclosure rejects and
uses the original verification-as-next-candidate schedule; p512 rejection reaches
Ceiling. Span/Exponent/certificate issues remain terminal. InputDerived rows keep
exact prescribed publication and absence sentinel; range-only rows retain their
outcome/class and do not gain a fabricated zero certificate.

The common publication_with function preserves original O9 maxima membership,
original-operand coupling, floors and class/bound logic. Candidate prescribed
values are replaced before this publication, exactly once per candidate reaching
the new gate. Finalization moves the certified Publication and radius slice and
does not reround/reclassify them. Failed or rejected draft/radii locals drop on
return. Combinations create their own CasePrep and call run_schedule, which runs
the new certificate; they do not inherit another selected solve's radius indices.

BoundVerification binds the report to the prep and verification Arc. Validation
checks precision pair, report/layout lengths, canonical layout identity and finite
metadata. The private radius accessor additionally checks source identity,
precision, row metadata, class/sentinel compatibility and finite radius ceiling.
No report or ExactWideSum is retained per row. RetainedSolve Clone clones its Box
payload. The accessor remains private pending the separately authorized facade
integration; this is consistent with the bounded kernel slice, not proof of F2a.

## Reach and per-slice gate applicability

The frozen search in REACH.txt identifies retained_api users in H and numerical
validation. It finds no product/native caller. The structural.rs change only adds
CertificateIssue and PublicationPredicate reexports inside retained_api. The
outer legacy `structural::POLICY` remains `M03-INTEGRITY-v1`; only the separate
retained policy becomes `M03-INTEGRITY-MP-v2`.

The legacy formation_check module uses retained::wide; sparse uses the unchanged
seeded F08 hook. Neither path reaches adaptive.rs and neither shared primitive
was modified. The fourth maintained file is an H test, and the new FK publication
tests are cfg(test). Therefore this exact delta has **kernel/harness execution
reach**, with no identified product/native behavior change. It does not activate
F2a or change legacy solver selection. Under ROOT's stated per-slice distinction,
T9/both-entry product-route witnesses are not triggered solely by this delta's
retained path. Required shared-FK compilation, kernel/H/validation evidence,
source review, CI and other applicable merge gates remain. This is a source-based
reach classification, not an executed native nonregression claim.

Existing H/validation record code formats typed reasons with Debug and receives
the new enums through the retained API; it does not silently rename v1 receipts
as v2. Product receipt/readers and facade certificates are absent from the changed
execution path and remain held. No policy-registration or reader-parity acceptance
is inferred from changing a string constant and exporting enums.

## Outstanding verification and next step

The current authored test file is a useful checkpoint, not the complete required
acceptance matrix. It lacks the frozen absolute ledger assertions, exact short-
budget stop positions, both executed PM16 patches, and the complete required
numeric/control mutant and ordinary-outcome evidence. Many value checks compare
new and old bound helpers; that is a regression comparison, not an independent
absolute-work oracle. Three source-control constructors have no actual result
assertions yet and require exact released source IDs/truth.

Before final review: backcheck the three bounded compile repairs and their exact
candidate hashes; then inspect authorized compile/focused test outputs. Subsequent
bounded grants still need independent bare-b/source comparisons, all-row/identity/
relative-boundary/sentinel coverage, zero/floor/O9/threshold controls, rejected and
terminal/budget accounting equality, old R7/A1/A2 mutant obligations plus new
mutants, combinations/replay and protected ordinary-selection outcomes. The new
gate may mask an old mutant; a surviving required mutant is not a pass. A safe
new refusal cannot silently rewrite a protected availability expectation.

K6c remains a separately owned required reconciliation: include provisional values,
Publication/radii, layout-validation allocation, simultaneously live context/sums/
clones/scratch, report/prep/state Arcs, clone multiplicity and rejection overlap.
Logical8Q radius payload is not an allocator/RSS bound. No resource, availability,
all-gates or final acceptance claim is supplied by this preliminary source review.
