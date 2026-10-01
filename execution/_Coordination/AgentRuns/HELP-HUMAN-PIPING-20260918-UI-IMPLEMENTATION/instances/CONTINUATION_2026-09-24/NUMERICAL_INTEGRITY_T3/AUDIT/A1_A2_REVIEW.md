# Design-level check of D1 revision 5a.3 amendments A1 and A2

Basis, abbreviations and authority: `REPORT.md`. Independent audit; no design
selection or source repair. The accepted R7 text and ROOT's amendments were
read separately from the slice reviewers' conclusions.

## A1: the missing coupling error

A1's local triangle-inequality argument is sound if its supplied S* safely
represents the verification scale. Its upward sum contains the retained-value
allowance, the row's own binary64 rounding allowance and one smallest
subnormal. That last term is enough for the row's direct subnormal rounding.

The unresolved step is earlier. `adaptive.rs:1904` builds the verification
scale from retained values. `adaptive.rs:2598` first rounds rows to binary64
and then couples their maxima through `coupled_scales` at `:296`. A subnormal
row's rounding error occurs **before** multiplication by L or division by L.
The absolute error must therefore be multiplied/divided too.

R7 `:342` asserts relative closeness `2^-64 + 2^-52`. RV19's extended argument
at `REVIEW/K4_REVIEW.md:456` adds only `2^-1075` absolute for subnormal
publication and coupling. Neither form accounts generally for that amplification.

### Exact arithmetic counterexample

Let `h = 2^-1074`, the smallest positive binary64 number. Take a non-input-derived
rotation whose candidate and verification values both equal `r = (5/4)h`.
It rounds to the nonzero binary64 value `h`; it is not an unpublishable row.
Let the body's extent be exactly `L = 2^100`, with zero raw translation scale.

- Verification translation scale: `S_v = Lr = (5/4)2^-974`.
- Published translation scale: `S_pub = fl(L fl(r)) = 2^-974`.
- `(S_v - S_pub)/S_v = 1/5`, not at most `2^-64 + 2^-52`.
- Adding `h/2` absolute does not close the gap.
- `S_pub >= 2^-988`, so A1's small-scale branch does not run for translation.

Every multiplication here is a power-of-two scaling and exact; no error is
being attributed to the coupling multiplication itself. The loss is in its
already-rounded operand. The values fit the retained formats and the body's
binary64 extent calculation; `PrimitiveSource::new` checks finite coordinates
at `source.rs:379` and has no corresponding 2^53 coordinate limit. Current
product capture's separate range restrictions may exclude this large-extent
construction; it is not a product-level witness.

An abstract stop-rule example illustrates why the missing inference matters.
Keep the same rotation values. Set candidate translation to zero and exact
verification/truth to `q* = (9/8)2^-1038`, with zero verification error. Its
disagreement is below `2^-64 S_v = (5/4)2^-1038`, but its published zero carries
`b = 2^-64 S_pub = 2^-1038`. The error is `9b/8`, exceeding the stated
`b(1 + 2^-22)` publication allowance. This shows that the stated inference
does not follow from the disagreement bound and exact verification alone.
It does **not** establish that an unmutated solver generates that candidate.

There is also a zero-scale boundary. With the same rotation and `L = 2^-100`,
the verification translation scale is positive while the binary64 coupled
scale rounds to zero. `row_bound` at `adaptive.rs:382` returns zero at S*=0;
the extra one-subnormal term is not added. Whether a realized case can exploit
that path also remains open.

`_run_records/audit_checks.py arithmetic` uses independent `Fraction` arithmetic
and binary64 conversion, with no K4 generator or implementation imports. Its
checks and exact ratios are preserved in `arithmetic.json`.

### Disposition

**AUD-T3-01, SHOULD-FIX:** the general proof premise is false as stated.
The task for design/implementation is to establish the realized-input boundary
and either derive a valid coupled error allowance or exclude the unsupported
cases by a justified guard. Candidate remedies include an outward upper scale
with suitable receipt evidence, or a separately charged coupling error. These
are options for ROOT/D2, not adopted changes. Boundary controls should cover
both directions of coupling, subnormal raw scales, zero published scales and
the A1 branch threshold. A realized false publication would make this BLOCKING.

## A2: minimum over available certified bounds

Let C be the set of bounds actually completed and certified for a data-carrying
free block, each satisfying `c >= ||K_tilde^-1||_1`. If C is nonempty,
`min(C)` also satisfies that inequality. Removing an unformed candidate cannot
make the remaining minimum too small. If C is empty, the block cannot support
selection. This argument requires neither a Hager–Higham approximation nor a
claim that a failed recurrence's partial value was a bound.

Tracing R7 §5:

- Lemmas D/E certify each completed Uc/S. A2 changes their availability, not
  the arithmetic required for an available bound to be certified.
- Lemma C uses B in the upward theta test and Neumann bound.
- The theorem uses per-block B and per-body maxima in its inverse-norm bounds;
  its t1, t3 and W+ terms require an upper bound, not a particular producer.
- The corollary depends on those inequalities. No step requires both candidates
  to exist, or requires an unavailable candidate's intermediate values.
- No-data blocks retain R7's existing zero-data/nonsingularity premises. A2
  does not independently prove those premises or resolve AUD-T3-01.

Source trace at the audit basis:

| Obligation | Implementation inspected |
|---|---|
| Only formation refusals become unavailable | `bound.rs:295` (`refusable`), `:314` (`refusal_kind`): only Span/Exponent; other stops propagate |
| Refused Uc cannot become a partial certificate | `bound.rs:601` (`BlockBound::refused`), `:669` (`uc_bounds`): `uc: None`; refused blocks skip bound formation |
| Other blocks retain their arithmetic | `bound.rs:365` (`u_pass`), `:473` (`nl_pass`): per-block refusal slots; nonzero factor entries stay within the standing block pattern |
| S has its own refusal state | `bound.rs:1011` (`shift_schedule`), `:1187` (`shift_start`), `:1231` (`shift_run`) |
| A bound is formed only from available candidates | `bound.rs:1149` (`certified`), `:1274` (`certificates`) |
| No bound cannot select | `verify.rs:1021` propagates refusal precedence; `adaptive.rs:2188` rejects `uc_missing` |
| Consumers need an upper bound only | `verify.rs:1052`, `:1096`: theta and per-body maxima; the following t1/t3/W+ formation |
| Refusals survive stopped builds | caller-owned slots and propagation before error return, plus the existing RV23 confirmation evidence |

The early estimate/charge work may still precede missing-bound rejection;
that is conservative for publication and is already disclosed by RV23-N2.
Existing tests and review records cover multiple refusal/budget/cache paths,
but were read rather than freshly executed here.

**Conclusion:** A2's honesty-preservation argument survives this design-level
check under the stated R7 premises. No additional defect was found in the
examined implementation trace. A2's new reachable 1024 phase still invalidates
the old memory estimate, which is the separately routed KF3-B2/K6c repair.
