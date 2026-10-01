# RV29 preparatory return

**COMPLETE for the immediate read-only ledger-preparation phase. NOT the final
independent implementation review; PC40–43/PM16 remain unclosed.**

Original-path independent totals: RU64(1+h) conversion adds **17,942 LME**
(17,506 context + 436 new local accumulator work); specified H/den setup adds
46. Original small-A1 row_bound(h,h)=3h costs **17,563 LME**, including its
8-LME setup and 17,555-LME conversion. Clones' inherited counters are separated
from new deltas; max_span is maximum-only evidence. Reference Span/Exponent
cases retain 2 / 6 / 40 LME as specified in PREPARATION.md and LEDGER.json.

The independent primitive/reference-path ledger is frozen before any new helper
counter observations. Exact candidate totals, successful/rejected/stopped paths,
final H<=b comparison and external short budgets depend on frozen call/checkpoint
order. ROOT should supply the frozen full diff/source identity and separate
execution grant; RV29 then extends this packet by addendum, freezes final
expectations before reading observed results, and owns the complete final review.

No confirmed implementation finding is asserted about unread/changing source.
No Rust/solver/mutant/native evidence, availability claim or acceptance is supplied.
Standard-library arithmetic checks and a byte-identical rerun passed. Five seals
and four original/audit source identities were verified. The initial status-query
environment omission and possible metadata-refresh limit are explicitly disclosed
in EXECUTION.md and were sent to ROOT; no explicit mutating Git command occurred.

Canonical packet: <A1_WT>/<R>/source_review_RV29/preparation_01.
All owned canonical files and hashes are in SHA256SUMS; sources/skill origins
and hashes are in BASIS.json; actual arithmetic output is CHECK.stdout.json.
