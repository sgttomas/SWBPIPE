# RV29 expanded-test review — f118

**Useful expanded coverage; not complete PC/PM closure or final acceptance.**
Read-only review of f1183b94624f4d1c84db5d6bb6d593694d2cba66, test hash
39fa1d4fd016d58eef9b8878ca48c117c3af3aa3285e741bdd6c67e0e58a3b59.
Only publication_tests.rs changes from dd1. Production helpers and original price
primitives are unchanged. No Rust, probe, mutant, maintained edit or old-golden
patch was run/authored by RV29. TESTS.diff and BASIS.json bind this review.
Line references below are to the frozen publication_tests.rs unless stated.

## Actionable test finding

**RV29-T02-1 — SHOULD-FIX before claiming the prescription discriminator.**
Lines722–725 say that (1+2^-100)-1 exposes a tail below retained spacing. But
1+2^-100 needs101 significant bits and is exactly representable at every
candidate precision128/256/512. Premature retained rounding therefore preserves
the same answer and can pass this test. Replace only this new fixture's tail
and expected result with a fixed tail that the candidate precision loses;
2^-600 is binary64-normal and below the half-ulp2^-p at1 for all three candidate
precisions. Exact evaluation gives2^-600; intermediate candidate rounding gives0.
Keep the separate magnitude subcheck unchanged. ROOT adopted this bounded repair
in COORD469c22efbd, A1_PRESCRIPTION_DISCRIMINATOR.md. The future authored repair
still needs frozen backcheck; f118 remains the reviewed old fixture.

**Stale comment, non-blocking:** lines440–442 still say independent truth was not
supplied and no test is attached to the three constructors. The later test at1094
and released truth contradict that. Correct the comment to distinguish supplied
truth, conditional selected-branch checking, and external output verification.

## Required discriminators still open

| Obligation | What the new tests actually establish | Concrete remaining check |
|---|---|---|
| PC44–46 rejection schedule | pc44_46 at1039 exercises zero-source replay and combination. C17 asserts first p128 rejection; extra cases allow Ceiling with any publication rejection. | Assert verification reuse after publication rejection, stable first failed row/predicate, no Accepted/Verified on failure, and the actual p512 ceiling sequence. Reuse C17 and a fixed extra case that reaches Ceiling, or one bounded controlled report fixture. Do not add a source sweep or infer accuracy from nonselection. |
| PC36–39 / PM15 pairing | pc36_39 at932 compares final draft bits, source/precision/tag errors and prep Arc counts, but every accepted radius is zero. | One nonzero, unequal-radius draft must detect radius zeroing/permutation and altered final publication/class/bound. Reuse a controlled pair/floor fixture; an all-zero vector cannot distinguish a swapped or zeroed positive radius. |
| Partial draft/failure | The rejection changes the first eligible W_plus; room0 stops before any completed radius. Arc count tracks prep ownership, not allocation of the partial Box. | Exercise a later-row rejection and conversion stop after an earlier radius exists, then a fresh accepted draft to exclude stale partial reuse. Source/K6c inspection should establish release; no new allocation-monitor framework is needed. |
| PC31–35 exact admission versus stored RU | Earlier zero/below-h/normal/max/overflow tests cover RU itself; pc09 decimal equality is dyadic and invokes only publication_predicate. | Carry an H that passes an exact non-binary64 relative threshold while RU64(H) exceeds that threshold through certify_rows. This catches incorrectly rejecting by the rounded private radius instead of exact H. |
| PC40–43 integration / PM16 | Independent local prices and guard prefixes are now explicit. | Finish observed rejection/small-bound assertions after the span correction; execute the two concrete PM16 patches; reconcile local work with actual attempt/stage/case/invocation totals on success, new numeric rejection and stopped paths. Reuse the existing stage/component reconciliation, not a second accounting framework. |

These are missing verification, not newly confirmed production defects. Prefer
one shared finite controlled fixture for the radius/failure boundaries and existing
source attempt evidence for scheduling. Do not generate mirror tests for every
field or duplicate the existing external comparator.

## Coverage that is substantive

