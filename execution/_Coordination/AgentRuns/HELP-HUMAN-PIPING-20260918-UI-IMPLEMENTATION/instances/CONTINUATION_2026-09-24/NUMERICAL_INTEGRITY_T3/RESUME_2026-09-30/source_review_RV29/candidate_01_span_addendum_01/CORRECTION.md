# RV29 correction: successful b=2 predicate maximum span

**CONFIRMED reviewer ledger metadata error.** Preserve candidate_01 and its
original ledger/packet seals. This addendum supersedes only the successful
H=1+h,b=2 path's aggregate max_span1075 claim in LEDGER_FREEZE.md. The correct
aggregate max_span is **1076**. It does not change any priced work delta, total,
checkpoint limit, clone inheritance, numerical value, predicate or mutant price.

Frozen adaptive.rs:3028–3032 first adds b=2 to the difference, then adds −H.
That first term has high=low=+1. ExactWideSum::add_raw, wide_sum.rs:218–245,
combines the lowest contributing term bit with the highest contributing term
bit, before signed netting: new_low=min(1,−1074)=−1074 and
new_high=max(1,0)=1. Therefore span=1−(−1074)+1=1076. Signed cancellation
does not lower this evidence maximum. H alone still has span1075.

Affected detailed rows: H_success's `predicate: difference.add_scaled(H,true,1,0)`
and following signum carry local max_span1076. When the predicate accumulator
is collected, the meter maximum becomes1076 and remains1076 for later
checkpoints. Individual H/num clones and RU scratch still have their recorded
1075 maxima. The b=1 numeric-rejected predicate has high0 and remains1075.
Small-A1's complete maximum2 and terminal-path maxima are unchanged.

The successful comparison's anchor shift was already correctly priced as1075
bits. old_used2 + floor(1075/64) +1 =19, so both-sign shift price remains38;
term58 and net19 also remain unchanged. Hence predicate work115, successful
path18103, rejected path159, small-A1 path17563, all external one-below budgets,
and the omission/duplication LME discriminators are unchanged. max_span is
maximum-only evidence and contributes no additive LME.

Trigger and independence: ROOT reported I22's max-span assertion failure. RV29
then reread the immutable source and evaluated the integer exponent formula
above. The source derivation independently fixes the corrected value; copying
an observed counter did not supply the expectation. The initial metadata was
wrong and is explicitly corrected, not relabelled as a successful prior check.
No production defect, author fault, Rust rerun or PM16 kill is claimed. ROOT
controls any separately authorized correction/rerun of the new test.
