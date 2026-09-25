# Requested proposal: checked exact-ratio publication bridge

**Recommendation:** the parent's outward interval approach is sound for a bounded binary64 projection if every enclosure operation is outward controlled and the ratio retains its exact numerator/positive denominator source. Add an exact identity shortcut first. This is a prospective design contribution, separate from the reviewed implementation: current Ratio::approximate_projection remains unqualified.

## Preconditions and meaning

Let N and D be the admitted exact expansion values, with D>0 already proved by Context. Let x=N/D. Arithmetic must use IEEE binary64 nearest rounding and gradual underflow, with correctly rounded scalar addition/division and next_down/next_up stepping to adjacent representable values. Reuse the checked expansion product/range handling for exact identity checks; unsupported arithmetic or exhausted budgets remains unresolved.

The bridge can prove a numerical error enclosure for a chosen finite q relative to this declared represented-source ratio. It cannot establish correct rounding, unrecorded primitive engineering fidelity, or accuracy of the underlying physical model. Do not replace exact contact signs with the projected value or reinterpret existing policy thresholds.

## Sound algorithm

1. If exact numerator is zero, return q=0 with exact-zero evidence and zero error. Zero is never inferred from a rounded sum or quotient.
2. Form any finite projection proposal q using the existing approximate operation or safe scaling. If q is nonzero with the exact ratio's sign, try checked exact expansion evaluation of N−qD. Exact zero certifies q=x and zero error. This shortcut recovers admitted exact dyadic values, including minimum subnormal and maximum finite values that blanket outward widening can otherwise reject. If the arithmetic cannot establish equality, retain unresolved status for this shortcut; do not assume equality.
3. Enclose each expansion sum. Start L=U=0, and for each exact represented term t update L=next_down(fl(L+t)), U=next_up(fl(U+t)). The resulting [N_L,N_U] and [D_L,D_U] contain the exact sums by induction and monotonic addition. Reject unsupported/nonfinite intervals and require D_L>0. A known exact-single-term optimization may avoid unnecessary zero-plus-term widening, but it is optional and needs its own traced exactness.
4. Compute all four endpoint quotients N_L/D_L, N_L/D_U, N_U/D_L, N_U/D_U. Let Q_L be next_down of their finite minimum and Q_U next_up of their finite maximum. This encloses x. Equivalently, sign-specific endpoints may be used: a negative numerator reverses which denominator gives the lower quotient. Applying the positive-numerator rule to negative reactions is unsound.
5. Require finite endpoints and an interval excluding zero for nonzero numerical publication. Choose finite nonzero q with the exact sign inside [Q_L,Q_U], such as a checked/clamped proposal. Do not fabricate a sentinel or call an interval crossing zero a contact tie.
6. Compute e_abs=max(next_up(fl(q−Q_L)), next_up(fl(Q_U−q))). Since q lies inside the interval, both distances are nonnegative and these outward distances bound |q−x|. Compute e_rel=next_up(fl(e_abs/min(|Q_L|,|Q_U|))). The denominator is a positive lower bound on |x|, so e_rel bounds relative error. Final subtraction/division rounding must be included; an ordinary computed radius or ratio is not enough.
7. Compare the conservative bound against the unchanged applicable criterion, e.g. relative 1e-9 for these existing analytical quantities. Preserve exact-zero handling separately. If the interval, sign, range, work budget or criterion fails, return explicit unresolved qualification. A failed bound is not proof of physical instability.

For a combined absolute/relative criterion, use the owning criterion's actual same-dimension scale; do not introduce a universal floor. A nonzero quantity may have a very small but accurately representable projection; that does not justify changing the comparison basis.

## Edge cases and optional improvements

- Unconditionally widening an exact min-subnormal sum can create a zero-crossing interval, and widening maximum finite can produce infinity. The exact N=qD shortcut avoids false rejection when the exact answer itself is representable.
- Half a minimum subnormal has no acceptable nonzero binary64 projection under relative 1e-9. It must remain unresolved. An exact zero and an underflowed nonzero ratio are different outcomes.
- Subnormal arithmetic is not inherently unsound for outward intervals under gradual underflow: adjacent-float bounds include absolute underflow rounding. A conservative implementation may reject the entire subnormal envelope, but should state that scope. Where admitted, enforce the actual relative-error bound; never use a pure relative-roundoff model that ignores subnormal errors.
- Near overflow, exact identity may resolve representable answers. Otherwise, jointly rescaling N and D by a reversible radix factor can improve range if **all** retained terms survive; losing a tiny term is forbidden. If no safe scaling/enclosure exists, return unresolved.
- Nonoverlapping normalized expansion terms already remove the huge cancelling terms before publication. Compute intervals from these exact final terms, not by interval-evaluating original K*u−f; the latter can erase useful tiny-reaction accuracy through dependency/cancellation width.
- Account for each proposal/predicate/projection's attempted and charged work, including failed attempts. The current Response::operations value omits extra predicate/candidate work; it must not become a total-work claim. Resource counters should state whether they count actual arithmetic or conservative precharged work.
- Return a private/source-bound qualified projection record carrying q, the exact ratio/sign association, interval/error bound and qualification basis. Final producer/receipt checks must consume that same response; re-evaluating reactions from rounded u defeats the bridge.

## Independent tightness probes

[_run_records/independent_review.py](_run_records/independent_review.py) implements this **proposal** in Python, with exact Fraction checks that every interval contains N/D and every reported error bound dominates actual error. It obtains raw numerator/denominator expansions from a checked transcription of the new kernel ratio formulas, compares the full responses against independent general rational elimination, and uses every state reached in the frozen neighbor references.

[Raw output](_run_records/independent_review.json):

| Probe | Result |
|---|---|
| 27 input triples × 2 senses × 4 seeds | 216 exact reference traces, 434 state rows |
| All three displacements and reactions per row | 2,604 projection attempts |
| Exact zero | 966 |
| Exact representable shortcut | 1,138 |
| Qualified outward interval | 500 |
| Largest certified relative upper bound over those references | approximately 6.67e-16, below unchanged 1e-9 |
| Exact −2^-60 force-tail reaction | Preserved exactly |
| Minimum subnormal / 1 | Exact shortcut, zero error |
| Minimum subnormal / 2 | Unresolved |
| Three minimum subnormals / 2 | Unresolved |
| Maximum finite / 1 | Exact shortcut, zero error |
| Maximum finite / 0.5 | Unresolved overflow |
| 1/3 | Enclosed, relative bound approximately 6.67e-16 |
| 1e-310 / 1.1 | Gradual-underflow interval qualifies with relative bound approximately 1.64e-13 |

These results establish feasibility/tightness for the bounded references and validate the Python proposal's arithmetic against exact oracles. They are not observed Rust bridge behavior, exact-rounding guarantees, dense/sparse backend execution, or full product acceptance. Implementation must be frozen, independently code-reviewed and executed on these controls plus negative-sense, bound-mutation, source-rebinding and budget-failure cases before qualification.