PC01–04's exact H tests cover all15 closed row-family/kind combinations at
P256/P512/P1024, with every formula term nonzero, plus final-x rounding. Exact
sum equality detects each single term omission without a separate copy of the
formula implementation. PC05–08 exercise below/equal/above bare b, zero b,
positive H at x0 and a qualified-radius counterexample.

PC09–12's fixed independent constants are correct. At x=S=1,
A_exact=A_f64+h with A_f64=4297064449*2^-85. At x=S=3*2^-1022,
A_exact=21481127939*2^-1107 < A_f64=3h, and the specified intermediate H lies
strictly between them. The decimal equality/above-boundary discriminator is exact.
RV29 independently checked these dyadic equations using standard-library Fraction.

Threshold, O9/input-derived membership, floor precision/kind, exact prescribed
range, report shape/source/precision and bad radius encodings have useful checks.
The four new power-of-two coupling examples alone do not distinguish all
sequential-rounding variants; use the existing409-set independent classification
corpus and required PM18 execution before adding another coupling test. Do not
claim changed-max-winner/source coupling coverage from those four algebra cases.

PC40's18103/159/17563 totals, both full successful sum tuples, context17506,
and span1076/1075/2 match RV29's frozen ledger plus its source-derived correction.
PC41 tests every priced H checkpoint one below in Case and Invocation scopes,
plus equality progression and the small-bound Case prefixes. Existing common
StageGuard tests cover scope precedence; repeating every generic combination
would add little. PC42 correctly prices Span4 and Exponent42, including earlier
budget stops and arithmetic-stop precedence. Duplicate zero-cost final checks
are explicitly not presented as separately budget-addressable checkpoints.

The frozen I22/verification_c return is historical pre-correction evidence:
22 passed/1 failed at the old span assertion. That PC40 run never reached its
later rejection and small-bound tuple assertions. Reading the repaired f118
source does not turn those previously unreached assertions into executed passes.
Later run evidence needs its own candidate/hash binding.

## Independence and source-outcome boundary

The extra_truth equations agree with all116 rows of the released independent
TRUTH.json: FM/MF use5h/4 displacement/rotation and signed actions; ZR uses5h/4
translation with signed5h actions. The expected physical values are not solver
outputs. However, lines1123–1129 feed the computed error into the same production
publication_predicate used for admission. This is a consistency test of an
independently supplied truth, **not an independent implementation of the final
predicates**. A permissive predicate defect can pass both paths. Row count and a
single load ID also do not replace the external full source/row-identity check.
Keep this useful unit test, but count the sealed external bare-b comparator as
the independent all-row claim check; do not build a duplicate Rust oracle here.

The Ceiling arm correctly makes no numerical-accuracy claim. A green result can
mean zero selected rows were checked for that case. Record the actual selected
versus Ceiling branches when integrating probe evidence. ROOT reported20 runs
completed with19 selected/one named Ceiling, packet pending; that report is not
promoted to independently reviewed matrix evidence in this packet.

The separately frozen C17.RETURN.md supplies an actual independent output check:
36 rows pass, selected256/verified512, all direct-rounded bits match frozen truth,
and all four relative rows pass the public and both sharper predicates. This
closes that released output's claim check only. It is separate from a passing
production-predicate unit test and does not qualify the full repair or F2a.

## PM disposition and return

All PM01–18 remain actual-execution obligations unless a separate sealed kill
packet supplies the concrete patch, intended failure and restored control.
The new exact-H, threshold, relative, radius and ledger tests provide plausible
finite discriminators for most of them; that is not a mutant kill. PM15 still
needs the nonzero paired-radius/final-output boundary above. PM16-a omission and
PM16-b full-counter duplication have independent absolute expectations already;
execute both rather than relying on snapshot drift. PM17/18 should first use
existing O9/floor/classification evidence. Preserve old R7/A1/A2 criterion-specific
checks even if the new gate masks their failures.

The old-golden adaptation and S11 reconciliation remain I22/ROOT work; RV29 did
not author either patch. This packet records one concrete test defect, minimal
remaining discriminators, validated independent constants/truth mapping and
claim boundaries. Final acceptance remains pending repaired-test review, actual
coverage/mutant evidence, the external matrix, K6c and exact-candidate gates.
