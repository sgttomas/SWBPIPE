# Accounting-test clarification — supersedes PC40–43 / PM16 acceptance detail

This additive clarification preserves the original five-file implementation_0
seal 61c0ecebd66c7ad2516dc3f0d2df375247ea51f449dd8bd757392cd2e7a77125.
No implementation, test or additional source investigation occurred.

Stage equality and a short budget computed from that same implementation's
reported cost are insufficient: they can agree while all new local work is
omitted. PC40–43 and PM16 must additionally use an independently enumerated,
frozen operation/limb-work ledger, or an equally independent absolute-work
discriminator, for these fixed paths:

1. **H/RU64 fixture:** x=1, v=0, present W_plus=h, so H=1+h, with finite b=2.
   Correct RU64(H)=1+2^-52. This forces a wide anchored sum and a directed
   correction from the rounded approximation, exposing clone and
   product_reaches scratch work. It is an algebra fixture, not a source witness.
2. **Small-A1 bound fixture:** x=h, S=h, so existing b_row=3h. Independently
   ledger the exact three-term formation, directed conversion, clones and
   comparison scratch; include it in the new candidate work only once.
3. For those fixtures, stop at each implemented budget checkpoint with a limit
   one below the independently enumerated cumulative amount. Include at least
   one stop within conversion/comparison work and the final comparison boundary.
   Separately ledger a forced existing Span/Exponent terminal path, preserving
   work completed before refusal. No new refusal order or fabricated price.

Before relying on the tests, an independent reviewer/checker must freeze a table
naming every primitive call and operand's exact significant/used limb range,
new term_limbs/shift_limbs/net_limbs/rounded_limbs delta, WideContext operation
count, cumulative candidate total and expected stop. It derives those entries
from fixed operand bits and the accepted LME operations/source algorithm, without
reading the implementation's reported work as the expected answer.
The new helper is not implemented yet, so concrete integer ledger entries are
an explicit **pre-execution verification obligation**, not invented results.

For each cloned accumulator record separately:
- counters already present in the source object before cloning;
- newly incurred counter deltas while netting/rounding that clone;
- newly created scratch accumulator/context work;
- which owner charges each delta exactly once.
max_span is evidence aggregated by maximum, never an additive work price.

Acceptance requires exact equality to the frozen absolute ledger on successful,
numeric-rejected and stopped paths, in addition to stage/case/invocation closure.
The helper cannot use the ledger as its own accounting implementation.

PM16 is split into two separately executed patches:
- **PM16-a omission:** suppress accounting of conversion clone/product_reaches
  scratch work while leaving arithmetic unchanged. The independent absolute
  ledger and its externally selected short budgets must fail.
- **PM16-b whole-counter duplication:** merge full clone counters, including
  inherited H/denominator work, rather than only newly incurred deltas.
  The same ledger must detect the overcharge and the changed budget outcome.

Both mutants must fail for the intended accounting reason; incidental snapshot
drift is not enough. If the fixed fixtures do not discriminate either mutant,
the accounting obligation remains open. None is currently claimed killed.
This sharpens RV28-N1 evidence within the existing four-file plan; it grants
no helper rewrite, new host job, source change or expanded test search.

